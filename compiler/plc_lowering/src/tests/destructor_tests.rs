use plc_ast::ast::{Implementation, LinkageType, PouType};
use plc_diagnostics::diagnostician::Diagnostician;
use plc_driver::pipelines::{AnnotatedProject, BuildPipeline};
use plc_source::SourceCode;

fn lower(src: &str) -> AnnotatedProject {
    let diagnostician = Diagnostician::buffered();
    let sources: Vec<SourceCode> = vec![src.into()];
    let mut pipeline = BuildPipeline::from_sources("test.st", sources, diagnostician).unwrap();
    pipeline.register_default_mut_participants();
    pipeline.parse_and_annotate().unwrap()
}

fn implementations(project: &AnnotatedProject) -> impl Iterator<Item = &Implementation> {
    project.units.iter().flat_map(|unit| unit.get_unit().implementations.iter())
}

fn find_implementation<'a>(project: &'a AnnotatedProject, name: &str) -> &'a Implementation {
    implementations(project)
        .find(|implementation| implementation.name == name)
        .unwrap_or_else(|| panic!("implementation `{name}` not found"))
}

fn body(project: &AnnotatedProject, name: &str) -> String {
    let statements = &find_implementation(project, name).statements;
    statements.iter().map(|it| it.as_string()).collect::<Vec<_>>().join("\n")
}

/// Returns the names of the generated destructors, without the hash in the name of the unit destructor
fn destructors(project: &AnnotatedProject) -> Vec<String> {
    implementations(project)
        .filter(|implementation| matches!(implementation.pou_type, PouType::Exit | PouType::ProjectExit))
        .map(|implementation| match implementation.pou_type {
            PouType::ProjectExit => "<unit destructor>".to_string(),
            _ => implementation.name.clone(),
        })
        .collect()
}

#[test]
fn function_block_with_fb_exit_gets_a_destructor_that_calls_it() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    assert_eq!(destructors(&project), vec!["Connection__dtor"]);
    insta::assert_snapshot!(body(&project, "Connection__dtor"), @"self.FB_EXIT()");
}

#[test]
fn types_that_need_no_cleanup_get_no_destructor() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Plain
            VAR
                value: DINT;
            END_VAR

            METHOD FB_INIT
            END_METHOD
        END_FUNCTION_BLOCK

        TYPE Point: STRUCT
            x, y: DINT;
        END_STRUCT END_TYPE

        PROGRAM prog
            VAR
                plain: Plain;
                points: ARRAY[0..1] OF Point;
            END_VAR
        END_PROGRAM

        VAR_GLOBAL
            plain: Plain;
        END_VAR

        FUNCTION work: DINT
            VAR
                plain: Plain;
            END_VAR

            RETURN;
        END_FUNCTION
        "#,
    );

    assert_eq!(destructors(&project), Vec::<String>::new());
    insta::assert_snapshot!(body(&project, "work"), @"
    Plain__ctor(plain)
    RETURN
    ");
}

#[test]
fn destructor_calls_fb_exit_then_members_in_reverse_order_then_the_base() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK Base
            VAR
                baseConnection: Connection;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK Client EXTENDS Base
            VAR
                primary: Connection;
                counter: DINT;
            END_VAR
            VAR_INPUT
                input: Connection;
            END_VAR
            VAR_OUTPUT
                output: Connection;
            END_VAR

            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    assert_eq!(destructors(&project), vec!["Connection__dtor", "Base__dtor", "Client__dtor"]);
    insta::assert_snapshot!(body(&project, "Base__dtor"), @"Connection__dtor(self.baseConnection)");
    insta::assert_snapshot!(body(&project, "Client__dtor"), @"
    self.FB_EXIT()
    Connection__dtor(self.output)
    Connection__dtor(self.input)
    Connection__dtor(self.primary)
    Base__dtor(self.__Base)
    ");
}

