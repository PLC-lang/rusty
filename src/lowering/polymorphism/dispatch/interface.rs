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
//! `__FATPOINTER` value, but the caller only has the concrete instance; there is no pre-existing fat pointer
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
//! # Interface upcasting
//!
//! When assigning a child interface to a parent interface variable (e.g. `refIA := refIB` where `IB EXTENDS
//! IA`), the `.data` pointer can be copied directly but the `.table` pointer must change — it points to an
//! `__itable_IB_*` but needs to point to the corresponding `__itable_IA_*` for the same POU. Since the
//! concrete POU is only known at runtime, each itable struct embeds `__upcast_<ancestor>` pointer fields
//! (one per ancestor interface, see [`crate::lowering::polymorphism::table::interface`]) that point directly
//! to the correct ancestor itable instance. This gives O(1) upcast resolution via a single field read:
//!
//! ```text
//! // Before:
//! refIA := refIB;
//!
//! // After:
//! refIA.data  := refIB.data;
//! refIA.table := __itable_IB#(refIB.table^).__upcast_IA;
//! ```
//!
//! The same transformation applies when passing a child interface as a call argument where a parent interface
//! is expected — a temporary fat pointer is allocated with the upcasted table.
//!
// TODO: Consider switching from `log` to the `tracing` crate for structured, span-based logging. This would
// give us automatic indentation and hierarchical span nesting, making the visitor call flow much easier to
// follow.

use plc_ast::{
    ast::{
        resolve_argument_slots, steal_expression_list, Allocation, Assignment, AstFactory, AstNode,
        AstStatement, BinaryExpression, CallStatement, CompilationUnit, DataType as AstDataType,
        DataTypeDeclaration, EmptyStatement, GenericBinding, LinkageType, Operator, ReferenceAccess,
        ReferenceExpr, UserTypeDeclaration, Variable, VariableBlock, VariableBlockType,
    },
    control_statements::{AstControlStatement, ConditionalBlock, IfStatement},
    literals::AstLiteral,
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    index::{Index, VariableIndexEntry},
    resolver::{AnnotationMap, AnnotationMapImpl, StatementAnnotation},
    typesystem::{DataType, DataTypeInformation, VOID_INTERNAL_NAME},
};

use super::validation;

const FATPOINTER_TYPE_NAME: &str = "__FATPOINTER";
const FATPOINTER_DATA_FIELD_NAME: &str = "data";
const FATPOINTER_TABLE_FIELD_NAME: &str = "table";

pub struct InterfaceDispatchLowerer<'a> {
    ids: IdProvider,
    index: &'a Index,
    annotations: &'a AnnotationMapImpl,

    /// Set to `true` when any interface-typed variable is encountered. Triggers injection of
    /// the `__FATPOINTER` struct definition into the first compilation unit.
    needs_fatpointer: bool,

    /// Tracks call-argument nesting depth, if any.
    in_call_args: usize,

    /// Diagnostics collected during traversal. Validation checks that must run during lowering
    /// (before interface types are rewritten) push errors here instead of into the normal
    /// validation pipeline.
    diagnostics: Vec<Diagnostic>,

    /// Statements to insert *before* the current statement in the drain loop. Cleared at the
    /// start of each iteration. Used when a call argument needs wrapping: the temporary fat
    /// pointer must be allocated and populated before the call itself:
    ///
    /// ```text
    /// // Source:
    /// consumer(instance);
    ///
    /// // Preamble (3 nodes):
    /// alloca __fatpointer_0 : __FATPOINTER;
    /// __fatpointer_0.data  := ADR(instance);
    /// __fatpointer_0.table := ADR(__itable_IA_FbA_instance);
    ///
    /// // Original statement (argument replaced in-place):
    /// consumer(__fatpointer_0);
    /// ```
    preamble: Vec<AstNode>,

    /// When set, the original statement is dropped and these nodes are emitted instead.
    /// Used for assignment expansion where one statement becomes two:
    ///
    /// ```text
    /// // Source (1 statement):
    /// reference := instance;
    ///
    /// // Replacement (2 statements):
    /// reference.data  := ADR(instance);
    /// reference.table := ADR(__itable_IA_FbA_instance);
    /// ```
    replacement: Option<Vec<AstNode>>,

    /// Names of the fat pointers behind `VAR_IN_OUT`, `VAR_INPUT {ref}` and `REFERENCE TO` parameters.
    /// A stateful POU keeps such a parameter between calls and any POU may save or forward it, so they
    /// outlive the calling function and are emitted as globals of the unit being lowered instead of
    /// statement-scoped allocations. Each is
    /// named after the implementation that binds it and numbered by `retained_counters`, so separately
    /// compiled units define the same symbol only if they define the same POU.
    globals: Vec<String>,

    /// Names of the retained fat pointers bound inside an implementation with external linkage or a
    /// generic template. Codegen omits such a body, so the unit only declares these globals: the object
    /// that defines the implementation defines them, and a monomorph of a template, which is declared
    /// external by the generic lowering as well, has no defining object.
    external_globals: Vec<String>,

    /// Whether codegen omits the body of the implementation being visited.
    current_pou_external: bool,

    /// Whether the implementation being visited is a generic template. Overloaded templates share one
    /// declaration for member lookup, so a `VAR_TEMP` added to one of them is not found from the
    /// other; a short-circuit in a template keeps the eager preamble.
    current_pou_generic: bool,

    /// Identity of the implementation being visited in retained global names: its mangled name, e.g.
    /// `FbA__foo` for the method `FbA.foo`, and the generic signature of a template, each prefixed with
    /// its length so that no other POU name reads the same, e.g. `4bind_7ANY_INT` for `bind<T: ANY_INT>`.
    current_pou: String,

    /// Generic signatures of the templates in the unit being lowered, keyed by their name location,
    /// which a POU shares with its implementation. Templates that only differ in their generics
    /// share a name, so the signature keeps their retained globals apart.
    generic_signatures: FxHashMap<SourceLocation, String>,

    /// Counts the retained fat pointers per implementation name. Generic overloads share a name, so
    /// the count continues across them instead of restarting per implementation.
    retained_counters: FxHashMap<String, usize>,

    /// Monotonic counter for generating unique fat-pointer temporary names
    /// (`__fatpointer_0`, `__fatpointer_1`, ...). Never reset; each call-argument wrap site
    /// gets a distinct alloca across the entire compilation unit.
    fp_counter: usize,

    /// Counter for the BOOL temporaries that carry the value of a short-circuit operand whose call
    /// arguments need a preamble, see [`AstVisitorMut::visit_binary_expression`].
    sc_counter: usize,

    /// Names of those BOOL temporaries per POU name. They are referenced inside the branches of the
    /// generated IF, which an allocation statement cannot serve, so they become `VAR_TEMP` variables
    /// of the POU once the unit is lowered.
    temps: FxHashMap<String, Vec<String>>,

    /// Name of the POU that owns the implementation being visited; an action names its parent.
    current_pou_type: String,

    /// Lowercase names of the POUs declared in the unit being lowered. Only those can receive a
    /// `VAR_TEMP`, so a short-circuit in an action whose parent is declared elsewhere keeps the
    /// eager preamble.
    unit_pous: FxHashSet<String>,

    /// Whether the node being visited is the first thing its statement evaluates. The IF that a
    /// short-circuit operand is lowered into is hoisted before the statement, which only preserves
    /// the evaluation order in that position.
    leading: bool,
}

impl<'a> InterfaceDispatchLowerer<'a> {
    pub fn new(ids: IdProvider, index: &'a Index, annotations: &'a AnnotationMapImpl) -> Self {
        InterfaceDispatchLowerer {
            ids,
            index,
            annotations,
            needs_fatpointer: false,
            in_call_args: 0,
            diagnostics: Vec::new(),
            preamble: Vec::new(),
            replacement: None,
            globals: Vec::new(),
            external_globals: Vec::new(),
            current_pou_external: false,
            current_pou_generic: false,
            current_pou: String::new(),
            generic_signatures: FxHashMap::default(),
            retained_counters: FxHashMap::default(),
            fp_counter: 0,
            sc_counter: 0,
            temps: FxHashMap::default(),
            current_pou_type: String::new(),
            unit_pous: FxHashSet::default(),
            leading: false,
        }
    }

    /// Entry point into the interface dispatch lowering pass.
    /// Returns any diagnostics produced by interface validation checks.
    pub fn lower(&mut self, units: &mut [CompilationUnit]) -> Vec<Diagnostic> {
        for unit in units.iter_mut() {
            self.generic_signatures = unit
                .pous
                .iter()
                .filter(|pou| !pou.generics.is_empty())
                .map(|pou| (pou.name_location.clone(), helper::generic_signature(&pou.generics)))
                .collect();
            self.unit_pous = unit.pous.iter().map(|pou| pou.name.to_lowercase()).collect();
            self.visit_compilation_unit(unit);

            // An included unit is compiled to its own object elsewhere, which defines the globals.
            let location = SourceLocation::internal_in_unit(unit.file.get_name());
            for pou in unit.pous.iter_mut() {
                let Some(names) = self.temps.remove(&pou.name.to_lowercase()) else {
                    continue;
                };
                let variables =
                    names.iter().map(|name| helper::create_variable(name, "BOOL", &location)).collect();
                pou.variable_blocks.push(
                    VariableBlock::default()
                        .with_block_type(VariableBlockType::Temp)
                        .with_variables(variables),
                );
            }
            self.temps.clear();
            if !self.globals.is_empty() {
                let variables =
                    self.globals.drain(..).map(|name| helper::create_global(&name, &location)).collect();
                let linkage =
                    if unit.linkage.is_included() { LinkageType::External } else { LinkageType::Internal };
                unit.global_vars
                    .push(VariableBlock::global().with_linkage(linkage).with_variables(variables));
            }
            if !self.external_globals.is_empty() {
                let variables = self
                    .external_globals
                    .drain(..)
                    .map(|name| helper::create_global(&name, &location))
                    .collect();
                unit.global_vars.push(
                    VariableBlock::global().with_linkage(LinkageType::External).with_variables(variables),
                );
            }
        }

        // If any interface-typed variable was encountered, inject the __FATPOINTER struct
        // into the first compilation unit so it is available for later pipeline stages.
        if self.needs_fatpointer {
            if let Some(unit) = units.first_mut() {
                unit.user_types.push(helper::create_fat_pointer_struct());
            }
        }

        std::mem::take(&mut self.diagnostics)
    }
}

impl<'a> AstVisitorMut for InterfaceDispatchLowerer<'a> {
    fn visit_data_type_declaration(&mut self, data_type_declaration: &mut DataTypeDeclaration) {
        if let DataTypeDeclaration::Reference { referenced_type, .. } = data_type_declaration {
            if self.index.find_effective_type_by_name(referenced_type).is_some_and(DataType::is_interface) {
                *referenced_type = FATPOINTER_TYPE_NAME.to_string();
                self.needs_fatpointer = true;
            }
        }

        data_type_declaration.walk(self);
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        if implementation.location.is_internal() {
            return;
        }

        self.current_pou_type = implementation.type_name.to_lowercase();
        let name = implementation.name.replace('.', "__");
        self.current_pou = format!("{}{name}", name.len());
        if let Some(signature) = self.generic_signatures.get(&implementation.name_location) {
            self.current_pou = format!("{}_{}{signature}", self.current_pou, signature.len());
        }
        self.current_pou_external = implementation.linkage.is_external() || implementation.generic;
        self.current_pou_generic = implementation.generic;
        implementation.walk(self);
    }

    fn visit_statement_list(&mut self, stmts: &mut Vec<AstNode>) {
        // Save any preamble accumulated by the parent context (e.g. from a control flow
        // condition walk). Without this, the drain loop below would clear it on its first
        // iteration, losing preamble generated outside this statement list.
        let saved_preamble = std::mem::take(&mut self.preamble);

        let original = std::mem::take(stmts);
        let mut output = Vec::with_capacity(original.len());

        for mut stmt in original {
            self.preamble.clear();
            self.replacement = None;
            self.leading = true;

            stmt.walk(self);

            // Flush any accumulated preamble before the statement (e.g. fat-pointer allocas).
            output.append(&mut self.preamble);

            // Either emit the replacement statements or the (potentially mutated) original.
            match self.replacement.take() {
                Some(expansion) => output.extend(expansion),
                None => output.push(stmt),
            }
        }

        *stmts = output;

        // Restore the parent's preamble so it gets flushed at the correct level.
        self.preamble = saved_preamble;
    }

