// Copyright (c) 2021 Ghaith Hachem and Mathias Rieder

//! Resolves (partial) expressions & statements and annotates the resulting types
//!
//! Recursively visits all statements and expressions of a `CompilationUnit` and
//! records all resulting types associated with the statement's id.

use indexmap::IndexMap;

use crate::{
    ast::{
        AstId, CompilationUnit, DataType, DataTypeDeclaration, Operator, Pou, Statement,
        UserTypeDeclaration, Variable,
    },
    index::{ImplementationType, Index},
    typesystem::{
        self, get_bigger_type_borrow, DataTypeInformation, BOOL_TYPE, DATE_AND_TIME_TYPE,
        DATE_TYPE, DINT_TYPE, LINT_TYPE, REAL_TYPE, STRING_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE,
        VOID_TYPE,
    },
};

#[cfg(test)]
mod tests;

/// helper macro that calls visit_statement for all given statements
/// use like `visit_all_statements!(self, ctx, stmt1, stmt2, stmt3, ...)`
macro_rules! visit_all_statements {
     ($self:expr, $ctx:expr, $last:expr ) => {
         $self.visit_statement($ctx, $last);
     };

     ($self:expr, $ctx:expr, $head:expr, $($tail:expr), +) => {
       $self.visit_statement($ctx, $head);
       visit_all_statements!($self, $ctx, $($tail),+)
     };
   }

