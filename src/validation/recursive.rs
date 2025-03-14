use std::hash::Hash;
use std::marker::PhantomData;

use itertools::Itertools;

use plc_ast::ast::PouType;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::index::{FxIndexSet, InterfaceIndexEntry};
use crate::{
    index::{Index, VariableIndexEntry},
    typesystem::{DataType, DataTypeInformation, DataTypeInformationProvider, StructSource},
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
#[derive(Default, Validators)]
pub struct RecursiveValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl RecursiveValidator {
    pub fn new() -> RecursiveValidator {
        RecursiveValidator { diagnostics: Vec::new() }
    }

    /// Entry point of finding and reporting all recursive data structures.
    pub fn validate(&mut self, index: &Index) {
        let mut detective = CycleInvestigator::<DataType>::new();
        self.diagnostics.extend(detective.investigate(index));

        let mut detective = CycleInvestigator::<InterfaceIndexEntry>::new();
        self.diagnostics.extend(detective.investigate(index));
    }
}

struct CycleInvestigator<T> {
    diagnostics: Vec<Diagnostic>,
    _marker: PhantomData<T>,
}

impl<T> CycleInvestigator<T> {
    fn new() -> Self {
        Self { diagnostics: vec![], _marker: PhantomData }
    }

    /// Returns the data type of `entry` distinguishing between two cases:
    /// 1. If the entry is any type but an array its datatype is returned (as usual)
    /// 2. If the entry is an array their inner type name is returned. For example calling the
    ///    [`index::VariableIndexEntry::get_type_name`] method on the following code snippet
    /// ```ST
    /// TYPE A : STRUCT
    ///     b : ARRAY[0..1] OF B;
    /// END_STRUCT END_TYPE
    /// ```
    /// would return `__A_b` as the datatype for `b` whereas it should have been `B`, i.e. their inner type
    /// name. This is important for the `dfs` method as it otherwise wouldn't correctly recognize cycles since
    /// it operate on these "normalized" type names.
    #[inline(always)]
    fn get_type<'idx>(&self, index: &'idx Index, entry: &'idx VariableIndexEntry) -> &'idx DataType {
        let name = entry.get_type_name();
        let dt = index.get_effective_type_or_void_by_name(name);

        if dt.is_array() {
            dt.get_type_information()
                .get_inner_array_type_name()
                .map(|it| index.get_effective_type_or_void_by_name(it))
                .unwrap_or(dt)
        } else {
            dt
        }
    }
}

trait CycleDetector {
    type Item;

    /// Entry point for the cycle detection. Returns a diagnostic for each cycle detected during investigation.
    fn investigate(&mut self, index: &Index) -> Vec<Diagnostic>;

    /// Finds cycles for the given nodes.
    fn find_cycle<'idx>(
        &mut self,
        index: &'idx Index,
        nodes_all: &FxIndexSet<&'idx Self::Item>,
        nodes_visited: &mut FxIndexSet<&'idx Self::Item>,
    ) where
        Self::Item: Hash + Eq,
    {
        let mut path = FxIndexSet::default();

        for node in nodes_all {
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
        path: &mut FxIndexSet<&'idx Self::Item>,
        node_curr: &'idx Self::Item,
        nodes_visited: &mut FxIndexSet<&'idx Self::Item>,
    );

    /// Generates and reports the minimal path of a cycle. Specifically `path` contains all nodes visited up
    /// until a cycle, e.g. `A -> B -> C -> B`. We are only interested in `B -> C -> B` as such this method
    /// finds the first occurence of `B` to create a vector slice of `B -> C -> B` for the diagnostician.
    fn report<'idx>(&mut self, node: &'idx Self::Item, path: &mut FxIndexSet<&'idx Self::Item>) -> Diagnostic
    where
        Self::Item: DataProvider,
        Self::Item: Hash + Eq,
    {
        let idx = path.get_index_of(node).expect("Node has to be in the IndexSet");

        let mut slice = path.iter().skip(idx).copied().collect::<Vec<_>>();
        let ranges = slice.iter().map(|node| node.get_location()).collect::<Vec<_>>();

        slice.push(node); // Append to get `B -> C -> B` instead of `B -> C` in the report
        let error = slice.iter().map(|it| it.get_name()).join(" -> ");
        let diagnostic =
            Diagnostic::new(format!("Recursive {} `{}` has infinite size", node.get_category_name(), error))
                .with_error_code("E029");

        let diagnostic =
            if let Some(first) = ranges.first() { diagnostic.with_location(first) } else { diagnostic };

        let diagnostic = if ranges.len() > 1 {
            ranges.iter().fold(diagnostic, |prev, it| prev.with_secondary_location(it))
        } else {
            diagnostic
        };

        diagnostic
    }
}

impl CycleDetector for CycleInvestigator<DataType> {
    type Item = DataType;

    fn investigate(&mut self, index: &Index) -> Vec<Diagnostic> {
        let mut nodes_all = FxIndexSet::default();
        let mut nodes_visited = FxIndexSet::default();

        // Structs (includes arrays defined in structs)
        nodes_all.extend(index.get_types().values().filter(|x| x.get_type_information().is_struct()));

        // Function Blocks
        nodes_all.extend(index.get_pou_types().values().filter(|x| {
            matches!(
                x.get_type_information(),
                DataTypeInformation::Struct { source: StructSource::Pou(PouType::FunctionBlock), .. }
            )
        }));

        self.find_cycle(index, &nodes_all, &mut nodes_visited);
        std::mem::take(&mut self.diagnostics)
    }

    fn dfs<'idx>(
        &mut self,
        index: &'idx Index,
        path: &mut FxIndexSet<&'idx Self::Item>,
        node_curr: &'idx Self::Item,
        nodes_visited: &mut FxIndexSet<&'idx Self::Item>,
    ) {
        nodes_visited.insert(node_curr);
        path.insert(node_curr);

        for node in
            node_curr.get_struct_members().iter().map(|x| self.get_type(index, x)).collect::<FxIndexSet<_>>()
        {
            if path.contains(node) {
                let diagnostic = self.report(node, path);
                self.diagnostics.push(diagnostic);
            } else if !nodes_visited.contains(node) {
                self.dfs(index, path, node, nodes_visited);
            }
        }
        path.pop();
    }
}

