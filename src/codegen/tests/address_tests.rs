use plc_util::filtered_assert_snapshot;

use crate::test_utils::tests::codegen;

#[test]
fn aliased_address_in_global_generated() {
    let res = codegen(
        r"
            VAR_GLOBAL
                foo AT %IX1.2.3.4 : BOOL;
            END_VAR
        ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @foo = global ptr null
    @__PI_1_2_3_4 = global i8 0
    "#);
}

#[test]
fn duplicate_aliased_address_in_global_generated() {
    let res = codegen(
        r"
            VAR_GLOBAL
                foo AT %IX1.2.3.4 : BOOL;
                baz AT %IX1.2.3.4 : BOOL;
            END_VAR
        ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @foo = global ptr null
    @__PI_1_2_3_4 = global i8 0
    @baz = global ptr null
    "#);
}

#[test]
fn address_variable_used_with_symbolic_name() {
    let res = codegen(
        r"
            VAR_GLOBAL
                foo AT %IX1.2.3.4 : BOOL;
                baz AT %IX1.2.3.4 : BOOL;
            END_VAR

            PROGRAM mainProg
                foo := FALSE;
                baz := TRUE;
            END_PROGRAM
        ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type {}

    @foo = global i8* null
    @__PI_1_2_3_4 = global i8 0
    @baz = global i8* null
    @mainProg_instance = global %mainProg zeroinitializer

    define void @mainProg(%mainProg* %0) {
    entry:
      %deref = load i8*, i8** @foo, align 8
      store i8 0, i8* %deref, align 1
      %deref1 = load i8*, i8** @baz, align 8
      store i8 1, i8* %deref1, align 1
      ret void
    }
    "#);
}

#[test]
fn address_used_in_body() {
    let res = codegen(
        r"
            VAR_GLOBAL
                foo AT %IX1.2.3.4 : BOOL;
                baz AT %QX1.2.3.5 : BOOL;
                x : BOOL := TRUE;
            END_VAR

            PROGRAM mainProg
                %IX1.2.3.4 := TRUE;
                x := %QX1.2.3.5;                 
            END_PROGRAM
        ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type {}

    @foo = global i8* null
    @__PI_1_2_3_4 = global i8 0
    @baz = global i8* null
    @__PI_1_2_3_5 = global i8 0
    @x = global i8 1
    @mainProg_instance = global %mainProg zeroinitializer

    define void @mainProg(%mainProg* %0) {
    entry:
      store i8 1, i8* @__PI_1_2_3_4, align 1
      %1 = load i8, i8* @__PI_1_2_3_5, align 1
      store i8 %1, i8* @x, align 1
      ret void
    }
    "#);
}
