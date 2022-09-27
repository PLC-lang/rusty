// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    ast::{self, DirectAccessType, SourceRange},
    codegen::llvm_typesystem,
    diagnostics::{Diagnostic, INTERNAL_LLVM_ERROR},
    index::{
        const_expressions::ConstId, ArgumentType, ImplementationIndexEntry, Index, PouIndexEntry,
        VariableIndexEntry,
    },
    resolver::{AnnotationMap, AstAnnotations, StatementAnnotation},
    typesystem::{
        is_same_type_class, Dimension, StringEncoding, VarArgs, DINT_TYPE, INT_SIZE, INT_TYPE,
        LINT_TYPE,
    },
};
use inkwell::{
    builder::Builder,
    types::{BasicType, BasicTypeEnum},
    values::{
        ArrayValue, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FloatValue, GlobalValue,
        IntValue, PointerValue, StructValue, VectorValue,
    },
    AddressSpace, FloatPredicate, IntPredicate,
};
use std::collections::HashSet;

use crate::{
    ast::{flatten_expression_list, AstStatement, Operator},
    codegen::{
        llvm_index::LlvmTypedIndex,
        llvm_typesystem::{cast_if_needed, get_llvm_int_type},
    },
    typesystem::{DataType, DataTypeInformation},
};

use super::{llvm::Llvm, statement_generator::FunctionContext};

/// the generator for expressions
pub struct ExpressionCodeGenerator<'a, 'b> {
    pub llvm: &'b Llvm<'a>,
    pub index: &'b Index,
    pub(crate) annotations: &'b AstAnnotations,
    pub llvm_index: &'b LlvmTypedIndex<'a>,
    /// the current function to create blocks in
    pub function_context: Option<&'b FunctionContext<'a>>,

    /// the string-prefix to use for temporary variables
    pub temp_variable_prefix: String,
    /// the string-suffix to use for temporary variables
    pub temp_variable_suffix: String,

    // the function on how to obtain the the length to use for the string
    string_len_provider: fn(type_length_declaration: usize, actual_length: usize) -> usize,
}

