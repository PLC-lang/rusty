use std::io::Write;

use plc_ast::{
    ast::{
        AstNode, AstStatement, CompilationUnit, DirectAccess, DirectAccessType, HardwareAccess,
        ReferenceAccess, ReferenceExpr,
    },
    literals::{Array, StringValue},
    visitor::{AstVisitor, Walker},
};

use crate::{
    index::Index,
    typesystem::{
        DataType, DataTypeInformation, BOOL_TYPE, DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, LINT_TYPE,
        REAL_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE,
    },
};

use super::{
    call_statement_resolving::{get_call_name, get_signature_of_callable},
    scope::{Scope, ScopeStack, ScopingStrategy},
    to_variable_annotation, AnnotationMap, AnnotationMapImpl, StatementAnnotation,
};

/// Annotates the AST with the declared types of literals and references
/// This is a first step to infer the types of the AST elements. It aims to annotate the types by directly looking at the declarations.
/// It does not try to annotate clever types to avoid casts, or to infer real types from the context (e.g. the right side of an assignment).
/// 
/// This is the first step and builds the foundation for upcoming steps of type inference.
pub struct DeclaredTypeAnnotator<'i> {
    /// The annotations map. Records annotations for nodes
    annotations: AnnotationMapImpl,
    /// the index to lookup names
    index: &'i Index,
    /// The scope stack is used to resolve the types of identifiers
    /// It is adapted while navigating the AST
    scope_stack: ScopeStack,
}

impl<'i> DeclaredTypeAnnotator<'i> {

    /// Annotates the given compilation unit with the initial types of the literals
    /// This method acts as an entry point for the declared_type_annotator.`
    pub fn visit_unit(unit: &CompilationUnit, index: &'i Index) -> AnnotationMapImpl {
        let mut annotator = DeclaredTypeAnnotator {
            annotations: AnnotationMapImpl::default(),
            index,
            scope_stack: ScopeStack::new(),
        };
        annotator.visit_compilation_unit(unit);
        annotator.annotations
    }

    fn value(&mut self, n: &plc_ast::ast::AstNode, type_name: String) {
        self.annotations.annotate(n, super::StatementAnnotation::Value { resulting_type: type_name });
    }

    fn string(&mut self, len: usize, encoding: crate::typesystem::StringEncoding) -> String {
        let target_type = DataType::new_string_type(len, encoding);
        let name = target_type.name.clone();
        self.annotations.new_index.register_type(target_type);
        name
    }

    fn get_type_name(&self, n: &plc_ast::ast::AstNode) -> Option<&str> {
        self.annotations.get(n).and_then(|a| self.annotations.get_type_name_for_annotation(a))
    }

    fn walk_scoped<T>(&mut self, t: &T, scope: Scope)
    where
        T: Walker,
    {
        self.scope_stack.push(ScopingStrategy::Hierarchical(scope));
        t.walk(self);
        self.scope_stack.pop();
    }

    fn walk_strict_scoped<T>(&mut self, t: &T, scope: Scope)
    where
        T: Walker,
    {
        self.scope_stack.push(ScopingStrategy::Strict(scope));
        t.walk(self);
        self.scope_stack.pop();
    }

    /// Returns the scope of the member access under the give node.
    /// In other words, for a statement like `n.x`, this function returns the scope necessary
    /// to resolve `x`.
    fn get_member_scope(&self, n: &plc_ast::ast::AstNode) -> Scope {
        if let Some(type_name) =
            self.annotations.get(n).and_then(|a| self.annotations.get_type_name_for_annotation(a))
        {
            Scope::Composite(vec![
                Scope::LocalVariable(type_name.to_string()),  // member variable
                Scope::Callable(Some(type_name.to_string())), // action
            ])
        } else {
            Scope::Empty
        }
    }

}

impl<'i> AstVisitor for DeclaredTypeAnnotator<'i> {
    fn visit_implementation(&mut self, implementation: &plc_ast::ast::Implementation) {
        self.walk_scoped(implementation, Scope::LocalVariable(implementation.type_name.clone()));
    }

