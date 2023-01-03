use std::cell::Cell;

use indexmap::{IndexMap, IndexSet};

use crate::{
    diagnostics::{Diagnostic, ErrNo},
    index::Index,
    typesystem::DataTypeInformationProvider,
};

/// Validator to find and report all recursive data structures using Depth-first search (DFS)[1].
/// Such data structures consists of structs and function-blocks, for example the following code would be
/// flagged as a recursive data structure:
/// ```st
/// TYPE A : STRUCT
///     b : B;
/// END_STRUCT END_TYPE
///
/// TYPE B : STRUCT
///     a : A;
/// END_STRUCT END_TYPE
/// ```
/// Without a validator the compiler would panic because generating `A -> B -> A -> ...` is impossible as it
/// overflows the stack.
///
/// [1] https://en.wikipedia.org/wiki/Depth-first_search
pub struct RecursiveValidator {
    pub diagnostics: Vec<Diagnostic>,
}

/// Status of whether a node has been visited or not.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Status {
    Visited,
    Unvisited,
}

impl RecursiveValidator {
    pub fn new() -> RecursiveValidator {
        RecursiveValidator { diagnostics: Vec::new() }
    }

    /// Entry point of finding and reporting all recursive data structures.
    pub fn validate_recursion(&mut self, index: &Index) {
        let structs = index
            .get_types()
            .values()
            .filter(|x| x.get_type_information().is_struct())
            .map(|x| (x.get_name(), Cell::new(Status::Unvisited)))
            .collect();

        let fbs = index
            .get_pous()
            .values()
            .filter(|x| x.is_function_block())
            .map(|x| (x.get_name(), Cell::new(Status::Unvisited)))
            .collect();

        for nodes in vec![structs, fbs] {
            self.find_cycle(index, nodes);
        }
    }

    /// Finds cycles for the given nodes.
    fn find_cycle<'idx>(&mut self, index: &'idx Index, nodes: IndexMap<&'idx str, Cell<Status>>) {
        let mut path = IndexSet::new();

        for node in &nodes {
            if node.1.get() == Status::Unvisited {
                self.dfs(index, node.0, &nodes, &mut path);
            }
        }
    }

    /// In DFS manner recursively visits a node and all its child nodes while simultaneously creating a path
    /// of it. Ends either by detecting a cycle, i.e. re-visting a node that is already present in our path,
    /// or by reaching a node with no further child nodes. In the former case the cycle is added to the
    /// diagnostician. In the latter case the function goes back one recursion re-doing the mentioned steps
    /// until either again a cycle is found or all nodes have been visited.
    fn dfs<'idx>(
        &mut self,
        index: &'idx Index,
        curr_node: &'idx str,
        nodes: &IndexMap<&'idx str, Cell<Status>>,
        path: &mut IndexSet<&'idx str>,
    ) {
        nodes[curr_node].set(Status::Visited);
        path.insert(curr_node);

        if let Some(edges) = index.get_members(curr_node) {
            for node in edges.values().map(|x| x.get_type_name()).collect::<IndexSet<_>>() {
                if let Some(status) = nodes.get(node) {
                    match status.get() {
                        Status::Visited if path.contains(node) => self.report(index, path, node),
                        Status::Visited | Status::Unvisited => self.dfs(index, node, nodes, path),
                    }
                }
            }
        }

        path.pop();
    }

    /// Generates and reports the minimal path of a cycle. Specifically `path` contains all nodes visited up
    /// until a cycle, e.g. `A -> B -> C -> B`. We are only interested in `B -> C -> B` as such this method
    /// finds the first occurence of `B` to create a vector slice of `B -> C -> B` for the diagnostician.
    fn report<'idx>(&mut self, index: &'idx Index, path: &mut IndexSet<&'idx str>, node: &'idx str) {
        match path.get_index_of(node) {
            Some(idx) => {
                // Extract the cycle from the full path and append the cyclic node to the tail for the report
                let mut slice = path.iter().skip(idx).copied().collect::<Vec<_>>();
                slice.push(node);

                self.diagnostics.push(Diagnostic::SyntaxError {
                    message: format!("Recursive data structure `{}` has infinite size", slice.join(" -> ")),
                    range: slice
                        .iter()
                        .map(|node| index.get_type(node).unwrap().location.source_range.to_owned())
                        .collect::<Vec<_>>(),
                    err_no: ErrNo::pou__recursive_data_structure,
                });
            }

            None => unreachable!("Node has to be in the vector"),
        }
    }
}

// Thank you ChatGPT for suggesting some test cases, much appreciated <3
#[cfg(test)]
mod tests {
    fn generate_message(path: &'static str) -> String {
        format!("Recursive data structure `{path}` has infinite size")
    }

