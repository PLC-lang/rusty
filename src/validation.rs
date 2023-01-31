use std::collections::HashSet;

use crate::{
    ast::{
        self, AstStatement, CompilationUnit, DataType, DataTypeDeclaration, Pou, PouType, SourceRange,
        UserTypeDeclaration, Variable, VariableBlock,
    },
    codegen::generators::expression_generator::get_implicit_call_parameter,
    index::{ArgumentType, Index, PouIndexEntry, VariableIndexEntry, VariableType},
    resolver::{const_evaluator, AnnotationMap, AnnotationMapImpl, StatementAnnotation},
    typesystem::{self, DataTypeInformation},
    Diagnostic,
};

use self::{
    global_validator::GlobalValidator, pou_validator::PouValidator, recursive_validator::RecursiveValidator,
    stmt_validator::StatementValidator, variable_validator::VariableValidator,
};

mod global_validator;
mod pou_validator;
mod recursive_validator;
mod stmt_validator;
mod variable_validator;

#[cfg(test)]
mod tests;

macro_rules! visit_all_statements {
     ($self:expr, $context:expr, $last:expr ) => {
         $self.visit_statement($last, $context);
     };

     ($self:expr, $context:expr, $head:expr, $($tail:expr), +) => {
       $self.visit_statement($head, $context);
       visit_all_statements!($self, $context, $($tail),+)
     };
   }

pub struct ValidationContext<'s> {
    ast_annotation: &'s AnnotationMapImpl,
    index: &'s Index,
    qualifier: Option<&'s str>,
}

