//! Changes the calls to aggregate return types
//! to make them VAR_IN_OUT calls, allowing them
//! to be called from C_APIs and simplifying code generation
//!
//! As a first step, the POU signature is changed. E.g. a function
//! returning a `STRING` will now return `__VOID` with the return variable
//! being moved into a `VAR_IN_OUT` block:
//! ```iec61131
//!     // user code
//!     FUNCTION foo : STRING
//!     VAR_INPUT
//!         a: DINT;
//!     END_VAR
//!     END_FUNCTION
//! ```
//! ```iec61131
//!     // lowered equivalent
//!     FUNCTION foo
//!     VAR_IN_OUT
//!         foo: STRING;
//!     END_VAR
//!     VAR_INPUT
//!         a: DINT;
//!     END_VAR
//!     END_FUNCTION
//! ```
//!
//! Next, every call-statement to that POU has it's arguments updated, with a temporary
//! variable being allocated to hold the value.
//! Locally allocated variables follow a naming-scheme of `__<function_name><number>`,
//! <number> being a value from an atomically incremented counter to avoid naming conflicts
//! (the same approach is used for allocated variables in LLVM-IR).
//! ```iec61131
//!     // user code. Let `s` be a variable of type `STRING`
//!     // ...
//!     s := foo(42);
//!     // ...
//! ```
//! ```iec61131
//!     // lowered equivalent
//!     // ...
//!     alloca __foo1 : STRING;
//!     foo(__foo1, 42);
//!     s := __foo1;
//!     // ...
//! ```

use std::{borrow::BorrowMut, sync::atomic::AtomicI32};

use plc_ast::{
    ast::{
        flatten_expression_list, steal_expression_list, AccessModifier, Allocation, Assignment, AstFactory,
        AstId, AstNode, AstStatement, CallStatement, CompilationUnit, LinkageType, Pou, Variable,
        VariableBlock, VariableBlockType,
    },
    control_statements::{AstControlStatement, ConditionalBlock},
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from_mut,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    lowering::helper::create_member_reference_with_location,
    resolver::{AnnotationMap, StatementAnnotation},
    typesystem::{DataType, DataTypeInformation},
    validation::statement::evaluate_generic_nature_violation,
};

// Performs lowering for aggregate types defined in functions
#[derive(Default)]
pub struct AggregateTypeLowerer {
    pub index: Option<Index>,
    pub annotation: Option<Box<dyn AnnotationMap>>,
    pub id_provider: IdProvider,
    /// New statements to be added during visit, that should happen before the call. This should always be drained when read
    pre_stmts: Vec<Vec<AstNode>>,
    /// New statements to be added during visit, that should happen after the call. This should always be drained when read
    post_stmts: Vec<Vec<AstNode>>,
    counter: AtomicI32,
    /// Diagnostics collected while visiting. Reported by the pipeline after lowering (see the
    /// `PipelineParticipantMut::diagnostics` impl). We must emit generic type-nature errors here,
    /// before lowering rewrites a generic call (e.g. `CONCAT`) into its concrete instantiation and
    /// the subsequent re-annotation erases the generic nature the check relies on.
    pub diagnostics: Vec<Diagnostic>,
}

impl AggregateTypeLowerer {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { id_provider, ..Default::default() }
    }

    pub fn visit(&mut self, units: &mut [CompilationUnit]) {
        units.iter_mut().for_each(|u| self.visit_compilation_unit(u));
    }

    pub fn visit_unit(&mut self, unit: &mut CompilationUnit) {
        self.visit_compilation_unit(unit);
    }

    /// Collects `E062` diagnostics for call arguments whose actual type does not satisfy the nature
    /// of the generic parameter they are bound to (e.g. a `DINT` passed to a `T: ANY_STRING`
    /// variadic).
    ///
    /// This only fires for generic calls that this lowerer rewrites to their concrete instantiation
    /// (e.g. `CONCAT` -> `CONCAT__STRING`): that rewrite plus the subsequent re-annotation erases
    /// the generic nature, so the regular `validate_type_nature` pass can no longer see it. Every
    /// other generic call keeps its nature and is reported by that pass, so we must skip them here
    /// to avoid duplicate diagnostics. The rewrite happens iff the resolved function is a concrete
    /// (non-generic) instantiation returning an aggregate type (see `visit_call_statement`).
    fn collect_generic_nature_violations(&self, stmt: &CallStatement) -> Vec<Diagnostic> {
        let Some((annotation, index)) = self.annotation.as_ref().zip(self.index.as_ref()) else {
            return Vec::new();
        };

        let Some(StatementAnnotation::Function { qualified_name, generic_name, return_type, .. }) =
            annotation.get(&stmt.operator).or_else(|| annotation.get_hint(&stmt.operator))
        else {
            return Vec::new();
        };

        let is_rewritten_generic = generic_name.is_some()
            && index.find_pou(qualified_name).is_some_and(|it| !it.is_generic() && !it.is_builtin())
            && index.get_effective_type_or_void_by_name(return_type).is_aggregate_type();
        if !is_rewritten_generic {
            return Vec::new();
        }

        let Some(parameters) = stmt.parameters.as_deref() else {
            return Vec::new();
        };

        let mut diagnostics = Vec::new();
        for argument in flatten_expression_list(parameters) {
            // The nature is attached to the passed value; for `x := val` that is the right-hand side.
            let value = match argument.get_stmt() {
                AstStatement::Assignment(data) | AstStatement::OutputAssignment(data) => &data.right,
                _ => argument,
            };

            let (Some(actual_type), Some(nature)) =
                (annotation.get_type(value, index), annotation.get_generic_nature(value))
            else {
                continue;
            };
            let type_hint = annotation.get_type_hint(value, index).unwrap_or(actual_type);

            if let Some(diagnostic) =
                evaluate_generic_nature_violation(actual_type, type_hint, *nature, index, value)
            {
                diagnostics.push(diagnostic);
            }
        }
        diagnostics
    }

    fn steal_and_walk_list(&mut self, list: &mut Vec<AstNode>) {
        //Enter new scope
        let mut new_stmts = vec![];
        for stmt in list.drain(..) {
            new_stmts.push(self.map(stmt));
        }
        std::mem::swap(list, &mut new_stmts);
    }

    fn walk_conditional_blocks(&mut self, blocks: &mut Vec<ConditionalBlock>) {
        for b in blocks {
            b.condition.walk(self);
            self.steal_and_walk_list(&mut b.body);
        }
    }

    fn push_pre_statement(&mut self, stmt: AstNode) {
        if let Some(stmts) = self.pre_stmts.last_mut() {
            stmts.push(stmt);
        } else {
            unreachable!("Statement lists should exist at this point");
        }
    }

    fn push_pre_statements(&mut self, stmts: &[AstNode]) {
        stmts.iter().for_each(|stmt| self.push_pre_statement(stmt.clone()));
    }

    fn push_post_statement(&mut self, stmt: AstNode) {
        if let Some(stmts) = self.post_stmts.last_mut() {
            stmts.push(stmt);
        } else {
            unreachable!("Statement lists should exist at this point");
        }
    }

    fn push_post_statements(&mut self, stmts: &[AstNode]) {
        stmts.iter().for_each(|stmt| self.push_post_statement(stmt.clone()));
    }

    fn enter_scope(&mut self) {
        self.pre_stmts.push(vec![]);
        self.post_stmts.push(vec![]);
    }

    fn exit_scope(&mut self) -> (Option<Vec<AstNode>>, Option<Vec<AstNode>>) {
        (self.pre_stmts.pop(), self.post_stmts.pop())
    }
}

