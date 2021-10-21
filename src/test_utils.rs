#[cfg(test)]
pub mod tests {

    use crate::{
        ast::{self, CompilationUnit},
        compile_error::CompileError,
        index::{self, Index},
        lexer::{self, IdProvider},
        parser,
        resolver::{const_evaluator::evaluate_constants, TypeAnnotator},
        Diagnostic, Validator,
    };

    pub fn parse(src: &str) -> (CompilationUnit, Vec<Diagnostic>) {
        parser::parse(lexer::lex_with_ids(src, IdProvider::new()))
    }

    pub fn index(src: &str) -> (CompilationUnit, Index) {
        let (mut unit, ..) = parse(src);
        ast::pre_process(&mut unit);
        let index = index::visitor::visit(&unit);
        (unit, index)
    }

    pub fn parse_and_validate(src: &str) -> Vec<Diagnostic> {
        let (unit, index) = index(src);

        let (index, ..) = evaluate_constants(index);
        let annotations = TypeAnnotator::visit_unit(&index, &unit);

        let mut validator = Validator::new();
        validator.visit_unit(&annotations, &index, &unit);
        validator.diagnostics()
    }

    pub fn codegen_without_unwrap(src: &str) -> Result<String, CompileError> {
        let (unit, index) = index(src);

        let (index, ..) = evaluate_constants(index);
        let annotations = TypeAnnotator::visit_unit(&index, &unit);

        let context = inkwell::context::Context::create();
        let code_generator = crate::codegen::CodeGen::new(&context, "main");
        code_generator.generate(&unit, &annotations, &index)
    }

    pub fn codegen(src: &str) -> String {
        codegen_without_unwrap(src).unwrap()
    }

    pub fn generate_with_empty_program(src: &str) -> String {
        let source = format!("{} {}", "PROGRAM main END_PROGRAM", src);
        codegen(source.as_str())
    }
}
