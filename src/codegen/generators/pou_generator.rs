/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::{expression_generator::ExpressionCodeGenerator, llvm::LLVM, statement_generator::{FunctionContext, StatementCodeGenerator}, struct_generator::{self, StructGenerator}};
use crate::{ast::{DataTypeDeclaration, LinkageType, POU, PouType, SourceRange, Statement, Variable}, compile_error::CompileError, index::{DataTypeIndexEntry, DataTypeInformation, Index}};
use inkwell::{AddressSpace, builder::Builder, context::Context, module::Module, types::{BasicTypeEnum, FunctionType}, values::{BasicValueEnum, FunctionValue}};

pub struct PouGenerator<'a, 'b> {
    llvm: &'b LLVM<'a>,
    index: &'b mut Index<'a>,
}

pub fn index_pou<'a>(pou_name: &str, context: &'a Context, index: &mut Index<'a>) {
    let struct_name = format!("{}_interface", pou_name);
    let struct_type = context.opaque_struct_type(struct_name.as_str());
    index.associate_type(
        pou_name,
        DataTypeInformation::Struct {
            name: struct_name,
            generated_type: struct_type.into(),
        },
    );
}

impl<'a, 'b> PouGenerator<'a, 'b> {
    pub fn new(
        llvm: &'b LLVM<'a>,
        global_index: &'b mut Index<'a>,
    ) -> PouGenerator<'a, 'b> {
        let pou_generator = PouGenerator {
            llvm,
            index: global_index,
        };

        pou_generator
    }

    pub fn generate_pou(&mut self, pou: &POU, module: &Module<'a>) -> Result<(), CompileError> {

        let context = self.llvm.context;

        let return_type = pou
            .return_type
            .as_ref()
            .and_then(DataTypeDeclaration::get_name)
            .and_then(|it| self.index.find_type(it))
            .and_then(DataTypeIndexEntry::get_type);

        let pou_name = &pou.name;

        //generate the instance-struct type
        let pou_members: Vec<&Variable> = pou
            .variable_blocks
            .iter()
            .flat_map(|it| it.variables.iter())
            .collect();
        let instance_struct_type = {
            let mut struct_generator =
                StructGenerator::new(self.llvm, self.index);
            let (struct_type, initial_value) = struct_generator.generate_struct_type(&pou_members, pou_name)?;
            self.index.associate_type_initial_value(pou_name, initial_value);
            struct_type
        };

        //generate a function that takes a instance-struct parameter
        let current_function = {
            let function_declaration = Self::create_llvm_function_declaration(
                context,
                vec![instance_struct_type.ptr_type(AddressSpace::Generic).into()],
                return_type,
            );

            let curr_f = module.add_function(pou_name, function_declaration, None);
            self.index
                .associate_callable_implementation(pou_name, curr_f);
            curr_f
        };

        //generate a global variable if it's a program
        if pou.pou_type == PouType::Program {
            let instance_initializer = self
                .index
                .find_type(pou_name)
                .and_then(DataTypeIndexEntry::get_initial_value);
            let global_value = self.llvm.create_global_variable(
                    module, 
                    &struct_generator::get_pou_instance_variable_name(pou_name),
                    instance_struct_type.into(),
                    instance_initializer);
            self.index
                .associate_global_variable(pou_name, global_value.as_pointer_value());
        }

        //Don't generate external functions
        if pou.linkage == LinkageType::External {
            return Ok(());
        }
        
        //generate the body
        let block = context.append_basic_block(current_function, "entry");
        self.llvm.builder.position_at_end(block);
        
        //generate the return-variable
        if let Some(return_type) = return_type {
            let return_variable = self.llvm.allocate_local_variable(pou_name, &return_type);
            self.index.associate_local_variable(pou_name, pou_name, return_variable);
        }

        Self::generate_local_variable_accessors(
            &self.llvm.builder,
            current_function,
            &mut self.index,
            &pou_members,
            pou_name,
        )?;
        let function_context = FunctionContext{linking_context: pou_name.clone(), function: current_function};
        {
            let statement_gen = StatementCodeGenerator::new(self.llvm, &self.index, &function_context);
            statement_gen.generate_body(&pou.statements, &self.llvm.builder)?
        }

        // generate return statement
        self.generate_return_statement(&function_context, pou.pou_type, Some(0..0))?; //TODO location

        Ok(())
    }

    fn create_llvm_function_declaration(
        context: &'a Context,
        parameters: Vec<BasicTypeEnum<'a>>,
        return_type: Option<BasicTypeEnum<'a>>,
    ) -> FunctionType<'a> {
        let params = parameters.as_slice();
        match return_type {
            Some(enum_type) if enum_type.is_int_type() => {
                enum_type.into_int_type().fn_type(params, false)
            }
            Some(enum_type) if enum_type.is_float_type() => {
                enum_type.into_float_type().fn_type(params, false)
            }
            Some(enum_type) if enum_type.is_array_type() => {
                enum_type.into_array_type().fn_type(params, false)
            }
            None => context.void_type().fn_type(params, false),
            _ => panic!(format!("Unsupported return type {:?}", return_type)),
        }
    }

    fn generate_local_variable_accessors(
        builder: &Builder<'a>,
        current_function: FunctionValue<'a>,
        index: &mut Index<'a>,
        members: &Vec<&Variable>,
        pou_name: &str,
    ) -> Result<(), CompileError> {

        //Generate reference to parameter
        for (i, m) in members.iter().enumerate() {
            let parameter_name = &m.name;

            let ptr_value = current_function
                .get_first_param()
                .map(BasicValueEnum::into_pointer_value)
                .ok_or_else(|| CompileError::MissingFunctionError{location: m.location.clone()})?;

            index.associate_local_variable(
                pou_name,
                parameter_name,
                builder
                    .build_struct_gep(ptr_value, i as u32, &parameter_name)
                    .unwrap(),
            )
        }

        Ok(())
    }

    fn generate_return_statement(&self, function_context: &FunctionContext<'a>, pou_type: PouType, location: Option<SourceRange>) -> Result<(), CompileError> {
        match pou_type {
            PouType::Function => {
                let reference = Statement::Reference{
                    name: function_context.linking_context.clone(),
                    location: location.unwrap_or(0usize..0usize)
                };
                let mut exp_gen = ExpressionCodeGenerator::new(self.llvm, self.index, None, &function_context);
                exp_gen.load_prefix = "".to_string();
                exp_gen.load_suffix = "_ret".to_string();
                let (_, value) = exp_gen.generate_expression(&reference)?;
                self.llvm.builder.build_return(Some(&value));
            }
            _ => {
                self.llvm.builder.build_return(None);
            }
        }
        Ok(())
    }
}


