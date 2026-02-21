//! Lowering of method calls into indirect calls through virtual tables
//!
//! This module is responsible for lowering / transforming method calls into method calls through the
//! virtual table (for information regarding virtual tables refer to [`crate::lowering::polymorphism::table`]). In a
//! nutshell it will transform a method call such as `ptr^.foo()` into `__vtable_Fb#(ptr^.__vtable^).foo^(ptr^)`.
//!
//! However, not all method calls must be lowered but rather the following cases:
//!
//! # 1. Method calls within methods (and function block bodies)
//! The reason we want to lower all method calls within (other) methods is for the fact that a non
//! overridden method may make use of an overridden method. For example consider
//! ```text
//! FUNCTION_BLOCK A
//!     METHOD getName: STRING
//!         getName := 'A';
//!     END_METHOD
//!
//!     METHOD printName
//!         printf('name = %s$N', ADR(getName()));
//!     END_METHOD
//! END_FUNCTION_BLOCK
//!
//! FUNCTION_BLOCK B EXTENDS A
//!     METHOD getName: STRING
//!         getName := 'B';
//!     END_METHOD
//!
//!     METHOD persistName
//!         // Persist name to some file
//!     END_METHOD
//! END_FUNCTION_BLOCK
//!
//! FUNCTION main
//!     VAR
//!         instanceA: A;
//!         instanceB: B;
//!
//!         refInstanceA: POINTER TO A;
//!     END_VAR
//!
//!     refInstanceA := ADR(instanceA);
//!     refInstanceA^.printName(); // Calls `A::printName` which calls `A::getName` yielding "name = A"
//!
//!     refInstanceA := ADR(instanceB);
//!     refInstanceA^.printName(); // Calls `A::printName` which calls `B::getName` yielding "name = B"
//! END_FUNCTION
//! ```
//!
//! As described in the main function, the calls to `printName` must happen at runtime. Were that not the case
//! then `printName` in A would resolve to `A::getName` at compile time, yielding an incorrect result for the
//! second `refInstanceA^.printName()` call. Lowering the call to `printName` would result in
//! `printf('name = %s$N', ADR(__vtable_A#(THIS^.__vtable^).getName^(THIS^))`.
//!
//! # 2. Method calls through a pointer variable pointing to a class or function block
//! Essentially what is illustrated in 1. within the main function, consider:
//!
//! ```text
//! FUNCTION main
//!     VAR
//!         name: STRING;
//!         instanceA: A;
//!         instanceB: B;
//!
//!         refInstanceA: POINTER TO A;
//!     END_VAR
//!
//!     refInstanceA := ADR(instanceA);
//!     name := refInstanceA^.getName(); // Calls `A::getName` yielding "name = A"
//!
//!     refInstanceA := ADR(instanceB);
//!     name := refInstanceA^.getName(); // Calls `B::getName` yielding "name = B"
//! END_FUNCTION
//! ```
//!
//! While this is a simple example, and in theory compilers would be able to derive the correct method calls
//! at compile time with some statical analysis, our compiler today is not able to do that. Specifically it
//! does not know that the second `refInstanceA` variable is pointing at `B` and the pointer call could be
//! simplified into a direct call to `B::getName()`. Instead, it relies on dynamic dispatch for a correct code
//! execution. Again, this is done by accessing the virtual table and calling the function pointer behind it.
//! In terms of ST code we would transform the calls into `__vtable_A#(refInstanceA^.__vtable^).getName^(refInstanceA^)`.
//!
//!
//! One final thing to note, while the casting of the virtual tables into concrete types is not really
//! interesting per-se, the upcasting from a child to its parent virtual table should at least be mentioned.
//! That is, as long as the virtual table definitions are compatible, upcasting can be performed without any
//! issues. Compatible here refers to the fact that the order of the member fields must be constant. More
//! specifically, the methods must be defined in "ancestral hierarchical order". To illustrate with the
//! previous examples, assume we have
//! ```text
//! TYPE __vtable_A:
//!     STRUCT
//!         getName: __FPOINTER TO A.getName := ADR(A.getName);
//!         printName: __FPOINTER TO A.printName := ADR(A.printName);
//!     END_STRUCT
//! END_TYPE
//!
//! TYPE __vtable_B:
//!     STRUCT
//!         getName: __FPOINTER TO B.getName := ADR(B.getName);             // Overridden
//!         printName: __FPOINTER TO A.printName := ADR(A.printName);       // Inherited
//!         persistName: __FPOINTER TO B.persistName := ADR(B.persistName); // New
//!     END_STRUCT
//! ```
//!
//! We can safely cast from B to A's virtual table because the layout is compatible and it would result in
//! `persistName` to be cut off in the cast. Were that not the case, e.g. if `getName` were to be swapped with
//! `persistName` then the call to `getName` would silently result in calling `persistName`.

