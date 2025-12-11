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
/// Each POU containing such statements will have a corresponding init function registered
/// in the global `Index` and a new `POU` named `__init_<pou-name>` created.
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
    assert!(index.find_pou("__init_foo").is_some());

    let units = units.iter().map(|unit| unit.get_unit()).collect::<Vec<_>>();
    let init_foo_unit = &units[1].pous[1];

    assert_debug_snapshot!(init_foo_unit, @r#"
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
    // the init-function for `foo` is expected to have a single assignment statement in its function body
    let init_foo_impl = &units[1].implementations[2];
    assert_eq!(&init_foo_impl.name, "__init_foo");
    let statements = &init_foo_impl.statements;
    assert_eq!(statements.len(), 2);
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__vtable",
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
                            name: "ADR",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__vtable_foo_instance",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        },
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
        },
    ]
    "#);

    // the init-function for `bar` will have a `CallStatement` to `__init_foo` as its only statement, passing the member-instance `self.fb`
    let init_bar_impl = &units[1].implementations[3];
    assert_eq!(&init_bar_impl.name, "__init_bar");
    let statements = &init_bar_impl.statements;
    assert_eq!(statements.len(), 2);
    assert_debug_snapshot!(statements, @r#"
    [
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
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__vtable",
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
                            name: "ADR",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__vtable_bar_instance",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        },
    ]
    "#);

    // the init-function for `baz` will have a `RefAssignment`, assigning `REF(d)` to `self.pd` (TODO: currently, it actually is an `Assignment`
    // in the AST which is redirected to `generate_ref_assignment` in codegen) followed by a `CallStatement` to `__init_bar`,
    // passing the member-instance `self.fb`
    let init_baz_impl = &units[1].implementations[4];
    assert_eq!(&init_baz_impl.name, "__init_baz");
    let statements = &init_baz_impl.statements;
    assert_eq!(statements.len(), 2);
    assert_debug_snapshot!(statements[0], @r#"
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
    "#);

    assert_debug_snapshot!(statements[1], @r#"
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

    assert!(index.find_pou("__init___Test").is_some());

    let init = &units[2].pous[0];
    assert_debug_snapshot!(init, @r#"
    POU {
        name: "__init___Test",
        variable_blocks: [],
        pou_type: ProjectInit,
        return_type: None,
        interfaces: [],
        properties: [],
    }
    "#);

    let init_impl = &units[2].implementations[0];
    assert_eq!(&init_impl.name, "__init___Test");
    assert_eq!(init_impl.statements.len(), 9);
    assert_debug_snapshot!(&init_impl.statements, @r#"
    [
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
        },
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
        },
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
        },
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__init___vtable_foo",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__vtable_foo_instance",
                        },
                    ),
                    base: None,
                },
            ),
        },
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
        },
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__user_init_baz",
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
        },
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__user_init_bar",
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
        },
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__user_init_qux",
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
        },
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__user_init___vtable_foo",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__vtable_foo_instance",
                        },
                    ),
                    base: None,
                },
            ),
        },
    ]
    "#);
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

    let res = generate_to_string("Test", vec![SourceCode::from(src)]).unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %myStruct = type { i8, i8 }
    %myRefStruct = type { ptr }

    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer
    @__myRefStruct__init = unnamed_addr constant %myRefStruct zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]

    define void @__init_mystruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init_myrefstruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_myStruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_myRefStruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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

    %baz = type { %bar }
    %bar = type { ptr, %foo }
    %foo = type { ptr, ptr }
    %myStruct = type { i8, i8 }
    %__vtable_foo = type { ptr }
    %__vtable_bar = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @baz_instance = global %baz zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer
    @s = global %myStruct zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %ps = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %fb = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 1
      ret void
    }

    define void @baz(ptr %0) {
    entry:
      %fb = getelementptr inbounds nuw %baz, ptr %0, i32 0, i32 0
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @bar, ptr %__body, align 8
      ret void
    }

    define void @__init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %fb = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 1
      call void @__init_foo(ptr %fb)
      %deref1 = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %ps = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store ptr @s, ptr %ps, align 8
      ret void
    }

    define void @__init_mystruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init_baz(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %fb = getelementptr inbounds nuw %baz, ptr %deref, i32 0, i32 0
      call void @__init_bar(ptr %fb)
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_baz(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %fb = getelementptr inbounds nuw %baz, ptr %deref, i32 0, i32 0
      call void @__user_init_bar(ptr %fb)
      ret void
    }

    define void @__user_init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %fb = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 1
      call void @__user_init_foo(ptr %fb)
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_myStruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_baz(ptr @baz_instance)
      call void @__init_mystruct(ptr @s)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_bar(ptr @__vtable_bar_instance)
      call void @__user_init_baz(ptr @baz_instance)
      call void @__user_init_myStruct(ptr @s)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_bar(ptr @__vtable_bar_instance)
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

    @ps2 = global [81 x i8] zeroinitializer
    @ps = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %s2 = alloca ptr, align 8
      store ptr @ps2, ptr %s2, align 8
      store ptr @ps2, ptr %s2, align 8
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %fb = alloca %foo, align 8
      %s = alloca ptr, align 8
      %s2 = alloca ptr, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %fb, ptr align 1 @__foo__init, i64 ptrtoint (ptr getelementptr (%foo, ptr null, i32 1) to i64), i1 false)
      store ptr null, ptr %s, align 8
      store ptr @ps2, ptr %s2, align 8
      store i32 0, ptr %main, align 4
      store ptr @ps, ptr %s, align 8
      store ptr @ps2, ptr %s2, align 8
      call void @__init_foo(ptr %fb)
      call void @__user_init_foo(ptr %fb)
      call void @foo(ptr %fb)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %s = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store ptr @ps, ptr %s, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
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

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align 4
      %px = alloca ptr, align 8
      store i32 10, ptr %x, align 4
      store ptr %x, ptr %px, align 8
      store ptr %x, ptr %px, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
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
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo { ptr null, i32 5 }
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %px = alloca ptr, align 8
      store ptr %x, ptr %px, align 8
      store ptr %x, ptr %px, align 8
      ret void
    }

    define void @foo__baz(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %px = alloca ptr, align 8
      store ptr @y, ptr %px, align 8
      store ptr @y, ptr %px, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__bar, ptr %bar, align 8
      %deref2 = load ptr, ptr %self, align 8
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref2, i32 0, i32 2
      store ptr @foo__baz, ptr %baz, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
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

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo { ptr null, i32 5 }
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %x1 = alloca i32, align 4
      %px = alloca ptr, align 8
      store i32 10, ptr %x1, align 4
      store ptr %x1, ptr %px, align 8
      store ptr %x1, ptr %px, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
}
