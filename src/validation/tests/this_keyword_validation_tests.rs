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
    todo!();
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
        this^.x := this^.this^.this^.y;
        this^.this^.this^.x := this^.y;
    END_FUNCTION_BLOCK
    "#,
    );
    dbg!(&diagnostics);
    assert_snapshot!(diagnostics, @r"");
}

#[test]
fn this_in_method_call_chain_is_allowed() {
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
fn this_in_program_is_not_allowed_() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM Main
            VAR
                x : INT := 5;
            END_VAR
            x := THIS^.x;
        END_PROGRAM
    "#,
    );
    assert_snapshot!(diagnostics, @r"
    error[E120]: Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK` or type `METHOD`
      ┌─ <internal>:6:18
      │
    6 │             x := THIS^.x;
      │                  ^^^^ Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK` or type `METHOD`
    ");
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
    assert_snapshot!(diagnostics, @r"
    error[E120]: Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK` or type `METHOD`
      ┌─ <internal>:6:25
      │
    6 │         SomeFunction := THIS^.SomeValue;
      │                         ^^^^ Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK` or type `METHOD`
    ");
}

#[test]
fn this_cannot_be_assigned_to() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION_BLOCK parent
    VAR
        x : LINT := 10;
        p : REF_TO parent;
    END_VAR
        this^ := 5;
        this := REF(x);
        this := REF(parent);
        this := ADR(parent); // this is not allowed
        this^:= parent;
        this := p;
    END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r"
    error[E037]: Invalid assignment: cannot assign 'DINT' to 'parent'
      ┌─ <internal>:7:9
      │
    7 │         this^ := 5;
      │         ^^^^^^^^^^ Invalid assignment: cannot assign 'DINT' to 'parent'

    error[E050]: Expression this is not assignable.
      ┌─ <internal>:8:9
      │
    8 │         this := REF(x);
      │         ^^^^ Expression this is not assignable.

    warning[E090]: Pointers parent and LINT have different types
      ┌─ <internal>:8:9
      │
    8 │         this := REF(x);
      │         ^^^^^^^^^^^^^^ Pointers parent and LINT have different types

    error[E050]: Expression this is not assignable.
      ┌─ <internal>:9:9
      │
    9 │         this := REF(parent);
      │         ^^^^ Expression this is not assignable.

    error[E050]: Expression this is not assignable.
       ┌─ <internal>:10:9
       │
    10 │         this := ADR(parent); // this is not allowed
       │         ^^^^ Expression this is not assignable.
    ");
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
            GetVal := THIS^.val;
        END_METHOD
        val := this^.val;
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn pass_this_to_method_is_ok() {
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

        END_METHOD
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
    todo!();
}

#[test]
fn shadowing_is_working() {
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
    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn nested_fbs_and_this_passing() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK OuterFB
            VAR
                Inner : InnerFB;
            END_VAR

            METHOD CallInner : INT
                Inner.UseOuter(THIS);
            END_METHOD
            METHOD doSomething : INT
                VAR
                    x : INT := 5;
                END_VAR
                x := 10;
            END_METHOD

        END_FUNCTION_BLOCK

        FUNCTION_BLOCK InnerFB
            METHOD UseOuter : INT
            VAR_INPUT
                ref : REF_TO OuterFB;
            END_VAR
                ref^.doSomething();
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn this_as_method_argument_is_ok() {
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
            this^ := test;
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r"
    error[E050]: Expression this is not assignable.
      ┌─ <internal>:8:13
      │
    8 │             this := ADR(test); // this is not allowed
      │             ^^^^ Expression this is not assignable.

    error[E037]: Invalid assignment: cannot assign 'FB_Test' to 'FB_Test2'
      ┌─ <internal>:9:13
      │
    9 │             this^ := test;
      │             ^^^^^^^^^^^^^ Invalid assignment: cannot assign 'FB_Test' to 'FB_Test2'
    ");
}

#[test]
fn this_chained_with_super_is_not_ok() {
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
    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` is not allowed in member-access position.
      ┌─ <internal>:8:19
      │
    8 │             this^.super^.this^.DoSomething();
      │                   ^^^^^ `SUPER` is not allowed in member-access position.

    error[E120]: `THIS` is not allowed in member-access position.
      ┌─ <internal>:8:26
      │
    8 │             this^.super^.this^.DoSomething();
      │                          ^^^^ `THIS` is not allowed in member-access position.
    ");
}

#[test]
fn this_in_properties_is_ok() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                prop_var : INT;
            END_VAR
            PROPERTY prop : INT
                GET
                    prop := THIS^.prop_var;
                END_GET
                SET
                    THIS^.prop_var := prop;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_calling_function_and_passing_this_is_ok() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                x : INT;
            END_VAR
            METHOD return_x : INT
                VAR_INPUT
                    fb_from_foo : REF_TO FB_Test;
                END_VAR
                return_x := fb_from_foo^.x;
            END_METHOD
            foo(this);
        END_FUNCTION_BLOCK
        FUNCTION foo : INT
            VAR_INPUT
                pfb: REF_TO FB_TEST;
            END_VAR
            foo := pfb^.return_x(pfb);
        END_FUNCTION
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
fn this_in_property_calling_method_is_ok() {
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
    todo!();
}

#[test]
fn this_with_self_pointer() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                refToSelf : POINTER TO FB_Test;
            END_VAR

            METHOD InitRef
                refToSelf := ADR(THIS^);
                refToSelf := REF(THIS^);
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
    todo!();
}

#[test]
fn this_in_variable_initialization_is_ok() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB
            VAR CONSTANT
                x : INT;
            END_VAR
            VAR
                y : INT := this^.x;
            END_VAR
        END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}

// TODO: test with incompatible types (refToSelf gets assigned something of different type)
// TODO: global namespaces operator tests
// TODO: .this^ tests
// TODO: codegen tests
// TODO: lit tests
// TODO: resolver tests (parenthesized expressions, nested binary expressions ...)
// TODO: this in variable initializers

#[test]
fn dummy() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}
