//! Lowering of interface method calls and assignments into fat-pointer based dispatch
//!
//! This module transforms interface-typed variables and method calls into a fat-pointer representation that
//! enables dynamic dispatch at runtime. While the sibling module [`super::pou`] handles dispatch through
//! virtual tables (vtables) for inheritance hierarchies, this module handles the distinct case of *interface
//! dispatch* using interface tables (itables, see [`crate::lowering::polymorphism::table::interface`]).
//!
//! # Fat pointers
//!
//! An interface-typed variable cannot hold a concrete instance directly, because different implementors may
//! have different sizes and layouts. Instead, each interface variable is replaced with a `__FATPOINTER`
//! struct containing two void pointers. For example, assuming an interface `IA`:
//!
//! ```text
//! // Before:
//! VAR
//!     reference : IA;
//! END_VAR
//!
//! // After:
//! VAR
//!     reference : __FATPOINTER;
//! END_VAR
//! ```
//!
//! Where `__FATPOINTER` is defined as:
//!
//! ```text
//! TYPE __FATPOINTER:
//!     STRUCT
//!         data:  POINTER TO __VOID;  // address of the concrete instance
//!         table: POINTER TO __VOID;  // address of the itable for this (interface, POU) pair
//!     END_STRUCT
//! END_TYPE
//! ```
//!
//! # Assignments
//!
//! When a concrete POU instance is assigned to an interface variable, the assignment is expanded into two
//! field assignments that populate the fat pointer. For example, given an `instance` of type `FbA` which
//! implements `IA`:
//!
//! ```text
//! // Before:
//! reference := instance;
//!
//! // After:
//! reference.data  := ADR(instance);
//! reference.table := ADR(__itable_IA_FbA_instance);
//! ```
//!
//! # Interface method calls
//!
//! Calling a method through an interface variable is transformed into an indirect call through the itable.
//! The transformation has four steps:
//!
//! ```text
//! // Before:
//! reference.foo(args)
//!
//! // After:
//! // Step 1: prepend the data pointer as implicit first argument:
//! reference.foo(reference.data^, args)
//!
//! // Step 2: replace the base with a dereferenced table access:
//! reference.table^.foo(reference.data^, args)
//!
//! // Step 3: cast to the concrete itable type:
//! __itable_IA#(reference.table^).foo(reference.data^, args)
//!
//! // Step 4 (final): dereference the function pointer:
//! __itable_IA#(reference.table^).foo^(reference.data^, args)
//! ```
//!
//! # Call arguments
//!
//! When a concrete POU instance is passed as an argument where an interface type is expected, a temporary fat
//! pointer is allocated and populated before the call. The alloca is necessary because the callee expects a
//! `__FATPOINTER` value, but the caller only has the concrete instance — there is no pre-existing fat pointer
//! to pass.
//!
//! ```text
//! // Before:
//! consumer(instance);
//!
//! // After:
//! alloca __fatpointer_0 : __FATPOINTER;
//! __fatpointer_0.data  := ADR(instance);
//! __fatpointer_0.table := ADR(__itable_IA_FbA_instance);
//! consumer(__fatpointer_0);
//! ```
//!
// TODO: Consider switching from `log` to the `tracing` crate for structured, span-based logging. This would
// give us automatic indentation and hierarchical span nesting, making the visitor call flow much easier to
// follow.

use plc_ast::{
    ast::{
        Allocation, AstFactory, AstNode, AstStatement, CallStatement, CompilationUnit, DataTypeDeclaration,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    resolver::{AnnotationMap, AnnotationMapImpl},
    typesystem::DataType,
};

const FATPOINTER_TYPE_NAME: &str = "__FATPOINTER";
const FATPOINTER_DATA_FIELD_NAME: &str = "data";
const FATPOINTER_TABLE_FIELD_NAME: &str = "table";

pub struct InterfaceDispatchLowerer<'a> {
    ids: IdProvider,
    index: &'a Index,
    annotations: &'a AnnotationMapImpl,

    // TODO: This might be obsolote if we decide to make the `__FATPOINTER` struct a builtin
    /// Do we need to generate the `__FATPOINTER` struct definition?
    generate_fatpointer: bool,

    /// Are we in a call statement and if so in how many depths?
    call_depth: usize,

    /// Stack of assignment LHS targets, needed because assignments can nest (e.g. named call
    /// parameters). See `visit_assignment` for details.
    assignment_ctx: Vec<AstNode>,

    /// Replacement statements for the current assignment (filled by `visit_reference_expr`).
    assignment_preamble: Vec<AstNode>,

    /// Preamble statements for the enclosing call (filled by `visit_reference_expr`, drained by
    /// `visit_call_statement`).
    call_preamble: Vec<AstNode>,

    /// Monotonic counter for generating unique alloca names.
    alloca_counter: u32,
}