use plc_ast::{
    ast::{
        AstFactory, AstNode, AstStatement, CallStatement, CompilationUnit, PouType, ReferenceAccess,
        ReferenceExpr,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    resolver::{AnnotationMap, AnnotationMapImpl, StatementAnnotation},
    typesystem::{DataType, DataTypeInformation},
};

// TODO: Rename to something more specific
pub struct PolymorphicCallLowerer<'a> {
    pub ids: IdProvider,
    pub index: &'a Index,
    pub annotations: &'a AnnotationMapImpl,

    pub in_method_or_function_block: Option<String>,
}

impl<'a> AstVisitorMut for PolymorphicCallLowerer<'a> {
    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        if implementation.location.is_internal() {
            return;
        }

        self.in_method_or_function_block = match &implementation.pou_type {
            PouType::FunctionBlock => Some(implementation.name.clone()),
            PouType::Method { parent, .. } => Some(parent.clone()),
            _ => None,
        };

        for statement in &mut implementation.statements {
            statement.walk(self);
        }

        self.in_method_or_function_block = None;
    }

    fn visit_call_statement(&mut self, node: &mut plc_ast::ast::AstNode) {
        // When dealing with a function call such as `ref^.foo()` we have to perform several steps to lower
        // it into a form that can be executed by the codegen without any intervention from our side, namely:
        // 1. We must add the expression (excluding the method name) as the first argument to the call
        //    -> ref^.foo(ref^)
        // 2. We must access the virtual table of the instance, a VOID pointer
        //    -> ref^.__vtable^.foo(ref^)
        // 3. We must cast the virtual table access into a concrete type
        //    -> __vtable_<POU_NAME>#(ref^.__vtable^).foo(ref^)
        // 4. We must dereference the method call, which is a function pointer
        //    -> __vtable_<POU_NAME>#(ref^.__vtable^).foo^(ref^)
        //
        // The final result transforms ref^.foo() into __vtable_<POU_NAME>#(ref^.__vtable^).foo^(ref^)
        let prev = node.as_string();
        let AstStatement::CallStatement(CallStatement { operator, parameters }) = &mut node.stmt else {
            unreachable!();
        };

        // We need to walk the parameters before deciding to potentially stop here, because parameters may
        // also contain polymorphic calls that need to be lowered, e.g. `functionCall(methodCall())`
        if let Some(ref mut parameters) = parameters {
            parameters.walk(self);
        }

        if !self.is_polymorphic_call_candidate(operator) {
            return;
        };

        let unit_name = match self.in_method_or_function_block {
            Some(ref name) => name.clone(),
            None => match operator.get_base_ref_expr() {
                Some(base) => {
                    // When dealing with e.g. __main_myVariable
                    let ty = self.annotations.get_type(base, self.index).unwrap();
                    self.index.find_elementary_pointer_type(ty.get_type_information()).get_name().to_string()
                }

                None => {
                    let ty = self.annotations.get_type(operator, self.index).unwrap();
                    self.index.find_elementary_pointer_type(ty.get_type_information()).get_name().to_string()
                }
            },
        };

        log::trace!("lowering {}", operator.as_string());

        // Pre-steps, add `__body` call when dealing with a direct function block and...
        self.maybe_patch_body_operator(operator);
        log::trace!("-1: {}", operator.as_string());

        // ...add a `THIS^` base when dealing with method calls without a base, e.g. `foo()`
        self.maybe_patch_this_base(operator);
        log::trace!("0: {}", operator.as_string());

        // Step 1: Add the expression (excluding the method name) as the first argument to the call
        self.patch_instance_argument(operator, parameters);
        log::trace!("1: {}", operator.as_string());

        // Step 2: Patch a dereferenced virtual table access into the operator
        self.patch_vtable_access(operator);
        log::trace!("2: {}", operator.as_string());

        // Step 3: Patch the virtual table cast into the operator
        self.patch_vtable_cast(operator, &unit_name);
        log::trace!("3: {}", operator.as_string());

        // Step 4: Patch the method call to a dereferenced call
        self.patch_method_call_deref(operator);
        log::trace!("4: {}", operator.as_string());

        log::debug!("converted `{}` into `{}`", prev, node.as_string());
    }
}

