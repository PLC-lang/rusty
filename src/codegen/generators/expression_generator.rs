// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use inkwell::{
    builder::Builder,
    types::{BasicType, BasicTypeEnum},
    values::{
        ArrayValue, BasicMetadataValueEnum, BasicValue, BasicValueEnum, CallSiteValue, FloatValue, IntValue,
        PointerValue, ScalableVectorValue, StructValue, ValueKind, VectorValue,
    },
    AddressSpace, FloatPredicate, IntPredicate,
};
use rustc_hash::FxHashSet;

use plc_ast::{
    ast::{
        flatten_expression_list, Assignment, AstFactory, AstNode, AstStatement, DirectAccessType, Operator,
        ReferenceAccess, ReferenceExpr,
    },
    literals::AstLiteral,
    try_from,
};
use plc_diagnostics::diagnostics::{Diagnostic, INTERNAL_LLVM_ERROR};
use plc_source::source_location::SourceLocation;
use plc_util::convention::qualified_name;

use crate::{
    codegen::{
        debug::{Debug, DebugBuilderEnum},
        llvm_index::LlvmTypedIndex,
        llvm_typesystem::cast_if_needed,
        CodegenError,
    },
    index::{
        const_expressions::ConstId, ArgumentType, ImplementationIndexEntry, ImplementationType, Index,
        PouIndexEntry, VariableIndexEntry, VariableType,
    },
    resolver::{AnnotationMap, AstAnnotations, StatementAnnotation},
    typesystem::{
        self, is_same_type_class, DataType, DataTypeInformation, DataTypeInformationProvider, Dimension,
        StringEncoding, VarArgs, DEFAULT_STRING_LEN, DINT_TYPE, LINT_TYPE,
    },
};

use super::{llvm::Llvm, statement_generator::FunctionContext, ADDRESS_SPACE_CONST, ADDRESS_SPACE_GENERIC};

/// the generator for expressions
pub struct ExpressionCodeGenerator<'a, 'b> {
    pub llvm: &'b Llvm<'a>,
    pub index: &'b Index,
    pub(crate) annotations: &'b AstAnnotations,
    pub llvm_index: &'b LlvmTypedIndex<'a>,
    /// the current function to create blocks in
    pub function_context: Option<&'b FunctionContext<'a, 'b>>,
    /// The debug context used to create breakpoint information
    pub debug: &'b DebugBuilderEnum<'a>,

    /// the string-prefix to use for temporary variables
    pub temp_variable_prefix: String,
    /// the string-suffix to use for temporary variables
    pub temp_variable_suffix: String,

    // the function on how to obtain the the length to use for the string
    string_len_provider: fn(type_length_declaration: usize, actual_length: usize) -> usize,
}

/// context information to generate a parameter
#[derive(Debug)]
struct CallParameterAssignment<'a, 'b> {
    /// The named argument node of the function call (e.g. `foo(in := 3)`)
    assignment: &'b AstNode,

    /// The name of the POU being called
    function_name: &'b str,

    /// The position of the parameter in the POUs variable declaration list.
    ///
    /// See also [`StatementAnnotation::Argument::position`].
    position: u32,

    /// The depth between where the parameter is declared versus where it is being called from.
    ///
    /// See also [`StatementAnnotation::Argument::depth`].
    depth: u32,

    /// The name of the POU where the parameter is declared
    ///
    /// See also [`StatementAnnotation::Argument::pou`].
    declaring_pou: &'b str,

    /// The pointer to the struct instance that carries the call's arguments
    parameter_struct: PointerValue<'a>,
}

#[derive(Debug)]
pub enum ExpressionValue<'ink> {
    /// A Locator-Value
    /// An lvalue (locator value) represents an object that occupies some identifiable location in memory (i.e. has an address).
    LValue(PointerValue<'ink>, BasicTypeEnum<'ink>),
    /// An expression that does not represent an object occupying some identifiable location in memory.
    RValue(BasicValueEnum<'ink>),
}

impl<'ink> ExpressionValue<'ink> {
    /// returns the value represented by this ExpressionValue
    pub fn get_basic_value_enum(&self) -> BasicValueEnum<'ink> {
        match self {
            ExpressionValue::LValue(value, _) => value.as_basic_value_enum(),
            ExpressionValue::RValue(value) => value.to_owned(),
        }
    }

    /// returns the given expression value as an r-value which means that it will load
    /// the pointer, if this is an l_value
    pub fn as_r_value(
        &self,
        llvm: &Llvm<'ink>,
        load_name: Option<String>,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        match self {
            ExpressionValue::LValue(value, pointee) => {
                llvm.load_pointer(*pointee, value, load_name.as_deref().unwrap_or_default())
            }
            ExpressionValue::RValue(value) => Ok(value.to_owned()),
        }
    }
}

impl<'ink, 'b> ExpressionCodeGenerator<'ink, 'b> {
    /// creates a new expression generator
    ///
    /// - `llvm` dependencies used to generate llvm IR
    /// - `index` the index / global symbol table
    /// - `type_hint` an optional type hint for generating literals
    /// - `function_context` the current function to create blocks
    pub fn new(
        llvm: &'b Llvm<'ink>,
        index: &'b Index,
        annotations: &'b AstAnnotations,
        llvm_index: &'b LlvmTypedIndex<'ink>,
        function_context: &'b FunctionContext<'ink, 'b>,
        debug: &'b DebugBuilderEnum<'ink>,
    ) -> ExpressionCodeGenerator<'ink, 'b> {
        ExpressionCodeGenerator {
            llvm,
            index,
            llvm_index,
            annotations,
            function_context: Some(function_context),
            debug,
            temp_variable_prefix: "load_".to_string(),
            temp_variable_suffix: "".to_string(),
            string_len_provider: |_, actual_length| actual_length, //when generating string-literals in a body, use the actual length
        }
    }

    /// creates a new expression generator without a function context
    /// this expression generator cannot generate all expressions. It can only generate
    /// expressions that need no blocks (e.g. literals, references, etc.)
    ///
    /// - `llvm` dependencies used to generate llvm IR
    /// - `index` the index / global symbol table
    /// - `type_hint` an optional type hint for generating literals
    pub fn new_context_free(
        llvm: &'b Llvm<'ink>,
        index: &'b Index,
        annotations: &'b AstAnnotations,
        llvm_index: &'b LlvmTypedIndex<'ink>,
    ) -> ExpressionCodeGenerator<'ink, 'b> {
        ExpressionCodeGenerator {
            llvm,
            index,
            llvm_index,
            annotations,
            function_context: None,
            debug: &DebugBuilderEnum::None,
            temp_variable_prefix: "load_".to_string(),
            temp_variable_suffix: "".to_string(),
            string_len_provider: |type_length_declaration, _| type_length_declaration, //when generating string-literals in declarations, use the declared length
        }
    }

