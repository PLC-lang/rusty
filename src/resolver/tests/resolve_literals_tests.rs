use plc_ast::literals::AstLiteral;
use plc_ast::{
    ast::{AstStatement, ReferenceAccess, ReferenceExpr, TypeNature},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::resolver::TypeAnnotator;
use crate::{
    assert_type_and_hint,
    index::ArgumentType,
    resolver::{AnnotationMap, StatementAnnotation},
    test_utils::tests::{annotate_with_ids, index_with_ids},
    typesystem::{DataType, DataTypeInformation, StringEncoding, TypeSize, DINT_TYPE},
};

#[test]
fn bool_literals_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "PROGRAM PRG
                TRUE;
                FALSE;
            END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    assert_eq!("BOOL", annotations.get_type_or_void(&statements[0], &index).get_name());
    assert_eq!("BOOL", annotations.get_type_or_void(&statements[1], &index).get_name());
}

#[test]
fn string_literals_are_annotated() {
    //GIVEN some string literals
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"PROGRAM PRG
                'abc';
                "xyzxyz";
            END_PROGRAM"#,
        id_provider.clone(),
    );

    //WHEN they are annotated
    let (mut annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    index.import(std::mem::take(&mut annotations.new_index));

    // THEN we expect them to be annotated with correctly sized string types
    let statements = &unit.implementations[0].statements;
    assert_type_and_hint!(&annotations, &index, &statements[0], "__STRING_3", None);
    assert_type_and_hint!(&annotations, &index, &statements[1], "__WSTRING_6", None);
    // AND we expect some newly created String-types
    assert_eq!(
        index.get_type_or_panic("__STRING_3"),
        &DataType {
            initial_value: None,
            name: "__STRING_3".into(),
            nature: TypeNature::String,
            information: DataTypeInformation::String {
                encoding: crate::typesystem::StringEncoding::Utf8,
                size: crate::typesystem::TypeSize::LiteralInteger(4)
            },
            location: SourceLocation::internal()
        }
    );
    assert_eq!(
        index.get_type_or_panic("__WSTRING_6"),
        &DataType {
            initial_value: None,
            name: "__WSTRING_6".into(),
            nature: TypeNature::String,
            information: DataTypeInformation::String {
                encoding: crate::typesystem::StringEncoding::Utf16,
                size: crate::typesystem::TypeSize::LiteralInteger(7)
            },
            location: SourceLocation::internal()
        }
    );
}

#[test]
fn int_literals_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "PROGRAM PRG
                0;
                127;
                128;
                32767;
                32768;
                2147483647;
                2147483648;
            END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["DINT", "DINT", "DINT", "DINT", "DINT", "DINT", "LINT"];

    let types: Vec<&str> =
        statements.iter().map(|s| annotations.get_type_or_void(s, &index).get_name()).collect();

    assert_eq!(expected_types, types);
}

#[test]
fn date_literals_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
            END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    let expected_types = [
        "TIME",
        "TIME",
        "TIME_OF_DAY",
        "TIME_OF_DAY",
        "TIME_OF_DAY",
        "DATE_AND_TIME",
        "DATE_AND_TIME",
        "DATE_AND_TIME",
        "DATE",
        "DATE",
    ];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(expected_types[i], annotations.get_type_or_void(s, &index).get_name(), "{:#?}", s);
    }
}

#[test]
fn long_date_literals_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "PROGRAM PRG
                LTIME#12.4d;
                LDATE#1984-10-01;
                LDT#1984-10-01-16:40:22;
                LTOD#00:00:12;
            END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    let expected_types = ["TIME", "DATE", "DATE_AND_TIME", "TIME_OF_DAY"];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(expected_types[i], annotations.get_type_or_void(s, &index).get_name(), "{:#?}", s);
    }
}

#[test]
fn real_literals_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "PROGRAM PRG
                3.1415;
                1.0;
            END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    let expected_types = ["REAL", "REAL"];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(
            expected_types[i].to_string(),
            annotations.get_type_or_void(s, &index).get_name(),
            "{:#?}",
            s
        );
    }
}

#[test]
fn casted_literals_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["SINT", "INT", "DINT", "LINT", "REAL", "LREAL", "BOOL", "BOOL"];
    let actual_types: Vec<&str> =
        statements.iter().map(|it| annotations.get_type_or_void(it, &index).get_name()).collect();

    assert_eq!(format!("{expected_types:#?}"), format!("{actual_types:#?}"),)
}

