use crate::{diagnostics::Diagnostic, test_utils::tests::parse_and_validate};

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
