use std::collections::HashMap;

use super::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::module::Linkage;

use inkwell::types::{BasicTypeEnum, StringRadix, StructType, BasicType, FunctionType};

use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue, BasicValue, GlobalValue};

use inkwell::{AddressSpace, ThreadLocalMode};
use inkwell::IntPredicate;

use inkwell::basic_block::BasicBlock;

#[cfg(test)]
mod tests;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

    current_pou: String,
    current_function : Option<FunctionValue<'ctx>>,
    current_parameters : HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> CodeGen<'ctx> {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let codegen = CodeGen {
            context: context,
            module,
            builder,
            current_pou: "".to_string(),
            current_function : None,
            current_parameters : HashMap::new(),    
        };
        codegen
    }

    fn get_struct_name(pou_name: &str) -> String {
        format!("{}_interface", pou_name)
    }

    fn get_struct_instance_name(pou_name: &str) -> String {
        format!("{}_instance", pou_name)
    }

    pub fn generate(&mut self, root: CompilationUnit) -> String {
        self.generate_compilation_unit(root);
        self.module.print_to_string().to_string()
    }

    pub fn generate_compilation_unit(&mut self, root: CompilationUnit) {
        for global_variables in &root.global_vars {
            self.generate_global_vars(global_variables);
        }
    
        for unit in &root.units {
            self.generate_pou(unit);
        }
    }

    fn get_type(&self, type_name: &Option<Type>) -> Option<BasicTypeEnum<'ctx>> {
        if let Some(type_name) = type_name {
            match type_name {
                Type::Primitive(primitive_type) => self.get_primitive_type(primitive_type),
                _ => None
            }
        } else {
            None
        }
    }

    fn get_primitive_type(&self, type_name: &PrimitiveType) -> Option<BasicTypeEnum<'ctx>> {
        match type_name {
            PrimitiveType::Int => Some(self.context.i32_type().as_basic_type_enum()),
            PrimitiveType::Bool => Some(self.context.bool_type().as_basic_type_enum()),
        }
    }

    fn get_function_type(&self, parameters: &[BasicTypeEnum<'ctx>], enum_type : Option<BasicTypeEnum<'ctx>>) -> FunctionType<'ctx> {
        if let Some(enum_type) = enum_type {
            if enum_type.is_int_type() {
                return enum_type.into_int_type().fn_type(parameters,false);
            }
        }
        self.context.void_type().fn_type(parameters,false)
    } 

    fn generate_global_vars(&mut self, global_vars: &VariableBlock) {
        let members = self.get_variables_information(global_vars);
        for (name, var_type) in members {
            self.generate_global_variable(var_type, None, &name);
        }
    }
    
    fn generate_pou(&mut self, p: &POU) {
        
        self.current_pou = p.name.clone();
        
        let mut pou_members: Vec<(String, BasicTypeEnum)> = Vec::new();
        let return_type = self.get_type(&p.return_type);

        for var_block in &p.variable_blocks {
            let mut members = self.get_variables_information(var_block);
            pou_members.append(&mut members);
        }

        //Create a struct with the value from the program
        let member_type = CodeGen::generate_instance_struct(
            self.context,
            &pou_members,
            &CodeGen::get_struct_name(p.name.as_str()),
        );
       
        let member_type_ptr = member_type.ptr_type(AddressSpace::Generic);
        //let return_type = self.context.i32_type();
        let f_type = self.get_function_type(&[member_type_ptr.as_basic_type_enum()],return_type);
        self.current_function = Some(self.module.add_function(self.current_pou.as_str(), f_type, None));
        let block = self.context.append_basic_block(self.current_function.unwrap(), "entry");




        //Create An instance variable for that struct
        //Place in global data
        let thread_local_mode = if p.pou_type == PouType::Function {Some(ThreadLocalMode::LocalExecTLSModel)} else {None};
        self.generate_global_variable(member_type.into(), thread_local_mode, CodeGen::get_struct_instance_name(p.name.as_str()).as_str());
        //let mut result = None;
        //Generate reference to parameter
        self.builder.position_at_end(block);
        self.current_parameters = pou_members.iter().enumerate().map(|(i,m)| 
            {
                let ptr_value = self.current_function.unwrap().get_first_param().unwrap().into_pointer_value();
                (m.0.clone(),self.builder.build_struct_gep(ptr_value, i as u32, &m.0).unwrap())
            } 
        ).collect();

        //Insert return variable
        if let Some(ret_type) = return_type {
            let ret_alloc = self.builder.build_alloca(ret_type, p.name.as_str());
            self.current_parameters.insert(p.name.clone(),ret_alloc);
        }

        self.generate_statement_list(block,&p.statements);
        //self.builder.build_return(Some(&result.unwrap()));i
        let ret_value =self.get_return_value(p.pou_type);
        if let Some(ret_value) = ret_value {
            self.builder.build_return(Some(&ret_value));
        } else {
            self.builder.build_return(None);
        };
    }

    fn get_return_value(&self, pou_type : PouType) -> Option<BasicValueEnum> {
        match pou_type {
            PouType::Function => {
                let value = self.generate_lvalue_for_reference(self.current_pou.as_str()).unwrap(); 
                Some(self.builder.build_load(value,format!("{}_ret",self.current_pou.as_str()).as_str()))
            },
            _ => None
        }
    }

    fn get_variables_information(&self, v: &VariableBlock) -> Vec<(String, BasicTypeEnum<'ctx>)> {
        let mut types: Vec<(String, BasicTypeEnum<'ctx>)> = Vec::new();
        for variable in &v.variables {
            let var_type = self.get_type(&Some(variable.data_type)).unwrap();
            types.push((variable.name.clone(), var_type));
        }
        types
    }

    fn generate_instance_struct(
        context: &'ctx Context,
        members: &Vec<(String, BasicTypeEnum)>,
        name: &str,
    ) -> StructType<'ctx> {
        let struct_type = context.opaque_struct_type(name);
        let member_types : Vec<BasicTypeEnum> = members.iter().map(|(_,t)| *t).collect();
        struct_type.set_body(member_types.as_slice(), false);
        struct_type
    }
    fn set_initializer_for_type(&self, global_value : &GlobalValue<'ctx>, variable_type : BasicTypeEnum<'ctx>) {
        if variable_type.is_int_type() {
            global_value.set_initializer(&variable_type.into_int_type().const_zero());
        } else if variable_type.is_struct_type() {
            global_value.set_initializer(&variable_type.into_struct_type().const_zero());
        }

    }

    fn generate_global_variable(
        &self,
        variable_type: BasicTypeEnum<'ctx>,
        thread_local_mode: Option<ThreadLocalMode>,
        name: &str,
    ) -> GlobalValue {
        let result = self.module
            .add_global(variable_type, Some(AddressSpace::Generic), name);
        self.set_initializer_for_type(&result, variable_type);
        result.set_thread_local_mode(thread_local_mode);
        result.set_linkage(Linkage::Common);
        result
    }

    fn generate_statement(&self, s: &Statement) -> Option<BasicValueEnum> {
        match s {
            Statement::IfStatement {
                blocks,
                else_block,
            } => self.generate_if_statement(blocks,else_block),
            Statement::CaseStatement {
                selector,
                case_blocks,
                else_block
            } => self.generate_case_statement(selector, case_blocks, else_block),
            //Loops
            Statement::ForLoopStatement {
                counter,
                start,
                end,
                by_step,
                body,
            } => self.generate_for_statement(counter, start, end,by_step,body),
            Statement::WhileLoopStatement {
                condition,
                body,
            } => self.generate_while_statement(condition, body),
            Statement::RepeatLoopStatement {
                condition,
                body,
            } => self.generate_repeat_statement(condition, body),
            //Expressions
            Statement::BinaryExpression {
                operator,
                left,
                right,
            } => self.generate_binary_expression(operator, left, right),
            Statement::LiteralInteger { value } => self.generate_literal_number(value.as_str()),
            Statement::LiteralBool { value } => self.generate_literal_boolean(*value),
            Statement::Reference { name } => self.generate_variable_reference(name),
            Statement::Assignment { left, right } => self.generate_assignment(&left, &right),
            Statement::UnaryExpression { operator, value } => self.generate_unary_expression(&operator, &value),
            Statement::CallStatement {operator, parameters} => self.generate_call_statement(&operator, &parameters),
            _ => panic!("{:?} not yet supported",s ),
        }
    }

    fn generate_case_statement(&self, selector : &Box<Statement>, conditional_blocks : &Vec<ConditionalBlock>, else_body : &Vec<Statement>) -> Option<BasicValueEnum> {
        
        //Continue
        let continue_block = self.context.append_basic_block(self.current_function?, "continue");
        
        let basic_block = self.builder.get_insert_block()?;
        let selector_statement = self.generate_statement(&*selector)?;
        let mut cases = Vec::new();
        
        
        //generate a int_value and a BasicBlock for every case-body
        for i in 0..conditional_blocks.len() {
            let conditional_block = &conditional_blocks[i];
            let basic_block = self.context.append_basic_block(self.current_function?, "case");
            let condition = self.generate_statement(&*conditional_block.condition)?;
            self.generate_statement_list(basic_block, &conditional_block.body);
            self.builder.build_unconditional_branch(continue_block);
            
            cases.push((condition.into_int_value(), basic_block));
        }

        let else_block = self.context.append_basic_block(self.current_function?, "else");
        self.generate_statement_list(else_block, else_body);
        self.builder.build_unconditional_branch(continue_block);
        

        //Move the continue block to after the else block
        continue_block.move_after(else_block).unwrap();
        //Position in initial block
        self.builder.position_at_end(basic_block);
        self.builder.build_switch(
            selector_statement.into_int_value(),
            else_block, 
            &cases);
        self.builder.position_at_end(continue_block);
        None
    }

    fn generate_if_statement(&self, conditional_blocks : &Vec<ConditionalBlock>, else_body : &Vec<Statement>) -> Option<BasicValueEnum> {
        let mut blocks = Vec::new();
        blocks.push(self.builder.get_insert_block().unwrap());
        for _ in 1..conditional_blocks.len() {
            blocks.push(self.context.append_basic_block(self.current_function?, "branch"));
        }

        let else_block = if else_body.len() > 0 {
            let result = self.context.append_basic_block(self.current_function?, "else");
            blocks.push(result);
            Some(result)
        } else {
            None
        };
        //Continue
        let continue_block = self.context.append_basic_block(self.current_function?, "continue");
        blocks.push(continue_block);

        for (i, block) in conditional_blocks.iter().enumerate() {

            let then_block = blocks[i];
            let else_block = blocks[i+1];
            
            self.builder.position_at_end(then_block);

            let condition = self.generate_statement(&block.condition).unwrap().into_int_value();
            let conditional_block = self.context.prepend_basic_block(else_block, "condition_body");
            
            //Generate if statement condition
            self.builder.build_conditional_branch(condition, conditional_block, else_block);
            

            //Generate if statement content
            self.generate_statement_list(conditional_block, &block.body);
            self.builder.build_unconditional_branch(continue_block);
        }
        //Else
        if let Some(else_block) = else_block {
            self.generate_statement_list(else_block, else_body);
            self.builder.build_unconditional_branch(continue_block);
        }
        //Continue
        self.builder.position_at_end(continue_block);
        None
    }



    fn generate_for_statement(&self, counter: &Box<Statement>, start : &Box<Statement>, end : &Box<Statement>, by_step : &Option<Box<Statement>>, body : &Vec<Statement> ) -> Option<BasicValueEnum> {
        self.generate_assignment(counter, start);        
        let condition_check = self.context.append_basic_block(self.current_function?, "condition_check");
        let for_body = self.context.append_basic_block(self.current_function?, "for_body");
        let continue_block = self.context.append_basic_block(self.current_function?, "continue");
        //Generate an initial jump to the for condition
        self.builder.build_unconditional_branch(condition_check);
        
        //Check loop condition
        self.builder.position_at_end(condition_check);
        let counter_statement = self.generate_statement(counter).unwrap().into_int_value();
        let end_statement = self.generate_statement(end).unwrap().into_int_value();
        let compare = self.builder.build_int_compare(IntPredicate::SLE, counter_statement, end_statement, "tmpVar");
        self.builder.build_conditional_branch(compare, for_body, continue_block);

        //Enter the for loop
        self.generate_statement_list(for_body, &body);
        
        //Increment
        let step_by_value = by_step.as_ref()
            .map(|step|self.generate_statement(&step).unwrap())
            .or(self.generate_literal_number("1")).unwrap().into_int_value();

        let next = self.builder.build_int_add(counter_statement,step_by_value, "tmpVar");
        let ptr = self.generate_lvalue_for(counter).unwrap();
        self.builder.build_store(ptr, next);

        //Loop back
        self.builder.build_unconditional_branch(condition_check); 
        
        //Continue
        self.builder.position_at_end(continue_block);
        None
    }

    fn generate_base_while_statement(&self, condition: &Box<Statement>, body: &Vec<Statement>) -> Option<BasicValueEnum> {
        let condition_check = self.context.append_basic_block(self.current_function?, "condition_check");
        let while_body = self.context.append_basic_block(self.current_function?, "while_body");
        let continue_block = self.context.append_basic_block(self.current_function?, "continue");
        
        //Check loop condition
        self.builder.position_at_end(condition_check);
        let condition_value = self.generate_statement(condition)?.into_int_value();
        self.builder.build_conditional_branch(condition_value, while_body, continue_block);

        //Enter the for loop
        self.generate_statement_list(while_body, &body);
        //Loop back
        self.builder.build_unconditional_branch(condition_check); 
        
        //Continue
        self.builder.position_at_end(continue_block);
        None
    }
        
    fn generate_while_statement(&self, condition: &Box<Statement>, body: &Vec<Statement>) -> Option<BasicValueEnum> {
        let basic_block = self.builder.get_insert_block()?;
        self.generate_base_while_statement(condition, body);

        let continue_block = self.builder.get_insert_block()?;

        let condition_block = basic_block.get_next_basic_block()?;
        self.builder.position_at_end(basic_block);
        self.builder.build_unconditional_branch(condition_block);

        self.builder.position_at_end(continue_block);
        None
    }

    fn generate_repeat_statement(&self, condition: &Box<Statement>, body: &Vec<Statement>) -> Option<BasicValueEnum> {
        let basic_block = self.builder.get_insert_block()?;
        self.generate_base_while_statement(condition, body);

        let continue_block = self.builder.get_insert_block()?;
        
        let while_block = continue_block.get_previous_basic_block()?;
        self.builder.position_at_end(basic_block);
        self.builder.build_unconditional_branch(while_block);

        self.builder.position_at_end(continue_block);
        None
    }

    fn generate_statement_list(&self, block : BasicBlock, statements:&Vec<Statement>) {
        self.builder.position_at_end(block);
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

    fn generate_call_statement(&self, operator : &Box<Statement>, parameter : &Box<Option<Statement>>) -> Option<BasicValueEnum> {
        //Figure out what the target is
        //Get the function name
        let (param,function) = match &**operator {
            Statement::Reference {name} => (self.get_function_parameter(&name, parameter),self.module.get_function(&name)),
            _ => (None,None),
        };
        //If the target is a function, declare the struct locally
        //Assign all parameters into the struct values
        let call_result = self.builder.build_call(function.unwrap(), &[param.unwrap().as_basic_value_enum()] , "call").try_as_basic_value();
        return call_result.left();
    }
    //Some(LiteralInteger { value: "2" })

    fn get_function_parameter(&self, name : &str, parameters: &Box<Option<Statement>>) -> Option<PointerValue<'ctx>> {
        let variable = self.get_variable(format!("{}_instance",name).as_str());
        println!("{:?}", parameters);
        match &**parameters {
            Some(Statement::ExpressionList{expressions}) => {
                for (index,exp) in expressions.iter().enumerate() {
                    self.generate_single_parameter(exp, index as u32, variable.unwrap());
                }
            },
            Some(statement) => 
                self.generate_single_parameter(statement, 0, variable.unwrap()),
            None =>{},
            
        }
        variable//None
    }

    fn generate_single_parameter(&self, statement : &Statement, index : u32, pointer_value : PointerValue<'ctx>) {
        let generated_exp = self.generate_statement(statement);
        let pointer_to_param = self.builder.build_struct_gep(pointer_value, index as u32, "").unwrap();
        self.builder.build_store(pointer_to_param, generated_exp.unwrap());
    }


    fn generate_lvalue_for(&self, statement: &Box<Statement>) -> Option<PointerValue> {
        match &**statement {
            Statement::Reference {name} => self.generate_lvalue_for_reference(name.as_str()),
            _ => None
        }
    }
    fn get_variable(&self, name: &str) -> Option<PointerValue<'ctx>> {
        if let Some(value) = self.current_parameters.get(name) {
            //Get from local scope
            Some(*value)
        } else {
            //Get from global scope
            self.get_global_variable(name)
        }
    }

    fn get_global_variable(&self, name : &str) -> Option<PointerValue<'ctx>> {
       self.module.get_global(name).map(|var| var.as_basic_value_enum().into_pointer_value()) 
    }

    fn generate_lvalue_for_reference(&self, name: &str) -> Option<PointerValue<'ctx>> {
        self.get_variable(name)
    }

    fn generate_variable_reference(&self, name: &str) -> Option<BasicValueEnum> {
        if let Some(ptr) =  self.generate_lvalue_for_reference(name) {
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
     * ```norun
            %1 = alloca i32, align 4
            %2 = alloca i32, align 4
            store i32 0, i32* %1, align 4
            store i32 7, i32* %2, align 4
            %3 = load i32, i32* %2, align 4
            %4 = add nsw i32 7, %3
            store i32 %4, i32* %2, align 4

            https://github.com/sinato/inkwell-playground/tree/master/examples
        ```
     */

    fn generate_assignment(&self, left: &Box<Statement>, right : &Box<Statement>) -> Option<BasicValueEnum> {
        
        if let Statement::Reference { name } = &**left {
            let left_expr = self.generate_lvalue_for_reference(&name);
            let right_res = self.generate_statement(right);
            self.builder.build_store(left_expr?, right_res?);
        }
        None
    }

    fn generate_literal_number(&self, value: &str) -> Option<BasicValueEnum> {
        let itype = self.context.i32_type();
        let value = itype.const_int_from_string(value, StringRadix::Decimal);
        Some(BasicValueEnum::IntValue(value?))
    }
    
    fn generate_literal_boolean(&self, value: bool) -> Option<BasicValueEnum> {
        let itype = self.context.bool_type();
        let value = itype.const_int(value as u64,false);
        Some(BasicValueEnum::IntValue(value))
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