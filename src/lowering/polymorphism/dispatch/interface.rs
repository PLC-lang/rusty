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
        steal_expression_list, Allocation, Assignment, AstFactory, AstNode, AstStatement, CallStatement,
        CompilationUnit, DataType as AstDataType, DataTypeDeclaration, LinkageType, UserTypeDeclaration,
        Variable,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    resolver::{AnnotationMap, AnnotationMapImpl},
    typesystem::{DataType, VOID_INTERNAL_NAME},
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

    /// Monotonic counter for generating unique fat-pointer temporary names
    /// (`__fatpointer_0`, `__fatpointer_1`, ...). Never reset; each call-argument wrap site
    /// gets a distinct alloca across the entire compilation unit.
    fp_counter: usize,
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
            fp_counter: 0,
        }
    }

    /// Entry point into the interface dispatch lowering pass.
    /// Returns any diagnostics produced by interface validation checks.
    pub fn lower(&mut self, units: &mut [CompilationUnit]) -> Vec<Diagnostic> {
        for unit in units.iter_mut() {
            self.visit_compilation_unit(unit);
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

        // Walk children first so nested calls/expressions are processed before we inspect types.
        left.walk(self);
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

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let AstStatement::CallStatement(CallStatement { operator, parameters }) = &mut node.stmt else {
            unreachable!();
        };

        // Walk the operator first (processes nested interface method calls depth-first).
        operator.walk(self);

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

            for arg in args.iter_mut() {
                match &mut arg.stmt {
                    // Named argument (input, output, or in-out). Inspect and potentially replace the RHS.
                    // TODO: Add tests for output (`out1 => x`) and in-out (`inout1 := x`) argument wrapping.
                    AstStatement::Assignment(Assignment { right, .. })
                    | AstStatement::OutputAssignment(Assignment { right, .. })
                    | AstStatement::RefAssignment(Assignment { right, .. }) => {
                        self.maybe_wrap_argument(right.as_mut());
                    }

                    // Positional argument. Inspect and potentially replace the whole node.
                    _ => {
                        self.maybe_wrap_argument(arg);
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

impl InterfaceDispatchLowerer<'_> {
    /// Checks whether `arg` is a concrete POU being passed where an interface is expected.
    /// If so, generates a temporary fat pointer (alloca + field assignments) in `self.preamble`
    /// and replaces `arg` in-place with a reference to the temporary.
    fn maybe_wrap_argument(&mut self, arg: &mut AstNode) {
        let actual_type = self.annotations.get_type_or_void(arg, self.index);
        let expected_type = self.annotations.get_hint_or_void(arg, self.index);

        // Not an interface parameter, nothing to do.
        if !expected_type.is_interface() {
            return;
        }

        // Argument is already interface-typed — validate compatibility before upcasting.
        if actual_type.is_interface() {
            let source_iface = actual_type.get_name();
            let target_iface = expected_type.get_name();

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

        if let Some(diagnostic) =
            validation::validate_pou_implements_interface(self.index, pou_name, interface_name, &arg.location)
        {
            self.diagnostics.push(diagnostic);

            // Replace the argument with a dummy fat pointer so the call remains structurally
            // valid and doesn't trigger confusing `__FATPOINTER` type errors downstream.
            let fp_name = format!("__fatpointer_{}", self.fp_counter);
            self.fp_counter += 1;
            self.preamble.push(helper::create_alloca(&mut self.ids, &fp_name));
            *arg = helper::create_identifier_ref(&mut self.ids, &fp_name);

            return;
        }

        // Generate a unique name for the temporary fat pointer.
        let fp_name = format!("__fatpointer_{}", self.fp_counter);
        self.fp_counter += 1;

        // Build the reference node that will replace the original argument.
        let fp_ref = helper::create_identifier_ref(&mut self.ids, &fp_name);
        let alloca = helper::create_alloca(&mut self.ids, &fp_name);
        self.preamble.push(alloca);

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
            }),
            id: ids.next_id(),
            location: SourceLocation::internal(),
            metadata: None,
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
    /// instance address) and `table` (the itable address).
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
            linkage: LinkageType::Internal,
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
                result.push(statement.as_string());
            }
        }

        result
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
                "alloca __producer0: FbA, producer(__producer0), reference.data := ADR(__producer0)",
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
                "alloca __producer0: FbA, fb.producer(__producer0), reference.data := ADR(__producer0)",
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
                "alloca __producer0: FbA, producer(__producer0), __fatpointer_0.data := ADR(__producer0)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "alloca __producer1: FbA, producer(__producer1), __fatpointer_1.data := ADR(__producer1)",
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
                "alloca __produce0: FbA, fb.produce(__produce0), __fatpointer_0.data := ADR(__produce0)",
                "__fatpointer_0.table := ADR(__itable_IA_FbA_instance)",
                "consumer(__fatpointer_0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "alloca __produce1: FbA, fb.produce(__produce1), __fatpointer_1.data := ADR(__produce1)",
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
                "alloca __inner0: __FATPOINTER, inner(__inner0, __fatpointer_0), outer(__inner0)",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "alloca __inner1: __FATPOINTER, inner(inner := __inner1, in1 := __fatpointer_1), outer(in1 := __inner1)",
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
                "alloca __consumer0: STRING, consumer(__consumer0, __fatpointer_0), result := __consumer0",
                "alloca __fatpointer_1: __FATPOINTER",
                "__fatpointer_1.data := ADR(instance)",
                "__fatpointer_1.table := ADR(__itable_IA_FbA_instance)",
                "alloca __consumer1: STRING, consumer(consumer := __consumer1, in1 := __fatpointer_1), result := __consumer1",
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

            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @r"
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

            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @r"
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
            FOR i := 0 TO 2 DO
                references[i].data := ADR(instances[i])
                references[i].table := ADR(__itable_IA_FbA_instance)
            END_FOR
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

            insta::assert_snapshot!(super::lower_and_serialize_statements(source, &["main"]).join("\n"), @r"
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
        #[ignore = "stale fat pointer: preamble is hoisted before the loop instead of re-evaluated each iteration"]
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
            alloca __fatpointer_0: __FATPOINTER
            __fatpointer_0.data := ADR(instances[i])
            __fatpointer_0.table := ADR(__itable_IA_FbA_instance)
            WHILE TRUE DO
                IF NOT consumer(__fatpointer_0) THEN

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
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
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
                    PROPERTY foo : DINT
                        GET END_GET
                        SET END_SET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
                        SET END_SET
                    END_PROPERTY
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
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
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
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
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
                    PROPERTY foo : DINT
                        GET END_GET
                        SET END_SET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
                        SET END_SET
                    END_PROPERTY
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
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
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

                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD bar : DINT END_METHOD

                    PROPERTY foo : DINT
                        GET END_GET
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
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
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
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
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
                    PROPERTY foo : DINT
                        GET END_GET
                        SET END_SET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
                        SET END_SET
                    END_PROPERTY
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
                    PROPERTY foo : DINT
                        GET END_GET
                        SET END_SET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    PROPERTY foo : DINT
                        GET END_GET
                        SET END_SET
                    END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbB EXTENDS FbA
                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
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

                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
                END_INTERFACE

                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD bar
                        VAR_INPUT
                            x : DINT;
                        END_VAR
                    END_METHOD

                    PROPERTY foo : DINT
                        GET END_GET
                    END_PROPERTY
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
