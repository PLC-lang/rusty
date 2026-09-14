use plc_util::filtered_assert_snapshot;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;

#[test]
fn bitaccess_generated_as_rsh_and_trunc_i1() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : BOOL;
x : DWORD;
y : DINT;
END_VAR
a := x.2;
a := y.%X4;
END_PROGRAM
"#,
    );
    filtered_assert_snapshot!(result);
}

#[test]
fn byteaccess_generated_as_rsh_and_trunc_i8() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : BYTE;
x : DWORD;
y : DINT;
END_VAR
a := x.%B0;
a := x.%B1;
a := y.%B3;
END_PROGRAM
"#,
    );
    filtered_assert_snapshot!(result);
}

#[test]
fn wordaccess_generated_as_rsh_and_trunc_i16() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : WORD;
x : DWORD;
y : DINT;
END_VAR
a := x.%W0;
a := x.%W1;
a := y.%W1;
END_PROGRAM
"#,
    );
    filtered_assert_snapshot!(result);
}

#[test]
fn dwordaccess_generated_as_rsh_and_trunc_i32() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : DWORD;
x : LWORD;
y : LINT;
END_VAR
a := x.%D0;
a := x.%D1;
a := y.%D1;
END_PROGRAM
"#,
    );
    filtered_assert_snapshot!(result);
}

#[test]
fn nested_bitwise_access() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : BOOL;
x : LWORD;
END_VAR
(* Second bit of the second byte of the second word of the second dword of an lword*)
a := x.%D1.%W1.%B1.%X1;
END_PROGRAM
"#,
    );
    filtered_assert_snapshot!(result);
}

#[test]
fn variable_based_bitwise_access() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : BOOL;
b : BYTE;
x : INT;
y : INT;
END_VAR
a := x.%Xy;
b := x.%By;
END_PROGRAM
"#,
    );
    filtered_assert_snapshot!(result);
}

