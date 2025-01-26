use std::{ops::Deref, sync::RwLock};

use ast::ast::Property;
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};

use super::{participant::PipelineParticipantMut, ParsedProject};

pub struct ParticipantValidator {
    diagnostics: Vec<Diagnostic>,
}

impl ParticipantValidator {
    // TODO: Temporary solution with that diagnostician, ideally the diagnostician lazy reads source files and
    // doesn't rely on register_file
    pub fn new() -> ParticipantValidator {
        ParticipantValidator { diagnostics: Vec::new() }
    }

    pub fn validate_properties(&mut self, properties: &Vec<Property>) {
        // todo!()
    }
}

impl PipelineParticipantMut for ParticipantValidator {
    fn pre_index(&mut self, project: ParsedProject) -> ParsedProject {
        let ParsedProject { units } = project;

        for unit in &units {
            self.validate_properties(&unit.properties);
        }

        let mut diagnostician = Diagnostician::clang_format_diagnostician();
        diagnostician.handle(&self.diagnostics);

        ParsedProject { units }
    }
}

// -------

pub struct ParticipantDiagnostician {
    inner: RwLock<Diagnostician>,
}

unsafe impl Sync for ParticipantDiagnostician {}
unsafe impl Send for ParticipantDiagnostician {}

impl ParticipantDiagnostician {
    pub fn new() -> ParticipantDiagnostician {
        ParticipantDiagnostician { inner: RwLock::new(Diagnostician::default()) }
    }
}

impl Deref for ParticipantDiagnostician {
    type Target = RwLock<Diagnostician>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
