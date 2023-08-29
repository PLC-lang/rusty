#[cfg(test)]
pub mod tests {

    use std::{cell::RefCell, path::PathBuf, rc::Rc, str::FromStr};

    use plc_ast::{
        ast::{pre_process, CompilationUnit, LinkageType, SourceRangeFactory},
        provider::IdProvider,
    };
    use plc_diagnostics::{
        diagnostician::Diagnostician,
        diagnostics::Diagnostic,
        reporter::{DiagnosticReporter, ResolvedDiagnostics},
    };
    use source::{Compilable, SourceCode, SourceContainer};

    use crate::{
        builtins,
        codegen::{CodegenContext, GeneratedModule},
        index::{self, Index},
        lexer, parser,
        resolver::{const_evaluator::evaluate_constants, AnnotationMapImpl, AstAnnotations, TypeAnnotator},
        typesystem::get_builtin_types,
        DebugLevel, Validator,
    };

    ///a Diagnostic reporter that holds all diagnostics in a list
    #[derive(Default)]
    #[cfg(test)]
    pub struct ListBasedDiagnosticReporter {
        last_id: usize,
        // RC to access from tests, RefCell to avoid changing the signature for the report() method
        diagnostics: Rc<RefCell<Vec<ResolvedDiagnostics>>>,
    }

    #[cfg(test)]
    impl DiagnosticReporter for ListBasedDiagnosticReporter {
        fn report(&mut self, diagnostics: &[ResolvedDiagnostics]) {
            self.diagnostics.borrow_mut().extend_from_slice(diagnostics);
        }

        fn register(&mut self, _path: String, _src: String) -> usize {
            // at least provide some unique ids
            self.last_id += 1;
            self.last_id
        }
    }

    pub fn parse(src: &str) -> (CompilationUnit, Vec<Diagnostic>) {
        parser::parse(
            lexer::lex_with_ids(src, IdProvider::default(), SourceRangeFactory::internal()),
            LinkageType::Internal,
            "test.st",
        )
    }

    pub fn parse_and_preprocess(src: &str) -> (CompilationUnit, Vec<Diagnostic>) {
        let id_provider = IdProvider::default();
        let (mut unit, diagnostic) = parser::parse(
            lexer::lex_with_ids(src, id_provider.clone(), SourceRangeFactory::internal()),
            LinkageType::Internal,
            "test.st",
        );
        pre_process(&mut unit, id_provider);
        (unit, diagnostic)
    }

    #[cfg(test)]
    fn do_index<T: Into<SourceCode>>(
        src: T,
        id_provider: IdProvider,
        mode: Mode,
    ) -> (CompilationUnit, Index) {
        let source = src.into();
        let source_str = &source.source;
        let source_path = source.get_location_str();
        let mut index = Index::default();
        //Import builtins
        let builtins = builtins::parse_built_ins(id_provider.clone());

        index.import(index::visitor::visit(&builtins));
        // import built-in types like INT, BOOL, etc.
        for data_type in get_builtin_types() {
            index.register_type(data_type);
        }

        let range_factory = if source_path == "<internal>" {
            SourceRangeFactory::internal()
        } else {
            SourceRangeFactory::for_file(source_path)
        };

        let mut unit = match mode {
            Mode::ST => {
                parser::parse(
                    lexer::lex_with_ids(source_str, id_provider.clone(), range_factory),
                    LinkageType::Internal,
                    source_path,
                )
                .0
            }

            Mode::CFC => {
                plc_xml::xml_parser::parse_file(
                    source_str,
                    source_path,
                    LinkageType::Internal,
                    id_provider.clone(),
                    &mut Diagnostician::null_diagnostician(), // TODO: Should this be a null diagnostician?
                )
            }
        };

        pre_process(&mut unit, id_provider);
        index.import(index::visitor::visit(&unit));
        (unit, index)
    }

    pub fn index(src: &str) -> (CompilationUnit, Index) {
        let id_provider = IdProvider::default();
        do_index(src, id_provider, Mode::ST)
    }

    pub fn index_with_ids<T: Into<SourceCode>>(src: T, id_provider: IdProvider) -> (CompilationUnit, Index) {
        do_index(src, id_provider, Mode::ST)
    }

    pub fn annotate_with_ids(
        parse_result: &CompilationUnit,
        index: &mut Index,
        id_provider: IdProvider,
    ) -> AnnotationMapImpl {
        let (mut annotations, ..) = TypeAnnotator::visit_unit(index, parse_result, id_provider);
        index.import(std::mem::take(&mut annotations.new_index));
        annotations
    }

    pub fn parse_and_validate_buffered(src: &str) -> String {
        let diagnostics = parse_and_validate(src);
        let mut reporter = Diagnostician::buffered();

        reporter.register_file("<internal>".to_string(), src.to_string());
        reporter.handle(diagnostics);

        reporter.buffer().expect(
            "This should be unreachable, otherwise somethings wrong with the buffered codespan reporter",
        )
    }