    fn visit_literal(&mut self, stmt: &plc_ast::literals::AstLiteral, n: &plc_ast::ast::AstNode) {
        stmt.walk(self);

        match stmt {
            plc_ast::literals::AstLiteral::Integer(value) => self.value(
                n,
                if i32::MIN as i128 <= *value && i32::MAX as i128 >= *value {
                    DINT_TYPE.into()
                } else {
                    LINT_TYPE.into()
                },
            ),
            plc_ast::literals::AstLiteral::Date(_) => self.value(n, DATE_TYPE.into()),
            plc_ast::literals::AstLiteral::DateAndTime(_) => self.value(n, DATE_AND_TIME_TYPE.into()),
            plc_ast::literals::AstLiteral::TimeOfDay(_) => self.value(n, TIME_OF_DAY_TYPE.into()),
            plc_ast::literals::AstLiteral::Time(_) => self.value(n, TIME_TYPE.into()),
            plc_ast::literals::AstLiteral::Real(_) => self.value(n, REAL_TYPE.into()),
            plc_ast::literals::AstLiteral::Bool(_) => self.value(n, BOOL_TYPE.into()),
            plc_ast::literals::AstLiteral::String(StringValue { is_wide, value }) => {
                if *is_wide {
                    let type_name = self.string(value.len(), crate::typesystem::StringEncoding::Utf16);
                    self.value(n, type_name);
                } else {
                    let type_name = self.string(value.len(), crate::typesystem::StringEncoding::Utf8);
                    self.value(n, type_name);
                }
            }
            plc_ast::literals::AstLiteral::Array(Array { elements: Some(elements) }) => {
                // the first type of the arraz dominates the array type
                if let Some(type_name) = elements.get_as_list().first().and_then(|it| self.get_type_name(it))
                {
                    self.value(n, type_name.into());
                }
            }
            _ => {}
        }
    }

    fn visit_hardware_access(&mut self, stmt: &HardwareAccess, node: &AstNode) {
        // a hardware access %QX0.1
        self.value(node, get_resulting_direct_access_type(&stmt.access).into());
        stmt.address.iter().for_each(|a| a.walk(self));
    }

    fn visit_reference_expr(&mut self, stmt: &plc_ast::ast::ReferenceExpr, node: &plc_ast::ast::AstNode) {
        stmt.base.as_ref().inspect(|b| b.walk(self));

        match stmt {
            ReferenceExpr { base: None, access: ReferenceAccess::Member(reference) } => {
                // an unqualified variable access
                reference.walk(self);
                self.annotations.copy_annotation(reference, node);
            }
            ReferenceExpr { base: Some(base), access: ReferenceAccess::Member(reference) } => {
                if let AstStatement::DirectAccess(DirectAccess { access, index: da_index }) =
                    reference.get_stmt()
                {
                    // a direct access a.1
                    self.value(node, get_resulting_direct_access_type(access).into());
                    da_index.walk(self);
                } else {
                    // a qualified variable access a.b
                    self.walk_strict_scoped(reference.as_ref(), self.get_member_scope(base));
                    self.annotations.copy_annotation(reference, node);
                }
            }
            ReferenceExpr { base: Some(base), access: ReferenceAccess::Index(idx) } => {
                // an array access - take bases type and remove one array dimension
                //TODO walk under different scope
                idx.walk(self);

                if let Some(DataTypeInformation::Array { inner_type_name, .. }) =
                    self.annotations.get_type(base.as_ref(), &self.index).map(|it| it.get_type_information())
                {
                    self.annotations.annotate(node, super::StatementAnnotation::value(inner_type_name));
                }
            }
            ReferenceExpr { base: Some(base), access: ReferenceAccess::Deref } => {
                // a pointer dereference
                if let Some(DataTypeInformation::Pointer { inner_type_name, auto_deref: None, .. }) =
                    self.annotations.get_type(base.as_ref(), &self.index).map(|it| it.get_type_information())
                {
                    self.annotations.annotate(node, super::StatementAnnotation::value(inner_type_name));
                }
            }
            _ => {}
        }
    }