impl AstVisitorMut for AggregateTypeLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut plc_ast::ast::CompilationUnit) {
        if self.index.is_none() {
            //don't walk if we have no index to use
            return;
        }
        unit.walk(self);
    }

    fn visit_interface(&mut self, interface: &mut plc_ast::ast::Interface) {
        interface.methods.iter_mut().for_each(|m| self.visit_pou(m));
    }

    // Change the signature for functions/methods with aggregate returns
    fn visit_pou(&mut self, pou: &mut Pou) {
        if pou.is_aggregate() || pou.is_generic() {
            //Skip types that have already been made aggregate or are generics
            return;
        }
        let index = self.index.as_ref().expect("Can't get here without an index");
        // Check if POU has a return type
        let Some(return_type_name) = pou
            .return_type
            .as_ref()
            .map(|it| it.get_name().expect("We should have names at this point").to_string())
        else {
            return;
        };

        // If the return type is aggregate, remove it from the signature and add a matching variable
        // in a VAR_IN_OUT block
        if index.get_effective_type_or_void_by_name(&return_type_name).is_aggregate_type() {
            let original_return = pou.return_type.take().unwrap();
            let location = original_return.get_location();
            //Create a new return type for the pou
            pou.return_type.replace(plc_ast::ast::DataTypeDeclaration::Aggregate {
                referenced_type: return_type_name,
                location,
            });
            //Insert a new in out var to the pou variable block declarations
            let block = VariableBlock {
                access: AccessModifier::Public,
                constant: false,
                retain: false,
                variables: vec![Variable {
                    name: pou.get_return_name().to_string(),
                    data_type_declaration: original_return,
                    initializer: None,
                    address: None,
                    location: pou.name_location.clone(),
                }],
                kind: VariableBlockType::InOut,
                linkage: LinkageType::Internal,
                location: SourceLocation::internal(),
            };
            pou.variable_blocks.insert(0, block)
        }
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        if self.annotation.is_none() {
            return;
        }
        let mut new_stmts = vec![];
        for stmt in implementation.statements.drain(..) {
            new_stmts.push(self.map(stmt));
        }
        implementation.statements.extend(new_stmts);
    }

    fn map(&mut self, mut node: AstNode) -> AstNode {
        self.enter_scope();
        node.borrow_mut().walk(self);

        let (pre_stmts, post_stmts) = self.exit_scope();
        let mut pre_stmts = pre_stmts.unwrap_or_default();
        let mut post_stmts = post_stmts.unwrap_or_default();

        if pre_stmts.is_empty() && post_stmts.is_empty() {
            node
        } else {
            let location = node.get_location();
            pre_stmts.push(node);
            pre_stmts.append(&mut post_stmts);
            AstFactory::create_expression_list(pre_stmts, location, self.id_provider.next_id())
        }
    }

    fn visit_assignment(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, Assignment).expect("Assignment");
        stmt.walk(self);
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let original_location = node.get_location();
        let mut deferred_reference = None;

        {
            let stmt = try_from_mut!(node, CallStatement).expect("CallStatement");
            stmt.walk(self);

            // Report generic type-nature violations (e.g. `CONCAT(aDint, aReal)`) now, while the call is
            // still generic. The normal validator can only catch these before lowering; once we rewrite
            // the call to its concrete instantiation the re-annotation drops the generic nature. Mirrors
            // the diagnostics-during-lowering approach used by the property lowerer.
            let nature_diagnostics = self.collect_generic_nature_violations(stmt);
            self.diagnostics.extend(nature_diagnostics);

            let (
                qualified_name,
                return_type_name,
                return_name,
                is_builtin_function,
                is_method,
                can_have_output_assignment_lowering,
                is_generic_function,
                call_name,
                is_fnptr_call,
                has_aggregate_return,
            ) = {
                let Some((annotation, index)) = self.annotation.as_ref().zip(self.index.as_ref()) else {
                    //Early exit if not annotated or indexed
                    return;
                };
                //Get the function being called
                let (qualified_name, return_type_name, generic_name, call_name) = match annotation
                    .get(&stmt.operator)
                    .or_else(|| annotation.get_hint(&stmt.operator))
                    .cloned()
                {
                    Some(StatementAnnotation::Function {
                        return_type,
                        qualified_name,
                        generic_name,
                        call_name,
                    }) => (qualified_name, return_type, generic_name, call_name),
                    Some(StatementAnnotation::FunctionPointer { return_type, qualified_name }) => {
                        (qualified_name, return_type, None, None)
                    }
                    _ => return,
                };
                //If there's a call name in the function, it is a generic and needs to be replaced.
                //HACK: this is because we don't lower generics
                let function_entry = index.find_pou(&qualified_name).expect("Function not found");
                let return_name = Pou::calc_return_name(function_entry.get_name()).to_string();
                let has_aggregate_return =
                    index.get_effective_type_or_void_by_name(&return_type_name).is_aggregate_type();

                let generic_function: Option<&crate::index::PouIndexEntry> =
                    generic_name.as_deref().and_then(|it| index.find_pou(it));
                let is_generic_function = generic_function.is_some_and(|it| it.is_generic());
                let is_fnptr_call = annotation.get(&stmt.operator).is_some_and(|opt| opt.is_fnptr());
                let is_method = function_entry.is_method();

                (
                    qualified_name,
                    return_type_name,
                    return_name,
                    function_entry.is_builtin(),
                    is_method,
                    function_entry.is_function() || function_entry.is_method(),
                    is_generic_function,
                    call_name,
                    is_fnptr_call,
                    has_aggregate_return,
                )
            };

            //TODO: needs to be on the function
            if has_aggregate_return && !is_builtin_function {
                //TODO: use qualified name
                let name = format!(
                    "__{}{}",
                    stmt.operator.get_flat_reference_name().unwrap_or_default(),
                    self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                );
                //Create an allocation of the new type
                let alloca = AstNode {
                    stmt: AstStatement::AllocationStatement(Allocation {
                        name: name.clone(),
                        reference_type: return_type_name.to_string(),
                    }),
                    id: self.id_provider.next_id(),
                    location: original_location.clone(),
                    metadata: None,
                };
                self.push_pre_statement(alloca);
                let location = stmt.parameters.as_ref().map(|it| it.get_location()).unwrap_or_default();
                let id = stmt.parameters.as_ref().map(|it| it.get_id()).unwrap_or(self.id_provider.next_id());
                let reference = create_member_reference_with_location(
                    &name,
                    self.id_provider.clone(),
                    None,
                    original_location.clone(),
                );
                //If the function has an formal arguments (foo(x := 1)), we need to add an assignment to the reference
                let reference = if stmt
                    .parameters
                    .as_ref()
                    .map(|it| flatten_expression_list(it))
                    .is_some_and(|it| it.iter().any(|it| it.is_assignment()))
                {
                    let left = AstFactory::create_member_reference(
                        AstFactory::create_identifier(
                            &return_name,
                            original_location.clone(),
                            self.id_provider.next_id(),
                        ),
                        None,
                        self.id_provider.next_id(),
                    );
                    AstFactory::create_assignment(left, reference, self.id_provider.next_id())
                } else {
                    reference
                };
                //TODO : we are creating th expression list twice in case of no params
                let mut parameters = stmt
                    .parameters
                    .as_mut()
                    .map(|it| steal_expression_list(it.borrow_mut()))
                    .unwrap_or_default();

                // Place the alloca aggregate variable at index 1 when dealing with function pointers because 0
                // is reserved for the instance variable.
                if is_fnptr_call {
                    parameters.insert(1, reference);
                } else {
                    parameters.insert(0, reference);
                }

                if is_generic_function {
                    // For generic functions, replace the operator with the concrete monomorphization
                    // to call. Prefer `call_name` (the resolved monomorph, e.g. `TO_STRING__DINT`)
                    // over `qualified_name`: when no concrete monomorph exists the resolver leaves
                    // `qualified_name` pointing at the *generic* itself, and resetting to that makes
                    // the re-annotation re-monomorphize against the injected aggregate return buffer —
                    // binding e.g. `TO_STRING(aDint)` to `TO_STRING__STRING` and reinterpreting the
                    // argument's bytes as a string. Emitting the true monomorph name instead yields an
                    // unresolved reference the compiler rejects.
                    let concrete_name = call_name.as_deref().unwrap_or(&qualified_name);
                    *stmt.operator = AstFactory::create_member_reference(
                        AstFactory::create_identifier(
                            concrete_name,
                            stmt.operator.get_location(),
                            self.id_provider.next_id(),
                        ),
                        None,
                        self.id_provider.next_id(),
                    )
                };
                stmt.parameters
                    .replace(Box::new(AstFactory::create_expression_list(parameters, location, id)));
                deferred_reference = Some(create_member_reference_with_location(
                    &name,
                    self.id_provider.clone(),
                    None,
                    original_location.clone(),
                ));
            }

            // We actually need to do this as well not instead of
            // Is this a function with any output assignments?
            // Do any of the output assignments have types that need casting?
            // Do any of the output assignments have direct access?
            let needs_output_assignment_lowering = {
                let Some((annotation, index)) = self.annotation.as_ref().zip(self.index.as_ref()) else {
                    return;
                };

                let aggregate_return_slot_present = index
                    .get_declared_parameter(&qualified_name, 0)
                    .is_some_and(|parameter| parameter.get_name() == return_name);

                stmt.parameters.as_ref().iter().any(|param| {
                    is_output_assignment_and_type_cast_needed(
                        param,
                        annotation.as_ref(),
                        index,
                        aggregate_return_slot_present,
                    ) || is_output_assignment_and_has_direct_access(
                        param,
                        annotation.as_ref(),
                        index,
                        aggregate_return_slot_present,
                    ) || is_output_assignment_of_sized_string(
                        param,
                        annotation.as_ref(),
                        index,
                        aggregate_return_slot_present,
                    )
                })
                    // Stateful structs (such as function blocks) have their own output assignment mechanism in codegen,
                    // and are not handled by this lowerer
                    && can_have_output_assignment_lowering
            };

            if needs_output_assignment_lowering {
                let mut pre_statements: Vec<AstNode> = Vec::new();
                let mut post_statements: Vec<AstNode> = Vec::new();
                let mut expressions: Vec<AstNode> = Vec::new();

                let aggregate_return_slot_present = self.index.as_ref().is_some_and(|index| {
                    index
                        .get_declared_parameter(&qualified_name, 0)
                        .is_some_and(|parameter| parameter.get_name() == return_name)
                });

                let location = stmt.parameters.as_ref().map(|it| it.get_location()).unwrap_or_default();
                let id = stmt.parameters.as_ref().map(|it| it.get_id()).unwrap_or(self.id_provider.next_id());

                for param in stmt.parameters.as_ref().iter() {
                    lower_output_assignments(
                        (&self.counter, self.id_provider.clone()),
                        param,
                        (&qualified_name, &return_name, is_method),
                        self.annotation.as_deref().expect("annotation exists when lowering calls"),
                        self.index.as_ref().expect("index exists when lowering calls"),
                        aggregate_return_slot_present,
                        (&mut pre_statements, &mut expressions, &mut post_statements),
                    );
                }

                stmt.parameters.replace(Box::new(AstFactory::create_expression_list(
                    expressions,
                    location,
                    id,
                )));

                self.push_pre_statements(&pre_statements);
                self.push_post_statements(&post_statements);
            }
        }

        if let Some(mut reference) = deferred_reference {
            let previous_stmt = std::mem::replace(node.get_stmt_mut(), reference.stmt);
            reference.stmt = previous_stmt;
            self.push_pre_statement(reference);
        }
    }

    fn visit_control_statement(&mut self, node: &mut AstNode) {
        let ctrl_stmt = try_from_mut!(node, AstControlStatement).expect("ControlStatement");
        match ctrl_stmt {
            AstControlStatement::If(stmt) => {
                self.walk_conditional_blocks(&mut stmt.blocks);
                self.steal_and_walk_list(&mut stmt.else_block);
            }
            AstControlStatement::ForLoop(stmt) => {
                stmt.counter.walk(self);
                stmt.start.walk(self);
                stmt.end.walk(self);
                if let Some(ref mut step) = stmt.by_step {
                    step.walk(self);
                }
                self.steal_and_walk_list(&mut stmt.body);
            }
            AstControlStatement::Case(stmt) => {
                stmt.selector.walk(self);
                self.walk_conditional_blocks(&mut stmt.case_blocks);
                self.steal_and_walk_list(&mut stmt.else_block);
            }
            AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
                stmt.condition.walk(self);
                self.steal_and_walk_list(&mut stmt.body);
            }
        }
    }
}

