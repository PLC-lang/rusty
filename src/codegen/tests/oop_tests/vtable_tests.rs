use insta::assert_snapshot;
use test_utils::codegen;

#[test]
fn vtables_are_created_for_function_blocks() {
    let result = codegen(
        "
            FUNCTION_BLOCK Test
            METHOD TestMethod
            END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK Test2 EXTENDS Test
            END_FUNCTION_BLOCK
",
    );
    //Expecting a vtable in the function block
    //Expecting a vtable type in the types
    //Expecting a global varaible for the vtable
    assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %Test = type { i32* }
    %Test2 = type { %Test }
    %__vtable_Test_type = type { i32*, i32* }
    %__vtable_Test2_type = type { i32*, %__vtable_Test_type }

    @__Test__init = constant %Test zeroinitializer
    @__Test2__init = constant %Test2 zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_Test_type__init = constant %__vtable_Test_type zeroinitializer
    @__vtable_Test = global %__vtable_Test_type zeroinitializer
    @____vtable_Test2_type__init = constant %__vtable_Test2_type zeroinitializer
    @__vtable_Test2 = global %__vtable_Test2_type zeroinitializer

    define void @Test(%Test* %0) {
    entry:
      %__vtable = getelementptr inbounds %Test, %Test* %0, i32 0, i32 0
      ret void
    }

    define void @Test_TestMethod(%Test* %0) {
    entry:
      %__vtable = getelementptr inbounds %Test, %Test* %0, i32 0, i32 0
      ret void
    }

    define void @Test2(%Test2* %0) {
    entry:
      %__Test = getelementptr inbounds %Test2, %Test2* %0, i32 0, i32 0
      ret void
    }

    define void @__init___vtable_test_type(%__vtable_Test_type* %0) {
    entry:
      %self = alloca %__vtable_Test_type*, align 8
      store %__vtable_Test_type* %0, %__vtable_Test_type** %self, align 8
      ret void
    }

    define void @__init___vtable_test2_type(%__vtable_Test2_type* %0) {
    entry:
      %self = alloca %__vtable_Test2_type*, align 8
      store %__vtable_Test2_type* %0, %__vtable_Test2_type** %self, align 8
      %deref = load %__vtable_Test2_type*, %__vtable_Test2_type** %self, align 8
      %__vtable_Test_type = getelementptr inbounds %__vtable_Test2_type, %__vtable_Test2_type* %deref, i32 0, i32 1
      call void @__init___vtable_test_type(%__vtable_Test_type* %__vtable_Test_type)
      ret void
    }

    define void @__init_test(%Test* %0) {
    entry:
      %self = alloca %Test*, align 8
      store %Test* %0, %Test** %self, align 8
      ret void
    }

    define void @__init_test2(%Test2* %0) {
    entry:
      %self = alloca %Test2*, align 8
      store %Test2* %0, %Test2** %self, align 8
      %deref = load %Test2*, %Test2** %self, align 8
      %__Test = getelementptr inbounds %Test2, %Test2* %deref, i32 0, i32 0
      call void @__init_test(%Test* %__Test)
      ret void
    }

    define void @__user_init_Test(%Test* %0) {
    entry:
      %self = alloca %Test*, align 8
      store %Test* %0, %Test** %self, align 8
      ret void
    }

    define void @__user_init_Test2(%Test2* %0) {
    entry:
      %self = alloca %Test2*, align 8
      store %Test2* %0, %Test2** %self, align 8
      %deref = load %Test2*, %Test2** %self, align 8
      %__Test = getelementptr inbounds %Test2, %Test2* %deref, i32 0, i32 0
      call void @__user_init_Test(%Test* %__Test)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_test_type(%__vtable_Test_type* @__vtable_Test)
      call void @__init___vtable_test2_type(%__vtable_Test2_type* @__vtable_Test2)
      ret void
    }
    "#);
}

