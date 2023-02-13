mod edgecases {
    use crate::test_utils::tests::parse_and_validate;

    #[test]
    fn pointers_should_not_be_considered_as_cycle() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT
                b : B;
            END_STRUCT END_TYPE

            TYPE B : STRUCT
                a : REF_TO A;
            END_STRUCT END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 0);
    }

    // From https://github.com/PLC-lang/rusty/pull/748:
    // Running cargo r -- ../standardfunctions/iec61131-st/*.st previously returned a weird TIME -> TIME
    // recursion which shouldn't happen. Instead of spending time debugging that one edge-case we now
    // explicitly filter for nodes within the dfs method. As a nice-to-have this is probably also more performant.
    //
    // This test covers the above edge-case
    #[test]
    fn external_function_should_not_trigger() {
        let diagnostics = parse_and_validate(
            "
            {external}
            FUNCTION TIME : TIME
            END_FUNCTION

            TYPE niceTimes : STRUCT
                x : TIME;
                y : DATE_AND_TIME;
            END_STRUCT
            END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn struct_and_function_with_same_name() {
        let diagnostics = parse_and_validate(
            "
            FUNCTION TM : TM
            END_FUNCTION

            TYPE TM : STRUCT
                hours, minutes, seconds : DINT;
            END_STRUCT
            END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn struct_and_function_with_same_name_2() {
        let diagnostics = parse_and_validate(
            "
            FUNCTION TM : DINT
                VAR_INPUT
                    x : TM;
                END_VAR
            END_FUNCTION

            TYPE TM : STRUCT
                hours, minutes, seconds : DINT;
            END_STRUCT
            END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn struct_and_function_with_same_name_3() {
        let diagnostics = parse_and_validate(
            "
            FUNCTION TM : DINT
                VAR_INPUT
                    x : TM;
                END_VAR
            END_FUNCTION

            TYPE TM : STRUCT
                TM : DINT;
            END_STRUCT
            END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 0);
    }
}

mod structs {
    use crate::{diagnostics::Diagnostic, test_utils::tests::parse_and_validate};

    #[test]
    fn one_cycle_abca() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT
                b : B;
            END_STRUCT END_TYPE

            TYPE B : STRUCT
                c : C;
            END_STRUCT END_TYPE

            TYPE C : STRUCT
                a : A;
                e : e;
            END_STRUCT END_TYPE
            
            TYPE E : STRUCT
                a_int: INT;
            END_STRUCT END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0],
            Diagnostic::recursive_datastructure(
                "A -> B -> C -> A",
                vec![(18..19).into(), (102..103).into(), (186..187).into()]
            )
        );
    }

    #[test]
    fn one_cycle_self_a() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT
                a : A;
            END_STRUCT END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0], Diagnostic::recursive_datastructure("A -> A", vec![(18..19).into()]));
    }

    #[test]
    fn one_cycle_multiple_self_a() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT
                a1 : A;
                a2 : A;
                a3 : A;
            END_STRUCT END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0], Diagnostic::recursive_datastructure("A -> A", vec![(18..19).into()]));
    }

    #[test]
    fn one_cycle_aba() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT
                b : B;
            END_STRUCT END_TYPE

            TYPE B : STRUCT
                a : A;
            END_STRUCT END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0],
            Diagnostic::recursive_datastructure("A -> B -> A", vec![(18..19).into(), (102..103).into()])
        );
    }

    #[test]
    fn one_cycle_bcb() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT
                b : B;
            END_STRUCT END_TYPE
            
            TYPE B : STRUCT
                c : C;
            END_STRUCT END_TYPE
            
            TYPE C : STRUCT
                b : B;
            END_STRUCT END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0],
            Diagnostic::recursive_datastructure("B -> C -> B", vec![(114..115).into(), (210..211).into()])
        );
    }

    #[test]
    fn one_cycle_with_multiple_identical_members_aba() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT 
                b1 : B;
                b2 : B;
                b3 : B;
            END_STRUCT END_TYPE

            TYPE B : STRUCT 
                a : A;
            END_STRUCT END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0],
            Diagnostic::recursive_datastructure("A -> B -> A", vec![(18..19).into(), (152..153).into()])
        );
    }

    #[test]
    fn two_cycles_aa_and_aba() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT
                a : A;
                b : B;
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
            Diagnostic::recursive_datastructure("A -> B -> A", vec![(18..19).into(), (137..138).into()])
        );
    }

    #[test]
    fn two_cycles_branch_cc_and_cec() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT
                b : B;
            END_STRUCT END_TYPE
            
            TYPE B : STRUCT
                c : C;
            END_STRUCT END_TYPE
            
            TYPE C : STRUCT
                c : C;
                e : E;
            END_STRUCT END_TYPE
            
            TYPE E : STRUCT
                c : C;
            END_STRUCT END_TYPE
            ",
        );

        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0], Diagnostic::recursive_datastructure("C -> C", vec![(210..211).into()]));
        assert_eq!(
            diagnostics[1],
            Diagnostic::recursive_datastructure("C -> E -> C", vec![(210..211).into(), (329..330).into()])
        );
    }

    #[test]
    fn two_cycles_with_branch() {
        let diagnostics = parse_and_validate(
            "
            TYPE A : STRUCT
                b : B;
            END_STRUCT END_TYPE

            TYPE B : STRUCT
                c : C;
            END_STRUCT END_TYPE

            TYPE C : STRUCT
                e : E;
            END_STRUCT END_TYPE

            TYPE E : STRUCT
                f : F;
            END_STRUCT END_TYPE

            TYPE F : STRUCT
                g : G;
                b : B;
            END_STRUCT END_TYPE

            TYPE G : STRUCT
                h : H;
            END_STRUCT END_TYPE

            TYPE H : STRUCT
                i : I;
            END_STRUCT END_TYPE

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
                vec![(354..355).into(), (461..462).into(), (545..546).into(), (629..630).into(),]
            )
        );
        assert_eq!(
            diagnostics[1],
            Diagnostic::recursive_datastructure(
                "B -> C -> E -> F -> B",
                vec![(102..103).into(), (186..187).into(), (270..271).into(), (354..355).into(),]
            )
        );
    }
}

mod arrays {
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
}

mod functionblocks {
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
            Diagnostic::recursive_datastructure("A -> B -> A", vec![(28..29).into(), (161..162).into()])
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
            Diagnostic::recursive_datastructure("A -> B -> A", vec![(28..29).into(), (167..168).into()])
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
            Diagnostic::recursive_datastructure("A -> B -> A", vec![(28..29).into(), (168..169).into()])
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
                vec![(592..593).into(), (769..770).into(), (919..920).into(), (1057..1058).into(),]
            )
        );
        assert_eq!(
            diagnostics[1],
            Diagnostic::recursive_datastructure(
                "B -> C -> E -> F -> B",
                vec![(166..167).into(), (304..305).into(), (442..443).into(), (592..593).into(),]
            )
        );
    }
}

mod mixed_structs_and_functionblocks {
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
}
