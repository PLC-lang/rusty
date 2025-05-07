use driver::generate_to_string;
use plc_source::SourceCode;

fn codegen(source: &str) -> String {
    generate_to_string("Test", vec![SourceCode::from(source)]).unwrap()
}

#[test]
fn function_block_without_parent() {
    let result = codegen(
        "
        FUNCTION_BLOCK fb
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %fb = type { i32* }
    %__vtable_fb_type = type { i32*, i32* }

    @__fb__init = constant %fb zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_fb_type__init = constant %__vtable_fb_type zeroinitializer
    @__vtable_fb = global %__vtable_fb_type zeroinitializer

    define void @fb(%fb* %0) {
    entry:
      %__vtable = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      ret void
    }

    define void @fb_foo(%fb* %0) {
    entry:
      %__vtable = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      ret void
    }

    define void @__init___vtable_fb_type(%__vtable_fb_type* %0) {
    entry:
      %self = alloca %__vtable_fb_type*, align 8
      store %__vtable_fb_type* %0, %__vtable_fb_type** %self, align 8
      ret void
    }

    define void @__init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      %deref = load %fb*, %fb** %self, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_fb_type* @__vtable_fb to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_type(%__vtable_fb_type* @__vtable_fb)
      ret void
    }
    "###);
}

#[test]
fn function_block_with_parent() {
    let result = codegen(
        "
        FUNCTION_BLOCK parent
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD bar
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %parent = type { i32* }
    %child = type { %parent }
    %__vtable_parent_type = type { i32*, i32* }
    %__vtable_child_type = type { %__vtable_parent_type, i32*, i32* }

    @__parent__init = constant %parent zeroinitializer
    @__child__init = constant %child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer
    @__vtable_parent = global %__vtable_parent_type zeroinitializer
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer
    @__vtable_child = global %__vtable_child_type zeroinitializer

    define void @parent(%parent* %0) {
    entry:
      %__vtable = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define void @parent_foo(%parent* %0) {
    entry:
      %__vtable = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define void @child_bar(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define void @__init___vtable_parent_type(%__vtable_parent_type* %0) {
    entry:
      %self = alloca %__vtable_parent_type*, align 8
      store %__vtable_parent_type* %0, %__vtable_parent_type** %self, align 8
      ret void
    }

    define void @__init___vtable_child_type(%__vtable_child_type* %0) {
    entry:
      %self = alloca %__vtable_child_type*, align 8
      store %__vtable_child_type* %0, %__vtable_child_type** %self, align 8
      %deref = load %__vtable_child_type*, %__vtable_child_type** %self, align 8
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 0
      call void @__init___vtable_parent_type(%__vtable_parent_type* %__vtable_parent_type)
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__vtable = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_parent_type* @__vtable_parent to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      %deref1 = load %child*, %child** %self, align 8
      %__parent2 = getelementptr inbounds %child, %child* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0
      store i32* bitcast (%__vtable_child_type* @__vtable_child to i32*), i32** %__vtable, align 8
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
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      ret void
    }
    "###);
}

#[test]
fn function_block_with_parent_chained() {
    let result = codegen(
        "
        FUNCTION_BLOCK grandparent
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
            METHOD bar
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD baz
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %parent = type { %grandparent }
    %grandparent = type { i32* }
    %child = type { %parent }
    %__vtable_grandparent_type = type { i32*, i32* }
    %__vtable_parent_type = type { %__vtable_grandparent_type, i32*, i32* }
    %__vtable_child_type = type { %__vtable_parent_type, i32*, i32* }

    @__parent__init = constant %parent zeroinitializer
    @__grandparent__init = constant %grandparent zeroinitializer
    @__child__init = constant %child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent_type__init = constant %__vtable_grandparent_type zeroinitializer
    @__vtable_grandparent = global %__vtable_grandparent_type zeroinitializer
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer
    @__vtable_parent = global %__vtable_parent_type zeroinitializer
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer
    @__vtable_child = global %__vtable_child_type zeroinitializer

    define void @grandparent(%grandparent* %0) {
    entry:
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      ret void
    }

    define void @grandparent_foo(%grandparent* %0) {
    entry:
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      ret void
    }

    define void @parent(%parent* %0) {
    entry:
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define void @parent_bar(%parent* %0) {
    entry:
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define void @child_baz(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      ret void
    }

    define void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %0) {
    entry:
      %self = alloca %__vtable_grandparent_type*, align 8
      store %__vtable_grandparent_type* %0, %__vtable_grandparent_type** %self, align 8
      ret void
    }

    define void @__init___vtable_parent_type(%__vtable_parent_type* %0) {
    entry:
      %self = alloca %__vtable_parent_type*, align 8
      store %__vtable_parent_type* %0, %__vtable_parent_type** %self, align 8
      %deref = load %__vtable_parent_type*, %__vtable_parent_type** %self, align 8
      %__vtable_grandparent_type = getelementptr inbounds %__vtable_parent_type, %__vtable_parent_type* %deref, i32 0, i32 0
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %__vtable_grandparent_type)
      ret void
    }

    define void @__init___vtable_child_type(%__vtable_child_type* %0) {
    entry:
      %self = alloca %__vtable_child_type*, align 8
      store %__vtable_child_type* %0, %__vtable_child_type** %self, align 8
      %deref = load %__vtable_child_type*, %__vtable_child_type** %self, align 8
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 0
      call void @__init___vtable_parent_type(%__vtable_parent_type* %__vtable_parent_type)
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      %deref1 = load %parent*, %parent** %self, align 8
      %__grandparent2 = getelementptr inbounds %parent, %parent* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent2, i32 0, i32 0
      store i32* bitcast (%__vtable_parent_type* @__vtable_parent to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      %deref = load %grandparent*, %grandparent** %self, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_grandparent_type* @__vtable_grandparent to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      %deref1 = load %child*, %child** %self, align 8
      %__parent2 = getelementptr inbounds %child, %child* %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0
      store i32* bitcast (%__vtable_child_type* @__vtable_child to i32*), i32** %__vtable, align 8
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
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* @__vtable_grandparent)
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      ret void
    }
    "###);
}
