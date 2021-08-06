use core::panic;
use std::ops::Range;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    ast::*,
    parser::{
        parse,
        tests::{empty_stmt, lex},
    },
};
use pretty_assertions::*;

#[test]
fn empty_returns_empty_compilation_unit() {
    let (result, ..) = parse(lex(""));
    assert_eq!(result.units.len(), 0);
}

#[test]
fn programs_can_be_external() {
    let lexer = lex("@EXTERNAL PROGRAM foo END_PROGRAM");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];
    assert_eq!(LinkageType::External, implementation.linkage);
}

#[test]
fn ids_are_assigned_to_parsed_literals() {
    let lexer = lex("
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
    ");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];

    assert_eq!(implementation.statements[0].get_id(), 1);
    assert_eq!(implementation.statements[1].get_id(), 2);
    assert_eq!(implementation.statements[2].get_id(), 3);
    assert_eq!(implementation.statements[3].get_id(), 4);
    assert_eq!(implementation.statements[4].get_id(), 5);
    assert_eq!(implementation.statements[5].get_id(), 6);
    assert_eq!(implementation.statements[6].get_id(), 7);
    assert_eq!(implementation.statements[7].get_id(), 8);
    assert_eq!(implementation.statements[8].get_id(), 9);
}

#[test]
fn ids_are_assigned_to_parsed_assignments() {
    let lexer = lex("
    PROGRAM PRG
        a := b;
    END_PROGRAM
    ");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];

    if let Statement::Assignment { id, left, right } = &implementation.statements[0] {
        assert_eq!(left.get_id(), 1);
        assert_eq!(right.get_id(), 2);
        assert_eq!(*id, 3);
    } else {
        panic!("unexpected statement");
    }
}

#[test]
fn ids_are_assigned_to_callstatements() {
    let lexer = lex("
    PROGRAM PRG
        foo();
        foo(1,2,3);
        foo(a := 1, b => c, d);
    END_PROGRAM
    ");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];

    if let Statement::CallStatement { id, operator, .. } = &implementation.statements[0] {
        assert_eq!(operator.get_id(), 1);
        assert_eq!(*id, 2);
    } else {
        panic!("unexpected statement");
    }

    if let Statement::CallStatement {
        id,
        operator,
        parameters,
        ..
    } = &implementation.statements[1]
    {
        assert_eq!(operator.get_id(), 3);
        if let Some(Statement::ExpressionList { expressions, id }) = &**parameters {
            assert_eq!(expressions[0].get_id(), 4);
            assert_eq!(expressions[1].get_id(), 5);
            assert_eq!(expressions[2].get_id(), 6);
            assert_eq!(*id, 7);
        }
        assert_eq!(*id, 8);
    } else {
        panic!("unexpected statement");
    }

    if let Statement::CallStatement {
        id,
        operator,
        parameters,
        ..
    } = &implementation.statements[2]
    {
        assert_eq!(operator.get_id(), 9);
        if let Some(Statement::ExpressionList { expressions, id }) = &**parameters {
            if let Statement::Assignment {
                left, right, id, ..
            } = &expressions[0]
            {
                assert_eq!(left.get_id(), 10);
                assert_eq!(right.get_id(), 11);
                assert_eq!(*id, 12);
            } else {
                panic!("unexpected statement");
            }
            if let Statement::OutputAssignment {
                left, right, id, ..
            } = &expressions[1]
            {
                assert_eq!(left.get_id(), 13);
                assert_eq!(right.get_id(), 14);
                assert_eq!(*id, 15);
            } else {
                panic!("unexpected statement");
            }
            assert_eq!(expressions[2].get_id(), 16);
            assert_eq!(*id, 17);
        }
        assert_eq!(*id, 18);
    } else {
        panic!("unexpected statement");
    }
}

#[test]
fn ids_are_assigned_to_expressions() {
    let lexer = lex("
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
    ");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];

    if let Statement::BinaryExpression {
        id, left, right, ..
    } = &implementation.statements[0]
    {
        assert_eq!(left.get_id(), 1);
        assert_eq!(right.get_id(), 2);
        assert_eq!(*id, 3);
    } else {
        panic!("unexpected statement");
    }

    if let Statement::QualifiedReference { id, elements, .. } = &implementation.statements[1] {
        assert_eq!(elements[0].get_id(), 4);
        assert_eq!(elements[1].get_id(), 5);
        assert_eq!(*id, 6);
    } else {
        panic!("unexpected statement");
    }

    if let Statement::Reference { id, .. } = &implementation.statements[2] {
        assert_eq!(*id, 7);
    } else {
        panic!("unexpected statement");
    }

    if let Statement::ArrayAccess {
        id,
        reference,
        access,
        ..
    } = &implementation.statements[3]
    {
        assert_eq!(reference.get_id(), 8);
        assert_eq!(access.get_id(), 9);
        assert_eq!(*id, 10);
    } else {
        panic!("unexpected statement");
    }

    if let Statement::UnaryExpression { id, value, .. } = &implementation.statements[4] {
        assert_eq!(value.get_id(), 11);
        assert_eq!(*id, 12);
    } else {
        panic!("unexpected statement");
    }

    if let Statement::ExpressionList {
        id, expressions, ..
    } = &implementation.statements[5]
    {
        assert_eq!(expressions[0].get_id(), 13);
        assert_eq!(expressions[1].get_id(), 14);
        assert_eq!(*id, 15);
    } else {
        panic!("unexpected statement");
    }

    if let Statement::RangeStatement { id, start, end, .. } = &implementation.statements[6] {
        assert_eq!(start.get_id(), 16);
        assert_eq!(end.get_id(), 17);
        assert_eq!(*id, 18);
    } else {
        panic!("unexpected statement");
    }

    if let Statement::MultipliedStatement { id, element, .. } = &implementation.statements[7] {
        assert_eq!(element.get_id(), 19);
        assert_eq!(*id, 20);
    } else {
        panic!("unexpected statement");
    }
}

#[test]
fn ids_are_assigned_to_if_statements() {
    let lexer = lex("
    PROGRAM PRG
        IF TRUE THEN
            ;
        ELSE    
            ;
        END_IF
    END_PROGRAM
    ");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];

    match &implementation.statements[0] {
        Statement::IfStatement {
            blocks, else_block, ..
        } => {
            assert_eq!(blocks[0].condition.get_id(), 1);
            assert_eq!(blocks[0].body[0].get_id(), 2);
            assert_eq!(else_block[0].get_id(), 3);
            assert_eq!(implementation.statements[0].get_id(), 4);
        }
        _ => panic!("invalid statement"),
    }
}

#[test]
fn ids_are_assigned_to_for_statements() {
    let lexer = lex("
    PROGRAM PRG
        FOR x := 1 TO 7 BY 2 DO
            ;
            ;
            ;
        END_FOR;
    END_PROGRAM
    ");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];

    match &implementation.statements[0] {
        Statement::ForLoopStatement {
            counter,
            start,
            end,
            by_step,
            id,
            body,
            ..
        } => {
            assert_eq!(counter.get_id(), 1);
            assert_eq!(start.get_id(), 2);
            assert_eq!(end.get_id(), 3);
            assert_eq!(by_step.as_ref().unwrap().get_id(), 4);
            assert_eq!(body[0].get_id(), 5);
            assert_eq!(body[1].get_id(), 6);
            assert_eq!(body[2].get_id(), 7);
            assert_eq!(*id, 8);
        }
        _ => panic!("invalid statement"),
    }
}

#[test]
fn ids_are_assigned_to_while_statements() {
    let lexer = lex("
    PROGRAM PRG
       WHILE TRUE DO
            ;;
        END_WHILE
    END_PROGRAM
    ");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];

    match &implementation.statements[0] {
        Statement::WhileLoopStatement {
            condition, body, ..
        } => {
            assert_eq!(condition.get_id(), 1);
            assert_eq!(body[0].get_id(), 2);
            assert_eq!(body[1].get_id(), 3);
            assert_eq!(implementation.statements[0].get_id(), 4);
        }
        _ => panic!("invalid statement"),
    }
}

#[test]
fn ids_are_assigned_to_repeat_statements() {
    let lexer = lex("
    PROGRAM PRG
       REPEAT
            ;;
       UNTIL TRUE END_REPEAT
    END_PROGRAM
    ");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];

    match &implementation.statements[0] {
        Statement::RepeatLoopStatement {
            condition, body, ..
        } => {
            assert_eq!(body[0].get_id(), 1);
            assert_eq!(body[1].get_id(), 2);
            assert_eq!(condition.get_id(), 3);
            assert_eq!(implementation.statements[0].get_id(), 4);
        }
        _ => panic!("invalid statement"),
    }
}

#[test]
fn ids_are_assigned_to_case_statements() {
    let lexer = lex("
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
    ");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];

    match &implementation.statements[0] {
        Statement::CaseStatement {
            case_blocks,
            else_block,
            selector,
            ..
        } => {
            //1st case block
            assert_eq!(selector.get_id(), 1);
            assert_eq!(case_blocks[0].condition.get_id(), 2);
            assert_eq!(case_blocks[0].body[0].get_id(), 4);

            //2nd case block
            if let Statement::ExpressionList {
                expressions, id, ..
            } = case_blocks[1].condition.as_ref()
            {
                assert_eq!(expressions[0].get_id(), 5);
                assert_eq!(expressions[1].get_id(), 6);
                assert_eq!(*id, 7);
            } else {
                panic!("expected expression list")
            }
            assert_eq!(case_blocks[1].body[0].get_id(), 9);

            //else block
            assert_eq!(else_block[0].get_id(), 10);
        }

        _ => panic!("invalid statement"),
    }
}

#[test]
fn id_implementation_for_all_statements() {
    assert_eq!(
        Statement::ArrayAccess {
            access: Box::new(empty_stmt()),
            reference: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::Assignment {
            left: Box::new(empty_stmt()),
            right: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::BinaryExpression {
            left: Box::new(empty_stmt()),
            right: Box::new(empty_stmt()),
            operator: Operator::And,
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::BinaryExpression {
            left: Box::new(empty_stmt()),
            right: Box::new(empty_stmt()),
            operator: Operator::And,
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::CallStatement {
            operator: Box::new(empty_stmt()),
            parameters: Box::new(None),
            id: 7,
            location: (1..5).into()
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::CaseCondition {
            condition: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::CaseStatement {
            selector: Box::new(empty_stmt()),
            case_blocks: vec![],
            else_block: vec![],
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::EmptyStatement {
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::ExpressionList {
            expressions: vec![],
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::ForLoopStatement {
            body: vec![],
            by_step: None,
            counter: Box::new(empty_stmt()),
            end: Box::new(empty_stmt()),
            start: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::IfStatement {
            blocks: vec![],
            else_block: vec![],
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::LiteralArray {
            elements: None,
            location: (1..5).into(),
            id: 7,
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::LiteralBool {
            value: true,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::LiteralInteger {
            value: 7,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::LiteralDate {
            day: 0,
            month: 0,
            year: 0,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::LiteralDateAndTime {
            day: 0,
            month: 0,
            year: 0,
            hour: 0,
            milli: 0,
            min: 0,
            sec: 0,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::LiteralReal {
            value: "2.3".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::LiteralString {
            is_wide: false,
            value: "2.3".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::LiteralTime {
            day: 0.0,
            hour: 0.0,
            milli: 0.0,
            min: 0.0,
            sec: 0.0,
            micro: 0.0,
            nano: 0,
            negative: false,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::LiteralTimeOfDay {
            hour: 0,
            min: 0,
            sec: 0,
            milli: 0,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::MultipliedStatement {
            element: Box::new(empty_stmt()),
            multiplier: 9,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::OutputAssignment {
            left: Box::new(empty_stmt()),
            right: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::QualifiedReference {
            elements: vec![],
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::RangeStatement {
            start: Box::new(empty_stmt()),
            end: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::Reference {
            name: "ab".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::RepeatLoopStatement {
            body: vec![],
            condition: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::UnaryExpression {
            operator: Operator::Minus,
            value: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        Statement::WhileLoopStatement {
            body: vec![],
            condition: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
}

fn at(location: Range<usize>) -> Statement {
    Statement::EmptyStatement {
        id: 7,
        location: location.into(),
    }
}

#[test]
fn location_implementation_for_all_statements() {
    assert_eq!(
        Statement::ArrayAccess {
            reference: Box::new(at(0..1)),
            access: Box::new(at(2..4)),
            id: 7
        }
        .get_location(),
        (0..4).into()
    );
    assert_eq!(
        Statement::Assignment {
            left: Box::new(at(0..2)),
            right: Box::new(at(3..8)),
            id: 7
        }
        .get_location(),
        (0..8).into()
    );
    assert_eq!(
        Statement::BinaryExpression {
            left: Box::new(at(0..2)),
            right: Box::new(at(3..8)),
            operator: Operator::And,
            id: 7
        }
        .get_location(),
        (0..8).into()
    );
    assert_eq!(
        Statement::CallStatement {
            operator: Box::new(empty_stmt()),
            parameters: Box::new(None),
            id: 7,
            location: (1..5).into()
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::CaseCondition {
            condition: Box::new(at(2..4)),
            id: 7
        }
        .get_location(),
        (2..4).into()
    );
    assert_eq!(
        Statement::CaseStatement {
            selector: Box::new(empty_stmt()),
            case_blocks: vec![],
            else_block: vec![],
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::EmptyStatement {
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::ExpressionList {
            expressions: vec![at(0..3), at(4..8)],
            id: 7
        }
        .get_location(),
        (0..8).into()
    );
    assert_eq!(
        Statement::ForLoopStatement {
            body: vec![],
            by_step: None,
            counter: Box::new(empty_stmt()),
            end: Box::new(empty_stmt()),
            start: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::IfStatement {
            blocks: vec![],
            else_block: vec![],
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::LiteralArray {
            elements: None,
            location: (1..5).into(),
            id: 7,
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::LiteralBool {
            value: true,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::LiteralInteger {
            value: 7,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::LiteralDate {
            day: 0,
            month: 0,
            year: 0,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::LiteralDateAndTime {
            day: 0,
            month: 0,
            year: 0,
            hour: 0,
            milli: 0,
            min: 0,
            sec: 0,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::LiteralReal {
            value: "2.3".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::LiteralString {
            is_wide: false,
            value: "2.3".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::LiteralTime {
            day: 0.0,
            hour: 0.0,
            milli: 0.0,
            min: 0.0,
            sec: 0.0,
            micro: 0.0,
            nano: 0,
            negative: false,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::LiteralTimeOfDay {
            hour: 0,
            min: 0,
            sec: 0,
            milli: 0,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::MultipliedStatement {
            element: Box::new(empty_stmt()),
            multiplier: 9,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::OutputAssignment {
            left: Box::new(at(0..3)),
            right: Box::new(at(4..9)),
            id: 7
        }
        .get_location(),
        (0..9).into()
    );
    assert_eq!(
        Statement::QualifiedReference {
            elements: vec![at(0..3), at(4..5)],
            id: 7
        }
        .get_location(),
        (0..5).into()
    );
    assert_eq!(
        Statement::RangeStatement {
            start: Box::new(at(0..3)),
            end: Box::new(at(6..9)),
            id: 7
        }
        .get_location(),
        (0..9).into()
    );
    assert_eq!(
        Statement::Reference {
            name: "ab".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::RepeatLoopStatement {
            body: vec![],
            condition: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::UnaryExpression {
            operator: Operator::Minus,
            value: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        Statement::WhileLoopStatement {
            body: vec![],
            condition: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
}
