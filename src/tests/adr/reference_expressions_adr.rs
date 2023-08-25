use crate::test_utils::tests::parse_and_preprocess;

/// # Architecture Design Records: Representation of Reference Expressions
///
/// Reference Experssions are expressions that reference variables, POUs,
/// datatypes, etc. There are traditional references that point to a value of
/// a local (`i`), global (`PI`), or even a member-variable (`point.x`). Furthermore
/// there are reference-expressions that point to an element of an array (`x[3]`),
/// dereference a pointer (`pNext^`) or references that access the address of a variable
/// (`&next`).
///
/// There is only one AST-Statement-Variant representing all different kinds of references. It
/// contains an access-member which represents the different kinds of accessing an element:
/// ReferenceExpression
/// - Member-Access (accessing a member of a struct, pou, etc.)
/// - Index-Access (accessing an element of an array)
/// - Casting a value
/// - Dereferencing a pointer
/// - accessing the Address of a variable


/// A flat reference is treated as a Qualified-Reference with no qualifier.
/// This means that a flat reference (a) and a qualified reference (a.b) 
/// are represented using the same AST-Structure. One has no qualifier (None), 
/// the other one has.
#[test]
fn representation_of_a_flat_reference() {
    let (unit, _) = parse_and_preprocess(
        "
    PROGRAM prg
        point;
    END_PROGRAM
    ");

    let statement = &unit.implementations[0].statements[0];
    // Note that `point` is a Member-AST strcture where base=None
    insta::assert_debug_snapshot!(statement, @r###"
    ReferenceExpr {
        kind: Member(
            Reference {
                name: "point",
            },
        ),
        base: None,
    }
    "###);
}

/// A qualified reference makes use of the recursive characteristics of
/// a Member-Access. Note that the sequence of the reference elements 
/// (obj, position, x) is actually stored in reverse order in the recursive
/// composite pattern (x -> position -> obj). While the other approach feels more 
/// intuitive on first sight, this representation has its benefits when recursively 
/// visiting the AST since every element has direct access to elements that define 
/// its context (e.g. when visiting `x`, you have full access to `position`).
#[test]
fn representation_of_a_qualified_reference() {
    let (unit, _) = parse_and_preprocess(
        "
    PROGRAM prg
        obj.position.x;
    END_PROGRAM
    ");

    let statement = &unit.implementations[0].statements[0];
    // Note that the expression is a recursive datastructure.
    // it is stored backwards! (x -> position -> obj). This representation
    // helps during type-resolving, validation and code-generation
    insta::assert_debug_snapshot!(statement, @r###"
    ReferenceExpr {
        kind: Member(
            Reference {
                name: "x",
            },
        ),
        base: Some(
            ReferenceExpr {
                kind: Member(
                    Reference {
                        name: "position",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Reference {
                                name: "obj",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ),
    }
    "###);
}

/// Accessing an element of an array simply stores the index-statement within the
/// Index-Variant and holds the array-reference in its base. Note that the index-Reference
/// may be a fully fletched Reference-Expression itself.
#[test]
fn representation_of_an_array_expression_reference() {
    let (unit, _) = parse_and_preprocess(
        "
    PROGRAM prg
        obj.pos[2];
    END_PROGRAM
    ");

    let statement = &unit.implementations[0].statements[0];
    // Note that the root of this expression is an Index-Access with a base-expression (again reversed order)
    insta::assert_debug_snapshot!(statement, @r###"
    ReferenceExpr {
        kind: Index(
            LiteralInteger {
                value: 2,
            },
        ),
        base: Some(
            ReferenceExpr {
                kind: Member(
                    Reference {
                        name: "pos",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Reference {
                                name: "obj",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ),
    }
    "###);
}


/// Address-of and Deref expressions are stateless ReferenceAccess-Variants. They simply indicate
/// the accessing operation and offer the operator as their base. Note the difference in the way they
/// handle their base:
/// - the adress-of operator treats the following reference-expression as one statement (`&(a.b.c)`) 
///   rather than only operating on the single segment right next to it (`(&a).b.c`).
/// - the deref operator only operates on the segment it is attache to.
#[test]
fn representation_of_an_pointer_expression_reference() {
    let (unit, _) = parse_and_preprocess(
        "
    PROGRAM prg
        &obj.pos;
        obj.pos^;
    END_PROGRAM
    ");

    let address_of = &unit.implementations[0].statements[0];
    insta::assert_debug_snapshot!(address_of, @r###"
    ReferenceExpr {
        kind: Address,
        base: Some(
            ReferenceExpr {
                kind: Member(
                    Reference {
                        name: "pos",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Reference {
                                name: "obj",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ),
    }
    "###);

    let deref = &unit.implementations[0].statements[1];
    insta::assert_debug_snapshot!(deref, @r###"
    "###);
}