impl CycleDetector for CycleInvestigator<InterfaceIndexEntry> {
    type Item = InterfaceIndexEntry;

    fn investigate(&mut self, index: &Index) -> Vec<Diagnostic> {
        let mut nodes_all = FxIndexSet::default();
        let mut nodes_visited = FxIndexSet::default();

        nodes_all.extend(index.get_interfaces().values());

        self.find_cycle(index, &nodes_all, &mut nodes_visited);
        std::mem::take(&mut self.diagnostics)
    }

    fn dfs<'idx>(
        &mut self,
        index: &'idx Index,
        path: &mut FxIndexSet<&'idx Self::Item>,
        node_curr: &'idx Self::Item,
        nodes_visited: &mut FxIndexSet<&'idx Self::Item>,
    ) {
        nodes_visited.insert(node_curr);
        path.insert(node_curr);

        for node in node_curr
            .get_extensions()
            .iter()
            .filter_map(|id| index.find_interface(&id.name))
            .collect::<FxIndexSet<_>>()
        {
            if path.contains(node) {
                let diagnostic = self.report(node, path);
                self.diagnostics.push(diagnostic);
            } else if !nodes_visited.contains(node) {
                self.dfs(index, path, node, nodes_visited);
            }
        }
        path.pop();
    }
}

trait DataProvider {
    fn get_location(&self) -> SourceLocation;
    fn get_name(&self) -> &str;
    fn get_category_name(&self) -> &str;
}

impl DataProvider for DataType {
    fn get_location(&self) -> SourceLocation {
        self.location.to_owned()
    }

    fn get_name(&self) -> &str {
        self.get_name()
    }

    fn get_category_name(&self) -> &str {
        "data structure"
    }
}

impl DataProvider for InterfaceIndexEntry {
    fn get_location(&self) -> SourceLocation {
        self.get_name_location().to_owned()
    }

    fn get_name(&self) -> &str {
        self.get_name()
    }

    fn get_category_name(&self) -> &str {
        "inheritance"
    }
}
