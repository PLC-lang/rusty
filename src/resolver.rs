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
    typesystem::{self, get_bigger_type, get_bigger_type_borrow, DataTypeInformation},
};

#[cfg(test)]
mod tests;

pub struct VisitorContext<'s> {
    current_qualifier: Option<&'s str>,
    current_pou: Option<&'s str>,
}
pub struct TypeAnnotator<'i, 's> {
    index: &'i Index,
    context: VisitorContext<'s>,
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

    /// annotates the given statement (using it's ID) with the given type-name
    pub fn annotate_type(&mut self, s: &Statement, type_name: &str) {
        self.type_map.insert(s.get_id(), type_name.to_string());
    }

    pub fn get_type<'i>(&self, s: &Statement, index: &'i Index) -> &'i DataTypeInformation {
        self.type_map
            .get(&s.get_id())
            .and_then(|name| index.get_type(name).ok())
            .map(|data_type| data_type.get_type_information())
            .unwrap_or_else(|| index.get_void_type().get_type_information())
    }
}

macro_rules! visit_all_statements {
     ($self:expr, $annotation:expr, $last:expr ) => {
         $self.visit_statement($annotation, $last);
     };

     ($self:expr, $annotation:expr, $head:expr, $($tail:expr), +) => {
       $self.visit_statement($annotation, $head);
       visit_all_statements!($self, $annotation, $($tail),+)
     };
   }

impl<'i, 's> TypeAnnotator<'i, 's> {
    /// constructs a new TypeAnnotater that works with the given index for type-lookups
    pub fn new(index: &'i Index) -> TypeAnnotator<'i, 's> {
        TypeAnnotator {
            context: VisitorContext {
                current_pou: None,
                current_qualifier: None,
            },
            index,
        }
    }

    /// annotates the given AST elements with the type-name resulting for the statements/expressions.
    ///
    /// Returns an AnnotationMap with the resulting types for all visited Statements. See `AnnotationMap`
    pub fn visit_unit(&mut self, unit: &'s CompilationUnit) -> AnnotationMap {
        let mut annotation_map = AnnotationMap::new();

        for pou in &unit.units {
            self.visit_pou(pou, &mut annotation_map);
        }

        for t in &unit.types {
            self.visit_user_type_declaration(t, &mut annotation_map);
        }

        for i in &unit.implementations {
            let old = std::mem::replace(&mut self.context.current_pou, Some(i.name.as_str()));
            i.statements
                .iter()
                .for_each(|s| self.visit_statement(&mut annotation_map, s));
            self.context.current_qualifier = old;
        }
        annotation_map
    }

    fn visit_user_type_declaration(
        &mut self,
        user_data_type: &UserTypeDeclaration,
        annotation_map: &mut AnnotationMap,
    ) {
        self.visit_data_type(&user_data_type.data_type, annotation_map);
        if let Some(initializer) = &user_data_type.initializer {
            self.visit_statement(annotation_map, &initializer);
        }
    }

    fn visit_pou(&mut self, pou: &'s Pou, annotation_map: &mut AnnotationMap) {
        let old = std::mem::replace(&mut self.context.current_pou, Some(pou.name.as_str()));
        for block in &pou.variable_blocks {
            for variable in &block.variables {
                self.visit_variable(variable, annotation_map);
            }
        }
        self.context.current_pou = old;
    }

    fn visit_variable(&mut self, variable: &Variable, annotation_map: &mut AnnotationMap) {
        self.visit_data_type_declaration(&variable.data_type, annotation_map);
    }

    fn visit_data_type_declaration(
        &mut self,
        declaration: &DataTypeDeclaration,
        annotation_map: &mut AnnotationMap,
    ) {
        if let DataTypeDeclaration::DataTypeDefinition { data_type } = declaration {
            self.visit_data_type(data_type, annotation_map);
        }
    }

    fn visit_data_type(&mut self, data_type: &DataType, annotation_map: &mut AnnotationMap) {
        match data_type {
            DataType::StructType { variables, .. } => variables
                .iter()
                .for_each(|v| self.visit_variable(v, annotation_map)),
            DataType::ArrayType {
                referenced_type, ..
            } => self.visit_data_type_declaration(referenced_type, annotation_map),
            DataType::VarArgs {
                referenced_type: Some(referenced_type),
            } => {
                self.visit_data_type_declaration(referenced_type.as_ref(), annotation_map);
            }
            _ => {}
        }
    }

