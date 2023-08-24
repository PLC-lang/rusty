// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{parser::tests::ref_to, Diagnostic};
use core::panic;
use std::{collections::HashSet, ops::Range};

use crate::{parser::tests::empty_stmt, test_utils::tests::parse};
use plc_ast::{
    ast::{
        AccessModifier, ArgumentProperty, AstFactory, AstStatement, DataTypeDeclaration, Implementation,
        LinkageType, Operator, Pou, PouType, ReferenceAccess, SourceRange, Variable, VariableBlock,
        VariableBlockType,
    },
    control_statements::{AstControlStatement, CaseStatement, ForLoopStatement, IfStatement, LoopStatement},
    literals::AstLiteral,
};
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
            variable_block_type: VariableBlockType::Input(ArgumentProperty::ByVal),
            access: AccessModifier::Internal,
            constant: false,
            retain: false,
            location: SourceRange::undefined(),
            linkage: LinkageType::Internal,
            variables: vec![Variable {
                name: "E3".into(),
                data_type_declaration: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "E4".into(),
                    location: SourceRange::undefined(),
                },
                initializer: None,
                address: None,
                location: SourceRange::undefined(),
            }],
        }],
        location: SourceRange::undefined(),
        name_location: SourceRange::undefined(),
        generics: vec![],
        linkage: LinkageType::Internal,
    };
    assert_eq!(format!("{expected:#?}"), format!("{pou:#?}").as_str());
    let implementation = &parse_result.implementations[0];
    let expected = Implementation {
        name: "E1".into(),
        type_name: "E1".into(),
        linkage: LinkageType::Internal,
        pou_type: PouType::Function,
        statements: vec![AstStatement::Assignment {
            left: Box::new(ref_to("E5")),
            right: Box::new(AstStatement::Literal {
                kind: AstLiteral::new_real("1.0E6".into()),
                id: 0,
                location: SourceRange::undefined(),
            }),
            id: 0,
        }],
        access: None,
        overriding: false,
        generic: false,
        location: (105..142).into(),
        name_location: (22..24).into(),
    };
    assert_eq!(format!("{expected:#?}"), format!("{implementation:#?}").as_str());
    assert_eq!(format!("{diagnostics:#?}"), format!("{:#?}", Vec::<Diagnostic>::new()).as_str());
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
    let mut ids = HashSet::new();
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
    let mut ids = HashSet::new();

    if let AstStatement::Assignment { id, left, right } = &implementation.statements[0] {
        assert!(ids.insert(left.get_id()));
        assert!(ids.insert(right.get_id()));
        assert!(ids.insert(*id));
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
    let mut ids = HashSet::new();
    if let AstStatement::CallStatement { id, operator, .. } = &implementation.statements[0] {
        assert!(ids.insert(operator.get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::CallStatement { id, operator, parameters, .. } = &implementation.statements[1] {
        assert!(ids.insert(operator.get_id()));
        if let Some(AstStatement::ExpressionList { expressions, id }) = &**parameters {
            assert!(ids.insert(expressions[0].get_id()));
            assert!(ids.insert(expressions[1].get_id()));
            assert!(ids.insert(expressions[2].get_id()));
            assert!(ids.insert(*id));
        }
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::CallStatement { id, operator, parameters, .. } = &implementation.statements[2] {
        assert!(ids.insert(operator.get_id()));
        if let Some(AstStatement::ExpressionList { expressions, id }) = &**parameters {
            if let AstStatement::Assignment { left, right, id, .. } = &expressions[0] {
                assert!(ids.insert(left.get_id()));
                assert!(ids.insert(right.get_id()));
                assert!(ids.insert(*id));
            } else {
                panic!("unexpected statement");
            }
            if let AstStatement::OutputAssignment { left, right, id, .. } = &expressions[1] {
                assert!(ids.insert(left.get_id()));
                assert!(ids.insert(right.get_id()));
                assert!(ids.insert(*id));
            } else {
                panic!("unexpected statement");
            }
            assert!(ids.insert(expressions[2].get_id()));
            assert!(ids.insert(*id));
        }
        assert!(ids.insert(*id));
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
    let mut ids = HashSet::new();

    if let AstStatement::BinaryExpression { id, left, right, .. } = &implementation.statements[0] {
        assert!(ids.insert(left.get_id()));
        assert!(ids.insert(right.get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::ReferenceExpr { access: ReferenceAccess::Member(m), base: Some(base), id, .. } =
        &implementation.statements[1]
    {
        assert!(ids.insert(*id));
        assert!(ids.insert(m.get_id()));
        if let AstStatement::ReferenceExpr { access: ReferenceAccess::Member(m), base: None, .. } =
            base.as_ref()
        {
            assert!(ids.insert(m.get_id()));
        } else {
            panic!("unexpected statement");
        }
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::ReferenceExpr { access: ReferenceAccess::Member(m), base: None, id, .. } =
        &implementation.statements[2]
    {
        assert!(ids.insert(*id));
        assert!(ids.insert(m.get_id()));
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::ReferenceExpr {
        access: ReferenceAccess::Index(access),
        base: Some(reference),
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

    if let AstStatement::UnaryExpression { id, value, .. } = &implementation.statements[4] {
        assert!(ids.insert(value.get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::ExpressionList { id, expressions, .. } = &implementation.statements[5] {
        assert!(ids.insert(expressions[0].get_id()));
        assert!(ids.insert(expressions[1].get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::RangeStatement { id, start, end, .. } = &implementation.statements[6] {
        assert!(ids.insert(start.get_id()));
        assert!(ids.insert(end.get_id()));
        assert!(ids.insert(*id));
    } else {
        panic!("unexpected statement");
    }

    if let AstStatement::MultipliedStatement { id, element, .. } = &implementation.statements[7] {
        assert!(ids.insert(element.get_id()));
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
    let mut ids = HashSet::new();
    match &implementation.statements[0] {
        AstStatement::ControlStatement {
            kind: AstControlStatement::If(IfStatement { blocks, else_block, .. }),
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
    let mut ids = HashSet::new();
    match &implementation.statements[0] {
        AstStatement::ControlStatement {
            id,
            kind: AstControlStatement::ForLoop(ForLoopStatement { counter, start, end, by_step, body, .. }),
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
    let mut ids = HashSet::new();
    match &implementation.statements[0] {
        AstStatement::ControlStatement {
            kind: AstControlStatement::WhileLoop(LoopStatement { condition, body, .. }),
            ..
        } => {
            assert!(ids.insert(condition.get_id()));
            assert!(ids.insert(body[0].get_id()));
            assert!(ids.insert(body[1].get_id()));
            assert!(ids.insert(implementation.statements[0].get_id()));
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
    let mut ids = HashSet::new();

    match &implementation.statements[0] {
        AstStatement::ControlStatement {
            kind: AstControlStatement::RepeatLoop(LoopStatement { condition, body, .. }),
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
    let mut ids = HashSet::new();
    match &implementation.statements[0] {
        AstStatement::ControlStatement {
            kind: AstControlStatement::Case(CaseStatement { case_blocks, else_block, selector, .. }),
            ..
        } => {
            //1st case block
            assert!(ids.insert(selector.get_id()));
            assert!(ids.insert(case_blocks[0].condition.get_id()));
            assert!(ids.insert(case_blocks[0].body[0].get_id()));

            //2nd case block
            if let AstStatement::ExpressionList { expressions, id, .. } = case_blocks[1].condition.as_ref() {
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

#[test]
fn id_implementation_for_all_statements() {
    assert_eq!(
        AstStatement::Assignment { left: Box::new(empty_stmt()), right: Box::new(empty_stmt()), id: 7 }
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
    assert_eq!(AstStatement::CaseCondition { condition: Box::new(empty_stmt()), id: 7 }.get_id(), 7);
    assert_eq!(AstFactory::create_case_statement(empty_stmt(), vec![], vec![], (1..5).into(), 7).get_id(), 7);
    assert_eq!(AstStatement::EmptyStatement { location: (1..5).into(), id: 7 }.get_id(), 7);
    assert_eq!(AstStatement::ExpressionList { expressions: vec![], id: 7 }.get_id(), 7);
    assert_eq!(
        AstFactory::create_for_loop(empty_stmt(), empty_stmt(), empty_stmt(), None, vec![], (1..5).into(), 7)
            .get_id(),
        7
    );
    assert_eq!(
        AstFactory::create_if_statement(Vec::new(), Vec::new(), SourceRange::undefined(), 7).get_id(),
        7
    );
    assert_eq!(AstStatement::Literal { kind: AstLiteral::Null, location: (1..5).into(), id: 7 }.get_id(), 7);
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
        AstStatement::OutputAssignment { left: Box::new(empty_stmt()), right: Box::new(empty_stmt()), id: 7 }
            .get_id(),
        7
    );
    assert_eq!(
        AstStatement::RangeStatement { start: Box::new(empty_stmt()), end: Box::new(empty_stmt()), id: 7 }
            .get_id(),
        7
    );
    assert_eq!(
        AstStatement::Reference { name: "ab".to_string(), location: (1..5).into(), id: 7 }.get_id(),
        7
    );
    assert_eq!(AstFactory::create_repeat_statement(empty_stmt(), vec![], (1..5).into(), 7).get_id(), 7);
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
    assert_eq!(AstFactory::create_while_statement(empty_stmt(), vec![], (1..5).into(), 7).get_id(), 7);
}

fn at(location: Range<usize>) -> AstStatement {
    AstStatement::EmptyStatement { id: 7, location: location.into() }
}

#[test]
fn location_implementation_for_all_statements() {
    assert_eq!(
        AstStatement::Assignment { left: Box::new(at(0..2)), right: Box::new(at(3..8)), id: 7 }
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
        AstStatement::CaseCondition { condition: Box::new(at(2..4)), id: 7 }.get_location(),
        (2..4).into()
    );
    assert_eq!(
        AstFactory::create_case_statement(empty_stmt(), vec![], vec![], (1..5).into(), 7).get_location(),
        (1..5).into()
    );
    assert_eq!(AstStatement::EmptyStatement { location: (1..5).into(), id: 7 }.get_location(), (1..5).into());
    assert_eq!(
        AstStatement::ExpressionList { expressions: vec![at(0..3), at(4..8)], id: 7 }.get_location(),
        (0..8).into()
    );
    assert_eq!(
        AstFactory::create_for_loop(empty_stmt(), empty_stmt(), empty_stmt(), None, vec![], (1..5).into(), 7)
            .get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstFactory::create_if_statement(Vec::new(), Vec::new(), (1..5).into(), 7).get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstStatement::Literal { kind: AstLiteral::Null, location: (1..5).into(), id: 7 }.get_location(),
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
        AstStatement::OutputAssignment { left: Box::new(at(0..3)), right: Box::new(at(4..9)), id: 7 }
            .get_location(),
        (0..9).into()
    );
    assert_eq!(
        AstStatement::RangeStatement { start: Box::new(at(0..3)), end: Box::new(at(6..9)), id: 7 }
            .get_location(),
        (0..9).into()
    );
    assert_eq!(
        AstStatement::Reference { name: "ab".to_string(), location: (1..5).into(), id: 7 }.get_location(),
        (1..5).into()
    );
    assert_eq!(
        AstFactory::create_repeat_statement(empty_stmt(), vec![], (1..5).into(), 7).get_location(),
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
        AstFactory::create_while_statement(empty_stmt(), vec![], (1..5).into(), 7).get_location(),
        (1..5).into()
    );
}
