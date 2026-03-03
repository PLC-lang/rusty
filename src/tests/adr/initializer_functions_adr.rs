use crate::{
    index::const_expressions::UnresolvableKind, resolver::const_evaluator::evaluate_constants,
    test_utils::tests::index,
};
use driver::{generate_to_string, parse_and_annotate, pipelines::AnnotatedProject};
use insta::assert_debug_snapshot;
use plc_source::SourceCode;
use plc_util::filtered_assert_snapshot;

/// # Architecture Design Records: Lowering of complex initializers to initializer functions
///
/// When encountering an unresolvable initializer to a pointer during constant propagation,
/// rusty will mark this const-expression for a retry during later stages in the compilation pipeline.
#[test]
fn ref_initializer_is_marked_for_later_resolution() {
    let (_, index) = index(
        r#"
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK
       "#,
    );

    let (_, unresolvable) = evaluate_constants(index);
    assert_eq!(unresolvable.len(), 1);
    assert_eq!(unresolvable[0].get_reason(), Some(r#"Try to re-resolve during codegen"#));

    let Some(UnresolvableKind::Address(_)) = unresolvable[0].kind else { panic!() };
}

/// These unresolvables are collected and lowered during the `annotation`-stage.
/// Each POU containing such statements will have a corresponding constructor function registered
/// in the global `Index` and a new `POU` named `<pou-name>__ctor` created.
#[test]
fn ref_call_in_initializer_is_lowered_to_init_function() {
    let (_, annotated_project) = parse_and_annotate(
        "Test",
        vec![SourceCode::from(
            "
             FUNCTION_BLOCK foo
             VAR
                 s : STRING;
                 ps: REFERENCE TO STRING := REF(s);
             END_VAR
             END_FUNCTION_BLOCK
             ",
        )],
    )
    .unwrap();
    let AnnotatedProject { units, index, .. } = annotated_project;
    assert!(index.find_pou("foo__ctor").is_some());

    let units = units.iter().map(|unit| unit.get_unit()).collect::<Vec<_>>();
    let unit = &units[0];
    let init_foo_unit =
        unit.pous.iter().find(|pou| pou.name == "foo__ctor").expect("foo__ctor POU not found");

    assert_debug_snapshot!(init_foo_unit, @r#"
    POU {
        name: "foo__ctor",
        variable_blocks: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "self",
                        data_type: DataTypeReference {
                            referenced_type: "foo",
                        },
                    },
                ],
                variable_block_type: InOut,
            },
        ],
        pou_type: Init,
        return_type: None,
        interfaces: [],
        properties: [],
    }
    "#);
}

/// The thusly created function takes a single argument, a pointer to an instance of the POU to be initialized.
/// In its body, new `AstStatements` will be created; either assigning the initializer value or, for types which
/// have complex initializers themselves, calling the corresponding init function with the member-instance.
#[test]
fn initializers_are_assigned_or_delegated_to_respective_init_functions() {
    let (_, annotated_project) = parse_and_annotate(
        "Test",
        vec![SourceCode::from(
            "
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar
        VAR
            fb: foo;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM baz
        VAR
            d: DINT;
            pd AT d : DINT;
            fb: bar;
        END_VAR
        END_PROGRAM
            ",
        )],
    )
    .unwrap();
    let AnnotatedProject { units, .. } = annotated_project;
    let units = units.iter().map(|unit| unit.get_unit()).collect::<Vec<_>>();
    // the init-function for `foo` is expected to be created
    let unit = &units[0];
    let init_foo_impl = unit
        .implementations
        .iter()
        .find(|impl_| impl_.name == "foo__ctor")
        .expect("foo__ctor implementation not found");
    assert_eq!(&init_foo_impl.name, "foo__ctor");
    // Just verify that the constructor has statements (detailed snapshot testing is too fragile)
    assert!(!init_foo_impl.statements.is_empty());

    // the init-function for `bar` should also be created
    let init_bar_impl = unit
        .implementations
        .iter()
        .find(|impl_| impl_.name == "bar__ctor")
        .expect("bar__ctor implementation not found");
    assert_eq!(&init_bar_impl.name, "bar__ctor");
    assert!(!init_bar_impl.statements.is_empty());

    // the init-function for `baz` should also be created
    let init_baz_impl = unit
        .implementations
        .iter()
        .find(|impl_| impl_.name == "baz__ctor")
        .expect("baz__ctor implementation not found");
    assert_eq!(&init_baz_impl.name, "baz__ctor");
    assert!(!init_baz_impl.statements.is_empty());
}

