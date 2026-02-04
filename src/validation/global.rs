use plc_ast::ast::PouType;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use rustc_hash::FxHashMap;

use crate::{
    index::{symbol::SymbolMap, Index, PouIndexEntry},
    typesystem::{DataTypeInformation, StructSource},
};

use super::Validators;

/// Validator that does not check a dedicated file but rather
/// uses the index to validate the project as a whole.
/// It performs validations including:
///  - naming-conflicts
///  - <tbc>
#[derive(Default, Validators)]
pub struct GlobalValidator {
    diagnostics: Vec<Diagnostic>,
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
        locations: &[&SourceLocation],
        additional_text: Option<&str>,
    ) {
        for v in locations.iter() {
            let others = locations.iter().filter(|it| *it != v).map(|it| (*it).clone()).collect::<Vec<_>>();

            // If the SourceRange of `v` is undefined, we can assume the user choose a name which clashes
            // with an (internal) built-in datatype, hence the undefined location.
            if v.is_builtin_internal() || v.is_undefined() {
                for other in others {
                    self.diagnostics.push(
                        Diagnostic::new(format!(
                            "{name} can not be used as a name because it is a built-in datatype"
                        ))
                        .with_location(other)
                        .with_error_code("E004"),
                    );
                }
            } else {
                let additional_text = additional_text.unwrap_or("Duplicate symbol.");
                self.push_diagnostic(
                    Diagnostic::new(format!("{name}: {additional_text}"))
                        .with_error_code("E004")
                        .with_location(*v)
                        .with_secondary_locations(others),
                );
            }
        }
    }

    /// checks all symbols of the given index for naming conflicts.
    /// all problems will be reported to self.diagnostics
    pub fn validate(&mut self, index: &Index) {
        // everything callable (funks, global FB-instances, programs)
        self.validate_unique_callables(index);

        // everything that can be a type (DTs, FBs)
        self.validate_unique_datatypes(index);

        // globals + PRGs
        self.validate_unique_variables(index);

        // all POUs
        self.validate_unique_pous(index);

        // all interfaces
        self.validate_unique_interfaces(index);
    }

    /// validates following uniqueness-clusters:
    /// - globals + programs
    /// - member-variables
    /// - enums
    fn validate_unique_variables(&mut self, index: &Index) {
        let globals = index.get_globals().values().map(|g| (g.get_name(), &g.source_location));
        let prgs = index
            .get_pous()
            .values()
            .filter(|pou| matches!(pou, PouIndexEntry::Program { .. }))
            .map(|p| (p.get_name(), p.get_location()));

        self.check_uniqueness_of_cluster(globals.chain(prgs), Some("Ambiguous global variable."));

        // Report name conflicts between any member variables in the VAR block
        for ty in index.get_types().values().chain(index.get_pou_types().values()) {
            let mut groups: FxHashMap<&str, Vec<(&str, &SourceLocation)>> = FxHashMap::default();
            let properties = index.get_properties_in_pou(ty.get_name());
            let properties = properties.iter().map(|it| (it.name.as_str(), &it.location));

            for (variable_name, location) in
                ty.get_members().iter().map(|it| (it.get_name(), &it.source_location)).chain(properties)
            {
                let group = groups.entry(variable_name).or_default();
                group.push((variable_name, location));
            }

            for duplicates in groups.values().filter(|it| it.len() > 1) {
                let locations = duplicates.iter().map(|(_, location)| *location).collect::<Vec<_>>();
                self.report_name_conflict(duplicates[0].0, &locations, None);
            }
        }
    }

    ///validates uniqueness of datatypes (types + functionblocks + classes)
    fn validate_unique_datatypes(&mut self, index: &Index) {
        let all_declared_types = index.get_types().values().filter_map(|dt| {
            let name = dt.get_name();
            if !index.get_type_information_or_void(name).is_generic(index) {
                Some((name, &dt.location))
            } else {
                None
            }
        });

        let all_function_blocks = index
            .get_pous()
            .values()
            .filter(|p| p.is_function_block() || p.is_class())
            .map(|p| (p.get_name(), p.get_location()));
        let vec = all_declared_types.collect::<Vec<_>>();
        let all_declared_types = vec.into_iter();
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
            .map(|it| (it.get_name(), &it.source_location));
        let all_prgs = index
            .get_pous()
            .values()
            .filter(|p| {
                matches!(
                    p,
                    PouIndexEntry::Program { .. }
                        | PouIndexEntry::Action { .. }
                        // Properties are already covered somewhere else
                        | PouIndexEntry::Method { property: None, .. }
                )
            })
            .map(|it| (it.get_name(), it.get_location()));

        let all_funcs = index
            .get_pous()
            .values()
            .filter(|p| p.is_function() && !p.is_generic())
            .map(|it| (it.get_name(), it.get_location()));

        self.check_uniqueness_of_cluster(
            all_fb_instances.chain(all_prgs).chain(all_funcs),
            Some("Ambiguous callable symbol."),
        );
    }

    ///validate the uniqueness of POUs (programs, functions, function_blocks, classes)
    fn validate_unique_pous(&mut self, index: &Index) {
        //inner filter
        fn only_toplevel_pous(pou: &&PouIndexEntry) -> bool {
            !pou.is_action() && !pou.is_method() && !pou.is_generic()
        }

        let pou_clusters = index
            .get_pous()
            .entries()
            .filter(|(_, entries_per_name)| entries_per_name.iter().filter(only_toplevel_pous).count() > 1)
            .map(|(name, pous)| {
                (
                    name.as_str(),
                    pous.iter()
                        .filter(|p| only_toplevel_pous(p) && !p.is_generic())
                        .map(|p| p.get_location()),
                )
            });

        for (name, cluster) in pou_clusters {
            self.report_name_conflict(name, &cluster.collect::<Vec<_>>(), None);
        }
    }

    fn validate_unique_interfaces(&mut self, index: &Index) {
        let interfaces = index
            .get_interfaces()
            .values()
            .map(|interface| (interface.get_name(), interface.get_name_location()));

        self.check_uniqueness_of_cluster(interfaces, Some("Ambiguous interface"));
    }

    fn check_uniqueness_of_cluster<'a, T>(&mut self, cluster: T, additional_text: Option<&str>)
    where
        T: Iterator<Item = (&'a str, &'a SourceLocation)>,
    {
        let mut cluster_map: SymbolMap<&str, &SourceLocation> = SymbolMap::default();
        for (name, loc) in cluster {
            cluster_map.insert(name, loc);
        }
        for (name, locations) in cluster_map.entries().filter(|(_, v)| v.len() > 1) {
            self.report_name_conflict(name, locations, additional_text);
        }
    }
}