#[test]
fn vtables_are_created_for_interfaces() {
    let result = codegen(
        "
            INTERFACE TestInt
            METHOD TestMethod
            END_METHOD
            END_INTERFACE
            INTERFACE TestInt2
            END_INTERFACE",
    );
    //Expecting a vtable type in the types for the interface vtable
    //Interfaces have no vtable global variables
    assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %__vtable_TestInt_type = type { i32* }
    %__vtable_TestInt2_type = type {}

    @____vtable_TestInt_type__init = constant %__vtable_TestInt_type zeroinitializer
    @____vtable_TestInt2_type__init = constant %__vtable_TestInt2_type zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @__init___vtable_testint_type(%__vtable_TestInt_type* %0) {
    entry:
      %self = alloca %__vtable_TestInt_type*, align 8
      store %__vtable_TestInt_type* %0, %__vtable_TestInt_type** %self, align 8
      ret void
    }

    define void @__init___vtable_testint2_type(%__vtable_TestInt2_type* %0) {
    entry:
      %self = alloca %__vtable_TestInt2_type*, align 8
      store %__vtable_TestInt2_type* %0, %__vtable_TestInt2_type** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }
    "#);
}

#[test]
fn vtable_codegen_for_function_block_with_interfaces_show_interface_in_type() {
    let result = codegen(
        "
            INTERFACE TestInt
              METHOD TestMethod
              END_METHOD
            END_INTERFACE

            INTERFACE TestInt2
            END_INTERFACE

            FUNCTION_BLOCK Test IMPLEMENTS TestInt, TestInt2
              METHOD TestMethod
              END_METHOD
            END_FUNCTION_BLOCK",
    );
    //Expecting a vtable type in the types for the interface vtable
    //Interfaces have no vtable global variables
    assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %Test = type { i32* }
    %__vtable_Test_type = type { i32*, %__vtable_TestInt_type, %__vtable_TestInt2_type, i32* }
    %__vtable_TestInt_type = type { i32* }
    %__vtable_TestInt2_type = type {}

    @__Test__init = constant %Test zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_Test_type__init = constant %__vtable_Test_type zeroinitializer
    @____vtable_TestInt_type__init = constant %__vtable_TestInt_type zeroinitializer
    @____vtable_TestInt2_type__init = constant %__vtable_TestInt2_type zeroinitializer
    @__vtable_Test = global %__vtable_Test_type zeroinitializer

    define void @Test(%Test* %0) {
    entry:
      %__vtable = getelementptr inbounds %Test, %Test* %0, i32 0, i32 0
      ret void
    }

    define void @Test_TestMethod(%Test* %0) {
    entry:
      %__vtable = getelementptr inbounds %Test, %Test* %0, i32 0, i32 0
      ret void
    }

    define void @__init___vtable_test_type(%__vtable_Test_type* %0) {
    entry:
      %self = alloca %__vtable_Test_type*, align 8
      store %__vtable_Test_type* %0, %__vtable_Test_type** %self, align 8
      %deref = load %__vtable_Test_type*, %__vtable_Test_type** %self, align 8
      %__vtable_TestInt_type = getelementptr inbounds %__vtable_Test_type, %__vtable_Test_type* %deref, i32 0, i32 1
      call void @__init___vtable_testint_type(%__vtable_TestInt_type* %__vtable_TestInt_type)
      %deref1 = load %__vtable_Test_type*, %__vtable_Test_type** %self, align 8
      %__vtable_TestInt2_type = getelementptr inbounds %__vtable_Test_type, %__vtable_Test_type* %deref1, i32 0, i32 2
      call void @__init___vtable_testint2_type(%__vtable_TestInt2_type* %__vtable_TestInt2_type)
      ret void
    }

    define void @__init___vtable_testint_type(%__vtable_TestInt_type* %0) {
    entry:
      %self = alloca %__vtable_TestInt_type*, align 8
      store %__vtable_TestInt_type* %0, %__vtable_TestInt_type** %self, align 8
      ret void
    }

    define void @__init___vtable_testint2_type(%__vtable_TestInt2_type* %0) {
    entry:
      %self = alloca %__vtable_TestInt2_type*, align 8
      store %__vtable_TestInt2_type* %0, %__vtable_TestInt2_type** %self, align 8
      ret void
    }

    define void @__init_test(%Test* %0) {
    entry:
      %self = alloca %Test*, align 8
      store %Test* %0, %Test** %self, align 8
      ret void
    }

    define void @__user_init_Test(%Test* %0) {
    entry:
      %self = alloca %Test*, align 8
      store %Test* %0, %Test** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_test_type(%__vtable_Test_type* @__vtable_Test)
      ret void
    }
    "#);
}

