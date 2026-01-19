use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;
/// # Architecture Design Record: Enums
/// An Enum (or Enumeration) is a special datatype in ST. An Enum datatype represents a numeric datatype (default is i32)
/// with a list of  well defined values with dedicated qualified names (e.g. `@qualifier.red`, `@qualifier.yellow`, `@qualifier.green`).
///
/// e.g.
/// ```st
/// TYPE MyEnum: (element1, element2, element3);
///     END_TYPE
/// ```
#[test]
fn enums_generate_a_global_constants_for_each_element() {
    let src = r#"
        TYPE Color : (red, yellow, green);
        END_TYPE;

        VAR_GLOBAL
            myColor : Color;
        END_VAR"#;
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myColor = global i32 0
    @Color.red = unnamed_addr constant i32 0
    @Color.yellow = unnamed_addr constant i32 1
    @Color.green = unnamed_addr constant i32 2
    "#);
}

/// The values of the enum constants are stored as (enum-local) unique numeric values. The values and their
/// datatype (see declaration of `State : BYTE`) can be defined by the user via an initialization statement,
/// or automatically assigned by the compiler.
/// `i32` is the default datatype for enums if no other type is specified (e.g. `Color`).
/// Values that are assigned by the compiler get unique ascending values.
#[test]
fn enums_constants_are_automatically_numbered_or_user_defined() {
    let src = r#"
        TYPE Color : (red := 1, yellow := 2, green := 4, blue := 8);
        END_TYPE;

        TYPE State : BYTE (open := 1, closed := 4, idle, running);
        END_TYPE;

        VAR_GLOBAL
            myColor : Color;
            myState : State;
        END_VAR"#;

    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myColor = global i32 1
    @myState = global i8 1
    @Color.red = unnamed_addr constant i32 1
    @Color.yellow = unnamed_addr constant i32 2
    @Color.green = unnamed_addr constant i32 4
    @Color.blue = unnamed_addr constant i32 8
    @State.open = unnamed_addr constant i8 1
    @State.closed = unnamed_addr constant i8 4
    @State.idle = unnamed_addr constant i8 5
    @State.running = unnamed_addr constant i8 6
    "#);
}

/// Enum types can be declared as dedicated DataTypes (see above) or direclty as part of a
/// variable declaration. Note that declaring the same enum twice will result in two distinct
/// enum-datatypes (and separate constant-variables - e.g. `@red` and `@red.1`).
#[test]
fn inline_declaration_of_enum_types() {
    let src = r#"
        VAR_GLOBAL
            frontColor : (red, green, yellow);
            backColor  : (red, green, yellow);
        END_VAR"#;

    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @frontColor = global i32 0
    @backColor = global i32 0
    @__global_frontColor.red = unnamed_addr constant i32 0
    @__global_frontColor.green = unnamed_addr constant i32 1
    @__global_frontColor.yellow = unnamed_addr constant i32 2
    @__global_backColor.red = unnamed_addr constant i32 0
    @__global_backColor.green = unnamed_addr constant i32 1
    @__global_backColor.yellow = unnamed_addr constant i32 2
    "#);
}

/// Enum elements can accessed like global variables. If there are mulitple candidates, one can
/// pick a specific element by qualifying it with the enum's name (note that this is not possible for inline
/// enums).
/// In this example the enum-element `open` is defined in the type `ProcessState` as well as in `Door`. We can now
/// select a specific element by qualifying it via `ProcessState#closed` or `Door#closed` to access the generated global
/// variables `@closed` or `@closed.2`.
#[test]
fn using_enums() {
    let src = r#"
        TYPE ProcessState : (open := 1, closed := 4, idle, running);
        END_TYPE;

        TYPE Door : (open := 8, closed := 16);
        END_TYPE;

        PROGRAM prg
            VAR x, y, z : DINT; END_VAR

            x := idle;
            y := ProcessState#closed;
            z := Door#closed;
        END_PROGRAM
    "#;

    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { i32, i32, i32 }

    @prg_instance = global %prg zeroinitializer
    @ProcessState.open = unnamed_addr constant i32 1
    @ProcessState.closed = unnamed_addr constant i32 4
    @ProcessState.idle = unnamed_addr constant i32 5
    @ProcessState.running = unnamed_addr constant i32 6
    @Door.open = unnamed_addr constant i32 8
    @Door.closed = unnamed_addr constant i32 16

    define void @prg(ptr %0) {
    entry:
      %x = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      %z = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 2
      store i32 5, ptr %x, align 4
      store i32 4, ptr %y, align 4
      store i32 16, ptr %z, align 4
      ret void
    }
    "#);
}

