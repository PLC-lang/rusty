use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

#[test]
fn elseif_is_lowered_to_else_with_nested_if() {
    let result = codegen(
        r#"
        PROGRAM mainProg
        VAR
            val : INT;
            cVar : CHAR;
        END_VAR

        val := 5;
        cVar := 'n';

        IF val = 3 THEN
            // Fizz
            cVar := 'f';
        ELSIF val = 5 THEN
            // Buzz
            cVar := 'b';
        ELSE
            cVar := 'x';
        END_IF
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { i16, i8 }

    @mainProg_instance = global %mainProg zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [2 x i8] c"b\00"
    @utf08_literal_1 = private unnamed_addr constant [2 x i8] c"f\00"
    @utf08_literal_2 = private unnamed_addr constant [2 x i8] c"n\00"
    @utf08_literal_3 = private unnamed_addr constant [2 x i8] c"x\00"

    define void @mainProg(ptr %0) {
    entry:
      %val = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 0
      %cVar = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 1
      store i16 5, ptr %val, align [filtered]
      store i8 110, ptr %cVar, align [filtered]
      %load_val = load i16, ptr %val, align [filtered]
      %1 = sext i16 %load_val to i32
      %tmpVar = icmp eq i32 %1, 3
      %2 = zext i1 %tmpVar to i8
      %3 = icmp ne i8 %2, 0
      br i1 %3, label %condition_body, label %else

    condition_body:                                   ; preds = %entry
      store i8 102, ptr %cVar, align [filtered]
      br label %continue

    else:                                             ; preds = %entry
      %load_val3 = load i16, ptr %val, align [filtered]
      %4 = sext i16 %load_val3 to i32
      %tmpVar4 = icmp eq i32 %4, 5
      %5 = zext i1 %tmpVar4 to i8
      %6 = icmp ne i8 %5, 0
      br i1 %6, label %condition_body5, label %else1

    continue:                                         ; preds = %continue2, %condition_body
      ret void

    condition_body5:                                  ; preds = %else
      store i8 98, ptr %cVar, align [filtered]
      br label %continue2

    else1:                                            ; preds = %else
      store i8 120, ptr %cVar, align [filtered]
      br label %continue2

    continue2:                                        ; preds = %else1, %condition_body5
      br label %continue
    }
    "#);
}

#[test]
fn elseif_is_lowered_to_else_with_nested_if_even_if_no_else_is_present() {
    let result = codegen(
        r#"
        PROGRAM mainProg
        VAR
            val : INT;
            cVar : CHAR;
        END_VAR

        val := 5;
        cVar := 'n';

        IF val = 3 THEN
            // Fizz
            cVar := 'f';
        ELSIF val = 5 THEN
            // Buzz
            cVar := 'b';
        END_IF
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { i16, i8 }

    @mainProg_instance = global %mainProg zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [2 x i8] c"b\00"
    @utf08_literal_1 = private unnamed_addr constant [2 x i8] c"f\00"
    @utf08_literal_2 = private unnamed_addr constant [2 x i8] c"n\00"

    define void @mainProg(ptr %0) {
    entry:
      %val = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 0
      %cVar = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 1
      store i16 5, ptr %val, align [filtered]
      store i8 110, ptr %cVar, align [filtered]
      %load_val = load i16, ptr %val, align [filtered]
      %1 = sext i16 %load_val to i32
      %tmpVar = icmp eq i32 %1, 3
      %2 = zext i1 %tmpVar to i8
      %3 = icmp ne i8 %2, 0
      br i1 %3, label %condition_body, label %else

    condition_body:                                   ; preds = %entry
      store i8 102, ptr %cVar, align [filtered]
      br label %continue

    else:                                             ; preds = %entry
      %load_val2 = load i16, ptr %val, align [filtered]
      %4 = sext i16 %load_val2 to i32
      %tmpVar3 = icmp eq i32 %4, 5
      %5 = zext i1 %tmpVar3 to i8
      %6 = icmp ne i8 %5, 0
      br i1 %6, label %condition_body4, label %continue1

    continue:                                         ; preds = %continue1, %condition_body
      ret void

    condition_body4:                                  ; preds = %else
      store i8 98, ptr %cVar, align [filtered]
      br label %continue1

    continue1:                                        ; preds = %condition_body4, %else
      br label %continue
    }
    "#);
}

#[test]
fn elseif_is_lowered_to_else_with_nested_if_when_prenested_in_if() {
    let result = codegen(
        r#"
        PROGRAM mainProg
        VAR
            val : INT;
            cVar : CHAR;
        END_VAR

        val := 5;
        cVar := 'n';

        IF val = 4 THEN
            cVar := 'a';
        ELSE
            IF val = 3 THEN
                cVar := 'f';
            ELSIF val = 5 THEN
                cVar := 'b';
            ELSE
                cVar := 'x';
            END_IF
        END_IF
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { i16, i8 }

    @mainProg_instance = global %mainProg zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [2 x i8] c"a\00"
    @utf08_literal_1 = private unnamed_addr constant [2 x i8] c"b\00"
    @utf08_literal_2 = private unnamed_addr constant [2 x i8] c"f\00"
    @utf08_literal_3 = private unnamed_addr constant [2 x i8] c"n\00"
    @utf08_literal_4 = private unnamed_addr constant [2 x i8] c"x\00"

    define void @mainProg(ptr %0) {
    entry:
      %val = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 0
      %cVar = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 1
      store i16 5, ptr %val, align [filtered]
      store i8 110, ptr %cVar, align [filtered]
      %load_val = load i16, ptr %val, align [filtered]
      %1 = sext i16 %load_val to i32
      %tmpVar = icmp eq i32 %1, 4
      %2 = zext i1 %tmpVar to i8
      %3 = icmp ne i8 %2, 0
      br i1 %3, label %condition_body, label %else

    condition_body:                                   ; preds = %entry
      store i8 97, ptr %cVar, align [filtered]
      br label %continue

    else:                                             ; preds = %entry
      %load_val3 = load i16, ptr %val, align [filtered]
      %4 = sext i16 %load_val3 to i32
      %tmpVar4 = icmp eq i32 %4, 3
      %5 = zext i1 %tmpVar4 to i8
      %6 = icmp ne i8 %5, 0
      br i1 %6, label %condition_body5, label %else1

    continue:                                         ; preds = %continue2, %condition_body
      ret void

    condition_body5:                                  ; preds = %else
      store i8 102, ptr %cVar, align [filtered]
      br label %continue2

    else1:                                            ; preds = %else
      %load_val8 = load i16, ptr %val, align [filtered]
      %7 = sext i16 %load_val8 to i32
      %tmpVar9 = icmp eq i32 %7, 5
      %8 = zext i1 %tmpVar9 to i8
      %9 = icmp ne i8 %8, 0
      br i1 %9, label %condition_body10, label %else6

    continue2:                                        ; preds = %continue7, %condition_body5
      br label %continue

    condition_body10:                                 ; preds = %else1
      store i8 98, ptr %cVar, align [filtered]
      br label %continue7

    else6:                                            ; preds = %else1
      store i8 120, ptr %cVar, align [filtered]
      br label %continue7

    continue7:                                        ; preds = %else6, %condition_body10
      br label %continue2
    }
    "#);
}

#[test]
fn elseif_is_lowered_to_else_with_nested_if_inside_for_loop() {
    let result = codegen(
        r#"
        PROGRAM mainProg
        VAR
            i : INT;
            val : INT;
            cVar : CHAR;
        END_VAR

        val := 5;
        cVar := 'n';

        FOR i := 0 TO 10 DO
            IF val = 3 THEN
                cVar := 'f';
            ELSIF val = 5 THEN
                cVar := 'b';
            ELSE
                cVar := 'x';
            END_IF
        END_FOR
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { i16, i16, i8 }

    @mainProg_instance = global %mainProg zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [2 x i8] c"b\00"
    @utf08_literal_1 = private unnamed_addr constant [2 x i8] c"f\00"
    @utf08_literal_2 = private unnamed_addr constant [2 x i8] c"n\00"
    @utf08_literal_3 = private unnamed_addr constant [2 x i8] c"x\00"

    define void @mainProg(ptr %0) {
    entry:
      %i = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 0
      %val = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 1
      %cVar = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 2
      store i16 5, ptr %val, align [filtered]
      store i8 110, ptr %cVar, align [filtered]
      %__ran_once_0 = alloca i8, align [filtered]
      store i8 0, ptr %__ran_once_0, align [filtered]
      %__is_incrementing_0 = alloca i8, align [filtered]
      store i8 0, ptr %__is_incrementing_0, align [filtered]
      %__past_end_0 = alloca i8, align [filtered]
      store i8 0, ptr %__past_end_0, align [filtered]
      %__next_0 = alloca i64, align [filtered]
      store i64 0, ptr %__next_0, align [filtered]
      store i16 0, ptr %i, align [filtered]
      store i8 1, ptr %__is_incrementing_0, align [filtered]
      br label %while_body

    while_body:                                       ; preds = %continue21, %entry
      %load___ran_once_0 = load i8, ptr %__ran_once_0, align [filtered]
      %1 = icmp ne i8 %load___ran_once_0, 0
      br i1 %1, label %condition_body, label %else

    continue:                                         ; preds = %condition_body19
      ret void

    condition_body:                                   ; preds = %while_body
      %load_i = load i16, ptr %i, align [filtered]
      %2 = sext i16 %load_i to i64
      store i64 %2, ptr %__next_0, align [filtered]
      %load___next_0 = load i64, ptr %__next_0, align [filtered]
      %tmpVar = add i64 %load___next_0, 1
      store i64 %tmpVar, ptr %__next_0, align [filtered]
      %load___is_incrementing_0 = load i8, ptr %__is_incrementing_0, align [filtered]
      %3 = icmp ne i8 %load___is_incrementing_0, 0
      br i1 %3, label %condition_body4, label %else2

    else:                                             ; preds = %while_body
      %load___is_incrementing_012 = load i8, ptr %__is_incrementing_0, align [filtered]
      %4 = icmp ne i8 %load___is_incrementing_012, 0
      br i1 %4, label %condition_body13, label %else10

    continue1:                                        ; preds = %continue11, %continue3
      store i8 1, ptr %__ran_once_0, align [filtered]
      %load___past_end_0 = load i8, ptr %__past_end_0, align [filtered]
      %5 = icmp ne i8 %load___past_end_0, 0
      br i1 %5, label %condition_body19, label %continue18

    condition_body4:                                  ; preds = %condition_body
      %load___next_05 = load i64, ptr %__next_0, align [filtered]
      %tmpVar6 = icmp sgt i64 %load___next_05, 10
      %6 = zext i1 %tmpVar6 to i8
      store i8 %6, ptr %__past_end_0, align [filtered]
      br label %continue3

    else2:                                            ; preds = %condition_body
      %load___next_07 = load i64, ptr %__next_0, align [filtered]
      %tmpVar8 = icmp slt i64 %load___next_07, 10
      %7 = zext i1 %tmpVar8 to i8
      store i8 %7, ptr %__past_end_0, align [filtered]
      br label %continue3

    continue3:                                        ; preds = %else2, %condition_body4
      %load___next_09 = load i64, ptr %__next_0, align [filtered]
      %8 = trunc i64 %load___next_09 to i16
      store i16 %8, ptr %i, align [filtered]
      br label %continue1

    condition_body13:                                 ; preds = %else
      %load_i14 = load i16, ptr %i, align [filtered]
      %9 = sext i16 %load_i14 to i32
      %tmpVar15 = icmp sgt i32 %9, 10
      %10 = zext i1 %tmpVar15 to i8
      store i8 %10, ptr %__past_end_0, align [filtered]
      br label %continue11

    else10:                                           ; preds = %else
      %load_i16 = load i16, ptr %i, align [filtered]
      %11 = sext i16 %load_i16 to i32
      %tmpVar17 = icmp slt i32 %11, 10
      %12 = zext i1 %tmpVar17 to i8
      store i8 %12, ptr %__past_end_0, align [filtered]
      br label %continue11

    continue11:                                       ; preds = %else10, %condition_body13
      br label %continue1

    condition_body19:                                 ; preds = %continue1
      br label %continue

    buffer_block:                                     ; No predecessors!
      br label %continue18

    continue18:                                       ; preds = %buffer_block, %continue1
      %load_val = load i16, ptr %val, align [filtered]
      %13 = sext i16 %load_val to i32
      %tmpVar22 = icmp eq i32 %13, 3
      %14 = zext i1 %tmpVar22 to i8
      %15 = icmp ne i8 %14, 0
      br i1 %15, label %condition_body23, label %else20

    condition_body23:                                 ; preds = %continue18
      store i8 102, ptr %cVar, align [filtered]
      br label %continue21

    else20:                                           ; preds = %continue18
      %load_val26 = load i16, ptr %val, align [filtered]
      %16 = sext i16 %load_val26 to i32
      %tmpVar27 = icmp eq i32 %16, 5
      %17 = zext i1 %tmpVar27 to i8
      %18 = icmp ne i8 %17, 0
      br i1 %18, label %condition_body28, label %else24

    continue21:                                       ; preds = %continue25, %condition_body23
      br label %while_body

    condition_body28:                                 ; preds = %else20
      store i8 98, ptr %cVar, align [filtered]
      br label %continue25

    else24:                                           ; preds = %else20
      store i8 120, ptr %cVar, align [filtered]
      br label %continue25

    continue25:                                       ; preds = %else24, %condition_body28
      br label %continue21
    }
    "#);
}

#[test]
fn elseif_is_lowered_to_else_with_nested_if_inside_while_loop() {
    let result = codegen(
        r#"
        PROGRAM mainProg
        VAR
            i : INT;
            breakOut: INT;
            val : INT;
            cVar : CHAR;
            someCon : BOOL;
        END_VAR

        val := 5;
        cVar := 'n';
        someCon := TRUE;
        breakOut := 0;

        WHILE someCon DO
            IF val = 3 THEN
                cVar := 'f';
                someCon := FALSE;
            ELSIF val = 5 THEN
                cVar := 'b';
                someCon := FALSE;
            ELSE
                cVar := 'x';
                IF breakOut = 10 THEN
                    someCon := FALSE;
                END_IF
                breakOut := breakOut + 1;
            END_IF
        END_WHILE
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { i16, i16, i16, i8, i8 }

    @mainProg_instance = global %mainProg zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [2 x i8] c"b\00"
    @utf08_literal_1 = private unnamed_addr constant [2 x i8] c"f\00"
    @utf08_literal_2 = private unnamed_addr constant [2 x i8] c"n\00"
    @utf08_literal_3 = private unnamed_addr constant [2 x i8] c"x\00"

    define void @mainProg(ptr %0) {
    entry:
      %i = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 0
      %breakOut = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 1
      %val = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 2
      %cVar = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 3
      %someCon = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 4
      store i16 5, ptr %val, align [filtered]
      store i8 110, ptr %cVar, align [filtered]
      store i8 1, ptr %someCon, align [filtered]
      store i16 0, ptr %breakOut, align [filtered]
      br label %while_body

    while_body:                                       ; preds = %continue2, %entry
      %load_someCon = load i8, ptr %someCon, align [filtered]
      %1 = icmp ne i8 %load_someCon, 0
      %tmpVar = xor i1 %1, true
      br i1 %tmpVar, label %condition_body, label %continue1

    continue:                                         ; preds = %condition_body
      ret void

    condition_body:                                   ; preds = %while_body
      br label %continue

    buffer_block:                                     ; No predecessors!
      br label %continue1

    continue1:                                        ; preds = %buffer_block, %while_body
      %load_val = load i16, ptr %val, align [filtered]
      %2 = sext i16 %load_val to i32
      %tmpVar3 = icmp eq i32 %2, 3
      %3 = zext i1 %tmpVar3 to i8
      %4 = icmp ne i8 %3, 0
      br i1 %4, label %condition_body4, label %else

    condition_body4:                                  ; preds = %continue1
      store i8 102, ptr %cVar, align [filtered]
      store i8 0, ptr %someCon, align [filtered]
      br label %continue2

    else:                                             ; preds = %continue1
      %load_val7 = load i16, ptr %val, align [filtered]
      %5 = sext i16 %load_val7 to i32
      %tmpVar8 = icmp eq i32 %5, 5
      %6 = zext i1 %tmpVar8 to i8
      %7 = icmp ne i8 %6, 0
      br i1 %7, label %condition_body9, label %else5

    continue2:                                        ; preds = %continue6, %condition_body4
      br label %while_body

    condition_body9:                                  ; preds = %else
      store i8 98, ptr %cVar, align [filtered]
      store i8 0, ptr %someCon, align [filtered]
      br label %continue6

    else5:                                            ; preds = %else
      store i8 120, ptr %cVar, align [filtered]
      %load_breakOut = load i16, ptr %breakOut, align [filtered]
      %8 = sext i16 %load_breakOut to i32
      %tmpVar11 = icmp eq i32 %8, 10
      %9 = zext i1 %tmpVar11 to i8
      %10 = icmp ne i8 %9, 0
      br i1 %10, label %condition_body12, label %continue10

    continue6:                                        ; preds = %continue10, %condition_body9
      br label %continue2

    condition_body12:                                 ; preds = %else5
      store i8 0, ptr %someCon, align [filtered]
      br label %continue10

    continue10:                                       ; preds = %condition_body12, %else5
      %load_breakOut13 = load i16, ptr %breakOut, align [filtered]
      %11 = sext i16 %load_breakOut13 to i32
      %tmpVar14 = add i32 %11, 1
      %12 = trunc i32 %tmpVar14 to i16
      store i16 %12, ptr %breakOut, align [filtered]
      br label %continue6
    }
    "#);
}
