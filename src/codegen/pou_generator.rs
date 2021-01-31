use super::{CodeGen, codegen_util, instance_struct_generator::InstanceStructGenerator, statement_generator::{FunctionContext, StatementCodeGenerator}, variable_generator};
use crate::{ast::{LinkageType, POU, PouType, Statement, Variable}, index::{DataTypeIndexEntry, DataTypeInformation, Index}};
use inkwell::{AddressSpace, builder::Builder, context::Context, module::Module, types::{BasicTypeEnum, FunctionType}, values::{BasicValueEnum, FunctionValue, PointerValue}};

pub struct PouGenerator<'a, 'b> {
    context: &'a Context,
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
        context: &'a Context,
        global_index: &'b mut Index<'a>,
    ) -> PouGenerator<'a, 'b> {
        let pou_generator = PouGenerator {
            context,
            index: global_index,
        };

        pou_generator
    }

    pub fn generate_pou(&mut self, pou: &POU, module: &Module<'a>) -> Result<(), String> {

        let return_type = pou
            .return_type
            .as_ref()
            .and_then(|data_type| codegen_util::find_type_in_index(&data_type, &self.index));

        let pou_name = &pou.name;

        //generate the instance-struct type
        let pou_members: Vec<&Variable> = pou
            .variable_blocks
            .iter()
            .flat_map(|it| it.variables.iter())
            .collect();
        let instance_struct_type = {
            let mut struct_generator =
                InstanceStructGenerator::new(self.context, self.index);
            let struct_type = struct_generator.generate_struct_type(&pou_members, pou_name, &self.context.create_builder())?;
            
            struct_type
        };

        //generate a function that takes a instance-struct parameter
        let current_function = {
            let function_declaration = Self::create_llvm_function_declaration(
                self.context,
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
            let instance_name = CodeGen::get_struct_instance_name(pou_name);
            let instance_initializer = self
                .index
                .find_type(pou_name)
                .and_then(DataTypeIndexEntry::get_initial_value);
            let global_value = variable_generator::create_llvm_global_variable(
                module,
                &instance_name,
                instance_struct_type.into(),
                instance_initializer,
            );
            self.index
                .associate_global_variable(pou_name, global_value.as_pointer_value());
        }

        //Don't generate external functions
        if pou.linkage == LinkageType::External {
            return Ok(());
        }
        
        //generate the body
        let block = self.context.append_basic_block(current_function, "entry");
        let builder = self.context.create_builder();
        builder.position_at_end(block);
        
        //generate the return-variable
        if let Some(return_type) = return_type {
            let return_variable = variable_generator::create_llvm_local_variable(&builder, pou_name, &return_type);
            self.index.associate_local_variable(pou_name, pou_name, return_variable);
        }

        Self::generate_local_variable_accessors(
            &builder,
            current_function,
            &mut self.index,
            &pou_members,
            pou_name,
        )?;
        let function_context = FunctionContext{linking_context: pou_name.clone(), function: current_function};
        {
            let mut statement_gen = StatementCodeGenerator::new(self.context, &self.index, Some(&function_context));
            statement_gen.generate_body(&pou.statements, &builder)?
        }

        // generate return statement
        self.generate_return_statement(&function_context, pou.pou_type, &builder)?;

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

    fn generate_return_variable(
        builder: &Builder<'a>,
        pou_name: &str,
        return_type: &BasicTypeEnum<'a>) -> PointerValue<'a> {
        let ret_name = format!("{}_ret", pou_name).as_str();
        builder.build_alloca(*return_type, "")
    }

    fn generate_local_variable_accessors(
        builder: &Builder<'a>,
        current_function: FunctionValue<'a>,
        index: &mut Index<'a>,
        members: &Vec<&Variable>,
        pou_name: &str,
    ) -> Result<(), String> {

        //Generate reference to parameter
        for (i, m) in members.iter().enumerate() {
            let parameter_name = &m.name;

            let ptr_value = current_function
                .get_first_param()
                .map(BasicValueEnum::into_pointer_value)
                .ok_or_else(|| "self.current_function must not be empty")?;

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

    fn generate_return_statement(&self, function_context: &FunctionContext, pou_type: PouType, builder: &Builder<'a>) -> Result<(), String> {
        match pou_type {
            PouType::Function => {
                let reference = Statement::Reference{
                    name: function_context.linking_context.clone(),
                    location: 0..0 //TODO
                };
                let mut statement_gen = StatementCodeGenerator::new(self.context, &self.index, Some(function_context));
                statement_gen.load_prefix = "".to_string();
                statement_gen.load_suffix = "_ret".to_string();
                let (_, value) = statement_gen.generate_expression(&reference, builder)?;
                builder.build_return(Some(&value));
            }
            _ => {
                builder.build_return(None);
            }
        }
        Ok(())
    }
}
