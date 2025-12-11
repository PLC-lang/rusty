use plc_util::filtered_assert_snapshot;
use test_utils::codegen;

#[test]
fn super_keyword_basic_access() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                x : INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Basic SUPER^ usage to access parent member
            SUPER^.x := 20;
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr }
    %parent = type { ptr, i16 }
    %__vtable_child = type { ptr }
    %child = type { %parent }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 10 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 10 } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      store i16 20, ptr %x, align 2
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_without_deref() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                x : INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                p : REF_TO parent;
            END_VAR
            // SUPER without deref, should create a reference
            p := SUPER;
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr }
    %parent = type { ptr, i16 }
    %__vtable_child = type { ptr }
    %child = type { %parent, ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 10 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 10 }, ptr null }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %p = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      store ptr %__parent, ptr %p, align 8
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_in_method_calls() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                value : INT := 10;
            END_VAR
            
            METHOD process : INT
                process := value * 2;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD process : INT // Override parent's method
                process := value + 5;
            END_METHOD
            
            METHOD test : INT
                // Call parent's implementation
                test := SUPER^.process();
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr, ptr }
    %parent = type { ptr, i16 }
    %__vtable_child = type { ptr, ptr, ptr }
    %child = type { %parent }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 10 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 10 } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define i16 @parent__process(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %parent.process = alloca i16, align 2
      store i16 0, ptr %parent.process, align 2
      %load_value = load i16, ptr %value, align 2
      %1 = sext i16 %load_value to i32
      %tmpVar = mul i32 %1, 2
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %parent.process, align 2
      %parent__process_ret = load i16, ptr %parent.process, align 2
      ret i16 %parent__process_ret
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @child__process(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %child.process = alloca i16, align 2
      store i16 0, ptr %child.process, align 2
      %value = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_value = load i16, ptr %value, align 2
      %1 = sext i16 %load_value to i32
      %tmpVar = add i32 %1, 5
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %child.process, align 2
      %child__process_ret = load i16, ptr %child.process, align 2
      ret i16 %child__process_ret
    }

    define i16 @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %child.test = alloca i16, align 2
      store i16 0, ptr %child.test, align 2
      %call = call i16 @parent__process(ptr %__parent)
      store i16 %call, ptr %child.test, align 2
      %child__test_ret = load i16, ptr %child.test, align 2
      ret i16 %child__test_ret
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %process = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 1
      store ptr @parent__process, ptr %process, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %process = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 1
      store ptr @child__process, ptr %process, align 8
      %deref2 = load ptr, ptr %self, align 8
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref2, i32 0, i32 2
      store ptr @child__test, ptr %test, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_in_complex_expressions() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                x : INT := 10;
                y : INT := 20;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                z : INT := 30;
            END_VAR
            // Use SUPER^ in complex expressions
            z := SUPER^.x + SUPER^.y * 2;
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr }
    %parent = type { ptr, i16, i16 }
    %__vtable_child = type { ptr }
    %child = type { %parent, i16 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 10, i16 20 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 10, i16 20 }, i16 30 }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %z = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      %x = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_x = load i16, ptr %x, align 2
      %1 = sext i16 %load_x to i32
      %y = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_y = load i16, ptr %y, align 2
      %2 = sext i16 %load_y to i32
      %tmpVar = mul i32 %2, 2
      %tmpVar1 = add i32 %1, %tmpVar
      %3 = trunc i32 %tmpVar1 to i16
      store i16 %3, ptr %z, align 2
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_with_array_access() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                arr : ARRAY[0..5] OF INT := [1,2,3,4,5,6];
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                index : INT := 3;
            END_VAR
            // Array access via SUPER
            SUPER^.arr[index] := 42;
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr }
    %parent = type { ptr, [6 x i16] }
    %__vtable_child = type { ptr }
    %child = type { %parent, i16 }

    @__parent.arr__init = unnamed_addr constant [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6]
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6] }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6] }, i16 3 }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %arr = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %index = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      %arr = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_index = load i16, ptr %index, align 2
      %1 = sext i16 %load_index to i32
      %tmpVar = mul i32 1, %1
      %tmpVar1 = add i32 %tmpVar, 0
      %tmpVar2 = getelementptr inbounds [6 x i16], ptr %arr, i32 0, i32 %tmpVar1
      store i16 42, ptr %tmpVar2, align 2
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_in_multi_level_inheritance() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK grandparent
            VAR
                g_val : INT := 10;
            END_VAR
            
            METHOD gp_method : INT
                gp_method := g_val;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
            VAR
                p_val : INT := 20;
            END_VAR
            
            METHOD p_method : INT
                p_method := p_val + SUPER^.gp_method();
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                c_val : INT := 30;
            END_VAR
            
            METHOD test : INT
                // Access parent's method which itself uses SUPER^
                test := SUPER^.p_method();
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_grandparent = type { ptr, ptr }
    %grandparent = type { ptr, i16 }
    %__vtable_parent = type { ptr, ptr, ptr }
    %parent = type { %grandparent, i16 }
    %__vtable_child = type { ptr, ptr, ptr, ptr }
    %child = type { %parent, i16 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer
    @__grandparent__init = unnamed_addr constant %grandparent { ptr null, i16 10 }
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { %grandparent { ptr null, i16 10 }, i16 20 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { %grandparent { ptr null, i16 10 }, i16 20 }, i16 30 }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @grandparent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %g_val = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      ret void
    }

    define i16 @grandparent__gp_method(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %g_val = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      %grandparent.gp_method = alloca i16, align 2
      store i16 0, ptr %grandparent.gp_method, align 2
      %load_g_val = load i16, ptr %g_val, align 2
      store i16 %load_g_val, ptr %grandparent.gp_method, align 2
      %grandparent__gp_method_ret = load i16, ptr %grandparent.gp_method, align 2
      ret i16 %grandparent__gp_method_ret
    }

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %p_val = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define i16 @parent__p_method(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %p_val = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %parent.p_method = alloca i16, align 2
      store i16 0, ptr %parent.p_method, align 2
      %load_p_val = load i16, ptr %p_val, align 2
      %1 = sext i16 %load_p_val to i32
      %call = call i16 @grandparent__gp_method(ptr %__grandparent)
      %2 = sext i16 %call to i32
      %tmpVar = add i32 %1, %2
      %3 = trunc i32 %tmpVar to i16
      store i16 %3, ptr %parent.p_method, align 2
      %parent__p_method_ret = load i16, ptr %parent.p_method, align 2
      ret i16 %parent__p_method_ret
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %c_val = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      ret void
    }

    define i16 @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %c_val = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      %child.test = alloca i16, align 2
      store i16 0, ptr %child.test, align 2
      %call = call i16 @parent__p_method(ptr %__parent)
      store i16 %call, ptr %child.test, align 2
      %child__test_ret = load i16, ptr %child.test, align 2
      ret i16 %child__test_ret
    }

    define void @__init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @grandparent, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %gp_method = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref1, i32 0, i32 1
      store ptr @grandparent__gp_method, ptr %gp_method, align 8
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %gp_method = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 1
      store ptr @grandparent__gp_method, ptr %gp_method, align 8
      %deref2 = load ptr, ptr %self, align 8
      %p_method = getelementptr inbounds nuw %__vtable_parent, ptr %deref2, i32 0, i32 2
      store ptr @parent__p_method, ptr %p_method, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %gp_method = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 1
      store ptr @grandparent__gp_method, ptr %gp_method, align 8
      %deref2 = load ptr, ptr %self, align 8
      %p_method = getelementptr inbounds nuw %__vtable_child, ptr %deref2, i32 0, i32 2
      store ptr @parent__p_method, ptr %p_method, align 8
      %deref3 = load ptr, ptr %self, align 8
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 3
      store ptr @child__test, ptr %test, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__init_grandparent(ptr %__grandparent)
      %deref1 = load ptr, ptr %self, align 8
      %__grandparent2 = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_grandparent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__user_init_grandparent(ptr %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_with_pointer_operations() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                val : INT := 10;
                ptr : REF_TO INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Pointer operations with SUPER^
            SUPER^.ptr := REF(SUPER^.val);
            // Dereferencing pointer from parent
            SUPER^.val := SUPER^.ptr^ + 5;
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr }
    %parent = type { ptr, i16, ptr }
    %__vtable_child = type { ptr }
    %child = type { %parent }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 10, ptr null }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 10, ptr null } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %val = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %ptr = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %ptr = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %val = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      store ptr %val, ptr %ptr, align 8
      %val1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %ptr2 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %deref = load ptr, ptr %ptr2, align 8
      %load_tmpVar = load i16, ptr %deref, align 2
      %1 = sext i16 %load_tmpVar to i32
      %tmpVar = add i32 %1, 5
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %val1, align 2
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_in_conditionals() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                threshold : INT := 50;
                value : INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD test
                // SUPER^ in IF statement
                IF SUPER^.value > SUPER^.threshold THEN
                    SUPER^.value := 0;
                ELSE
                    SUPER^.value := 100;
                END_IF;
                
                // In CASE statement
                CASE SUPER^.value OF
                    10: SUPER^.threshold := 40;
                    20: SUPER^.threshold := 60;
                END_CASE;
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr }
    %parent = type { ptr, i16, i16 }
    %__vtable_child = type { ptr, ptr }
    %child = type { %parent }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 50, i16 10 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 50, i16 10 } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %threshold = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_value = load i16, ptr %value, align 2
      %1 = sext i16 %load_value to i32
      %threshold = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_threshold = load i16, ptr %threshold, align 2
      %2 = sext i16 %load_threshold to i32
      %tmpVar = icmp sgt i32 %1, %2
      %3 = zext i1 %tmpVar to i8
      %4 = icmp ne i8 %3, 0
      br i1 %4, label %condition_body, label %else

    condition_body:                                   ; preds = %entry
      %value1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      store i16 0, ptr %value1, align 2
      br label %continue

    else:                                             ; preds = %entry
      %value2 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      store i16 100, ptr %value2, align 2
      br label %continue

    continue:                                         ; preds = %else, %condition_body
      %value4 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_value5 = load i16, ptr %value4, align 2
      switch i16 %load_value5, label %else6 [
        i16 10, label %case
        i16 20, label %case8
      ]

    case:                                             ; preds = %continue
      %threshold7 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      store i16 40, ptr %threshold7, align 2
      br label %continue3

    case8:                                            ; preds = %continue
      %threshold9 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      store i16 60, ptr %threshold9, align 2
      br label %continue3

    else6:                                            ; preds = %continue
      br label %continue3

    continue3:                                        ; preds = %else6, %case8, %case
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 1
      store ptr @child__test, ptr %test, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_with_const_variables() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR CONSTANT
                MAX_VALUE : INT := 100;
            END_VAR
            VAR
                current : INT := 50;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Access constant from parent
            SUPER^.current := SUPER^.MAX_VALUE / 2;
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr }
    %parent = type { ptr, i16, i16 }
    %__vtable_child = type { ptr }
    %child = type { %parent }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 100, i16 50 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 100, i16 50 } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %MAX_VALUE = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %current = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %current = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      store i16 50, ptr %current, align 2
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_as_function_parameter() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                val : INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD test
                // Pass SUPER as function parameter
                process_ref(SUPER);
                process_val(SUPER^);
            END_METHOD
        END_FUNCTION_BLOCK
        
        FUNCTION process_ref : INT
        VAR_INPUT
            ref : REF_TO parent;
        END_VAR
            ref^.val := 20;
        END_FUNCTION
        
        FUNCTION process_val : INT
        VAR_INPUT
            val : parent;
        END_VAR
            val.val := 30; // No effect since this is passed by value
        END_FUNCTION
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr }
    %parent = type { ptr, i16 }
    %__vtable_child = type { ptr, ptr }
    %child = type { %parent }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 10 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 10 } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %val = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %call = call i16 @process_ref(ptr %__parent)
      %call1 = call i16 @process_val(ptr %__parent)
      ret void
    }

    define i16 @process_ref(ptr %0) {
    entry:
      %process_ref = alloca i16, align 2
      %ref = alloca ptr, align 8
      store ptr %0, ptr %ref, align 8
      store i16 0, ptr %process_ref, align 2
      %deref = load ptr, ptr %ref, align 8
      %val = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 1
      store i16 20, ptr %val, align 2
      %process_ref_ret = load i16, ptr %process_ref, align 2
      ret i16 %process_ref_ret
    }

    define i16 @process_val(ptr %0) {
    entry:
      %process_val = alloca i16, align 2
      %val = alloca %parent, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %val, ptr align 1 %0, i64 ptrtoint (ptr getelementptr (%parent, ptr null, i32 1) to i64), i1 false)
      store i16 0, ptr %process_val, align 2
      %val1 = getelementptr inbounds nuw %parent, ptr %val, i32 0, i32 1
      store i16 30, ptr %val1, align 2
      %process_val_ret = load i16, ptr %process_val, align 2
      ret i16 %process_val_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 1
      store ptr @child__test, ptr %test, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn super_with_deeply_nested_expressions() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                a : INT := 1;
                b : INT := 2;
                c : INT := 3;
            END_VAR
            
            METHOD calc : INT
                calc := a + b * c;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD test : INT
                // Deeply nested expressions with SUPER^
                test := ((SUPER^.a + SUPER^.b) * SUPER^.c + SUPER^.calc()) / (SUPER^.a + 1);
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr, ptr }
    %parent = type { ptr, i16, i16, i16 }
    %__vtable_child = type { ptr, ptr, ptr }
    %child = type { %parent }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 1, i16 2, i16 3 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 1, i16 2, i16 3 } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %a = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      %c = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 3
      ret void
    }

    define i16 @parent__calc(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %a = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      %c = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 3
      %parent.calc = alloca i16, align 2
      store i16 0, ptr %parent.calc, align 2
      %load_a = load i16, ptr %a, align 2
      %1 = sext i16 %load_a to i32
      %load_b = load i16, ptr %b, align 2
      %2 = sext i16 %load_b to i32
      %load_c = load i16, ptr %c, align 2
      %3 = sext i16 %load_c to i32
      %tmpVar = mul i32 %2, %3
      %tmpVar1 = add i32 %1, %tmpVar
      %4 = trunc i32 %tmpVar1 to i16
      store i16 %4, ptr %parent.calc, align 2
      %parent__calc_ret = load i16, ptr %parent.calc, align 2
      ret i16 %parent__calc_ret
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %child.test = alloca i16, align 2
      store i16 0, ptr %child.test, align 2
      %a = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_a = load i16, ptr %a, align 2
      %1 = sext i16 %load_a to i32
      %b = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_b = load i16, ptr %b, align 2
      %2 = sext i16 %load_b to i32
      %tmpVar = add i32 %1, %2
      %c = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 3
      %load_c = load i16, ptr %c, align 2
      %3 = sext i16 %load_c to i32
      %tmpVar1 = mul i32 %tmpVar, %3
      %call = call i16 @parent__calc(ptr %__parent)
      %4 = sext i16 %call to i32
      %tmpVar2 = add i32 %tmpVar1, %4
      %a3 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_a4 = load i16, ptr %a3, align 2
      %5 = sext i16 %load_a4 to i32
      %tmpVar5 = add i32 %5, 1
      %tmpVar6 = sdiv i32 %tmpVar2, %tmpVar5
      %6 = trunc i32 %tmpVar6 to i16
      store i16 %6, ptr %child.test, align 2
      %child__test_ret = load i16, ptr %child.test, align 2
      ret i16 %child__test_ret
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %calc = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 1
      store ptr @parent__calc, ptr %calc, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %calc = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 1
      store ptr @parent__calc, ptr %calc, align 8
      %deref2 = load ptr, ptr %self, align 8
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref2, i32 0, i32 2
      store ptr @child__test, ptr %test, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_in_loop_constructs() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                counter : INT := 0;
                arr : ARRAY[0..5] OF INT := [1,2,3,4,5,6];
            END_VAR
            
            METHOD increment
                counter := counter + 1;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD process
                VAR
                    i : INT;
                    sum : INT := 0;
                END_VAR
                
                // FOR loop with SUPER^
                FOR i := 0 TO 5 BY 1 DO
                    sum := sum + SUPER^.arr[i];
                    SUPER^.increment();
                END_FOR;
                
                // WHILE loop with SUPER^
                WHILE SUPER^.counter < 10 DO
                    SUPER^.increment();
                END_WHILE;
                
                // REPEAT loop with SUPER^
                REPEAT
                    SUPER^.counter := SUPER^.counter - 1;
                UNTIL SUPER^.counter <= 0
                END_REPEAT;
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr, ptr }
    %parent = type { ptr, i16, [6 x i16] }
    %__vtable_child = type { ptr, ptr, ptr }
    %child = type { %parent }

    @__parent.arr__init = unnamed_addr constant [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6]
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 0, [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6] }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 0, [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6] } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %counter = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %arr = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @parent__increment(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %counter = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %arr = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      %load_counter = load i16, ptr %counter, align 2
      %1 = sext i16 %load_counter to i32
      %tmpVar = add i32 %1, 1
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %counter, align 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @child__process(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %i = alloca i16, align 2
      %sum = alloca i16, align 2
      store i16 0, ptr %i, align 2
      store i16 0, ptr %sum, align 2
      store i16 0, ptr %i, align 2
      br i1 true, label %predicate_sle, label %predicate_sge

    predicate_sle:                                    ; preds = %increment, %entry
      %1 = load i16, ptr %i, align 2
      %2 = sext i16 %1 to i32
      %condition = icmp sle i32 %2, 5
      br i1 %condition, label %loop, label %continue

    predicate_sge:                                    ; preds = %increment, %entry
      %3 = load i16, ptr %i, align 2
      %4 = sext i16 %3 to i32
      %condition1 = icmp sge i32 %4, 5
      br i1 %condition1, label %loop, label %continue

    loop:                                             ; preds = %predicate_sge, %predicate_sle
      %load_sum = load i16, ptr %sum, align 2
      %5 = sext i16 %load_sum to i32
      %arr = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_i = load i16, ptr %i, align 2
      %6 = sext i16 %load_i to i32
      %tmpVar = mul i32 1, %6
      %tmpVar2 = add i32 %tmpVar, 0
      %tmpVar3 = getelementptr inbounds [6 x i16], ptr %arr, i32 0, i32 %tmpVar2
      %load_tmpVar = load i16, ptr %tmpVar3, align 2
      %7 = sext i16 %load_tmpVar to i32
      %tmpVar4 = add i32 %5, %7
      %8 = trunc i32 %tmpVar4 to i16
      store i16 %8, ptr %sum, align 2
      call void @parent__increment(ptr %__parent)
      br label %increment

    increment:                                        ; preds = %loop
      %9 = load i16, ptr %i, align 2
      %10 = sext i16 %9 to i32
      %next = add i32 1, %10
      %11 = trunc i32 %next to i16
      store i16 %11, ptr %i, align 2
      br i1 true, label %predicate_sle, label %predicate_sge

    continue:                                         ; preds = %predicate_sge, %predicate_sle
      br label %condition_check

    condition_check:                                  ; preds = %continue6, %continue
      br i1 true, label %while_body, label %continue5

    while_body:                                       ; preds = %condition_check
      %counter = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_counter = load i16, ptr %counter, align 2
      %12 = sext i16 %load_counter to i32
      %tmpVar7 = icmp slt i32 %12, 10
      %13 = zext i1 %tmpVar7 to i8
      %14 = icmp ne i8 %13, 0
      %tmpVar8 = xor i1 %14, true
      br i1 %tmpVar8, label %condition_body, label %continue6

    continue5:                                        ; preds = %condition_body, %condition_check
      br label %condition_check9

    condition_body:                                   ; preds = %while_body
      br label %continue5

    buffer_block:                                     ; No predecessors!
      br label %continue6

    continue6:                                        ; preds = %buffer_block, %while_body
      call void @parent__increment(ptr %__parent)
      br label %condition_check

    condition_check9:                                 ; preds = %continue16, %continue5
      br i1 true, label %while_body10, label %continue11

    while_body10:                                     ; preds = %condition_check9
      %counter12 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %counter13 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_counter14 = load i16, ptr %counter13, align 2
      %15 = sext i16 %load_counter14 to i32
      %tmpVar15 = sub i32 %15, 1
      %16 = trunc i32 %tmpVar15 to i16
      store i16 %16, ptr %counter12, align 2
      %counter17 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_counter18 = load i16, ptr %counter17, align 2
      %17 = sext i16 %load_counter18 to i32
      %tmpVar19 = icmp sle i32 %17, 0
      %18 = zext i1 %tmpVar19 to i8
      %19 = icmp ne i8 %18, 0
      br i1 %19, label %condition_body20, label %continue16

    continue11:                                       ; preds = %condition_body20, %condition_check9
      ret void

    condition_body20:                                 ; preds = %while_body10
      br label %continue11

    buffer_block21:                                   ; No predecessors!
      br label %continue16

    continue16:                                       ; preds = %buffer_block21, %while_body10
      br label %condition_check9
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %increment = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 1
      store ptr @parent__increment, ptr %increment, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %increment = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 1
      store ptr @parent__increment, ptr %increment, align 8
      %deref2 = load ptr, ptr %self, align 8
      %process = getelementptr inbounds nuw %__vtable_child, ptr %deref2, i32 0, i32 2
      store ptr @child__process, ptr %process, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn super_with_method_overrides_in_three_levels() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK grandparent
            METHOD calculate : INT
                calculate := 100;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
            METHOD calculate : INT
                // Call grandparent's implementation and modify
                calculate := SUPER^.calculate() + 50;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD calculate : INT
                // Call parent's implementation (which calls grandparent's) and modify again
                calculate := SUPER^.calculate() + 25;
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_grandparent = type { ptr, ptr }
    %grandparent = type { ptr }
    %__vtable_parent = type { ptr, ptr }
    %parent = type { %grandparent }
    %__vtable_child = type { ptr, ptr }
    %child = type { %parent }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent zeroinitializer
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @grandparent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @grandparent__calculate(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %grandparent.calculate = alloca i16, align 2
      store i16 0, ptr %grandparent.calculate, align 2
      store i16 100, ptr %grandparent.calculate, align 2
      %grandparent__calculate_ret = load i16, ptr %grandparent.calculate, align 2
      ret i16 %grandparent__calculate_ret
    }

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @parent__calculate(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %parent.calculate = alloca i16, align 2
      store i16 0, ptr %parent.calculate, align 2
      %call = call i16 @grandparent__calculate(ptr %__grandparent)
      %1 = sext i16 %call to i32
      %tmpVar = add i32 %1, 50
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %parent.calculate, align 2
      %parent__calculate_ret = load i16, ptr %parent.calculate, align 2
      ret i16 %parent__calculate_ret
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @child__calculate(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %child.calculate = alloca i16, align 2
      store i16 0, ptr %child.calculate, align 2
      %call = call i16 @parent__calculate(ptr %__parent)
      %1 = sext i16 %call to i32
      %tmpVar = add i32 %1, 25
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %child.calculate, align 2
      %child__calculate_ret = load i16, ptr %child.calculate, align 2
      ret i16 %child__calculate_ret
    }

    define void @__init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @grandparent, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %calculate = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref1, i32 0, i32 1
      store ptr @grandparent__calculate, ptr %calculate, align 8
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %calculate = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref1, i32 0, i32 1
      store ptr @parent__calculate, ptr %calculate, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %calculate = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref1, i32 0, i32 1
      store ptr @child__calculate, ptr %calculate, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__init_grandparent(ptr %__grandparent)
      %deref1 = load ptr, ptr %self, align 8
      %__grandparent2 = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_grandparent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__user_init_grandparent(ptr %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
#[ignore = "needs `THIS` pointer and direct dereferencing of fn results to be implemented"]
fn super_with_return_value_in_multiple_contexts() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                value : INT := 10;
            END_VAR
            
            METHOD get_value : INT
                get_value := value;
            END_METHOD
            
            METHOD get_ref : REF_TO parent
                get_ref := THIS;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD test_value : INT
                // Return value directly from SUPER^ call
                test_value := SUPER^.get_value();
            END_METHOD
            
            METHOD test_ref : REF_TO parent
                // Return REF_TO parent from SUPER
                test_ref := SUPER;
            END_METHOD
            
            METHOD test_mixed : INT
                // Use SUPER in complex return expression
                test_mixed := SUPER^.get_value() + SUPER^.get_ref()^.value;
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#""#);
}

#[test]
fn super_with_structured_types() {
    let result = codegen(
        r#"
        TYPE Complex_Type :
            STRUCT
                x : INT;
                y : INT;
                z : REAL;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK parent
            VAR
                data : Complex_Type := (x := 10, y := 20, z := 30.5);
                arr_data : ARRAY[0..1] OF Complex_Type := [(x := 1, y := 2, z := 3.5), (x := 4, y := 5, z := 6.5)];
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD test
                VAR
                    local_data : Complex_Type;
                END_VAR
                
                // Access structured type through SUPER^
                local_data.x := SUPER^.data.x;
                local_data.y := SUPER^.data.y;
                local_data.z := SUPER^.data.z;
                
                // Access structured array through SUPER^
                SUPER^.arr_data[0].x := SUPER^.arr_data[1].x;
                
                // Nested access
                SUPER^.arr_data[0].z := SUPER^.data.z;
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %Complex_Type = type { i16, i16, float }
    %__vtable_parent = type { ptr }
    %parent = type { ptr, %Complex_Type, [2 x %Complex_Type] }
    %__vtable_child = type { ptr, ptr }
    %child = type { %parent }

    @__parent.data__init = unnamed_addr constant %Complex_Type { i16 10, i16 20, float 3.050000e+01 }
    @__parent.arr_data__init = unnamed_addr constant [2 x %Complex_Type] [%Complex_Type { i16 1, i16 2, float 3.500000e+00 }, %Complex_Type { i16 4, i16 5, float 6.500000e+00 }]
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, %Complex_Type { i16 10, i16 20, float 3.050000e+01 }, [2 x %Complex_Type] [%Complex_Type { i16 1, i16 2, float 3.500000e+00 }, %Complex_Type { i16 4, i16 5, float 6.500000e+00 }] }
    @__Complex_Type__init = unnamed_addr constant %Complex_Type zeroinitializer
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, %Complex_Type { i16 10, i16 20, float 3.050000e+01 }, [2 x %Complex_Type] [%Complex_Type { i16 1, i16 2, float 3.500000e+00 }, %Complex_Type { i16 4, i16 5, float 6.500000e+00 }] } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %data = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %arr_data = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %local_data = alloca %Complex_Type, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %local_data, ptr align 1 @__Complex_Type__init, i64 ptrtoint (ptr getelementptr (%Complex_Type, ptr null, i32 1) to i64), i1 false)
      call void @__init_complex_type(ptr %local_data)
      call void @__user_init_Complex_Type(ptr %local_data)
      %x = getelementptr inbounds nuw %Complex_Type, ptr %local_data, i32 0, i32 0
      %data = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %x1 = getelementptr inbounds nuw %Complex_Type, ptr %data, i32 0, i32 0
      %load_x = load i16, ptr %x1, align 2
      store i16 %load_x, ptr %x, align 2
      %y = getelementptr inbounds nuw %Complex_Type, ptr %local_data, i32 0, i32 1
      %data2 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %y3 = getelementptr inbounds nuw %Complex_Type, ptr %data2, i32 0, i32 1
      %load_y = load i16, ptr %y3, align 2
      store i16 %load_y, ptr %y, align 2
      %z = getelementptr inbounds nuw %Complex_Type, ptr %local_data, i32 0, i32 2
      %data4 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %z5 = getelementptr inbounds nuw %Complex_Type, ptr %data4, i32 0, i32 2
      %load_z = load float, ptr %z5, align 4
      store float %load_z, ptr %z, align 4
      %arr_data = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %tmpVar = getelementptr inbounds [2 x %Complex_Type], ptr %arr_data, i32 0, i32 0
      %x6 = getelementptr inbounds nuw %Complex_Type, ptr %tmpVar, i32 0, i32 0
      %arr_data7 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %tmpVar8 = getelementptr inbounds [2 x %Complex_Type], ptr %arr_data7, i32 0, i32 1
      %x9 = getelementptr inbounds nuw %Complex_Type, ptr %tmpVar8, i32 0, i32 0
      %load_x10 = load i16, ptr %x9, align 2
      store i16 %load_x10, ptr %x6, align 2
      %arr_data11 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %tmpVar12 = getelementptr inbounds [2 x %Complex_Type], ptr %arr_data11, i32 0, i32 0
      %z13 = getelementptr inbounds nuw %Complex_Type, ptr %tmpVar12, i32 0, i32 2
      %data14 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %z15 = getelementptr inbounds nuw %Complex_Type, ptr %data14, i32 0, i32 2
      %load_z16 = load float, ptr %z15, align 4
      store float %load_z16, ptr %z13, align 4
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 1
      store ptr @child__test, ptr %test, align 8
      ret void
    }

    define void @__init_complex_type(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %data = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 1
      call void @__init_complex_type(ptr %data)
      %deref1 = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_Complex_Type(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %data = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 1
      call void @__user_init_Complex_Type(ptr %data)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn super_in_action_blocks() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK parent
            VAR
                value : INT := 10;
            END_VAR
            
            METHOD increment
                value := value + 1;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        END_FUNCTION_BLOCK
        
        ACTION child.increase
            // Using SUPER^ inside an ACTION block
            SUPER^.value := SUPER^.value + 5;
            SUPER^.increment(); // Call parent's method from action
        END_ACTION
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { ptr, ptr }
    %parent = type { ptr, i16 }
    %__vtable_child = type { ptr, ptr }
    %child = type { %parent }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent { ptr null, i16 10 }
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child { %parent { ptr null, i16 10 } }
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @parent__increment(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %load_value = load i16, ptr %value, align 2
      %1 = sext i16 %load_value to i32
      %tmpVar = add i32 %1, 1
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %value, align 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @child__increase(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %value1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_value = load i16, ptr %value1, align 2
      %1 = sext i16 %load_value to i32
      %tmpVar = add i32 %1, 5
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %value, align 2
      call void @parent__increment(ptr %__parent)
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %increment = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 1
      store ptr @parent__increment, ptr %increment, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %increment = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 1
      store ptr @parent__increment, ptr %increment, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}