impl<'a> AstVisitorMut for InterfaceDispatchLowerer<'a> {
    /// Visits each data type declaration, replacing interface types with `__FATPOINTER`.
    fn visit_data_type_declaration(&mut self, data_type_declaration: &mut DataTypeDeclaration) {
        if let DataTypeDeclaration::Reference { referenced_type, .. } = data_type_declaration {
            if self.index.find_effective_type_by_name(referenced_type).is_some_and(DataType::is_interface) {
                helper::replace_datatype_with_fatpointer(data_type_declaration);
                self.generate_fatpointer = true;
            }
        }

        data_type_declaration.walk(self);
    }

    /// Walks into interface method declarations so that their data type declarations are also visited. This
    /// is needed when a method has an interface type as a return type or parameter, which must be replaced
    /// with `__FATPOINTER`.
    fn visit_interface(&mut self, interface: &mut plc_ast::ast::Interface) {
        for method in &mut interface.methods {
            self.visit_pou(method);
        }
    }

    fn visit_assignment(&mut self, node: &mut plc_ast::ast::AstNode) {
        let AstStatement::Assignment(assignment) = &mut node.stmt else { unreachable!() };

        assignment.left.walk(self);

        // Push the LHS onto the assignment context stack so that downstream visitors know
        // the current assignment target. For example, given `result := consumer(in := instance)`:
        //   - Outer visit_assignment pushes `result`  → stack: [result]
        //   - Inner visit_assignment pushes `in`      → stack: [result, in]
        //   - Inner visit_assignment pops `in`        → stack: [result]
        //   - unwrap_call_preamble pops `result`      → stack: []
        // The stack ensures that when a nested assignment cleans up after itself, the outer
        // assignment's context is preserved.
        self.assignment_ctx.push((*assignment.left).clone());
        assignment.right.walk(self);

        // Imagine `reference := instance`, which needs to be lowered to `reference.data := ADR(...)` and
        // `reference.table := ADR(...)` In that case the `assignment_preamble` will contain these two
        // statements, which need to be replaced by the current `reference := instance` statement. Since we
        // are traversing the tree, we are unable to just expand its statements. Thus we do something "hacky"
        // (but quite elegant tbh), namely we replace the original node with an expression list containing >1
        // nodes.
        if !self.assignment_preamble.is_empty() {
            node.stmt = AstStatement::ExpressionList(std::mem::take(&mut self.assignment_preamble));
        }

        // Pop our LHS off the stack, restoring the outer assignment's context (if any).
        self.assignment_ctx.pop();
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let AstStatement::CallStatement(call) = &mut node.stmt else { unreachable!() };

        self.call_depth += 1;
        call.walk(self);
        self.call_depth -= 1;

        let interface_name = node
            .get_call_operator()
            .and_then(|op| op.get_base_ref_expr())
            .and_then(|base| self.annotations.get_type(base, self.index))
            .filter(|ty| ty.is_interface())
            .map(|ty| ty.get_name().to_string());

        // Some interface method call, e.g. `reference.foo()`, which needs to be transformed into
        //  `__itable_IA#(reference.table^).foo^(reference.data^)`
        if let Some(interface_name) = interface_name {
            let AstStatement::CallStatement(CallStatement { operator, parameters }) = &mut node.stmt else {
                unreachable!()
            };

            self.patch_data_argument(operator, parameters);
            self.patch_table_access(operator);
            self.patch_itable_cast(operator, &interface_name);
            self.patch_method_call_deref(operator);
        }

        // Unwrap any call preambles, e.g. `consumer(instance)` will have produced a preamble of an alloca and
        // two assignments
        self.unwrap_call_preamble(node);
    }

    fn visit_reference_expr(&mut self, node: &mut plc_ast::ast::AstNode) {
        let Some(ty) = self.annotations.get_type(node, self.index) else { return };
        let Some(ty_hint) = self.annotations.get_type_hint(node, self.index) else { return };

        // TODO: We need to be able to handle `referenceA := referenceB`
        if ty.is_interface() || !ty_hint.is_interface() {
            return;
        }

        let interface_name = ty_hint.get_name();
        let pou_name = ty.get_name();

        if self.call_depth > 0 {
            // Something like `consumer(instance)` which needs a temporary fat pointer because the call
            // expects a `__FATPOINTER` but we only have an instance So `consumer(instance)` is replaced by
            // `consumer(__fatpointer_0)` and the call preamble contains: `[alloca __fatpointer_0 :
            // __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table :=
            // ADR(__itable_..._instance)]`.
            //
            // Same deal with `consumer(in := instance)` which will be transofrmed into
            // `consumer(in := __fatpointer_0)`
            self.create_fat_pointer_alloca_assignment_and_replace_node(node, interface_name, pou_name);
        } else if let Some(left) = self.assignment_ctx.last() {
            // Something like `reference := instance` which needs to be lowered to `reference.data :=
            // ADR(...)` and `reference.table := ADR(...)`
            let left = left.clone();
            self.create_fat_pointer_assignment(&left, node, interface_name, pou_name);
        }
    }
}

