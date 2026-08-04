use insta::assert_snapshot;

use plc_ast::{
    ast::{pre_process, LinkageType},
    provider::IdProvider,
};
use plc_index::GlobalContext;
use plc_source::source_location::SourceLocationFactory;
use plc_source::SourceCode;

use crate::{
    index::{indexer, Index},
    lexer, parser,
    test_utils::tests::{annotate_and_lower_in_place, index_with_ids, parse_and_validate_buffered},
    validation::Validator,
};

#[test]
fn duplicate_pous_validation() {
    // GIVEN two POUs witht he same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION        foo : INT  END_FUNCTION

        PROGRAM         foo  END_PROGRAM

        FUNCTION_BLOCK  foo  END_FUNCTION_BLOCK
    "#,
    );
    // THEN there should be 3 duplication diagnostics
    assert_snapshot!(&diagnostics);
}

#[test]
fn duplicate_pous_and_types_validation() {
    // GIVEN a POU and a Type with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK  foo  END_FUNCTION_BLOCK
        TYPE foo : INT; END_TYPE
    "#,
    );
    // THEN there should be 3 duplication diagnostics
    assert_snapshot!(&diagnostics);
}

#[test]
fn duplicate_function_and_type_is_no_issue() {
    // GIVEN a Function and a Type with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION  foo: INT  END_FUNCTION
        TYPE foo : INT; END_TYPE
    "#,
    );
    // THEN there should be 0 duplication diagnostics
    assert!(diagnostics.is_empty());
}

#[test]
fn duplicate_global_variables() {
    // GIVEN some duplicate global variables
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL
            a: INT;
            b: INT;
            c: INT;
        END_VAR

        VAR_GLOBAL
            a: BOOL;
        END_VAR

        "#,
    );
    // THEN there should be 0 duplication diagnostics
    assert_snapshot!(&diagnostics);
}

#[test]
fn duplicate_variables_in_same_pou() {
    // GIVEN a POU with a duplicate variable
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM prg
        VAR
            a: INT;
            b: INT;
            c: INT;
        END_VAR
        VAR
            b: BOOL;
        END_VAR
        END_PROGRAM
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_snapshot!(&diagnostics);
}

#[test]
fn duplicate_enum_members_in_different_types_is_no_issue() {
    // GIVEN a two enums with the same elements
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            TYPE enum1 : (red, green, yellow); END_TYPE
            TYPE enum2 : (red, green, yellow); END_TYPE
        "#,
    );
    // THEN there should be no issues
    assert!(diagnostics.is_empty());
}

#[test]
fn duplicate_fb_inst_and_function() {
    // GIVEN a global fb-instance called foo and a function called foo
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            FUNCTION_BLOCK FooFB
                VAR x : INT; END_VAR
            END_FUNCTION_BLOCK

            VAR_GLOBAL
                foo: FooFB;
            END_VAR

            FUNCTION foo: INT
                VAR_INPUT
                    x: INT;
                END_VAR
            END_FUNCTION
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_snapshot!(&diagnostics);
}

#[test]
fn duplicate_enum_variables() {
    // GIVEN an enum with two identical elements
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            TYPE enum1 : (red, green, yellow, red); END_TYPE
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_snapshot!(&diagnostics);
}

#[test]
fn duplicate_global_and_program() {
    // GIVEN a global variable `prg` and a Program `prg`
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            VAR_GLOBAL
                a: INT;
                prg: INT;
                b: INT;
            END_VAR

            PROGRAM prg
                VAR_INPUT
                    x: INT;
                END_VAR
            END_PROGRAM
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_snapshot!(&diagnostics);
}

#[test]
fn duplicate_action_should_be_a_problem() {
    // GIVEN a program with two actions with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            PROGRAM prg
                VAR_INPUT
                    x: INT;
                END_VAR
            END_PROGRAM

            ACTIONS
            ACTION foo
                x := 2;
            END_ACTION

            ACTION baz
                x := 2;
            END_ACTION

            ACTION foo
                x := 2;
            END_ACTION

            END_ACTIONS
        "#,
    );

    // THEN there should be 2 duplication diagnostics
    assert_snapshot!(&diagnostics);
}

#[test]
fn duplicate_actions_in_different_pous_are_no_issue() {
    // GIVEN two POUs with actions with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            PROGRAM prg
            END_PROGRAM

            ACTIONS
                ACTION foo END_ACTION
            END_ACTIONS

            PROGRAM prg2
            END_PROGRAM

            ACTIONS
                ACTION foo END_ACTION
            END_ACTIONS
        "#,
    );

    // THEN there should be no duplication diagnostics
    assert!(diagnostics.is_empty());
}

