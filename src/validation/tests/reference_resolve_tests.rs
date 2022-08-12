use crate::ast::SourceRange;
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
            Diagnostic::unresolved_reference("b", SourceRange::new(168..169,Some(9),Some(17),Some(9),Some(18))),
            Diagnostic::unresolved_reference("gb", SourceRange::new(207..209,Some(11),Some(17),Some(11),Some(19))),
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
            Diagnostic::unresolved_reference("boo", SourceRange::new(101..104,Some(4),Some(17),Some(4),Some(20))),
            Diagnostic::unresolved_reference("c", SourceRange::new(105..106,Some(4),Some(21),Some(4),Some(22))),
            Diagnostic::unresolved_reference("c", SourceRange::new(163..164,Some(6),Some(26),Some(6),Some(27))),
            Diagnostic::unresolved_reference("y", SourceRange::new(187..188,Some(7),Some(21),Some(7),Some(22))),
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
            Diagnostic::unresolved_reference("field10", SourceRange::new(694..701,Some(27),Some(19),Some(27),Some(24))),
            Diagnostic::unresolved_reference("field20", SourceRange::new(721..728,Some(28),Some(19),Some(28),Some(24))),
            Diagnostic::unresolved_reference("field30", SourceRange::new(748..755,Some(29),Some(19),Some(29),Some(24))),
            Diagnostic::unresolved_reference("subfield10", SourceRange::new(955..965,Some(37),Some(19),Some(37),Some(24))),
            Diagnostic::unresolved_reference("subfield20", SourceRange::new(989..999,Some(38),Some(19),Some(38),Some(24))),
            Diagnostic::unresolved_reference("subfield30", SourceRange::new(1023..1033,Some(39),Some(19),Some(39),Some(24))),
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
            Diagnostic::unresolved_reference("fb3", SourceRange::new(650..653,Some(24),Some(30),Some(24),Some(33))),
            Diagnostic::unresolved_reference("a", SourceRange::new(654..655,Some(24),Some(34),Some(24),Some(35))),
            Diagnostic::unresolved_reference("fb3", SourceRange::new(664..667,Some(24),Some(44),Some(24),Some(47))),
            Diagnostic::unresolved_reference("b", SourceRange::new(668..669,Some(24),Some(48),Some(24),Some(49))),
            Diagnostic::unresolved_reference("fb3", SourceRange::new(678..681,Some(24),Some(58),Some(24),Some(61))),
            Diagnostic::unresolved_reference("c", SourceRange::new(682..683,Some(24),Some(62),Some(24),Some(63))),
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
            Diagnostic::unresolved_reference("a", SourceRange::new(181..182,Some(6),Some(21),Some(6),Some(22))),
            Diagnostic::unresolved_reference("b", SourceRange::new(217..218,Some(7),Some(21),Some(7),Some(22))),
            Diagnostic::unresolved_reference("c", SourceRange::new(253..254,Some(8),Some(21),Some(8),Some(22))),
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

    assert_eq!(
        diagnostics,
        vec![Diagnostic::illegal_access("prg.s", SourceRange::new(175..176,Some(8),Some(21),Some(8),Some(22))),]
    );
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
    assert_eq!(
        diagnostics,
        vec![Diagnostic::illegal_access("fb1.f", SourceRange::new(413..414,Some(18),Some(19),Some(18),Some(20))),]
    );
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