impl<'a> InterfaceDispatchLowerer<'a> {
    pub fn new(ids: IdProvider, index: &'a Index, annotations: &'a AnnotationMapImpl) -> Self {
        Self {
            ids,
            index,
            annotations,
            generate_fatpointer: false,
            call_depth: 0,
            assignment_ctx: Vec::new(),
            assignment_preamble: Vec::new(),
            call_preamble: Vec::new(),
            alloca_counter: 0,
        }
    }

    pub fn lower(&mut self, units: &mut [CompilationUnit]) {
        for unit in &mut *units {
            self.visit_compilation_unit(unit);
        }

        if self.generate_fatpointer {
            units[0].user_types.push(helper::create_fat_pointer_struct());
        }
    }

    fn create_fat_pointer_alloca_assignment_and_replace_node(
        &mut self,
        node: &mut AstNode,
        interface_name: &str,
        pou_name: &str,
    ) {
        let tmp_name = self.next_fatpointer_alloca_name();

        // Reference node for the temporary fat pointer
        let tmp_ref = AstFactory::create_member_reference(
            AstFactory::create_identifier(&tmp_name, SourceLocation::internal(), self.ids.next_id()),
            None,
            self.ids.next_id(),
        );

        // alloca __fatpointer_N : __FATPOINTER
        let alloca = AstNode {
            stmt: AstStatement::AllocationStatement(Allocation {
                name: tmp_name.clone(),
                reference_type: FATPOINTER_TYPE_NAME.to_string(),
            }),
            id: self.ids.next_id(),
            location: SourceLocation::internal(),
            metadata: None,
        };
        self.call_preamble.push(alloca);

        // __fatpointer_N.data := ADR(node)
        let assign_data = helper::create_fat_pointer_field_assignment(
            &mut self.ids,
            &tmp_ref,
            FATPOINTER_DATA_FIELD_NAME,
            node,
        );
        self.call_preamble.push(assign_data);

        // __fatpointer_N.table := ADR(__itable_<interface>_<pou>_instance)
        let itable_name = format!("__itable_{interface_name}_{pou_name}_instance");
        let itable_ref = AstFactory::create_member_reference(
            AstFactory::create_identifier(itable_name, SourceLocation::internal(), self.ids.next_id()),
            None,
            self.ids.next_id(),
        );
        let assign_table = helper::create_fat_pointer_field_assignment(
            &mut self.ids,
            &tmp_ref,
            FATPOINTER_TABLE_FIELD_NAME,
            &itable_ref,
        );
        self.call_preamble.push(assign_table);

        // Replace the argument with a reference to __fatpointer_N
        *node = AstFactory::create_member_reference(
            AstFactory::create_identifier(&tmp_name, SourceLocation::internal(), self.ids.next_id()),
            None,
            self.ids.next_id(),
        );
    }

    fn create_fat_pointer_assignment(
        &mut self,
        left: &AstNode,
        node: &AstNode,
        interface_name: &str,
        pou_name: &str,
    ) {
        // left.data := ADR(right)
        let assign_data = helper::create_fat_pointer_field_assignment(
            &mut self.ids,
            left,
            FATPOINTER_DATA_FIELD_NAME,
            node,
        );

        // left.table := ADR(__itable_<interface>_<pou>_instance)
        let itable_name = format!("__itable_{interface_name}_{pou_name}_instance");
        let itable_ref = AstFactory::create_member_reference(
            AstFactory::create_identifier(itable_name, SourceLocation::internal(), self.ids.next_id()),
            None,
            self.ids.next_id(),
        );
        let assign_table = helper::create_fat_pointer_field_assignment(
            &mut self.ids,
            left,
            FATPOINTER_TABLE_FIELD_NAME,
            &itable_ref,
        );

        self.assignment_preamble = vec![assign_data, assign_table];
    }

    fn patch_data_argument(&mut self, operator: &mut AstNode, parameters: &mut Option<Box<AstNode>>) {
        let base = operator.get_base_ref_expr().expect("interface call must have a base");
        let data_member = AstFactory::create_member_reference(
            AstFactory::create_identifier(
                FATPOINTER_DATA_FIELD_NAME,
                SourceLocation::internal(),
                self.ids.next_id(),
            ),
            Some(base.clone()),
            self.ids.next_id(),
        );
        let data_ref =
            AstFactory::create_deref_reference(data_member, self.ids.next_id(), SourceLocation::internal());

        match parameters {
            None => {
                parameters.replace(Box::new(data_ref));
            }

            Some(ref mut expr) => match &mut expr.stmt {
                AstStatement::ExpressionList(expressions) => {
                    expressions.insert(0, data_ref);
                }

                _ => {
                    let mut expressions = Box::new(AstFactory::create_expression_list(
                        vec![data_ref, std::mem::take(expr)],
                        SourceLocation::internal(),
                        self.ids.next_id(),
                    ));

                    std::mem::swap(expr, &mut expressions);
                }
            },
        }
    }

