use insta::{assert_debug_snapshot, assert_snapshot};
use plc_ast::provider::IdProvider;

use crate::{
    index::const_expressions::UnresolvableKind,
    resolver::const_evaluator::evaluate_constants,
    test_utils::tests::{annotate_and_lower_with_ids, codegen, index, index_with_ids},
};

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
/// Each POU containing such statements will have a corresponding init function registered
/// in the global `Index` and a new `POU` named `__init_<pou-name>` created.
#[test]
fn ref_call_in_initializer_is_lowered_to_init_function() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REFERENCE TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK
        ",
        id_provider.clone(),
    );
    let (_, index, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);

    assert!(index.find_pou("__init_foo").is_some());

    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();
    let init_foo_unit = &units[1].units[0];

    assert_debug_snapshot!(init_foo_unit, @r###"
    POU {
        name: "__init_foo",
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
    }
    "###);
}

/// The thusly created function takes a single argument, a pointer to an instance of the POU to be initialized.
/// In its body, new `AstStatements` will be created; either assigning the initializer value or, for types which
/// have complex initializers themselves, calling the corresponding init function with the member-instance.
#[test]
fn initializers_are_assigned_or_delegated_to_respective_init_functions() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (_, _, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();
    // the init-function for `foo` is expected to have a single assignment statement in its function body
    let init_foo_impl = &units[1].implementations[0];
    assert_eq!(&init_foo_impl.name, "__init_foo");
    let statements = &init_foo_impl.statements;
    assert_eq!(statements.len(), 1);
    assert_debug_snapshot!(statements[0], @r#"
    Assignment {
        left: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "ps",
                },
            ),
            base: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "self",
                        },
                    ),
                    base: None,
                },
            ),
        },
        right: CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "REF",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "s",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "self",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            ),
        },
    }
    "#);

    // the init-function for `bar` will have a `CallStatement` to `__init_foo` as its only statement, passing the member-instance `self.fb`
    let init_bar_impl = &units[1].implementations[1];
    assert_eq!(&init_bar_impl.name, "__init_bar");
    let statements = &init_bar_impl.statements;
    assert_eq!(statements.len(), 1);
    assert_debug_snapshot!(statements[0], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_foo",
                },
            ),
            base: None,
        },
        parameters: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "fb",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "self",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ),
    }
    "###);

    // the init-function for `baz` will have a `RefAssignment`, assigning `REF(d)` to `self.pd` (TODO: currently, it actually is an `Assignment`
    // in the AST which is redirected to `generate_ref_assignment` in codegen) followed by a `CallStatement` to `__init_bar`,
    // passing the member-instance `self.fb`
    let init_baz_impl = &units[1].implementations[2];
    assert_eq!(&init_baz_impl.name, "__init_baz");
    let statements = &init_baz_impl.statements;
    assert_eq!(statements.len(), 2);
    assert_debug_snapshot!(statements[0], @r#"
    Assignment {
        left: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "pd",
                },
            ),
            base: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "self",
                        },
                    ),
                    base: None,
                },
            ),
        },
        right: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "d",
                },
            ),
            base: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "self",
                        },
                    ),
                    base: None,
                },
            ),
        },
    }
    "#);

    assert_debug_snapshot!(statements[1], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_bar",
                },
            ),
            base: None,
        },
        parameters: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "fb",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "self",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ),
    }
    "###);
}