impl<'a> PolymorphicCallLowerer<'a> {
    pub fn new(ids: IdProvider, index: &'a Index, annotations: &'a AnnotationMapImpl) -> Self {
        PolymorphicCallLowerer { ids, index, annotations, in_method_or_function_block: None }
    }

    pub fn lower(&mut self, units: &mut [CompilationUnit]) {
        for unit in units {
            self.visit_compilation_unit(unit);
        }
    }

    /// Returns true if the given AST node is a candidate that needs to be lowered into a polymorphic call.
    fn is_polymorphic_call_candidate(&self, operator: &AstNode) -> bool {
        // We do not want to lower THIS and SUPER access, e.g. {THIS,SUPER}^() or {THIS,SUPER}^.foo()
        if operator.is_super_or_super_deref()
            || operator.get_base_ref_expr().is_some_and(AstNode::is_super_or_super_deref)
            || operator.is_this()
            || operator.is_this_deref()
        {
            return false;
        }

        if self.annotations.get(operator).is_some_and(|opt| opt.is_fnptr()) {
            return false;
        }

        // Case 1 (Method call within methods or function block bodies)
        if self.in_method_or_function_block.is_some()
            && self.annotations.get_type(operator, self.index).is_some_and(DataType::is_method)
            // Only lower something alike `THIS^.foo()` or `foo()` as opposed to `SUPER^.foo()` or `instanceFb.foo()`
            && (operator.get_base_ref_expr().is_none() || operator.get_base_ref_expr().is_some_and(|opt| opt.is_this()))
        {
            return true;
        }

        // Case 2 (Method invocation via a pointer to a class or function block instance)
        let AstStatement::ReferenceExpr(ReferenceExpr { access, base }) = &operator.stmt else {
            return false;
        };

        match (access, base) {
            // Dealing with `MyFbRef^.foo()`
            (ReferenceAccess::Member(_), Some(base)) => self.is_polymorphic_call_candidate(base),

            // Dealing with `MyFbRef^()`
            (ReferenceAccess::Deref, Some(base)) => {
                if self.annotations.get(operator).is_some_and(StatementAnnotation::is_fnptr) {
                    return false;
                };

                let info = self.annotations.get_type_or_void(base, self.index).get_type_information();
                let DataTypeInformation::Pointer { inner_type_name, .. } = info else {
                    return false;
                };

                let inner_pointer_type = self.index.get_type_information_or_void(inner_type_name);
                inner_pointer_type.is_class() || inner_pointer_type.is_function_block()
            }

            // Auto-deref, e.g. `refInstance: REFERENCE TO ...`
            (ReferenceAccess::Member(member), None) => {
                self.annotations.get(member).is_some_and(StatementAnnotation::is_reference_to)
            }

            _ => false,
        }
    }

    fn maybe_patch_body_operator(&mut self, operator: &mut AstNode) {
        if !self
            .annotations
            .get_type(operator, self.index)
            .is_some_and(|opt| opt.information.is_function_block())
        {
            return;
        }

        let old_base = std::mem::take(operator);
        let mut new_base = AstFactory::create_member_reference(
            AstFactory::create_identifier("__body", SourceLocation::internal(), self.ids.next_id()),
            Some(old_base),
            self.ids.next_id(),
        );

        std::mem::swap(operator, &mut new_base);
    }

