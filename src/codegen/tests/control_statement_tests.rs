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
      store i16 0, ptr %i, align [filtered]
      br i1 true, label %predicate_sle, label %predicate_sge

    predicate_sle:                                    ; preds = %increment, %entry
      %1 = load i16, ptr %i, align [filtered]
      %2 = sext i16 %1 to i32
      %condition = icmp sle i32 %2, 10
      br i1 %condition, label %loop, label %continue

    predicate_sge:                                    ; preds = %increment, %entry
      %3 = load i16, ptr %i, align [filtered]
      %4 = sext i16 %3 to i32
      %condition1 = icmp sge i32 %4, 10
      br i1 %condition1, label %loop, label %continue

    loop:                                             ; preds = %predicate_sge, %predicate_sle
      %load_val = load i16, ptr %val, align [filtered]
      %5 = sext i16 %load_val to i32
      %tmpVar = icmp eq i32 %5, 3
      %6 = zext i1 %tmpVar to i8
      %7 = icmp ne i8 %6, 0
      br i1 %7, label %condition_body, label %else

    increment:                                        ; preds = %continue2
      %8 = load i16, ptr %i, align [filtered]
      %9 = sext i16 %8 to i32
      %next = add i32 1, %9
      %10 = trunc i32 %next to i16
      store i16 %10, ptr %i, align [filtered]
      br i1 true, label %predicate_sle, label %predicate_sge

    continue:                                         ; preds = %predicate_sge, %predicate_sle
      ret void

    condition_body:                                   ; preds = %loop
      store i8 102, ptr %cVar, align [filtered]
      br label %continue2

    else:                                             ; preds = %loop
      %load_val5 = load i16, ptr %val, align [filtered]
      %11 = sext i16 %load_val5 to i32
      %tmpVar6 = icmp eq i32 %11, 5
      %12 = zext i1 %tmpVar6 to i8
      %13 = icmp ne i8 %12, 0
      br i1 %13, label %condition_body7, label %else3

    continue2:                                        ; preds = %continue4, %condition_body
      br label %increment

    condition_body7:                                  ; preds = %else
      store i8 98, ptr %cVar, align [filtered]
      br label %continue4

    else3:                                            ; preds = %else
      store i8 120, ptr %cVar, align [filtered]
      br label %continue4

    continue4:                                        ; preds = %else3, %condition_body7
      br label %continue2
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
      br label %condition_check

    condition_check:                                  ; preds = %entry, %continue2
      br i1 true, label %while_body, label %continue

    while_body:                                       ; preds = %condition_check
      %load_someCon = load i8, ptr %someCon, align [filtered]
      %1 = icmp ne i8 %load_someCon, 0
      %tmpVar = xor i1 %1, true
      br i1 %tmpVar, label %condition_body, label %continue1

    continue:                                         ; preds = %condition_body, %condition_check
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
      br label %condition_check

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
