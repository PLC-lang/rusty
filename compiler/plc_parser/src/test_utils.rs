// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

pub mod tests {
    use plc_ast::{
        ast::{CompilationUnit, LinkageType},
        provider::IdProvider,
    };
    use plc_diagnostics::{
        diagnostician::Diagnostician, diagnostics::Diagnostic, reporter::DiagnosticReporter,
    };
    use plc_source::source_location::SourceLocationFactory;

    use crate::{lexer, parser};

    pub fn parse(src: &str) -> (CompilationUnit, Vec<Diagnostic>) {
        parser::parse(
            lexer::lex_with_ids(src, IdProvider::default(), SourceLocationFactory::internal(src)),
            LinkageType::Internal,
            "test.st",
        )
    }

    pub fn parse_buffered(src: &str) -> (CompilationUnit, String) {
        let mut reporter = Diagnostician::buffered();
        reporter.register_file("<internal>".to_string(), src.to_string());
        let (unit, diagnostics) = parse(src);
        reporter.handle(&diagnostics);
        (unit, reporter.buffer().unwrap_or_default())
    }
}