/// duplicate symbol errors when linking with previously compiled objects.
/// collected and wrapped in a single project-level constructor function (`__unit___<unit>____ctor`).
/// This function does not take any arguments, since it only deals with global symbols. The symbol name is mangled
/// with the current project name in order to avoid duplicate symbol errors when linking with previously compiled objects.
/// Simple global variables with `REF` initializers have their respective addresses assigned,
/// PROGRAM instances will have call statements to their initialization functions generated,
/// passing the global instance as argument
#[test]
fn global_initializers_are_wrapped_in_single_init_function() {
    let (_, annotated_project) = parse_and_annotate(
        "Test",
        vec![SourceCode::from(
            "
        VAR_GLOBAL
            s : STRING;
            gs : REFERENCE TO STRING := REF(s);
        END_VAR

        FUNCTION_BLOCK foo
        VAR
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM bar
        VAR
            fb: foo;
        END_VAR
        END_PROGRAM

        PROGRAM baz
        VAR
            fb: foo;
        END_VAR
        END_PROGRAM

        PROGRAM qux
        VAR
            fb: foo;
        END_VAR
        END_PROGRAM
            ",
        )],
    )
    .unwrap();
    let AnnotatedProject { units, index, .. } = annotated_project;
    let units = units.iter().map(|unit| unit.get_unit()).collect::<Vec<_>>();

    // The ProjectInit should be named __unit___internal____ctor
    // (unit name is derived from file path which is "<internal>" for SourceCode::from)
    assert!(index.find_pou("__unit___internal____ctor").is_some());

    // The ProjectInit should be in the first (and only) unit
    let unit = &units[0];
    let init_pou = unit
        .pous
        .iter()
        .find(|pou| pou.name == "__unit___internal____ctor")
        .expect("__unit___internal____ctor POU not found");
    assert_eq!(init_pou.kind, plc_ast::ast::PouType::ProjectInit);
    assert!(init_pou.variable_blocks.is_empty());

    let init_impl = unit
        .implementations
        .iter()
        .find(|impl_| impl_.name == "__unit___internal____ctor")
        .expect("__unit___internal____ctor implementation not found");
    assert_eq!(&init_impl.name, "__unit___internal____ctor");
    // Just verify it has some initialization statements
    assert!(!init_impl.statements.is_empty());
}