/// context information to generate a parameter
struct CallParameterAssignment<'a, 'b> {
    /// the assignmentstatement in the call-argument list (a:=3)
    assignment_statement: &'b AstStatement,
    /// the name of the function we're calling
    function_name: &'b str,
    /// the position of the argument in the POU's argument's list
    index: u32,
    /// a pointer to the struct instance that carries the call's arguments
    parameter_struct: PointerValue<'a>,
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
        function_context: &'b FunctionContext<'ink>,
    ) -> ExpressionCodeGenerator<'ink, 'b> {
        ExpressionCodeGenerator {
            llvm,
            index,
            llvm_index,
            annotations,
            function_context: Some(function_context),
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
            temp_variable_prefix: "load_".to_string(),
            temp_variable_suffix: "".to_string(),
            string_len_provider: |type_length_declaration, _| type_length_declaration, //when generating string-literals in declarations, use the declared length
        }
    }

    /// returns the function context or returns a Compile-Error
    pub fn get_function_context(
        &self,
        statement: &AstStatement,
    ) -> Result<&'b FunctionContext<'ink>, Diagnostic> {
        self.function_context
            .ok_or_else(|| Diagnostic::missing_function(statement.get_location()))
    }

    /// entry point into the expression generator.
    /// generates the given expression and returns the resulting BasicValueEnum
    pub fn generate_expression(
        &self,
        expression: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let v = self.do_generate_expression(expression)?;

        //see if we need a cast
        if let Some(target_type) = self.annotations.get_type_hint(expression, self.index) {
            let actual_type = self.annotations.get_type_or_void(expression, self.index);
            Ok(llvm_typesystem::cast_if_needed(
                self.llvm,
                self.index,
                self.llvm_index,
                target_type,
                v,
                actual_type,
                expression,
            )?)
        } else {
            Ok(v)
        }
    }

    fn do_generate_expression(
        &self,
        expression: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        //see if this is a constant - maybe we can short curcuit this codegen
        if let Some(StatementAnnotation::Variable { qualified_name, .. }) =
            self.annotations.get(expression)
        {
            if let Some(basic_value_enum) = self.llvm_index.find_constant_value(qualified_name) {
                //this is a constant and we have a value for it
                return Ok(basic_value_enum);
            }
        }

        // generate the expression
        match expression {
            AstStatement::Reference { name, .. } => {
                let load_name = format!(
                    "{}{}{}",
                    self.temp_variable_prefix, name, self.temp_variable_suffix
                );
                if let Some(StatementAnnotation::Variable {
                    qualified_name,
                    constant: true,
                    ..
                }) = self.annotations.get(expression)
                {
                    // constant propagation
                    self.generate_constant_expression(qualified_name, expression)
                } else {
                    // general reference generation
                    let l_value = self.generate_element_pointer(expression)?;
                    Ok(self.llvm.load_pointer(&l_value, load_name.as_str()))
                }
            }
            AstStatement::QualifiedReference { elements, .. } => {
                //If direct access, don't load pointers
                if expression.has_direct_access() {
                    //Split the qualified reference at the last element
                    self.generate_directaccess(elements)
                } else {
                    let l_value = self.generate_element_pointer(expression)?;
                    Ok(self.llvm.load_pointer(&l_value, &self.temp_variable_prefix))
                }
            }
            AstStatement::ArrayAccess { .. } => {
                let l_value = self.generate_element_pointer(expression)?;
                Ok(self.llvm.load_pointer(&l_value, "load_tmpVar"))
            }
            AstStatement::PointerAccess { .. } => {
                let l_value = self.generate_element_pointer(expression)?;
                Ok(self.llvm.load_pointer(&l_value, "load_tmpVar"))
            }
            AstStatement::BinaryExpression {
                left,
                right,
                operator,
                ..
            } => self.generate_binary_expression(left, right, operator, expression),
            AstStatement::CallStatement {
                operator,
                parameters,
                ..
            } => self.generate_call_statement(operator, parameters),
            AstStatement::UnaryExpression {
                operator, value, ..
            } => self.generate_unary_expression(operator, value),
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
        expression: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let const_expression = self
            .index
            // try to find a constant variable
            .find_variable(None, &qualified_name.split('.').collect::<Vec<_>>())
            // or else try to find an enum element
            .or_else(|| self.index.find_qualified_enum_element(qualified_name))
            // if this is no constant we have a problem
            .filter(|v| v.is_constant())
            .and_then(|v| v.initial_value)
            // fetch the constant's initial value fron the const-expressions arena
            .and_then(|constant_variable| {
                self.index
                    .get_const_expressions()
                    .get_resolved_constant_statement(&constant_variable)
            })
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    format!("Cannot propagate constant value for '{:}'", qualified_name).as_str(),
                    expression.get_location(),
                )
            })?;

        //  generate the resulting constant-expression (which should be a Value, no ptr-reference)
        self.generate_expression(const_expression)
    }

    /// generates a binary expression (e.g. a + b, x AND y, etc.) and returns the resulting `BasicValueEnum`
    /// - `left` the AstStatement left of the operator
    /// - `right` the AstStatement right of the operator
    /// - `operator` the binary expression's operator
    /// - `expression` the whole expression for diagnostic reasons
    fn generate_binary_expression(
        &self,
        left: &AstStatement,
        right: &AstStatement,
        operator: &Operator,
        expression: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let l_type_hint = self.get_type_hint_for(left)?;
        let ltype = self
            .index
            .get_intrinsic_type_by_name(l_type_hint.get_name())
            .get_type_information();
        let r_type_hint = self.get_type_hint_for(right)?;
        let rtype = self
            .index
            .get_intrinsic_type_by_name(r_type_hint.get_name())
            .get_type_information();
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
            self.create_llvm_binary_expression_for_pointer(
                operator, left, ltype, right, rtype, expression,
            )
        } else {
            self.create_llvm_generic_binary_expression(operator, left, right, expression)
        }
    }

    fn generate_directaccess(
        &self,
        elements: &[AstStatement],
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let (expression, last) = match elements {
            [qualifier, last] => {
                // a.%w1
                (qualifier.clone(), last)
            }
            [qualifier @ .., last_qualifier, last] => {
                // a.b.c.%w1
                let id = last_qualifier.get_id();
                (
                    AstStatement::QualifiedReference {
                        elements: [qualifier, &[last_qualifier.clone()]].concat().to_vec(),
                        id,
                    },
                    last,
                )
            }
            _ => {
                return Err(Diagnostic::codegen_error(
                    &format!("Invalid direct-access: {:?}", elements),
                    SourceRange::undefined(),
                ));
            }
        };

        //Generate a load for the qualifer
        // a.%b1.%x1
        let value = self.generate_expression(&expression)?;
        let expression_type = self.get_type_hint_for(&expression)?;
        if let AstStatement::DirectAccess { access, index, .. } = last {
            //Generate and load the index value
            let datatype = self.get_type_hint_info_for(last)?;
            let rhs =
                self.generate_direct_access_index(access, index, datatype, expression_type)?;
            //Shift the qualifer value right by the index value
            let shift = self.llvm.builder.build_right_shift(
                value.into_int_value(),
                rhs,
                expression_type.get_type_information().is_signed_int(),
                "shift",
            );
            //Trunc the result to the get only the target size
            let llvm_target_type = self
                .llvm_index
                .get_associated_type(datatype.get_name())?
                .into_int_type();
            let result =
                self.llvm
                    .builder
                    .build_int_truncate_or_bit_cast(shift, llvm_target_type, "");
            Ok(result.as_basic_value_enum())
        } else {
            unreachable!()
        }
    }

    pub fn generate_direct_access_index(
        &self,
        access: &DirectAccessType,
        index: &AstStatement,
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
            let reference = cast_if_needed(
                self.llvm,
                self.index,
                self.llvm_index,
                target_type,
                reference,
                self.get_type_hint_for(index)?,
                index,
            )
            .map(BasicValueEnum::into_int_value)?;
            // let reference = reference.into_int_value();
            //Multiply by the bitwitdh
            if access.get_bit_width() > 1 {
                let bitwidth = reference
                    .get_type()
                    .const_int(access.get_bit_width(), access_type.is_signed_int());

                Ok(self.llvm.builder.build_int_mul(reference, bitwidth, ""))
            } else {
                Ok(reference)
            }
        } else {
            Err(Diagnostic::casting_error(
                access_type.get_name(),
                "Integer Type",
                index.get_location(),
            ))
        }
    }

    /// generates a Unary-Expression e.g. -<expr> or !<expr>
    fn generate_unary_expression(
        &self,
        unary_operator: &Operator,
        expression: &AstStatement,
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

                Ok(self
                    .llvm
                    .builder
                    .build_not(operator, "tmpVar")
                    .as_basic_value_enum())
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
            Operator::Address => {
                //datatype is a pointer to the address
                //value is the address
                self.generate_element_pointer(expression)
                    .map(|result| result.as_basic_value_enum())
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
        operator: &AstStatement,
        parameters: &Option<AstStatement>,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let function_context = self.get_function_context(operator)?;

        //find the pou we're calling
        let pou = self.annotations.get_call_name(operator).zip(self.annotations.get_qualified_name(operator))
            .and_then(|(call_name, qualified_name)| self.index.find_pou(call_name)
            //For some functions (builtins) the call name does not exist in the index, we try to call with the originally defined generic functions
            .or_else(|| self.index.find_pou(qualified_name)))
            .or_else(||
                // some rare situations have a callstatement that's not properly annotated (e.g. checkRange-call of ranged datatypes)
                if let AstStatement::Reference { name, .. } = operator {
                    self.index.find_pou(name)
                } else {
                    None
                })
            .ok_or_else(|| Diagnostic::cannot_generate_call_statement(operator))?;

        // find corresponding implementation
        let implementation = pou
            .find_implementation(self.index)
            .ok_or_else(|| Diagnostic::cannot_generate_call_statement(operator))?;

        //If the function is builtin, generate a basic value enum for it
        if let Some(builtin) = self
            .index
            .get_builtin_function(implementation.get_call_name())
        {
            //adr, ref, etc.
            return builtin.codegen(
                self,
                parameters
                    .as_ref()
                    .map(ast::flatten_expression_list)
                    .unwrap_or_default()
                    .as_slice(),
                operator.get_location(),
            );
        }

        let function_name = implementation.get_call_name();
        let arguments_list = self.generate_pou_call_arguments_list(
            pou,
            parameters,
            implementation,
            operator,
            function_context,
            function_name,
        )?;

        let builder = &self.llvm.builder;
        let function = self
            .llvm_index
            .find_associated_implementation(function_name) //using the non error option to control the output error
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    &format!(
                        "No callable implementation associated to {:?}",
                        function_name
                    ),
                    operator.get_location(),
                )
            })?;
        //If the target is a function, declare the struct locally
        //Assign all parameters into the struct values
        let call_result = builder
            .build_call(function, &arguments_list, "call")
            .try_as_basic_value();

        // we return an uninitialized int pointer for void methods :-/
        // dont deref it!!
        let value = call_result.either(Ok, |_| {
            get_llvm_int_type(self.llvm.context, INT_SIZE, INT_TYPE).map(|int| {
                int.ptr_type(AddressSpace::Const)
                    .const_null()
                    .as_basic_value_enum()
            })
        })?;

        Ok(value)
    }

    /// generates the argument list for a call to a pou
    /// a call to a function returns a Vec with all parameters for the function,
    /// a call to a Program/Fb will return a Vec with a single struct carrying all parameters
    fn generate_pou_call_arguments_list(
        &self,
        pou: &PouIndexEntry,
        parameters: &Option<AstStatement>,
        implementation: &ImplementationIndexEntry,
        operator: &AstStatement,
        function_context: &'b FunctionContext<'ink>,
        function_name: &str,
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, Diagnostic> {
        let arguments_list = if matches!(pou, PouIndexEntry::Function { .. }) {
            //we're calling a function

            let call_params = parameters
                .as_ref()
                .map(ast::flatten_expression_list)
                .unwrap_or_default();

            // foo(a,b,c)
            // foo(z:= a, x:=c, y := b);
            let declared_parameters = self
                .index
                .get_declared_parameters(implementation.get_type_name());

            // the parameters to be passed to the function call
            self.generate_function_arguments(pou, call_params, declared_parameters)?
        } else {
            // no function
            let (class_ptr, call_ptr) = match pou {
                PouIndexEntry::Method { .. } => {
                    let class_ptr = self.generate_element_pointer(operator)?;
                    let call_ptr = self.allocate_function_struct_instance(
                        implementation.get_call_name(),
                        operator,
                    )?;
                    (Some(class_ptr), call_ptr)
                }
                PouIndexEntry::Action { .. }
                    if matches!(operator, AstStatement::Reference { .. }) =>
                {
                    //Special handling for local actions, get the parameter from the function context
                    function_context
                        .function
                        .get_first_param()
                        .map(|call_ptr| (None, call_ptr.into_pointer_value()))
                        .ok_or_else(|| Diagnostic::cannot_generate_call_statement(operator))?
                }
                _ => {
                    let class_ptr = self.generate_element_pointer(operator)?;
                    (None, class_ptr)
                }
            };

            //First go to the input block
            //Generate all parameters, this function may jump to the output block
            self.generate_stateful_pou_call_parameters(
                function_name,
                class_ptr,
                call_ptr,
                parameters,
            )?
        };
        Ok(arguments_list)
    }

    fn generate_function_arguments(
        &self,
        pou: &PouIndexEntry,
        arguments: Vec<&AstStatement>,
        declared_parameters: Vec<&VariableIndexEntry>,
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, Diagnostic> {
        let mut result = Vec::new();
        let mut variadic_params = Vec::new();

        let mut passed_args = Vec::new();
        for (idx, param_statement) in arguments.into_iter().enumerate() {
            let (location, param_statement) =
                get_implicit_call_parameter(param_statement, &declared_parameters, idx)?;

            //None -> possibly variadic
            let param = declared_parameters
                //get paremeter at location
                .get(location)
                //find the parameter's type and name
                .map(|it| {
                    let name = it.get_type_name();
                    if let Some(DataTypeInformation::Pointer {
                        inner_type_name,
                        auto_deref: true,
                        ..
                    }) = self.index.find_effective_type_info(name)
                    {
                        Some((it.get_declaration_type(), inner_type_name.as_str()))
                    } else {
                        Some((it.get_declaration_type(), name))
                    }
                })
                //TODO : Is this idomatic, we need to wrap in ok because the next step does not necessarily fail
                .map(Ok)
                .unwrap_or_else(|| {
                    //If we are dealing with a variadic function, we can accept all extra parameters
                    if pou.is_variadic() {
                        variadic_params.push(param_statement);
                        Ok(None)
                    } else {
                        //We are not variadic, we have too many parameters here
                        Err(Diagnostic::codegen_error(
                            "Too many parameters",
                            param_statement.get_location(),
                        ))
                    }
                })?;

            if let Some((declaration_type, type_name)) = param {
                let argument: BasicValueEnum = if declaration_type.is_by_ref() {
                    let declared_parameter = declared_parameters.get(location);
                    self.generate_argument_by_ref(
                        param_statement,
                        type_name,
                        declared_parameter.copied(),
                    )?
                } else {
                    //pass by val
                    self.generate_argument_by_val(type_name, param_statement)?
                };
                result.push((location, argument));
            }

            passed_args.push(location);
        }

        // handle missing declared parameters
        if declared_parameters.len() > passed_args.len() {
            for (i, param) in declared_parameters.into_iter().enumerate() {
                if !passed_args.contains(&i) {
                    let generated_exp = self.generate_empty_expression(param)?;

                    result.push((i, generated_exp));
                }
            }
        }

        //Push variadic collection and optionally the variadic size
        if pou.is_variadic() {
            let last_location = result.len();
            let variadic_params = self.generate_variadic_arguments_list(pou, &variadic_params)?;
            for (i, param) in variadic_params.into_iter().enumerate() {
                result.push((i + last_location, param));
            }
        }
        result.sort_by(|(idx_a, _), (idx_b, _)| idx_a.cmp(idx_b));
        Ok(result
            .into_iter()
            .map(|(_, v)| v.into())
            .collect::<Vec<BasicMetadataValueEnum>>())
    }

    fn generate_argument_by_val(
        &self,
        type_name: &str,
        param_statement: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        Ok(match self.index.find_effective_type_by_name(type_name) {
            Some(type_info) if type_info.information.is_string() => {
                self.generate_string_argument(type_info, param_statement)?
            }
            _ => self.generate_expression(param_statement)?,
        })
    }

    /// Before passing a string to a function, it is copied to a new string with the
    /// appropriate size for the called function
    fn generate_string_argument(
        &self,
        type_info: &DataType,
        argument: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        // allocate a temporary string of correct size and pass it
        let llvm_type = self
            .llvm_index
            .find_associated_type(type_info.get_name())
            .ok_or_else(|| {
                Diagnostic::unknown_type(type_info.get_name(), argument.get_location())
            })?;
        let temp_variable = self.llvm.builder.build_alloca(llvm_type, "");
        self.llvm
            .builder
            .build_memset(
                temp_variable,
                1,
                self.llvm.context.i8_type().const_zero(),
                llvm_type.size_of().ok_or_else(|| {
                    Diagnostic::unknown_type(type_info.get_name(), argument.get_location())
                })?,
            )
            .map_err(|it| Diagnostic::codegen_error(it, argument.get_location()))?;
        self.generate_store(type_info.get_type_information(), argument, temp_variable)?;
        Ok(self.llvm.builder.build_load(temp_variable, ""))
    }

    /// generates a value that is passed by reference
    /// this generates and returns a PointerValue
    /// pointing to the given `argument`
    fn generate_argument_by_ref(
        &self,
        argument: &AstStatement,
        type_name: &str,
        declared_parameter: Option<&VariableIndexEntry>,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        if matches!(argument, AstStatement::EmptyStatement { .. }) {
            //uninitialized var_output/var_in_out
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
            Ok(ptr_value)
        } else {
            self.generate_element_pointer(argument)
                .or_else::<Diagnostic, _>(|_| {
                    //passed a literal to byref parameter?
                    //TODO: find more defensive solution - check early
                    let value = self.generate_expression(argument)?;
                    let argument = self.llvm.builder.build_alloca(value.get_type(), "");
                    self.llvm.builder.build_store(argument, value);
                    Ok(argument)
                })
        }
        .map(Into::into)
    }

    pub fn generate_variadic_arguments_list(
        &self,
        pou: &PouIndexEntry,
        variadic_params: &[&AstStatement],
    ) -> Result<Vec<BasicValueEnum<'ink>>, Diagnostic> {
        //get the real varargs from the index
        if let Some((var_args, argument_type)) = self
            .index
            .get_variadic_member(pou.get_name())
            .and_then(|it| it.get_varargs().zip(Some(it.get_declaration_type())))
        {
            let generated_params = variadic_params
                .iter()
                .map(|param_statement| {
                    self.get_type_hint_for(param_statement)
                        .map(|it| it.get_name())
                        .and_then(|type_name| {
                            // If the variadic is defined in a by_ref block, we need to pass the argument as reference
                            if let ArgumentType::ByVal(_) = argument_type {
                                self.generate_argument_by_val(type_name, param_statement)
                            } else {
                                self.generate_argument_by_ref(
                                    param_statement,
                                    type_name,
                                    self.index.get_variadic_member(pou.get_name()),
                                )
                            }
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            // For sized variadics we create an array and store all the arguments in that array
            if let VarArgs::Sized(Some(type_name)) = var_args {
                let llvm_type = self.llvm_index.get_associated_type(type_name)?;
                // If the variadic argument is ByRef, wrap it in a pointer.
                let llvm_type = if matches!(argument_type, ArgumentType::ByRef(_)) {
                    llvm_type.ptr_type(AddressSpace::Generic).into()
                } else {
                    llvm_type
                };
                let size = generated_params.len();
                let size_param = self.llvm.i32_type().const_int(size as u64, true);
                let arr = Llvm::get_array_type(llvm_type, size as u32);
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
                Ok(vec![size_param.into(), arr_storage.into()])
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
        context: &AstStatement,
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        let instance_name = format!("{}_instance", function_name);
        let function_type = self
            .llvm_index
            .find_associated_pou_type(function_name) //Using find instead of get to control the compile error
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    &format!("No type associated with '{:}'", instance_name),
                    context.get_location(),
                )
            })?;

        Ok(self
            .llvm
            .create_local_variable(&instance_name, &function_type))
    }

    /// generates the assignments of a function-call's parameters
    /// the call parameters are passed to the function using a struct-instance with all the parameters
    ///
    /// - `function_name` the name of the function we're calling
    /// - `parameter_struct` a pointer to a struct-instance that holds all function-parameters
    /// - `input_block` the block to generate the input-assignments into
    /// - `output_block` the block to generate the output-assignments into
    fn generate_stateful_pou_call_parameters(
        &self,
        function_name: &str,
        class_struct: Option<PointerValue<'ink>>,
        parameter_struct: PointerValue<'ink>,
        parameters: &Option<AstStatement>,
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, Diagnostic> {
        let mut result = class_struct
            .map(|class_struct| {
                vec![
                    class_struct.as_basic_value_enum().into(),
                    parameter_struct.as_basic_value_enum().into(),
                ]
            })
            .unwrap_or_else(|| vec![parameter_struct.as_basic_value_enum().into()]);

        let expressions = parameters
            .as_ref()
            .map(ast::flatten_expression_list)
            .unwrap_or_else(std::vec::Vec::new);

        for (index, exp) in expressions.iter().enumerate() {
            let parameter =
                self.generate_call_struct_argument_assignment(&CallParameterAssignment {
                    assignment_statement: exp,
                    function_name,
                    index: index as u32,
                    parameter_struct,
                })?;
            if let Some(parameter) = parameter {
                result.push(parameter.into());
            };
        }

        Ok(result)
    }

    fn get_parameter_type(&self, parameter: &VariableIndexEntry) -> String {
        if let Some(DataTypeInformation::Pointer {
            inner_type_name,
            auto_deref: true,
            ..
        }) = self
            .index
            .find_effective_type_info(parameter.get_type_name())
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
    fn get_initial_value(&self, id: &Option<ConstId>, type_name: &str) -> Option<&AstStatement> {
        self.index
            .get_initial_value(id)
            .or_else(|| self.index.get_initial_value_for_type(type_name))
    }

    /// generates an assignemnt of a single call's argument
    ///
    /// - `assignment_statement' the parameter-assignment, either an AssignmentStatement, an OutputAssignmentStatement or an expression
    /// - `function_name` the name of the callable
    /// - `parameter_type` the datatype of the parameter
    /// - `index` the index of the parameter (0 for first parameter, 1 for the next one, etc.)
    /// - `parameter_struct' a pointer to a struct-instance that holds all function-parameters
    /// - `input_block` the block to generate the input-assignments into
    /// - `output_block` the block to generate the output-assignments into
    fn generate_call_struct_argument_assignment(
        &self,
        param_context: &CallParameterAssignment,
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        let assignment_statement = param_context.assignment_statement;

        let parameter_value = match assignment_statement {
            // explicit call parameter: foo(param := value)
            AstStatement::OutputAssignment { left, right, .. }
            | AstStatement::Assignment { left, right, .. } => {
                self.generate_formal_parameter(param_context, left, right)?;
                None
            }
            // foo(x)
            _ => self.generate_nameless_parameter(param_context, assignment_statement)?,
        };

        Ok(parameter_value)
    }

    /// generates the appropriate value for the given expression where the expression
    /// is a call's implicit argument (hence: foo(3), not foo(in := 3))
    fn generate_nameless_parameter(
        &self,
        param_context: &CallParameterAssignment,
        expression: &AstStatement,
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        let builder = &self.llvm.builder;
        let function_name = param_context.function_name;
        let index = param_context.index;
        let parameter_struct = param_context.parameter_struct;
        if self.index.is_declared_parameter(function_name, index) {
            let pointer_to_param = builder
                .build_struct_gep(parameter_struct, index as u32, "")
                .map_err(|_| {
                    Diagnostic::codegen_error(
                        &format!("Cannot build generate parameter: {:#?}", expression),
                        expression.get_location(),
                    )
                })?;

            let parameter = self
                .index
                .find_parameter(function_name, index as u32)
                .and_then(|var| self.index.find_effective_type_by_name(var.get_type_name()))
                .map(|var| var.get_type_information())
                .unwrap_or_else(|| self.index.get_void_type().get_type_information());

            if let DataTypeInformation::Pointer {
                auto_deref: true,
                inner_type_name,
                ..
            } = parameter
            {
                //this is VAR_OUT or VAR_IN_OUT assignemt, so don't load the value, assign the pointer

                //expression may be empty -> generate a local variable for it
                let generated_exp = if matches!(expression, AstStatement::EmptyStatement { .. }) {
                    let temp_type = self
                        .llvm_index
                        .find_associated_type(inner_type_name)
                        .ok_or_else(|| {
                            Diagnostic::unknown_type(
                                parameter.get_name(),
                                expression.get_location(),
                            )
                        })?;
                    builder
                        .build_alloca(temp_type, "empty_varinout")
                        .as_basic_value_enum()
                } else {
                    self.generate_element_pointer(expression)?
                        .as_basic_value_enum()
                };
                builder.build_store(pointer_to_param, generated_exp);
            } else {
                self.generate_store(parameter, expression, pointer_to_param)?;
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
        left: &AstStatement,
        right: &AstStatement,
    ) -> Result<(), Diagnostic> {
        let function_name = param_context.function_name;
        let parameter_struct = param_context.parameter_struct;
        if let AstStatement::Reference { name, .. } = left {
            let parameter = self
                .index
                .find_member(function_name, name)
                .ok_or_else(|| Diagnostic::unresolved_reference(name, left.get_location()))?;
            let index = parameter.get_location_in_parent();
            self.generate_call_struct_argument_assignment(&CallParameterAssignment {
                assignment_statement: right,
                function_name,
                index,
                parameter_struct,
            })?;
        };
        Ok(())
    }

    /// generates an gep-statement and returns the resulting pointer and DataTypeInfo
    ///
    /// - `reference_statement` - the statement to load (either a reference, an arrayAccess or a qualifiedReference)
    pub fn generate_element_pointer(
        &self,
        reference_statement: &AstStatement,
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        if let AstStatement::QualifiedReference { elements, .. } = reference_statement {
            self.generate_element_pointer_from_elements(
                elements,
                reference_statement.get_location(),
            )
        } else {
            self.do_generate_element_pointer(None, reference_statement)
        }
    }

    pub fn generate_element_pointer_from_elements(
        &self,
        elements: &[AstStatement],
        location: SourceRange,
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        let mut qualifier: Option<PointerValue> = None;
        for e in elements {
            qualifier = Some(self.do_generate_element_pointer(qualifier, e)?);
        }
        qualifier.ok_or_else(|| {
            Diagnostic::codegen_error(
                &format!("Cannot generate a LValue for {:?}", elements),
                location,
            )
        })
    }

    fn do_generate_element_pointer(
        &self,
        qualifier: Option<PointerValue<'ink>>,
        reference_statement: &AstStatement,
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        let result = match reference_statement {
            AstStatement::Reference { name, .. } => self.create_llvm_pointer_value_for_reference(
                qualifier.as_ref(),
                name.as_str(),
                reference_statement,
            ),
            AstStatement::ArrayAccess {
                reference, access, ..
            } => self.generate_element_pointer_for_array(qualifier.as_ref(), reference, access),
            AstStatement::PointerAccess { reference, .. } => self
                .do_generate_element_pointer(qualifier, reference)
                .map(|it| self.deref(it)),
            AstStatement::LiteralString { value, is_wide, .. } => if *is_wide {
                self.llvm_index.find_utf16_literal_string(value)
            } else {
                self.llvm_index.find_utf08_literal_string(value)
            }
            .map(|it| it.as_pointer_value())
            .ok_or_else(|| unreachable!("All string literals have to be constants")),
            _ => Err(Diagnostic::codegen_error(
                &format!("Cannot generate a LValue for {:?}", reference_statement),
                reference_statement.get_location(),
            )),
        };

        result.and_then(|it| self.auto_deref_if_necessary(it, reference_statement))
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
        context: &AstStatement,
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
                        .ok_or_else(|| {
                            Diagnostic::unresolved_reference(qualified_name, offset.clone())
                        })?;
                    let gep = self.llvm.get_member_pointer_from_struct(
                        *qualifier,
                        member_location,
                        name,
                        offset,
                    )?;

                    return Ok(gep);
                }
                _ => {
                    let qualifier_name = self.get_type_hint_for(context)?.get_name();
                    let qualified_name = format!("{}.{}", qualifier_name, name);
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
        self.llvm
            .load_pointer(&accessor_ptr, "deref")
            .into_pointer_value()
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
        statement: &AstStatement,
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        if let Some(StatementAnnotation::Variable {
            is_auto_deref: true,
            ..
        }) = self.annotations.get(statement)
        {
            Ok(self.deref(accessor_ptr))
        } else {
            Ok(accessor_ptr)
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
        access_expression: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let start_offset = dimension
            .start_offset
            .as_int_value(self.index)
            .map_err(|it| Diagnostic::codegen_error(&it, access_expression.get_location()))?;

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
        llvm_typesystem::cast_if_needed(
            self.llvm,
            self.index,
            self.llvm_index,
            self.index.get_type(DINT_TYPE)?,
            result.as_basic_value_enum(),
            self.get_type_hint_for(access_expression)?,
            access_expression,
        )
    }

    /// generates a gep statement for a array-reference with an optional qualifier
    ///
    /// - `qualifier` an optional qualifier for a reference (e.g. myStruct.x[2] where myStruct is the qualifier for x)
    /// - `reference` the reference-statement pointing to the array
    /// - `access` the accessor expression (the expression between the brackets: reference[access])
    fn generate_element_pointer_for_array(
        &self,
        qualifier: Option<&PointerValue<'ink>>,
        reference: &AstStatement,
        access: &AstStatement,
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        //Load the reference
        self.do_generate_element_pointer(qualifier.cloned(), reference)
            .and_then(|lvalue| {
                if let DataTypeInformation::Array { dimensions, .. } =
                    self.get_type_hint_info_for(reference)?
                {
                    //Make sure dimensions match statement list
                    let statements = access.get_as_list();
                    if statements.is_empty() || statements.len() != dimensions.len() {
                        return Err(Diagnostic::codegen_error(
                            "Invalid array access",
                            access.get_location(),
                        ));
                    }

                    // e.g. an array like `ARRAY[0..3, 0..2, 0..1] OF ...` has the lengths [ 4 , 3 , 2 ]
                    let lengths = dimensions
                        .iter()
                        .map(|d| d.get_length(self.index))
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|msg| {
                            Diagnostic::codegen_error(
                                format!("Invalid array dimensions access: {}", msg).as_str(),
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

                    //accessing [ 1, 2, 2] means to access [ 1*6 + 2*2 + 2*1 ] = 12
                    let (index_access, _) = accessors_and_portions.fold(
                        (
                            Ok(self.llvm.i32_type().const_zero().as_basic_value_enum()),
                            1,
                        ),
                        |(accumulated_value, _), (current_v, current_portion)| {
                            let result = accumulated_value.and_then(|last_v| {
                                current_v.map(|v| {
                                    let current_portion_value = self
                                        .llvm
                                        .i32_type()
                                        .const_int(current_portion as u64, false)
                                        .as_basic_value_enum();
                                    //multiply the accessor with the dimension's portion
                                    let m_v = self.create_llvm_int_binary_expression(
                                        &Operator::Multiplication,
                                        current_portion_value,
                                        v,
                                    );
                                    // take the sum of the mulitlication and the previous accumulated_value
                                    // this now becomes the new accumulated value
                                    self.create_llvm_int_binary_expression(
                                        &Operator::Plus,
                                        m_v,
                                        last_v,
                                    )
                                })
                            });
                            (result, 0 /* the 0 will be ignored */)
                        },
                    );

                    //make sure we got an int-value
                    let index_access: IntValue = index_access.and_then(|it| {
                        it.try_into().map_err(|_| {
                            Diagnostic::codegen_error(
                                "non-numeric index-access",
                                access.get_location(),
                            )
                        })
                    })?;

                    //Load the access from that array
                    //First 0 is to access the pointer, then we access the array
                    let pointer = self.llvm.load_array_element(
                        lvalue,
                        &[self.llvm.i32_type().const_zero(), index_access],
                        "tmpVar",
                    )?;

                    return Ok(pointer);
                }
                Err(Diagnostic::codegen_error(
                    "Invalid array access",
                    access.get_location(),
                ))
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
        left: &AstStatement,
        left_type: &DataTypeInformation,
        right: &AstStatement,
        right_type: &DataTypeInformation,
        expression: &AstStatement,
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

                    Ok(self
                        .llvm
                        .load_array_element(ptr, &[index], name.as_str())?
                        .as_basic_value_enum())
                } else {
                    Err(Diagnostic::codegen_error(
                        format!("'{}' operation must contain one int type", operator).as_str(),
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
                format!("Operator '{}' unimplemented for pointers", operator).as_str(),
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
            Operator::Plus => self
                .llvm
                .builder
                .build_int_add(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Minus => self
                .llvm
                .builder
                .build_int_sub(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Multiplication => self
                .llvm
                .builder
                .build_int_mul(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Division => self
                .llvm
                .builder
                .build_int_signed_div(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Modulo => self
                .llvm
                .builder
                .build_int_signed_rem(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Equal => self.llvm.builder.build_int_compare(
                IntPredicate::EQ,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::NotEqual => self.llvm.builder.build_int_compare(
                IntPredicate::NE,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::Less => self.llvm.builder.build_int_compare(
                IntPredicate::SLT,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::Greater => self.llvm.builder.build_int_compare(
                IntPredicate::SGT,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::LessOrEqual => self.llvm.builder.build_int_compare(
                IntPredicate::SLE,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::GreaterOrEqual => self.llvm.builder.build_int_compare(
                IntPredicate::SGE,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),
            Operator::Xor => self
                .llvm
                .builder
                .build_xor(int_lvalue, int_rvalue, "tmpVar"),
            Operator::And => self
                .llvm
                .builder
                .build_and(int_lvalue, int_rvalue, "tmpVar"),
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
            Operator::Plus => self
                .llvm
                .builder
                .build_float_add(float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Minus => self
                .llvm
                .builder
                .build_float_sub(float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Multiplication => self
                .llvm
                .builder
                .build_float_mul(float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Division => self
                .llvm
                .builder
                .build_float_div(float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Modulo => self
                .llvm
                .builder
                .build_float_rem(float_lvalue, float_rvalue, "tmpVar")
                .into(),

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
        stmt: &AstStatement,
        number: &str,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let type_hint = self.get_type_hint_for(stmt)?;
        let actual_type = self.annotations.get_type_or_void(stmt, self.index);
        let literal_type = if is_same_type_class(
            type_hint.get_type_information(),
            actual_type.get_type_information(),
            self.index,
        ) {
            type_hint
        } else {
            actual_type
        };
        let literal_type = self
            .llvm_index
            .get_associated_type(literal_type.get_name())?;
        self.llvm
            .create_const_numeric(&literal_type, number, stmt.get_location())
    }

    /// generates the literal statement and returns the resulting value
    ///
    /// - `literal_statement` one of LiteralBool, LiteralInteger, LiteralReal, LiteralString
    pub fn generate_literal(
        &self,
        literal_statement: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        match literal_statement {
            AstStatement::LiteralBool { value, .. } => self.llvm.create_const_bool(*value),
            AstStatement::LiteralInteger { value, .. } => {
                self.generate_numeric_literal(literal_statement, value.to_string().as_str())
            }
            AstStatement::LiteralReal { value, .. } => {
                self.generate_numeric_literal(literal_statement, value)
            }
            AstStatement::LiteralDate {
                year,
                month,
                day,
                location,
                ..
            } => self.create_const_int(
                super::date_time_util::calculate_date_time(*year, *month, *day, 0, 0, 0, 0)
                    .map_err(|op| Diagnostic::codegen_error(op.as_str(), location.clone()))?,
            ),
            AstStatement::LiteralDateAndTime {
                year,
                month,
                day,
                hour,
                min,
                sec,
                milli,
                location,
                ..
            } => self.create_const_int(
                super::date_time_util::calculate_date_time(
                    *year, *month, *day, *hour, *min, *sec, *milli,
                )
                .map_err(|op| Diagnostic::codegen_error(op.as_str(), location.clone()))?,
            ),
            AstStatement::LiteralTimeOfDay {
                hour,
                min,
                sec,
                milli,
                location,
                ..
            } => self.create_const_int(
                super::date_time_util::calculate_date_time(1970, 1, 1, *hour, *min, *sec, *milli)
                    .map_err(|op| Diagnostic::codegen_error(op.as_str(), location.clone()))?,
            ),
            AstStatement::LiteralTime {
                day,
                hour,
                min,
                sec,
                milli,
                micro,
                nano,
                negative,
                ..
            } => self.create_const_int(super::date_time_util::calculate_time_nano(
                *negative,
                super::date_time_util::calculate_dhm_time_seconds(*day, *hour, *min, *sec),
                *milli,
                *micro,
                *nano,
            )),

            AstStatement::LiteralString {
                value, location, ..
            } => self.generate_string_literal(literal_statement, value, location),
            AstStatement::LiteralArray {
                elements: Some(elements),
                ..
            } => self.generate_literal_array(elements),
            AstStatement::MultipliedStatement { .. } => {
                self.generate_literal_array(literal_statement)
            }
            AstStatement::LiteralNull { .. } => self.llvm.create_null_ptr(),
            // if there is an expression-list this might be a struct-initialization or array-initialization
            AstStatement::ExpressionList { .. } => {
                let type_hint = self.get_type_hint_info_for(literal_statement)?;
                match type_hint {
                    DataTypeInformation::Array { .. } => {
                        self.generate_literal_array(literal_statement)
                    }
                    _ => self.generate_literal_struct(
                        literal_statement,
                        &literal_statement.get_location(),
                    ),
                }
            }
            // if there is just one assignment, this may be an struct-initialization (TODO this is not very elegant :-/ )
            AstStatement::Assignment { .. } => {
                self.generate_literal_struct(literal_statement, &literal_statement.get_location())
            }
            AstStatement::CastStatement { target, .. } => self.generate_expression(target),
            _ => Err(Diagnostic::codegen_error(
                &format!("Cannot generate Literal for {:?}", literal_statement),
                literal_statement.get_location(),
            )),
        }
    }

    /// generates the string-literal `value` represented by `literal_statement`
    fn generate_string_literal(
        &self,
        literal_statement: &AstStatement,
        value: &str,
        location: &SourceRange,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let expected_type = self.get_type_hint_info_for(literal_statement)?;
        self.generate_string_literal_for_type(expected_type, value, location)
    }

    fn generate_string_literal_for_type(
        &self,
        expected_type: &DataTypeInformation,
        value: &str,
        location: &SourceRange,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        match expected_type {
            DataTypeInformation::String { encoding, size, .. } => {
                let declared_length = size.as_int_value(self.index).map_err(|msg| {
                    Diagnostic::codegen_error(
                        format!("Unable to generate string-literal: {}", msg).as_str(),
                        location.clone(),
                    )
                })? as usize;

                match encoding {
                    StringEncoding::Utf8 => {
                        let literal = self
                            .llvm_index
                            .find_utf08_literal_string(value)
                            .map(GlobalValue::as_basic_value_enum);
                        if let Some((literal_value, _)) = literal.zip(self.function_context) {
                            //global constant string
                            Ok(literal_value)
                        } else {
                            //note that .len() will give us the number of bytes, not the number of characters
                            let actual_length = value.chars().count() + 1; // +1 to account for a final \0
                            let str_len = std::cmp::min(
                                (self.string_len_provider)(declared_length, actual_length),
                                declared_length,
                            );
                            self.llvm.create_const_utf8_string(value, str_len)
                        }
                    }
                    StringEncoding::Utf16 => {
                        let literal = self.llvm_index.find_utf16_literal_string(value);
                        if literal.is_some()
                            && self.function_context.is_some()
                            && self.function_context.is_some()
                        {
                            //global constant string
                            Ok(literal.map(|it| it.as_basic_value_enum()).unwrap())
                        } else {
                            //note that .len() will give us the number of bytes, not the number of characters
                            let actual_length = value.encode_utf16().count() + 1; // +1 to account for a final \0
                            let str_len = std::cmp::min(
                                (self.string_len_provider)(declared_length, actual_length),
                                declared_length,
                            );
                            self.llvm.create_const_utf16_string(value, str_len)
                        }
                    }
                }
            }
            DataTypeInformation::Pointer {
                inner_type_name,
                auto_deref: true,
                ..
            } => {
                let inner_type = self.index.get_type_information_or_void(inner_type_name);
                self.generate_string_literal_for_type(inner_type, value, location)
            }
            DataTypeInformation::Integer { size: 8, .. } if expected_type.is_character() => {
                self.llvm.create_llvm_const_i8_char(value, location)
            }
            DataTypeInformation::Integer { size: 16, .. } if expected_type.is_character() => {
                self.llvm.create_llvm_const_i16_char(value, location)
            }
            _ => Err(Diagnostic::cannot_generate_string_literal(
                expected_type.get_name(),
                location.clone(),
            )),
        }
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_info_for(
        &self,
        statement: &AstStatement,
    ) -> Result<&DataTypeInformation, Diagnostic> {
        self.get_type_hint_for(statement)
            .map(DataType::get_type_information)
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_for(&self, statement: &AstStatement) -> Result<&DataType, Diagnostic> {
        self.annotations
            .get_type_hint(statement, self.index)
            .or_else(|| self.annotations.get_type(statement, self.index))
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    &format!("no type hint available for {:#?}", statement),
                    statement.get_location(),
                )
            })
    }

    /// generates a struct literal value with the given value assignments (ExpressionList)
    fn generate_literal_struct(
        &self,
        assignments: &AstStatement,
        declaration_location: &SourceRange,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        if let DataTypeInformation::Struct {
            name: struct_name,
            member_names,
            ..
        } = self.get_type_hint_info_for(assignments)?
        {
            let mut uninitialized_members: HashSet<&str> =
                member_names.iter().map(|it| it.as_str()).collect();
            let mut member_values: Vec<(u32, BasicValueEnum<'ink>)> = Vec::new();
            for assignment in flatten_expression_list(assignments) {
                if let AstStatement::Assignment { left, right, .. } = assignment {
                    if let AstStatement::Reference {
                        name: variable_name,
                        location,
                        ..
                    } = &**left
                    {
                        let member = self
                            .index
                            .find_member(struct_name, variable_name)
                            .ok_or_else(|| {
                                Diagnostic::unresolved_reference(
                                    format!("{}.{}", struct_name, variable_name).as_str(),
                                    location.clone(),
                                )
                            })?;

                        let index_in_parent = member.get_location_in_parent();
                        let value = self.generate_expression(right)?;

                        uninitialized_members.remove(member.get_name());
                        member_values.push((index_in_parent, value));
                    } else {
                        return Err(Diagnostic::codegen_error(
                            "struct member lvalue required as left operand of assignment",
                            left.get_location(),
                        ));
                    }
                } else {
                    return Err(Diagnostic::codegen_error("struct literal must consist of explicit assignments in the form of member := value", assignment.get_location()));
                }
            }

            //fill the struct with fields we didnt mention yet
            for variable_name in uninitialized_members {
                let member = self
                    .index
                    .find_member(struct_name, variable_name)
                    .ok_or_else(|| {
                        Diagnostic::unresolved_reference(
                            format!("{}.{}", struct_name, variable_name).as_str(),
                            declaration_location.clone(),
                        )
                    })?;

                let initial_value = self
                    .llvm_index
                    .find_associated_variable_value(member.get_qualified_name())
                    // .or_else(|| self.index.find_associated_variable_value(name))
                    .or_else(|| {
                        self.llvm_index
                            .find_associated_initial_value(member.get_type_name())
                    })
                    .ok_or_else(|| {
                        Diagnostic::cannot_generate_initializer(
                            member.get_qualified_name(),
                            assignments.get_location(),
                        )
                    })?;

                member_values.push((member.get_location_in_parent(), initial_value));
            }
            let struct_type = self
                .llvm_index
                .get_associated_type(struct_name)?
                .into_struct_type();
            if member_values.len() == struct_type.count_fields() as usize {
                member_values.sort_by(|(a, _), (b, _)| a.cmp(b));
                let ordered_values: Vec<BasicValueEnum<'ink>> =
                    member_values.iter().map(|(_, v)| *v).collect();

                return Ok(struct_type
                    .const_named_struct(ordered_values.as_slice())
                    .as_basic_value_enum());
            } else {
                Err(Diagnostic::codegen_error(
                    &format!(
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
                &format!("Expected Struct-literal, got {:#?}", assignments),
                assignments.get_location(),
            ))
        }
    }

    /// generates an array literal with the given optional elements (represented as an ExpressionList)
    fn generate_literal_array(
        &self,
        initializer: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let array_value = self.generate_literal_array_value(
            flatten_expression_list(initializer),
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
        elements: Vec<&AstStatement>,
        data_type: &DataTypeInformation,
        location: &SourceRange,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let (inner_type, expected_len) = if let DataTypeInformation::Array {
            inner_type_name,
            dimensions,
            ..
        } = data_type
        {
            let len: u32 = dimensions
                .iter()
                .map(|d| d.get_length(self.index))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|msg| Diagnostic::codegen_error(msg.as_str(), location.clone()))?
                .into_iter()
                .product();

            self.index
                .get_type(inner_type_name)
                .map(|inner_type| (inner_type, len as usize))
        } else {
            Err(Diagnostic::codegen_error(
                format!("Expected array type but found: {:}", data_type.get_name()).as_str(),
                location.clone(),
            ))
        }?;

        let llvm_type = self.llvm_index.get_associated_type(inner_type.get_name())?;
        let mut v = Vec::new();
        for e in elements {
            //generate with correct type hint
            let value = self.generate_literal(e)?;
            v.push(value.as_basic_value_enum());
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
                v.iter()
                    .map(|it| it.into_array_value())
                    .collect::<Vec<ArrayValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::FloatType(_) => llvm_type.into_float_type().const_array(
                v.iter()
                    .map(|it| it.into_float_value())
                    .collect::<Vec<FloatValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::IntType(_) => llvm_type.into_int_type().const_array(
                v.iter()
                    .map(|it| it.into_int_value())
                    .collect::<Vec<IntValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::PointerType(_) => llvm_type.into_pointer_type().const_array(
                v.iter()
                    .map(|it| it.into_pointer_value())
                    .collect::<Vec<PointerValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::StructType(_) => llvm_type.into_struct_type().const_array(
                v.iter()
                    .map(|it| it.into_struct_value())
                    .collect::<Vec<StructValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::VectorType(_) => llvm_type.into_vector_type().const_array(
                v.iter()
                    .map(|it| it.into_vector_value())
                    .collect::<Vec<VectorValue>>()
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
        left: &AstStatement,
        right: &AstStatement,
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
                    to_i1(
                        self.generate_expression(left)?.into_int_value(),
                        &self.llvm.builder,
                    ),
                    to_i1(
                        self.generate_expression(right)?.into_int_value(),
                        &self.llvm.builder,
                    ),
                    "",
                )
                .as_basic_value_enum()),
            Operator::NotEqual => Ok(self
                .llvm
                .builder
                .build_int_compare(
                    IntPredicate::NE,
                    to_i1(
                        self.generate_expression(left)?.into_int_value(),
                        &self.llvm.builder,
                    ),
                    to_i1(
                        self.generate_expression(right)?.into_int_value(),
                        &self.llvm.builder,
                    ),
                    "",
                )
                .as_basic_value_enum()),
            Operator::Xor => Ok(self
                .llvm
                .builder
                .build_xor(
                    to_i1(
                        self.generate_expression(left)?.into_int_value(),
                        &self.llvm.builder,
                    ),
                    to_i1(
                        self.generate_expression(right)?.into_int_value(),
                        &self.llvm.builder,
                    ),
                    "",
                )
                .as_basic_value_enum()),
            _ => Err(Diagnostic::codegen_error(
                format!("illegal boolean expresspion for operator {:}", operator).as_str(),
                (left.get_location().get_start()..right.get_location().get_end()).into(),
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
        left: &AstStatement,
        right: &AstStatement,
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
                    &format!("Cannot generate phi-expression for operator {:}", operator),
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
            SourceRange::undefined(),
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
        operator: &Operator,
        left: &AstStatement,
        right: &AstStatement,
        binary_statement: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        if let Some(StatementAnnotation::Value { .. }) = self.annotations.get(binary_statement) {
            // we trust that the validator only passed us valid parameters (so left & right should be same type)
            let call_statement = match operator {
                // a <> b expression is handled as Not(Equal(a,b))
                Operator::NotEqual => ast::create_not_expression(
                    self.create_typed_compare_call_statement(
                        &Operator::Equal,
                        left,
                        right,
                        binary_statement,
                    )?,
                    binary_statement.get_location(),
                ),
                // a <= b expression is handled as a = b OR a < b
                Operator::LessOrEqual => ast::create_or_expression(
                    self.create_typed_compare_call_statement(
                        &Operator::Equal,
                        left,
                        right,
                        binary_statement,
                    )?,
                    self.create_typed_compare_call_statement(
                        &Operator::Less,
                        left,
                        right,
                        binary_statement,
                    )?,
                ),
                // a >= b expression is handled as a = b OR a > b
                Operator::GreaterOrEqual => ast::create_or_expression(
                    self.create_typed_compare_call_statement(
                        &Operator::Equal,
                        left,
                        right,
                        binary_statement,
                    )?,
                    self.create_typed_compare_call_statement(
                        &Operator::Greater,
                        left,
                        right,
                        binary_statement,
                    )?,
                ),
                _ => self.create_typed_compare_call_statement(
                    operator,
                    left,
                    right,
                    binary_statement,
                )?,
            };
            self.generate_expression(&call_statement)
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

    /// tries to call one of the EQUAL_XXX, LESS_XXX, GREATER_XXX functions for the
    /// given type (of left). The given operator has to be a comparison-operator
    fn create_typed_compare_call_statement(
        &self,
        operator: &Operator,
        left: &AstStatement,
        right: &AstStatement,
        binary_statement: &AstStatement,
    ) -> Result<AstStatement, Diagnostic> {
        let left_type = self.get_type_hint_for(left)?;
        let right_type = self.get_type_hint_for(right)?;
        let cmp_function_name = crate::typesystem::get_equals_function_name_for(
            left_type.get_type_information().get_name(),
            operator,
        );

        cmp_function_name
            .map(|name| {
                crate::ast::create_call_to(
                    name,
                    vec![left.clone(), right.clone()],
                    binary_statement.get_id(),
                    left.get_id(),
                    &binary_statement.get_location(),
                )
            })
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    format!(
                        "Invalid operator {} for types {} and {}",
                        operator,
                        left_type.get_name(),
                        right_type.get_name()
                    )
                    .as_str(),
                    binary_statement.get_location(),
                )
            })
    }

    pub fn generate_store(
        &self,
        left_type: &DataTypeInformation,
        right_statement: &AstStatement,
        left: inkwell::values::PointerValue,
    ) -> Result<(), Diagnostic> {
        let right_type = self
            .annotations
            .get_type_or_void(right_statement, self.index)
            .get_type_information();

        //Special string handling
        if left_type.is_string() && right_type.is_string()
        //string-literals are also generated as global constant variables so we can always assume that
        //we have a pointer-value
        {
            let right = match right_statement {
                AstStatement::QualifiedReference { .. } | AstStatement::Reference { .. } => {
                    self.generate_element_pointer(right_statement)?
                }
                _ => {
                    let expression = self.generate_expression(right_statement)?;

                    if expression.is_pointer_value() {
                        expression.into_pointer_value()
                    } else {
                        //TODO should this ever happen?
                        let right = self.llvm.builder.build_alloca(expression.get_type(), "");
                        self.llvm.builder.build_store(right, expression);

                        right
                    }
                }
            };
            let target_size = self.get_string_size(left_type, right_statement.get_location())?; //we report error on parameter :-/
            let value_size = self.get_string_size(right_type, right_statement.get_location())?;
            let size = std::cmp::min(target_size - 1, value_size) as i64;
            let align_left = left_type.get_string_character_width(self.index).value();
            let align_right = right_type.get_string_character_width(self.index).value();
            //Multiply by the string alignment to copy enough for widestrings
            //This is done at compile time to avoid generating an extra mul
            let size = self
                .llvm
                .context
                .i32_type()
                .const_int((size * align_left as i64) as u64, true);
            self.llvm
                .builder
                .build_memcpy(left, align_left, right, align_right, size)
                .map_err(|err| Diagnostic::codegen_error(err, right_statement.get_location()))?;
        } else {
            let expression = self.generate_expression(right_statement)?;
            self.llvm.builder.build_store(left, expression);
        }
        Ok(())
    }

    fn get_string_size(
        &self,
        datatype: &DataTypeInformation,
        location: SourceRange,
    ) -> Result<i64, Diagnostic> {
        if let DataTypeInformation::String { size, .. } = datatype {
            size.as_int_value(self.index)
                .map_err(|err| Diagnostic::codegen_error(err.as_str(), location))
        } else {
            Err(Diagnostic::codegen_error(
                format!("{} is not a String", datatype.get_name()).as_str(),
                location,
            ))
        }
    }
}

/// Returns the information required to call a parameter implicitly in a function
/// If the parameter is already implicit, it does nothing.
/// if the parameter is explicit ´param := value´,
/// it returns the location of the parameter in the function declaration
///  as well as the parameter value (right side) ´param := value´ => ´value´
pub fn get_implicit_call_parameter<'a>(
    param_statement: &'a AstStatement,
    declared_parameters: &[&VariableIndexEntry],
    idx: usize,
) -> Result<(usize, &'a AstStatement), Diagnostic> {
    let (location, param_statement) = match param_statement {
        AstStatement::Assignment { left, right, .. }
        | AstStatement::OutputAssignment { left, right, .. } => {
            //explicit
            let loc = if let AstStatement::Reference {
                name: left_name, ..
            } = left.as_ref()
            {
                let position = declared_parameters
                    .iter()
                    .position(|p| p.get_name() == left_name);
                position
                    .ok_or_else(|| Diagnostic::unresolved_reference(left_name, left.get_location()))
            } else {
                unreachable!("left of an assignment must be a reference");
            }?;

            (loc, right.as_ref())
        }
        _ => {
            //implicit
            (idx, param_statement)
        }
    };
    Ok((location, param_statement))
}

/// turns the given intValue into an i1 by comparing it to 0 (of the same size)
pub fn to_i1<'a>(value: IntValue<'a>, builder: &Builder<'a>) -> IntValue<'a> {
    if value.get_type().get_bit_width() > 1 {
        builder.build_int_compare(
            IntPredicate::NE,
            value,
            value.get_type().const_int(0, false),
            "",
        )
    } else {
        value
    }
}
