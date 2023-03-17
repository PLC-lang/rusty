use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    ast::{PouType, SourceRange},
    diagnostics::Diagnostic,
    index::{symbol::SymbolMap, Index, PouIndexEntry},
    typesystem::{self, DataTypeInformation, StructSource},
};

use super::Validators;

/// Validator that does not check a dedicated file but rather
/// uses the index to validate the project as a whole.
/// It performs validations including:
///  - naming-conflicts
///  - <tbc>
#[derive(Default)]
pub struct GlobalValidator {
    diagnostics: Vec<Diagnostic>,
}

impl Validators for GlobalValidator {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
}

impl GlobalValidator {
    pub fn new() -> GlobalValidator {
        GlobalValidator { diagnostics: Vec::new() }
    }

    /// reports a name-conflict for the given name. the locations indicate the
    /// locations of the declared symbols that make up the conflict. this method will
    /// create a diagnostic per location where it attaches the other locations as additional information.
    fn report_name_conflict(
        &mut self,
        name: &str,
        locations: &[&SourceRange],
        additional_text: Option<&str>,
    ) {
        for (idx, v) in locations.iter().enumerate() {
            let others = locations
                .iter()
                .enumerate()
                .filter(|(j, _)| idx != (*j))
                .map(|(_, it)| (*it).clone())
                .collect::<Vec<_>>();

            if let Some(additional_text) = additional_text {
                self.push_diagnostic(Diagnostic::global_name_conflict_with_text(
                    name,
                    (*v).clone(),
                    others,
                    additional_text,
                ));
            } else {
                self.push_diagnostic(Diagnostic::global_name_conflict(name, (*v).clone(), others));
            }
        }
    }

    pub fn validate(&mut self, index: &Index) {
        self.validate_unique_symbols(index);
        self.validate_user_defined_names(index);
    }

    /// checks all symbols of the given index for naming conflicts.
    /// all problems will be reported to self.diagnostics
    fn validate_unique_symbols(&mut self, index: &Index) {
        // everything callable (funks, global FB-instances, programs)
        self.validate_unique_callables(index);

        // everything that can be a type (DTs, FBs)
        self.validate_unique_datatypes(index);

        // globals + PRGs
        self.validate_unique_variables(index);

        // all POUs
        self.validate_unique_pous(index);
    }

    /// Checks if user-defined datatype names clash with built-in ones. For example `TYPE DINT : ... END_TYPE`
    /// is invalid because `DINT` is an built-in datatype and as such can't be used as an alias.
    fn validate_user_defined_names(&mut self, index: &Index) {
        let types = index.get_types().values().filter(|t| !t.is_internal());
        let builtin = typesystem::get_builtin_types().into_iter().map(|t| t.name).collect::<HashSet<_>>();

        types.filter(|t| builtin.get(&t.name).is_some()).for_each(|t| {
            self.diagnostics.push(Diagnostic::invalid_type_name(&t.name, t.location.source_range.clone()))
        });
    }

    /// validates following uniqueness-clusters:
    /// - globals + programs
    /// - member-variables
    /// - enums
    fn validate_unique_variables(&mut self, index: &Index) {
        let globals = index.get_globals().values().map(|g| (g.get_name(), &g.source_location.source_range));
        let prgs = index
            .get_pous()
            .values()
            .filter(|pou| matches!(pou, PouIndexEntry::Program { .. }))
            .map(|p| (p.get_name(), &p.get_location().source_range));

        self.check_uniqueness_of_cluster(globals.chain(prgs), Some("Ambiguous global variable."));

        for ty in index.get_types().values().chain(index.get_pou_types().values()) {
            let members = ty.get_members().iter().sorted_by_key(|it| it.get_qualified_name().to_lowercase());
            for (_, mut vars) in &members.group_by(|it| it.get_qualified_name().to_lowercase()) {
                if let Some(first) = vars.next() {
                    if let Some(second) = vars.next() {
                        //Collect remaining
                        let mut locations: Vec<_> = vars.map(|it| &it.source_location.source_range).collect();
                        locations.push(&first.source_location.source_range);
                        locations.push(&second.source_location.source_range);
                        self.report_name_conflict(first.get_qualified_name(), locations.as_slice(), None)
                    }
                }
            }
        }
        //check enums
        let duplication_enums = index
            .get_global_qualified_enums()
            .entries()
            .filter(|(_, vars)| vars.len() > 1)
            .map(|(_, variables)| {
                (variables[0].get_qualified_name(), variables.iter().map(|v| &v.source_location.source_range))
            });

        for (name, locations) in duplication_enums {
            self.report_name_conflict(name, &locations.collect::<Vec<_>>(), None);
        }
    }

