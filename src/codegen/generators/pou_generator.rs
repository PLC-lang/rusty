// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use super::{
    data_type_generator::get_default_for,
    expression_generator::ExpressionCodeGenerator,
    llvm::{GlobalValueExt, Llvm},
    section_names,
    statement_generator::{FunctionContext, StatementCodeGenerator},
    ADDRESS_SPACE_GENERIC,
};
use crate::{
    codegen::{
        debug::{Debug, DebugBuilderEnum},
        llvm_index::LlvmTypedIndex,
    },
    index::{self, ImplementationType},
    resolver::{AstAnnotations, Dependency},
    typesystem::{DataType, DataTypeInformation, VarArgs, DINT_TYPE},
};

/// The pou_generator contains functions to generate the code for POUs (PROGRAM, FUNCTION, FUNCTION_BLOCK)
/// # responsibilities
/// - generates a struct-datatype for the POU's members
/// - generates a function for the pou
/// - declares a global instance if the POU is a PROGRAM
use crate::index::{ArgumentType, FxIndexMap, FxIndexSet, ImplementationIndexEntry, VariableIndexEntry};

use crate::index::Index;
use index::VariableType;

use inkwell::{
    module::Module,
    types::{BasicMetadataTypeEnum, BasicTypeEnum, FunctionType},
    values::{BasicValue, BasicValueEnum, FunctionValue},
    AddressSpace,
};
use inkwell::{
    types::{BasicType, StructType},
    values::PointerValue,
};
use plc_ast::ast::{AstNode, Implementation, PouType};
use plc_diagnostics::diagnostics::{Diagnostic, INTERNAL_LLVM_ERROR};
use plc_source::source_location::SourceLocation;
use rustc_hash::FxHashMap;
use section_mangler::{FunctionArgument, SectionMangler};

pub struct PouGenerator<'ink, 'cg> {
    llvm: Llvm<'ink>,
    index: &'cg Index,
    annotations: &'cg AstAnnotations,
    llvm_index: &'cg LlvmTypedIndex<'ink>,
}

/// Creates opaque implementations for all callable items in the index
/// Returns a Typed index containing the associated implementations.
pub fn generate_implementation_stubs<'ink>(
    module: &Module<'ink>,
    llvm: Llvm<'ink>,
    dependencies: &FxIndexSet<Dependency>,
    index: &Index,
    annotations: &AstAnnotations,
    types_index: &LlvmTypedIndex<'ink>,
    debug: &mut DebugBuilderEnum<'ink>,
) -> Result<LlvmTypedIndex<'ink>, Diagnostic> {
    let mut llvm_index = LlvmTypedIndex::default();
    let pou_generator = PouGenerator::new(llvm, index, annotations, types_index);
    let implementations = dependencies
        .into_iter()
        .filter_map(|it| {
            if let Dependency::Call(name) | Dependency::Datatype(name) = it {
                index.find_implementation_by_name(name).map(|it| (name.as_str(), it))
            } else {
                None
            }
        })
        .collect::<FxIndexMap<_, _>>();
    for (name, implementation) in implementations {
        if !implementation.is_generic() {
            let curr_f =
                pou_generator.generate_implementation_stub(implementation, module, debug, &mut llvm_index)?;
            llvm_index.associate_implementation(name, curr_f)?;
        }
    }

    Ok(llvm_index)
}