#[test]
fn enum_literals_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
            TYPE Color: (Green, Yellow, Red); END_TYPE
            TYPE Animal: (Dog, Cat, Horse); END_TYPE

            VAR_GLOBAL
                Cat : BOOL;
            END_VAR

            PROGRAM PRG
                VAR Yellow: BYTE; END_VAR

                Green;  //Color
                Dog;    //Animal

                Yellow;     //BYTE (local variable)
                Color#Yellow;  //Color

                Cat;   //BOOL (global variable)
                Animal#Cat;  //Animal

                // make sure these dont accidentally resolve to wrong enum
                Animal#Green;   //INVALID (invalid cast, validation must handle this)
                Color#Dog;      //INVALID (invalid cast, validation must handle this)
                invalid#Dog;    //invalid (VOID)
                Animal.Dog;     //Dog (invalid access, validation must handle this)
                PRG.Cat;        //invalid (VOID)

            END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    let actual_resolves: Vec<&str> =
        statements.iter().map(|it| annotations.get_type_or_void(it, &index).get_name()).collect();
    assert_eq!(
        vec!["Color", "Animal", "BYTE", "Color", "BOOL", "Animal", "VOID", "VOID", "VOID", "Animal", "VOID"],
        actual_resolves
    )
}

#[test]
fn enum_literals_target_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
            TYPE Color: (Green, Yellow, Red) := 0; END_TYPE

            PROGRAM PRG
                VAR Red: BYTE; END_VAR
                Color#Red;  //we should resolve to the enum, not the local!
            END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let color_red = &unit.implementations[0].statements[0];

    let DataTypeInformation::Enum { name, variants, referenced_type } =
        annotations.get_type_or_void(color_red, &index).get_type_information()
    else {
        unreachable!()
    };

    assert_eq!(name, "Color");
    assert_eq!(
        variants.iter().map(|variant| variant.get_name()).collect::<Vec<_>>(),
        vec!["Green", "Yellow", "Red"]
    );
    assert_eq!(referenced_type, DINT_TYPE);

    if let AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Cast(target), .. }) =
        color_red.get_stmt()
    {
        let DataTypeInformation::Enum { name, variants, referenced_type } =
            annotations.get_type_or_void(target, &index).get_type_information()
        else {
            unreachable!();
        };

        // right type gets annotated
        assert_eq!(name, "Color");
        assert_eq!(
            variants.iter().map(|variant| variant.get_name()).collect::<Vec<_>>(),
            vec!["Green", "Yellow", "Red"]
        );
        assert_eq!(referenced_type, DINT_TYPE);

        // Red gets annotated to the declared variable, not only the type
        assert_eq!(
            Some(&StatementAnnotation::Variable {
                resulting_type: "Color".into(),
                qualified_name: "Color.Red".into(),
                constant: true,
                argument_type: ArgumentType::ByVal(crate::index::VariableType::Global),
                auto_deref: None,
            }),
            annotations.get(target)
        );
    } else {
        panic!("no cast statement")
    }
}

#[test]
fn casted_inner_literals_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;
    let expected_types = vec!["SINT", "INT", "DINT", "LINT", "REAL", "LREAL", "BOOL", "BOOL"];
    let actual_types: Vec<&str> = statements
        .iter()
        .map(|it| {
            if let AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Cast(target), ..
            }) = it.get_stmt()
            {
                target.as_ref()
            } else {
                panic!("no cast")
            }
        })
        .map(|it| annotations.get_type_or_void(it, &index).get_name())
        .collect();

    assert_eq!(format!("{expected_types:#?}"), format!("{actual_types:#?}"),)
}

#[test]
fn casted_literals_enums_are_annotated_correctly() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
            TYPE Color: (red, green, blue); END_TYPE
            PROGRAM PRG
                Color#red;
                Color#green;
                Color#blue;
            END_PROGRAM",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["Color", "Color", "Color"];
    let actual_types: Vec<&str> = statements
        .iter()
        .map(|it| {
            if let AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Cast(target), ..
            }) = it.get_stmt()
            {
                target.as_ref()
            } else {
                unreachable!();
            }
        })
        .map(|it| annotations.get_type_or_void(it, &index).get_name())
        .collect();

    assert_eq!(format!("{expected_types:#?}"), format!("{actual_types:#?}"),)
}

#[test]
fn expression_list_members_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM PRG
                (1,TRUE,3.1415);
            END_PROGRAM",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statement = &unit.implementations[0].statements[0];

    let expected_types = vec!["DINT", "BOOL", "REAL"];

    let AstStatement::ParenExpression(expr) = statement.get_stmt() else { panic!() };
    let AstStatement::ExpressionList(expressions, ..) = expr.get_stmt() else { panic!() };

    let actual_types: Vec<&str> =
        expressions.iter().map(|it| annotations.get_type_or_void(it, &index).get_name()).collect();

    assert_eq!(format!("{expected_types:#?}"), format!("{actual_types:#?}"),)
}

