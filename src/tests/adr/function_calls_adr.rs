use crate::test_utils::tests::codegen;

/// # Architecture Design Record: Function Calls
///
/// Function calls are handled differently compared to calls to Programs and FunctionBlocks. While
/// most of the parameters of a Program/FB are part of its state, they are stored in a struct that
/// is passed to it. Functions are stateless (between calls) and therefore get all their
/// parameters handed one by one, exactly like we would expect it from a C-like language.
#[test]
fn function_calls_pass_single_arguments() {
    let src = r###"
        FUNCTION add : DINT
            VAR_INPUT a,b : DINT END_VAR
            add := a + b;
        END_FUNCTION

        PROGRAM main
            add(2, 3);
        END_PROGRAM
    "###;

    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main = type {}

    @main_instance = global %main zeroinitializer

    define i32 @add(i32 %0, i32 %1) {
    entry:
      %add = alloca i32, align 4
      %a = alloca i32, align 4
      store i32 %0, i32* %a, align 4
      %b = alloca i32, align 4
      store i32 %1, i32* %b, align 4
      store i32 0, i32* %add, align 4
      %load_a = load i32, i32* %a, align 4
      %load_b = load i32, i32* %b, align 4
      %tmpVar = add i32 %load_a, %load_b
      store i32 %tmpVar, i32* %add, align 4
      %add_ret = load i32, i32* %add, align 4
      ret i32 %add_ret
    }

    define void @main(%main* %0) {
    entry:
      %call = call i32 @add(i32 2, i32 3)
      ret void
    }
    "###);
}

/// Returning aggregate types is an expensive operation, this is why the compiler will
/// turn a aggregate return type (string, array, struct) into a VAR_OUT variable.
///  ... an aggregate type will handled via a var_out variable which means it is passed as a pointer
///  ... internally the function is treated as a void function
///  ... the caller allocates the required memory on its stack and passes a pointer to it
///  ... after the call, the out pointer is used as the function's result
///  ... scalar return types are treated as a classical return value
#[test]
fn returning_aggregate_types_from_functions() {
    let src = r###"
        FUNCTION GetString : STRING
            // ... 
        END_FUNCTION
        FUNCTION GetInt : INT
            // ... 
        END_FUNCTION

        PROGRAM main
            VAR 
                str : STRING; 
                i : INT;
            END_VAR
            str := GetString();
            i := GetInt();
        END_PROGRAM
    "###;

    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main = type { [81 x i8], i16 }

    @main_instance = global %main zeroinitializer

    define void @GetString([81 x i8]* %0) {
    entry:
      %GetString = alloca [81 x i8]*, align 8
      store [81 x i8]* %0, [81 x i8]** %GetString, align 8
      %deref = load [81 x i8]*, [81 x i8]** %GetString, align 8
      %1 = bitcast [81 x i8]* %deref to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      ret void
    }

    define i16 @GetInt() {
    entry:
      %GetInt = alloca i16, align 2
      store i16 0, i16* %GetInt, align 2
      %GetInt_ret = load i16, i16* %GetInt, align 2
      ret i16 %GetInt_ret
    }

    define void @main(%main* %0) {
    entry:
      %str = getelementptr inbounds %main, %main* %0, i32 0, i32 0
      %i = getelementptr inbounds %main, %main* %0, i32 0, i32 1
      %1 = alloca [81 x i8], align 1
      call void @GetString([81 x i8]* %1)
      %2 = bitcast [81 x i8]* %str to i8*
      %3 = bitcast [81 x i8]* %1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %2, i8* align 1 %3, i32 80, i1 false)
      %call = call i16 @GetInt()
      store i16 %call, i16* %i, align 2
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #1

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    "###);
}

/// Passing aggregate types to a function is an expensive operation, this is why the compiler offers
/// a {ref} macro that turns a block of variables into by-ref parameters without changing the
/// functions interface during compile-time.
///  ... all variables of a block marked by {ref} will be passed by-reference
///  ... the function's signature does not change, a caller passes the variable still the same way.
///      Although it looks like a value is passed by value, it is internally treated as a by-ref parameter
///  ... this allows more efficient function calls if the function can assure that it will not create any
///      side-effects regarding the passed values (e.g. change the variable).
#[test]
fn passing_by_ref_to_functions() {
    let src = r###"
        FUNCTION StrEqual : BOOL
          VAR_INPUT {ref}
            o1: STRING;
            o2: STRING;
          END_VAR
            // ...
        END_FUNCTION
        PROGRAM main
            VAR 
                str1, str2 : STRING; 
            END_VAR
            StrEqual(str1, str2); //looks like pass by-val
        END_PROGRAM
    "###;

    //internally we pass the two strings str1, and str2 as pointers to StrEqual because of the {ref}
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main = type { [81 x i8], [81 x i8] }

    @main_instance = global %main zeroinitializer

    define i8 @StrEqual(i8* %0, i8* %1) {
    entry:
      %StrEqual = alloca i8, align 1
      %o1 = alloca i8*, align 8
      store i8* %0, i8** %o1, align 8
      %o2 = alloca i8*, align 8
      store i8* %1, i8** %o2, align 8
      store i8 0, i8* %StrEqual, align 1
      %StrEqual_ret = load i8, i8* %StrEqual, align 1
      ret i8 %StrEqual_ret
    }

    define void @main(%main* %0) {
    entry:
      %str1 = getelementptr inbounds %main, %main* %0, i32 0, i32 0
      %str2 = getelementptr inbounds %main, %main* %0, i32 0, i32 1
      %1 = bitcast [81 x i8]* %str1 to i8*
      %2 = bitcast [81 x i8]* %str2 to i8*
      %call = call i8 @StrEqual(i8* %1, i8* %2)
      ret void
    }
    "###);
}
