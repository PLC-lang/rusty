use crate::{
    ast::AstStatement,
    index::VariableType,
    lexer::IdProvider,
    resolver::{AnnotationMap, StatementAnnotation},
    test_utils::tests::{annotate_with_ids, index_with_ids},
};

// some helper macros to get more concise tests:
macro_rules! annotate {
    ($src:expr) => {{
        let id_provider = IdProvider::default();
        let (mut cu, mut index) = index_with_ids($src, id_provider.clone());
        let annotations = annotate_with_ids(&cu, &mut index, id_provider);
        (std::mem::take(&mut cu.implementations[0].statements), annotations, index)
    }};
}

macro_rules! deconstruct_assignment {
    ($src:expr) => {{
        if let AstStatement::Assignment{left, right, ..} = $src {
            (left, right)
        } else {
            unreachable!();
        }
    }};
}

/// # Architecture Design Record: Annotated AST
///
/// The rusty parser creates an AST. After everything was indexed, the AST is revisited and
/// `annotated`. While the AST itself only represents the source code from a syntactical point
/// of view, the annotations add semantics to single AST-elements. The semantic information
/// allows for much easier validation and code-generation.
///
/// With the help of annotations we can easily find out the target of references,
/// and their types.
#[test]
fn variables_are_annotated() {
    // parse and index
    let (statements, annotations, ..) = annotate!(
        r#"
    PROGRAM prg 
        VAR
            a : SINT;
            b : SINT;
        END_VAR

        a := gX;
        b := 4;
    END_PROGRAM

    VAR_GLOBAL constant
        gX : INT := 7;
    END_VAR
    "#
    );

    // lets look at a := gX;
    let (a, gX) = deconstruct_assignment!(&statements[0]);
    // the left side is a reference to the Variable prg.a
    assert_eq!(
        annotations.get(a).unwrap(),
        &StatementAnnotation::Variable {
            resulting_type: "SINT".into(),
            qualified_name: "prg.a".into(),
            constant: false,
            variable_type: VariableType::Local,
            is_auto_deref: false
        }
    );

    // the right side is a reference to the global variable gX
    assert_eq!(
        annotations.get(gX).unwrap(),
        &StatementAnnotation::Variable {
            resulting_type: "INT".into(),
            qualified_name: "gX".into(),
            constant: true,
            variable_type: VariableType::Global,
            is_auto_deref: false
        }
    );


}