    fn visit_assignment(&mut self, node: &mut AstNode) {
        let AstStatement::Assignment(Assignment { left, right }) = &mut node.stmt else {
            return;
        };

        // Walk children first so nested calls/expressions are processed before we inspect types. The
        // target's address is computed before the value, so the value is only evaluated first when
        // that address needs no evaluation of its own.
        let leading = self.leading && self.has_static_address(left);
        self.leading = false;
        left.walk(self);
        self.leading = leading;
        right.walk(self);

        // Named call arguments like `in1 := instance` are Assignment nodes too, but
        // `visit_call_statement` is responsible for those.
        if self.in_call_args > 0 {
            return;
        }

        let lhs_type = self.annotations.get_type_or_void(left, self.index);
        let rhs_type = self.annotations.get_type_or_void(right, self.index);
        let rhs_type_hint = self.annotations.get_hint_or_void(right, self.index);

        // Interface-to-interface assignment (e.g. `refIA := refIB` where IB extends IA)
        if lhs_type.is_interface() && rhs_type.is_interface() {
            if lhs_type == rhs_type {
                // Simple memcpy, codegen does this for us; the itable layout is identical,
                // only the data field points to potentially different instances.
                return;
            }

            let rhs_interface_name = rhs_type.get_name();
            let lhs_interface_name = lhs_type.get_name();

            if let Some(diagnostic) = validation::validate_interface_assignment(
                self.index,
                rhs_interface_name,
                lhs_interface_name,
                &node.location,
            ) {
                self.diagnostics.push(diagnostic);

                // Drop the invalid statement so downstream validation doesn't see a
                // confusing `__FATPOINTER` type mismatch after interface types are rewritten.
                self.replacement = Some(vec![]);
                return;
            }

            // Upcasting: RHS is a child interface, LHS is an ancestor interface.
            // Expand into: lhs.data := rhs.data; lhs.table := __itable_<rhs>#(rhs.table^).__upcast_<lhs>
            let data_assign = helper::create_data_copy(&mut self.ids, left, right);
            let table_assign = helper::create_upcast_table_assignment(
                &mut self.ids,
                left,
                right,
                rhs_interface_name,
                lhs_interface_name,
            );
            self.replacement = Some(vec![data_assign, table_assign]);
            return;
        }

        // Check if this is an interface assignment: LHS is interface-typed, RHS is a concrete
        // POU that will be treated as the lhs-interface type (type-hint).
        if !(lhs_type.is_interface() && rhs_type_hint.is_interface()) {
            // Not a valid polymorphic assignment
            return;
        }

        let interface_name = lhs_type.get_name();
        let pou_name = rhs_type.get_name();

        if let Some(diagnostic) = validation::validate_pou_implements_interface(
            self.index,
            pou_name,
            interface_name,
            &node.location,
        ) {
            self.diagnostics.push(diagnostic);

            // Drop the invalid statement so downstream validation doesn't see a
            // confusing `__FATPOINTER` type mismatch after interface types are rewritten.
            self.replacement = Some(vec![]);
            return;
        }

        // Replace the original assignment with the two fat-pointer field assignments.
        let data_assign = helper::create_data_assignment(&mut self.ids, left, right);
        let table_assign = helper::create_table_assignment(&mut self.ids, left, interface_name, pou_name);
        self.replacement = Some(vec![data_assign, table_assign]);
    }

    /// A preamble is hoisted before the enclosing statement, which would run it even when AND_THEN or
    /// OR_ELSE skips the operand that needs it, and so rebind a retained fat pointer for a call that
    /// never happens. Such an operand is lowered into an IF that carries its preamble in the branch
    /// that evaluates it and leaves its value in a BOOL `VAR_TEMP` of the POU. The IF is hoisted as
    /// well, so this only applies where the short-circuit is the first thing its statement evaluates,
    /// where both operands are BOOL so that validation still sees a wrong operand, and where the POU
    /// is declared in this unit, is not a generic template and can take the `VAR_TEMP`; elsewhere the
    /// preamble stays eager.
    ///
    /// ```text
    /// // Before:
    /// enabled AND_THEN holder.bind(instance)
    ///
    /// // After:
    /// IF enabled THEN
    ///     __fatpointer_4main_0.data := ADR(instance);
    ///     __fatpointer_4main_0.table := ADR(__itable_IA_FbA_instance);
    ///     __shortcircuit_0 := holder.bind(__fatpointer_4main_0);
    /// ELSE
    ///     __shortcircuit_0 := FALSE;
    /// END_IF
    /// __shortcircuit_0
    /// ```
    fn visit_binary_expression(&mut self, node: &mut AstNode) {
        let AstStatement::BinaryExpression(BinaryExpression { operator, left, right }) = &mut node.stmt
        else {
            unreachable!();
        };

        let short_circuit = matches!(operator, Operator::AndThen | Operator::OrElse);
        let conditional = short_circuit
            && self.leading
            && !self.current_pou_generic
            && self.unit_pous.contains(&self.current_pou_type)
            && self.annotations.get_type_or_void(left, self.index).get_type_information().is_bool()
            && self.annotations.get_type_or_void(right, self.index).get_type_information().is_bool();

        // The left operand always runs, so its preamble stays with the enclosing statement. The right
        // operand is never the first thing the statement evaluates.
        self.visit(left);
        self.leading = false;
        if !conditional {
            self.visit(right);
            return;
        }

        // Collect the preamble the right operand needs on its own.
        let outer_preamble = std::mem::take(&mut self.preamble);
        self.visit(right);
        let right_preamble = std::mem::replace(&mut self.preamble, outer_preamble);
        if right_preamble.is_empty() {
            return;
        }

        let operator = *operator;
        let location = node.location.clone();
        let AstStatement::BinaryExpression(BinaryExpression { left, right, .. }) =
            std::mem::replace(&mut node.stmt, AstStatement::EmptyStatement(EmptyStatement {}))
        else {
            unreachable!();
        };

        let name = format!("__shortcircuit_{}", self.sc_counter);
        self.sc_counter += 1;
        self.temps.entry(self.current_pou_type.clone()).or_default().push(name.clone());

        // Only the branch that evaluates the right operand runs its preamble.
        let mut evaluate = right_preamble;
        evaluate.push(helper::create_assignment_to(&mut self.ids, &name, *right));
        let skipped_value = helper::create_bool_literal(&mut self.ids, operator == Operator::OrElse);
        let skip = vec![helper::create_assignment_to(&mut self.ids, &name, skipped_value)];
        let (body, else_block) = match operator {
            Operator::AndThen => (evaluate, skip),
            _ => (skip, evaluate),
        };

        self.preamble.push(AstFactory::create_if_statement(
            IfStatement {
                blocks: vec![ConditionalBlock { condition: left, body }],
                else_block,
                end_location: SourceLocation::internal(),
            },
            location,
            self.ids.next_id(),
        ));
        *node = helper::create_identifier_ref(&mut self.ids, &name);
    }

    /// The value of a `REF=` must be a reference, so a short-circuit there is never rewritten into a
    /// temporary; validation rejects the expression as written.
    fn visit_ref_assignment(&mut self, node: &mut AstNode) {
        let AstStatement::RefAssignment(Assignment { left, right }) = &mut node.stmt else {
            unreachable!();
        };

        self.leading = false;
        self.visit(left);
        self.visit(right);
    }

    /// A base whose address needs evaluation, such as a call result or a dereferenced pointer, is
    /// evaluated before the index or member that follows it.
    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        let AstStatement::ReferenceExpr(ReferenceExpr { access, base }) = &mut node.stmt else {
            unreachable!();
        };

        if let Some(base) = base {
            let static_base = self.has_static_address(base);
            self.visit(base);
            self.leading = self.leading && static_base;
        }

        match access {
            ReferenceAccess::Member(node) | ReferenceAccess::Index(node) | ReferenceAccess::Cast(node) => {
                self.visit(node)
            }
            _ => {}
        }
    }

    /// The elements of an expression list, such as the dimensions of an array index, are evaluated in
    /// order, so only the first one can be the first thing the statement evaluates.
    fn visit_expression_list(&mut self, node: &mut AstNode) {
        let AstStatement::ExpressionList(expressions) = &mut node.stmt else {
            unreachable!();
        };

        for expression in expressions.iter_mut() {
            self.visit(expression);
            self.leading = false;
        }
    }

    /// Only the first condition of an IF and the selector of a CASE are evaluated first; a later
    /// condition or a loop condition is not, so a short-circuit there keeps the eager preamble.
    fn visit_control_statement(&mut self, node: &mut AstNode) {
        let AstStatement::ControlStatement(statement) = &mut node.stmt else {
            unreachable!();
        };

        match statement {
            AstControlStatement::If(statement) => {
                for (position, block) in statement.blocks.iter_mut().enumerate() {
                    self.leading = position == 0;
                    self.visit(&mut block.condition);
                    self.visit_statement_list(&mut block.body);
                }
                self.visit_statement_list(&mut statement.else_block);
            }
            AstControlStatement::Case(statement) => {
                self.visit(&mut statement.selector);
                self.leading = false;
                statement.case_blocks.walk(self);
                self.visit_statement_list(&mut statement.else_block);
            }
            _ => {
                self.leading = false;
                statement.walk(self);
            }
        }
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let AstStatement::CallStatement(CallStatement { operator, parameters }) = &mut node.stmt else {
            unreachable!();
        };

        // Walk the operator first (processes nested interface method calls depth-first). Nothing after
        // the call is the first thing the statement evaluates.
        operator.walk(self);
        self.leading = false;

        // Walk parameters with the `in_call_args` guard active. This prevents `visit_assignment`
        // from expanding named arguments like `in1 := instance` into fat-pointer field assignments.
        // The counter nests correctly for cases like `foo(in1 := bar(in2 := instance))`.
        if let Some(ref mut params) = parameters {
            self.in_call_args += 1;
            params.walk(self);
            self.in_call_args -= 1;
        }

        // --- Call argument wrapping ---
        // Inspect each argument: if a concrete POU is passed where an interface is expected,
        // allocate a temporary fat pointer in the preamble and replace the argument in-place.
        if let Some(ref mut params) = parameters {
            let mut args = steal_expression_list(params);
            let bindings = self.bind_arguments_to_parameters(operator, &args);

            for (arg, parameter) in args.iter_mut().zip(bindings) {
                match &mut arg.stmt {
                    // Named argument (input, output, or in-out). Inspect and potentially replace the RHS.
                    AstStatement::Assignment(Assignment { right, .. })
                    | AstStatement::OutputAssignment(Assignment { right, .. })
                    | AstStatement::RefAssignment(Assignment { right, .. }) => {
                        self.maybe_wrap_argument(parameter, right.as_mut());
                    }

                    // Positional argument. Inspect and potentially replace the whole node.
                    _ => {
                        self.maybe_wrap_argument(parameter, arg);
                    }
                }
            }

            // Rebuild the parameter list from the (potentially mutated) arguments.
            let location = params.get_location();
            **params = AstFactory::create_expression_list(args, location, self.ids.next_id());
        }

        // --- Direct interface reference calls ---
        // `refIA()` is invalid. Validate it here before codegen would later stumble over the
        // still-unlowered interface-typed operator.
        if let Some(diagnostic) =
            validation::validate_direct_interface_call(self.annotations, self.index, operator)
        {
            self.diagnostics.push(diagnostic);

            // Drop the enclosing statement so downstream codegen does not trip over the still-invalid
            // direct interface call after we already emitted the user-facing diagnostic.
            self.replacement = Some(vec![]);
            return;
        }

        // --- Interface method call rewriting ---
        // Check if the call operator's base is an interface-typed variable. If so, rewrite the
        // call into an indirect dispatch through the itable.
        let Some(base) = operator.get_base_ref_expr() else {
            return;
        };

        let Some(base_type) = self.annotations.get_type(base, self.index) else {
            return;
        };

        if !base_type.is_interface() {
            return;
        }

        let interface_name = base_type.get_name().to_string();
        helper::rewrite_interface_method_call(&mut self.ids, operator, parameters, &interface_name);
    }
}