/// Finally, after lowering each individual init function, all global initializers are
/// collected and wrapped in a single `__init___<projectname>` function. This function does not take any arguments,
/// since it only deals with global symbols. The symbol name is mangled with the current project name in order to avoid
/// duplicate symbol errors when linking with previously compiled objects.
/// collected and wrapped in a single `__init___<projectname>` function. This function does not take any arguments,
/// since it only deals with global symbols. The symbol name is mangled with the current project name in order to avoid
/// duplicate symbol errors when linking with previously compiled objects.
/// Simple global variables with `REF` initializers have their respective addresses assigned,
/// PROGRAM instances will have call statements to their initialization functions generated,
/// passing the global instance as argument
#[test]
fn global_initializers_are_wrapped_in_single_init_function() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (_, index, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);

    assert!(index.find_pou("__init___testproject").is_some());

    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();

    let init = &units[2].units[0];
    assert_debug_snapshot!(init, @r###"
    POU {
        name: "__init___testproject",
        variable_blocks: [],
        pou_type: ProjectInit,
        return_type: None,
        interfaces: [],
    }
    "###);

    let init_impl = &units[2].implementations[0];
    assert_eq!(&init_impl.name, "__init___testproject");
    assert_eq!(init_impl.statements.len(), 4);
    // global variable blocks are initialized first, hence we expect the first statement in the `__init` body to be an
    // `Assignment`, assigning `REF(s)` to `gs`. This is followed by three `CallStatements`, one for each global `PROGRAM`
    // instance.
    assert_debug_snapshot!(&init_impl.statements[0], @r###"
    Assignment {
        left: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "gs",
                },
            ),
            base: None,
        },
        right: CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "REF",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "s",
                        },
                    ),
                    base: None,
                },
            ),
        },
    }
    "###);
    assert_debug_snapshot!(&init_impl.statements[1], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_baz",
                },
            ),
            base: None,
        },
        parameters: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "baz",
                    },
                ),
                base: None,
            },
        ),
    }
    "###);
    assert_debug_snapshot!(&init_impl.statements[2], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_bar",
                },
            ),
            base: None,
        },
        parameters: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "bar",
                    },
                ),
                base: None,
            },
        ),
    }
    "###);
    assert_debug_snapshot!(&init_impl.statements[3], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_qux",
                },
            ),
            base: None,
        },
        parameters: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "qux",
                    },
                ),
                base: None,
            },
        ),
    }
    "###);
}

