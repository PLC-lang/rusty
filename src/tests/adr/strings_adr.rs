use crate::test_utils::tests::codegen;

/// # Architecture Design Record: Strings
/// ST supports two types of Strings: UTF8 and UTF16 Strings. Strings are fixed size and
/// stored using i8-arrays for utf8 strings and i16-arrays for utf16 strings.
#[test]
fn declaring_a_string() {
    // two string variables
    let src = r#"
        VAR_GLOBAL
            myUtf8  : STRING[20];
            myUtf16 : WSTRING[20];
        END_VAR
        "#;

    // ... are stored as i8/i16 arrays and get initialized to blank (0)
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    @myUtf8 = global [21 x i8] zeroinitializer, section "var-$RUSTY$myUtf8:s8u21"
    @myUtf16 = global [21 x i16] zeroinitializer, section "var-$RUSTY$myUtf16:s16u21"
    "###);
}

/// rusty treats strings like C-strings (char arrays) so the interoperability
/// with C-libraries is seamless.
#[test]
fn strings_are_terminated_with_0byte() {
    // two strings with an initial value ...
    let src = r#"
        VAR_GLOBAL
            myUtf8  : STRING[5]  := 'Hello';
            myUtf16 : WSTRING[5] := "World";
        END_VAR
        "#;

    // ... get stored as c-like char arrays with 0-terminators
    // ... offer one extra entry (length 21 while only 20 were declared) for a terminator
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    @myUtf8 = global [6 x i8] c"Hello\00", section "var-$RUSTY$myUtf8:s8u6"
    @myUtf16 = global [6 x i16] [i16 87, i16 111, i16 114, i16 108, i16 100, i16 0], section "var-$RUSTY$myUtf16:s16u6"
    "###);
}

/// Strings are aggregate types. This means that passing them to functions and assigning them
/// are expensive operations (when compared to passing ar assigning an INT). Aggregate types
/// are assigned using memcpy.
#[test]
fn assigning_strings() {
    // two strings get assigned ...
    let src = r#"
        PROGRAM prg
            VAR
                a,b : STRING[10];
            END_VAR
             a := b;
        END_PROGRAM
        "#;

    // ... the assignments will be performed as a memcpy
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { [11 x i8], [11 x i8] }

    @prg_instance = global %prg zeroinitializer, section "var-$RUSTY$prg_instance:r2s8u11s8u11"

    define void @prg(%prg* %0) section "fn-$RUSTY$prg:v" {
    entry:
      %a = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %b = getelementptr inbounds %prg, %prg* %0, i32 0, i32 1
      %1 = bitcast [11 x i8]* %a to i8*
      %2 = bitcast [11 x i8]* %b to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 %2, i32 10, i1 false)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
}

/// STRING literals will be generated as global constants, so they can be used as a source of a memcpy
/// operation
#[test]
fn assigning_string_literals() {
    // two nested struct-variables that get initialized ...
    let src = r#"
        PROGRAM prg
            VAR
                a,b : STRING[10];
            END_VAR
             a := 'hello';
             b := 'world';
        END_PROGRAM
        "#;

    // ... will be initialized directly in the variable's definition
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { [11 x i8], [11 x i8] }

    @prg_instance = global %prg zeroinitializer, section "var-$RUSTY$prg_instance:r2s8u11s8u11"
    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"

    define void @prg(%prg* %0) section "fn-$RUSTY$prg:v" {
    entry:
      %a = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %b = getelementptr inbounds %prg, %prg* %0, i32 0, i32 1
      %1 = bitcast [11 x i8]* %a to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false)
      %2 = bitcast [11 x i8]* %b to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %2, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
}
