use driver::parse_and_annotate;
use driver::pipelines::AnnotatedProject;
use plc_ast::ast::CompilationUnit;
use plc_source::SourceCode;

#[test]
fn overriden_method_is_annotated() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK base
            METHOD foo : BOOL
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK derived EXTENDS base
            METHOD foo : BOOL
            END_METHOD
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, AnnotatedProject { units, index, annotations }) =
        parse_and_annotate("TestProject", vec![src]).unwrap();
    let unit: &CompilationUnit = units[0].get_unit();
    let annotations: AstAnnotations = annotations;
    let index: Index = index;
}