    /// returns the function context or returns a Compile-Error
    pub fn get_function_context(
        &self,
        statement: &AstNode,
    ) -> Result<&'b FunctionContext<'ink, 'b>, CodegenError> {
        self.function_context.ok_or_else(|| CodegenError::missing_function(statement.get_location().clone()))
    }

    /// entry point into the expression generator.
    /// generates the given expression and returns the resulting BasicValueEnum
    pub fn generate_expression(&self, expression: &AstNode) -> Result<BasicValueEnum<'ink>, CodegenError> {
        // If the expression was replaced by the resolver, generate the replacement
        if let Some(StatementAnnotation::ReplacementAst { statement }) = self.annotations.get(expression) {
            // we trust that the validator only passed us valid parameters (so left & right should be same type)
            return self.generate_expression(statement);
        }

        let v = self
            .generate_expression_value(expression)?
            .as_r_value(self.llvm, self.get_load_name(expression))?
            .as_basic_value_enum();

        let Some(target_type) = self.annotations.get_type_hint(expression, self.index) else {
            // no type-hint -> we can return the value as is
            return Ok(v);
        };
        let actual_type = self.annotations.get_type_or_void(expression, self.index);
        cast_if_needed!(self, target_type, actual_type, v, self.annotations.get(expression))
    }

    pub fn generate_expression_with_cast_to_type_of_secondary_expression(
        &self,
        expression: &AstNode,
        target_expression: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        if let Some(target_type) = self.annotations.get_type_hint(target_expression, self.index) {
            return cast_if_needed!(
                self,
                target_type,
                self.get_type_hint_for(expression)?,
                self.generate_expression(expression)?,
                None
            );
        }

        self.generate_expression(expression)
    }

    fn register_debug_location(&self, statement: &AstNode) {
        let function_context =
            self.function_context.expect("Cannot generate debug info without function context");
        let line = statement.get_location().get_line_plus_one();
        let column = statement.get_location().get_column();
        self.debug.set_debug_location(self.llvm, function_context, line, column);
    }

    pub fn generate_expression_value(
        &self,
        expression: &AstNode,
    ) -> Result<ExpressionValue<'ink>, CodegenError> {
        //see if this is a constant - maybe we can short curcuit this codegen
        if let Some(StatementAnnotation::Variable {
            qualified_name, constant: true, resulting_type, ..
        }) = self.annotations.get(expression)
        {
            if !self.index.get_type_information_or_void(resulting_type).is_aggregate() {
                match self.generate_constant_expression(qualified_name, expression) {
                    // We return here if constant propagation worked
                    Ok(expr) => return Ok(expr),
                    // ...and fall-back to generating the expression further down if it didn't
                    Err(why) => log::info!("{why}"),
                }
            }
        }

        // generate the expression
        match expression.get_stmt() {
            AstStatement::This => {
                self.function_context.ok_or_else(|| {
                    Diagnostic::codegen_error("Cannot use 'this' without context", expression)
                })?;
                let Some(this_name) = self.annotations.get_call_name(expression) else {
                    unreachable!("this should have a name");
                };
                let this_value =
                    self.llvm_index.find_loaded_associated_variable_value(this_name).ok_or_else(|| {
                        let message = format!("Cannot find '{}' in associated variable values", this_name);
                        Diagnostic::codegen_error(message, expression)
                    })?;

                let pointee = self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into();
                Ok(ExpressionValue::LValue(this_value, pointee))
            }
            AstStatement::ReferenceExpr(data) => {
                let res =
                    self.generate_reference_expression(&data.access, data.base.as_deref(), expression)?;
                let val = match res {
                    ExpressionValue::LValue(value, _) => {
                        let value = self.auto_deref_if_necessary(value, expression)?;
                        let pointee = {
                            let datatype = self.annotations.get_type(expression, self.index).unwrap();
                            let effective_ty = self.index.find_effective_type(datatype).unwrap_or(datatype);
                            self.llvm_index.get_associated_type(&effective_ty.name).unwrap()
                        };

                        ExpressionValue::LValue(value, pointee)
                    }
                    ExpressionValue::RValue(val) => {
                        let val = if val.is_pointer_value() {
                            self.auto_deref_if_necessary(val.into_pointer_value(), expression)?
                                .as_basic_value_enum()
                        } else {
                            val
                        };
                        ExpressionValue::RValue(val)
                    }
                };
                Ok(val)
            }
            AstStatement::HardwareAccess(..) => {
                let value = self.create_llvm_pointer_value_for_reference(None, "address", expression)?;
                let pointee = {
                    let datatype = self.annotations.get_type(expression, self.index).unwrap();
                    self.llvm_index.get_associated_type(datatype.get_name()).unwrap()
                };

                Ok(ExpressionValue::LValue(value, pointee))
            }

            AstStatement::BinaryExpression(data) => self
                .generate_binary_expression(&data.left, &data.right, &data.operator, expression)
                .map(ExpressionValue::RValue),
            AstStatement::CallStatement(data) => {
                self.generate_call_statement(&data.operator, data.parameters.as_deref())
            }
            AstStatement::UnaryExpression(data) => {
                self.generate_unary_expression(&data.operator, &data.value).map(ExpressionValue::RValue)
            }
            AstStatement::ParenExpression(expr) => self.generate_expression_value(expr),
            //fallback
            _ => self.generate_literal(expression),
        }
    }

    /// Propagate the constant value of the constant reference to  `qualified_name`.
    /// - `qualified _name` the qualified name of the referenced constant variable we want to propagate
    /// - `expression` the original expression
    fn generate_constant_expression(
        &self,
        qualified_name: &str,
        expression: &AstNode,
    ) -> Result<ExpressionValue<'ink>, CodegenError> {
        let const_expression = self
            .index
            // try to find a constant variable
            .find_variable(None, &qualified_name.split('.').collect::<Vec<_>>())
            // or else try to find an enum element
            .or_else(|| self.index.find_enum_variant_by_qualified_name(qualified_name))
            // if this is no constant we have a problem
            .filter(|v| v.is_constant())
            .and_then(|v| v.initial_value)
            // fetch the constant's initial value fron the const-expressions arena
            .and_then(|constant_variable| {
                self.index.get_const_expressions().get_resolved_constant_statement(&constant_variable)
            })
            .ok_or_else(|| {
                // We'll _probably_ land here because we're dealing with aggregate types, see also
                // https://github.com/PLC-lang/rusty/issues/288
                let message = format!("Cannot propagate constant value for '{qualified_name:}'");
                Diagnostic::codegen_error(message, expression)
            })?;

        //  generate the resulting constant-expression (which should be a Value, no ptr-reference)
        self.generate_expression_value(const_expression)
    }

    /// generates a binary expression (e.g. a + b, x AND y, etc.) and returns the resulting `BasicValueEnum`
    /// - `left` the AstStatement left of the operator
    /// - `right` the AstStatement right of the operator
    /// - `operator` the binary expression's operator
    /// - `expression` the whole expression for diagnostic reasons
    fn generate_binary_expression(
        &self,
        left: &AstNode,
        right: &AstNode,
        operator: &Operator,
        expression: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let l_type_hint = self.get_type_hint_for(left)?;
        let ltype = self.index.get_intrinsic_type_by_name(l_type_hint.get_name()).get_type_information();
        let r_type_hint = self.get_type_hint_for(right)?;
        let rtype = self.index.get_intrinsic_type_by_name(r_type_hint.get_name()).get_type_information();
        if ltype.is_bool() && rtype.is_bool() {
            return self.generate_bool_binary_expression(operator, left, right);
        }
        if ltype.is_int() && rtype.is_int() {
            self.create_llvm_int_binary_expression(
                operator,
                self.generate_expression(left)?,
                self.generate_expression(right)?,
            )
        } else if ltype.is_float() && rtype.is_float() {
            self.create_llvm_float_binary_expression(
                operator,
                self.generate_expression(left)?,
                self.generate_expression(right)?,
            )
        } else if (ltype.is_pointer() && rtype.is_int())
            || (ltype.is_int() && rtype.is_pointer())
            || (ltype.is_pointer() && rtype.is_pointer())
        {
            self.create_llvm_binary_expression_for_pointer(operator, left, ltype, right, rtype, expression)
        } else {
            self.create_llvm_generic_binary_expression(left, right, expression)
        }
    }

    pub fn generate_direct_access_index(
        &self,
        access: &DirectAccessType,
        index: &AstNode,
        access_type: &DataTypeInformation,
        target_type: &DataType,
    ) -> Result<IntValue<'ink>, CodegenError> {
        let reference = self.generate_expression(index)?;
        //Load the reference
        if reference.is_int_value() {
            //This cast is needed to convert the index/reference to the type of original expression
            //being accessed.
            //The reason is that llvm expects a shift operation to happen on the same type, and
            //this is what the direct access will eventually end up in.
            let reference =
                cast_if_needed!(self, target_type, self.get_type_hint_for(index)?, reference, None)?
                    .into_int_value();
            //Multiply by the bitwitdh
            if access.get_bit_width() > 1 {
                let bitwidth =
                    reference.get_type().const_int(access.get_bit_width(), access_type.is_signed_int());

                Ok(self.llvm.builder.build_int_mul(reference, bitwidth, "")?)
            } else {
                Ok(reference)
            }
        } else {
            Err(Diagnostic::new(format!("Cannot cast from {} to Integer Type", access_type.get_name()))
                .with_error_code("E051")
                .with_location(index)
                .into())
        }
    }

    /// generates a Unary-Expression e.g. -<expr> or !<expr>
    fn generate_unary_expression(
        &self,
        unary_operator: &Operator,
        expression: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let value = match unary_operator {
            Operator::Not => {
                let operator = self.generate_expression(expression)?.into_int_value();
                let operator = if self
                    .get_type_hint_for(expression)
                    .map(|it| it.get_type_information().is_bool())
                    .unwrap_or_default()
                {
                    to_i1(operator, &self.llvm.builder)?
                } else {
                    operator
                };

                Ok(self.llvm.builder.build_not(operator, "tmpVar")?.as_basic_value_enum())
            }
            Operator::Minus => {
                let generated_exp = self.generate_expression(expression)?;
                if generated_exp.is_float_value() {
                    Ok(self
                        .llvm
                        .builder
                        .build_float_neg(generated_exp.into_float_value(), "tmpVar")?
                        .as_basic_value_enum())
                } else if generated_exp.is_int_value() {
                    Ok(self
                        .llvm
                        .builder
                        .build_int_neg(generated_exp.into_int_value(), "tmpVar")?
                        .as_basic_value_enum())
                } else {
                    Err(Diagnostic::codegen_error("Negated expression must be numeric", expression).into())
                }
            }
            Operator::Plus => self.generate_expression(expression),
            _ => unimplemented!(),
        };
        value
    }

    /// generates the given call-statement <operator>(<parameters>)
    /// returns the call's result as a BasicValueEnum (may be a void-type for PROGRAMs)
    ///
    /// - `operator` - the expression that points to the callable instance (e.g. a PROGRAM, FUNCTION or FUNCTION_BLOCK instance)
    /// - `parameters` - an optional StatementList of parameters
    pub fn generate_call_statement(
        &self,
        operator: &AstNode,
        parameters: Option<&AstNode>,
    ) -> Result<ExpressionValue<'ink>, CodegenError> {
        if self.annotations.get(operator).is_some_and(StatementAnnotation::is_fnptr) {
            return self.generate_fnptr_call(operator, parameters).map_err(Into::into);
        }

        // find the pou we're calling
        let pou = self.annotations.get_call_name(operator).zip(self.annotations.get_qualified_name(operator))
            .and_then(|(call_name, qualified_name)| {
                self.index.find_pou(call_name)
                    // for some functions (builtins) the call name does not exist in the index, we try to call with the originally defined generic functions
                    .or_else(|| self.index.find_pou(qualified_name))
            })
            .or_else(||
                // some rare situations have a callstatement that's not properly annotated (e.g. checkRange-call of ranged datatypes)
                if let Some(name) = operator.get_flat_reference_name() {
                    self.index.find_pou(name)
                } else {
                    None
                })
            .ok_or_else(|| Diagnostic::cannot_generate_call_statement(operator))?;

        // find corresponding implementation
        let implementation = pou
            .find_implementation(self.index)
            .ok_or_else(|| Diagnostic::cannot_generate_call_statement(operator))?;

        let parameters_list = parameters.map(flatten_expression_list).unwrap_or_default();
        let implementation_name = &implementation.get_call_name();
        // if the function is builtin, generate a basic value enum for it
        if let Some(builtin) = self.index.get_builtin_function(implementation_name) {
            // adr, ref, etc.
            return builtin.codegen(self, parameters_list.as_slice(), operator.get_location());
        }

        let arguments_list = self.generate_pou_call_arguments_list(
            pou,
            parameters_list.as_slice(),
            implementation,
            operator,
            self.get_function_context(operator)?,
        )?;

        let function = self
            .llvm_index
            .find_associated_implementation(implementation_name) // using the non error option to control the output error
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    format!("No callable implementation associated to {implementation_name:?}"),
                    operator,
                )
            })?;

        // generate the debug statetment for a call
        self.register_debug_location(operator);

        // Check for the function within the GOT. If it's there, we need to generate an indirect
        // call to its location within the GOT, which should contain a function pointer.
        // First get the function type so our function pointer can have the correct type.
        let call = self.llvm.builder.build_call(function, &arguments_list, "call")?;

        // if the target is a function, declare the struct locally
        // assign all parameters into the struct values

        // so grab either:
        // - the call's return value
        // - or a null-ptr
        let value = match call.try_as_basic_value() {
            ValueKind::Basic(value) => value,
            ValueKind::Instruction(_) => self
                .llvm
                .context
                .ptr_type(AddressSpace::from(ADDRESS_SPACE_CONST))
                .const_null()
                .as_basic_value_enum(),
        };

        // after the call we need to copy the values for assigned outputs
        // this is only necessary for outputs defined as `rusty::index::ArgumentType::ByVal` (PROGRAM, FUNCTION_BLOCK)
        // FUNCTION outputs are defined as `rusty::index::ArgumentType::ByRef` // FIXME(mhasel): for standard-compliance functions also need to support VAR_OUTPUT
        if !(pou.is_function() || pou.is_method()) {
            let parameter_struct = match arguments_list.first() {
                Some(v) => v.into_pointer_value(),
                None => self.generate_lvalue(operator)?,
            };

            self.assign_output_values(parameter_struct, implementation_name, parameters_list)?
        }

        Ok(ExpressionValue::RValue(value))
    }

    fn generate_fnptr_call(
        &self,
        operator: &AstNode,
        arguments: Option<&AstNode>,
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
        let Some(ReferenceExpr { base: Some(ref base), .. }) = operator.get_deref_expr() else {
            // XXX: This would fail for auto-deref pointers, but given (for now) function pointers are never
            // auto-derefed this should be fine.
            unreachable!("internal error, invalid method call")
        };

        let qualified_pou_name = self.annotations.get(operator).unwrap().qualified_name().unwrap();
        let impl_entry = self.index.find_implementation_by_name(qualified_pou_name).unwrap();
        debug_assert!(
            impl_entry.is_method() | impl_entry.is_function_block(),
            "internal error, invalid method call"
        );

        // Get the associated variable then load it, e.g. `%localFnPtrVariable = alloca void (%Fb*)*, align 8`
        // followed by `%1 = load void (%Fb*)*, void (%Fb*)** %localFnPtrVariable, align 8``
        let function_pointer_value = match self.generate_expression_value(base)? {
            ExpressionValue::LValue(value, pointee) => {
                self.llvm.load_pointer(pointee, &value, "")?.into_pointer_value()
            }
            ExpressionValue::RValue(_) => unreachable!("fnptr base must be an lvalue"),
        };

        // Generate the argument list; our assumption is function pointers are only supported for methods and
        // direct function block calls, hence we explicitly fetch the instance argument from the list. In
        // terms of lowered ST code you can imagine something alike `fnPtr^(instanceFb, arg1, ..., argN)`
        let (instance, arguments_raw, arguments_llvm) = {
            let arguments = arguments.map(flatten_expression_list).unwrap_or_default();
            let (instance, arguments) = match arguments.len() {
                0 => panic!("invalid lowered code, no instance argument found"),
                1 => (self.generate_lvalue(arguments[0])?, Vec::new()),
                _ => (self.generate_lvalue(arguments[0])?, arguments[1..].to_vec()),
            };

            let mut generated_arguments = match &impl_entry.implementation_type {
                ImplementationType::Method => self.generate_function_arguments(
                    self.index.find_pou(qualified_pou_name).unwrap(),
                    &arguments,
                    self.index.get_available_parameters(qualified_pou_name),
                )?,

                // Function Block body calls have a slightly different calling convention compared to regular
                // methods. Specifically the arguments aren't passed to the call itself but rather the
                // instance struct is gep'ed and the arguments are stored into the gep'ed pointer value.
                // Best to call `--ir` on a simple function block body call to see the generated IR.
                ImplementationType::FunctionBlock => {
                    self.generate_stateful_pou_arguments(qualified_pou_name, None, instance, &arguments)?;
                    Vec::new()
                }

                _ => unreachable!("internal error, invalid method call"),
            };

            generated_arguments.insert(0, instance.clone().as_basic_value_enum().into());
            (instance, arguments, generated_arguments)
        };

        // Finally generate the function pointer call.
        // We get the function type from the associated implementation stub in the llvm_index,
        // and use it together with the loaded function pointer value.
        let function_value =
            self.llvm_index.find_associated_implementation(qualified_pou_name).ok_or_else(|| {
                Diagnostic::codegen_error(
                    format!("No callable implementation associated to {qualified_pou_name:?}"),
                    operator,
                )
            })?;
        let function_type = function_value.get_type();

        let call: CallSiteValue = self
            .llvm
            .builder
            .build_indirect_call(function_type, function_pointer_value, &arguments_llvm, "fnptr_call")
            .map_err(CodegenError::from)?;

        let value = match call.try_as_basic_value() {
            ValueKind::Basic(value) => value,
            ValueKind::Instruction(_) => self
                .llvm
                .context
                .ptr_type(AddressSpace::from(ADDRESS_SPACE_CONST))
                .const_null()
                .as_basic_value_enum(),
        };

        // Output variables are assigned after the function block call, effectively gep'ing the instance
        // struct fetching the output values
        if impl_entry.is_function_block() {
            self.assign_output_values(instance, qualified_pou_name, arguments_raw)?
        }

        Ok(ExpressionValue::RValue(value))
    }

    /// copies the output values to the assigned output variables
    /// - `parameter_struct` a pointer to a struct-instance that holds all function-parameters
    /// - `function_name` the name of the callable
    /// - `parameters` vec of passed parameters to the call
    fn assign_output_values(
        &self,
        parameter_struct: PointerValue<'ink>,
        function_name: &str,
        parameters: Vec<&AstNode>,
    ) -> Result<(), CodegenError> {
        let pou_info = self.index.get_available_parameters(function_name);
        let implicit = arguments_are_implicit(&parameters);

        for (index, assignment_statement) in parameters.into_iter().enumerate() {
            let is_output = pou_info.get(index).is_some_and(|param| param.get_variable_type().is_output());

            if assignment_statement.is_output_assignment() || (implicit && is_output) {
                let Some(StatementAnnotation::Argument { position, depth, pou, .. }) =
                    self.annotations.get_hint(assignment_statement)
                else {
                    unreachable!("must have an annotation")
                };

                self.assign_output_value(&CallParameterAssignment {
                    assignment: assignment_statement,
                    function_name,
                    position: *position as u32,
                    depth: *depth as u32,
                    declaring_pou: pou,
                    parameter_struct,
                })?
            }
        }

        Ok(())
    }

    fn assign_output_value(&self, param_context: &CallParameterAssignment) -> Result<(), CodegenError> {
        match &param_context.assignment.stmt {
            AstStatement::OutputAssignment(assignment) => {
                self.generate_explicit_output_assignment(param_context, assignment)
            }

            _ => self.generate_output_assignment(param_context),
        }
    }

    pub fn generate_assignment_with_direct_access(
        &self,
        left_statement: &AstNode,
        left_value: IntValue,
        left_pointer: PointerValue,
        right_type: &DataType,
        right_expr: BasicValueEnum,
    ) -> Result<(), CodegenError> {
        let Some((target, access_sequence)) = collect_base_and_direct_access_for_assignment(left_statement)
        else {
            unreachable!("Invalid direct-access expression: {left_statement:#?}")
        };

        let type_left = self.get_type_hint_for(target)?;
        let type_right =
            if let DataTypeInformation::Integer { semantic_size: Some(typesystem::U1_SIZE), .. } =
                *right_type.get_type_information()
            {
                // we need to switch to a faked u1 type, when dealing with a single bit
                self.index.get_type_or_panic(typesystem::U1_TYPE)
            } else {
                right_type
            };

        let Some((element, direct_access)) = access_sequence.split_first() else { unreachable!("") };

        // Build index
        let mut index = if let AstStatement::DirectAccess(data, ..) = element.get_stmt() {
            self.generate_direct_access_index(
                &data.access,
                &data.index,
                type_right.get_type_information(),
                type_left,
            )
        } else {
            // TODO: using the global context we could get a slice here; currently not possible because the
            //       global context isn't passed into codegen
            Err(Diagnostic::new(format!("{element:?} not a direct access"))
                .with_error_code("E055")
                .with_location(*element)
                .into())
        }?;
        for element in direct_access {
            let rhs_next = if let AstStatement::DirectAccess(data, ..) = element.get_stmt() {
                self.generate_direct_access_index(
                    &data.access,
                    &data.index,
                    type_right.get_type_information(),
                    type_left,
                )
            } else {
                // TODO: using the global context we could get a slice here; currently not possible because the
                //       global context isn't passed into codegen
                Err(Diagnostic::new(format!("{element:?} not a direct access"))
                    .with_error_code("E055")
                    .with_location(*element)
                    .into())
            }?;
            index = self.llvm.builder.build_int_add(index, rhs_next, "")?;
        }

        //Build mask for the index
        //Get the target bit type as all ones
        let rhs_type = self.llvm_index.get_associated_type(type_right.get_name())?.into_int_type();
        let ones = rhs_type.const_all_ones();

        //Extend the mask to the target type
        let extended_mask = self.llvm.builder.build_int_z_extend(ones, left_value.get_type(), "ext")?;
        //Position the ones in their correct locations
        let shifted_mask = self.llvm.builder.build_left_shift(extended_mask, index, "shift")?;
        //Invert the mask
        let mask = self.llvm.builder.build_not(shifted_mask, "invert")?;
        //And the result with the mask to erase the set bits at the target location
        let and_value = self.llvm.builder.build_and(left_value, mask, "erase")?;

        //Cast the right side to the left side type
        let lhs = cast_if_needed!(self, type_left, type_right, right_expr, None)?.into_int_value();
        //Shift left by the direct access
        let value = self.llvm.builder.build_left_shift(lhs, index, "value")?;

        //OR the result and store it in the left side
        let or_value = self.llvm.builder.build_or(and_value, value, "or")?;
        self.llvm.builder.build_store(left_pointer, or_value)?;

        Ok(())
    }

    fn generate_output_assignment_with_direct_access(
        &self,
        left_statement: &AstNode,
        left_pointer: PointerValue,
        left_type: &DataType,
        right_pointer: PointerValue,
        right_type: &DataType,
    ) -> Result<(), CodegenError> {
        let pointee = self.llvm_index.get_associated_type(&left_type.name).unwrap();
        let left_value = self.llvm.builder.build_load(pointee, left_pointer, "")?.into_int_value();

        //Generate an expression for the right size
        let pointee = self.llvm_index.get_associated_type(&right_type.name).unwrap();
        let right = self.llvm.builder.build_load(pointee, right_pointer, "")?;
        self.generate_assignment_with_direct_access(
            left_statement,
            left_value,
            left_pointer,
            right_type,
            right,
        )?;

        Ok(())
    }

    fn build_parameter_struct_gep(
        &self,
        pointee: BasicTypeEnum<'ink>,
        context: &CallParameterAssignment<'ink, 'b>,
    ) -> PointerValue<'ink> {
        // Assuming we have an inheritance hierarchy of `A <- B <- C` and a call `objC(inA := 1, ...)`
        // then in order to access `inA` in `C` we have to generate some IR of form `objC.__B.__A.inA`.
        // Because the parent structs are always the first member of these function blocks / classes, we
        // can simply generate a GEP with indices `0` and repeat that `depth` times followed by the
        // actual position of the parameter. So `objC.__B.__A.inA` becomes `GEP objC, 0, 0, 0, <inA pos>`
        let mut gep_index = vec![0; (context.depth + 1) as usize];
        gep_index.push(context.position as u64);

        let i32_type = self.llvm.context.i32_type();
        let gep_index = gep_index.into_iter().map(|idx| i32_type.const_int(idx, false)).collect::<Vec<_>>();

        unsafe {
            self.llvm.builder.build_in_bounds_gep(pointee, context.parameter_struct, &gep_index, "").unwrap()
        }
    }

    fn generate_output_assignment(&self, context: &CallParameterAssignment) -> Result<(), CodegenError> {
        let &CallParameterAssignment {
            assignment: expr,
            function_name,
            position: index,
            depth: _,
            declaring_pou,
            parameter_struct,
        } = context;

        let builder = &self.llvm.builder;

        // We don't want to generate any code if the right side of an assignment is empty, e.g. `foo(out =>)`
        if expr.is_empty_statement() {
            return Ok(());
        }

        let parameter = self.index.get_declared_parameter(declaring_pou, index).expect("must exist");

        match expr.get_stmt() {
            AstStatement::ReferenceExpr(_) if expr.has_direct_access() => {
                let rhs_type = {
                    let pou = self.index.find_pou(function_name).unwrap();
                    let pou_struct = &pou.find_instance_struct_type(self.index).unwrap().information;
                    let DataTypeInformation::Struct { members, .. } = pou_struct else { unreachable!() };

                    self.index.find_effective_type_by_name(&members[index as usize].data_type_name).unwrap()
                };

                let AstStatement::ReferenceExpr(ReferenceExpr {
                    access: ReferenceAccess::Member(member),
                    base,
                }) = &expr.get_stmt()
                else {
                    unreachable!("must be a bitaccess, will return early for all other cases")
                };

                if let AstStatement::DirectAccess(_) = member.as_ref().get_stmt() {
                    // Given `foo.bar.baz.%W1.%B1.%X3`, we want to grab the base i.e. `foo.bar.baz`
                    let (Some(base), _) = (base, ..) else { panic!() };
                    let (base, _) = collect_base_and_direct_access_for_assignment(base).unwrap();

                    let lhs = self.generate_expression_value(base)?.get_basic_value_enum();
                    let lhs_type = self.annotations.get_type(base, self.index).unwrap();

                    let pointee = self.llvm_index.get_associated_pou_type(function_name).unwrap();
                    let rhs =
                        self.llvm.builder.build_struct_gep(pointee, parameter_struct, index, "").unwrap();

                    // func(outVar => foo.bar.baz.%W3)
                    //      ^^^^^^    ^^^^^^^^^^^^^^^
                    //      rhs       lhs             => foo.bar.baz.%W3 = outVar;
                    self.generate_output_assignment_with_direct_access(
                        expr,
                        lhs.into_pointer_value(),
                        lhs_type,
                        rhs,
                        rhs_type,
                    )?;
                };
            }

            _ => {
                let assigned_output = self.generate_lvalue(expr)?;
                let assigned_output_type =
                    self.annotations.get_type_or_void(expr, self.index).get_type_information();

                let pou_type_name = self
                    .index
                    .find_pou(function_name)
                    .and_then(|pou| pou.get_instance_struct_type_name())
                    .unwrap_or(function_name);
                let pointee = self.llvm_index.get_associated_pou_type(pou_type_name).unwrap();
                let output = self.build_parameter_struct_gep(pointee, context);

                let output_value_type = self.index.get_type_information_or_void(parameter.get_type_name());

                if assigned_output_type.is_aggregate() && output_value_type.is_aggregate() {
                    self.build_memcpy(
                        assigned_output,
                        assigned_output_type,
                        expr.get_location(),
                        output,
                        output_value_type,
                        parameter.source_location.clone(),
                    )?;
                } else {
                    let pointee = self.llvm_index.get_associated_type(output_value_type.get_name()).unwrap();
                    let output_value = builder.build_load(pointee, output, "")?;
                    builder.build_store(assigned_output, output_value)?;
                }
            }
        };

        Ok(())
    }

    fn generate_explicit_output_assignment(
        &self,
        param_context: &CallParameterAssignment,
        assignment: &Assignment,
    ) -> Result<(), CodegenError> {
        let parameter_struct = param_context.parameter_struct;
        let function_name = param_context.function_name;
        let Assignment { left, right } = assignment;

        if let Some(StatementAnnotation::Variable { .. }) = self.annotations.get(left) {
            self.generate_output_assignment(&CallParameterAssignment {
                assignment: right,
                function_name,
                position: param_context.position,
                depth: param_context.depth,
                parameter_struct,
                declaring_pou: param_context.declaring_pou,
            })?
        };

        Ok(())
    }

    /// generates the argument list for a call to a pou
    /// a call to a function returns a Vec with all parameters for the function,
    /// a call to a Program/Fb will return a Vec with a single struct carrying all parameters
    fn generate_pou_call_arguments_list(
        &self,
        pou: &PouIndexEntry,
        passed_parameters: &[&AstNode],
        implementation: &ImplementationIndexEntry,
        operator: &AstNode,
        function_context: &'b FunctionContext<'ink, 'b>,
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, CodegenError> {
        match pou {
            PouIndexEntry::Function { .. } => {
                // we're calling a function
                let declared_parameters = self.index.get_available_parameters(implementation.get_type_name());
                self.generate_function_arguments(pou, passed_parameters, declared_parameters)
            }
            PouIndexEntry::Method { .. } => {
                let class_ptr = self.generate_lvalue(operator).or_else(|_| {
                    // this might be a local method
                    function_context
                        .function
                        .get_first_param()
                        .map(|class_ptr| class_ptr.into_pointer_value())
                        .ok_or_else(|| Diagnostic::cannot_generate_call_statement(operator))
                })?;
                let declared_parameters = self.index.get_available_parameters(implementation.get_type_name());
                let mut parameters =
                    self.generate_function_arguments(pou, passed_parameters, declared_parameters)?;
                parameters.insert(0, class_ptr.into());
                Ok(parameters)
            }
            PouIndexEntry::Action { .. }
                if try_from!(operator, ReferenceExpr).is_some_and(|it| it.base.is_none()) =>
            {
                // special handling for local actions, get the parameter from the function context
                let call_ptr = function_context
                    .function
                    .get_first_param()
                    .map(|call_ptr| call_ptr.into_pointer_value())
                    .ok_or_else(|| Diagnostic::cannot_generate_call_statement(operator))?;

                self.generate_stateful_pou_arguments(
                    implementation.get_type_name(),
                    None,
                    call_ptr,
                    passed_parameters,
                )
            }
            _ => {
                let call_ptr = self.generate_lvalue(operator)?;
                self.generate_stateful_pou_arguments(
                    implementation.get_type_name(),
                    None,
                    call_ptr,
                    passed_parameters,
                )
            }
        }
    }

    fn generate_function_arguments(
        &self,
        pou: &PouIndexEntry,
        arguments: &[&AstNode],
        declared_parameters: Vec<&VariableIndexEntry>,
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, CodegenError> {
        let mut result = Vec::new();
        let mut variadic_parameters = Vec::new();
        let mut passed_param_indices = Vec::new();
        for (i, argument) in arguments.iter().enumerate() {
            let (i, argument, _) = get_implicit_call_parameter(argument, &declared_parameters, i)?;

            // parameter_info includes the declaration type and type name
            let parameter_info = declared_parameters
                .get(i)
                .map(|it| {
                    let name = it.get_type_name();
                    if let Some(DataTypeInformation::Pointer {
                        inner_type_name, auto_deref: Some(_), ..
                    }) = self.index.find_effective_type_info(name)
                    {
                        // for auto_deref pointers (VAR_INPUT {ref}, VAR_IN_OUT) we call generate_argument_by_ref()
                        // we need the inner_type and not pointer to type otherwise we would generate a double pointer
                        Some((it.get_declaration_type(), inner_type_name.as_str()))
                    } else {
                        Some((it.get_declaration_type(), name))
                    }
                })
                // TODO : Is this idomatic, we need to wrap in ok because the next step does not necessarily fail
                .map(Ok)
                // None -> possibly variadic
                .unwrap_or_else(|| {
                    // if we are dealing with a variadic function, we can accept all extra parameters
                    if pou.is_variadic() {
                        variadic_parameters.push(argument);
                        Ok(None)
                    } else {
                        // we are not variadic, we have too many parameters here
                        Err(Diagnostic::codegen_error("Too many parameters", argument))
                    }
                })?;

            if let Some((declaration_type, type_name)) = parameter_info {
                let argument: BasicValueEnum = if declaration_type.is_by_ref()
                    || (self.index.get_effective_type_or_void_by_name(type_name).is_aggregate_type()
                        && declaration_type.is_input())
                {
                    let declared_parameter = declared_parameters.get(i);
                    self.generate_argument_by_ref(argument, type_name, declared_parameter.copied())?
                } else {
                    // by val
                    if !argument.is_empty_statement() {
                        self.generate_expression(argument)?
                    } else if let Some(param) = declared_parameters.get(i) {
                        self.generate_empty_expression(param)?
                    } else {
                        unreachable!("Statement param must have an index entry at this point.");
                    }
                };
                result.push((i, argument));
            }

            passed_param_indices.push(i);
        }

        // handle missing parameters, generate empty expression
        if declared_parameters.len() > passed_param_indices.len() {
            for (i, param) in declared_parameters.into_iter().enumerate() {
                if !passed_param_indices.contains(&i) {
                    let generated_exp = self.generate_empty_expression(param)?;
                    result.push((i, generated_exp));
                }
            }
        }

        // push variadic collection and optionally the variadic size
        if pou.is_variadic() {
            let last_location = result.len();
            for (i, parameter) in
                self.generate_variadic_arguments_list(pou, &variadic_parameters)?.into_iter().enumerate()
            {
                result.push((i + last_location, parameter));
            }
        }

        result.sort_by(|(idx_a, _), (idx_b, _)| idx_a.cmp(idx_b));
        Ok(result.into_iter().map(|(_, v)| v.into()).collect::<Vec<BasicMetadataValueEnum>>())
    }

    /// generates a value that is passed by reference
    /// this generates and returns a PointerValue
    /// pointing to the given `argument`
    fn generate_argument_by_ref(
        &self,
        argument: &AstNode,
        type_name: &str,
        declared_parameter: Option<&VariableIndexEntry>,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        if argument.is_empty_statement() {
            // Uninitialized var_output / var_in_out
            let v_type = self
                .llvm_index
                .find_associated_type(type_name)
                .ok_or_else(|| Diagnostic::unknown_type(type_name, argument))?;

            let ptr_value = self.llvm.builder.build_alloca(v_type, "")?;
            if let Some(p) = declared_parameter {
                if let Some(initial_value) =
                    self.get_initial_value(&p.initial_value, &self.get_parameter_type(p))
                {
                    let value = self.generate_expression(initial_value)?;
                    self.llvm.builder.build_store(ptr_value, value)?;
                }
            }

            return Ok(ptr_value.into());
        }

        // Generate the element pointer, then...
        let value = {
            let value = self.generate_expression_value(argument)?;
            match value {
                ExpressionValue::LValue(value, _) => value,
                ExpressionValue::RValue(_) => {
                    // Passed a literal to a byref parameter?
                    let value = self.generate_expression(argument)?;
                    let argument = self.llvm.builder.build_alloca(value.get_type(), "")?;
                    self.llvm.builder.build_store(argument, value)?;
                    argument
                }
            }
        };

        // ...check if we can bitcast a reference to their hinted type
        if let Some(hint) = self.annotations.get_type_hint(argument, self.index) {
            let actual_type = self.annotations.get_type_or_void(argument, self.index);
            let actual_type_info = self.index.find_elementary_pointer_type(&actual_type.information);
            let target_type_info = self.index.find_elementary_pointer_type(&hint.information);

            if target_type_info.is_vla() {
                // XXX: Calling `cast_if_needed` will result in an `alloca` call for EVERY function call.
                // LLVM might be able to optimize it away but ideally we find a solution for this at some
                // point? For a more in-depth description see the `pass` function in `vla_adr.rs`
                return cast_if_needed!(
                    self,
                    hint,
                    actual_type,
                    value.into(),
                    self.annotations.get(argument)
                );
            };

            // From https://llvm.org/docs/LangRef.html#bitcast-to-instruction: The ‘bitcast’ instruction takes
            // a value to cast, which must be a **non-aggregate** first class value [...]
            if !actual_type_info.is_aggregate() && actual_type_info != target_type_info {
                return Ok(self.llvm.builder.build_bit_cast(
                    value,
                    self.llvm_index.get_associated_type(hint.get_name())?,
                    "",
                )?);
            }
        }

        // ...otherwise no bitcasting was needed, thus return the generated element pointer as is
        Ok(value.into())
    }

    pub fn generate_variadic_arguments_list(
        &self,
        pou: &PouIndexEntry,
        variadic_params: &[&AstNode],
    ) -> Result<Vec<BasicValueEnum<'ink>>, CodegenError> {
        // get the real varargs from the index
        if let Some((var_args, argument_type)) = self
            .index
            .get_variadic_member(pou.get_name())
            .and_then(|it| it.get_varargs().zip(Some(it.get_declaration_type())))
        {
            // For unsized variadics, we need to follow C ABI rules
            let is_unsized = matches!(var_args, VarArgs::Unsized(_));

            let generated_params = variadic_params
                .iter()
                .map(|param_statement| {
                    self.get_type_hint_for(param_statement).map(|it| it.get_name()).and_then(|type_name| {
                        // Check if we need to pass by reference:
                        // 1. If the variadic is defined in a by_ref block
                        // 2. For unsized variadics: arrays and strings decay to pointers per C ABI
                        let type_info = self.index.get_effective_type_or_void_by_name(type_name);
                        let is_array_or_string = type_info.is_array() || type_info.is_string();
                        if argument_type.is_by_ref() || (is_unsized && is_array_or_string) {
                            self.generate_argument_by_ref(
                                param_statement,
                                type_name,
                                self.index.get_variadic_member(pou.get_name()),
                            )
                        } else {
                            self.generate_expression(param_statement)
                        }
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            // for sized variadics we create an array and store all the arguments in that array
            if let VarArgs::Sized(Some(type_name)) = var_args {
                let ty = self.llvm_index.get_associated_type(type_name).map(|it| {
                    if argument_type.is_by_ref() && it.is_array_type() {
                        it.into_array_type().get_element_type()
                    } else {
                        it
                    }
                })?;
                // if the variadic argument is ByRef, wrap it in a pointer.
                let ty = if argument_type.is_by_ref() {
                    self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into()
                } else {
                    ty
                };

                let size = generated_params.len();
                let size_param = self.llvm.i32_type().const_int(size as u64, true);

                let arr = ty.array_type(size as u32);
                let arr_storage = self.llvm.builder.build_alloca(arr, "")?;
                for (i, ele) in generated_params.iter().enumerate() {
                    let ele_ptr = self.llvm.load_array_element(
                        arr.as_basic_type_enum(),
                        arr_storage,
                        &[
                            self.llvm.context.i32_type().const_zero(),
                            self.llvm.context.i32_type().const_int(i as u64, true),
                        ],
                        "",
                    )?;
                    self.llvm.builder.build_store(ele_ptr, *ele)?;
                }

                // bitcast the array to pointer so it matches the declared function signature
                let arr_storage = self.llvm.builder.build_bit_cast(
                    arr_storage,
                    self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)),
                    "",
                )?;

                Ok(vec![size_param.into(), arr_storage])
            } else {
                Ok(generated_params)
            }
        } else {
            unreachable!("Function must be variadic")
        }
    }

    /// generates the assignments of a pou-call's parameters
    /// the call parameters are passed to the pou using a struct-instance with all the parameters
    ///
    /// - `pou_name` the name of the pou we're calling
    /// - `parameter_struct` a pointer to a struct-instance that holds all pou-parameters
    /// - `passed_parameters` a vec of all passed parameters to the pou-call
    fn generate_stateful_pou_arguments(
        &self,
        pou_name: &str,
        class_struct: Option<PointerValue<'ink>>,
        parameter_struct: PointerValue<'ink>,
        arguments: &[&AstNode],
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, CodegenError> {
        let mut result = class_struct
            .map(|class_struct| {
                vec![class_struct.as_basic_value_enum().into(), parameter_struct.as_basic_value_enum().into()]
            })
            .unwrap_or_else(|| vec![parameter_struct.as_basic_value_enum().into()]);
        for argument in arguments.iter() {
            let Some(StatementAnnotation::Argument { position, depth, pou, .. }) =
                self.annotations.get_hint(argument)
            else {
                panic!()
            };

            let parameter = self.generate_call_struct_argument_assignment(&CallParameterAssignment {
                assignment: argument,
                function_name: pou_name,
                position: *position as u32,
                depth: *depth as u32,
                declaring_pou: pou,
                parameter_struct,
            })?;
            if let Some(parameter) = parameter {
                result.push(parameter.into());
            };
        }

        Ok(result)
    }

    fn get_parameter_type(&self, parameter: &VariableIndexEntry) -> String {
        if let Some(DataTypeInformation::Pointer { inner_type_name, auto_deref: Some(_), .. }) =
            self.index.find_effective_type_info(parameter.get_type_name())
        {
            inner_type_name.into()
        } else {
            parameter.get_type_name().into()
        }
    }

    fn generate_empty_expression(
        &self,
        parameter: &VariableIndexEntry,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let parameter_type_name = self.get_parameter_type(parameter);
        let parameter_type = self.llvm_index.get_associated_type(&parameter_type_name)?;
        match parameter.get_declaration_type() {
            ArgumentType::ByVal(..) => {
                if let Some(initial_value) =
                    self.get_initial_value(&parameter.initial_value, &parameter_type_name)
                {
                    self.generate_expression(initial_value)
                } else {
                    let ptr_value = self.llvm.builder.build_alloca(parameter_type, "")?;
                    let pointee = parameter_type;
                    Ok(self.llvm.load_pointer(pointee, &ptr_value, "")?)
                }
            }
            _ => {
                let ptr_value = self.llvm.builder.build_alloca(parameter_type, "")?;

                // if default value is given for an output
                // we need to initialize the pointer value before returning
                if let Some(initial_value) =
                    self.get_initial_value(&parameter.initial_value, &parameter_type_name)
                {
                    let value = self.generate_expression(initial_value)?;
                    self.llvm.builder.build_store(ptr_value, value)?;
                }
                Ok(ptr_value.as_basic_value_enum())
            }
        }
    }

    /// returns an initial value if there is some
    ///
    /// first try to find an initial value for the given id
    ///
    /// if there is none try to find an initial value for the given type
    fn get_initial_value(&self, id: &Option<ConstId>, type_name: &str) -> Option<&AstNode> {
        self.index.get_initial_value(id).or_else(|| self.index.get_initial_value_for_type(type_name))
    }

    /// generates an assignemnt of a single call's argument
    ///
    /// - `CallParameterAssignment` containing following information
    /// - `assignment_statement` the parameter-assignment, either an AssignmentStatement, an OutputAssignmentStatement or an expression
    /// - `function_name` the name of the callable
    /// - `index` the index of the parameter (0 for first parameter, 1 for the next one, etc.)
    /// - `parameter_struct` a pointer to a struct-instance that holds all function-parameters
    fn generate_call_struct_argument_assignment(
        &self,
        param_context: &CallParameterAssignment,
    ) -> Result<Option<BasicValueEnum<'ink>>, CodegenError> {
        let Some(data) = try_from!(param_context.assignment, Assignment) else {
            // foo(x)
            return self.generate_nameless_parameter(param_context);
        };

        // explicit call parameter: foo(param := value)
        self.generate_formal_parameter(param_context, &data.left, &data.right)?;
        Ok(None)
    }

    /// generates the appropriate value for the given expression where the expression
    /// is a call's implicit argument (hence: foo(3), not foo(in := 3))
    fn generate_nameless_parameter(
        &self,
        param_context: &CallParameterAssignment,
    ) -> Result<Option<BasicValueEnum<'ink>>, CodegenError> {
        let builder = &self.llvm.builder;
        let index = param_context.position;
        let expression = param_context.assignment;

        if let Some(parameter) = self.index.get_declared_parameter(param_context.declaring_pou, index) {
            // this happens before the pou call
            // before the call statement we may only consider inputs and inouts
            // after the call we need to copy the output values to the correct assigned variables
            if matches!(parameter.get_variable_type(), VariableType::Output) {
                return Ok(None);
            }

            let pou_type_name = self
                .index
                .find_pou(param_context.function_name)
                .and_then(|pou| pou.get_instance_struct_type_name())
                .unwrap_or(param_context.function_name);
            let pointee = self
                .llvm_index
                .get_associated_pou_type(pou_type_name)
                .expect("POU type for parameter struct must exist");
            let pointer_to_param = self.build_parameter_struct_gep(pointee, param_context);

            let parameter = self
                .index
                .find_parameter(param_context.declaring_pou, index)
                .and_then(|var| self.index.find_effective_type_by_name(var.get_type_name()))
                .map(|var| var.get_type_information())
                .unwrap_or_else(|| self.index.get_void_type().get_type_information());

            if let DataTypeInformation::Pointer { auto_deref: Some(_), inner_type_name, .. } = parameter {
                //this is a VAR_IN_OUT assignment, so don't load the value, assign the pointer
                //expression may be empty -> generate a local variable for it
                let generated_exp = if expression.is_empty_statement() {
                    let temp_type = self
                        .llvm_index
                        .find_associated_type(inner_type_name)
                        .ok_or_else(|| Diagnostic::unknown_type(parameter.get_name(), expression))?;
                    builder.build_alloca(temp_type, "empty_varinout")?.as_basic_value_enum()
                } else {
                    self.generate_lvalue(expression)?.as_basic_value_enum()
                };
                builder.build_store(pointer_to_param, generated_exp)?;
            } else {
                self.generate_store(pointer_to_param, parameter, expression)?;
            };

            Ok(None)
        } else {
            Ok(Some(self.generate_expression(expression)?))
        }
    }

    /// generates the appropriate value for the given `right` expression  that is passed in a call
    /// to the parameter represented by `left`
    /// it's a call's explicit argument (hence: foo(left := right), not foo(right))
    fn generate_formal_parameter(
        &self,
        param_context: &CallParameterAssignment,
        left: &AstNode,
        right: &AstNode,
    ) -> Result<(), CodegenError> {
        let function_name = param_context.function_name;
        let parameter_struct = param_context.parameter_struct;

        if let Some(StatementAnnotation::Variable { qualified_name, .. }) = self.annotations.get(left) {
            let parameter = self
                .index
                .find_fully_qualified_variable(qualified_name)
                .ok_or_else(|| Diagnostic::unresolved_reference(qualified_name, left))?;

            // don't generate param assignments for empty statements, with the exception
            // of VAR_IN_OUT params - they need an address to point to
            let is_auto_deref = self
                .index
                .find_effective_type_by_name(parameter.get_type_name())
                .map(DataType::get_type_information)
                .unwrap_or_else(|| self.index.get_void_type().get_type_information())
                .is_auto_deref();

            if !right.is_empty_statement() || is_auto_deref {
                self.generate_call_struct_argument_assignment(&CallParameterAssignment {
                    assignment: right,
                    function_name,
                    position: param_context.position,
                    depth: param_context.depth,
                    declaring_pou: param_context.declaring_pou,
                    parameter_struct,
                })?;
            };
        }
        Ok(())
    }

    /// generates an gep-statement and returns the resulting pointer
    ///
    /// - `reference_statement` - the statement to get an lvalue from
    pub fn generate_lvalue(&self, reference_statement: &AstNode) -> Result<PointerValue<'ink>, CodegenError> {
        self.generate_expression_value(reference_statement).and_then(|it| {
            let v: Result<PointerValue, _> = it.get_basic_value_enum().try_into();
            v.map_err(|err| {
                CodegenError::GenericError(format!("{err:?}"), reference_statement.get_location().clone())
            })
        })
    }

    /// geneartes a gep for the given reference with an optional qualifier
    ///
    /// - `qualifier` an optional qualifier for a reference (e.g. myStruct.x where myStruct is the qualifier for x)
    /// - `name` the name of the reference-name (e.g. myStruct.x where 'x' is the reference-name)
    /// - `context` the statement to obtain the location from when returning an error
    fn create_llvm_pointer_value_for_reference(
        &self,
        qualifier: Option<(&AstNode, PointerValue<'ink>)>,
        name: &str,
        context: &AstNode,
    ) -> Result<PointerValue<'ink>, CodegenError> {
        let offset = &context.get_location();
        if let Some((qualifier_node, qualifier)) = qualifier {
            //if we're loading a reference like PLC_PRG.ACTION we already loaded PLC_PRG pointer into qualifier,
            //so we should not load anything in addition for the action (or the method)
            match self.annotations.get(context) {
                Some(StatementAnnotation::Function { qualified_name, .. })
                | Some(StatementAnnotation::Program { qualified_name, .. }) => {
                    if self.index.find_pou_implementation(qualified_name).is_some() {
                        return Ok(qualifier.to_owned());
                    }
                }
                Some(StatementAnnotation::Variable { qualified_name, .. }) => {
                    // Get the container (qualifier) type name for struct member index lookup
                    let qualifier_type = self.annotations.get_type(qualifier_node, self.index).unwrap();
                    let container_name = qualifier_type.get_name();

                    // For POUs (programs, function blocks, classes), use get_struct_member_index
                    // to compute the correct GEP index. This properly handles POUs with
                    // VAR_TEMP/VAR_EXTERNAL variables which are not part of the struct
                    // (they're stack-allocated or external).
                    // For regular structs (including vtables), use location_in_parent directly.
                    let member_location = if self.index.find_pou(container_name).is_some() {
                        self.index
                            .get_struct_member_index(container_name, name)
                            .ok_or_else(|| Diagnostic::unresolved_reference(qualified_name, offset))?
                    } else {
                        self.index
                            .find_fully_qualified_variable(qualified_name)
                            .map(VariableIndexEntry::get_location_in_parent)
                            .ok_or_else(|| Diagnostic::unresolved_reference(qualified_name, offset))?
                    };

                    let pointee = self.llvm_index.get_associated_type(container_name).unwrap();

                    let gep: PointerValue<'_> = self.llvm.get_member_pointer_from_struct(
                        pointee,
                        qualifier,
                        member_location,
                        name,
                    )?;

                    return Ok(gep);
                }
                _ => {
                    let qualifier_name = self.get_type_hint_for(context)?.get_name();
                    let qualified_name = qualified_name(qualifier_name, name);
                    let implementation = self.index.find_pou_implementation(&qualified_name);
                    if implementation.is_some() {
                        return Ok(qualifier.to_owned());
                    }
                }
            }
        }

        // no context ... so just something like 'x'
        match self.annotations.get(context) {
            Some(StatementAnnotation::Variable { qualified_name, .. })
            | Some(StatementAnnotation::Program { qualified_name, .. }) => self
                .llvm_index
                .find_loaded_associated_variable_value(qualified_name)
                .ok_or_else(|| Diagnostic::unresolved_reference(name, offset).into()),
            _ => Err(Diagnostic::unresolved_reference(name, offset).into()),
        }
    }

    pub fn ptr_as_value(&self, ptr: PointerValue<'ink>) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let int_type = self.llvm.context.i64_type();
        Ok(if ptr.is_const() {
            ptr.const_to_int(int_type)
        } else {
            self.llvm.builder.build_ptr_to_int(ptr, int_type, "")?
        }
        .as_basic_value_enum())
    }

    pub fn int_neg(&self, value: IntValue<'ink>) -> Result<IntValue<'ink>, CodegenError> {
        Ok(if value.is_const() { value.const_neg() } else { self.llvm.builder.build_int_neg(value, "")? })
    }

    /// automatically derefs an inout variable pointer so it can be used like a normal variable
    ///
    /// # Arguments
    /// - `variable_type` the reference's data type, this type will be used to determine if this variable needs to be auto-derefeferenced (var_in_out)
    /// - `access_ptr` the original pointer value loaded for the reference. will be returned if no auto-deref is necessary
    fn auto_deref_if_necessary(
        &self,
        accessor_ptr: PointerValue<'ink>,
        statement: &AstNode,
    ) -> Result<PointerValue<'ink>, CodegenError> {
        if self.annotations.get(statement).is_some_and(StatementAnnotation::is_auto_deref) {
            // Normally it wouldn't be safe to just assume the pointee in the `load_pointer` call is just of
            // type `ptr`. However, we return a `PointerValue` here, so LLVM actually expects a `ptr` type
            // from us as it would panic with any other type otherwise.
            let pointee = self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into();
            return Ok(self.llvm.load_pointer(pointee, &accessor_ptr, "deref")?.into_pointer_value());
        }

        Ok(accessor_ptr)
    }

    /// generates the access-expression for an array-reference
    /// myArray[array_expression] where array_expression is the access-expression
    ///
    /// - `dimension` the array's dimension
    /// - `access_expression` the expression inside the array-statement
    fn generate_access_for_dimension(
        &self,
        dimension: &Dimension,
        access_expression: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let start_offset = dimension
            .start_offset
            .as_int_value(self.index)
            .map_err(|it| Diagnostic::codegen_error(it, access_expression))?;

        let access_value = self.generate_expression(access_expression)?;
        //If start offset is not 0, adjust the current statement with an add operation
        let result = if start_offset != 0 {
            let access_int_value = access_value.into_int_value();
            let access_int_type = access_int_value.get_type();
            self.llvm.builder.build_int_sub(
                access_int_value,
                access_int_type.const_int(start_offset as u64, true), //TODO error handling for cast
                "",
            )?
        } else {
            access_value.into_int_value()
        };
        //turn it into i32 immediately
        cast_if_needed!(
            self,
            self.index.get_type(DINT_TYPE)?,
            self.get_type_hint_for(access_expression)?,
            result.as_basic_value_enum(),
            None
        )
    }

    /// generates a gep statement for a array-reference with an optional qualifier
    ///
    /// - `qualifier` an optional qualifier for a reference (e.g. myStruct.x[2] where myStruct is the qualifier for x)
    /// - `reference` the reference-statement pointing to the array
    /// - `access` the accessor expression (the expression between the brackets: reference[access])
    fn generate_element_pointer_for_array(
        &self,
        reference: &AstNode,
        access: &AstNode,
    ) -> Result<PointerValue<'ink>, CodegenError> {
        //Load the reference
        self.generate_expression_value(reference)
            .map(|it| it.get_basic_value_enum().into_pointer_value())
            .and_then(|lvalue| {
                if let DataTypeInformation::Array { name, dimensions, inner_type_name } =
                    self.get_type_hint_info_for(reference)?
                {
                    // make sure dimensions match statement list
                    let statements = access.get_as_list();
                    if statements.is_empty() || statements.len() != dimensions.len() {
                        return Err(Diagnostic::codegen_error("Invalid array access", access).into());
                    }

                    // e.g. an array like `ARRAY[0..3, 0..2, 0..1] OF ...` has the lengths [ 4 , 3 , 2 ]
                    let lengths = dimensions
                        .iter()
                        .map(|d| d.get_length(self.index))
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|msg| {
                            CodegenError::from(Diagnostic::codegen_error(
                                format!("Invalid array dimensions access: {msg}").as_str(),
                                access,
                            ))
                        })?;

                    // the portion indicates how many elements are represented by the corresponding dimension
                    // the first dimensions corresponds to the number of elements of the rest of the dimensions
                    //          - portion = 6 (size to the right = 3 x 2 = 6)
                    //         /        - portion = 2 (because the size to the right of this dimension = 2)
                    //        /        /         - the last dimension is directly translated into array-coordinates (skip-size = 1)
                    //                /         /
                    // [    4   ,    3    ,    2  ]
                    let dimension_portions = (0..lengths.len())
                        .map(|index| {
                            if index == lengths.len() - 1 {
                                1
                            } else {
                                lengths[index + 1..lengths.len()].iter().product()
                            }
                        })
                        .collect::<Vec<u32>>();

                    let accessors_and_portions = statements
                        .iter()
                        .zip(dimensions)
                        .map(|(statement, dimension)|
                            // generate array-accessors
                            self.generate_access_for_dimension(dimension, statement))
                        .zip(dimension_portions);

                    // accessing [ 1, 2, 2] means to access [ 1*6 + 2*2 + 2*1 ] = 12
                    let (index_access, _) = accessors_and_portions.fold(
                        (Ok(self.llvm.i32_type().const_zero().as_basic_value_enum()), 1),
                        |(accumulated_value, _), (current_v, current_portion)| {
                            let result = accumulated_value.and_then(|last_v| {
                                current_v.map(|v| {
                                    let current_portion_value = self
                                        .llvm
                                        .i32_type()
                                        .const_int(current_portion as u64, false)
                                        .as_basic_value_enum();
                                    // multiply the accessor with the dimension's portion
                                    let m_v = self.create_llvm_int_binary_expression(
                                        &Operator::Multiplication,
                                        current_portion_value,
                                        v,
                                    )?;
                                    // take the sum of the mulitlication and the previous accumulated_value
                                    // this now becomes the new accumulated value
                                    self.create_llvm_int_binary_expression(&Operator::Plus, m_v, last_v)
                                })?
                            });
                            (result, 0 /* the 0 will be ignored */)
                        },
                    );

                    // make sure we got an int-value
                    let index_access: IntValue = index_access.and_then(|it| {
                        it.try_into()
                            .map_err(|_| Diagnostic::codegen_error("non-numeric index-access", access).into())
                    })?;

                    let accessor_sequence =
                        if self.llvm_index.get_associated_type(name.as_str())?.is_array_type() {
                            // For typed array pointers (e.g.: [81 x i32]*):
                            // the first index (0) will point to the array -> [81 x i32]
                            // the second index (index_access) will point to the element in the array
                            vec![self.llvm.i32_type().const_zero(), index_access]
                        } else if self.index.find_effective_type_by_name(inner_type_name).is_some_and(|it| {
                            matches!(
                                it.get_type_information(),
                                DataTypeInformation::Array { .. } | DataTypeInformation::String { .. }
                            )
                        }) {
                            // For flattened array-of-array parameters (fundamental element pointer):
                            // Calculate proper stride: index * element_size
                            // This handles cases like i8* representing [N x STRING]* or i32* representing [N x [M x i32]]*
                            let DataTypeInformation::Array { inner_type_name, .. } =
                                self.get_type_hint_info_for(reference)?
                            else {
                                log::error!("Uncaught resolve error for inner type of nested array");
                                return Err(Diagnostic::codegen_error(
                                    "Expected inner type to be resolvable",
                                    reference,
                                )
                                .into());
                            };

                            // Get the size of the inner type (STRING or nested array)
                            // TODO: use `if let Some(...) if <guard>` once rust is updated and get rid of "is_nested_array" above
                            let Some(inner_type) = self.index.find_effective_type_by_name(inner_type_name)
                            else {
                                unreachable!("type must exist in index due to previous checks")
                            };

                            let element_size = match inner_type.get_type_information() {
                                DataTypeInformation::String { size, .. } => size
                                    .as_int_value(self.index)
                                    .unwrap_or((DEFAULT_STRING_LEN + 1_u32).into())
                                    as u64,
                                DataTypeInformation::Array { dimensions, .. } => {
                                    // For nested arrays, calculate total size
                                    dimensions
                                        .iter()
                                        .map(|d| d.get_length(self.index).unwrap_or(1) as u64)
                                        .product()
                                }
                                _ => unreachable!("Must be STRING or ARRAY type due to previous checks"),
                            };

                            let byte_offset = self.llvm.builder.build_int_mul(
                                index_access,
                                self.llvm.i32_type().const_int(element_size, false),
                                "array_stride_offset",
                            )?;
                            vec![byte_offset]
                        } else {
                            // lvalue is a simple pointer to type -> e.g.: i32*
                            // only one index (index_access) is needed to access the element
                            vec![index_access]
                        };

                    // load the access from that array
                    let pointee = self.llvm_index.get_associated_type(name).unwrap();
                    let pointer =
                        self.llvm.load_array_element(pointee, lvalue, &accessor_sequence, "tmpVar")?;

                    return Ok(pointer);
                }
                Err(Diagnostic::codegen_error("Invalid array access", access).into())
            })
    }

    /// generates the result of an pointer binary-expression
    ///
    /// - `operator` the binary operator
    /// - `left` the left side of the binary expression, needs to be an pointer/int-value
    /// - `left_type` DataTypeInformation of the left side
    /// - `right` the right side of the binary expression, needs to be an pointer/int-value
    /// - `right_type` DataTypeInformation of the right side
    /// - `expression` the binary expression
    pub fn create_llvm_binary_expression_for_pointer(
        &self,
        operator: &Operator,
        left: &AstNode,
        left_type: &DataTypeInformation,
        right: &AstNode,
        right_type: &DataTypeInformation,
        expression: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let left_expr = self.generate_expression(left)?;
        let right_expr = self.generate_expression(right)?;

        let result = match operator {
            Operator::Plus | Operator::Minus => {
                let (ptr, pointee, index, name) = if left_type.is_pointer() && right_type.is_int() {
                    let ptr = left_expr.into_pointer_value();
                    let pointee = self.llvm_index.get_associated_type(left_type.get_inner_name()).unwrap();
                    let index = right_expr.into_int_value();
                    let name = format!("access_{}", left_type.get_name());
                    (Some(ptr), Some(pointee), Some(index), Some(name))
                } else if left_type.is_int() && right_type.is_pointer() {
                    let ptr = right_expr.into_pointer_value();
                    let pointee = self.llvm_index.get_associated_type(right_type.get_inner_name()).unwrap();
                    let index = left_expr.into_int_value();
                    let name = format!("access_{}", right_type.get_name());
                    (Some(ptr), Some(pointee), Some(index), Some(name))
                } else {
                    // if left and right are both pointers we can not perform plus/minus
                    (None, None, None, None)
                };

                if let (Some(ptr), Some(pointee), Some(mut index), Some(name)) = (ptr, pointee, index, name) {
                    // if operator is minus we need to negate the index
                    if let Operator::Minus = operator {
                        index = self.int_neg(index)?;
                    }

                    Ok(self
                        .llvm
                        .load_array_element(pointee, ptr, &[index], name.as_str())?
                        .as_basic_value_enum())
                } else {
                    Err(Diagnostic::codegen_error(
                        format!("'{operator}' operation must contain one int type").as_str(),
                        expression,
                    )
                    .into())
                }
            }
            Operator::Equal => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::EQ,
                    self.convert_to_int_value_if_pointer(left_expr)?,
                    self.convert_to_int_value_if_pointer(right_expr)?,
                    "tmpVar",
                )?
                .as_basic_value_enum()),
            Operator::NotEqual => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::NE,
                    self.convert_to_int_value_if_pointer(left_expr)?,
                    self.convert_to_int_value_if_pointer(right_expr)?,
                    "tmpVar",
                )?
                .as_basic_value_enum()),
            Operator::Less => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::SLT,
                    self.convert_to_int_value_if_pointer(left_expr)?,
                    self.convert_to_int_value_if_pointer(right_expr)?,
                    "tmpVar",
                )?
                .as_basic_value_enum()),
            Operator::Greater => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::SGT,
                    self.convert_to_int_value_if_pointer(left_expr)?,
                    self.convert_to_int_value_if_pointer(right_expr)?,
                    "tmpVar",
                )?
                .as_basic_value_enum()),
            Operator::LessOrEqual => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::SLE,
                    self.convert_to_int_value_if_pointer(left_expr)?,
                    self.convert_to_int_value_if_pointer(right_expr)?,
                    "tmpVar",
                )?
                .as_basic_value_enum()),
            Operator::GreaterOrEqual => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::SGE,
                    self.convert_to_int_value_if_pointer(left_expr)?,
                    self.convert_to_int_value_if_pointer(right_expr)?,
                    "tmpVar",
                )?
                .as_basic_value_enum()),
            _ => Err(Diagnostic::codegen_error(
                format!("Operator '{operator}' unimplemented for pointers").as_str(),
                expression,
            )
            .into()),
        };

        result
    }

    /// if the given `value` is a pointer value, it converts the pointer into an int_value to access the pointer's
    /// address, if the given `value` is already an IntValue it is returned as is
    pub fn convert_to_int_value_if_pointer(
        &self,
        value: BasicValueEnum<'ink>,
    ) -> Result<IntValue<'ink>, CodegenError> {
        match value {
            BasicValueEnum::PointerValue(v) => Ok(self.ptr_as_value(v)?.into_int_value()),
            BasicValueEnum::IntValue(v) => Ok(v),
            _ => Err(Diagnostic::codegen_error(
                format!("Cannot convert {value} to int value"),
                SourceLocation::undefined(),
            )
            .into()),
        }
    }

    /// generates the result of an int/bool binary-expression (+, -, *, /, %, ==)
    ///
    /// - `operator` the binary operator
    /// - `left_value` the left side of the binary expression, needs to be an int-value
    /// - `right_value` the right side of the binary expression, needs to be an int-value
    /// - `target_type` the resulting type
    pub fn create_llvm_int_binary_expression(
        &self,
        operator: &Operator,
        left_value: BasicValueEnum<'ink>,
        right_value: BasicValueEnum<'ink>,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let int_lvalue = left_value.into_int_value();
        let int_rvalue = right_value.into_int_value();

        let value = match operator {
            Operator::Plus => self.llvm.builder.build_int_add(int_lvalue, int_rvalue, "tmpVar")?,
            Operator::Minus => self.llvm.builder.build_int_sub(int_lvalue, int_rvalue, "tmpVar")?,
            Operator::Multiplication => self.llvm.builder.build_int_mul(int_lvalue, int_rvalue, "tmpVar")?,
            Operator::Division => self.llvm.builder.build_int_signed_div(int_lvalue, int_rvalue, "tmpVar")?,
            Operator::Modulo => self.llvm.builder.build_int_signed_rem(int_lvalue, int_rvalue, "tmpVar")?,
            Operator::Equal => {
                self.llvm.builder.build_int_compare(IntPredicate::EQ, int_lvalue, int_rvalue, "tmpVar")?
            }

            Operator::NotEqual => {
                self.llvm.builder.build_int_compare(IntPredicate::NE, int_lvalue, int_rvalue, "tmpVar")?
            }

            Operator::Less => {
                self.llvm.builder.build_int_compare(IntPredicate::SLT, int_lvalue, int_rvalue, "tmpVar")?
            }

            Operator::Greater => {
                self.llvm.builder.build_int_compare(IntPredicate::SGT, int_lvalue, int_rvalue, "tmpVar")?
            }

            Operator::LessOrEqual => {
                self.llvm.builder.build_int_compare(IntPredicate::SLE, int_lvalue, int_rvalue, "tmpVar")?
            }

            Operator::GreaterOrEqual => {
                self.llvm.builder.build_int_compare(IntPredicate::SGE, int_lvalue, int_rvalue, "tmpVar")?
            }
            Operator::Xor => self.llvm.builder.build_xor(int_lvalue, int_rvalue, "tmpVar")?,
            Operator::And => self.llvm.builder.build_and(int_lvalue, int_rvalue, "tmpVar")?,
            Operator::Or => self.llvm.builder.build_or(int_lvalue, int_rvalue, "tmpVar")?,
            _ => Err(Diagnostic::codegen_error(
                format!("Operator '{operator}' unimplemented for int").as_str(),
                SourceLocation::undefined(),
            ))?,
        };
        Ok(value.into())
    }

    /// generates the result of a float binary-expression (+, -, *, /, %, ==)
    ///
    /// - `operator` the binary operator
    /// - `left_value` the left side of the binary expression, needs to be a float-value
    /// - `right_value` the right side of the binary expression, needs to be a float-value
    /// - `target_type` the resulting type
    fn create_llvm_float_binary_expression(
        &self,
        operator: &Operator,
        lvalue: BasicValueEnum<'ink>,
        rvalue: BasicValueEnum<'ink>,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let float_lvalue = lvalue.into_float_value();
        let float_rvalue = rvalue.into_float_value();

        let value = match operator {
            Operator::Plus => self.llvm.builder.build_float_add(float_lvalue, float_rvalue, "tmpVar")?.into(),
            Operator::Minus => {
                self.llvm.builder.build_float_sub(float_lvalue, float_rvalue, "tmpVar")?.into()
            }
            Operator::Multiplication => {
                self.llvm.builder.build_float_mul(float_lvalue, float_rvalue, "tmpVar")?.into()
            }
            Operator::Division => {
                self.llvm.builder.build_float_div(float_lvalue, float_rvalue, "tmpVar")?.into()
            }
            Operator::Modulo => {
                self.llvm.builder.build_float_rem(float_lvalue, float_rvalue, "tmpVar")?.into()
            }

            Operator::Equal => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OEQ, float_lvalue, float_rvalue, "tmpVar")?
                .into(),
            Operator::NotEqual => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::ONE, float_lvalue, float_rvalue, "tmpVar")?
                .into(),
            Operator::Less => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OLT, float_lvalue, float_rvalue, "tmpVar")?
                .into(),
            Operator::Greater => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OGT, float_lvalue, float_rvalue, "tmpVar")?
                .into(),
            Operator::LessOrEqual => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OLE, float_lvalue, float_rvalue, "tmpVar")?
                .into(),
            Operator::GreaterOrEqual => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OGE, float_lvalue, float_rvalue, "tmpVar")?
                .into(),

            _ => Err(Diagnostic::codegen_error(
                format!("Operator '{operator}' unimplemented for float").as_str(),
                SourceLocation::undefined(),
            ))?,
        };
        Ok(value)
    }

    fn generate_numeric_literal(
        &self,
        stmt: &AstNode,
        number: &str,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let type_hint = self.get_type_hint_for(stmt)?;
        let actual_type = self.annotations.get_type_or_void(stmt, self.index);
        let literal_type_name = if is_same_type_class(
            type_hint.get_type_information(),
            actual_type.get_type_information(),
            self.index,
        ) {
            type_hint.get_name()
        } else {
            actual_type.get_name()
        };
        let literal_type = self.llvm_index.get_associated_type(literal_type_name)?;
        self.llvm.create_const_numeric(&literal_type, number, stmt.get_location())
    }

    /// generates the literal statement and returns the resulting value
    ///
    /// - `literal_statement` one of LiteralBool, LiteralInteger, LiteralReal, LiteralString
    pub fn generate_literal(
        &self,
        literal_statement: &AstNode,
    ) -> Result<ExpressionValue<'ink>, CodegenError> {
        let cannot_generate_literal = || {
            Diagnostic::codegen_error(
                format!("Cannot generate Literal for {literal_statement:?}"),
                literal_statement,
            )
        };

        let location = &literal_statement.get_location();
        match literal_statement.get_stmt() {
            AstStatement::Literal(kind) => match kind {
                AstLiteral::Bool(b) => self.llvm.create_const_bool(*b).map(ExpressionValue::RValue),
                AstLiteral::Integer(i, ..) => self
                    .generate_numeric_literal(literal_statement, i.to_string().as_str())
                    .map(ExpressionValue::RValue),
                AstLiteral::Real(r, ..) => {
                    self.generate_numeric_literal(literal_statement, r).map(ExpressionValue::RValue)
                }
                AstLiteral::Date(d) => d
                    .value()
                    .map_err(|op| Diagnostic::codegen_error(op.as_str(), location).into())
                    .and_then(|ns| self.create_const_int(ns))
                    .map(ExpressionValue::RValue),
                AstLiteral::DateAndTime(dt) => dt
                    .value()
                    .map_err(|op| Diagnostic::codegen_error(op.as_str(), location).into())
                    .and_then(|ns| self.create_const_int(ns))
                    .map(ExpressionValue::RValue),
                AstLiteral::TimeOfDay(tod) => tod
                    .value()
                    .map_err(|op| Diagnostic::codegen_error(op.as_str(), location).into())
                    .and_then(|ns| self.create_const_int(ns))
                    .map(ExpressionValue::RValue),
                AstLiteral::Time(t) => self.create_const_int(t.value()).map(ExpressionValue::RValue),
                AstLiteral::String(s) => self.generate_string_literal(literal_statement, s.value(), location),
                AstLiteral::Array(arr) => self
                    .generate_literal_array(arr.elements().ok_or_else(cannot_generate_literal)?)
                    .map(ExpressionValue::RValue),
                AstLiteral::Null => self.llvm.create_null_ptr().map(ExpressionValue::RValue),
            },

            AstStatement::MultipliedStatement { .. } => {
                self.generate_literal_array(literal_statement).map(ExpressionValue::RValue)
            }
            AstStatement::ParenExpression(expr) => self.generate_literal(expr),
            // if there is an expression-list this might be a struct-initialization or array-initialization
            AstStatement::ExpressionList { .. } => {
                let type_hint = self.get_type_hint_info_for(literal_statement)?;
                match type_hint {
                    DataTypeInformation::Array { .. } => {
                        self.generate_literal_array(literal_statement).map(ExpressionValue::RValue)
                    }
                    _ => self.generate_literal_struct(literal_statement),
                }
            }
            // if there is just one assignment, this may be an struct-initialization (TODO this is not very elegant :-/ )
            AstStatement::Assignment { .. } => self.generate_literal_struct(literal_statement),
            // Reference expressions (e.g., variable references in array literals like `[..., str]`)
            // should be evaluated as regular expressions
            AstStatement::ReferenceExpr(_) => {
                self.generate_expression(literal_statement).map(ExpressionValue::RValue)
            }
            _ => Err(cannot_generate_literal().into()),
        }
    }

    /// generates the string-literal `value` represented by `literal_statement`
    fn generate_string_literal(
        &self,
        literal_statement: &AstNode,
        value: &str,
        location: &SourceLocation,
    ) -> Result<ExpressionValue<'ink>, CodegenError> {
        let expected_type = self.get_type_hint_info_for(literal_statement)?;
        self.generate_string_literal_for_type(expected_type, value, location)
    }

    fn generate_string_literal_for_type(
        &self,
        expected_type: &DataTypeInformation,
        value: &str,
        location: &SourceLocation,
    ) -> Result<ExpressionValue<'ink>, CodegenError> {
        match expected_type {
            DataTypeInformation::String { encoding, size, .. } => {
                let declared_length = size.as_int_value(self.index).map_err(|msg| {
                    Diagnostic::codegen_error(
                        format!("Unable to generate string-literal: {msg}").as_str(),
                        location,
                    )
                })? as usize;

                let pointee = self.llvm_index.get_associated_type(expected_type.get_name()).unwrap();
                match encoding {
                    StringEncoding::Utf8 => {
                        let literal =
                            self.llvm_index.find_utf08_literal_string(value).map(|it| it.as_pointer_value());
                        if let Some((literal_value, _)) = literal.zip(self.function_context) {
                            //global constant string
                            Ok(ExpressionValue::LValue(literal_value, pointee))
                        } else {
                            //note that .len() will give us the number of bytes, not the number of characters
                            let actual_length = value.chars().count() + 1; // +1 to account for a final \0
                            let str_len = std::cmp::min(
                                (self.string_len_provider)(declared_length, actual_length),
                                declared_length,
                            );
                            self.llvm.create_const_utf8_string(value, str_len).map(ExpressionValue::RValue)
                        }
                    }
                    StringEncoding::Utf16 => {
                        let literal = self.llvm_index.find_utf16_literal_string(value);
                        if literal.is_some()
                            && self.function_context.is_some()
                            && self.function_context.is_some()
                        {
                            //global constant string
                            Ok(literal
                                .map(|it| ExpressionValue::LValue(it.as_pointer_value(), pointee))
                                .unwrap())
                        } else {
                            //note that .len() will give us the number of bytes, not the number of characters
                            let actual_length = value.encode_utf16().count() + 1; // +1 to account for a final \0
                            let str_len = std::cmp::min(
                                (self.string_len_provider)(declared_length, actual_length),
                                declared_length,
                            );
                            self.llvm.create_const_utf16_string(value, str_len).map(ExpressionValue::RValue)
                        }
                    }
                }
            }
            DataTypeInformation::Pointer { inner_type_name, auto_deref: Some(_), .. } => {
                let inner_type = self.index.get_type_information_or_void(inner_type_name);
                self.generate_string_literal_for_type(inner_type, value, location)
            }
            DataTypeInformation::Integer { size: 8, .. } if expected_type.is_character() => {
                self.llvm.create_llvm_const_i8_char(value, location).map(ExpressionValue::RValue)
            }
            DataTypeInformation::Integer { size: 16, .. } if expected_type.is_character() => {
                self.llvm.create_llvm_const_i16_char(value, location).map(ExpressionValue::RValue)
            }
            _ => Err(Diagnostic::new(format!(
                "Cannot generate String-Literal for type {}",
                expected_type.get_name()
            ))
            .with_error_code("E074")
            .with_location(location)
            .into()),
        }
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_info_for(&self, statement: &AstNode) -> Result<&DataTypeInformation, CodegenError> {
        self.get_type_hint_for(statement).map(DataType::get_type_information)
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_for(&self, statement: &AstNode) -> Result<&DataType, CodegenError> {
        self.annotations
            .get_type_hint(statement, self.index)
            .or_else(|| self.annotations.get_type(statement, self.index))
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    format!("no type hint available for {}", statement.as_string()),
                    statement,
                )
                .into()
            })
    }

    /// generates a struct literal value with the given value assignments (ExpressionList)
    fn generate_literal_struct(&self, assignments: &AstNode) -> Result<ExpressionValue<'ink>, CodegenError> {
        if let DataTypeInformation::Struct { name: struct_name, members, .. } =
            self.get_type_hint_info_for(assignments)?
        {
            let mut uninitialized_members: FxHashSet<&VariableIndexEntry> = FxHashSet::from_iter(members);
            let mut member_values: Vec<(u32, BasicValueEnum<'ink>)> = Vec::new();
            for assignment in flatten_expression_list(assignments) {
                if let AstStatement::Assignment(data) = assignment.get_stmt() {
                    if let Some(StatementAnnotation::Variable { qualified_name, .. }) =
                        self.annotations.get(data.left.as_ref())
                    {
                        let member: &VariableIndexEntry =
                            self.index.find_fully_qualified_variable(qualified_name).ok_or_else(|| {
                                Diagnostic::unresolved_reference(qualified_name, data.left.as_ref())
                            })?;

                        let index_in_parent = member.get_location_in_parent();
                        let value = self.generate_expression(data.right.as_ref())?;

                        uninitialized_members.remove(member);
                        member_values.push((index_in_parent, value));
                    } else {
                        return Err(Diagnostic::codegen_error(
                            "struct member lvalue required as left operand of assignment",
                            data.left.as_ref(),
                        )
                        .into());
                    }
                } else {
                    return Err(Diagnostic::codegen_error(
                        "struct literal must consist of explicit assignments in the form of member := value",
                        assignment,
                    )
                    .into());
                }
            }

            //fill the struct with fields we didnt mention yet
            for member in uninitialized_members {
                let initial_value = self
                    .llvm_index
                    .find_associated_variable_value(member.get_qualified_name())
                    // .or_else(|| self.index.find_associated_variable_value(name))
                    .or_else(|| self.llvm_index.find_associated_initial_value(member.get_type_name()))
                    .ok_or_else(|| {
                        Diagnostic::cannot_generate_initializer(member.get_qualified_name(), assignments)
                    })?;

                member_values.push((member.get_location_in_parent(), initial_value));
            }
            let struct_type = self.llvm_index.get_associated_type(struct_name)?.into_struct_type();
            if member_values.len() == struct_type.count_fields() as usize {
                member_values.sort_by(|(a, _), (b, _)| a.cmp(b));
                let ordered_values: Vec<BasicValueEnum<'ink>> =
                    member_values.iter().map(|(_, v)| *v).collect();

                Ok(ExpressionValue::RValue(
                    struct_type.const_named_struct(ordered_values.as_slice()).as_basic_value_enum(),
                ))
            } else {
                Err(Diagnostic::codegen_error(
                    format!(
                        "Expected {} fields for Struct {}, but found {}.",
                        struct_type.count_fields(),
                        struct_name,
                        member_values.len()
                    ),
                    assignments,
                )
                .into())
            }
        } else {
            Err(Diagnostic::codegen_error(
                format!("Expected Struct-literal, got {assignments:#?}"),
                assignments,
            )
            .into())
        }
    }

    /// generates an array literal with the given optional elements (represented as an ExpressionList)
    pub fn generate_literal_array(
        &self,
        initializer: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let array_value = self.generate_literal_array_value(
            initializer,
            self.get_type_hint_info_for(initializer)?,
            &initializer.get_location(),
        )?;
        Ok(array_value.as_basic_value_enum())
    }

    /// constructs an ArrayValue (returned as a BasicValueEnum) of the given element-literals constructing an array-value of the
    /// type described by inner_array_type.
    ///
    /// passing an epxression-lists with LiteralIntegers and inner_array_type is INT-description will return an
    /// i16-array-value
    fn generate_literal_array_value(
        &self,
        elements: &AstNode,
        data_type: &DataTypeInformation,
        location: &SourceLocation,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let (inner_type, expected_len) =
            if let DataTypeInformation::Array { inner_type_name, dimensions, .. } = data_type {
                let len: u32 = dimensions
                    .iter()
                    .map(|d| d.get_length(self.index))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|msg| Diagnostic::codegen_error(msg.as_str(), location))?
                    .into_iter()
                    .product();

                self.index.get_type(inner_type_name).map(|inner_type| (inner_type, len as usize))
            } else {
                Err(Diagnostic::codegen_error(
                    format!("Expected array type but found: {:}", data_type.get_name()).as_str(),
                    location,
                ))
            }?;

        // for arrays of struct we cannot flatten the expression list
        // to generate the passed structs we need an expression list of assignments
        // flatten_expression_list will will return a vec of only assignments
        let elements =
            if self.index.get_effective_type_or_void_by_name(inner_type.get_name()).information.is_struct() {
                match elements.get_stmt() {
                    // ExpressionList of struct initializers: [(a:=1), (b:=2)] or [(a:=1, b:=2), (c:=3, d:=4)]
                    AstStatement::ExpressionList(expressions) => expressions.iter().collect(),
                    // Single struct initializer wrapped in parentheses: [(a := 1, b := 2)]
                    // The ParenExpression represents ONE struct, not multiple elements
                    AstStatement::ParenExpression(_) => vec![elements],
                    // Single struct initializer without parentheses
                    _ => vec![elements],
                }
            } else {
                flatten_expression_list(elements)
            };

        let llvm_type = self.llvm_index.get_associated_type(inner_type.get_name())?;
        let mut v = Vec::new();
        for e in elements {
            //generate with correct type hint
            let value = self.generate_literal(e)?;
            v.push(value.get_basic_value_enum());
        }

        if v.len() < expected_len {
            let initial = self
                .llvm_index
                .find_associated_initial_value(inner_type.get_name())
                .unwrap_or_else(|| llvm_type.const_zero());
            while v.len() < expected_len {
                //generate additional defaults for data_type
                v.push(initial);
            }
        }

        //TODO Validation: fail with compile-error if value cannot be converted into... correctly
        let array_value = match llvm_type {
            BasicTypeEnum::ArrayType(_) => llvm_type.into_array_type().const_array(
                v.iter().map(|it| it.into_array_value()).collect::<Vec<ArrayValue>>().as_slice(),
            ),
            BasicTypeEnum::FloatType(_) => llvm_type.into_float_type().const_array(
                v.iter().map(|it| it.into_float_value()).collect::<Vec<FloatValue>>().as_slice(),
            ),
            BasicTypeEnum::IntType(_) => llvm_type
                .into_int_type()
                .const_array(v.iter().map(|it| it.into_int_value()).collect::<Vec<IntValue>>().as_slice()),
            BasicTypeEnum::PointerType(_) => llvm_type.into_pointer_type().const_array(
                v.iter().map(|it| it.into_pointer_value()).collect::<Vec<PointerValue>>().as_slice(),
            ),
            BasicTypeEnum::StructType(_) => llvm_type.into_struct_type().const_array(
                v.iter().map(|it| it.into_struct_value()).collect::<Vec<StructValue>>().as_slice(),
            ),
            BasicTypeEnum::VectorType(_) => llvm_type.into_vector_type().const_array(
                v.iter().map(|it| it.into_vector_value()).collect::<Vec<VectorValue>>().as_slice(),
            ),
            BasicTypeEnum::ScalableVectorType(_) => llvm_type.into_scalable_vector_type().const_array(
                v.iter()
                    .map(|it| it.into_scalable_vector_value())
                    .collect::<Vec<ScalableVectorValue>>()
                    .as_slice(),
            ),
        };
        Ok(array_value.as_basic_value_enum())
    }

    /// generates a phi-expression (&& or || expression) with respect to short-circuit evaluation
    ///
    /// - `operator` an operator suitable for bool variables
    /// - `left` the left side of the expression
    /// - `right` the right side of the expression
    pub fn generate_bool_binary_expression(
        &self,
        operator: &Operator,
        left: &AstNode,
        right: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        match operator {
            Operator::And | Operator::Or => {
                self.generate_bool_short_circuit_expression(operator, left, right)
            }
            Operator::Equal => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::EQ,
                    to_i1(self.generate_expression(left)?.into_int_value(), &self.llvm.builder)?,
                    to_i1(self.generate_expression(right)?.into_int_value(), &self.llvm.builder)?,
                    "",
                )?
                .as_basic_value_enum()),
            Operator::NotEqual => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::NE,
                    to_i1(self.generate_expression(left)?.into_int_value(), &self.llvm.builder)?,
                    to_i1(self.generate_expression(right)?.into_int_value(), &self.llvm.builder)?,
                    "",
                )?
                .as_basic_value_enum()),
            Operator::Xor => Ok(self
                .llvm
                .builder
                .build_xor(
                    to_i1(self.generate_expression(left)?.into_int_value(), &self.llvm.builder)?,
                    to_i1(self.generate_expression(right)?.into_int_value(), &self.llvm.builder)?,
                    "",
                )?
                .as_basic_value_enum()),
            _ => Err(Diagnostic::codegen_error(
                format!("illegal boolean expresspion for operator {operator:}").as_str(),
                left.get_location().span(&right.get_location()),
            )
            .into()),
        }
    }

    /// generates a phi-expression (&& or || expression) with respect to short-circuit evaluation
    ///
    /// - `operator` AND / OR
    /// - `left` the left side of the expression as an i1 value
    /// - `right` the right side of an expression as an i1 value
    pub fn generate_bool_short_circuit_expression(
        &self,
        operator: &Operator,
        left: &AstNode,
        right: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let builder = &self.llvm.builder;
        let lhs = to_i1(self.generate_expression(left)?.into_int_value(), builder)?;
        let function = self.get_function_context(left)?.function;

        let right_branch = self.llvm.context.append_basic_block(function, "");
        let continue_branch = self.llvm.context.append_basic_block(function, "");

        let final_left_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        //Compare left to 0

        match operator {
            Operator::Or => builder.build_conditional_branch(lhs, continue_branch, right_branch)?,
            Operator::And => builder.build_conditional_branch(lhs, right_branch, continue_branch)?,
            _ => {
                return Err(Diagnostic::codegen_error(
                    format!("Cannot generate phi-expression for operator {operator:}"),
                    left,
                )
                .into());
            }
        };

        builder.position_at_end(right_branch);
        let rhs = to_i1(self.generate_expression(right)?.into_int_value(), builder)?;
        let final_right_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        builder.build_unconditional_branch(continue_branch)?;

        builder.position_at_end(continue_branch);
        //Generate phi
        let phi_value = builder.build_phi(lhs.get_type(), "")?;
        //assert
        phi_value.add_incoming(&[(&lhs, final_left_block), (&rhs, final_right_block)]);

        Ok(phi_value.as_basic_value())
    }

    fn create_const_int(&self, value: i64) -> Result<BasicValueEnum<'ink>, CodegenError> {
        let value = self.llvm.create_const_numeric(
            &self.llvm_index.get_associated_type(LINT_TYPE)?,
            value.to_string().as_str(),
            SourceLocation::internal(),
        )?;
        Ok(value)
    }

    /// creates a binary expression (left op right) with generic
    /// left & right expressions (non-numerics)
    /// this function attempts to call optional
    /// EQUAL_XXX, LESS_XXX or GREATER_XXX functions for comparison
    /// expressions
    fn create_llvm_generic_binary_expression(
        &self,
        left: &AstNode,
        right: &AstNode,
        binary_statement: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, CodegenError> {
        if let Some(StatementAnnotation::ReplacementAst { statement }) =
            self.annotations.get(binary_statement)
        {
            // we trust that the validator only passed us valid parameters (so left & right should be same type)
            self.generate_expression(statement)
        } else {
            Err(Diagnostic::codegen_error(
                format!(
                    "Invalid types, cannot generate binary expression for {:?} and {:?}",
                    self.get_type_hint_for(left)?.get_name(),
                    self.get_type_hint_for(right)?.get_name(),
                )
                .as_str(),
                left,
            )
            .into())
        }
    }

    pub fn generate_store(
        &self,
        left: inkwell::values::PointerValue,
        left_type: &DataTypeInformation,
        right_statement: &AstNode,
    ) -> Result<(), CodegenError> {
        let right_type =
            self.annotations.get_type_or_void(right_statement, self.index).get_type_information();

        // redirect aggregate types
        if left_type.is_aggregate() && right_type.is_aggregate() {
            let right =
                self.generate_expression_value(right_statement)?.get_basic_value_enum().into_pointer_value();
            self.build_memcpy(
                left,
                left_type,
                right_statement.get_location(),
                right,
                right_type,
                right_statement.get_location(),
            )?;
        } else {
            let expression = self.generate_expression(right_statement)?;
            self.llvm.builder.build_store(left, expression)?;
        }
        Ok(())
    }

    fn build_memcpy(
        &self,
        left: inkwell::values::PointerValue<'ink>,
        left_type: &DataTypeInformation,
        left_location: SourceLocation,
        right: inkwell::values::PointerValue<'ink>,
        right_type: &DataTypeInformation,
        right_location: SourceLocation,
    ) -> Result<PointerValue<'ink>, CodegenError> {
        let (size, alignment) = match (left_type, right_type) {
            (
                DataTypeInformation::String { size: lsize, .. },
                DataTypeInformation::String { size: rsize, .. },
            ) => {
                let target_size = lsize
                    .as_int_value(self.index)
                    .map_err(|err| Diagnostic::codegen_error(err.as_str(), &left_location))?;
                let value_size = rsize
                    .as_int_value(self.index)
                    .map_err(|err| Diagnostic::codegen_error(err.as_str(), right_location))?;
                let size = std::cmp::min(target_size - 1, value_size);
                //FIXME: use the target_layout for this operation
                let alignment = left_type.get_string_character_width(self.index).value();
                //Multiply by the string alignment to copy enough for widestrings
                //This is done at compile time to avoid generating an extra mul
                let size = self.llvm.context.i32_type().const_int((size * alignment as i64) as u64, true);
                (size, alignment)
            }
            (DataTypeInformation::Array { .. }, DataTypeInformation::Array { .. })
            | (DataTypeInformation::Struct { .. }, DataTypeInformation::Struct { .. }) => {
                let size = self.llvm_index.get_associated_type(right_type.get_name())?.size_of().ok_or_else(
                    || {
                        Diagnostic::codegen_error(
                            format!("Unknown size of type {}.", right_type.get_name()).as_str(),
                            right_location,
                        )
                    },
                )?;
                (size, 1)
            }
            _ => unreachable!("memcpy is not used for non-aggregate types"),
        };

        self.llvm.builder.build_memcpy(left, alignment, right, alignment, size).map_err(Into::into)
    }

    /// returns an optional name used for a temporary variable when loading a pointer represented by `expression`
    fn get_load_name(&self, expression: &AstNode) -> Option<String> {
        match expression.get_stmt() {
            AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Deref, .. })
            | AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Index(_), .. }) => {
                Some("load_tmpVar".to_string())
            }
            AstStatement::ReferenceExpr { .. } => expression
                .get_flat_reference_name()
                .map(|name| format!("{}{}{}", self.temp_variable_prefix, name, self.temp_variable_suffix))
                .or_else(|| Some(self.temp_variable_prefix.clone())),
            AstStatement::Identifier(name, ..) => Some(format!("{}{}", name, self.temp_variable_suffix)),
            _ => None,
        }
    }

    /// Generate a GEP instruction, accessing the array pointed to within the VLA struct at runtime
    fn generate_element_pointer_for_vla(
        &self,
        reference: ExpressionValue<'ink>,
        reference_annotation: &StatementAnnotation,
        access: &AstNode,
    ) -> Result<PointerValue<'ink>, CodegenError> {
        let builder = &self.llvm.builder;

        let StatementAnnotation::Variable { resulting_type: reference_type, .. } = reference_annotation
        else {
            unreachable!();
        };

        let struct_ptr = reference.get_basic_value_enum().into_pointer_value();

        let type_ = self.index.get_type_information_or_void(reference_type);
        let vla_struct_type = self.llvm_index.get_associated_type(type_.get_name())?;

        let arr_ptr_gep =
            builder.build_struct_gep(vla_struct_type, struct_ptr, 0, "vla_arr_gep").map_err(|_| {
                Diagnostic::codegen_error("Cannot access VLA array pointer", access.get_location())
            })?;
        let ptr_type = self.llvm.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC));
        let vla_arr_ptr = builder.build_load(ptr_type, arr_ptr_gep, "vla_arr_ptr")?.into_pointer_value();

        let dim_arr_gep = builder.build_struct_gep(vla_struct_type, struct_ptr, 1, "dim_arr")?;

        let Some(ndims) = type_.get_type_information().get_dimension_count() else { unreachable!() };

        let dimensions_array_type = vla_struct_type.into_struct_type().get_field_type_at_index(1).unwrap();
        let index_offsets = get_indices(self.llvm, ndims, dim_arr_gep, dimensions_array_type)?;

        let access_statements = access.get_as_list();
        let accessor = if access_statements.len() == 1 {
            let Some(stmt) = access_statements.first() else {
                unreachable!("Must have exactly 1 access statement")
            };
            let access_value = cast_if_needed!(
                self,
                self.index.get_type(DINT_TYPE)?,
                self.get_type_hint_for(stmt)?,
                self.generate_expression(stmt)?,
                None
            )?;

            let Some(start_offset) = index_offsets.first().map(|(start, _)| *start) else {
                unreachable!("VLA must have information about dimension offsets")
            };
            self.create_llvm_int_binary_expression(&Operator::Minus, access_value, start_offset.into())?
                .into_int_value()
        } else {
            let accessors = access_statements
                .iter()
                .map(|it| {
                    let value = self.generate_expression(it).expect("Uncaught invalid accessor statement");
                    Ok(cast_if_needed!(
                        self,
                        self.index.get_type(DINT_TYPE)?,
                        self.get_type_hint_for(it)?,
                        value,
                        None
                    )?
                    .into_int_value())
                })
                .collect::<Result<Vec<_>, Diagnostic>>()?;

            if access_statements.len() != index_offsets.len() {
                unreachable!("Amount of access statements and dimensions does not match.")
            }

            let lengths = get_dimension_lengths(self.llvm, &index_offsets)?;
            let dimension_offsets = get_vla_accessor_factors(self.llvm, &lengths)?;
            let adjusted_accessors = normalize_offsets(self.llvm, &accessors, &index_offsets)?;

            int_value_multiply_accumulate(
                self.llvm,
                &adjusted_accessors.iter().zip(&dimension_offsets).collect::<Vec<_>>(),
            )?
        };

        let inner_type = self.index.find_elementary_pointer_type(type_.get_type_information());
        let inner_type_name = inner_type.get_inner_array_type_name().unwrap_or_else(|| inner_type.get_name());
        let element_type = self.llvm_index.get_associated_type(inner_type_name)?;

        unsafe { builder.build_in_bounds_gep(element_type, vla_arr_ptr, &[accessor], "arr_val") }
            .map_err(Into::into)
    }

    /// generates a reference expression (member, index, deref, etc.)
    ///
    /// - `access` the ReferenceAccess of the reference to generate
    /// - `base` the "previous" segment of an optional qualified reference-access
    /// - `original_expression` the original ast-statement used to report Diagnostics
    pub(crate) fn generate_reference_expression(
        &self,
        access: &ReferenceAccess,
        base: Option<&AstNode>,
        original_expression: &AstNode,
    ) -> Result<ExpressionValue<'ink>, CodegenError> {
        match (access, base) {
            // `.foo`
            (ReferenceAccess::Global(node), _) => {
                let name = node.get_flat_reference_name().unwrap_or("unknown");

                let value = self.create_llvm_pointer_value_for_reference(
                    None,
                    self.get_load_name(node).as_deref().unwrap_or(name),
                    node,
                )?;

                let pointee = {
                    let datatype = self.annotations.get_type(node, self.index).unwrap();
                    self.llvm_index.get_associated_type(&datatype.name).unwrap()
                };

                Ok(ExpressionValue::LValue(value, pointee))
            }

            // `base.member` or just `member`
            (ReferenceAccess::Member(member), base) => {
                let base_value = base.map(|it| self.generate_expression_value(it)).transpose()?;

                if let AstStatement::DirectAccess(data) = member.as_ref().get_stmt() {
                    let (Some(base), Some(base_value)) = (base, base_value) else {
                        return Err(Diagnostic::codegen_error("Cannot generate DirectAccess without base value.", original_expression).into());
                    };
                    self.generate_direct_access_expression(base, &base_value, member, &data.access, &data.index)
                } else {
                    let member_name = member.get_flat_reference_name().unwrap_or("unknown");

                    let value = self.create_llvm_pointer_value_for_reference(
                        base_value.map(|it| (base.unwrap(), it.get_basic_value_enum().into_pointer_value())),
                        self.get_load_name(member).as_deref().unwrap_or(member_name),
                        original_expression,
                    )?;

                    let pointee = {
                        let datatype = self.annotations.get_type(original_expression, self.index).unwrap();
                        let effective_ty = self.index.find_effective_type(datatype).unwrap_or(datatype);
                        self.llvm_index.get_associated_type(&effective_ty.name).unwrap()
                    };

                    Ok(ExpressionValue::LValue(value, pointee))
                }
            }

            // `base[idx]`
            (ReferenceAccess::Index(array_idx), Some(base)) => {
                if self.annotations.get_type_or_void(base, self.index).is_vla() {
                    let value = self.generate_element_pointer_for_vla(
                        self.generate_expression_value(base)?,
                        self.annotations.get(base).expect(""),
                        array_idx.as_ref(),
                    )?;
                    let pointee = {
                        let datatype = self.annotations.get_type(original_expression, self.index).unwrap();
                        self.llvm_index.get_associated_type(datatype.get_name()).unwrap()
                    };
                    Ok(ExpressionValue::LValue(value, pointee))
                } else {
                    // normal array expression
                    let value = self.generate_element_pointer_for_array(base, array_idx)?;
                    let pointee = {
                        let datatype = self.annotations.get_type(original_expression, self.index).unwrap();
                        self.llvm_index.get_associated_type(datatype.get_name()).unwrap()
                    };

                    Ok(ExpressionValue::LValue(value, pointee))
                }
            }

            // `INT#target` (INT = base)
            (ReferenceAccess::Cast(target), Some(base)) => {
                if target.as_ref().is_identifier() {
                    let mr =
                        AstFactory::create_member_reference(target.as_ref().clone(), None, target.get_id());
                    self.generate_expression_value(&mr)
                } else if target.as_ref().is_literal(){
                    self.generate_expression_value(target.as_ref())
                } else {
                    // Otherwise just bitcast the target to the given type
                    let base_type = self.annotations.get_type_or_void(base, self.index);
                    let base_type_name = base_type.get_name();

                    // Generate the value we're casting
                    let target_value = self.generate_expression_value(target.as_ref())?;

                    // Get the LLVM type for the cast target
                    let _ = self.llvm_index.get_associated_type(base_type_name);
                    let target_llvm_type = self.llvm.context.ptr_type(AddressSpace::from(0));

                    // Perform the bitcast
                    let basic_value = target_value.get_basic_value_enum();
                    let cast_ptr = self.llvm.builder.build_bit_cast(basic_value, target_llvm_type, "cast")?;
                    let cast_value = ExpressionValue::RValue(cast_ptr);

                    Ok(cast_value)
                }
            }

            // `base^`
            (ReferenceAccess::Deref, Some(base)) => {
                let base_lvalue = self.generate_expression_value(base)?;

                let value = self.llvm.load_pointer(
                    // Normally it wouldn't be safe to just assume the pointee in the `load_pointer` call to
                    // be of type `ptr`. However, we call `into_pointer_value` on the result of it, as such
                    // LLVM actually expects the type to be `ptr` as it will panic otherwise.
                    self.llvm.context.ptr_type(
                        AddressSpace::from(ADDRESS_SPACE_GENERIC)).into(),
                        &base_lvalue.get_basic_value_enum().into_pointer_value(),
                        "deref"
                    )?;

                    let pointee = {
                        let datatype = self.annotations.get_type(original_expression, self.index).unwrap();
                        self.llvm_index.get_associated_type(&datatype.name).unwrap()
                    };
                Ok(ExpressionValue::LValue(value.into_pointer_value(), pointee))
            }

            // `&base`
            (ReferenceAccess::Address, Some(base)) => {
                let lvalue = self.generate_expression_value(base)?;
                Ok(ExpressionValue::RValue(lvalue.get_basic_value_enum()))
            }

            (ReferenceAccess::Index(_), None)   // [idx];
            | (ReferenceAccess::Cast(_), None)  // INT#;
            | (ReferenceAccess::Deref, None)    // ^;
            | (ReferenceAccess::Address, None)  // &;
            => Err(Diagnostic::codegen_error(
                "Expected a base expression, but found none",
                original_expression,
            ).into())
        }
    }

    /// generates a direct-access expression like `x.%B4`
    /// - `qualifier` the qualifier statement (see `x` above)
    /// - `qualifier_value` the generated value of the qualifier
    /// - `member` the member AstStatement (see `%B4`above)
    /// - `access` the type of access (see `B` above)
    fn generate_direct_access_expression(
        &self,
        qualifier: &AstNode,
        qualifier_value: &ExpressionValue<'ink>,
        member: &AstNode,
        access: &DirectAccessType,
        index: &AstNode,
    ) -> Result<ExpressionValue<'ink>, CodegenError> {
        let loaded_base_value = qualifier_value.as_r_value(self.llvm, self.get_load_name(qualifier))?;
        let datatype = self.get_type_hint_info_for(member)?;
        let base_type = self.get_type_hint_for(qualifier)?;

        //Generate and load the index value
        let rhs = self.generate_direct_access_index(access, index, datatype, base_type)?;
        //Shift the qualifer value right by the index value
        let shift = self.llvm.builder.build_right_shift(
            loaded_base_value.into_int_value(),
            rhs,
            base_type.get_type_information().is_signed_int(),
            "shift",
        )?;
        //Trunc the result to the get only the target size
        let value = self.llvm.builder.build_int_truncate_or_bit_cast(
            shift,
            self.llvm_index.get_associated_type(datatype.get_name())?.into_int_type(),
            "",
        )?;

        let result = if datatype.get_type_information().is_bool() {
            // since booleans are i1 internally, but i8 in llvm, we need to bitwise-AND the value with 1 to make sure we end up with the expected value
            self.llvm.builder.build_and(value, self.llvm.context.i8_type().const_int(1, false), "")?
        } else {
            value
        };

        Ok(ExpressionValue::RValue(result.as_basic_value_enum()))
    }
}

