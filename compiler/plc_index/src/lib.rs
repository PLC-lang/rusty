use encoding_rs::Encoding;
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use plc_source::{SourceCode, SourceContainer};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct GlobalContext {
    sources: HashMap<&'static str, SourceCode>,
    provider: IdProvider,
    // TODO: The following would be also nice, to have a cleaner API i.e. instead of working with different structs such
    //       as the index or the diagnostics one could instead ONLY use the `GlobalContext` with methods like
    //       `ctxt.{add,get}_diagnostics(...)` making the code perhaps a bit cleaner / reducing the # of arguments for
    //       some functions / methods?
    //       RefCells may or may not make sense here, because maybe we dont want to pass the GlobalContext as a mutable reference?
    //       -> diagnostics: RefCell<Diagnostics>, (private visibility)
    //       -> index: RefCell<Index>, (private visibility; `get_index(&self) -> &mut Index`?)
}

impl GlobalContext {
    pub fn new() -> Self {
        Self { sources: HashMap::new(), provider: IdProvider::default() }
    }

    pub fn with_source<S>(mut self, sources: &[S], enc: Option<&'static Encoding>) -> Result<Self, Diagnostic>
    where
        S: SourceContainer,
    {
        for source in sources {
            self.insert(source, enc)?;
        }

        Ok(self)
    }

    pub fn insert<S>(&mut self, container: &S, encoding: Option<&'static Encoding>) -> Result<(), Diagnostic>
    where
        S: SourceContainer,
    {
        match container.load_source(encoding) {
            Ok(value) => self.sources.insert(container.get_location_str(), value),
            Err(why) => return Err(Diagnostic::io_read_error(container.get_location_str(), &why)),
        };

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&SourceCode> {
        self.sources.get(key)
    }

    pub fn provider(&self) -> IdProvider {
        self.provider.clone()
    }

    // TODO: `impl Into<SourceLocation>` would be nice here, but adding `plc_ast` as a dep in `plc_source` yields a circular dep so not possible right now
    pub fn slice(&self, location: &SourceLocation) -> &str {
        let path = location.get_file_name().unwrap_or("<internal>");

        let Some(code) = self.sources.get(path) else { return "" };
        let Some(span) = location.get_span().to_range() else { return "" };

        &code.source[span]
    }

    // // TODO: Importing `plc_project` would make life easier here and allow for the code below, but we get a circular dep
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
}
