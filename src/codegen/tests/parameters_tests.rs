use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %str = alloca [81 x i8], align 1
      call void @llvm.memset.p0.i64(ptr align 1 %str, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      %call = call i32 @foo(ptr %str)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    define i32 @foo(ptr %0) {
    entry:
      %foo = alloca i32, align 4
      %val = alloca [81 x i8], align 1
      call void @llvm.memset.p0.i64(ptr align 1 %val, i8 0, i64 81, i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %val, ptr align 1 %0, i64 80, i1 false)
      store i32 0, ptr %foo, align 4
      %foo_ret = load i32, ptr %foo, align 4
      ret i32 %foo_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %str = alloca [65537 x i8], align 1
      call void @llvm.memset.p0.i64(ptr align 1 %str, i8 0, i64 ptrtoint (ptr getelementptr ([65537 x i8], ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      %call = call i32 @foo(ptr %str)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    define i32 @foo(ptr %0) {
    entry:
      %foo = alloca i32, align 4
      %val = alloca [65537 x i8], align 1
      call void @llvm.memset.p0.i64(ptr align 1 %val, i8 0, i64 65537, i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %val, ptr align 1 %0, i64 65536, i1 false)
      store i32 0, ptr %foo, align 4
      %foo_ret = load i32, ptr %foo, align 4
      ret i32 %foo_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %arr = alloca [65537 x i32], align 4
      call void @llvm.memset.p0.i64(ptr align 1 %arr, i8 0, i64 ptrtoint (ptr getelementptr ([65537 x i32], ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      %call = call i32 @foo(ptr %arr)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    define i32 @foo(ptr %0) {
    entry:
      %foo = alloca i32, align 4
      %val = alloca [65537 x i32], align 4
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %val, ptr align 1 %0, i64 ptrtoint (ptr getelementptr ([65537 x i32], ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %foo, align 4
      %foo_ret = load i32, ptr %foo, align 4
      ret i32 %foo_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %S_TY = type { i8, i8 }

    @__S_TY__init = unnamed_addr constant %S_TY zeroinitializer

    define i32 @foo(ptr %0) {
    entry:
      %foo = alloca i32, align 4
      %val = alloca %S_TY, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %val, ptr align 1 %0, i64 ptrtoint (ptr getelementptr (%S_TY, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %foo, align 4
      %foo_ret = load i32, ptr %foo, align 4
      ret i32 %foo_ret
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %s = alloca %S_TY, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %s, ptr align 1 @__S_TY__init, i64 ptrtoint (ptr getelementptr (%S_TY, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      %call = call i32 @foo(ptr %s)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %AGGREGATE_COLLECTOR_TY = type { [65537 x i32], [65537 x i8], %S_TY }
    %S_TY = type { i8, i8 }

    @__AGGREGATE_COLLECTOR_TY__init = unnamed_addr constant %AGGREGATE_COLLECTOR_TY zeroinitializer
    @__S_TY__init = unnamed_addr constant %S_TY zeroinitializer

    define i32 @foo(ptr %0) {
    entry:
      %foo = alloca i32, align 4
      %val = alloca %AGGREGATE_COLLECTOR_TY, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %val, ptr align 1 %0, i64 ptrtoint (ptr getelementptr (%AGGREGATE_COLLECTOR_TY, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %foo, align 4
      %foo_ret = load i32, ptr %foo, align 4
      ret i32 %foo_ret
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %s = alloca %AGGREGATE_COLLECTOR_TY, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %s, ptr align 1 @__AGGREGATE_COLLECTOR_TY__init, i64 ptrtoint (ptr getelementptr (%AGGREGATE_COLLECTOR_TY, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      %call = call i32 @foo(ptr %s)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FOO = type { [65537 x i8], [1024 x i32] }

    @__FOO__init = unnamed_addr constant %FOO zeroinitializer

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %str = alloca [65537 x i8], align 1
      %arr = alloca [1024 x i32], align 4
      %fb = alloca %FOO, align 8
      call void @llvm.memset.p0.i64(ptr align 1 %str, i8 0, i64 ptrtoint (ptr getelementptr ([65537 x i8], ptr null, i32 1) to i64), i1 false)
      call void @llvm.memset.p0.i64(ptr align 1 %arr, i8 0, i64 ptrtoint (ptr getelementptr ([1024 x i32], ptr null, i32 1) to i64), i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %fb, ptr align 1 @__FOO__init, i64 ptrtoint (ptr getelementptr (%FOO, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      %0 = getelementptr inbounds %FOO, ptr %fb, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %0, ptr align 1 %str, i32 65536, i1 false)
      %1 = getelementptr inbounds %FOO, ptr %fb, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %1, ptr align 1 %arr, i64 ptrtoint (ptr getelementptr ([1024 x i32], ptr null, i32 1) to i64), i1 false)
      call void @FOO(ptr %fb)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    define void @FOO(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %val = getelementptr inbounds nuw %FOO, ptr %0, i32 0, i32 0
      %field = getelementptr inbounds nuw %FOO, ptr %0, i32 0, i32 1
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FB = type { %OUT_TYPE, [11 x i32], [11 x %OUT_TYPE], [81 x i8], [81 x i16] }
    %OUT_TYPE = type { i8 }
    %PRG = type { %OUT_TYPE, [11 x i32], [11 x %OUT_TYPE], [81 x i8], [81 x i16], %FB }

    @__FB__init = unnamed_addr constant %FB zeroinitializer
    @__OUT_TYPE__init = unnamed_addr constant %OUT_TYPE zeroinitializer
    @PRG_instance = global %PRG zeroinitializer

    define void @FB(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %output = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 0
      %output2 = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 1
      %output3 = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 2
      %output4 = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 3
      %output5 = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 4
      ret void
    }

    define void @PRG(ptr %0) {
    entry:
      %out = getelementptr inbounds nuw %PRG, ptr %0, i32 0, i32 0
      %out2 = getelementptr inbounds nuw %PRG, ptr %0, i32 0, i32 1
      %out3 = getelementptr inbounds nuw %PRG, ptr %0, i32 0, i32 2
      %out4 = getelementptr inbounds nuw %PRG, ptr %0, i32 0, i32 3
      %out5 = getelementptr inbounds nuw %PRG, ptr %0, i32 0, i32 4
      %station = getelementptr inbounds nuw %PRG, ptr %0, i32 0, i32 5
      call void @FB(ptr %station)
      %1 = getelementptr inbounds %FB, ptr %station, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %out, ptr align 1 %1, i64 ptrtoint (ptr getelementptr (%OUT_TYPE, ptr null, i32 1) to i64), i1 false)
      %2 = getelementptr inbounds %FB, ptr %station, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %out2, ptr align 1 %2, i64 ptrtoint (ptr getelementptr ([11 x i32], ptr null, i32 1) to i64), i1 false)
      %3 = getelementptr inbounds %FB, ptr %station, i32 0, i32 2
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %out3, ptr align 1 %3, i64 ptrtoint (ptr getelementptr ([11 x %OUT_TYPE], ptr null, i32 1) to i64), i1 false)
      %4 = getelementptr inbounds %FB, ptr %station, i32 0, i32 3
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %out4, ptr align 1 %4, i32 80, i1 false)
      %5 = getelementptr inbounds %FB, ptr %station, i32 0, i32 4
      call void @llvm.memcpy.p0.p0.i32(ptr align 2 %out5, ptr align 2 %5, i32 160, i1 false)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn array_of_string_parameter_with_stride_calculation() {
    let result = codegen(
        "
        FUNCTION foo
            VAR_IN_OUT
                strings : ARRAY[0..1] OF STRING[80];
            END_VAR
            strings[0] := 'Hello';
            strings[1] := 'World';
        END_FUNCTION

        FUNCTION main
            VAR
                arr : ARRAY[0..1] OF STRING[80];
            END_VAR
            foo(arr);
        END_FUNCTION
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"Hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"World\00"

    define void @foo(ptr %0) {
    entry:
      %strings = alloca ptr, align 8
      store ptr %0, ptr %strings, align 8
      %deref = load ptr, ptr %strings, align 8
      %tmpVar = getelementptr inbounds [2 x [81 x i8]], ptr %deref, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar, ptr align 1 @utf08_literal_0, i32 6, i1 false)
      %deref1 = load ptr, ptr %strings, align 8
      %tmpVar2 = getelementptr inbounds [2 x [81 x i8]], ptr %deref1, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar2, ptr align 1 @utf08_literal_1, i32 6, i1 false)
      ret void
    }

    define void @main() {
    entry:
      %arr = alloca [2 x [81 x i8]], align 1
      call void @llvm.memset.p0.i64(ptr align 1 %arr, i8 0, i64 ptrtoint (ptr getelementptr ([2 x [81 x i8]], ptr null, i32 1) to i64), i1 false)
      call void @foo(ptr %arr)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#)
}

#[test]
fn array_of_array_integer_parameter_with_stride_calculation() {
    let result = codegen(
        "
        FUNCTION foo
            VAR_IN_OUT
                numbers : ARRAY[0..1] OF ARRAY[0..2] OF DINT;
            END_VAR
            numbers[0][0] := 1;
            numbers[0][1] := 2;
            numbers[1][0] := 3;
            numbers[1][1] := 4;
        END_FUNCTION

        FUNCTION main
            VAR
                arr : ARRAY[0..1] OF ARRAY[0..2] OF DINT;
            END_VAR
            foo(arr);
        END_FUNCTION
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define void @foo(ptr %0) {
    entry:
      %numbers = alloca ptr, align 8
      store ptr %0, ptr %numbers, align 8
      %deref = load ptr, ptr %numbers, align 8
      %tmpVar = getelementptr inbounds [2 x [3 x i32]], ptr %deref, i32 0, i32 0
      %tmpVar1 = getelementptr inbounds [3 x i32], ptr %tmpVar, i32 0, i32 0
      store i32 1, ptr %tmpVar1, align 4
      %deref2 = load ptr, ptr %numbers, align 8
      %tmpVar3 = getelementptr inbounds [2 x [3 x i32]], ptr %deref2, i32 0, i32 0
      %tmpVar4 = getelementptr inbounds [3 x i32], ptr %tmpVar3, i32 0, i32 1
      store i32 2, ptr %tmpVar4, align 4
      %deref5 = load ptr, ptr %numbers, align 8
      %tmpVar6 = getelementptr inbounds [2 x [3 x i32]], ptr %deref5, i32 0, i32 1
      %tmpVar7 = getelementptr inbounds [3 x i32], ptr %tmpVar6, i32 0, i32 0
      store i32 3, ptr %tmpVar7, align 4
      %deref8 = load ptr, ptr %numbers, align 8
      %tmpVar9 = getelementptr inbounds [2 x [3 x i32]], ptr %deref8, i32 0, i32 1
      %tmpVar10 = getelementptr inbounds [3 x i32], ptr %tmpVar9, i32 0, i32 1
      store i32 4, ptr %tmpVar10, align 4
      ret void
    }

    define void @main() {
    entry:
      %arr = alloca [2 x [3 x i32]], align 4
      call void @llvm.memset.p0.i64(ptr align 1 %arr, i8 0, i64 ptrtoint (ptr getelementptr ([2 x [3 x i32]], ptr null, i32 1) to i64), i1 false)
      call void @foo(ptr %arr)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#)
}

#[test]
fn mixed_string_lengths_parameter_compatibility() {
    let result = codegen(
        "
        FUNCTION foo
            VAR_IN_OUT
                short_strings : ARRAY[0..1] OF STRING[10];
            END_VAR
            short_strings[0] := 'Hi';
            short_strings[1] := 'Bye';
        END_FUNCTION

        FUNCTION main
            VAR
                long_strings : ARRAY[0..1] OF STRING[80];
            END_VAR
            // This should work - passing longer strings to function expecting shorter ones
            foo(long_strings);
        END_FUNCTION
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @utf08_literal_0 = private unnamed_addr constant [4 x i8] c"Bye\00"
    @utf08_literal_1 = private unnamed_addr constant [3 x i8] c"Hi\00"

    define void @foo(ptr %0) {
    entry:
      %short_strings = alloca ptr, align 8
      store ptr %0, ptr %short_strings, align 8
      %deref = load ptr, ptr %short_strings, align 8
      %tmpVar = getelementptr inbounds [2 x [11 x i8]], ptr %deref, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar, ptr align 1 @utf08_literal_1, i32 3, i1 false)
      %deref1 = load ptr, ptr %short_strings, align 8
      %tmpVar2 = getelementptr inbounds [2 x [11 x i8]], ptr %deref1, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar2, ptr align 1 @utf08_literal_0, i32 4, i1 false)
      ret void
    }

    define void @main() {
    entry:
      %long_strings = alloca [2 x [81 x i8]], align 1
      call void @llvm.memset.p0.i64(ptr align 1 %long_strings, i8 0, i64 ptrtoint (ptr getelementptr ([2 x [81 x i8]], ptr null, i32 1) to i64), i1 false)
      call void @foo(ptr %long_strings)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#)
}

#[test]
fn program_with_array_of_string_parameter_stride_calculation() {
    let result = codegen(
        "
        PROGRAM StringProcessor
            VAR_IN_OUT
                messages : ARRAY[0..2] OF STRING[50];
            END_VAR
            messages[0] := 'First';
            messages[1] := 'Second';
            messages[2] := 'Third';
        END_PROGRAM

        PROGRAM main
            VAR
                text_array : ARRAY[0..2] OF STRING[50];
            END_VAR
            StringProcessor(text_array);
        END_PROGRAM
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %StringProcessor = type { ptr }
    %main = type { [3 x [51 x i8]] }

    @StringProcessor_instance = global %StringProcessor zeroinitializer
    @main_instance = global %main zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"First\00"
    @utf08_literal_1 = private unnamed_addr constant [7 x i8] c"Second\00"
    @utf08_literal_2 = private unnamed_addr constant [6 x i8] c"Third\00"

    define void @StringProcessor(ptr %0) {
    entry:
      %messages = getelementptr inbounds nuw %StringProcessor, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %messages, align 8
      %tmpVar = getelementptr inbounds [3 x [51 x i8]], ptr %deref, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar, ptr align 1 @utf08_literal_0, i32 6, i1 false)
      %deref1 = load ptr, ptr %messages, align 8
      %tmpVar2 = getelementptr inbounds [3 x [51 x i8]], ptr %deref1, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar2, ptr align 1 @utf08_literal_1, i32 7, i1 false)
      %deref3 = load ptr, ptr %messages, align 8
      %tmpVar4 = getelementptr inbounds [3 x [51 x i8]], ptr %deref3, i32 0, i32 2
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar4, ptr align 1 @utf08_literal_2, i32 6, i1 false)
      ret void
    }

    define void @main(ptr %0) {
    entry:
      %text_array = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      store ptr %text_array, ptr @StringProcessor_instance, align 8
      call void @StringProcessor(ptr @StringProcessor_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#)
}

#[test]
fn function_block_with_array_of_array_parameter_stride_calculation() {
    let result = codegen(
        "
        FUNCTION_BLOCK MatrixProcessor
            VAR_IN_OUT
                matrix : ARRAY[0..1] OF ARRAY[0..3] OF REAL;
            END_VAR
            matrix[0][0] := 1.1;
            matrix[0][1] := 2.2;
            matrix[1][0] := 3.3;
            matrix[1][1] := 4.4;
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                processor : MatrixProcessor;
                data : ARRAY[0..1] OF ARRAY[0..3] OF REAL;
            END_VAR
            processor(matrix := data);
        END_PROGRAM
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %MatrixProcessor = type { ptr }
    %main = type { %MatrixProcessor, [2 x [4 x float]] }

    @__MatrixProcessor__init = unnamed_addr constant %MatrixProcessor zeroinitializer
    @main_instance = global %main zeroinitializer

    define void @MatrixProcessor(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %matrix = getelementptr inbounds nuw %MatrixProcessor, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %matrix, align 8
      %tmpVar = getelementptr inbounds [2 x [4 x float]], ptr %deref, i32 0, i32 0
      %tmpVar1 = getelementptr inbounds [4 x float], ptr %tmpVar, i32 0, i32 0
      store float 0x3FF19999A0000000, ptr %tmpVar1, align 4
      %deref2 = load ptr, ptr %matrix, align 8
      %tmpVar3 = getelementptr inbounds [2 x [4 x float]], ptr %deref2, i32 0, i32 0
      %tmpVar4 = getelementptr inbounds [4 x float], ptr %tmpVar3, i32 0, i32 1
      store float 0x40019999A0000000, ptr %tmpVar4, align 4
      %deref5 = load ptr, ptr %matrix, align 8
      %tmpVar6 = getelementptr inbounds [2 x [4 x float]], ptr %deref5, i32 0, i32 1
      %tmpVar7 = getelementptr inbounds [4 x float], ptr %tmpVar6, i32 0, i32 0
      store float 0x400A666660000000, ptr %tmpVar7, align 4
      %deref8 = load ptr, ptr %matrix, align 8
      %tmpVar9 = getelementptr inbounds [2 x [4 x float]], ptr %deref8, i32 0, i32 1
      %tmpVar10 = getelementptr inbounds [4 x float], ptr %tmpVar9, i32 0, i32 1
      store float 0x40119999A0000000, ptr %tmpVar10, align 4
      ret void
    }

    define void @main(ptr %0) {
    entry:
      %processor = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %data = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      %1 = getelementptr inbounds %MatrixProcessor, ptr %processor, i32 0, i32 0
      store ptr %data, ptr %1, align 8
      call void @MatrixProcessor(ptr %processor)
      ret void
    }
    "#)
}

#[test]
fn method_with_var_in_out_array_of_strings() {
    let result = codegen(
        "
        PROGRAM StringHandler
            METHOD process_strings
                VAR_IN_OUT
                    string_list : ARRAY[0..1] OF STRING[30];
                END_VAR
                string_list[0] := 'Hello';
                string_list[1] := 'World';
            END_METHOD
        END_PROGRAM

        PROGRAM main
            VAR
                handler : StringHandler;
                my_strings : ARRAY[0..1] OF STRING[30];
            END_VAR
            handler.process_strings(my_strings);
        END_PROGRAM
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %StringHandler = type {}
    %main = type { %StringHandler, [2 x [31 x i8]] }

    @StringHandler_instance = global %StringHandler zeroinitializer
    @main_instance = global %main zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"Hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"World\00"

    define void @StringHandler(ptr %0) {
    entry:
      ret void
    }

    define void @StringHandler__process_strings(ptr %0, ptr %1) {
    entry:
      %string_list = alloca ptr, align 8
      store ptr %1, ptr %string_list, align 8
      %deref = load ptr, ptr %string_list, align 8
      %tmpVar = getelementptr inbounds [2 x [31 x i8]], ptr %deref, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar, ptr align 1 @utf08_literal_0, i32 6, i1 false)
      %deref1 = load ptr, ptr %string_list, align 8
      %tmpVar2 = getelementptr inbounds [2 x [31 x i8]], ptr %deref1, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar2, ptr align 1 @utf08_literal_1, i32 6, i1 false)
      ret void
    }

    define void @main(ptr %0) {
    entry:
      %handler = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %my_strings = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      call void @StringHandler__process_strings(ptr %handler, ptr %my_strings)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#)
}

#[test]
fn method_with_var_in_out_nested_integer_arrays() {
    let result = codegen(
        "
        FUNCTION_BLOCK DataProcessor
            METHOD process_matrix
                VAR_IN_OUT
                    data : ARRAY[0..1] OF ARRAY[0..1] OF DINT;
                END_VAR
                data[0][0] := 10;
                data[0][1] := 20;
                data[1][0] := 30;
                data[1][1] := 40;
            END_METHOD
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                processor : DataProcessor;
                matrix : ARRAY[0..1] OF ARRAY[0..1] OF DINT;
            END_VAR
            processor.process_matrix(matrix);
        END_PROGRAM
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %DataProcessor = type {}
    %main = type { %DataProcessor, [2 x [2 x i32]] }

    @__DataProcessor__init = unnamed_addr constant %DataProcessor zeroinitializer
    @main_instance = global %main zeroinitializer

    define void @DataProcessor(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      ret void
    }

    define void @DataProcessor__process_matrix(ptr %0, ptr %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %data = alloca ptr, align 8
      store ptr %1, ptr %data, align 8
      %deref = load ptr, ptr %data, align 8
      %tmpVar = getelementptr inbounds [2 x [2 x i32]], ptr %deref, i32 0, i32 0
      %tmpVar1 = getelementptr inbounds [2 x i32], ptr %tmpVar, i32 0, i32 0
      store i32 10, ptr %tmpVar1, align 4
      %deref2 = load ptr, ptr %data, align 8
      %tmpVar3 = getelementptr inbounds [2 x [2 x i32]], ptr %deref2, i32 0, i32 0
      %tmpVar4 = getelementptr inbounds [2 x i32], ptr %tmpVar3, i32 0, i32 1
      store i32 20, ptr %tmpVar4, align 4
      %deref5 = load ptr, ptr %data, align 8
      %tmpVar6 = getelementptr inbounds [2 x [2 x i32]], ptr %deref5, i32 0, i32 1
      %tmpVar7 = getelementptr inbounds [2 x i32], ptr %tmpVar6, i32 0, i32 0
      store i32 30, ptr %tmpVar7, align 4
      %deref8 = load ptr, ptr %data, align 8
      %tmpVar9 = getelementptr inbounds [2 x [2 x i32]], ptr %deref8, i32 0, i32 1
      %tmpVar10 = getelementptr inbounds [2 x i32], ptr %tmpVar9, i32 0, i32 1
      store i32 40, ptr %tmpVar10, align 4
      ret void
    }

    define void @main(ptr %0) {
    entry:
      %processor = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %matrix = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      call void @DataProcessor__process_matrix(ptr %processor, ptr %matrix)
      ret void
    }
    "#)
}

#[test]
fn method_with_mixed_array_types() {
    let result = codegen(
        "
        FUNCTION_BLOCK ComplexHandler
            METHOD handle_data
                VAR_IN_OUT
                    strings : ARRAY[0..1] OF STRING[20];
                    numbers : ARRAY[0..2] OF ARRAY[0..1] OF INT;
                END_VAR
                strings[0] := 'Data';
                strings[1] := 'Processing';
                numbers[0][0] := 100;
                numbers[1][1] := 200;
            END_METHOD
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                handler : ComplexHandler;
                text_data : ARRAY[0..1] OF STRING[20];
                num_data : ARRAY[0..2] OF ARRAY[0..1] OF INT;
            END_VAR
            handler.handle_data(strings := text_data, numbers := num_data);
        END_PROGRAM
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %ComplexHandler = type {}
    %main = type { %ComplexHandler, [2 x [21 x i8]], [3 x [2 x i16]] }

    @__ComplexHandler__init = unnamed_addr constant %ComplexHandler zeroinitializer
    @main_instance = global %main zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [5 x i8] c"Data\00"
    @utf08_literal_1 = private unnamed_addr constant [11 x i8] c"Processing\00"

    define void @ComplexHandler(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      ret void
    }

    define void @ComplexHandler__handle_data(ptr %0, ptr %1, ptr %2) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %strings = alloca ptr, align 8
      store ptr %1, ptr %strings, align 8
      %numbers = alloca ptr, align 8
      store ptr %2, ptr %numbers, align 8
      %deref = load ptr, ptr %strings, align 8
      %tmpVar = getelementptr inbounds [2 x [21 x i8]], ptr %deref, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar, ptr align 1 @utf08_literal_0, i32 5, i1 false)
      %deref1 = load ptr, ptr %strings, align 8
      %tmpVar2 = getelementptr inbounds [2 x [21 x i8]], ptr %deref1, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar2, ptr align 1 @utf08_literal_1, i32 11, i1 false)
      %deref3 = load ptr, ptr %numbers, align 8
      %tmpVar4 = getelementptr inbounds [3 x [2 x i16]], ptr %deref3, i32 0, i32 0
      %tmpVar5 = getelementptr inbounds [2 x i16], ptr %tmpVar4, i32 0, i32 0
      store i16 100, ptr %tmpVar5, align 2
      %deref6 = load ptr, ptr %numbers, align 8
      %tmpVar7 = getelementptr inbounds [3 x [2 x i16]], ptr %deref6, i32 0, i32 1
      %tmpVar8 = getelementptr inbounds [2 x i16], ptr %tmpVar7, i32 0, i32 1
      store i16 200, ptr %tmpVar8, align 2
      ret void
    }

    define void @main(ptr %0) {
    entry:
      %handler = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %text_data = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      %num_data = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 2
      call void @ComplexHandler__handle_data(ptr %handler, ptr %text_data, ptr %num_data)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#)
}

#[test]
fn function_with_array_of_array_return() {
    let result = codegen(
        "
        FUNCTION foo : ARRAY[0..1] OF ARRAY[0..1] OF INT
            VAR
                result : ARRAY[0..1] OF ARRAY[0..1] OF INT;
            END_VAR
            result[0][0] := 5;
            result[0][1] := 10;
            result[1][0] := 15;
            result[1][1] := 20;
            foo := result;
        END_FUNCTION

        FUNCTION bar : ARRAY[0..1] OF ARRAY[0..1] OF INT
            VAR
                data : ARRAY[0..1] OF ARRAY[0..1] OF INT;
            END_VAR
            data := foo();
            bar := data;
        END_FUNCTION

        FUNCTION baz : ARRAY[0..2] OF STRING[20]
            VAR
                texts : ARRAY[0..2] OF STRING[20];
            END_VAR
            texts[0] := 'One';
            texts[1] := 'Two';
            texts[2] := 'Three';
            baz := texts;
        END_FUNCTION

        PROGRAM main
            VAR
                numbers : ARRAY[0..1] OF ARRAY[0..1] OF INT;
                strings : ARRAY[0..2] OF STRING[20];
            END_VAR
            numbers := bar();
            strings := baz();
        END_PROGRAM
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { [2 x [2 x i16]], [3 x [21 x i8]] }

    @main_instance = global %main zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [4 x i8] c"One\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"Three\00"
    @utf08_literal_2 = private unnamed_addr constant [4 x i8] c"Two\00"

    define void @foo(ptr %0) {
    entry:
      %foo = alloca ptr, align 8
      store ptr %0, ptr %foo, align 8
      %result = alloca [2 x [2 x i16]], align 2
      call void @llvm.memset.p0.i64(ptr align 1 %result, i8 0, i64 ptrtoint (ptr getelementptr ([2 x [2 x i16]], ptr null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [2 x [2 x i16]], ptr %result, i32 0, i32 0
      %tmpVar1 = getelementptr inbounds [2 x i16], ptr %tmpVar, i32 0, i32 0
      store i16 5, ptr %tmpVar1, align 2
      %tmpVar2 = getelementptr inbounds [2 x [2 x i16]], ptr %result, i32 0, i32 0
      %tmpVar3 = getelementptr inbounds [2 x i16], ptr %tmpVar2, i32 0, i32 1
      store i16 10, ptr %tmpVar3, align 2
      %tmpVar4 = getelementptr inbounds [2 x [2 x i16]], ptr %result, i32 0, i32 1
      %tmpVar5 = getelementptr inbounds [2 x i16], ptr %tmpVar4, i32 0, i32 0
      store i16 15, ptr %tmpVar5, align 2
      %tmpVar6 = getelementptr inbounds [2 x [2 x i16]], ptr %result, i32 0, i32 1
      %tmpVar7 = getelementptr inbounds [2 x i16], ptr %tmpVar6, i32 0, i32 1
      store i16 20, ptr %tmpVar7, align 2
      %deref = load ptr, ptr %foo, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %deref, ptr align 1 %result, i64 ptrtoint (ptr getelementptr ([2 x [2 x i16]], ptr null, i32 1) to i64), i1 false)
      ret void
    }

    define void @bar(ptr %0) {
    entry:
      %bar = alloca ptr, align 8
      store ptr %0, ptr %bar, align 8
      %data = alloca [2 x [2 x i16]], align 2
      call void @llvm.memset.p0.i64(ptr align 1 %data, i8 0, i64 ptrtoint (ptr getelementptr ([2 x [2 x i16]], ptr null, i32 1) to i64), i1 false)
      %__foo0 = alloca [2 x [2 x i16]], align 2
      call void @llvm.memset.p0.i64(ptr align 1 %__foo0, i8 0, i64 ptrtoint (ptr getelementptr ([2 x [2 x i16]], ptr null, i32 1) to i64), i1 false)
      call void @foo(ptr %__foo0)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %data, ptr align 1 %__foo0, i64 ptrtoint (ptr getelementptr ([2 x [2 x i16]], ptr null, i32 1) to i64), i1 false)
      %deref = load ptr, ptr %bar, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %deref, ptr align 1 %data, i64 ptrtoint (ptr getelementptr ([2 x [2 x i16]], ptr null, i32 1) to i64), i1 false)
      ret void
    }

    define void @baz(ptr %0) {
    entry:
      %baz = alloca ptr, align 8
      store ptr %0, ptr %baz, align 8
      %texts = alloca [3 x [21 x i8]], align 1
      call void @llvm.memset.p0.i64(ptr align 1 %texts, i8 0, i64 ptrtoint (ptr getelementptr ([3 x [21 x i8]], ptr null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [3 x [21 x i8]], ptr %texts, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar, ptr align 1 @utf08_literal_0, i32 4, i1 false)
      %tmpVar1 = getelementptr inbounds [3 x [21 x i8]], ptr %texts, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar1, ptr align 1 @utf08_literal_2, i32 4, i1 false)
      %tmpVar2 = getelementptr inbounds [3 x [21 x i8]], ptr %texts, i32 0, i32 2
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmpVar2, ptr align 1 @utf08_literal_1, i32 6, i1 false)
      %deref = load ptr, ptr %baz, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %deref, ptr align 1 %texts, i64 ptrtoint (ptr getelementptr ([3 x [21 x i8]], ptr null, i32 1) to i64), i1 false)
      ret void
    }

    define void @main(ptr %0) {
    entry:
      %numbers = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %strings = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      %__bar1 = alloca [2 x [2 x i16]], align 2
      call void @llvm.memset.p0.i64(ptr align 1 %__bar1, i8 0, i64 ptrtoint (ptr getelementptr ([2 x [2 x i16]], ptr null, i32 1) to i64), i1 false)
      call void @bar(ptr %__bar1)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %numbers, ptr align 1 %__bar1, i64 ptrtoint (ptr getelementptr ([2 x [2 x i16]], ptr null, i32 1) to i64), i1 false)
      %__baz2 = alloca [3 x [21 x i8]], align 1
      call void @llvm.memset.p0.i64(ptr align 1 %__baz2, i8 0, i64 ptrtoint (ptr getelementptr ([3 x [21 x i8]], ptr null, i32 1) to i64), i1 false)
      call void @baz(ptr %__baz2)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %strings, ptr align 1 %__baz2, i64 ptrtoint (ptr getelementptr ([3 x [21 x i8]], ptr null, i32 1) to i64), i1 false)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}
