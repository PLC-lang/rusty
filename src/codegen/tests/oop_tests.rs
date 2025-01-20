use crate::test_utils::tests::codegen;

#[test]
fn members_from_base_class_are_available_in_subclasses() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK foo
        VAR
            a : INT;
            b : STRING;
            c : ARRAY[0..10] OF STRING;
        END_VAR
        END_FUNCTION_BLOCK
      
        FUNCTION_BLOCK bar EXTENDS foo
        END_FUNCTION_BLOCK
        "#,
    );
    insta::assert_snapshot!(result, @r###""###);
}

#[test]
fn write_to_parent_variable_qualified_access() {
    let res = codegen(
        "
        FUNCTION_BLOCK fb
        VAR
            x : INT;
            y : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb
        END_FUNCTION_BLOCK
        
        FUNCTION_BLOCK foo
        VAR 
            myFb : fb2;
        END_VAR
            myFb.x = 1;
        END_FUNCTION_BLOCK
       ",
    );

    insta::assert_snapshot!(res);
}

#[test] // TODO: add lit test for this
fn write_to_parent_variable_in_instance() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK foo
        VAR 
            s : STRING;
        END_VAR
        METHOD baz
            s := 'hello';
        END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo 
            s := 'world';
        END_FUNCTION_BLOCK

        FUNCTION main
        VAR 
            s: STRING;
            fb: bar;
        END_VAR
            fb.baz();
            fb();
        END_FUNCTION
    "#,
    );
    insta::assert_snapshot!(result, @r###""###);
}
