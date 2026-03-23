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

    @foo = global ptr null
    @__PI_1_2_3_4 = global i8 0
    @baz = global ptr null
    @mainProg_instance = global %mainProg zeroinitializer

    define void @mainProg(ptr %0) {
    entry:
      %deref = load ptr, ptr @foo, align [filtered]
      store i8 0, ptr %deref, align [filtered]
      %deref1 = load ptr, ptr @baz, align [filtered]
      store i8 1, ptr %deref1, align [filtered]
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

    @foo = global ptr null
    @__PI_1_2_3_4 = global i8 0
    @baz = global ptr null
    @__PI_1_2_3_5 = global i8 0
    @x = global i8 1
    @mainProg_instance = global %mainProg zeroinitializer

    define void @mainProg(ptr %0) {
    entry:
      store i8 1, ptr @__PI_1_2_3_4, align [filtered]
      %1 = load i8, ptr @__PI_1_2_3_5, align [filtered]
      store i8 %1, ptr @x, align [filtered]
      ret void
    }
    "#);
}

#[test]
fn struct_member_with_hardware_address() {
    let res = codegen(
        r"
            TYPE NodeB : STRUCT
                c AT %IX1.2.3.4 : BOOL;
            END_STRUCT
            END_TYPE

            TYPE NodeA : STRUCT
                b : NodeB;
            END_STRUCT
            END_TYPE

            VAR_GLOBAL
                myNode : NodeA;
            END_VAR
        ",
    );

    // The backing global __PI_1_2_3_4 is created for the hardware address.
    // NodeB.c becomes a pointer initialized to &__PI_1_2_3_4 in NodeB__ctor (separate module).
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %NodeA = type { %NodeB }
    %NodeB = type { ptr }

    @myNode = global %NodeA zeroinitializer
    @__PI_1_2_3_4 = global i8 0
    "#);
}