/// Returns the information required to call a parameter implicitly in a function
/// If the parameter is already implicit, it does nothing.
/// if the parameter is explicit ´param := value´,
/// it returns the location of the parameter in the function declaration
/// as well as the parameter value (right side) ´param := value´ => ´value´
/// and `true` for implicit / `false` for explicit parameters
pub fn get_implicit_call_parameter<'a>(
    argument: &'a AstNode,
    parameters: &[&VariableIndexEntry],
    idx: usize,
) -> Result<(usize, &'a AstNode, bool), CodegenError> {
    let (location, rhs_assignment_value, is_implicit) = match argument.get_stmt() {
        // Explicit
        AstStatement::Assignment(data) | AstStatement::OutputAssignment(data) => {
            let Some(left_name) = data.left.as_ref().get_flat_reference_name() else {
                return Err(
                    //TODO: use global context to get an expression slice
                    Diagnostic::new("Expression is not assignable")
                        .with_error_code("E050")
                        .with_location(argument)
                        .into(),
                );
            };

            let loc = parameters
                .iter()
                .position(|p| p.get_name().eq_ignore_ascii_case(left_name))
                .ok_or_else(|| Diagnostic::unresolved_reference(left_name, data.left.as_ref()))?;

            (loc, data.right.as_ref(), false)
        }

        // Implicit
        _ => (idx, argument, true),
    };

    Ok((location, rhs_assignment_value, is_implicit))
}

