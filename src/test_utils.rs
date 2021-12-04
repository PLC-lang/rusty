#[cfg(test)]
pub mod tests {

    use crate::{
        ast::{self, CompilationUnit},
        diagnostics::Diagnostic,
        index::{self, Index},
        lexer::{self, IdProvider},
        parser,
        resolver::{
            const_evaluator::evaluate_constants, AnnotationMapImpl, AstAnnotations, TypeAnnotator,
        },
        Validator,
    };

    pub fn parse(src: &str) -> (CompilationUnit, Vec<Diagnostic>) {
        parser::parse(lexer::lex_with_ids(src, IdProvider::default()))
    }

    fn do_index(src: &str, id_provider: IdProvider) -> (CompilationUnit, Index) {
        let (mut unit, ..) = parser::parse(lexer::lex_with_ids(src, id_provider.clone()));
        ast::pre_process(&mut unit);
        let index = index::visitor::visit(&unit, id_provider);
        (unit, index)
    }

    pub fn index(src: &str) -> (CompilationUnit, Index) {
        let id_provider = IdProvider::default();
        do_index(src, id_provider)
    }

    pub fn annotate(parse_result: &CompilationUnit, index: &mut Index) -> AnnotationMapImpl {
        let mut annotations = TypeAnnotator::visit_unit(index, parse_result);
        index.import(std::mem::take(&mut annotations.new_index));
        annotations
    }

    pub fn parse_and_validate(src: &str) -> Vec<Diagnostic> {
        let (unit, index) = index(src);

        let (mut index, ..) = evaluate_constants(index);
        let mut annotations = TypeAnnotator::visit_unit(&index, &unit);
        index.import(std::mem::take(&mut annotations.new_index));

        let mut validator = Validator::new();
        validator.visit_unit(&annotations, &index, &unit);
        validator.diagnostics()
    }

    pub fn codegen_without_unwrap(src: &str) -> Result<String, Diagnostic> {
        let mut id_provider = IdProvider::default();
        let (unit, index) = do_index(src, id_provider.clone());

        let (mut index, ..) = evaluate_constants(index);
        let mut annotations = TypeAnnotator::visit_unit(&index, &unit);
        index.import(std::mem::take(&mut annotations.new_index));

        let context = inkwell::context::Context::create();
        let code_generator = crate::codegen::CodeGen::new(&context, "main");
        let annotations = AstAnnotations::new(annotations, id_provider.next_id());
        let llvm_index = code_generator.generate_llvm_index(&annotations, &index)?;
        code_generator.generate(&unit, &annotations, &index, &llvm_index)
    }

    pub fn codegen(src: &str) -> String {
        codegen_without_unwrap(src).unwrap()
    }

    pub fn generate_with_empty_program(src: &str) -> String {
        let source = format!("{} {}", "PROGRAM main END_PROGRAM", src);
        codegen(source.as_str())
    }
}