///Generates a global constant for each initialized pou member
/// The given constant can then be used to initialize the variable using memcpy without re-evaluating the expression
/// Retrieves the POUs from the index (implementation)
/// Returns a new LLVM index to be merged with the parent codegen index.
pub fn generate_global_constants_for_pou_members<'ink>(
    module: &Module<'ink>,
    llvm: &Llvm<'ink>,
    dependencies: &FxIndexSet<Dependency>,
    index: &Index,
    annotations: &AstAnnotations,
    llvm_index: &LlvmTypedIndex<'ink>,
    location: &str,
) -> Result<LlvmTypedIndex<'ink>, Diagnostic> {
    let mut local_llvm_index = LlvmTypedIndex::default();
    let implementations = dependencies.into_iter().filter_map(|it| {
        if let Dependency::Call(name) | Dependency::Datatype(name) = it {
            index.find_implementation_by_name(name).filter(|it| it.is_in_unit(location))
        } else {
            None
        }
    });
    for implementation in implementations {
        let type_name = implementation.get_type_name();
        let pou_members = index.get_pou_members(type_name);
        let variables = pou_members.iter().filter(|it| it.is_local() || it.is_temp()).filter(|it| {
            let var_type =
                index.get_effective_type_or_void_by_name(it.get_type_name()).get_type_information();
            var_type.is_struct() || var_type.is_array() || var_type.is_string()
        });
        let exp_gen = ExpressionCodeGenerator::new_context_free(llvm, index, annotations, llvm_index);
        for variable in variables {
            let name = index::get_initializer_name(variable.get_qualified_name());
            let right_stmt =
                index.get_const_expressions().maybe_get_constant_statement(&variable.initial_value);

            if right_stmt.is_some() && llvm_index.find_global_value(&name).is_none() {
                let variable_type = llvm_index.get_associated_type(variable.get_type_name())?;
                let value = if let Some(stmt) = right_stmt {
                    Some(exp_gen.generate_expression(stmt)?)
                } else {
                    llvm_index.find_associated_initial_value(variable.get_qualified_name())
                };
                if let Some(value) = value {
                    let global_value = llvm
                        .create_global_variable(module, &name, variable_type)
                        .make_constant()
                        .set_initial_value(Some(value), variable_type);
                    local_llvm_index.associate_global(&name, global_value)?;
                    local_llvm_index.insert_new_got_index(&name)?;
                }
            }
        }
    }
    Ok(local_llvm_index)
}

impl<'ink, 'cg> PouGenerator<'ink, 'cg> {
    /// creates a new PouGenerator
    ///
    /// the PouGenerator needs a mutable index to register the generated pou
    pub fn new(
        llvm: Llvm<'ink>,
        index: &'cg Index,
        annotations: &'cg AstAnnotations,
        llvm_index: &'cg LlvmTypedIndex<'ink>,
    ) -> PouGenerator<'ink, 'cg> {
        PouGenerator { llvm, index, annotations, llvm_index }
    }

    fn mangle_function(&self, implementation: &ImplementationIndexEntry) -> Result<String, Diagnostic> {
        let ctx = SectionMangler::function(implementation.get_call_name().to_lowercase());

        let params = self.index.get_declared_parameters(implementation.get_call_name());

        let ctx = params.into_iter().try_fold(ctx, |ctx, param| -> Result<SectionMangler, Diagnostic> {
            let ty = section_names::mangle_type(
                self.index,
                self.index.get_effective_type_by_name(&param.data_type_name)?,
            )?;
            let parameter = match param.argument_type {
                // TODO: We need to handle the `VariableType` enum as well - this describes the mode of
                // argument passing, e.g. inout
                index::ArgumentType::ByVal(_) => FunctionArgument::ByValue(ty),
                index::ArgumentType::ByRef(_) => FunctionArgument::ByRef(ty),
            };

            Ok(ctx.with_parameter(parameter))
        })?;

        let return_ty = self
            .index
            .find_return_type(implementation.get_type_name())
            .map(|ty| section_names::mangle_type(self.index, ty));

        let ctx = match return_ty {
            Some(rty) => ctx.with_return_type(rty?),
            None => ctx,
        };

        Ok(ctx.mangle())
    }

