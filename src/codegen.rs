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
use inkwell::values::GlobalValue;
use inkwell::AddressSpace;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

    variables: HashMap<String, u32>,
    current_pou: String,
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
        
        let return_type = self.context.i32_type();
        let f_type = return_type.fn_type(&[], false);
        let function = self.module.add_function("main", f_type, None);
        let block = self.context.append_basic_block(function, "entry");

        let mut program_members: Vec<(String, BasicTypeEnum<'ctx>)> = Vec::new();

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
        let instance_variable = self.generate_instance_variable(member_type, CodeGen::get_struct_instance_name(p.name.as_str()).as_str());
        let mut result = None;
        for stmt in &p.statements {
            self.builder.position_at_end(&block);
            result = self.generate_statement(stmt);
        }
        self.builder.build_return(Some(&result.unwrap()));
        //self.builder.build_return(None);
    }

    fn get_variables_information(&self, v: &VariableBlock) -> Vec<(String, BasicTypeEnum<'ctx>)> {
        let mut types: Vec<(String, BasicTypeEnum<'ctx>)> = Vec::new();
        for variable in &v.variables {
            let var_type = self.context.i32_type();
            types.push((variable.name.clone(), var_type.into()));
        }
        types
    }

    fn generate_instance_struct(
        context: &'ctx Context,
        variables: &mut HashMap<String, u32>,
        members: Vec<(String, BasicTypeEnum<'ctx>)>,
        name: &str,
    ) -> StructType<'ctx> {
        let struct_type = context.opaque_struct_type(name);
        let mut member_types: Vec<BasicTypeEnum<'ctx>> = Vec::new();

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
            Statement::BinaryExpression {
                operator,
                left,
                right,
            } => self.generate_binary_expression(operator, left, right),
            Statement::LiteralNumber { value } => self.generate_literal_number(value.as_str()),
            Statement::Reference { name } => self.generate_variable_reference(name),
            _ => None,
        }
    }

    fn generate_variable_reference(&self, name: &str) -> Option<BasicValueEnum> {
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
             let result = self.builder.build_load(ptr_result, "deref"); 
             Some(result)
            //self.builder.build_extract_value(struct_value.into_struct_value(), *index, name)
        } else {
            None
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
        let lval_opt = self.generate_statement(left);
        let lvalue = lval_opt.unwrap().into_int_value();

        let rval_opt = self.generate_statement(right);
        let rvalue = rval_opt.unwrap().into_int_value();

        let result = match operator {
            Operator::Plus => self.builder.build_int_add(lvalue, rvalue,"tmpVar"),
            Operator::Minus => unimplemented!(),
            Operator::Multiplication => unimplemented!(),
            Operator::Division => unimplemented!(),
            Operator::Equal => unimplemented!(),
            Operator::NotEqual => unimplemented!(),
            Operator::Modulo => unimplemented!(),
            Operator::Less => unimplemented!(),
            Operator::Greater => unimplemented!(),
            Operator::LessOrEqual => unimplemented!(),
            Operator::GreaterOrEqual => unimplemented!(),
        };
        Some(BasicValueEnum::IntValue(result))
    }
}
