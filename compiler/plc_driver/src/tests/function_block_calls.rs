use plc_diagnostics::{diagnostician::Diagnostician, reporter::DiagnosticReporter};
use plc_index::GlobalContext;
use project::project::Project;
use source_code::SourceCode;

use crate::{
    pipelines::{AnnotatedProject, BuildPipeline, Pipeline},
    tests::get_pipeline,
    CompileOptions,
};

fn annotate_source(source: &str, severity: Option<&str>) -> (BuildPipeline<SourceCode>, AnnotatedProject) {
    let project = Project::new("fb_calls".into()).with_sources([SourceCode::new(source, "fb_calls.st")]);
    let context = GlobalContext::new().with_source(project.get_sources(), None).unwrap();
    let mut pipeline = get_pipeline(project, context);
    let diagnostician = Diagnostician::buffered();
    pipeline.diagnostician = match severity {
        Some(severity) => diagnostician
            .with_configuration(serde_json::from_str(&format!(r#"{{"{severity}":["E156"]}}"#)).unwrap()),
        None => diagnostician,
    };
    let parsed = pipeline.parse().unwrap();
    let indexed = pipeline.index(parsed).unwrap();
    let annotated = pipeline.annotate(indexed).unwrap();
    (pipeline, annotated)
}

#[test]
fn function_block_type_calls_stop_at_validation_by_default() {
    for operator in ["MyFb", "FbAlias"] {
        let source = format!(
            "FUNCTION_BLOCK MyFb
                VAR_INPUT x : DINT; END_VAR
            END_FUNCTION_BLOCK
            TYPE FbAlias : MyFb; END_TYPE
            FUNCTION main : DINT
                {operator}(x := 1);
            END_FUNCTION"
        );
        let (mut pipeline, annotated) = annotate_source(&source, None);

        let error = annotated.validate(&pipeline.context, &mut pipeline.diagnostician).unwrap_err();
        assert_eq!(error.get_message(), "Compilation aborted due to critical errors");
        let diagnostics = pipeline.diagnostician.buffer().unwrap();
        assert!(diagnostics.contains("error[E156]"), "{diagnostics}");
        assert!(!diagnostics.contains("E048"), "{diagnostics}");
    }
}

#[test]
fn downgraded_missing_instance_diagnostic_does_not_make_type_calls_compilable() {
    for severity in ["warning", "info", "ignore"] {
        for (operator, error_code) in [("MyFb", "E048"), ("FbAlias", "E071")] {
            let source = format!(
                "FUNCTION_BLOCK MyFb
                    VAR_INPUT x : DINT; END_VAR
                END_FUNCTION_BLOCK
                TYPE FbAlias : MyFb; END_TYPE
                FUNCTION main : DINT
                    {operator}(x := 1);
                END_FUNCTION"
            );
            let (mut pipeline, annotated) = annotate_source(&source, Some(severity));

            annotated.validate(&pipeline.context, &mut pipeline.diagnostician).unwrap();
            let diagnostics = pipeline.diagnostician.buffer().unwrap();
            if severity == "ignore" {
                assert!(diagnostics.is_empty(), "{diagnostics}");
            } else {
                assert!(diagnostics.contains("E156"), "{diagnostics}");
            }
            let error = annotated.codegen_to_string(&CompileOptions::default()).unwrap_err();
            assert_eq!(error.get_error_code(), error_code, "{severity}, {operator}: {error:?}");
        }
    }
}

#[test]
fn named_instance_calls_compile_through_aliases_and_qualified_references() {
    let source = "
        FUNCTION_BLOCK MyFb
            VAR_INPUT x : DINT; END_VAR
            VAR_OUTPUT y : DINT; END_VAR
            y := x;
        END_FUNCTION_BLOCK
        TYPE FbAlias : MyFb; END_TYPE
        TYPE SecondAlias : FbAlias; END_TYPE
        VAR_GLOBAL MyFb : MyFb; END_VAR
        PROGRAM holder
            VAR_INPUT inst : SecondAlias; END_VAR
        END_PROGRAM
        FUNCTION main : DINT
            VAR inst : FbAlias; END_VAR
            inst(x := 1, y => main);
            holder.inst(x := 2, y => main);
            MyFb(x := 3, y => main);
            .MyFb(x := 4, y => main);
        END_FUNCTION
    ";
    let (mut pipeline, annotated) = annotate_source(source, None);

    annotated.validate(&pipeline.context, &mut pipeline.diagnostician).unwrap();
    assert_eq!(pipeline.diagnostician.buffer().unwrap(), "");
    let modules = annotated.codegen_to_string(&CompileOptions::default()).unwrap();
    assert_eq!(modules.len(), 1);
}
