use indexmap::IndexSet;

use crate::{
    diagnostics::Diagnostic,
    index::{Index, VariableIndexEntry},
    typesystem::DataTypeInformationProvider,
};

use super::Validators;

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
#[derive(Default)]
pub struct RecursiveValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl Validators for RecursiveValidator {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
}

impl RecursiveValidator {
    pub fn new() -> RecursiveValidator {
        RecursiveValidator { diagnostics: Vec::new() }
    }

    /// Entry point of finding and reporting all recursive data structures.
    pub fn validate_recursion(&mut self, index: &Index) {
        let mut nodes_all: IndexSet<&str> = IndexSet::new();
        let mut nodes_visited = IndexSet::new();

        // Structs (includes arrays defined in structs)
        nodes_all.extend(
            index.get_types().values().filter(|x| x.get_type_information().is_struct()).map(|x| x.get_name()),
        );

        // Function Blocks
        nodes_all.extend(index.get_pous().values().filter(|x| x.is_function_block()).map(|x| x.get_name()));

        self.find_cycle(index, nodes_all, &mut nodes_visited);
    }

    /// Finds cycles for the given nodes.
    fn find_cycle<'idx>(
        &mut self,
        index: &'idx Index,
        nodes_all: IndexSet<&'idx str>,
        nodes_visited: &mut IndexSet<&'idx str>,
    ) {
        let mut path = IndexSet::new();

        for node in &nodes_all {
            if !nodes_visited.contains(node) {
                self.dfs(index, &mut path, node, nodes_visited);
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
        path: &mut IndexSet<&'idx str>,
        node_curr: &'idx str,
        nodes_visited: &mut IndexSet<&'idx str>,
    ) {
        nodes_visited.insert(node_curr);
        path.insert(node_curr);

        for node in index
            .get_container_members(node_curr).iter()
            .map(|x| self.get_type_name(index, x))
            .collect::<IndexSet<_>>()
        {
            if path.contains(node) {
                self.report(index, node, path);
            } else if !nodes_visited.contains(node) {
                self.dfs(index, path, node, nodes_visited);
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

    /// Returns the type name of `entry` distinguishing between two cases:
    /// 1. If the entry is any type but an array its datatype is returned (as usual)
    /// 2. If the entry is an array their inner type name is returned. For example calling the
    /// [`index::VariableIndexEntry::get_type_name`] method on the following code snippet
    /// ```ST
    /// TYPE A : STRUCT
    ///     b : ARRAY[0..1] OF B;
    /// END_STRUCT END_TYPE
    /// ```
    /// would return `__A_b` as the datatype for `b` whereas it should have been `B`, i.e. their inner type
    /// name. This is important for the `dfs` method as it otherwise wouldn't correctly recognize cycles since
    /// it operate on these "normalized" type names.
    #[inline(always)]
    fn get_type_name<'idx>(&self, index: &'idx Index, entry: &'idx VariableIndexEntry) -> &'idx str {
        let name = entry.get_type_name();
        let info = index.get_type_information_or_void(name);

        if info.is_array() {
            return info.get_inner_array_type_name().unwrap_or(name); // the `unwrap_or` _should_ never trigger
        }

        name
    }
}