/// If zero is defined in an enum and no default value is specified,
/// the enum should be initialized with 0
#[test]
fn enum_with_zero_element_no_default_initializes_to_zero() {
    let src = r#"
        TYPE STATE_WITH_ZERO : BYTE (
            idle := 0,
            running := 1,
            stopped := 2
        );
        END_TYPE

        VAR_GLOBAL
            myState : STATE_WITH_ZERO;
        END_VAR"#;

    // Should initialize to 0 (idle)
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myState = global i8 0
    @STATE_WITH_ZERO.idle = unnamed_addr constant i8 0
    @STATE_WITH_ZERO.running = unnamed_addr constant i8 1
    @STATE_WITH_ZERO.stopped = unnamed_addr constant i8 2
    "#);
}

/// If zero is defined in an enum with a default value,
/// the enum should be initialized with the default value
#[test]
fn enum_with_zero_element_and_default_initializes_to_default() {
    let src = r#"
        TYPE STATE_WITH_DEFAULT : BYTE (
            idle := 0,
            running := 1,
            stopped := 2
        ) := running;
        END_TYPE

        VAR_GLOBAL
            myState : STATE_WITH_DEFAULT;
        END_VAR"#;

    // Should initialize to 1 (running)
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myState = global i8 1
    @STATE_WITH_DEFAULT.idle = unnamed_addr constant i8 0
    @STATE_WITH_DEFAULT.running = unnamed_addr constant i8 1
    @STATE_WITH_DEFAULT.stopped = unnamed_addr constant i8 2
    "#);
}

/// If no zero is defined and no default value is specified,
/// the enum should be initialized with the first element
#[test]
fn enum_without_zero_no_default_initializes_to_first_element() {
    let src = r#"
        TYPE PRIORITY : INT (
            low := 10,
            medium := 20,
            high := 30
        );
        END_TYPE

        VAR_GLOBAL
            myPriority : PRIORITY;
        END_VAR"#;

    // Should initialize to 10 (low - first element)
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myPriority = global i16 10
    @PRIORITY.low = unnamed_addr constant i16 10
    @PRIORITY.medium = unnamed_addr constant i16 20
    @PRIORITY.high = unnamed_addr constant i16 30
    "#);
}

/// If no zero is defined but a default value is specified,
/// the enum should be initialized with the default value
#[test]
fn enum_without_zero_with_default_initializes_to_default() {
    let src = r#"
        TYPE PRIORITY_WITH_DEFAULT : INT (
            low := 10,
            medium := 20,
            high := 30
        ) := medium;
        END_TYPE

        VAR_GLOBAL
            myPriority : PRIORITY_WITH_DEFAULT;
        END_VAR"#;

    // Should initialize to 20 (medium)
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myPriority = global i16 20
    @PRIORITY_WITH_DEFAULT.low = unnamed_addr constant i16 10
    @PRIORITY_WITH_DEFAULT.medium = unnamed_addr constant i16 20
    @PRIORITY_WITH_DEFAULT.high = unnamed_addr constant i16 30
    "#);
}

/// Test 61131-Standard style syntax: TYPE COLOR : DWORD (...) := default;
#[test]
fn enum_61131_standard_style_with_type_before_list() {
    let src = r#"
        TYPE COLOR : DWORD (
            white := 16#FFFFFF00,
            yellow := 16#FFFF0000,
            green := 16#FF00FF00,
            blue := 16#FF0000FF,
            black := 16#88000000
        ) := black;
        END_TYPE

        VAR_GLOBAL
            myColor : COLOR;
        END_VAR"#;

    // Should initialize to 16#88000000 (black)
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myColor = global i32 -2013265920
    @COLOR.white = unnamed_addr constant i32 -256
    @COLOR.yellow = unnamed_addr constant i32 -65536
    @COLOR.green = unnamed_addr constant i32 -16711936
    @COLOR.blue = unnamed_addr constant i32 -16776961
    @COLOR.black = unnamed_addr constant i32 -2013265920
    "#);
}

/// Test Codesys style syntax: TYPE COLOR : (...) DWORD := default;
#[test]
fn enum_codesys_style_with_type_after_list() {
    let src = r#"
        TYPE COLOR_CODESYS : (
            white := 16#FFFFFF00,
            yellow := 16#FFFF0000,
            green := 16#FF00FF00,
            blue := 16#FF0000FF,
            black := 16#88000000
        ) DWORD := black;
        END_TYPE

        VAR_GLOBAL
            myColor : COLOR_CODESYS;
        END_VAR"#;

    // Should initialize to 16#88000000 (black)
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myColor = global i32 -2013265920
    @COLOR_CODESYS.white = unnamed_addr constant i32 -256
    @COLOR_CODESYS.yellow = unnamed_addr constant i32 -65536
    @COLOR_CODESYS.green = unnamed_addr constant i32 -16711936
    @COLOR_CODESYS.blue = unnamed_addr constant i32 -16776961
    @COLOR_CODESYS.black = unnamed_addr constant i32 -2013265920
    "#);
}
