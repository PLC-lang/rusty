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
        let affected_names = index
            .get_types()
            .elements()
            .chain(index.get_pou_types().elements())
            .map(|(k, _)| k);

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

            let collision_locations = pou_locations.chain(type_locations).collect::<Vec<_>>();
            //create an issue on every conflict that points to the other occurences
            for (idx, cl) in collision_locations.iter().enumerate() {
                let others = collision_locations
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| idx != (*j))
                    .map(|(_, it)| it.source_range.clone())
                    .collect::<Vec<_>>();
                self.diagnostics.push(Diagnostic::global_name_conflict(
                    collision,
                    cl.source_range.clone(),
                    others,
                ));
            }
        }
    }
}
