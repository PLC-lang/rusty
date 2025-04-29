use insta::assert_snapshot;
use test_utils::parse_and_validate_buffered;

#[test]
fn this_pointer_arithmetic_is_ok() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK fb
        VAR
            a : INT;
        END_VAR
            // Pointer arithmetic with THIS
            a := (THIS + 1)^ + 5;
        END_FUNCTION_BLOCK
    "#,
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn chaining_this_is_not_allowed() {
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
    assert_snapshot!(diagnostics, @r"
    error[E120]: `THIS` is not allowed in member-access position.
      ┌─ <internal>:7:26
      │
    7 │         this^.x := this^.this^.this^.y;
      │                          ^^^^ `THIS` is not allowed in member-access position.

    error[E120]: `THIS` is not allowed in member-access position.
      ┌─ <internal>:7:32
      │
    7 │         this^.x := this^.this^.this^.y;
      │                                ^^^^ `THIS` is not allowed in member-access position.

    error[E120]: `THIS` is not allowed in member-access position.
      ┌─ <internal>:8:15
      │
    8 │         this^.this^.this^.x := this^.y;
      │               ^^^^ `THIS` is not allowed in member-access position.

    error[E120]: `THIS` is not allowed in member-access position.
      ┌─ <internal>:8:21
      │
    8 │         this^.this^.this^.x := this^.y;
      │                     ^^^^ `THIS` is not allowed in member-access position.
    ");
}

#[test]
fn this_in_method_call_chain_is_ok() {
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
    assert!(diagnostics.is_empty());
}

#[test]
fn assignment_to_this_is_not_allowed() {
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

    error[E050]: Expression this is not assignable.
       ┌─ <internal>:12:9
       │
    12 │         this := p;
       │         ^^^^ Expression this is not assignable.
    ");
}

#[test]
fn this_in_method_and_body_in_function_block_is_ok() {
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
    assert!(diagnostics.is_empty());
}

#[test]
fn pass_this_to_method_is_ok() {
    // pass `this` pointer of FB1 to a method of another fb called FB2 which calls a method of FB1
    // and changes a value of the passed `this` pointer
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
        METHOD foo
            VAR
                test : FB_Test2;
            END_VAR
            test.bar(THIS^);
        END_METHOD
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK FB_Test2
            VAR_INPUT
                test : FB_Test;
            END_VAR
            METHOD bar: INT
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert!(diagnostics.is_empty());
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
#[ignore = "reference resolve issue"]
/// #TODO: #THIS reference gh issue
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
fn this_in_recursive_method_is_ok() {
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
    assert!(diagnostics.is_empty());
}

#[test]
fn this_chained_with_super_is_not_ok() {
    let diagnostics = parse_and_validate_buffered(
        // TODO: #THIS check with Michael (nested)
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
fn this_calling_function_and_passing_this_is_ok() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            foo(this);
        END_FUNCTION_BLOCK
        FUNCTION foo : INT
            VAR_INPUT
                pfb: REF_TO FB_TEST;
            END_VAR
        END_FUNCTION
    "#,
    );
    assert!(diagnostics.is_empty());
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
                SET
                    this^.x := Value;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
    "#,
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn this_with_self_pointer_is_ok() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                refToSelf : REF_TO FB_Test;
            END_VAR

            METHOD InitRef
                refToSelf := ADR(THIS^);
                refToSelf := REF(THIS^);
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert!(diagnostics.is_empty());
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
    assert!(diagnostics.is_empty());
}

// TODO: test with incompatible types (refToSelf gets assigned something of different type)
// TODO: global namespaces operator tests
// TODO: .this^ tests
// TODO: codegen tests
// TODO: lit tests
// TODO: resolver tests (parenthesized expressions, nested binary expressions ...)
// TODO: this in variable initializers
#[test]
fn this_in_unsupported_pous_is_not_allowed() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM foo
            VAR
                x : INT;
            END_VAR

            METHOD bar : INT
                bar := THIS^.x; // this in method in program is not allowed
            END_METHOD
            THIS^;
        END_PROGRAM

        ACTION foo.act
            THIS^;
        END_ACTION

        FUNCTION baz : INT
            THIS^;
        END_FUNCTION
    "#,
    );
    assert_snapshot!(diagnostics, @r"
    error[E120]: Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK`
      ┌─ <internal>:8:24
      │
    8 │                 bar := THIS^.x; // this in method in program is not allowed
      │                        ^^^^ Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK`

    error[E120]: Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK`
       ┌─ <internal>:10:13
       │
    10 │             THIS^;
       │             ^^^^ Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK`

    error[E120]: Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK`
       ┌─ <internal>:14:13
       │
    14 │             THIS^;
       │             ^^^^ Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK`

    error[E120]: Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK`
       ┌─ <internal>:18:13
       │
    18 │             THIS^;
       │             ^^^^ Invalid use of `THIS`. Usage is only allowed within POU of type `FUNCTION_BLOCK`
    ");
}

#[test]
fn this_in_action_in_functionblock_is_ok() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK fb
        END_FUNCTION_BLOCK

        ACTION fb.foo
            THIS^;
        END_ACTION
    "#,
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn this_calling_functionblock_body_from_method_is_ok() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK fb
            METHOD foo : INT
                THIS^();
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    assert!(diagnostics.is_empty());
}

#[test]
fn dummy() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}