#[test]
fn inherited_fb_exit_is_called_by_the_destructor_of_the_base_only() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Base
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK Derived EXTENDS Base
        END_FUNCTION_BLOCK
        "#,
    );

    insta::assert_snapshot!(body(&project, "Base__dtor"), @"self.FB_EXIT()");
    insta::assert_snapshot!(body(&project, "Derived__dtor"), @"Base__dtor(self.__Base)");
}

#[test]
fn members_that_do_not_own_an_instance_are_not_destructed() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK Client
            VAR
                owned: Connection;
                ptr: REF_TO Connection;
                reference: REFERENCE TO Connection;
            END_VAR
            VAR_IN_OUT
                borrowed: Connection;
            END_VAR
            VAR_INPUT {ref}
                byReference: Connection;
            END_VAR
            VAR_TEMP
                temporary: Connection;
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    insta::assert_snapshot!(body(&project, "Client__dtor"), @"Connection__dtor(self.owned)");
}

#[test]
fn struct_destructor_calls_the_fields_in_reverse_order() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        TYPE Pair: STRUCT
            first: Connection;
            count: DINT;
            second: Connection;
        END_STRUCT END_TYPE
        "#,
    );

    insta::assert_snapshot!(body(&project, "Pair__dtor"), @"
    Connection__dtor(self.second)
    Connection__dtor(self.first)
    ");
}

#[test]
fn alias_destructor_calls_the_destructor_of_the_renamed_type() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        TYPE Link: Connection; END_TYPE
        "#,
    );

    insta::assert_snapshot!(body(&project, "Link__dtor"), @"Connection__dtor(self)");
}

#[test]
fn array_destructor_calls_each_element_from_the_lowest_to_the_highest_index() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        TYPE Connections: ARRAY[-2..4] OF Connection; END_TYPE
        "#,
    );

    insta::assert_snapshot!(body(&project, "Connections__dtor"), @"
    alloca Connections__dtor__idx0: DINT
    Connections__dtor__idx0 := -2
    WHILE TRUE DO
        IF Connections__dtor__idx0 > 4 THEN
            EXIT;
        END_IF
        Connection__dtor(self[Connections__dtor__idx0])
        Connections__dtor__idx0 := Connections__dtor__idx0 + 1
    END_WHILE
    ");
}

#[test]
fn array_destructor_has_one_loop_for_each_dimension() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        TYPE Grid: ARRAY[0..1, 5..6] OF Connection; END_TYPE
        "#,
    );

    insta::assert_snapshot!(body(&project, "Grid__dtor"), @"
    alloca Grid__dtor__idx0: DINT
    Grid__dtor__idx0 := 0
    WHILE TRUE DO
        IF Grid__dtor__idx0 > 1 THEN
            EXIT;
        END_IF
        alloca Grid__dtor__idx1: DINT
        Grid__dtor__idx1 := 5
        WHILE TRUE DO
            IF Grid__dtor__idx1 > 6 THEN
                EXIT;
            END_IF
            Connection__dtor(self[Grid__dtor__idx0, Grid__dtor__idx1])
            Grid__dtor__idx1 := Grid__dtor__idx1 + 1
        END_WHILE
        Grid__dtor__idx0 := Grid__dtor__idx0 + 1
    END_WHILE
    ");
}

#[test]
fn unit_destructor_cleans_up_programs_and_then_globals_in_reverse_order() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        VAR_GLOBAL
            first: Connection;
            number: DINT;
            second: ARRAY[0..1] OF Connection;
        END_VAR

        VAR_GLOBAL CONSTANT
            limit: DINT := 3;
        END_VAR

        PROGRAM progWithMember
            VAR
                connection: Connection;
            END_VAR
        END_PROGRAM

        PROGRAM progWithoutCleanup
        END_PROGRAM

        PROGRAM progWithFbExit
            METHOD FB_EXIT
            END_METHOD
        END_PROGRAM
        "#,
    );

    let unit_destructor = implementations(&project)
        .find(|implementation| implementation.pou_type == PouType::ProjectExit)
        .expect("the unit has a destructor");
    insta::assert_snapshot!(body(&project, &unit_destructor.name), @"
    progWithFbExit__dtor(progWithFbExit)
    progWithMember__dtor(progWithMember)
    __global_second__dtor(second)
    Connection__dtor(first)
    ");
    insta::assert_snapshot!(body(&project, "progWithFbExit__dtor"), @"self.FB_EXIT()");
}