    fn visit_call_statement(&mut self, stmt: &plc_ast::ast::CallStatement, node: &AstNode) {
        self.walk_scoped(stmt.operator.as_ref(), Scope::Callable(None));
        // annotate the overall callstatement
        // annotate the parameters
        if let Some(signature) = get_call_name(stmt.operator.as_ref(), &self.annotations, self.index)
            .and_then(|name| get_signature_of_callable(name.as_str(), &self.index))
        {
            // anotate type of the call statement
            signature.return_type.inspect(|rt| self.value(node, rt.get_name().to_string()));

            if let Some(parameter_nodes) = stmt.parameters.as_ref() {
                for p in parameter_nodes.get_as_list() {
                    if let AstStatement::Assignment(assignment) = p.get_stmt() {
                        // foo(a:=1, b:=2, c:=3)
                        // we have an assignment, lets walk the right side
                        let left_identifier = assignment.left.as_ref().get_flat_reference_name();
                        if let Some(left_identifier) =
                            left_identifier.and_then(|i| signature.find_parameter(i))
                        {
                            self.annotations.annotate(
                                &assignment.left,
                                to_variable_annotation(left_identifier, &self.index, false),
                            );
                        }

                        assignment.right.walk(self);
                    } else {
                        // implicit call parameter foo(a,b,c)
                        p.walk(self);
                    }
                }
            } else {
                // strange, we cannot find the signature of the callable
                // lets just walk the parameters
                stmt.parameters.as_ref().inspect(|p| p.walk(self));
            }
        }
    }

    fn visit_identifier(&mut self, identifier: &str, node: &AstNode) {
        if let Some(annotation) = self.scope_stack.lookup(identifier, &self.index) {
            self.annotations.annotate(node, annotation);
        }
    }
}

