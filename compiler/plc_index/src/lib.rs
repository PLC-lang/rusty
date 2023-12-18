use encoding_rs::Encoding;
use plc_ast::provider::IdProvider;
// use plc_project::project::{LibraryInformation, Project};
use plc_source::{SourceCode, SourceContainer};
use std::collections::HashMap;

pub struct GlobalContext {
    sources: HashMap<&'static str, SourceCode>,
    provider: IdProvider,
    // TODO: The following would be also nice, to have a cleaner API i.e. instead of working directly on a diagnostic one
    //       could use `ctxt.add_diagnostic(...)` allowing us to ONLY work with the `GlobalContext` struct
    //       -> index: Index,
    //       -> diagnostics: RefCell<Diagnostics>, -> should be private, `{add,get}_diagnostic`
}

impl GlobalContext {
    pub fn new() -> Self {
        Self { sources: HashMap::new(), provider: IdProvider::default() }
    }

    // // TODO: Importing `plc_project` would make life easier here, but we get a circular dependency if imported; try to fix it?
    // pub fn project<S: SourceContainer>(project: &Project<S>, encoding: Option<&'static Encoding>) -> Self {
    //     let mut ctxt = Self::new();
    //
    //     for source in project.get_sources() {
    //         ctxt.insert(source, encoding);
    //     }
    //
    //     for source in project.get_includes() {
    //         ctxt.insert(source, encoding);
    //     }
    //
    //     for source in project.get_libraries().iter().flat_map(LibraryInformation::get_includes) {
    //         ctxt.insert(source, encoding);
    //     }
    //
    //     ctxt
    // }

    pub fn sources<S: SourceContainer>(
        mut self,
        container: &[S],
        encoding: Option<&'static Encoding>,
    ) -> Self {
        for source in container {
            self.insert(source, encoding);
        }

        self
    }

    fn insert(&mut self, container: &impl SourceContainer, encoding: Option<&'static Encoding>) {
        let key = container.get_location_str();
        let value = container.load_source(encoding).unwrap();

        self.sources.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&SourceCode> {
        self.sources.get(key)
    }

    pub fn provider(&self) -> IdProvider {
        self.provider.clone()
    }
}
