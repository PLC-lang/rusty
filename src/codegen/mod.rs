/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use super::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::module::Module;

use inkwell::types::{BasicType, BasicTypeEnum, FunctionType, StringRadix, ArrayType, StructType};

use inkwell::values::{
    BasicValue, BasicValueEnum, IntValue, FunctionValue, GlobalValue, PointerValue,
};

use inkwell::AddressSpace;
use inkwell::FloatPredicate;
use inkwell::IntPredicate;

use super::index::*;
use inkwell::basic_block::BasicBlock;

#[cfg(test)]
mod tests;
mod typesystem;

type ExpressionValue<'a> = (Option<DataTypeInformation<'a>>, Option<BasicValueEnum<'a>>);
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub index: &'ctx mut Index<'ctx>,

    scope: Option<String>,
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, index: &'ctx mut Index<'ctx>) -> CodeGen<'ctx> {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let mut codegen = CodeGen {
            context,
            module,
            builder,
            index,
            scope: None,
            current_function: None,
        };
        codegen.initialize_type_system();
        codegen
    }

    fn get_scope(&self) -> Option<&str> {
        self.scope.as_ref().map(|it| it.as_str())
    }

    fn get_struct_instance_name(pou_name: &str) -> String {
        format!("{}_instance", pou_name)
    }

    pub fn generate(&mut self, root: CompilationUnit) -> String {
        self.generate_compilation_unit(root);
        self.module.print_to_string().to_string()
    }

    pub fn generate_compilation_unit(&mut self, root: CompilationUnit) {
        for data_type in &root.types {
            self.generate_data_type_stub(data_type);
        }

        self.generate_data_types(&root.types);

        for global_variables in &root.global_vars {
            self.generate_global_vars(global_variables);
        }

        for unit in &root.units {
            let struct_name = format!("{}_interface", &unit.name);
            let struct_type = self.context.opaque_struct_type(struct_name.as_str());
            self.index.associate_type(
                &unit.name,
                DataTypeInformation::Struct {
                    name: struct_name,
                    generated_type: struct_type.into(),
                },
            );
        }

        for unit in &root.units {
            self.generate_pou(unit);
        }
    }

    fn generate_data_type_stub(&mut self, data_type: &DataType) {
        match data_type {
            DataType::StructType { name, variables: _ } => {
                self.index.associate_type(
                    name.as_ref().unwrap().as_str(),
                    DataTypeInformation::Struct {
                        name: name.clone().unwrap(),
                        generated_type: self
                            .context
                            .opaque_struct_type(name.as_ref().unwrap())
                            .into(),
                    },
                );
            }
            DataType::EnumType { name, elements: _ } => self.index.associate_type(
                name.as_ref().unwrap().as_str(),
                DataTypeInformation::Integer {
                    signed: true,
                    size: 32,
                    generated_type: self.context.i32_type().as_basic_type_enum(),
                },
            ),
            DataType::SubRangeType { .. } => unimplemented!(),
            DataType::ArrayType { name , bounds, referenced_type} => {
                let dimensions = CodeGen::get_array_dimensions(bounds);
                let target_type = self.get_type(referenced_type).unwrap();
                let internal_type = self.index.find_type_information(referenced_type.get_name().unwrap()).unwrap();
                self.index.associate_type(name.as_ref().unwrap().as_str(), 
                    DataTypeInformation::Array {
                        name: name.as_ref().unwrap().clone(),
                        internal_type_information : Box::new(internal_type),
                        generated_type : CodeGen::create_nested_array_type(target_type, dimensions.clone()).as_basic_type_enum(),
                        dimensions,
                    }
                )
            },
        };
    }

    fn get_array_dimensions(bounds : &Statement) -> Vec<Dimension> {
        let mut result = vec!();
        for statement in bounds.get_as_list() {
            result.push(CodeGen::get_single_array_dimension(statement).expect("Could not parse array bounds"));
        }
        result
    }

    fn get_single_array_dimension(bounds : &Statement) -> Result<Dimension, String> {
        if let Statement::RangeStatement{start, end} = bounds {
            let start_offset = CodeGen::evaluate_constant_int(start).unwrap_or(0);
            let end_offset = CodeGen::evaluate_constant_int(end).unwrap_or(0);
            Ok(Dimension{start_offset, end_offset}) 
        } else {
            Err(format!("Unexpected Statement {:?}, expected range", bounds))
        }
    }

    fn extract_value(s : &Statement) -> Option<String> {
        match s {
            Statement::UnaryExpression {operator, value , ..} => 
            CodeGen::extract_value(value).map(|result| format!("{}{}", operator, result)),
            Statement::LiteralInteger { value, ..} => Some(value.to_string()), 
            //TODO constatnts
            _ => None,
        }

    }

    fn evaluate_constant_int(s : &Statement) -> Option<i32>{
        let value = CodeGen::extract_value(s);
        value.map(|v| v.parse().unwrap_or(0))
    }

    fn get_type(&self, data_type: &DataTypeDeclaration) -> Option<BasicTypeEnum<'ctx>> {
        data_type
            .get_name()
            .and_then(|name| 
                self.index.find_type(name).map(|it| 
                    it.get_type()
                ).flatten()
            )
    }

    fn create_nested_array_type(end_type : BasicTypeEnum, dimensions : Vec<Dimension> ) -> ArrayType {
        let mut result : Option<ArrayType> = None;
        let mut current_type = end_type;
        for dimension in dimensions.iter().rev() {
            result = Some(CodeGen::create_array_type(current_type, dimension.get_length()));
            current_type = result.unwrap().into();
        }
        result.unwrap()
    }

    fn create_array_type(basic_type : BasicTypeEnum, size : u32) -> ArrayType {
        match basic_type {
            BasicTypeEnum::IntType(..) => basic_type.into_int_type().array_type(size),
            BasicTypeEnum::FloatType(..)  => basic_type.into_float_type().array_type(size),
            BasicTypeEnum::StructType(..)  => basic_type.into_struct_type().array_type(size),
            BasicTypeEnum::ArrayType(..) => basic_type.into_array_type().array_type(size),
            BasicTypeEnum::PointerType(..) => basic_type.into_pointer_type().array_type(size),
            BasicTypeEnum::VectorType(..) => basic_type.into_vector_type().array_type(size),
        }
    }

    fn generate_data_types(&mut self, data_types: &Vec<DataType>) {
        for data_type in data_types {
            match data_type {
                DataType::StructType { name, variables } => {
                    let members = self.get_variables_information(&variables);
                    self.generate_instance_struct(&members, name.as_ref().unwrap().as_str());
                }
                DataType::EnumType { name: _, elements } => {
                    for (i, element) in elements.iter().enumerate() {
                        let int_type = self.context.i32_type();
                        let element_variable =
                            self.generate_global_variable(int_type.as_basic_type_enum(), element);
                        element_variable.set_initializer(&int_type.const_int(i as u64, false));
                        
                        //associate the enum element's global variable
                        self.index.associate_global_variable(element, element_variable.as_pointer_value());
                    }
                }
                DataType::SubRangeType { .. } => unimplemented!(),
                DataType::ArrayType { .. } => {},
            }
        }
    }

    fn get_function_type(
        &self,
        parameters: &[BasicTypeEnum<'ctx>],
        enum_type: Option<BasicTypeEnum<'ctx>>,
    ) -> FunctionType<'ctx> {
            match enum_type {
                Some(enum_type) if enum_type.is_int_type() =>  enum_type.into_int_type().fn_type(parameters, false),
                Some(enum_type) if enum_type.is_float_type() =>  enum_type.into_float_type().fn_type(parameters, false),
                Some(enum_type) if enum_type.is_array_type() => enum_type.into_array_type().fn_type(parameters,false),
                None => self.context.void_type().fn_type(parameters, false), 
                _ => panic!(format!("Unsupported return type {:?}", enum_type))
            }
    }

    fn generate_global_vars(&mut self, global_vars: &VariableBlock) {
        let members = self.get_variables_information(&global_vars.variables);
        for (name, var_type) in members {
            let global_value = self.generate_global_variable(var_type, &name);
            self.index
                .associate_global_variable(name.as_str(), global_value.as_pointer_value());
        }
    }

    fn generate_pou(&mut self, p: &POU) {
        self.scope = Some(p.name.clone());

        let mut pou_members: Vec<(String, BasicTypeEnum)> = Vec::new();
        let return_type = p
            .return_type
            .as_ref()
            .map(|return_type| self.get_type(return_type))
            .flatten();

        for var_block in &p.variable_blocks {
            let mut members = self.get_variables_information(&var_block.variables);
            pou_members.append(&mut members);
        }

        //Create a struct with the value from the program
        let member_type = self.generate_instance_struct(&pou_members, &p.name);

        let member_type_ptr = member_type.ptr_type(AddressSpace::Generic);
        //let return_type = self.context.i32_type();
        let f_type = self.get_function_type(&[member_type_ptr.as_basic_type_enum()], return_type);
        self.current_function = Some(self.module.add_function(
            self.get_scope().unwrap(),
            f_type,
            None,
        ));
        self.index
            .associate_callable_implementation(p.name.as_str(), self.current_function.unwrap());
        
        //Create An instance variable for that struct
        //Place in global data
        if p.pou_type == PouType::Program {
            let instance_name = CodeGen::get_struct_instance_name(p.name.as_str());
            let global_value =
                self.generate_global_variable(member_type.into(), instance_name.as_str());
            self.index
                .associate_global_variable(p.name.as_str(), global_value.as_pointer_value());
        }

        //Don't generate external functions
        if p.linkage == LinkageType::External {
            return; 
        }

        let block = self
            .context
            .append_basic_block(self.current_function.unwrap(), "entry");


        //let mut result = None;
        //Generate reference to parameter
        self.builder.position_at_end(block);
        for (i, m) in pou_members.iter().enumerate() {
            let parameter_name = &m.0;
            let ptr_value = self
                .current_function
                .unwrap()
                .get_first_param()
                .unwrap()
                .into_pointer_value();
            self.index.associate_local_variable(
                p.name.as_str(),
                parameter_name,
                self.builder
                    .build_struct_gep(ptr_value, i as u32, &parameter_name)
                    .unwrap(),
            )
        }
        //Insert return variable
        if let Some(ret_type) = return_type {
            let ret_alloc = self.builder.build_alloca(ret_type, p.name.as_str());
            self.index
                .associate_local_variable(p.name.as_str(), p.name.as_str(), ret_alloc);
        }

        self.generate_statement_list(block, &p.statements);
        //self.builder.build_return(Some(&result.unwrap()));i
        let ret_value = self.get_return_value(p.pou_type);
        if let Some(ret_value) = ret_value {
            self.builder.build_return(Some(&ret_value));
        } else {
            self.builder.build_return(None);
        };
    }

    fn get_return_value(&self, pou_type: PouType) -> Option<BasicValueEnum> {
        match pou_type {
            PouType::Function => {
                let pou_name = self.get_scope().unwrap();
                let value = self
                    .generate_lvalue_for_reference(&[pou_name.to_string()]).unwrap()
                    .1
                    .unwrap();
                Some(
                    self.builder
                        .build_load(value, format!("{}_ret", pou_name).as_str()),
                )
            }
            _ => None,
        }
    }

    fn get_variables_information(
        &self,
        variables: &Vec<Variable>,
    ) -> Vec<(String, BasicTypeEnum<'ctx>)> {
        let mut types: Vec<(String, BasicTypeEnum<'ctx>)> = Vec::new();
        for variable in variables {
            let var_type = self.get_type(&variable.data_type).unwrap();
            types.push((variable.name.clone(), var_type));
        }
        types
    }

    fn generate_instance_struct(
        &self,
        members: &Vec<(String, BasicTypeEnum)>,
        name: &str,
    ) -> StructType<'ctx> {
        let struct_type = self
            .index
            .find_type(name)
            .unwrap()
            .get_type()
            .unwrap()
            .into_struct_type();
        let member_types: Vec<BasicTypeEnum> = members.iter().map(|(_, t)| *t).collect();
        struct_type.set_body(member_types.as_slice(), false);
        struct_type
    }
    fn set_initializer_for_type(
        &self,
        global_value: &GlobalValue<'ctx>,
        variable_type: BasicTypeEnum<'ctx>,
    ) {
        if variable_type.is_int_type() {
            global_value.set_initializer(&variable_type.into_int_type().const_zero());
        } else if variable_type.is_struct_type() {
            global_value.set_initializer(&variable_type.into_struct_type().const_zero());
        }
    }

    fn generate_global_variable(
        &self,
        variable_type: BasicTypeEnum<'ctx>,
        name: &str,
    ) -> GlobalValue<'ctx> {
        let result = self
            .module
            .add_global(variable_type, Some(AddressSpace::Generic), name);
        self.set_initializer_for_type(&result, variable_type);
        result.set_thread_local_mode(None);
        result.set_linkage(Linkage::External);
        result
    }
    fn generate_statement(&self, s: &Statement) -> ExpressionValue<'ctx> {
        match s {
            Statement::IfStatement { blocks, else_block, ..} => {
                (None, self.generate_if_statement(blocks, else_block))
            }
            Statement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => (
                None,
                self.generate_case_statement(selector, case_blocks, else_block),
            ),
            //Loops
            Statement::ForLoopStatement {
                counter,
                start,
                end,
                by_step,
                body,
                ..
            } => (
                None,
                self.generate_for_statement(counter, start, end, by_step, body),
            ),
            Statement::WhileLoopStatement { condition, body, .. } => {
                (None, self.generate_while_statement(condition, body))
            }
            Statement::RepeatLoopStatement { condition, body, .. } => {
                (None, self.generate_repeat_statement(condition, body))
            }
            //Expressions
            Statement::BinaryExpression {
                operator,
                left,
                right, ..
            } => self.generate_binary_expression(operator, left, right),
            Statement::LiteralInteger { value, location: _ } => self.generate_literal_integer(value.as_str()),
            Statement::LiteralReal { value, location: _  } => self.generate_literal_real(value.as_str()),
            Statement::LiteralBool { value, location: _ } => self.generate_literal_boolean(*value),
            Statement::LiteralString { value, location: _ } => self.generate_literal_string(value.as_bytes()),
            Statement::Reference { elements, location: _} => self.generate_variable_reference(&elements),
            Statement::Assignment { left, right } => {
                (None, self.generate_assignment(&left, &right))
            }
            Statement::UnaryExpression { operator, value , ..} => {
                self.generate_unary_expression(&operator, &value)
            }
            Statement::CallStatement {
                operator,
                parameters,
                ..
            } => self.generate_call_statement(&operator, &parameters),
            Statement::ArrayAccess {reference, access} => self.generate_array_access(&reference, &access),
            _ => panic!("{:?} not yet supported", s),
        }
    }

    fn generate_array_access(
        &self,
        reference: &Box<Statement>,
        access: &Box<Statement>,
    ) -> ExpressionValue<'ctx>{
        //Generate Reference
        let value = self.generate_lvalue_for_array(reference, access).unwrap();
        self.generate_reference_from_value(value,"tmpVar")
    }

    fn generate_case_statement(
        &self,
        selector: &Box<Statement>,
        conditional_blocks: &Vec<ConditionalBlock>,
        else_body: &Vec<Statement>,
    ) -> Option<BasicValueEnum<'ctx>> {
        //Continue
        let continue_block = self
            .context
            .append_basic_block(self.current_function?, "continue");

        let basic_block = self.builder.get_insert_block()?;
        let selector_statement = self.generate_statement(&*selector).1?;
        let mut cases = Vec::new();

        //generate a int_value and a BasicBlock for every case-body
        for i in 0..conditional_blocks.len() {
            let conditional_block = &conditional_blocks[i];
            let basic_block = self
                .context
                .append_basic_block(self.current_function?, "case");
            let condition = self.generate_statement(&*conditional_block.condition).1?; //TODO : Is a type conversion needed here?
            self.generate_statement_list(basic_block, &conditional_block.body);
            self.builder.build_unconditional_branch(continue_block);

            cases.push((condition.into_int_value(), basic_block));
        }

        let else_block = self
            .context
            .append_basic_block(self.current_function?, "else");
        self.generate_statement_list(else_block, else_body);
        self.builder.build_unconditional_branch(continue_block);

        //Move the continue block to after the else block
        continue_block.move_after(else_block).unwrap();
        //Position in initial block
        self.builder.position_at_end(basic_block);
        self.builder
            .build_switch(selector_statement.into_int_value(), else_block, &cases);
        self.builder.position_at_end(continue_block);
        None
    }

    fn generate_if_statement(
        &self,
        conditional_blocks: &Vec<ConditionalBlock>,
        else_body: &Vec<Statement>,
    ) -> Option<BasicValueEnum<'ctx>> {
        let mut blocks = Vec::new();
        blocks.push(self.builder.get_insert_block().unwrap());
        for _ in 1..conditional_blocks.len() {
            blocks.push(
                self.context
                    .append_basic_block(self.current_function?, "branch"),
            );
        }

        let else_block = if else_body.len() > 0 {
            let result = self
                .context
                .append_basic_block(self.current_function?, "else");
            blocks.push(result);
            Some(result)
        } else {
            None
        };
        //Continue
        let continue_block = self
            .context
            .append_basic_block(self.current_function?, "continue");
        blocks.push(continue_block);

        for (i, block) in conditional_blocks.iter().enumerate() {
            let then_block = blocks[i];
            let else_block = blocks[i + 1];

            self.builder.position_at_end(then_block);

            let condition = self
                .generate_statement(&block.condition)
                .1?
                .into_int_value();
            let conditional_block = self
                .context
                .prepend_basic_block(else_block, "condition_body");

            //Generate if statement condition
            self.builder
                .build_conditional_branch(condition, conditional_block, else_block);

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

    fn generate_for_statement(
        &self,
        counter: &Box<Statement>,
        start: &Box<Statement>,
        end: &Box<Statement>,
        by_step: &Option<Box<Statement>>,
        body: &Vec<Statement>,
    ) -> Option<BasicValueEnum<'ctx>> {
        self.generate_assignment(counter, start);
        let condition_check = self
            .context
            .append_basic_block(self.current_function?, "condition_check");
        let for_body = self
            .context
            .append_basic_block(self.current_function?, "for_body");
        let continue_block = self
            .context
            .append_basic_block(self.current_function?, "continue");
        //Generate an initial jump to the for condition
        self.builder.build_unconditional_branch(condition_check);

        //Check loop condition
        self.builder.position_at_end(condition_check);
        let counter_statement = self.generate_statement(counter).1.unwrap().into_int_value();
        let end_statement = self.generate_statement(end).1.unwrap().into_int_value();
        let compare = self.builder.build_int_compare(
            IntPredicate::SLE,
            counter_statement,
            end_statement,
            "tmpVar",
        );
        self.builder
            .build_conditional_branch(compare, for_body, continue_block);

        //Enter the for loop
        self.generate_statement_list(for_body, &body);

        //Increment
        let step_by_value = by_step
            .as_ref()
            .map(|step| self.generate_statement(&step).1.unwrap())
            .or(self.generate_literal_integer("1").1)
            .unwrap()
            .into_int_value();

        let next = self
            .builder
            .build_int_add(counter_statement, step_by_value, "tmpVar");
        let ptr = self.generate_lvalue_for(counter).expect("Cannot generate lvalue").1.unwrap();
        self.builder.build_store(ptr, next);

        //Loop back
        self.builder.build_unconditional_branch(condition_check);

        //Continue
        self.builder.position_at_end(continue_block);
        None
    }

    fn generate_base_while_statement(
        &self,
        condition: &Box<Statement>,
        body: &Vec<Statement>,
    ) -> Option<BasicValueEnum> {
        let condition_check = self
            .context
            .append_basic_block(self.current_function?, "condition_check");
        let while_body = self
            .context
            .append_basic_block(self.current_function?, "while_body");
        let continue_block = self
            .context
            .append_basic_block(self.current_function?, "continue");

        //Check loop condition
        self.builder.position_at_end(condition_check);
        let condition_value = self.generate_statement(condition).1?.into_int_value();
        self.builder
            .build_conditional_branch(condition_value, while_body, continue_block);

        //Enter the for loop
        self.generate_statement_list(while_body, &body);
        //Loop back
        self.builder.build_unconditional_branch(condition_check);

        //Continue
        self.builder.position_at_end(continue_block);
        None
    }

    fn generate_while_statement(
        &self,
        condition: &Box<Statement>,
        body: &Vec<Statement>,
    ) -> Option<BasicValueEnum<'ctx>> {
        let basic_block = self.builder.get_insert_block()?;
        self.generate_base_while_statement(condition, body);

        let continue_block = self.builder.get_insert_block()?;

        let condition_block = basic_block.get_next_basic_block()?;
        self.builder.position_at_end(basic_block);
        self.builder.build_unconditional_branch(condition_block);

        self.builder.position_at_end(continue_block);
        None
    }

    fn generate_repeat_statement(
        &self,
        condition: &Box<Statement>,
        body: &Vec<Statement>,
    ) -> Option<BasicValueEnum<'ctx>> {
        let basic_block = self.builder.get_insert_block()?;
        self.generate_base_while_statement(condition, body);

        let continue_block = self.builder.get_insert_block()?;

        let while_block = continue_block.get_previous_basic_block()?;
        self.builder.position_at_end(basic_block);
        self.builder.build_unconditional_branch(while_block);

        self.builder.position_at_end(continue_block);
        None
    }

    fn generate_statement_list(&self, block: BasicBlock, statements: &Vec<Statement>) {
        self.builder.position_at_end(block);
        for statement in statements {
            self.generate_statement(statement);
        }
    }

    fn generate_unary_expression(
        &self,
        operator: &Operator,
        value: &Box<Statement>,
    ) -> ExpressionValue<'ctx> {
        if let (Some(data_type), Some(loaded_value)) = self.generate_statement(value) {
            let (data_type, value) = match operator {
                Operator::Not => (
                    data_type,
                    self.builder
                        .build_not(loaded_value.into_int_value(), "tmpVar"),
                ),
                Operator::Minus => (
                    data_type,
                    self.builder
                        .build_int_neg(loaded_value.into_int_value(), "tmpVar"),
                ),
                _ => unimplemented!(),
            };
            (Some(data_type), Some(BasicValueEnum::IntValue(value)))
        } else {
            (None, None)
        }
    }

    /**
     *
     * FUNCTION foo : INT;
     * END_FUNCTION
     * PROGRAM a
     * VAR
     * foo : INT
     * END_VAR
     * foo();
     * END_PROGRAM
     *
     */

    fn get_callable_type_instance(
        &self,
        expressions: &Vec<String>,
    ) -> Option<&VariableIndexEntry<'ctx>> {
        self.index
            .find_callable_instance_variable(self.get_scope(), &expressions)
    }

    fn allocate_variable(&self, data_type: &str) -> Option<PointerValue<'ctx>> {
        let instance_name = CodeGen::get_struct_instance_name(data_type);
        let function_type = self.index.find_type(data_type).unwrap().get_type(); //TODO Store as datatype in the index and fetch it?
        Some(
            self.builder
                .build_alloca(function_type.unwrap(), instance_name.as_str()),
        )
    }

    fn generate_call_statement(
        &self,
        operator: &Box<Statement>,
        parameter: &Box<Option<Statement>>,
    ) -> ExpressionValue<'ctx> {
        //Figure out what the target is
        //Get the function name
        let (variable, index_entry) = match &**operator {
            Statement::Reference { elements, .. } => {
                //Get associated Variable or generate a variable for the type with the same name
                let ast_variable = self.get_callable_type_instance(&elements); //Look for the instance variable
                let variable_instance = ast_variable
                    .map(|it| it.get_generated_reference())
                    .flatten() //look for the generated parameters-struct
                    .or_else(|| self.allocate_variable(&elements[0])); //there is no generated parameters-struct --> function call!
                                                                       //Get Function from Datatype
                let call_name = ast_variable
                    .map(|it| it.get_type_name()) // we called f() --> look for f's datatype
                    .or(Some(&elements[0])); // we didnt call a variable ([0so we treat the string as the function's name
                let index_entry = self.index.find_type(call_name.unwrap());
                (variable_instance, index_entry)
            }
            _ => (None, None),
        };
        let instance = variable.unwrap();
        let index_entry = index_entry;
        let function_name = index_entry.map(DataTypeIndexEntry::get_name).unwrap();
        //Create parameters for input and output blocks
        let input_block = self.context.append_basic_block(self.current_function.unwrap(), "input");
        let call_block = self.context.append_basic_block(self.current_function.unwrap(), "call");
        let output_block = self.context.append_basic_block(self.current_function.unwrap(), "output");
        let continue_block = self.context.append_basic_block(self.current_function.unwrap(), "continue");
        //First go to the input block
        self.builder.build_unconditional_branch(input_block);
        self.builder.position_at_end(input_block);
        //Generate all parameters, this function may jump to the output block
        self.generate_function_parameters(function_name, instance, parameter,&input_block, &output_block);
        //Generate the label jumps from input to call to output 
        self.builder.build_unconditional_branch(call_block);
        self.builder.position_at_end(output_block);
        self.builder.build_unconditional_branch(continue_block);
        self.builder.position_at_end(call_block);
        let return_type = self
            .index
            .find_member(function_name, function_name)
            .map(VariableIndexEntry::get_type_name)
            .and_then(|it| self.index.find_type_information(it));
        let function = index_entry
            .map(|it| it.get_implementation())
            .flatten()
            .unwrap();
        let call_result = self
        //If the target is a function, declare the struct locally
        //Assign all parameters into the struct values
            .builder
            .build_call(function, &[instance.as_basic_value_enum()], "call")
            .try_as_basic_value();
        self.builder.build_unconditional_branch(output_block);
        //Continue here after function call
        self.builder.position_at_end(continue_block);
        return (return_type, call_result.left());
    }
    //Some(LiteralInteger { value: "2" })

    fn generate_function_parameters(
        &self,
        function_name: &str,
        variable: PointerValue<'ctx>,
        parameters: &Box<Option<Statement>>,
        input_block : &BasicBlock,
        output_block : &BasicBlock,
    ) {
        match &**parameters {
            Some(Statement::ExpressionList { expressions }) => {
                for (index, exp) in expressions.iter().enumerate() {
                    self.generate_single_parameter(exp, function_name, None, index as u32, variable,input_block, output_block);
                }
            }
            Some(statement) => {
                self.generate_single_parameter(statement, function_name, None, 0, variable,input_block, output_block)
            }
            None => {}
        }
    }

    fn generate_single_parameter(
        &self,
        statement: &Statement,
        function_name: &str,
        parameter_type : Option<&DataTypeIndexEntry<'ctx>>,
        index: u32,
        pointer_value: PointerValue<'ctx>,
        input_block : &BasicBlock,
        output_block : &BasicBlock,
    ) {
        match statement {
            Statement::Assignment { left, right } => {
                self.builder.position_at_end(*input_block);
                if let Statement::Reference { elements, ..} = &**left {
                    let parameter = self
                        .index
                        .find_member(function_name, &elements.join("."))
                        .unwrap();
                    let index = parameter
                        .get_location_in_parent()
                        .unwrap();
                    let param_type = self.index.find_type(parameter.get_type_name());
                    self.generate_single_parameter(right, function_name, param_type, index, pointer_value, input_block, output_block);
                }
            }
            Statement::OutputAssignment { left, right } => {
                let current_block = self.builder.get_insert_block().unwrap();
                self.builder.position_at_end(*output_block);
                if let Statement::Reference { elements, ..} = &**left {
                    let parameter = self
                        .index
                        .find_member(function_name, &elements.join("."))
                        .unwrap();
                    let index = parameter
                        .get_location_in_parent()
                        .unwrap();
                    let param_type = self.index.find_type(parameter.get_type_name()).or_else(|| 
                        self.index.find_input_parameter(function_name, index as u32).and_then(|var| self.index.find_type(var.get_type_name()))).and_then(|var| var.get_type_information()).unwrap();
                    //load the function prameter
                    let pointer_to_param = self
                        .builder
                        .build_struct_gep(pointer_value, index as u32, "")
                        .unwrap();
                    let (target_type,pointer_to_target) = self.generate_lvalue_for(right).unwrap();
                    let loaded_value = self.builder.build_load(pointer_to_param,parameter.get_name());
                    let value = self.cast_if_needed(&target_type.unwrap(),loaded_value,param_type).unwrap();
                    self.builder
                        .build_store(pointer_to_target.unwrap(), value);
                }
                self.builder.position_at_end(current_block);
            }
            _ => {
                if let (Some(value_type), Some(generated_exp)) = self.generate_statement(statement) {
                    let pointer_to_param = self
                        .builder
                        .build_struct_gep(pointer_value, index as u32, "")
                        .unwrap();
                    let parameter = parameter_type.or_else(|| 
                        self.index.find_input_parameter(function_name, index as u32).and_then(|var| self.index.find_type(var.get_type_name()))).and_then(|var| var.get_type_information()).unwrap();
                    let value = self.cast_if_needed(parameter, generated_exp, &value_type);
                    self.builder
                        .build_store(pointer_to_param, value.unwrap());
                }
            }
        }
    }

    fn generate_access_for_dimension(
        &self,
        dimension : &Dimension,
        access_statement  : &Statement,
    ) -> Result<IntValue<'ctx>, String> {
        let start_offset = dimension.start_offset;
        if let (_ , Some(access_value)) = self.generate_statement(access_statement) {
                //If start offset is not 0, adjust the current statement with an add operation
                if start_offset != 0 {
                    Ok(self.builder.build_int_sub(access_value.into_int_value(), self.context.i32_type().const_int(start_offset as u64, true), ""))
                } else {
                    Ok(access_value.into_int_value())
                }
        } else {
            Err(format!("Cannot generate int access for {:?}", access_statement))
        }
    }
    
    fn generate_lvalue_for_array(
        &self,
        reference: &Box<Statement>,
        access: &Box<Statement>,
    ) -> Result<(Option<DataTypeInformation<'ctx>>, Option<PointerValue<'ctx>>), String> {
        //Load the reference
        self.generate_lvalue_for(reference).and_then(|lvalue| {
            if let (Some(DataTypeInformation::Array{internal_type_information,dimensions,  ..}), Some(value)) =  lvalue {

                //First 0 is to access the pointer, then we access the array
                let mut indices = vec![self.context.i32_type().const_int(0,false)];
                
                //Make sure dimensions match statement list
                let statements = access.get_as_list();
                if statements.len() == 0 || statements.len() != dimensions.len() {
                    panic!("Mismatched array access : {} -> {} ", statements.len(), dimensions.len())
                } 
                for (i, statement) in statements.iter().enumerate() {
                    indices.push(self.generate_access_for_dimension(&dimensions[i], statement).expect("Cannot read array index"))
                }
                //Load the access from that reference
                let pointer = unsafe {
                    self.builder.build_in_bounds_gep(value, indices.as_slice(), "tmpVar")
                };
                return Ok((Some(*internal_type_information),Some(pointer)));
            }
            Err(format!("Could not generate Array Value {:?}[{:?}", reference,access))
        })
    }

    fn generate_lvalue_for(
        &self,
        statement: &Box<Statement>,
    ) -> Result<(Option<DataTypeInformation<'ctx>>, Option<PointerValue<'ctx>>), String> {
        match &**statement {
            Statement::Reference { elements , ..} => self.generate_lvalue_for_reference(elements),
            Statement::ArrayAccess {reference, access } => self.generate_lvalue_for_array(reference, access),
            _ => Err(format!("Unsupported Statement {:?}", statement)),
        }
    }

    fn get_variable(&self, name: &[String]) -> Option<PointerValue<'ctx>> {
        self.index
            .find_variable(self.get_scope(), name)
            .map(|e| e.get_generated_reference())
            .flatten()
    }

    fn generate_lvalue_for_reference(
        &self,
        segments: &[String],
    ) -> Result<(
        Option<DataTypeInformation<'ctx>>,
        Option<PointerValue<'ctx>>,
    ), String> {
        let mut name = segments.iter();
        let first_name = name.next().unwrap();
        let type_name = self
            .index
            .find_variable(self.get_scope(), &[first_name.clone()])
            .unwrap()
            .get_type_name();
        let first_ptr = (type_name, self.get_variable(&[first_name.to_string()]));

        let (data_type, ptr) = name.fold(first_ptr, |qualifier, operator| {
            if let (qualifier_name, Some(qualifier)) = qualifier {
                let member = self.index.find_member(qualifier_name, operator);
                let member_location = member
                    .map(|it| it.get_location_in_parent())
                    .flatten()
                    .unwrap();
                let member_data_type = member.map(|it| it.get_type_name()).unwrap();
                let gep = self
                    .builder
                    .build_struct_gep(qualifier, member_location, operator);
                (member_data_type, gep.ok())
            } else {
                ("",None)
            }
        });
        ptr.map(|it| (self.index.find_type_information(data_type),Some(it))).ok_or(format!("Could not generate reference for {}", segments.join(".")))
    }

    fn generate_reference_from_value(&self, lvalue: (Option<DataTypeInformation<'ctx>>, Option<PointerValue<'ctx>>), name : &str) -> ExpressionValue<'ctx> {
        let (data_type, ptr) = lvalue;

        let temp_var_name = format!("load_{var_name}", var_name = name);
        (
            data_type,
            ptr.map(|value| (self.builder.build_load(value, &temp_var_name).into())),
        )
    }

    fn generate_variable_reference(&self, segments: &[String]) -> ExpressionValue<'ctx> {
        let lvalue = self.generate_lvalue_for_reference(segments).unwrap();

        self.generate_reference_from_value(lvalue, &segments.join("."))

    }

    fn generate_assignment(
        &self,
        left: &Box<Statement>,
        right: &Box<Statement>,
    ) -> Option<BasicValueEnum<'ctx>> {
        if let (Some(left_type), Some(left_expr)) = self.generate_lvalue_for(left).unwrap()
        {
            if let (Some(right_type), Some(right_res)) = self.generate_statement(right) {
                let value = self
                    .cast_if_needed(&left_type, right_res, &right_type).unwrap();
                self.builder.build_store(left_expr, value);
            }
        };
        None
    }

    fn generate_literal_integer(&self, value: &str) -> ExpressionValue<'ctx> {
        let itype = self.context.i32_type();
        let value = itype.const_int_from_string(value, StringRadix::Decimal);
        let data_type = self.index.find_type_information("DINT");
        (data_type, Some(BasicValueEnum::IntValue(value.unwrap())))
    }

    fn generate_literal_real(&self, value: &str) -> ExpressionValue<'ctx> {
        let itype = self.context.f32_type();
        let value = itype.const_float_from_string(value);
        let data_type = self.index.find_type_information("REAL");
        (data_type, Some(BasicValueEnum::FloatValue(value)))
    }

    fn generate_literal_boolean(&self, value: bool) -> ExpressionValue<'ctx> {
        let itype = self.context.bool_type();
        let value = itype.const_int(value as u64, false);
        let data_type = self.index.find_type_information("BOOL");
        (data_type, Some(BasicValueEnum::IntValue(value)))
    }
    
    fn generate_literal_string(&self, value: &[u8]) -> ExpressionValue<'ctx> {
        let exp_value = self.context.const_string(value , true);
        (
            Some(self.new_string_information(value.len() as u32)), 
            Some(exp_value.into())
        )
    }

    fn generate_phi_expression(
        &self, 
        operator: &Operator, 
        left: &Box<Statement>, 
        right: &Box<Statement>
    ) -> ExpressionValue<'ctx>{
        let right_branch = self.context.append_basic_block(self.current_function.unwrap(), "");
        let continue_branch = self.context.append_basic_block(self.current_function.unwrap(),"");

        let (left_type, left_value) = self.generate_statement(left);
        let final_left_block = self.builder.get_insert_block().unwrap();
        //Compare left to 0
        let lhs = self.builder.build_int_compare(IntPredicate::NE, left_value.unwrap().into_int_value(), left_type.as_ref().unwrap().get_type().into_int_type().const_int(0,false), "");
        match operator {
            Operator::Or => self.builder.build_conditional_branch(lhs,continue_branch,right_branch),
            Operator::And => self.builder.build_conditional_branch(lhs,right_branch,continue_branch),
            _ => unreachable!() 
        };

        self.builder.position_at_end(right_branch);
        let (right_type, right_value) = self.generate_statement(right);
        let final_right_block = self.builder.get_insert_block().unwrap();
        let rhs = right_value.unwrap();
        self.builder.build_unconditional_branch(continue_branch);

        self.builder.position_at_end(continue_branch);
        //Generate phi
        let target_type = if left_type.as_ref().unwrap().get_size() > right_type.as_ref().unwrap().get_size() { left_type } else { right_type };
        let phi_value = self.builder.build_phi(target_type.as_ref().unwrap().get_type(),"");
        phi_value.add_incoming(&[(&left_value.unwrap().into_int_value(),final_left_block), (&rhs,final_right_block)]);


        (target_type,Some(phi_value.as_basic_value()))
    }

    fn generate_binary_expression(
        &self,
        operator: &Operator,
        left: &Box<Statement>,
        right: &Box<Statement>,
    ) -> ExpressionValue<'ctx> {
        //If OR, or AND handle before generating the statements
        match operator {
            Operator::And | Operator::Or => 
                return self.generate_phi_expression(operator, left, right),
            _ => {}
        }

        if let (Some(ltype), Some(lval_opt)) = self.generate_statement(left) {
            if let (Some(rtype), Some(rval_opt)) = self.generate_statement(right) {
                //Step 1 convert all to i32
                let (target_type, lvalue, rvalue) =
                    self.promote_if_needed(lval_opt, &ltype, rval_opt, &rtype);
                let (value, target_type) = match target_type {
                    DataTypeInformation::Integer { .. } => {
                        self.generate_int_binary_expression(operator, lvalue, rvalue, &target_type)
                    }
                    DataTypeInformation::Float { .. } => self.generate_float_binary_expression(
                        operator,
                        lvalue,
                        rvalue,
                        &target_type,
                    ),
                    _ => unimplemented!(),
                };
                return (Some(target_type), Some(value));
            }
        }
        (None, None)
    }

    fn generate_int_binary_expression(
        &self,
        operator: &Operator,
        lvalue: BasicValueEnum<'ctx>,
        rvalue: BasicValueEnum<'ctx>,
        target_type: &DataTypeInformation<'ctx>,
    ) -> (BasicValueEnum<'ctx>, DataTypeInformation<'ctx>) {
        let int_lvalue = lvalue.into_int_value();
        let int_rvalue = rvalue.into_int_value();

        match operator {
            Operator::Plus => (
                self.builder
                    .build_int_add(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Minus => (
                self.builder
                    .build_int_sub(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Multiplication => (
                self.builder
                    .build_int_mul(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Division => (
                self.builder
                    .build_int_signed_div(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Modulo => (
                self.builder
                    .build_int_signed_rem(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Equal => (
                self.builder
                    .build_int_compare(IntPredicate::EQ, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::NotEqual => (
                self.builder
                    .build_int_compare(IntPredicate::NE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::Less => (
                self.builder
                    .build_int_compare(IntPredicate::SLT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::Greater => (
                self.builder
                    .build_int_compare(IntPredicate::SGT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::LessOrEqual => (
                self.builder
                    .build_int_compare(IntPredicate::SLE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::GreaterOrEqual => (
                self.builder
                    .build_int_compare(IntPredicate::SGE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),
            Operator::Xor => (
                self.builder
                    .build_xor(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),
            _ => unimplemented!(),
        }
    }

    fn generate_float_binary_expression(
        &self,
        operator: &Operator,
        lvalue: BasicValueEnum<'ctx>,
        rvalue: BasicValueEnum<'ctx>,
        target_type: &DataTypeInformation<'ctx>,
    ) -> (BasicValueEnum<'ctx>, DataTypeInformation<'ctx>) {
        let int_lvalue = lvalue.into_float_value();
        let int_rvalue = rvalue.into_float_value();

        match operator {
            Operator::Plus => (
                self.builder
                    .build_float_add(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Minus => (
                self.builder
                    .build_float_sub(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Multiplication => (
                self.builder
                    .build_float_mul(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Division => (
                self.builder
                    .build_float_div(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Modulo => (
                self.builder
                    .build_float_rem(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Equal => (
                self.builder
                    .build_float_compare(FloatPredicate::OEQ, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::NotEqual => (
                self.builder
                    .build_float_compare(FloatPredicate::ONE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::Less => (
                self.builder
                    .build_float_compare(FloatPredicate::OLT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::Greater => (
                self.builder
                    .build_float_compare(FloatPredicate::OGT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::LessOrEqual => (
                self.builder
                    .build_float_compare(FloatPredicate::OLE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            Operator::GreaterOrEqual => (
                self.builder
                    .build_float_compare(FloatPredicate::OGE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.get_bool_type_information(),
            ),

            _ => unimplemented!(),
        }
    }
}
