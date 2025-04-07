use insta::assert_snapshot;
use test_utils::parse_and_validate_buffered;

#[test]
fn pointer_arithmetic_with_this() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION_BLOCK parent
    VAR
        x : LINT := 10;
        y : LINT := 20;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK child EXTENDS parent
    VAR
        a : INT;
    END_VAR
        // Pointer arithmetic with SUPER
        a := (THIS + 1)^ + 5;
    END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn cant_chain_this() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION_BLOCK parent
    VAR
        x : LINT := 10;
        y : LINT := 20;
    END_VAR
        this^.x := this^.this^.y;
        this^.this^.x := this^.y;
    END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
    panic!("This test should not work");
}

#[test]
fn this_in_method_call_chain() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                counter : INT := 0;
            END_VAR

            METHOD Step
                THIS^.Increment();
            END_METHOD

            METHOD Increment
                counter := counter + 1;
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_not_allowed_in_program() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM Main
            VAR
                x : INT := 5;
            END_VAR
            x := THIS.x;
        END_PROGRAM
    "#,
    );
    assert_snapshot!(diagnostics, @r###"
"###);
}

#[test]
fn this_in_function_is_not_allowed() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION SomeFunction : INT
        VAR
            SomeValue : INT := 5;
        END_VAR
        SomeFunction := THIS^.SomeValue;
        END_FUNCTION
    "#,
    );
    assert_snapshot!(diagnostics, @r###"
"###);
}

#[test]
fn cant_assign_to_this() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION_BLOCK parent
    VAR
        x : LINT := 10;
    END_VAR
        this^ := 5;
        this := REF(x);
    END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn basic_use() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
        VAR
            val : INT := 5;
        END_VAR

        METHOD GetVal : INT
        GetVal := THIS.val;
        END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn pass_this_to_method() {
    // pass `this` pointer of FB1 to a method of another fb called FB2 which calls a method of FB1
    // and changes a value of the passed `this` pointer
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
        METHOD increment_from_other
            VAR
                test : FB_Test;
            END_VAR
            test.method2(THIS);

        END_FUNCTION_BLOCK
        FUNCTION_BLOCK FB_Test2
        METHOD method1
            VAR
                test : FB_Test;
            END_VAR
            test.method2(THIS);
        END_METHOD
        METHOD method2
            VAR
                test : FB_Test;
            END_VAR
            test := THIS;
        END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn simple_shadowing() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
        VAR
            val : INT := 5;
        END_VAR
        METHOD shadow_val
            VAR
                val : INT := 10;
                local_val: INT;
                shadow_val : INT;
            END_VAR
            local_val := THIS^.val;
            shadow_val := val;
        END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn nested_fbs_and_this_passing() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK OuterFB
            VAR
                Inner : InnerFB;
            END_VAR

            METHOD CallInner
                Inner.UseOuter(THIS);
            END_METHOD
            METHOD DoSomething
                VAR
                    x : INT := 5;
                END_VAR
                x := 10;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK InnerFB
            METHOD UseOuter
            VAR_INPUT
                ref : OuterFB;
            END_VAR
                ref.DoSomething();
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_as_method_argument() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                helper : FB_Helper;
            END_VAR
            METHOD CallHelper
                helper.DoSomething(THIS^);
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK FB_Helper
            METHOD DoSomething
                VAR_INPUT input : FB_Test; END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_in_recursive_method() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                count : INT := 0;
            END_VAR
            METHOD recursive_method
                IF count < 3 THEN
                    count := count + 1;
                    THIS^.recursive_method();
                END_IF
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_is_read_only() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK FB_Test2
            VAR
                test : FB_Test;
            END_VAR
            this := ADR(test); // this is not allowed
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_chained_with_super() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            METHOD DoSomething : DINT
                DoSomething := 5;
            END_METHOD
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK child EXTENDS parent
            this^.super^.this^.DoSomething();
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_in_properties() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                prop : INT;
            END_VAR
            PROPERTY GetProp : INT
                GetProp := THIS^.prop;
            END_PROPERTY
            PROPERTY SetProp : INT
                SetProp := THIS^.prop;
            END_PROPERTY
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_in_property_calling_method() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                x : INT;
            END_VAR

            METHOD DoubleX : INT
                DoubleX := 2 * THIS^.x;
            END_METHOD

            PROPERTY Value : INT
                GET
                    Value := THIS^.DoubleX();
                END_GET
            END_PROPERTY
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_with_adr_pointer() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                refToSelf : POINTER TO FB_Test;
            END_VAR

            METHOD InitRef
                refToSelf := ADR(THIS^);
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn dummy() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}
