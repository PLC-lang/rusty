// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use super::{
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
        CodegenError,
    },
    index::{self, ImplementationType},
    resolver::{AstAnnotations, Dependency},
    typesystem::{DataType, DataTypeInformation, VarArgs, DINT_TYPE},
    OnlineChange,
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
    module::{Linkage, Module},
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType},
    values::{BasicValue, BasicValueEnum, FunctionValue},
    AddressSpace,
};
use inkwell::{types::StructType, values::PointerValue};
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
    online_change: &'cg OnlineChange,
}

/// Creates opaque implementations for all callable items in the index
/// Returns a Typed index containing the associated implementations.
/// FIXME: Ignoring the arguments warning for now, should refactor later to pass the options in a struct
#[allow(clippy::too_many_arguments)]
pub fn generate_implementation_stubs<'ink>(
    module: &Module<'ink>,
    llvm: Llvm<'ink>,
    dependencies: &FxIndexSet<Dependency>,
    index: &Index,
    annotations: &AstAnnotations,
    types_index: &LlvmTypedIndex<'ink>,
    debug: &mut DebugBuilderEnum<'ink>,
    online_change: &OnlineChange,
) -> Result<LlvmTypedIndex<'ink>, CodegenError> {
    let mut llvm_index = LlvmTypedIndex::default();
    let pou_generator = PouGenerator::new(llvm, index, annotations, types_index, online_change);
    let implementations = dependencies
        .into_iter()
        .filter_map(|it| match it {
            Dependency::Call(name) | Dependency::Datatype(name) => Some(name),
            _ => None,
        })
        .filter_map(|name| index.find_implementation_by_name(name).map(|it| (name, it)))
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
) -> Result<LlvmTypedIndex<'ink>, CodegenError> {
    let mut local_llvm_index = LlvmTypedIndex::default();
    let implementations = dependencies
        .into_iter()
        .filter_map(|it| match it {
            Dependency::Call(name) | Dependency::Datatype(name) => index.find_implementation_by_name(name),
            _ => None,
        })
        .filter(|it| it.is_in_unit(location));
    for implementation in implementations {
        let type_name = implementation.get_type_name();
        if implementation.is_init() {
            // initializer functions don't need global constants to initialize members
            continue;
        }
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
        online_change: &'cg OnlineChange,
    ) -> PouGenerator<'ink, 'cg> {
        PouGenerator { llvm, index, annotations, llvm_index, online_change }
    }

    fn mangle_function(&self, implementation: &ImplementationIndexEntry) -> Result<String, CodegenError> {
        let ctx = SectionMangler::function(implementation.get_call_name_for_ir().to_lowercase());

        let params = self.index.get_available_parameters(implementation.get_call_name());

        let ctx = params.into_iter().try_fold(ctx, |ctx, param| -> Result<SectionMangler, CodegenError> {
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
    ) -> Result<FunctionValue<'ink>, CodegenError> {
        let declared_parameters = self.index.get_available_parameters(implementation.get_call_name());
        let mut parameters = self.collect_parameters_for_implementation(implementation)?;
        // if we are handling a method, take the first parameter as the instance
        let instance = if implementation.is_method() { Some(parameters.remove(0)) } else { None };
        let mut parameters = parameters
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let param = declared_parameters.get(i);
                let dti = param.map(|it| self.index.get_type_information_or_void(it.get_type_name()));
                match param {
                    Some(v) if v.is_in_parameter_by_ref() => {
                        let ty = self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC));
                        let _ = new_llvm_index.associate_type(v.get_type_name(), ty.into());
                        ty.into()
                    }
                    _ => {
                        dti.map(|it| {
                            if !implementation.get_implementation_type().is_function_method_or_init() {
                                return *p;
                            }
                            // for aggregate function parameters we will generate a pointer instead of the value type.
                            // it will then later be memcopied into a locally allocated variable
                            match it {
                                DataTypeInformation::Struct { .. } => self
                                    .llvm
                                    .context
                                    .ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC))
                                    .into(),
                                DataTypeInformation::Array { .. } | DataTypeInformation::String { .. } => {
                                    self.llvm
                                        .context
                                        .ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC))
                                        .into()
                                }
                                _ => *p,
                            }
                        })
                        .unwrap_or(*p)
                    }
                }
            })
            .collect::<Vec<BasicMetadataTypeEnum>>();
        // insert the instance as the first parameter
        if let Some(instance) = instance {
            parameters.insert(0, instance);
        }

        let return_type = self
            .index
            .find_return_type(implementation.get_type_name())
            .and_then(|dt| self.index.find_effective_type(dt));

        // see if we need to adapt the parameters list
        let (return_type_llvm, parameters) = match return_type {
            // function with an intrinsic return-type
            Some(r_type) => (Some(self.llvm_index.get_associated_type(r_type.get_name())?), parameters),
            // no return
            None => (None, parameters),
        };

        let variadic = self.index.get_variadic_member(implementation.get_type_name());

        let function_declaration = self.create_llvm_function_type(parameters, variadic, return_type_llvm)?;

        let curr_f: FunctionValue<'_> =
            module.add_function(&implementation.get_call_name_for_ir(), function_declaration, None);

        let section_name = self.get_section(implementation)?;
        curr_f.set_section(section_name.as_deref());

        if implementation.get_implementation_type().is_project_init() {
            self.add_global_constructor(module, curr_f)?;
        }

        let mut parameter_types = declared_parameters
            .iter()
            .map(|v| self.index.get_effective_type_or_void_by_name(v.get_type_name()))
            .collect::<Vec<&DataType>>();

        // If the implementation is a method, the first parameter is the instance
        if let Some(class_name) = implementation.get_associated_class_name() {
            if let Some(class_type) = self.index.find_type(class_name) {
                parameter_types.insert(0, class_type);
            }
        } else if !implementation.get_implementation_type().is_function_method_or_init() {
            //For non functions or methods, the first parameter is self
            if let Some(type_name) = self.index.find_type(implementation.get_type_name()) {
                parameter_types.insert(0, type_name);
            }
        }

        let parent_function =
            implementation.get_associated_class_name().and_then(|it| module.get_function(it));
        let function_context = FunctionContext {
            linking_context: implementation,
            function: curr_f,
            blocks: FxHashMap::default(),
        };
        debug.register_function(
            (self.index, self.llvm_index),
            &function_context,
            return_type,
            parent_function,
            parameter_types.as_slice(),
            implementation.get_location().get_line(),
        );
        Ok(curr_f)
    }

    /// Generates a global constructors entry
    /// The entry contains the a call to the initializer function
    fn add_global_constructor(
        &self,
        module: &Module<'ink>,
        curr_f: FunctionValue<'ink>,
    ) -> Result<(), CodegenError> {
        //Create a constructor struct
        let ctor_str = self.llvm.context.struct_type(
            &[
                //Priority
                self.llvm.context.i32_type().as_basic_type_enum(),
                // Function pointer
                curr_f.as_global_value().as_basic_value_enum().get_type(),
                //Data
                self.llvm.context.ptr_type(AddressSpace::default()).as_basic_type_enum(),
            ],
            false,
        );

        //Create an entry for the global constructor of the project
        let str_value = ctor_str.const_named_struct(&[
            self.llvm.context.i32_type().const_zero().as_basic_value_enum(),
            curr_f.as_global_value().as_basic_value_enum(),
            self.llvm.context.ptr_type(AddressSpace::default()).const_zero().as_basic_value_enum(),
        ]);
        //Create an array with the global constructor as an entry
        let arr = ctor_str.const_array(&[str_value]);
        //Create the global constructors variable or fetch it and append to it if already
        //availabe
        let global_ctors = module.get_global("llvm.global_ctors").unwrap_or_else(|| {
            module.add_global(arr.get_type().as_basic_type_enum(), None, "llvm.global_ctors")
        });

        global_ctors.set_initializer(&arr);
        global_ctors.set_linkage(Linkage::Appending);
        Ok(())
    }
    /// creates and returns all parameters for the given implementation
    /// for functions, this method creates a full list of parameters, for other POUs
    /// this method creates a single state-struct parameter
    fn collect_parameters_for_implementation(
        &self,
        implementation: &ImplementationIndexEntry,
    ) -> Result<Vec<BasicMetadataTypeEnum<'ink>>, CodegenError> {
        if !implementation.implementation_type.is_function_method_or_init() {
            let mut parameters = vec![];
            let _instance_struct_type: StructType = self
                .llvm_index
                .get_associated_pou_type(implementation.get_type_name())
                .map(|it| it.into_struct_type())?;
            parameters.push(self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into());
            Ok(parameters)
        } else {
            let declared_params = self.index.get_available_parameters(implementation.get_call_name());
            //find the function's parameters
            let mut parameters = declared_params
                .iter()
                .map(|v| self.llvm_index.get_associated_type(v.get_type_name()).map(Into::into))
                .collect::<Result<Vec<BasicMetadataTypeEnum>, _>>()?;

            if implementation.get_implementation_type() == &ImplementationType::Method {
                let class_name =
                    implementation.get_associated_class_name().expect("Method needs to have a class-name");
                let _instance_members_struct_type: StructType =
                    self.llvm_index.get_associated_type(class_name).map(|it| it.into_struct_type())?;
                parameters
                    .insert(0, self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into());
            }

            Ok(parameters)
        }
    }

    /// generates a function for the given pou
    pub fn generate_implementation(
        &self,
        implementation: &Implementation,
        debug: &DebugBuilderEnum<'ink>,
    ) -> Result<(), CodegenError> {
        let context = self.llvm.context;
        let mut local_index = LlvmTypedIndex::create_child(self.llvm_index);

        let pou_name = &implementation.name;

        let current_function = self.llvm_index.find_associated_implementation(pou_name).ok_or_else(|| {
            CodegenError::new(
                format!("Could not find generated stub for {pou_name}"),
                &implementation.location,
            )
        })?;

        //Unset the debug location so we ignore initialization logic
        debug.unset_debug_location(&self.llvm);

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
                    CodegenError::new(
                        format!("Could not find implementation for {}", &implementation.name),
                        &implementation.location,
                    )
                },
            )?,
            function: current_function,
            blocks,
        };

        if let PouType::Method { .. } = implementation.pou_type {
            let class_name = implementation.type_name.split('.').collect::<Vec<&str>>()[0];
            self.generate_local_pou_variable_accessors(
                &mut local_index,
                class_name,
                &function_context,
                &implementation.location,
                debug,
            )?;
        }

        // generate local variables
        if implementation.pou_type.is_function_method_or_init() {
            self.generate_local_function_arguments_accessors(
                &mut local_index,
                &implementation.type_name,
                &function_context,
                debug,
            )?;
        } else {
            self.generate_local_pou_variable_accessors(
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
            let (line, column) = implementation
                .statements
                .first()
                .map(|it| (it.get_location().get_line_plus_one(), it.get_location().get_column()))
                .unwrap_or_else(|| {
                    (implementation.location.get_line_plus_one(), implementation.location.get_column())
                });
            //Set the debug location to the first statement in the body
            debug.set_debug_location(&self.llvm, &function_context, line, column);
            statement_gen.generate_body(&implementation.statements)?;
            //TODO the return statement should be lowered
            let line = implementation.end_location.get_line_plus_one();
            let column = implementation.end_location.get_column();
            debug.set_debug_location(&self.llvm, &function_context, line, column);
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
    ) -> Result<FunctionType<'ink>, CodegenError> {
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
            )
            .into()),
        }
    }

    /// generates a load-statement for the given members of a function
    fn generate_local_function_arguments_accessors(
        &self,
        index: &mut LlvmTypedIndex<'ink>,
        type_name: &str,
        function_context: &FunctionContext<'ink, '_>,
        debug: &DebugBuilderEnum<'ink>,
    ) -> Result<(), CodegenError> {
        let members = self.index.get_pou_members(type_name);
        //Generate reference to parameter
        // cannot use index from members because return and temp variables may not be considered for index in build_struct_gep
        // eagerly handle the return-variable
        let mut params_iter = function_context.function.get_param_iter();
        // if we are in a method, skip the first parameter (the instance)
        if matches!(function_context.linking_context.get_implementation_type(), ImplementationType::Method) {
            params_iter.next();
        }
        if let Some(ret_v) = members.iter().find(|it| it.is_return()) {
            let return_type = index.get_associated_type(ret_v.get_type_name())?;
            let return_variable = self.llvm.create_local_variable(type_name, &return_type)?;
            index.associate_loaded_local_variable(type_name, ret_v.get_name(), return_variable)?;
        }

        // handle all parameters (without return!)
        for m in members.iter().filter(|it| !(it.is_return() || it.is_var_external())) {
            let parameter_name = m.get_name();

            let (name, variable) = if m.is_parameter() {
                let ptr_value = params_iter
                    .next()
                    .ok_or_else(|| CodegenError::from(Diagnostic::missing_function(&m.source_location)))?;
                let member_type_name = m.get_type_name();
                let type_info = self.index.get_type_information_or_void(member_type_name);
                let ty = index.get_associated_type(member_type_name)?;
                let ptr = self.llvm.create_local_variable(m.get_name(), &ty)?;
                if let Some(block) = self.llvm.builder.get_insert_block() {
                    debug.add_variable_declaration(
                        m.get_qualified_name(),
                        ptr,
                        function_context,
                        block,
                        m.source_location.get_line(),
                        m.source_location.get_column(),
                    );
                }

                if matches!(m.argument_type, ArgumentType::ByVal(VariableType::Input))
                    && type_info.is_aggregate()
                {
                    // a by-value aggregate type => we need to memcpy the data into the local variable
                    let ty = self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC));
                    let bitcast = self.llvm.builder.build_bit_cast(ptr, ty, "bitcast")?.into_pointer_value();
                    let (size, alignment) = if let DataTypeInformation::String { size, encoding } = type_info
                    {
                        // since passed string args might be larger than the local acceptor, we need to first memset the local variable to 0
                        let size = size
                            .as_int_value(self.index)
                            .map_err(|err| CodegenError::new(err.as_str(), &m.source_location))?
                            as u64;
                        let char_width = encoding.get_bytes_per_char();
                        self.llvm.builder.build_memset(
                            bitcast,
                            char_width,
                            self.llvm.context.i8_type().const_zero(),
                            self.llvm.context.i64_type().const_int(size * char_width as u64, true),
                        )?;
                        (
                            // we then reduce the amount of bytes to be memcopied by the equivalent of one grapheme in bytes to preserve the null-terminator
                            self.llvm.context.i64_type().const_int((size - 1) * char_width as u64, true),
                            char_width,
                        )
                    } else {
                        let Some(size) = index.get_associated_type(member_type_name)?.size_of() else {
                            // XXX: can this still fail at this point? might be `unreachable`
                            return Err(CodegenError::new(
                                "Unable to determine type size",
                                &m.source_location,
                            ));
                        };
                        (size, 1)
                    };
                    self.llvm.builder.build_memcpy(
                        bitcast,
                        alignment,
                        ptr_value.into_pointer_value(),
                        alignment,
                        size,
                    )?;
                } else {
                    self.llvm.builder.build_store(ptr, ptr_value)?;
                };

                (parameter_name, ptr)
            } else {
                let temp_type = index.get_associated_type(m.get_type_name())?;
                let value = self.llvm.create_local_variable(parameter_name, &temp_type)?;
                (parameter_name, value)
            };

            index.associate_loaded_local_variable(type_name, name, variable)?;
        }

        Ok(())
    }

    /// generates a load-statement for the given members
    /// for pous that take a struct-state-variable
    fn generate_local_pou_variable_accessors(
        &self,
        index: &mut LlvmTypedIndex<'ink>,
        type_name: &str,
        function_context: &FunctionContext<'ink, '_>,
        location: &SourceLocation,
        debug: &DebugBuilderEnum<'ink>,
    ) -> Result<(), CodegenError> {
        let members = self.index.get_pou_members(type_name);
        let param_pointer = function_context
            .function
            .get_nth_param(0)
            .map(BasicValueEnum::into_pointer_value)
            .ok_or_else(|| CodegenError::from(Diagnostic::missing_function(location)))?;
        //Generate POU struct declaration for debug
        if let Some(block) = self.llvm.builder.get_insert_block() {
            debug.add_variable_declaration(
                type_name,
                param_pointer,
                function_context,
                block,
                location.get_line(),
                location.get_column(),
            );
        }

        if ((function_context.linking_context.is_method() || function_context.linking_context.is_action())
            && self.index.find_pou(type_name).is_some_and(|it| it.is_function_block()))
            || function_context.linking_context.get_implementation_type()
                == &ImplementationType::FunctionBlock
        {
            let alloca = self.llvm.builder.build_alloca(param_pointer.get_type(), "this")?;
            self.llvm.builder.build_store(alloca, param_pointer)?;
            index.associate_loaded_local_variable(type_name, "__THIS", alloca)?;
        }

        //Generate reference to parameter
        // cannot use index from members because return and temp variables may not be considered for index in build_struct_gep
        let mut var_count = 0;
        for m in members.iter().filter(|it| !it.is_var_external()) {
            let parameter_name = m.get_name();
            //TODO: this is not creating local variables
            let (name, variable) = if m.is_temp() || m.is_return() {
                let temp_type = index.get_associated_type(m.get_type_name())?;
                (parameter_name, self.llvm.create_local_variable(parameter_name, &temp_type)?)
            } else {
                let pointee = self.llvm_index.get_associated_pou_type(type_name).unwrap();
                let ptr = self
                    .llvm
                    .builder
                    .build_struct_gep(pointee, param_pointer, var_count as u32, parameter_name)
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
    ) -> Result<(), CodegenError> {
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
                        function_context,
                        block,
                        variable.source_location.get_line(),
                        variable.source_location.get_column(),
                    );
                }

                if self
                    .index
                    .find_effective_type_by_name(variable.get_type_name())
                    .map(|it| it.get_type_information())
                    .is_some_and(|it| it.is_reference_to() || it.is_alias())
                {
                    // aliases and reference to variables have special handling for initialization. initialize with a nullpointer
                    let pointee =
                        self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).const_null();
                    self.llvm.builder.build_store(left, pointee)?;
                    continue;
                };
                let right_stmt =
                    self.index.get_const_expressions().maybe_get_constant_statement(&variable.initial_value);
                self.generate_variable_initializer(variable, left, right_stmt, &exp_gen)?;
            } else {
                return Err(Diagnostic::cannot_generate_initializer(
                    variable.get_qualified_name(),
                    &variable.source_location,
                )
                .into());
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
    ) -> Result<(), CodegenError> {
        self.llvm.generate_variable_initializer(
            self.llvm_index,
            self.index,
            (variable.get_qualified_name(), variable.get_type_name(), &variable.source_location),
            variable_to_initialize,
            initializer_statement,
            exp_gen,
        )
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

            let _ = self.llvm_index.find_associated_type(type_name).map(|it| {
                if it.is_array_type() && var.get_declaration_type().is_by_ref() {
                    // the declaration for array types passed by ref are generatad as pointer to element type
                    // we need to match the declaration
                    it.into_array_type().get_element_type()
                } else {
                    it
                }
            })?;

            let ty = self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into();

            Some([size, ty])
        } else {
            None
        }
    }

    fn get_section(&self, implementation: &ImplementationIndexEntry) -> Result<Option<String>, CodegenError> {
        if self.online_change.is_enabled() {
            self.mangle_function(implementation).map(Some)
        } else {
            Ok(None)
        }
    }
}