/// turns the given IntValue into an i1 by comparing it to 0 (of the same size)
pub fn to_i1<'a>(value: IntValue<'a>, builder: &Builder<'a>) -> Result<IntValue<'a>, CodegenError> {
    if value.get_type().get_bit_width() > 1 {
        builder
            .build_int_compare(IntPredicate::NE, value, value.get_type().const_int(0, false), "")
            .map_err(Into::into)
    } else {
        Ok(value)
    }
}

/// Gets a collection of start- and end-values for each dimension of a variable length array.
///
/// # Safety
///
/// performs in-bounds GEP operations at runtime
fn get_indices<'ink>(
    llvm: &Llvm<'ink>,
    ndims: usize,
    dimensions_array: PointerValue<'ink>,
    dimensions_array_type: BasicTypeEnum<'ink>,
) -> Result<Vec<(IntValue<'ink>, IntValue<'ink>)>, CodegenError> {
    let i32_type = llvm.i32_type();
    (0..ndims)
        .map(|i| unsafe {
            let (start_ptr, end_ptr) = (
                llvm.builder.build_in_bounds_gep(
                    dimensions_array_type,
                    dimensions_array,
                    &[i32_type.const_zero(), i32_type.const_int(i as u64 * 2, false)],
                    format!("start_idx_ptr{i}").as_str(),
                )?,
                llvm.builder.build_in_bounds_gep(
                    dimensions_array_type,
                    dimensions_array,
                    &[i32_type.const_zero(), i32_type.const_int(1 + i as u64 * 2, false)],
                    format!("end_idx_ptr{i}").as_str(),
                )?,
            );
            Ok((
                llvm.builder
                    .build_load(i32_type, start_ptr, format!("start_idx_value{i}").as_str())?
                    .into_int_value(),
                llvm.builder
                    .build_load(i32_type, end_ptr, format!("end_idx_value{i}").as_str())?
                    .into_int_value(),
            ))
        })
        .collect::<Result<Vec<_>, CodegenError>>()
}

