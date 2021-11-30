use crate::{
    assert_type_and_hint,
    ast::{self, AstStatement},
    resolver::TypeAnnotator,
    test_utils::tests::index,
    typesystem::{BYTE_TYPE, DINT_TYPE, INT_TYPE, REAL_TYPE},
};

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
    let annotations = TypeAnnotator::visit_unit_without_index(&index, &unit);
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
    let annotations = TypeAnnotator::visit_unit_without_index(&index, &unit);

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
