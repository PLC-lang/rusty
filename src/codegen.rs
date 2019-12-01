use super::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

use inkwell::types::StringRadix;
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

    pub fn generate(&self, root: &CompilationUnit) -> String {
        self.generate_compilation_unit(root);
        self.module.print_to_string().to_string()
    }

    fn generate_compilation_unit(&self, root: &CompilationUnit) {
        for unit in &root.units {
            self.generate_program(unit);
        }
    }

    fn generate_program(&self, p: &Program) {
        let void_type = self.context.i32_type(); //void_type();
        let f_type = void_type.fn_type(&[], false);
        let function = self.module.add_function(p.name.as_str(), f_type, None);
        let block = self.context.append_basic_block(function, "entry");

        for stmt in &p.statements {
            self.builder.position_at_end(&block);
            let res = self.generate_statement(stmt);
            self.builder.build_return(Some(&res.unwrap()));
        }
    }

    fn generate_statement(&self, s: &Statement) -> Option<BasicValueEnum> {
        match s {
            Statement::BinaryExpression {
                operator,
                left,
                right,
            } => self.generate_binary_expression(operator, left, right),
            Statement::LiteralNumber { value } => self.generate_literal_number(value.as_str()),
            _ => None,
        }
    }

    fn generate_literal_number(&self, value: &str) -> Option<BasicValueEnum> {
        let itype = self.context.i32_type();
        println!("Generating Literal {}", value);
        let value = itype.const_int_from_string(value, StringRadix::Decimal);
        Some(BasicValueEnum::IntValue(value?))
    }

    fn generate_binary_expression(
        &self,
        operator: &Operator,
        left: &Box<Statement>,
        right: &Box<Statement>,
    ) -> Option<BasicValueEnum> {
        let lvalue = self.generate_statement(left).unwrap().into_int_value();
        let rvalue = self.generate_statement(right).unwrap().into_int_value();

        let result = match operator {
            Operator::Plus => self.builder.build_int_add(lvalue, rvalue, "tmpVar"),
            Operator::Minus => unimplemented!(),
            Operator::Multiplication => unimplemented!(),
            Operator::Division => unimplemented!(),
        };
        Some(BasicValueEnum::IntValue(result))
    }
}
