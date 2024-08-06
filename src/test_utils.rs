#[cfg(test)]
pub mod tests {

    use std::{path::PathBuf, str::FromStr, sync::Mutex};

    use plc_ast::{
        ast::{pre_process, CompilationUnit, LinkageType},
        provider::IdProvider,
    };
    use plc_diagnostics::{
        diagnostician::Diagnostician, diagnostics::Diagnostic, reporter::DiagnosticReporter,
    };
    use plc_index::GlobalContext;
    use plc_source::{source_location::SourceLocationFactory, Compilable, SourceCode, SourceContainer};

    use crate::{
        builtins,
        codegen::{CodegenContext, GeneratedModule},
        index::{self, Index},
        lexer, parser,
        resolver::{const_evaluator::evaluate_constants, AnnotationMapImpl, AstAnnotations, TypeAnnotator},
        typesystem::get_builtin_types,
        DebugLevel, Validator,
    };

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

    pub fn parse_and_preprocess(src: &str) -> (CompilationUnit, String) {
        let mut reporter = Diagnostician::buffered();
        reporter.register_file("<internal>".to_string(), src.to_string());
        let id_provider = IdProvider::default();
        let (mut unit, diagnostic) = parser::parse(
            lexer::lex_with_ids(src, id_provider.clone(), SourceLocationFactory::internal(src)),
            LinkageType::Internal,
            "test.st",
        );
        pre_process(&mut unit, id_provider);
        reporter.handle(&diagnostic);

        (unit, reporter.buffer().unwrap_or_default())
    }

    fn do_index<T: Into<SourceCode>>(
        src: T,
        id_provider: IdProvider,
    ) -> (CompilationUnit, Index, Vec<Diagnostic>) {
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

        let range_factory = SourceLocationFactory::for_source(&source);
        let (mut unit, diagnostics) = parser::parse(
            lexer::lex_with_ids(source_str, id_provider.clone(), range_factory),
            LinkageType::Internal,
            source_path,
        );

        pre_process(&mut unit, id_provider);
        index.import(index::visitor::visit(&unit));
        (unit, index, diagnostics)
    }

    pub fn index(src: &str) -> (CompilationUnit, Index) {
        let id_provider = IdProvider::default();
        let (unit, index, _) = do_index(src, id_provider);
        (unit, index)
    }

    pub fn index_with_ids<T: Into<SourceCode>>(src: T, id_provider: IdProvider) -> (CompilationUnit, Index) {
        let (unit, index, _) = do_index(src, id_provider);
        (unit, index)
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
        reporter.handle(&diagnostics);

        reporter.buffer().expect(
            "This should be unreachable, otherwise somethings wrong with the buffered codespan reporter",
        )
    }

    pub fn parse_and_validate(src: &str) -> Vec<Diagnostic> {
        let src = SourceCode::from(src);

        let mut ctxt = GlobalContext::new();
        ctxt.insert(&src, None).unwrap();

        let (unit, index, mut diagnostics) = do_index(src, ctxt.provider());

        let (mut index, ..) = evaluate_constants(index);
        let (mut annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, ctxt.provider());
        index.import(std::mem::take(&mut annotations.new_index));

        let mut validator = Validator::new(&ctxt);
        validator.perform_global_validation(&index);
        validator.visit_unit(&annotations, &index, &unit);
        diagnostics.extend(validator.diagnostics());
        diagnostics
    }

    pub fn codegen_without_unwrap(src: &str) -> Result<String, String> {
        codegen_debug_without_unwrap(src, DebugLevel::None)
    }

    /// Returns either a string or an error, in addition it always returns
    /// reported diagnostics. Therefor the return value of this method is always a tuple.
    /// TODO: This should not be so, we should have a diagnostic type that holds multiple new
    /// issues.
    pub fn codegen_debug_without_unwrap(src: &str, debug_level: DebugLevel) -> Result<String, String> {
        let mut reporter = Diagnostician::buffered();
        reporter.register_file("<internal>".to_string(), src.to_string());
        let mut id_provider = IdProvider::default();
        let (unit, index, diagnostics) = do_index(src, id_provider.clone());
        reporter.handle(&diagnostics);

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
            None,
            crate::OptimizationLevel::None,
            debug_level,
        );
        let annotations = AstAnnotations::new(annotations, id_provider.next_id());

        let got_layout = Mutex::new(None);

        let llvm_index = code_generator
            .generate_llvm_index(&context, &annotations, &literals, &dependencies, &index, &got_layout)
            .map_err(|err| {
                reporter.handle(&[err]);
                reporter.buffer().unwrap()
            })?;

        code_generator
            .generate(&context, &unit, &annotations, &index, llvm_index)
            .map(|module| module.persist_to_string())
            .map_err(|err| {
                reporter.handle(&[err]);
                reporter.buffer().unwrap()
            })
    }

    pub fn codegen_with_debug(src: &str) -> String {
        codegen_debug_without_unwrap(src, DebugLevel::Full(crate::DEFAULT_DWARF_VERSION)).unwrap()
    }

    pub fn codegen_with_debug_version(src: &str, dwarf_version: usize) -> String {
        codegen_debug_without_unwrap(src, DebugLevel::Full(dwarf_version)).unwrap()
    }

    pub fn codegen(src: &str) -> String {
        codegen_without_unwrap(src).unwrap()
    }

    fn codegen_into_modules<T: Compilable>(
        context: &CodegenContext,
        sources: T,
        debug_level: DebugLevel,
    ) -> Result<Vec<GeneratedModule<'_>>, Diagnostic>
    where
        SourceCode: From<<T as Compilable>::T>,
    {
        let mut id_provider = IdProvider::default();
        let mut units = vec![];
        let mut index = Index::default();
        sources.containers().into_iter().map(|source| do_index(source, id_provider.clone())).for_each(
            |(unit, idx, ..)| {
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
                    None,
                    crate::OptimizationLevel::None,
                    debug_level,
                );
                let got_layout = Mutex::new(None);

                let llvm_index = code_generator.generate_llvm_index(
                    context,
                    &annotations,
                    &literals,
                    &dependencies,
                    &index,
                    &got_layout,
                )?;

                code_generator.generate(context, &unit, &annotations, &index, llvm_index)
            })
            .collect::<Result<Vec<_>, Diagnostic>>()
    }

    pub fn codegen_multi<T: Compilable>(sources: T, debug_level: DebugLevel) -> Vec<String>
    where
        SourceCode: From<<T as Compilable>::T>,
    {
        let context = CodegenContext::create();
        codegen_into_modules(&context, sources, debug_level)
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