fn create_member_reference_by_name(
    name: &str,
    location: SourceLocation,
    identifier_id: AstId,
    reference_id: AstId,
) -> AstNode {
    AstFactory::create_member_reference(
        AstFactory::create_identifier(name, location, identifier_id),
        None,
        reference_id,
    )
}

fn is_output_assignment_and_type_cast_needed(
    node: &AstNode,
    annotations: &dyn AnnotationMap,
    index: &Index,
    aggregate_return_slot_present: bool,
) -> bool {
    match &node.stmt {
        AstStatement::ExpressionList(nodes) => nodes.iter().any(|node| {
            is_output_assignment_and_type_cast_needed(node, annotations, index, aggregate_return_slot_present)
        }),
        AstStatement::OutputAssignment(assignment) => {
            // For output assignment in a call these types need to be swapped
            // output => value_to_assign_to --> should be evaluated as value_to_assign_to := output
            let type_lhs = annotations.get_type_or_void(&assignment.right, index);
            let type_rhs = annotations.get_type_or_void(&assignment.left, index);

            // Aggregate types are handled by codegen
            if type_lhs.is_aggregate_type() || type_rhs.is_aggregate_type() {
                return false;
            }

            type_cast_needed(type_lhs, type_rhs, index)
        }
        _ => {
            // We don't want to accidentally assign a pointer back to a literal that is passed
            if node.is_literal() {
                return false;
            }

            let Some((declaring_pou_name, param_index)) = find_argument_parameter_position(annotations, node)
            else {
                return false;
            };

            let param_index = if aggregate_return_slot_present { param_index + 1 } else { param_index };

            let Some(param_index_entry) = index.get_declared_parameter(declaring_pou_name, param_index)
            else {
                return false;
            };

            if !param_index_entry.is_output() {
                return false;
            }

            let Some(type_lhs_pointer) = index.find_effective_type_by_name(&param_index_entry.data_type_name)
            else {
                return false;
            };

            let type_lhs = match type_lhs_pointer.get_type_information() {
                DataTypeInformation::Pointer { inner_type_name, .. } => {
                    let Some(type_lhs) = index.find_effective_type_by_name(inner_type_name) else {
                        return false;
                    };

                    type_lhs
                }
                _ => type_lhs_pointer,
            };

            let type_rhs = annotations.get_type_or_void(node, index);

            // Aggregate types are handled by codegen
            if type_lhs.is_aggregate_type() || type_rhs.is_aggregate_type() {
                return false;
            }

            type_cast_needed(type_lhs, type_rhs, index)
        }
    }
}

