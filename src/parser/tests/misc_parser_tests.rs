// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use core::panic;

use crate::test_utils::tests::parse;
use insta::assert_debug_snapshot;
use plc_ast::{
    ast::{
        Assignment, AstNode, AstStatement, BinaryExpression, CallStatement, LinkageType, ReferenceAccess,
        ReferenceExpr, UnaryExpression,
    },
    control_statements::{AstControlStatement, CaseStatement, ForLoopStatement, IfStatement, LoopStatement},
};
use pretty_assertions::*;
use rustc_hash::FxHashSet;

#[test]
fn empty_returns_empty_compilation_unit() {
    let (result, ..) = parse("");
    assert_eq!(result.units.len(), 0);
}

#[test]
fn programs_can_be_external() {
    let src = "@EXTERNAL PROGRAM foo END_PROGRAM";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    assert_eq!(LinkageType::External, implementation.linkage);
}

#[test]
fn exponent_literals_parsed_as_variables() {
    let src = "
            FUNCTION E1 : E2
            VAR_INPUT
            E3 : E4;
            END_VAR
            E5 := 1.0E6;
            END_FUNCTION
           ";

    let (parse_result, diagnostics) = parse(src);

    let pou = &parse_result.units[0];
    assert_debug_snapshot!(pou);
    let implementation = &parse_result.implementations[0];
    assert_debug_snapshot!(implementation);
    assert!(diagnostics.is_empty());
}

#[test]
fn ids_are_assigned_to_parsed_literals() {
    let src = "
    PROGRAM PRG
        ;
        (* literals *)
        1;
        D#2021-10-01;
        DT#2021-10-01-20:15:00;
        TOD#23:59:59.999;
        TIME#2d4h6m8s10ms;
        3.1415;
        TRUE;
        'abc';
        [1,2,3];
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    let mut ids = FxHashSet::default();
    assert!(ids.insert(implementation.statements[0].get_id()));
    assert!(ids.insert(implementation.statements[1].get_id()));
    assert!(ids.insert(implementation.statements[2].get_id()));
    assert!(ids.insert(implementation.statements[3].get_id()));
    assert!(ids.insert(implementation.statements[4].get_id()));
    assert!(ids.insert(implementation.statements[5].get_id()));
    assert!(ids.insert(implementation.statements[6].get_id()));
    assert!(ids.insert(implementation.statements[7].get_id()));
    assert!(ids.insert(implementation.statements[8].get_id()));
}

#[test]
fn ids_are_assigned_to_parsed_assignments() {
    let src = "
    PROGRAM PRG
        a := b;
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    let mut ids = FxHashSet::default();

    if let AstStatement::Assignment(Assignment { left, right }) = &implementation.statements[0].get_stmt() {
        assert!(ids.insert(left.get_id()));
        assert!(ids.insert(right.get_id()));
        assert!(ids.insert(implementation.statements[0].get_id()));
    } else {
        panic!("unexpected statement");
    }
}

