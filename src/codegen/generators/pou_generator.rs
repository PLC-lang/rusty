// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::{
    expression_generator::ExpressionCodeGenerator,
    llvm::Llvm,
    statement_generator::{FunctionContext, StatementCodeGenerator},
};
use crate::codegen::llvm_index::LlvmTypedIndex;

/// The pou_generator contains functions to generate the code for POUs (PROGRAM, FUNCTION, FUNCTION_BLOCK)
/// # responsibilities
/// - generates a struct-datatype for the POU's members
/// - generates a function for the pou
/// - declares a global instance if the POU is a PROGRAM
use crate::index::{ImplementationIndexEntry, VariableIndexEntry};
use crate::typesystem::*;
use crate::{
    ast::{Implementation, PouType, SourceRange, Statement},
    compile_error::CompileError,
    index::Index,
};
use inkwell::types::StructType;
use inkwell::{
    module::Module,
    types::{BasicTypeEnum, FunctionType},
    values::{BasicValueEnum, FunctionValue},
    AddressSpace,
};

pub struct PouGenerator<'ink, 'cg> {
    llvm: Llvm<'ink>,
    index: &'cg Index,
    llvm_index: &'cg LlvmTypedIndex<'ink>,
}

/// Creates opaque implementations for all callable items in the index
/// Returns a Typed index containing the associated implementations.
pub fn generate_implementation_stubs<'ink>(
    module: &Module<'ink>,
    llvm: Llvm<'ink>,
    index: &Index,
    types_index: &LlvmTypedIndex<'ink>,
) -> Result<LlvmTypedIndex<'ink>, CompileError> {
    let mut llvm_index = LlvmTypedIndex::new();
    let pou_generator = PouGenerator::new(llvm, index, types_index);
    for (name, implementation) in index.get_implementations() {
        let curr_f = pou_generator.generate_implementation_stub(implementation, module)?;
        llvm_index.associate_implementation(name, curr_f)?;
    }

    Ok(llvm_index)
}

impl<'ink, 'cg> PouGenerator<'ink, 'cg> {
    /// creates a new PouGenerator
    ///
    /// the PouGenerator needs a mutable index to register the generated pou
    pub fn new(
        llvm: Llvm<'ink>,
        index: &'cg Index,
        llvm_index: &'cg LlvmTypedIndex<'ink>,
    ) -> PouGenerator<'ink, 'cg> {
        PouGenerator {
            llvm,
            index,
            llvm_index,
        }
    }

    pub fn generate_implementation_stub(
        &self,
        implementation: &ImplementationIndexEntry,
        module: &Module<'ink>,
    ) -> Result<FunctionValue<'ink>, CompileError> {
        let global_index = self.index;
        //generate a function that takes a instance-struct parameter
        let pou_name = implementation.get_call_name();
        let instance_struct_type: StructType = self
            .llvm_index
            .get_associated_type(implementation.get_type_name())
            .map(|it| it.into_struct_type())?;
        let return_type: Option<&DataType> =
            global_index.find_return_type(implementation.get_type_name());
        let return_type = return_type
            .map(DataType::get_name)
            .map(|it| self.llvm_index.get_associated_type(it).unwrap());
        let parameters = vec![instance_struct_type.ptr_type(AddressSpace::Generic).into()];
        let variadic = global_index
            .find_type_information(implementation.get_type_name())
            .map(|it| it.is_variadic())
            .unwrap_or(false);

        let function_declaration =
            self.create_llvm_function_type(parameters, variadic, return_type)?;

