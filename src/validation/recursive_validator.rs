use std::cell::Cell;

use indexmap::{IndexMap, IndexSet};

use crate::{
    diagnostics::Diagnostic,
    index::{Index, VariableIndexEntry},
    typesystem::DataTypeInformationProvider,
};

/// Status of whether a node has been visited or not.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Status {
    Visited,
    Unvisited,
}

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

impl RecursiveValidator {
    pub fn new() -> RecursiveValidator {
        RecursiveValidator { diagnostics: Vec::new() }
    }

    /// Entry point of finding and reporting all recursive data structures.
    pub fn validate_recursion(&mut self, index: &Index) {
        let mut nodes: IndexMap<&str, Cell<Status>> = IndexMap::new();

        // Structs
        nodes.extend(
            index
                .get_types()
                .values()
                .filter(|x| x.get_type_information().is_struct())
                .map(|x| (x.get_name(), Cell::new(Status::Unvisited))),
        );

        // Function Blocks
        nodes.extend(
            index
                .get_pous()
                .values()
                .filter(|x| x.is_function_block())
                .map(|x| (x.get_name(), Cell::new(Status::Unvisited))),
        );

        self.find_cycle(index, nodes);
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
    /// or by reaching a node with no further child nodes. In both cases the function goes back one recursion
    /// depth repeating itself for the remaining child nodes until all nodes have been visited. All detected
    /// cycles are added to the diagnostician.
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
            for node in edges.values().map(|x| self.get_type_name(index, x)).collect::<IndexSet<_>>() {
                if let Some(status) = nodes.get(node) {
                    // Check if we would enter a cycle and otherwise ONLY
                    // visit the next node if we haven't already visited it.
                    if path.contains(node) {
                        self.report(index, node, path);
                    } else if status.get() == Status::Unvisited {
                        self.dfs(index, node, nodes, path);
                    }
                }
            }
        }

        path.pop();
    }

    /// Generates and reports the minimal path of a cycle. Specifically `path` contains all nodes visited up
    /// until a cycle, e.g. `A -> B -> C -> B`. We are only interested in `B -> C -> B` as such this method
    /// finds the first occurence of `B` to create a vector slice of `B -> C -> B` for the diagnostician.
    fn report<'idx>(&mut self, index: &'idx Index, node: &'idx str, path: &mut IndexSet<&'idx str>) {
        match path.get_index_of(node) {
            Some(idx) => {
                let mut slice = path.iter().skip(idx).copied().collect::<Vec<_>>();
                let ranges = slice
                    .iter()
                    .map(|node| index.get_type(node).unwrap().location.source_range.to_owned())
                    .collect();

                slice.push(node); // Append to get `B -> C -> B` instead of `B -> C` in the report
                self.diagnostics.push(Diagnostic::recursive_datastructure(&slice.join(" -> "), ranges));
            }

            None => unreachable!("Node has to be in the IndexSet"),
        }
    }

    /// Returns the type name of `entry` distinguishing between two cases,
    /// 1. If the entry is any type but array its data-type name is returned
    /// 2. If the entry is an arrary, its inner type name is returned because their data-type name would
    /// be `A.b` for an array named `b` inside a struct named `A` which the `dfs` method would not correctly
    /// recognize as a cycle.
    #[inline(always)]
    fn get_type_name<'idx>(&self, index: &'idx Index, entry: &'idx VariableIndexEntry) -> &'idx str {
        dbg!(&entry);
        let type_name = entry.get_type_name();
        match index.get_type_information_or_void(type_name).get_inner_array_type_name() {
            Some(inner_type_name) => inner_type_name,
            None => type_name,
        }
    }
}