#[test]
fn function_result_assignment_on_string() {
    let result = codegen(
        r#"
        @EXTERNAL
        FUNCTION CONCAT : STRING[1024]
        VAR_INPUT a,b : STRING[1024]; END_VAR
        END_FUNCTION

        FUNCTION LIST_ADD : BOOL
        VAR_INPUT
            INS : STRING[1000];
            sx : STRING[1] := ' ';
        END_VAR

        INS := CONCAT(sx, INS);
        END_FUNCTION
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn function_result_assignment_on_aliased_string() {
    let result = codegen(
        r#"
        TYPE MyStr : STRING[1000]; END_TYPE
        TYPE LongStr : STRING[1024]; END_TYPE

        @EXTERNAL
        FUNCTION CONCAT : LongStr
        VAR_INPUT a,b : LongStr; END_VAR
        END_FUNCTION

        FUNCTION LIST_ADD : BOOL
        VAR_INPUT
            INS : MyStr;
            sx : STRING[1] := ' ';
        END_VAR

        INS := CONCAT(sx, INS);
        END_FUNCTION
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn floating_point_type_casting() {
    let result = codegen(
        r#"
        FUNCTION fn : DINT
            VAR
                a : REAL  :=       7 / 2; // => 3.0 (because we do a integer division first and only then cast the result)
                b : REAL  :=  REAL#7 / 2; // => 3.5 (because we first cast then divide)
                c : REAL  := LREAL#7 / 2; // => 3.5 ^

                d : LREAL :=       7 / 2;  // => 3.0 (because we do a integer division first and only then cast the result)
                e : LREAL :=  REAL#7 / 2;  // => 3.5 (because we first cast then divide)
                f : LREAL := LREAL#7 / 2;  // => 3.5 ^
            END_VAR

            // Same reasoning as above
            a :=       7 / 2;
            b :=  REAL#7 / 2;
            c := LREAL#7 / 2;

            d :=       7 / 2;
            e :=  REAL#7 / 2;
            f := LREAL#7 / 2;
        END_FUNCTION
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn ref_assignment() {
    let result = codegen(
        r#"
        FUNCTION main
        VAR
            a : REF_TO DINT;
            b : DINT;
        END_VAR
            a REF= b;
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define void @main() {
    entry:
      %a = alloca ptr, align [filtered]
      %b = alloca i32, align [filtered]
      store ptr null, ptr %a, align [filtered]
      store i32 0, ptr %b, align [filtered]
      store ptr %b, ptr %a, align [filtered]
      ret void
    }
    "#);
}

#[test]
fn ref_assignment_to_null() {
    let result = codegen(
        r#"
        FUNCTION main
        VAR
            a : REF_TO DINT;
        END_VAR
            a REF= 0;
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define void @main() {
    entry:
      %a = alloca ptr, align [filtered]
      store ptr null, ptr %a, align [filtered]
      store i32 0, ptr %a, align [filtered]
      ret void
    }
    "#);
}

#[test]
fn reference_to_assignment() {
    let auto_deref = codegen(
        r#"
        FUNCTION main
            VAR
                a : REFERENCE TO DINT;
            END_VAR
            a := 5;
        END_FUNCTION
        "#,
    );

    let manual_deref = codegen(
        r#"
        FUNCTION main
            VAR
                a : REF_TO DINT;
            END_VAR
            a^ := 5;
        END_FUNCTION
        "#,
    );

    // We want to assert that `a := 5` and `a^ := 5` yield identical IR
    assert_eq!(auto_deref, manual_deref);

    filtered_assert_snapshot!(auto_deref, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define void @main() {
    entry:
      %a = alloca ptr, align [filtered]
      store ptr null, ptr %a, align [filtered]
      %deref = load ptr, ptr %a, align [filtered]
      store i32 5, ptr %deref, align [filtered]
      ret void
    }
    "#);
}

#[test]
fn reference_to_string_assignment() {
    let auto_deref = codegen(
        r#"
        FUNCTION main
            VAR
                a : REFERENCE TO STRING;
            END_VAR

            a := 'hello';
        END_FUNCTION
        "#,
    );

    let manual_deref = codegen(
        r#"
        FUNCTION main
            VAR
                a : REF_TO STRING;
            END_VAR
            a^ := 'hello';
        END_FUNCTION
        "#,
    );

    // We want to assert that `a := 'hello'` and `a^ := 'hello'` yield identical IR
    assert_eq!(auto_deref, manual_deref);

    filtered_assert_snapshot!(auto_deref, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"

    define void @main() {
    entry:
      %a = alloca ptr, align [filtered]
      store ptr null, ptr %a, align [filtered]
      %deref = load ptr, ptr %a, align [filtered]
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %deref, ptr align [filtered] @utf08_literal_0, i32 6, i1 false)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn local_alias() {
    let content = codegen(
        r#"
        FUNCTION main
            VAR
                foo AT bar : DINT;
                bar : DINT;
            END_VAR
        END_FUNCTION
        "#,
    );

    filtered_assert_snapshot!(content, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define void @main() {
    entry:
      %foo = alloca ptr, align [filtered]
      %bar = alloca i32, align [filtered]
      store ptr null, ptr %foo, align [filtered]
      store i32 0, ptr %bar, align [filtered]
      ret void
    }
    "#);
}

#[test]
fn local_string_alias() {
    let content = codegen(
        r#"
        FUNCTION main
            VAR
                foo AT bar : STRING;
                bar : STRING;
            END_VAR
        END_FUNCTION
        "#,
    );

    filtered_assert_snapshot!(content, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define void @main() {
    entry:
      %foo = alloca ptr, align [filtered]
      %bar = alloca [81 x i8], align [filtered]
      store ptr null, ptr %foo, align [filtered]
      call void @llvm.memset.p0.i64(ptr align [filtered] %bar, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#);
}

#[test]
#[ignore = "stack overflow regression with 28b6b41"]
fn local_struct_alias() {
    let content = codegen(
        r#"
        TYPE Node : STRUCT
            id      : DINT;
            child   : REF_TO Node;
            parent  : REF_TO Node;
        END_STRUCT END_TYPE

        FUNCTION main
            VAR
                foo AT bar : Node;
                bar : Node;
            END_VAR
        END_FUNCTION
        "#,
    );

    filtered_assert_snapshot!(content, @r"");
}

#[test]
#[ignore = "REF(x) initializers where x is a member variable are not yet implemented (https://github.com/PLC-lang/rusty/issues/1286)"]
fn stateful_local() {
    let content = codegen(
        r#"
        FUNCTION_BLOCK foo
            VAR
                foo AT bar : STRING;
                bar : STRING;
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    filtered_assert_snapshot!(content, @r"");
}

#[test]
fn real_conversion_with_pretty_syntax() {
    let non_pretty_syntax = codegen(
        r#"
        {external}
        FUNCTION REAL_TO_INT : INT
        VAR_INPUT
            in : REAL;
        END_VAR
        END_FUNCTION

        FUNCTION main
            VAR
                var_int     : INT;
            END_VAR

            // Cast the REAL value to INT
            var_int := REAL_TO_INT(70000.4);

        END_FUNCTION
        "#,
    );

    let pretty_syntax = codegen(
        r#"
        {external}
        FUNCTION REAL_TO_INT : INT
        VAR_INPUT
            in : REAL;
        END_VAR
        END_FUNCTION

        FUNCTION main
            VAR
                var_int     : INT;
            END_VAR

            // Cast the REAL value to INT
            var_int := REAL_TO_INT(70_000.4);

        END_FUNCTION
        "#,
    );

    assert_eq!(non_pretty_syntax, pretty_syntax);

    filtered_assert_snapshot!(pretty_syntax, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    declare i16 @REAL_TO_INT(float)

    define void @main() {
    entry:
      %var_int = alloca i16, align [filtered]
      store i16 0, ptr %var_int, align [filtered]
      %call = call i16 @REAL_TO_INT(float 0x40F1170660000000)
      store i16 %call, ptr %var_int, align [filtered]
      ret void
    }
    "#);
}

#[test]
fn struct_literal_assignment_with_sized_string_member() {
    let result = codegen(
        r#"
        TYPE Motor : STRUCT
            name  : STRING[20];
            speed : REAL;
        END_STRUCT END_TYPE

        PROGRAM prg
        VAR
            motor  : Motor;
            motors : ARRAY[0..1] OF Motor;
        END_VAR
            motor := (name := 'Motor1', speed := 10.5);
            motors[1] := (name := 'Motor2', speed := 20.5);
        END_PROGRAM
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { %Motor, [2 x %Motor] }
    %Motor = type { [21 x i8], float }

    @prg_instance = global %prg zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [7 x i8] c"Motor1\00"
    @utf08_literal_1 = private unnamed_addr constant [7 x i8] c"Motor2\00"
    @.const_init = private unnamed_addr constant %Motor { [21 x i8] c"Motor1\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", float 1.050000e+01 }
    @.const_init.1 = private unnamed_addr constant %Motor { [21 x i8] c"Motor2\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", float 2.050000e+01 }

    define void @prg(ptr %0) {
    entry:
      %motor = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %motors = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %motor, ptr align [filtered] @.const_init, i64 ptrtoint (ptr getelementptr (%Motor, ptr null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [2 x %Motor], ptr %motors, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %tmpVar, ptr align [filtered] @.const_init.1, i64 ptrtoint (ptr getelementptr (%Motor, ptr null, i32 1) to i64), i1 false)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn struct_literal_member_from_string_variable_of_different_size() {
    let result = codegen(
        r#"
        TYPE Motor : STRUCT
            name  : STRING[20];
            speed : REAL;
        END_STRUCT END_TYPE

        PROGRAM prg
        VAR
            motor      : Motor;
            long_name  : STRING;
            short_name : STRING[5];
        END_VAR
            motor := (name := long_name, speed := 1.0);
            motor := (name := short_name, speed := 2.0);
        END_PROGRAM
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { %Motor, [81 x i8], [6 x i8] }
    %Motor = type { [21 x i8], float }

    @prg_instance = global %prg zeroinitializer
    @.const_init = private unnamed_addr constant %Motor zeroinitializer
    @.const_init.1 = private unnamed_addr constant %Motor zeroinitializer

    define void @prg(ptr %0) {
    entry:
      %motor = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %long_name = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      %short_name = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 2
      %struct_literal = alloca %Motor, align [filtered]
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %struct_literal, ptr align [filtered] @.const_init, i64 ptrtoint (ptr getelementptr (%Motor, ptr null, i32 1) to i64), i1 false)
      %name = getelementptr inbounds nuw %Motor, ptr %struct_literal, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %name, ptr align [filtered] %long_name, i32 20, i1 false)
      %speed = getelementptr inbounds nuw %Motor, ptr %struct_literal, i32 0, i32 1
      store float 1.000000e+00, ptr %speed, align [filtered]
      %1 = load %Motor, ptr %struct_literal, align [filtered]
      store %Motor %1, ptr %motor, align [filtered]
      %struct_literal1 = alloca %Motor, align [filtered]
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %struct_literal1, ptr align [filtered] @.const_init.1, i64 ptrtoint (ptr getelementptr (%Motor, ptr null, i32 1) to i64), i1 false)
      %name2 = getelementptr inbounds nuw %Motor, ptr %struct_literal1, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %name2, ptr align [filtered] %short_name, i32 6, i1 false)
      %speed3 = getelementptr inbounds nuw %Motor, ptr %struct_literal1, i32 0, i32 1
      store float 2.000000e+00, ptr %speed3, align [filtered]
      %2 = load %Motor, ptr %struct_literal1, align [filtered]
      store %Motor %2, ptr %motor, align [filtered]
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}