        let curr_f = module.add_function(pou_name, function_declaration, None);
        Ok(curr_f)
    }

    /// generates a function for the given pou
    pub fn generate_implementation(
        &self,
        implementation: &Implementation,
    ) -> Result<(), CompileError> {
        let context = self.llvm.context;
        let mut local_index = LlvmTypedIndex::create_child(self.llvm_index);

        let pou_name = &implementation.name;

        let pou_members = self.index.find_local_members(&implementation.type_name);

        let current_function = self
            .llvm_index
            .find_associated_implementation(pou_name)
            .ok_or_else(|| {
                CompileError::codegen_error(
                    format!("Could not find generated stub for {}", pou_name),
                    implementation.location.clone(),
                )
            })?;

        //generate the body
        let block = context.append_basic_block(current_function, "entry");
        self.llvm.builder.position_at_end(block);

        // generate loads for all the parameters
        self.generate_local_variable_accessors(
            &mut local_index,
            &implementation.type_name,
            current_function,
            &pou_members,
        )?;

        let function_context = FunctionContext {
            linking_context: implementation.into(),
            function: current_function,
        };
        {
            let statement_gen = StatementCodeGenerator::new(
                &self.llvm,
                self.index,
                self,
                implementation.pou_type,
                &local_index,
                &function_context,
            );
            //if this is a function, we need to initilialize the VAR-variables
            if implementation.pou_type == PouType::Function {
                self.generate_initialization_of_local_vars(pou_members, &statement_gen)?;
            }
            statement_gen.generate_body(&implementation.statements)?
        }

        // generate return statement
        self.generate_return_statement(
            &function_context,
            &local_index,
            implementation.pou_type,
            None,
        )?; //TODO location

        Ok(())
    }

    /// TODO llvm.rs
    /// generates a llvm `FunctionType` that takes the given list of `parameters` and
    /// returns the given `return_type`
    fn create_llvm_function_type(
        &self,
        parameters: Vec<BasicTypeEnum<'ink>>,
        is_var_args: bool,
        return_type: Option<BasicTypeEnum<'ink>>,
    ) -> Result<FunctionType<'ink>, CompileError> {
        let params = parameters.as_slice();
        match return_type {
            Some(enum_type) if enum_type.is_int_type() => {
                Ok(enum_type.into_int_type().fn_type(params, is_var_args))
            }
            Some(enum_type) if enum_type.is_float_type() => {
                Ok(enum_type.into_float_type().fn_type(params, is_var_args))
            }
            Some(enum_type) if enum_type.is_array_type() => {
                Ok(enum_type.into_array_type().fn_type(params, is_var_args))
            }
            None => Ok(self.llvm.context.void_type().fn_type(params, is_var_args)),
            _ => Err(CompileError::codegen_error(
                format!("Unsupported return type {:?}", return_type),
                SourceRange::undefined(),
            )),
        }
    }

    /// generates a load-statement for the given member
    fn generate_local_variable_accessors(
        &self,
        index: &mut LlvmTypedIndex<'ink>,
        type_name: &str,
        current_function: FunctionValue<'ink>,
        members: &[&VariableIndexEntry],
    ) -> Result<(), CompileError> {
        //Generate reference to parameter
        for (i, m) in members.iter().enumerate() {
            let parameter_name = m.get_name();

            let (name, variable) = if m.is_return() {
                let return_type = index.get_associated_type(m.get_type_name())?;
                (
                    type_name,
                    self.llvm.create_local_variable(type_name, &return_type),
                )
            } else {
                let ptr_value = current_function
                    .get_first_param()
                    .map(BasicValueEnum::into_pointer_value)
                    .ok_or_else(|| CompileError::MissingFunctionError {
                        location: m.source_location.clone(),
                    })?;

                (
                    parameter_name,
                    self.llvm
                        .builder
                        .build_struct_gep(ptr_value, i as u32, parameter_name)
                        .unwrap(),
                )
            };
            index.associate_loaded_local_variable(type_name, name, variable)?;
        }

        Ok(())
    }

    /// generates assignment statements for initialized variables in the VAR-block
    ///
    /// - `blocks` - all declaration blocks of the current pou
    fn generate_initialization_of_local_vars(
        &self,
        variables: Vec<&VariableIndexEntry>,
        statement_generator: &StatementCodeGenerator<'ink, '_>,
    ) -> Result<(), CompileError> {
        let variables_with_initializers = variables
            .iter()
            .filter(|it| it.is_local())
            .filter(|it| it.initial_value.is_some());

        for variable in variables_with_initializers {
            let left = Statement::Reference {
                name: variable.get_name().into(),
                location: variable.source_location.clone(),
                id: 0, //TODO
            };
            let right = variable.initial_value.as_ref().unwrap();
            statement_generator.generate_assignment_statement(&left, right)?;
        }
        Ok(())
    }

    /// generates the function's return statement only if the given pou_type is a `PouType::Function`
    ///
    /// a function returns the value of the local variable that has the function's name
    pub fn generate_return_statement(
        &self,
        function_context: &FunctionContext<'ink>,
        local_index: &LlvmTypedIndex<'ink>,
        pou_type: PouType,
        location: Option<SourceRange>,
    ) -> Result<(), CompileError> {
        match pou_type {
            PouType::Function => {
                let reference = Statement::Reference {
                    name: function_context.linking_context.get_call_name().into(),
                    location: location.unwrap_or_else(SourceRange::undefined),
                    id: 0, //TODO
                };
                let mut exp_gen = ExpressionCodeGenerator::new(
                    &self.llvm,
                    self.index,
                    local_index,
                    None,
                    function_context,
                );
                exp_gen.temp_variable_prefix = "".to_string();
                exp_gen.temp_variable_suffix = "_ret".to_string();
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