impl<'a> InterfaceDispatchLowerer<'a> {
    /// Whether `node` names storage whose address needs no evaluation: a variable or a chain of plain
    /// member accesses, without an index, a dereference, a call or an auto-dereferenced reference
    /// whose pointer codegen loads first and a side effect could change.
    fn has_static_address(&self, node: &AstNode) -> bool {
        let static_variable = match self.annotations.get(node) {
            Some(StatementAnnotation::Variable { auto_deref, .. }) => auto_deref.is_none(),
            Some(StatementAnnotation::Program { .. }) => true,
            _ => false,
        };

        match &node.stmt {
            AstStatement::Identifier(_) => static_variable,
            AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(member), base }) => {
                static_variable
                    && matches!(member.stmt, AstStatement::Identifier(_))
                    && base.as_ref().is_none_or(|base| self.has_static_address(base))
            }
            _ => false,
        }
    }

    /// Resolves the callee parameter each argument binds to. A named argument carries its declaring
    /// POU and position in its argument annotation. A positional argument carries the callee instead,
    /// so it is bound by slot over the callee's parameters, inherited ones included, as the validator
    /// does, and a surplus positional argument binds to the variadic parameter. A function pointer
    /// call passes the instance as a first argument that binds to nothing.
    fn bind_arguments_to_parameters(
        &self,
        operator: &AstNode,
        args: &[AstNode],
    ) -> Vec<Option<&'a VariableIndexEntry>> {
        let index = self.index;
        let arguments: Vec<&AstNode> = args.iter().collect();
        let callee = arguments.iter().find_map(|arg| match self.annotations.get_hint(arg) {
            Some(StatementAnnotation::Argument { pou, .. }) if arg.get_assignment_identifier().is_none() => {
                Some(pou.as_str())
            }
            _ => None,
        });
        let parameters = callee.map(|pou| index.get_available_parameters(pou)).unwrap_or_default();
        let skip = usize::from(self.annotations.get(operator).is_some_and(StatementAnnotation::is_fnptr))
            .min(arguments.len());
        let slots = resolve_argument_slots(&arguments[skip..], parameters.iter().map(|it| it.get_name()));

        let mut bindings = vec![None; skip];
        for (arg, slot) in arguments.iter().skip(skip).zip(slots) {
            let parameter = match self.annotations.get_hint(arg) {
                Some(StatementAnnotation::Argument { pou, position, .. })
                    if arg.get_assignment_identifier().is_some() =>
                {
                    index.find_parameter(pou, *position as u32)
                }
                _ => slot
                    .and_then(|slot| parameters.get(slot))
                    .copied()
                    .or_else(|| callee.and_then(|pou| index.get_variadic_member(pou))),
            };
            bindings.push(parameter);
        }

        bindings
    }

    /// Checks whether `arg` is a concrete POU being passed where an interface is expected.
    /// If so, generates a temporary fat pointer (alloca + field assignments) in `self.preamble`
    /// and replaces `arg` in-place with a reference to the temporary.
    ///
    /// `parameter` is the callee parameter `arg` binds to, see [`Self::bind_arguments_to_parameters`].
    fn maybe_wrap_argument(&mut self, parameter: Option<&VariableIndexEntry>, arg: &mut AstNode) {
        // An omitted argument has nothing to wrap; the callee resets the output.
        if arg.is_empty_statement() {
            return;
        }

        // A by-reference variable is annotated with the raw name of its target type, so resolve an alias.
        let actual_type = self.annotations.get_type_or_void(arg, self.index);
        let actual_type = self.index.find_effective_type(actual_type).unwrap_or(actual_type);

        // An argument that binds to no parameter, such as one of a generic builtin, keeps the type the
        // resolver hinted for it and is passed by value.
        let Some(parameter) = parameter else {
            let expected_type = self.annotations.get_hint_or_void(arg, self.index);
            self.wrap_argument(arg, actual_type, expected_type, None, false);
            return;
        };

        let parameter_type = self.index.get_effective_type_or_void_by_name(parameter.get_type_name());

        // VAR_IN_OUT, VAR_INPUT {ref}, REFERENCE TO and function VAR_OUTPUT parameters are indexed as
        // auto-deref pointers to the interface, so look through the pointer to find the interface the
        // callee expects.
        let (expected_type, pointer) = match parameter_type.get_type_information() {
            DataTypeInformation::Pointer { auto_deref: Some(_), inner_type_name, .. } => {
                (self.index.get_effective_type_or_void_by_name(inner_type_name), true)
            }
            _ => (parameter_type, false),
        };

        // A function block or program VAR_OUTPUT is a plain member, but the call writes through it too.
        let by_ref = pointer || parameter.is_output();

        // A stateful POU keeps a pointer-typed parameter between calls, and any POU may save or forward
        // the reference, so the fat pointer behind a pointer-typed parameter must outlive the calling
        // function.
        self.wrap_argument(arg, actual_type, expected_type, by_ref.then_some(parameter), pointer);
    }

    /// Wraps `arg` for a parameter of `expected_type`. `by_ref` carries the parameter when the callee
    /// writes through it, and `retained` whether the callee may keep the reference.
    fn wrap_argument(
        &mut self,
        arg: &mut AstNode,
        actual_type: &DataType,
        expected_type: &DataType,
        by_ref: Option<&VariableIndexEntry>,
        retained: bool,
    ) {
        // Not an interface parameter, nothing to do.
        if !expected_type.is_interface() {
            return;
        }

        // Argument is already interface-typed — validate compatibility before upcasting.
        if actual_type.is_interface() {
            let source_iface = actual_type.get_name();
            let target_iface = expected_type.get_name();

            // A by-reference parameter is storage the callee writes through, so the argument is passed
            // as is: it must be a variable, since the callee may keep its address, and the exact
            // interface, since an upcast temporary would never reach the caller.
            if let Some(parameter) = by_ref {
                let diagnostic = validation::validate_byref_argument_is_variable(self.index, parameter, arg)
                    .or_else(|| {
                        validation::validate_byref_interface_argument(
                            parameter.get_name(),
                            target_iface,
                            source_iface,
                            &arg.location,
                        )
                    })
                    .or_else(|| {
                        validation::validate_variadic_takes_argument(
                            self.index,
                            parameter,
                            source_iface,
                            target_iface,
                            &arg.location,
                        )
                    });
                if let Some(diagnostic) = diagnostic {
                    self.diagnostics.push(diagnostic);
                }

                return;
            }

            if let Some(diagnostic) = validation::validate_interface_assignment(
                self.index,
                source_iface,
                target_iface,
                &arg.location,
            ) {
                self.diagnostics.push(diagnostic);
                return;
            }
        }

        // Already the right interface type (or same concrete type), no wrapping needed.
        if actual_type == expected_type {
            return;
        }

        let pou_name = actual_type.get_name();
        let interface_name = expected_type.get_name();

        // A by-reference parameter needs a variable: a generated fat pointer would point at the temporary
        // of a call result. Validate that here, before the argument is replaced, mirroring E031 for
        // other by-reference parameters.
        let diagnostic = by_ref
            .and_then(|parameter| validation::validate_byref_argument_is_variable(self.index, parameter, arg))
            .or_else(|| {
                validation::validate_pou_implements_interface(
                    self.index,
                    pou_name,
                    interface_name,
                    &arg.location,
                )
            })
            .or_else(|| {
                by_ref.and_then(|parameter| {
                    validation::validate_variadic_takes_argument(
                        self.index,
                        parameter,
                        pou_name,
                        interface_name,
                        &arg.location,
                    )
                })
            });

        if let Some(diagnostic) = diagnostic {
            self.diagnostics.push(diagnostic);

            // Replace the argument with a dummy fat pointer so the call remains structurally
            // valid and doesn't trigger confusing `__FATPOINTER` type errors downstream.
            let fp_name = format!("__fatpointer_{}", self.fp_counter);
            self.fp_counter += 1;
            self.preamble.push(helper::create_alloca(&mut self.ids, &fp_name));
            *arg = helper::create_identifier_ref(&mut self.ids, &fp_name);

            return;
        }

        // Generate a unique name for the fat pointer and allocate it.
        let fp_name = if retained {
            let counter = self.retained_counters.entry(self.current_pou.clone()).or_default();
            let name = format!("__fatpointer_{}_{}", self.current_pou, counter);
            *counter += 1;
            if self.current_pou_external {
                self.external_globals.push(name.clone());
            } else {
                self.globals.push(name.clone());
            }
            name
        } else {
            let name = format!("__fatpointer_{}", self.fp_counter);
            self.fp_counter += 1;
            self.preamble.push(helper::create_alloca(&mut self.ids, &name));
            name
        };

        // Build the reference node that will replace the original argument.
        let fp_ref = helper::create_identifier_ref(&mut self.ids, &fp_name);

        if actual_type.is_interface() {
            // Upcast: actual is a different (child) interface, copy .data and read __upcast_* for .table.
            let rhs_interface_name = actual_type.get_name();
            let lhs_interface_name = expected_type.get_name();
            let data_copy = helper::create_data_copy(&mut self.ids, &fp_ref, arg);
            let table_assign = helper::create_upcast_table_assignment(
                &mut self.ids,
                &fp_ref,
                arg,
                rhs_interface_name,
                lhs_interface_name,
            );

            self.preamble.push(data_copy);
            self.preamble.push(table_assign);
        } else {
            // POU-to-interface: wrap concrete instance in a fat pointer with ADR() + itable instance.
            let interface_name = expected_type.get_name();
            let pou_name = actual_type.get_name();
            let data_assign = helper::create_data_assignment(&mut self.ids, &fp_ref, arg);
            let table_assign =
                helper::create_table_assignment(&mut self.ids, &fp_ref, interface_name, pou_name);

            self.preamble.push(data_assign);
            self.preamble.push(table_assign);
        }

        // Replace the argument in-place with the fat pointer reference.
        *arg = fp_ref;
    }
}

/// Helper functions for AST construction and type checking.
mod helper {
    use super::*;
    use crate::lowering::polymorphism::table::interface::helper as itable_helper;