fn type_cast_needed(type_lhs: &DataType, type_rhs: &DataType, index: &Index) -> bool {
    // If either type is void then this is an empty assignment and casting is not necessary
    if type_lhs.is_void() || type_rhs.is_void() {
        return false;
    }

    let type_info_lhs = type_lhs.get_type_information();
    let type_info_rhs = type_rhs.get_type_information();

    let Ok(size_lhs) = type_info_lhs.get_size(index) else {
        return false;
    };
    let Ok(size_rhs) = type_info_rhs.get_size(index) else {
        return false;
    };

    let types_are_the_same_size = size_lhs == size_rhs;

    !types_are_the_same_size
        || type_info_lhs.is_signed_int() && type_info_rhs.is_unsigned_int()
        || type_info_lhs.is_int() && type_info_rhs.is_float()
}

fn string_type_alloc_needed(type_lhs: &DataType, type_rhs: &DataType, index: &Index) -> bool {
    // If either type is void then this is an empty assignment and casting is not necessary
    if type_lhs.is_void() || type_rhs.is_void() {
        return false;
    }

    // Work with effective types because many call parameters are represented as
    // alias wrapper types (e.g. __Foo_Bar).
    let type_lhs = index.find_effective_type_by_name(type_lhs.get_name()).unwrap_or(type_lhs);
    let type_rhs = index.find_effective_type_by_name(type_rhs.get_name()).unwrap_or(type_rhs);

    // This helper is only relevant for string-like outputs.
    if !type_lhs.is_string() || !type_rhs.is_string() {
        return false;
    }

    // If both sides are unsized strings we keep the existing direct path.
    if !type_lhs.is_sized_string() && !type_rhs.is_sized_string() {
        return false;
    }

    // If only one side is explicitly sized, lower conservatively to avoid writing
    // directly into a potentially smaller implicit buffer.
    if type_lhs.is_sized_string() != type_rhs.is_sized_string() {
        return true;
    }

    let type_info_lhs = type_lhs.get_type_information();
    let type_info_rhs = type_rhs.get_type_information();

    let Ok(size_lhs) = type_info_lhs.get_size(index) else {
        return false;
    };
    let Ok(size_rhs) = type_info_rhs.get_size(index) else {
        return false;
    };

    size_lhs > size_rhs
}

fn is_output_assignment_and_has_direct_access(
    node: &AstNode,
    annotations: &dyn AnnotationMap,
    index: &Index,
    aggregate_return_slot_present: bool,
) -> bool {
    match &node.stmt {
        AstStatement::ExpressionList(nodes) => nodes.iter().any(|node| {
            is_output_assignment_and_has_direct_access(
                node,
                annotations,
                index,
                aggregate_return_slot_present,
            )
        }),
        AstStatement::OutputAssignment(assignment) => {
            let node_type = annotations.get_type_or_void(assignment.right.as_ref(), index);

            // Aggregate types are handled by codegen
            if node_type.is_aggregate_type() {
                return false;
            }

            assignment.right.has_direct_access()
        }
        _ => {
            // We don't want to accidentally assign a pointer back to a literal that is passed
            if node.is_literal() {
                return false;
            }

            let Some((declaring_pou_name, param_index)) = find_argument_parameter_position(annotations, node)
            else {
                return false;
            };

            let param_index = if aggregate_return_slot_present { param_index + 1 } else { param_index };

            let Some(param_index_entry) = index.get_declared_parameter(declaring_pou_name, param_index)
            else {
                return false;
            };

            if !param_index_entry.is_output() {
                return false;
            }

            let node_type = annotations.get_type_or_void(node, index);

            // Aggregate types are handled by codegen
            if node_type.is_aggregate_type() {
                return false;
            }

            node.has_direct_access()
        }
    }
}

fn is_output_assignment_of_sized_string(
    node: &AstNode,
    annotations: &dyn AnnotationMap,
    index: &Index,
    aggregate_return_slot_present: bool,
) -> bool {
    match &node.stmt {
        AstStatement::ExpressionList(nodes) => nodes.iter().any(|node| {
            is_output_assignment_of_sized_string(node, annotations, index, aggregate_return_slot_present)
        }),
        AstStatement::OutputAssignment(assignment) => {
            let left_node_type = annotations.get_type_or_void(assignment.left.as_ref(), index);
            let right_node_type = annotations.get_type_or_void(assignment.right.as_ref(), index);

            // Empty assignments don't need to be lowered
            if left_node_type.is_void() || right_node_type.is_void() {
                return false;
            }

            string_type_alloc_needed(left_node_type, right_node_type, index)
        }
        _ => {
            // We don't want to accidentally assign a pointer back to a literal that is passed
            if node.is_literal() {
                return false;
            }

            let Some((declaring_pou_name, param_index)) = find_argument_parameter_position(annotations, node)
            else {
                return false;
            };

            let param_index = if aggregate_return_slot_present { param_index + 1 } else { param_index };

            let Some(param_index_entry) = index.get_declared_parameter(declaring_pou_name, param_index)
            else {
                return false;
            };

            if !param_index_entry.is_output() {
                return false;
            }

            let Some(type_lhs_pointer) = index.find_effective_type_by_name(&param_index_entry.data_type_name)
            else {
                return false;
            };

            let type_lhs = match type_lhs_pointer.get_type_information() {
                DataTypeInformation::Pointer { inner_type_name, .. } => {
                    let Some(type_lhs) = index.find_effective_type_by_name(inner_type_name) else {
                        return false;
                    };

                    type_lhs
                }
                _ => type_lhs_pointer,
            };

            let type_rhs = annotations.get_type_or_void(node, index);

            string_type_alloc_needed(type_lhs, type_rhs, index)
        }
    }
}

