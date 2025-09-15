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

    %__vtable_A = type { void (%A*)*, i16 (%A*, i32)* }
    %A = type { i32*, i16, i16 }
    %__vtable_B = type { void (%B*)*, i16 (%B*, i32)* }
    %B = type { %A, i16, i16 }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_A__init = unnamed_addr constant %__vtable_A zeroinitializer
    @__A__init = unnamed_addr constant %A zeroinitializer
    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @____vtable_B__init = unnamed_addr constant %__vtable_B zeroinitializer
    @__B__init = unnamed_addr constant %B zeroinitializer
    @__vtable_B_instance = global %__vtable_B zeroinitializer

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      %one = getelementptr inbounds %A, %A* %0, i32 0, i32 1
      %two = getelementptr inbounds %A, %A* %0, i32 0, i32 2
      ret void
    }

    define i16 @A__foo(%A* %0, i32 %1) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      %one = getelementptr inbounds %A, %A* %0, i32 0, i32 1
      %two = getelementptr inbounds %A, %A* %0, i32 0, i32 2
      %A.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      store i16 0, i16* %A.foo, align 2
      %A__foo_ret = load i16, i16* %A.foo, align 2
      ret i16 %A__foo_ret
    }

    define void @B(%B* %0) {
    entry:
      %this = alloca %B*, align 8
      store %B* %0, %B** %this, align 8
      %__A = getelementptr inbounds %B, %B* %0, i32 0, i32 0
      %three = getelementptr inbounds %B, %B* %0, i32 0, i32 1
      %four = getelementptr inbounds %B, %B* %0, i32 0, i32 2
      ret void
    }

    define i16 @B__foo(%B* %0, i32 %1) {
    entry:
      %this = alloca %B*, align 8
      store %B* %0, %B** %this, align 8
      %__A = getelementptr inbounds %B, %B* %0, i32 0, i32 0
      %three = getelementptr inbounds %B, %B* %0, i32 0, i32 1
      %four = getelementptr inbounds %B, %B* %0, i32 0, i32 2
      %B.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      store i16 0, i16* %B.foo, align 2
      %B__foo_ret = load i16, i16* %B.foo, align 2
      ret i16 %B__foo_ret
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %instanceB = alloca %B, align 8
      %refInstanceA = alloca %A*, align 8
      %0 = bitcast %A* %instanceA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%A* @__A__init to i8*), i64 ptrtoint (%A* getelementptr (%A, %A* null, i32 1) to i64), i1 false)
      %1 = bitcast %B* %instanceB to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%B* @__B__init to i8*), i64 ptrtoint (%B* getelementptr (%B, %B* null, i32 1) to i64), i1 false)
      store %A* null, %A** %refInstanceA, align 8
      call void @__init_a(%A* %instanceA)
      call void @__init_b(%B* %instanceB)
      call void @__user_init_A(%A* %instanceA)
      call void @__user_init_B(%B* %instanceB)
      store %A* %instanceA, %A** %refInstanceA, align 8
      %deref = load %A*, %A** %refInstanceA, align 8
      %__vtable = getelementptr inbounds %A, %A* %deref, i32 0, i32 0
      %deref1 = load i32*, i32** %__vtable, align 8
      %cast = bitcast i32* %deref1 to %__vtable_A*
      %foo = getelementptr inbounds %__vtable_A, %__vtable_A* %cast, i32 0, i32 1
      %2 = load i16 (%A*, i32)*, i16 (%A*, i32)** %foo, align 8
      %deref2 = load %A*, %A** %refInstanceA, align 8
      %fnptr_call = call i16 %2(%A* %deref2, i32 5)
      %3 = bitcast %B* %instanceB to %A*
      store %A* %3, %A** %refInstanceA, align 8
      %deref3 = load %A*, %A** %refInstanceA, align 8
      %__vtable4 = getelementptr inbounds %A, %A* %deref3, i32 0, i32 0
      %deref5 = load i32*, i32** %__vtable4, align 8
      %cast6 = bitcast i32* %deref5 to %__vtable_A*
      %foo7 = getelementptr inbounds %__vtable_A, %__vtable_A* %cast6, i32 0, i32 1
      %4 = load i16 (%A*, i32)*, i16 (%A*, i32)** %foo7, align 8
      %deref8 = load %A*, %A** %refInstanceA, align 8
      %fnptr_call9 = call i16 %4(%A* %deref8, i32 10)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    define void @__init___vtable_a(%__vtable_A* %0) {
    entry:
      %self = alloca %__vtable_A*, align 8
      store %__vtable_A* %0, %__vtable_A** %self, align 8
      %deref = load %__vtable_A*, %__vtable_A** %self, align 8
      %__body = getelementptr inbounds %__vtable_A, %__vtable_A* %deref, i32 0, i32 0
      store void (%A*)* @A, void (%A*)** %__body, align 8
      %deref1 = load %__vtable_A*, %__vtable_A** %self, align 8
      %foo = getelementptr inbounds %__vtable_A, %__vtable_A* %deref1, i32 0, i32 1
      store i16 (%A*, i32)* @A__foo, i16 (%A*, i32)** %foo, align 8
      ret void
    }

    define void @__init___vtable_b(%__vtable_B* %0) {
    entry:
      %self = alloca %__vtable_B*, align 8
      store %__vtable_B* %0, %__vtable_B** %self, align 8
      %deref = load %__vtable_B*, %__vtable_B** %self, align 8
      %__body = getelementptr inbounds %__vtable_B, %__vtable_B* %deref, i32 0, i32 0
      store void (%B*)* @B, void (%B*)** %__body, align 8
      %deref1 = load %__vtable_B*, %__vtable_B** %self, align 8
      %foo = getelementptr inbounds %__vtable_B, %__vtable_B* %deref1, i32 0, i32 1
      store i16 (%B*, i32)* @B__foo, i16 (%B*, i32)** %foo, align 8
      ret void
    }

    define void @__init_b(%B* %0) {
    entry:
      %self = alloca %B*, align 8
      store %B* %0, %B** %self, align 8
      %deref = load %B*, %B** %self, align 8
      %__A = getelementptr inbounds %B, %B* %deref, i32 0, i32 0
      call void @__init_a(%A* %__A)
      %deref1 = load %B*, %B** %self, align 8
      %__A2 = getelementptr inbounds %B, %B* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %A, %A* %__A2, i32 0, i32 0
      store i32* bitcast (%__vtable_B* @__vtable_B_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_a(%A* %0) {
    entry:
      %self = alloca %A*, align 8
      store %A* %0, %A** %self, align 8
      %deref = load %A*, %A** %self, align 8
      %__vtable = getelementptr inbounds %A, %A* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_A* @__vtable_A_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_A(%__vtable_A* %0) {
    entry:
      %self = alloca %__vtable_A*, align 8
      store %__vtable_A* %0, %__vtable_A** %self, align 8
      ret void
    }

    define void @__user_init_B(%B* %0) {
    entry:
      %self = alloca %B*, align 8
      store %B* %0, %B** %self, align 8
      %deref = load %B*, %B** %self, align 8
      %__A = getelementptr inbounds %B, %B* %deref, i32 0, i32 0
      call void @__user_init_A(%A* %__A)
      ret void
    }

    define void @__user_init_A(%A* %0) {
    entry:
      %self = alloca %A*, align 8
      store %A* %0, %A** %self, align 8
      ret void
    }

    define void @__user_init___vtable_B(%__vtable_B* %0) {
    entry:
      %self = alloca %__vtable_B*, align 8
      store %__vtable_B* %0, %__vtable_B** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_a(%__vtable_A* @__vtable_A_instance)
      call void @__init___vtable_b(%__vtable_B* @__vtable_B_instance)
      call void @__user_init___vtable_A(%__vtable_A* @__vtable_A_instance)
      call void @__user_init___vtable_B(%__vtable_B* @__vtable_B_instance)
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn }
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

    %__vtable_A = type { void (%A*)*, i16 (%A*, i32)*, void (%A*)* }
    %A = type { i32* }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_A__init = unnamed_addr constant %__vtable_A zeroinitializer
    @__A__init = unnamed_addr constant %A zeroinitializer
    @__vtable_A_instance = global %__vtable_A zeroinitializer

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      ret void
    }

    define i16 @A__foo(%A* %0, i32 %1) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      %A.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      store i16 0, i16* %A.foo, align 2
      %A__foo_ret = load i16, i16* %A.foo, align 2
      ret i16 %A__foo_ret
    }

    define void @A__bar(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      %deref = load %A*, %A** %this, align 8
      %__vtable1 = getelementptr inbounds %A, %A* %deref, i32 0, i32 0
      %deref2 = load i32*, i32** %__vtable1, align 8
      %cast = bitcast i32* %deref2 to %__vtable_A*
      %foo = getelementptr inbounds %__vtable_A, %__vtable_A* %cast, i32 0, i32 1
      %1 = load i16 (%A*, i32)*, i16 (%A*, i32)** %foo, align 8
      %deref3 = load %A*, %A** %this, align 8
      %fnptr_call = call i16 %1(%A* %deref3, i32 5)
      ret void
    }

    define void @__init___vtable_a(%__vtable_A* %0) {
    entry:
      %self = alloca %__vtable_A*, align 8
      store %__vtable_A* %0, %__vtable_A** %self, align 8
      %deref = load %__vtable_A*, %__vtable_A** %self, align 8
      %__body = getelementptr inbounds %__vtable_A, %__vtable_A* %deref, i32 0, i32 0
      store void (%A*)* @A, void (%A*)** %__body, align 8
      %deref1 = load %__vtable_A*, %__vtable_A** %self, align 8
      %foo = getelementptr inbounds %__vtable_A, %__vtable_A* %deref1, i32 0, i32 1
      store i16 (%A*, i32)* @A__foo, i16 (%A*, i32)** %foo, align 8
      %deref2 = load %__vtable_A*, %__vtable_A** %self, align 8
      %bar = getelementptr inbounds %__vtable_A, %__vtable_A* %deref2, i32 0, i32 2
      store void (%A*)* @A__bar, void (%A*)** %bar, align 8
      ret void
    }

    define void @__init_a(%A* %0) {
    entry:
      %self = alloca %A*, align 8
      store %A* %0, %A** %self, align 8
      %deref = load %A*, %A** %self, align 8
      %__vtable = getelementptr inbounds %A, %A* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_A* @__vtable_A_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_A(%A* %0) {
    entry:
      %self = alloca %A*, align 8
      store %A* %0, %A** %self, align 8
      ret void
    }

    define void @__user_init___vtable_A(%__vtable_A* %0) {
    entry:
      %self = alloca %__vtable_A*, align 8
      store %__vtable_A* %0, %__vtable_A** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_a(%__vtable_A* @__vtable_A_instance)
      call void @__user_init___vtable_A(%__vtable_A* @__vtable_A_instance)
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

    %__vtable_A = type { void (%A*)*, i16 (%A*, i32)*, void (%A*)* }
    %A = type { i32* }
    %__vtable_B = type { void (%B*)*, i16 (%A*, i32)*, void (%B*)* }
    %B = type { %A }
    %__vtable_C = type { void (%C*)*, i16 (%C*, i32)*, void (%C*)* }
    %C = type { %A }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_A__init = unnamed_addr constant %__vtable_A zeroinitializer
    @__A__init = unnamed_addr constant %A zeroinitializer
    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @____vtable_B__init = unnamed_addr constant %__vtable_B zeroinitializer
    @__B__init = unnamed_addr constant %B zeroinitializer
    @__vtable_B_instance = global %__vtable_B zeroinitializer
    @____vtable_C__init = unnamed_addr constant %__vtable_C zeroinitializer
    @__C__init = unnamed_addr constant %C zeroinitializer
    @__vtable_C_instance = global %__vtable_C zeroinitializer

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      ret void
    }

    define i16 @A__foo(%A* %0, i32 %1) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      %A.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      store i16 0, i16* %A.foo, align 2
      %A__foo_ret = load i16, i16* %A.foo, align 2
      ret i16 %A__foo_ret
    }

    define void @A__bar(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      ret void
    }

    define void @B(%B* %0) {
    entry:
      %this = alloca %B*, align 8
      store %B* %0, %B** %this, align 8
      %__A = getelementptr inbounds %B, %B* %0, i32 0, i32 0
      ret void
    }

    define void @B__bar(%B* %0) {
    entry:
      %this = alloca %B*, align 8
      store %B* %0, %B** %this, align 8
      %__A = getelementptr inbounds %B, %B* %0, i32 0, i32 0
      %deref = load %B*, %B** %this, align 8
      %__A1 = getelementptr inbounds %B, %B* %deref, i32 0, i32 0
      %call = call i16 @A__foo(%A* %__A1, i32 5)
      ret void
    }

    define void @C(%C* %0) {
    entry:
      %this = alloca %C*, align 8
      store %C* %0, %C** %this, align 8
      %__A = getelementptr inbounds %C, %C* %0, i32 0, i32 0
      ret void
    }

    define i16 @C__foo(%C* %0, i32 %1) {
    entry:
      %this = alloca %C*, align 8
      store %C* %0, %C** %this, align 8
      %__A = getelementptr inbounds %C, %C* %0, i32 0, i32 0
      %C.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      store i16 0, i16* %C.foo, align 2
      %deref = load %C*, %C** %this, align 8
      call void @C__bar(%C* %deref)
      %C__foo_ret = load i16, i16* %C.foo, align 2
      ret i16 %C__foo_ret
    }

    define void @C__bar(%C* %0) {
    entry:
      %this = alloca %C*, align 8
      store %C* %0, %C** %this, align 8
      %__A = getelementptr inbounds %C, %C* %0, i32 0, i32 0
      %deref = load %C*, %C** %this, align 8
      %call = call i16 @C__foo(%C* %deref, i32 5)
      ret void
    }

    define void @__init___vtable_a(%__vtable_A* %0) {
    entry:
      %self = alloca %__vtable_A*, align 8
      store %__vtable_A* %0, %__vtable_A** %self, align 8
      %deref = load %__vtable_A*, %__vtable_A** %self, align 8
      %__body = getelementptr inbounds %__vtable_A, %__vtable_A* %deref, i32 0, i32 0
      store void (%A*)* @A, void (%A*)** %__body, align 8
      %deref1 = load %__vtable_A*, %__vtable_A** %self, align 8
      %foo = getelementptr inbounds %__vtable_A, %__vtable_A* %deref1, i32 0, i32 1
      store i16 (%A*, i32)* @A__foo, i16 (%A*, i32)** %foo, align 8
      %deref2 = load %__vtable_A*, %__vtable_A** %self, align 8
      %bar = getelementptr inbounds %__vtable_A, %__vtable_A* %deref2, i32 0, i32 2
      store void (%A*)* @A__bar, void (%A*)** %bar, align 8
      ret void
    }

    define void @__init___vtable_b(%__vtable_B* %0) {
    entry:
      %self = alloca %__vtable_B*, align 8
      store %__vtable_B* %0, %__vtable_B** %self, align 8
      %deref = load %__vtable_B*, %__vtable_B** %self, align 8
      %__body = getelementptr inbounds %__vtable_B, %__vtable_B* %deref, i32 0, i32 0
      store void (%B*)* @B, void (%B*)** %__body, align 8
      %deref1 = load %__vtable_B*, %__vtable_B** %self, align 8
      %foo = getelementptr inbounds %__vtable_B, %__vtable_B* %deref1, i32 0, i32 1
      store i16 (%A*, i32)* @A__foo, i16 (%A*, i32)** %foo, align 8
      %deref2 = load %__vtable_B*, %__vtable_B** %self, align 8
      %bar = getelementptr inbounds %__vtable_B, %__vtable_B* %deref2, i32 0, i32 2
      store void (%B*)* @B__bar, void (%B*)** %bar, align 8
      ret void
    }

    define void @__init___vtable_c(%__vtable_C* %0) {
    entry:
      %self = alloca %__vtable_C*, align 8
      store %__vtable_C* %0, %__vtable_C** %self, align 8
      %deref = load %__vtable_C*, %__vtable_C** %self, align 8
      %__body = getelementptr inbounds %__vtable_C, %__vtable_C* %deref, i32 0, i32 0
      store void (%C*)* @C, void (%C*)** %__body, align 8
      %deref1 = load %__vtable_C*, %__vtable_C** %self, align 8
      %foo = getelementptr inbounds %__vtable_C, %__vtable_C* %deref1, i32 0, i32 1
      store i16 (%C*, i32)* @C__foo, i16 (%C*, i32)** %foo, align 8
      %deref2 = load %__vtable_C*, %__vtable_C** %self, align 8
      %bar = getelementptr inbounds %__vtable_C, %__vtable_C* %deref2, i32 0, i32 2
      store void (%C*)* @C__bar, void (%C*)** %bar, align 8
      ret void
    }

    define void @__init_a(%A* %0) {
    entry:
      %self = alloca %A*, align 8
      store %A* %0, %A** %self, align 8
      %deref = load %A*, %A** %self, align 8
      %__vtable = getelementptr inbounds %A, %A* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_A* @__vtable_A_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_b(%B* %0) {
    entry:
      %self = alloca %B*, align 8
      store %B* %0, %B** %self, align 8
      %deref = load %B*, %B** %self, align 8
      %__A = getelementptr inbounds %B, %B* %deref, i32 0, i32 0
      call void @__init_a(%A* %__A)
      %deref1 = load %B*, %B** %self, align 8
      %__A2 = getelementptr inbounds %B, %B* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %A, %A* %__A2, i32 0, i32 0
      store i32* bitcast (%__vtable_B* @__vtable_B_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_c(%C* %0) {
    entry:
      %self = alloca %C*, align 8
      store %C* %0, %C** %self, align 8
      %deref = load %C*, %C** %self, align 8
      %__A = getelementptr inbounds %C, %C* %deref, i32 0, i32 0
      call void @__init_a(%A* %__A)
      %deref1 = load %C*, %C** %self, align 8
      %__A2 = getelementptr inbounds %C, %C* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %A, %A* %__A2, i32 0, i32 0
      store i32* bitcast (%__vtable_C* @__vtable_C_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_C(%C* %0) {
    entry:
      %self = alloca %C*, align 8
      store %C* %0, %C** %self, align 8
      %deref = load %C*, %C** %self, align 8
      %__A = getelementptr inbounds %C, %C* %deref, i32 0, i32 0
      call void @__user_init_A(%A* %__A)
      ret void
    }

    define void @__user_init_A(%A* %0) {
    entry:
      %self = alloca %A*, align 8
      store %A* %0, %A** %self, align 8
      ret void
    }

    define void @__user_init___vtable_A(%__vtable_A* %0) {
    entry:
      %self = alloca %__vtable_A*, align 8
      store %__vtable_A* %0, %__vtable_A** %self, align 8
      ret void
    }

    define void @__user_init_B(%B* %0) {
    entry:
      %self = alloca %B*, align 8
      store %B* %0, %B** %self, align 8
      %deref = load %B*, %B** %self, align 8
      %__A = getelementptr inbounds %B, %B* %deref, i32 0, i32 0
      call void @__user_init_A(%A* %__A)
      ret void
    }

    define void @__user_init___vtable_C(%__vtable_C* %0) {
    entry:
      %self = alloca %__vtable_C*, align 8
      store %__vtable_C* %0, %__vtable_C** %self, align 8
      ret void
    }

    define void @__user_init___vtable_B(%__vtable_B* %0) {
    entry:
      %self = alloca %__vtable_B*, align 8
      store %__vtable_B* %0, %__vtable_B** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_a(%__vtable_A* @__vtable_A_instance)
      call void @__init___vtable_b(%__vtable_B* @__vtable_B_instance)
      call void @__init___vtable_c(%__vtable_C* @__vtable_C_instance)
      call void @__user_init___vtable_A(%__vtable_A* @__vtable_A_instance)
      call void @__user_init___vtable_B(%__vtable_B* @__vtable_B_instance)
      call void @__user_init___vtable_C(%__vtable_C* @__vtable_C_instance)
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

    %__vtable_A = type { void (%A*)*, i16 (%A*, i32)*, void (%A*)* }
    %A = type { i32* }
    %__vtable_B = type { void (%B*)*, i16 (%B*, i32)*, void (%B*)* }
    %B = type { %A }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_A__init = unnamed_addr constant %__vtable_A zeroinitializer
    @__A__init = unnamed_addr constant %A zeroinitializer
    @__vtable_A_instance = global %__vtable_A zeroinitializer
    @____vtable_B__init = unnamed_addr constant %__vtable_B zeroinitializer
    @__B__init = unnamed_addr constant %B zeroinitializer
    @__vtable_B_instance = global %__vtable_B zeroinitializer

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      ret void
    }

    define i16 @A__foo(%A* %0, i32 %1) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      %A.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      store i16 0, i16* %A.foo, align 2
      %A__foo_ret = load i16, i16* %A.foo, align 2
      ret i16 %A__foo_ret
    }

    define void @A__bar(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %__vtable = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      ret void
    }

    define void @B(%B* %0) {
    entry:
      %this = alloca %B*, align 8
      store %B* %0, %B** %this, align 8
      %__A = getelementptr inbounds %B, %B* %0, i32 0, i32 0
      ret void
    }

    define i16 @B__foo(%B* %0, i32 %1) {
    entry:
      %this = alloca %B*, align 8
      store %B* %0, %B** %this, align 8
      %__A = getelementptr inbounds %B, %B* %0, i32 0, i32 0
      %B.foo = alloca i16, align 2
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      store i16 0, i16* %B.foo, align 2
      %call = call i16 @A__foo(%A* %__A, i32 5)
      call void @A__bar(%A* %__A)
      %B__foo_ret = load i16, i16* %B.foo, align 2
      ret i16 %B__foo_ret
    }

    define void @B__bar(%B* %0) {
    entry:
      %this = alloca %B*, align 8
      store %B* %0, %B** %this, align 8
      %__A = getelementptr inbounds %B, %B* %0, i32 0, i32 0
      %call = call i16 @A__foo(%A* %__A, i32 5)
      call void @A__bar(%A* %__A)
      ret void
    }

    define void @__init___vtable_a(%__vtable_A* %0) {
    entry:
      %self = alloca %__vtable_A*, align 8
      store %__vtable_A* %0, %__vtable_A** %self, align 8
      %deref = load %__vtable_A*, %__vtable_A** %self, align 8
      %__body = getelementptr inbounds %__vtable_A, %__vtable_A* %deref, i32 0, i32 0
      store void (%A*)* @A, void (%A*)** %__body, align 8
      %deref1 = load %__vtable_A*, %__vtable_A** %self, align 8
      %foo = getelementptr inbounds %__vtable_A, %__vtable_A* %deref1, i32 0, i32 1
      store i16 (%A*, i32)* @A__foo, i16 (%A*, i32)** %foo, align 8
      %deref2 = load %__vtable_A*, %__vtable_A** %self, align 8
      %bar = getelementptr inbounds %__vtable_A, %__vtable_A* %deref2, i32 0, i32 2
      store void (%A*)* @A__bar, void (%A*)** %bar, align 8
      ret void
    }

    define void @__init___vtable_b(%__vtable_B* %0) {
    entry:
      %self = alloca %__vtable_B*, align 8
      store %__vtable_B* %0, %__vtable_B** %self, align 8
      %deref = load %__vtable_B*, %__vtable_B** %self, align 8
      %__body = getelementptr inbounds %__vtable_B, %__vtable_B* %deref, i32 0, i32 0
      store void (%B*)* @B, void (%B*)** %__body, align 8
      %deref1 = load %__vtable_B*, %__vtable_B** %self, align 8
      %foo = getelementptr inbounds %__vtable_B, %__vtable_B* %deref1, i32 0, i32 1
      store i16 (%B*, i32)* @B__foo, i16 (%B*, i32)** %foo, align 8
      %deref2 = load %__vtable_B*, %__vtable_B** %self, align 8
      %bar = getelementptr inbounds %__vtable_B, %__vtable_B* %deref2, i32 0, i32 2
      store void (%B*)* @B__bar, void (%B*)** %bar, align 8
      ret void
    }

    define void @__init_a(%A* %0) {
    entry:
      %self = alloca %A*, align 8
      store %A* %0, %A** %self, align 8
      %deref = load %A*, %A** %self, align 8
      %__vtable = getelementptr inbounds %A, %A* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_A* @__vtable_A_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_b(%B* %0) {
    entry:
      %self = alloca %B*, align 8
      store %B* %0, %B** %self, align 8
      %deref = load %B*, %B** %self, align 8
      %__A = getelementptr inbounds %B, %B* %deref, i32 0, i32 0
      call void @__init_a(%A* %__A)
      %deref1 = load %B*, %B** %self, align 8
      %__A2 = getelementptr inbounds %B, %B* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %A, %A* %__A2, i32 0, i32 0
      store i32* bitcast (%__vtable_B* @__vtable_B_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_A(%__vtable_A* %0) {
    entry:
      %self = alloca %__vtable_A*, align 8
      store %__vtable_A* %0, %__vtable_A** %self, align 8
      ret void
    }

    define void @__user_init_B(%B* %0) {
    entry:
      %self = alloca %B*, align 8
      store %B* %0, %B** %self, align 8
      %deref = load %B*, %B** %self, align 8
      %__A = getelementptr inbounds %B, %B* %deref, i32 0, i32 0
      call void @__user_init_A(%A* %__A)
      ret void
    }

    define void @__user_init_A(%A* %0) {
    entry:
      %self = alloca %A*, align 8
      store %A* %0, %A** %self, align 8
      ret void
    }

    define void @__user_init___vtable_B(%__vtable_B* %0) {
    entry:
      %self = alloca %__vtable_B*, align 8
      store %__vtable_B* %0, %__vtable_B** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_a(%__vtable_A* @__vtable_A_instance)
      call void @__init___vtable_b(%__vtable_B* @__vtable_B_instance)
      call void @__user_init___vtable_A(%__vtable_A* @__vtable_A_instance)
      call void @__user_init___vtable_B(%__vtable_B* @__vtable_B_instance)
      ret void
    }
    "#);
}
