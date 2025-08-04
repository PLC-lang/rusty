use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

/// tests wheter simple local and global variables can be resolved and
/// errors are reported properly
#[test]
fn resolve_simple_variable_references() {
    let diagnostics = parse_and_validate_buffered(
        "
            VAR_GLOBAL
                ga : INT;
            END_VAR

            PROGRAM prg
                VAR a : INT; END_VAR

                a;
                b;
                ga;
                gb;

           END_PROGRAM
       ",
    );

    assert_snapshot!(diagnostics);
}

/// tests wheter functions and function parameters can be resolved and
/// errors are reported properly
#[test]
fn resolve_function_calls_and_parameters() {
    let diagnostics = parse_and_validate_buffered(
        "
           PROGRAM prg
                VAR a : INT; END_VAR
                foo(a);
                boo(c);
                foo(x := a);
                foo(x := c);
                foo(y := a);
            END_PROGRAM

            FUNCTION foo : INT
                VAR_INPUT x : INT; END_VAR
            END_FUNCTION
        ",
    );

    assert_snapshot!(&diagnostics);
}

/// tests wheter structs and struct member variables can be resolved and
/// errors are reported properly
#[test]
fn resole_struct_member_access() {
    let diagnostics = parse_and_validate_buffered(
        "
            TYPE MySubStruct: STRUCT
                subfield1: INT;
                subfield2: INT;
                subfield3: INT;
                END_STRUCT
            END_TYPE

            TYPE MyStruct: STRUCT
                field1: INT;
                field2: INT;
                field3: INT;
                sub: MySubStruct;
                END_STRUCT
            END_TYPE

            PROGRAM prg
                VAR
                    a : INT;
                    s : MyStruct;
                END_VAR
                (* should be fine *)
                s.field1;
                s.field2;
                s.field3;

                (* should not exist *)
                s.field10;
                s.field20;
                s.field30;

                (* should be fine*)
                s.sub.subfield1;
                s.sub.subfield2;
                s.sub.subfield3;

                (* should not exist*)
                s.sub.subfield10;
                s.sub.subfield20;
                s.sub.subfield30;
           END_PROGRAM
       ",
    );

    assert_snapshot!(&diagnostics);
}

/// tests wheter function_block members can be resolved and
/// errors are reported properly
#[test]
fn resolve_function_block_calls_field_access() {
    let diagnostics = parse_and_validate_buffered(
        "
            FUNCTION_BLOCK FB
                VAR_INPUT
                    a,b,c : INT;
                END_VAR
            END_FUNCTION_BLOCK

            PROGRAM prg
                VAR
                    s : FB;
                END_VAR
                s;
 (*               s.a;
                s.b;
                s.c;
                s(a := 1, b := 2, c := 3);
                s(a := s.a, b := s.b, c := s.c);
                (* problem - x,y,z do not not exist *)
                s(a := s.x, b := s.y, c := s.z); *)
            END_PROGRAM
       ",
    );

    assert!(diagnostics.is_empty());
}

/// tests wheter function_block types and member variables can be resolved and
/// errors are reported properly
#[test]
fn resolve_function_block_calls_in_structs_and_field_access() {
    let diagnostics = parse_and_validate_buffered(
        "
            FUNCTION_BLOCK FB
                VAR_INPUT
                    a,b,c : INT;
                END_VAR
            END_FUNCTION_BLOCK

            TYPE MyStruct: STRUCT
                fb1: FB;
                fb2: FB;
                END_STRUCT
            END_TYPE

           PROGRAM prg
                VAR
                    s : MyStruct;
                END_VAR

                s.fb1.a;
                s.fb1.b;
                s.fb1.c;
                s.fb1(a := 1, b := 2, c := 3);
                s.fb1(a := s.fb2.a, b := s.fb2.b, c := s.fb2.c);
                (* problem - sb3 does not exist *)
                s.fb1(a := s.fb3.a, b := s.fb3.b, c := s.fb3.c);
           END_PROGRAM
       ",
    );

    assert_snapshot!(&diagnostics);
}

/// tests whether references to privater variables do resolve, but end up in an validation problem
#[test]
fn reference_to_private_variable_is_illegal() {
    let diagnostics = parse_and_validate_buffered(
        "
            PROGRAM prg
                VAR
                    s : INT;
                END_VAR
            END_PROGRAM

            FUNCTION foo : INT
                prg.s := 7;
            END_FUNCTION
       ",
    );

    assert_snapshot!(&diagnostics);
}

/// tests whether an intermediate access like: `a.priv.b` (where priv is a var_local)
/// produces the correct error message without follow-up errors (b)
#[test]
fn reference_to_private_variable_in_intermediate_fb() {
    // GIVEN a qualified reference prg.a.f.x where f is a
    // private member of a functionblock (VAR)
    // WHEN this is validated
    let diagnostics = parse_and_validate_buffered(
        "
            FUNCTION_BLOCK fb1
                VAR
                    f: fb2;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2
                VAR_INPUT
                    x : INT;
                END_VAR
            END_FUNCTION_BLOCK


            PROGRAM prg
                VAR
                    a: fb1;
                END_VAR
                a.f.x := 7;
            END_PROGRAM
       ",
    );

    // THEN we get a validtion-error for accessing fb1.f, but no follow-up errors for
    // the access of fb2 which is legit
    assert_snapshot!(&diagnostics);
}

#[test]
fn program_vars_are_allowed_in_their_actions() {
    let diagnostics = parse_and_validate_buffered(
        "
            PROGRAM prg
                VAR
                    s : INT;
                END_VAR
            END_PROGRAM

            ACTION prg.foo
                prg.s := 7;
                s := 7;
            END_ACTION
       ",
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn fb_pointer_access_call_statement_resolves_without_validation_errors() {
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM main
        VAR
            FileOpen : REF_TO file;
        END_VAR
            FileOpen^(var2:=TRUE);
        END_PROGRAM

        FUNCTION_BLOCK file
        VAR_INPUT
            var1 : BOOL;
            var2 : BOOL;
        END_VAR
        VAR_OUTPUT
        END_VAR
        VAR
        END_VAR
        END_FUNCTION_BLOCK
       ",
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn resolve_array_of_struct_as_member_of_another_struct_initializer() {
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM mainProg
        VAR
            var_str1 : STRUCT1 := (myArr := [(x1 := FALSE, x2 := TRUE)]);
        END_VAR
        END_PROGRAM

        TYPE STRUCT1 :
            STRUCT
                myArr : ARRAY[0..10] OF STRUCT2;
            END_STRUCT
        END_TYPE

        TYPE STRUCT2 :
            STRUCT
                x1 : BOOL;
                x2 : BOOL;
                x3 : DINT;
                x4 : DINT;
            END_STRUCT
        END_TYPE

       ",
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn array_of_struct_as_member_of_another_struct_and_variable_declaration_is_initialized() {
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM mainProg
        VAR
            var_str1 : ARRAY[1..5] OF STRUCT1 := [
                (myInt := 1, myArr := [(x1 := TRUE, x2 := 128), (x1 := FALSE, x2 := 1024)]),
                (myInt := 2, myArr := [(x1 := TRUE, x2 := 256), (x1 := FALSE, x2 := 2048)])
            ];
        END_VAR
        END_PROGRAM

        TYPE STRUCT1 :
            STRUCT
                myInt : INT;
                myArr : ARRAY[0..4] OF STRUCT2;
            END_STRUCT
        END_TYPE

        TYPE STRUCT2 :
            STRUCT
                x1 : BOOL;
                x2 : DINT;
            END_STRUCT
        END_TYPE
       ",
    );

    assert!(diagnostics.is_empty());
}