#[test]
fn automatically_generated_ptr_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a pointer type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            PROGRAM prg
            VAR
                a: LINT;
                x : INT;
            END_VAR

            a := REF(x);  //generates ptr_to_INT type
            a := REF(x);  //also? generates ptr to INT type
            END_PROGRAM
            "#,
    );

    // THEN there should be no duplication diagnostics
    assert!(diagnostics.is_empty());
}

#[test]
fn automatically_generated_string_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a pointer type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            PROGRAM prg
            VAR
                a: STRING;
            END_VAR

            a := 'abc';  //implicitely creates STRING[4] type
            a := 'xyz';  //implicityly creates STRING[4] type again
            END_PROGRAM
            "#,
    );

    // THEN there should be no duplication diagnostics
    assert!(diagnostics.is_empty());
}

#[test]
fn automatically_generated_byref_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a ref-types
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            FUNCTION foo : INT
                VAR_INPUT {ref}
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
    );

    // THEN there should be no duplication diagnostics
    assert!(diagnostics.is_empty());
}

#[test]
fn automatically_generated_inout_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a ptr-types
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            FUNCTION foo : INT
                VAR_IN_OUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
    );

    // THEN there should be no duplication diagnostics
    assert!(diagnostics.is_empty());
}

#[test]
fn automatically_generated_output_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a ptr-types
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        r#"
            FUNCTION foo : INT
                VAR_OUTPUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
    );

    // THEN there should be no duplication diagnostics
    assert!(diagnostics.is_empty());
}

#[test]
fn automatically_generated_output_types_in_different_files_dont_cause_duplication_issues() {
    // a version of the test-util function that does not import the built-in and std-types
    // (or they will cause a duplication issue)
    fn do_index(src: &str, id_provider: IdProvider) -> Index {
        let mut index = Index::default();
        let (mut unit, ..) = parser::parse(
            lexer::lex_with_ids(src, id_provider.clone(), SourceLocationFactory::internal(src)),
            LinkageType::Internal,
            "test.st",
        );
        pre_process(&mut unit, id_provider);
        index.import(indexer::index(&unit));
        index
    }

    let mut ctxt = GlobalContext::new();

    // GIVEN some code that automatically generates a ptr-types
    let code1 = SourceCode::from(
        r#"
            FUNCTION foo : INT
                VAR_OUTPUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
    )
    .with_path("file1");

    let code2 = SourceCode::from(
        r#"
            FUNCTION foo2 : INT
                VAR_OUTPUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
    )
    .with_path("file2");

    ctxt.insert(&code1, None).unwrap();
    ctxt.insert(&code2, None).unwrap();
    let index1 = do_index(&code1.source, ctxt.provider());
    let index2 = do_index(&code2.source, ctxt.provider());

    // WHEN the index is combined
    let mut global_index = Index::default();
    global_index.import(index1); //import file 1
    global_index.import(index2); //import file 2

    // THEN there should be no duplication diagnostics
    let mut validator = Validator::new(&ctxt);
    validator.perform_global_validation(&global_index);
    let diagnostics = validator.diagnostics();
    assert!(diagnostics.is_empty());
}