fn get_resulting_direct_access_type(access: &DirectAccessType) -> &'static str {
    match access {
        DirectAccessType::Bit => crate::typesystem::BOOL_TYPE,
        DirectAccessType::Byte => crate::typesystem::BYTE_TYPE,
        DirectAccessType::Word => crate::typesystem::WORD_TYPE,
        DirectAccessType::DWord => crate::typesystem::DWORD_TYPE,
        DirectAccessType::LWord => crate::typesystem::LWORD_TYPE,
        DirectAccessType::Template => crate::typesystem::VOID_TYPE,
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use plc_diagnostics::diagnostics::Diagnostic;

    use crate::{resolver::annotation_printer::AnnotationPrinter, test_utils::tests::index_safe};

    use super::*;

    fn annotate(src: &str, allow_diagnostics: Vec<&str>) -> (CompilationUnit, AnnotationMapImpl, Index) {
        let (unit, index, diagnostics) = index_safe(src);

        let diagnostics: Vec<Diagnostic> = diagnostics
            .into_iter()
            .filter(|d| !allow_diagnostics.iter().any(|it| d.get_error_code().eq_ignore_ascii_case(it)))
            .collect();

        assert_eq!(diagnostics, vec![]); // make sure there are no

        let a = DeclaredTypeAnnotator::visit_unit(&unit, &index);
        (unit, a, index)
    }

    macro_rules! snapshot {
        ($src:expr) => {
            let (unit, annotations, _) = annotate($src, vec![]);
            assert_snapshot!(AnnotationPrinter::print($src, &annotations, &unit));
        };
        ($src:expr, $allow_diagnostics:expr) => {
            let (unit, annotations, _) = annotate($src, $allow_diagnostics);
            assert_snapshot!(AnnotationPrinter::print($src, &annotations, &unit));
        };
    }

    #[test]
    fn numeric_literals_are_annotated() {
        snapshot!(
            r#"PROGRAM PRG
                // bool literals
                TRUE;
                FALSE;
                // int
                0;
                128;
                32768;
                2147483647;
                2147483648;
                // float literals
                0.0;
                3.1415;
            END_PROGRAM"#
        );
    }

    #[test]
    fn string_literals_are_annotated() {
        snapshot!(
            r#"PROGRAM PRG
                // string literals
                'string';
                "wstring";
            END_PROGRAM"#
        );
    }

    #[test]
    fn date_literals_are_annotated() {
        snapshot!(
            "PROGRAM PRG
                T#12.4d;
                TIME#-12m;
                TOD#00:00:12;
                TIME_OF_DAY#04:16:22;
                TIME_OF_DAY#04:16;
                DATE_AND_TIME#1984-10-01-16:40:22;
                DT#2021-04-20-22:33:14;
                DATE_AND_TIME#2000-01-01-20:15;
                DATE#1984-10-01;
                D#2021-04-20;
                LTIME#12.4d;
                LDATE#1984-10-01;
                LDT#1984-10-01-16:40:22;
                LTOD#00:00:12;
            END_PROGRAM"
        );
    }

    #[test]
    fn global_variables_are_annotated() {
        snapshot!(
            "
            VAR_GLOBAL
                a: INT;
                b: BOOL;
                c: BYTE;
                d: REAL;
            END_VAR
            
            PROGRAM PRG
                VAR
                    a: INT;
                    b: BOOL;
                END_VAR
                a;
                b;
                c;
                d;
            END_PROGRAM"
        );
    }

    #[test]
    fn member_variables_are_annotated() {
        snapshot!(
            "
            TYPE Point : STRUCT
                x,y : INT;
            END_STRUCT END_TYPE

            PROGRAM PRG1
                VAR_INPUT
                    a,b,c : INT;
                    p: Point;
                END_VAR
            END_PROGRAM
            
            PROGRAM PRG2
                PRG1.a;
                PRG1.b;
                PRG1.c;
                PRG1.p.x;
                PRG1.p.y;
            END_PROGRAM
            "
        );
    }

    #[test]
    fn array_access_is_annotated() {
        snapshot!(
            "
            PROGRAM PRG
                VAR
                    x : ARRAY[0..10] OF INT;
                    y : ARRAY[0..10] OF ARRAY[0..10] OF BOOL;
                    c : INT;
                END_VAR
                x[c];
                y[0][c];
            END_PROGRAM
            "
        );
    }

    #[test]
    fn pointer_access_is_annotated() {
        snapshot!(
            "
            PROGRAM PRG
                VAR
                    x : POINTER TO INT;
                    y : REF_TO REF_TO BOOL;
                END_VAR
                x^;
                y^^;
            END_PROGRAM
            ",
            vec!["E015"]
        );
    }

    #[test]
    fn call_statements_are_annotated() {
        snapshot!(
            "
            FUNCTION FOO : INT
                VAR_INPUT a: INT; b: BOOL; c: BYTE; END_VAR
            END_FUNCTION

            PROGRAM PRG
                VAR_INPUT a: INT; b: BOOL; c: BYTE; END_VAR
            END_PROGRAM
            
            FUNCTION_BLOCK FB
                VAR_INPUT a: INT; b: BOOL; c: BYTE; END_VAR
            END_FUNCTION_BLOCK
            
            PROGRAM TEST
                VAR fb : FB; END_VAR

                // function calls
                FOO();
                FOO(1, TRUE, 7);
                FOO( a := 1, b := TRUE, c := 7);
                // program calls
                PRG();
                PRG(1, TRUE, 7);
                PRG( a := 1, b := TRUE, c := 7);
                // function block calls
                fb();
                fb(1, TRUE, 7);
                fb( a := 1, b := TRUE, c := 7);
            END_PROGRAM
            "
        );
    }

    #[test]
    fn call_statements_to_actions_are_annotated() {
        snapshot!(
            "
            PROGRAM PRG
                VAR_INPUT a: INT; b: BOOL; c: BYTE; END_VAR
            END_PROGRAM
            ACTIONS ACTION prgAction END_ACTION END_ACTIONS

            FUNCTION_BLOCK FB
                VAR_INPUT a: INT; b: BOOL; c: BYTE; END_VAR
            END_FUNCTION_BLOCK
            ACTIONS ACTION fbAction END_ACTION END_ACTIONS
            
            PROGRAM TEST
                VAR fb : FB; END_VAR
                prg.prgAction(a := 1, b := TRUE, c := 7);      
                fb.fbAction(a := 1, b := TRUE, c := 7);      
            END_PROGRAM
            "
        );
    }

    #[test]
    #[ignore]
    fn casted_literals_are_annotated() {
        let (unit, annotations, index) = annotate(
            "PROGRAM PRG
                SINT#7;
                INT#7;
                DINT#7;
                LINT#7;
                REAL#7.7;
                LREAL#7.7;
                BOOL#1;
                BOOL#FALSE;
            END_PROGRAM",
            vec![],
        );
        let statements = &unit.implementations[0].statements;
        let expected_types = vec!["SINT", "INT", "DINT", "LINT", "REAL", "LREAL", "BOOL", "BOOL"];
        let actual_types: Vec<&str> =
            statements.iter().map(|it| annotations.get_type_or_void(it, &index).get_name()).collect();

        assert_eq!(format!("{expected_types:#?}"), format!("{actual_types:#?}"),)
    }

    #[test]
    fn direct_access_is_annotated() {
        snapshot!(
            "
            PROGRAM PRG
                VAR a, b, c : LWORD; END_VAR
                %MD1.4;   //hw access records wrong range!
                a.%X0;
                b.%W0.%X2;
                c.%L0.%D0;
            END_PROGRAM
            "
        );
    }
}