    fn maybe_patch_this_base(&mut self, operator: &mut AstNode) {
        if !(self.in_method_or_function_block.is_some() && operator.get_base_ref_expr().is_none()) {
            return;
        }

        let this_node = Box::new(AstFactory::create_deref_reference(
            AstFactory::create_this_reference(SourceLocation::internal(), self.ids.next_id()),
            self.ids.next_id(),
            SourceLocation::internal(),
        ));

        operator.get_ref_expr_mut().unwrap().base.replace(this_node);
    }

    fn patch_instance_argument(&mut self, operator: &mut AstNode, parameters: &mut Option<Box<AstNode>>) {
        // foo.bar()
        // ^^^ base
        let base = self.maybe_cast_instance(operator);

        match parameters {
            None => {
                parameters.replace(Box::new(base));
            }

            Some(ref mut expr) => match &mut expr.stmt {
                AstStatement::ExpressionList(expressions) => {
                    expressions.insert(0, base);
                }

                _ => {
                    let mut expressions = Box::new(AstFactory::create_expression_list(
                        vec![base, std::mem::take(expr)],
                        SourceLocation::internal(),
                        self.ids.next_id(),
                    ));

                    std::mem::swap(expr, &mut expressions);
                }
            },
        }
    }

    /// Patches a `__vtable` member access into the given node, e.g. `ref^.foo()` becomes `ref^.__vtable^.foo()`
    fn patch_vtable_access(&mut self, node: &mut AstNode) {
        let old_base = node.get_base_ref_expr_mut().unwrap(); // `ref^` in `ref^.foo()`

        let mut new_base = AstFactory::create_deref_reference(
            AstFactory::create_member_reference(
                AstFactory::create_identifier("__vtable", SourceLocation::internal(), self.ids.next_id()),
                Some(std::mem::take(old_base)),
                self.ids.next_id(),
            ),
            self.ids.next_id(),
            SourceLocation::internal(),
        );

        std::mem::swap(old_base, &mut new_base);
    }

    /// ref^.__vtable^.foo()` -> `__vtable_{POU_NAME}#(ref^.__vtable^).foo()
    fn patch_vtable_cast(&mut self, node: &mut AstNode, pou_type_name: &str) {
        let base_old = node.get_base_ref_expr_mut().unwrap(); // `ref^.__vtable^` in `ref^.__vtable^.foo()`
        let base_old_paren = AstFactory::create_paren_expression(
            std::mem::take(base_old),
            SourceLocation::internal(),
            self.ids.next_id(),
        );

        let mut base_new = AstFactory::create_cast_statement(
            AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    format!("__vtable_{pou_type_name}"),
                    SourceLocation::internal(),
                    self.ids.next_id(),
                ),
                None,
                self.ids.next_id(),
            ),
            base_old_paren,
            &SourceLocation::internal(),
            self.ids.next_id(),
        );

        std::mem::swap(base_old, &mut base_new);
    }

    // `__vtable_{POU_NAME}#(ref^.__vtable^).foo()` -> `__vtable_{POU_NAME}#(ref^.__vtable^).foo^()`
    fn patch_method_call_deref(&mut self, node: &mut AstNode) {
        let mut base_new = AstFactory::create_deref_reference(
            std::mem::take(node),
            self.ids.next_id(),
            SourceLocation::internal(),
        );

        std::mem::swap(node, &mut base_new);
    }

    /// Casts the instance argument to its concrete type, as defined by the method. While not strictly
    /// necessary, because the code would compile without this step, it is required when generating IR using
    /// the `--ir` flag. This is because some type-validation happens (afaik).
    ///
    /// For example `alpha(1, 2)` will be transformed into` `<...>.alpha^(A#(THIS^), 1, 2)` instead of
    /// `<...>.alpha^(THIS^, 1, 2)`. The same would be true for `refInstance.alpha(1, 2)` which would become
    /// `<...>.alpha^(A#(refInstance), 1, 2)`.
    /// ```text
    /// FUNCTION_BLOCK A
    ///     METHOD alpha
    ///         VAR_INPUT
    ///             a, b: DINT;
    ///         END_VAR
    ///     END_METHOD
    /// END_FUNCTION_BLOCK
    ///
    /// FUNCTION_BLOCK B EXTENDS A
    ///     METHOD bravo
    ///         alpha(1, 2);
    ///     END_METHOD
    /// END_FUNCTION_BLOCK
    /// ```
    fn maybe_cast_instance(&mut self, operator: &AstNode) -> AstNode {
        let Some(method_owner) = self
            .annotations
            .get_type(operator, self.index)
            .map(DataType::get_type_information)
            .and_then(DataTypeInformation::get_method_owner)
        else {
            return operator.get_base_ref_expr().unwrap().clone();
        };

        let base = operator.get_base_ref_expr().unwrap();
        AstFactory::create_cast_statement(
            AstFactory::create_member_reference(
                AstFactory::create_identifier(method_owner, SourceLocation::internal(), self.ids.next_id()),
                None,
                self.ids.next_id(),
            ),
            AstFactory::create_paren_expression(base.clone(), SourceLocation::internal(), self.ids.next_id()),
            &SourceLocation::internal(),
            self.ids.next_id(),
        )
    }
}

