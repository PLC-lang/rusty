use std::collections::HashSet;

use crate::{
    ast::{
        AstStatement, CompilationUnit, DataType, DataTypeDeclaration, Pou, PouType, SourceRange,
        UserTypeDeclaration, Variable, VariableBlock,
    },
    codegen::generators::expression_generator::get_implicit_call_parameter,
    index::{Index, PouIndexEntry, VariableIndexEntry, VariableType},
    resolver::{const_evaluator, AnnotationMap, AnnotationMapImpl, StatementAnnotation},
    Diagnostic,
};

use self::{
    pou_validator::PouValidator, stmt_validator::StatementValidator,
    variable_validator::VariableValidator,
};

mod pou_validator;
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
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            pou_validator: PouValidator::new(),
            variable_validator: VariableValidator::new(),
            stmt_validator: StatementValidator::new(),
        }
    }

    pub fn diagnostics(&mut self) -> Vec<Diagnostic> {
        let mut all_diagnostics = Vec::new();
        all_diagnostics.append(&mut self.pou_validator.diagnostics);
        all_diagnostics.append(&mut self.variable_validator.diagnostics);
        all_diagnostics.append(&mut self.stmt_validator.diagnostics);
        all_diagnostics
    }

    pub fn visit_unit(
        &mut self,
        annotations: &AnnotationMapImpl,
        index: &Index,
        unit: &CompilationUnit,
    ) {
        for pou in &unit.units {
            self.visit_pou(
                pou,
                &ValidationContext {
                    ast_annotation: annotations,
                    index,
                    qualifier: Some(pou.name.as_str()),
                },
            );
        }

        let no_context = &ValidationContext {
            ast_annotation: annotations,
            index,
            qualifier: None,
        };
        for t in &unit.types {
            self.visit_user_type_declaration(t, no_context);
        }

        for gv in &unit.global_vars {
            self.visit_variable_container(no_context, gv);
        }

        for i in &unit.implementations {
            let context = ValidationContext {
                ast_annotation: annotations,
                index,
                qualifier: Some(i.name.as_str()),
            };
            if i.pou_type == PouType::Action && i.type_name == "__unknown__" {
                self.pou_validator
                    .diagnostics
                    .push(Diagnostic::missing_action_container(i.location.clone()));
            }
            i.statements
                .iter()
                .for_each(|s| self.visit_statement(s, &context));
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

    pub fn visit_variable_container(
        &mut self,
        context: &ValidationContext,
        container: &VariableBlock,
    ) {
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
        self.variable_validator
            .validate_data_type_declaration(declaration);

        if let DataTypeDeclaration::DataTypeDefinition {
            data_type,
            location,
            ..
        } = declaration
        {
            self.visit_data_type(context, data_type, location);
        }
    }

    pub fn visit_data_type(
        &mut self,
        context: &ValidationContext,
        data_type: &DataType,
        location: &SourceRange,
    ) {
        self.variable_validator
            .validate_data_type(data_type, location);

        match data_type {
            DataType::StructType { variables, .. } => variables
                .iter()
                .for_each(|v| self.visit_variable(context, v)),
            DataType::ArrayType {
                referenced_type, ..
            } => self.visit_data_type_declaration(context, referenced_type),
            DataType::VarArgs {
                referenced_type: Some(referenced_type),
                ..
            } => {
                self.visit_data_type_declaration(context, referenced_type.as_ref());
            }
            _ => {}
        }
    }

    pub fn visit_statement(&mut self, statement: &AstStatement, context: &ValidationContext) {
        match statement {
            AstStatement::LiteralArray {
                elements: Some(elements),
                ..
            } => self.visit_statement(elements.as_ref(), context),
            AstStatement::MultipliedStatement { element, .. } => {
                self.visit_statement(element, context)
            }
            AstStatement::QualifiedReference { elements, .. } => elements
                .iter()
                .for_each(|e| self.visit_statement(e, context)),
            AstStatement::ArrayAccess {
                reference, access, ..
            } => {
                visit_all_statements!(self, context, reference, access);
            }
            AstStatement::BinaryExpression { left, right, .. } => {
                visit_all_statements!(self, context, left, right);
            }
            AstStatement::UnaryExpression { value, .. } => self.visit_statement(value, context),
            AstStatement::ExpressionList { expressions, .. } => expressions
                .iter()
                .for_each(|e| self.visit_statement(e, context)),
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
            AstStatement::CallStatement {
                parameters,
                operator,
                ..
            } => {
                // visit called pou
                self.visit_statement(operator, context);

                // for PROGRAM/FB we need special inout validation
                if let Some(PouIndexEntry::FunctionBlock { name, .. })
                | Some(PouIndexEntry::Program { name, .. }) = context.find_pou(operator)
                {
                    let declared_parameters = context.index.get_declared_parameters(name);
                    let inouts: Vec<&&VariableIndexEntry> = declared_parameters
                        .iter()
                        .filter(|p| VariableType::InOut == p.get_variable_type())
                        .collect();
                    // if the called pou has declared inouts, we need to make sure that these were passed to the pou call
                    if !inouts.is_empty() {
                        let mut passed_params_idx = Vec::new();
                        if let Some(s) = parameters.as_ref() {
                            match s {
                                AstStatement::ExpressionList { expressions, .. } => {
                                    for (i, e) in expressions.iter().enumerate() {
                                        // safe index of passed parameter
                                        if let Ok((idx, _)) =
                                            get_implicit_call_parameter(e, &declared_parameters, i)
                                        {
                                            passed_params_idx.push(idx);
                                        }
                                    }
                                }
                                _ => {
                                    // safe index of passed parameter
                                    if let Ok((idx, _)) =
                                        get_implicit_call_parameter(s, &declared_parameters, 0)
                                    {
                                        passed_params_idx.push(idx);
                                    }
                                }
                            }
                        }
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
                if let Some(s) = parameters.as_ref() {
                    self.visit_statement(s, context);
                }
            }
            AstStatement::IfStatement {
                blocks, else_block, ..
            } => {
                blocks.iter().for_each(|b| {
                    self.visit_statement(b.condition.as_ref(), context);
                    b.body.iter().for_each(|s| self.visit_statement(s, context));
                });
                else_block
                    .iter()
                    .for_each(|e| self.visit_statement(e, context));
            }
            AstStatement::ForLoopStatement {
                counter,
                start,
                end,
                by_step,
                body,
                ..
            } => {
                visit_all_statements!(self, context, counter, start, end);
                if let Some(by_step) = by_step {
                    self.visit_statement(by_step, context);
                }
                body.iter().for_each(|s| self.visit_statement(s, context));
            }
            AstStatement::WhileLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(condition, context);
                body.iter().for_each(|s| self.visit_statement(s, context));
            }
            AstStatement::RepeatLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(condition, context);
                body.iter().for_each(|s| self.visit_statement(s, context));
            }
            AstStatement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => {
                self.visit_statement(selector, context);

                let mut cases = HashSet::new();
                case_blocks.iter().for_each(|b| {
                    let condition = b.condition.as_ref();

                    // validate for duplicate conditions
                    // first try to evaluate the conditions value
                    const_evaluator::evaluate(condition, context.qualifier, context.index)
                        .map_err(|err| {
                            // value evaluation and validation not possible with non constants
                            self.stmt_validator.diagnostics.push(
                                Diagnostic::non_constant_case_condition(
                                    &err,
                                    condition.get_location(),
                                ),
                            )
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

                else_block
                    .iter()
                    .for_each(|s| self.visit_statement(s, context));
            }
            AstStatement::CaseCondition { condition, .. } => {
                self.visit_statement(condition, context)
            }
            _ => {}
        }

        self.stmt_validator.validate_statement(statement, context);
    }
}