#[test]
fn ids_are_assigned_to_callstatements() {
    let src = "
    PROGRAM PRG
    foo();
    foo(1,2,3);
    foo(a := 1, b => c, d);
    END_PROGRAM
    ";

    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    let mut ids = FxHashSet::default();
    if let AstStatement::CallStatement(CallStatement { operator, .. }, ..) =
        &implementation.statements[0].get_stmt()
    {
        assert!(ids.insert(operator.get_id()));
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::CallStatement(CallStatement { operator, parameters, .. }, ..) =
        &implementation.statements[1].get_stmt()
    {
        assert!(ids.insert(operator.get_id()));
        if let Some(AstNode { stmt: AstStatement::ExpressionList(expressions), id, .. }) =
            parameters.as_deref()
        {
            assert!(ids.insert(expressions[0].get_id()));
            assert!(ids.insert(expressions[1].get_id()));
            assert!(ids.insert(expressions[2].get_id()));
            assert!(ids.insert(*id));
        }
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::CallStatement(CallStatement { operator, parameters }, ..) =
        &implementation.statements[2].get_stmt()
    {
        assert!(ids.insert(operator.get_id()));
        if let Some(AstNode { stmt: AstStatement::ExpressionList(expressions), id, .. }) =
            parameters.as_deref()
        {
            if let AstNode { stmt: AstStatement::Assignment(Assignment { left, right }), id, .. } =
                &expressions[0]
            {
                assert!(ids.insert(left.get_id()));
                assert!(ids.insert(right.get_id()));
                assert!(ids.insert(*id));
            } else {
                panic!("unexpected statement");
            }
            if let AstNode { stmt: AstStatement::OutputAssignment(Assignment { left, right }), id, .. } =
                &expressions[1]
            {
                assert!(ids.insert(left.get_id()));
                assert!(ids.insert(right.get_id()));
                assert!(ids.insert(*id));
            } else {
                panic!("unexpected statement");
            }
            assert!(ids.insert(expressions[2].get_id()));
            assert!(ids.insert(*id));
        }
    } else {
        panic!("unexpected statement");
    }

    for s in &implementation.statements {
        assert!(ids.insert(s.get_id()));
    }
}

#[test]
fn ids_are_assigned_to_expressions() {
    let src = "
    PROGRAM PRG
        a * b;
        a.b;
        a;
        a[2];
        -b;
        a,b;
        1..2;
        5(a);
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    let mut ids = FxHashSet::default();

    if let AstNode {
        id, stmt: AstStatement::BinaryExpression(BinaryExpression { left, right, .. }), ..
    } = &implementation.statements[0]
    {
        assert!(ids.insert(left.get_id()));
        assert!(ids.insert(right.get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstNode {
        stmt:
            AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(m), base: Some(base) }),
        id,
        ..
    } = &implementation.statements[1]
    {
        assert!(ids.insert(*id));
        assert!(ids.insert(m.get_id()));

        if let AstNode {
            stmt:
                AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(m), base: None }),
            ..
        } = base.as_ref()
        {
            assert!(ids.insert(m.get_id()));
        } else {
            panic!("unexpected statement");
        }
    } else {
        panic!("unexpected statement");
    }

    if let AstNode {
        stmt: AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(m), base: None }),
        id,
        ..
    } = &implementation.statements[2]
    {
        assert!(ids.insert(*id));
        assert!(ids.insert(m.get_id()));
    } else {
        panic!("unexpected statement");
    }

    if let AstNode {
        stmt:
            AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Index(access),
                base: Some(reference),
            }),
        id,
        ..
    } = &implementation.statements[3]
    {
        assert!(ids.insert(reference.get_id()));
        assert!(ids.insert(access.get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstNode { stmt: AstStatement::UnaryExpression(UnaryExpression { value, .. }), id, .. } =
        &implementation.statements[4]
    {
        assert!(ids.insert(value.get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstNode { stmt: AstStatement::ExpressionList(expressions, ..), id, .. } =
        &implementation.statements[5]
    {
        assert!(ids.insert(expressions[0].get_id()));
        assert!(ids.insert(expressions[1].get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstNode { stmt: AstStatement::RangeStatement(data, ..), id, .. } = &implementation.statements[6] {
        assert!(ids.insert(data.start.get_id()));
        assert!(ids.insert(data.end.get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstNode { stmt: AstStatement::MultipliedStatement(data, ..), id, .. } =
        &implementation.statements[7]
    {
        assert!(ids.insert(data.element.get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }
}

#[test]
fn ids_are_assigned_to_if_statements() {
    let src = "
    PROGRAM PRG
        IF TRUE THEN
            ;
        ELSE
            ;
        END_IF
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    let mut ids = FxHashSet::default();
    match &implementation.statements[0] {
        AstNode {
            stmt:
                AstStatement::ControlStatement(AstControlStatement::If(IfStatement {
                    blocks, else_block, ..
                })),
            ..
        } => {
            assert!(ids.insert(blocks[0].condition.get_id()));
            assert!(ids.insert(blocks[0].body[0].get_id()));
            assert!(ids.insert(else_block[0].get_id()));
            assert!(ids.insert(implementation.statements[0].get_id()));
        }
        _ => panic!("invalid statement"),
    }
}

#[test]
fn ids_are_assigned_to_for_statements() {
    let src = "
    PROGRAM PRG
        FOR x := 1 TO 7 BY 2 DO
            ;
            ;
            ;
        END_FOR;
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    let mut ids = FxHashSet::default();
    match &implementation.statements[0] {
        AstNode {
            stmt:
                AstStatement::ControlStatement(AstControlStatement::ForLoop(ForLoopStatement {
                    counter,
                    start,
                    end,
                    by_step,
                    body,
                    ..
                })),
            id,
            ..
        } => {
            assert!(ids.insert(counter.get_id()));
            assert!(ids.insert(start.get_id()));
            assert!(ids.insert(end.get_id()));
            assert!(ids.insert(by_step.as_ref().unwrap().get_id()));
            assert!(ids.insert(body[0].get_id()));
            assert!(ids.insert(body[1].get_id()));
            assert!(ids.insert(body[2].get_id()));
            assert!(ids.insert(*id));
        }
        _ => panic!("invalid statement"),
    }
}

#[test]
fn ids_are_assigned_to_while_statements() {
    let src = "
    PROGRAM PRG
       WHILE TRUE DO
            ;;
        END_WHILE
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    let mut ids = FxHashSet::default();
    match &implementation.statements[0] {
        AstNode {
            stmt:
                AstStatement::ControlStatement(AstControlStatement::WhileLoop(LoopStatement {
                    condition,
                    body,
                    ..
                })),
            id,
            ..
        } => {
            assert!(ids.insert(condition.get_id()));
            assert!(ids.insert(body[0].get_id()));
            assert!(ids.insert(body[1].get_id()));
            assert!(ids.insert(*id));
        }
        _ => panic!("invalid statement"),
    }
}

#[test]
fn ids_are_assigned_to_repeat_statements() {
    let src = "
    PROGRAM PRG
       REPEAT
            ;;
       UNTIL TRUE END_REPEAT
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    let mut ids = FxHashSet::default();

    match &implementation.statements[0] {
        AstNode {
            stmt:
                AstStatement::ControlStatement(AstControlStatement::RepeatLoop(LoopStatement {
                    condition,
                    body,
                    ..
                })),
            id: _,
            ..
        } => {
            assert!(ids.insert(body[0].get_id()));
            assert!(ids.insert(body[1].get_id()));
            assert!(ids.insert(condition.get_id()));
            assert!(ids.insert(implementation.statements[0].get_id()));
        }
        _ => panic!("invalid statement"),
    }
}

#[test]
fn ids_are_assigned_to_case_statements() {
    let src = "
    PROGRAM PRG
    CASE PumpState OF
    0:
        ;
    1,2:
        ;
    ELSE
        ;
    END_CASE;
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];
    let mut ids = FxHashSet::default();
    match &implementation.statements[0] {
        AstNode {
            stmt:
                AstStatement::ControlStatement(AstControlStatement::Case(CaseStatement {
                    case_blocks,
                    else_block,
                    selector,
                    ..
                })),
            id: _,
            ..
        } => {
            //1st case block
            assert!(ids.insert(selector.get_id()));
            assert!(ids.insert(case_blocks[0].condition.get_id()));
            assert!(ids.insert(case_blocks[0].body[0].get_id()));

            //2nd case block
            if let AstNode { stmt: AstStatement::ExpressionList(expressions), id, .. } =
                case_blocks[1].condition.as_ref()
            {
                assert!(ids.insert(expressions[0].get_id()));
                assert!(ids.insert(expressions[1].get_id()));
                assert!(ids.insert(*id));
            } else {
                panic!("expected expression list")
            }
            assert!(ids.insert(case_blocks[1].body[0].get_id()));

            //else block
            assert!(ids.insert(else_block[0].get_id()));
        }

        _ => panic!("invalid statement"),
    }
}
