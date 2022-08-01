use crate::{
    ast::{
        AstStatement, CompilationUnit, DataType, DataTypeDeclaration, Pou, SourceRange,
        UserTypeDeclaration, Variable, VariableBlock,
    },
    index::Index,
    resolver::AnnotationMapImpl,
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
                self.visit_statement(operator, context);
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
                case_blocks.iter().for_each(|b| {
                    self.visit_statement(b.condition.as_ref(), context);
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
