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
    assert_snapshot!(src, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @__custom_got = weak_odr global [4 x i8*] zeroinitializer

    define i32 @foo() section "$RUSTY$fn-foo:i32[]" {
    entry:
      %foo = alloca i32, align 4
      %x = alloca i32, align 4
      store i32 0, i32* %x, align 4
      store i32 0, i32* %foo, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @__custom_got = weak_odr global [4 x i8*] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() section "$RUSTY$fn-__init___testproject:v[]" {
    entry:
      ret void
    }
    "###)
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

    %prg = type { i32 }

    @prg_instance = global %prg zeroinitializer, section "$RUSTY$var-prg_instance:r1i32"
    @__custom_got = weak_odr global [12 x i8*] zeroinitializer

    define void @prg(%prg* %0) section "$RUSTY$fn-prg:v[]" {
    entry:
      %x = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %prg = type { i32 }

    @prg_instance = external global %prg, section "$RUSTY$var-prg_instance:r1i32"
    @__custom_got = weak_odr global [12 x i8*] zeroinitializer

    define void @__init_prg(%prg* %0) section "$RUSTY$fn-__init_prg:v[pr1i32]" {
    entry:
      %self = alloca %prg*, align 8
      store %prg* %0, %prg** %self, align 8
      ret void
    }

    declare void @prg(%prg*) section "$RUSTY$fn-prg:v[]"

    define void @__user_init_prg(%prg* %0) section "$RUSTY$fn-__user_init_prg:v[pr1i32]" {
    entry:
      %self = alloca %prg*, align 8
      store %prg* %0, %prg** %self, align 8
      ret void
    }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prg = type { i32 }

    @prg_instance = external global %prg, section "$RUSTY$var-prg_instance:r1i32"
    @__custom_got = weak_odr global [12 x i8*] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() section "$RUSTY$fn-__init___testproject:v[]" {
    entry:
      %0 = load void (%prg*)*, void (%prg*)** getelementptr inbounds (void (%prg*)*, void (%prg*)** inttoptr (i64 -2401053092612145152 to void (%prg*)**), i32 7), align 8
      call void %0(%prg* @prg_instance)
      %1 = load void (%prg*)*, void (%prg*)** getelementptr inbounds (void (%prg*)*, void (%prg*)** inttoptr (i64 -2401053092612145152 to void (%prg*)**), i32 9), align 8
      call void %1(%prg* @prg_instance)
      ret void
    }

    declare void @__init_prg(%prg*) section "$RUSTY$fn-__init_prg:v[pr1i32]"

    declare void @prg(%prg*) section "$RUSTY$fn-prg:v[]"

    declare void @__user_init_prg(%prg*) section "$RUSTY$fn-__user_init_prg:v[pr1i32]"
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

    %prg = type { i32 }

    @gV = global i32 0, section "$RUSTY$var-gv:i32"
    @prg_instance = global %prg zeroinitializer, section "$RUSTY$var-prg_instance:r1i32"
    @__custom_got = weak_odr global [14 x i8*] zeroinitializer

    define void @prg(%prg* %0) section "$RUSTY$fn-prg:v[]" {
    entry:
      %x = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %1 = load i32*, i32** getelementptr inbounds (i32*, i32** inttoptr (i64 -2401053092612145152 to i32**), i32 1), align 8
      %load_x = load i32, i32* %x, align 4
      store i32 %load_x, i32* %1, align 4
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %prg = type { i32 }

    @prg_instance = external global %prg, section "$RUSTY$var-prg_instance:r1i32"
    @__custom_got = weak_odr global [14 x i8*] zeroinitializer

    define void @__init_prg(%prg* %0) section "$RUSTY$fn-__init_prg:v[pr1i32]" {
    entry:
      %self = alloca %prg*, align 8
      store %prg* %0, %prg** %self, align 8
      ret void
    }

    declare void @prg(%prg*) section "$RUSTY$fn-prg:v[]"

    define void @__user_init_prg(%prg* %0) section "$RUSTY$fn-__user_init_prg:v[pr1i32]" {
    entry:
      %self = alloca %prg*, align 8
      store %prg* %0, %prg** %self, align 8
      ret void
    }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prg = type { i32 }

    @prg_instance = external global %prg, section "$RUSTY$var-prg_instance:r1i32"
    @__custom_got = weak_odr global [14 x i8*] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() section "$RUSTY$fn-__init___testproject:v[]" {
    entry:
      %0 = load void (%prg*)*, void (%prg*)** getelementptr inbounds (void (%prg*)*, void (%prg*)** inttoptr (i64 -2401053092612145152 to void (%prg*)**), i32 9), align 8
      call void %0(%prg* @prg_instance)
      %1 = load void (%prg*)*, void (%prg*)** getelementptr inbounds (void (%prg*)*, void (%prg*)** inttoptr (i64 -2401053092612145152 to void (%prg*)**), i32 11), align 8
      call void %1(%prg* @prg_instance)
      ret void
    }

    declare void @__init_prg(%prg*) section "$RUSTY$fn-__init_prg:v[pr1i32]"

    declare void @prg(%prg*) section "$RUSTY$fn-prg:v[]"

    declare void @__user_init_prg(%prg*) section "$RUSTY$fn-__user_init_prg:v[pr1i32]"
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
    assert_snapshot!(src, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @gV = global i32 0, section "$RUSTY$var-gv:i32"
    @__custom_got = weak_odr global [6 x i8*] zeroinitializer

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
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @__custom_got = weak_odr global [6 x i8*] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() section "$RUSTY$fn-__init___testproject:v[]" {
    entry:
      ret void
    }
    "###)
}