    mod structs {
        use crate::{
            diagnostics::ErrNo, test_utils::tests::parse_and_validate,
            validation::recursive_validator::tests::generate_message,
        };

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
            assert_eq!(diagnostics[0].get_message(), generate_message("A -> B -> C -> A"));
            assert_eq!(diagnostics[0].get_type(), &ErrNo::pou__recursive_data_structure);
            assert_eq!(
                diagnostics[0].get_affected_ranges(),
                vec![(18..19).into(), (102..103).into(), (186..187).into(), (18..19).into()]
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
            assert_eq!(diagnostics[0].get_message(), generate_message("A -> A"));
            assert_eq!(diagnostics[0].get_affected_ranges(), vec![(18..19).into(), (18..19).into()]);
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
            assert_eq!(diagnostics[0].get_message(), generate_message("A -> A"));
            assert_eq!(diagnostics[0].get_affected_ranges(), vec![(18..19).into(), (18..19).into()]);
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
            assert_eq!(diagnostics[0].get_message(), generate_message("A -> B -> A"));
            assert_eq!(diagnostics[0].get_type(), &ErrNo::pou__recursive_data_structure);
            assert_eq!(
                diagnostics[0].get_affected_ranges(),
                vec![(18..19).into(), (102..103).into(), (18..19).into()]
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
            assert_eq!(diagnostics[0].get_message(), generate_message("B -> C -> B"));
            assert_eq!(
                diagnostics[0].get_affected_ranges(),
                vec![(114..115).into(), (210..211).into(), (114..115).into()]
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
            assert_eq!(diagnostics[0].get_message(), generate_message("A -> B -> A"));
            assert_eq!(
                diagnostics[0].get_affected_ranges(),
                vec![(18..19).into(), (152..153).into(), (18..19).into()]
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
            assert_eq!(diagnostics[0].get_message(), generate_message("A -> A"));
            assert_eq!(diagnostics[0].get_affected_ranges(), vec![(18..19).into(), (18..19).into()]);

            assert_eq!(diagnostics[1].get_message(), generate_message("A -> B -> A"));
            assert_eq!(
                diagnostics[1].get_affected_ranges(),
                vec![(18..19).into(), (137..138).into(), (18..19).into()]
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
            assert_eq!(diagnostics[0].get_message(), generate_message("C -> C"));
            assert_eq!(diagnostics[0].get_affected_ranges(), vec![(210..211).into(), (210..211).into()]);

            assert_eq!(diagnostics[1].get_message(), generate_message("C -> E -> C"));
            assert_eq!(
                diagnostics[1].get_affected_ranges(),
                vec![(210..211).into(), (329..330).into(), (210..211).into()]
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

            assert_eq!(diagnostics[0].get_message(), generate_message("F -> G -> H -> I -> F"),);
            assert_eq!(diagnostics[0].get_type(), &ErrNo::pou__recursive_data_structure);
            assert_eq!(
                diagnostics[0].get_affected_ranges(),
                vec![
                    (354..355).into(),
                    (461..462).into(),
                    (545..546).into(),
                    (629..630).into(),
                    (354..355).into()
                ]
            );

            assert_eq!(diagnostics[1].get_message(), generate_message("B -> C -> E -> F -> B"),);
            assert_eq!(diagnostics[1].get_type(), &ErrNo::pou__recursive_data_structure);
            assert_eq!(
                diagnostics[1].get_affected_ranges(),
                vec![
                    (102..103).into(),
                    (186..187).into(),
                    (270..271).into(),
                    (354..355).into(),
                    (102..103).into()
                ]
            );
        }
    }

    mod function_blocks {
        use crate::{
            diagnostics::ErrNo, test_utils::tests::parse_and_validate,
            validation::recursive_validator::tests::generate_message,
        };

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

            assert_eq!(diagnostics[0].get_message(), generate_message("A -> B -> A"));
            assert_eq!(diagnostics[0].get_type(), &ErrNo::pou__recursive_data_structure);
            assert_eq!(
                diagnostics[0].get_affected_ranges(),
                vec![(32..33).into(), (185..186).into(), (32..33).into(),]
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

            assert_eq!(diagnostics[0].get_message(), generate_message("A -> B -> A"));
            assert_eq!(diagnostics[0].get_type(), &ErrNo::pou__recursive_data_structure);
            assert_eq!(
                diagnostics[0].get_affected_ranges(),
                vec![(32..33).into(), (191..192).into(), (32..33).into(),]
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

            assert_eq!(diagnostics[0].get_message(), generate_message("A -> B -> A"));
            assert_eq!(diagnostics[0].get_type(), &ErrNo::pou__recursive_data_structure);
            assert_eq!(
                diagnostics[0].get_affected_ranges(),
                vec![(32..33).into(), (192..193).into(), (32..33).into(),]
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

            assert_eq!(diagnostics.len(), 0);
        }
    }
}
