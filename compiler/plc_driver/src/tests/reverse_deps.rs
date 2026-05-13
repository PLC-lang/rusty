//! Tests for [`AnnotatedProject::reverse_dependencies`].

use plc::index::UnitId;
use source_code::SourceCode;

use crate::tests::{
    progress_pipeline_to_step_annotated, progress_pipeline_to_step_indexed, progress_pipeline_to_step_parsed,
};

fn build(sources: Vec<SourceCode>) -> crate::tests::AnnotatedProjectWrapper {
    let includes: Vec<SourceCode> = vec![];
    let parsed = progress_pipeline_to_step_parsed(sources.clone(), includes.clone()).expect("parse");
    let indexed =
        progress_pipeline_to_step_indexed(sources.clone(), includes.clone(), parsed).expect("index");
    progress_pipeline_to_step_annotated(sources, includes, indexed).expect("annotate")
}

#[test]
fn reverse_graph_links_caller_to_callee() {
    let callee = SourceCode::new(
        "
        FUNCTION callee : DINT
            VAR_INPUT x : DINT; END_VAR
            callee := x;
        END_FUNCTION
        ",
        "callee.st",
    );
    let caller = SourceCode::new(
        "
        FUNCTION main : DINT
            VAR y : DINT; END_VAR
            y := callee(1);
            main := y;
        END_FUNCTION
        ",
        "caller.st",
    );

    let project = build(vec![callee, caller]).annotated_project;
    let graph = project.reverse_dependencies();

    let dependents = graph.dependents("callee").expect("callee has dependents");
    assert!(dependents.contains(&UnitId::source(1)), "caller (unit 1) depends on callee");
    // The defining unit itself also lists `callee` in its dep set
    // (function bodies record self-reference through the visitor); we don't
    // assert exclusion of unit 0 because that's an implementation detail of
    // the resolver, but unit 1's presence is the load-bearing claim.
}

#[test]
fn reverse_graph_links_struct_users() {
    let define = SourceCode::new(
        "
        TYPE Vec3 : STRUCT x, y, z : REAL; END_STRUCT END_TYPE
        ",
        "types.st",
    );
    let user = SourceCode::new(
        "
        FUNCTION main : DINT
            VAR v : Vec3; END_VAR
            v.x := 1.0;
            main := 0;
        END_FUNCTION
        ",
        "user.st",
    );

    let project = build(vec![define, user]).annotated_project;
    let graph = project.reverse_dependencies();

    let dependents = graph.dependents("Vec3").expect("Vec3 has dependents");
    assert!(dependents.contains(&UnitId::source(1)), "user unit depends on Vec3");
}

#[test]
fn unrelated_unit_does_not_appear_as_dependent() {
    let callee = SourceCode::new(
        "
        FUNCTION callee : DINT
            callee := 0;
        END_FUNCTION
        ",
        "callee.st",
    );
    let unrelated = SourceCode::new(
        "
        FUNCTION other : DINT
            other := 42;
        END_FUNCTION
        ",
        "other.st",
    );

    let project = build(vec![callee, unrelated]).annotated_project;
    let graph = project.reverse_dependencies();

    if let Some(dependents) = graph.dependents("callee") {
        assert!(
            !dependents.contains(&UnitId::source(1)),
            "the unrelated unit must not appear as a dependent of `callee`"
        );
    }
}