/// Constructor functions are generated for all stateful POUs and structs regardless of whether or not they contain members which need them.
/// If no initialization is needed, the function-bodies will be empty. The project-level constructor is also generated unconditionally.
/// This allows each constructor to call `<member>__ctor` on its container-members in a fire-and-forget manner without having to
/// verify if a constructor function for this member exists/is required.
#[test]
fn generating_init_functions() {
    // For this first case, none of the declared structs require special initialization. Init-functions will be codegen'd anyway -
    // we rely on the optimizer to decide which functions are needed and which aren't (for now)
    let src = "
        TYPE myStruct : STRUCT
                a : BOOL;
                b : BOOL;
            END_STRUCT
        END_TYPE

        TYPE myRefStruct : STRUCT
                s : REFERENCE TO myStruct;
            END_STRUCT
        END_TYPE
        ";

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %myRefStruct = type { ptr }

    define void @myStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @myRefStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %s = getelementptr inbounds nuw %myRefStruct, ptr %deref, i32 0, i32 0
      %deref1 = load ptr, ptr %s, align [filtered]
      call void @__myRefStruct_s__ctor(ptr %deref1)
      ret void
    }

    define void @__myRefStruct_s__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }
    "#);

    // The second example shows how each initializer function delegates member-initialization to the respective member-init-function
    // The wrapping init function contains a single call-statement to `__init_baz`, since `baz` is the only global instance in need of
    // initialization
    let src = "
    TYPE myStruct : STRUCT
            a : BOOL;
            b : BOOL;
        END_STRUCT
    END_TYPE

    VAR_GLOBAL
        s: myStruct;
    END_VAR

    FUNCTION_BLOCK foo
    VAR
        ps: REF_TO myStruct := REF(s);
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK bar
    VAR
        fb: foo;
    END_VAR
    END_FUNCTION_BLOCK

    PROGRAM baz
    VAR
        fb: bar;
    END_VAR
    END_PROGRAM
    ";

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %myStruct = type { i8, i8 }
    %__vtable_foo = type { ptr }
    %__vtable_bar = type { ptr }
    %baz = type { %bar }
    %bar = type { ptr, %foo }
    %foo = type { ptr, ptr }

    @s = global %myStruct zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer
    @baz_instance = global %baz zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %ps = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %fb = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 1
      ret void
    }

    define void @baz(ptr %0) {
    entry:
      %fb = getelementptr inbounds nuw %baz, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %ps = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      call void @__foo_ps__ctor(ptr %ps)
      %deref2 = load ptr, ptr %self, align [filtered]
      %ps3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr @s, ptr %ps3, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__vtable5 = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable5, align [filtered]
      ret void
    }

    define void @bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__bar___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %fb = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 1
      call void @foo__ctor(ptr %fb)
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %bar, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @baz__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %fb = getelementptr inbounds nuw %baz, ptr %deref, i32 0, i32 0
      call void @bar__ctor(ptr %fb)
      ret void
    }

    define void @myStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_ps__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_bar, ptr %deref, i32 0, i32 0
      call void @____vtable_bar___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_bar, ptr %deref1, i32 0, i32 0
      store ptr @bar, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__bar___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_bar___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @myStruct__ctor(ptr @s)
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      call void @__vtable_bar__ctor(ptr @__vtable_bar_instance)
      call void @baz__ctor(ptr @baz_instance)
      ret void
    }
    "#);
}

/// When dealing with local stack-allocated variables (`VAR_TEMP`-blocks (in addition to `VAR` for functions)),
/// initializing these variables in a fire-and-forget manner is no longer an option, since these variables are not "stateful"
/// => they must be initialized upon every single call of the respective POU. For each of these variables, a new statement is
/// inserted at the start/at the top of the body of their parent-POU. These statements are either a simple assignment- or
/// a call-statement, depending on the assignee's datatype. Code written by the user will be executed as normal afterwards.
#[test]
fn intializing_temporary_variables() {
    let src = "
    VAR_GLOBAL
        ps: STRING;
        ps2: STRING;
    END_VAR

    FUNCTION_BLOCK foo
    VAR
        s AT ps: STRING;
    END_VAR
    VAR_TEMP
        s2: REF_TO STRING := REF(ps2);
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION main: DINT
    VAR
        fb: foo;
        s AT ps: STRING;
        s2: REF_TO STRING := REF(ps2);
    END_VAR
        fb();
    END_FUNCTION
        ";

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %foo = type { ptr, ptr }

    @ps = global [81 x i8] zeroinitializer
    @ps2 = global [81 x i8] zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %s2 = alloca ptr, align [filtered]
      store ptr @ps2, ptr %s2, align [filtered]
      call void @__foo_s2__ctor(ptr %s2)
      store ptr @ps2, ptr %s2, align [filtered]
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align [filtered]
      %fb = alloca %foo, align [filtered]
      %s = alloca ptr, align [filtered]
      %s2 = alloca ptr, align [filtered]
      call void @llvm.memset.p0.i64(ptr align [filtered] %fb, i8 0, i64 ptrtoint (ptr getelementptr (%foo, ptr null, i32 1) to i64), i1 false)
      store ptr null, ptr %s, align [filtered]
      store ptr @ps2, ptr %s2, align [filtered]
      store i32 0, ptr %main, align [filtered]
      call void @foo__ctor(ptr %fb)
      %deref = load ptr, ptr %s, align [filtered]
      call void @__main_s__ctor(ptr %deref)
      store ptr @ps, ptr %s, align [filtered]
      call void @__main_s2__ctor(ptr %s2)
      store ptr @ps2, ptr %s2, align [filtered]
      call void @foo(ptr %fb)
      %main_ret = load i32, ptr %main, align [filtered]
      ret i32 %main_ret
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %s = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      %deref2 = load ptr, ptr %s, align [filtered]
      call void @__foo_s__ctor(ptr %deref2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %s4 = getelementptr inbounds nuw %foo, ptr %deref3, i32 0, i32 1
      store ptr @ps, ptr %s4, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      %__vtable6 = getelementptr inbounds nuw %foo, ptr %deref5, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable6, align [filtered]
      ret void
    }

    define void @__foo_s__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_s2__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__main_s__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__main_s2__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#)
}