fn find_argument_parameter_position<'a>(
    annotations: &'a dyn AnnotationMap,
    node: &AstNode,
) -> Option<(&'a str, u32)> {
    match annotations.get(node) {
        Some(StatementAnnotation::Argument { position, pou, .. }) => Some((pou.as_str(), *position as u32)),
        _ => match annotations.get_hint(node) {
            Some(StatementAnnotation::Argument { position, pou, .. }) => {
                Some((pou.as_str(), *position as u32))
            }
            _ => None,
        },
    }
}

fn lower_output_assignments(
    (counter, id_provider): (&AtomicI32, IdProvider),
    param: &AstNode,
    (qualified_pou_name, pou_name, is_method): (&str, &str, bool),
    annotations: &dyn AnnotationMap,
    index: &Index,
    aggregate_return_slot_present: bool,
    (pre_statements, expressions, post_statements): (&mut Vec<AstNode>, &mut Vec<AstNode>, &mut Vec<AstNode>),
) {
    let should_be_lowered =
        is_output_assignment_and_type_cast_needed(param, annotations, index, aggregate_return_slot_present)
            || is_output_assignment_and_has_direct_access(
                param,
                annotations,
                index,
                aggregate_return_slot_present,
            )
            || is_output_assignment_of_sized_string(param, annotations, index, aggregate_return_slot_present);

    match &param.stmt {
        AstStatement::ExpressionList(list) => {
            list.iter().for_each(|item| {
                lower_output_assignments(
                    (counter, id_provider.clone()),
                    item,
                    (qualified_pou_name, pou_name, is_method),
                    annotations,
                    index,
                    aggregate_return_slot_present,
                    (pre_statements, expressions, post_statements),
                )
            });
        }
        AstStatement::OutputAssignment(output_assignment) => {
            if !should_be_lowered {
                expressions.push(param.clone());
                return;
            }

            let Some(left_type) = annotations.get_type(&output_assignment.left, index) else {
                // If this fails, simply return the expression
                expressions.push(param.clone());
                return;
            };

            let Some(left_reference_name) = output_assignment.left.get_flat_reference_name() else {
                // If this fails, simply return the expression
                expressions.push(param.clone());
                return;
            };

            lower_output_assignment(
                (counter, id_provider),
                (left_type, left_reference_name, Some(&output_assignment.left)),
                &output_assignment.right,
                pou_name,
                &param.location,
                (pre_statements, expressions, post_statements),
            );
        }
        _ => {
            if !should_be_lowered {
                expressions.push(param.clone());
                return;
            }
            let Some((declaring_pou_name, param_index)) =
                find_argument_parameter_position(annotations, param)
            else {
                // If this fails, simply return the expression
                expressions.push(param.clone());
                return;
            };

            let param_index = if aggregate_return_slot_present { param_index + 1 } else { param_index };

            let Some(param_index_entry) = index.get_declared_parameter(declaring_pou_name, param_index)
            else {
                // If this fails, simply return the expression
                expressions.push(param.clone());
                return;
            };

            if !param_index_entry.is_output() {
                expressions.push(param.clone());
                return;
            }

            let Some(left_pointer_type) =
                index.find_effective_type_by_name(&param_index_entry.data_type_name)
            else {
                // If this fails, simply return the expression
                expressions.push(param.clone());
                return;
            };

            let left_type = match left_pointer_type.get_type_information() {
                DataTypeInformation::Pointer { inner_type_name, .. } => {
                    let Some(left_type) = index.find_effective_type_by_name(inner_type_name) else {
                        // If this fails, simply return the expression
                        expressions.push(param.clone());
                        return;
                    };

                    left_type
                }
                _ => left_pointer_type,
            };

            lower_output_assignment(
                (counter, id_provider),
                (left_type, param_index_entry.get_name(), None),
                param,
                pou_name,
                &param.location,
                (pre_statements, expressions, post_statements),
            );
        }
    }
}

fn lower_output_assignment(
    (counter, mut id_provider): (&AtomicI32, IdProvider),
    (left_type, left_reference_name, left_assignment): (&DataType, &str, Option<&AstNode>),
    right_assignment: &AstNode,
    pou_name: &str,
    location: &SourceLocation,
    (pre_statements, expressions, post_statements): (&mut Vec<AstNode>, &mut Vec<AstNode>, &mut Vec<AstNode>),
) {
    // Generate a temporary variable of the type that the function is expecting and pass it by reference to the function
    let name = format!(
        "__{}_{}{}",
        pou_name,
        left_reference_name,
        counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    );

    let alloca = AstNode {
        stmt: AstStatement::AllocationStatement(Allocation {
            name: name.clone(),
            reference_type: left_type.get_name().to_string(),
        }),
        id: id_provider.next_id(),
        location: location.clone(),
        metadata: None,
    };
    pre_statements.push(alloca);

    // Replace variable in parameters with our new temporary variable that will be passed to the function
    if let Some(left_assignment) = left_assignment {
        expressions.push(AstFactory::create_output_assignment(
            left_assignment.clone(),
            create_member_reference_by_name(
                &name,
                location.clone(),
                id_provider.next_id(),
                id_provider.next_id(),
            ),
            id_provider.next_id(),
        ));
    } else {
        expressions.push(create_member_reference_by_name(
            &name,
            location.clone(),
            id_provider.next_id(),
            id_provider.next_id(),
        ));
    }

    // After the call is complete, assign the temporary variable that was passed to the function back to the original variable
    post_statements.push(AstFactory::create_assignment(
        right_assignment.clone(),
        create_member_reference_by_name(
            &name,
            location.clone(),
            id_provider.next_id(),
            id_provider.next_id(),
        ),
        id_provider.next_id(),
    ));
}

#[cfg(test)]
mod tests {
    use insta::{assert_debug_snapshot, assert_snapshot};
    use plc_ast::mut_visitor::AstVisitorMut;
    use plc_ast::provider::IdProvider;
    use plc_ast::ser::AstSerializer;
    use plc_lowering::control_statement::ControlStatementLowerer;
    use pretty_assertions::assert_eq;