    fn patch_table_access(&mut self, operator: &mut AstNode) {
        let old_base = operator.get_base_ref_expr_mut().expect("interface call must have a base");
        let mut new_base = AstFactory::create_deref_reference(
            AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    FATPOINTER_TABLE_FIELD_NAME,
                    SourceLocation::internal(),
                    self.ids.next_id(),
                ),
                Some(std::mem::take(old_base)),
                self.ids.next_id(),
            ),
            self.ids.next_id(),
            SourceLocation::internal(),
        );

        std::mem::swap(old_base, &mut new_base);
    }

    fn patch_itable_cast(&mut self, operator: &mut AstNode, interface_name: &str) {
        let old_base = operator.get_base_ref_expr_mut().expect("interface call must have a base");
        let old_base_paren = AstFactory::create_paren_expression(
            std::mem::take(old_base),
            SourceLocation::internal(),
            self.ids.next_id(),
        );

        let mut new_base = AstFactory::create_cast_statement(
            AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    format!("__itable_{interface_name}"),
                    SourceLocation::internal(),
                    self.ids.next_id(),
                ),
                None,
                self.ids.next_id(),
            ),
            old_base_paren,
            &SourceLocation::internal(),
            self.ids.next_id(),
        );

        std::mem::swap(old_base, &mut new_base);
    }

    fn patch_method_call_deref(&mut self, operator: &mut AstNode) {
        let mut deref = AstFactory::create_deref_reference(
            std::mem::take(operator),
            self.ids.next_id(),
            SourceLocation::internal(),
        );

        std::mem::swap(operator, &mut deref);
    }

    fn unwrap_call_preamble(&mut self, node: &mut AstNode) {
        // We only want to unwrap the preamble at the top-level call, everything else would be invalid. TODO:
        // Give an example why it would be invalid
        if self.call_depth != 0 || self.call_preamble.is_empty() {
            return;
        }

        let mut statements = std::mem::take(&mut self.call_preamble);
        let original_call = std::mem::replace(
            node,
            AstFactory::create_empty_statement(SourceLocation::internal(), self.ids.next_id()),
        );

        if let Some(left) = self.assignment_ctx.pop() {
            // Something like `result := consumer(instance)`, in which case we need to ensure the
            // assignment statement among the preamble is the last statement. For that example the
            // preamble will have the form:
            // ```
            // alloca __fatpointer_0: __FATPOINTER,
            // __fatpointer_0.data := ADR(instance),
            // __fatpointer_0.table := ADR(__itable_..._instance),
            // ```
            // The alloca needs to happen before the call, so we push the assignment into the
            // preamble such that it becomes:
            // ```
            // alloca __fatpointer_0: __FATPOINTER,
            // __fatpointer_0.data := ADR(instance),
            // __fatpointer_0.table := ADR(__itable_..._instance),
            // result := consumer(__fatpointer_0)
            // ```
            // We can't replace the node directly here because `visit_assignment` owns the
            // assignment node. Instead we route through `assignment_preamble`, which
            // `visit_assignment` picks up and uses to replace the node.
            statements.push(AstFactory::create_assignment(left, original_call, self.ids.next_id()));
            self.assignment_preamble = statements;
        } else {
            // Bare call without an enclosing assignment, e.g. `consumer(instance)`. The preamble
            // will have the form:
            // ```
            // alloca __fatpointer_0: __FATPOINTER,
            // __fatpointer_0.data := ADR(instance),
            // __fatpointer_0.table := ADR(__itable_..._instance),
            // ```
            // We append the original call and replace the node directly with an ExpressionList:
            // ```
            // alloca __fatpointer_0: __FATPOINTER,
            // __fatpointer_0.data := ADR(instance),
            // __fatpointer_0.table := ADR(__itable_..._instance),
            // consumer(__fatpointer_0)
            // ```
            // Unlike the assignment case above, there is no enclosing visitor waiting to do the
            // replacement, so we do it here.
            statements.push(original_call);
            node.stmt = AstStatement::ExpressionList(statements);
        }
    }

    fn next_fatpointer_alloca_name(&mut self) -> String {
        let n = self.alloca_counter;
        self.alloca_counter += 1;
        format!("__fatpointer_{n}")
    }
}

mod helper {
    use plc_ast::{
        ast::{
            AstFactory, AstNode, DataType, DataTypeDeclaration, LinkageType, UserTypeDeclaration, Variable,
        },
        provider::IdProvider,
    };
    use plc_source::source_location::SourceLocation;

