//! Module responsible for desugaring function calls that are polymorphic in nature.
//!
//! In general there are three cases in which method calls need to be desugared
//! 1. Method calls within methods
//! 2. Method calls within the body of a function block
//! 3. Method calls through a pointer variable pointing to a class or function block, e.g. `fbPtr^.alpha()`
//!
//! Desugaring these method calls is achieved by accessing the virtual table, see [`crate::lowering::vtable`].
//! TODO: Give a simple step-by-step process, then finish up with complete example showcasing points 1 - 3
//!
//! Finally, note that because ST lacks a `virtual` keyword, we have to assume every method call to be
//! polymorphic in nature regarding point 1 and 2.

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

pub struct PolymorphicCallDesugarer {
    pub ids: IdProvider,
    pub index: Option<Index>,
    pub annotations: Option<AnnotationMapImpl>,

    pub in_method_or_function_block: Option<String>,
}

impl AstVisitorMut for PolymorphicCallDesugarer {
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
        // When dealing with a function call such as `ref^.foo()` we have to perform several steps to desugar
        // it into a form that can be executed by the codegen without any intervention from our side, namely:
        // 1. We must add the expression (excluding the method name) as the first argument to the call
        //    -> ref^.foo(ref^)
        // 2. We must access the virtual table of the instance, a VOID pointer
        //    -> ref^.__vtable^.foo(ref^)
        // 3. We must cast the virtual table access into a concrete type
        //    -> __vtable_XXX#(ref^.__vtable^).foo(ref^)
        // 4. We must dereference the method call, which is a function pointer
        //    -> __vtable_XXX#(ref^.__vtable^).foo^(ref^)
        //
        // The final result transforms ref^.foo() into __vtable_XXX#(ref^.__vtable^).foo^(ref^)
        let prev = node.as_string();
        let AstStatement::CallStatement(CallStatement { operator, parameters }) = &mut node.stmt else {
            unreachable!();
        };

        // We need to walk the parameters before deciding to potentially stop here, because parameters may
        // also contain polymorphic calls that need to be desugared, e.g. `functionCall(methodCall())`
        if let Some(ref mut parameters) = parameters {
            parameters.walk(self);
        }

        if !self.is_polymorphic_call_candidate(operator) {
            return;
        };

        let index = self.index.as_ref().unwrap();
        let annotations = self.annotations.as_ref().unwrap();

        let unit_name = match self.in_method_or_function_block {
            Some(ref name) => name.clone(),
            None => match operator.get_base() {
                Some(base) => {
                    // When dealing with e.g. __main_myVariable
                    let ty = annotations.get_type(base, index).unwrap();
                    index.find_elementary_pointer_type(ty.get_type_information()).get_name().to_string()
                }

                None => {
                    let ty = annotations.get_type(operator, index).unwrap();
                    index.find_elementary_pointer_type(ty.get_type_information()).get_name().to_string()
                }
            },
        };

        log::trace!("desugaring {}", operator.as_string());

        // Step -1: Add a `__body` call when dealing with a direct function block call, e.g. `refFb^()`
        self.maybe_patch_body_operator(operator);
        log::trace!("-1: {}", operator.as_string());

        // Step 0: Add a `THIS^` base when dealing with method calls without a base, e.g. `foo()`
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

impl PolymorphicCallDesugarer {
    pub fn new(ids: IdProvider) -> PolymorphicCallDesugarer {
        PolymorphicCallDesugarer { ids, index: None, annotations: None, in_method_or_function_block: None }
    }

    pub fn desugar_unit(&mut self, unit: &mut CompilationUnit) {
        self.visit_compilation_unit(unit);
    }