    use crate::index::indexer;
    use crate::lowering::calls::AggregateTypeLowerer;
    use crate::test_utils::tests::{
        annotate_and_lower_with_ids, annotate_with_ids, index as test_index, index_and_lower,
        index_unit_with_id, index_with_ids,
    };

    #[test]
    fn function_with_simple_return_not_changed() {
        let (mut unit, index) = test_index(
            r#"
        FUNCTION simpleFunc : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
        simpleFunc := 5;
        END_FUNCTION
        "#,
        );

        let (original_unit, _index) = test_index(
            r#"
        FUNCTION simpleFunc : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
        simpleFunc := 5;
        END_FUNCTION
        "#,
        );

        let mut lowerer = AggregateTypeLowerer { index: Some(index), annotation: None, ..Default::default() };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(indexer::index(&unit));
        assert_eq!(unit, original_unit);
        assert_debug_snapshot!(lowerer.index.unwrap().find_pou_type("simpleFunc").unwrap());
    }

    #[test]
    fn function_with_string_return_is_changed() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexType : STRING
        VAR_INPUT
            x : DINT;
        END_VAR
        complexType := 'hello';
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        assert_debug_snapshot!(unit.pous[0]);
        assert_debug_snapshot!(lowerer.index.unwrap().find_pou_type("complexType").unwrap());
    }

    #[test]
    fn method_with_string_return_is_changed() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION_BLOCK fb
        METHOD complexMethod : STRING
            complexMethod := 'hello';
        END_METHOD
        END_FUNCTION_BLOCK
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };

        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        assert_debug_snapshot!(unit.pous[1]);
        assert_debug_snapshot!(lowerer.index.unwrap().find_pou_type("fb.complexMethod").unwrap());
    }

    #[test]
    fn simple_call_statement() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION simpleFunc : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
        simpleFunc := 5;
        END_FUNCTION

        FUNCTION main
            simpleFunc();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        //re-index the new unit
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        //Reparse the original unit without modifications
        let (original_unit, _index) = test_index(
            r#"
        FUNCTION simpleFunc : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
        simpleFunc := 5;
        END_FUNCTION

        FUNCTION main
            simpleFunc();
        END_FUNCTION
        "#,
        );

        assert_eq!(unit, original_unit);
    }

    #[test]
    fn complex_call_statement_in_body() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR
            x : DINT;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
            // Should turn to
            // __alloca __complexFunc1 : STRING;
            // complexFunc(__complexFunc1);
            // __complexFunc1;
            complexFunc();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_assignment_method() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION_BLOCK fb
        METHOD complexMethod : STRING
            complexMethod := 'hello';
        END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
        VAR
            a : STRING;
            myFb : fb;
        END_VAR
            // Should turn to
            // __alloca __complexFunc1 : STRING;
            // complexFunc(__complexFunc1);
            // a := __complexFunc1;
            a := myFb.complexMethod();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };

        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[2]);
    }

    #[test]
    fn complex_call_statement_in_assignment() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR
            x : DINT;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
            // Should turn to
            // __alloca __complexFunc1 : STRING;
            // complexFunc(__complexFunc1);
            // a := __complexFunc1;
            a := complexFunc();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_call() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR_INPUT
            x : STRING;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
            //Should be turned to:
            //alloca __complexFunc1 : STRING;
            //complexFunc(__complexFunc1, 'hello');
            //alloca __complexFunc2;
            //complexFunc(__complexFunc2, __complexFunc1);
            //a := __complexFunc2;
            a := complexFunc(x := complexFunc(x := 'hello'));
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_call_with_implicit_parameter() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR_INPUT
            x : STRING;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a, b : STRING; END_VAR
            //Should be turned to:
            //alloca __complexFunc1 : STRING;
            //complexFunc(__complexFunc1, b);
            //alloca __complexFunc2;
            //complexFunc(__complexFunc2, __complexFunc1);
            //a := __complexFunc2;
            a := complexFunc(x := complexFunc(b));
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_call_with_implicit_literal_parameter() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR_INPUT
            x : STRING;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
            //Should be turned to:
            //alloca __complexFunc1 : STRING;
            //complexFunc(__complexFunc1, 'hello');
            //alloca __complexFunc2;
            //complexFunc(__complexFunc2, __complexFunc1);
            //a := __complexFunc2;
            a := complexFunc(x := complexFunc('hello'));
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_assignment_twice() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR_INPUT
            x : DINT;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
            // Should turn to
            // __alloca __complexFunc1 : STRING;
            // complexFunc(__complexFunc1);
            // a := __complexFunc1;
            a := complexFunc();
            // Should turn to
            // __alloca __complexFunc2 : STRING;
            // complexFunc(__complexFunc2);
            // a := __complexFunc2;
            a := complexFunc();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_if_statement_body() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
        IF TRUE THEN
            //Should be turned to:
            //alloca __complexFunc1 : STRING;
            //complexFunc(__complexFunc1);
            //a := __complexFunc1;
            a := complexFunc();
        END_IF

        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_if_statement_condition() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
        //Should be turned to:
        //alloca __complexFunc1 : STRING;
        //complexFunc(__complexFunc1);
        //IF __complexFunc1 = 'hello; THEN ... END_IF
        IF complexFunc() = 'hello' THEN
            // do nothing
        END_IF

        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_else_block() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
        IF FALSE THEN
            // do nothing
        ELSE
            //Should be turned to:
            //alloca __complexFunc1 : STRING;
            //complexFunc(__complexFunc1);
            //a := __complexFunc1;
            a := complexFunc();
        END_IF

        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_elif_condition() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
        IF TRUE THEN
            // do nothing
        ELSIF complexFunc() = 'hello' THEN
            // do nothing
        END_IF

        END_FUNCTION

        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut control_statement_lowerer = ControlStatementLowerer::new(id_provider.clone());
        control_statement_lowerer.visit_compilation_unit(&mut unit);

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn do_not_change_builtin_call() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION main
        VAR a : STRING; END_VAR
            a := SEL('hello', 'world');
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };

        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[0]);
    }

    #[test]
    fn call_statements_in_initializers_not_changed() {
        let id_provider = IdProvider::default();
        let src = r#"
        FUNCTION main
        VAR
            a : STRING;
            b : REF_TO STRING := REF(a);
            c : REFERENCE TO STRING REF=b;
            b : POINTER TO STRING := ADR(a);
        END_VAR
        END_FUNCTION
        "#;

        let (unit, index, ..) = index_and_lower(src, id_provider.clone());
        let (_, _, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        assert_debug_snapshot!(units.0.implementations[0]);
    }

    #[test]
    fn call_statemements_in_global() {
        let id_provider = IdProvider::default();
        let src = r#"
        VAR_GLOBAL
            a : STRING;
            b : REF_TO STRING := REF(a);
            c : REFERENCE TO STRING REF=b;
            b : POINTER TO STRING := ADR(a);
        END_VAR
        "#;

        let (unit, index, ..) = index_and_lower(src, id_provider.clone());
        let (_, _, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        assert_debug_snapshot!(units.0.global_vars);
    }

    #[test]
    fn generic_call_statement() {
        let id_provider = IdProvider::default();
        let src = r#"
        FUNCTION main : STRING
            main := MID('hello');
        END_FUNCTION

        {external}
        FUNCTION MID < T: ANY_STRING >: T
        VAR_INPUT
            IN: T;
        END_VAR
        END_FUNCTION
        "#;

        let (unit, index, ..) = index_and_lower(src, id_provider.clone());
        let (_, index, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        assert_debug_snapshot!(index.find_pou_type("MID__STRING").unwrap());
        assert_debug_snapshot!(units.0.implementations[1]);
    }

    #[test]
    fn generic_call_statement_with_aggregate_return() {
        let id_provider = IdProvider::default();
        let src = r#"
        FUNCTION main : STRING
            main := MID('hello');
        END_FUNCTION

        {external}
        FUNCTION MID < T: ANY_STRING >: STRING
        VAR_INPUT
            IN: T;
        END_VAR
        END_FUNCTION
        "#;

        let (unit, index, ..) = index_and_lower(src, id_provider.clone());
        let (_, index, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        assert_debug_snapshot!(index.find_pou_type("MID__STRING").unwrap());
        assert_debug_snapshot!(units.0.implementations[1]);
    }

    #[test]
    fn nested_complex_calls_in_if_condition() {
        let id_provider = IdProvider::default();
        let src = r#"
            FUNCTION CLEAN : STRING
            VAR_INPUT
                CX : STRING;
            END_VAR
            VAR
                pos: INT := 1;
            END_VAR
                IF FIND(CX, MID(CLEAN, 1, pos)) > 0 THEN
                    pos := pos + 1;
                END_IF;
            END_FUNCTION

            FUNCTION FIND<T: ANY_STRING> : INT
            VAR_INPUT
                needle: T;
                haystack: T;
            END_VAR
            END_FUNCTION

            {external}
            FUNCTION FIND__STRING : INT
            VAR_INPUT
                needle: STRING;
                haystack: STRING;
            END_VAR
            END_FUNCTION

            FUNCTION MID<T: ANY_STRING> : T
            VAR_INPUT
                str: T;
                len: INT;
                start: INT;
            END_VAR
            END_FUNCTION

            {external}
            FUNCTION MID__STRING : STRING
            VAR_INPUT
                str: STRING;
                len: INT;
                start: INT;
            END_VAR
            END_FUNCTION
        "#;

        let (unit, index, ..) = index_and_lower(src, id_provider.clone());
        let (_, _, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        let unit = &units.0;
        assert_debug_snapshot!(unit.implementations[0]);
    }

    #[test]
    fn function_wirh_array_of_string_return() {
        let id_provider = IdProvider::default();
        let (unit, index, ..) = index_and_lower(
            r#"
        FUNCTION foo : ARRAY[0..1] OF STRING
            foo[0] := 'hello';
            foo[1] := 'world';
        END_FUNCTION

        FUNCTION main
            foo();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        assert_debug_snapshot!(index.find_pou_type("foo").unwrap());
        let res_type = index.find_type("__foo_return").unwrap();
        assert_debug_snapshot!(res_type);
        let (_, _, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        let unit = &units.0;
        assert_debug_snapshot!(unit);
    }

    #[test]
    fn function_with_explicit_call_statement_has_explicit_return() {
        let id_provider = IdProvider::default();
        let (unit, index, ..) = index_and_lower(
            r#"
        FUNCTION foo : STRING
        VAR_INPUT
            x : DINT;
        END_VAR
            foo := 'hello';
        END_FUNCTION

        FUNCTION main
            foo(x := 1);
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        assert_debug_snapshot!(index.find_pou_type("foo").unwrap());
        let (_, _, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        let unit = &units.0;
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn fnptr_with_arguments() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
            FUNCTION_BLOCK FbA
                METHOD foo: STRING
                    VAR_INPUT
                        x: DINT;
                        y: STRING;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceFbA: FbA;
                    fooPtr: __FPOINTER FbA.foo;
                    localX: DINT;
                    localY: STRING;
                    result: STRING;
                END_VAR

                result := fooPtr^(instanceFbA, localX, localY);
            END_FUNCTION
            "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[2].statements[0]);
    }

    #[test]
    fn fnptr_without_arguments() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
            FUNCTION_BLOCK FbA
                METHOD foo: STRING
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceFbA: FbA;
                    fooPtr: __FPOINTER FbA.foo;
                    result: STRING;
                END_VAR

                result := fooPtr^(instanceFbA);
            END_FUNCTION
            "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[2].statements[0]);
    }

    #[test]
    fn calls_to_functions_with_output_assignments_are_lowererd_if_type_cast_is_needed() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION libFunction : INT
        VAR_INPUT
            inVar1 : INT;
            inVar2 : REAL;
        END_VAR
        VAR_OUTPUT
            result : REAL;
        END_VAR
        END_FUNCTION

        PROGRAM mainProg
            VAR
                i1 : INT;
            END_VAR

            libFunction(
                inVar1 := 0,
                inVar2 := 0.0,
                result => i1
            );
        END_PROGRAM
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);

        let implementations = &unit.implementations;
        let implementation = implementations
            .iter()
            .find(|i| i.name == "mainProg")
            .expect("mainProg implementation should exist");
        assert_eq!(implementation.name, "mainProg");

        let statement = &implementation.statements[0];
        assert_snapshot!(AstSerializer::format(statement), @"
        alloca __libFunction_result0: REAL;
        libFunction(inVar1 := 0, inVar2 := 0.0, result => __libFunction_result0);
        i1 := __libFunction_result0;
        ");
    }

    #[test]
    fn calls_to_functions_with_output_assignments_are_not_lowererd_if_no_type_cast_is_needed() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION libFunction : INT
        VAR_INPUT
            inVar1 : INT;
            inVar2 : REAL;
        END_VAR
        VAR_OUTPUT
            result : REAL;
        END_VAR
        END_FUNCTION

        PROGRAM mainProg
            VAR
                i1 : REAL;
            END_VAR

            libFunction(
                inVar1 := 0,
                inVar2 := 0.0,
                result => i1
            );
        END_PROGRAM
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);

        let implementations = &unit.implementations;
        let implementation = implementations
            .iter()
            .find(|i| i.name == "mainProg")
            .expect("mainProg implementation should exist");
        assert_eq!(implementation.name, "mainProg");

        let statement = &implementation.statements[0];
        assert_snapshot!(AstSerializer::format(statement), @"libFunction(inVar1 := 0, inVar2 := 0.0, result => i1)");
    }

    #[test]
    fn calls_to_functions_with_sized_string_as_output_correctly_receive_size() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION Config_GetString : DINT
            VAR_INPUT
                Key : STRING[1023];
            END_VAR
            VAR_OUTPUT
                Value : STRING[1023];
            END_VAR
        END_FUNCTION

        PROGRAM mainProg
            VAR
                MyString : STRING;
                MyInt: INT := 1234;
                retValue : DINT;
            END_VAR

            retValue := Config_GetString('/some/config/value', MyString);
        END_PROGRAM
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);

        let implementations = &unit.implementations;
        let implementation = implementations
            .iter()
            .find(|i| i.name == "mainProg")
            .expect("mainProg implementation should exist");
        assert_eq!(implementation.name, "mainProg");

        assert_snapshot!(&AstSerializer::format_nodes(&implementation.statements), @"
        alloca __Config_GetString_Value0: __Config_GetString_Value;
        retValue := Config_GetString('/some/config/value', __Config_GetString_Value0);
        MyString := __Config_GetString_Value0;
        ");
    }

    #[test]
    fn calls_to_functions_with_unsized_string_as_output_should_behave_as_default() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION Config_GetString : DINT
            VAR_INPUT
                Key : STRING;
            END_VAR
            VAR_OUTPUT
                Value : STRING;
            END_VAR
        END_FUNCTION

        PROGRAM mainProg
            VAR
                MyString : STRING;
                MyInt: INT := 1234;
                retValue : DINT;
            END_VAR

            retValue := Config_GetString('/some/config/value', MyString);
        END_PROGRAM
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);

        let implementations = &unit.implementations;
        let implementation = implementations
            .iter()
            .find(|i| i.name == "mainProg")
            .expect("mainProg implementation should exist");
        assert_eq!(implementation.name, "mainProg");

        assert_snapshot!(&AstSerializer::format_nodes(&implementation.statements), @"retValue := Config_GetString('/some/config/value', MyString);");
    }

    #[test]
    fn calls_to_functions_with_no_output_assigned_should_not_be_lowered() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION Config_GetString : DINT
            VAR_INPUT
                Key : STRING;
            END_VAR
            VAR_OUTPUT
                Value : STRING;
            END_VAR
        END_FUNCTION

        PROGRAM mainProg
            retValue := Config_GetString(Key := '/some/config/value', Value =>);
        END_PROGRAM
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);

        let implementations = &unit.implementations;
        let implementation = implementations
            .iter()
            .find(|i| i.name == "mainProg")
            .expect("mainProg implementation should exist");
        assert_eq!(implementation.name, "mainProg");

        assert_snapshot!(&AstSerializer::format_nodes(&implementation.statements), @"retValue := Config_GetString(Key := '/some/config/value', Value => );");
    }

    #[test]
    fn calls_to_functions_where_callee_has_a_large_enough_buffer_should_result_in_no_lowering() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION Config_GetString : DINT
            VAR_INPUT
                Key : STRING[1023];
            END_VAR
            VAR_OUTPUT
                Value : STRING[1023];
            END_VAR
        END_FUNCTION

        PROGRAM mainProg
            VAR
                MyString : STRING[1024];
                MyInt: INT := 1234;
                retValue : DINT;
            END_VAR

            retValue := Config_GetString('/some/config/value', MyString);
        END_PROGRAM
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);

        let implementations = &unit.implementations;
        let implementation = implementations
            .iter()
            .find(|i| i.name == "mainProg")
            .expect("mainProg implementation should exist");
        assert_eq!(implementation.name, "mainProg");

        assert_snapshot!(&AstSerializer::format_nodes(&implementation.statements), @"retValue := Config_GetString('/some/config/value', MyString);");
    }

    #[test]
    fn calls_to_functions_where_callee_has_a_same_size_buffer_should_result_in_no_lowering() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION Config_GetString : DINT
            VAR_INPUT
                Key : STRING[1023];
            END_VAR
            VAR_OUTPUT
                Value : STRING[1023];
            END_VAR
        END_FUNCTION

        PROGRAM mainProg
            VAR
                MyString : STRING[1023];
                MyInt: INT := 1234;
                retValue : DINT;
            END_VAR

            retValue := Config_GetString('/some/config/value', MyString);
        END_PROGRAM
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);

        let implementations = &unit.implementations;
        let implementation = implementations
            .iter()
            .find(|i| i.name == "mainProg")
            .expect("mainProg implementation should exist");
        assert_eq!(implementation.name, "mainProg");

        assert_snapshot!(&AstSerializer::format_nodes(&implementation.statements), @"retValue := Config_GetString('/some/config/value', MyString);");
    }

    #[test]
    fn calls_to_functions_with_aggregate_return_type_should_still_be_lowered() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION Config_GetString : STRING
            VAR_INPUT
                Key : STRING[30];
            END_VAR
            VAR_OUTPUT
                Value : STRING[30];
            END_VAR
        END_FUNCTION

        PROGRAM mainProg
            VAR
                MyString : STRING[10];
                MyInt: INT := 1234;
                retValue : DINT;
            END_VAR

            retValue := Config_GetString('/some/config/value', MyString);
        END_PROGRAM
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);

        let implementations = &unit.implementations;
        let implementation = implementations
            .iter()
            .find(|i| i.name == "mainProg")
            .expect("mainProg implementation should exist");
        assert_eq!(implementation.name, "mainProg");

        assert_snapshot!(&AstSerializer::format_nodes(&implementation.statements), @"
        alloca __Config_GetString0: STRING;
        alloca __Config_GetString_Value1: __Config_GetString_Value;
        Config_GetString(__Config_GetString0, '/some/config/value', __Config_GetString_Value1);
        retValue := __Config_GetString0;
        MyString := __Config_GetString_Value1;
        ");
    }

    #[test]
    fn calls_to_functions_with_string_output_should_lower_the_correct_variable() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION Config_GetString : DINT
            VAR_INPUT
                Key   : STRING[1023];
            END_VAR
            VAR_OUTPUT
                Value : STRING[28];
            END_VAR

            Value := 'hi';
        END_FUNCTION

        PROGRAM mainProg
            VAR
                keyVar : STRING[10];
                outVar : STRING[28];
                r : DINT;
            END_VAR

            keyVar := 'my-key';
            r := Config_GetString(keyVar, outVar);
        END_PROGRAM

        FUNCTION main
            mainProg();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            id_provider: id_provider.clone(),
            ..Default::default()
        };
        lowerer.visit_compilation_unit(&mut unit);
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);

        let implementations = &unit.implementations;
        let implementation = implementations
            .iter()
            .find(|i| i.name == "mainProg")
            .expect("mainProg implementation should exist");
        assert_eq!(implementation.name, "mainProg");

        assert_snapshot!(&AstSerializer::format_nodes(&implementation.statements), @"
        keyVar := 'my-key';
        r := Config_GetString(keyVar, outVar);
        ");
    }
}
