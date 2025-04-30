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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16 }
    %child = type { %parent }

    @__parent__init = constant %parent { i16 10 }
    @__child__init = constant %child { %parent { i16 10 } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      store i16 20, i16* %x, align 2
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16 }
    %child = type { %parent, %parent* }

    @__parent__init = constant %parent { i16 10 }
    @__child__init = constant %child { %parent { i16 10 }, %parent* null }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %p = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      store %parent* %__parent, %parent** %p, align 8
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16 }
    %child = type { %parent }

    @__parent__init = constant %parent { i16 10 }
    @__child__init = constant %child { %parent { i16 10 } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %value = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define i16 @parent_process(%parent* %0) {
    entry:
      %value = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %parent.process = alloca i16, align 2
      store i16 0, i16* %parent.process, align 2
      %load_value = load i16, i16* %value, align 2
      %1 = sext i16 %load_value to i32
      %tmpVar = mul i32 %1, 2
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, i16* %parent.process, align 2
      %parent_process_ret = load i16, i16* %parent.process, align 2
      ret i16 %parent_process_ret
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define i16 @child_process(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %child.process = alloca i16, align 2
      store i16 0, i16* %child.process, align 2
      %value = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_value = load i16, i16* %value, align 2
      %1 = sext i16 %load_value to i32
      %tmpVar = add i32 %1, 5
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, i16* %child.process, align 2
      %child_process_ret = load i16, i16* %child.process, align 2
      ret i16 %child_process_ret
    }

    define i16 @child_test(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %child.test = alloca i16, align 2
      store i16 0, i16* %child.test, align 2
      %call = call i16 @parent_process(%parent* %__parent)
      store i16 %call, i16* %child.test, align 2
      %child_test_ret = load i16, i16* %child.test, align 2
      ret i16 %child_test_ret
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16, i16 }
    %child = type { %parent, i16 }

    @__parent__init = constant %parent { i16 10, i16 20 }
    @__child__init = constant %child { %parent { i16 10, i16 20 }, i16 30 }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %y = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %x = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_x = load i16, i16* %x, align 2
      %1 = sext i16 %load_x to i32
      %y = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %load_y = load i16, i16* %y, align 2
      %2 = sext i16 %load_y to i32
      %tmpVar = mul i32 %2, 2
      %tmpVar1 = add i32 %1, %tmpVar
      %3 = trunc i32 %tmpVar1 to i16
      store i16 %3, i16* %z, align 2
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { [6 x i16] }
    %child = type { %parent, i16 }

    @__parent.arr__init = unnamed_addr constant [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6]
    @__parent__init = constant %parent { [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6] }
    @__child__init = constant %child { %parent { [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6] }, i16 3 }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %arr = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %index = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %arr = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_index = load i16, i16* %index, align 2
      %1 = sext i16 %load_index to i32
      %tmpVar = mul i32 1, %1
      %tmpVar1 = add i32 %tmpVar, 0
      %tmpVar2 = getelementptr inbounds [6 x i16], [6 x i16]* %arr, i32 0, i32 %tmpVar1
      store i16 42, i16* %tmpVar2, align 2
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { %grandparent, i16 }
    %grandparent = type { i16 }
    %child = type { %parent, i16 }

    @__parent__init = constant %parent { %grandparent { i16 10 }, i16 20 }
    @__grandparent__init = constant %grandparent { i16 10 }
    @__child__init = constant %child { %parent { %grandparent { i16 10 }, i16 20 }, i16 30 }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) {
    entry:
      %g_val = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      ret void
    }

    define i16 @grandparent_gp_method(%grandparent* %0) {
    entry:
      %g_val = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %grandparent.gp_method = alloca i16, align 2
      store i16 0, i16* %grandparent.gp_method, align 2
      %load_g_val = load i16, i16* %g_val, align 2
      store i16 %load_g_val, i16* %grandparent.gp_method, align 2
      %grandparent_gp_method_ret = load i16, i16* %grandparent.gp_method, align 2
      ret i16 %grandparent_gp_method_ret
    }

    define void @parent(%parent* %0) {
    entry:
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %p_val = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void
    }

    define i16 @parent_p_method(%parent* %0) {
    entry:
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %p_val = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %parent.p_method = alloca i16, align 2
      store i16 0, i16* %parent.p_method, align 2
      %load_p_val = load i16, i16* %p_val, align 2
      %1 = sext i16 %load_p_val to i32
      %call = call i16 @grandparent_gp_method(%grandparent* %__grandparent)
      %2 = sext i16 %call to i32
      %tmpVar = add i32 %1, %2
      %3 = trunc i32 %tmpVar to i16
      store i16 %3, i16* %parent.p_method, align 2
      %parent_p_method_ret = load i16, i16* %parent.p_method, align 2
      ret i16 %parent_p_method_ret
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %c_val = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void
    }

    define i16 @child_test(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %c_val = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %child.test = alloca i16, align 2
      store i16 0, i16* %child.test, align 2
      %call = call i16 @parent_p_method(%parent* %__parent)
      store i16 %call, i16* %child.test, align 2
      %child_test_ret = load i16, i16* %child.test, align 2
      ret i16 %child_test_ret
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__user_init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16, i16* }
    %child = type { %parent }

    @__parent__init = constant %parent { i16 10, i16* null }
    @__child__init = constant %child { %parent { i16 10, i16* null } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %val = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %ptr = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %ptr = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %val = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      store i16* %val, i16** %ptr, align 8
      %val1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %ptr2 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %deref = load i16*, i16** %ptr2, align 8
      %load_tmpVar = load i16, i16* %deref, align 2
      %1 = sext i16 %load_tmpVar to i32
      %tmpVar = add i32 %1, 5
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, i16* %val1, align 2
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16, i16 }
    %child = type { %parent }

    @__parent__init = constant %parent { i16 50, i16 10 }
    @__child__init = constant %child { %parent { i16 50, i16 10 } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %threshold = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %value = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define void @child_test(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %value = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %load_value = load i16, i16* %value, align 2
      %1 = sext i16 %load_value to i32
      %threshold = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_threshold = load i16, i16* %threshold, align 2
      %2 = sext i16 %load_threshold to i32
      %tmpVar = icmp sgt i32 %1, %2
      %3 = zext i1 %tmpVar to i8
      %4 = icmp ne i8 %3, 0
      br i1 %4, label %condition_body, label %else

    condition_body:                                   ; preds = %entry
      %value1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      store i16 0, i16* %value1, align 2
      br label %continue

    else:                                             ; preds = %entry
      %value2 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      store i16 100, i16* %value2, align 2
      br label %continue

    continue:                                         ; preds = %else, %condition_body
      %value4 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %load_value5 = load i16, i16* %value4, align 2
      switch i16 %load_value5, label %else6 [
        i16 10, label %case
        i16 20, label %case8
      ]

    case:                                             ; preds = %continue
      %threshold7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      store i16 40, i16* %threshold7, align 2
      br label %continue3

    case8:                                            ; preds = %continue
      %threshold9 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      store i16 60, i16* %threshold9, align 2
      br label %continue3

    else6:                                            ; preds = %continue
      br label %continue3

    continue3:                                        ; preds = %else6, %case8, %case
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16, i16 }
    %child = type { %parent }

    @__parent__init = constant %parent { i16 100, i16 50 }
    @__child__init = constant %child { %parent { i16 100, i16 50 } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %MAX_VALUE = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %current = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %current = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      store i16 50, i16* %current, align 2
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16 }
    %child = type { %parent }

    @__parent__init = constant %parent { i16 10 }
    @__child__init = constant %child { %parent { i16 10 } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %val = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define void @child_test(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %call = call i16 @process_ref(%parent* %__parent)
      %call1 = call i16 @process_val(%parent* %__parent)
      ret void
    }

    define i16 @process_ref(%parent* %0) {
    entry:
      %process_ref = alloca i16, align 2
      %ref = alloca %parent*, align 8
      store %parent* %0, %parent** %ref, align 8
      store i16 0, i16* %process_ref, align 2
      %deref = load %parent*, %parent** %ref, align 8
      %val = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      store i16 20, i16* %val, align 2
      %process_ref_ret = load i16, i16* %process_ref, align 2
      ret i16 %process_ref_ret
    }

    define i16 @process_val(%parent* %0) {
    entry:
      %process_val = alloca i16, align 2
      %val = alloca %parent, align 8
      %1 = bitcast %parent* %val to i8*
      %2 = bitcast %parent* %0 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 %2, i64 ptrtoint (%parent* getelementptr (%parent, %parent* null, i32 1) to i64), i1 false)
      store i16 0, i16* %process_val, align 2
      %val1 = getelementptr inbounds %parent, %parent* %val, i32 0, i32 0
      store i16 30, i16* %val1, align 2
      %process_val_ret = load i16, i16* %process_val, align 2
      ret i16 %process_val_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn }
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16, i16, i16 }
    %child = type { %parent }

    @__parent__init = constant %parent { i16 1, i16 2, i16 3 }
    @__child__init = constant %child { %parent { i16 1, i16 2, i16 3 } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %a = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %c = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void
    }

    define i16 @parent_calc(%parent* %0) {
    entry:
      %a = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %c = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      %parent.calc = alloca i16, align 2
      store i16 0, i16* %parent.calc, align 2
      %load_a = load i16, i16* %a, align 2
      %1 = sext i16 %load_a to i32
      %load_b = load i16, i16* %b, align 2
      %2 = sext i16 %load_b to i32
      %load_c = load i16, i16* %c, align 2
      %3 = sext i16 %load_c to i32
      %tmpVar = mul i32 %2, %3
      %tmpVar1 = add i32 %1, %tmpVar
      %4 = trunc i32 %tmpVar1 to i16
      store i16 %4, i16* %parent.calc, align 2
      %parent_calc_ret = load i16, i16* %parent.calc, align 2
      ret i16 %parent_calc_ret
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define i16 @child_test(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %child.test = alloca i16, align 2
      store i16 0, i16* %child.test, align 2
      %a = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_a = load i16, i16* %a, align 2
      %1 = sext i16 %load_a to i32
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %load_b = load i16, i16* %b, align 2
      %2 = sext i16 %load_b to i32
      %tmpVar = add i32 %1, %2
      %c = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2
      %load_c = load i16, i16* %c, align 2
      %3 = sext i16 %load_c to i32
      %tmpVar1 = mul i32 %tmpVar, %3
      %call = call i16 @parent_calc(%parent* %__parent)
      %4 = sext i16 %call to i32
      %tmpVar2 = add i32 %tmpVar1, %4
      %a3 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_a4 = load i16, i16* %a3, align 2
      %5 = sext i16 %load_a4 to i32
      %tmpVar5 = add i32 %5, 1
      %tmpVar6 = sdiv i32 %tmpVar2, %tmpVar5
      %6 = trunc i32 %tmpVar6 to i16
      store i16 %6, i16* %child.test, align 2
      %child_test_ret = load i16, i16* %child.test, align 2
      ret i16 %child_test_ret
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16, [6 x i16] }
    %child = type { %parent }

    @__parent.arr__init = unnamed_addr constant [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6]
    @__parent__init = constant %parent { i16 0, [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6] }
    @__child__init = constant %child { %parent { i16 0, [6 x i16] [i16 1, i16 2, i16 3, i16 4, i16 5, i16 6] } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %counter = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %arr = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void
    }

    define void @parent_increment(%parent* %0) {
    entry:
      %counter = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %arr = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %load_counter = load i16, i16* %counter, align 2
      %1 = sext i16 %load_counter to i32
      %tmpVar = add i32 %1, 1
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, i16* %counter, align 2
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define void @child_process(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %i = alloca i16, align 2
      %sum = alloca i16, align 2
      store i16 0, i16* %i, align 2
      store i16 0, i16* %sum, align 2
      store i16 0, i16* %i, align 2
      br i1 true, label %predicate_sle, label %predicate_sge

    predicate_sle:                                    ; preds = %increment, %entry
      %1 = load i16, i16* %i, align 2
      %2 = sext i16 %1 to i32
      %condition = icmp sle i32 %2, 5
      br i1 %condition, label %loop, label %continue

    predicate_sge:                                    ; preds = %increment, %entry
      %3 = load i16, i16* %i, align 2
      %4 = sext i16 %3 to i32
      %condition1 = icmp sge i32 %4, 5
      br i1 %condition1, label %loop, label %continue

    loop:                                             ; preds = %predicate_sge, %predicate_sle
      %load_sum = load i16, i16* %sum, align 2
      %5 = sext i16 %load_sum to i32
      %arr = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %load_i = load i16, i16* %i, align 2
      %6 = sext i16 %load_i to i32
      %tmpVar = mul i32 1, %6
      %tmpVar2 = add i32 %tmpVar, 0
      %tmpVar3 = getelementptr inbounds [6 x i16], [6 x i16]* %arr, i32 0, i32 %tmpVar2
      %load_tmpVar = load i16, i16* %tmpVar3, align 2
      %7 = sext i16 %load_tmpVar to i32
      %tmpVar4 = add i32 %5, %7
      %8 = trunc i32 %tmpVar4 to i16
      store i16 %8, i16* %sum, align 2
      call void @parent_increment(%parent* %__parent)
      br label %increment

    increment:                                        ; preds = %loop
      %9 = load i16, i16* %i, align 2
      %10 = sext i16 %9 to i32
      %next = add i32 1, %10
      %11 = trunc i32 %next to i16
      store i16 %11, i16* %i, align 2
      br i1 true, label %predicate_sle, label %predicate_sge

    continue:                                         ; preds = %predicate_sge, %predicate_sle
      br label %condition_check

    condition_check:                                  ; preds = %continue6, %continue
      br i1 true, label %while_body, label %continue5

    while_body:                                       ; preds = %condition_check
      %counter = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_counter = load i16, i16* %counter, align 2
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
      call void @parent_increment(%parent* %__parent)
      br label %condition_check

    condition_check9:                                 ; preds = %continue16, %continue5
      br i1 true, label %while_body10, label %continue11

    while_body10:                                     ; preds = %condition_check9
      %counter12 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %counter13 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_counter14 = load i16, i16* %counter13, align 2
      %15 = sext i16 %load_counter14 to i32
      %tmpVar15 = sub i32 %15, 1
      %16 = trunc i32 %tmpVar15 to i16
      store i16 %16, i16* %counter12, align 2
      %counter17 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_counter18 = load i16, i16* %counter17, align 2
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

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { %grandparent }
    %grandparent = type {}
    %child = type { %parent }

    @__parent__init = constant %parent zeroinitializer
    @__grandparent__init = constant %grandparent zeroinitializer
    @__child__init = constant %child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) {
    entry:
      ret void
    }

    define i16 @grandparent_calculate(%grandparent* %0) {
    entry:
      %grandparent.calculate = alloca i16, align 2
      store i16 0, i16* %grandparent.calculate, align 2
      store i16 100, i16* %grandparent.calculate, align 2
      %grandparent_calculate_ret = load i16, i16* %grandparent.calculate, align 2
      ret i16 %grandparent_calculate_ret
    }

    define void @parent(%parent* %0) {
    entry:
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define i16 @parent_calculate(%parent* %0) {
    entry:
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %parent.calculate = alloca i16, align 2
      store i16 0, i16* %parent.calculate, align 2
      %call = call i16 @grandparent_calculate(%grandparent* %__grandparent)
      %1 = sext i16 %call to i32
      %tmpVar = add i32 %1, 50
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, i16* %parent.calculate, align 2
      %parent_calculate_ret = load i16, i16* %parent.calculate, align 2
      ret i16 %parent_calculate_ret
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define i16 @child_calculate(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %child.calculate = alloca i16, align 2
      store i16 0, i16* %child.calculate, align 2
      %call = call i16 @parent_calculate(%parent* %__parent)
      %1 = sext i16 %call to i32
      %tmpVar = add i32 %1, 25
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, i16* %child.calculate, align 2
      %child_calculate_ret = load i16, i16* %child.calculate, align 2
      ret i16 %child_calculate_ret
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__user_init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#""#);
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %Complex_Type = type { i16, i16, float }
    %parent = type { %Complex_Type, [2 x %Complex_Type] }
    %child = type { %parent }

    @__parent.data__init = unnamed_addr constant %Complex_Type { i16 10, i16 20, float 3.050000e+01 }
    @__parent.arr_data__init = unnamed_addr constant [2 x %Complex_Type] [%Complex_Type { i16 1, i16 2, float 3.500000e+00 }, %Complex_Type { i16 4, i16 5, float 6.500000e+00 }]
    @__Complex_Type__init = constant %Complex_Type zeroinitializer
    @__parent__init = constant %parent { %Complex_Type { i16 10, i16 20, float 3.050000e+01 }, [2 x %Complex_Type] [%Complex_Type { i16 1, i16 2, float 3.500000e+00 }, %Complex_Type { i16 4, i16 5, float 6.500000e+00 }] }
    @__child__init = constant %child { %parent { %Complex_Type { i16 10, i16 20, float 3.050000e+01 }, [2 x %Complex_Type] [%Complex_Type { i16 1, i16 2, float 3.500000e+00 }, %Complex_Type { i16 4, i16 5, float 6.500000e+00 }] } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %data = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %arr_data = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define void @child_test(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %local_data = alloca %Complex_Type, align 8
      %1 = bitcast %Complex_Type* %local_data to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%Complex_Type* @__Complex_Type__init to i8*), i64 ptrtoint (%Complex_Type* getelementptr (%Complex_Type, %Complex_Type* null, i32 1) to i64), i1 false)
      call void @__init_complex_type(%Complex_Type* %local_data)
      call void @__user_init_Complex_Type(%Complex_Type* %local_data)
      %x = getelementptr inbounds %Complex_Type, %Complex_Type* %local_data, i32 0, i32 0
      %data = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %x1 = getelementptr inbounds %Complex_Type, %Complex_Type* %data, i32 0, i32 0
      %load_x = load i16, i16* %x1, align 2
      store i16 %load_x, i16* %x, align 2
      %y = getelementptr inbounds %Complex_Type, %Complex_Type* %local_data, i32 0, i32 1
      %data2 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %y3 = getelementptr inbounds %Complex_Type, %Complex_Type* %data2, i32 0, i32 1
      %load_y = load i16, i16* %y3, align 2
      store i16 %load_y, i16* %y, align 2
      %z = getelementptr inbounds %Complex_Type, %Complex_Type* %local_data, i32 0, i32 2
      %data4 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %z5 = getelementptr inbounds %Complex_Type, %Complex_Type* %data4, i32 0, i32 2
      %load_z = load float, float* %z5, align 4
      store float %load_z, float* %z, align 4
      %arr_data = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %tmpVar = getelementptr inbounds [2 x %Complex_Type], [2 x %Complex_Type]* %arr_data, i32 0, i32 0
      %x6 = getelementptr inbounds %Complex_Type, %Complex_Type* %tmpVar, i32 0, i32 0
      %arr_data7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %tmpVar8 = getelementptr inbounds [2 x %Complex_Type], [2 x %Complex_Type]* %arr_data7, i32 0, i32 1
      %x9 = getelementptr inbounds %Complex_Type, %Complex_Type* %tmpVar8, i32 0, i32 0
      %load_x10 = load i16, i16* %x9, align 2
      store i16 %load_x10, i16* %x6, align 2
      %arr_data11 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1
      %tmpVar12 = getelementptr inbounds [2 x %Complex_Type], [2 x %Complex_Type]* %arr_data11, i32 0, i32 0
      %z13 = getelementptr inbounds %Complex_Type, %Complex_Type* %tmpVar12, i32 0, i32 2
      %data14 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %z15 = getelementptr inbounds %Complex_Type, %Complex_Type* %data14, i32 0, i32 2
      %load_z16 = load float, float* %z15, align 4
      store float %load_z16, float* %z13, align 4
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    define void @__init_complex_type(%Complex_Type* %0) {
    entry:
      %self = alloca %Complex_Type*, align 8
      store %Complex_Type* %0, %Complex_Type** %self, align 8
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %data = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_complex_type(%Complex_Type* %data)
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_Complex_Type(%Complex_Type* %0) {
    entry:
      %self = alloca %Complex_Type*, align 8
      store %Complex_Type* %0, %Complex_Type** %self, align 8
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %data = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__user_init_Complex_Type(%Complex_Type* %data)
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn }
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %parent = type { i16 }
    %child = type { %parent }

    @__parent__init = constant %parent { i16 10 }
    @__child__init = constant %child { %parent { i16 10 } }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) {
    entry:
      %value = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define void @parent_increment(%parent* %0) {
    entry:
      %value = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %load_value = load i16, i16* %value, align 2
      %1 = sext i16 %load_value to i32
      %tmpVar = add i32 %1, 1
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, i16* %value, align 2
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define void @child_increase(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %value = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %value1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %load_value = load i16, i16* %value1, align 2
      %1 = sext i16 %load_value to i32
      %tmpVar = add i32 %1, 5
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, i16* %value, align 2
      call void @parent_increment(%parent* %__parent)
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }
    "#);
}
