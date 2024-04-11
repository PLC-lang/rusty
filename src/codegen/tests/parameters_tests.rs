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
            FileOpen := &file;
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
fn by_value_function_arg_strings_are_memcopied() {
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

    define i32 @foo(i8* %0) section "fn-foo:i32[ps8u65537]" {
    entry:
      %foo = alloca i32, align 4
      %val = alloca [65537 x i8], align 1
      %bitcast = bitcast [65537 x i8]* %val to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %bitcast, i8* align 1 %0, i64 ptrtoint ([65537 x i8]* getelementptr ([65537 x i8], [65537 x i8]* null, i32 1) to i64), i1 false)
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

    define i32 @foo(i32* %0) section "fn-foo:i32[pv]" {
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
        "#,
    );

    assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define i32 @bar(i8* %0) {
    entry:
      %bar = alloca i32, align 4
      %val = alloca [65537 x i8], align 1
      store i8* %0, [65537 x i8]* %val, align 8
      store i32 0, i32* %bar, align 4
      %bar_ret = load i32, i32* %bar, align 4
      ret i32 %bar_ret
    }
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
                val : S_TY;
            END_VAR
        END_FUNCTION
        "#,
    );

    assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define i32 @bar(i8* %0) {
    entry:
      %bar = alloca i32, align 4
      %val = alloca [65537 x i8], align 1
      store i8* %0, [65537 x i8]* %val, align 8
      store i32 0, i32* %bar, align 4
      %bar_ret = load i32, i32* %bar, align 4
      ret i32 %bar_ret
    }
    "###);
}
