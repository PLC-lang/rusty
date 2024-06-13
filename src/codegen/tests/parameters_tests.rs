use crate::test_utils::tests::codegen;
use insta::assert_snapshot;

#[test]
fn function_all_parameters_assigned() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(var1, var2, var3);
            foo(input1 := var1, output1 => var2, inout1 := var3);
            foo(inout1 := var3, input1 := var1, output1 => var2);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_empty_input_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := , output1 => var2, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_empty_output_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, output1 => , inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_empty_output_default_value_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT := 3;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, output1 => , inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_empty_inout_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, output1 => var2, inout1 := );
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_input_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(output1 => var2, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_input_default_value_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT := 10;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(output1 => var2, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_output_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_output_default_value_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT := 3;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_inout_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, output1 => var2);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_default_value_parameter_type() {
    // GIVEN
    let result = codegen(
        "
        TYPE myType : DINT := 20; END_TYPE

        FUNCTION foo : DINT
        VAR_INPUT
            input1 : myType;
        END_VAR
        VAR_OUTPUT
            output1 : myType;
            output2 : myType;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(output2 => , inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_all_parameters_assigned_explicit() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(input1 := var1, output1 => var2, inout1 := var3);
            prog(inout1 := var3, input1 := var1, output1 => var2);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_all_parameters_assigned_implicit() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(var1, var2, var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_empty_inout_assignment() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(input1 := var1, output1 => var2, inout1 := );
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_missing_input_assignment() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(output1 => var2, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_missing_output_assignment() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(input1 := var1, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_accepts_empty_statement_as_input_param() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            in1: DINT;
            in2: DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
            prog(in1 := 1, in2 := );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_accepts_empty_statement_as_output_param() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_OUTPUT
            out1 : DINT;
            out2 : DINT;
        END_VAR
            out1 := 1;
            out2 := 2;
        END_PROGRAM

        PROGRAM main
        VAR
            x : DINT;
        END_VAR
            prog( out1 => x, out2 => );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn fb_accepts_empty_statement_as_input_param() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION_BLOCK fb_t
        VAR_INPUT
            in1: DINT;
            in2: DINT;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
        VAR
            fb : fb_t;
        END_VAR
            fb(in1 := 1, in2 := );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn fb_accepts_empty_statement_as_output_param() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION_BLOCK fb_t
        VAR_OUTPUT
            out1 : DINT;
            out2 : DINT;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
        VAR
            fb : fb_t;
            x : DINT;
        END_VAR
            fb( out1 => x, out2 => );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_accepts_empty_statement_as_input_param() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo
        VAR_INPUT
            in1: DINT;
            in2: DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
            foo(in1 := 1, in2 := );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_accepts_empty_statement_as_output_param() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo
        VAR_OUTPUT
            out1 : DINT;
            out2 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            x: DINT;
        END_VAR
            foo( out1 => x, out2 => );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn parameters_behind_function_block_pointer_are_assigned_to() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM main
        VAR
            file : file_t;
            FileOpen : REF_TO file_t;
        END_VAR
            FileOpen := REF(file);
            FileOpen^(var2:=TRUE);
        END_PROGRAM

        FUNCTION_BLOCK file_t
        VAR_INPUT
            var1 : BOOL;
            var2 : BOOL;
        END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn var_in_out_params_can_be_out_of_order() {
    let res = codegen(
        "PROGRAM mainProg
    VAR
        fb : fb_t;
        out1, out2 : BOOL;
    END_VAR
        fb(myOtherInOut := out1, myInOut := out2);
        fb(myInOut := out1, myOtherInOut := out2);

        fb.foo(myOtherInOut := out2, myInOut := out1);
        fb.foo(myInOut := out2, myOtherInOut := out1);
    END_PROGRAM

    FUNCTION_BLOCK fb_t
    VAR
        myVar   : BOOL;
    END_VAR
    VAR_INPUT
        myInput : USINT;
    END_VAR
    VAR_IN_OUT
        myInOut : BOOL;
    END_VAR
    VAR_OUTPUT
        myOut   : BOOL;
    END_VAR
    VAR_IN_OUT
        myOtherInOut : BOOL;
    END_VAR
    END_FUNCTION_BLOCK

    ACTIONS
        ACTION foo
            myInOut := myOtherInOut;
        END_ACTION
    END_ACTIONS",
    );

    insta::assert_snapshot!(res);
}

#[test]
fn by_value_function_arg_builtin_type_strings_are_memcopied() {
    let result = codegen(
        r#"
        FUNCTION main : DINT
        VAR
            str: STRING;
        END_VAR
            FOO(str);
        END_FUNCTION

        FUNCTION foo : DINT
            VAR_INPUT
                val : STRING;
            END_VAR
        END_FUNCTION
        "#,
    );

    assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define i32 @main() section "fn-main:i32" {
    entry:
      %main = alloca i32, align 4
      %str = alloca [81 x i8], align 1
      %0 = bitcast [81 x i8]* %str to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      %1 = bitcast [81 x i8]* %str to i8*
      %call = call i32 @foo(i8* %1)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    define i32 @foo(i8* %0) section "fn-foo:i32[s8u81]" {
    entry:
      %foo = alloca i32, align 4
      %val = alloca [81 x i8], align 1
      %bitcast = bitcast [81 x i8]* %val to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %bitcast, i8 0, i64 81, i1 false)
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %bitcast, i8* align 1 %0, i64 80, i1 false)
      store i32 0, i32* %foo, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    "###);
}

#[test]
fn by_value_function_arg_user_type_strings_are_memcopied() {
    let result = codegen(
        r#"
        FUNCTION main : DINT
        VAR
            str: STRING[65536];
        END_VAR
            FOO(str);
        END_FUNCTION

        FUNCTION foo : DINT
            VAR_INPUT
                val : STRING[65536];
            END_VAR
        END_FUNCTION
        "#,
    );

    assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define i32 @main() section "fn-main:i32" {
    entry:
      %main = alloca i32, align 4
      %str = alloca [65537 x i8], align 1
      %0 = bitcast [65537 x i8]* %str to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([65537 x i8]* getelementptr ([65537 x i8], [65537 x i8]* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      %1 = bitcast [65537 x i8]* %str to i8*
      %call = call i32 @foo(i8* %1)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    define i32 @foo(i8* %0) section "fn-foo:i32[s8u65537]" {
    entry:
      %foo = alloca i32, align 4
      %val = alloca [65537 x i8], align 1
      %bitcast = bitcast [65537 x i8]* %val to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %bitcast, i8 0, i64 65537, i1 false)
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %bitcast, i8* align 1 %0, i64 65536, i1 false)
      store i32 0, i32* %foo, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    "###);
}

#[test]
fn by_value_function_arg_arrays_are_memcopied() {
    let result = codegen(
        r#"
        FUNCTION main : DINT
        VAR
            arr: ARRAY[0..65536] OF DINT;
        END_VAR
            FOO(arr);
        END_FUNCTION

        FUNCTION foo : DINT
            VAR_INPUT
                val : ARRAY[0..65536] OF DINT;
            END_VAR
        END_FUNCTION
        "#,
    );

    assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define i32 @main() section "fn-main:i32" {
    entry:
      %main = alloca i32, align 4
      %arr = alloca [65537 x i32], align 4
      %0 = bitcast [65537 x i32]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([65537 x i32]* getelementptr ([65537 x i32], [65537 x i32]* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      %1 = bitcast [65537 x i32]* %arr to i32*
      %call = call i32 @foo(i32* %1)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    define i32 @foo(i32* %0) section "fn-foo:i32[ai32]" {
    entry:
      %foo = alloca i32, align 4
      %val = alloca [65537 x i32], align 4
      %bitcast = bitcast [65537 x i32]* %val to i32*
      %1 = bitcast i32* %bitcast to i8*
      %2 = bitcast i32* %0 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 %2, i64 ptrtoint ([65537 x i32]* getelementptr ([65537 x i32], [65537 x i32]* null, i32 1) to i64), i1 false)
      store i32 0, i32* %foo, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    "###);
}

#[test]
fn by_value_function_arg_structs_are_memcopied() {
    let result = codegen(
        r#"
        TYPE S_TY : STRUCT
            v1 : BOOL;
            v2 : BOOL;
            END_STRUCT
        END_TYPE
        
        FUNCTION foo : DINT
            VAR_INPUT
                val : S_TY;
            END_VAR
        END_FUNCTION

        FUNCTION main : DINT
        VAR
            s: S_TY;
        END_VAR
            FOO(s);
        END_FUNCTION
        "#,
    );

    assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %S_TY = type { i8, i8 }

    @__S_TY__init = unnamed_addr constant %S_TY zeroinitializer, section "var-__S_TY__init:r2u8u8"

    define i32 @foo(%S_TY* %0) section "fn-foo:i32[r2u8u8]" {
    entry:
      %foo = alloca i32, align 4
      %val = alloca %S_TY, align 8
      %1 = bitcast %S_TY* %val to i8*
      %2 = bitcast %S_TY* %0 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 %2, i64 ptrtoint (%S_TY* getelementptr (%S_TY, %S_TY* null, i32 1) to i64), i1 false)
      store i32 0, i32* %foo, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }

    define i32 @main() section "fn-main:i32" {
    entry:
      %main = alloca i32, align 4
      %s = alloca %S_TY, align 8
      %0 = bitcast %S_TY* %s to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 getelementptr inbounds (%S_TY, %S_TY* @__S_TY__init, i32 0, i32 0), i64 ptrtoint (%S_TY* getelementptr (%S_TY, %S_TY* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      %call = call i32 @foo(%S_TY* %s)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
}

#[test]
fn by_value_function_arg_structs_with_aggregate_members_are_memcopied() {
    let result = codegen(
        r#"
        TYPE S_TY : STRUCT
            v1 : BOOL;
            v2 : BOOL;
            END_STRUCT
        END_TYPE

        TYPE AGGREGATE_COLLECTOR_TY : STRUCT
            v1 : ARRAY[0..65536] OF DINT;
            v2 : STRING[65536];
            v3 : S_TY;
            END_STRUCT
        END_TYPE
        
        FUNCTION foo : DINT
            VAR_INPUT
                val : AGGREGATE_COLLECTOR_TY;
            END_VAR
        END_FUNCTION

        FUNCTION main : DINT
        VAR
            s: AGGREGATE_COLLECTOR_TY;
        END_VAR
            FOO(s);
        END_FUNCTION
        "#,
    );

    assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %AGGREGATE_COLLECTOR_TY = type { [65537 x i32], [65537 x i8], %S_TY }
    %S_TY = type { i8, i8 }

    @__AGGREGATE_COLLECTOR_TY__init = unnamed_addr constant %AGGREGATE_COLLECTOR_TY zeroinitializer, section "var-__AGGREGATE_COLLECTOR_TY__init:r3ai32s8u65537r2u8u8"
    @__S_TY__init = unnamed_addr constant %S_TY zeroinitializer, section "var-__S_TY__init:r2u8u8"

    define i32 @foo(%AGGREGATE_COLLECTOR_TY* %0) section "fn-foo:i32[r3ai32s8u65537r2u8u8]" {
    entry:
      %foo = alloca i32, align 4
      %val = alloca %AGGREGATE_COLLECTOR_TY, align 8
      %1 = bitcast %AGGREGATE_COLLECTOR_TY* %val to i8*
      %2 = bitcast %AGGREGATE_COLLECTOR_TY* %0 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 %2, i64 ptrtoint (%AGGREGATE_COLLECTOR_TY* getelementptr (%AGGREGATE_COLLECTOR_TY, %AGGREGATE_COLLECTOR_TY* null, i32 1) to i64), i1 false)
      store i32 0, i32* %foo, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }

    define i32 @main() section "fn-main:i32" {
    entry:
      %main = alloca i32, align 4
      %s = alloca %AGGREGATE_COLLECTOR_TY, align 8
      %0 = bitcast %AGGREGATE_COLLECTOR_TY* %s to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%AGGREGATE_COLLECTOR_TY* @__AGGREGATE_COLLECTOR_TY__init to i8*), i64 ptrtoint (%AGGREGATE_COLLECTOR_TY* getelementptr (%AGGREGATE_COLLECTOR_TY, %AGGREGATE_COLLECTOR_TY* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      %call = call i32 @foo(%AGGREGATE_COLLECTOR_TY* %s)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
}

#[test]
fn by_value_fb_arg_aggregates_are_memcopied() {
    let result = codegen(
        r#"
        FUNCTION main : DINT
        VAR
            str: STRING[65536];
            arr: ARRAY[1..1024] OF DINT;
            fb: FOO;
        END_VAR
            fb(str, arr);
        END_FUNCTION

        FUNCTION_BLOCK FOO
        VAR_INPUT
            val : STRING[65536];
            field : ARRAY[1..1024] OF DINT;
        END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %FOO = type { [65537 x i8], [1024 x i32] }

    @__FOO__init = unnamed_addr constant %FOO zeroinitializer, section "var-__FOO__init:r2s8u65537ai32"

    define i32 @main() section "fn-main:i32" {
    entry:
      %main = alloca i32, align 4
      %str = alloca [65537 x i8], align 1
      %arr = alloca [1024 x i32], align 4
      %fb = alloca %FOO, align 8
      %0 = bitcast [65537 x i8]* %str to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([65537 x i8]* getelementptr ([65537 x i8], [65537 x i8]* null, i32 1) to i64), i1 false)
      %1 = bitcast [1024 x i32]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([1024 x i32]* getelementptr ([1024 x i32], [1024 x i32]* null, i32 1) to i64), i1 false)
      %2 = bitcast %FOO* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %2, i8* align 1 getelementptr inbounds (%FOO, %FOO* @__FOO__init, i32 0, i32 0, i32 0), i64 ptrtoint (%FOO* getelementptr (%FOO, %FOO* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      %3 = getelementptr inbounds %FOO, %FOO* %fb, i32 0, i32 0
      %4 = bitcast [65537 x i8]* %3 to i8*
      %5 = bitcast [65537 x i8]* %str to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %4, i8* align 1 %5, i32 65536, i1 false)
      %6 = getelementptr inbounds %FOO, %FOO* %fb, i32 0, i32 1
      %7 = bitcast [1024 x i32]* %6 to i8*
      %8 = bitcast [1024 x i32]* %arr to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %7, i8* align 1 %8, i64 ptrtoint ([1024 x i32]* getelementptr ([1024 x i32], [1024 x i32]* null, i32 1) to i64), i1 false)
      call void @FOO(%FOO* %fb)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    define void @FOO(%FOO* %0) section "fn-FOO:v[s8u65537][ai32]" {
    entry:
      %val = getelementptr inbounds %FOO, %FOO* %0, i32 0, i32 0
      %field = getelementptr inbounds %FOO, %FOO* %0, i32 0, i32 1
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #1

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    "###);
}

#[test]
fn var_output_aggregate_types_are_memcopied() {
    let result = codegen(
        r#"
        TYPE OUT_TYPE : STRUCT 
            a : BYTE;
        END_STRUCT;
        END_TYPE

        FUNCTION_BLOCK FB
        VAR_OUTPUT 
            output : OUT_TYPE;
            output2 : ARRAY[0..10] OF DINT;
            output3 : ARRAY[0..10] OF OUT_TYPE;
            output4 : STRING;
            output5 : WSTRING;
        END_VAR    
        END_FUNCTION_BLOCK

        PROGRAM PRG
        VAR
            out: OUT_TYPE;
            out2 : ARRAY[0..10] OF DINT;
            out3 : ARRAY[0..10] OF OUT_TYPE;
            out4 : STRING;
            out5 : WSTRING;
            station: FB;
        END_VAR
            station(output => out, output2 => out2, output3 => out3, output4 => out4, output5 => out5);
        END_PROGRAM
        "#,
    );

    assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %FB = type { %OUT_TYPE, [11 x i32], [11 x %OUT_TYPE], [81 x i8], [81 x i16] }
    %OUT_TYPE = type { i8 }
    %PRG = type { %OUT_TYPE, [11 x i32], [11 x %OUT_TYPE], [81 x i8], [81 x i16], %FB }

    @__FB__init = unnamed_addr constant %FB zeroinitializer, section "var-__FB__init:r5r1u8ai32ar1u8s8u81s16u81"
    @__OUT_TYPE__init = unnamed_addr constant %OUT_TYPE zeroinitializer, section "var-__OUT_TYPE__init:r1u8"
    @PRG_instance = global %PRG zeroinitializer, section "var-PRG_instance:r6r1u8ai32ar1u8s8u81s16u81r5r1u8ai32ar1u8s8u81s16u81"

    define void @FB(%FB* %0) section "fn-FB:v[r1u8][ai32][ar1u8][s8u81][s16u81]" {
    entry:
      %output = getelementptr inbounds %FB, %FB* %0, i32 0, i32 0
      %output2 = getelementptr inbounds %FB, %FB* %0, i32 0, i32 1
      %output3 = getelementptr inbounds %FB, %FB* %0, i32 0, i32 2
      %output4 = getelementptr inbounds %FB, %FB* %0, i32 0, i32 3
      %output5 = getelementptr inbounds %FB, %FB* %0, i32 0, i32 4
      ret void
    }

    define void @PRG(%PRG* %0) section "fn-PRG:v" {
    entry:
      %out = getelementptr inbounds %PRG, %PRG* %0, i32 0, i32 0
      %out2 = getelementptr inbounds %PRG, %PRG* %0, i32 0, i32 1
      %out3 = getelementptr inbounds %PRG, %PRG* %0, i32 0, i32 2
      %out4 = getelementptr inbounds %PRG, %PRG* %0, i32 0, i32 3
      %out5 = getelementptr inbounds %PRG, %PRG* %0, i32 0, i32 4
      %station = getelementptr inbounds %PRG, %PRG* %0, i32 0, i32 5
      call void @FB(%FB* %station)
      %1 = getelementptr inbounds %FB, %FB* %station, i32 0, i32 0
      %2 = bitcast %OUT_TYPE* %out to i8*
      %3 = bitcast %OUT_TYPE* %1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %2, i8* align 1 %3, i64 ptrtoint (%OUT_TYPE* getelementptr (%OUT_TYPE, %OUT_TYPE* null, i32 1) to i64), i1 false)
      %4 = getelementptr inbounds %FB, %FB* %station, i32 0, i32 1
      %5 = bitcast [11 x i32]* %out2 to i8*
      %6 = bitcast [11 x i32]* %4 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %5, i8* align 1 %6, i64 ptrtoint ([11 x i32]* getelementptr ([11 x i32], [11 x i32]* null, i32 1) to i64), i1 false)
      %7 = getelementptr inbounds %FB, %FB* %station, i32 0, i32 2
      %8 = bitcast [11 x %OUT_TYPE]* %out3 to i8*
      %9 = bitcast [11 x %OUT_TYPE]* %7 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %8, i8* align 1 %9, i64 ptrtoint ([11 x %OUT_TYPE]* getelementptr ([11 x %OUT_TYPE], [11 x %OUT_TYPE]* null, i32 1) to i64), i1 false)
      %10 = getelementptr inbounds %FB, %FB* %station, i32 0, i32 3
      %11 = bitcast [81 x i8]* %out4 to i8*
      %12 = bitcast [81 x i8]* %10 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %11, i8* align 1 %12, i32 80, i1 false)
      %13 = getelementptr inbounds %FB, %FB* %station, i32 0, i32 4
      %14 = bitcast [81 x i16]* %out5 to i8*
      %15 = bitcast [81 x i16]* %13 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 2 %14, i8* align 2 %15, i32 160, i1 false)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
}