    /// Returns true if the given AST node is a candidate that needs to be desugared into a polymorphic call.
    /// Cases such as:
    /// 1. A method call within (another) method
    /// 2. A method call within a function block body
    /// 3. A method invocation via a pointer to a class or function block instance, e.g. `refInstance^.foo()`
    fn is_polymorphic_call_candidate(&self, operator: &AstNode) -> bool {
        let index = self.index.as_ref().expect("index must be set before desugaring");
        let annotations = self.annotations.as_ref().expect("annotations must be set before desugaring");

        // We do not want to desugar SUPER access, e.g. SUPER^() or SUPER^.foo()
        if operator.is_super_or_super_deref()
            || operator.get_base().is_some_and(AstNode::is_super_or_super_deref)
            || operator.is_this()
            || operator.is_this_deref()
        {
            return false;
        }

        if annotations.get(operator).is_some_and(|opt| opt.is_fnptr()) {
            return false;
        }

        // Case 1 & 2
        if self.in_method_or_function_block.is_some()
            && annotations.get_type(operator, index).is_some_and(DataType::is_method)
            // Only desugar something alike `THIS^.foo()` or `foo()` as opposed to `SUPER^.foo()` or `instanceFb.foo()`
            && (operator.get_base().is_none() || operator.get_base().is_some_and(|opt| opt.is_this()))
        {
            return true;
        }

        // Case 3
        let AstStatement::ReferenceExpr(ReferenceExpr { access, base }) = &operator.stmt else {
            return false;
        };

        match (access, base) {
            // Probably dealing with `MyFbRef^.foo()`
            (ReferenceAccess::Member(_), Some(base)) => self.is_polymorphic_call_candidate(base),

            // Probably dealing with `MyFbRef^()`
            (ReferenceAccess::Deref, Some(base)) => {
                if annotations.get(operator).is_some_and(StatementAnnotation::is_fnptr) {
                    return false;
                };

                let info = annotations.get_type_or_void(base, index).get_type_information();
                let DataTypeInformation::Pointer { inner_type_name, .. } = info else {
                    return false;
                };

                let inner_pointer_type = index.get_type_information_or_void(inner_type_name);
                inner_pointer_type.is_class() || inner_pointer_type.is_function_block()
            }

            // Auto-deref, e.g. `refInstance: REFERENCE TO ...`
            (ReferenceAccess::Member(member), None) => {
                annotations.get(member).is_some_and(StatementAnnotation::is_reference_to)
            }

            _ => false,
        }
    }