#[test]
fn stack_variables_are_destructed_at_the_end_of_the_body_in_reverse_order() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION work: DINT
            VAR
                first: Connection;
                number: DINT;
                second: Connection;
            END_VAR
            VAR_TEMP
                temporary: Connection;
            END_VAR

            work := 5;
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(body(&project, "work"), @"
    Connection__ctor(first)
    Connection__ctor(second)
    Connection__ctor(temporary)
    work := 5
    Connection__dtor(temporary)
    Connection__dtor(second)
    Connection__dtor(first)
    ");
}

#[test]
fn parameters_of_a_function_are_not_destructed() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION work: DINT
            VAR_INPUT
                input: Connection;
            END_VAR
            VAR_IN_OUT
                borrowed: Connection;
            END_VAR

            work := 5;
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(body(&project, "work"), @"work := 5");
}

#[test]
fn stack_variables_are_destructed_before_a_return_in_a_branch() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION work: DINT
            VAR_INPUT
                leave: BOOL;
            END_VAR
            VAR
                connection: Connection;
            END_VAR

            IF leave THEN
                RETURN;
            END_IF
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(body(&project, "work"), @"
    Connection__ctor(connection)
    IF leave THEN
        Connection__dtor(connection)
        RETURN
        
    END_IF
    Connection__dtor(connection)
    ");
}

#[test]
fn stack_variables_are_destructed_before_a_return_in_a_loop() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION work: DINT
            VAR_INPUT
                leave: BOOL;
            END_VAR
            VAR
                connection: Connection;
            END_VAR

            WHILE leave DO
                RETURN;
            END_WHILE
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(body(&project, "work"), @"
    Connection__ctor(connection)
    WHILE TRUE DO
        IF NOT leave THEN
            EXIT;
        END_IF
        Connection__dtor(connection)
        RETURN
        
    END_WHILE
    Connection__dtor(connection)
    ");
}

#[test]
fn temporary_of_a_function_block_is_destructed_at_the_end_of_the_body() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK Worker
            VAR_TEMP
                temporary: Connection;
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    insta::assert_snapshot!(body(&project, "Worker"), @"
    Connection__ctor(temporary)
    Connection__dtor(temporary)
    ");
}

#[test]
fn temporary_of_a_program_is_destructed_at_the_end_of_the_body() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        PROGRAM prog
            VAR_TEMP
                temporary: Connection;
            END_VAR
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(body(&project, "prog"), @"
    Connection__ctor(temporary)
    Connection__dtor(temporary)
    ");
}

#[test]
fn local_of_a_method_is_destructed_at_the_end_of_the_body() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK Worker
            METHOD run
                VAR
                    local: Connection;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    insta::assert_snapshot!(body(&project, "Worker.run"), @"
    Connection__ctor(local)
    Connection__dtor(local)
    ");
}

#[test]
fn return_value_of_a_function_is_destructed_before_the_locals() {
    let project = lower(
        r#"
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION connect: Connection
            VAR
                local: Connection;
            END_VAR
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(body(&project, "connect"), @"
    Connection__ctor(local)
    Connection__ctor(connect)
    Connection__dtor(connect)
    Connection__dtor(local)
    ");
}

#[test]
fn destructor_of_an_external_function_block_is_declared_only() {
    let project = lower(
        r#"
        {external}
        FUNCTION_BLOCK Connection
            METHOD FB_EXIT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK Client
            VAR
                connection: Connection;
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_eq!(find_implementation(&project, "Connection__dtor").linkage, LinkageType::External);
    assert_eq!(find_implementation(&project, "Client__dtor").linkage, LinkageType::Internal);
    insta::assert_snapshot!(body(&project, "Client__dtor"), @"Connection__dtor(self.connection)");
}
