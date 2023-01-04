use crate::{diagnostics::Diagnostic, test_utils::tests::parse_and_validate};

#[test]
fn one_cycle_aba_var() {
    let diagnostics = parse_and_validate(
        "
                FUNCTION_BLOCK A
                    VAR
                        b : B;
                    END_VAR
                END_FUNCTION_BLOCK


                FUNCTION_BLOCK B
                    VAR
                        a : A;
                    END_VAR
                END_FUNCTION_BLOCK
                ",
    );

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0],
        Diagnostic::recursive_datastructure(
            "A -> B -> A",
            vec![(32..33).into(), (185..186).into(), (32..33).into(),]
        )
    );
}

#[test]
fn one_cycle_aba_input() {
    let diagnostics = parse_and_validate(
        "
                FUNCTION_BLOCK A
                    VAR_INPUT
                        b : B;
                    END_VAR
                END_FUNCTION_BLOCK


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
        Diagnostic::recursive_datastructure(
            "A -> B -> A",
            vec![(32..33).into(), (191..192).into(), (32..33).into()]
        )
    );
}

#[test]
fn one_cycle_aba_output() {
    let diagnostics = parse_and_validate(
        "
                FUNCTION_BLOCK A
                    VAR_OUTPUT
                        b : B;
                    END_VAR
                END_FUNCTION_BLOCK


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
        Diagnostic::recursive_datastructure(
            "A -> B -> A",
            vec![(32..33).into(), (192..193).into(), (32..33).into()]
        )
    );
}

#[test]
fn one_cycle_aba_inout() {
    let diagnostics = parse_and_validate(
        "
                FUNCTION_BLOCK A
                    VAR_IN_OUT
                        b : B;
                    END_VAR
                END_FUNCTION_BLOCK


                FUNCTION_BLOCK B
                    VAR_IN_OUT
                        a : A;
                    END_VAR
                END_FUNCTION_BLOCK
                ",
    );

    // No recursion because VAR_IN_OUT are treated as pointers
    assert_eq!(diagnostics.len(), 0);
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

            FUNCTION_BLOCK B
                VAR_INPUT
                    c : C;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK C
                VAR_INPUT
                    e : E;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK E
                VAR_INPUT
                    f : F;
                END_VAR
            END_FUNCTION_BLOCK
            
            FUNCTION_BLOCK F
                VAR_INPUT
                    g : G;
                    b : B;
                END_VAR
            END_FUNCTION_BLOCK
            
            FUNCTION_BLOCK G
                VAR_INPUT
                    h : H;
                END_VAR
            END_FUNCTION_BLOCK
            
            FUNCTION_BLOCK H
                VAR_INPUT
                    i : I;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK I
                VAR_INPUT
                    f : F;
                END_VAR
            END_FUNCTION_BLOCK
            ",
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(
        diagnostics[0],
        Diagnostic::recursive_datastructure(
            "F -> G -> H -> I -> F",
            vec![
                (592..593).into(),
                (769..770).into(),
                (919..920).into(),
                (1057..1058).into(),
                (592..593).into()
            ]
        )
    );
    assert_eq!(
        diagnostics[1],
        Diagnostic::recursive_datastructure(
            "B -> C -> E -> F -> B",
            vec![
                (166..167).into(),
                (304..305).into(),
                (442..443).into(),
                (592..593).into(),
                (166..167).into()
            ]
        )
    );
}
