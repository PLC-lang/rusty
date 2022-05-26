// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::Diagnostic;
use core::panic;
use std::ops::Range;

use crate::{ast::*, parser::tests::empty_stmt, test_utils::tests::parse};
use pretty_assertions::*;

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
    let expected = Pou {
        name: "E1".into(),
        pou_type: PouType::Function,
        poly_mode: None,
        return_type: Some(DataTypeDeclaration::DataTypeReference {
            referenced_type: "E2".into(),
            location: SourceRange::undefined(),
        }),
        variable_blocks: vec![VariableBlock {
            variable_block_type: VariableBlockType::Input,
            access: AccessModifier::Internal,
            constant: false,
            retain: false,
            location: SourceRange::undefined(),
            linkage: LinkageType::Internal,
            variables: vec![Variable {
                name: "E3".into(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "E4".into(),
                    location: SourceRange::undefined(),
                },
                initializer: None,
                address: None,
                location: SourceRange::undefined(),
            }],
        }],
        location: SourceRange::undefined(),
        generics: vec![],
        linkage: crate::ast::LinkageType::Internal,
    };
    assert_eq!(format!("{:#?}", expected), format!("{:#?}", pou).as_str());
    let implementation = &parse_result.implementations[0];
    let expected = Implementation {
        name: "E1".into(),
        type_name: "E1".into(),
        linkage: LinkageType::Internal,
        pou_type: PouType::Function,
        statements: vec![AstStatement::Assignment {
            left: Box::new(AstStatement::Reference {
                name: "E5".into(),
                id: 0,
                location: SourceRange::undefined(),
            }),
            right: Box::new(AstStatement::LiteralReal {
                value: "1.0E6".into(),
                id: 0,
                location: SourceRange::undefined(),
            }),
            id: 0,
        }],
        access: None,
        overriding: false,
        generic: false,
        location: (105..142).into(),
    };
    assert_eq!(
        format!("{:#?}", expected),
        format!("{:#?}", implementation).as_str()
    );
    assert_eq!(
        format!("{:#?}", diagnostics),
        format!("{:#?}", Vec::<Diagnostic>::new()).as_str()
    );
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
    let src = "
    PROGRAM PRG
        a := b;
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];

    if let AstStatement::Assignment { id, left, right } = &implementation.statements[0] {
        assert_eq!(left.get_id(), 1);
        assert_eq!(right.get_id(), 2);
        assert_eq!(*id, 3);
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

    if let AstStatement::CallStatement { id, operator, .. } = &implementation.statements[0] {
        assert_eq!(operator.get_id(), 1);
        assert_eq!(*id, 2);
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::CallStatement {
        id,
        operator,
        parameters,
        ..
    } = &implementation.statements[1]
    {
        assert_eq!(operator.get_id(), 3);
        if let Some(AstStatement::ExpressionList { expressions, id }) = &**parameters {
            assert_eq!(expressions[0].get_id(), 4);
            assert_eq!(expressions[1].get_id(), 5);
            assert_eq!(expressions[2].get_id(), 6);
            assert_eq!(*id, 7);
        }
        assert_eq!(*id, 8);
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::CallStatement {
        id,
        operator,
        parameters,
        ..
    } = &implementation.statements[2]
    {
        assert_eq!(operator.get_id(), 9);
        if let Some(AstStatement::ExpressionList { expressions, id }) = &**parameters {
            if let AstStatement::Assignment {
                left, right, id, ..
            } = &expressions[0]
            {
                assert_eq!(left.get_id(), 10);
                assert_eq!(right.get_id(), 11);
                assert_eq!(*id, 12);
            } else {
                panic!("unexpected statement");
            }
            if let AstStatement::OutputAssignment {
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

    if let AstStatement::BinaryExpression {
        id, left, right, ..
    } = &implementation.statements[0]
    {
        assert_eq!(left.get_id(), 1);
        assert_eq!(right.get_id(), 2);
        assert_eq!(*id, 3);
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::QualifiedReference { id, elements, .. } = &implementation.statements[1] {
        assert_eq!(elements[0].get_id(), 4);
        assert_eq!(elements[1].get_id(), 5);
        assert_eq!(*id, 6);
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::Reference { id, .. } = &implementation.statements[2] {
        assert_eq!(*id, 7);
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::ArrayAccess {
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

    if let AstStatement::UnaryExpression { id, value, .. } = &implementation.statements[4] {
        assert_eq!(value.get_id(), 11);
        assert_eq!(*id, 12);
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::ExpressionList {
        id, expressions, ..
    } = &implementation.statements[5]
    {
        assert_eq!(expressions[0].get_id(), 13);
        assert_eq!(expressions[1].get_id(), 14);
        assert_eq!(*id, 15);
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::RangeStatement { id, start, end, .. } = &implementation.statements[6] {
        assert_eq!(start.get_id(), 16);
        assert_eq!(end.get_id(), 17);
        assert_eq!(*id, 18);
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::MultipliedStatement { id, element, .. } = &implementation.statements[7] {
        assert_eq!(element.get_id(), 19);
        assert_eq!(*id, 20);
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

    match &implementation.statements[0] {
        AstStatement::IfStatement {
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

    match &implementation.statements[0] {
        AstStatement::ForLoopStatement {
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
    let src = "
    PROGRAM PRG
       WHILE TRUE DO
            ;;
        END_WHILE
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];

    match &implementation.statements[0] {
        AstStatement::WhileLoopStatement {
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
    let src = "
    PROGRAM PRG
       REPEAT
            ;;
       UNTIL TRUE END_REPEAT
    END_PROGRAM
    ";
    let parse_result = parse(src).0;
    let implementation = &parse_result.implementations[0];

    match &implementation.statements[0] {
        AstStatement::RepeatLoopStatement {
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

    match &implementation.statements[0] {
        AstStatement::CaseStatement {
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
            if let AstStatement::ExpressionList {
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
        AstStatement::ArrayAccess {
            access: Box::new(empty_stmt()),
            reference: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::Assignment {
            left: Box::new(empty_stmt()),
            right: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::BinaryExpression {
            left: Box::new(empty_stmt()),
            right: Box::new(empty_stmt()),
            operator: Operator::And,
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::BinaryExpression {
            left: Box::new(empty_stmt()),
            right: Box::new(empty_stmt()),
            operator: Operator::And,
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::CallStatement {
            operator: Box::new(empty_stmt()),
            parameters: Box::new(None),
            id: 7,
            location: (1..5).into()
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::CaseCondition {
            condition: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::CaseStatement {
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
        AstStatement::EmptyStatement {
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::ExpressionList {
            expressions: vec![],
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::ForLoopStatement {
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
        AstStatement::IfStatement {
            blocks: vec![],
            else_block: vec![],
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::LiteralArray {
            elements: None,
            location: (1..5).into(),
            id: 7,
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::LiteralBool {
            value: true,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::LiteralInteger {
            value: 7,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::LiteralDate {
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
        AstStatement::LiteralDateAndTime {
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
        AstStatement::LiteralReal {
            value: "2.3".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::LiteralString {
            is_wide: false,
            value: "2.3".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::LiteralTime {
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
        AstStatement::LiteralTimeOfDay {
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
        AstStatement::MultipliedStatement {
            element: Box::new(empty_stmt()),
            multiplier: 9,
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::OutputAssignment {
            left: Box::new(empty_stmt()),
            right: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::QualifiedReference {
            elements: vec![],
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::RangeStatement {
            start: Box::new(empty_stmt()),
            end: Box::new(empty_stmt()),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::Reference {
            name: "ab".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::RepeatLoopStatement {
            body: vec![],
            condition: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::UnaryExpression {
            operator: Operator::Minus,
            value: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
    assert_eq!(
        AstStatement::WhileLoopStatement {
            body: vec![],
            condition: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_id(),
        7
    );
}

fn at(location: Range<usize>) -> AstStatement {
    AstStatement::EmptyStatement {
        id: 7,
        location: location.into(),
    }
}

#[test]
fn location_implementation_for_all_statements() {
    assert_eq!(
        AstStatement::ArrayAccess {
            reference: Box::new(at(0..1)),
            access: Box::new(at(2..4)),
            id: 7
        }
        .get_location(),
        (0..4).into()
    );
    assert_eq!(
        AstStatement::Assignment {
            left: Box::new(at(0..2)),
            right: Box::new(at(3..8)),
            id: 7
        }
        .get_location(),
        (0..8).into()
    );
    assert_eq!(
        AstStatement::BinaryExpression {
            left: Box::new(at(0..2)),
            right: Box::new(at(3..8)),
            operator: Operator::And,
            id: 7
        }
        .get_location(),
        (0..8).into()
    );
    assert_eq!(
        AstStatement::CallStatement {
            operator: Box::new(empty_stmt()),
            parameters: Box::new(None),
            id: 7,
            location: (1..5).into()
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::CaseCondition {
            condition: Box::new(at(2..4)),
            id: 7
        }
        .get_location(),
        (2..4).into()
    );
    assert_eq!(
        AstStatement::CaseStatement {
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
        AstStatement::EmptyStatement {
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::ExpressionList {
            expressions: vec![at(0..3), at(4..8)],
            id: 7
        }
        .get_location(),
        (0..8).into()
    );
    assert_eq!(
        AstStatement::ForLoopStatement {
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
        AstStatement::IfStatement {
            blocks: vec![],
            else_block: vec![],
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::LiteralArray {
            elements: None,
            location: (1..5).into(),
            id: 7,
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::LiteralBool {
            value: true,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::LiteralInteger {
            value: 7,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::LiteralDate {
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
        AstStatement::LiteralDateAndTime {
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
        AstStatement::LiteralReal {
            value: "2.3".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::LiteralString {
            is_wide: false,
            value: "2.3".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::LiteralTime {
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
        AstStatement::LiteralTimeOfDay {
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
        AstStatement::MultipliedStatement {
            element: Box::new(empty_stmt()),
            multiplier: 9,
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::OutputAssignment {
            left: Box::new(at(0..3)),
            right: Box::new(at(4..9)),
            id: 7
        }
        .get_location(),
        (0..9).into()
    );
    assert_eq!(
        AstStatement::QualifiedReference {
            elements: vec![at(0..3), at(4..5)],
            id: 7
        }
        .get_location(),
        (0..5).into()
    );
    assert_eq!(
        AstStatement::RangeStatement {
            start: Box::new(at(0..3)),
            end: Box::new(at(6..9)),
            id: 7
        }
        .get_location(),
        (0..9).into()
    );
    assert_eq!(
        AstStatement::Reference {
            name: "ab".to_string(),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::RepeatLoopStatement {
            body: vec![],
            condition: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::UnaryExpression {
            operator: Operator::Minus,
            value: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::WhileLoopStatement {
            body: vec![],
            condition: Box::new(empty_stmt()),
            location: (1..5).into(),
            id: 7
        }
        .get_location(),
        (1..5).into()
    );
}
