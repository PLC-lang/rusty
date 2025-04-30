use insta::assert_snapshot;

use crate::test_utils::tests::codegen_with_online_change as codegen;

#[test]
#[cfg_attr(target_os = "macos", ignore)]
fn generate_function_with_online_change() {
    let src = codegen(
        "
        FUNCTION foo : DINT
           VAR
            x : DINT;
           END_VAR
        END_FUNCTION
        ",
    );
    assert_snapshot!(src, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    @__custom_got = weak_odr global [2 x i8*] zeroinitializer

    define i32 @foo() section "$RUSTY$fn-foo:i32[]" {
    entry:
      %foo = alloca i32, align 4
      %x = alloca i32, align 4
      store i32 0, i32* %x, align 4
      store i32 0, i32* %foo, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }
    "#)
}

#[test]
#[cfg_attr(target_os = "macos", ignore)]
fn generate_program_with_online_change() {
    let src = codegen(
        "
        PROGRAM prg
           VAR
            x : DINT;
           END_VAR
        END_PROGRAM
        ",
    );
    assert_snapshot!(src, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %prg = type { i32 }

    @prg_instance = global %prg zeroinitializer, section "$RUSTY$var-prg_instance:r1i32"
    @__custom_got = weak_odr global [6 x i8*] zeroinitializer

    define void @prg(%prg* %0) section "$RUSTY$fn-prg:v[]" {
    entry:
      %x = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      ret void
    }
    "#)
}

#[test]
#[cfg_attr(target_os = "macos", ignore)]
fn generate_program_and_var_with_online_change() {
    let src = codegen(
        "
        PROGRAM prg
           VAR
            x : DINT;
           END_VAR
           gV := x;
        END_PROGRAM
        VAR_GLOBAL
            gV : DINT;
        END_VAR
        ",
    );
    assert_snapshot!(src, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %prg = type { i32 }

    @gV = global i32 0, section "$RUSTY$var-gv:i32"
    @prg_instance = global %prg zeroinitializer, section "$RUSTY$var-prg_instance:r1i32"
    @__custom_got = weak_odr global [8 x i8*] zeroinitializer

    define void @prg(%prg* %0) section "$RUSTY$fn-prg:v[]" {
    entry:
      %x = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %1 = load i32*, i32** getelementptr inbounds (i32*, i32** inttoptr (i64 -2401053092612145152 to i32**), i32 1), align 8
      %load_x = load i32, i32* %x, align 4
      store i32 %load_x, i32* %1, align 4
      ret void
    }
    "#)
}

#[test]
#[cfg_attr(target_os = "macos", ignore)]
fn generate_function_and_var_with_online_change() {
    let src = codegen(
        "
        FUNCTION foo : DINT
           VAR
            x : DINT;
           END_VAR
           gV := x;
        END_FUNCTION
        VAR_GLOBAL
            gV : DINT;
        END_VAR
        ",
    );
    assert_snapshot!(src, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    @gV = global i32 0, section "$RUSTY$var-gv:i32"
    @__custom_got = weak_odr global [4 x i8*] zeroinitializer

    define i32 @foo() section "$RUSTY$fn-foo:i32[]" {
    entry:
      %foo = alloca i32, align 4
      %x = alloca i32, align 4
      store i32 0, i32* %x, align 4
      store i32 0, i32* %foo, align 4
      %0 = load i32*, i32** getelementptr inbounds (i32*, i32** inttoptr (i64 -2401053092612145152 to i32**), i32 1), align 8
      %load_x = load i32, i32* %x, align 4
      store i32 %load_x, i32* %0, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }
    "#)
}