#[test]
fn expression_lists_with_expressions_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
            VAR_GLOBAL CONSTANT
                a : INT : = 2;
                b : BOOL : = FALSE;
                c : LREAL : = 3.14;
            END_VAR

            PROGRAM PRG
                (a + a, b OR b , 2 * c, a + c);
            END_PROGRAM",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statement = &unit.implementations[0].statements[0];
    let expected_types = vec!["DINT", "BOOL", "LREAL", "LREAL"];

    let AstStatement::ParenExpression(expr) = statement.get_stmt() else { panic!() };
    let AstStatement::ExpressionList(expressions, ..) = expr.get_stmt() else { panic!() };

    let actual_types =
        expressions.iter().map(|it| annotations.get_type_or_void(it, &index).get_name()).collect::<Vec<_>>();

    assert_eq!(format!("{expected_types:#?}"), format!("{actual_types:#?}"),)
}

#[test]
fn array_initialization_is_annotated_correctly() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
            VAR_GLOBAL CONSTANT
                a : ARRAY[0..2] OF BYTE := [1,2,3];
            END_VAR
            ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    let a_init = unit.global_vars[0].variables[0].initializer.as_ref().unwrap();
    let t = annotations.get_type_hint(a_init, &index).unwrap();
    assert_eq!(index.find_global_variable("a").unwrap().get_type_name(), t.get_name())
}

#[test]
fn expression_list_as_array_initilization_is_annotated_correctly() {
    // GIVEN two global variables beeing initialized with expression lists
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
            VAR_GLOBAL
                a : ARRAY[0..2] OF INT := 1+1,2;
                b : ARRAY[0..2] OF STRING[3] := 'ABC','D';
            END_VAR
        ",
        id_provider.clone(),
    );

    // WHEN annotation is done
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // THEN for the first statement
    let a_init = unit.global_vars[0].variables[0].initializer.as_ref().unwrap();
    // all expressions should be annotated with the right type [INT]
    if let AstStatement::ExpressionList(expressions, ..) = a_init.get_stmt() {
        for exp in expressions {
            if let Some(data_type) = annotations.get_type_hint(exp, &index) {
                let type_info = data_type.get_type_information();
                assert!(matches!(type_info, DataTypeInformation::Integer { .. }))
            } else {
                unreachable!();
            }
        }
    } else {
        unreachable!();
    }

    // AND for the second statement
    let b_init = unit.global_vars[0].variables[1].initializer.as_ref().unwrap();
    // all expressions should be annotated with the right type [STRING]
    if let AstStatement::ExpressionList(expressions, ..) = b_init.get_stmt() {
        for exp in expressions {
            let data_type = annotations.get_type_hint(exp, &index).unwrap();
            let type_info = data_type.get_type_information();
            assert_eq!(
                type_info,
                &DataTypeInformation::String {
                    encoding: StringEncoding::Utf8,
                    size: TypeSize::from_literal(4),
                }
            )
        }
    } else {
        unreachable!();
    }
}

#[test]
fn struct_field_members_assignments_are_annotated_correctly_in_array_of_structs() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        TYPE STRUCT1 : STRUCT
            x    : DINT;
            arr   : ARRAY[0..1] OF STRUCT2;
        END_STRUCT END_TYPE

        TYPE STRUCT2 : STRUCT
            y  : INT;
            z  : INT;
        END_STRUCT END_TYPE

        PROGRAM main
            VAR
                var_init1 : ARRAY[0..1] OF STRUCT1 := [
                    (x := 0, arr := [(y := 0), (z := 0)])
                ];
            END_VAR
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let var = unit.pous[0].variable_blocks[0].variables[0].initializer.clone().unwrap();

    // (x := 0, arr := [(y := 0), (z := 0)])
    let AstStatement::Literal(AstLiteral::Array(arr)) = &var.stmt else { panic!() };
    let AstStatement::ParenExpression(expr) = &arr.elements().unwrap().stmt else { panic!() };
    let AstStatement::ExpressionList(elements) = &expr.stmt else { panic!() };

    // x := 0
    let x = &elements[0];
    assert_eq!(&annotations.get_type_hint(x, &index).unwrap().name, "STRUCT1");

    // arr := [(y := 0), (z := 0)]
    let AstStatement::Assignment(assignment) = &elements[1].stmt else { panic!() };

    // [(y := 0), (z := 0)]
    let AstStatement::Literal(AstLiteral::Array(arr)) = &assignment.right.stmt else { panic!() };
    let AstStatement::ExpressionList(elements) = &arr.elements.as_ref().unwrap().stmt else { panic!() };

    // y := 0
    let AstStatement::ParenExpression(y) = &elements[0].stmt else { panic!() };
    assert_eq!(&annotations.get_type_hint(y, &index).unwrap().name, "STRUCT2");

    // z := 0
    let AstStatement::ParenExpression(z) = &elements[1].stmt else { panic!() };
    assert_eq!(&annotations.get_type_hint(z, &index).unwrap().name, "STRUCT2");
}