/// Context object passed by the visitor
/// Denotes the current context of expressions (e.g. the current pou, a defined context, etc.)
///
/// Helper methods `qualifier`, `current_pou` and `lhs_pou` copy the current context and
/// change one field.
#[derive(Clone)]
struct VisitorContext<'s> {
    /// the type_name of the context for a reference (e.g. `a.b` where `a`'s type is the context of `b`)
    qualifier: Option<String>,
    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU`'s body)
    pou: Option<&'s str>,
    /// special context of the left-hand-side of an assignment in call statements
    /// Inside the left hand side of an assignment is in the context of the call's POU
    /// `foo(a := a)` actually means: `foo(foo.a := POU.a)`
    call: Option<&'s str>,
}

impl<'s> VisitorContext<'s> {
    /// returns a copy of the current context and changes the `current_qualifier` to the given qualifier
    fn with_qualifier(&self, qualifier: String) -> VisitorContext<'s> {
        VisitorContext {
            pou: self.pou,
            qualifier: Some(qualifier),
            call: self.call,
        }
    }

    /// returns a copy of the current context and changes the `current_pou` to the given pou
    fn with_pou(&self, pou: &'s str) -> VisitorContext<'s> {
        VisitorContext {
            pou: Some(pou),
            qualifier: self.qualifier.clone(),
            call: self.call,
        }
    }

    /// returns a copy of the current context and changes the `lhs_pou` to the given pou
    fn with_call(&self, lhs_pou: &'s str) -> VisitorContext<'s> {
        VisitorContext {
            pou: self.pou,
            qualifier: self.qualifier.clone(),
            call: Some(lhs_pou),
        }
    }
}

pub struct TypeAnnotator<'i> {
    index: &'i Index,
    annotation_map: AnnotationMap,
    //context: VisitorContext<'i>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementAnnotation {
    /// an expression that resolves to a certain type (e.g. `a + b` --> `INT`)
    ExpressionAnnotation { resulting_type: String },
    /// a reference that resolves to a declared variable (e.g. `a` --> `PLC_PROGRAM.a`)
    VariableAnnotation {
        resulting_type: String,
        qualified_name: String,
    },
    /// a reference to a function
    FunctionAnnotation {
        return_type: String,
        qualified_name: String,
    },
    /// a reference to a type (e.g. `INT`)
    TypeAnnotation { type_name: String },
    /// a reference to a program call or reference (e.g. `PLC_PRG`)
    ProgramAnnotation { qualified_name: String },
}

impl StatementAnnotation {
    fn expression(type_name: &str) -> StatementAnnotation {
        StatementAnnotation::ExpressionAnnotation {
            resulting_type: type_name.to_string(),
        }
    }
}

pub struct AnnotationMap {
    /// maps a statement to the type it resolves to
    type_map: IndexMap<AstId, StatementAnnotation>,
}

impl AnnotationMap {
    /// creates a new empty AnnotationMap
    pub fn new() -> AnnotationMap {
        AnnotationMap {
            type_map: IndexMap::new(),
        }
    }

    /// annotates the given statement (using it's `get_id()`) with the given type-name
    pub fn annotate(&mut self, s: &Statement, annotation: StatementAnnotation) {
        self.type_map.insert(s.get_id(), annotation);
    }

    pub fn get(&self, s: &Statement) -> Option<&StatementAnnotation> {
        self.type_map.get(&s.get_id())
    }

    /// returns the annotated type or void if none was annotated
    pub fn get_type_or_void<'i>(
        &self,
        s: &Statement,
        index: &'i Index,
    ) -> &'i typesystem::DataType {
        self.get_type(s, index)
            .unwrap_or_else(|| index.get_void_type())
    }

    /// returns the annotated type - for now only used by test
    pub fn get_type<'i>(
        &self,
        s: &Statement,
        index: &'i Index,
    ) -> Option<&'i typesystem::DataType> {
        self.get_annotation(s)
            .and_then(|annotation| match annotation {
                StatementAnnotation::ExpressionAnnotation { resulting_type } => {
                    Some(resulting_type.as_str())
                }
                StatementAnnotation::VariableAnnotation { resulting_type, .. } => {
                    Some(resulting_type.as_str())
                }
                StatementAnnotation::FunctionAnnotation { .. } => None,
                StatementAnnotation::TypeAnnotation { .. } => None,
                StatementAnnotation::ProgramAnnotation { .. } => None,
            })
            .and_then(|type_name| index.get_type(type_name).ok())
    }

    /// returns the annotation of the given statement or none if it was not annotated
    pub fn get_annotation(&self, s: &Statement) -> Option<&StatementAnnotation> {
        self.type_map.get(&s.get_id())
    }

    pub fn has_type_annotation(&self, id: &usize) -> bool {
        self.type_map.contains_key(id)
    }
}

impl<'i> TypeAnnotator<'i> {
    /// constructs a new TypeAnnotater that works with the given index for type-lookups
    fn new(index: &'i Index) -> TypeAnnotator<'i> {
        TypeAnnotator {
            annotation_map: AnnotationMap::new(),
            index,
        }
    }

    /// annotates the given AST elements with the type-name resulting for the statements/expressions.
    /// Returns an AnnotationMap with the resulting types for all visited Statements. See `AnnotationMap`
    pub fn visit_unit(index: &Index, unit: &'i CompilationUnit) -> AnnotationMap {
        let mut visitor = TypeAnnotator::new(index);

        let ctx = &VisitorContext {
            pou: None,
            qualifier: None,
            call: None,
        };

        for pou in &unit.units {
            visitor.visit_pou(ctx, pou);
        }

        for t in &unit.types {
            visitor.visit_user_type_declaration(t, ctx);
        }

        for i in &unit.implementations {
            i.statements
                .iter()
                .for_each(|s| visitor.visit_statement(&ctx.with_pou(i.name.as_str()), s));
        }
        visitor.annotation_map
    }

    fn visit_user_type_declaration(
        &mut self,
        user_data_type: &UserTypeDeclaration,
        ctx: &VisitorContext,
    ) {
        self.visit_data_type(ctx, &user_data_type.data_type);
        if let Some(initializer) = &user_data_type.initializer {
            self.visit_statement(ctx, &initializer);
        }
    }

    fn visit_pou(&mut self, ctx: &VisitorContext, pou: &'i Pou) {
        let pou_ctx = ctx.with_pou(pou.name.as_str());
        for block in &pou.variable_blocks {
            for variable in &block.variables {
                self.visit_variable(&pou_ctx, variable);
            }
        }
    }

    fn visit_variable(&mut self, ctx: &VisitorContext, variable: &Variable) {
        self.visit_data_type_declaration(ctx, &variable.data_type);
    }

    fn visit_data_type_declaration(
        &mut self,
        ctx: &VisitorContext,
        declaration: &DataTypeDeclaration,
    ) {
        if let DataTypeDeclaration::DataTypeDefinition { data_type, .. } = declaration {
            self.visit_data_type(ctx, data_type);
        }
    }

    fn visit_data_type(&mut self, ctx: &VisitorContext, data_type: &DataType) {
        match data_type {
            DataType::StructType { variables, .. } => {
                variables.iter().for_each(|v| self.visit_variable(ctx, v))
            }
            DataType::ArrayType {
                referenced_type, ..
            } => self.visit_data_type_declaration(ctx, referenced_type),
            DataType::VarArgs {
                referenced_type: Some(referenced_type),
            } => {
                self.visit_data_type_declaration(ctx, referenced_type.as_ref());
            }
            _ => {}
        }
    }

    fn visit_statement(&mut self, ctx: &VisitorContext, statement: &Statement) {
        self.visit_statement_control(ctx, statement);
    }

    /// annotate a control statement
    fn visit_statement_control(&mut self, ctx: &VisitorContext, statement: &Statement) {
        match statement {
            Statement::IfStatement {
                blocks, else_block, ..
            } => {
                blocks.iter().for_each(|b| {
                    self.visit_statement(ctx, b.condition.as_ref());
                    b.body.iter().for_each(|s| self.visit_statement(ctx, s));
                });
                else_block.iter().for_each(|e| self.visit_statement(ctx, e));
            }
            Statement::ForLoopStatement {
                counter,
                start,
                end,
                by_step,
                body,
                ..
            } => {
                visit_all_statements!(self, ctx, counter, start, end);
                if let Some(by_step) = by_step {
                    self.visit_statement(ctx, by_step);
                }
                body.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            Statement::WhileLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(ctx, condition);
                body.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            Statement::RepeatLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(ctx, condition);
                body.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            Statement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => {
                self.visit_statement(ctx, selector);
                case_blocks.iter().for_each(|b| {
                    self.visit_statement(ctx, b.condition.as_ref());
                    b.body.iter().for_each(|s| self.visit_statement(ctx, s));
                });
                else_block.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            Statement::CaseCondition { condition, .. } => self.visit_statement(ctx, condition),
            _ => {
                self.visit_statement_expression(ctx, statement);
            }
        }
    }

    /// annotate an expression statement
    fn visit_statement_expression(&mut self, ctx: &VisitorContext, statement: &Statement) {
        match statement {
            Statement::ArrayAccess {
                reference, access, ..
            } => {
                visit_all_statements!(self, ctx, reference, access);
                let array_type = self
                    .annotation_map
                    .get_type_or_void(reference, self.index)
                    .get_type_information();
                if let DataTypeInformation::Array {
                    inner_type_name, ..
                } = array_type
                {
                    let t = self
                        .index
                        .get_effective_type_by_name(inner_type_name)
                        .get_name();

                    self.annotation_map
                        .annotate(statement, StatementAnnotation::expression(t));
                }
            }
            Statement::BinaryExpression { left, right, .. } => {
                visit_all_statements!(self, ctx, left, right);
                let left = &self
                    .annotation_map
                    .get_type_or_void(left, self.index)
                    .get_type_information();
                let right = &self
                    .annotation_map
                    .get_type_or_void(right, self.index)
                    .get_type_information();

                if left.is_numerical() && right.is_numerical() {
                    let bigger_name = get_bigger_type_borrow(left, right, self.index).get_name();
                    self.annotation_map
                        .annotate(statement, StatementAnnotation::expression(bigger_name));
                }
            }
            Statement::UnaryExpression {
                value, operator, ..
            } => {
                self.visit_statement(ctx, value);
                let inner_type = self
                    .annotation_map
                    .get_type_or_void(value, self.index)
                    .get_type_information();
                if operator == &Operator::Minus {
                    //keep the same type but switch to signed
                    if let Some(target) = typesystem::get_signed_type(inner_type, self.index) {
                        self.annotation_map.annotate(
                            statement,
                            StatementAnnotation::expression(target.get_name()),
                        );
                    }
                } else {
                    self.annotation_map.annotate(
                        statement,
                        StatementAnnotation::expression(inner_type.get_name()),
                    );
                }
            }
            Statement::Reference { name, .. } => {
                let qualifier = ctx.qualifier.as_deref().or(ctx.pou);

                let annotation = qualifier
                    .and_then(|pou| {
                        self.index.find_member(pou, name).map(|v| {
                            StatementAnnotation::VariableAnnotation {
                                qualified_name: v.get_qualified_name().into(),
                                resulting_type: self
                                    .index
                                    .get_effective_type_by_name(v.get_type_name())
                                    .get_name()
                                    .into(),
                            }
                        })
                    })
                    .or_else(|| {
                        self.index.find_implementation(name).and_then(|it| {
                            match it.get_implementation_type() {
                                crate::index::ImplementationType::Program => {
                                    Some(StatementAnnotation::ProgramAnnotation {
                                        qualified_name: it.get_call_name().into(),
                                    })
                                }
                                crate::index::ImplementationType::Function => {
                                    Some(StatementAnnotation::FunctionAnnotation {
                                        qualified_name: it.get_call_name().into(),
                                        return_type: self
                                            .index
                                            .find_return_type(it.get_call_name())
                                            //.and_then(|it|
                                            //    self.index.find_effective_type_by_name(it.get_name()))
                                            .map(|it| it.get_name())
                                            .unwrap_or(VOID_TYPE)
                                            .into(),
                                    })
                                }
                                crate::index::ImplementationType::FunctionBlock => {
                                    Some(StatementAnnotation::TypeAnnotation {
                                        type_name: name.into(),
                                    })
                                }
                                _ => None,
                            }
                        })
                    })
                    .or_else(|| {
                        self.index.find_global_variable(name).map(|v| {
                            StatementAnnotation::VariableAnnotation {
                                qualified_name: name.into(),
                                resulting_type: self
                                    .index
                                    .get_effective_type_by_name(v.get_type_name())
                                    .get_name()
                                    .into(),
                            }
                        })
                    });

                if let Some(annotation) = annotation {
                    self.annotation_map.annotate(statement, annotation)
                }
            }
            Statement::QualifiedReference { elements, .. } => {
                let mut ctx = ctx.clone();
                for s in elements.iter() {
                    self.visit_statement(&ctx, s);

                    let qualifier = self
                        .annotation_map
                        .get_annotation(s)
                        .map(|it| match it {
                            StatementAnnotation::ExpressionAnnotation { resulting_type } => {
                                resulting_type.as_str()
                            }
                            StatementAnnotation::VariableAnnotation { resulting_type, .. } => {
                                resulting_type.as_str()
                            }
                            StatementAnnotation::FunctionAnnotation { .. } => VOID_TYPE,
                            StatementAnnotation::TypeAnnotation { type_name } => type_name.as_str(),
                            StatementAnnotation::ProgramAnnotation { qualified_name } => {
                                qualified_name.as_str()
                            }
                        })
                        .unwrap_or_else(|| VOID_TYPE);

                    ctx = ctx.with_qualifier(qualifier.to_string());
                }

                //the last guy represents the type of the whole qualified expression
                if let Some(last) = elements.last() {
                    if let Some(annotation) = self.annotation_map.get_annotation(last).cloned() {
                        self.annotation_map.annotate(statement, annotation);
                    }
                }
            }
            Statement::ExpressionList { expressions, .. } => expressions
                .iter()
                .for_each(|e| self.visit_statement(ctx, e)),
            Statement::RangeStatement { start, end, .. } => {
                visit_all_statements!(self, ctx, start, end);
            }
            Statement::Assignment { left, right, .. } => {
                self.visit_statement(ctx, right);
                if let Some(lhs) = ctx.call {
                    //special context for left hand side
                    self.visit_statement(&ctx.with_pou(lhs), left);
                } else {
                    self.visit_statement(ctx, left);
                }
            }
            Statement::OutputAssignment { left, right, .. } => {
                visit_all_statements!(self, ctx, left, right);
                if let Some(lhs) = ctx.call {
                    //special context for left hand side
                    self.visit_statement(&ctx.with_pou(lhs), left);
                } else {
                    self.visit_statement(ctx, left);
                }
            }
            Statement::CallStatement {
                parameters,
                operator,
                ..
            } => {
                self.visit_statement(ctx, operator);
                if let Some(s) = parameters.as_ref() {
                    let operator_qualifier = self
                        .annotation_map
                        .get_annotation(operator)
                        .and_then(|it| match it {
                            StatementAnnotation::FunctionAnnotation { qualified_name, .. } => {
                                Some(qualified_name.clone())
                            }
                            StatementAnnotation::ProgramAnnotation { qualified_name } => {
                                Some(qualified_name.clone())
                            }
                            StatementAnnotation::VariableAnnotation { resulting_type, .. } => {
                                //lets see if this is a FB
                                if let Some(implementation) =
                                    self.index.find_implementation(resulting_type.as_str())
                                {
                                    if let ImplementationType::FunctionBlock {} =
                                        implementation.get_implementation_type()
                                    {
                                        return Some(resulting_type.clone());
                                    }
                                }
                                None
                            }
                            _ => {
                                println!("{:#?}", it);
                                None
                            }
                        })
                        .unwrap_or_else(|| VOID_TYPE.to_string());
                    let ctx = ctx.with_call(operator_qualifier.as_str());
                    //need to clone the qualifier string because of borrow checker :-( - //todo look into this
                    self.visit_statement(&ctx, s);
                }

                if let Some(StatementAnnotation::FunctionAnnotation { return_type, .. }) =
                    self.annotation_map.get(operator)
                {
                    if let Some(return_type) = self
                        .index
                        .find_type(return_type)
                        .and_then(|it| self.index.find_effective_type(it))
                        .map(|it| it.get_name())
                    {
                        self.annotation_map
                            .annotate(statement, StatementAnnotation::expression(return_type));
                    }
                }
            }
            _ => {
                self.visit_statement_literals(ctx, statement);
            }
        }
    }

    /// annotate a literal statement
    fn visit_statement_literals(&mut self, ctx: &VisitorContext, statement: &Statement) {
        match statement {
            Statement::LiteralBool { .. } => {
                self.annotation_map
                    .annotate(statement, StatementAnnotation::expression(BOOL_TYPE));
            }

            Statement::LiteralString { .. } => {
                self.annotation_map
                    .annotate(statement, StatementAnnotation::expression(STRING_TYPE));
            }
            Statement::LiteralInteger { value, .. } => {
                self.annotation_map.annotate(
                    statement,
                    StatementAnnotation::expression(get_int_type_name_for(*value)),
                );
            }
            Statement::LiteralTime { .. } => self
                .annotation_map
                .annotate(statement, StatementAnnotation::expression(TIME_TYPE)),
            Statement::LiteralTimeOfDay { .. } => {
                self.annotation_map
                    .annotate(statement, StatementAnnotation::expression(TIME_OF_DAY_TYPE));
            }
            Statement::LiteralDate { .. } => {
                self.annotation_map
                    .annotate(statement, StatementAnnotation::expression(DATE_TYPE));
            }
            Statement::LiteralDateAndTime { .. } => {
                self.annotation_map.annotate(
                    statement,
                    StatementAnnotation::expression(DATE_AND_TIME_TYPE),
                );
            }
            Statement::LiteralReal { .. } => {
                //TODO when do we need a LREAL literal?
                self.annotation_map
                    .annotate(statement, StatementAnnotation::expression(REAL_TYPE));
            }
            Statement::LiteralArray {
                elements: Some(elements),
                ..
            } => {
                self.visit_statement(ctx, elements.as_ref());
                //TODO as of yet we have no way to derive a name that reflects a fixed size array
            }
            Statement::MultipliedStatement { element, .. } => {
                self.visit_statement(ctx, element)
                //TODO as of yet we have no way to derive a name that reflects a fixed size array
            }
            _ => {}
        }
    }
}

fn get_int_type_name_for(value: i64) -> &'static str {
    if i32::MIN as i64 <= value && i32::MAX as i64 >= value {
        DINT_TYPE
    } else {
        LINT_TYPE
    }
}

#[cfg(test)]
mod resolver_tests {
    use super::get_int_type_name_for;

    #[test]
    fn correct_int_types_name_for_numbers() {
        assert_eq!(get_int_type_name_for(0), "DINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 8) - 1), "DINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 8)), "DINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 16) - 1), "DINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 16)), "DINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 31) - 1), "DINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 31)), "LINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 32)), "LINT");
        assert_eq!(get_int_type_name_for(i64::MAX), "LINT");
    }
}