impl<'s> ValidationContext<'s> {
    /// try find a POU for the given statement
    fn find_pou(&self, stmt: &AstStatement) -> Option<&PouIndexEntry> {
        match stmt {
            AstStatement::Reference { name, .. } => Some(name),
            AstStatement::QualifiedReference { elements, .. } => {
                if let Some(stmt) = elements.last() {
                    if let Some(StatementAnnotation::Variable { resulting_type, .. }) =
                        self.ast_annotation.get(stmt)
                    {
                        Some(resulting_type)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
        .and_then(|pou_name| self.index.find_pou(pou_name))
    }
}

pub struct Validator {
    //context: ValidationContext<'s>,
    pou_validator: PouValidator,
    variable_validator: VariableValidator,
    stmt_validator: StatementValidator,
    global_validator: GlobalValidator,
    recursive_validator: RecursiveValidator,
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            pou_validator: PouValidator::new(),
            variable_validator: VariableValidator::new(),
            stmt_validator: StatementValidator::new(),
            global_validator: GlobalValidator::new(),
            recursive_validator: RecursiveValidator::new(),
        }
    }

    pub fn diagnostics(&mut self) -> Vec<Diagnostic> {
        let mut all_diagnostics = Vec::new();
        all_diagnostics.append(&mut self.pou_validator.diagnostics);
        all_diagnostics.append(&mut self.variable_validator.diagnostics);
        all_diagnostics.append(&mut self.stmt_validator.diagnostics);
        all_diagnostics.append(&mut self.global_validator.diagnostics);
        all_diagnostics.append(&mut self.recursive_validator.diagnostics);
        all_diagnostics
    }

    pub fn perform_global_validation(&mut self, index: &Index) {
        self.global_validator.validate_unique_symbols(index);
        self.recursive_validator.validate_recursion(index);
    }

    pub fn visit_unit(&mut self, annotations: &AnnotationMapImpl, index: &Index, unit: &CompilationUnit) {
        for pou in &unit.units {
            self.visit_pou(
                pou,
                &ValidationContext { ast_annotation: annotations, index, qualifier: Some(pou.name.as_str()) },
            );
        }

        let no_context = &ValidationContext { ast_annotation: annotations, index, qualifier: None };
        for t in &unit.types {
            self.visit_user_type_declaration(t, no_context);
        }

        for gv in &unit.global_vars {
            self.visit_variable_container(no_context, gv);
        }

        for i in &unit.implementations {
            let context =
                ValidationContext { ast_annotation: annotations, index, qualifier: Some(i.name.as_str()) };
            if i.pou_type == PouType::Action && i.type_name == "__unknown__" {
                self.pou_validator.diagnostics.push(Diagnostic::missing_action_container(i.location.clone()));
            }
            i.statements.iter().for_each(|s| self.visit_statement(s, &context));
        }
    }

    pub fn visit_user_type_declaration(
        &mut self,
        user_data_type: &UserTypeDeclaration,
        context: &ValidationContext,
    ) {
        self.visit_data_type(context, &user_data_type.data_type, &user_data_type.location);
    }

    pub fn visit_pou(&mut self, pou: &Pou, context: &ValidationContext) {
        self.pou_validator.validate_pou(pou, context);

        for block in &pou.variable_blocks {
            self.visit_variable_container(context, block);
        }
    }

    pub fn visit_variable_container(&mut self, context: &ValidationContext, container: &VariableBlock) {
        self.variable_validator.validate_variable_block(container);

        for variable in &container.variables {
            self.visit_variable(context, variable);
        }
    }

    pub fn visit_variable(&mut self, context: &ValidationContext, variable: &Variable) {
        self.variable_validator.validate_variable(variable, context);

        self.visit_data_type_declaration(context, &variable.data_type);
    }

    pub fn visit_data_type_declaration(
        &mut self,
        context: &ValidationContext,
        declaration: &DataTypeDeclaration,
    ) {
        self.variable_validator.validate_data_type_declaration(declaration);

        if let DataTypeDeclaration::DataTypeDefinition { data_type, location, .. } = declaration {
            self.visit_data_type(context, data_type, location);
        }
    }

    pub fn visit_data_type(
        &mut self,
        context: &ValidationContext,
        data_type: &DataType,
        location: &SourceRange,
    ) {
        self.variable_validator.validate_data_type(data_type, location);

        match data_type {
            DataType::StructType { variables, .. } => {
                variables.iter().for_each(|v| self.visit_variable(context, v))
            }
            DataType::ArrayType { referenced_type, .. } => {
                self.visit_data_type_declaration(context, referenced_type)
            }
            DataType::VarArgs { referenced_type: Some(referenced_type), .. } => {
                self.visit_data_type_declaration(context, referenced_type.as_ref());
            }
            _ => {}
        }
    }

    pub fn visit_statement(&mut self, statement: &AstStatement, context: &ValidationContext) {
        match statement {
            AstStatement::LiteralArray { elements: Some(elements), .. } => {
                self.visit_statement(elements.as_ref(), context)
            }
            AstStatement::MultipliedStatement { element, .. } => self.visit_statement(element, context),
            AstStatement::QualifiedReference { elements, .. } => {
                elements.iter().for_each(|e| self.visit_statement(e, context))
            }
            AstStatement::ArrayAccess { reference, access, .. } => {
                visit_all_statements!(self, context, reference, access);
            }
            AstStatement::BinaryExpression { left, right, .. } => {
                visit_all_statements!(self, context, left, right);
            }
            AstStatement::UnaryExpression { value, .. } => self.visit_statement(value, context),
            AstStatement::ExpressionList { expressions, .. } => {
                expressions.iter().for_each(|e| self.visit_statement(e, context))
            }
            AstStatement::RangeStatement { start, end, .. } => {
                visit_all_statements!(self, context, start, end);
            }
            AstStatement::Assignment { left, right, .. } => {
                self.visit_statement(left, context);
                self.visit_statement(right, context);
            }
            AstStatement::OutputAssignment { left, right, .. } => {
                self.visit_statement(left, context);
                self.visit_statement(right, context);
            }
            AstStatement::CallStatement { parameters, operator, .. } => {
                // visit called pou
                self.visit_statement(operator, context);

                if let Some(pou) = context.find_pou(operator) {
                    let declared_parameters = context.index.get_declared_parameters(pou.get_name());
                    let passed_parameters =
                        parameters.as_ref().as_ref().map(ast::flatten_expression_list).unwrap_or_default();

                    let mut passed_params_idx = Vec::new();
                    let mut are_implicit_parameters = true;
                    // validate parameters
                    for (i, p) in passed_parameters.iter().enumerate() {
                        if let Ok((location_in_parent, right, is_implicit)) =
                            get_implicit_call_parameter(p, &declared_parameters, i)
                        {
                            // safe index of passed parameter
                            passed_params_idx.push(location_in_parent);

                            let left = declared_parameters.get(location_in_parent);
                            let left_type = left.map(|param| {
                                context.index.get_effective_type_or_void_by_name(param.get_type_name())
                            });
                            let right_type = context.ast_annotation.get_type(right, context.index);

                            if let (Some(left), Some(left_type), Some(right_type)) =
                                (left, left_type, right_type)
                            {
                                self.validate_call_parameter_assignment(
                                    left,
                                    left_type,
                                    right_type,
                                    p.get_location(),
                                    context.index,
                                );

                                self.validate_call_by_ref(left, p);

                                self.validate_passed_call_parameter_size(
                                    left_type,
                                    right_type,
                                    p.get_location(),
                                    context.index,
                                );
                            }

                            // mixing implicit and explicit parameters is not allowed
                            // allways compare to the first passed parameter
                            if i == 0 {
                                are_implicit_parameters = is_implicit;
                            } else if are_implicit_parameters != is_implicit {
                                self.stmt_validator
                                    .diagnostics
                                    .push(Diagnostic::invalid_parameter_type(p.get_location()));
                            }
                        }

                        self.visit_statement(p, context);
                    }

                    // for PROGRAM/FB we need special inout validation
                    if let PouIndexEntry::FunctionBlock { .. } | PouIndexEntry::Program { .. } = pou {
                        let inouts: Vec<&&VariableIndexEntry> = declared_parameters
                            .iter()
                            .filter(|p| VariableType::InOut == p.get_variable_type())
                            .collect();
                        // if the called pou has declared inouts, we need to make sure that these were passed to the pou call
                        if !inouts.is_empty() {
                            // check if all inouts were passed to the pou call
                            inouts.into_iter().for_each(|p| {
                                if !passed_params_idx.contains(&(p.get_location_in_parent() as usize)) {
                                    self.stmt_validator.diagnostics.push(
                                        Diagnostic::missing_inout_parameter(
                                            p.get_name(),
                                            operator.get_location(),
                                        ),
                                    );
                                }
                            });
                        }
                    }
                } else {
                    // Pou could not be found, we can still partially validate the passed parameters
                    if let Some(s) = parameters.as_ref() {
                        self.visit_statement(s, context);
                    }
                }
            }
            AstStatement::IfStatement { blocks, else_block, .. } => {
                blocks.iter().for_each(|b| {
                    self.visit_statement(b.condition.as_ref(), context);
                    b.body.iter().for_each(|s| self.visit_statement(s, context));
                });
                else_block.iter().for_each(|e| self.visit_statement(e, context));
            }
            AstStatement::ForLoopStatement { counter, start, end, by_step, body, .. } => {
                visit_all_statements!(self, context, counter, start, end);
                if let Some(by_step) = by_step {
                    self.visit_statement(by_step, context);
                }
                body.iter().for_each(|s| self.visit_statement(s, context));
            }
            AstStatement::WhileLoopStatement { condition, body, .. } => {
                self.visit_statement(condition, context);
                body.iter().for_each(|s| self.visit_statement(s, context));
            }
            AstStatement::RepeatLoopStatement { condition, body, .. } => {
                self.visit_statement(condition, context);
                body.iter().for_each(|s| self.visit_statement(s, context));
            }
            AstStatement::CaseStatement { selector, case_blocks, else_block, .. } => {
                self.visit_statement(selector, context);

                let mut cases = HashSet::new();
                case_blocks.iter().for_each(|b| {
                    let condition = b.condition.as_ref();

                    // invalid case conditions
                    if matches!(
                        condition,
                        AstStatement::Assignment { .. } | AstStatement::CallStatement { .. }
                    ) {
                        self.stmt_validator
                            .diagnostics
                            .push(Diagnostic::invalid_case_condition(condition.get_location()));
                    }

                    // validate for duplicate conditions
                    // first try to evaluate the conditions value
                    const_evaluator::evaluate(condition, context.qualifier, context.index)
                        .map_err(|err| {
                            // value evaluation and validation not possible with non constants
                            self.stmt_validator
                                .diagnostics
                                .push(Diagnostic::non_constant_case_condition(&err, condition.get_location()))
                        })
                        .map(|v| {
                            // check for duplicates if we got a value
                            if let Some(AstStatement::LiteralInteger { value, .. }) = v {
                                if !cases.insert(value) {
                                    self.stmt_validator.diagnostics.push(
                                        Diagnostic::duplicate_case_condition(
                                            &value,
                                            condition.get_location(),
                                        ),
                                    );
                                }
                            };
                        })
                        .ok(); // no need to worry about the result

                    self.visit_statement(condition, context);
                    b.body.iter().for_each(|s| self.visit_statement(s, context));
                });

                else_block.iter().for_each(|s| self.visit_statement(s, context));
            }
            AstStatement::CaseCondition { condition, .. } => {
                // if we get here, then a `CaseCondition` is used outside a `CaseStatement`
                // `CaseCondition` are used as a marker for `CaseStatements` and are not passed as such to the `CaseStatement.case_blocks`
                // see `control_parser` `parse_case_statement()`
                self.stmt_validator
                    .diagnostics
                    .push(Diagnostic::case_condition_used_outside_case_statement(condition.get_location()));
                self.visit_statement(condition, context)
            }
            _ => {}
        }

        self.stmt_validator.validate_statement(statement, context);
    }

    /// Validates if an argument can be passed to a function with [`VariableType::Output`] and
    /// [`VariableType::InOut`] parameter types by checking if the argument is a reference (e.g. `foo(x)`) or
    /// an assignment (e.g. `foo(x := y)`, `foo(x => y)`). If neither is the case a diagnostic is generated.
    fn validate_call_by_ref(&mut self, param: &VariableIndexEntry, arg: &AstStatement) {
        if matches!(param.variable_type.get_variable_type(), VariableType::Output | VariableType::InOut) {
            match arg {
                AstStatement::Reference { .. } | AstStatement::QualifiedReference { .. } => (),

                AstStatement::Assignment { right, .. } | AstStatement::OutputAssignment { right, .. } => {
                    self.validate_call_by_ref(param, right);
                }

                _ => self.stmt_validator.diagnostics.push(Diagnostic::invalid_argument_type(
                    param.get_name(),
                    param.get_variable_type(),
                    arg.get_location(),
                )),
            }
        }
    }

    fn validate_call_parameter_assignment(
        &mut self,
        left: &VariableIndexEntry,
        left_type: &typesystem::DataType,
        right_type: &typesystem::DataType,
        location: SourceRange,
        index: &Index,
    ) {
        // for parameters passed `ByRef` we need to check the inner type of the pointer
        let left_type_info = if matches!(left.variable_type, ArgumentType::ByRef(..)) {
            index.find_elementary_pointer_type(left_type.get_type_information())
        } else {
            index.find_intrinsic_type(left_type.get_type_information())
        };
        let right_type_info = index.find_intrinsic_type(right_type.get_type_information());
        // stmt_validator `validate_type_nature()` should report any error see `generic_validation_tests` ignore generics here and safe work
        if !matches!(left_type_info, DataTypeInformation::Generic { .. })
            & !typesystem::is_same_type_class(left_type_info, right_type_info, index)
        {
            self.stmt_validator.diagnostics.push(Diagnostic::invalid_assignment(
                right_type_info.get_name(),
                left_type_info.get_name(),
                location,
            ))
        }
    }

    fn validate_passed_call_parameter_size(
        &mut self,
        declared_type: &typesystem::DataType,
        passed_type: &typesystem::DataType,
        location: SourceRange,
        index: &Index,
    ) {
        let passed_type_info = passed_type.get_type_information();
        let declared_type_info = declared_type.get_type_information();

        let (declared_size, declared_name) = if let DataTypeInformation::Pointer { inner_type_name, .. } =
            declared_type_info
        {
            (index.get_type_information_or_void(inner_type_name).get_size(index), inner_type_name.as_str())
        } else {
            (declared_type_info.get_size(index), declared_type_info.get_name())
        };

        let (passed_size, passed_name) = if let DataTypeInformation::Pointer { inner_type_name, .. } =
            passed_type_info
        {
            (index.get_type_information_or_void(inner_type_name).get_size(index), inner_type_name.as_str())
        } else {
            (passed_type_info.get_size(index), passed_type_info.get_name())
        };

        if declared_size < passed_size {
            self.stmt_validator.diagnostics.push(Diagnostic::implicit_truncation(
                declared_name,
                passed_name,
                location,
            ))
        }
    }
}
