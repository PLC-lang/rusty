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
    %__vtable_child = type { ptr }
    %parent = type { ptr, i16 }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      store i16 20, ptr %x, align [filtered]
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 10, ptr %x, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_child = type { ptr }
    %parent = type { ptr, i16 }
    %child = type { %parent, ptr }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %p = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      store ptr %__parent, ptr %p, align [filtered]
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 10, ptr %x, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %p = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 1
      call void @__child_p__ctor(ptr %p)
      %deref4 = load ptr, ptr %self, align [filtered]
      %__parent5 = getelementptr inbounds nuw %child, ptr %deref4, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent5, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__child_p__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_child = type { ptr, ptr, ptr }
    %parent = type { ptr, i16 }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define i16 @parent__process(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %parent.process = alloca i16, align [filtered]
      store i16 0, ptr %parent.process, align [filtered]
      %load_value = load i16, ptr %value, align [filtered]
      %1 = sext i16 %load_value to i32
      %tmpVar = mul i32 %1, 2
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %parent.process, align [filtered]
      %parent__process_ret = load i16, ptr %parent.process, align [filtered]
      ret i16 %parent__process_ret
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @child__process(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %child.process = alloca i16, align [filtered]
      store i16 0, ptr %child.process, align [filtered]
      %value = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_value = load i16, ptr %value, align [filtered]
      %1 = sext i16 %load_value to i32
      %tmpVar = add i32 %1, 5
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %child.process, align [filtered]
      %child__process_ret = load i16, ptr %child.process, align [filtered]
      ret i16 %child__process_ret
    }

    define i16 @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %child.test = alloca i16, align [filtered]
      store i16 0, ptr %child.test, align [filtered]
      %call = call i16 @parent__process(ptr %__parent)
      store i16 %call, ptr %child.test, align [filtered]
      %child__test_ret = load i16, ptr %child.test, align [filtered]
      ret i16 %child__test_ret
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %value = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 10, ptr %value, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %process = getelementptr inbounds nuw %__vtable_parent, ptr %deref3, i32 0, i32 1
      call void @____vtable_parent_process__ctor(ptr %process)
      %deref4 = load ptr, ptr %self, align [filtered]
      %process5 = getelementptr inbounds nuw %__vtable_parent, ptr %deref4, i32 0, i32 1
      store ptr @parent__process, ptr %process5, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %process = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 1
      call void @____vtable_child_process__ctor(ptr %process)
      %deref4 = load ptr, ptr %self, align [filtered]
      %process5 = getelementptr inbounds nuw %__vtable_child, ptr %deref4, i32 0, i32 1
      store ptr @child__process, ptr %process5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref6, i32 0, i32 2
      call void @____vtable_child_test__ctor(ptr %test)
      %deref7 = load ptr, ptr %self, align [filtered]
      %test8 = getelementptr inbounds nuw %__vtable_child, ptr %deref7, i32 0, i32 2
      store ptr @child__test, ptr %test8, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_parent_process__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_process__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_test__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_child = type { ptr }
    %parent = type { ptr, i16, i16 }
    %child = type { %parent, i16 }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %z = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      %x = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_x = load i16, ptr %x, align [filtered]
      %1 = sext i16 %load_x to i32
      %y = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_y = load i16, ptr %y, align [filtered]
      %2 = sext i16 %load_y to i32
      %tmpVar = mul i32 %2, 2
      %tmpVar1 = add i32 %1, %tmpVar
      %3 = trunc i32 %tmpVar1 to i16
      store i16 %3, ptr %z, align [filtered]
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 10, ptr %x, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %y = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 2
      store i16 20, ptr %y, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %__vtable4 = getelementptr inbounds nuw %parent, ptr %deref3, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable4, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %z = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 1
      store i16 30, ptr %z, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__parent5 = getelementptr inbounds nuw %child, ptr %deref4, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent5, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_child = type { ptr }
    %parent = type { ptr, [6 x i16] }
    %child = type { %parent, i16 }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @__parent.arr__init = unnamed_addr constant [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %arr = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %index = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      %arr = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_index = load i16, ptr %index, align [filtered]
      %1 = sext i16 %load_index to i32
      %tmpVar = mul i32 1, %1
      %tmpVar1 = add i32 %tmpVar, 0
      %tmpVar2 = getelementptr inbounds [6 x i16], ptr %arr, i32 0, i32 %tmpVar1
      store i16 42, ptr %tmpVar2, align [filtered]
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %arr = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      call void @__parent_arr__ctor(ptr %arr)
      %deref2 = load ptr, ptr %self, align [filtered]
      %arr3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 1
      store [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6], ptr %arr3, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__vtable5 = getelementptr inbounds nuw %parent, ptr %deref4, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable5, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %index = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 1
      store i16 3, ptr %index, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__parent5 = getelementptr inbounds nuw %child, ptr %deref4, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent5, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__parent_arr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_parent = type { ptr, ptr, ptr }
    %__vtable_child = type { ptr, ptr, ptr, ptr }
    %grandparent = type { ptr, i16 }
    %parent = type { %grandparent, i16 }
    %child = type { %parent, i16 }

    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @grandparent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %g_val = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      ret void
    }

    define i16 @grandparent__gp_method(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %g_val = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      %grandparent.gp_method = alloca i16, align [filtered]
      store i16 0, ptr %grandparent.gp_method, align [filtered]
      %load_g_val = load i16, ptr %g_val, align [filtered]
      store i16 %load_g_val, ptr %grandparent.gp_method, align [filtered]
      %grandparent__gp_method_ret = load i16, ptr %grandparent.gp_method, align [filtered]
      ret i16 %grandparent__gp_method_ret
    }

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %p_val = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define i16 @parent__p_method(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %p_val = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %parent.p_method = alloca i16, align [filtered]
      store i16 0, ptr %parent.p_method, align [filtered]
      %load_p_val = load i16, ptr %p_val, align [filtered]
      %1 = sext i16 %load_p_val to i32
      %call = call i16 @grandparent__gp_method(ptr %__grandparent)
      %2 = sext i16 %call to i32
      %tmpVar = add i32 %1, %2
      %3 = trunc i32 %tmpVar to i16
      store i16 %3, ptr %parent.p_method, align [filtered]
      %parent__p_method_ret = load i16, ptr %parent.p_method, align [filtered]
      ret i16 %parent__p_method_ret
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %c_val = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      ret void
    }

    define i16 @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %c_val = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      %child.test = alloca i16, align [filtered]
      store i16 0, ptr %child.test, align [filtered]
      %call = call i16 @parent__p_method(ptr %__parent)
      store i16 %call, ptr %child.test, align [filtered]
      %child__test_ret = load i16, ptr %child.test, align [filtered]
      ret i16 %child__test_ret
    }

    define void @grandparent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0
      call void @__grandparent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %g_val = getelementptr inbounds nuw %grandparent, ptr %deref1, i32 0, i32 1
      store i16 10, ptr %g_val, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %grandparent, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_grandparent_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @grandparent__ctor(ptr %__grandparent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__grandparent2 = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0
      call void @grandparent__ctor(ptr %__grandparent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %p_val = getelementptr inbounds nuw %parent, ptr %deref3, i32 0, i32 1
      store i16 20, ptr %p_val, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__grandparent5 = getelementptr inbounds nuw %parent, ptr %deref4, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent5, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %c_val = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 1
      store i16 30, ptr %c_val, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__parent5 = getelementptr inbounds nuw %child, ptr %deref4, i32 0, i32 0
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent5, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_grandparent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      call void @____vtable_grandparent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref1, i32 0, i32 0
      store ptr @grandparent, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %gp_method = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref3, i32 0, i32 1
      call void @____vtable_grandparent_gp_method__ctor(ptr %gp_method)
      %deref4 = load ptr, ptr %self, align [filtered]
      %gp_method5 = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref4, i32 0, i32 1
      store ptr @grandparent__gp_method, ptr %gp_method5, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %gp_method = getelementptr inbounds nuw %__vtable_parent, ptr %deref3, i32 0, i32 1
      call void @____vtable_parent_gp_method__ctor(ptr %gp_method)
      %deref4 = load ptr, ptr %self, align [filtered]
      %gp_method5 = getelementptr inbounds nuw %__vtable_parent, ptr %deref4, i32 0, i32 1
      store ptr @grandparent__gp_method, ptr %gp_method5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %p_method = getelementptr inbounds nuw %__vtable_parent, ptr %deref6, i32 0, i32 2
      call void @____vtable_parent_p_method__ctor(ptr %p_method)
      %deref7 = load ptr, ptr %self, align [filtered]
      %p_method8 = getelementptr inbounds nuw %__vtable_parent, ptr %deref7, i32 0, i32 2
      store ptr @parent__p_method, ptr %p_method8, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %gp_method = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 1
      call void @____vtable_child_gp_method__ctor(ptr %gp_method)
      %deref4 = load ptr, ptr %self, align [filtered]
      %gp_method5 = getelementptr inbounds nuw %__vtable_child, ptr %deref4, i32 0, i32 1
      store ptr @grandparent__gp_method, ptr %gp_method5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %p_method = getelementptr inbounds nuw %__vtable_child, ptr %deref6, i32 0, i32 2
      call void @____vtable_child_p_method__ctor(ptr %p_method)
      %deref7 = load ptr, ptr %self, align [filtered]
      %p_method8 = getelementptr inbounds nuw %__vtable_child, ptr %deref7, i32 0, i32 2
      store ptr @parent__p_method, ptr %p_method8, align [filtered]
      %deref9 = load ptr, ptr %self, align [filtered]
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref9, i32 0, i32 3
      call void @____vtable_child_test__ctor(ptr %test)
      %deref10 = load ptr, ptr %self, align [filtered]
      %test11 = getelementptr inbounds nuw %__vtable_child, ptr %deref10, i32 0, i32 3
      store ptr @child__test, ptr %test11, align [filtered]
      ret void
    }

    define void @____vtable_grandparent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_grandparent_gp_method__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_parent_gp_method__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_parent_p_method__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_gp_method__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_p_method__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_test__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__grandparent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_grandparent__ctor(ptr @__vtable_grandparent_instance)
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_child = type { ptr }
    %parent = type { ptr, i16, ptr }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %val = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %ptr = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %ptr = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %val = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      store ptr %val, ptr %ptr, align [filtered]
      %val1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %ptr2 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %deref = load ptr, ptr %ptr2, align [filtered]
      %load_tmpVar = load i16, ptr %deref, align [filtered]
      %1 = sext i16 %load_tmpVar to i32
      %tmpVar = add i32 %1, 5
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %val1, align [filtered]
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %val = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 10, ptr %val, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %ptr = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 2
      call void @__parent_ptr__ctor(ptr %ptr)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__vtable4 = getelementptr inbounds nuw %parent, ptr %deref3, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable4, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__parent_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_child = type { ptr, ptr }
    %parent = type { ptr, i16, i16 }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %threshold = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_value = load i16, ptr %value, align [filtered]
      %1 = sext i16 %load_value to i32
      %threshold = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_threshold = load i16, ptr %threshold, align [filtered]
      %2 = sext i16 %load_threshold to i32
      %tmpVar = icmp sgt i32 %1, %2
      %3 = zext i1 %tmpVar to i8
      %4 = icmp ne i8 %3, 0
      br i1 %4, label %condition_body, label %else

    condition_body:                                   ; preds = %entry
      %value1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      store i16 0, ptr %value1, align [filtered]
      br label %continue

    else:                                             ; preds = %entry
      %value2 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      store i16 100, ptr %value2, align [filtered]
      br label %continue

    continue:                                         ; preds = %else, %condition_body
      %value4 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_value5 = load i16, ptr %value4, align [filtered]
      switch i16 %load_value5, label %else6 [
        i16 10, label %case
        i16 20, label %case8
      ]

    case:                                             ; preds = %continue
      %threshold7 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      store i16 40, ptr %threshold7, align [filtered]
      br label %continue3

    case8:                                            ; preds = %continue
      %threshold9 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      store i16 60, ptr %threshold9, align [filtered]
      br label %continue3

    else6:                                            ; preds = %continue
      br label %continue3

    continue3:                                        ; preds = %else6, %case8, %case
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %threshold = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 50, ptr %threshold, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %value = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 2
      store i16 10, ptr %value, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %__vtable4 = getelementptr inbounds nuw %parent, ptr %deref3, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable4, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 1
      call void @____vtable_child_test__ctor(ptr %test)
      %deref4 = load ptr, ptr %self, align [filtered]
      %test5 = getelementptr inbounds nuw %__vtable_child, ptr %deref4, i32 0, i32 1
      store ptr @child__test, ptr %test5, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_test__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_child = type { ptr }
    %parent = type { ptr, i16, i16 }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %MAX_VALUE = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %current = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %current = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      store i16 50, ptr %current, align [filtered]
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %current = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 2
      store i16 50, ptr %current, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_child = type { ptr, ptr }
    %parent = type { ptr, i16 }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %val = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %call = call i16 @process_ref(ptr %__parent)
      %call1 = call i16 @process_val(ptr %__parent)
      ret void
    }

    define i16 @process_ref(ptr %0) {
    entry:
      %process_ref = alloca i16, align [filtered]
      %ref = alloca ptr, align [filtered]
      store ptr %0, ptr %ref, align [filtered]
      store i16 0, ptr %process_ref, align [filtered]
      %deref = load ptr, ptr %ref, align [filtered]
      %val = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 1
      store i16 20, ptr %val, align [filtered]
      %process_ref_ret = load i16, ptr %process_ref, align [filtered]
      ret i16 %process_ref_ret
    }

    define i16 @process_val(ptr %0) {
    entry:
      %process_val = alloca i16, align [filtered]
      %val = alloca %parent, align [filtered]
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %val, ptr align [filtered] %0, i64 ptrtoint (ptr getelementptr (%parent, ptr null, i32 1) to i64), i1 false)
      store i16 0, ptr %process_val, align [filtered]
      %val1 = getelementptr inbounds nuw %parent, ptr %val, i32 0, i32 1
      store i16 30, ptr %val1, align [filtered]
      %process_val_ret = load i16, ptr %process_val, align [filtered]
      ret i16 %process_val_ret
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %val = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 10, ptr %val, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__process_ref_ref__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 1
      call void @____vtable_child_test__ctor(ptr %test)
      %deref4 = load ptr, ptr %self, align [filtered]
      %test5 = getelementptr inbounds nuw %__vtable_child, ptr %deref4, i32 0, i32 1
      store ptr @child__test, ptr %test5, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_test__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

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
    %__vtable_child = type { ptr, ptr, ptr }
    %parent = type { ptr, i16, i16, i16 }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %a = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      %c = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 3
      ret void
    }

    define i16 @parent__calc(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %a = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      %c = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 3
      %parent.calc = alloca i16, align [filtered]
      store i16 0, ptr %parent.calc, align [filtered]
      %load_a = load i16, ptr %a, align [filtered]
      %1 = sext i16 %load_a to i32
      %load_b = load i16, ptr %b, align [filtered]
      %2 = sext i16 %load_b to i32
      %load_c = load i16, ptr %c, align [filtered]
      %3 = sext i16 %load_c to i32
      %tmpVar = mul i32 %2, %3
      %tmpVar1 = add i32 %1, %tmpVar
      %4 = trunc i32 %tmpVar1 to i16
      store i16 %4, ptr %parent.calc, align [filtered]
      %parent__calc_ret = load i16, ptr %parent.calc, align [filtered]
      ret i16 %parent__calc_ret
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %child.test = alloca i16, align [filtered]
      store i16 0, ptr %child.test, align [filtered]
      %a = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_a = load i16, ptr %a, align [filtered]
      %1 = sext i16 %load_a to i32
      %b = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_b = load i16, ptr %b, align [filtered]
      %2 = sext i16 %load_b to i32
      %tmpVar = add i32 %1, %2
      %c = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 3
      %load_c = load i16, ptr %c, align [filtered]
      %3 = sext i16 %load_c to i32
      %tmpVar1 = mul i32 %tmpVar, %3
      %call = call i16 @parent__calc(ptr %__parent)
      %4 = sext i16 %call to i32
      %tmpVar2 = add i32 %tmpVar1, %4
      %a3 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_a4 = load i16, ptr %a3, align [filtered]
      %5 = sext i16 %load_a4 to i32
      %tmpVar5 = add i32 %5, 1
      %tmpVar6 = sdiv i32 %tmpVar2, %tmpVar5
      %6 = trunc i32 %tmpVar6 to i16
      store i16 %6, ptr %child.test, align [filtered]
      %child__test_ret = load i16, ptr %child.test, align [filtered]
      ret i16 %child__test_ret
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %a = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 1, ptr %a, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %b = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 2
      store i16 2, ptr %b, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %c = getelementptr inbounds nuw %parent, ptr %deref3, i32 0, i32 3
      store i16 3, ptr %c, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__vtable5 = getelementptr inbounds nuw %parent, ptr %deref4, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable5, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %calc = getelementptr inbounds nuw %__vtable_parent, ptr %deref3, i32 0, i32 1
      call void @____vtable_parent_calc__ctor(ptr %calc)
      %deref4 = load ptr, ptr %self, align [filtered]
      %calc5 = getelementptr inbounds nuw %__vtable_parent, ptr %deref4, i32 0, i32 1
      store ptr @parent__calc, ptr %calc5, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %calc = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 1
      call void @____vtable_child_calc__ctor(ptr %calc)
      %deref4 = load ptr, ptr %self, align [filtered]
      %calc5 = getelementptr inbounds nuw %__vtable_child, ptr %deref4, i32 0, i32 1
      store ptr @parent__calc, ptr %calc5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref6, i32 0, i32 2
      call void @____vtable_child_test__ctor(ptr %test)
      %deref7 = load ptr, ptr %self, align [filtered]
      %test8 = getelementptr inbounds nuw %__vtable_child, ptr %deref7, i32 0, i32 2
      store ptr @child__test, ptr %test8, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_parent_calc__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_calc__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_test__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_child = type { ptr, ptr, ptr }
    %parent = type { ptr, i16, [6 x i16] }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @__parent.arr__init = unnamed_addr constant [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %counter = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %arr = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @parent__increment(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %counter = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %arr = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      %load_counter = load i16, ptr %counter, align [filtered]
      %1 = sext i16 %load_counter to i32
      %tmpVar = add i32 %1, 1
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %counter, align [filtered]
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @child__process(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %i = alloca i16, align [filtered]
      %sum = alloca i16, align [filtered]
      store i16 0, ptr %i, align [filtered]
      store i16 0, ptr %sum, align [filtered]
      store i16 0, ptr %sum, align [filtered]
      store i16 0, ptr %i, align [filtered]
      br i1 true, label %predicate_sle, label %predicate_sge

    predicate_sle:                                    ; preds = %increment, %entry
      %1 = load i16, ptr %i, align [filtered]
      %2 = sext i16 %1 to i32
      %condition = icmp sle i32 %2, 5
      br i1 %condition, label %loop, label %continue

    predicate_sge:                                    ; preds = %increment, %entry
      %3 = load i16, ptr %i, align [filtered]
      %4 = sext i16 %3 to i32
      %condition1 = icmp sge i32 %4, 5
      br i1 %condition1, label %loop, label %continue

    loop:                                             ; preds = %predicate_sge, %predicate_sle
      %load_sum = load i16, ptr %sum, align [filtered]
      %5 = sext i16 %load_sum to i32
      %arr = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_i = load i16, ptr %i, align [filtered]
      %6 = sext i16 %load_i to i32
      %tmpVar = mul i32 1, %6
      %tmpVar2 = add i32 %tmpVar, 0
      %tmpVar3 = getelementptr inbounds [6 x i16], ptr %arr, i32 0, i32 %tmpVar2
      %load_tmpVar = load i16, ptr %tmpVar3, align [filtered]
      %7 = sext i16 %load_tmpVar to i32
      %tmpVar4 = add i32 %5, %7
      %8 = trunc i32 %tmpVar4 to i16
      store i16 %8, ptr %sum, align [filtered]
      call void @parent__increment(ptr %__parent)
      br label %increment

    increment:                                        ; preds = %loop
      %9 = load i16, ptr %i, align [filtered]
      %10 = sext i16 %9 to i32
      %next = add i32 1, %10
      %11 = trunc i32 %next to i16
      store i16 %11, ptr %i, align [filtered]
      br i1 true, label %predicate_sle, label %predicate_sge

    continue:                                         ; preds = %predicate_sge, %predicate_sle
      br label %condition_check

    condition_check:                                  ; preds = %continue6, %continue
      br i1 true, label %while_body, label %continue5

    while_body:                                       ; preds = %condition_check
      %counter = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_counter = load i16, ptr %counter, align [filtered]
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
      %load_counter14 = load i16, ptr %counter13, align [filtered]
      %15 = sext i16 %load_counter14 to i32
      %tmpVar15 = sub i32 %15, 1
      %16 = trunc i32 %tmpVar15 to i16
      store i16 %16, ptr %counter12, align [filtered]
      %counter17 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_counter18 = load i16, ptr %counter17, align [filtered]
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

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %counter = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 0, ptr %counter, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %arr = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 2
      call void @__parent_arr__ctor(ptr %arr)
      %deref3 = load ptr, ptr %self, align [filtered]
      %arr4 = getelementptr inbounds nuw %parent, ptr %deref3, i32 0, i32 2
      store [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6], ptr %arr4, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      %__vtable6 = getelementptr inbounds nuw %parent, ptr %deref5, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable6, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__parent_arr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %increment = getelementptr inbounds nuw %__vtable_parent, ptr %deref3, i32 0, i32 1
      call void @____vtable_parent_increment__ctor(ptr %increment)
      %deref4 = load ptr, ptr %self, align [filtered]
      %increment5 = getelementptr inbounds nuw %__vtable_parent, ptr %deref4, i32 0, i32 1
      store ptr @parent__increment, ptr %increment5, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %increment = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 1
      call void @____vtable_child_increment__ctor(ptr %increment)
      %deref4 = load ptr, ptr %self, align [filtered]
      %increment5 = getelementptr inbounds nuw %__vtable_child, ptr %deref4, i32 0, i32 1
      store ptr @parent__increment, ptr %increment5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %process = getelementptr inbounds nuw %__vtable_child, ptr %deref6, i32 0, i32 2
      call void @____vtable_child_process__ctor(ptr %process)
      %deref7 = load ptr, ptr %self, align [filtered]
      %process8 = getelementptr inbounds nuw %__vtable_child, ptr %deref7, i32 0, i32 2
      store ptr @child__process, ptr %process8, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_parent_increment__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_increment__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_process__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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
    %__vtable_parent = type { ptr, ptr }
    %__vtable_child = type { ptr, ptr }
    %grandparent = type { ptr }
    %parent = type { %grandparent }
    %child = type { %parent }

    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @grandparent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @grandparent__calculate(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %grandparent.calculate = alloca i16, align [filtered]
      store i16 0, ptr %grandparent.calculate, align [filtered]
      store i16 100, ptr %grandparent.calculate, align [filtered]
      %grandparent__calculate_ret = load i16, ptr %grandparent.calculate, align [filtered]
      ret i16 %grandparent__calculate_ret
    }

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @parent__calculate(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %parent.calculate = alloca i16, align [filtered]
      store i16 0, ptr %parent.calculate, align [filtered]
      %call = call i16 @grandparent__calculate(ptr %__grandparent)
      %1 = sext i16 %call to i32
      %tmpVar = add i32 %1, 50
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %parent.calculate, align [filtered]
      %parent__calculate_ret = load i16, ptr %parent.calculate, align [filtered]
      ret i16 %parent__calculate_ret
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @child__calculate(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %child.calculate = alloca i16, align [filtered]
      store i16 0, ptr %child.calculate, align [filtered]
      %call = call i16 @parent__calculate(ptr %__parent)
      %1 = sext i16 %call to i32
      %tmpVar = add i32 %1, 25
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %child.calculate, align [filtered]
      %child__calculate_ret = load i16, ptr %child.calculate, align [filtered]
      ret i16 %child__calculate_ret
    }

    define void @grandparent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0
      call void @__grandparent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %grandparent, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_grandparent_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @grandparent__ctor(ptr %__grandparent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__grandparent2 = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0
      call void @grandparent__ctor(ptr %__grandparent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__grandparent4 = getelementptr inbounds nuw %parent, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent4, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_grandparent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      call void @____vtable_grandparent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref1, i32 0, i32 0
      store ptr @grandparent, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %calculate = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref3, i32 0, i32 1
      call void @____vtable_grandparent_calculate__ctor(ptr %calculate)
      %deref4 = load ptr, ptr %self, align [filtered]
      %calculate5 = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref4, i32 0, i32 1
      store ptr @grandparent__calculate, ptr %calculate5, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %calculate = getelementptr inbounds nuw %__vtable_parent, ptr %deref3, i32 0, i32 1
      call void @____vtable_parent_calculate__ctor(ptr %calculate)
      %deref4 = load ptr, ptr %self, align [filtered]
      %calculate5 = getelementptr inbounds nuw %__vtable_parent, ptr %deref4, i32 0, i32 1
      store ptr @parent__calculate, ptr %calculate5, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %calculate = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 1
      call void @____vtable_child_calculate__ctor(ptr %calculate)
      %deref4 = load ptr, ptr %self, align [filtered]
      %calculate5 = getelementptr inbounds nuw %__vtable_child, ptr %deref4, i32 0, i32 1
      store ptr @child__calculate, ptr %calculate5, align [filtered]
      ret void
    }

    define void @____vtable_grandparent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_grandparent_calculate__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_parent_calculate__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_calculate__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__grandparent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_grandparent__ctor(ptr @__vtable_grandparent_instance)
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
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

    %__vtable_parent = type { ptr }
    %__vtable_child = type { ptr, ptr }
    %Complex_Type = type { i16, i16, float }
    %parent = type { ptr, %Complex_Type, [2 x %Complex_Type] }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @__parent.data__init = unnamed_addr constant %Complex_Type { i16 10, i16 20, float 3.050000e+01 }
    @__parent.arr_data__init = unnamed_addr constant [2 x %Complex_Type] [%Complex_Type { i16 1, i16 2, float 3.500000e+00 }, %Complex_Type { i16 4, i16 5, float 6.500000e+00 }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %data = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %arr_data = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @child__test(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %local_data = alloca %Complex_Type, align [filtered]
      call void @llvm.memset.p0.i64(ptr align [filtered] %local_data, i8 0, i64 ptrtoint (ptr getelementptr (%Complex_Type, ptr null, i32 1) to i64), i1 false)
      call void @Complex_Type__ctor(ptr %local_data)
      %x = getelementptr inbounds nuw %Complex_Type, ptr %local_data, i32 0, i32 0
      %data = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %x1 = getelementptr inbounds nuw %Complex_Type, ptr %data, i32 0, i32 0
      %load_x = load i16, ptr %x1, align [filtered]
      store i16 %load_x, ptr %x, align [filtered]
      %y = getelementptr inbounds nuw %Complex_Type, ptr %local_data, i32 0, i32 1
      %data2 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %y3 = getelementptr inbounds nuw %Complex_Type, ptr %data2, i32 0, i32 1
      %load_y = load i16, ptr %y3, align [filtered]
      store i16 %load_y, ptr %y, align [filtered]
      %z = getelementptr inbounds nuw %Complex_Type, ptr %local_data, i32 0, i32 2
      %data4 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %z5 = getelementptr inbounds nuw %Complex_Type, ptr %data4, i32 0, i32 2
      %load_z = load float, ptr %z5, align [filtered]
      store float %load_z, ptr %z, align [filtered]
      %arr_data = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %tmpVar = getelementptr inbounds [2 x %Complex_Type], ptr %arr_data, i32 0, i32 0
      %x6 = getelementptr inbounds nuw %Complex_Type, ptr %tmpVar, i32 0, i32 0
      %arr_data7 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %tmpVar8 = getelementptr inbounds [2 x %Complex_Type], ptr %arr_data7, i32 0, i32 1
      %x9 = getelementptr inbounds nuw %Complex_Type, ptr %tmpVar8, i32 0, i32 0
      %load_x10 = load i16, ptr %x9, align [filtered]
      store i16 %load_x10, ptr %x6, align [filtered]
      %arr_data11 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %tmpVar12 = getelementptr inbounds [2 x %Complex_Type], ptr %arr_data11, i32 0, i32 0
      %z13 = getelementptr inbounds nuw %Complex_Type, ptr %tmpVar12, i32 0, i32 2
      %data14 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %z15 = getelementptr inbounds nuw %Complex_Type, ptr %data14, i32 0, i32 2
      %load_z16 = load float, ptr %z15, align [filtered]
      store float %load_z16, ptr %z13, align [filtered]
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %data = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      call void @Complex_Type__ctor(ptr %data)
      %deref2 = load ptr, ptr %self, align [filtered]
      %data3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 1
      %x = getelementptr inbounds nuw %Complex_Type, ptr %data3, i32 0, i32 0
      store i16 10, ptr %x, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %data5 = getelementptr inbounds nuw %parent, ptr %deref4, i32 0, i32 1
      %y = getelementptr inbounds nuw %Complex_Type, ptr %data5, i32 0, i32 1
      store i16 20, ptr %y, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %data7 = getelementptr inbounds nuw %parent, ptr %deref6, i32 0, i32 1
      %z = getelementptr inbounds nuw %Complex_Type, ptr %data7, i32 0, i32 2
      store float 3.050000e+01, ptr %z, align [filtered]
      %deref8 = load ptr, ptr %self, align [filtered]
      %arr_data = getelementptr inbounds nuw %parent, ptr %deref8, i32 0, i32 2
      call void @__parent_arr_data__ctor(ptr %arr_data)
      %deref9 = load ptr, ptr %self, align [filtered]
      %arr_data10 = getelementptr inbounds nuw %parent, ptr %deref9, i32 0, i32 2
      store [2 x %Complex_Type] [%Complex_Type { i16 1, i16 2, float 3.500000e+00 }, %Complex_Type { i16 4, i16 5, float 6.500000e+00 }], ptr %arr_data10, align [filtered]
      %deref11 = load ptr, ptr %self, align [filtered]
      %__vtable12 = getelementptr inbounds nuw %parent, ptr %deref11, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable12, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @Complex_Type__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent_arr_data__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %test = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 1
      call void @____vtable_child_test__ctor(ptr %test)
      %deref4 = load ptr, ptr %self, align [filtered]
      %test5 = getelementptr inbounds nuw %__vtable_child, ptr %deref4, i32 0, i32 1
      store ptr @child__test, ptr %test5, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_test__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
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
    %__vtable_child = type { ptr, ptr }
    %parent = type { ptr, i16 }
    %child = type { %parent }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void
    }

    define void @parent__increment(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %load_value = load i16, ptr %value, align [filtered]
      %1 = sext i16 %load_value to i32
      %tmpVar = add i32 %1, 1
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %value, align [filtered]
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      ret void
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__parent___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %value = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1
      store i16 10, ptr %value, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @parent__ctor(ptr %__parent)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      call void @parent__ctor(ptr %__parent2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__parent4 = getelementptr inbounds nuw %child, ptr %deref3, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent4, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered]
      ret void
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      call void @____vtable_parent___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0
      store ptr @parent, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %increment = getelementptr inbounds nuw %__vtable_parent, ptr %deref3, i32 0, i32 1
      call void @____vtable_parent_increment__ctor(ptr %increment)
      %deref4 = load ptr, ptr %self, align [filtered]
      %increment5 = getelementptr inbounds nuw %__vtable_parent, ptr %deref4, i32 0, i32 1
      store ptr @parent__increment, ptr %increment5, align [filtered]
      ret void
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0
      call void @____vtable_child___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0
      store ptr @child, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %increment = getelementptr inbounds nuw %__vtable_child, ptr %deref3, i32 0, i32 1
      call void @____vtable_child_increment__ctor(ptr %increment)
      %deref4 = load ptr, ptr %self, align [filtered]
      %increment5 = getelementptr inbounds nuw %__vtable_child, ptr %deref4, i32 0, i32 1
      store ptr @parent__increment, ptr %increment5, align [filtered]
      ret void
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_parent_increment__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_child_increment__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance)
      call void @__vtable_child__ctor(ptr @__vtable_child_instance)
      ret void
    }

    define void @child__increase(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %value = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %value1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1
      %load_value = load i16, ptr %value1, align [filtered]
      %1 = sext i16 %load_value to i32
      %tmpVar = add i32 %1, 5
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %value, align [filtered]
      call void @parent__increment(ptr %__parent)
      ret void
    }
    "#);
}
