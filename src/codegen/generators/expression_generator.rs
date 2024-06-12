// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    codegen::{
        debug::{Debug, DebugBuilderEnum},
        llvm_index::LlvmTypedIndex,
        llvm_typesystem::{cast_if_needed, get_llvm_int_type},
    },
    index::{
        const_expressions::ConstId, ArgumentType, ImplementationIndexEntry, Index, PouIndexEntry,
        VariableIndexEntry, VariableType,
    },
    resolver::{AnnotationMap, AstAnnotations, StatementAnnotation},
    typesystem::{
        is_same_type_class, DataType, DataTypeInformation, DataTypeInformationProvider, Dimension,
        StringEncoding, VarArgs, DINT_TYPE, INT_SIZE, INT_TYPE, LINT_TYPE,
    },
};
use inkwell::{
    builder::Builder,
    types::{BasicType, BasicTypeEnum},
    values::{
        ArrayValue, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FloatValue, IntValue, PointerValue,
        StructValue, VectorValue,
    },
    AddressSpace, FloatPredicate, IntPredicate,
};
use plc_ast::{
    ast::{
        flatten_expression_list, AstFactory, AstNode, AstStatement, DirectAccessType, Operator,
        ReferenceAccess, ReferenceExpr,
    },
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::{Diagnostic, INTERNAL_LLVM_ERROR};
use plc_source::source_location::SourceLocation;
use plc_util::convention::qualified_name;
use rustc_hash::FxHashSet;
use std::vec;

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
    /// the assignmentstatement in the call-argument list (a:=3)
    assignment_statement: &'b AstNode,
    /// the name of the function we're calling
    function_name: &'b str,
    /// the position of the argument in the POU's argument's list
    index: u32,
    /// a pointer to the struct instance that carries the call's arguments
    parameter_struct: PointerValue<'a>,
}

