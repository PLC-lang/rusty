use insta::assert_snapshot;

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

    assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @foo = global i8* null, section "var-$RUSTY$foo:pu8"
    @__PI_1_2_3_4 = global i8 0, section "var-$RUSTY$__PI_1_2_3_4:u8"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @__PI_1_2_3_4 = external global i8, section "var-$RUSTY$__PI_1_2_3_4:u8"
    @foo = external global i8*, section "var-$RUSTY$foo:pu8"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      store i8* @__PI_1_2_3_4, i8** @foo, align 8
      ret void
    }
    "###);
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

    assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @foo = global i8* null, section "var-$RUSTY$foo:pu8"
    @__PI_1_2_3_4 = global i8 0, section "var-$RUSTY$__PI_1_2_3_4:u8"
    @baz = global i8* null, section "var-$RUSTY$baz:pu8"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @__PI_1_2_3_4 = external global i8, section "var-$RUSTY$__PI_1_2_3_4:u8"
    @foo = external global i8*, section "var-$RUSTY$foo:pu8"
    @baz = external global i8*, section "var-$RUSTY$baz:pu8"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      store i8* @__PI_1_2_3_4, i8** @foo, align 8
      store i8* @__PI_1_2_3_4, i8** @baz, align 8
      ret void
    }
    "###);
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

    assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %mainProg = type {}

    @foo = global i8* null, section "var-$RUSTY$foo:pu8"
    @__PI_1_2_3_4 = global i8 0, section "var-$RUSTY$__PI_1_2_3_4:u8"
    @baz = global i8* null, section "var-$RUSTY$baz:pu8"
    @mainProg_instance = global %mainProg zeroinitializer, section "var-$RUSTY$mainProg_instance:r0"

    define void @mainProg(%mainProg* %0) section "fn-$RUSTY$mainProg:v" {
    entry:
      %deref = load i8*, i8** @foo, align 8
      store i8 0, i8* %deref, align 1
      %deref1 = load i8*, i8** @baz, align 8
      store i8 1, i8* %deref1, align 1
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %mainProg = type {}

    @mainProg_instance = external global %mainProg, section "var-$RUSTY$mainProg_instance:r0"

    define void @__init_mainprog(%mainProg* %0) section "fn-$RUSTY$__init_mainprog:v[pr0]" {
    entry:
      %self = alloca %mainProg*, align 8
      store %mainProg* %0, %mainProg** %self, align 8
      ret void
    }

    declare void @mainProg(%mainProg*) section "fn-$RUSTY$mainProg:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %mainProg = type {}

    @__PI_1_2_3_4 = external global i8, section "var-$RUSTY$__PI_1_2_3_4:u8"
    @foo = external global i8*, section "var-$RUSTY$foo:pu8"
    @baz = external global i8*, section "var-$RUSTY$baz:pu8"
    @mainProg_instance = external global %mainProg, section "var-$RUSTY$mainProg_instance:r0"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      store i8* @__PI_1_2_3_4, i8** @foo, align 8
      store i8* @__PI_1_2_3_4, i8** @baz, align 8
      call void @__init_mainprog(%mainProg* @mainProg_instance)
      ret void
    }

    declare void @__init_mainprog(%mainProg*) section "fn-$RUSTY$__init_mainprog:v[pr0]"

    declare void @mainProg(%mainProg*) section "fn-$RUSTY$mainProg:v"
    "###);
}
