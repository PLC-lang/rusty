use std::collections::HashSet;

use crate::{diagnostics::Diagnostic, index::Index};

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

    pub fn validate_unique_symbols(&mut self, index: &Index) {
        let affected_names = index.get_types().elements()
            .chain(index.get_pou_types().elements()).map(|(k, _)| k);

        let mut collisions = HashSet::new();
        let mut unique_names = HashSet::new();
        for name in affected_names {
            if !unique_names.insert(name) {
                collisions.insert(name);
            }
        }

        for collision in collisions {
            let pou_locations = index
                .get_pou_types()
                .get_all(collision)
                .map(|it| it.iter().map(|p| &p.location))
                .into_iter()
                .flatten();

            let type_locations = index
                .get_types()
                .get_all(collision)
                .map(|it| it.iter().map(|d| &d.location))
                .into_iter()
                .flatten();

            let collision_locations = pou_locations.chain(type_locations).map(|it| {
                format!(
                    "{}:{}",
                    it.source_range.get_file_name().unwrap_or("<internal>"),
                    it.line_number
                )
            });
            self.diagnostics.push(Diagnostic::global_name_conflict(
                collision,
                collision_locations.collect::<Vec<_>>(),
            ));
        }
    }
}
