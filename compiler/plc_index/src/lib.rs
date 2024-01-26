use encoding_rs::Encoding;
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use plc_source::{SourceCode, SourceContainer};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct GlobalContext {
    /// HashMap containing all read, i.e. parsed, sources where the key represents
    /// the relative file path and the value some [`SourceCode`]
    sources: HashMap<&'static str, SourceCode>,

    /// [`IdProvider`] used during the parsing session
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

    /// Inserts a single [`SourceCode`] to the internal source map
    pub fn insert<S>(&mut self, container: &S, encoding: Option<&'static Encoding>) -> Result<(), Diagnostic>
    where
        S: SourceContainer,
    {
        match container.load_source(encoding) {
            Ok(value) => self.sources.insert(container.get_location_str(), value),
            Err(why) => {
                return Err(Diagnostic::new(format!(
                    "Cannot read file '{}': {}'",
                    container.get_location_str(),
                    &why
                ))
                .with_error_code("E002"))
            }
        };

        Ok(())
    }

    /// Inserts multiple [`SourceCode`]s to the internal source map
    pub fn with_source<S>(mut self, sources: &[S], enc: Option<&'static Encoding>) -> Result<Self, Diagnostic>
    where
        S: SourceContainer,
    {
        for source in sources {
            self.insert(source, enc)?;
        }

        Ok(self)
    }

    /// Returns some [`SourceCode`] based on the given key
    pub fn get(&self, key: &str) -> Option<&SourceCode> {
        self.sources.get(key)
    }

    /// Returns a cloned [`IdProvider`]
    pub fn provider(&self) -> IdProvider {
        self.provider.clone()
    }

    // TODO: `impl Into<SourceLocation>` would be nice here, but adding `plc_ast` as a dep in `plc_source` yields a circular dep so not possible right now
    /// Returns a (whitespace) trimmed slice representing the specified location of the source code.
    /// For example if the location represents `ARRAY[1..5]\n\nOF\t  DINT` the slice `ARRAY[1..5] OF DINT` will be returned instead.
    pub fn slice(&self, location: &SourceLocation) -> String {
        let slice = self.slice_raw(location);
        slice.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Returns a slice representing the specified location of the source code.
    /// If the location, i.e. file path, does not exist an empty string will be returned.
    pub fn slice_raw(&self, location: &SourceLocation) -> &str {
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

#[cfg(test)]
mod tests {
    use crate::GlobalContext;
    use plc_source::source_location::SourceLocationFactory;
    use plc_source::SourceCode;

    #[test]
    fn slice() {
        let input = SourceCode::from("ARRAY[1..5]   \n\n\n\nOF  \n\n\n    \t                 DINT");
        let mut ctxt = GlobalContext::new();
        ctxt.insert(&input, None).unwrap();

        let factory = SourceLocationFactory::default();
        let location = factory.create_range(0..input.source.len());

        assert_eq!(ctxt.slice(&location), "ARRAY[1..5] OF DINT");
        assert_eq!(ctxt.slice_raw(&location), "ARRAY[1..5]   \n\n\n\nOF  \n\n\n    \t                 DINT");
    }
}
