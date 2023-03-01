use insta::assert_snapshot;

use crate::{
    ast::{self, CompilationUnit, SourceRangeFactory},
    diagnostics::Diagnostician,
    index::{visitor, Index},
    lexer::{self, IdProvider},
    parser,
    resolver::TypeAnnotator,
    test_utils::tests::{compile_to_string, parse_and_validate},
    typesystem,
    validation::{tests::make_readable, Validator},
    SourceCode,
};

#[test]
fn duplicate_pous_validation() {
    // GIVEN two POUs witht he same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION        foo : INT  END_FUNCTION

        PROGRAM         foo  END_PROGRAM

        FUNCTION_BLOCK  foo  END_FUNCTION_BLOCK
    "#,
    );
    // THEN there should be 3 duplication diagnostics
    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn duplicate_pous_and_types_validation() {
    // GIVEN a POU and a Type with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION_BLOCK  foo  END_FUNCTION_BLOCK
        TYPE foo : INT END_TYPE
    "#,
    );
    // THEN there should be 3 duplication diagnostics
    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn duplicate_function_and_type_is_no_issue() {
    // GIVEN a Function and a Type with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION  foo: INT  END_FUNCTION
        TYPE foo : INT; END_TYPE
    "#,
    );
    // THEN there should be 0 duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn duplicate_global_variables() {
    // GIVEN some duplicate global variables
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
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
    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn duplicate_variables_in_same_pou() {
    // GIVEN a POU with a duplicate variable
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
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
    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn duplicate_enum_members_in_different_types_is_no_issue() {
    // GIVEN a two enums with the same elements
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            TYPE enum1 : (red, green, yellow); END_TYPE
            TYPE enum2 : (red, green, yellow); END_TYPE
        "#,
    );
    // THEN there should be no issues
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn duplicate_fb_inst_and_function() {
    // GIVEN a global fb-instance called foo and a function called foo
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            FUNCTION_BLOCK FooFB
                VAR x : INT END_VAR
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
    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn duplicate_enum_variables() {
    // GIVEN an enum with two identical elements
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            TYPE enum1 : (red, green, yellow, red); END_TYPE
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn duplicate_global_and_program() {
    // GIVEN a global variable `prg` and a Program `prg`
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
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
    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn duplicate_action_should_be_a_problem() {
    // GIVEN a program with two actions with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
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
    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn duplicate_actions_in_different_pous_are_no_issue() {
    // GIVEN two POUs with actions with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_ptr_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a pointer type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            PROGRAM prg
            VAR
                a: LINT;
                x : INT;
            END_VAR

            a := &x;  //generates ptr_to_INT type
            a := &x;  //also? generates ptr to INT type
            END_PROGRAM
            "#,
    );

    // THEN there should be no duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_string_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a pointer type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_byref_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a ref-types
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_inout_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a ptr-types
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_output_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a ptr-types
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_output_types_in_different_files_dont_cause_duplication_issues() {
    // a version of the test-util function that does not import the built-in and std-types
    // (or they will cause a duplication issue)
    fn do_index(src: &str, id_provider: IdProvider) -> Index {
        let mut index = Index::default();
        let (mut unit, ..) = parser::parse(
            lexer::lex_with_ids(src, id_provider.clone(), SourceRangeFactory::internal()),
            ast::LinkageType::Internal,
            "test.st",
        );
        ast::pre_process(&mut unit, id_provider);
        index.import(visitor::visit(&unit));
        index
    }

    // GIVEN some code that automatically generates a ptr-types
    let ids = IdProvider::default();
    let index1 = do_index(
        r#"
            FUNCTION foo : INT
                VAR_OUTPUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
        ids.clone(),
    );

    //AND another file with also OUTPUT-INTS
    let index2 = do_index(
        r#"
            FUNCTION foo2 : INT
                VAR_OUTPUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
        ids,
    );

    // WHEN the index is combined
    let mut global_index = Index::default();
    global_index.import(index1); //import file 1
    global_index.import(index2); //import file 2

    // THEN there should be no duplication diagnostics
    let mut validator = Validator::new();
    validator.perform_global_validation(&global_index);
    let diagnostics = validator.diagnostics();
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn duplicate_with_generic() {
    // a version of the test-util function that does not import the built-in and std-types
    // (or they will cause a duplication issue)
    fn do_index(src: &str, id_provider: IdProvider, file_name: &str) -> (Index, CompilationUnit) {
        let mut index = Index::default();
        let (mut unit, ..) = parser::parse(
            lexer::lex_with_ids(src, id_provider.clone(), SourceRangeFactory::internal()),
            ast::LinkageType::Internal,
            file_name,
        );
        ast::pre_process(&mut unit, id_provider);
        index.import(visitor::visit(&unit));
        (index, unit)
    }

    // GIVEN a generic function defined in its own file
    let ids = IdProvider::default();
    let (index1, unit1) = do_index(
        r#"
            {external}
            FUNCTION foo <T: ANY_INT> : DATE
            VAR_INPUT
                a : T;
                b : T;
                c : T;
            END_VAR
            END_FUNCTION
        "#,
        ids.clone(),
        "file1.st",
    );

    // AND another file that calls that generic function and implicitely
    // create type-specific foo-implementations
    let (index2, unit2) = do_index(
        r#"
        PROGRAM prg1
            foo(INT#1, SINT#2, SINT#3);
            foo(DINT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
        END_PROGRAM
        "#,
        ids.clone(),
        "file2.st",
    );

    // AND another file that calls that generic function and implicitely
    // create type-specific foo-implementations
    let (index3, unit3) = do_index(
        r#"
        PROGRAM prg2
            foo(INT#1, SINT#2, SINT#3);
            foo(DINT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
        END_PROGRAM
        "#,
        ids.clone(),
        "file3.st",
    );
    // WHEN the index is combined
    let mut global_index = Index::default();
    for data_type in typesystem::get_builtin_types() {
        global_index.register_type(data_type);
    }
    global_index.import(index1); //import file 1
    global_index.import(index2); //import file 2
    global_index.import(index3); //import file 3

    // AND the resolvers does its job
    let (mut annotations1, _) = TypeAnnotator::visit_unit(&global_index, &unit1, ids.clone());
    let (mut annotations2, _) = TypeAnnotator::visit_unit(&global_index, &unit2, ids.clone());
    let (mut annotations3, _) = TypeAnnotator::visit_unit(&global_index, &unit3, ids);
    global_index.import(std::mem::take(&mut annotations1.new_index));
    global_index.import(std::mem::take(&mut annotations2.new_index));
    global_index.import(std::mem::take(&mut annotations3.new_index));

    // THEN the index should contain 5 pous, 2 were dynamically generated by the visitor (foo__INT & foo__DINT)
    assert_eq!(
        vec!["foo", "prg1", "prg2", "foo__INT", "foo__DINT"],
        global_index.get_pous().values().map(|it| it.get_name()).collect::<Vec<_>>()
    );

    // AND there should be no duplication diagnostics
    let mut validator = Validator::new();
    validator.perform_global_validation(&global_index);
    let diagnostics = validator.diagnostics();
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn duplicate_with_generic_ir() {
    // GIVEN several files with calls to a generic function
    let file1: SourceCode = r"
            {external}
            FUNCTION foo <T: ANY_INT> : DATE
            VAR_INPUT
                a : T;
                b : T;
                c : T;
            END_VAR
            END_FUNCTION
            "
    .into();

    let file2: SourceCode = r"
        PROGRAM prg1
            foo(INT#1, SINT#2, SINT#3);
            foo(DINT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
        END_PROGRAM
        "
    .into();
    let file3: SourceCode = r"
        PROGRAM prg2
            foo(INT#1, SINT#2, SINT#3);
            foo(DINT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
            foo(INT#1, SINT#2, SINT#3);
        END_PROGRAM
        "
    .into();
    // WHEN we compile
    let ir = compile_to_string(
        vec![file1, file2, file3],
        vec![],
        None,
        Diagnostician::default(),
        crate::DebugLevel::None,
    )
    .unwrap();

    // THEN we expect only 1 declaration per type-specific implementation of the generic function
    // although file2 & file3 both discovered them independently
    assert_snapshot!(ir);
}
