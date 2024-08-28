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
        pou_type: Function,
        return_type: None,
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
    assert_debug_snapshot!(statements[0], @r###"
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
                    base: None,
                },
            ),
        },
    }
    "###);

    // the init-function for `bar` will have a `CallStatement` to `__init_foo` as its only statement, passing the member-instance `self.fb`
    let init_bar_impl = &units[1].implementations[2];
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
    let init_baz_impl = &units[1].implementations[1];
    assert_eq!(&init_baz_impl.name, "__init_baz");
    let statements = &init_baz_impl.statements;
    assert_eq!(statements.len(), 2);
    assert_debug_snapshot!(statements[0], @r###"
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
            base: None,
        },
    }
    "###);

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
        pou_type: Function,
        return_type: None,
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
    assert_debug_snapshot!(&init_impl.statements[2], @r###"
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

    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer, section "var-$RUSTY$__myStruct__init:r2u8u8"
    @__myRefStruct__init = unnamed_addr constant %myRefStruct zeroinitializer, section "var-$RUSTY$__myRefStruct__init:r1pr2u8u8"
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %myStruct = type { i8, i8 }
    %myRefStruct = type { %myStruct* }

    @__myStruct__init = external global %myStruct, section "var-$RUSTY$__myStruct__init:r2u8u8"
    @__myRefStruct__init = external global %myRefStruct, section "var-$RUSTY$__myRefStruct__init:r1pr2u8u8"

    define void @__init_mystruct(%myStruct* %0) section "fn-$RUSTY$__init_mystruct:v[pr2u8u8]" {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init_myrefstruct(%myRefStruct* %0) section "fn-$RUSTY$__init_myrefstruct:v[pr1pr2u8u8]" {
    entry:
      %self = alloca %myRefStruct*, align 8
      store %myRefStruct* %0, %myRefStruct** %self, align 8
      ret void
    }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
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

    @s = global %myStruct zeroinitializer, section "var-$RUSTY$s:r2u8u8"
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer, section "var-$RUSTY$__myStruct__init:r2u8u8"
    @__foo__init = unnamed_addr constant %foo zeroinitializer, section "var-$RUSTY$__foo__init:r1ps8u81"
    @__bar__init = unnamed_addr constant %bar zeroinitializer, section "var-$RUSTY$__bar__init:r1r1ps8u81"
    @baz_instance = global %baz zeroinitializer, section "var-$RUSTY$baz_instance:r1r1r1ps8u81"

    define void @foo(%foo* %0) section "fn-$RUSTY$foo:v" {
    entry:
      %ps = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @bar(%bar* %0) section "fn-$RUSTY$bar:v" {
    entry:
      %fb = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void
    }

    define void @baz(%baz* %0) section "fn-$RUSTY$baz:v" {
    entry:
      %fb = getelementptr inbounds %baz, %baz* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8]* }
    %myStruct = type { i8, i8 }
    %bar = type { %foo }
    %baz = type { %bar }

    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r1ps8u81"
    @__myStruct__init = external global %myStruct, section "var-$RUSTY$__myStruct__init:r2u8u8"
    @__bar__init = external global %bar, section "var-$RUSTY$__bar__init:r1r1ps8u81"
    @baz_instance = external global %baz, section "var-$RUSTY$baz_instance:r1r1r1ps8u81"
    @s = external global %myStruct, section "var-$RUSTY$s:r2u8u8"

    define void @__init_foo(%foo* %0) section "fn-$RUSTY$__init_foo:v[pr1ps8u81]" {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %ps = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* bitcast (%myStruct* @s to [81 x i8]*), [81 x i8]** %ps, align 8
      ret void
    }

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"

    define void @__init_mystruct(%myStruct* %0) section "fn-$RUSTY$__init_mystruct:v[pr2u8u8]" {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init_bar(%bar* %0) section "fn-$RUSTY$__init_bar:v[pr1r1ps8u81]" {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %fb = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %fb)
      ret void
    }

    declare void @bar(%bar*) section "fn-$RUSTY$bar:v"

    define void @__init_baz(%baz* %0) section "fn-$RUSTY$__init_baz:v[pr1r1r1ps8u81]" {
    entry:
      %self = alloca %baz*, align 8
      store %baz* %0, %baz** %self, align 8
      %deref = load %baz*, %baz** %self, align 8
      %fb = getelementptr inbounds %baz, %baz* %deref, i32 0, i32 0
      call void @__init_bar(%bar* %fb)
      ret void
    }

    declare void @baz(%baz*) section "fn-$RUSTY$baz:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %baz = type { %bar }
    %bar = type { %foo }
    %foo = type { [81 x i8]* }

    @baz_instance = external global %baz, section "var-$RUSTY$baz_instance:r1r1r1ps8u81"
    @__bar__init = external global %bar, section "var-$RUSTY$__bar__init:r1r1ps8u81"
    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r1ps8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_baz(%baz* @baz_instance)
      ret void
    }

    declare void @__init_baz(%baz*) section "fn-$RUSTY$__init_baz:v[pr1r1r1ps8u81]"

    declare void @baz(%baz*) section "fn-$RUSTY$baz:v"

    declare void @bar(%bar*) section "fn-$RUSTY$bar:v"

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"
    "###);

    // This example shows how more complex initializers are generated. We expect the global default struct initializer `__myStruct__init`
    // to be initialized with two null-pointers for the `REF_TO` and `ALIAS`, but `member3` should be initialized with its assigned value
    // before `__init_myStruct` is called.
    let src = "
    TYPE myStruct : STRUCT
            member : REF_TO STRING := REF(s);
            member2 AT s2 : ARRAY[0..1] OF STRING;
            member3: DINT := 123456;
        END_STRUCT
    END_TYPE

    VAR_GLOBAL
        s : STRING := 'Hello world!';
        s2 : ARRAY[0..1] OF STRING := ['hello', 'world'];
    END_VAR

    PROGRAM prog 
    VAR 
        str: myStruct;
    END_VAR
    END_PROGRAM
    ";

    let res = codegen(src);
    assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %prog = type { %myStruct }
    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]*, i32 }

    @s = global [81 x i8] c"Hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$s:s8u81"
    @s2 = global [2 x [81 x i8]] [[81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"world\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], section "var-$RUSTY$s2:as8u81"
    @prog_instance = global %prog { %myStruct { [81 x i8]* null, [2 x [81 x i8]]* null, i32 123456 } }, section "var-$RUSTY$prog_instance:r1r3ps8u81pas8u81i32"
    @__myStruct__init = unnamed_addr constant %myStruct { [81 x i8]* null, [2 x [81 x i8]]* null, i32 123456 }, section "var-$RUSTY$__myStruct__init:r3ps8u81pas8u81i32"

    define void @prog(%prog* %0) section "fn-$RUSTY$prog:v" {
    entry:
      %str = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]*, i32 }
    %prog = type { %myStruct }

    @__myStruct__init = external global %myStruct, section "var-$RUSTY$__myStruct__init:r3ps8u81pas8u81i32"
    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r1r3ps8u81pas8u81i32"
    @s2 = external global [2 x [81 x i8]], section "var-$RUSTY$s2:as8u81"
    @s = external global [81 x i8], section "var-$RUSTY$s:s8u81"

    define void @__init_mystruct(%myStruct* %0) section "fn-$RUSTY$__init_mystruct:v[pr3ps8u81pas8u81i32]" {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      %deref = load %myStruct*, %myStruct** %self, align 8
      %member2 = getelementptr inbounds %myStruct, %myStruct* %deref, i32 0, i32 1
      store [2 x [81 x i8]]* @s2, [2 x [81 x i8]]** %member2, align 8
      %deref1 = load %myStruct*, %myStruct** %self, align 8
      %member = getelementptr inbounds %myStruct, %myStruct* %deref1, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %member, align 8
      ret void
    }

    define void @__init_prog(%prog* %0) section "fn-$RUSTY$__init_prog:v[pr1r3ps8u81pas8u81i32]" {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %str = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__init_mystruct(%myStruct* %str)
      ret void
    }

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type { %myStruct }
    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]*, i32 }

    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r1r3ps8u81pas8u81i32"
    @__myStruct__init = external global %myStruct, section "var-$RUSTY$__myStruct__init:r3ps8u81pas8u81i32"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      ret void
    }

    declare void @__init_prog(%prog*) section "fn-$RUSTY$__init_prog:v[pr1r3ps8u81pas8u81i32]"

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"
    "###);
}
