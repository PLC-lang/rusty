// TODOs:
// - [x] Include ranges in reports (first node in path)
// - [x] Correctly report tail-end cycles (where the first node is not part of a cycle; mathias' example)
// - [x] Fix naming of tests ;)
// - [ ] Include ranges in unit tests
// - [x] Improve error message
// - [x] Structs
// - [?] Arrays
// - [x] Function blocks

// - [ ] Discuss aborting codegen for specific errors with team, specifically:
// We try to generate the LLVM IR which results in a stack overflow because of recursion
// Specifically this line in `lib.rs` panics:
// let llvm_index = code_generator.generate_llvm_index(&annotations, index.all_literals, &full_index, &diagnostician)?;
use indexmap::{IndexMap, IndexSet};

use crate::{
    diagnostics::{Diagnostic, ErrNo},
    index::Index,
    typesystem::DataTypeInformationProvider,
};

/// Validator to find and report all recursive data structures using Depth-first search (DFS)[1].
/// Such data structures consists of structs and function-blocks, for example the following code would be
/// flagged as a recursive data structure:
/// ```ignore
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
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Status {
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
            .map(|x| (x.get_name(), Status::Unvisited))
            .collect();

        let fbs = index
            .get_pous()
            .values()
            .filter(|x| x.is_function_block())
            .map(|x| (x.get_name(), Status::Unvisited))
            .collect();

        for nodes in vec![structs, fbs] {
            self.find_cycle(index, nodes);
        }
    }

    /// Finds cycles for the given nodes.
    fn find_cycle<'idx>(&mut self, index: &'idx Index, mut nodes: IndexMap<&'idx str, Status>) {
        let mut path = IndexSet::new();

        // TODO: Runtime of the iterator?
        while let Some(node) = nodes.iter_mut().find(|x| x.1 == &Status::Unvisited) {
            self.dfs(index, node.0, &mut nodes, &mut path);
        }
    }

    /// In DFS manner recursively visits a node and all its child nodes while simultaneously creating a path
    /// of it. Ends either by detecting a cycle, i.e. re-visting a node that is already present in our path,
    /// or by reaching a node with no further child nodes. In the former case the cycle is added to the
    /// diagnostician.
    fn dfs<'idx>(
        &mut self,
        index: &'idx Index,
        curr_node: &'idx str,
        nodes: &mut IndexMap<&'idx str, Status>,
        path: &mut IndexSet<&'idx str>,
    ) {
        nodes[curr_node] = Status::Visited;
        path.insert(curr_node);

        if let Some(edges) = index.get_members(curr_node) {
            for node in edges.values().map(|x| x.get_type_name()).collect::<IndexSet<_>>() {
                if let Some(status) = nodes.get(node) {
                    match status {
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
        // _Convert_ path to vector and append cyclic node to get `A -> B -> C -> B` instead of `A -> B -> C`
        let mut path = path.iter().copied().collect::<Vec<_>>();
        path.push(node);

        match path.iter().position(|x| x == &node) {
            Some(idx) => {
                let path = &path[idx..];
                let ranges = path
                    .iter()
                    .map(|node| index.get_type(node).unwrap().location.source_range.to_owned())
                    .collect();

                self.diagnostics.push(Diagnostic::SyntaxError {
                    message: format!("Found recursive data structure in the form of `{}`", path.join(" -> ")),
                    range: ranges,
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
        format!("Found recursive data structure in the form of `{path}`")
    }

    mod structs {
        use crate::{
            diagnostics::ErrNo, test_utils::tests::parse_and_validate,
            validation::recursive_validator::tests::generate_message,
        };

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

            assert_eq!(diagnostics[1].get_message(), generate_message("B -> C -> E -> F -> B"),);
            assert_eq!(diagnostics[1].get_type(), &ErrNo::pou__recursive_data_structure);
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
        }

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
            for idx in 0..diagnostics.len() {
                assert_eq!(diagnostics[idx].get_message(), generate_message("A -> A"));
            }
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
            assert_eq!(diagnostics[1].get_message(), generate_message("A -> B -> A"));
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
            for idx in 0..diagnostics.len() {
                assert_eq!(diagnostics[idx].get_message(), generate_message("A -> B -> A"));
            }
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
            assert_eq!(diagnostics[1].get_message(), generate_message("C -> E -> C"));
        }
    }

    mod function_blocks {
        use crate::{
            diagnostics::ErrNo, test_utils::tests::parse_and_validate,
            validation::recursive_validator::tests::generate_message,
        };

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
        }

        // TODO: Doesn't currently work
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

            assert_eq!(diagnostics.len(), 1);

            assert_eq!(diagnostics[0].get_message(), generate_message("A -> B -> A"));
            assert_eq!(diagnostics[0].get_type(), &ErrNo::pou__recursive_data_structure);
        }
    }
}