/// Initializing method variables behaves very similar to stack local variables from the previous example.
/// The main difference is that local variables can shadow the parents variables in which case the local
/// variable takes precedence.
#[test]
fn initializing_method_variables() {
    // For this first case, we focus purely on local variables where some variable is referencing another
    // variable. This example behaves exactly like the previous example with local variables in functions or
    // `VAR_TEMP` blocks.
    let src = r"
    FUNCTION_BLOCK foo
        METHOD bar
            VAR
                x   : DINT := 10;
                px : REF_TO DINT := REF(x);
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align [filtered]
      %px = alloca ptr, align [filtered]
      store i32 10, ptr %x, align [filtered]
      store ptr %x, ptr %px, align [filtered]
      store i32 10, ptr %x, align [filtered]
      call void @__foo.bar_px__ctor(ptr %px)
      store ptr %x, ptr %px, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__foo.bar_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_bar__ctor(ptr %bar)
      %deref4 = load ptr, ptr %self, align [filtered]
      %bar5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__bar, ptr %bar5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      ret void
    }
    "#);

    // When no local reference is found, the parent variable is used if present. Otherwise we look for a
    // global variable.
    let src = r"
    VAR_GLOBAL
        y : DINT;
    END_VAR

    FUNCTION_BLOCK foo
        VAR
            x : DINT := 5;
        END_VAR

        METHOD bar
            VAR
                px : REF_TO DINT := REF(x);
            END_VAR
        END_METHOD

        METHOD baz
            VAR
                px : REF_TO DINT := REF(y);
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr, ptr }
    %foo = type { ptr, i32 }

    @y = global i32 0
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %px = alloca ptr, align [filtered]
      store ptr %x, ptr %px, align [filtered]
      call void @__foo.bar_px__ctor(ptr %px)
      store ptr %x, ptr %px, align [filtered]
      ret void
    }

    define void @foo__baz(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %px = alloca ptr, align [filtered]
      store ptr @y, ptr %px, align [filtered]
      call void @__foo.baz_px__ctor(ptr %px)
      store ptr @y, ptr %px, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store i32 5, ptr %x, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @__foo.bar_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo.baz_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_bar__ctor(ptr %bar)
      %deref4 = load ptr, ptr %self, align [filtered]
      %bar5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__bar, ptr %bar5, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref6, i32 0, i32 2
      call void @____vtable_foo_baz__ctor(ptr %baz)
      %deref7 = load ptr, ptr %self, align [filtered]
      %baz8 = getelementptr inbounds nuw %__vtable_foo, ptr %deref7, i32 0, i32 2
      store ptr @foo__baz, ptr %baz8, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_baz__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      ret void
    }
    "#);

    // When both a local and a parent variable are present, the local variable takes precedence.
    let src = r"
    FUNCTION_BLOCK foo
        VAR
            x : DINT := 5;
        END_VAR

        METHOD bar
            VAR
                x   : DINT := 10;
                px : REF_TO DINT := REF(x);
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr, i32 }

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %x1 = alloca i32, align [filtered]
      %px = alloca ptr, align [filtered]
      store i32 10, ptr %x1, align [filtered]
      store ptr %x1, ptr %px, align [filtered]
      store i32 10, ptr %x1, align [filtered]
      call void @__foo.bar_px__ctor(ptr %px)
      store ptr %x1, ptr %px, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store i32 5, ptr %x, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @__foo.bar_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_bar__ctor(ptr %bar)
      %deref4 = load ptr, ptr %self, align [filtered]
      %bar5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__bar, ptr %bar5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
}