/// Adjusts VLA accessor values to 0-indexed accessors
fn normalize_offsets<'ink>(
    llvm: &Llvm<'ink>,
    accessors: &[IntValue<'ink>],
    offsets: &[(IntValue<'ink>, IntValue<'ink>)],
) -> Result<Vec<IntValue<'ink>>, CodegenError> {
    accessors
        .iter()
        .enumerate()
        .zip(offsets.iter().map(|(start, _)| start))
        .map(|((idx, accessor), start_offset)| {
            llvm.builder
                .build_int_sub(*accessor, *start_offset, format!("adj_access{idx}").as_str())
                .map_err(Into::into)
        })
        .collect::<Result<Vec<_>, _>>()
}

fn get_dimension_lengths<'ink>(
    llvm: &Llvm<'ink>,
    offsets: &[(IntValue<'ink>, IntValue<'ink>)],
) -> Result<Vec<IntValue<'ink>>, CodegenError> {
    offsets
        .iter()
        .enumerate()
        .map(|(idx, (start, end))| {
            llvm.builder
                .build_int_add(
                    llvm.i32_type().const_int(1, false),
                    llvm.builder.build_int_sub(*end, *start, "")?,
                    format!("len_dim{idx}").as_str(),
                )
                .map_err(Into::into)
        })
        .collect::<Result<Vec<_>, _>>()
}

