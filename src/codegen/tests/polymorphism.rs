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
    %__vtable_B = type { ptr, ptr }
    %A = type { ptr, i16, i16 }
    %B = type { %A, i16, i16 }

    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @__vtable_B_instance = global %__vtable_B zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %one = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 1
      %two = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 2
      ret void
    }

    define i16 @A__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %one = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 1
      %two = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 2
      %A.foo = alloca i16, align [filtered]
      %in = alloca i32, align [filtered]
      store i32 %1, ptr %in, align [filtered]
      store i16 0, ptr %A.foo, align [filtered]
      %A__foo_ret = load i16, ptr %A.foo, align [filtered]
      ret i16 %A__foo_ret
    }

    define void @B(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %three = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 1
      %four = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 2
      ret void
    }

    define i16 @B__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %three = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 1
      %four = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 2
      %B.foo = alloca i16, align [filtered]
      %in = alloca i32, align [filtered]
      store i32 %1, ptr %in, align [filtered]
      store i16 0, ptr %B.foo, align [filtered]
      %B__foo_ret = load i16, ptr %B.foo, align [filtered]
      ret i16 %B__foo_ret
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align [filtered]
      %instanceB = alloca %B, align [filtered]
      %refInstanceA = alloca ptr, align [filtered]
      call void @llvm.memset.p0.i64(ptr align [filtered] %instanceA, i8 0, i64 ptrtoint (ptr getelementptr (%A, ptr null, i32 1) to i64), i1 false)
      call void @llvm.memset.p0.i64(ptr align [filtered] %instanceB, i8 0, i64 ptrtoint (ptr getelementptr (%B, ptr null, i32 1) to i64), i1 false)
      store ptr null, ptr %refInstanceA, align [filtered]
      call void @A__ctor(ptr %instanceA)
      call void @B__ctor(ptr %instanceB)
      call void @__main_refInstanceA__ctor(ptr %refInstanceA)
      store ptr %instanceA, ptr %refInstanceA, align [filtered]
      %deref = load ptr, ptr %refInstanceA, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      %deref1 = load ptr, ptr %__vtable, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 1
      %0 = load ptr, ptr %foo, align [filtered]
      %deref2 = load ptr, ptr %refInstanceA, align [filtered]
      %fnptr_call = call i16 %0(ptr %deref2, i32 5)
      store ptr %instanceB, ptr %refInstanceA, align [filtered]
      %deref3 = load ptr, ptr %refInstanceA, align [filtered]
      %__vtable4 = getelementptr inbounds nuw %A, ptr %deref3, i32 0, i32 0
      %deref5 = load ptr, ptr %__vtable4, align [filtered]
      %foo6 = getelementptr inbounds nuw %__vtable_A, ptr %deref5, i32 0, i32 1
      %1 = load ptr, ptr %foo6, align [filtered]
      %deref7 = load ptr, ptr %refInstanceA, align [filtered]
      %fnptr_call8 = call i16 %1(ptr %deref7, i32 10)
      ret void
    }

    define void @A__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      call void @__A___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %A, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_A_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @B__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @A__ctor(ptr %__A)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__A2 = getelementptr inbounds nuw %B, ptr %deref1, i32 0, i32 0
      call void @A__ctor(ptr %__A2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__A4 = getelementptr inbounds nuw %B, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %A, ptr %__A4, i32 0, i32 0
      store ptr @__vtable_B_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__main_refInstanceA__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_A__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      call void @____vtable_A___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 0
      store ptr @A, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref3, i32 0, i32 1
      call void @____vtable_A_foo__ctor(ptr %foo)
      %deref4 = load ptr, ptr %self, align [filtered]
      %foo5 = getelementptr inbounds nuw %__vtable_A, ptr %deref4, i32 0, i32 1
      store ptr @A__foo, ptr %foo5, align [filtered]
      ret void
    }

    define void @__vtable_B__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_B, ptr %deref, i32 0, i32 0
      call void @____vtable_B___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_B, ptr %deref1, i32 0, i32 0
      store ptr @B, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_B, ptr %deref3, i32 0, i32 1
      call void @____vtable_B_foo__ctor(ptr %foo)
      %deref4 = load ptr, ptr %self, align [filtered]
      %foo5 = getelementptr inbounds nuw %__vtable_B, ptr %deref4, i32 0, i32 1
      store ptr @B__foo, ptr %foo5, align [filtered]
      ret void
    }

    define void @__A___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_B___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_B_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_A__ctor(ptr @__vtable_A_instance)
      call void @__vtable_B__ctor(ptr @__vtable_B_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
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

    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @A__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %A.foo = alloca i16, align [filtered]
      %in = alloca i32, align [filtered]
      store i32 %1, ptr %in, align [filtered]
      store i16 0, ptr %A.foo, align [filtered]
      %A__foo_ret = load i16, ptr %A.foo, align [filtered]
      ret i16 %A__foo_ret
    }

    define void @A__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %this, align [filtered]
      %__vtable1 = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      %deref2 = load ptr, ptr %__vtable1, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref2, i32 0, i32 1
      %1 = load ptr, ptr %foo, align [filtered]
      %deref3 = load ptr, ptr %this, align [filtered]
      %fnptr_call = call i16 %1(ptr %deref3, i32 5)
      ret void
    }

    define void @A__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      call void @__A___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %A, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_A_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__vtable_A__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      call void @____vtable_A___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 0
      store ptr @A, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref3, i32 0, i32 1
      call void @____vtable_A_foo__ctor(ptr %foo)
      %deref4 = load ptr, ptr %self, align [filtered]
      %foo5 = getelementptr inbounds nuw %__vtable_A, ptr %deref4, i32 0, i32 1
      store ptr @A__foo, ptr %foo5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_A, ptr %deref6, i32 0, i32 2
      call void @____vtable_A_bar__ctor(ptr %bar)
      %deref7 = load ptr, ptr %self, align [filtered]
      %bar8 = getelementptr inbounds nuw %__vtable_A, ptr %deref7, i32 0, i32 2
      store ptr @A__bar, ptr %bar8, align [filtered]
      ret void
    }

    define void @__A___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_A__ctor(ptr @__vtable_A_instance)
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
    %__vtable_B = type { ptr, ptr, ptr }
    %__vtable_C = type { ptr, ptr, ptr }
    %A = type { ptr }
    %B = type { %A }
    %C = type { %A }

    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @__vtable_B_instance = global %__vtable_B zeroinitializer
    @__vtable_C_instance = global %__vtable_C zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @A__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %A.foo = alloca i16, align [filtered]
      %in = alloca i32, align [filtered]
      store i32 %1, ptr %in, align [filtered]
      store i16 0, ptr %A.foo, align [filtered]
      %A__foo_ret = load i16, ptr %A.foo, align [filtered]
      ret i16 %A__foo_ret
    }

    define void @A__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define void @B(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      ret void
    }

    define void @B__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %this, align [filtered]
      %__A1 = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      %call = call i16 @A__foo(ptr %__A1, i32 5)
      ret void
    }

    define void @C(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %C, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @C__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %C, ptr %0, i32 0, i32 0
      %C.foo = alloca i16, align [filtered]
      %in = alloca i32, align [filtered]
      store i32 %1, ptr %in, align [filtered]
      store i16 0, ptr %C.foo, align [filtered]
      %deref = load ptr, ptr %this, align [filtered]
      call void @C__bar(ptr %deref)
      %C__foo_ret = load i16, ptr %C.foo, align [filtered]
      ret i16 %C__foo_ret
    }

    define void @C__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %C, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %this, align [filtered]
      %call = call i16 @C__foo(ptr %deref, i32 5)
      ret void
    }

    define void @A__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      call void @__A___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %A, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_A_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @B__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @A__ctor(ptr %__A)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__A2 = getelementptr inbounds nuw %B, ptr %deref1, i32 0, i32 0
      call void @A__ctor(ptr %__A2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__A4 = getelementptr inbounds nuw %B, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %A, ptr %__A4, i32 0, i32 0
      store ptr @__vtable_B_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @C__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__A = getelementptr inbounds nuw %C, ptr %deref, i32 0, i32 0
      call void @A__ctor(ptr %__A)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__A2 = getelementptr inbounds nuw %C, ptr %deref1, i32 0, i32 0
      call void @A__ctor(ptr %__A2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__A4 = getelementptr inbounds nuw %C, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %A, ptr %__A4, i32 0, i32 0
      store ptr @__vtable_C_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_A__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      call void @____vtable_A___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 0
      store ptr @A, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref3, i32 0, i32 1
      call void @____vtable_A_foo__ctor(ptr %foo)
      %deref4 = load ptr, ptr %self, align [filtered]
      %foo5 = getelementptr inbounds nuw %__vtable_A, ptr %deref4, i32 0, i32 1
      store ptr @A__foo, ptr %foo5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_A, ptr %deref6, i32 0, i32 2
      call void @____vtable_A_bar__ctor(ptr %bar)
      %deref7 = load ptr, ptr %self, align [filtered]
      %bar8 = getelementptr inbounds nuw %__vtable_A, ptr %deref7, i32 0, i32 2
      store ptr @A__bar, ptr %bar8, align [filtered]
      ret void
    }

    define void @__vtable_B__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_B, ptr %deref, i32 0, i32 0
      call void @____vtable_B___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_B, ptr %deref1, i32 0, i32 0
      store ptr @B, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_B, ptr %deref3, i32 0, i32 1
      call void @____vtable_B_foo__ctor(ptr %foo)
      %deref4 = load ptr, ptr %self, align [filtered]
      %foo5 = getelementptr inbounds nuw %__vtable_B, ptr %deref4, i32 0, i32 1
      store ptr @A__foo, ptr %foo5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_B, ptr %deref6, i32 0, i32 2
      call void @____vtable_B_bar__ctor(ptr %bar)
      %deref7 = load ptr, ptr %self, align [filtered]
      %bar8 = getelementptr inbounds nuw %__vtable_B, ptr %deref7, i32 0, i32 2
      store ptr @B__bar, ptr %bar8, align [filtered]
      ret void
    }

    define void @__vtable_C__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_C, ptr %deref, i32 0, i32 0
      call void @____vtable_C___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_C, ptr %deref1, i32 0, i32 0
      store ptr @C, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_C, ptr %deref3, i32 0, i32 1
      call void @____vtable_C_foo__ctor(ptr %foo)
      %deref4 = load ptr, ptr %self, align [filtered]
      %foo5 = getelementptr inbounds nuw %__vtable_C, ptr %deref4, i32 0, i32 1
      store ptr @C__foo, ptr %foo5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_C, ptr %deref6, i32 0, i32 2
      call void @____vtable_C_bar__ctor(ptr %bar)
      %deref7 = load ptr, ptr %self, align [filtered]
      %bar8 = getelementptr inbounds nuw %__vtable_C, ptr %deref7, i32 0, i32 2
      store ptr @C__bar, ptr %bar8, align [filtered]
      ret void
    }

    define void @__A___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_B___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_B_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_B_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_C___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_C_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_C_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_A__ctor(ptr @__vtable_A_instance)
      call void @__vtable_B__ctor(ptr @__vtable_B_instance)
      call void @__vtable_C__ctor(ptr @__vtable_C_instance)
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
    %__vtable_B = type { ptr, ptr, ptr }
    %A = type { ptr }
    %B = type { %A }

    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @__vtable_B_instance = global %__vtable_B zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @A__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %A.foo = alloca i16, align [filtered]
      %in = alloca i32, align [filtered]
      store i32 %1, ptr %in, align [filtered]
      store i16 0, ptr %A.foo, align [filtered]
      %A__foo_ret = load i16, ptr %A.foo, align [filtered]
      ret i16 %A__foo_ret
    }

    define void @A__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      ret void
    }

    define void @B(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @B__foo(ptr %0, i32 %1) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %B.foo = alloca i16, align [filtered]
      %in = alloca i32, align [filtered]
      store i32 %1, ptr %in, align [filtered]
      store i16 0, ptr %B.foo, align [filtered]
      %call = call i16 @A__foo(ptr %__A, i32 5)
      call void @A__bar(ptr %__A)
      %B__foo_ret = load i16, ptr %B.foo, align [filtered]
      ret i16 %B__foo_ret
    }

    define void @B__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %0, i32 0, i32 0
      %call = call i16 @A__foo(ptr %__A, i32 5)
      call void @A__bar(ptr %__A)
      ret void
    }

    define void @A__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %A, ptr %deref, i32 0, i32 0
      call void @__A___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %A, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_A_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @B__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__A = getelementptr inbounds nuw %B, ptr %deref, i32 0, i32 0
      call void @A__ctor(ptr %__A)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__A2 = getelementptr inbounds nuw %B, ptr %deref1, i32 0, i32 0
      call void @A__ctor(ptr %__A2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__A4 = getelementptr inbounds nuw %B, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %A, ptr %__A4, i32 0, i32 0
      store ptr @__vtable_B_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_A__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_A, ptr %deref, i32 0, i32 0
      call void @____vtable_A___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_A, ptr %deref1, i32 0, i32 0
      store ptr @A, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_A, ptr %deref3, i32 0, i32 1
      call void @____vtable_A_foo__ctor(ptr %foo)
      %deref4 = load ptr, ptr %self, align [filtered]
      %foo5 = getelementptr inbounds nuw %__vtable_A, ptr %deref4, i32 0, i32 1
      store ptr @A__foo, ptr %foo5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_A, ptr %deref6, i32 0, i32 2
      call void @____vtable_A_bar__ctor(ptr %bar)
      %deref7 = load ptr, ptr %self, align [filtered]
      %bar8 = getelementptr inbounds nuw %__vtable_A, ptr %deref7, i32 0, i32 2
      store ptr @A__bar, ptr %bar8, align [filtered]
      ret void
    }

    define void @__vtable_B__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_B, ptr %deref, i32 0, i32 0
      call void @____vtable_B___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_B, ptr %deref1, i32 0, i32 0
      store ptr @B, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %foo = getelementptr inbounds nuw %__vtable_B, ptr %deref3, i32 0, i32 1
      call void @____vtable_B_foo__ctor(ptr %foo)
      %deref4 = load ptr, ptr %self, align [filtered]
      %foo5 = getelementptr inbounds nuw %__vtable_B, ptr %deref4, i32 0, i32 1
      store ptr @B__foo, ptr %foo5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_B, ptr %deref6, i32 0, i32 2
      call void @____vtable_B_bar__ctor(ptr %bar)
      %deref7 = load ptr, ptr %self, align [filtered]
      %bar8 = getelementptr inbounds nuw %__vtable_B, ptr %deref7, i32 0, i32 2
      store ptr @B__bar, ptr %bar8, align [filtered]
      ret void
    }

    define void @__A___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_A_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_B___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_B_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_B_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_A__ctor(ptr @__vtable_A_instance)
      call void @__vtable_B__ctor(ptr @__vtable_B_instance)
      ret void
    }
    "#);
}
