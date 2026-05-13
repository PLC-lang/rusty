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

#[test]
fn aliased_address_on_function_block_member_generates_backing_global() {
    // Issue #1736: A complete hardware address (AT %IX...) on a POU-scoped variable used to be
    // silently dropped — the FB field stayed null and the first access SIGSEGV'd at runtime.
    // The preprocessor now mirrors the VAR_GLOBAL / STRUCT-member path: it synthesises the
    // backing __PI_1_2_3_4 global and sets the member's initializer so the FB instance ctor
    // (emitted in the lowering pass) binds the field with a `store ptr @__PI_..., ptr %in1`.
    // This snapshot covers the codegen-visible artifacts (alias-pointer field and backing
    // global); the lit test `var_global_and_fb_share_hw_address.st` covers the runtime binding.
    let res = codegen(
        r"
            FUNCTION_BLOCK probe
            VAR_INPUT
                in1 AT %IX1.2.3.4 : BOOL;
            END_VAR
            END_FUNCTION_BLOCK

            PROGRAM mainProg
            VAR p : probe; END_VAR
                p();
            END_PROGRAM
        ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { %probe }
    %probe = type { ptr }

    @__PI_1_2_3_4 = global i8 0
    @mainProg_instance = global %mainProg zeroinitializer

    define void @probe(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %in1 = getelementptr inbounds nuw %probe, ptr %0, i32 0, i32 0
      ret void
    }

    define void @mainProg(ptr %0) {
    entry:
      %p = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 0
      call void @probe(ptr %p)
      ret void
    }
    "#);
}

#[test]
fn aliased_address_shared_across_var_global_and_function_block() {
    // Issue #1736: a VAR_GLOBAL and an FB VAR_INPUT at the same address must share one backing
    // global. The known_hw_globals dedup in the preprocessor ensures `__PI_1_2_3_4` is emitted
    // exactly once even though both declarations would otherwise synthesise it.
    let res = codegen(
        r"
            VAR_GLOBAL
                slot AT %IX1.2.3.4 : BOOL;
            END_VAR

            FUNCTION_BLOCK probe
            VAR_INPUT
                shadow AT %IX1.2.3.4 : BOOL;
            END_VAR
            END_FUNCTION_BLOCK
        ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %probe = type { ptr }

    @slot = global ptr null
    @__PI_1_2_3_4 = global i8 0

    define void @probe(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %shadow = getelementptr inbounds nuw %probe, ptr %0, i32 0, i32 0
      ret void
    }
    "#);
}
