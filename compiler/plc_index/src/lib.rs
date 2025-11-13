use annotate_snippets::Renderer;
use encoding_rs::Encoding;
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use plc_source::{SourceCode, SourceContainer};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

// TODO: The clone is super expensive here, eventually we should have a inner type wrapped around Arc, e.g.
//       `struct GlobalContext { inner: Arc<GlobalContextInner> }`
// TODO: The following would be also nice, to have a cleaner API i.e. instead of working with different structs such
//       as the index or the diagnostics one could instead ONLY use the `GlobalContext` with methods like
//       `ctxt.{add,get}_diagnostics(...)` making the code perhaps a bit cleaner / reducing the # of arguments for
//       some functions / methods?
//       RefCells may or may not make sense here, because maybe we dont want to pass the GlobalContext as a mutable reference?
//       -> diagnostics: RefCell<Diagnostics>, (private visibility)
//       -> index: RefCell<Index>, (private visibility; `get_index(&self) -> &mut Index`?)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct GlobalContext {
    /// HashMap containing all read, i.e. parsed, sources where the key represents
    /// the relative file path and the value some [`SourceCode`]
    sources: FxHashMap<&'static str, SourceCode>,

    /// [`IdProvider`] used during the parsing session
    provider: IdProvider,
    error_fmt: ErrorFormat,
}

// XXX: Temporary
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorFormat {
    #[default]
    Rich,
    Clang,
    Null,
}

impl GlobalContext {
    pub fn new() -> GlobalContext {
        GlobalContext::default()
    }

    pub fn with_error_fmt(&mut self, error_fmt: ErrorFormat) {
        self.error_fmt = error_fmt;
    }

    // TODO: Remove Diagnostic dependency here, instead return a plc_index specific error enum
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

    // TODO(volsa): Transfer `get_datatype_name_or_slice(...)` here once crates / workspaces have been finalized

    /// Returns a slice representing the specified location of the source code.
    /// If the location, i.e. file path, does not exist an empty string will be returned.
    pub fn slice_raw(&self, location: &SourceLocation) -> &str {
        let path = location.get_file_name().unwrap_or("<internal>");

        let Some(code) = self.sources.get(path) else { return "" };
        let Some(span) = location.get_span().to_range() else { return "" };

        &code.source[span]
    }

    // TODO: This is a proof-of-concept version how we could use the GlobalContext to handle
    // diagnostics across the whole compiler pipeline. This will currently **only** display the diagnostics
    // defined in the `ParticipantValidator`. Ideally we can use this to handle all diagnostics in the future.
    // This method currently only mimics the `CodeSpanDiagnosticReporter` and its codespan style
    // `codespan_reporting::term::DisplayStyle::Rich`, meaning other reporters such as the `ClangReporter` are
    // not working as of now.
    //
    // TODO: Maybe we can make this async by creating using a `mpsc` channel for receiving diagnostics and
    //       in a final handle call loop over the queue with `while channel.recv() { ... }`
    fn handle_inner(&self, diagnostic: &Diagnostic) -> String {
        // NOTE: We need to properly error handle these unwraps, for now they should never panic because we
        //       (assume) we always have a valid file name and location
        let location = diagnostic.get_location();
        let name = location.get_file_name().unwrap_or("<internal>");
        let Some(code) = &self.sources.get(name) else { return String::new() };
        let Some(location) = &diagnostic.get_location().get_span().to_range() else { return String::new() };
        let secondary_locations = diagnostic
            .get_secondary_locations()
            .unwrap_or_default()
            .iter()
            .flat_map(|it| it.get_span().to_range())
            .map(|it| annotate_snippets::Level::Error.span(it).label("see also"))
            .collect::<Vec<_>>();

        let message = annotate_snippets::Level::Error.title(diagnostic.get_message()).snippet(
            annotate_snippets::Snippet::source(&code.source)
                .line_start(diagnostic.get_location().get_line())
                .origin(name)
                .fold(true)
                .annotation(
                    annotate_snippets::Level::Error.span(location.clone()).label(diagnostic.get_message()),
                )
                .annotations(secondary_locations),
        );

        // XXX: Temporary
        match self.error_fmt {
            ErrorFormat::Clang => self.clang_format(diagnostic),
            ErrorFormat::Rich => {
                let renderer = annotate_snippets::Renderer::styled();
                let res = renderer.render(message);
                res.to_string()
            }
            ErrorFormat::Null => {
                let renderer = annotate_snippets::Renderer::plain();
                let res = renderer.render(message);
                res.to_string()
            }
        }
    }

    pub fn handle(&self, diagnostic: &Diagnostic) {
        eprintln!("{}", self.handle_inner(diagnostic))
    }

    pub fn handle_as_str(&mut self, diagnostic: &Diagnostic) -> String {
        self.handle_inner(diagnostic)
    }

    // XXX: Temporary solution, at some point we should use the annotation-snippet Renderer trait to properly
    //      format the diagnostics
    fn clang_format(&self, diagnostic: &Diagnostic) -> String {
        let span = diagnostic.get_location().get_span().to_range().unwrap();
        if span.start == span.end {
            format!(
                "{fname}:{line}:{column}: error[{code}]: {message}",
                fname = diagnostic.get_location().get_file_name().unwrap(),
                line = diagnostic.get_location().get_line(),
                column = diagnostic.get_location().get_column(),
                code = diagnostic.get_error_code(),
                message = diagnostic.get_message()
            )
        } else {
            format!(
                "{fname}:{sline}:{scolumn}:{{{sline}:{scolumn}-{eline}:{ecolumn}}}: error[{code}]: {message}",
                fname = diagnostic.get_location().get_file_name().unwrap(),
                sline = diagnostic.get_location().get_line(),
                scolumn = diagnostic.get_location().get_column(),
                eline = diagnostic.get_location().get_line_end(),
                ecolumn = diagnostic.get_location().get_column_end(),
                code = diagnostic.get_error_code(),
                message = diagnostic.get_message(),
            )
        }
    }
}

trait _ClangFormat {
    fn clang() -> Renderer;
}

impl _ClangFormat for Renderer {
    fn clang() -> Renderer {
        todo!()
    }
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
