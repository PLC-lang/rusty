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
    index::Index,
    typesystem::{
        self, get_bigger_type_borrow, DataTypeInformation, BOOL_TYPE, BYTE_TYPE,
        DATE_AND_TIME_TYPE, DATE_TYPE, REAL_TYPE, STRING_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE,
        UDINT_TYPE, UINT_TYPE, ULINT_TYPE,
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
    qualifier: Option<&'s str>,
    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU`'s body)
    pou: Option<&'s str>,
    /// special context of the left-hand-side of an assignment in call statements
    /// Inside the left hand side of an assignment is in the context of the call's POU
    /// `foo(a := a)` actually means: `foo(foo.a := POU.a)`
    call: Option<&'s str>,
}

impl<'s> VisitorContext<'s> {
    /// returns a copy of the current context and changes the `current_qualifier` to the given qualifier
    fn with_qualifier(&self, qualifier: &'s str) -> VisitorContext<'s> {
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
            qualifier: self.qualifier,
            call: self.call,
        }
    }

    /// returns a copy of the current context and changes the `lhs_pou` to the given pou
    fn with_call(&self, lhs_pou: &'s str) -> VisitorContext<'s> {
        VisitorContext {
            pou: self.pou,
            qualifier: self.qualifier,
            call: Some(lhs_pou),
        }
    }
}

pub struct TypeAnnotator<'i> {
    index: &'i Index,
    annotation_map: AnnotationMap,
    //context: VisitorContext<'i>,
}

pub struct AnnotationMap {
    //TODO try to only borrow names?
    type_map: IndexMap<AstId, String>, // Statement -> type-name
}

impl AnnotationMap {
    /// creates a new empty AnnotationMap
    pub fn new() -> AnnotationMap {
        AnnotationMap {
            type_map: IndexMap::new(),
        }
    }

    /// annotates the given statement (using it's `get_id()`) with the given type-name
    pub fn annotate_type(&mut self, s: &Statement, type_name: &str) {
        self.type_map.insert(s.get_id(), type_name.to_string());
    }

    /// returns the annotated type or void if none was annotated
    pub fn get_type<'i>(&self, s: &Statement, index: &'i Index) -> &'i typesystem::DataType {
        self.type_map
            .get(&s.get_id())
            .and_then(|name| index.get_type(name).ok())
            .unwrap_or_else(|| index.get_void_type())
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
        if let DataTypeDeclaration::DataTypeDefinition { data_type } = declaration {
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
                    .get_type(reference, self.index)
                    .get_type_information();
                if let DataTypeInformation::Array {
                    inner_type_name, ..
                } = array_type
                {
                    let t = self
                        .index
                        .get_effective_type_by_name(inner_type_name)
                        .get_name();
                    self.annotation_map.annotate_type(statement, t);
                }
            }
            Statement::BinaryExpression { left, right, .. } => {
                visit_all_statements!(self, ctx, left, right);
                let left = &self
                    .annotation_map
                    .get_type(left, self.index)
                    .get_type_information();
                let right = &self
                    .annotation_map
                    .get_type(right, self.index)
                    .get_type_information();

                if left.is_numerical() && right.is_numerical() {
                    let bigger_name = get_bigger_type_borrow(left, right, self.index).get_name();
                    self.annotation_map.annotate_type(statement, bigger_name);
                }
            }
            Statement::UnaryExpression {
                value, operator, ..
            } => {
                self.visit_statement(ctx, value);
                let inner_type = self
                    .annotation_map
                    .get_type(value, self.index)
                    .get_type_information();
                if operator == &Operator::Minus {
                    //keep the same type but switch to signed
                    if let Some(target) = typesystem::get_signed_type(inner_type, self.index) {
                        self.annotation_map.annotate_type(value, target.get_name());
                    }
                } else {
                    self.annotation_map
                        .annotate_type(value, inner_type.get_name());
                }
            }
            Statement::Reference { name, .. } => {
                let qualifier = ctx.qualifier.or(ctx.pou);

                let type_name = qualifier
                    .and_then(|pou| self.index.find_member(pou, name).map(|v| v.get_type_name()))
                    .or_else(|| {
                        self.index
                            .find_implementation(name)
                            .map(|_it| name.as_str() /* this is a pou */)
                    })
                    .or_else(|| {
                        self.index
                            .find_global_variable(name)
                            .map(|v| v.get_type_name())
                    });

                let effective_type =
                    type_name.map(|name| self.index.get_effective_type_by_name(name));

                if let Some(data_type) = effective_type {
                    self.annotation_map
                        .annotate_type(statement, data_type.get_name());
                }
            }
            Statement::QualifiedReference { elements, .. } => {
                let mut ctx = ctx.clone();
                for s in elements.iter() {
                    self.visit_statement(&ctx, s);
                    ctx =
                        ctx.with_qualifier(self.annotation_map.get_type(s, self.index).get_name());
                }

                //the last guy represents the type of the whole qualified expression
                if let Some(t) = ctx.qualifier {
                    self.annotation_map.annotate_type(statement, t);
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
            }
            Statement::CallStatement {
                parameters,
                operator,
                ..
            } => {
                self.visit_statement(ctx, operator);
                let operator_type_name = self
                    .annotation_map
                    .get_type(operator, self.index)
                    .get_name();
                if let Some(s) = parameters.as_ref() {
                    let ctx = ctx.with_call(operator_type_name);
                    self.visit_statement(&ctx, s);
                }

                if let Some(return_type) = self
                    .index
                    .find_return_type(operator_type_name)
                    .and_then(|it| self.index.find_effective_type(it))
                {
                    self.annotation_map
                        .annotate_type(statement, return_type.get_name());
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
                self.annotation_map.annotate_type(statement, BOOL_TYPE)
            }
            Statement::LiteralString { .. } => {
                self.annotation_map.annotate_type(statement, STRING_TYPE);
            }
            Statement::LiteralInteger { value, .. } => {
                self.annotation_map
                    .annotate_type(statement, get_int_type_name_for(*value));
            }
            Statement::LiteralTime { .. } => {
                self.annotation_map.annotate_type(statement, TIME_TYPE)
            }
            Statement::LiteralTimeOfDay { .. } => {
                self.annotation_map
                    .annotate_type(statement, TIME_OF_DAY_TYPE);
            }
            Statement::LiteralDate { .. } => {
                self.annotation_map.annotate_type(statement, DATE_TYPE);
            }
            Statement::LiteralDateAndTime { .. } => {
                self.annotation_map
                    .annotate_type(statement, DATE_AND_TIME_TYPE);
            }
            Statement::LiteralReal { .. } => {
                //TODO when do we need a LREAL literal?
                self.annotation_map.annotate_type(statement, REAL_TYPE);
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
    //TODO how will this ever be a negative number?
    if value <= u8::MAX.into() {
        BYTE_TYPE
    } else if value <= u16::MAX.into() {
        UINT_TYPE
    } else if value <= u32::MAX.into() {
        UDINT_TYPE
    } else {
        ULINT_TYPE
    }
}

#[cfg(test)]
mod resolver_tests {
    use super::get_int_type_name_for;

    #[test]
    fn correct_int_types_name_for_numbers() {
        assert_eq!(get_int_type_name_for(0), "BYTE");
        assert_eq!(get_int_type_name_for(i64::pow(2, 8) - 1), "BYTE");
        assert_eq!(get_int_type_name_for(i64::pow(2, 8)), "UINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 16) - 1), "UINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 16)), "UDINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 32) - 1), "UDINT");
        assert_eq!(get_int_type_name_for(i64::pow(2, 32)), "ULINT");
        assert_eq!(get_int_type_name_for(i64::MAX), "ULINT");
    }
}