#[test]
fn duplicate_with_generic() {
    // A generic function called many times, from multiple programs, must materialize each
    // monomorphization exactly once (deduped by the generic-lowering phase) — so no duplicate-symbol
    // diagnostics are produced.
    let id_provider = IdProvider::default();
    let ctxt = GlobalContext::new();
    let (unit, index) = index_with_ids(
        r#"
        {external}
        FUNCTION foo <T: ANY_INT> : DATE
        VAR_INPUT
            a : T;
            b : T;
            c : T;
        END_VAR
        END_FUNCTION

        PROGRAM prg1
            foo(INT#1, SINT#2, SINT#3);
            foo(DINT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
        END_PROGRAM

        PROGRAM prg2
            foo(INT#1, SINT#2, SINT#3);
            foo(DINT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    let (_unit, index, _annotations) = annotate_and_lower_in_place(unit, index, id_provider);

    // THEN each distinct monomorphization exists exactly once, regardless of how many call sites or
    // programs reference it.
    assert!(index.find_pou("foo__INT").is_some());
    assert!(index.find_pou("foo__DINT").is_some());

    // AND there are no duplication diagnostics.
    let mut validator = Validator::new(&ctxt);
    validator.perform_global_validation(&index);
    let diagnostics = validator.diagnostics();
    assert!(diagnostics.is_empty(), "unexpected diagnostics: {diagnostics:?}");
}

#[test]
fn generics_with_duplicate_symbol_dont_err() {
    // GIVEN a builtin function with the signature
    // FUNCTION ADD<T: ANY_NUM> : T
    // VAR_INPUT
    //     args: {sized} T...;
    // END_VAR
    // END_FUNCTION

    // WHEN it is indexed and validated with other generic functions with the same name
    let diagnostics = parse_and_validate_buffered(
        r#"
            FUNCTION ADD < T1: ANY, T2: ANY >: T1
            VAR_INPUT
                IN1: T1;
                IN2: T2;
            END_VAR
            END_FUNCTION

            FUNCTION ADD < K: ANY, V: ANY > : K
            VAR_INPUT
                IN1: K;
                IN2: V;
            END_VAR
            END_FUNCTION
        "#,
    );

    // THEN there should be no duplication diagnostics
    assert!(diagnostics.is_empty());
}

#[test]
fn duplicate_enum_names() {
    let diagnostics = parse_and_validate_buffered(
        r"
        TYPE
            foo : (a, b);
            foo : (c, d);
        END_TYPE
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E004]: foo: Ambiguous datatype.
      ┌─ <internal>:3:13
      │
    3 │             foo : (a, b);
      │             ^^^ foo: Ambiguous datatype.
    4 │             foo : (c, d);
      │             --- see also

    error[E004]: foo: Ambiguous datatype.
      ┌─ <internal>:4:13
      │
    3 │             foo : (a, b);
      │             --- see also
    4 │             foo : (c, d);
      │             ^^^ foo: Ambiguous datatype.
    ");
}

#[test]
fn duplicate_enum_inline_variants() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION main : DINT
            VAR
                foo : (a, b, c, a);
            END_VAR
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @"
    error[E004]: a: Duplicate symbol.
      ┌─ <internal>:4:24
      │
    4 │                 foo : (a, b, c, a);
      │                        ^        - see also
      │                        │         
      │                        a: Duplicate symbol.

    error[E004]: a: Duplicate symbol.
      ┌─ <internal>:4:33
      │
    4 │                 foo : (a, b, c, a);
      │                        -        ^ a: Duplicate symbol.
      │                        │         
      │                        see also
    ");
}

// https://github.com/PLC-lang/rusty/issues/1156
#[test]
fn multiple_enum_instances_in_var_block_wont_trigger_duplicate_check() {
    let diagnostics = parse_and_validate_buffered(
        r"
        TYPE
            Foo : (A, B, C);
        END_TYPE

        FUNCTION main
            VAR
                fooA : Foo;
                fooB : Foo;
                fooC : Foo;
            END_VAR
        END_FUNCTION
        ",
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn duplicate_method_names_should_return_an_error() {
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM prg
            // This shouldn't be fine
            METHOD foo
            END_METHOD

            METHOD foo
            END_METHOD

            // This should
            METHOD bar
            END_METHOD
        END_PROGRAM

        FUNCTION_BLOCK fb
            // This should be fine
            METHOD foo
            END_METHOD

            // This shouldnt
            METHOD bar
            END_METHOD

            METHOD bar
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E004]: prg.foo: Ambiguous callable symbol.
      ┌─ <internal>:4:20
      │
    4 │             METHOD foo
      │                    ^^^ prg.foo: Ambiguous callable symbol.
      ·
    7 │             METHOD foo
      │                    --- see also

    error[E004]: prg.foo: Ambiguous callable symbol.
      ┌─ <internal>:7:20
      │
    4 │             METHOD foo
      │                    --- see also
      ·
    7 │             METHOD foo
      │                    ^^^ prg.foo: Ambiguous callable symbol.

    error[E004]: fb.bar: Ambiguous callable symbol.
       ┌─ <internal>:21:20
       │
    21 │             METHOD bar
       │                    ^^^ fb.bar: Ambiguous callable symbol.
       ·
    24 │             METHOD bar
       │                    --- see also

    error[E004]: fb.bar: Ambiguous callable symbol.
       ┌─ <internal>:24:20
       │
    21 │             METHOD bar
       │                    --- see also
       ·
    24 │             METHOD bar
       │                    ^^^ fb.bar: Ambiguous callable symbol.
    ");
}

#[test]
fn duplicate_interfaces() {
    let source = r"
    INTERFACE foo /* ... */ END_INTERFACE
    INTERFACE foo /* ... */ END_INTERFACE
    ";

    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @r"
    error[E004]: foo: Ambiguous datatype.
      ┌─ <internal>:2:15
      │
    2 │     INTERFACE foo /* ... */ END_INTERFACE
      │               ^^^ foo: Ambiguous datatype.
    3 │     INTERFACE foo /* ... */ END_INTERFACE
      │               --- see also

    error[E004]: foo: Ambiguous datatype.
      ┌─ <internal>:3:15
      │
    2 │     INTERFACE foo /* ... */ END_INTERFACE
      │               --- see also
    3 │     INTERFACE foo /* ... */ END_INTERFACE
      │               ^^^ foo: Ambiguous datatype.
    ");
}

#[test]
fn user_function_block_clashing_with_generic_function_is_flagged() {
    // Reproduces the case-insensitive POU name collision that previously crashed
    // codegen with "Could not find generated stub for <X>": a user-declared
    // top-level FB sharing a (case-insensitive) name with a generic stdlib-style
    // function must be reported here, not silently accepted.
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION mid <T : ANY_STRING> : T
        VAR_INPUT IN : T; END_VAR
        END_FUNCTION

        FUNCTION_BLOCK Mid
        END_FUNCTION_BLOCK
        "#,
    );
    assert_snapshot!(diagnostics, @r"
    error[E004]: mid: Duplicate symbol.
      ┌─ <internal>:2:18
      │
    2 │         FUNCTION mid <T : ANY_STRING> : T
      │                  ^^^ mid: Duplicate symbol.
      ·
    6 │         FUNCTION_BLOCK Mid
      │                        --- see also

    error[E004]: mid: Duplicate symbol.
      ┌─ <internal>:6:24
      │
    2 │         FUNCTION mid <T : ANY_STRING> : T
      │                  --- see also
      ·
    6 │         FUNCTION_BLOCK Mid
      │                        ^^^ mid: Duplicate symbol.
    ");
}

#[test]
fn user_function_clashing_with_generic_function_is_flagged() {
    // Same root cause as the FB case but with a non-generic FUNCTION sharing
    // a name with a generic FUNCTION.
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION mid <T : ANY_STRING> : T
        VAR_INPUT IN : T; END_VAR
        END_FUNCTION

        FUNCTION mid : DINT
        END_FUNCTION
        "#,
    );
    assert_snapshot!(diagnostics, @r"
    error[E004]: mid: Duplicate symbol.
      ┌─ <internal>:2:18
      │
    2 │         FUNCTION mid <T : ANY_STRING> : T
      │                  ^^^ mid: Duplicate symbol.
      ·
    6 │         FUNCTION mid : DINT
      │                  --- see also

    error[E004]: mid: Duplicate symbol.
      ┌─ <internal>:6:18
      │
    2 │         FUNCTION mid <T : ANY_STRING> : T
      │                  --- see also
      ·
    6 │         FUNCTION mid : DINT
      │                  ^^^ mid: Duplicate symbol.
    ");
}

#[test]
fn non_generic_pou_in_a_three_way_cluster_with_generics_is_flagged() {
    // Non-generic in a cluster also containing multiple generics: each pair
    // (non-generic vs generic) is a real ambiguity — flag the whole cluster.
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION foo <T1: ANY, T2: ANY> : T1
        VAR_INPUT IN1: T1; IN2: T2; END_VAR
        END_FUNCTION

        FUNCTION foo <K: ANY, V: ANY> : K
        VAR_INPUT IN1: K; IN2: V; END_VAR
        END_FUNCTION

        FUNCTION_BLOCK foo
        END_FUNCTION_BLOCK
        "#,
    );
    // Three locations are reported at each site — both generics plus the FB.
    assert_snapshot!(diagnostics, @"
    error[E004]: foo: Duplicate symbol.
       ┌─ <internal>:2:18
       │
     2 │         FUNCTION foo <T1: ANY, T2: ANY> : T1
       │                  ^^^ foo: Duplicate symbol.
       ·
     6 │         FUNCTION foo <K: ANY, V: ANY> : K
       │                  --- see also
       ·
    10 │         FUNCTION_BLOCK foo
       │                        --- see also

    error[E004]: foo: Duplicate symbol.
       ┌─ <internal>:6:18
       │
     2 │         FUNCTION foo <T1: ANY, T2: ANY> : T1
       │                  --- see also
       ·
     6 │         FUNCTION foo <K: ANY, V: ANY> : K
       │                  ^^^ foo: Duplicate symbol.
       ·
    10 │         FUNCTION_BLOCK foo
       │                        --- see also

    error[E004]: foo: Duplicate symbol.
       ┌─ <internal>:10:24
       │
     2 │         FUNCTION foo <T1: ANY, T2: ANY> : T1
       │                  --- see also
       ·
     6 │         FUNCTION foo <K: ANY, V: ANY> : K
       │                  --- see also
       ·
    10 │         FUNCTION_BLOCK foo
       │                        ^^^ foo: Duplicate symbol.
    ");
}
