use crate::{
    ast::SourceRange,
    diagnostics::Diagnostic,
    index::{Index, SymbolMap},
};

/// validator for the whole project using the index
pub struct GlobalValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl GlobalValidator {
    pub fn new() -> GlobalValidator {
        GlobalValidator {
            diagnostics: Vec::new(),
        }
    }

    fn report(&mut self, name: &str, locations: &Vec<&SourceRange>) {
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

    pub fn validate_unique_symbols(&mut self, index: &Index) {
        // check uniqueness of POUs and DataTypes
        let mut duplicates: SymbolMap<&str, &SourceRange> = SymbolMap::default();
        for (name, dt) in index.get_pou_types().elements() {
            duplicates.insert(name.as_str(), &dt.location.source_range);
        }
        for (name, dt) in index.get_types().elements() {
            duplicates.insert(name.as_str(), &dt.location.source_range);
        }

        for (name, locations) in duplicates.entries().filter(|(_, v)| v.len() > 1) {
            self.report(*name, locations);
        }

        // check uniqueness of global variables
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
            self.report(name, &locations.collect());
        }

        // check uniqueness of member_variables
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
            self.report(
                full_name,
                &variables
                    .iter()
                    .map(|v| &v.source_location.source_range)
                    .collect(),
            )
        }

        // check enum elements
        for (name, variables) in index
            .get_global_qualified_enums()
            .entries()
            .filter(|(_, elements)| elements.len() > 1)
        {
            self.report(
                name,
                &variables
                    .iter()
                    .map(|v| &v.source_location.source_range)
                    .collect(),
            )
        }
    }
}