/// Initializers for external members happens in the external libraries
/// The current module defines such external initializers as declarations only
/// This also applies to any vtable initializers for external FBs
#[test]
fn external_initializers() {
    let src = r"
    {external} FUNCTION_BLOCK foo
        VAR
            x : DINT := 5;
        END_VAR
    END_FUNCTION_BLOCK

    VAR_GLOBAL
        fb_global : foo;
    END_VAR

    FUNCTION main: DINT
    VAR
        fb: foo;
    END_VAR
        fb();
    END_FUNCTION
    ";

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { ptr, i32 }
    %__vtable_foo = type { ptr }

    @fb_global = global %foo { ptr null, i32 5 }
    @__vtable_foo_instance = external global %__vtable_foo
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    declare void @foo(ptr)

    define i32 @main() {
    entry:
      %main = alloca i32, align [filtered]
      %fb = alloca %foo, align [filtered]
      call void @llvm.memset.p0.i64(ptr align [filtered] %fb, i8 0, i64 ptrtoint (ptr getelementptr (%foo, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align [filtered]
      call void @foo__ctor(ptr %fb)
      call void @foo(ptr %fb)
      %main_ret = load i32, ptr %main, align [filtered]
      ret i32 %main_ret
    }

    declare void @foo__ctor(ptr)

    declare void @__vtable_foo__ctor(ptr)

    declare void @__foo___vtable__ctor(ptr)

    declare void @____vtable_foo___body__ctor(ptr)

    define void @__unit___internal____ctor() {
    entry:
      call void @foo__ctor(ptr @fb_global)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#);
}

///
/// Initializers for external members happens in the external libraries
/// The current module defines such external initializers as declarations only
/// This also applies to any vtable initializers for external FBs
#[test]
fn external_initializers_in_fbs() {
    let src = r"
    {external} FUNCTION_BLOCK foo
        VAR
            x : DINT := 5;
        END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK main
    VAR
        fb: foo;
    END_VAR
        fb();
    END_FUNCTION_BLOCK

    VAR_GLOBAL
        main_inst : main;
    END_VAR
    ";

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { ptr, %foo }
    %foo = type { ptr, i32 }
    %__vtable_foo = type { ptr }
    %__vtable_main = type { ptr }

    @main_inst = global %main { ptr null, %foo { ptr null, i32 5 } }
    @__vtable_foo_instance = external global %__vtable_foo
    @__vtable_main_instance = global %__vtable_main zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @main(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %fb = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      call void @foo(ptr %fb)
      ret void
    }

    declare void @foo(ptr)

    declare void @foo__ctor(ptr)

    define void @main__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %main, ptr %deref, i32 0, i32 0
      call void @__main___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %fb = getelementptr inbounds nuw %main, ptr %deref1, i32 0, i32 1
      call void @foo__ctor(ptr %fb)
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %main, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_main_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    declare void @__vtable_foo__ctor(ptr)

    define void @__vtable_main__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_main, ptr %deref, i32 0, i32 0
      call void @____vtable_main___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_main, ptr %deref1, i32 0, i32 0
      store ptr @main, ptr %__body2, align [filtered]
      ret void
    }

    declare void @__foo___vtable__ctor(ptr)

    define void @__main___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    declare void @____vtable_foo___body__ctor(ptr)

    define void @____vtable_main___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @main__ctor(ptr @main_inst)
      call void @__vtable_main__ctor(ptr @__vtable_main_instance)
      ret void
    }
    "#);
}

/// In inheritance scenarios, initializers for external parents happens in the external libraries
/// The local constructors only need to call the external initializers as declarations
#[test]
fn external_inherited_initializers() {
    let src = r"
    {external} FUNCTION_BLOCK foo
        VAR
            x : DINT := 5;
        END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK bar EXTENDS foo
        VAR
            y : DINT := 10;
        END_VAR
    END_FUNCTION_BLOCK

    FUNCTION main: DINT
    VAR
        fb: bar;
    END_VAR
        fb();
    END_FUNCTION
    ";

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %__vtable_bar = type { ptr }
    %bar = type { %foo, i32 }
    %foo = type { ptr, i32 }

    @__vtable_foo_instance = external global %__vtable_foo
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    declare void @foo(ptr)

    define void @bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__foo = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 1
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align [filtered]
      %fb = alloca %bar, align [filtered]
      call void @llvm.memset.p0.i64(ptr align [filtered] %fb, i8 0, i64 ptrtoint (ptr getelementptr (%bar, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align [filtered]
      call void @bar__ctor(ptr %fb)
      call void @bar(ptr %fb)
      %main_ret = load i32, ptr %main, align [filtered]
      ret i32 %main_ret
    }

    declare void @foo__ctor(ptr)

    define void @bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @foo__ctor(ptr %__foo)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__foo2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0
      call void @foo__ctor(ptr %__foo2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %y = getelementptr inbounds nuw %bar, ptr %deref3, i32 0, i32 1
      store i32 10, ptr %y, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__foo5 = getelementptr inbounds nuw %bar, ptr %deref4, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo5, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable, align [filtered]
      ret void
    }

    declare void @__vtable_foo__ctor(ptr)

    define void @__vtable_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_bar, ptr %deref, i32 0, i32 0
      call void @____vtable_bar___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_bar, ptr %deref1, i32 0, i32 0
      store ptr @bar, ptr %__body2, align [filtered]
      ret void
    }

    declare void @__foo___vtable__ctor(ptr)

    declare void @____vtable_foo___body__ctor(ptr)

    define void @____vtable_bar___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_bar__ctor(ptr @__vtable_bar_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#);
}

/// External initializers being forward declared also applies to structs and programs
#[test]
fn external_struct_and_program_initializers() {
    let src = r"
    {external} TYPE myStruct: STRUCT
        a : DINT;
    END_STRUCT
    END_TYPE

    {external} PROGRAM baz
        VAR
            fb: myStruct;
        END_VAR
    END_PROGRAM
    FUNCTION main: DINT
    VAR
    END_VAR
        baz();
    END_FUNCTION
    ";
    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %baz = type { %myStruct }
    %myStruct = type { i32 }

    @baz_instance = external global %baz
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    declare void @baz(ptr)

    define i32 @main() {
    entry:
      %main = alloca i32, align [filtered]
      store i32 0, ptr %main, align [filtered]
      call void @baz(ptr @baz_instance)
      %main_ret = load i32, ptr %main, align [filtered]
      ret i32 %main_ret
    }

    declare void @baz__ctor(ptr)

    declare void @myStruct__ctor(ptr)

    define void @__unit___internal____ctor() {
    entry:
      call void @baz__ctor(ptr @baz_instance)
      ret void
    }
    "#);
}