#[test]
fn struct_field_members_assignments_are_annotated_correctly_in_array_of_structs_assignment_in_body() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        TYPE STRUCT1 : STRUCT
            x    : DINT;
            arr   : ARRAY[0..1] OF STRUCT2;
        END_STRUCT END_TYPE

        TYPE STRUCT2 : STRUCT
            y  : INT;
            z  : INT;
        END_STRUCT END_TYPE

        PROGRAM main
            VAR_TEMP
                var_init1 : ARRAY[0..1] OF STRUCT1;
            END_VAR
            var_init1 := [(x := 0, arr := [(y := 0), (z := 0)])];
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // Get the assignment statement: var_init1 := [(x := 0, arr := [(y := 0), (z := 0)])];
    let assignment_stmt = &unit.implementations[0].statements[0];
    let AstStatement::Assignment(assignment) = &assignment_stmt.stmt else { panic!() };

    // [(x := 0, arr := [(y := 0), (z := 0)])]
    let AstStatement::Literal(AstLiteral::Array(arr)) = &assignment.right.stmt else { panic!() };
    let AstStatement::ParenExpression(paren) = &arr.elements().unwrap().stmt else { panic!() };
    let AstStatement::ExpressionList(elements) = &paren.stmt else { panic!() };

    // x := 0
    let x = &elements[0];
    assert_eq!(&annotations.get_type_hint(x, &index).unwrap().name, "STRUCT1");

    // arr := [(y := 0), (z := 0)]
    let AstStatement::Assignment(arr_assignment) = &elements[1].stmt else { panic!() };

    // [(y := 0), (z := 0)]
    let AstStatement::Literal(AstLiteral::Array(inner_arr)) = &arr_assignment.right.stmt else { panic!() };
    let AstStatement::ExpressionList(inner_elements) = &inner_arr.elements.as_ref().unwrap().stmt else {
        panic!()
    };

    // y := 0
    let AstStatement::ParenExpression(y) = &inner_elements[0].stmt else { panic!() };
    assert_eq!(&annotations.get_type_hint(y, &index).unwrap().name, "STRUCT2");

    // z := 0
    let AstStatement::ParenExpression(z) = &inner_elements[1].stmt else { panic!() };
    assert_eq!(&annotations.get_type_hint(z, &index).unwrap().name, "STRUCT2");
}

#[test]
fn struct_with_nested_array_in_array_of_structs_assignment_in_body() {
    // This test specifically checks that arrays inside struct initializers within
    // array literals in assignment statements are properly annotated
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        TYPE myStruct : STRUCT
            a, b : DINT;
            c : ARRAY[0..1] OF DINT;
        END_STRUCT END_TYPE

        PROGRAM main
            VAR_TEMP
                arr : ARRAY[0..1] OF myStruct;
            END_VAR
            arr := [(a := 10, b := 20, c := [30, 40])];
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // Get the assignment statement: arr := [(a := 10, b := 20, c := [30, 40])];
    let assignment_stmt = &unit.implementations[0].statements[0];
    let AstStatement::Assignment(assignment) = &assignment_stmt.stmt else { panic!() };

    // [(a := 10, b := 20, c := [30, 40])]
    let AstStatement::Literal(AstLiteral::Array(arr)) = &assignment.right.stmt else { panic!() };
    let AstStatement::ParenExpression(paren) = &arr.elements().unwrap().stmt else { panic!() };
    let AstStatement::ExpressionList(elements) = &paren.stmt else { panic!() };

    // a := 10
    let a = &elements[0];
    assert_eq!(&annotations.get_type_hint(a, &index).unwrap().name, "myStruct");

    // b := 20
    let b = &elements[1];
    assert_eq!(&annotations.get_type_hint(b, &index).unwrap().name, "myStruct");

    // c := [30, 40]
    let c = &elements[2];
    assert_eq!(&annotations.get_type_hint(c, &index).unwrap().name, "myStruct");

    // Check that the inner array [30, 40] has the correct type hint
    let AstStatement::Assignment(c_assignment) = &c.stmt else { panic!() };
    let inner_array = &c_assignment.right;
    assert_eq!(&annotations.get_type_hint(inner_array, &index).unwrap().name, "__myStruct_c");
}
