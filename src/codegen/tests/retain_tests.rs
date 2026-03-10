use plc_util::filtered_assert_snapshot;
use test_utils::codegen;

#[test]
fn retain_variables_in_global_are_in_retain_linker_section() {
    let res = codegen(
        "
        VAR_GLOBAL RETAIN
            x : INT;
            y : STRING;
        END_VAR
        ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @x = global i16 0, section ".retain"
    @y = global [81 x i8] zeroinitializer, section ".retain"
    "#);
}

#[test]
fn retain_variables_in_programs_are_in_retain_linker_section() {
    let res = codegen(
        "
        PROGRAM main
        VAR RETAIN
            x : INT;
            y : STRING;
        END_VAR
        END_PROGRAM
        ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { ptr, ptr }

    @__main_x__retain = global i16 0, section ".retain"
    @__main_y__retain = global [81 x i8] zeroinitializer, section ".retain"
    @main_instance = global %main zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @main(ptr %0) {
    entry:
      %x = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      ret void
    }

    define void @main__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %main, ptr %deref, i32 0, i32 0
      %deref1 = load ptr, ptr %x, align [filtered]
      call void @__main_x__ctor(ptr %deref1)
      %deref2 = load ptr, ptr %self, align [filtered]
      %x3 = getelementptr inbounds nuw %main, ptr %deref2, i32 0, i32 0
      store ptr @__main_x__retain, ptr %x3, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %y = getelementptr inbounds nuw %main, ptr %deref4, i32 0, i32 1
      %deref5 = load ptr, ptr %y, align [filtered]
      call void @__main_y__ctor(ptr %deref5)
      %deref6 = load ptr, ptr %self, align [filtered]
      %y7 = getelementptr inbounds nuw %main, ptr %deref6, i32 0, i32 1
      store ptr @__main_y__retain, ptr %y7, align [filtered]
      ret void
    }

    define void @__main_x__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__main_y__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @main__ctor(ptr @main_instance)
      ret void
    }
    "#);
}

#[test]
fn nested_retain_variables_are_in_the_retain_section() {
    let res = codegen(
        "
        FUNCTION_BLOCK fb
        VAR RETAIN
            x : INT;
            y : STRING;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL
            fb_instance : fb;
        END_VAR
        ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_fb = type { ptr }
    %fb = type { ptr, i16, [81 x i8] }

    @__vtable_fb_instance = global %__vtable_fb zeroinitializer
    @fb_instance = global %fb zeroinitializer, section ".retain"
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @fb(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 2
      ret void
    }

    define void @fb__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %fb, ptr %deref, i32 0, i32 0
      call void @__fb___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %fb, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_fb_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__vtable_fb__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      call void @____vtable_fb___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_fb, ptr %deref1, i32 0, i32 0
      store ptr @fb, ptr %__body2, align [filtered]
      ret void
    }

    define void @__fb___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_fb___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_fb__ctor(ptr @__vtable_fb_instance)
      call void @fb__ctor(ptr @fb_instance)
      ret void
    }
    "#);
}