    fn visit_statement(&mut self, annotation_map: &mut AnnotationMap, statement: &Statement) {
        match statement {
            Statement::LiteralBool { .. } => annotation_map.annotate_type(statement, "BOOL"),
            Statement::LiteralString { .. } => {
                annotation_map.annotate_type(statement, "STRING");
            }
            Statement::LiteralInteger { value, .. } => {
                annotation_map.annotate_type(statement, get_int_type_name_for(*value));
            }
            Statement::LiteralTime { .. } => annotation_map.annotate_type(statement, "TIME"),
            Statement::LiteralTimeOfDay { .. } => {
                annotation_map.annotate_type(statement, "TIME_OF_DAY");
            }
            Statement::LiteralDate { .. } => {
                annotation_map.annotate_type(statement, "DATE");
            }
            Statement::LiteralDateAndTime { .. } => {
                annotation_map.annotate_type(statement, "DATE_AND_TIME");
            }
            Statement::LiteralReal { .. } => {
                annotation_map.annotate_type(statement, "REAL");
            }
            Statement::LiteralArray {
                elements: Some(elements),
                ..
            } => {
                self.visit_statement(annotation_map, elements.as_ref());
                //TODO
            }
            Statement::MultipliedStatement { element, .. } => {
                self.visit_statement(annotation_map, element)
                //TODO
            }
            Statement::QualifiedReference { elements, .. } => {
                let elements = elements.iter();

                if let Some(qualifier) = elements.next() {
                    self.visit_statement(annotation_map, qualifier);
                    let context = annotation_map.get_type(qualifier, self.index).get_name();
                    self.context.current_qualifier = Some(context);
                    elements.for_each(|s| {
                        self.visit_statement(annotation_map, qualifier);
                        let context = annotation_map.get_type(s, self.index).get_name();
                        self.context.current_qualifier = Some(context);
                    });
                }
            }
            Statement::ArrayAccess {
                reference, access, ..
            } => {
                visit_all_statements!(self, annotation_map, reference, access);
                //TODO
            }
            Statement::BinaryExpression { left, right, .. } => {
                visit_all_statements!(self, annotation_map, left, right);
                let left = &annotation_map.get_type(left, self.index);
                let right = &annotation_map.get_type(right, self.index);

                if left.is_numerical() && right.is_numerical() {
                    let bigger_name = get_bigger_type_borrow(left, right, self.index).get_name();
                    annotation_map.annotate_type(statement, bigger_name);
                }
            }
            Statement::UnaryExpression {
                value, operator, ..
            } => {
                self.visit_statement(annotation_map, value);
                let inner_type = annotation_map.get_type(value, self.index);
                if operator == &Operator::Minus {
                    //keep the same type but switch to signed
                    if let Some(target) = typesystem::get_signed_type(inner_type, self.index) {
                        annotation_map.annotate_type(value, target.get_name());
                    }
                } else {
                    annotation_map.annotate_type(value, inner_type.get_name());
                }
            }
            Statement::Reference { name, .. } => {
                let qualifier = self.context.current_qualifier.or(self.context.current_pou);

                let type_name = qualifier
                    .and_then(|pou| self.index.find_member(pou, name).map(|v| v.get_type_name()))
                    .or_else(|| {
                        self.index
                            .find_global_variable(name)
                            .map(|v| v.get_type_name())
                    })
                    .or_else(|| {
                        self.index
                            .find_implementation(name)
                            .map(|it| name.as_str() /* this is a pou */)
                    });

                let effective_type = type_name
                    .and_then(|dt| self.index.get_type(dt).ok())
                    .map(|it| it.get_type_information())
                    .and_then(|it| self.index.find_effective_type(it));

                if let Some(data_type) = effective_type {
                    annotation_map.annotate_type(statement, data_type.get_name());
                }
            }
            Statement::ExpressionList { expressions, .. } => expressions
                .iter()
                .for_each(|e| self.visit_statement(annotation_map, e)),
            Statement::RangeStatement { start, end, .. } => {
                visit_all_statements!(self, annotation_map, start, end);
            }
            Statement::Assignment { left, right, .. } => {
                self.visit_statement(annotation_map, left);
                self.visit_statement(annotation_map, right);
            }
            Statement::OutputAssignment { left, right, .. } => {
                self.visit_statement(annotation_map, left);
                self.visit_statement(annotation_map, right);
            }
            Statement::CallStatement {
                parameters,
                operator,
                ..
            } => {
                self.visit_statement(annotation_map, operator);
                if let Some(s) = parameters.as_ref() {
                    self.visit_statement(annotation_map, s);
                }
            }
            Statement::IfStatement {
                blocks, else_block, ..
            } => {
                blocks.iter().for_each(|b| {
                    self.visit_statement(annotation_map, b.condition.as_ref());
                    b.body
                        .iter()
                        .for_each(|s| self.visit_statement(annotation_map, s));
                });
                else_block
                    .iter()
                    .for_each(|e| self.visit_statement(annotation_map, e));
            }
            Statement::ForLoopStatement {
                counter,
                start,
                end,
                by_step,
                body,
                ..
            } => {
                visit_all_statements!(self, annotation_map, counter, start, end);
                if let Some(by_step) = by_step {
                    self.visit_statement(annotation_map, by_step);
                }
                body.iter()
                    .for_each(|s| self.visit_statement(annotation_map, s));
            }
            Statement::WhileLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(annotation_map, condition);
                body.iter()
                    .for_each(|s| self.visit_statement(annotation_map, s));
            }
            Statement::RepeatLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(annotation_map, condition);
                body.iter()
                    .for_each(|s| self.visit_statement(annotation_map, s));
            }
            Statement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => {
                self.visit_statement(annotation_map, selector);
                case_blocks.iter().for_each(|b| {
                    self.visit_statement(annotation_map, b.condition.as_ref());
                    b.body
                        .iter()
                        .for_each(|s| self.visit_statement(annotation_map, s));
                });
                else_block
                    .iter()
                    .for_each(|s| self.visit_statement(annotation_map, s));
            }
            Statement::CaseCondition { condition, .. } => {
                self.visit_statement(annotation_map, condition)
            }
            _ => {}
        }
    }
}

fn get_int_type_name_for(value: i64) -> &'static str {
    //TODO how will this ever be a negative number?
    if value <= u8::MAX.into() {
        "BYTE"
    } else if value <= u16::MAX.into() {
        "UINT"
    } else if value <= u32::MAX.into() {
        "UDINT"
    } else {
        "ULINT"
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