#[derive(Debug)]
pub enum ExpressionValue<'ink> {
    /// A Locator-Value
    /// An lvalue (locator value) represents an object that occupies some identifiable location in memory (i.e. has an address).
    LValue(PointerValue<'ink>),
    /// An expression that does not represent an object occupying some identifiable location in memory.
    RValue(BasicValueEnum<'ink>),
}

impl<'ink> ExpressionValue<'ink> {
    /// returns the value represented by this ExpressionValue
    pub fn get_basic_value_enum(&self) -> BasicValueEnum<'ink> {
        match self {
            ExpressionValue::LValue(it) => it.as_basic_value_enum(),
            ExpressionValue::RValue(it) => it.to_owned(),
        }
    }

    /// returns the given expression value as an r-value which means that it will load
    /// the pointer, if this is an l_value
    pub fn as_r_value(&self, llvm: &Llvm<'ink>, load_name: Option<String>) -> BasicValueEnum<'ink> {
        match self {
            ExpressionValue::LValue(it) => llvm.load_pointer(it, load_name.as_deref().unwrap_or("")),
            ExpressionValue::RValue(it) => it.to_owned(),
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
    ) -> Result<&'b FunctionContext<'ink, 'b>, Diagnostic> {
        self.function_context.ok_or_else(|| Diagnostic::missing_function(statement.get_location()))
    }

    /// entry point into the expression generator.
    /// generates the given expression and returns the resulting BasicValueEnum
    pub fn generate_expression(&self, expression: &AstNode) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        // If the expression was replaced by the resolver, generate the replacement
        if let Some(StatementAnnotation::ReplacementAst { statement }) = self.annotations.get(expression) {
            // we trust that the validator only passed us valid parameters (so left & right should be same type)
            return self.generate_expression(statement);
        }

        let v = self
            .generate_expression_value(expression)?
            .as_r_value(self.llvm, self.get_load_name(expression))
            .as_basic_value_enum();

        let Some(target_type) = self.annotations.get_type_hint(expression, self.index) else {
            // no type-hint -> we can return the value as is
            return Ok(v);
        };
        let actual_type = self.annotations.get_type_or_void(expression, self.index);
        Ok(cast_if_needed!(self, target_type, actual_type, v, self.annotations.get(expression)))
    }

    fn register_debug_location(&self, statement: &AstNode) {
        let function_context =
            self.function_context.expect("Cannot generate debug info without function context");
        let line = statement.get_location().get_line_plus_one();
        let column = statement.get_location().get_column();
        self.debug.set_debug_location(self.llvm, &function_context.function, line, column);
    }

    pub fn generate_expression_value(
        &self,
        expression: &AstNode,
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
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
            AstStatement::ReferenceExpr(data) => {
                let res =
                    self.generate_reference_expression(&data.access, data.base.as_deref(), expression)?;
                let val = match res {
                    ExpressionValue::LValue(val) => {
                        ExpressionValue::LValue(self.auto_deref_if_necessary(val, expression))
                    }
                    ExpressionValue::RValue(val) => {
                        let val = if val.is_pointer_value() {
                            self.auto_deref_if_necessary(val.into_pointer_value(), expression)
                                .as_basic_value_enum()
                        } else {
                            val
                        };
                        ExpressionValue::RValue(val)
                    }
                };
                Ok(val)
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
            // TODO: Hardware access needs to be evaluated, see #648
            AstStatement::HardwareAccess { .. } => {
                Ok(ExpressionValue::RValue(self.llvm.i32_type().const_zero().into()))
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
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
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
                Diagnostic::codegen_error(message, expression.get_location())
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
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let l_type_hint = self.get_type_hint_for(left)?;
        let ltype = self.index.get_intrinsic_type_by_name(l_type_hint.get_name()).get_type_information();
        let r_type_hint = self.get_type_hint_for(right)?;
        let rtype = self.index.get_intrinsic_type_by_name(r_type_hint.get_name()).get_type_information();
        if ltype.is_bool() && rtype.is_bool() {
            return self.generate_bool_binary_expression(operator, left, right);
        }
        if ltype.is_int() && rtype.is_int() {
            Ok(self.create_llvm_int_binary_expression(
                operator,
                self.generate_expression(left)?,
                self.generate_expression(right)?,
            ))
        } else if ltype.is_float() && rtype.is_float() {
            Ok(self.create_llvm_float_binary_expression(
                operator,
                self.generate_expression(left)?,
                self.generate_expression(right)?,
            ))
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
    ) -> Result<IntValue<'ink>, Diagnostic> {
        let reference = self.generate_expression(index)?;
        //Load the reference
        if reference.is_int_value() {
            //This cast is needed to convert the index/reference to the type of original expression
            //being accessed.
            //The reason is that llvm expects a shift operation to happen on the same type, and
            //this is what the direct access will eventually end up in.
            let reference =
                cast_if_needed!(self, target_type, self.get_type_hint_for(index)?, reference, None)
                    .into_int_value();
            //Multiply by the bitwitdh
            if access.get_bit_width() > 1 {
                let bitwidth =
                    reference.get_type().const_int(access.get_bit_width(), access_type.is_signed_int());

                Ok(self.llvm.builder.build_int_mul(reference, bitwidth, ""))
            } else {
                Ok(reference)
            }
        } else {
            Err(Diagnostic::new(format!("Cannot cast from {} to Integer Type", access_type.get_name()))
                .with_error_code("E051")
                .with_location(index.get_location()))
        }
    }

    /// generates a Unary-Expression e.g. -<expr> or !<expr>
    fn generate_unary_expression(
        &self,
        unary_operator: &Operator,
        expression: &AstNode,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let value = match unary_operator {
            Operator::Not => {
                let operator = self.generate_expression(expression)?.into_int_value();
                let operator = if self
                    .get_type_hint_for(expression)
                    .map(|it| it.get_type_information().is_bool())
                    .unwrap_or_default()
                {
                    to_i1(operator, &self.llvm.builder)
                } else {
                    operator
                };

                Ok(self.llvm.builder.build_not(operator, "tmpVar").as_basic_value_enum())
            }
            Operator::Minus => {
                let generated_exp = self.generate_expression(expression)?;
                if generated_exp.is_float_value() {
                    Ok(self
                        .llvm
                        .builder
                        .build_float_neg(generated_exp.into_float_value(), "tmpVar")
                        .as_basic_value_enum())
                } else if generated_exp.is_int_value() {
                    Ok(self
                        .llvm
                        .builder
                        .build_int_neg(generated_exp.into_int_value(), "tmpVar")
                        .as_basic_value_enum())
                } else {
                    Err(Diagnostic::codegen_error(
                        "Negated expression must be numeric",
                        expression.get_location(),
                    ))
                }
            }
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
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
        // find the pou we're calling
        let pou = self.annotations.get_call_name(operator).zip(self.annotations.get_qualified_name(operator))
            .and_then(|(call_name, qualified_name)| self.index.find_pou(call_name)
            // for some functions (builtins) the call name does not exist in the index, we try to call with the originally defined generic functions
            .or_else(|| self.index.find_pou(qualified_name)))
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
        let implementation_name = implementation.get_call_name();
        // if the function is builtin, generate a basic value enum for it
        if let Some(builtin) = self.index.get_builtin_function(implementation_name) {
            // adr, ref, etc.
            return builtin.codegen(self, parameters_list.as_slice(), operator.get_location());
        }

        let mut arguments_list = self.generate_pou_call_arguments_list(
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
                    operator.get_location(),
                )
            })?;

        // generate the debug statetment for a call
        self.register_debug_location(operator);

        // if this is a function that returns an aggregate type we need to allocate an out.pointer
        let by_ref_func_out: Option<PointerValue> = if let PouIndexEntry::Function { return_type, .. } = pou {
            let data_type = self.index.get_effective_type_or_void_by_name(return_type);
            if data_type.is_aggregate_type() {
                // this is a function call with a return variable fed as an out-pointer
                let llvm_type = self.llvm_index.get_associated_type(data_type.get_name())?;
                let out_pointer = self.llvm.create_local_variable("", &llvm_type);
                // add the out-ptr as its first parameter
                arguments_list.insert(0, out_pointer.into());
                Some(out_pointer)
            } else {
                None
            }
        } else {
            None
        };

        // if the target is a function, declare the struct locally
        // assign all parameters into the struct values
        let call = &self.llvm.builder.build_call(function, &arguments_list, "call");

        // so grab either:
        // - the out-pointer if we generated one in by_ref_func_out
        // - or the call's return value
        // - or a null-ptr
        let value = by_ref_func_out.map(|it| Ok(ExpressionValue::LValue(it))).unwrap_or_else(|| {
            let v = call.try_as_basic_value().either(Ok, |_| {
                // we return an uninitialized int pointer for void methods :-/
                // dont deref it!!
                Ok(get_llvm_int_type(self.llvm.context, INT_SIZE, INT_TYPE)
                    .ptr_type(AddressSpace::from(ADDRESS_SPACE_CONST))
                    .const_null()
                    .as_basic_value_enum())
            });
            v.map(ExpressionValue::RValue)
        });

        // after the call we need to copy the values for assigned outputs
        // this is only necessary for outputs defined as `rusty::index::ArgumentType::ByVal` (PROGRAM, FUNCTION_BLOCK)
        // FUNCTION outputs are defined as `rusty::index::ArgumentType::ByRef`
        if !pou.is_function() {
            let parameter_struct = match arguments_list.first() {
                Some(v) => v.into_pointer_value(),
                None => self.generate_lvalue(operator)?,
            };
            self.assign_output_values(parameter_struct, implementation_name, parameters_list)?
        }

        value
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
    ) -> Result<(), Diagnostic> {
        for (index, assignment_statement) in parameters.into_iter().enumerate() {
            self.assign_output_value(&CallParameterAssignment {
                assignment_statement,
                function_name,
                index: index as u32,
                parameter_struct,
            })?
        }
        Ok(())
    }

    fn assign_output_value(&self, param_context: &CallParameterAssignment) -> Result<(), Diagnostic> {
        match param_context.assignment_statement.get_stmt() {
            AstStatement::OutputAssignment(data) | AstStatement::Assignment(data) => self
                .generate_explicit_output_assignment(
                    param_context.parameter_struct,
                    param_context.function_name,
                    &data.left,
                    &data.right,
                ),
            _ => self.generate_output_assignment(param_context),
        }
    }

    fn generate_output_assignment(&self, param_context: &CallParameterAssignment) -> Result<(), Diagnostic> {
        let builder = &self.llvm.builder;
        let expression = param_context.assignment_statement;
        let parameter_struct = param_context.parameter_struct;
        let function_name = param_context.function_name;
        let index = param_context.index;
        if let Some(parameter) = self.index.get_declared_parameter(function_name, index) {
            if matches!(parameter.get_variable_type(), VariableType::Output)
                && !matches!(expression.get_stmt(), AstStatement::EmptyStatement { .. })
            {
                {
                    let assigned_output = self.generate_lvalue(expression)?;

                    let assigned_output_type =
                        self.annotations.get_type_or_void(expression, self.index).get_type_information();

                    let output = builder.build_struct_gep(parameter_struct, index, "").map_err(|_| {
                        Diagnostic::codegen_error(
                            format!("Cannot build generate parameter: {parameter:#?}"),
                            parameter.source_location.clone(),
                        )
                    })?;

                    let output_value_type =
                        self.index.get_type_information_or_void(parameter.get_type_name());

                    if assigned_output_type.is_aggregate() && output_value_type.is_aggregate() {
                        self.build_memcpy(
                            assigned_output,
                            assigned_output_type,
                            expression.get_location(),
                            output,
                            output_value_type,
                            parameter.source_location.clone(),
                        )?;
                    } else {
                        let output_value = builder.build_load(output, "");
                        builder.build_store(assigned_output, output_value);
                    }
                }
            }
        }
        Ok(())
    }

    fn generate_explicit_output_assignment(
        &self,
        parameter_struct: PointerValue<'ink>,
        function_name: &str,
        left: &AstNode,
        right: &AstNode,
    ) -> Result<(), Diagnostic> {
        if let Some(StatementAnnotation::Variable { qualified_name, .. }) = self.annotations.get(left) {
            let parameter = self
                .index
                .find_fully_qualified_variable(qualified_name)
                .ok_or_else(|| Diagnostic::unresolved_reference(qualified_name, left.get_location()))?;
            let index = parameter.get_location_in_parent();
            self.assign_output_value(&CallParameterAssignment {
                assignment_statement: right,
                function_name,
                index,
                parameter_struct,
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
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, Diagnostic> {
        let arguments_list = if matches!(pou, PouIndexEntry::Function { .. }) {
            // we're calling a function
            let declared_parameters = self.index.get_declared_parameters(implementation.get_type_name());
            self.generate_function_arguments(pou, passed_parameters, declared_parameters)?
        } else {
            // no function
            let (class_ptr, call_ptr) = match pou {
                PouIndexEntry::Method { .. } => {
                    let class_ptr = self.generate_lvalue(operator)?;
                    let call_ptr =
                        self.allocate_function_struct_instance(implementation.get_call_name(), operator)?;
                    (Some(class_ptr), call_ptr)
                }
                // TODO: find a more reliable way to make sure if this is a call into a local action!!
                PouIndexEntry::Action { .. }
                    if matches!(
                        operator.get_stmt(),
                        AstStatement::ReferenceExpr(ReferenceExpr { base: None, .. })
                    ) =>
                {
                    // special handling for local actions, get the parameter from the function context
                    function_context
                        .function
                        .get_first_param()
                        .map(|call_ptr| (None, call_ptr.into_pointer_value()))
                        .ok_or_else(|| Diagnostic::cannot_generate_call_statement(operator))?
                }
                _ => {
                    let call_ptr = self.generate_lvalue(operator)?;
                    (None, call_ptr)
                }
            };

            // generate the pou call assignments
            self.generate_stateful_pou_arguments(
                implementation.get_call_name(),
                class_ptr,
                call_ptr,
                passed_parameters,
            )?
        };
        Ok(arguments_list)
    }

    fn generate_function_arguments(
        &self,
        pou: &PouIndexEntry,
        passed_parameters: &[&AstNode],
        declared_parameters: Vec<&VariableIndexEntry>,
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, Diagnostic> {
        let mut result = Vec::new();
        let mut variadic_parameters = Vec::new();
        let mut passed_param_indices = Vec::new();
        for (i, parameter) in passed_parameters.iter().enumerate() {
            let (i, parameter, _) = get_implicit_call_parameter(parameter, &declared_parameters, i)?;

            // parameter_info includes the declaration type and type name
            let parameter_info = declared_parameters
                .get(i)
                .map(|it| {
                    let name = it.get_type_name();
                    if let Some(DataTypeInformation::Pointer { inner_type_name, auto_deref: true, .. }) =
                        self.index.find_effective_type_info(name)
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
                        variadic_parameters.push(parameter);
                        Ok(None)
                    } else {
                        // we are not variadic, we have too many parameters here
                        Err(Diagnostic::codegen_error("Too many parameters", parameter.get_location()))
                    }
                })?;

            if let Some((declaration_type, type_name)) = parameter_info {
                let argument: BasicValueEnum = if declaration_type.is_by_ref()
                    || (self.index.get_effective_type_or_void_by_name(type_name).is_aggregate_type()
                        && declaration_type.is_input())
                {
                    let declared_parameter = declared_parameters.get(i);
                    self.generate_argument_by_ref(parameter, type_name, declared_parameter.copied())?
                } else {
                    // by val
                    if !parameter.is_empty_statement() {
                        self.generate_expression(parameter)?
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
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        if argument.is_empty_statement() {
            // Uninitialized var_output / var_in_out
            let v_type = self
                .llvm_index
                .find_associated_type(type_name)
                .ok_or_else(|| Diagnostic::unknown_type(type_name, argument.get_location()))?;

            let ptr_value = self.llvm.builder.build_alloca(v_type, "");
            if let Some(p) = declared_parameter {
                if let Some(initial_value) =
                    self.get_initial_value(&p.initial_value, &self.get_parameter_type(p))
                {
                    let value = self.generate_expression(initial_value)?;
                    self.llvm.builder.build_store(ptr_value, value);
                }
            }

            return Ok(ptr_value.into());
        }

        // Generate the element pointer, then...
        let value = {
            let value = self.generate_expression_value(argument)?;
            match value {
                ExpressionValue::LValue(v) => v,
                ExpressionValue::RValue(_v) => {
                    // Passed a literal to a byref parameter?
                    let value = self.generate_expression(argument)?;
                    let argument = self.llvm.builder.build_alloca(value.get_type(), "");
                    self.llvm.builder.build_store(argument, value);
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
                return Ok(cast_if_needed!(
                    self,
                    hint,
                    actual_type,
                    value.into(),
                    self.annotations.get(argument)
                ));
            };

            // From https://llvm.org/docs/LangRef.html#bitcast-to-instruction: The ‘bitcast’ instruction takes
            // a value to cast, which must be a **non-aggregate** first class value [...]
            if !actual_type_info.is_aggregate() && actual_type_info != target_type_info {
                return Ok(self.llvm.builder.build_bitcast(
                    value,
                    self.llvm_index.get_associated_type(hint.get_name())?,
                    "",
                ));
            }
        }

        // ...check if we can bitcast an array to a pointer, i.e. `[81 x i8]*` should be passed as a `i8*`
        if value.get_type().get_element_type().is_array_type() {
            let res = self.llvm.builder.build_bitcast(
                value,
                value
                    .get_type()
                    .get_element_type()
                    .into_array_type()
                    .get_element_type()
                    .ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)),
                "",
            );

            return Ok(res.into_pointer_value().into());
        }

        // ...otherwise no bitcasting was needed, thus return the generated element pointer as is
        Ok(value.into())
    }

    pub fn generate_variadic_arguments_list(
        &self,
        pou: &PouIndexEntry,
        variadic_params: &[&AstNode],
    ) -> Result<Vec<BasicValueEnum<'ink>>, Diagnostic> {
        // get the real varargs from the index
        if let Some((var_args, argument_type)) = self
            .index
            .get_variadic_member(pou.get_name())
            .and_then(|it| it.get_varargs().zip(Some(it.get_declaration_type())))
        {
            let generated_params = variadic_params
                .iter()
                .map(|param_statement| {
                    self.get_type_hint_for(param_statement).map(|it| it.get_name()).and_then(|type_name| {
                        // if the variadic is defined in a by_ref block, we need to pass the argument as reference
                        if argument_type.is_by_ref() {
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
                    ty.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into()
                } else {
                    ty
                };

                let size = generated_params.len();
                let size_param = self.llvm.i32_type().const_int(size as u64, true);

                let arr = ty.array_type(size as u32);
                let arr_storage = self.llvm.builder.build_alloca(arr, "");
                for (i, ele) in generated_params.iter().enumerate() {
                    let ele_ptr = self.llvm.load_array_element(
                        arr_storage,
                        &[
                            self.llvm.context.i32_type().const_zero(),
                            self.llvm.context.i32_type().const_int(i as u64, true),
                        ],
                        "",
                    )?;
                    self.llvm.builder.build_store(ele_ptr, *ele);
                }

                // bitcast the array to pointer so it matches the declared function signature
                let arr_storage = self.llvm.builder.build_bitcast(
                    arr_storage,
                    ty.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)),
                    "",
                );

                Ok(vec![size_param.into(), arr_storage])
            } else {
                Ok(generated_params)
            }
        } else {
            unreachable!("Function must be variadic")
        }
    }

    /// generates a new instance of a function called `function_name` and returns a PointerValue to it
    ///
    /// - `function_name` the name of the function as registered in the index
    /// - `context` the statement used to report a possible Diagnostic on
    /// TODO: will be deleted once methods work properly (like functions)
    fn allocate_function_struct_instance(
        &self,
        function_name: &str,
        context: &AstNode,
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        let instance_name = format!("{function_name}_instance"); // TODO: Naming convention (see plc_util/src/convention.rs)
        let function_type = self
            .llvm_index
            .find_associated_pou_type(function_name) //Using find instead of get to control the compile error
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    format!("No type associated with '{instance_name:}'"),
                    context.get_location(),
                )
            })?;

        Ok(self.llvm.create_local_variable(&instance_name, &function_type))
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
        passed_parameters: &[&AstNode],
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, Diagnostic> {
        let mut result = class_struct
            .map(|class_struct| {
                vec![class_struct.as_basic_value_enum().into(), parameter_struct.as_basic_value_enum().into()]
            })
            .unwrap_or_else(|| vec![parameter_struct.as_basic_value_enum().into()]);
        for (i, stmt) in passed_parameters.iter().enumerate() {
            let parameter = self.generate_call_struct_argument_assignment(&CallParameterAssignment {
                assignment_statement: stmt,
                function_name: pou_name,
                index: i as u32,
                parameter_struct,
            })?;
            if let Some(parameter) = parameter {
                result.push(parameter.into());
            };
        }

        Ok(result)
    }

    fn get_parameter_type(&self, parameter: &VariableIndexEntry) -> String {
        if let Some(DataTypeInformation::Pointer { inner_type_name, auto_deref: true, .. }) =
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
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let parameter_type_name = self.get_parameter_type(parameter);
        let parameter_type = self.llvm_index.get_associated_type(&parameter_type_name)?;
        match parameter.get_declaration_type() {
            ArgumentType::ByVal(..) => {
                if let Some(initial_value) =
                    self.get_initial_value(&parameter.initial_value, &parameter_type_name)
                {
                    self.generate_expression(initial_value)
                } else {
                    let ptr_value = self.llvm.builder.build_alloca(parameter_type, "");
                    Ok(self.llvm.load_pointer(&ptr_value, ""))
                }
            }
            _ => {
                let ptr_value = self.llvm.builder.build_alloca(parameter_type, "");

                // if default value is given for an output
                // we need to initialize the pointer value before returning
                if let Some(initial_value) =
                    self.get_initial_value(&parameter.initial_value, &parameter_type_name)
                {
                    let value = self.generate_expression(initial_value)?;
                    self.llvm.builder.build_store(ptr_value, value);
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
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        let parameter_value = match param_context.assignment_statement.get_stmt() {
            // explicit call parameter: foo(param := value)
            AstStatement::OutputAssignment(data) | AstStatement::Assignment(data) => {
                self.generate_formal_parameter(param_context, &data.left, &data.right)?;
                None
            }
            // foo(x)
            _ => self.generate_nameless_parameter(param_context)?,
        };

        Ok(parameter_value)
    }

    /// generates the appropriate value for the given expression where the expression
    /// is a call's implicit argument (hence: foo(3), not foo(in := 3))
    fn generate_nameless_parameter(
        &self,
        param_context: &CallParameterAssignment,
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        let builder = &self.llvm.builder;
        let function_name = param_context.function_name;
        let index = param_context.index;
        let parameter_struct = param_context.parameter_struct;
        let expression = param_context.assignment_statement;
        if let Some(parameter) = self.index.get_declared_parameter(function_name, index) {
            // this happens before the pou call
            // before the call statement we may only consider inputs and inouts
            // after the call we need to copy the output values to the correct assigned variables
            if matches!(parameter.get_variable_type(), VariableType::Output) {
                return Ok(None);
            }

            let pointer_to_param = builder.build_struct_gep(parameter_struct, index, "").map_err(|_| {
                Diagnostic::codegen_error(
                    format!("Cannot build generate parameter: {expression:#?}"),
                    expression.get_location(),
                )
            })?;

            let parameter = self
                .index
                .find_parameter(function_name, index)
                .and_then(|var| self.index.find_effective_type_by_name(var.get_type_name()))
                .map(|var| var.get_type_information())
                .unwrap_or_else(|| self.index.get_void_type().get_type_information());

            if let DataTypeInformation::Pointer { auto_deref: true, inner_type_name, .. } = parameter {
                //this is a VAR_IN_OUT assignment, so don't load the value, assign the pointer
                //expression may be empty -> generate a local variable for it
                let generated_exp = if expression.is_empty_statement() {
                    let temp_type =
                        self.llvm_index.find_associated_type(inner_type_name).ok_or_else(|| {
                            Diagnostic::unknown_type(parameter.get_name(), expression.get_location())
                        })?;
                    builder.build_alloca(temp_type, "empty_varinout").as_basic_value_enum()
                } else {
                    self.generate_lvalue(expression)?.as_basic_value_enum()
                };
                builder.build_store(pointer_to_param, generated_exp);
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
    ) -> Result<(), Diagnostic> {
        let function_name = param_context.function_name;
        let parameter_struct = param_context.parameter_struct;

        if let Some(StatementAnnotation::Variable { qualified_name, .. }) = self.annotations.get(left) {
            let parameter = self
                .index
                .find_fully_qualified_variable(qualified_name)
                .ok_or_else(|| Diagnostic::unresolved_reference(qualified_name, left.get_location()))?;
            let index = parameter.get_location_in_parent();

            // don't generate param assignments for empty statements, with the exception
            // of VAR_IN_OUT params - they need an address to point to
            let is_auto_deref = matches!(
                self.index
                    .find_effective_type_by_name(parameter.get_type_name())
                    .map(|var| var.get_type_information())
                    .unwrap_or_else(|| self.index.get_void_type().get_type_information()),
                DataTypeInformation::Pointer { auto_deref: true, .. }
            );
            if !right.is_empty_statement() || is_auto_deref {
                self.generate_call_struct_argument_assignment(&CallParameterAssignment {
                    assignment_statement: right,
                    function_name,
                    index,
                    parameter_struct,
                })?;
            };
        }
        Ok(())
    }

    /// generates an gep-statement and returns the resulting pointer
    ///
    /// - `reference_statement` - the statement to get an lvalue from
    pub fn generate_lvalue(&self, reference_statement: &AstNode) -> Result<PointerValue<'ink>, Diagnostic> {
        self.generate_expression_value(reference_statement).and_then(|it| {
            let v: Result<PointerValue, _> = it.get_basic_value_enum().try_into();
            v.map_err(|err| {
                Diagnostic::codegen_error(format!("{err:?}").as_str(), reference_statement.get_location())
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
        qualifier: Option<&PointerValue<'ink>>,
        name: &str,
        context: &AstNode,
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        let offset = &context.get_location();
        if let Some(qualifier) = qualifier {
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
                    let member_location = self
                        .index
                        .find_fully_qualified_variable(qualified_name)
                        .map(VariableIndexEntry::get_location_in_parent)
                        .ok_or_else(|| Diagnostic::unresolved_reference(qualified_name, offset.clone()))?;
                    let gep: PointerValue<'_> = self.llvm.get_member_pointer_from_struct(
                        *qualifier,
                        member_location,
                        name,
                        offset,
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
                .ok_or_else(|| Diagnostic::unresolved_reference(name, offset.clone())),
            _ => Err(Diagnostic::unresolved_reference(name, offset.clone())),
        }
    }

    fn deref(&self, accessor_ptr: PointerValue<'ink>) -> PointerValue<'ink> {
        self.llvm.load_pointer(&accessor_ptr, "deref").into_pointer_value()
    }

    pub fn ptr_as_value(&self, ptr: PointerValue<'ink>) -> BasicValueEnum<'ink> {
        let int_type = self.llvm.context.i64_type();
        if ptr.is_const() {
            ptr.const_to_int(int_type)
        } else {
            self.llvm.builder.build_ptr_to_int(ptr, int_type, "")
        }
        .as_basic_value_enum()
    }

    pub fn int_neg(&self, value: IntValue<'ink>) -> IntValue<'ink> {
        if value.is_const() {
            value.const_neg()
        } else {
            self.llvm.builder.build_int_neg(value, "")
        }
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
    ) -> PointerValue<'ink> {
        if let Some(StatementAnnotation::Variable { is_auto_deref: true, .. }) =
            self.annotations.get(statement)
        {
            self.deref(accessor_ptr)
        } else {
            accessor_ptr
        }
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
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let start_offset = dimension
            .start_offset
            .as_int_value(self.index)
            .map_err(|it| Diagnostic::codegen_error(it, access_expression.get_location()))?;

        let access_value = self.generate_expression(access_expression)?;
        //If start offset is not 0, adjust the current statement with an add operation
        let result = if start_offset != 0 {
            let access_int_value = access_value.into_int_value();
            let access_int_type = access_int_value.get_type();
            self.llvm.builder.build_int_sub(
                access_int_value,
                access_int_type.const_int(start_offset as u64, true), //TODO error handling for cast
                "",
            )
        } else {
            access_value.into_int_value()
        };
        //turn it into i32 immediately
        Ok(cast_if_needed!(
            self,
            self.index.get_type(DINT_TYPE)?,
            self.get_type_hint_for(access_expression)?,
            result.as_basic_value_enum(),
            None
        ))
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
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        //Load the reference
        self.generate_expression_value(reference)
            .map(|it| it.get_basic_value_enum().into_pointer_value())
            .and_then(|lvalue| {
                if let DataTypeInformation::Array { dimensions, .. } =
                    self.get_type_hint_info_for(reference)?
                {
                    // make sure dimensions match statement list
                    let statements = access.get_as_list();
                    if statements.is_empty() || statements.len() != dimensions.len() {
                        return Err(Diagnostic::codegen_error("Invalid array access", access.get_location()));
                    }

                    // e.g. an array like `ARRAY[0..3, 0..2, 0..1] OF ...` has the lengths [ 4 , 3 , 2 ]
                    let lengths = dimensions
                        .iter()
                        .map(|d| d.get_length(self.index))
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|msg| {
                            Diagnostic::codegen_error(
                                format!("Invalid array dimensions access: {msg}").as_str(),
                                access.get_location(),
                            )
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
                                    );
                                    // take the sum of the mulitlication and the previous accumulated_value
                                    // this now becomes the new accumulated value
                                    self.create_llvm_int_binary_expression(&Operator::Plus, m_v, last_v)
                                })
                            });
                            (result, 0 /* the 0 will be ignored */)
                        },
                    );

                    // make sure we got an int-value
                    let index_access: IntValue = index_access.and_then(|it| {
                        it.try_into().map_err(|_| {
                            Diagnostic::codegen_error("non-numeric index-access", access.get_location())
                        })
                    })?;

                    let accessor_sequence = if lvalue.get_type().get_element_type().is_array_type() {
                        // e.g.: [81 x i32]*
                        // the first index (0) will point to the array -> [81 x i32]
                        // the second index (index_access) will point to the element in the array
                        vec![self.llvm.i32_type().const_zero(), index_access]
                    } else {
                        // lvalue is a pointer to type -> e.g.: i32*
                        // only one index (index_access) is needed to access the element

                        // IGNORE the additional first index (0)
                        // it would point to -> i32
                        // we can't access any element of i32
                        vec![index_access]
                    };

                    // load the access from that array
                    let pointer = self.llvm.load_array_element(lvalue, &accessor_sequence, "tmpVar")?;

                    return Ok(pointer);
                }
                Err(Diagnostic::codegen_error("Invalid array access", access.get_location()))
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
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let left_expr = self.generate_expression(left)?;
        let right_expr = self.generate_expression(right)?;

        let result = match operator {
            Operator::Plus | Operator::Minus => {
                let (ptr, index, name) = if left_type.is_pointer() && right_type.is_int() {
                    let ptr = left_expr.into_pointer_value();
                    let index = right_expr.into_int_value();
                    let name = format!("access_{}", left_type.get_name());
                    (Some(ptr), Some(index), Some(name))
                } else if left_type.is_int() && right_type.is_pointer() {
                    let ptr = right_expr.into_pointer_value();
                    let index = left_expr.into_int_value();
                    let name = format!("access_{}", right_type.get_name());
                    (Some(ptr), Some(index), Some(name))
                } else {
                    // if left and right are both pointers we can not perform plus/minus
                    (None, None, None)
                };

                if let (Some(ptr), Some(mut index), Some(name)) = (ptr, index, name) {
                    // if operator is minus we need to negate the index
                    if let Operator::Minus = operator {
                        index = self.int_neg(index);
                    }

                    Ok(self.llvm.load_array_element(ptr, &[index], name.as_str())?.as_basic_value_enum())
                } else {
                    Err(Diagnostic::codegen_error(
                        format!("'{operator}' operation must contain one int type").as_str(),
                        expression.get_location(),
                    ))
                }
            }
            Operator::Equal => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::EQ,
                    self.convert_to_int_value_if_pointer(left_expr),
                    self.convert_to_int_value_if_pointer(right_expr),
                    "tmpVar",
                )
                .as_basic_value_enum()),
            Operator::NotEqual => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::NE,
                    self.convert_to_int_value_if_pointer(left_expr),
                    self.convert_to_int_value_if_pointer(right_expr),
                    "tmpVar",
                )
                .as_basic_value_enum()),
            Operator::Less => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::SLT,
                    self.convert_to_int_value_if_pointer(left_expr),
                    self.convert_to_int_value_if_pointer(right_expr),
                    "tmpVar",
                )
                .as_basic_value_enum()),
            Operator::Greater => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::SGT,
                    self.convert_to_int_value_if_pointer(left_expr),
                    self.convert_to_int_value_if_pointer(right_expr),
                    "tmpVar",
                )
                .as_basic_value_enum()),
            Operator::LessOrEqual => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::SLE,
                    self.convert_to_int_value_if_pointer(left_expr),
                    self.convert_to_int_value_if_pointer(right_expr),
                    "tmpVar",
                )
                .as_basic_value_enum()),
            Operator::GreaterOrEqual => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::SGE,
                    self.convert_to_int_value_if_pointer(left_expr),
                    self.convert_to_int_value_if_pointer(right_expr),
                    "tmpVar",
                )
                .as_basic_value_enum()),
            _ => Err(Diagnostic::codegen_error(
                format!("Operator '{operator}' unimplemented for pointers").as_str(),
                expression.get_location(),
            )),
        };

        result
    }

    /// if the given `value` is a pointer value, it converts the pointer into an int_value to access the pointer's
    /// address, if the given `value` is already an IntValue it is returned as is
    pub fn convert_to_int_value_if_pointer(&self, value: BasicValueEnum<'ink>) -> IntValue<'ink> {
        match value {
            BasicValueEnum::PointerValue(v) => self.ptr_as_value(v).into_int_value(),
            BasicValueEnum::IntValue(v) => v,
            _ => unimplemented!(),
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
    ) -> BasicValueEnum<'ink> {
        let int_lvalue = left_value.into_int_value();
        let int_rvalue = right_value.into_int_value();

        let value = match operator {
            Operator::Plus => self.llvm.builder.build_int_add(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Minus => self.llvm.builder.build_int_sub(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Multiplication => self.llvm.builder.build_int_mul(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Division => self.llvm.builder.build_int_signed_div(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Modulo => self.llvm.builder.build_int_signed_rem(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Equal => {
                self.llvm.builder.build_int_compare(IntPredicate::EQ, int_lvalue, int_rvalue, "tmpVar")
            }

            Operator::NotEqual => {
                self.llvm.builder.build_int_compare(IntPredicate::NE, int_lvalue, int_rvalue, "tmpVar")
            }

            Operator::Less => {
                self.llvm.builder.build_int_compare(IntPredicate::SLT, int_lvalue, int_rvalue, "tmpVar")
            }

            Operator::Greater => {
                self.llvm.builder.build_int_compare(IntPredicate::SGT, int_lvalue, int_rvalue, "tmpVar")
            }

            Operator::LessOrEqual => {
                self.llvm.builder.build_int_compare(IntPredicate::SLE, int_lvalue, int_rvalue, "tmpVar")
            }

            Operator::GreaterOrEqual => {
                self.llvm.builder.build_int_compare(IntPredicate::SGE, int_lvalue, int_rvalue, "tmpVar")
            }
            Operator::Xor => self.llvm.builder.build_xor(int_lvalue, int_rvalue, "tmpVar"),
            Operator::And => self.llvm.builder.build_and(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Or => self.llvm.builder.build_or(int_lvalue, int_rvalue, "tmpVar"),
            _ => unimplemented!(),
        };
        value.into()
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
    ) -> BasicValueEnum<'ink> {
        let float_lvalue = lvalue.into_float_value();
        let float_rvalue = rvalue.into_float_value();

        let value = match operator {
            Operator::Plus => self.llvm.builder.build_float_add(float_lvalue, float_rvalue, "tmpVar").into(),
            Operator::Minus => self.llvm.builder.build_float_sub(float_lvalue, float_rvalue, "tmpVar").into(),
            Operator::Multiplication => {
                self.llvm.builder.build_float_mul(float_lvalue, float_rvalue, "tmpVar").into()
            }
            Operator::Division => {
                self.llvm.builder.build_float_div(float_lvalue, float_rvalue, "tmpVar").into()
            }
            Operator::Modulo => {
                self.llvm.builder.build_float_rem(float_lvalue, float_rvalue, "tmpVar").into()
            }

            Operator::Equal => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OEQ, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::NotEqual => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::ONE, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Less => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OLT, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Greater => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OGT, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::LessOrEqual => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OLE, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::GreaterOrEqual => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OGE, float_lvalue, float_rvalue, "tmpVar")
                .into(),

            _ => unimplemented!(),
        };
        value
    }

    fn generate_numeric_literal(
        &self,
        stmt: &AstNode,
        number: &str,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
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
    pub fn generate_literal(&self, literal_statement: &AstNode) -> Result<ExpressionValue<'ink>, Diagnostic> {
        let cannot_generate_literal = || {
            Diagnostic::codegen_error(
                format!("Cannot generate Literal for {literal_statement:?}"),
                literal_statement.get_location(),
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
                    .map_err(|op| Diagnostic::codegen_error(op.as_str(), location.clone()))
                    .and_then(|ns| self.create_const_int(ns))
                    .map(ExpressionValue::RValue),
                AstLiteral::DateAndTime(dt) => dt
                    .value()
                    .map_err(|op| Diagnostic::codegen_error(op.as_str(), location.clone()))
                    .and_then(|ns| self.create_const_int(ns))
                    .map(ExpressionValue::RValue),
                AstLiteral::TimeOfDay(tod) => tod
                    .value()
                    .map_err(|op| Diagnostic::codegen_error(op.as_str(), location.clone()))
                    .and_then(|ns| self.create_const_int(ns))
                    .map(ExpressionValue::RValue),
                AstLiteral::Time(t) => self.create_const_int(t.value()).map(ExpressionValue::RValue),
                AstLiteral::String(s) => self.generate_string_literal(literal_statement, s.value(), location),
                AstLiteral::Array(arr) => self
                    .generate_literal_array(arr.elements().ok_or_else(cannot_generate_literal)?)
                    .map(ExpressionValue::RValue),
                AstLiteral::Null { .. } => self.llvm.create_null_ptr().map(ExpressionValue::RValue),
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
            _ => Err(cannot_generate_literal()),
        }
    }

    /// generates the string-literal `value` represented by `literal_statement`
    fn generate_string_literal(
        &self,
        literal_statement: &AstNode,
        value: &str,
        location: &SourceLocation,
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
        let expected_type = self.get_type_hint_info_for(literal_statement)?;
        self.generate_string_literal_for_type(expected_type, value, location)
    }

    fn generate_string_literal_for_type(
        &self,
        expected_type: &DataTypeInformation,
        value: &str,
        location: &SourceLocation,
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
        match expected_type {
            DataTypeInformation::String { encoding, size, .. } => {
                let declared_length = size.as_int_value(self.index).map_err(|msg| {
                    Diagnostic::codegen_error(
                        format!("Unable to generate string-literal: {msg}").as_str(),
                        location.clone(),
                    )
                })? as usize;

                match encoding {
                    StringEncoding::Utf8 => {
                        let literal =
                            self.llvm_index.find_utf08_literal_string(value).map(|it| it.as_pointer_value());
                        if let Some((literal_value, _)) = literal.zip(self.function_context) {
                            //global constant string
                            Ok(ExpressionValue::LValue(literal_value))
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
                            Ok(literal.map(|it| ExpressionValue::LValue(it.as_pointer_value())).unwrap())
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
            DataTypeInformation::Pointer { inner_type_name, auto_deref: true, .. } => {
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
            .with_location(location)),
        }
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_info_for(&self, statement: &AstNode) -> Result<&DataTypeInformation, Diagnostic> {
        self.get_type_hint_for(statement).map(DataType::get_type_information)
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_for(&self, statement: &AstNode) -> Result<&DataType, Diagnostic> {
        self.annotations
            .get_type_hint(statement, self.index)
            .or_else(|| self.annotations.get_type(statement, self.index))
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    format!("no type hint available for {statement:#?}"),
                    statement.get_location(),
                )
            })
    }

    /// generates a struct literal value with the given value assignments (ExpressionList)
    fn generate_literal_struct(&self, assignments: &AstNode) -> Result<ExpressionValue<'ink>, Diagnostic> {
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
                                Diagnostic::unresolved_reference(qualified_name, data.left.get_location())
                            })?;

                        let index_in_parent = member.get_location_in_parent();
                        let value = self.generate_expression(data.right.as_ref())?;

                        uninitialized_members.remove(member);
                        member_values.push((index_in_parent, value));
                    } else {
                        return Err(Diagnostic::codegen_error(
                            "struct member lvalue required as left operand of assignment",
                            data.left.get_location(),
                        ));
                    }
                } else {
                    return Err(Diagnostic::codegen_error(
                        "struct literal must consist of explicit assignments in the form of member := value",
                        assignment.get_location(),
                    ));
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
                        Diagnostic::cannot_generate_initializer(
                            member.get_qualified_name(),
                            assignments.get_location(),
                        )
                    })?;

                member_values.push((member.get_location_in_parent(), initial_value));
            }
            let struct_type = self.llvm_index.get_associated_type(struct_name)?.into_struct_type();
            if member_values.len() == struct_type.count_fields() as usize {
                member_values.sort_by(|(a, _), (b, _)| a.cmp(b));
                let ordered_values: Vec<BasicValueEnum<'ink>> =
                    member_values.iter().map(|(_, v)| *v).collect();

                return Ok(ExpressionValue::RValue(
                    struct_type.const_named_struct(ordered_values.as_slice()).as_basic_value_enum(),
                ));
            } else {
                Err(Diagnostic::codegen_error(
                    format!(
                        "Expected {} fields for Struct {}, but found {}.",
                        struct_type.count_fields(),
                        struct_name,
                        member_values.len()
                    ),
                    assignments.get_location(),
                ))
            }
        } else {
            Err(Diagnostic::codegen_error(
                format!("Expected Struct-literal, got {assignments:#?}"),
                assignments.get_location(),
            ))
        }
    }

    /// generates an array literal with the given optional elements (represented as an ExpressionList)
    pub fn generate_literal_array(&self, initializer: &AstNode) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let array_value = self.generate_literal_array_value(
            initializer,
            self.get_type_hint_info_for(initializer)?,
            &initializer.get_location(),
        )?;
        return Ok(array_value.as_basic_value_enum());
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
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let (inner_type, expected_len) =
            if let DataTypeInformation::Array { inner_type_name, dimensions, .. } = data_type {
                let len: u32 = dimensions
                    .iter()
                    .map(|d| d.get_length(self.index))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|msg| Diagnostic::codegen_error(msg.as_str(), location.clone()))?
                    .into_iter()
                    .product();

                self.index.get_type(inner_type_name).map(|inner_type| (inner_type, len as usize))
            } else {
                Err(Diagnostic::codegen_error(
                    format!("Expected array type but found: {:}", data_type.get_name()).as_str(),
                    location.clone(),
                ))
            }?;

        // for arrays of struct we cannot flatten the expression list
        // to generate the passed structs we need an expression list of assignments
        // flatten_expression_list will will return a vec of only assignments
        let elements =
            if self.index.get_effective_type_or_void_by_name(inner_type.get_name()).information.is_struct() {
                match elements.get_stmt() {
                    AstStatement::ExpressionList(expressions) => expressions.iter().collect(),
                    _ => unreachable!("This should always be an expression list"),
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
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        match operator {
            Operator::And | Operator::Or => {
                self.generate_bool_short_circuit_expression(operator, left, right)
            }
            Operator::Equal => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::EQ,
                    to_i1(self.generate_expression(left)?.into_int_value(), &self.llvm.builder),
                    to_i1(self.generate_expression(right)?.into_int_value(), &self.llvm.builder),
                    "",
                )
                .as_basic_value_enum()),
            Operator::NotEqual => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::NE,
                    to_i1(self.generate_expression(left)?.into_int_value(), &self.llvm.builder),
                    to_i1(self.generate_expression(right)?.into_int_value(), &self.llvm.builder),
                    "",
                )
                .as_basic_value_enum()),
            Operator::Xor => Ok(self
                .llvm
                .builder
                .build_xor(
                    to_i1(self.generate_expression(left)?.into_int_value(), &self.llvm.builder),
                    to_i1(self.generate_expression(right)?.into_int_value(), &self.llvm.builder),
                    "",
                )
                .as_basic_value_enum()),
            _ => Err(Diagnostic::codegen_error(
                format!("illegal boolean expresspion for operator {operator:}").as_str(),
                left.get_location().span(&right.get_location()),
            )),
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
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let builder = &self.llvm.builder;
        let lhs = to_i1(self.generate_expression(left)?.into_int_value(), builder);
        let function = self.get_function_context(left)?.function;

        let right_branch = self.llvm.context.append_basic_block(function, "");
        let continue_branch = self.llvm.context.append_basic_block(function, "");

        let final_left_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        //Compare left to 0

        match operator {
            Operator::Or => builder.build_conditional_branch(lhs, continue_branch, right_branch),
            Operator::And => builder.build_conditional_branch(lhs, right_branch, continue_branch),
            _ => {
                return Err(Diagnostic::codegen_error(
                    format!("Cannot generate phi-expression for operator {operator:}"),
                    left.get_location(),
                ))
            }
        };

        builder.position_at_end(right_branch);
        let rhs = to_i1(self.generate_expression(right)?.into_int_value(), builder);
        let final_right_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        builder.build_unconditional_branch(continue_branch);

        builder.position_at_end(continue_branch);
        //Generate phi
        let phi_value = builder.build_phi(lhs.get_type(), "");
        //assert
        phi_value.add_incoming(&[(&lhs, final_left_block), (&rhs, final_right_block)]);

        Ok(phi_value.as_basic_value())
    }

    fn create_const_int(&self, value: i64) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let value = self.llvm.create_const_numeric(
            &self.llvm_index.get_associated_type(LINT_TYPE)?,
            value.to_string().as_str(),
            SourceLocation::undefined(),
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
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
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
                left.get_location(),
            ))
        }
    }

    pub fn generate_store(
        &self,
        left: inkwell::values::PointerValue,
        left_type: &DataTypeInformation,
        right_statement: &AstNode,
    ) -> Result<(), Diagnostic> {
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
            self.llvm.builder.build_store(left, expression);
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
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        let (size, alignment) = match (left_type, right_type) {
            (
                DataTypeInformation::String { size: lsize, .. },
                DataTypeInformation::String { size: rsize, .. },
            ) => {
                let target_size = lsize
                    .as_int_value(self.index)
                    .map_err(|err| Diagnostic::codegen_error(err.as_str(), left_location.clone()))?;
                let value_size = rsize
                    .as_int_value(self.index)
                    .map_err(|err| Diagnostic::codegen_error(err.as_str(), right_location))?;
                let size = std::cmp::min(target_size - 1, value_size);
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

        self.llvm
            .builder
            .build_memcpy(left, alignment, right, alignment, size)
            .map_err(|err| Diagnostic::codegen_error(err, left_location))
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
    ) -> Result<PointerValue<'ink>, ()> {
        let builder = &self.llvm.builder;

        // array access is either directly on a reference or on another array access (ARRAY OF ARRAY)

        let StatementAnnotation::Variable { resulting_type: reference_type, .. } = reference_annotation
        else {
            unreachable!();
        };

        let struct_ptr = reference.get_basic_value_enum().into_pointer_value();
        // GEPs into the VLA struct, getting an LValue for the array pointer and the dimension array and
        // dereferences the former
        let arr_ptr_gep = self.llvm.builder.build_struct_gep(struct_ptr, 0, "vla_arr_gep")?;
        let vla_arr_ptr = builder.build_load(arr_ptr_gep, "vla_arr_ptr").into_pointer_value();
        // get pointer to array containing dimension information
        let dim_arr_gep = builder.build_struct_gep(struct_ptr, 1, "dim_arr").unwrap();

        // get lengths of dimensions
        let type_ = self.index.get_type_information_or_void(reference_type);
        let Some(ndims) = type_.get_type_information().get_dimensions() else { unreachable!() };

        // get the start/end offsets for each dimension ( ARRAY[4..10, -4..4] ...)
        let index_offsets = get_indices(self.llvm, ndims, dim_arr_gep);

        // calculate the required offset from the array pointer for the given accessor statements. this is
        // relatively straightforward for single-dimensional arrays, but is quite costly (O(n²)) for multi-dimensional arrays
        let access_statements = access.get_as_list();
        let accessor = if access_statements.len() == 1 {
            let Some(stmt) = access_statements.first() else {
                unreachable!("Must have exactly 1 access statement")
            };
            let access_value = self.generate_expression(stmt).map_err(|_| ())?;

            // if start offset is not 0, adjust the access value accordingly
            let Some(start_offset) = index_offsets.first().map(|(start, _)| *start) else {
                unreachable!("VLA must have information about dimension offsets")
            };
            self.create_llvm_int_binary_expression(&Operator::Minus, access_value, start_offset.into())
                .into_int_value()
        } else {
            // see https://plc-lang.github.io/rusty/arch/codegen.html#multi-dimensional-arrays
            // for more details on multi-dimensional array accessor calculation
            let accessors = access_statements
                .iter()
                .map(|it| {
                    self.generate_expression(it)
                        .expect("Uncaught invalid accessor statement")
                        .into_int_value()
                })
                .collect::<Vec<_>>();

            if access_statements.len() != index_offsets.len() {
                unreachable!("Amount of access statements and dimensions does not match.")
            }

            // length of a dimension is 'end - start + 1'
            let lengths = get_dimension_lengths(self.llvm, &index_offsets);

            // calculate the accessor multiplicators for each dimension.
            let dimension_offsets = get_vla_accessor_factors(self.llvm, &lengths);

            // adjust accessors for 0-indexing
            let adjusted_accessors = normalize_offsets(self.llvm, &accessors, &index_offsets);

            // calculate the resulting accessor for the given accessor statements and dimension offsets
            int_value_multiply_accumulate(
                self.llvm,
                &adjusted_accessors.iter().zip(&dimension_offsets).collect::<Vec<_>>(),
            )
        };

        Ok(unsafe { builder.build_in_bounds_gep(vla_arr_ptr, &[accessor], "arr_val") })
    }

    /// generates a reference expression (member, index, deref, etc.)
    ///
    /// - `access` the ReferenceAccess of the reference to generate
    /// - `base` the "previous" segment of an optional qualified reference-access
    /// - `original_expression` the original ast-statement used to report Diagnostics
    fn generate_reference_expression(
        &self,
        access: &ReferenceAccess,
        base: Option<&AstNode>,
        original_expression: &AstNode,
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
        match (access, base) {

            // expressions like `base.member`, or just `member`
            (ReferenceAccess::Member(member), base) => {
                let base_value = base.map(|it| self.generate_expression_value(it)).transpose()?;

                if let AstStatement::DirectAccess (data) = member.as_ref().get_stmt() {
                    let (Some(base), Some(base_value)) = (base, base_value) else {
                        return Err(Diagnostic::codegen_error("Cannot generate DirectAccess without base value.", original_expression.get_location()));
                    };
                    self.generate_direct_access_expression(base, &base_value, member, &data.access, &data.index)
                } else {
                    let member_name = member.get_flat_reference_name().unwrap_or("unknown");
                    self.create_llvm_pointer_value_for_reference(
                        base_value.map(|it| it.get_basic_value_enum().into_pointer_value()).as_ref(),
                        self.get_load_name(member).as_deref().unwrap_or(member_name),
                        original_expression,
                    )
                    .map(ExpressionValue::LValue)
                }
            }

            // expressions like: base[idx]
            (ReferenceAccess::Index(array_idx), Some(base)) => {
                if self.annotations.get_type_or_void(base, self.index).is_vla() {
                    // vla array needs special handling
                    self.generate_element_pointer_for_vla(
                        self.generate_expression_value(base)?,
                        self.annotations.get(base).expect(""),
                        array_idx.as_ref(),
                    )
                    .map_err(|_| unreachable!("invalid access statement"))
                    .map(ExpressionValue::LValue)
                } else {
                    // normal array expression
                    self.generate_element_pointer_for_array(base, array_idx).map(ExpressionValue::LValue)
                }
            }

            // INT#target (INT = base)
            (ReferenceAccess::Cast(target), Some(_base)) => {
                if target.as_ref().is_identifier() {
                    let mr =
                        AstFactory::create_member_reference(target.as_ref().clone(), None, target.get_id());
                    self.generate_expression_value(&mr)
                } else {
                    self.generate_expression_value(target.as_ref())
                }
            }

            // base^
            (ReferenceAccess::Deref, Some(base)) => {
                let ptr = self.generate_expression_value(base)?;
                Ok(ExpressionValue::LValue(
                    self.llvm
                        .load_pointer(&ptr.get_basic_value_enum().into_pointer_value(), "deref")
                        .into_pointer_value(),
                ))
            }

            // &base
            (ReferenceAccess::Address, Some(base)) => {
                let lvalue = self.generate_expression_value(base)?;
                Ok(ExpressionValue::RValue(lvalue.get_basic_value_enum()))
            }

            (ReferenceAccess::Index(_), None) // [idx];
            | (ReferenceAccess::Cast(_), None) // INT#;
            | (ReferenceAccess::Deref, None)  // ^;
            | (ReferenceAccess::Address, None) // &;
                => Err(Diagnostic::codegen_error(
                "Expected a base-expressions, but found none.",
                original_expression.get_location(),
            )),
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
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
        let loaded_base_value = qualifier_value.as_r_value(self.llvm, self.get_load_name(qualifier));
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
        );
        //Trunc the result to the get only the target size
        let value = self.llvm.builder.build_int_truncate_or_bit_cast(
            shift,
            self.llvm_index.get_associated_type(datatype.get_name())?.into_int_type(),
            "",
        );

        let result = if datatype.get_type_information().is_bool() {
            // since booleans are i1 internally, but i8 in llvm, we need to bitwise-AND the value with 1 to make sure we end up with the expected value
            self.llvm.builder.build_and(value, self.llvm.context.i8_type().const_int(1, false), "")
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
    param_statement: &'a AstNode,
    declared_parameters: &[&VariableIndexEntry],
    idx: usize,
) -> Result<(usize, &'a AstNode, bool), Diagnostic> {
    let (location, param_statement, is_implicit) = match param_statement.get_stmt() {
        AstStatement::Assignment(data) | AstStatement::OutputAssignment(data) => {
            //explicit
            let Some(left_name) = data.left.as_ref().get_flat_reference_name() else {
                return Err(
                    //TODO: use global context to get an expression slice
                    Diagnostic::new("Expression is not assignable")
                        .with_error_code("E050")
                        .with_location(param_statement.get_location()),
                );
            };
            let loc = declared_parameters
                .iter()
                .position(|p| p.get_name().eq_ignore_ascii_case(left_name))
                .ok_or_else(|| Diagnostic::unresolved_reference(left_name, data.left.get_location()))?;
            (loc, data.right.as_ref(), false)
        }
        _ => {
            //implicit
            (idx, param_statement, true)
        }
    };
    Ok((location, param_statement, is_implicit))
}

/// turns the given IntValue into an i1 by comparing it to 0 (of the same size)
pub fn to_i1<'a>(value: IntValue<'a>, builder: &Builder<'a>) -> IntValue<'a> {
    if value.get_type().get_bit_width() > 1 {
        builder.build_int_compare(IntPredicate::NE, value, value.get_type().const_int(0, false), "")
    } else {
        value
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
) -> Vec<(IntValue<'ink>, IntValue<'ink>)> {
    (0..ndims)
        .map(|i| unsafe {
            let (start_ptr, end_ptr) = (
                llvm.builder.build_in_bounds_gep(
                    dimensions_array,
                    &[llvm.i32_type().const_zero(), llvm.i32_type().const_int(i as u64 * 2, false)],
                    format!("start_idx_ptr{i}").as_str(),
                ),
                llvm.builder.build_in_bounds_gep(
                    dimensions_array,
                    &[llvm.i32_type().const_zero(), llvm.i32_type().const_int(1 + i as u64 * 2, false)],
                    format!("end_idx_ptr{i}").as_str(),
                ),
            );
            (
                llvm.builder.build_load(start_ptr, format!("start_idx_value{i}").as_str()).into_int_value(),
                llvm.builder.build_load(end_ptr, format!("end_idx_value{i}").as_str()).into_int_value(),
            )
        })
        .collect::<Vec<_>>()
}

/// Adjusts VLA accessor values to 0-indexed accessors
fn normalize_offsets<'ink>(
    llvm: &Llvm<'ink>,
    accessors: &[IntValue<'ink>],
    offsets: &[(IntValue<'ink>, IntValue<'ink>)],
) -> Vec<IntValue<'ink>> {
    accessors
        .iter()
        .enumerate()
        .zip(offsets.iter().map(|(start, _)| start))
        .map(|((idx, accessor), start_offset)| {
            llvm.builder.build_int_sub(*accessor, *start_offset, format!("adj_access{idx}").as_str())
        })
        .collect::<Vec<_>>()
}

fn get_dimension_lengths<'ink>(
    llvm: &Llvm<'ink>,
    offsets: &[(IntValue<'ink>, IntValue<'ink>)],
) -> Vec<IntValue<'ink>> {
    offsets
        .iter()
        .enumerate()
        .map(|(idx, (start, end))| {
            llvm.builder.build_int_add(
                llvm.i32_type().const_int(1, false),
                llvm.builder.build_int_sub(*end, *start, ""),
                format!("len_dim{idx}").as_str(),
            )
        })
        .collect::<Vec<_>>()
}