fn get_vla_accessor_factors<'ink>(
    llvm: &Llvm<'ink>,
    lengths: &[IntValue<'ink>],
) -> Result<Vec<IntValue<'ink>>, CodegenError> {
    (0..lengths.len())
        .map(|idx| {
            if idx == lengths.len() - 1 {
                // the last dimension has a factor of 1
                Ok(llvm.i32_type().const_int(1, false))
            } else {
                // for other dimensions, calculate size to the right
                int_value_product(llvm, &lengths[idx + 1..lengths.len()])
            }
        })
        .collect::<Result<Vec<_>, CodegenError>>()
}

/// Computes the product of all elements in a collection of IntValues
///
/// a <- a * b
fn int_value_product<'ink>(
    llvm: &Llvm<'ink>,
    values: &[IntValue<'ink>],
) -> Result<IntValue<'ink>, CodegenError> {
    let i32_type = llvm.i32_type();
    let accum_ptr = llvm.builder.build_alloca(i32_type, "accum")?;
    llvm.builder.build_store(accum_ptr, i32_type.const_int(1, false))?;
    for val in values {
        let product = llvm.builder.build_int_mul(
            llvm.builder.build_load(i32_type, accum_ptr, "load_accum")?.into_int_value(),
            *val,
            "product",
        )?;
        llvm.builder.build_store(accum_ptr, product)?;
    }

    Ok(llvm.builder.build_load(i32_type, accum_ptr, "accessor_factor")?.into_int_value())
}

