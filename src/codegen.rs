use super::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

use inkwell::values::BasicValueEnum;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> CodeGen<'ctx> {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let codegen = CodeGen {
            context: context,
            module,
            builder,
        };
        codegen
    }

    pub fn generate(&mut self, root: &CompilationUnit) -> String {
        self.module.print_to_string().to_string()
    }

    fn generate_compilation_unit(&mut self, root: &CompilationUnit) {
        for unit in &root.units {
            self.generate_program(unit);
        }
    }

    fn generate_program(&mut self, p: &Program) {
        let void_type = self.context.void_type();
        let f_type = void_type.fn_type(&[], false);
        let function = self.module.add_function(p.name.as_str(), f_type, None);
        let block = self.context.append_basic_block(function, "entry");

        for stmt in &p.statements {
            self.builder.position_at_end(&block);
            self.generate_statement(stmt);
        }
    }

    fn generate_statement(&mut self, s: &Statement) -> Option<BasicValueEnum> {
        match s {
            Statement::BinaryExpression {
                operator,
                left,
                right,
            } => self.generate_binary_expression(operator, left, right),
            _ => None,
        }
    }

    fn generate_binary_expression(
        &mut self,
        operator: &Operator,
        left: &Box<Statement>,
        right: &Box<Statement>,
    ) -> Option<BasicValueEnum> {
        let lvalue = self.generate_statement(left).unwrap().into_int_value();
        let rvalue = self.generate_statement(right).unwrap().into_int_value();

        match operator {
            Operator::Plus => Some(self.builder.build_int_add(lvalue, rvalue, "tmpVar")),
            Operator::Minus => None,
            Operator::Multiplication => None,
            Operator::Division => None,
        }
    }
}
