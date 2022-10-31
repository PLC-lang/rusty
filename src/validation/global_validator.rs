use crate::{
    ast::SourceRange,
    diagnostics::Diagnostic,
    index::{symbol::SymbolMap, Index},
};

/// Validator that does not check a dedicated file but rather
/// uses the index to validate the project as a whole.
/// It performs validations including:
///  - naming-conflicts
///  - <tbc>
pub struct GlobalValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl GlobalValidator {
    pub fn new() -> GlobalValidator {
        GlobalValidator {
            diagnostics: Vec::new(),
        }
    }

    /// reports a name-conflict for the given name. the locations indicate the
    /// locations of the declared symbols that make up the conflict. this method will
    /// create a diagnostic per location where it attaches the other locations as additional information.
    fn report_name_conflict(&mut self, name: &str, locations: &[&SourceRange]) {
        for (idx, v) in locations.iter().enumerate() {
            let others = locations
                .iter()
                .enumerate()
                .filter(|(j, _)| idx != (*j))
                .map(|(_, it)| (*it).clone())
                .collect::<Vec<_>>();

            self.diagnostics
                .push(Diagnostic::global_name_conflict(name, (*v).clone(), others));
        }
    }

    /// checks all symbols of the given index for naming conflicts.
    /// all problems will be reported to self.diagnostics
    pub fn validate_unique_symbols(&mut self, index: &Index) {
        // 1) check uniqueness of POUs and DataTypes
        // collect all POUs and DataTypes into a SymbolMap
        let mut duplicates: SymbolMap<&str, &SourceRange> = SymbolMap::default();
        for (name, dt) in index.get_pou_types().elements() {
            duplicates.insert(name.as_str(), &dt.location.source_range);
        }
        for (name, dt) in index.get_types().elements() {
            duplicates.insert(name.as_str(), &dt.location.source_range);
        }
        // every key with more than 1 location associated is a problem
        for (name, locations) in duplicates.entries().filter(|(_, v)| v.len() > 1) {
            self.report_name_conflict(*name, locations);
        }

        // 2) check uniqueness of global variables
        // every entry in index.get_globals with more than 1 association indicates a problem
        let duplicate_variables = index
            .get_globals()
            .entries()
            .filter(|(_, variables)| variables.len() > 1)
            .map(|(name, variables)| {
                (
                    name,
                    variables
                        .iter()
                        .map(|vie| &vie.source_location.source_range),
                )
            });

        for (name, locations) in duplicate_variables {
            self.report_name_conflict(name, &locations.collect::<Vec<_>>());
        }

        // 3) check uniqueness of member_variables
        // all index.member_variables with more than 1 associations indicate a problem
        let duplication_members = index
            .get_all_members_by_container()
            .values()
            .flat_map(|it| it.entries())
            .filter(|(_, vars)| vars.len() > 1);
        for (name, variables) in duplication_members {
            let full_name = variables
                .get(0)
                .map(|it| it.get_qualified_name())
                .unwrap_or(name.as_str());
            self.report_name_conflict(
                full_name,
                &variables
                    .iter()
                    .map(|v| &v.source_location.source_range)
                    .collect::<Vec<_>>(),
            )
        }

        // 4) check enum elements
        // all index.global_qualified_enums with more than 1 association indicates a problem
        for (name, variables) in index
            .get_global_qualified_enums()
            .entries()
            .filter(|(_, elements)| elements.len() > 1)
        {
            self.report_name_conflict(
                name,
                &variables
                    .iter()
                    .map(|v| &v.source_location.source_range)
                    .collect::<Vec<_>>(),
            )
        }
    }
}
