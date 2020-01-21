use std::collections::HashMap;

use super::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::module::Linkage;

use inkwell::types::BasicTypeEnum;
use inkwell::types::StringRadix;
use inkwell::types::StructType;

use inkwell::values::BasicValueEnum;
use inkwell::values::BasicValue;
use inkwell::values::FunctionValue;
use inkwell::values::PointerValue;
use inkwell::values::GlobalValue;

use inkwell::AddressSpace;
use inkwell::IntPredicate;

use inkwell::basic_block::BasicBlock;

#[cfg(test)]
mod tests;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

    variables: HashMap<String, u32>,
    current_pou: String,
    current_function : Option<FunctionValue<'ctx>>,

}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> CodeGen<'ctx> {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let codegen = CodeGen {
            context: context,
            module,
            builder,
            variables: HashMap::new(),
            current_pou: "".to_string(),
            current_function : None,
        };
        codegen
    }

    fn get_struct_name(pou_name: &str) -> String {
        format!("{}_interface", pou_name)
    }

    fn get_struct_instance_name(pou_name: &str) -> String {
        format!("{}_instance", pou_name)
    }

    pub fn generate(&mut self, root: &CompilationUnit) -> String {
        self.generate_compilation_unit(root);
        self.module.print_to_string().to_string()
    }

    fn generate_compilation_unit(&mut self, root: &CompilationUnit) {
        for unit in &root.units {
            self.generate_program(unit);
        }
    }

    fn generate_program(&mut self, p: &Program) {
        
        self.current_pou = p.name.clone();
        
        let return_type = self.context.void_type(); 
        //let return_type = self.context.i32_type();
        let f_type = return_type.fn_type(&[], false);
        self.current_function = Some(self.module.add_function(self.current_pou.as_str(), f_type, None));
        let block = self.context.append_basic_block(self.current_function.unwrap(), "entry");

        let mut program_members: Vec<(String, BasicTypeEnum)> = Vec::new();

        for var_block in &p.variable_blocks {
            let mut members = self.get_variables_information(var_block);
            program_members.append(&mut members);
        }
        //Create a struct with the value from the program
        let member_type = CodeGen::generate_instance_struct(
            self.context,
            &mut self.variables,
            program_members,
            &CodeGen::get_struct_name(p.name.as_str()),
        );

        //Create An instance variable for that struct
        //Place in global data
        self.generate_instance_variable(member_type, CodeGen::get_struct_instance_name(p.name.as_str()).as_str());
        //let mut result = None;
        self.generate_statement_list(&block,&p.statements);
        //self.builder.build_return(Some(&result.unwrap()));
        self.builder.build_return(None);
    }

    fn get_variables_information(&self, v: &VariableBlock) -> Vec<(String, BasicTypeEnum<'ctx>)> {
        let mut types: Vec<(String, BasicTypeEnum)> = Vec::new();
        for variable in &v.variables {
            let var_type = self.get_type(&variable.data_type);
            types.push((variable.name.clone(), var_type.into()));
        }
        types
    }

    fn get_type(&self, t: &Type) -> BasicTypeEnum<'ctx> {
        if let Type::Primitive(name) = t {
            match name {
                PrimitiveType::Int => self.context.i32_type().into(),
                PrimitiveType::Bool => self.context.bool_type().into(),
            }
        } else {
            panic!("Unkown type {:?}", t)
        }
    }

    fn generate_instance_struct(
        context: &'ctx Context,
        variables: &mut HashMap<String, u32>,
        members: Vec<(String, BasicTypeEnum)>,
        name: &str,
    ) -> StructType<'ctx> {
        let struct_type = context.opaque_struct_type(name);
        let mut member_types: Vec<BasicTypeEnum> = Vec::new();

        //let member_types = members.into_iter().map(|(_, it)| it).collect::<Vec<_>>();

        for (index, (type_name, t)) in members.iter().enumerate() {
            member_types.push(*t);
            variables.insert(type_name.to_string(), index as u32);
        }

        struct_type.set_body(member_types.as_slice(), false);
        struct_type
    }

    fn generate_instance_variable(
        &self,
        variable_type: StructType<'ctx>,
        name: &str,
    ) -> GlobalValue {
        let result = self.module
            .add_global(variable_type, Some(AddressSpace::Generic), name);

        result.set_initializer(&variable_type.const_zero());
        result.set_linkage(Linkage::Common);
        result
    }

    fn generate_statement(&self, s: &Statement) -> Option<BasicValueEnum> {
        match s {
            Statement::IfStatement {
                blocks,
                else_block,
            } => self.generate_if_statement(blocks,else_block),
            Statement::BinaryExpression {
                operator,
                left,
                right,
            } => self.generate_binary_expression(operator, left, right),
            Statement::LiteralNumber { value } => self.generate_literal_number(value.as_str()),
            Statement::Reference { name } => self.generate_variable_reference(name),
            Statement::Assignment { left, right } => self.generate_assignment(&left, &right),
            Statement::UnaryExpression { operator, value } => self.generate_unary_expression(&operator, &value),
            _ => unimplemented!(),
        }
    }

    fn generate_if_statement(&self, conditional_blocks : &Vec<ConditionalBlock>, else_body : &Vec<Statement>) -> Option<BasicValueEnum> {
        let mut blocks = Vec::new();
        blocks.push(self.builder.get_insert_block().unwrap());
        for _ in 1..conditional_blocks.len() {
            blocks.push(self.context.append_basic_block(self.current_function?, ""));
        }

        let else_block = if else_body.len() > 0 {
            let result = self.context.append_basic_block(self.current_function?, "");
            blocks.push(result);
            Some(result)
        } else {
            None
        };
        //Continue
        let continue_block = self.context.append_basic_block(self.current_function?, "");
        blocks.push(continue_block);

        for (i, block) in conditional_blocks.iter().enumerate() {

            let then_block = blocks[i];
            let else_block = blocks[i+1];
            
            self.builder.position_at_end(&then_block);

            let condition = self.generate_statement(&block.condition).unwrap().into_int_value();
            let conditional_block = self.context.prepend_basic_block(&else_block, "");
            
            //Generate if statement condition
            self.builder.build_conditional_branch(condition, &conditional_block, &else_block);
            

            //Generate if statement content
            self.generate_statement_list(&conditional_block, &block.body);
            self.builder.build_unconditional_branch(&continue_block);
        }
        //Else
        if let Some(else_block) = else_block {
            self.generate_statement_list(&else_block, else_body);
            self.builder.build_unconditional_branch(&continue_block);
        }
        //Continue
        self.builder.position_at_end(&continue_block);

        None
    }

    fn generate_statement_list(&self, block : &BasicBlock, statements:&Vec<Statement>) {
        self.builder.position_at_end(&block);
        for statement in statements {
            self.generate_statement(statement);
        }
    }

    fn generate_unary_expression(&self, operator: &Operator, value: &Box<Statement>) -> Option<BasicValueEnum> {
        let loaded_value = self.generate_statement(value).unwrap().into_int_value();    
        let value = match operator {
            Operator::Not => self.builder.build_not(loaded_value, "tmpVar"),
            Operator::Minus => self.builder.build_int_neg(loaded_value, "tmpVar"),
            _ => unimplemented!()
        };
        Some(BasicValueEnum::IntValue(value))
    }

    fn generate_variable_lreference(&self, name: &str) -> Option<PointerValue> {
        // for now we only support locals
        let struct_type = self.module.get_type(CodeGen::get_struct_name(self.current_pou.as_str()).as_str()).unwrap().into_struct_type();
        println!("{:?} ->{}",struct_type.get_name().unwrap(), struct_type.count_fields());
        
        let ptr_struct_ype = struct_type.ptr_type(AddressSpace::Generic);
        let void_ptr_value = self
            .module
            .get_global(CodeGen::get_struct_instance_name(self.current_pou.as_str()).as_str()).unwrap().as_basic_value_enum().into_pointer_value();
        
        let ptr_value = self.builder.build_pointer_cast(void_ptr_value, ptr_struct_ype, "temp_struct");
        let index = self.variables.get(name);

        if let Some(index) = index {
            // let struct_value = self.builder.build_load(ptr_value, "temp_struct");
            // let tt = struct_value.get_type().into_struct_type();
            //println!("{:?} -> {:?}", tt, tt.get_field_types());
             let ptr_result = 
             unsafe {self.builder.build_struct_gep(ptr_value, *index, "temp_struct")};
             Some(ptr_result)
            //self.builder.build_extract_value(struct_value.into_struct_value(), *index, name)
        } else {
            None
        }
    }

    fn generate_variable_reference(&self, name: &str) -> Option<BasicValueEnum> {
        if let Some(ptr) =  self.generate_variable_lreference(name) {
            let result = self.builder.build_load(ptr, format!("load_{var_name}", var_name = name).as_str()); 
            Some(result)
        } else {
            None
        }
    }

    /**
     *  int x = 7;
        x = 7 + x;
     *  return x;
     * 
            %1 = alloca i32, align 4
            %2 = alloca i32, align 4
            store i32 0, i32* %1, align 4
            store i32 7, i32* %2, align 4
            %3 = load i32, i32* %2, align 4
            %4 = add nsw i32 7, %3
            store i32 %4, i32* %2, align 4

            https://github.com/sinato/inkwell-playground/tree/master/examples

     */

    fn generate_assignment(&self, left: &Box<Statement>, right : &Box<Statement>) -> Option<BasicValueEnum> {
        
        if let Statement::Reference { name } = &**left {
            let left_expr = self.generate_variable_lreference(&name);
            let right_res = self.generate_statement(right);
            self.builder.build_store(left_expr?, right_res?);
        }
        None


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
        let lval_opt = self.generate_statement(left);
        let lvalue = lval_opt.unwrap().into_int_value();

        let rval_opt = self.generate_statement(right);
        let rvalue = rval_opt.unwrap().into_int_value();

        let result = match operator {
            Operator::Plus => self.builder.build_int_add(lvalue, rvalue, "tmpVar"),
            Operator::Minus => self.builder.build_int_sub(lvalue, rvalue, "tmpVar") ,
            Operator::Multiplication => self.builder.build_int_mul(lvalue, rvalue, "tmpVar"),
            Operator::Division => self.builder.build_int_signed_div(lvalue, rvalue, "tmpVar"),
            Operator::Modulo => self.builder.build_int_signed_rem(lvalue, rvalue, "tmpVar"),
            Operator::Equal => self.builder.build_int_compare(IntPredicate::EQ, lvalue, rvalue, "tmpVar"),
            Operator::NotEqual => self.builder.build_int_compare(IntPredicate::NE, lvalue, rvalue, "tmpVar"),
            Operator::Less => self.builder.build_int_compare(IntPredicate::SLT, lvalue, rvalue, "tmpVar"),
            Operator::Greater => self.builder.build_int_compare(IntPredicate::SGT, lvalue, rvalue, "tmpVar"),
            Operator::LessOrEqual => self.builder.build_int_compare(IntPredicate::SLE, lvalue, rvalue, "tmpVar"),
            Operator::GreaterOrEqual => self.builder.build_int_compare(IntPredicate::SGE, lvalue, rvalue, "tmpVar"),
            Operator::And => self.builder.build_and(lvalue, rvalue, "tmpVar"),
            Operator::Or => self.builder.build_or(lvalue, rvalue, "tmpVar"),
            Operator::Xor => self.builder.build_xor(lvalue, rvalue, "tmpVar"),
            _ => unimplemented!(),
        };
        Some(BasicValueEnum::IntValue(result))
    }
}