    /// generates an empty llvm function for the given implementation, including all parameters and the return type
    pub fn generate_implementation_stub(
        &self,
        implementation: &ImplementationIndexEntry,
        module: &Module<'ink>,
        debug: &mut DebugBuilderEnum<'ink>,
        new_llvm_index: &mut LlvmTypedIndex<'ink>,
    ) -> Result<FunctionValue<'ink>, Diagnostic> {
        let declared_parameters = self.index.get_declared_parameters(implementation.get_call_name());
        let parameters = self
            .collect_parameters_for_implementation(implementation)?
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let param = declared_parameters.get(i);
                let dti = param.map(|it| self.index.get_type_information_or_void(it.get_type_name()));
                match param {
                    Some(v)
                        if v.is_in_parameter_by_ref() &&
                        // parameters by ref will always be a pointer
                        p.into_pointer_type().get_element_type().is_array_type() =>
                    {
                        // for by-ref array types we will generate a pointer to the arrays element type
                        // not a pointer to array
                        let ty = p
                            .into_pointer_type()
                            .get_element_type()
                            .into_array_type()
                            .get_element_type()
                            .ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC));

                        // set the new type for further codegen
                        let _ = new_llvm_index.associate_type(v.get_type_name(), ty.into());

                        ty.into()
                    }
                    _ => {
                        dti.map(|it| {
                            if !matches!(
                                implementation.get_implementation_type(),
                                ImplementationType::Function
                            ) {
                                return *p;
                            }
                            // for aggregate function parameters we will generate a pointer instead of the value type.
                            // it will then later be memcopied into a locally allocated variable
                            match it {
                                DataTypeInformation::Struct { .. } => p
                                    .into_struct_type()
                                    .ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC))
                                    .into(),
                                DataTypeInformation::Array { .. } | DataTypeInformation::String { .. } => p
                                    .into_array_type()
                                    .get_element_type()
                                    .ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC))
                                    .into(),
                                _ => *p,
                            }
                        })
                        .unwrap_or(*p)
                    }
                }
            })
            .collect::<Vec<BasicMetadataTypeEnum>>();

        let return_type = self
            .index
            .find_return_type(implementation.get_type_name())
            .and_then(|dt| self.index.find_effective_type(dt));

        // see if we need to adapt the parameters list
        let (return_type_llvm, parameters) = match return_type {
            // function with a aggrate-return type
            Some(r_type) if r_type.is_aggregate_type() => {
                let mut params_with_inout = Vec::with_capacity(parameters.len() + 1);

                // add the out pointer as an extra parameter in the beginning
                let return_llvm_type = self.llvm_index.get_associated_type(r_type.get_name())?;
                params_with_inout
                    .push(return_llvm_type.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into()); //TODO: what is the correct address space?

                // add the remaining parameters
                params_with_inout.extend(parameters.iter().cloned());

                // no return, adapted parameters
                (None, params_with_inout)
            }
            // function with an intrinsic return-type
            Some(r_type) => (Some(self.llvm_index.get_associated_type(r_type.get_name())?), parameters),
            // no return
            None => (None, parameters),
        };

        let variadic = self.index.get_variadic_member(implementation.get_type_name());

        let function_declaration = self.create_llvm_function_type(parameters, variadic, return_type_llvm)?;

        let curr_f = module.add_function(implementation.get_call_name(), function_declaration, None);

        let section_name = self.mangle_function(implementation)?;
        curr_f.set_section(Some(&section_name));

        let pou_name = implementation.get_call_name();
        if let Some(pou) = self.index.find_pou(pou_name) {
            let parameter_types = declared_parameters
                .iter()
                .map(|v| self.index.get_effective_type_or_void_by_name(v.get_type_name()))
                .collect::<Vec<&DataType>>();

            debug.register_function(
                self.index,
                curr_f,
                pou,
                return_type,
                parameter_types.as_slice(),
                implementation.get_location().get_line(),
            );
        }
        Ok(curr_f)
    }

    /// creates and returns all parameters for the given implementation
    /// for functions, this method creates a full list of parameters, for other POUs
    /// this method creates a single state-struct parameter
    fn collect_parameters_for_implementation(
        &self,
        implementation: &ImplementationIndexEntry,
    ) -> Result<Vec<BasicMetadataTypeEnum<'ink>>, Diagnostic> {
        if implementation.implementation_type != ImplementationType::Function {
            let mut parameters = vec![];
            if implementation.get_implementation_type() == &ImplementationType::Method {
                let class_name =
                    implementation.get_associated_class_name().expect("Method needs to have a class-name");
                let instance_members_struct_type: StructType =
                    self.llvm_index.get_associated_type(class_name).map(|it| it.into_struct_type())?;
                parameters.push(
                    instance_members_struct_type.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into(),
                );
            }
            let instance_struct_type: StructType = self
                .llvm_index
                .get_associated_pou_type(implementation.get_type_name())
                .map(|it| it.into_struct_type())?;
            parameters.push(instance_struct_type.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into());

            Ok(parameters)
        } else {
            let declared_params = self.index.get_declared_parameters(implementation.get_call_name());

            //find the function's parameters
            declared_params
                .iter()
                .map(|v| self.llvm_index.get_associated_type(v.get_type_name()).map(Into::into))
                .collect::<Result<Vec<BasicMetadataTypeEnum>, _>>()
        }
    }

    /// generates a function for the given pou
    pub fn generate_implementation(
        &self,
        implementation: &Implementation,
        debug: &DebugBuilderEnum<'ink>,
    ) -> Result<(), Diagnostic> {
        let context = self.llvm.context;
        let mut local_index = LlvmTypedIndex::create_child(self.llvm_index);

        let pou_name = &implementation.name;

        let current_function = self.llvm_index.find_associated_implementation(pou_name).ok_or_else(|| {
            Diagnostic::codegen_error(
                format!("Could not find generated stub for {pou_name}"),
                implementation.location.clone(),
            )
        })?;

        let (line, column) = implementation
            .statements
            .first()
            .map(|it| (it.get_location().get_line_plus_one(), it.get_location().get_column()))
            .or_else(|| {
                Some((implementation.location.get_line_plus_one(), implementation.location.get_column()))
            })
            // .or_else(|| Some(implementation.location.get_start()))
            .unwrap();
        debug.set_debug_location(&self.llvm, &current_function, line, column);

        //generate the body
        let block = context.append_basic_block(current_function, "entry");

        //Create all labels this function will have
        let mut blocks = FxHashMap::default();
        if let Some(labels) = self.index.get_labels(&implementation.name) {
            for name in labels.keys() {
                blocks.insert(name.to_string(), self.llvm.context.append_basic_block(current_function, name));
            }
        }
        self.llvm.builder.position_at_end(block);
        blocks.insert("entry".into(), block);

        let function_context = FunctionContext {
            linking_context: self.index.find_implementation_by_name(&implementation.name).ok_or_else(
                || {
                    Diagnostic::codegen_error(
                        format!("Could not find implementation for {}", &implementation.name),
                        implementation.location.clone(),
                    )
                },
            )?,
            function: current_function,
            blocks,
        };

        let mut param_index = 0;
        if let PouType::Method { .. } = implementation.pou_type {
            let class_name = implementation.type_name.split('.').collect::<Vec<&str>>()[0];
            self.generate_local_pou_variable_accessors(
                param_index,
                &mut local_index,
                class_name,
                &function_context,
                &implementation.location,
                debug,
            )?;
            param_index += 1;
        }

        // generate local variables
        if implementation.pou_type == PouType::Function {
            self.generate_local_function_arguments_accessors(
                &mut local_index,
                &implementation.type_name,
                &function_context,
                debug,
            )?;
        } else {
            self.generate_local_pou_variable_accessors(
                param_index,
                &mut local_index,
                &implementation.type_name,
                &function_context,
                &implementation.location,
                debug,
            )?;
        }
        {
            let pou_members =
                self.index.get_pou_members(&implementation.type_name).iter().collect::<Vec<_>>();
            //if this is a function, we need to initilialize the VAR-variables
            if matches!(implementation.pou_type, PouType::Function | PouType::Method { .. }) {
                self.generate_initialization_of_local_vars(
                    &pou_members,
                    &local_index,
                    &function_context,
                    debug,
                )?;
            } else {
                //Generate temp variables
                let members = pou_members.into_iter().filter(|it| it.is_temp()).collect::<Vec<_>>();
                self.generate_initialization_of_local_vars(&members, &local_index, &function_context, debug)?;
            }
            let statement_gen = StatementCodeGenerator::new(
                &self.llvm,
                self.index,
                self.annotations,
                &local_index,
                &function_context,
                debug,
            );
            statement_gen.generate_body(&implementation.statements)?;
            statement_gen.generate_return_statement()?;
        }

        Ok(())
    }

    /// TODO llvm.rs
    /// generates a llvm `FunctionType` that takes the given list of `parameters` and
    /// returns the given `return_type`
    fn create_llvm_function_type(
        &self,
        parameters: Vec<BasicMetadataTypeEnum<'ink>>,
        variadic: Option<&'cg VariableIndexEntry>,
        return_type: Option<BasicTypeEnum<'ink>>,
    ) -> Result<FunctionType<'ink>, Diagnostic> {
        // sized variadic is not considered a variadic function, but receives 2 extra parameters, size and a pointer
        let is_var_args = variadic
            .map(|it| it.get_varargs().map(|it| !it.is_sized()).unwrap_or_default())
            .unwrap_or_default();
        let size_and_type = self.get_variadic_size_and_pointer(variadic);
        let mut params = parameters;
        if let Some(sized_variadics) = size_and_type {
            params.extend_from_slice(&sized_variadics);
        };

        match return_type {
            Some(enum_type) if enum_type.is_int_type() => {
                Ok(enum_type.into_int_type().fn_type(&params, is_var_args))
            }
            Some(enum_type) if enum_type.is_float_type() => {
                Ok(enum_type.into_float_type().fn_type(&params, is_var_args))
            }
            Some(enum_type) if enum_type.is_array_type() => {
                Ok(enum_type.into_array_type().fn_type(&params, is_var_args))
            }
            Some(enum_type) if enum_type.is_pointer_type() => {
                Ok(enum_type.into_pointer_type().fn_type(&params, is_var_args))
            }
            Some(enum_type) if enum_type.is_struct_type() => {
                Ok(enum_type.into_struct_type().fn_type(&params, is_var_args))
            }
            None => Ok(self.llvm.context.void_type().fn_type(&params, is_var_args)),
            _ => Err(Diagnostic::codegen_error(
                format!("Unsupported return type {return_type:?}"),
                SourceLocation::undefined(),
            )),
        }
    }

    /// generates a load-statement for the given members of a function
    fn generate_local_function_arguments_accessors(
        &self,
        index: &mut LlvmTypedIndex<'ink>,
        type_name: &str,
        function_context: &FunctionContext<'ink, '_>,
        debug: &DebugBuilderEnum<'ink>,
    ) -> Result<(), Diagnostic> {
        let members = self.index.get_pou_members(type_name);
        //Generate reference to parameter
        // cannot use index from members because return and temp variables may not be considered for index in build_struct_gep
        // eagerly handle the return-variable
        let mut params_iter = function_context.function.get_param_iter();
        if let Some(ret_v) = members.iter().find(|it| it.is_return()) {
            let return_type = index.get_associated_type(ret_v.get_type_name())?;
            let return_variable = if self
                .index
                .find_effective_type_by_name(ret_v.get_type_name())
                .filter(|it| it.is_aggregate_type())
                .is_some()
            {
                // function return is handled by an out-pointer
                let parameter = params_iter
                    .next()
                    .ok_or_else(|| Diagnostic::missing_function(ret_v.source_location.clone()))?;

                // remove the out-param so the loop below will not see it again
                // generate special accessor for aggrate function output (out-ptr)
                let accessor = self.llvm.create_local_variable(
                    ret_v.get_name(),
                    &return_type.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).as_basic_type_enum(),
                );
                self.llvm.builder.build_store(accessor, parameter);
                accessor
            } else {
                // function return is a real return
                self.llvm.create_local_variable(type_name, &return_type)
            };
            index.associate_loaded_local_variable(type_name, ret_v.get_name(), return_variable)?;
        }

        // handle all parameters (without return!)
        for m in members.iter().filter(|it| !it.is_return()) {
            let parameter_name = m.get_name();

            let (name, variable) = if m.is_parameter() {
                let ptr_value = params_iter
                    .next()
                    .ok_or_else(|| Diagnostic::missing_function(m.source_location.clone()))?;
                let member_type_name = m.get_type_name();
                let type_info = self.index.get_type_information_or_void(member_type_name);
                let ty = index.get_associated_type(member_type_name)?;
                let ptr = self.llvm.create_local_variable(m.get_name(), &ty);
                if let Some(block) = self.llvm.builder.get_insert_block() {
                    debug.add_variable_declaration(
                        m.get_qualified_name(),
                        ptr,
                        function_context.function,
                        block,
                        m.source_location.get_line(),
                        m.source_location.get_column(),
                    );
                }

                if matches!(m.argument_type, ArgumentType::ByVal(VariableType::Input))
                    && type_info.is_aggregate()
                {
                    // a by-value aggregate type => we need to memcpy the data into the local variable
                    let ty = if ty.is_array_type() {
                        ty.into_array_type()
                            .get_element_type()
                            .ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC))
                    } else {
                        ty.into_struct_type().ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC))
                    };
                    let bitcast = self.llvm.builder.build_bitcast(ptr, ty, "bitcast").into_pointer_value();
                    let (size, alignment) = if let DataTypeInformation::String { size, encoding } = type_info
                    {
                        // since passed string args might be larger than the local acceptor, we need to first memset the local variable to 0
                        let size = size.as_int_value(self.index).map_err(|err| {
                            Diagnostic::codegen_error(err.as_str(), m.source_location.clone())
                        })? as u64;
                        let char_width = encoding.get_bytes_per_char();
                        self.llvm
                            .builder
                            .build_memset(
                                bitcast,
                                char_width,
                                self.llvm.context.i8_type().const_zero(),
                                self.llvm.context.i64_type().const_int(size * char_width as u64, true),
                            )
                            .map_err(|e| Diagnostic::codegen_error(e, m.source_location.clone()))?;
                        (
                            // we then reduce the amount of bytes to be memcopied by the equivalent of one grapheme in bytes to preserve the null-terminator
                            self.llvm.context.i64_type().const_int((size - 1) * char_width as u64, true),
                            char_width,
                        )
                    } else {
                        let Some(size) = index.get_associated_type(member_type_name)?.size_of() else {
                            // XXX: can this still fail at this point? might be `unreachable`
                            return Err(Diagnostic::codegen_error(
                                "Unable to determine type size",
                                m.source_location.clone(),
                            ));
                        };
                        (size, 1)
                    };
                    self.llvm
                        .builder
                        .build_memcpy(bitcast, alignment, ptr_value.into_pointer_value(), alignment, size)
                        .map_err(|e| Diagnostic::codegen_error(e, m.source_location.clone()))?;
                } else {
                    self.llvm.builder.build_store(ptr, ptr_value);
                };

                (parameter_name, ptr)
            } else {
                let temp_type = index.get_associated_type(m.get_type_name())?;
                let value = self.llvm.create_local_variable(parameter_name, &temp_type);
                (parameter_name, value)
            };

            index.associate_loaded_local_variable(type_name, name, variable)?;
        }

        Ok(())
    }

    /// generates a load-statement for the given members
    /// for pous that take a struct-state-variable (or two for methods)
    fn generate_local_pou_variable_accessors(
        &self,
        arg_index: u32,
        index: &mut LlvmTypedIndex<'ink>,
        type_name: &str,
        function_context: &FunctionContext<'ink, '_>,
        location: &SourceLocation,
        debug: &DebugBuilderEnum<'ink>,
    ) -> Result<(), Diagnostic> {
        let members = self.index.get_pou_members(type_name);
        let param_pointer = function_context
            .function
            .get_nth_param(arg_index)
            .map(BasicValueEnum::into_pointer_value)
            .ok_or_else(|| Diagnostic::missing_function(location.clone()))?;
        //Generate POU struct declaration for debug
        if let Some(block) = self.llvm.builder.get_insert_block() {
            debug.add_variable_declaration(
                type_name,
                param_pointer,
                function_context.function,
                block,
                location.get_line(),
                location.get_column(),
            );
        }
        //Generate reference to parameter
        // cannot use index from members because return and temp variables may not be considered for index in build_struct_gep
        let mut var_count = 0;
        for m in members.iter() {
            let parameter_name = m.get_name();

            let (name, variable) = if m.is_temp() || m.is_return() {
                let temp_type = index.get_associated_type(m.get_type_name())?;
                (parameter_name, self.llvm.create_local_variable(parameter_name, &temp_type))
            } else {
                let ptr = self
                    .llvm
                    .builder
                    .build_struct_gep(param_pointer, var_count as u32, parameter_name)
                    .expect(INTERNAL_LLVM_ERROR);

                var_count += 1;

                (parameter_name, ptr)
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
        variables: &[&VariableIndexEntry],
        local_llvm_index: &LlvmTypedIndex,
        function_context: &FunctionContext,
        debug: &DebugBuilderEnum<'ink>,
    ) -> Result<(), Diagnostic> {
        let variables_with_initializers =
            variables.iter().filter(|it| it.is_local() || it.is_temp() || it.is_return());

        let exp_gen = ExpressionCodeGenerator::new_context_free(
            &self.llvm,
            self.index,
            self.annotations,
            local_llvm_index,
        );

        for variable in variables_with_initializers {
            //get the loaded_ptr for the parameter and store right in it
            if let Some(left) =
                local_llvm_index.find_loaded_associated_variable_value(variable.get_qualified_name())
            {
                if let Some(block) = self.llvm.builder.get_insert_block() {
                    debug.add_variable_declaration(
                        variable.get_qualified_name(),
                        left,
                        function_context.function,
                        block,
                        variable.source_location.get_line(),
                        variable.source_location.get_column(),
                    );
                }
                let right_stmt =
                    self.index.get_const_expressions().maybe_get_constant_statement(&variable.initial_value);
                self.generate_variable_initializer(variable, left, right_stmt, &exp_gen)?;
            } else {
                return Err(Diagnostic::cannot_generate_initializer(
                    variable.get_qualified_name(),
                    variable.source_location.clone(),
                ));
            }
        }
        Ok(())
    }

    /// initializes the variable represented by `variable` by storing into the given `variable_to_initialize` pointer using either
    /// the optional `initializer_statement` (hence code like: `variable : type := initializer_statement`), or determine the initial
    /// value with the help of the `variable`'s index entry by e.g. looking for a default value of the variable's type
    fn generate_variable_initializer(
        &self,
        variable: &&VariableIndexEntry,
        variable_to_initialize: PointerValue,
        initializer_statement: Option<&AstNode>,
        exp_gen: &ExpressionCodeGenerator,
    ) -> Result<(), Diagnostic> {
        let variable_llvm_type = self
            .llvm_index
            .get_associated_type(variable.get_type_name())
            .map_err(|err| err.with_location(&variable.source_location))?;

        let type_size = variable_llvm_type.size_of().ok_or_else(|| {
            Diagnostic::codegen_error("Couldn't determine type size", variable.source_location.clone())
        });

        // initialize the variable with the initial_value
        let variable_data_type = self.index.get_effective_type_or_void_by_name(variable.get_type_name());

        let v_type_info = variable_data_type.get_type_information();

        const DEFAULT_ALIGNMENT: u32 = 1;
        let (value, alignment) =
        // 1st try: see if there is a global variable with the right name - naming convention :-(
        if let Some(global_variable) =  self.llvm_index.find_global_value(&index::get_initializer_name(variable.get_qualified_name())) {
            (global_variable.as_basic_value_enum(), global_variable.get_alignment())
        // 2nd try: see if there is an initializer-statement
        } else if let Some(initializer) = initializer_statement {
            (exp_gen.generate_expression(initializer)?, DEFAULT_ALIGNMENT)
        // 3rd try: see if ther is a global variable with the variable's type name - naming convention :-(
        } else if let Some(global_variable) = self.llvm_index.find_global_value(&index::get_initializer_name(variable.get_type_name())) {
            (global_variable.as_basic_value_enum(), global_variable.get_alignment())
        // 4th try, see if the datatype has a default initializer
        } else if let Some(initial_value) = self.llvm_index.find_associated_initial_value(variable.get_type_name()) {
            (initial_value, DEFAULT_ALIGNMENT)
        // no inital value defined + array type - so we use a 0 byte the memset the array to 0
        }else if v_type_info.is_array() || v_type_info.is_string() {
            (self.llvm.context.i8_type().const_zero().as_basic_value_enum(), DEFAULT_ALIGNMENT)
        // no initial value defined + no-array
        } else {
            (get_default_for(variable_llvm_type), DEFAULT_ALIGNMENT)
        };

        let is_aggregate_type = variable_data_type.is_aggregate_type();
        let variable_to_initialize = if variable.is_return() && is_aggregate_type {
            //if this is an out-pointer we need to deref it first
            self.llvm.builder.build_load(variable_to_initialize, "deref").into_pointer_value()
        } else {
            variable_to_initialize
        };

        // initialize the variable with the initial_value
        if is_aggregate_type {
            // for arrays/structs, we prefere a memcpy, not a store operation
            // we assume that we got a global variable with the initial value that we can copy from
            let init_result: Result<(), &str> = if value.is_pointer_value() {
                // mem-copy from an global constant variable
                self.llvm
                    .builder
                    .build_memcpy(
                        variable_to_initialize,
                        std::cmp::max(1, alignment),
                        value.into_pointer_value(),
                        std::cmp::max(1, alignment),
                        type_size?,
                    )
                    .map(|_| ())
            } else if value.is_int_value() {
                // mem-set the value (usually 0) over the whole memory-area
                self.llvm
                    .builder
                    .build_memset(
                        variable_to_initialize,
                        std::cmp::max(1, alignment),
                        value.into_int_value(),
                        type_size?,
                    )
                    .map(|_| ())
            } else {
                unreachable!("initializing an array should be memcpy-able or memset-able");
            };
            init_result.map_err(|msg| Diagnostic::codegen_error(msg, variable.source_location.clone()))?;
        } else {
            self.llvm.builder.build_store(variable_to_initialize, value);
        }
        Ok(())
    }

    fn get_variadic_size_and_pointer(
        &self,
        variadic: Option<&'cg VariableIndexEntry>,
    ) -> Option<[BasicMetadataTypeEnum<'ink>; 2]> {
        if let (Some(var), Some(VarArgs::Sized(Some(type_name)))) =
            (variadic, variadic.and_then(VariableIndexEntry::get_varargs))
        {
            //Create a size parameter of type i32 (DINT)
            let size = self.llvm_index.find_associated_type(DINT_TYPE).map(Into::into)?;

            let ty = self
                .llvm_index
                .find_associated_type(type_name)
                .map(|it| {
                    if it.is_array_type() && var.get_declaration_type().is_by_ref() {
                        // the declaration for array types passed by ref are generatad as pointer to element type
                        // we need to match the declaration
                        it.into_array_type().get_element_type()
                    } else {
                        it
                    }
                })
                .map(|it| {
                    if var.get_declaration_type().is_by_ref() {
                        // variadic by ref will result in a double pointer
                        it.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).as_basic_type_enum()
                    } else {
                        it
                    }
                })?
                .ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC))
                .into();

            Some([size, ty])
        } else {
            None
        }
    }
}
