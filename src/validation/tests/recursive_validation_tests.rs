mod edgecases {
    use crate::test_utils::tests::parse_and_validate_buffered;

    #[test]
    fn pointers_should_not_be_considered_as_cycle() {
        let diagnostics = parse_and_validate_buffered(
            "
            TYPE A : STRUCT
                b : B;
            END_STRUCT END_TYPE

            TYPE B : STRUCT
                a : REF_TO A;
            END_STRUCT END_TYPE
            ",
        );

        assert!(diagnostics.is_empty());
    }

    // From https://github.com/PLC-lang/rusty/pull/748:
    // Running cargo r -- ../standardfunctions/iec61131-st/*.st previously returned a weird TIME -> TIME
    // recursion which shouldn't happen. Instead of spending time debugging that one edge-case we now
    // explicitly filter for nodes within the dfs method. As a nice-to-have this is probably also more performant.
    //
    // This test covers the above edge-case
    #[test]
    fn external_function_should_not_trigger() {
        let diagnostics = parse_and_validate_buffered(
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

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn struct_and_function_with_same_name() {
        let diagnostics = parse_and_validate_buffered(
            "
            FUNCTION TM : TM
            END_FUNCTION

            TYPE TM : STRUCT
                hours, minutes, seconds : DINT;
            END_STRUCT
            END_TYPE
            ",
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn struct_and_function_with_same_name_2() {
        let diagnostics = parse_and_validate_buffered(
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

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn struct_and_function_with_same_name_3() {
        let diagnostics = parse_and_validate_buffered(
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

        assert!(diagnostics.is_empty());
    }
}

mod structs {
    use insta::assert_snapshot;

    use crate::test_utils::tests::parse_and_validate_buffered;

    #[test]
    fn one_cycle_abca() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_self_a() {
        let diagnostics = parse_and_validate_buffered(
            "
            TYPE A : STRUCT
                a : A;
            END_STRUCT END_TYPE
            ",
        );

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_multiple_self_a() {
        let diagnostics = parse_and_validate_buffered(
            "
            TYPE A : STRUCT
                a1 : A;
                a2 : A;
                a3 : A;
            END_STRUCT END_TYPE
            ",
        );

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_aba() {
        let diagnostics = parse_and_validate_buffered(
            "
            TYPE A : STRUCT
                b : B;
            END_STRUCT END_TYPE

            TYPE B : STRUCT
                a : A;
            END_STRUCT END_TYPE
            ",
        );

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_bcb() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_with_multiple_identical_members_aba() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn two_cycles_aa_and_aba() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn two_cycles_branch_cc_and_cec() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn two_cycles_with_branch() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }
}

mod arrays {
    use insta::assert_snapshot;

    use crate::test_utils::tests::parse_and_validate_buffered;

    #[test]
    fn two_cycles_aa_and_aba() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_bcb() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_with_multiple_identical_members_aba() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_aba_output() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_aba_input() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn two_cycles_with_branch_input() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }
}

mod functionblocks {
    use insta::assert_snapshot;

    use crate::test_utils::tests::parse_and_validate_buffered;

    #[test]
    fn one_cycle_aba_var() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_aba_input() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_aba_output() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_aba_inout() {
        let diagnostics = parse_and_validate_buffered(
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
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn two_cycles_with_branch_input() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }
}

mod mixed_structs_and_functionblocks {
    use insta::assert_snapshot;

    use crate::test_utils::tests::parse_and_validate_buffered;

    #[test]
    fn one_cycle_aba_output() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn one_cycle_aba_input() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }

    #[test]
    fn two_cycles_with_branch_input() {
        let diagnostics = parse_and_validate_buffered(
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

        assert_snapshot!(&diagnostics);
    }
}

mod enums {
    use crate::test_utils::tests::parse_and_validate_buffered;

    #[test]
    fn enums_are_not_considered_for_duplicate_checks() {
        //... because they're integers (duh)
        let diagnostics = parse_and_validate_buffered(
            "
            FUNCTION_BLOCK FOO
            VAR
                foo_enum : (start, stop) := stop;
            END_VAR
            END_FUNCTION_BLOCK
            ",
        );

        assert!(diagnostics.is_empty());
    }
}