    fn maybe_patch_body_operator(&mut self, operator: &mut AstNode) {
        let index = self.index.as_ref().unwrap();
        let annotations = self.annotations.as_ref().unwrap();

        if !annotations.get_type(operator, index).is_some_and(|opt| opt.information.is_function_block()) {
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
        if !(self.in_method_or_function_block.is_some() && operator.get_base().is_none()) {
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
        let base = operator.get_base().unwrap(); // XXX: I think this might fail on `MyBlockRef^()`

        match parameters {
            None => {
                parameters.replace(Box::new(base.clone()));
            }

            Some(ref mut expr) => match &mut expr.stmt {
                AstStatement::ExpressionList(expressions) => {
                    expressions.insert(0, base.clone());
                }

                _ => {
                    let mut expressions = Box::new(AstFactory::create_expression_list(
                        vec![base.clone(), std::mem::take(expr)],
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
        let old_base = node.get_base_mut().unwrap(); // `ref^` in `ref^.foo()`

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
        let base_old = node.get_base_mut().unwrap(); // `ref^.__vtable^` in `ref^.__vtable^.foo()`
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
}

#[cfg(test)]
mod tests {
    use driver::parse_and_annotate;
    use plc_source::SourceCode;

    fn desugared_statements(source: impl Into<SourceCode>, pous: &[&str]) -> Vec<String> {
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["A", "A.foo"]), @r#"
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["B", "B.foo"]), @r#"
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["main"]), @r#"
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["B", "B.bravo"]), @r#"
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["A"]), @r#"
        [
            "// Statements in A",
            "__vtable_A#(THIS^.__vtable^).alpha^(THIS^)",
            "__vtable_A#(THIS^.__vtable^).bravo^(THIS^, 1)",
            "__vtable_A#(THIS^.__vtable^).bravo^(THIS^, localIn)",
            "__vtable_A#(THIS^.__vtable^).bravo^(THIS^, in := 1)",
            "__vtable_A#(THIS^.__vtable^).bravo^(THIS^, in := localIn)",
            "__vtable_A#(THIS^.__vtable^).charlie^(THIS^, 1, localOut)",
            "__vtable_A#(THIS^.__vtable^).charlie^(THIS^, localIn, localOut)",
            "__vtable_A#(THIS^.__vtable^).charlie^(THIS^, in := 1, out => localOut)",
            "__vtable_A#(THIS^.__vtable^).charlie^(THIS^, in := localIn, out => localOut)",
            "__vtable_A#(THIS^.__vtable^).delta^(THIS^, 1, localOut, localInout)",
            "__vtable_A#(THIS^.__vtable^).delta^(THIS^, localIn, localOut, localInout)",
            "__vtable_A#(THIS^.__vtable^).delta^(THIS^, in := 1, out => localOut, inout := localInout)",
            "__vtable_A#(THIS^.__vtable^).delta^(THIS^, inout := localInout, in := localIn, out => localOut)",
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["A", "A.alpha", "A.bravo"]), @r#"
        [
            "// Statements in A",
            "__vtable_A#(THIS^.__vtable^).alpha^(THIS^)",
            "__vtable_A#(THIS^.__vtable^).bravo^(THIS^)",
            "// Statements in A.alpha",
            "__vtable_A#(THIS^.__vtable^).bravo^(THIS^)",
            "// Statements in A.bravo",
            "__vtable_A#(THIS^.__vtable^).alpha^(THIS^)",
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
            desugared_statements(source, &["A", "A.alpha", "B", "B.bravo", "C", "C.bravo", "C.charlie"]),
            @r#"
        [
            "// Statements in A",
            "__vtable_A#(THIS^.__vtable^).alpha^(THIS^)",
            "// Statements in A.alpha",
            "// Statements in B",
            "__vtable_B#(THIS^.__A.__vtable^).alpha^(THIS^)",
            "__vtable_B#(THIS^.__A.__vtable^).bravo^(THIS^)",
            "// Statements in B.bravo",
            "__vtable_B#(THIS^.__A.__vtable^).alpha^(THIS^)",
            "// Statements in C",
            "__vtable_C#(THIS^.__B.__A.__vtable^).alpha^(THIS^)",
            "__vtable_C#(THIS^.__B.__A.__vtable^).bravo^(THIS^)",
            "__vtable_C#(THIS^.__B.__A.__vtable^).charlie^(THIS^)",
            "// Statements in C.bravo",
            "__vtable_C#(THIS^.__B.__A.__vtable^).alpha^(THIS^)",
            "__vtable_C#(THIS^.__B.__A.__vtable^).charlie^(THIS^)",
            "// Statements in C.charlie",
            "__vtable_C#(THIS^.__B.__A.__vtable^).alpha^(THIS^)",
            "__vtable_C#(THIS^.__B.__A.__vtable^).bravo^(THIS^)",
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["A"]), @r#"
        [
            "// Statements in A",
            "__vtable_A#(THIS^.__vtable^).bravo^(THIS^, __vtable_A#(THIS^.__vtable^).alpha^(THIS^))",
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["main", "operateOnA"]), @r#"
        [
            "// Statements in main",
            "__vtable_A#(refInstanceA^.__vtable^).alpha^(refInstanceA^)",
            "__vtable_B#(refInstanceB^.__A.__vtable^).bravo^(refInstanceB^)",
            "__vtable_C#(refInstanceC^.__B.__A.__vtable^).charlie^(refInstanceC^)",
            "__vtable_A#(refInstanceArrayA[1]^.__vtable^).alpha^(refInstanceArrayA[1]^)",
            "__vtable_B#(refInstanceArrayB[1]^.__A.__vtable^).bravo^(refInstanceArrayB[1]^)",
            "__vtable_C#(refInstanceArrayC[1]^.__B.__A.__vtable^).charlie^(refInstanceArrayC[1]^)",
            "// Statements in operateOnA",
            "__vtable_A#(refInstanceA^.__vtable^).alpha^(refInstanceA^)",
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["main"]), @r#"
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["main"]), @r#"
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

                refInstanceA();
                refInstanceA.refB();
                refInstanceA.refB.refC();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(desugared_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__vtable_A#(refInstanceA.__vtable^).__body^(refInstanceA)",
            "__vtable_A#(refInstanceA.refB.__vtable^).__body^(refInstanceA.refB)",
            "__vtable_B#(refInstanceA.refB.refC.__vtable^).__body^(refInstanceA.refB.refC)",
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

        insta::assert_debug_snapshot!(desugared_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__vtable_A#(refInstanceA.__vtable^).__body^(refInstanceA)",
            "__vtable_A#(refInstanceA.refB.__vtable^).__body^(refInstanceA.refB)",
            "__vtable_B#(refInstanceA.refB.refC.__vtable^).__body^(refInstanceA.refB.refC)",
        ]
        "#);
    }
}
