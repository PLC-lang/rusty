use std::slice::Iter;

use crate::{
    ast::{
        CompilationUnit, DataType, DataTypeDeclaration, Pou, Statement, Variable, VariableBlock,
    },
    index::Index,
    Diagnostic,
};

use self::{pou_validator::PouValidator, stmt_validator::StatementValidator, variable_validator::VariableValidator};

mod pou_validator;
mod variable_validator;
mod stmt_validator;

macro_rules! visit_all_statements {
     ($self:expr, $last:expr ) => {
         $self.visit_statement($last);
     };

     ($self:expr, $head:expr, $($tail:expr), +) => {
       $self.visit_statement($head);
       visit_all_statements!($self, $($tail),+)
     };
   }

pub struct ValidationContext<'s> {
    diagnostic: Vec<Diagnostic>,
    current_qualifier: Option<&'s str>,
}

impl<'s> ValidationContext<'s> {
    fn report(&mut self, diagnostic: Diagnostic) {
        self.diagnostic.push(diagnostic);
    }

    fn get_qualifier(&self) -> Option<&'s str> {
        self.current_qualifier
    }
}

pub struct Validator<'i, 's> {
    context: ValidationContext<'s>,
    pou_validator: PouValidator,
    variable_validator: VariableValidator<'i>,
    stmt_validator: StatementValidator<'i>,
}

impl<'i, 's> Validator<'i, 's> {
    pub fn new(idx: &'i Index) -> Validator {
        Validator {
            context: ValidationContext{
                current_qualifier: None,
                diagnostic: Vec::new(),
            },
            pou_validator: PouValidator::new(),
            variable_validator: VariableValidator::new(idx),
            stmt_validator: StatementValidator::new(idx),
        }
    }

    pub fn diagnostics(&self) -> Iter<Diagnostic> {
        self.context.diagnostic.iter()
    }

    pub fn visit_unit(&mut self, unit: &'s CompilationUnit) {
        for pou in &unit.units {
            self.visit_pou(pou);
        }

        for i in &unit.implementations {
            i.statements.iter().for_each(|s| self.visit_statement(s));
        }
    }

    pub fn visit_pou(&mut self, pou: &'s Pou) {
        self.context.current_qualifier = Some(pou.name.as_str());
        self.pou_validator.validate_pou(pou, &mut self.context);

        for block in &pou.variable_blocks {
            self.visit_variable_container(block);
        }
    }

    pub fn visit_variable_container(&mut self, container: &VariableBlock) {
        self.variable_validator
            .validate_variable_block(container, &mut self.context);

        for variable in &container.variables {
            self.visit_variable(variable);
        }
    }

    pub fn visit_variable(&mut self, variable: &Variable) {
        self.variable_validator
            .validate_variable(variable, &mut self.context);

        self.visit_data_type_declaration(&variable.data_type);
    }

    pub fn visit_data_type_declaration(&mut self, declaration: &DataTypeDeclaration) {
        self.variable_validator
            .validate_data_type_declaration(declaration, &mut self.context);

        if let DataTypeDeclaration::DataTypeDefinition { data_type, .. } = declaration {
            self.visit_data_type(data_type);
        }
    }

    pub fn visit_data_type(&mut self, data_type: &DataType) {
        self.variable_validator
            .validate_data_type(data_type, &mut self.context);

        match data_type {
            DataType::StructType { variables, .. } => {
                variables.iter().for_each(|v| self.visit_variable(v))
            }
            DataType::ArrayType {
                referenced_type, ..
            } => self.visit_data_type_declaration(referenced_type),
            DataType::VarArgs {
                referenced_type: Some(referenced_type),
            } => {
                self.visit_data_type_declaration(referenced_type.as_ref());
            }
            _ => {}
        }
    }

    pub fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::LiteralArray {
                elements: Some(elements),
                ..
            } => self.visit_statement(elements.as_ref()),
            Statement::MultipliedStatement {
                element,
                ..
            } => self.visit_statement(element),
            Statement::QualifiedReference { elements } => {
                elements.iter().for_each(|e| self.visit_statement(e))
            }
            Statement::ArrayAccess { reference, access } => {
                visit_all_statements!(self, reference, access);
            }
            Statement::BinaryExpression {
                left,
                right,
                ..
            } => {
                visit_all_statements!(self, left, right);
            }
            Statement::UnaryExpression {
                value,
                ..
            } => self.visit_statement(value),
            Statement::ExpressionList { expressions } => {
                expressions.iter().for_each(|e| self.visit_statement(e))
            }
            Statement::RangeStatement { start, end } => {
                visit_all_statements!(self, start, end);
            }
            Statement::Assignment { left, right } => {
                self.visit_statement(left);
                self.visit_statement(right);
            }
            Statement::OutputAssignment { left, right } => {
                self.visit_statement(left);
                self.visit_statement(right);
            }
            Statement::CallStatement {
                parameters,
                ..
            } => {
                if let Some(s) = parameters.as_ref() {
                    self.visit_statement(s);
                }
            }
            Statement::IfStatement {
                blocks,
                else_block,
                ..
            } => {
                blocks.iter().for_each(|b| {
                    self.visit_statement(b.condition.as_ref());
                    b.body.iter().for_each(|s| self.visit_statement(s));
                });
                else_block.iter().for_each(|e| self.visit_statement(e));
            }
            Statement::ForLoopStatement {
                counter,
                start,
                end,
                by_step,
                body,
                ..
            } => {
                visit_all_statements!(self, counter, start, end);
                if let Some(by_step) = by_step {
                    self.visit_statement(by_step);
                }
                body.iter().for_each(|s| self.visit_statement(s));
            }
            Statement::WhileLoopStatement {
                condition,
                body,
                ..
            } => {
                self.visit_statement(condition);
                body.iter().for_each(|s| self.visit_statement(s));
            }
            Statement::RepeatLoopStatement {
                condition,
                body,
                ..
            } => {
                self.visit_statement(condition);
                body.iter().for_each(|s| self.visit_statement(s));
            }
            Statement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => {
                self.visit_statement(selector);
                case_blocks.iter().for_each(|b| {
                    self.visit_statement(b.condition.as_ref());
                    b.body.iter().for_each(|s| self.visit_statement(s));
                });
                else_block.iter().for_each(|s| self.visit_statement(s));
            }
            Statement::CaseCondition { condition } => self.visit_statement(condition),
            _ => {}
        }

        self.stmt_validator.validate_statement(statement, &mut self.context);
    }
}
