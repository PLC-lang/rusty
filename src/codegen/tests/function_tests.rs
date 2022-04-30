use crate::test_utils::tests::codegen;

#[test]
fn member_variables_in_body() {
    let result = codegen(
        r#"FUNCTION func : DINT
            VAR_INPUT   i   : INT := 6 END_VAR
            VAR_IN_OUT  io  : SINT;      END_VAR
            VAR_OUTPUT  o   : LINT;      END_VAR
            VAR         v   : INT := 1; END_VAR
            VAR_TEMP    vt  : INT := 2; END_VAR
            
            func := i * io - o + v * vt;
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define i32 @func(i16 %0, i8* %1, i64 %2) {
    entry:
      %i = alloca i16, align 2
      store i16 %0, i16* %i, align 2
      %io = alloca i8*, align 8
      store i8* %1, i8** %io, align 8
      %o = alloca i64, align 8  <<-- this needs to be a pointer!
      store i64 %2, i64* %o, align 4
      %v = alloca i16, align 2
      %vt = alloca i16, align 2
      %func = alloca i32, align 4
      store i16 1, i16* %v, align 2
      store i16 2, i16* %vt, align 2
      store i32 0, i32* %func, align 4
      %load_i = load i16, i16* %i, align 2
      %3 = sext i16 %load_i to i32
      %deref = load i8*, i8** %io, align 8
      %load_io = load i8, i8* %deref, align 1
      %4 = sext i8 %load_io to i32
      %tmpVar = mul i32 %3, %4
      %5 = sext i32 %tmpVar to i64
      %load_o = load i64, i64* %o, align 4
      %tmpVar1 = sub i64 %5, %load_o
      %load_v = load i16, i16* %v, align 2
      %6 = sext i16 %load_v to i32
      %load_vt = load i16, i16* %vt, align 2
      %7 = sext i16 %load_vt to i32
      %tmpVar2 = mul i32 %6, %7
      %8 = sext i32 %tmpVar2 to i64
      %tmpVar3 = add i64 %tmpVar1, %8
      %9 = trunc i64 %tmpVar3 to i32
      store i32 %9, i32* %func, align 4
      %func_ret = load i32, i32* %func, align 4
      ret i32 %func_ret
    }
    "###);
}

#[test]
fn simple_call() {
    let result = codegen(
        r#"FUNCTION func : DINT
            VAR_INPUT x : DINT; END_VAR
        END_FUNCTION

        PROGRAM main
            VAR a : DINT; END_VAR

            func(a);
            func(1);
            func(1+a);
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main_interface = type { i32 }

    @main_instance = global %main_interface zeroinitializer

    define i32 @func(i32 %0) {
    entry:
      %x = alloca i32, align 4
      store i32 %0, i32* %x, align 4
      %func = alloca i32, align 4
      store i32 0, i32* %func, align 4
      %func_ret = load i32, i32* %func, align 4
      ret i32 %func_ret
    }

    define void @main(%main_interface* %0) {
    entry:
      %a = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 0
      %load_a = load i32, i32* %a, align 4
      %call = call i32 @func(i32 %load_a)
      %call1 = call i32 @func(i32 1)
      %load_a2 = load i32, i32* %a, align 4
      %tmpVar = add i32 1, %load_a2
      %call3 = call i32 @func(i32 %tmpVar)
      ret void
    }
    "###);
}

#[test]
fn passing_a_string_to_a_function() {
    let result = codegen(
        r#"FUNCTION func : DINT
            VAR_INPUT x : STRING[5]; END_VAR
        END_FUNCTION

        PROGRAM main
            VAR a : STRING[5]; END_VAR

            func(a);
            func('12345');
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main_interface = type { [6 x i8] }

    @main_instance = global %main_interface zeroinitializer
    @utf08_literal_0 = unnamed_addr constant [6 x i8] c"12345\00"

    define i32 @func([6 x i8] %0) {
    entry:
      %x = alloca [6 x i8], align 1
      store [6 x i8] %0, [6 x i8]* %x, align 1
      %func = alloca i32, align 4
      store i32 0, i32* %func, align 4
      %func_ret = load i32, i32* %func, align 4
      ret i32 %func_ret
    }

    define void @main(%main_interface* %0) {
    entry:
      %a = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 0
      %load_a = load [6 x i8], [6 x i8]* %a, align 1
      %call = call i32 @func([6 x i8] %load_a)
      %1 = load [6 x i8], [6 x i8]* @utf08_literal_0, align 1
      %call1 = call i32 @func([6 x i8] %1)
      ret void
    }
    "###);
}
