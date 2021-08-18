use crate::{
    ast::{
        AstStatement, CompilationUnit, DataType, DataTypeDeclaration, Pou, SourceRange,
        UserTypeDeclaration, Variable, VariableBlock,
    },
    resolver::AnnotationMap,
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
         $self.visit_statement($context, $last);
     };

     ($self:expr, $context:expr, $head:expr, $($tail:expr), +) => {
       $self.visit_statement($context, $head);
       visit_all_statements!($self, $context, $($tail),+)
     };
   }

pub struct ValidationContext<'s> {
    ast_annotation: &'s AnnotationMap,
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

    pub fn visit_unit(&mut self, annotations: &AnnotationMap, unit: &CompilationUnit) {
        let context = ValidationContext {
            ast_annotation: annotations,
        };

        for pou in &unit.units {
            self.visit_pou(&context, pou);
        }

        for t in &unit.types {
            self.visit_user_type_declaration(&context, t);
        }

        for i in &unit.implementations {
            i.statements
                .iter()
                .for_each(|s| self.visit_statement(&context, s));
        }
    }

    pub fn visit_user_type_declaration(
        &mut self,
        _context: &ValidationContext,
        user_data_type: &UserTypeDeclaration,
    ) {
        self.variable_validator
            .validate_data_type(&user_data_type.data_type, &user_data_type.location);
    }

    pub fn visit_pou(&mut self, context: &ValidationContext, pou: &Pou) {
        self.pou_validator.validate_pou(pou);

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
        self.variable_validator.validate_variable(variable);

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
            } => {
                self.visit_data_type_declaration(context, referenced_type.as_ref());
            }
            _ => {}
        }
    }

    pub fn visit_statement(&mut self, context: &ValidationContext, statement: &AstStatement) {
        match statement {
            AstStatement::LiteralArray {
                elements: Some(elements),
                ..
            } => self.visit_statement(context, elements.as_ref()),
            AstStatement::MultipliedStatement { element, .. } => {
                self.visit_statement(context, element)
            }
            AstStatement::QualifiedReference { elements, .. } => elements
                .iter()
                .for_each(|e| self.visit_statement(context, e)),
            AstStatement::ArrayAccess {
                reference, access, ..
            } => {
                visit_all_statements!(self, context, reference, access);
            }
            AstStatement::BinaryExpression { left, right, .. } => {
                visit_all_statements!(self, context, left, right);
            }
            AstStatement::UnaryExpression { value, .. } => self.visit_statement(context, value),
            AstStatement::ExpressionList { expressions, .. } => expressions
                .iter()
                .for_each(|e| self.visit_statement(context, e)),
            AstStatement::RangeStatement { start, end, .. } => {
                visit_all_statements!(self, context, start, end);
            }
            AstStatement::Assignment { left, right, .. } => {
                self.visit_statement(context, left);
                self.visit_statement(context, right);
            }
            AstStatement::OutputAssignment { left, right, .. } => {
                self.visit_statement(context, left);
                self.visit_statement(context, right);
            }
            AstStatement::CallStatement {
                parameters,
                operator,
                ..
            } => {
                self.visit_statement(context, operator);
                if let Some(s) = parameters.as_ref() {
                    self.visit_statement(context, s);
                }
            }
            AstStatement::IfStatement {
                blocks, else_block, ..
            } => {
                blocks.iter().for_each(|b| {
                    self.visit_statement(context, b.condition.as_ref());
                    b.body.iter().for_each(|s| self.visit_statement(context, s));
                });
                else_block
                    .iter()
                    .for_each(|e| self.visit_statement(context, e));
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
                    self.visit_statement(context, by_step);
                }
                body.iter().for_each(|s| self.visit_statement(context, s));
            }
            AstStatement::WhileLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(context, condition);
                body.iter().for_each(|s| self.visit_statement(context, s));
            }
            AstStatement::RepeatLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(context, condition);
                body.iter().for_each(|s| self.visit_statement(context, s));
            }
            AstStatement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => {
                self.visit_statement(context, selector);
                case_blocks.iter().for_each(|b| {
                    self.visit_statement(context, b.condition.as_ref());
                    b.body.iter().for_each(|s| self.visit_statement(context, s));
                });
                else_block
                    .iter()
                    .for_each(|s| self.visit_statement(context, s));
            }
            AstStatement::CaseCondition { condition, .. } => {
                self.visit_statement(context, condition)
            }
            _ => {}
        }

        self.stmt_validator.validate_statement(statement, context);
    }
}
