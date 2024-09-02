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
