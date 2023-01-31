use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

/// tests wheter simple local and global variables can be resolved and
/// errors are reported properly
#[test]
fn resolve_simple_variable_references() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unresolved_reference("b", (168..169).into()),
            Diagnostic::unresolved_reference("gb", (207..209).into()),
        ]
    );
}

/// tests wheter functions and function parameters can be resolved and
/// errors are reported properly
#[test]
fn resolve_function_calls_and_parameters() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unresolved_reference("boo", (101..104).into()),
            Diagnostic::unresolved_reference("c", (105..106).into()),
            Diagnostic::unresolved_reference("c", (163..164).into()),
            Diagnostic::unresolved_reference("y", (187..188).into()),
        ]
    );
}

/// tests wheter structs and struct member variables can be resolved and
/// errors are reported properly
#[test]
fn resole_struct_member_access() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unresolved_reference("field10", (694..701).into()),
            Diagnostic::unresolved_reference("field20", (721..728).into()),
            Diagnostic::unresolved_reference("field30", (748..755).into()),
            Diagnostic::unresolved_reference("subfield10", (955..965).into()),
            Diagnostic::unresolved_reference("subfield20", (989..999).into()),
            Diagnostic::unresolved_reference("subfield30", (1023..1033).into()),
        ]
    );
}

/// tests wheter function_block members can be resolved and
/// errors are reported properly
#[test]
fn resolve_function_block_calls_field_access() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(diagnostics, vec![]);
}

/// tests wheter function_block types and member variables can be resolved and
/// errors are reported properly
#[test]
fn resolve_function_block_calls_in_structs_and_field_access() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unresolved_reference("fb3", (650..653).into()),
            Diagnostic::unresolved_reference("a", (654..655).into()),
            Diagnostic::unresolved_reference("fb3", (664..667).into()),
            Diagnostic::unresolved_reference("b", (668..669).into()),
            Diagnostic::unresolved_reference("fb3", (678..681).into()),
            Diagnostic::unresolved_reference("c", (682..683).into()),
        ]
    );
}

/// tests wheter function's members cannot be access using the function's name as a qualifier
#[test]
fn resolve_function_members_via_qualifier() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
                VAR 
                    s : MyStruct;
                END_VAR
                foo(a := 1, b := 2, c := 3);    (* ok *)
                foo.a; (* not ok *)
                foo.b; (* not ok *)
                foo.c; (* not ok *)
            END_PROGRAM

            FUNCTION foo : INT
                VAR_INPUT
                    a,b,c : INT;
                END_VAR
            END_FUNCTION
       ",
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unresolved_reference("a", (181..182).into()),
            Diagnostic::unresolved_reference("b", (217..218).into()),
            Diagnostic::unresolved_reference("c", (253..254).into()),
        ]
    );
}

/// tests whether references to privater variables do resolve, but end up in an validation problem
#[test]
fn reference_to_private_variable_is_illegal() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(diagnostics, vec![Diagnostic::illegal_access("prg.s", (175..176).into()),]);
}

/// tests whether an intermediate access like: `a.priv.b` (where priv is a var_local)
/// produces the correct error message without follow-up errors (b)
#[test]
fn reference_to_private_variable_in_intermediate_fb() {
    // GIVEN a qualified reference prg.a.f.x where f is a
    // private member of a functionblock (VAR)
    // WHEN this is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(diagnostics, vec![Diagnostic::illegal_access("fb1.f", (413..414).into()),]);
}

#[test]
fn program_vars_are_allowed_in_their_actions() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(diagnostics, vec![]);
}

#[test]
fn fb_pointer_access_call_statement_resolves_without_validation_errors() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(diagnostics, vec![]);
}
