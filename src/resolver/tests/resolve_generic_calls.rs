use crate::{
    assert_type_and_hint,
    ast::{self, flatten_expression_list, AstStatement},
    resolver::{AnnotationMap, StatementAnnotation, TypeAnnotator},
    test_utils::tests::{annotate, index},
    typesystem::{
        DataTypeInformation, BYTE_TYPE, DINT_TYPE, INT_TYPE, LREAL_TYPE, LWORD_TYPE, REAL_TYPE,
        SINT_TYPE, STRING_TYPE,
    },
};

#[test]
fn resolved_generic_call_added_to_index() {
    let (unit, index) = index(
        "
        FUNCTION myFunc<G: ANY_NUM> : G
        VAR_INPUT   x : G;  END_VAR
        END_FUNCTION

        FUNCTION myFunc__BYTE : BYTE
        VAR_INPUT   x : BYTE; END_VAR
        END_FUNCTION

        PROGRAM PRG
            VAR
                a : INT;
            END_VAR
            myFunc(x := a);
            myFunc(6);
            myFunc(1.0);
            myFunc(BYTE#1);
        END_PROGRAM",
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    // The implementations are added to the index
    let implementations = annotations.new_index.get_implementations();
    assert!(implementations.contains_key("myfunc__int"));
    assert!(implementations.contains_key("myfunc__dint"));
    assert!(implementations.contains_key("myfunc__real"));
    assert_eq!(3, implementations.len()); //make sure BYTE-implementation was not added by the annotator

    //The pous are added to the index
    let pous = annotations.new_index.get_pous();
    assert!(pous.contains_key("myfunc__int"));
    assert!(pous.contains_key("myfunc__dint"));
    assert!(pous.contains_key("myfunc__real"));
    assert_eq!(3, pous.len()); //make sure BYTE-implementation was not added by the annotator

    //Each POU has members
    assert_eq!(
        "INT",
        annotations
            .new_index
            .find_member("myfunc__int", "x")
            .unwrap()
            .get_type_name()
    );
    assert_eq!(
        "INT",
        annotations
            .new_index
            .find_member("myfunc__int", "myfunc__int")
            .unwrap()
            .get_type_name()
    );
    assert_eq!(
        "DINT",
        annotations
            .new_index
            .find_member("myfunc__dint", "x")
            .unwrap()
            .get_type_name()
    );
    assert_eq!(
        "DINT",
        annotations
            .new_index
            .find_member("myfunc__dint", "myfunc__dint")
            .unwrap()
            .get_type_name()
    );
    assert_eq!(
        "REAL",
        annotations
            .new_index
            .find_member("myfunc__real", "x")
            .unwrap()
            .get_type_name()
    );
    assert_eq!(
        "REAL",
        annotations
            .new_index
            .find_member("myfunc__real", "myfunc__real")
            .unwrap()
            .get_type_name()
    );
}

#[test]
fn generic_call_annotated_with_correct_type() {
    let (unit, index) = index(
        "
        FUNCTION myFunc<G: ANY_NUM> : G
        VAR_INPUT
            x : G;
        END_VAR
        END_FUNCTION

        PROGRAM PRG
            VAR
                a : INT;
            END_VAR
            myFunc(x := a);
            myFunc(6);
            myFunc(1.0);
        END_PROGRAM",
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    let call = &unit.implementations[1].statements[0];

    //The return type should have the correct type
    assert_type_and_hint!(&annotations, &index, call, INT_TYPE, None);

    if let AstStatement::CallStatement {
        operator,
        parameters,
        ..
    } = call
    {
        //The call name should nave the correct type
        assert_eq!(Some("myFunc__INT"), annotations.get_call_name(operator));
        //parameters should have the correct type
        if let Some(AstStatement::Assignment { left, right, .. }) = &**parameters {
            assert_type_and_hint!(&annotations, &index, left, INT_TYPE, None);
            assert_type_and_hint!(&annotations, &index, right, INT_TYPE, Some(INT_TYPE));
        } else {
            unreachable!("Not an assignment");
        }
    } else {
        unreachable!("Not a call statement");
    }

    let call = &unit.implementations[1].statements[1];

    //The return type should have the correct type
    assert_type_and_hint!(&annotations, &index, call, DINT_TYPE, None);

    if let AstStatement::CallStatement {
        operator,
        parameters,
        ..
    } = call
    {
        //The call name should nave the correct type
        assert_eq!(Some("myFunc__DINT"), annotations.get_call_name(operator));
        if let Some(parameter) = &**parameters {
            //parameters should have the correct type
            assert_type_and_hint!(&annotations, &index, parameter, DINT_TYPE, Some(DINT_TYPE));
        } else {
            unreachable!("No Parameters");
        }
    } else {
        unreachable!("Not a call statement");
    }

    let call = &unit.implementations[1].statements[2];

    //The return type should have the correct type
    assert_type_and_hint!(&annotations, &index, call, REAL_TYPE, None);

    if let AstStatement::CallStatement {
        operator,
        parameters,
        ..
    } = call
    {
        //The call name should nave the correct type
        assert_eq!(Some("myFunc__REAL"), annotations.get_call_name(operator));
        if let Some(parameter) = &**parameters {
            //parameters should have the correct type
            assert_type_and_hint!(&annotations, &index, parameter, REAL_TYPE, Some(REAL_TYPE));
        } else {
            unreachable!("No Parameters");
        }
    } else {
        unreachable!("Not a call statement");
    }
}

#[test]
fn generic_call_multi_params_annotated_with_correct_type() {
    let (unit, index) = index(
        "
        FUNCTION myFunc<G: ANY_NUM, F : ANY_INT> : G
        VAR_INPUT
            x,y : G;
            z : F;
        END_VAR
        END_FUNCTION

        PROGRAM PRG
            VAR
                a : INT;
                b : DINT;
                c : INT;
            END_VAR
            myFunc(x := a, y := b, z := c);
            myFunc(a,b,c);
            myFunc(1.0, 2, BYTE#2);
        END_PROGRAM",
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);

    let call = &unit.implementations[1].statements[0];

    //The return type should have the correct type
    assert_type_and_hint!(&annotations, &index, call, DINT_TYPE, None);

    if let AstStatement::CallStatement {
        operator,
        parameters,
        ..
    } = call
    {
        //The call name should nave the correct type
        assert_eq!(
            Some("myFunc__DINT__INT"),
            annotations.get_call_name(operator)
        );
        //parameters should have the correct type
        if let Some(parameters) = &**parameters {
            if let [x, y, z] = ast::flatten_expression_list(parameters)[..] {
                if let AstStatement::Assignment { left, right, .. } = x {
                    assert_type_and_hint!(&annotations, &index, left, DINT_TYPE, None);
                    assert_type_and_hint!(&annotations, &index, right, INT_TYPE, Some(DINT_TYPE));
                } else {
                    unreachable!("Not an assignment");
                }

                if let AstStatement::Assignment { left, right, .. } = y {
                    assert_type_and_hint!(&annotations, &index, left, DINT_TYPE, None);
                    assert_type_and_hint!(&annotations, &index, right, DINT_TYPE, Some(DINT_TYPE));
                } else {
                    unreachable!("Not an assignment");
                }

                if let AstStatement::Assignment { left, right, .. } = z {
                    assert_type_and_hint!(&annotations, &index, left, INT_TYPE, None);
                    assert_type_and_hint!(&annotations, &index, right, INT_TYPE, Some(INT_TYPE));
                } else {
                    unreachable!("Not an assignment");
                }
            } else {
                unreachable!("Wrong parameters {:?}", parameters)
            }
        } else {
            unreachable!("No parameters")
        }
    }
    let call = &unit.implementations[1].statements[1];

    //The return type should have the correct type
    assert_type_and_hint!(&annotations, &index, call, DINT_TYPE, None);

    if let AstStatement::CallStatement {
        operator,
        parameters,
        ..
    } = call
    {
        //The call name should nave the correct type
        assert_eq!(
            Some("myFunc__DINT__INT"),
            annotations.get_call_name(operator)
        );
        //parameters should have the correct type
        if let Some(parameters) = &**parameters {
            if let [x, y, z] = ast::flatten_expression_list(parameters)[..] {
                assert_type_and_hint!(&annotations, &index, x, INT_TYPE, Some(DINT_TYPE));
                assert_type_and_hint!(&annotations, &index, y, DINT_TYPE, Some(DINT_TYPE));
                assert_type_and_hint!(&annotations, &index, z, INT_TYPE, Some(INT_TYPE));
            } else {
                unreachable!("Wrong parameters {:?}", parameters)
            }
        } else {
            unreachable!("No parameters")
        }
    }

    let call = &unit.implementations[1].statements[2];

    //The return type should have the correct type
    assert_type_and_hint!(&annotations, &index, call, REAL_TYPE, None);

    if let AstStatement::CallStatement {
        operator,
        parameters,
        ..
    } = call
    {
        //The call name should nave the correct type
        assert_eq!(
            Some("myFunc__REAL__BYTE"),
            annotations.get_call_name(operator)
        );
        //parameters should have the correct type
        if let Some(parameters) = &**parameters {
            if let [x, y, z] = ast::flatten_expression_list(parameters)[..] {
                assert_type_and_hint!(&annotations, &index, x, REAL_TYPE, Some(REAL_TYPE));
                assert_type_and_hint!(&annotations, &index, y, DINT_TYPE, Some(REAL_TYPE));
                assert_type_and_hint!(&annotations, &index, z, BYTE_TYPE, Some(BYTE_TYPE));
            } else {
                unreachable!("Wrong parameters {:?}", parameters)
            }
        } else {
            unreachable!("No parameters")
        }
    }
}

#[test]
fn call_order_of_parameters_does_not_change_annotations() {
    let (unit, index) = index(
        "
        FUNCTION myFunc : INT
        VAR_INPUT
            x,y : DINT;
            z : INT;
        END_VAR
        END_FUNCTION

        PROGRAM PRG
            VAR
                a : INT;
                b : DINT;
                c : INT;
            END_VAR
            myFunc(x := a, y := b, z := c);
            myFunc(y := b, x := a, z := c);
            myFunc(z := c, y := b, x := a);
        END_PROGRAM",
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);

    fn get_parameter_with_name<'a>(
        parameters_list: &[&'a AstStatement],
        expected_name: &str,
    ) -> &'a AstStatement {
        parameters_list.iter().find(|it| matches!(it, AstStatement::Assignment { left, .. } 
                        if { matches!(&**left, AstStatement::Reference{name, ..} if {name == expected_name})})).unwrap()
    }

    // all three call-statements should give the exact same annotations
    // the order of the parameters should not matter
    for call in &unit.implementations[1].statements {
        if let AstStatement::CallStatement {
            operator,
            parameters,
            ..
        } = call
        {
            //The call name should nave the correct type
            assert_eq!(Some("myFunc"), annotations.get_call_name(operator));
            //parameters should have the correct type
            if let Some(parameters) = &**parameters {
                let parameters_list = ast::flatten_expression_list(parameters);
                let [x, y, z] = [
                    get_parameter_with_name(&parameters_list, "x"),
                    get_parameter_with_name(&parameters_list, "y"),
                    get_parameter_with_name(&parameters_list, "z"),
                ];
                if let [AstStatement::Assignment {
                    left: x, right: a, ..
                }, AstStatement::Assignment {
                    left: y, right: b, ..
                }, AstStatement::Assignment {
                    left: z, right: c, ..
                }] = [x, y, z]
                {
                    assert_type_and_hint!(&annotations, &index, x, DINT_TYPE, None);
                    assert_type_and_hint!(&annotations, &index, a, INT_TYPE, Some(DINT_TYPE));

                    assert_type_and_hint!(&annotations, &index, y, DINT_TYPE, None);
                    assert_type_and_hint!(&annotations, &index, b, DINT_TYPE, Some(DINT_TYPE));

                    assert_type_and_hint!(&annotations, &index, z, INT_TYPE, None);
                    assert_type_and_hint!(&annotations, &index, c, INT_TYPE, Some(INT_TYPE));
                } else {
                    unreachable!("Not an assignment");
                }
            } else {
                unreachable!("No parameters")
            }
        }
    }
}

#[test]
fn call_order_of_generic_parameters_does_not_change_annotations() {
    let (unit, index) = index(
        "
        FUNCTION myFunc<G: ANY_NUM, F : ANY_INT> : G
        VAR_INPUT
            x,y : G;
            z : F;
        END_VAR
        END_FUNCTION

        PROGRAM PRG
            VAR
                a : INT;
                b : DINT;
                c : INT;
            END_VAR
            myFunc(x := a, y := b, z := c);
            myFunc(y := b, x := a, z := c);
            myFunc(z := c, y := b, x := a);
        END_PROGRAM",
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);

    fn get_parameter_with_name<'a>(
        parameters_list: &[&'a AstStatement],
        expected_name: &str,
    ) -> &'a AstStatement {
        parameters_list
            .iter()
            .find(|it| {
                matches!(it, AstStatement::Assignment { left, .. } 
            if { matches!(&**left, AstStatement::Reference{name, ..} if {name == expected_name})})
            })
            .unwrap()
    }

    // all three call-statements should give the exact same annotations
    // the order of the parameters should not matter
    for call in &unit.implementations[1].statements {
        if let AstStatement::CallStatement {
            operator,
            parameters,
            ..
        } = call
        {
            //The call name should nave the correct type
            assert_eq!(
                Some("myFunc__DINT__INT"),
                annotations.get_call_name(operator)
            );
            //parameters should have the correct type
            if let Some(parameters) = &**parameters {
                let parameters_list = ast::flatten_expression_list(parameters);
                let [x, y, z] = [
                    get_parameter_with_name(&parameters_list, "x"),
                    get_parameter_with_name(&parameters_list, "y"),
                    get_parameter_with_name(&parameters_list, "z"),
                ];
                if let [AstStatement::Assignment {
                    left: x, right: a, ..
                }, AstStatement::Assignment {
                    left: y, right: b, ..
                }, AstStatement::Assignment {
                    left: z, right: c, ..
                }] = [x, y, z]
                {
                    assert_type_and_hint!(&annotations, &index, x, DINT_TYPE, None);
                    assert_type_and_hint!(&annotations, &index, a, INT_TYPE, Some(DINT_TYPE));

                    assert_type_and_hint!(&annotations, &index, y, DINT_TYPE, None);
                    assert_type_and_hint!(&annotations, &index, b, DINT_TYPE, Some(DINT_TYPE));

                    assert_type_and_hint!(&annotations, &index, z, INT_TYPE, None);
                    assert_type_and_hint!(&annotations, &index, c, INT_TYPE, Some(INT_TYPE));
                } else {
                    unreachable!("Not an assignment");
                }
            } else {
                unreachable!("No parameters")
            }
        }
    }
}

#[test]
fn builtin_generic_functions_do_not_get_specialized_calls() {
    let (unit, index) = index(
        "
        PROGRAM PRG
            VAR
                a : INT;
            END_VAR
            ADR(x := a);
            ADR(6);
            ADR(1.0);
            REF(x := a);
            REF(6);
            REF(1.0);
        END_PROGRAM",
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    //The implementations are not added to the index
    let implementations = annotations.new_index.get_implementations();
    assert!(!implementations.contains_key("adr__int"));
    assert!(!implementations.contains_key("adr__dint"));
    assert!(!implementations.contains_key("adr__real"));
    assert!(!implementations.contains_key("ref__int"));
    assert!(!implementations.contains_key("ref__dint"));
    assert!(!implementations.contains_key("ref__real"));

    //The pous are not added to the index
    let pous = annotations.new_index.get_pou_types();
    assert!(!pous.contains_key("adr__int"));
    assert!(!pous.contains_key("adr__dint"));
    assert!(!pous.contains_key("adr__real"));
    assert!(!pous.contains_key("ref__int"));
    assert!(!pous.contains_key("ref__dint"));
    assert!(!pous.contains_key("ref__real"));

    let call = &unit.implementations[0].statements[0];

    //The return type should have the correct type
    assert_type_and_hint!(&annotations, &index, call, LWORD_TYPE, None);

    let call = &unit.implementations[0].statements[1];

    //The return type should have the correct type
    assert_type_and_hint!(&annotations, &index, call, LWORD_TYPE, None);

    //The parameter should have the correct (original) type
    if let AstStatement::CallStatement { parameters, .. } = call {
        let params = flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(&annotations, &index, params[0], DINT_TYPE, Some(DINT_TYPE));
    } else {
        panic!("Expected call statement")
    }
    let call = &unit.implementations[0].statements[2];
    if let AstStatement::CallStatement { parameters, .. } = call {
        let params = flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(&annotations, &index, params[0], REAL_TYPE, Some(REAL_TYPE));
    } else {
        panic!("Expected call statement")
    }
}

#[test]
fn builtin_sel_param_type_is_not_changed() {
    let (unit, index) = index(
        "
    FUNCTION test : DINT
    VAR
        a,b: DINT;
    END_VAR
        SEL(FALSE,a,b);
    END_FUNCTION
    ",
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    //get the type/hints for a and b in the call, they should be unchanged (DINT, None)
    let call = &unit.implementations[0].statements[0];
    if let AstStatement::CallStatement { parameters, .. } = call {
        let params = flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(&annotations, &index, params[1], DINT_TYPE, Some(DINT_TYPE));
        assert_type_and_hint!(&annotations, &index, params[2], DINT_TYPE, Some(DINT_TYPE));
    } else {
        panic!("Expected call statement")
    }
}

#[test]
fn resolve_variadic_generics() {
    let (unit, index) = index(
        "
    FUNCTION ex<U: ANY> : U 
    VAR_INPUT
        ar : {sized}U...;
    END_VAR
    END_FUNCTION

    FUNCTION test : DINT
    VAR
        a,b: DINT;
    END_VAR
        ex(a,b);
    END_FUNCTION
    ",
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    //ex should resolve to ex__dint
    //a and b have the type dint
    let call = &unit.implementations[1].statements[0];
    //The call statement should return a DINT
    assert_type_and_hint!(&annotations, &index, call, DINT_TYPE, None);
    if let AstStatement::CallStatement {
        operator,
        parameters,
        ..
    } = call
    {
        assert_eq!(Some("ex__DINT"), annotations.get_call_name(operator));
        let params = flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(&annotations, &index, params[0], DINT_TYPE, Some(DINT_TYPE));
        assert_type_and_hint!(&annotations, &index, params[1], DINT_TYPE, Some(DINT_TYPE));
    } else {
        panic!("Expected call statement")
    }
}

#[test]
fn generic_call_gets_cast_to_biggest_type() {
    let (unit, index) = index(
        r"
 
    {external}
    FUNCTION MAX<T : ANY> : T
        VAR_INPUT
            args : {sized} T...;
        END_VAR
    END_FUNCTION
 
    FUNCTION main : LREAL
        MAX(SINT#5,DINT#1,LREAL#1.5,1.2);
    END_FUNCTION",
    );

    //Expecting all values to be LREAL
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    let call = &unit.implementations[1].statements[0];
    assert_type_and_hint!(&annotations, &index, call, LREAL_TYPE, None);
    //Call returns LREAL
    if let AstStatement::CallStatement { parameters, .. } = call {
        let params = ast::flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(&annotations, &index, params[0], SINT_TYPE, Some(LREAL_TYPE));
        assert_type_and_hint!(&annotations, &index, params[1], DINT_TYPE, Some(LREAL_TYPE));
        assert_type_and_hint!(
            &annotations,
            &index,
            params[2],
            LREAL_TYPE,
            Some(LREAL_TYPE)
        );
        assert_type_and_hint!(&annotations, &index, params[3], REAL_TYPE, Some(LREAL_TYPE));
    } else {
        panic!("Expected call statement")
    }
}

#[test]
fn sel_return_type_follows_params() {
    let (unit, index) = index(
        "
    FUNCTION main
        SEL(TRUE,1,2);
        SEL(TRUE,1,2) + 10;
        15 + SEL(TRUE,1,2);
    END_FUNCTION",
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    assert_type_and_hint!(
        &annotations,
        &index,
        &unit.implementations[0].statements[0],
        DINT_TYPE,
        None
    );
    assert_type_and_hint!(
        &annotations,
        &index,
        &unit.implementations[0].statements[1],
        DINT_TYPE,
        None
    );
    //Also test that the left side of the operator is dint
    assert_type_and_hint!(
        &annotations,
        &index,
        &unit.implementations[0].statements[2],
        DINT_TYPE,
        None
    );
    //Also test that the right side of the operator is dint
}

#[test]
fn mux_return_type_follows_params() {
    let (unit, index) = index(
        "
    FUNCTION main
        MUX(0,1,2);
        MUX(0,1,2) + 10;
        15 + MUX(0,1,2);
    END_FUNCTION",
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    assert_type_and_hint!(
        &annotations,
        &index,
        &unit.implementations[0].statements[0],
        DINT_TYPE,
        None
    );
    assert_type_and_hint!(
        &annotations,
        &index,
        &unit.implementations[0].statements[1],
        DINT_TYPE,
        None
    );
    //Also test that the left side of the operator is dint
    assert_type_and_hint!(
        &annotations,
        &index,
        &unit.implementations[0].statements[2],
        DINT_TYPE,
        None
    );
    //Also test that the right side of the operator is dint
}

#[test]
fn auto_pointer_of_generic_resolved() {
    let (unit, mut index) = index(
        " FUNCTION LEFT<T : ANY> : T
        VAR_IN_OUT
            IN : T;
        END_VAR
        END_FUNCTION
    
        FUNCTION LEFT_EXT<T : ANY> : DINT
        VAR_IN_OUT
            IN : T;
        END_VAR
        END_FUNCTION
    
        FUNCTION LEFT__DINT : DINT
        VAR_INPUT
            IN : DINT;
        END_VAR
            LEFT_EXT(IN);
        END_FUNCTION
        ",
    );

    annotate(&unit, &mut index);

    let member = index.find_member("LEFT_EXT__DINT", "IN").unwrap();
    let dt = index
        .find_effective_type_info(&member.data_type_name)
        .unwrap();
    if let DataTypeInformation::Pointer {
        inner_type_name,
        auto_deref: true,
        ..
    } = dt
    {
        assert_eq!(inner_type_name, "DINT")
    } else {
        panic!("Expecting a pointer to dint, found {:?}", dt)
    }
}

#[test]
fn string_ref_as_generic_resolved() {
    let (unit, mut index) = index(
        " FUNCTION LEFT<T: ANY_STRING> : T
        VAR_INPUT {ref}
            IN : T;
        END_VAR
        END_FUNCTION
    
        FUNCTION LEFT_EXT<T: ANY_STRING> : DINT
        VAR_INPUT {ref}
            IN : T;
        END_VAR
        END_FUNCTION
    
        FUNCTION LEFT__STRING : STRING 
        VAR_INPUT
            IN : STRING;
        END_VAR
            LEFT_EXT(IN);
        END_FUNCTION
        ",
    );

    let annotations = annotate(&unit, &mut index);

    let call_statement = &unit.implementations[2].statements[0];

    if let AstStatement::CallStatement { parameters, .. } = call_statement {
        let parameters = flatten_expression_list(parameters.as_ref().as_ref().unwrap());

        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[0],
            "__LEFT__STRING_IN",
            None
        );
    } else {
        unreachable!("Should be a call statement")
    }

    assert_type_and_hint!(&annotations, &index, call_statement, DINT_TYPE, None);

    let member = index.find_member("LEFT_EXT__STRING", "IN").unwrap();
    let dt = index
        .find_effective_type_info(&member.data_type_name)
        .unwrap();
    if let DataTypeInformation::Pointer {
        inner_type_name,
        auto_deref: true,
        ..
    } = dt
    {
        assert_eq!(inner_type_name, STRING_TYPE)
    } else {
        panic!("Expecting auto deref pointer to string, found {:?}", dt)
    }
}

#[test]
fn resolved_generic_any_real_call_with_ints_added_to_index() {
    // Make sure INTs implementations were not added to index
    let (unit, index) = index(
        "
        FUNCTION myFunc<T: ANY_REAL> : T
        VAR_INPUT   x : T;  END_VAR
        END_FUNCTION

        FUNCTION myFunc__REAL : REAL
        VAR_INPUT   x : REAL; END_VAR
        END_FUNCTION

        PROGRAM PRG
            VAR
                a : INT;
				b : UINT;
            END_VAR
            myFunc(REAL#1.0);
            myFunc(SINT#1);
            myFunc(a);
            myFunc(DINT#1);
			myFunc(LINT#1);

			myFunc(USINT#1);
            myFunc(b);
            myFunc(UDINT#1);
			myFunc(ULINT#1);
        END_PROGRAM",
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    // The implementations are added to the index
    let implementations = annotations.new_index.get_implementations();
    assert!(implementations.contains_key("myfunc__lreal"));
    assert_eq!(1, implementations.len()); //make sure REAL-implementation was not added by the annotator

    //The pous are added to the index
    let pous = annotations.new_index.get_pous();
    assert!(pous.contains_key("myfunc__lreal"));
    assert_eq!(1, pous.len()); //make sure REAL-implementation was not added by the annotator

    //Each POU has members
    assert_eq!(
        "LREAL",
        annotations
            .new_index
            .find_member("myfunc__lreal", "x")
            .unwrap()
            .get_type_name()
    );
    assert_eq!(
        "LREAL",
        annotations
            .new_index
            .find_member("myfunc__lreal", "myfunc__lreal")
            .unwrap()
            .get_type_name()
    );
}

#[test]
fn generic_string_functions_are_annotated_correctly() {
    let (unit, index) = index(
        "
        FUNCTION foo<T: ANY_STRING> : T
        VAR_INPUT {ref}
            in : T;
        END_VAR        
        END_FUNCTION

        FUNCTION foo__STRING : STRING
        VAR_INPUT {ref}
            param : STRING;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            s1 : STRING;
        END_VAR
            s1 := '     this is   a  very   long          sentence   with plenty  of    characters.';
            foo(s1);
        END_PROGRAM
        ",
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);

    let mut functions = vec![];
    let mut values = vec![];
    annotations
        .type_map
        .iter()
        .map(|(_, v)| v)
        .for_each(|v| match v {
            StatementAnnotation::Function { .. } => functions.push(v),
            StatementAnnotation::Value {
                resulting_type: res_type,
            } => values.push(res_type),
            _ => (),
        });

    assert_eq!(
        functions[0],
        &StatementAnnotation::Function {
            return_type: "__foo__STRING_return".to_string(),
            qualified_name: "foo__STRING".to_string(),
            call_name: None,
        }
    );

    assert_eq!(values[0], &String::from("__STRING_80"),)
}

#[test]
fn generic_string_functions_with_non_default_length_are_annotated_correctly() {
    let (unit, index) = index(
        "
        FUNCTION foo<T: ANY_STRING> : T
        VAR_INPUT {ref}
            in : T;
        END_VAR      
        VAR_OUTPUT {ref}
            out : T;
        END_VAR
        END_FUNCTION

        FUNCTION foo__STRING : STRING[100]
        VAR_INPUT {ref}
            param : STRING[100];
        END_VAR
        VAR_OUTPUT
            s : STRING[100];
        END_VAR
            s := param;
        END_FUNCTION

        PROGRAM main
        VAR
            s1 : STRING;
            s2 : STRING[100];
        END_VAR
            s1 := '     this is   a  very   long          sentence   with plenty  of      characters and weird spacing.';
            s2 := foo(s1);
        END_PROGRAM
        ",
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);

    let mut functions = vec![];
    let mut values = vec![];
    annotations
        .type_map
        .iter()
        .map(|(_, v)| v)
        .for_each(|v| match v {
            StatementAnnotation::Function { .. } => functions.push(v),
            StatementAnnotation::Value {
                resulting_type: res_type,
            } => values.push(res_type),
            _ => (),
        });

    assert_eq!(
        functions[0],
        &StatementAnnotation::Function {
            return_type: "__foo__STRING_return".to_string(),
            qualified_name: "foo__STRING".to_string(),
            call_name: None,
        }
    );

    assert_eq!(values[0], &String::from("__STRING_100"),)
}

#[test]
fn generic_string_functions_are_annotated_correctly_when_given_parameter_is_shorter_than_in_function_definition() {
    let (unit, index) = index(
        "
        FUNCTION COPY<T: ANY_STRING> : T
        VAR_INPUT {ref}
            in : T;
        END_VAR    
        END_FUNCTION

        {external}
        FUNCTION COPY_EXT<T: ANY_STRING> : DINT
        VAR_INPUT {ref}
            IN : T;
        END_VAR
        VAR_IN_OUT
            OUT: T;
        END_VAR
        END_FUNCTION

        FUNCTION COPY__STRING : STRING[2048]
        VAR_INPUT {ref}
            param : STRING[2048];
        END_VAR
            bar(param, COPY__STRING);
        END_FUNCTION

        {external}
        FUNCTION COPY_EXT__STRING : DINT
        VAR_INPUT {ref}
            IN : STRING[2048];
        END_VAR
        VAR_IN_OUT
            OUT : STRING[2048];
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            s : STRING[1024];
        END_VAR
            foo('     this is   a  very   long          sentence   with plenty  of      characters and weird spacing.', s);
        END_PROGRAM
        ",
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);

    let mut functions = vec![];
    let mut values = vec![];
    annotations
        .type_map
        .iter()
        .map(|(_, v)| v)
        .for_each(|v| match v {
            StatementAnnotation::Function { .. } => functions.push(v),
            StatementAnnotation::Value {
                resulting_type: res_type,
            } => values.push(res_type),
            _ => (),
        });

    assert_eq!(
        functions[0],
        &StatementAnnotation::Function {
            return_type: "__COPY__STRING_return".to_string(),
            qualified_name: "COPY__STRING".to_string(),
            call_name: None,
        }
    );

    assert_eq!(values[0], &String::from("__STRING_2048"),)
}
#[test]
fn placeholder() {
    let (unit, index) = index(
        r#"                
        {external}
        FUNCTION LEN <T: ANY_STRING> : DINT
        VAR_INPUT {ref}
            IN : T;
        END_VAR
        END_FUNCTION

        FUNCTION main
        VAR_
            len('abc');
        END_FUNCTION
        "#
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);

    let mut functions = vec![];
    let mut values = vec![];
    annotations
        .type_map
        .iter()
        .map(|(_, v)| v)
        .for_each(|v| match v {
            StatementAnnotation::Function { .. } => functions.push(v),
            StatementAnnotation::Value {
                resulting_type: res_type,
            } => values.push(res_type),
            _ => (),
        });

    assert_eq!(
        functions[0],
        &StatementAnnotation::Function {
            return_type: "__LEN__STRING_return".to_string(),
            qualified_name: "LEN__STRING".to_string(),
            call_name: None,
        }
    );

    assert_eq!(values[0], &String::from("DINT"),)
}

#[test]
fn generic_return_type_name_resolved_correctly() {
    let (unit, index) = index(
        "
    FUNCTION foo<T: ANY_INT> : T
    VAR_INPUT
        param: T;
    END_VAR
    END_FUNCTION

    FUNCTION main
        foo(3);
    END_FUNCTION",
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);

    let mut functions = vec![];
    let mut values = vec![];
    annotations
        .type_map
        .iter()
        .map(|(_, v)| v)
        .for_each(|v| match v {
            StatementAnnotation::Function { .. } => functions.push(v),
            StatementAnnotation::Value {
                resulting_type: res_type,
            } => values.push(res_type),
            _ => (),
        });

    assert_eq!(
        functions[0],
        &StatementAnnotation::Function {
            return_type: "DINT".to_string(),
            qualified_name: "foo".to_string(),
            call_name: Some("foo__DINT".to_string()),
        }
    );

    assert_eq!(values[0], &String::from("DINT"),)
}


#[test]
fn literal_string_as_parameter_resolves_correctly() {
    let (unit, index) = index(r#"
        FUNCTION foo<T: ANY_STRING> : T
        VAR_INPUT
            x : T;
        END_VAR
        END_FUNCTION

        FUNCTION main : DINT
            foo('     this is   a  very   long          sentence   with');
        END_FUNCTION
    "#);

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit);
    let statement = &unit.implementations[1].statements[0];
    
    
    if let AstStatement::CallStatement { operator, parameters, .. } =  statement {
        let parameters = flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(&annotations, &index, parameters[0], STRING_TYPE, None)     ;               
        assert_type_and_hint!(&annotations, &index, statement, STRING_TYPE, None);
        assert_eq!(
            annotations.get(operator).unwrap(),
            &StatementAnnotation::Function {
                return_type: "STRING".to_string(),
                qualified_name: "foo".to_string(),
                call_name: Some("foo__STRING".to_string()),
            }
        );        
        
    } else {
        unreachable!("This should always be a call statement.")
    }

}