#[cfg(test)]
mod tests {
    use driver::parse_and_annotate;
    use plc_source::SourceCode;

    fn lower_statements(source: impl Into<SourceCode>, pous: &[&str]) -> Vec<String> {
        let (_, project) = parse_and_annotate("unit-test", vec![source.into()]).unwrap();
        let unit = project.units[0].get_unit();

        let mut result = Vec::new();
        for pou in pous {
            result.push(format!("// Statements in {pou}"));
            let statements = &unit.implementations.iter().find(|it| &it.name == pou).unwrap().statements;
            let statements_str = statements.iter().map(|statement| statement.as_string()).collect::<Vec<_>>();

            result.extend(statements_str);
        }

        result
    }

    #[test]
    fn this_calls_are_untouched() {
        let source = r#"
            FUNCTION_BLOCK A
                METHOD foo
                    THIS^();
                    THIS^.bar();
                END_METHOD

                METHOD bar
                END_METHOD

                THIS^();
                THIS^.bar();
            END_FUNCTION_BLOCK
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["A", "A.foo"]), @r#"
        [
            "// Statements in A",
            "THIS^()",
            "THIS^.bar()",
            "// Statements in A.foo",
            "THIS^()",
            "THIS^.bar()",
        ]
        "#);
    }

    #[test]
    fn super_calls_are_untouched() {
        let source = r#"
            FUNCTION_BLOCK A
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK B EXTENDS A
                METHOD foo
                    SUPER^();
                    SUPER^.foo();
                END_METHOD

                SUPER^();
                SUPER^.foo();
            END_FUNCTION_BLOCK
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["B", "B.foo"]), @r#"
        [
            "// Statements in B",
            "__A()",
            "__A.foo()",
            "// Statements in B.foo",
            "__A()",
            "__A.foo()",
        ]
        "#);
    }

    #[test]
    fn function_calls_are_untouched() {
        let source = r#"
            FUNCTION foo
            END_FUNCTION

            FUNCTION main
                foo();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "foo()",
        ]
        "#);
    }

    #[test]
    fn method_calls_with_instance_base_are_untouched() {
        let source = r#"
            FUNCTION_BLOCK A
                METHOD alpha
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK B
                VAR
                    instanceA: A;
                END_VAR

                METHOD bravo
                    instanceA();
                    instanceA.alpha();
                END_METHOD

                instanceA();
                instanceA.alpha();
            END_FUNCTION_BLOCK
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["B", "B.bravo"]), @r#"
        [
            "// Statements in B",
            "instanceA()",
            "instanceA.alpha()",
            "// Statements in B.bravo",
            "instanceA()",
            "instanceA.alpha()",
        ]
        "#);
    }

    #[test]
    fn method_arguments_are_untouched() {
        let source = r#"
            FUNCTION_BLOCK A
                VAR
                    localIn, localOut, localInout: DINT;
                END_VAR

                METHOD alpha
                END_METHOD

                METHOD bravo
                    VAR_INPUT
                        in: DINT;
                    END_VAR
                END_METHOD

                METHOD charlie
                    VAR_INPUT
                        in: DINT;
                    END_VAR

                    VAR_OUTPUT
                        out: DINT;
                    END_VAR
                END_METHOD

                METHOD delta
                    VAR_INPUT
                        in: DINT;
                    END_VAR

                    VAR_OUTPUT
                        out: DINT;
                    END_VAR

                    VAR_IN_OUT
                        inout: DINT;
                    END_VAR
                END_METHOD

                alpha();

                bravo(1);
                bravo(localIn);

                bravo(in := 1);
                bravo(in := localIn);

                charlie(1, localOut);
                charlie(localIn, localOut);

                charlie(in := 1, out => localOut);
                charlie(in := localIn, out => localOut);

                delta(1, localOut, localInout);
                delta(localIn, localOut, localInout);

                delta(in := 1, out => localOut, inout := localInout);
                delta(inout := localInout, in := localIn, out => localOut);
            END_FUNCTION_BLOCK
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["A"]), @r#"
        [
            "// Statements in A",
            "__vtable_A#(THIS^.__vtable^).alpha^(A#(THIS^))",
            "__vtable_A#(THIS^.__vtable^).bravo^(A#(THIS^), 1)",
            "__vtable_A#(THIS^.__vtable^).bravo^(A#(THIS^), localIn)",
            "__vtable_A#(THIS^.__vtable^).bravo^(A#(THIS^), in := 1)",
            "__vtable_A#(THIS^.__vtable^).bravo^(A#(THIS^), in := localIn)",
            "__vtable_A#(THIS^.__vtable^).charlie^(A#(THIS^), 1, localOut)",
            "__vtable_A#(THIS^.__vtable^).charlie^(A#(THIS^), localIn, localOut)",
            "__vtable_A#(THIS^.__vtable^).charlie^(A#(THIS^), in := 1, out => localOut)",
            "__vtable_A#(THIS^.__vtable^).charlie^(A#(THIS^), in := localIn, out => localOut)",
            "__vtable_A#(THIS^.__vtable^).delta^(A#(THIS^), 1, localOut, localInout)",
            "__vtable_A#(THIS^.__vtable^).delta^(A#(THIS^), localIn, localOut, localInout)",
            "__vtable_A#(THIS^.__vtable^).delta^(A#(THIS^), in := 1, out => localOut, inout := localInout)",
            "__vtable_A#(THIS^.__vtable^).delta^(A#(THIS^), inout := localInout, in := localIn, out => localOut)",
        ]
        "#);
    }

    #[test]
    fn polymorphic_calls_in_methods() {
        let source = r#"
            FUNCTION_BLOCK A
                METHOD alpha
                    bravo();
                END_METHOD

                METHOD bravo
                    alpha();
                END_METHOD

                alpha();
                bravo();
            END_FUNCTION_BLOCK
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["A", "A.alpha", "A.bravo"]), @r#"
        [
            "// Statements in A",
            "__vtable_A#(THIS^.__vtable^).alpha^(A#(THIS^))",
            "__vtable_A#(THIS^.__vtable^).bravo^(A#(THIS^))",
            "// Statements in A.alpha",
            "__vtable_A#(THIS^.__vtable^).bravo^(A#(THIS^))",
            "// Statements in A.bravo",
            "__vtable_A#(THIS^.__vtable^).alpha^(A#(THIS^))",
        ]
        "#);
    }

    #[test]
    fn polymorphic_calls_in_methods_with_inheritance() {
        let source = r#"
            FUNCTION_BLOCK A
                METHOD alpha
                END_METHOD

                alpha();
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK B EXTENDS A
                METHOD bravo
                    alpha();
                END_METHOD

                alpha();
                bravo();
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK C EXTENDS B
                METHOD bravo
                    // Overridden
                    alpha();
                    charlie();
                END_METHOD

                METHOD charlie
                    alpha();
                    bravo();
                END_METHOD

                alpha();
                bravo();
                charlie();
            END_FUNCTION_BLOCK
        "#;

        insta::assert_debug_snapshot!(
            lower_statements(source, &["A", "A.alpha", "B", "B.bravo", "C", "C.bravo", "C.charlie"]),
            @r#"
        [
            "// Statements in A",
            "__vtable_A#(THIS^.__vtable^).alpha^(A#(THIS^))",
            "// Statements in A.alpha",
            "// Statements in B",
            "__vtable_B#(THIS^.__A.__vtable^).alpha^(A#(THIS^))",
            "__vtable_B#(THIS^.__A.__vtable^).bravo^(B#(THIS^))",
            "// Statements in B.bravo",
            "__vtable_B#(THIS^.__A.__vtable^).alpha^(A#(THIS^))",
            "// Statements in C",
            "__vtable_C#(THIS^.__B.__A.__vtable^).alpha^(A#(THIS^))",
            "__vtable_C#(THIS^.__B.__A.__vtable^).bravo^(C#(THIS^))",
            "__vtable_C#(THIS^.__B.__A.__vtable^).charlie^(C#(THIS^))",
            "// Statements in C.bravo",
            "__vtable_C#(THIS^.__B.__A.__vtable^).alpha^(A#(THIS^))",
            "__vtable_C#(THIS^.__B.__A.__vtable^).charlie^(C#(THIS^))",
            "// Statements in C.charlie",
            "__vtable_C#(THIS^.__B.__A.__vtable^).alpha^(A#(THIS^))",
            "__vtable_C#(THIS^.__B.__A.__vtable^).bravo^(C#(THIS^))",
        ]
        "#
        );
    }

    #[test]
    fn polymorphic_call_as_argument() {
        let source = r#"
            FUNCTION_BLOCK A
                METHOD alpha: DINT
                    alpha := 5;
                END_METHOD

                METHOD bravo
                    VAR_INPUT
                        in: DINT;
                    END_VAR
                END_METHOD

                bravo(alpha());
            END_FUNCTION_BLOCK
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["A"]), @r#"
        [
            "// Statements in A",
            "__vtable_A#(THIS^.__vtable^).bravo^(A#(THIS^), __vtable_A#(THIS^.__vtable^).alpha^(A#(THIS^)))",
        ]
        "#);
    }

    #[test]
    fn polymorphic_calls_through_pointer_variables() {
        let source = r#"
            FUNCTION_BLOCK A
                METHOD alpha
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK B EXTENDS A
                METHOD bravo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK C EXTENDS B
                METHOD charlie
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION operateOnA
                VAR_IN_OUT
                    refInstanceA: POINTER TO A;
                END_VAR

                refInstanceA^.alpha();
            END_FUNCTION

            FUNCTION main
                VAR
                    refInstanceA: POINTER TO A;
                    refInstanceB: POINTER TO B;
                    refInstanceC: POINTER TO C;

                    refInstanceArrayA: ARRAY[1..5] OF POINTER TO A;
                    refInstanceArrayB: ARRAY[1..5] OF POINTER TO B;
                    refInstanceArrayC: ARRAY[1..5] OF POINTER TO C;
                END_VAR

                refInstanceA^.alpha();
                refInstanceB^.bravo();
                refInstanceC^.charlie();

                refInstanceArrayA[1]^.alpha();
                refInstanceArrayB[1]^.bravo();
                refInstanceArrayC[1]^.charlie();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["main", "operateOnA"]), @r#"
        [
            "// Statements in main",
            "__vtable_A#(refInstanceA^.__vtable^).alpha^(A#(refInstanceA^))",
            "__vtable_B#(refInstanceB^.__A.__vtable^).bravo^(B#(refInstanceB^))",
            "__vtable_C#(refInstanceC^.__B.__A.__vtable^).charlie^(C#(refInstanceC^))",
            "__vtable_A#(refInstanceArrayA[1]^.__vtable^).alpha^(A#(refInstanceArrayA[1]^))",
            "__vtable_B#(refInstanceArrayB[1]^.__A.__vtable^).bravo^(B#(refInstanceArrayB[1]^))",
            "__vtable_C#(refInstanceArrayC[1]^.__B.__A.__vtable^).charlie^(C#(refInstanceArrayC[1]^))",
            "// Statements in operateOnA",
            "__vtable_A#(refInstanceA^.__vtable^).alpha^(A#(refInstanceA^))",
        ]
        "#);
    }

    #[test]
    fn polymorphic_function_block_calls() {
        let source = r#"
            FUNCTION_BLOCK A
                VAR_INPUT
                    in: DINT;
                END_VAR

                VAR_OUTPUT
                    out: DINT;
                END_VAR

                VAR_IN_OUT
                    inout: DINT;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refInstanceA: POINTER TO A;
                    localIn, localOut, localInout: DINT;
                END_VAR

                refInstanceA^();
                refInstanceA^(1, 2, 3); // Not valid per-se
                refInstanceA^(localIn, localOut, localInout);
                refInstanceA^(in := localIn, out => localOut, inout := localInOut);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__vtable_A#(refInstanceA^.__vtable^).__body^(refInstanceA^)",
            "__vtable_A#(refInstanceA^.__vtable^).__body^(refInstanceA^, 1, 2, 3)",
            "__vtable_A#(refInstanceA^.__vtable^).__body^(refInstanceA^, localIn, localOut, localInout)",
            "__vtable_A#(refInstanceA^.__vtable^).__body^(refInstanceA^, in := localIn, out => localOut, inout := localInOut)",
        ]
        "#);
    }

    #[test]
    fn polymorphic_function_block_call_through_member_variable() {
        let source = r#"
            FUNCTION_BLOCK A
                VAR
                    refB: POINTER TO B;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK B
                VAR
                    refC: POINTER TO C;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK C
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refInstanceA: POINTER TO A;
                END_VAR

                refInstanceA^();
                refInstanceA^.refB^();
                refInstanceA^.refB^.refC^();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__vtable_A#(refInstanceA^.__vtable^).__body^(refInstanceA^)",
            "__vtable_B#(refInstanceA^.refB^.__vtable^).__body^(refInstanceA^.refB^)",
            "__vtable_C#(refInstanceA^.refB^.refC^.__vtable^).__body^(refInstanceA^.refB^.refC^)",
        ]
        "#);
    }

    #[test]
    fn ref_to() {
        let source = r#"
            FUNCTION_BLOCK A
                VAR
                    refB: REF_TO B;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK B
                VAR
                    refC: REF_TO C;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK C
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refInstanceA: REF_TO A;
                END_VAR

                refInstanceA^();
                refInstanceA^.refB^();
                refInstanceA^.refB^.refC^();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__vtable_A#(refInstanceA^.__vtable^).__body^(refInstanceA^)",
            "__vtable_B#(refInstanceA^.refB^.__vtable^).__body^(refInstanceA^.refB^)",
            "__vtable_C#(refInstanceA^.refB^.refC^.__vtable^).__body^(refInstanceA^.refB^.refC^)",
        ]
        "#);
    }

    #[test]
    fn reference_to() {
        let source = r#"
            FUNCTION_BLOCK A
                VAR
                    refB: REFERENCE TO B;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK B
                VAR
                    refC: REFERENCE TO C;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK C
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refInstanceA: REFERENCE TO A;
                END_VAR

                refInstanceA();
                refInstanceA.refB();
                refInstanceA.refB.refC();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__vtable_A#(refInstanceA.__vtable^).__body^(refInstanceA)",
            "__vtable_A#(refInstanceA.refB.__vtable^).__body^(refInstanceA.refB)",
            "__vtable_B#(refInstanceA.refB.refC.__vtable^).__body^(refInstanceA.refB.refC)",
        ]
        "#);
    }
}