#[test]
fn vtables_for_external_types_are_marked_as_external() {
    let result = codegen(
        "
            {external}
            FUNCTION_BLOCK Test
            METHOD TestMethod
            END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK Test2 EXTENDS Test
            END_FUNCTION_BLOCK
",
    );
    //Expecting a vtable in the function block
    //Expecting a vtable type in the types
    //Expecting a global varaible for the vtable
    assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %Test2 = type { %Test }
    %Test = type { i32* }
    %__vtable_Test_type = type { i32*, i32* }
    %__vtable_Test2_type = type { i32*, %__vtable_Test_type }

    @__Test2__init = constant %Test2 zeroinitializer
    @__Test__init = external global %Test
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_Test_type__init = constant %__vtable_Test_type zeroinitializer
    @__vtable_Test = external global %__vtable_Test_type
    @____vtable_Test2_type__init = constant %__vtable_Test2_type zeroinitializer
    @__vtable_Test2 = global %__vtable_Test2_type zeroinitializer

    declare void @Test(%Test*)

    declare void @Test_TestMethod(%Test*)

    define void @Test2(%Test2* %0) {
    entry:
      %__Test = getelementptr inbounds %Test2, %Test2* %0, i32 0, i32 0
      ret void
    }

    define void @__init___vtable_test_type(%__vtable_Test_type* %0) {
    entry:
      %self = alloca %__vtable_Test_type*, align 8
      store %__vtable_Test_type* %0, %__vtable_Test_type** %self, align 8
      ret void
    }

    define void @__init___vtable_test2_type(%__vtable_Test2_type* %0) {
    entry:
      %self = alloca %__vtable_Test2_type*, align 8
      store %__vtable_Test2_type* %0, %__vtable_Test2_type** %self, align 8
      %deref = load %__vtable_Test2_type*, %__vtable_Test2_type** %self, align 8
      %__vtable_Test_type = getelementptr inbounds %__vtable_Test2_type, %__vtable_Test2_type* %deref, i32 0, i32 1
      call void @__init___vtable_test_type(%__vtable_Test_type* %__vtable_Test_type)
      ret void
    }

    define void @__init_test2(%Test2* %0) {
    entry:
      %self = alloca %Test2*, align 8
      store %Test2* %0, %Test2** %self, align 8
      ret void
    }

    define void @__user_init_Test(%Test* %0) {
    entry:
      %self = alloca %Test*, align 8
      store %Test* %0, %Test** %self, align 8
      ret void
    }

    define void @__user_init_Test2(%Test2* %0) {
    entry:
      %self = alloca %Test2*, align 8
      store %Test2* %0, %Test2** %self, align 8
      %deref = load %Test2*, %Test2** %self, align 8
      %__Test = getelementptr inbounds %Test2, %Test2* %deref, i32 0, i32 0
      call void @__user_init_Test(%Test* %__Test)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_test_type(%__vtable_Test_type* @__vtable_Test)
      call void @__init___vtable_test2_type(%__vtable_Test2_type* @__vtable_Test2)
      ret void
    }
    "#);
}