/// Initializer functions are generated for all stateful POUs and structs regardless of whether or not they contain members which need them.
/// If no initialization is needed, the function-bodies will be empty. The wrapping initializer for the project is also generated unconditionally.
/// This allows each initializer to call `__init_<member>` on its container-members in a fire-and-forget manner without having to
/// verify if an initializer function for this member exists/is required.
/// Initializer functions are generated in two modules, one containing all dedicated POU/struct initializers and another one containing only the
/// final project initializer, wrapping call statements to each to-be-initialized global in use.
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

    let res = codegen(src);
    assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %myStruct = type { i8, i8 }
    %myRefStruct = type { %myStruct* }

    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer
    @__myRefStruct__init = unnamed_addr constant %myRefStruct zeroinitializer
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %myStruct = type { i8, i8 }
    %myRefStruct = type { %myStruct* }

    @__myStruct__init = external global %myStruct
    @__myRefStruct__init = external global %myRefStruct

    define void @__init_mystruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init_myrefstruct(%myRefStruct* %0) {
    entry:
      %self = alloca %myRefStruct*, align 8
      store %myRefStruct* %0, %myRefStruct** %self, align 8
      ret void
    }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "###);

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
        fb: bar;
    END_VAR
    END_PROGRAM
    ";

    let res = codegen(src);
    assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %myStruct = type { i8, i8 }
    %foo = type { [81 x i8]* }
    %bar = type { %foo }
    %baz = type { %bar }

    @s = global %myStruct zeroinitializer
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer
    @baz_instance = global %baz zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %ps = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      %fb = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void
    }

    define void @baz(%baz* %0) {
    entry:
      %fb = getelementptr inbounds %baz, %baz* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %bar = type { %foo }
    %foo = type { [81 x i8]* }
    %myStruct = type { i8, i8 }
    %baz = type { %bar }

    @__bar__init = external global %bar
    @__foo__init = external global %foo
    @__myStruct__init = external global %myStruct
    @baz_instance = external global %baz
    @s = external global %myStruct

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %fb = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %fb)
      ret void
    }

    declare void @bar(%bar*)

    declare void @foo(%foo*)

    define void @__init_mystruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %ps = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* bitcast (%myStruct* @s to [81 x i8]*), [81 x i8]** %ps, align 8
      ret void
    }

    define void @__init_baz(%baz* %0) {
    entry:
      %self = alloca %baz*, align 8
      store %baz* %0, %baz** %self, align 8
      %deref = load %baz*, %baz** %self, align 8
      %fb = getelementptr inbounds %baz, %baz* %deref, i32 0, i32 0
      call void @__init_bar(%bar* %fb)
      ret void
    }

    declare void @baz(%baz*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %baz = type { %bar }
    %bar = type { %foo }
    %foo = type { [81 x i8]* }
    %myStruct = type { i8, i8 }

    @baz_instance = external global %baz
    @__bar__init = external global %bar
    @__foo__init = external global %foo
    @__myStruct__init = external global %myStruct
    @s = external global %myStruct
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_baz(%baz* @baz_instance)
      call void @__init_mystruct(%myStruct* @s)
      ret void
    }

    declare void @__init_baz(%baz*)

    declare void @baz(%baz*)

    declare void @bar(%bar*)

    declare void @foo(%foo*)

    declare void @__init_mystruct(%myStruct*)
    "###);
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

    let res = codegen(src);
    assert_snapshot!(res, @r##"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8]* }

    @ps = global [81 x i8] zeroinitializer
    @ps2 = global [81 x i8] zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s2 = alloca [81 x i8]*, align 8
      store [81 x i8]* @ps2, [81 x i8]** %s2, align 8
      store [81 x i8]* @ps2, [81 x i8]** %s2, align 8
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %fb = alloca %foo, align 8
      %s = alloca [81 x i8]*, align 8
      %s2 = alloca [81 x i8]*, align 8
      %0 = bitcast %foo* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%foo* @__foo__init to i8*), i64 ptrtoint (%foo* getelementptr (%foo, %foo* null, i32 1) to i64), i1 false)
      store [81 x i8]* null, [81 x i8]** %s, align 8
      store [81 x i8]* @ps2, [81 x i8]** %s2, align 8
      store i32 0, i32* %main, align 4
      store [81 x i8]* @ps, [81 x i8]** %s, align 8
      store [81 x i8]* @ps2, [81 x i8]** %s2, align 8
      call void @__init_foo(%foo* %fb)
      call void @foo(%foo* %fb)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    declare void @__init_foo(%foo*)

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8]* }

    @__foo__init = external global %foo
    @ps = external global [81 x i8]

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* @ps, [81 x i8]** %s, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "##)
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

    insta::assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type {}
    %foo.bar = type { i32, i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      ret void
    }

    define void @foo.bar(%foo* %0, %foo.bar* %1) {
    entry:
      %x = getelementptr inbounds %foo.bar, %foo.bar* %1, i32 0, i32 0
      %px = getelementptr inbounds %foo.bar, %foo.bar* %1, i32 0, i32 1
      store i32 10, i32* %x, align 4
      store i32* %x, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type {}

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
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

    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { i32 }
    %foo.bar = type { i32* }
    %foo.baz = type { i32* }

    @y = global i32 0
    @__foo__init = unnamed_addr constant %foo { i32 5 }

    define void @foo(%foo* %0) {
    entry:
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @foo.bar(%foo* %0, %foo.bar* %1) {
    entry:
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %px = getelementptr inbounds %foo.bar, %foo.bar* %1, i32 0, i32 0
      store i32* %x, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }

    define void @foo.baz(%foo* %0, %foo.baz* %1) {
    entry:
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %px = getelementptr inbounds %foo.baz, %foo.baz* %1, i32 0, i32 0
      store i32* @y, i32** %px, align 8
      store i32* @y, i32** %px, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { i32 }
    %foo.baz = type { i32* }
    %foo.bar = type { i32* }

    @__foo__init = external global %foo

    define void @__init_foo.baz(%foo.baz* %0) {
    entry:
      %self = alloca %foo.baz*, align 8
      store %foo.baz* %0, %foo.baz** %self, align 8
      ret void
    }

    declare void @foo.baz(%foo*, %foo.baz*)

    define void @__init_foo.bar(%foo.bar* %0) {
    entry:
      %self = alloca %foo.bar*, align 8
      store %foo.bar* %0, %foo.bar** %self, align 8
      ret void
    }

    declare void @foo.bar(%foo*, %foo.bar*)

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "###);

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

    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { i32 }
    %foo.bar = type { i32, i32* }

    @__foo__init = unnamed_addr constant %foo { i32 5 }

    define void @foo(%foo* %0) {
    entry:
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @foo.bar(%foo* %0, %foo.bar* %1) {
    entry:
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %x1 = getelementptr inbounds %foo.bar, %foo.bar* %1, i32 0, i32 0
      %px = getelementptr inbounds %foo.bar, %foo.bar* %1, i32 0, i32 1
      store i32 10, i32* %x1, align 4
      store i32* %x1, i32** %px, align 8
      store i32* %x1, i32** %px, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { i32 }
    %foo.bar = type { i32, i32* }

    @__foo__init = external global %foo

    define void @__init_foo.bar(%foo.bar* %0) {
    entry:
      %self = alloca %foo.bar*, align 8
      store %foo.bar* %0, %foo.bar** %self, align 8
      ret void
    }

    declare void @foo.bar(%foo*, %foo.bar*)

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "###);
}