    /// A visitor that reassigns all AST node IDs in a subtree with fresh IDs.
    /// Used after cloning AST subtrees to ensure each node in the tree has a unique ID.
    struct IdReassigner<'a> {
        ids: &'a mut IdProvider,
    }

    impl AstVisitorMut for IdReassigner<'_> {
        fn visit(&mut self, node: &mut AstNode) {
            node.id = self.ids.next_id();
            node.walk(self);
        }
    }

    /// Clones an AST node and reassigns all node IDs in the cloned subtree with fresh IDs,
    /// preventing duplicate IDs between the original and cloned trees.
    fn clone_with_new_ids(node: &AstNode, ids: &mut IdProvider) -> AstNode {
        let mut cloned = node.clone();
        IdReassigner { ids }.visit(&mut cloned);
        cloned
    }

    /// Rewrites an interface method call into an indirect dispatch through the itable.
    /// Applies four in-place transformations to the operator and parameters:
    ///
    /// ```text
    /// reference.foo(args)
    ///   → __itable_IA#(reference.table^).foo^(reference.data^, args)
    /// ```
    pub fn rewrite_interface_method_call(
        ids: &mut IdProvider,
        operator: &mut AstNode,
        parameters: &mut Option<Box<AstNode>>,
        interface_name: &str,
    ) {
        // Clone the base with fresh IDs before any mutation. We need the original for building
        // `.data^` (step 1) while step 2 will consume the in-tree base for `.table^`.
        // The caller (`visit_call_statement`) already verified that operator has a base ref expression.
        let base = clone_with_new_ids(operator.get_base_ref_expr().unwrap(), ids);

        // Step 1: Prepend the data pointer as the implicit first argument.
        // reference.foo(args) → reference.foo(reference.data^, args)
        let data_deref = AstFactory::create_deref_reference(
            AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    FATPOINTER_DATA_FIELD_NAME,
                    SourceLocation::internal(),
                    ids.next_id(),
                ),
                Some(base),
                ids.next_id(),
            ),
            ids.next_id(),
            SourceLocation::internal(),
        );

        match parameters {
            None => {
                *parameters = Some(Box::new(data_deref));
            }
            Some(ref mut params) => match &mut params.stmt {
                AstStatement::ExpressionList(expressions) => {
                    expressions.insert(0, data_deref);
                }
                _ => {
                    let mut new_params = Box::new(AstFactory::create_expression_list(
                        vec![data_deref, std::mem::take(params)],
                        SourceLocation::internal(),
                        ids.next_id(),
                    ));
                    std::mem::swap(params, &mut new_params);
                }
            },
        }

        // Step 2: Replace the base with a dereferenced table access.
        // reference.foo(...) → reference.table^.foo(...)
        let old_base = operator.get_base_ref_expr_mut().unwrap();
        let mut new_base = AstFactory::create_deref_reference(
            AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    FATPOINTER_TABLE_FIELD_NAME,
                    SourceLocation::internal(),
                    ids.next_id(),
                ),
                Some(std::mem::take(old_base)),
                ids.next_id(),
            ),
            ids.next_id(),
            SourceLocation::internal(),
        );
        std::mem::swap(old_base, &mut new_base);

        // Step 3: Cast the table access to the concrete itable type.
        // reference.table^.foo(...) → __itable_IA#(reference.table^).foo(...)
        let cast_base = operator.get_base_ref_expr_mut().unwrap();
        let itable_name = itable_helper::get_itable_name(interface_name);
        let cast_target = AstFactory::create_member_reference(
            AstFactory::create_identifier(itable_name, SourceLocation::internal(), ids.next_id()),
            None,
            ids.next_id(),
        );
        let mut casted = AstFactory::create_cast_statement(
            cast_target,
            AstFactory::create_paren_expression(
                std::mem::take(cast_base),
                SourceLocation::internal(),
                ids.next_id(),
            ),
            &SourceLocation::internal(),
            ids.next_id(),
        );
        std::mem::swap(cast_base, &mut casted);

        // Step 4: Dereference the function pointer (the method entry in the itable).
        // __itable_IA#(reference.table^).foo(...) → __itable_IA#(reference.table^).foo^(...)
        let mut derefed = AstFactory::create_deref_reference(
            std::mem::take(operator),
            ids.next_id(),
            SourceLocation::internal(),
        );
        std::mem::swap(operator, &mut derefed);
    }

    /// Builds `lhs.data := ADR(rhs)`.
    ///
    /// Clones both sides with fresh IDs: the left-hand side gets a `.data` member access
    /// appended, and the right-hand side is wrapped in an `ADR()` call.
    pub fn create_data_assignment(ids: &mut IdProvider, lhs: &AstNode, rhs: &AstNode) -> AstNode {
        let lhs_data = create_member_access(ids, lhs, FATPOINTER_DATA_FIELD_NAME);
        let rhs_clone = clone_with_new_ids(rhs, ids);
        let adr_rhs = create_adr_call(ids, rhs_clone);
        AstFactory::create_assignment(lhs_data, adr_rhs, ids.next_id())
    }

    /// Builds `lhs.table := ADR(__itable_<interface>_<pou>_instance)`.
    ///
    /// Clones the left-hand side, appends a `.table` member access, creates a reference to the
    /// itable global instance, wraps it in `ADR()`, and returns the assignment node.
    pub fn create_table_assignment(
        ids: &mut IdProvider,
        lhs: &AstNode,
        interface_name: &str,
        pou_name: &str,
    ) -> AstNode {
        let lhs_table = create_member_access(ids, lhs, FATPOINTER_TABLE_FIELD_NAME);

        // Build reference to __itable_<interface>_<pou>_instance
        let instance_name = itable_helper::get_itable_instance_name(interface_name, pou_name);
        let itable_ref = AstFactory::create_member_reference(
            AstFactory::create_identifier(instance_name, SourceLocation::internal(), ids.next_id()),
            None,
            ids.next_id(),
        );
        let adr_itable = create_adr_call(ids, itable_ref);
        AstFactory::create_assignment(lhs_table, adr_itable, ids.next_id())
    }

    /// Builds `lhs.data := rhs.data` — a direct member-to-member copy used for interface upcasting
    /// where both sides are already fat pointers.
    pub fn create_data_copy(ids: &mut IdProvider, lhs: &AstNode, rhs: &AstNode) -> AstNode {
        let lhs_data = create_member_access(ids, lhs, FATPOINTER_DATA_FIELD_NAME);
        let rhs_data = create_member_access(ids, rhs, FATPOINTER_DATA_FIELD_NAME);

        AstFactory::create_assignment(lhs_data, rhs_data, ids.next_id())
    }

    /// Builds `lhs.table := __itable_<rhs_interface>#(rhs.table^).__upcast_<lhs_interface>`.
    ///
    /// Used for interface upcasting: reads the upcast pointer from the RHS itable to get the
    /// correct parent itable pointer for the LHS interface type.
    pub fn create_upcast_table_assignment(
        ids: &mut IdProvider,
        lhs: &AstNode,
        rhs: &AstNode,
        rhs_interface_name: &str,
        lhs_interface_name: &str,
    ) -> AstNode {
        let lhs_table = create_member_access(ids, lhs, FATPOINTER_TABLE_FIELD_NAME);

        // Build: rhs.table^
        let rhs_table_deref = AstFactory::create_deref_reference(
            create_member_access(ids, rhs, FATPOINTER_TABLE_FIELD_NAME),
            ids.next_id(),
            SourceLocation::internal(),
        );

        // Build: __itable_<rhs_interface>#(rhs.table^)
        let itable_name = itable_helper::get_itable_name(rhs_interface_name);
        let cast_target = AstFactory::create_member_reference(
            AstFactory::create_identifier(itable_name, SourceLocation::internal(), ids.next_id()),
            None,
            ids.next_id(),
        );
        let casted = AstFactory::create_cast_statement(
            cast_target,
            AstFactory::create_paren_expression(rhs_table_deref, SourceLocation::internal(), ids.next_id()),
            &SourceLocation::internal(),
            ids.next_id(),
        );

        // Build: __itable_<rhs_interface>#(rhs.table^).__upcast_<lhs_interface>
        let upcast_field_name = itable_helper::get_upcast_field_name(lhs_interface_name);
        let upcast_read = AstFactory::create_member_reference(
            AstFactory::create_identifier(upcast_field_name, SourceLocation::internal(), ids.next_id()),
            Some(casted),
            ids.next_id(),
        );

        AstFactory::create_assignment(lhs_table, upcast_read, ids.next_id())
    }

    /// Appends a member access to a base expression: `base.field_name`.
    /// Clones the base with fresh IDs to avoid duplicate AST node IDs in the tree.
    fn create_member_access(ids: &mut IdProvider, base: &AstNode, field_name: &str) -> AstNode {
        AstFactory::create_member_reference(
            AstFactory::create_identifier(field_name, SourceLocation::internal(), ids.next_id()),
            Some(clone_with_new_ids(base, ids)),
            ids.next_id(),
        )
    }

    /// Wraps an expression in an `ADR(expr)` call.
    fn create_adr_call(ids: &mut IdProvider, expr: AstNode) -> AstNode {
        let operator = AstFactory::create_member_reference(
            AstFactory::create_identifier("ADR", SourceLocation::internal(), ids.next_id()),
            None,
            ids.next_id(),
        );
        AstFactory::create_call_statement(operator, Some(expr), ids.next_id(), SourceLocation::internal())
    }

    /// Creates an `AllocationStatement` for a `__FATPOINTER` temporary with the given name.
    pub fn create_alloca(ids: &mut IdProvider, name: &str) -> AstNode {
        AstNode {
            stmt: AstStatement::AllocationStatement(Allocation {
                name: name.to_string(),
                reference_type: FATPOINTER_TYPE_NAME.to_string(),
                // The fat pointer itself is consumed by the dispatch call of its own
                // statement; only the object it points at may need pinning.
                statement_scoped: true,
            }),
            id: ids.next_id(),
            location: SourceLocation::internal(),
            metadata: None,
        }
    }

    /// Creates `name := value`.
    pub fn create_assignment_to(ids: &mut IdProvider, name: &str, value: AstNode) -> AstNode {
        let target = create_identifier_ref(ids, name);
        AstFactory::create_assignment(target, value, ids.next_id())
    }

    /// Creates a `TRUE` or `FALSE` literal.
    pub fn create_bool_literal(ids: &mut IdProvider, value: bool) -> AstNode {
        AstFactory::create_literal(AstLiteral::Bool(value), SourceLocation::internal(), ids.next_id())
    }

    /// Mangles the natures of a template's generics into an identifier, e.g. `ANY_INT_ANY_REAL` for
    /// `<T: ANY_INT, U: ANY_REAL>`. The names of the generics do not tell templates apart: templates
    /// that differ only in them produce the same monomorphs. Every nature starts with `ANY`, so the
    /// sequence reads back unambiguously.
    pub fn generic_signature(generics: &[GenericBinding]) -> String {
        generics.iter().map(|binding| binding.nature.to_string()).collect::<Vec<_>>().join("_")
    }

    /// Creates a global `__FATPOINTER` variable with the given name, for a fat pointer that a POU may
    /// keep beyond the call. `location` places it in the unit that defines it.
    pub fn create_global(name: &str, location: &SourceLocation) -> Variable {
        create_variable(name, FATPOINTER_TYPE_NAME, location)
    }

    /// Creates a variable declaration of the given type.
    pub fn create_variable(name: &str, type_name: &str, location: &SourceLocation) -> Variable {
        Variable {
            name: name.to_string(),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: type_name.to_string(),
                location: location.clone(),
            },
            initializer: None,
            address: None,
            location: location.clone(),
        }
    }

    /// Creates a simple identifier reference node (e.g. `__fatpointer_0`).
    pub fn create_identifier_ref(ids: &mut IdProvider, name: &str) -> AstNode {
        AstFactory::create_member_reference(
            AstFactory::create_identifier(name, SourceLocation::internal(), ids.next_id()),
            None,
            ids.next_id(),
        )
    }

    /// Creates a variable of type `POINTER TO __VOID` with the given name.
    pub fn create_void_pointer_variable(name: &str, location: &SourceLocation) -> Variable {
        Variable {
            name: name.to_string(),
            data_type_declaration: DataTypeDeclaration::Definition {
                data_type: Box::new(AstDataType::PointerType {
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

    /// Builds the `__FATPOINTER` struct type declaration used to represent interface references
    /// at runtime. The struct contains two `POINTER TO __VOID` fields: `data` (the concrete
    /// instance address) and `table` (the itable address). Every compilation that uses interfaces
    /// declares this type, so it is built-in: no object defines a constructor for it.
    pub fn create_fat_pointer_struct() -> UserTypeDeclaration {
        let location = SourceLocation::internal();
        UserTypeDeclaration {
            data_type: AstDataType::StructType {
                name: Some(FATPOINTER_TYPE_NAME.to_string()),
                variables: vec![
                    create_void_pointer_variable(FATPOINTER_DATA_FIELD_NAME, &location),
                    create_void_pointer_variable(FATPOINTER_TABLE_FIELD_NAME, &location),
                ],
            },
            initializer: None,
            location,
            scope: None,
            linkage: LinkageType::BuiltIn,
        }
    }
}

#[cfg(test)]
mod tests {
    use driver::{parse_and_annotate, pipelines::AnnotatedProject, pipelines::AnnotatedUnit};
    use plc_source::SourceCode;

    fn lower(source: impl Into<SourceCode>) -> AnnotatedUnit {
        let (_, mut project): (_, AnnotatedProject) =
            parse_and_annotate("unit-test", vec![source.into()]).unwrap();

        project.units.remove(0)
    }

    fn lower_and_serialize_statements(source: impl Into<SourceCode>, pous: &[&str]) -> Vec<String> {
        let (_, project) = parse_and_annotate("unit-test", vec![source.into()]).unwrap();
        let unit = project.units[0].get_unit();

        let mut result = Vec::new();
        for pou in pous {
            result.push(format!("// Statements in {pou}"));
            let statements = &unit.implementations.iter().find(|it| &it.name == pou).unwrap().statements;

            for statement in statements {
                let statement = statement.as_string();
                let mut split_statements = statement.lines().peekable();

                if split_statements.peek().is_some() {
                    for split_statement in split_statements {
                        result.push(split_statement.to_string());
                    }
                } else {
                    result.push(statement);
                }
            }
        }

        result
    }

    fn lower_and_serialize_globals(source: impl Into<SourceCode>) -> Vec<String> {
        let (_, project) = parse_and_annotate("unit-test", vec![source.into()]).unwrap();
        let unit = project.units[0].get_unit();

        unit.global_vars
            .iter()
            .flat_map(|block| block.variables.iter().map(move |variable| (block, variable)))
            .map(|(block, variable)| {
                let linkage = if block.linkage.is_external() { " {external}" } else { "" };
                format!(
                    "{} : {}{linkage}",
                    variable.name,
                    variable.data_type_declaration.get_name().unwrap_or_default()
                )
            })
            .collect()
    }

    mod fatpointer {
        use crate::lowering::polymorphism::dispatch::interface::FATPOINTER_TYPE_NAME;

        #[test]
        fn is_generated_on_demand() {
            // Initially, no POU makes use of a interface as a variable
            {
                let source = r#"
                    INTERFACE IA
                    END_INTERFACE

                    FUNCTION_BLOCK FbA
                    END_FUNCTION_BLOCK
                "#;

                let annotated_unit = super::lower(source);
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

                let annotated_unit = super::lower(source);
                let mut user_types = annotated_unit.get_unit().user_types.iter();
                assert!(user_types.any(|ty| ty.data_type.get_name().unwrap() == FATPOINTER_TYPE_NAME));
            }
        }

        #[test]
        fn replaces_interface_typed_return_type() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION foo: IA
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower(source).get_unit().pous, @r#"
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
                POU {
                    name: "__itable_IA__ctor",
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "self",
                                    data_type: DataTypeReference {
                                        referenced_type: "__itable_IA",
                                    },
                                },
                            ],
                            variable_block_type: InOut,
                        },
                    ],
                    pou_type: Init,
                    return_type: None,
                    interfaces: [],
                    properties: [],
                },
            ]
            "#);
        }

        #[test]
        fn replaces_interface_typed_variables() {
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

            insta::assert_debug_snapshot!(super::lower(source).get_unit().pous[0].variable_blocks, @r#"
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
        fn replaces_interface_typed_aliased_variables() {
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

            insta::assert_debug_snapshot!(super::lower(source).get_unit().pous[0].variable_blocks, @r#"
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
        fn replaces_interface_typed_array_variables() {
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

            // The variables reference compiler-generated array type names (e.g. `__main_localVariable`)
            // because the pre-processor extracts inline array definitions into `user_types`. The actual
            // IA → __FATPOINTER replacement is visible in the user_types snapshot below.
            let unit = super::lower(source);
            let compilation_unit = unit.get_unit();

            insta::assert_debug_snapshot!(compilation_unit.pous[0].variable_blocks, @r#"
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

            // Verify the inner element type was replaced with __FATPOINTER in the generated array
            // types. We extract just the (name, inner_type) pairs to keep the snapshot focused.
            let array_inner_types: Vec<_> = compilation_unit
                .user_types
                .iter()
                .filter_map(|ut| match &ut.data_type {
                    plc_ast::ast::DataType::ArrayType { name, referenced_type, .. } => {
                        Some((name.as_deref().unwrap_or("?"), format!("{referenced_type:?}")))
                    }
                    _ => None,
                })
                .collect();
            insta::assert_debug_snapshot!(array_inner_types, @r#"
            [
                (
                    "__main_localVariable",
                    "DataTypeReference { referenced_type: \"__FATPOINTER\" }",
                ),
                (
                    "__main_localVariableNested_",
                    "DataTypeReference { referenced_type: \"__FATPOINTER\" }",
                ),
                (
                    "__main_localVariableNested",
                    "DataTypeReference { referenced_type: \"__main_localVariableNested_\" }",
                ),
                (
                    "__main_localVariableNestedNested__",
                    "DataTypeReference { referenced_type: \"__FATPOINTER\" }",
                ),
                (
                    "__main_localVariableNestedNested_",
                    "DataTypeReference { referenced_type: \"__main_localVariableNestedNested__\" }",
                ),
                (
                    "__main_localVariableNestedNested",
                    "DataTypeReference { referenced_type: \"__main_localVariableNestedNested_\" }",
                ),
            ]
            "#);
        }

        #[test]
        fn replaces_interface_typed_interface_method_signatures() {
            let source = r#"
                INTERFACE IA
                    METHOD foo : IA
                        VAR_INPUT
                            in1 : IA;
                        END_VAR
                    END_METHOD
                END_INTERFACE
            "#;

            let unit = super::lower(source);
            let compilation_unit = unit.get_unit();
            let method = &compilation_unit.interfaces[0].methods[0];

            insta::assert_debug_snapshot!(method.variable_blocks, @r#"
            [
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
                VariableBlock {
                    variables: [
                        Variable {
                            name: "in1",
                            data_type: DataTypeReference {
                                referenced_type: "__FATPOINTER",
                            },
                        },
                    ],
                    variable_block_type: Input(
                        ByVal,
                    ),
                },
            ]
            "#);

            insta::assert_debug_snapshot!(method.return_type, @r#"
            Some(
                Aggregate {
                    referenced_type: "__FATPOINTER",
                },
            )
            "#);
        }
    }

    mod assignments {
        #[test]
        fn simple() {
            let source = r#"
                INTERFACE IA
                    // intentionally empty
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    // intentionally empty
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        instance: FbA;
                        reference: IA;
                    END_VAR

                    reference := instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "reference.data := ADR(instance)",
                "reference.table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn qualified_lhs() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Container
                    VAR
                        reference: IA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        fb: Container;
                        instance: FbA;
                    END_VAR

                    fb.reference := instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Container__ctor(fb)",
                "FbA__ctor(instance)",
                "fb.reference.data := ADR(instance)",
                "fb.reference.table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn qualified_rhs() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Container
                    VAR
                        instance: FbA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                        fb: Container;
                    END_VAR

                    reference := fb.instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Container__ctor(fb)",
                "reference.data := ADR(fb.instance)",
                "reference.table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn both_qualified() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK ContainerA
                    VAR
                        reference: IA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK ContainerB
                    VAR
                        instance: FbA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        fb: ContainerA;
                        otherFb: ContainerB;
                    END_VAR

                    fb.reference := otherFb.instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "ContainerA__ctor(fb)",
                "ContainerB__ctor(otherFb)",
                "fb.reference.data := ADR(otherFb.instance)",
                "fb.reference.table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn array_lhs() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        references: ARRAY[1..2] OF IA;
                        instance: FbA;
                    END_VAR

                    references[1] := instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_references__ctor(references)",
                "FbA__ctor(instance)",
                "references[1].data := ADR(instance)",
                "references[1].table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn array_rhs() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                        instances: ARRAY[1..2] OF FbA;
                    END_VAR

                    reference := instances[1];
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_instances__ctor(instances)",
                "reference.data := ADR(instances[1])",
                "reference.table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn both_arrays() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        references: ARRAY[1..2] OF IA;
                        instances: ARRAY[1..2] OF FbA;
                    END_VAR

                    references[1] := instances[2];
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_references__ctor(references)",
                "__main_instances__ctor(instances)",
                "references[1].data := ADR(instances[2])",
                "references[1].table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn qualified_and_array() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK ContainerA
                    VAR
                        references: ARRAY[1..2] OF IA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK ContainerB
                    VAR
                        instances: ARRAY[1..2] OF FbA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        fb: ContainerA;
                        otherFb: ContainerB;
                    END_VAR

                    fb.references[1] := otherFb.instances[2];
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "ContainerA__ctor(fb)",
                "ContainerB__ctor(otherFb)",
                "fb.references[1].data := ADR(otherFb.instances[2])",
                "fb.references[1].table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn pointer_dereference_rhs() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                        pointerToInstance: POINTER TO FbA;
                    END_VAR

                    reference := pointerToInstance^;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_pointerToInstance__ctor(pointerToInstance)",
                "reference.data := ADR(pointerToInstance^)",
                "reference.table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn function_call_rhs() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION producer : FbA
                END_FUNCTION

                FUNCTION main
                    VAR
                        reference: IA;
                    END_VAR

                    reference := producer();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "alloca __producer0: FbA;",
                "producer(__producer0);",
                "reference.data := ADR(__producer0);",
                "reference.table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn method_call_rhs() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Factory
                    METHOD producer : FbA
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                        fb: Factory;
                    END_VAR

                    reference := fb.producer();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Factory__ctor(fb)",
                "alloca __producer0: FbA;",
                "fb.producer(__producer0);",
                "reference.data := ADR(__producer0);",
                "reference.table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn multi_dimensional_array() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        references: ARRAY[1..2, 1..2] OF IA;
                        instance: FbA;
                        i, j: DINT;
                    END_VAR

                    references[i, j] := instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_references__ctor(references)",
                "FbA__ctor(instance)",
                "references[i, j].data := ADR(instance)",
                "references[i, j].table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn array_of_array() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        references: ARRAY[1..2] OF ARRAY[1..2] OF IA;
                        instance: FbA;
                        i, j: DINT;
                    END_VAR

                    references[i][j] := instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_references__ctor(references)",
                "FbA__ctor(instance)",
                "references[i][j].data := ADR(instance)",
                "references[i][j].table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn runtime_index() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION getIndex : DINT
                END_FUNCTION

                FUNCTION main
                    VAR
                        references: ARRAY[1..10] OF IA;
                        instance: FbA;
                    END_VAR

                    references[getIndex()] := instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_references__ctor(references)",
                "FbA__ctor(instance)",
                "references[getIndex()].data := ADR(instance)",
                "references[getIndex()].table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn nested_access_in_index() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    VAR
                        id: DINT;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        references: ARRAY[0..10] OF IA;
                        instances: ARRAY[0..10] OF FbA;
                        instance: FbA;
                    END_VAR

                    references[instances[0].id] := instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_references__ctor(references)",
                "__main_instances__ctor(instances)",
                "FbA__ctor(instance)",
                "references[instances[0].id].data := ADR(instance)",
                "references[instances[0].id].table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }
    }

    mod arguments {
        #[test]
        fn simple() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                    END_VAR

                    consumer(instance);
                    consumer(in1 := instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn mixed_args() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        x: DINT;
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                    END_VAR

                    consumer(42, instance);
                    consumer(x := 42, in1 := instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(42, __fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(x := 42, in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn multiple_interface_args() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                        in2: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instanceA: FbA;
                        instanceB: FbA;
                    END_VAR

                    consumer(instanceA, instanceB);
                    consumer(in1 := instanceA, in2 := instanceB);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instanceA)",
                "FbA__ctor(instanceB)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instanceA)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instanceB)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0, __fatpointer_1)",
                "alloca __fatpointer_2: __FATPOINTER",
                "__fatpointer_2.data := ADR(instanceA)",
                "__fatpointer_2.table := ADR(__itable_IA_FbA_instance)",
                "alloca __fatpointer_3: __FATPOINTER",
                "__fatpointer_3.data := ADR(instanceB)",
                "__fatpointer_3.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_2, in2 := __fatpointer_3)",
            ]
            "#);
        }

        #[test]
        fn different_interfaces() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                INTERFACE IB
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbB IMPLEMENTS IB
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                        in2: IB;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instanceA: FbA;
                        instanceB: FbB;
                    END_VAR

                    consumer(instanceA, instanceB);
                    consumer(in1 := instanceA, in2 := instanceB);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instanceA)",
                "FbB__ctor(instanceB)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instanceA)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instanceB)",
                "__fatpointer_1.table := ADR(__itable_IB_FbB_instance)",
                "consumer(__fatpointer_0, __fatpointer_1)",
                "alloca __fatpointer_2: __FATPOINTER",
                "__fatpointer_2.data := ADR(instanceA)",
                "__fatpointer_2.table := ADR(__itable_IA_FbA_instance)",
                "alloca __fatpointer_3: __FATPOINTER",
                "__fatpointer_3.data := ADR(instanceB)",
                "__fatpointer_3.table := ADR(__itable_IB_FbB_instance)",
                "consumer(in1 := __fatpointer_2, in2 := __fatpointer_3)",
            ]
            "#);
        }

        #[test]
        fn qualified_arg() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Container
                    VAR
                        instance: FbA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        fb: Container;
                    END_VAR

                    consumer(fb.instance);
                    consumer(in1 := fb.instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Container__ctor(fb)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(fb.instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(fb.instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn array_arg() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instances: ARRAY[1..2] OF FbA;
                    END_VAR

                    consumer(instances[1]);
                    consumer(in1 := instances[1]);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_instances__ctor(instances)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instances[1])",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instances[1])",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn qualified_and_array_arg() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Container
                    VAR
                        instances: ARRAY[1..2] OF FbA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        fb: Container;
                        i: DINT;
                    END_VAR

                    consumer(fb.instances[i]);
                    consumer(in1 := fb.instances[i]);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Container__ctor(fb)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(fb.instances[i])",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(fb.instances[i])",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn method_consumer() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Processor
                    METHOD consume
                        VAR_INPUT
                            in1: IA;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        instance: FbA;
                        fb: Processor;
                    END_VAR

                    fb.consume(instance);
                    fb.consume(in1 := instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "Processor__ctor(fb)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "fb.consume(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "fb.consume(in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn multiple_calls() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION consumer2
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                    END_VAR

                    consumer(instance);
                    consumer2(instance);
                    consumer(in1 := instance);
                    consumer2(in1 := instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer2(__fatpointer_1)",
                "alloca __fatpointer_2: __FATPOINTER",
                "__fatpointer_2.data := ADR(instance)",
                "__fatpointer_2.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_2)",
                "alloca __fatpointer_3: __FATPOINTER",
                "__fatpointer_3.data := ADR(instance)",
                "__fatpointer_3.table := ADR(__itable_IA_FbA_instance)",
                "consumer2(in1 := __fatpointer_3)",
            ]
            "#);
        }

        #[test]
        fn no_wrapping_needed() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        reference: IA;
                    END_VAR

                    consumer(reference);
                    consumer(in1 := reference);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "consumer(reference)",
                "consumer(in1 := reference)",
            ]
            "#);
        }

        #[test]
        fn pointer_deref_arg() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        ptr: POINTER TO FbA;
                    END_VAR

                    consumer(ptr^);
                    consumer(in1 := ptr^);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_ptr__ctor(ptr)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(ptr^)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(ptr^)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn function_call_arg() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION producer : FbA
                END_FUNCTION

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    consumer(producer());
                    consumer(in1 := producer());
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "alloca __fatpointer_0: __FATPOINTER",
                "alloca __producer0: FbA;",
                "producer(__producer0);",
                "__fatpointer_0.data := ADR(__producer0);",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "alloca __producer1: FbA;",
                "producer(__producer1);",
                "__fatpointer_1.data := ADR(__producer1);",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn method_call_arg() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Factory
                    METHOD produce : FbA
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        fb: Factory;
                    END_VAR

                    consumer(fb.produce());
                    consumer(in1 := fb.produce());
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Factory__ctor(fb)",
                "alloca __fatpointer_0: __FATPOINTER",
                "alloca __produce0: FbA;",
                "fb.produce(__produce0);",
                "__fatpointer_0.data := ADR(__produce0);",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "alloca __produce1: FbA;",
                "fb.produce(__produce1);",
                "__fatpointer_1.data := ADR(__produce1);",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn nested_call() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION inner : IA
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION outer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                    END_VAR

                    outer(inner(instance));
                    outer(in1 := inner(in1 := instance));
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "alloca __inner0: __FATPOINTER;",
                "inner(__inner0, __fatpointer_0);",
                "outer(__inner0);",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "alloca __inner1: __FATPOINTER;",
                "inner(inner := __inner1, in1 := __fatpointer_1);",
                "outer(in1 := __inner1);",
            ]
            "#);
        }

        #[test]
        fn aggregate_return_consumer() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer : STRING
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                        result: STRING;
                    END_VAR

                    result := consumer(instance);
                    result := consumer(in1 := instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "alloca __consumer0: STRING;",
                "consumer(__consumer0, __fatpointer_0);",
                "result := __consumer0;",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "alloca __consumer1: STRING;",
                "consumer(consumer := __consumer1, in1 := __fatpointer_1);",
                "result := __consumer1;",
            ]
            "#);
        }

        /// Tests all three parameter directions (input, output, in-out) with both positional
        /// and named call syntax. Verifies that:
        /// - VAR_INPUT arguments wrapping works for concrete POUs passed where interface is expected
        /// - VAR_OUTPUT arguments (`=>`) pass interface-typed variables through unchanged
        /// - VAR_IN_OUT arguments pass interface-typed variables through unchanged
        #[test]
        fn all_parameter_directions() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR

                    VAR_OUTPUT
                        out1: IA;
                    END_VAR

                    VAR_IN_OUT
                        inout1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                        refOut: IA;
                        refInout: IA;
                    END_VAR

                    // Positional: only the concrete POU (input) needs wrapping;
                    // the interface-typed variables (output, inout) pass through unchanged.
                    consumer(instance, refOut, refInout);

                    // Named input: concrete POU wrapped in fat pointer.
                    consumer(in1 := instance);

                    // Named output: interface ref, no wrapping needed.
                    consumer(out1 => refOut);

                    // Named in-out: interface ref, no wrapping needed.
                    consumer(inout1 := refInout);

                    // All named together: only the input gets wrapped.
                    consumer(in1 := instance, out1 => refOut, inout1 := refInout);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0, refOut, refInout)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_1)",
                "consumer(out1 => refOut)",
                "consumer(inout1 := refInout)",
                "alloca __fatpointer_2: __FATPOINTER",
                "__fatpointer_2.data := ADR(instance)",
                "__fatpointer_2.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_2, out1 => refOut, inout1 := refInout)",
            ]
            "#);
        }

        /// A concrete POU passed to a VAR_OUTPUT or VAR_IN_OUT interface parameter is wrapped in a fat
        /// pointer, exactly like a VAR_INPUT argument. The callee may keep the reference, so the fat
        /// pointer is a global of the unit rather than a statement-scoped allocation.
        #[test]
        fn concrete_pou_as_output_and_inout_argument() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_OUTPUT
                        out1: IA;
                    END_VAR

                    VAR_IN_OUT
                        inout1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instanceOut: FbA;
                        instanceInout: FbA;
                    END_VAR

                    consumer(instanceOut, instanceInout);
                    consumer(out1 => instanceOut, inout1 := instanceInout);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instanceOut)",
                "FbA__ctor(instanceInout)",
                "__fatpointer_4main_0.data := ADR(instanceOut)",
                "__fatpointer_4main_0.table := ADR(__itable_IA_FbA_instance)",
                "__fatpointer_4main_1.data := ADR(instanceInout)",
                "__fatpointer_4main_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_4main_0, __fatpointer_4main_1)",
                "__fatpointer_4main_2.data := ADR(instanceOut)",
                "__fatpointer_4main_2.table := ADR(__itable_IA_FbA_instance)",
                "__fatpointer_4main_3.data := ADR(instanceInout)",
                "__fatpointer_4main_3.table := ADR(__itable_IA_FbA_instance)",
                "consumer(out1 => __fatpointer_4main_2, inout1 := __fatpointer_4main_3)",
            ]
            "#);
        }

        /// A concrete POU passed to a `REFERENCE TO` interface parameter is wrapped in a fat pointer,
        /// which the callee then references. The callee may save that reference, so the fat pointer is
        /// a global of the unit rather than a statement-scoped allocation.
        #[test]
        fn concrete_pou_as_reference_to_argument() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: REFERENCE TO IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                        reference: IA;
                    END_VAR

                    consumer(instance);
                    consumer(in1 := instance);
                    consumer(reference);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "__fatpointer_4main_0.data := ADR(instance)",
                "__fatpointer_4main_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_4main_0)",
                "__fatpointer_4main_1.data := ADR(instance)",
                "__fatpointer_4main_1.table := ADR(__itable_IA_FbA_instance)",
                "consumer(in1 := __fatpointer_4main_1)",
                "consumer(reference)",
            ]
            "#);
        }

        /// A variable declared with an alias of an interface passes through without wrapping. A local
        /// variable is annotated with the interface itself, a by-reference parameter with the raw alias.
        #[test]
        fn aliased_interface_argument_passes_through() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                TYPE AliasIA : IA; END_TYPE

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR

                    VAR_IN_OUT
                        inout1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        reference: AliasIA;
                    END_VAR

                    consumer(reference, reference);
                    consumer(in1 := reference, inout1 := reference);
                END_FUNCTION

                FUNCTION forward
                    VAR_IN_OUT
                        source: AliasIA;
                    END_VAR

                    consumer(source, source);
                    consumer(in1 := source, inout1 := source);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main", "forward"]), @r#"
            [
                "// Statements in main",
                "consumer(reference, reference)",
                "consumer(in1 := reference, inout1 := reference)",
                "// Statements in forward",
                "consumer(source, source)",
                "consumer(in1 := source, inout1 := source)",
            ]
            "#);
        }

        /// Surplus positional arguments bind to a sized variadic interface parameter: a concrete POU
        /// is wrapped and a child interface is upcast, exactly like for a fixed parameter.
        #[test]
        fn variadic_interface_arguments() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                INTERFACE IB EXTENDS IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        first: IA;
                        rest: {sized} IA...;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                        refIA: IA;
                        refIB: IB;
                    END_VAR

                    consumer(refIA, instance, refIB, refIA);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := refIB.data",
                "__fatpointer_1.table := __itable_IB#(refIB.table^).__upcast_IA",
                "consumer(refIA, __fatpointer_0, __fatpointer_1, refIA)",
            ]
            "#);
        }

        /// A stateful POU keeps a `VAR_IN_OUT`, `VAR_INPUT {ref}` or `REFERENCE TO` parameter between
        /// calls, and any POU may save or forward such a parameter, so the fat pointer wrapping a
        /// concrete instance becomes a global of the unit named after the binding implementation. Only a
        /// by-value input, which copies the fat pointer, keeps the statement-scoped allocation.
        #[test]
        fn retained_arguments_use_global_fat_pointers() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbConsumer
                    VAR_INPUT
                        ref1: REFERENCE TO IA;
                    END_VAR
                    VAR_INPUT {ref}
                        in1: IA;
                    END_VAR
                    VAR_IN_OUT
                        inout1: IA;
                    END_VAR

                    METHOD bind
                        VAR_INPUT
                            other: REFERENCE TO IA;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                PROGRAM progConsumer
                    VAR_IN_OUT
                        inout1: IA;
                    END_VAR
                END_PROGRAM

                FUNCTION consumer
                    VAR_INPUT
                        ref1: REFERENCE TO IA;
                        in1: IA;
                    END_VAR
                    VAR_IN_OUT
                        inout1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                        fbConsumer: FbConsumer;
                    END_VAR

                    fbConsumer(instance, instance, instance);
                    fbConsumer(ref1 := instance, in1 := instance, inout1 := instance);
                    progConsumer(instance);
                    consumer(instance, instance, instance);
                    fbConsumer.bind(instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "FbConsumer__ctor(fbConsumer)",
                "__fatpointer_4main_0.data := ADR(instance)",
                "__fatpointer_4main_0.table := ADR(__itable_IA_FbA_instance)",
                "__fatpointer_4main_1.data := ADR(instance)",
                "__fatpointer_4main_1.table := ADR(__itable_IA_FbA_instance)",
                "__fatpointer_4main_2.data := ADR(instance)",
                "__fatpointer_4main_2.table := ADR(__itable_IA_FbA_instance)",
                "fbConsumer(__fatpointer_4main_0, __fatpointer_4main_1, __fatpointer_4main_2)",
                "__fatpointer_4main_3.data := ADR(instance)",
                "__fatpointer_4main_3.table := ADR(__itable_IA_FbA_instance)",
                "__fatpointer_4main_4.data := ADR(instance)",
                "__fatpointer_4main_4.table := ADR(__itable_IA_FbA_instance)",
                "__fatpointer_4main_5.data := ADR(instance)",
                "__fatpointer_4main_5.table := ADR(__itable_IA_FbA_instance)",
                "fbConsumer(ref1 := __fatpointer_4main_3, in1 := __fatpointer_4main_4, inout1 := __fatpointer_4main_5)",
                "__fatpointer_4main_6.data := ADR(instance)",
                "__fatpointer_4main_6.table := ADR(__itable_IA_FbA_instance)",
                "progConsumer(__fatpointer_4main_6)",
                "__fatpointer_4main_7.data := ADR(instance)",
                "__fatpointer_4main_7.table := ADR(__itable_IA_FbA_instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "__fatpointer_4main_8.data := ADR(instance)",
                "__fatpointer_4main_8.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_4main_7, __fatpointer_0, __fatpointer_4main_8)",
                "__fatpointer_4main_9.data := ADR(instance)",
                "__fatpointer_4main_9.table := ADR(__itable_IA_FbA_instance)",
                "fbConsumer.bind(__fatpointer_4main_9)",
            ]
            "#);
            insta::assert_debug_snapshot!(super::lower_and_serialize_globals(source), @r#"
            [
                "__vtable_FbA_instance : __vtable_FbA",
                "__vtable_FbConsumer_instance : __vtable_FbConsumer",
                "__itable_IA_FbA_instance : __itable_IA",
                "__fatpointer_4main_0 : __FATPOINTER",
                "__fatpointer_4main_1 : __FATPOINTER",
                "__fatpointer_4main_2 : __FATPOINTER",
                "__fatpointer_4main_3 : __FATPOINTER",
                "__fatpointer_4main_4 : __FATPOINTER",
                "__fatpointer_4main_5 : __FATPOINTER",
                "__fatpointer_4main_6 : __FATPOINTER",
                "__fatpointer_4main_7 : __FATPOINTER",
                "__fatpointer_4main_8 : __FATPOINTER",
                "__fatpointer_4main_9 : __FATPOINTER",
            ]
            "#);
        }

        /// Generic templates that only differ in their generics share a name. Their retained globals carry
        /// the generic signature and are only declared, because a template emits no code and separately
        /// compiled libraries may contain the same template.
        #[test]
        fn retained_arguments_in_generic_templates() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbConsumer
                    VAR_INPUT
                        ref1: REFERENCE TO IA;
                    END_VAR
                END_FUNCTION_BLOCK

                VAR_GLOBAL
                    fbConsumer: FbConsumer;
                    instance: FbA;
                END_VAR

                FUNCTION bind<T: ANY_INT> : T
                    VAR_INPUT
                        value: T;
                    END_VAR

                    fbConsumer(instance);
                    bind := value;
                END_FUNCTION

                FUNCTION bind<U: ANY_REAL> : U
                    VAR_INPUT
                        value: U;
                    END_VAR

                    fbConsumer(instance);
                    bind := value;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_globals(source), @r#"
            [
                "fbConsumer : FbConsumer",
                "instance : FbA",
                "__vtable_FbA_instance : __vtable_FbA",
                "__vtable_FbConsumer_instance : __vtable_FbConsumer",
                "__itable_IA_FbA_instance : __itable_IA",
                "__fatpointer_4bind_7ANY_INT_0 : __FATPOINTER {external}",
                "__fatpointer_4bind_8ANY_REAL_0 : __FATPOINTER {external}",
            ]
            "#);
        }

        /// An implementation with external linkage keeps its body, but codegen omits it, so the retained
        /// fat pointers it binds are only declared and the object defining the implementation defines them.
        #[test]
        fn retained_arguments_in_external_implementation_are_declared() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                {external}
                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                {external}
                FUNCTION_BLOCK FbConsumer
                    VAR_INPUT
                        ref1: REFERENCE TO IA;
                    END_VAR
                END_FUNCTION_BLOCK

                {external}
                FUNCTION bindExt
                    VAR_IN_OUT
                        consumer: FbConsumer;
                        instance: FbA;
                    END_VAR

                    consumer(instance);
                END_FUNCTION

                FUNCTION bind
                    VAR_IN_OUT
                        consumer: FbConsumer;
                        instance: FbA;
                    END_VAR

                    consumer(instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_globals(source), @r#"
            [
                "__vtable_FbA_instance : __vtable_FbA {external}",
                "__vtable_FbConsumer_instance : __vtable_FbConsumer {external}",
                "__itable_IA_FbA_instance : __itable_IA {external}",
                "__fatpointer_4bind_0 : __FATPOINTER",
                "__fatpointer_7bindExt_0 : __FATPOINTER {external}",
            ]
            "#);
        }

        /// An omitted interface output is not wrapped; the callee resets it and the call discards it.
        #[test]
        fn omitted_output_argument_is_not_wrapped() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION consumer
                    VAR_OUTPUT
                        out1: IA;
                        out2: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        refIA: IA;
                    END_VAR

                    consumer(out1 =>, out2 =>);
                    consumer(out1 => refIA, out2 =>);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "consumer(out1 => , out2 => )",
                "consumer(out1 => refIA, out2 => )",
            ]
            "#);
        }

        /// The arguments of a generic builtin bind to no indexed parameter and keep the type the resolver
        /// hinted for them. The resolver types a builtin over interfaces as REAL, so no interface hint
        /// exists, nothing is wrapped, and the assignment of the result cannot be lowered.
        #[test]
        fn generic_builtin_arguments_keep_their_hint() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                INTERFACE IB EXTENDS IA
                END_INTERFACE

                FUNCTION main
                    VAR
                        refIA: IA;
                        refIB: IB;
                        selected: IA;
                    END_VAR

                    selected := SEL(FALSE, refIB, refIA);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "selected.data := ADR(SEL(FALSE, refIB, refIA))",
                "selected.table := ADR(__itable_IA_REAL_instance)",
            ]
            "#);
        }

        /// A call result passed to a by-reference interface parameter is rejected with E031, as for any
        /// other by-reference parameter, and replaced by an unfilled dummy. Passed by value it is wrapped.
        #[test]
        fn call_result_for_byref_parameter_is_rejected() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION producer : FbA
                END_FUNCTION

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                    VAR_IN_OUT
                        inout1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                    END_VAR

                    consumer(producer(), instance);
                    consumer(instance, producer());
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "alloca __producer0: FbA;",
                "producer(__producer0);",
                "__fatpointer_0.data := ADR(__producer0);",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "__fatpointer_4main_0.data := ADR(instance)",
                "__fatpointer_4main_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0, __fatpointer_4main_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "alloca __fatpointer_2: __FATPOINTER",
                "consumer(__fatpointer_1, __fatpointer_2)",
            ]
            "#);
        }

        /// A short-circuit operand whose call needs a preamble is lowered into an IF, so a skipped call
        /// does not rebind the retained fat pointer of its site.
        #[test]
        fn short_circuit_operand_runs_preamble_only_when_evaluated() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Holder
                    METHOD bind : BOOL
                        VAR_INPUT
                            refIn : REFERENCE TO IA;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main : BOOL
                    VAR
                        enabled : BOOL;
                        holder : Holder;
                        instance : FbA;
                    END_VAR

                    main := enabled AND_THEN holder.bind(instance);
                    main := enabled OR_ELSE holder.bind(instance);
                    main := holder.bind(instance) AND_THEN enabled;
                    IF enabled AND_THEN holder.bind(instance) THEN
                        main := TRUE;
                    END_IF
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Holder__ctor(holder)",
                "FbA__ctor(instance)",
                "IF enabled THEN",
                "    __fatpointer_4main_0.data := ADR(instance)",
                "    __fatpointer_4main_0.table := ADR(__itable_IA_FbA_instance)",
                "    __shortcircuit_0 := holder.bind(__fatpointer_4main_0)",
                "ELSE",
                "    __shortcircuit_0 := FALSE",
                "END_IF",
                "main := __shortcircuit_0",
                "IF enabled THEN",
                "    __shortcircuit_1 := TRUE",
                "ELSE",
                "    __fatpointer_4main_1.data := ADR(instance)",
                "    __fatpointer_4main_1.table := ADR(__itable_IA_FbA_instance)",
                "    __shortcircuit_1 := holder.bind(__fatpointer_4main_1)",
                "END_IF",
                "main := __shortcircuit_1",
                "__fatpointer_4main_2.data := ADR(instance)",
                "__fatpointer_4main_2.table := ADR(__itable_IA_FbA_instance)",
                "main := holder.bind(__fatpointer_4main_2) AND_THEN enabled",
                "IF enabled THEN",
                "    __fatpointer_4main_3.data := ADR(instance)",
                "    __fatpointer_4main_3.table := ADR(__itable_IA_FbA_instance)",
                "    __shortcircuit_2 := holder.bind(__fatpointer_4main_3)",
                "ELSE",
                "    __shortcircuit_2 := FALSE",
                "END_IF",
                "IF __shortcircuit_2 THEN",
                "    main := TRUE",
                "END_IF",
            ]
            "#);
        }

        /// The IF a short-circuit operand is lowered into is hoisted before the statement. Where the
        /// short-circuit is not the first thing the statement evaluates, including the value of an
        /// assignment whose target address must be computed first, that would reorder side effects,
        /// so the operand keeps the eager preamble; the same holds for an operand that is not a BOOL,
        /// which validation must still reject.
        #[test]
        fn short_circuit_operand_keeps_eager_preamble_outside_leading_position() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Holder
                    METHOD bind : BOOL
                        VAR_INPUT
                            refIn : REFERENCE TO IA;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION enable : BOOL
                END_FUNCTION

                FUNCTION consume : BOOL
                    VAR_INPUT
                        flag : BOOL;
                    END_VAR
                END_FUNCTION

                FUNCTION count : DINT
                    VAR_INPUT
                        refIn : REFERENCE TO IA;
                    END_VAR
                END_FUNCTION

                FUNCTION values : ARRAY[0..1] OF BOOL
                END_FUNCTION

                FUNCTION main : BOOL
                    VAR
                        enabled : BOOL;
                        holder : Holder;
                        instance : FbA;
                        values : ARRAY[0..1] OF BOOL;
                        matrix : ARRAY[0..1, 0..1] OF BOOL;
                        target : REFERENCE TO BOOL;
                        ptrs : ARRAY[0..1] OF REFERENCE TO BOOL;
                        i : DINT;
                    END_VAR

                    main := enable() AND (enabled AND_THEN holder.bind(instance));
                    main := consume(enabled AND_THEN holder.bind(instance));
                    main := 1 AND_THEN holder.bind(instance);
                    main := enabled AND_THEN count(instance);
                    values[i] := enabled AND_THEN holder.bind(instance);
                    main := values()[enabled AND_THEN holder.bind(instance)];
                    main := matrix[i, enabled AND_THEN holder.bind(instance)];
                    target := enabled AND_THEN holder.bind(instance);
                    ptrs[i] REF= values[enabled AND_THEN holder.bind(instance)];
                    target REF= enabled AND_THEN holder.bind(instance);
                    IF enable() THEN
                    ELSIF enabled AND_THEN holder.bind(instance) THEN
                    END_IF
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Holder__ctor(holder)",
                "FbA__ctor(instance)",
                "__main_values__ctor(values)",
                "__main_matrix__ctor(matrix)",
                "__main_target__ctor(target)",
                "__main_ptrs__ctor(ptrs)",
                "__fatpointer_4main_0.data := ADR(instance)",
                "__fatpointer_4main_0.table := ADR(__itable_IA_FbA_instance)",
                "main := enable() AND (enabled AND_THEN holder.bind(__fatpointer_4main_0))",
                "__fatpointer_4main_1.data := ADR(instance)",
                "__fatpointer_4main_1.table := ADR(__itable_IA_FbA_instance)",
                "main := consume(enabled AND_THEN holder.bind(__fatpointer_4main_1))",
                "__fatpointer_4main_2.data := ADR(instance)",
                "__fatpointer_4main_2.table := ADR(__itable_IA_FbA_instance)",
                "main := 1 AND_THEN holder.bind(__fatpointer_4main_2)",
                "__fatpointer_4main_3.data := ADR(instance)",
                "__fatpointer_4main_3.table := ADR(__itable_IA_FbA_instance)",
                "main := enabled AND_THEN count(__fatpointer_4main_3)",
                "__fatpointer_4main_4.data := ADR(instance)",
                "__fatpointer_4main_4.table := ADR(__itable_IA_FbA_instance)",
                "values[i] := enabled AND_THEN holder.bind(__fatpointer_4main_4)",
                "__fatpointer_4main_5.data := ADR(instance)",
                "__fatpointer_4main_5.table := ADR(__itable_IA_FbA_instance)",
                "alloca __values0: __values_return;",
                "values(__values0);",
                "main := __values0[enabled AND_THEN holder.bind(__fatpointer_4main_5)];",
                "__fatpointer_4main_6.data := ADR(instance)",
                "__fatpointer_4main_6.table := ADR(__itable_IA_FbA_instance)",
                "main := matrix[i, enabled AND_THEN holder.bind(__fatpointer_4main_6)]",
                "__fatpointer_4main_7.data := ADR(instance)",
                "__fatpointer_4main_7.table := ADR(__itable_IA_FbA_instance)",
                "target := enabled AND_THEN holder.bind(__fatpointer_4main_7)",
                "__fatpointer_4main_8.data := ADR(instance)",
                "__fatpointer_4main_8.table := ADR(__itable_IA_FbA_instance)",
                "ptrs[i] REF= values[enabled AND_THEN holder.bind(__fatpointer_4main_8)]",
                "__fatpointer_4main_9.data := ADR(instance)",
                "__fatpointer_4main_9.table := ADR(__itable_IA_FbA_instance)",
                "target REF= enabled AND_THEN holder.bind(__fatpointer_4main_9)",
                "IF enable() THEN",
                "ELSE",
                "    IF enabled THEN",
                "        __fatpointer_4main_10.data := ADR(instance)",
                "        __fatpointer_4main_10.table := ADR(__itable_IA_FbA_instance)",
                "        __shortcircuit_0 := holder.bind(__fatpointer_4main_10)",
                "    ELSE",
                "        __shortcircuit_0 := FALSE",
                "    END_IF",
                "    IF __shortcircuit_0 THEN",
                "    END_IF",
                "END_IF",
            ]
            "#);
        }

        /// A generic template keeps the eager preamble: overloaded templates share one declaration for
        /// member lookup, so a `VAR_TEMP` added to one of them is not found from the other.
        #[test]
        fn short_circuit_operand_in_generic_template_keeps_eager_preamble() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consume : BOOL
                    VAR_INPUT
                        in1 : IA;
                    END_VAR
                END_FUNCTION

                VAR_GLOBAL
                    enabled : BOOL;
                    instance : FbA;
                END_VAR

                FUNCTION bind<T: ANY_INT> : BOOL
                    VAR_INPUT
                        value : T;
                    END_VAR

                    bind := enabled AND_THEN consume(instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["bind"]), @r#"
            [
                "// Statements in bind",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "bind := enabled AND_THEN consume(__fatpointer_0)",
            ]
            "#);
        }
    }

    mod calls {
        #[test]
        fn no_args() {
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
                        reference: IA;
                    END_VAR

                    reference.foo();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).foo^(reference.data^)",
            ]
            "#);
        }

        #[test]
        fn scalar_arg() {
            let source = r#"
                INTERFACE IA
                    METHOD foo
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                    END_VAR

                    reference.foo(42);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).foo^(reference.data^, 42)",
            ]
            "#);
        }

        #[test]
        fn multiple_args() {
            let source = r#"
                INTERFACE IA
                    METHOD foo
                        VAR_INPUT
                            x: DINT;
                            y: BOOL;
                        END_VAR
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo
                        VAR_INPUT
                            x: DINT;
                            y: BOOL;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                    END_VAR

                    reference.foo(42, TRUE);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).foo^(reference.data^, 42, TRUE)",
            ]
            "#);
        }

        #[test]
        fn return_value() {
            let source = r#"
                INTERFACE IA
                    METHOD foo : DINT
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo : DINT
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                        result: DINT;
                    END_VAR

                    result := reference.foo();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "result := __itable_IA#(reference.table^).foo^(reference.data^)",
            ]
            "#);
        }

        #[test]
        fn return_value_in_expression() {
            let source = r#"
                INTERFACE IA
                    METHOD foo : DINT
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo : DINT
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                        result: DINT;
                    END_VAR

                    result := reference.foo() + 1;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "result := __itable_IA#(reference.table^).foo^(reference.data^) + 1",
            ]
            "#);
        }

        #[test]
        fn qualified_base() {
            let source = r#"
                INTERFACE IA
                    METHOD foo
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Container
                    VAR
                        reference: IA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        fb: Container;
                    END_VAR

                    fb.reference.foo();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Container__ctor(fb)",
                "__itable_IA#(fb.reference.table^).foo^(fb.reference.data^)",
            ]
            "#);
        }

        #[test]
        fn array_base() {
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
                        references: ARRAY[0..10] OF IA;
                        i: DINT;
                    END_VAR

                    references[i].foo();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_references__ctor(references)",
                "__itable_IA#(references[i].table^).foo^(references[i].data^)",
            ]
            "#);
        }

        #[test]
        fn qualified_and_array_base() {
            let source = r#"
                INTERFACE IA
                    METHOD foo
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Container
                    VAR
                        references: ARRAY[0..10] OF IA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        fb: Container;
                        i: DINT;
                    END_VAR

                    fb.references[i].foo();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Container__ctor(fb)",
                "__itable_IA#(fb.references[i].table^).foo^(fb.references[i].data^)",
            ]
            "#);
        }

        #[test]
        fn multiple_methods() {
            let source = r#"
                INTERFACE IA
                    METHOD foo
                    END_METHOD
                    METHOD bar
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo
                    END_METHOD
                    METHOD bar
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                    END_VAR

                    reference.foo();
                    reference.bar();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).foo^(reference.data^)",
                "__itable_IA#(reference.table^).bar^(reference.data^)",
            ]
            "#);
        }

        #[test]
        fn positional_and_named() {
            let source = r#"
                INTERFACE IA
                    METHOD foo
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                    END_VAR

                    reference.foo(42);
                    reference.foo(x := 42);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).foo^(reference.data^, 42)",
                "__itable_IA#(reference.table^).foo^(reference.data^, x := 42)",
            ]
            "#);
        }

        #[test]
        fn arg_needs_wrapping() {
            let source = r#"
                INTERFACE IA
                    METHOD foo
                        VAR_INPUT
                            in1: IA;
                        END_VAR
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo
                        VAR_INPUT
                            in1: IA;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                        instance: FbA;
                    END_VAR

                    reference.foo(instance);
                    reference.foo(in1 := instance);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "__itable_IA#(reference.table^).foo^(reference.data^, __fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "__itable_IA#(reference.table^).foo^(reference.data^, in1 := __fatpointer_1)",
            ]
            "#);
        }

        #[test]
        fn nested_interface_call_as_arg() {
            let source = r#"
                INTERFACE IA
                    METHOD foo : DINT
                    END_METHOD
                    METHOD bar
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo : DINT
                    END_METHOD
                    METHOD bar
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        a: IA;
                        b: IA;
                    END_VAR

                    a.bar(b.foo());
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(a.table^).bar^(a.data^, __itable_IA#(b.table^).foo^(b.data^))",
            ]
            "#);
        }

        #[test]
        fn different_interfaces() {
            let source = r#"
                INTERFACE IA
                    METHOD foo
                    END_METHOD
                END_INTERFACE

                INTERFACE IB
                    METHOD bar
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbB IMPLEMENTS IB
                    METHOD bar
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        refA: IA;
                        refB: IB;
                    END_VAR

                    refA.foo();
                    refB.bar();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(refA.table^).foo^(refA.data^)",
                "__itable_IB#(refB.table^).bar^(refB.data^)",
            ]
            "#);
        }

        #[test]
        fn method_returns_concrete_assigned_to_interface() {
            let source = r#"
                INTERFACE IA
                    METHOD produce : DINT
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD produce : DINT
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference: IA;
                        result: DINT;
                    END_VAR

                    result := reference.produce();
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "result := __itable_IA#(reference.table^).produce^(reference.data^)",
            ]
            "#);
        }

        #[test]
        fn three_levels_nested() {
            let source = r#"
                INTERFACE IA
                    METHOD foo : DINT
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo : DINT
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        a: IA;
                        b: IA;
                        c: IA;
                    END_VAR

                    a.foo(b.foo(c.foo(42)));
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(a.table^).foo^(a.data^, __itable_IA#(b.table^).foo^(b.data^, __itable_IA#(c.table^).foo^(c.data^, 42)))",
            ]
            "#);
        }

        #[test]
        fn nested_with_wrapping_and_return() {
            let source = r#"
                INTERFACE IA
                    METHOD foo : DINT
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                    METHOD bar : DINT
                        VAR_INPUT
                            in1: IA;
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
                        VAR_INPUT
                            in1: IA;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        a: IA;
                        b: IA;
                        instance: FbA;
                        result: DINT;
                    END_VAR

                    result := a.foo(b.bar(instance));
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "alloca __fatpointer_0: __FATPOINTER",
                "__fatpointer_0.data := ADR(instance)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "result := __itable_IA#(a.table^).foo^(a.data^, __itable_IA#(b.table^).bar^(b.data^, __fatpointer_0))",
            ]
            "#);
        }

        #[test]
        fn deep_nesting_with_qualified_and_array() {
            let source = r#"
                INTERFACE IA
                    METHOD foo : DINT
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo : DINT
                        VAR_INPUT
                            x: DINT;
                        END_VAR
                    END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Container
                    VAR
                        references: ARRAY[0..10] OF IA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        a: Container;
                        b: Container;
                        i, j: DINT;
                    END_VAR

                    a.references[i].foo(b.references[j].foo(42));
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Container__ctor(a)",
                "Container__ctor(b)",
                "__itable_IA#(a.references[i].table^).foo^(a.references[i].data^, __itable_IA#(b.references[j].table^).foo^(b.references[j].data^, 42))",
            ]
            "#);
        }
    }

    /// Tests for interface dispatch lowering inside control flow bodies (IF, CASE, FOR, etc.).
    mod control {
        #[test]
        fn assignment_inside_if_branches() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbAlpha IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbBravo IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        selector: DINT;
                        alpha: FbAlpha;
                        bravo: FbBravo;
                        reference: IA;
                    END_VAR

                    IF selector = 1 THEN
                        reference := alpha;
                    ELSE
                        reference := bravo;
                    END_IF;
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @"
            // Statements in main
            FbAlpha__ctor(alpha)
            FbBravo__ctor(bravo)
            IF selector = 1 THEN
                reference.data := ADR(alpha)
                reference.table := ADR(__itable_IA_FbAlpha_instance)
            ELSE
                reference.data := ADR(bravo)
                reference.table := ADR(__itable_IA_FbBravo_instance)
            END_IF
            ");
        }

        #[test]
        fn assignment_inside_case_branches() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbAlpha IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbBravo IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbCharlie IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        selector: DINT;
                        alpha: FbAlpha;
                        bravo: FbBravo;
                        charlie: FbCharlie;
                        reference: IA;
                    END_VAR

                    CASE selector OF
                        1: reference := alpha;
                        2: reference := bravo;
                    ELSE
                        reference := charlie;
                    END_CASE;
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @"
            // Statements in main
            FbAlpha__ctor(alpha)
            FbBravo__ctor(bravo)
            FbCharlie__ctor(charlie)
            CASE selector OF
                1:
                    reference.data := ADR(alpha)
                    reference.table := ADR(__itable_IA_FbAlpha_instance)
                2:
                    reference.data := ADR(bravo)
                    reference.table := ADR(__itable_IA_FbBravo_instance)
                ELSE
                    reference.data := ADR(charlie)
                    reference.table := ADR(__itable_IA_FbCharlie_instance)
            END_CASE
            ");
        }

        #[test]
        fn assignment_inside_for_loop() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        instances: ARRAY[0..2] OF FbA;
                        references: ARRAY[0..2] OF IA;
                        i: DINT;
                    END_VAR

                    FOR i := 0 TO 2 DO
                        references[i] := instances[i];
                    END_FOR;
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @r"
            // Statements in main
            __main_instances__ctor(instances)
            __main_references__ctor(references)
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            i := 0
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    i := i + 1
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF i > 2 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < 2 THEN
                        EXIT;
                    END_IF
                END_IF
                references[i].data := ADR(instances[i])
                references[i].table := ADR(__itable_IA_FbA_instance)
            END_WHILE
            ");
        }

        #[test]
        fn call_with_wrapping_inside_if() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        flag: BOOL;
                        instance: FbA;
                    END_VAR

                    IF flag THEN
                        consumer(instance);
                    END_IF;
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @r"
            // Statements in main
            FbA__ctor(instance)
            IF flag THEN
                alloca __fatpointer_0: __FATPOINTER
                __fatpointer_0.data := ADR(instance)
                __fatpointer_0.table := ADR(__itable_IA_FbA_instance)
                consumer(__fatpointer_0)
            END_IF
            ");
        }

        #[test]
        fn interface_to_interface_assignment() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        refA: IA;
                        refB: IA;
                    END_VAR

                    refB := refA;
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @"
            // Statements in main
            refB := refA
            ");
        }

        #[test]
        fn call_argument_wrapping_in_if_condition() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer : BOOL
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instance: FbA;
                        x: DINT;
                    END_VAR

                    IF consumer(instance) THEN
                        x := 1;
                    END_IF;
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @r"
            // Statements in main
            FbA__ctor(instance)
            alloca __fatpointer_0: __FATPOINTER
            __fatpointer_0.data := ADR(instance)
            __fatpointer_0.table := ADR(__itable_IA_FbA_instance)
            IF consumer(__fatpointer_0) THEN
                x := 1
            END_IF
            ");
        }

        // FIXME: A dedicated loop desugarer (running before this pass) that rewrites WHILE/FOR
        // into `WHILE TRUE DO IF NOT <cond> THEN EXIT END_IF <body> END_WHILE` would fix this:
        // the condition ends up inside the loop body where our preamble mechanism handles it
        // correctly. The AggregateLowerer already does a similar WHILE transformation, but it
        // runs later and is arguably the wrong place for structural loop rewrites.
        #[test]
        fn call_argument_wrapping_in_while_condition() {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                END_FUNCTION_BLOCK

                FUNCTION consumer : BOOL
                    VAR_INPUT
                        in1: IA;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        instances: ARRAY[0..9] OF FbA;
                        i: DINT;
                    END_VAR

                    WHILE consumer(instances[i]) DO
                        i := i + 1;
                    END_WHILE;
                END_FUNCTION
            "#;

            // The preamble must be inside the loop so `instances[i]` is re-evaluated each
            // iteration. This requires restructuring WHILE into WHILE TRUE + EXIT.
            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @"
            // Statements in main
            __main_instances__ctor(instances)
            WHILE TRUE DO
                alloca __fatpointer_0: __FATPOINTER
                __fatpointer_0.data := ADR(instances[i])
                __fatpointer_0.table := ADR(__itable_IA_FbA_instance)
                IF NOT consumer(__fatpointer_0) THEN
                    EXIT;
                END_IF
                i := i + 1
            END_WHILE
            ");
        }
    }

    mod upcast {
        mod assignments {
            #[test]
            fn simple() {
                // IB extends IA: refIA := refIB should copy .data and read .table
                // through __upcast_IA on the IB itable.
                let source = r#"
                    INTERFACE IA
                        METHOD foo END_METHOD
                    END_INTERFACE

                    INTERFACE IB EXTENDS IA
                        METHOD bar END_METHOD
                    END_INTERFACE

                    FUNCTION_BLOCK FbA IMPLEMENTS IB
                        METHOD foo END_METHOD
                        METHOD bar END_METHOD
                    END_FUNCTION_BLOCK

                    FUNCTION main
                        VAR
                            refA: IA;
                            refB: IB;
                        END_VAR

                        refA := refB;
                    END_FUNCTION
                "#;

                insta::assert_debug_snapshot!(super::super::lower_and_serialize_statements(source, &["main"]), @r#"
                [
                    "// Statements in main",
                    "refA.data := refB.data",
                    "refA.table := __itable_IB#(refB.table^).__upcast_IA",
                ]
                "#);
            }

            #[test]
            fn transitive() {
                // IC extends IB extends IA: refIA := refIC should use a single
                // __upcast_IA read (flattened), not chain through IB.
                let source = r#"
                    INTERFACE IA
                        METHOD foo END_METHOD
                    END_INTERFACE

                    INTERFACE IB EXTENDS IA
                        METHOD bar END_METHOD
                    END_INTERFACE

                    INTERFACE IC EXTENDS IB
                        METHOD baz END_METHOD
                    END_INTERFACE

                    FUNCTION_BLOCK FbA IMPLEMENTS IC
                        METHOD foo END_METHOD
                        METHOD bar END_METHOD
                        METHOD baz END_METHOD
                    END_FUNCTION_BLOCK

                    FUNCTION main
                        VAR
                            refA: IA;
                            refC: IC;
                        END_VAR

                        refA := refC;
                    END_FUNCTION
                "#;

                insta::assert_debug_snapshot!(super::super::lower_and_serialize_statements(source, &["main"]), @r#"
                [
                    "// Statements in main",
                    "refA.data := refC.data",
                    "refA.table := __itable_IC#(refC.table^).__upcast_IA",
                ]
                "#);
            }

            #[test]
            fn same_interface_unchanged() {
                // Same interface assignment should remain a plain memcpy (no upcast).
                let source = r#"
                    INTERFACE IA
                        METHOD foo END_METHOD
                    END_INTERFACE

                    FUNCTION_BLOCK FbA IMPLEMENTS IA
                        METHOD foo END_METHOD
                    END_FUNCTION_BLOCK

                    FUNCTION main
                        VAR
                            refA1: IA;
                            refA2: IA;
                        END_VAR

                        refA1 := refA2;
                    END_FUNCTION
                "#;

                insta::assert_debug_snapshot!(super::super::lower_and_serialize_statements(source, &["main"]), @r#"
                [
                    "// Statements in main",
                    "refA1 := refA2",
                ]
                "#);
            }
        }

        mod arguments {
            #[test]
            fn simple() {
                // Passing refIB where IA is expected should allocate a temp fat pointer
                // with upcasted table.
                let source = r#"
                    INTERFACE IA
                        METHOD foo END_METHOD
                    END_INTERFACE

                    INTERFACE IB EXTENDS IA
                        METHOD bar END_METHOD
                    END_INTERFACE

                    FUNCTION_BLOCK FbA IMPLEMENTS IB
                        METHOD foo END_METHOD
                        METHOD bar END_METHOD
                    END_FUNCTION_BLOCK

                    FUNCTION consumer
                        VAR_INPUT
                            in1: IA;
                        END_VAR
                    END_FUNCTION

                    FUNCTION main
                        VAR
                            refB: IB;
                        END_VAR

                        consumer(refB);
                    END_FUNCTION
                "#;

                insta::assert_debug_snapshot!(super::super::lower_and_serialize_statements(source, &["main"]), @r#"
                [
                    "// Statements in main",
                    "alloca __fatpointer_0: __FATPOINTER",
                    "__fatpointer_0.data := refB.data",
                    "__fatpointer_0.table := __itable_IB#(refB.table^).__upcast_IA",
                    "consumer(__fatpointer_0)",
                ]
                "#);
            }

            #[test]
            fn named() {
                // Named argument syntax: consumer(in1 := refIB) where in1: IA.
                let source = r#"
                    INTERFACE IA
                        METHOD foo END_METHOD
                    END_INTERFACE

                    INTERFACE IB EXTENDS IA
                        METHOD bar END_METHOD
                    END_INTERFACE

                    FUNCTION_BLOCK FbA IMPLEMENTS IB
                        METHOD foo END_METHOD
                        METHOD bar END_METHOD
                    END_FUNCTION_BLOCK

                    FUNCTION consumer
                        VAR_INPUT
                            in1: IA;
                        END_VAR
                    END_FUNCTION

                    FUNCTION main
                        VAR
                            refB: IB;
                        END_VAR

                        consumer(in1 := refB);
                    END_FUNCTION
                "#;

                insta::assert_debug_snapshot!(super::super::lower_and_serialize_statements(source, &["main"]), @r#"
                [
                    "// Statements in main",
                    "alloca __fatpointer_0: __FATPOINTER",
                    "__fatpointer_0.data := refB.data",
                    "__fatpointer_0.table := __itable_IB#(refB.table^).__upcast_IA",
                    "consumer(in1 := __fatpointer_0)",
                ]
                "#);
            }

            #[test]
            fn same_interface_no_wrap() {
                // Passing refIA where IA is expected: no wrapping needed.
                let source = r#"
                    INTERFACE IA
                        METHOD foo END_METHOD
                    END_INTERFACE

                    FUNCTION_BLOCK FbA IMPLEMENTS IA
                        METHOD foo END_METHOD
                    END_FUNCTION_BLOCK

                    FUNCTION consumer
                        VAR_INPUT
                            in1: IA;
                        END_VAR
                    END_FUNCTION

                    FUNCTION main
                        VAR
                            refA: IA;
                        END_VAR

                        consumer(refA);
                        consumer(in1 := refA);
                    END_FUNCTION
                "#;

                insta::assert_debug_snapshot!(super::super::lower_and_serialize_statements(source, &["main"]), @r#"
                [
                    "// Statements in main",
                    "consumer(refA)",
                    "consumer(in1 := refA)",
                ]
                "#);
            }
        }
    }

    mod properties {
        #[test]
        fn property_get_through_interface() {
            // A standalone property get through an interface reference.
            // Should lower to an itable indirect call to __get_foo.
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference : IA;
                    END_VAR

                    reference.foo;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).__get_foo^(reference.data^)",
            ]
            "#);
        }

        #[test]
        fn property_set_through_interface() {
            // Setting a property through an interface reference.
            // Should lower to an itable indirect call to __set_foo with the value as argument.
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                    PROPERTY_SET foo: DINT END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                    PROPERTY_SET foo: DINT END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference : IA;
                    END_VAR

                    reference.foo := 5;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).__set_foo^(reference.data^, 5)",
            ]
            "#);
        }

        #[test]
        fn property_get_in_assignment() {
            // Getting a property through an interface and assigning it to a local variable.
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference : IA;
                        result : DINT;
                    END_VAR

                    result := reference.foo;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "result := __itable_IA#(reference.table^).__get_foo^(reference.data^)",
            ]
            "#);
        }

        #[test]
        fn property_get_in_expression() {
            // Using a property getter in a binary expression.
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference : IA;
                        result : DINT;
                    END_VAR

                    result := reference.foo + 1;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "result := __itable_IA#(reference.table^).__get_foo^(reference.data^) + 1",
            ]
            "#);
        }

        #[test]
        fn property_self_assignment_through_interface() {
            // Self-assignment: reference.foo := reference.foo
            // Should produce a setter whose argument is the getter, both through the itable.
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                    PROPERTY_SET foo: DINT END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                    PROPERTY_SET foo: DINT END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference : IA;
                    END_VAR

                    reference.foo := reference.foo;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).__set_foo^(reference.data^, __itable_IA#(reference.table^).__get_foo^(reference.data^))",
            ]
            "#);
        }

        #[test]
        fn property_get_as_function_argument() {
            // Passing a property getter result as an argument to a function.
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT
                    END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION consumer
                    VAR_INPUT
                        x : DINT;
                    END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR
                        reference : IA;
                    END_VAR

                    consumer(reference.foo);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "consumer(__itable_IA#(reference.table^).__get_foo^(reference.data^))",
            ]
            "#);
        }

        #[test]
        fn property_and_method_calls_mixed() {
            // An interface has both a method and a property. Both are called through the
            // interface reference and should dispatch through the same itable.
            let source = r#"
                INTERFACE IA
                    METHOD bar : DINT END_METHOD

                    PROPERTY_GET foo: DINT
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD bar : DINT END_METHOD

                    PROPERTY_GET foo: DINT
                    END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference : IA;
                        result : DINT;
                    END_VAR

                    reference.bar();
                    result := reference.foo;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).bar^(reference.data^)",
                "result := __itable_IA#(reference.table^).__get_foo^(reference.data^)",
            ]
            "#);
        }

        #[test]
        fn property_through_qualified_interface() {
            // Accessing a property through a qualified path: container.reference.foo
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT
                    END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Container
                    VAR
                        reference : IA;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        container : Container;
                        result : DINT;
                    END_VAR

                    result := container.reference.foo;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "Container__ctor(container)",
                "result := __itable_IA#(container.reference.table^).__get_foo^(container.reference.data^)",
            ]
            "#);
        }

        #[test]
        fn property_through_array_of_interfaces() {
            // Accessing a property through an array of interface references.
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT
                    END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        references : ARRAY[0..10] OF IA;
                        i : DINT;
                        result : DINT;
                    END_VAR

                    result := references[i].foo;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__main_references__ctor(references)",
                "result := __itable_IA#(references[i].table^).__get_foo^(references[i].data^)",
            ]
            "#);
        }

        #[test]
        fn property_assignment_expands_for_interface_variable() {
            // Assigning a concrete POU instance (that has properties) to an interface variable.
            // Verifies normal fat-pointer expansion still works when the POU has properties.
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                    PROPERTY_SET foo: DINT END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                    PROPERTY_SET foo: DINT END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference : IA;
                        instance : FbA;
                    END_VAR

                    reference := instance;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instance)",
                "reference.data := ADR(instance)",
                "reference.table := ADR(__itable_IA_FbA_instance)",
            ]
            "#);
        }

        #[test]
        fn property_assignment_uses_correct_itable_for_overridden_pou() {
            // Assigning different concrete POU instances to the same interface variable.
            // FbB extends FbA and overrides the getter. Each assignment should use the
            // correct itable instance for the .table field.
            let source = r#"
                INTERFACE IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                    PROPERTY_SET foo: DINT END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY_GET foo: DINT END_PROPERTY
                    PROPERTY_SET foo: DINT END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbB EXTENDS FbA
                    PROPERTY_GET foo: DINT END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference : IA;
                        instanceA : FbA;
                        instanceB : FbB;
                    END_VAR

                    reference := instanceA;
                    reference := instanceB;
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "FbA__ctor(instanceA)",
                "FbB__ctor(instanceB)",
                "reference.data := ADR(instanceA)",
                "reference.table := ADR(__itable_IA_FbA_instance)",
                "reference.data := ADR(instanceB)",
                "reference.table := ADR(__itable_IA_FbB_instance)",
            ]
            "#);
        }

        #[test]
        fn property_get_nested_in_interface_method_call() {
            // Using a property getter as an argument to an interface method call.
            // reference.bar(reference.foo) where bar is a method and foo is a property.
            let source = r#"
                INTERFACE IA
                    METHOD bar
                        VAR_INPUT
                            x : DINT;
                        END_VAR
                    END_METHOD

                    PROPERTY_GET foo: DINT END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD bar
                        VAR_INPUT
                            x : DINT;
                        END_VAR
                    END_METHOD

                    PROPERTY_GET foo: DINT END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR
                        reference : IA;
                    END_VAR

                    reference.bar(reference.foo);
                END_FUNCTION
            "#;

            insta::assert_debug_snapshot!(super::lower_and_serialize_statements(source, &["main"]), @r#"
            [
                "// Statements in main",
                "__itable_IA#(reference.table^).bar^(reference.data^, __itable_IA#(reference.table^).__get_foo^(reference.data^))",
            ]
            "#);
        }
    }
}
