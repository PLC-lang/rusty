use crate::{
    assert_type_and_hint,
    ast::{self, flatten_expression_list, AstStatement},
    resolver::{AnnotationMap, TypeAnnotator},
    test_utils::tests::index,
    typesystem::{BYTE_TYPE, DINT_TYPE, INT_TYPE, LWORD_TYPE, REAL_TYPE},
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
        assert_type_and_hint!(&annotations, &index, params[0], DINT_TYPE, None);
    } else {
        panic!("Expected call statement")
    }
    let call = &unit.implementations[0].statements[2];
    if let AstStatement::CallStatement { parameters, .. } = call {
        let params = flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(&annotations, &index, params[0], REAL_TYPE, None);
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
        assert_type_and_hint!(&annotations, &index, params[1], DINT_TYPE, None);
        assert_type_and_hint!(&annotations, &index, params[2], DINT_TYPE, None);
    } else {
        panic!("Expected call statement")
    }
}