/// Iterates over a collection of tuples, computes the product of the two numbers
/// and adds that product to an accumulator.
///
/// a <- a + (b * c)
fn int_value_multiply_accumulate<'ink>(
    llvm: &Llvm<'ink>,
    values: &[(&IntValue<'ink>, &IntValue<'ink>)],
) -> Result<IntValue<'ink>, CodegenError> {
    let i32_type = llvm.i32_type();
    let accum = llvm.builder.build_alloca(i32_type, "accum")?;
    llvm.builder.build_store(accum, i32_type.const_zero())?;
    for (left, right) in values {
        let product = llvm.builder.build_int_mul(**left, **right, "multiply")?;
        let curr = llvm.builder.build_int_add(
            llvm.builder.build_load(i32_type, accum, "load_accum")?.into_int_value(),
            product,
            "accumulate",
        )?;
        llvm.builder.build_store(accum, curr)?;
    }
    Ok(llvm.builder.build_load(i32_type, accum, "accessor")?.into_int_value())
}

// XXX: Could be problematic with https://github.com/PLC-lang/rusty/issues/668
/// Returns false if any argument in the given list is an (output-)assignment and true otherwise
fn arguments_are_implicit(arguments: &[&AstNode]) -> bool {
    !arguments.iter().any(|argument| argument.is_assignment() || argument.is_output_assignment())
}

/// when generating an assignment to a direct-access (e.g. a.b.c.%W3.%X2 := 2;)
/// we want to deconstruct the sequence into the base-statement  (a.b.c) and the sequence
/// of direct-access commands (vec![%W3, %X2])
fn collect_base_and_direct_access_for_assignment(
    left_statement: &AstNode,
) -> Option<(&AstNode, Vec<&AstNode>)> {
    let mut current = Some(left_statement);
    let mut access_sequence = Vec::new();

    while let Some(AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(m), base })) =
        current.map(|it| it.get_stmt())
    {
        if matches!(m.get_stmt(), AstStatement::DirectAccess { .. }) {
            access_sequence.insert(0, m.as_ref());
            current = base.as_deref();
        } else {
            break;
        }
    }

    current.zip(Some(access_sequence))
}
