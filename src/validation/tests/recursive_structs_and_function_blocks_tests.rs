use crate::{diagnostics::Diagnostic, test_utils::tests::parse_and_validate};

#[test]
fn one_cycle_aba_output() {
    let diagnostics = parse_and_validate(
        "
            TYPE A : STRUCT
                b : B;
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
        Diagnostic::recursive_datastructure("A -> B -> A", vec![(18..19).into(), (112..113).into()])
    );
}

#[test]
fn one_cycle_aba_input() {
    let diagnostics = parse_and_validate(
        "
            TYPE A : STRUCT
                b : B;
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
        Diagnostic::recursive_datastructure("A -> B -> A", vec![(18..19).into(), (112..113).into()])
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
                f : F;
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
                f : F;
            END_STRUCT END_TYPE
            ",
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(
        diagnostics[0],
        Diagnostic::recursive_datastructure(
            "F -> G -> H -> I -> F",
            vec![(484..485).into(), (651..652).into(), (757..758).into(), (885..886).into(),]
        )
    );
    assert_eq!(
        diagnostics[1],
        Diagnostic::recursive_datastructure(
            "B -> C -> E -> F -> B",
            vec![(156..157).into(), (250..251).into(), (378..379).into(), (484..485).into(),]
        )
    );
}