    use crate::{
        lowering::polymorphism::dispatch::interface::{
            FATPOINTER_DATA_FIELD_NAME, FATPOINTER_TABLE_FIELD_NAME, FATPOINTER_TYPE_NAME,
        },
        typesystem::VOID_INTERNAL_NAME,
    };

    pub fn replace_datatype_with_fatpointer(type_decl: &mut DataTypeDeclaration) {
        *type_decl = DataTypeDeclaration::Reference {
            referenced_type: String::from(FATPOINTER_TYPE_NAME),
            location: type_decl.get_location(),
        };
    }

    pub fn create_fat_pointer_struct() -> UserTypeDeclaration {
        /// Creates a struct member of type `POINTER TO __VOID`.
        fn create_void_pointer_member(name: &str, location: &SourceLocation) -> Variable {
            Variable {
                name: name.to_string(),
                data_type_declaration: DataTypeDeclaration::Definition {
                    data_type: Box::new(DataType::PointerType {
                        name: None,
                        referenced_type: Box::new(DataTypeDeclaration::Reference {
                            referenced_type: VOID_INTERNAL_NAME.to_string(),
                            location: location.clone(),
                        }),
                        auto_deref: None,
                        type_safe: false,
                        is_function: false,
                    }),
                    location: location.clone(),
                    scope: None,
                },
                initializer: None,
                address: None,
                location: location.clone(),
            }
        }

        let location = SourceLocation::internal();

        UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some(String::from(FATPOINTER_TYPE_NAME)),
                variables: vec![
                    create_void_pointer_member(FATPOINTER_DATA_FIELD_NAME, &location),
                    create_void_pointer_member(FATPOINTER_TABLE_FIELD_NAME, &location),
                ],
            },
            initializer: None,
            location,
            scope: None,
            linkage: LinkageType::Internal,
        }
    }

    /// Creates `<base>.<field> := ADR(<target>)`.
    pub fn create_fat_pointer_field_assignment(
        ids: &mut IdProvider,
        base: &AstNode,
        field: &str,
        target: &AstNode,
    ) -> AstNode {
        let location = SourceLocation::internal();

        // LHS: <base>.<field>
        let lhs = AstFactory::create_member_reference(
            AstFactory::create_identifier(field, &location, ids.next_id()),
            Some(base.clone()),
            ids.next_id(),
        );

        // RHS: ADR(<target>)
        let rhs = AstFactory::create_call_statement(
            AstFactory::create_member_reference(
                AstFactory::create_identifier("ADR", &location, ids.next_id()),
                None,
                ids.next_id(),
            ),
            Some(target.clone()),
            ids.next_id(),
            &location,
        );

        AstFactory::create_assignment(lhs, rhs, ids.next_id())
    }
}

#[cfg(test)]
mod tests {
    use crate::lowering::polymorphism::dispatch::interface::{
        tests::helper::lower_and_serialize_statements, FATPOINTER_TYPE_NAME,
    };

    #[test]
    fn fatpointer_is_generated_on_demand() {
        // Initially, no POU makes use of a interface as a variable
        {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA
                END_FUNCTION_BLOCK
            "#;

            let annotated_unit = helper::lower(source);
            let mut user_types = annotated_unit.get_unit().user_types.iter();
            assert!(!user_types.any(|ty| ty.data_type.get_name().unwrap() == FATPOINTER_TYPE_NAME));
        }

        // However, if the interface is used as a variable, a fat-pointer MUST be generated
        {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA
                    VAR
                        localVariable: IA;
                    END_VAR
                END_FUNCTION_BLOCK
            "#;

            let annotated_unit = helper::lower(source);
            let mut user_types = annotated_unit.get_unit().user_types.iter();
            assert!(user_types.any(|ty| ty.data_type.get_name().unwrap() == FATPOINTER_TYPE_NAME));
        }
    }

