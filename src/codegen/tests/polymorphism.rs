use plc_util::filtered_assert_snapshot;
use test_utils::codegen;

#[test]
fn simple_overridden_method() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK A
            VAR
                one, two: INT;
            END_VAR

            METHOD foo: INT
                VAR_INPUT
                    in: DINT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK B EXTENDS A
            VAR
                three, four: INT;
            END_VAR

            METHOD foo: INT
                VAR_INPUT
                    in: DINT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceA: A;
                instanceB: B;
                refInstanceA: POINTER TO A;
            END_VAR

            refInstanceA := ADR(instanceA);
            refInstanceA^.foo(5);

            refInstanceA := ADR(instanceB);
            refInstanceA^.foo(10);
        END_FUNCTION
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_A = type { ptr, ptr }
    %A = type { ptr, i16, i16 }
    %__vtable_B = type { ptr, ptr }
    %B = type { %A, i16, i16 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_A__init = unnamed_addr constant %__vtable_A zeroinitializer
    @__A__init = unnamed_addr constant %A zeroinitializer
    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @____vtable_B__init = unnamed_addr constant %__vtable_B zeroinitializer
    @__B__init = unnamed_addr constant %B zeroinitializer
    @__vtable_B_instance = global %__vtable_B zeroinitializer

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %one = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 1
      %two = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 2
      ret void
    }

    define i16 @A__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %one = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 1
      %two = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 2
      %A.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, ptr %in, align 4
      store i16 0, ptr %A.foo, align 2
      %A__foo_ret = load i16, ptr %A.foo, align 2
      ret i16 %A__foo_ret
    }

    define void @B(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %three = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 1
      %four = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 2
      ret void
    }

    define i16 @B__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %three = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 1
      %four = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 2
      %B.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, ptr %in, align 4
      store i16 0, ptr %B.foo, align 2
      %B__foo_ret = load i16, ptr %B.foo, align 2
      ret i16 %B__foo_ret
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %instanceB = alloca %B, align 8
      %refInstanceA = alloca ptr, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %instanceA, ptr align 1 @__A__init, i64 ptrtoint (ptr getelementptr (%A, ptr null, i32 1) to i64), i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %instanceB, ptr align 1 @__B__init, i64 ptrtoint (ptr getelementptr (%B, ptr null, i32 1) to i64), i1 false)
      store ptr null, ptr %refInstanceA, align 8
      call void @__init_a(ptr %instanceA)
      call void @__init_b(ptr %instanceB)
      call void @__user_init_A(ptr %instanceA)
      call void @__user_init_B(ptr %instanceB)
      store ptr %instanceA, ptr %refInstanceA, align 8
      %deref = load ptr, ptr %refInstanceA, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      %deref1 = load ptr, ptr %__vtable, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      %0 = load ptr, ptr %foo, align 8
      %deref2 = load ptr, ptr %refInstanceA, align 8
      %fnptr_call = call i16 %0(ptr %deref2, i32 5)
      store ptr %instanceB, ptr %refInstanceA, align 8
      %deref3 = load ptr, ptr %refInstanceA, align 8
      %__vtable4 = getelementptr inbounds nuw %A, ptr %deref3, i32 0, i32 0
      %deref5 = load ptr, ptr %__vtable4, align 8
      %foo6 = getelementptr inbounds nuw %__vtable_A, ptr %deref5, i32 0, i32 1
      %1 = load ptr, ptr %foo6, align 8
      %deref7 = load ptr, ptr %refInstanceA, align 8
      %fnptr_call8 = call i16 %1(ptr %deref7, i32 10)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    define void @__init___vtable_a(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      store ptr @A, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      store ptr @A__foo, ptr %foo, align 8
      ret void
    }

    define void @__init___vtable_b(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      store ptr @B, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      store ptr @B__foo, ptr %foo, align 8
      ret void
    }

    define void @__init_b(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @__init_a(ptr %__A)
      %deref1 = load ptr, ptr %self, align 8
      %__A2 = getelementptr inbounds nuw %B, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %A, ptr %__A2, i32 0, i32 0
      store ptr @__vtable_B_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_a(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      store ptr @__vtable_A_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_A(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_B(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @__user_init_A(ptr %__A)
      ret void
    }

    define void @__user_init_A(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_B(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_a(ptr @__vtable_A_instance)
      call void @__init___vtable_b(ptr @__vtable_B_instance)
      call void @__user_init___vtable_A(ptr @__vtable_A_instance)
      call void @__user_init___vtable_B(ptr @__vtable_B_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn method_call_within_method() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK A
            METHOD foo: INT
                VAR_INPUT
                    in: DINT;
                END_VAR
            END_METHOD

            METHOD bar
                // foo could be overridden in a child pou, such that when calling bar we must ensure the
                // "correct" foo method is called. In a polymorphic context this means access to the vtable
                foo(5);
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_A = type { ptr, ptr, ptr }
    %A = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_A__init = unnamed_addr constant %__vtable_A zeroinitializer
    @__A__init = unnamed_addr constant %A zeroinitializer
    @__vtable_A_instance = global %__vtable_A zeroinitializer

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @A__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %A.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, ptr %in, align 4
      store i16 0, ptr %A.foo, align 2
      %A__foo_ret = load i16, ptr %A.foo, align 2
      ret i16 %A__foo_ret
    }

    define void @A__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %this, align 8
      %__vtable1 = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      %deref2 = load ptr, ptr %__vtable1, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref2, i32 0, i32 1
      %1 = load ptr, ptr %foo, align 8
      %deref3 = load ptr, ptr %this, align 8
      %fnptr_call = call i16 %1(ptr %deref3, i32 5)
      ret void
    }

    define void @__init___vtable_a(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      store ptr @A, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      store ptr @A__foo, ptr %foo, align 8
      %deref2 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_A, ptr %deref2, i32 0, i32 2
      store ptr @A__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_a(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      store ptr @__vtable_A_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_A(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_A(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_a(ptr @__vtable_A_instance)
      call void @__user_init___vtable_A(ptr @__vtable_A_instance)
      ret void
    }
    "#);
}

#[test]
fn this_is_untouched() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK A
            METHOD foo: INT
                VAR_INPUT
                    in: DINT;
                END_VAR
            END_METHOD

            METHOD bar
            END_METHOD
        END_FUNCTION_BLOCK

        // Only bar overridden, THIS should still point to A.foo
        FUNCTION_BLOCK B EXTENDS A
            METHOD bar
                THIS^.foo(5);
            END_METHOD
        END_FUNCTION_BLOCK

        // Both foo and bar overridden, THIS should point to B.{foo,bar}
        FUNCTION_BLOCK C EXTENDS A
            METHOD foo: INT
                VAR_INPUT
                    in: DINT;
                END_VAR

                THIS^.bar();
            END_METHOD

            METHOD bar
                THIS^.foo(5);
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_A = type { ptr, ptr, ptr }
    %A = type { ptr }
    %__vtable_B = type { ptr, ptr, ptr }
    %B = type { %A }
    %__vtable_C = type { ptr, ptr, ptr }
    %C = type { %A }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_A__init = unnamed_addr constant %__vtable_A zeroinitializer
    @__A__init = unnamed_addr constant %A zeroinitializer
    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @____vtable_B__init = unnamed_addr constant %__vtable_B zeroinitializer
    @__B__init = unnamed_addr constant %B zeroinitializer
    @__vtable_B_instance = global %__vtable_B zeroinitializer
    @____vtable_C__init = unnamed_addr constant %__vtable_C zeroinitializer
    @__C__init = unnamed_addr constant %C zeroinitializer
    @__vtable_C_instance = global %__vtable_C zeroinitializer

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @A__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %A.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, ptr %in, align 4
      store i16 0, ptr %A.foo, align 2
      %A__foo_ret = load i16, ptr %A.foo, align 2
      ret i16 %A__foo_ret
    }

    define void @A__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define void @B(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      ret void
    }

    define void @B__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %this, align 8
      %__A1 = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      %call = call i16 @A__foo(ptr %__A1, i32 5)
      ret void
    }

    define void @C(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %C, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @C__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %C, ptr %0, i32 0, i32 0
      %C.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, ptr %in, align 4
      store i16 0, ptr %C.foo, align 2
      %deref = load ptr, ptr %this, align 8
      call void @C__bar(ptr %deref)
      %C__foo_ret = load i16, ptr %C.foo, align 2
      ret i16 %C__foo_ret
    }

    define void @C__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %C, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %this, align 8
      %call = call i16 @C__foo(ptr %deref, i32 5)
      ret void
    }

    define void @__init___vtable_a(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      store ptr @A, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      store ptr @A__foo, ptr %foo, align 8
      %deref2 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_A, ptr %deref2, i32 0, i32 2
      store ptr @A__bar, ptr %bar, align 8
      ret void
    }

    define void @__init___vtable_b(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      store ptr @B, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      store ptr @A__foo, ptr %foo, align 8
      %deref2 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_A, ptr %deref2, i32 0, i32 2
      store ptr @B__bar, ptr %bar, align 8
      ret void
    }

    define void @__init___vtable_c(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      store ptr @C, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      store ptr @C__foo, ptr %foo, align 8
      %deref2 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_A, ptr %deref2, i32 0, i32 2
      store ptr @C__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_a(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      store ptr @__vtable_A_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_b(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @__init_a(ptr %__A)
      %deref1 = load ptr, ptr %self, align 8
      %__A2 = getelementptr inbounds nuw %B, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %A, ptr %__A2, i32 0, i32 0
      store ptr @__vtable_B_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_c(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @__init_a(ptr %__A)
      %deref1 = load ptr, ptr %self, align 8
      %__A2 = getelementptr inbounds nuw %B, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %A, ptr %__A2, i32 0, i32 0
      store ptr @__vtable_C_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_C(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @__user_init_A(ptr %__A)
      ret void
    }

    define void @__user_init_A(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_A(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_B(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @__user_init_A(ptr %__A)
      ret void
    }

    define void @__user_init___vtable_C(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_B(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_a(ptr @__vtable_A_instance)
      call void @__init___vtable_b(ptr @__vtable_B_instance)
      call void @__init___vtable_c(ptr @__vtable_C_instance)
      call void @__user_init___vtable_A(ptr @__vtable_A_instance)
      call void @__user_init___vtable_B(ptr @__vtable_B_instance)
      call void @__user_init___vtable_C(ptr @__vtable_C_instance)
      ret void
    }
    "#);
}

#[test]
fn super_is_untouched() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK A
            METHOD foo: INT
                VAR_INPUT
                    in: DINT;
                END_VAR
            END_METHOD

            METHOD bar
            END_METHOD
        END_FUNCTION_BLOCK

        // Both foo and bar overridden, a `SUPER^.{foo,bar}` call should still call the A's methods
        FUNCTION_BLOCK B EXTENDS A
            METHOD foo: INT
                VAR_INPUT
                    in: DINT;
                END_VAR

                SUPER^.foo(5);
                SUPER^.bar();
            END_METHOD

            METHOD bar
                SUPER^.foo(5);
                SUPER^.bar();
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_A = type { ptr, ptr, ptr }
    %A = type { ptr }
    %__vtable_B = type { ptr, ptr, ptr }
    %B = type { %A }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_A__init = unnamed_addr constant %__vtable_A zeroinitializer
    @__A__init = unnamed_addr constant %A zeroinitializer
    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @____vtable_B__init = unnamed_addr constant %__vtable_B zeroinitializer
    @__B__init = unnamed_addr constant %B zeroinitializer
    @__vtable_B_instance = global %__vtable_B zeroinitializer

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @A__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %A.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, ptr %in, align 4
      store i16 0, ptr %A.foo, align 2
      %A__foo_ret = load i16, ptr %A.foo, align 2
      ret i16 %A__foo_ret
    }

    define void @A__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define void @B(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @B__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %B.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, ptr %in, align 4
      store i16 0, ptr %B.foo, align 2
      %call = call i16 @A__foo(ptr %__A, i32 5)
      call void @A__bar(ptr %__A)
      %B__foo_ret = load i16, ptr %B.foo, align 2
      ret i16 %B__foo_ret
    }

    define void @B__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %call = call i16 @A__foo(ptr %__A, i32 5)
      call void @A__bar(ptr %__A)
      ret void
    }

    define void @__init___vtable_a(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      store ptr @A, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      store ptr @A__foo, ptr %foo, align 8
      %deref2 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_A, ptr %deref2, i32 0, i32 2
      store ptr @A__bar, ptr %bar, align 8
      ret void
    }

    define void @__init___vtable_b(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      store ptr @B, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      store ptr @B__foo, ptr %foo, align 8
      %deref2 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_A, ptr %deref2, i32 0, i32 2
      store ptr @B__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_a(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      store ptr @__vtable_A_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_b(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @__init_a(ptr %__A)
      %deref1 = load ptr, ptr %self, align 8
      %__A2 = getelementptr inbounds nuw %B, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %A, ptr %__A2, i32 0, i32 0
      store ptr @__vtable_B_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_A(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_B(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @__user_init_A(ptr %__A)
      ret void
    }

    define void @__user_init_A(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_B(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_a(ptr @__vtable_A_instance)
      call void @__init___vtable_b(ptr @__vtable_B_instance)
      call void @__user_init___vtable_A(ptr @__vtable_A_instance)
      call void @__user_init___vtable_B(ptr @__vtable_B_instance)
      ret void
    }
    "#);
}