    pub fn parse_and_validate(src: &str) -> Vec<Diagnostic> {
        let id_provider = IdProvider::default();
        let (unit, index) = index_with_ids(src, id_provider.clone());

        let (mut index, ..) = evaluate_constants(index);
        let (mut annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
        index.import(std::mem::take(&mut annotations.new_index));

        let mut validator = Validator::new();
        validator.perform_global_validation(&index);
        validator.visit_unit(&annotations, &index, &unit);
        validator.diagnostics()
    }

    pub fn codegen_without_unwrap(src: &str) -> Result<String, Diagnostic> {
        codegen_debug_without_unwrap(src, DebugLevel::None)
    }

    pub fn codegen_debug_without_unwrap(src: &str, debug_level: DebugLevel) -> Result<String, Diagnostic> {
        _codegen_debug_without_unwrap(src, debug_level, Mode::ST)
    }

    pub fn codegen_debug_without_unwrap_cfc(src: &str) -> Result<String, Diagnostic> {
        _codegen_debug_without_unwrap(src, DebugLevel::None, Mode::CFC)
    }

    /// Returns either a string or an error, in addition it always returns
    /// reported diagnostics. Therefor the return value of this method is always a tuple.
    /// TODO: This should not be so, we should have a diagnostic type that holds multiple new
    /// issues.
    pub fn _codegen_debug_without_unwrap(
        src: &str,
        debug_level: DebugLevel,
        mode: Mode,
    ) -> Result<String, Diagnostic> {
        let mut id_provider = IdProvider::default();
        let (unit, index) = match mode {
            Mode::ST => do_index(src, id_provider.clone(), mode),
            Mode::CFC => do_index(src, id_provider.clone(), mode),
        };

        let (mut index, ..) = evaluate_constants(index);
        let (mut annotations, dependencies, literals) =
            TypeAnnotator::visit_unit(&index, &unit, id_provider.clone());
        index.import(std::mem::take(&mut annotations.new_index));

        let context = CodegenContext::create();
        let path = PathBuf::from_str("src").ok();
        let mut code_generator = crate::codegen::CodeGen::new(
            &context,
            path.as_deref(),
            "main",
            crate::OptimizationLevel::None,
            debug_level,
        );
        let annotations = AstAnnotations::new(annotations, id_provider.next_id());
        let llvm_index =
            code_generator.generate_llvm_index(&context, &annotations, &literals, &dependencies, &index)?;

        code_generator
            .generate(&context, &unit, &annotations, &index, &llvm_index)
            .map(|module| module.persist_to_string())
    }

    #[derive(Copy, Clone)]
    pub enum Mode {
        ST,
        CFC,
    }

    pub fn codegen_with_debug(src: &str) -> String {
        codegen_debug_without_unwrap(src, DebugLevel::Full).unwrap()
    }

    pub fn codegen(src: &str) -> String {
        codegen_without_unwrap(src).unwrap()
    }

    fn codegen_into_modules<T: Compilable>(
        context: &CodegenContext,
        sources: T,
        debug_level: DebugLevel,
        mode: Mode,
    ) -> Result<Vec<GeneratedModule<'_>>, Diagnostic>
    where
        SourceCode: From<<T as Compilable>::T>,
    {
        let mut id_provider = IdProvider::default();
        let mut units = vec![];
        let mut index = Index::default();
        sources.containers().into_iter().map(|source| do_index(source, id_provider.clone(), mode)).for_each(
            |(unit, idx)| {
                units.push(unit);
                index.import(idx);
            },
        );
        let (mut index, ..) = evaluate_constants(index);
        let mut all_annotations = AnnotationMapImpl::default();
        let units = units
            .into_iter()
            .map(|unit| {
                let (mut annotation, dependencies, literals) =
                    TypeAnnotator::visit_unit(&index, &unit, id_provider.clone());
                index.import(std::mem::take(&mut annotation.new_index));
                all_annotations.import(annotation);
                (unit, dependencies, literals)
            })
            .collect::<Vec<_>>();

        let path = PathBuf::from_str("src").ok();
        let annotations = AstAnnotations::new(all_annotations, id_provider.next_id());
        units
            .into_iter()
            .map(|(unit, dependencies, literals)| {
                let mut code_generator = crate::codegen::CodeGen::new(
                    context,
                    path.as_deref(),
                    &unit.file_name,
                    crate::OptimizationLevel::None,
                    debug_level,
                );
                let llvm_index = code_generator.generate_llvm_index(
                    context,
                    &annotations,
                    &literals,
                    &dependencies,
                    &index,
                )?;

                code_generator.generate(context, &unit, &annotations, &index, &llvm_index)
            })
            .collect::<Result<Vec<_>, Diagnostic>>()
    }

    pub fn codegen_multi<T: Compilable>(sources: T, debug_level: DebugLevel) -> Vec<String>
    where
        SourceCode: From<<T as Compilable>::T>,
    {
        let context = CodegenContext::create();
        codegen_into_modules(&context, sources, debug_level, Mode::ST)
            .unwrap()
            .into_iter()
            .map(|module| module.persist_to_string())
            .collect()
    }

    pub fn generate_with_empty_program(src: &str) -> String {
        let source = format!("{} {}", "PROGRAM main END_PROGRAM", src);
        codegen(source.as_str())
    }
}