    ///validates uniqueness of datatypes (types + functionblocks + classes)
    fn validate_unique_datatypes(&mut self, index: &Index) {
        let all_declared_types =
            index.get_types().values().map(|dt| (dt.get_name(), &dt.location.source_range));
        let all_function_blocks = index
            .get_pous()
            .values()
            .filter(|p| p.is_function_block() || p.is_class())
            .map(|p| (p.get_name(), &p.get_location().source_range));
        self.check_uniqueness_of_cluster(
            all_declared_types.chain(all_function_blocks),
            Some("Ambiguous datatype."),
        );
    }

    /// validates the uniqueness of everything callable (global fb-instances + programs + functions )
    fn validate_unique_callables(&mut self, index: &Index) {
        let all_fb_instances = index
            .get_globals()
            .values()
            .filter(|g| {
                index
                    .find_effective_type_by_name(g.get_type_name())
                    .map(|it| {
                        matches!(
                            it.information,
                            DataTypeInformation::Struct {
                                source: StructSource::Pou(PouType::FunctionBlock),
                                ..
                            }
                        )
                    })
                    .unwrap_or(false)
            })
            .map(|it| (it.get_name(), &it.source_location.source_range));
        let all_prgs_and_funcs = index
            .get_pous()
            .values()
            .filter(|p| {
                matches!(
                    p,
                    PouIndexEntry::Program { .. }
                        | PouIndexEntry::Function { .. }
                        | PouIndexEntry::Method { .. }
                        | PouIndexEntry::Action { .. }
                )
            })
            .map(|it| (it.get_name(), &it.get_location().source_range));

        self.check_uniqueness_of_cluster(
            all_fb_instances.chain(all_prgs_and_funcs),
            Some("Ambiguous callable symbol."),
        );
    }

    ///validate the uniqueness of POUs (programs, functions, function_blocks, classes)

    fn validate_unique_pous(&mut self, index: &Index) {
        //inner filter
        fn only_toplevel_pous(pou: &&PouIndexEntry) -> bool {
            !pou.is_action() && !pou.is_method()
        }

        let pou_clusters = index
            .get_pous()
            .entries()
            .filter(|(_, entries_per_name)| entries_per_name.iter().filter(only_toplevel_pous).count() > 1)
            .map(|(name, pous)| {
                (
                    name.as_str(),
                    pous.iter().filter(only_toplevel_pous).map(|p| &p.get_location().source_range),
                )
            });

        for (name, cluster) in pou_clusters {
            self.report_name_conflict(name, &cluster.collect::<Vec<_>>(), None);
        }
    }

    fn check_uniqueness_of_cluster<'a, T>(&mut self, cluster: T, additional_text: Option<&str>)
    where
        T: Iterator<Item = (&'a str, &'a SourceRange)>,
    {
        let mut cluster_map: SymbolMap<&str, &SourceRange> = SymbolMap::default();
        for (name, loc) in cluster {
            cluster_map.insert(name, loc);
        }
        for (name, locations) in cluster_map.entries().filter(|(_, v)| v.len() > 1) {
            self.report_name_conflict(name, locations, additional_text);
        }
    }
}
