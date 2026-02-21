use indoc::indoc;
use itertools::Itertools;

use crate::{
    ast::{AstNode, CompilationUnit, Expression, HasName, Name},
    expect_all,
};

#[test]
fn for_accessors() {
    let src = indoc! {"
            PROGRAM Test
                FOR nCounter := 1 TO 5 BY 1 DO
                    x := 1;
                END_FOR
            END_PROGRAM
        "};
    let cu = CompilationUnit::parse(src).ok().unwrap();
    let pou = cu.pous().next().unwrap();
    let body = pou.body().unwrap();
    let stmt = body.expression_stmts().next().unwrap();
    let Expression::ForStatement(for_stmt) = stmt.expression().unwrap() else {
        panic!("expected ForStatement");
    };

    // counter target is "nCounter"
    let counter = for_stmt.counter().unwrap();
    assert_eq!(counter.target().unwrap().ident_token().unwrap().text(), "nCounter");
    // step (BY 1) is present
    assert!(for_stmt.step().is_some());

    // All keywords are present
    expect_all!(
        for_stmt.FOR_token(),
        for_stmt.TO_token(),
        for_stmt.BY_token(),
        for_stmt.DO_token(),
        for_stmt.END_FOR_token()
    );
}

#[test]
fn parse_different_pous() {
    let text = indoc! {"
            PROGRAM PRG
            END_PROGRAM

            FUNCTION_BLOCK FB
            END_FUNCTION_BLOCK

            FUNCTION FN : INT
            END_FUNCTION
        "};

    let cu = CompilationUnit::parse(text).ok().unwrap();
    let mut pous = dbg!(cu).pous();
    // --- PROGRAM ---
    let program = pous.next().unwrap();
    assert_eq!(program.PouStartKeyword_token().unwrap().text(), "PROGRAM");
    assert_eq!(program.ident_token().unwrap().text(), "PRG");
    assert_eq!(program.PouEndKeyword_token().unwrap().text(), "END_PROGRAM");

    // --- FUNCTION_BLOCK ---
    let fb = pous.next().unwrap();
    assert_eq!(fb.PouStartKeyword_token().unwrap().text(), "FUNCTION_BLOCK");
    assert_eq!(fb.ident_token().unwrap().text(), "FB");
    assert_eq!(fb.PouEndKeyword_token().unwrap().text(), "END_FUNCTION_BLOCK");

    // --- FUNCTION ---
    let func = pous.next().unwrap();
    assert_eq!(func.PouStartKeyword_token().unwrap().text(), "FUNCTION");
    assert_eq!(func.ident_token().unwrap().text(), "FN");
    assert_eq!(func.colon_token().unwrap().text(), ":");
    assert_eq!(func.type_ref().unwrap().ident_token().unwrap().text(), "INT");
    assert_eq!(func.PouEndKeyword_token().unwrap().text(), "END_FUNCTION");

    // no more POUs
    assert!(pous.next().is_none());
}