fn get_vla_accessor_factors<'ink>(llvm: &Llvm<'ink>, lengths: &[IntValue<'ink>]) -> Vec<IntValue<'ink>> {
    (0..lengths.len())
        .map(|idx| {
            if idx == lengths.len() - 1 {
                // the last dimension has a factor of 1
                llvm.i32_type().const_int(1, false)
            } else {
                // for other dimensions, calculate size to the right
                int_value_product(llvm, &lengths[idx + 1..lengths.len()])
            }
        })
        .collect::<Vec<_>>()
}

/// Computes the product of all elements in a collection of IntValues
///
/// a <- a * b
fn int_value_product<'ink>(llvm: &Llvm<'ink>, values: &[IntValue<'ink>]) -> IntValue<'ink> {
    // initialize the accumulator with 1
    let accum_ptr = llvm.builder.build_alloca(llvm.i32_type(), "accum");
    llvm.builder.build_store(accum_ptr, llvm.i32_type().const_int(1, false));
    for val in values {
        // load previous value from accumulator and multiply with current value
        let product = llvm.builder.build_int_mul(
            llvm.builder.build_load(accum_ptr, "load_accum").into_int_value(),
            *val,
            "product",
        );
        // store new value into accumulator
        llvm.builder.build_store(accum_ptr, product);
    }

    llvm.builder.build_load(accum_ptr, "accessor_factor").into_int_value()
}

/// Iterates over a collection of tuples, computes the product of the two numbers
/// and adds that product to an accumulator.
///
/// a <- a + (b * c)
fn int_value_multiply_accumulate<'ink>(
    llvm: &Llvm<'ink>,
    values: &[(&IntValue<'ink>, &IntValue<'ink>)],
) -> IntValue<'ink> {
    // initialize the accumulator with 0
    let accum = llvm.builder.build_alloca(llvm.i32_type(), "accum");
    llvm.builder.build_store(accum, llvm.i32_type().const_zero());
    for (left, right) in values {
        // multiply accessor with dimension factor
        let product = llvm.builder.build_int_mul(**left, **right, "multiply");
        // load previous value from accum and add product
        let curr = llvm.builder.build_int_add(
            llvm.builder.build_load(accum, "load_accum").into_int_value(),
            product,
            "accumulate",
        );
        // store new value into accumulator
        llvm.builder.build_store(accum, curr);
    }
    llvm.builder.build_load(accum, "accessor").into_int_value()
}