    #[test]
    fn fatpointer_replaces_function_return_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION foo: IA
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous, @r#"
        [
            POU {
                name: "foo",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "foo",
                                data_type: DataTypeReference {
                                    referenced_type: "__FATPOINTER",
                                },
                            },
                        ],
                        variable_block_type: InOut,
                    },
                ],
                pou_type: Function,
                return_type: Some(
                    Aggregate {
                        referenced_type: "__FATPOINTER",
                    },
                ),
                interfaces: [],
                properties: [],
            },
        ]
        "#);
    }

    #[test]
    fn fatpointer_replaces_interface_variable_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION main
                VAR
                    localVariable: IA;
                END_VAR

                VAR_INPUT
                    inVariable: IA;
                END_VAR

                VAR_OUTPUT
                    outVariable: IA;
                END_VAR

                VAR_IN_OUT
                    inOutVariable: IA;
                END_VAR
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous[0].variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Local,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Input(
                    ByVal,
                ),
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "outVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Output,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inOutVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: InOut,
            },
        ]
        "#);
    }

    #[test]
    fn fatpointer_replaces_interface_variable_aliased_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            TYPE
                AliasedIA: IA;
            END_TYPE

            FUNCTION main
                VAR
                    localVariable: AliasedIA;
                END_VAR

                VAR_INPUT
                    inVariable: AliasedIA;
                END_VAR

                VAR_OUTPUT
                    outVariable: AliasedIA;
                END_VAR

                VAR_IN_OUT
                    inOutVariable: AliasedIA;
                END_VAR
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous[0].variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Local,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Input(
                    ByVal,
                ),
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "outVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Output,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inOutVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: InOut,
            },
        ]
        "#);
    }

    #[test]
    fn fatpointer_replaces_interface_variable_array_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION main
                VAR
                    localVariable: ARRAY[1..2] OF IA;
                    localVariableNested: ARRAY[1..2] OF ARRAY[3..4] OF IA;
                    localVariableNestedNested: ARRAY[1..2] OF ARRAY[3..4] OF ARRAY[5..6] OF IA;
                END_VAR
            END_FUNCTION
        "#;

        // TODO: snapshot needs to resolve to inner-most type, I want to see __FATPOINTER here Put
        // differently, the replacement of these datatypes works, but the snapshot doesn't reflect that
        // because it does currently not resolve these internal `__main_...` types.
        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous[0].variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__main_localVariable",
                        },
                    },
                    Variable {
                        name: "localVariableNested",
                        data_type: DataTypeReference {
                            referenced_type: "__main_localVariableNested",
                        },
                    },
                    Variable {
                        name: "localVariableNestedNested",
                        data_type: DataTypeReference {
                            referenced_type: "__main_localVariableNestedNested",
                        },
                    },
                ],
                variable_block_type: Local,
            },
        ]
        "#);
    }

    #[test]
    fn assignments_expand() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                VAR
                    localInstance: FbA;
                    localReference: IA;
                END_VAR

                VAR_INPUT
                    in: IA;
                    instance: FbA;
                END_VAR

                in := instance;
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR

                reference := instance;
                instance.localReference := instance.localInstance;
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main", "FbA"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "instance.localReference.data := ADR(instance.localInstance), instance.localReference.table := ADR(__itable_IA_FbA_instance)",
            "// Statements in FbA",
            "in.data := ADR(instance), in.table := ADR(__itable_IA_FbA_instance)",
        ]
        "#)
    }

    #[test]
    fn array_assignments_expand() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceA: FbA;
                    instanceB: FbB;
                    references: ARRAY[1..2] OF IA;
                END_VAR

                references[1] := instanceA;
                references[2] := instanceB;
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instanceA)",
            "__init_fbb(instanceB)",
            "__user_init_FbA(instanceA)",
            "__user_init_FbB(instanceB)",
            "references[1].data := ADR(instanceA), references[1].table := ADR(__itable_IA_FbA_instance)",
            "references[2].data := ADR(instanceB), references[2].table := ADR(__itable_IA_FbB_instance)",
        ]
        "#)
    }

    #[test]
    fn call_argument_is_expanded() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance: FbA;
                END_VAR

                consumer(instance);
                consumer(in := instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), consumer(__fatpointer_0)",
            "alloca __fatpointer_1: __FATPOINTER, __fatpointer_1.data := ADR(instance), __fatpointer_1.table := ADR(__itable_IA_FbA_instance), consumer(in := __fatpointer_1)",
        ]
        "#);
    }

    #[test]
    fn call_arguments_are_expanded() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    inOne, inTwo, inThree: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instanceA: FbA;
                    instanceB: FbB;
                    instanceC: FbC;
                END_VAR

                consumer(instanceA, instanceB, instanceC);
                consumer(inOne := instanceA, inTwo := instanceB, inThree := instanceC);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instanceA)",
            "__init_fbb(instanceB)",
            "__init_fbc(instanceC)",
            "__user_init_FbA(instanceA)",
            "__user_init_FbB(instanceB)",
            "__user_init_FbC(instanceC)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instanceA), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), alloca __fatpointer_1: __FATPOINTER, __fatpointer_1.data := ADR(instanceB), __fatpointer_1.table := ADR(__itable_IA_FbB_instance), alloca __fatpointer_2: __FATPOINTER, __fatpointer_2.data := ADR(instanceC), __fatpointer_2.table := ADR(__itable_IA_FbC_instance), consumer(__fatpointer_0, __fatpointer_1, __fatpointer_2)",
            "alloca __fatpointer_3: __FATPOINTER, __fatpointer_3.data := ADR(instanceA), __fatpointer_3.table := ADR(__itable_IA_FbA_instance), alloca __fatpointer_4: __FATPOINTER, __fatpointer_4.data := ADR(instanceB), __fatpointer_4.table := ADR(__itable_IA_FbB_instance), alloca __fatpointer_5: __FATPOINTER, __fatpointer_5.data := ADR(instanceC), __fatpointer_5.table := ADR(__itable_IA_FbC_instance), consumer(inOne := __fatpointer_3, inTwo := __fatpointer_4, inThree := __fatpointer_5)",
        ]
        "#);
    }

    #[test]
    fn nesting_single_depth_wrapping() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION producer : DINT
                VAR_INPUT
                    ref: IA;
                END_VAR
            END_FUNCTION

            FUNCTION consumer
                VAR_INPUT
                    value: DINT;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance: FbA;
                END_VAR

                consumer(producer(instance));
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), consumer(producer(__fatpointer_0))",
        ]
        "#);
    }

    #[test]
    fn nesting_multi_depth_wrapping_with_mixed_args() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION inner : DINT
                VAR_INPUT
                    ref: IA;
                END_VAR
            END_FUNCTION

            FUNCTION middle : DINT
                VAR_INPUT
                    a: DINT;
                    b: IA;
                END_VAR
            END_FUNCTION

            FUNCTION outer
                VAR_INPUT
                    a: DINT;
                    b: IA;
                    c: DINT;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    x: FbA;
                    y: FbA;
                    z: FbA;
                END_VAR

                outer(middle(inner(x), y), z, 42);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(x)",
            "__init_fba(y)",
            "__init_fba(z)",
            "__user_init_FbA(x)",
            "__user_init_FbA(y)",
            "__user_init_FbA(z)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(x), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), alloca __fatpointer_1: __FATPOINTER, __fatpointer_1.data := ADR(y), __fatpointer_1.table := ADR(__itable_IA_FbA_instance), alloca __fatpointer_2: __FATPOINTER, __fatpointer_2.data := ADR(z), __fatpointer_2.table := ADR(__itable_IA_FbA_instance), outer(middle(inner(__fatpointer_0), __fatpointer_1), __fatpointer_2, 42)",
        ]
        "#);
    }

    #[test]
    fn interface_method_call_simple() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR

                reference := instance;
                reference.foo();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "__itable_IA#(reference.table^).foo^(reference.data^)",
        ]
        "#);
    }

    #[test]
    fn interface_method_call_with_arguments() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                    VAR_INPUT
                        a: DINT;
                        b: DINT;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                    VAR_INPUT
                        a: DINT;
                        b: DINT;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR

                reference := instance;
                reference.foo(1, 2);
                reference.foo(a := 10, b := 20);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "__itable_IA#(reference.table^).foo^(reference.data^, 1, 2)",
            "__itable_IA#(reference.table^).foo^(reference.data^, a := 10, b := 20)",
        ]
        "#);
    }

    #[test]
    fn interface_method_call_nested() {
        let source = r#"
            INTERFACE IA
                METHOD foo : DINT
                    VAR_INPUT
                        x: DINT;
                    END_VAR
                END_METHOD

                METHOD bar : DINT
                END_METHOD

                METHOD baz
                    VAR_INPUT
                        a: DINT;
                        b: DINT;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : DINT
                    VAR_INPUT
                        x: DINT;
                    END_VAR
                END_METHOD

                METHOD bar : DINT
                END_METHOD

                METHOD baz
                    VAR_INPUT
                        a: DINT;
                        b: DINT;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR

                reference := instance;
                reference.baz(reference.foo(reference.bar()), 42);
                reference.baz(a := reference.foo(x := reference.bar()), b := 42);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "__itable_IA#(reference.table^).baz^(reference.data^, __itable_IA#(reference.table^).foo^(reference.data^, __itable_IA#(reference.table^).bar^(reference.data^)), 42)",
            "__itable_IA#(reference.table^).baz^(reference.data^, a := __itable_IA#(reference.table^).foo^(reference.data^, x := __itable_IA#(reference.table^).bar^(reference.data^)), b := 42)",
        ]
        "#);
    }

    #[test]
    fn interface_method_call_with_aggregate_return() {
        let source = r#"
            INTERFACE IA
                METHOD foo : STRING
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : STRING
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                    result: STRING;
                END_VAR

                reference := instance;
                result := reference.foo();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "alloca __0: STRING, __itable_IA#(reference.table^).foo^(reference.data^, __0), result := __0",
        ]
        "#);
    }

    #[test]
    fn interface_method_call_with_aggregate_return_and_interface_argument() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer : STRING
                VAR_INPUT
                    reference: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance: FbA;
                    result: STRING;
                END_VAR

                result := consumer(instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), alloca __consumer0: STRING, consumer(__consumer0, __fatpointer_0), result := __consumer0",
        ]
        "#);
    }

    #[test]
    fn interface_dispatch_with_aggregate_return_and_interface_argument() {
        let source = r#"
            INTERFACE IA
                METHOD foo : STRING
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : STRING
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                    result: STRING;
                END_VAR

                reference := instance;
                result := reference.foo(instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), alloca __0: STRING, __itable_IA#(reference.table^).foo^(reference.data^, __0, __fatpointer_0), result := __0",
        ]
        "#);
    }

    #[test]
    fn interface_dispatch_aggregate_return_mixed_args_unnamed_and_named() {
        let source = r#"
            INTERFACE IA
                METHOD foo : STRING
                    VAR_INPUT
                        x: DINT;
                        a: IA;
                        y: REAL;
                        b: IB;
                    END_VAR
                END_METHOD
            END_INTERFACE

            INTERFACE IB
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : STRING
                    VAR_INPUT
                        x: DINT;
                        a: IA;
                        y: REAL;
                        b: IB;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB IMPLEMENTS IB
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    inst_a: FbA;
                    inst_b: FbB;
                    reference: IA;
                    result: STRING;
                END_VAR

                reference := inst_a;
                // Unnamed (positional) arguments
                result := reference.foo(42, inst_a, 3.14, inst_b);
                // Named arguments in completely different order
                result := reference.foo(b := inst_b, y := 3.14, a := inst_a, x := 42);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(inst_a)",
            "__init_fbb(inst_b)",
            "__user_init_FbA(inst_a)",
            "__user_init_FbB(inst_b)",
            "reference.data := ADR(inst_a), reference.table := ADR(__itable_IA_FbA_instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(inst_a), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), alloca __fatpointer_1: __FATPOINTER, __fatpointer_1.data := ADR(inst_b), __fatpointer_1.table := ADR(__itable_IB_FbB_instance), alloca __0: STRING, __itable_IA#(reference.table^).foo^(reference.data^, __0, 42, __fatpointer_0, 3.14, __fatpointer_1), result := __0",
            "alloca __fatpointer_2: __FATPOINTER, __fatpointer_2.data := ADR(inst_b), __fatpointer_2.table := ADR(__itable_IB_FbB_instance), alloca __fatpointer_3: __FATPOINTER, __fatpointer_3.data := ADR(inst_a), __fatpointer_3.table := ADR(__itable_IA_FbA_instance), alloca __1: STRING, __itable_IA#(reference.table^).foo^(reference.data^, foo := __1, b := __fatpointer_2, y := 3.14, a := __fatpointer_3, x := 42), result := __1",
        ]
        "#);
    }

    #[test]
    #[ignore = "TODO: Parse is unable to call result of another function"]
    fn nesting_chained_method_calls_with_wrapping() {
        let source = r#"
            INTERFACE IA
                METHOD transform : IA
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD transform : IA
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION producer : IA
                VAR_INPUT
                    ref: IA;
                END_VAR
            END_FUNCTION

            FUNCTION consumer
                VAR_INPUT
                    a: IA;
                    b: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    x: FbA;
                    y: FbA;
                    z: FbA;
                END_VAR

                consumer(a := producer(x).transform(producer(y)), b := z);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#""#);
    }

    #[test]
    fn assignment_on_call_with_interface_argument() {
        let source = r#"
            INTERFACE IA
                METHOD foo : DINT
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : DINT
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                    result: DINT;
                END_VAR

                reference := instance;
                result := reference.foo(instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), result := __itable_IA#(reference.table^).foo^(reference.data^, __fatpointer_0)",
        ]
        "#);
    }

    #[test]
    fn assignment_on_call_with_named_interface_argument() {
        let source = r#"
            INTERFACE IA
                METHOD foo : DINT
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : DINT
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                    result: DINT;
                END_VAR

                reference := instance;
                result := reference.foo(ref := instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), result := __itable_IA#(reference.table^).foo^(reference.data^, ref := __fatpointer_0)",
        ]
        "#);
    }

    mod helper {
        use driver::{parse_and_annotate, pipelines::AnnotatedProject, pipelines::AnnotatedUnit};
        use plc_source::SourceCode;

        pub fn lower(source: impl Into<SourceCode>) -> AnnotatedUnit {
            let (_, mut project): (_, AnnotatedProject) =
                parse_and_annotate("unit-test", vec![source.into()]).unwrap();

            // (project.index, project.units.remove(0))
            project.units.remove(0)
        }

        pub fn lower_and_serialize_statements(source: impl Into<SourceCode>, pous: &[&str]) -> Vec<String> {
            let (_, project) = parse_and_annotate("unit-test", vec![source.into()]).unwrap();
            let unit = project.units[0].get_unit();

            let mut result = Vec::new();
            for pou in pous {
                result.push(format!("// Statements in {pou}"));
                let statements = &unit.implementations.iter().find(|it| &it.name == pou).unwrap().statements;
                let statements_str =
                    statements.iter().map(|statement| statement.as_string()).collect::<Vec<_>>();

                result.extend(statements_str);
            }

            result
        }
    }
}
