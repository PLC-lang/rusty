use crate::{diagnostics::Diagnostic, test_utils::tests::parse_and_validate};

#[test]
fn two_cycles_aa_and_aba() {
    let diagnostics = parse_and_validate(
        "
            TYPE A : STRUCT
                a : ARRAY[0..1] OF A;
                b : ARRAY[0..1] OF B;
            END_STRUCT END_TYPE
            
            TYPE B : STRUCT
                a : A;
            END_STRUCT END_TYPE
            ",
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0], Diagnostic::recursive_datastructure("A -> A", vec![(18..19).into()]));
    assert_eq!(
        diagnostics[1],
        Diagnostic::recursive_datastructure("A -> B -> A", vec![(18..19).into(), (167..168).into()])
    );
}

#[test]
fn one_cycle_bcb() {
    let diagnostics = parse_and_validate(
        "
            TYPE A : STRUCT
                b : ARRAY[0..1] OF B;
            END_STRUCT END_TYPE
            
            TYPE B : STRUCT
                c : ARRAY[0..1] OF C;
            END_STRUCT END_TYPE
            
            TYPE C : STRUCT
                b : ARRAY[0..1] OF B;
            END_STRUCT END_TYPE
            ",
    );

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0],
        Diagnostic::recursive_datastructure("B -> C -> B", vec![(129..130).into(), (240..241).into()])
    );
}

#[test]
fn one_cycle_with_multiple_identical_members_aba() {
    let diagnostics = parse_and_validate(
        "
            TYPE A : STRUCT 
                b1 : ARRAY[0..1] OF B;
                b2 : ARRAY[0..1] OF B;
                b3 : ARRAY[0..1] OF B;
            END_STRUCT END_TYPE

            TYPE B : STRUCT 
                a : A;
            END_STRUCT END_TYPE
            ",
    );

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0],
        Diagnostic::recursive_datastructure("A -> B -> A", vec![(18..19).into(), (197..198).into()])
    );
}

#[test]
fn one_cycle_aba_output() {
    let diagnostics = parse_and_validate(
        "
            TYPE A : STRUCT
                b : ARRAY [0..1] OF B;
            END_STRUCT END_TYPE

            FUNCTION_BLOCK B
                VAR_OUTPUT
                    a : A;
                END_VAR
            END_FUNCTION_BLOCK
            ",
    );

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0],
        Diagnostic::recursive_datastructure("A -> B -> A", vec![(18..19).into(), (128..129).into()])
    );
}

#[test]
fn one_cycle_aba_input() {
    let diagnostics = parse_and_validate(
        "
            TYPE A : STRUCT
                b : ARRAY [0..1] OF B;
            END_STRUCT END_TYPE

            FUNCTION_BLOCK B
                VAR_INPUT
                    a : A;
                END_VAR
            END_FUNCTION_BLOCK
            ",
    );

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0],
        Diagnostic::recursive_datastructure("A -> B -> A", vec![(18..19).into(), (128..129).into()])
    );
}

#[test]
fn two_cycles_with_branch_input() {
    let diagnostics = parse_and_validate(
        "
            FUNCTION_BLOCK A
                VAR_INPUT
                    b : B;
                END_VAR
            END_FUNCTION_BLOCK

            TYPE B : STRUCT
                c : C;
            END_STRUCT END_TYPE

            FUNCTION_BLOCK C
                VAR_INPUT
                    e : E;
                END_VAR
            END_FUNCTION_BLOCK

            TYPE E : STRUCT
                f : ARRAY [0..1] OF F;
            END_STRUCT END_TYPE
            
            FUNCTION_BLOCK F
                VAR_INPUT
                    g : G;
                    b : B;
                END_VAR
            END_FUNCTION_BLOCK
            
            TYPE G : STRUCT
                h : H;
            END_STRUCT END_TYPE
            
            FUNCTION_BLOCK H
                VAR_INPUT
                    i : I;
                END_VAR
            END_FUNCTION_BLOCK

            TYPE I : STRUCT
                f : ARRAY [0..1] OF F;
            END_STRUCT END_TYPE
            ",
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(
        diagnostics[0],
        Diagnostic::recursive_datastructure(
            "F -> G -> H -> I -> F",
            vec![(500..501).into(), (667..668).into(), (773..774).into(), (901..902).into(),]
        )
    );
    assert_eq!(
        diagnostics[1],
        Diagnostic::recursive_datastructure(
            "B -> C -> E -> F -> B",
            vec![(156..157).into(), (250..251).into(), (378..379).into(), (500..501).into(),]
        )
    );
}
