// Copyright (c) 2021 Ghaith Hachem and Mathias Rieder

//! Resolves (partial) expressions & statements and annotates the resulting types
//!
//! Recursively visits all statements and expressions of a `CompilationUnit` and
//! records all resulting types associated with the statement's id.

use indexmap::IndexMap;

pub mod const_evaluator;

use crate::{
    ast::{
        self, AstId, AstStatement, CompilationUnit, DataType, DataTypeDeclaration, Operator, Pou,
        UserTypeDeclaration, Variable,
    },
    index::{ImplementationIndexEntry, ImplementationType, Index, VariableIndexEntry},
    typesystem::{
        self, get_bigger_type, DataTypeInformation, BOOL_TYPE, BYTE_TYPE, CONST_STRING_TYPE,
        CONST_WSTRING_TYPE, DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, DWORD_TYPE, LINT_TYPE,
        REAL_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE, VOID_TYPE, WORD_TYPE,
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

    /// true if the expression passed a constant-variable on the way
    /// e.g. true for `x` if x is declared in a constant block
    /// e.g. true for `a.b.c` if either a,b or c is declared in a constant block
    constant: bool,
}

impl<'s> VisitorContext<'s> {
    /// returns a copy of the current context and changes the `current_qualifier` to the given qualifier
    fn with_qualifier(&self, qualifier: String) -> VisitorContext<'s> {
        VisitorContext {
            pou: self.pou,
            qualifier: Some(qualifier),
            call: self.call,
            constant: false,
        }
    }

    /// returns a copy of the current context and changes the `current_pou` to the given pou
    fn with_pou(&self, pou: &'s str) -> VisitorContext<'s> {
        VisitorContext {
            pou: Some(pou),
            qualifier: self.qualifier.clone(),
            call: self.call,
            constant: false,
        }
    }

    /// returns a copy of the current context and changes the `lhs_pou` to the given pou
    fn with_call(&self, lhs_pou: &'s str) -> VisitorContext<'s> {
        VisitorContext {
            pou: self.pou,
            qualifier: self.qualifier.clone(),
            call: Some(lhs_pou),
            constant: false,
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
    Value { resulting_type: String },
    /// a reference that resolves to a declared variable (e.g. `a` --> `PLC_PROGRAM.a`)
    Variable {
        resulting_type: String,
        qualified_name: String,
        constant: bool,
        is_auto_deref: bool,
    },
    /// a reference to a function
    Function {
        return_type: String,
        qualified_name: String,
    },
    /// a reference to a type (e.g. `INT`)
    Type { type_name: String },
    /// a reference to a program call or reference (e.g. `PLC_PRG`)
    Program { qualified_name: String },
}

impl StatementAnnotation {
    pub fn value(type_name: &str) -> StatementAnnotation {
        StatementAnnotation::Value {
            resulting_type: type_name.to_string(),
        }
    }
}

pub struct AnnotationMap {
    /// maps a statement to the type it resolves to
    type_map: IndexMap<AstId, StatementAnnotation>,

    /// maps a statement to the target-type it should eventually resolve to
    /// example:
    /// x : BYTE := 1;  //1's actual type is DINT, 1's target type is BYTE
    /// x : INT := 1;   //1's actual type is DINT, 1's target type is INT
    ///
    /// if the type-hint is equal to the actual type, or there is no
    /// useful type-hint to resolve, there is no mapping in this map
    type_hint_map: IndexMap<AstId, StatementAnnotation>,
}

impl AnnotationMap {
    /// creates a new empty AnnotationMap
    pub fn new() -> Self {
        AnnotationMap {
            type_map: IndexMap::new(),
            type_hint_map: IndexMap::new(),
        }
    }

    pub fn import(&mut self, other: AnnotationMap) {
        self.type_map.extend(other.type_map);
        self.type_hint_map.extend(other.type_hint_map);
    }

    /// annotates the given statement (using it's `get_id()`) with the given type-name
    pub fn annotate(&mut self, s: &AstStatement, annotation: StatementAnnotation) {
        self.type_map.insert(s.get_id(), annotation);
    }

    pub fn annotate_type_hint(&mut self, s: &AstStatement, annotation: StatementAnnotation) {
        self.type_hint_map.insert(s.get_id(), annotation);
    }

    pub fn get(&self, s: &AstStatement) -> Option<&StatementAnnotation> {
        self.type_map.get(&s.get_id())
    }

    pub fn get_hint(&self, s: &AstStatement) -> Option<&StatementAnnotation> {
        self.type_hint_map.get(&s.get_id())
    }

    /// returns the annotated type or void if none was annotated
    pub fn get_type_or_void<'i>(
        &self,
        s: &AstStatement,
        index: &'i Index,
    ) -> &'i typesystem::DataType {
        self.get_type(s, index)
            .unwrap_or_else(|| index.get_void_type())
    }

    pub fn get_type_hint<'i>(
        &self,
        s: &AstStatement,
        index: &'i Index,
    ) -> Option<&'i typesystem::DataType> {
        self.get_from_map(s, &self.type_hint_map, index)
    }

    /// returns the annotated type
    pub fn get_type<'i>(
        &self,
        s: &AstStatement,
        index: &'i Index,
    ) -> Option<&'i typesystem::DataType> {
        self.get_from_map(s, &self.type_map, index)
    }

    /// returns the annotated type from the given map
    fn get_from_map<'i>(
        &self,
        s: &AstStatement,
        map: &IndexMap<AstId, StatementAnnotation>,
        index: &'i Index,
    ) -> Option<&'i typesystem::DataType> {
        map.get(&s.get_id())
            .and_then(|annotation| match annotation {
                StatementAnnotation::Value { resulting_type } => Some(resulting_type.as_str()),
                StatementAnnotation::Variable { resulting_type, .. } => {
                    Some(resulting_type.as_str())
                }
                StatementAnnotation::Function { .. } => None,
                StatementAnnotation::Type { .. } => None,
                StatementAnnotation::Program { .. } => None,
            })
            .and_then(|type_name| index.get_type(type_name).ok())
    }

    /// reutrns the name of the callable that is refered by the given statemt
    /// or none if this thing may not be callable
    pub fn get_call_name(&self, s: &AstStatement) -> Option<&str> {
        match self.type_map.get(&s.get_id()) {
            Some(StatementAnnotation::Function { qualified_name, .. }) => {
                Some(qualified_name.as_str())
            }
            Some(StatementAnnotation::Program { qualified_name }) => Some(qualified_name.as_str()),
            Some(StatementAnnotation::Variable { resulting_type, .. }) => {
                Some(resulting_type.as_str())
            }
            _ => None,
        }
    }

    /// returns the annotation of the given statement or none if it was not annotated
    pub fn get_annotation(&self, s: &AstStatement) -> Option<&StatementAnnotation> {
        self.type_map.get(&s.get_id())
    }

    pub fn has_type_annotation(&self, id: &usize) -> bool {
        self.type_map.contains_key(id)
    }
}

impl Default for AnnotationMap {
    fn default() -> Self {
        Self::new()
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
            constant: false,
        };

        for global_variable in unit.global_vars.iter().flat_map(|it| it.variables.iter()) {
            visitor.visit_variable(ctx, global_variable);
        }

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

        // enum initializers may have been introduced by the visitor (indexer)
        // so we shoul try to resolve and type-annotate them here as well
        for (_, enum_element) in index.get_global_qualified_enums() {
            if let Some((Some(statement), scope)) = enum_element
                .initial_value
                .map(|i| index.get_const_expressions().find_expression(&i))
            {
                if let Some(scope) = scope {
                    visitor.visit_statement(&ctx.with_pou(scope), statement);
                } else {
                    visitor.visit_statement(ctx, statement);
                }
            }
        }

        visitor.annotation_map
    }

    fn visit_user_type_declaration(
        &mut self,
        user_data_type: &UserTypeDeclaration,
        ctx: &VisitorContext,
    ) {
        self.visit_data_type(ctx, &user_data_type.data_type);
        if let Some(name) = user_data_type.data_type.get_name() {
            let ctx = &ctx.with_pou(name);
            if let Some((initializer, name)) = user_data_type
                .initializer
                .as_ref()
                .zip(user_data_type.data_type.get_name())
            {
                self.visit_statement(ctx, initializer);

                //update the type-hint for the initializer
                if let Some(right_type) = self.index.find_effective_type(name) {
                    self.update_expected_types(right_type, initializer);
                }
            }
        } else {
            unreachable!("datatype without a name");
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

    /// updates the expected types of statement on the right side of an assignment
    /// e.g. x : ARRAY [0..1] OF BYTE := [2,3];
    /// note that the left side needs to be annotated before this call
    fn update_right_hand_side_expected_type(
        &mut self,
        annotated_left_side: &AstStatement,
        right_side: &AstStatement,
    ) {
        if let Some(t) = self
            .annotation_map
            .get_type(annotated_left_side, self.index)
        {
            //annotate the right-hand side as a whole
            self.annotation_map
                .annotate_type_hint(right_side, StatementAnnotation::value(t.get_name()));

            //dive into the right hand side
            self.update_expected_types(t, right_side);
        }
    }

    /// updates the expected types of statements on the right side of an assignment
    /// e.g. x : ARRAY [0..1] OF BYTE := [2,3];
    fn update_expected_types(
        &mut self,
        expected_type: &typesystem::DataType,
        statement: &AstStatement,
    ) {
        //see if we need to dive into it
        match statement {
            AstStatement::LiteralArray {
                elements: Some(elements),
                ..
            } => {
                //annotate the literal-array itself
                self.annotation_map.annotate_type_hint(
                    statement,
                    StatementAnnotation::value(expected_type.get_name()),
                );
                //TODO exprssionList and MultipliedExpressions are a mess!
                if matches!(
                    elements.as_ref(),
                    AstStatement::ExpressionList { .. } | AstStatement::MultipliedStatement { .. }
                ) {
                    self.annotation_map.annotate_type_hint(
                        elements,
                        StatementAnnotation::value(expected_type.get_name()),
                    );
                }
                //annotate the array's member elements with the array's inner type
                if let DataTypeInformation::Array {
                    inner_type_name, ..
                } = expected_type.get_type_information()
                {
                    if let Some(inner_type) = self.index.find_effective_type(inner_type_name) {
                        self.update_expected_types(inner_type, elements);
                    }
                }
            }
            AstStatement::Assignment { left, right, .. } => {
                //struct initialization (left := right)
                //find out left's type and update a type hint for right
                if let (
                    typesystem::DataTypeInformation::Struct {
                        name: qualifier, ..
                    },
                    AstStatement::Reference {
                        name: variable_name,
                        ..
                    },
                ) = (expected_type.get_type_information(), left.as_ref())
                {
                    if let Some(v) = self.index.find_member(qualifier, variable_name) {
                        if let Some(target_type) = self.index.find_effective_type(v.get_type_name())
                        {
                            self.annotation_map.annotate(
                                left.as_ref(),
                                to_variable_annotation(v, self.index, false),
                            );
                            self.annotation_map.annotate_type_hint(
                                right.as_ref(),
                                StatementAnnotation::value(v.get_type_name()),
                            );
                            self.update_expected_types(target_type, right);
                        }
                    }
                }
            }
            AstStatement::MultipliedStatement {
                element: elements, ..
            } => {
                // n(elements)
                //annotate the type to all multiplied elements
                for ele in AstStatement::get_as_list(elements) {
                    self.update_expected_types(expected_type, ele);
                }
            }
            AstStatement::ExpressionList { expressions, .. } => {
                //annotate the type to all elements
                for ele in expressions {
                    self.update_expected_types(expected_type, ele);
                }
            }
            AstStatement::RangeStatement { start, end, .. } => {
                self.update_expected_types(expected_type, start);
                self.update_expected_types(expected_type, end);
            }
            AstStatement::LiteralInteger { .. } => {
                //special case -> promote a literal-Integer directly, not via type-hint
                // (avoid later cast)
                if expected_type.get_type_information().is_float() {
                    self.annotation_map.annotate(
                        statement,
                        StatementAnnotation::value(expected_type.get_name()),
                    )
                } else {
                    //annotate the statement, whatever it is
                    self.annotation_map.annotate_type_hint(
                        statement,
                        StatementAnnotation::value(expected_type.get_name()),
                    )
                }
            }
            _ => {
                //annotate the statement, whatever it is
                self.annotation_map.annotate_type_hint(
                    statement,
                    StatementAnnotation::value(expected_type.get_name()),
                )
            }
        }
    }

    fn visit_variable(&mut self, ctx: &VisitorContext, variable: &Variable) {
        self.visit_data_type_declaration(ctx, &variable.data_type);
        if let Some(initializer) = variable.initializer.as_ref() {
            self.visit_statement(ctx, initializer);

            // annotate a type-hint for the initializer, it should be the same type as the variable
            // e.g. x : BYTE := 7 + 3;  --> 7+3 should be casted into a byte
            if let Some(expected_type) = self
                .index
                .find_variable(
                    ctx.qualifier.as_deref().or(ctx.pou),
                    &[variable.name.as_str()],
                )
                .and_then(|ve| self.index.find_effective_type(ve.get_type_name()))
            {
                self.annotation_map.annotate_type_hint(
                    initializer,
                    StatementAnnotation::value(expected_type.get_name()),
                );
                self.update_expected_types(expected_type, initializer);
            }
        }
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
            DataType::StructType {
                name: Some(name),
                variables,
                ..
            } => {
                let ctx = ctx.with_qualifier(name.clone());
                variables.iter().for_each(|v| self.visit_variable(&ctx, v))
            }
            DataType::ArrayType {
                referenced_type, ..
            } => self.visit_data_type_declaration(ctx, referenced_type),
            DataType::VarArgs {
                referenced_type: Some(referenced_type),
            } => {
                self.visit_data_type_declaration(ctx, referenced_type.as_ref());
            }
            DataType::SubRangeType {
                referenced_type,
                bounds: Some(bounds),
                ..
            } => {
                if let Some(expected_type) = self.index.find_effective_type(referenced_type) {
                    self.visit_statement(ctx, bounds);
                    self.update_expected_types(expected_type, bounds);
                }
            }
            _ => {}
        }
    }

    fn visit_statement(&mut self, ctx: &VisitorContext, statement: &AstStatement) {
        self.visit_statement_control(ctx, statement);
    }

    /// annotate a control statement
    fn visit_statement_control(&mut self, ctx: &VisitorContext, statement: &AstStatement) {
        match statement {
            AstStatement::IfStatement {
                blocks, else_block, ..
            } => {
                blocks.iter().for_each(|b| {
                    self.visit_statement(ctx, b.condition.as_ref());
                    b.body.iter().for_each(|s| self.visit_statement(ctx, s));
                });
                else_block.iter().for_each(|e| self.visit_statement(ctx, e));
            }
            AstStatement::ForLoopStatement {
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
            AstStatement::WhileLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(ctx, condition);
                body.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            AstStatement::RepeatLoopStatement {
                condition, body, ..
            } => {
                self.visit_statement(ctx, condition);
                body.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            AstStatement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => {
                self.visit_statement(ctx, selector);
                let selector_type = self.annotation_map.get_type(selector, self.index);
                case_blocks.iter().for_each(|b| {
                    self.visit_statement(ctx, b.condition.as_ref());
                    if let Some(selector_type) = selector_type {
                        self.update_expected_types(selector_type, b.condition.as_ref());
                    }
                    b.body.iter().for_each(|s| self.visit_statement(ctx, s));
                });
                else_block.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            AstStatement::CaseCondition { condition, .. } => self.visit_statement(ctx, condition),
            _ => {
                self.visit_statement_expression(ctx, statement);
            }
        }
    }

    /// annotate an expression statement
    fn visit_statement_expression(&mut self, ctx: &VisitorContext, statement: &AstStatement) {
        match statement {
            AstStatement::ArrayAccess {
                reference, access, ..
            } => {
                visit_all_statements!(self, ctx, reference);
                self.visit_statement(
                    &VisitorContext {
                        call: None,
                        constant: false,
                        pou: ctx.pou,
                        qualifier: None,
                    },
                    access,
                );
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
                        .annotate(statement, StatementAnnotation::value(t));
                }
            }
            AstStatement::PointerAccess { reference, .. } => {
                visit_all_statements!(self, ctx, reference);
                let pointer_type = self
                    .annotation_map
                    .get_type_or_void(reference, self.index)
                    .get_type_information();
                if let DataTypeInformation::Pointer {
                    inner_type_name, ..
                } = pointer_type
                {
                    let t = self
                        .index
                        .get_effective_type_by_name(inner_type_name)
                        .get_name();
                    self.annotation_map
                        .annotate(statement, StatementAnnotation::value(t));
                }
            }
            AstStatement::DirectAccess { access, index, .. } => {
                let ctx = VisitorContext {
                    qualifier: None,
                    ..ctx.clone()
                };
                visit_all_statements!(self, &ctx, index);
                match access {
                    crate::ast::DirectAccessType::Bit => self.annotation_map.annotate(
                        statement,
                        StatementAnnotation::Value {
                            resulting_type: BOOL_TYPE.into(),
                        },
                    ),
                    crate::ast::DirectAccessType::Byte => self.annotation_map.annotate(
                        statement,
                        StatementAnnotation::Value {
                            resulting_type: BYTE_TYPE.into(),
                        },
                    ),
                    crate::ast::DirectAccessType::Word => self.annotation_map.annotate(
                        statement,
                        StatementAnnotation::Value {
                            resulting_type: WORD_TYPE.into(),
                        },
                    ),
                    crate::ast::DirectAccessType::DWord => self.annotation_map.annotate(
                        statement,
                        StatementAnnotation::Value {
                            resulting_type: DWORD_TYPE.into(),
                        },
                    ),
                }
            }
            AstStatement::BinaryExpression {
                left,
                right,
                operator,
                ..
            } => {
                visit_all_statements!(self, ctx, left, right);
                let left_type = self.annotation_map.get_type_or_void(left, self.index);
                let right_type = self.annotation_map.get_type_or_void(right, self.index);

                if left_type.get_type_information().is_numerical()
                    && right_type.get_type_information().is_numerical()
                {
                    let dint = self.index.get_type_or_panic(DINT_TYPE);
                    let bigger_type = get_bigger_type(
                        get_bigger_type(left_type, right_type, self.index),
                        dint,
                        self.index,
                    );
                    if bigger_type != left_type {
                        self.update_expected_types(bigger_type, left);
                    }
                    if bigger_type != right_type {
                        self.update_expected_types(bigger_type, right);
                    }

                    let target_name = if operator.is_bool_type() {
                        BOOL_TYPE
                    } else {
                        bigger_type.get_name()
                    };
                    self.annotation_map
                        .annotate(statement, StatementAnnotation::value(target_name));
                }
            }
            AstStatement::UnaryExpression {
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
                        self.annotation_map
                            .annotate(statement, StatementAnnotation::value(target.get_name()));
                    }
                } else {
                    //TODO: The adderss operator should report a correct pointer type. We need to have reproducable type names for that first.
                    self.annotation_map
                        .annotate(statement, StatementAnnotation::value(inner_type.get_name()));
                }
            }
            AstStatement::Reference { name, .. } => {
                let annotation = if let Some(qualifier) = ctx.qualifier.as_deref() {
                    // if we see a qualifier, we only consider [qualifier].[name] as candidates
                    self.index
                        // 1st try a qualified member variable qualifier.name
                        .find_member(qualifier, name)
                        // 2nd try an enum-element qualifier#name
                        .or_else(|| self.index.find_enum_element(qualifier, name.as_str()))
                        // 3rd try - look for a method qualifier.name
                        .map_or_else(
                            || {
                                find_implementation_annotation(
                                    format!("{}.{}", qualifier, name).as_str(),
                                    self.index,
                                )
                            },
                            |v| Some(to_variable_annotation(v, self.index, ctx.constant)),
                        )
                } else {
                    // if we see no qualifier, we try some strategies ...
                    ctx.pou
                        .and_then(|qualifier| {
                            // ... first look at POU-local variables
                            self.index
                                .find_member(qualifier, name)
                                .or_else(|| {
                                    // ... then check if we're in a method and we're referencing
                                    // a member variable of the corresponding class
                                    self.index
                                        .find_implementation(qualifier)
                                        .and_then(
                                            ImplementationIndexEntry::get_associated_class_name,
                                        )
                                        .and_then(|it| self.index.find_member(it, name))
                                })
                                .map(|v| to_variable_annotation(v, self.index, ctx.constant))
                                .or_else(|| {
                                    //Try to find an action with this name
                                    let action_call_name = format!("{}.{}", qualifier, name);
                                    self.index.find_implementation(&action_call_name).and_then(
                                        |entry| {
                                            find_implementation_annotation(
                                                entry.get_call_name(),
                                                self.index,
                                            )
                                        },
                                    )
                                })
                        })
                        .or_else(|| {
                            // ... then try if we find a pou with that name (maybe it's a call?)
                            let class_name = ctx
                                .pou
                                .and_then(|pou_name| self.index.find_implementation(pou_name))
                                .and_then(ImplementationIndexEntry::get_associated_class_name);

                            //TODO introduce qualified names!
                            let call_name = class_name
                                .map(|it| format!("{}.{}", it, name))
                                .unwrap_or_else(|| name.into());
                            find_implementation_annotation(&call_name, self.index)
                        })
                        .or_else(|| {
                            // ... last option is a global variable, where we ignore the current pou's name as a qualifier
                            self.index
                                .find_global_variable(name)
                                .map(|v| to_variable_annotation(v, self.index, ctx.constant))
                        })
                };
                if let Some(annotation) = annotation {
                    self.annotation_map.annotate(statement, annotation)
                }
            }
            AstStatement::QualifiedReference { elements, .. } => {
                let mut ctx = ctx.clone();
                for s in elements.iter() {
                    self.visit_statement(&ctx, s);

                    let (qualifier, constant) = self
                        .annotation_map
                        .get_annotation(s)
                        .map(|it| match it {
                            StatementAnnotation::Value { resulting_type } => {
                                (resulting_type.as_str(), false)
                            }
                            StatementAnnotation::Variable {
                                resulting_type,
                                constant,
                                ..
                            } => (resulting_type.as_str(), *constant),
                            StatementAnnotation::Function { .. } => (VOID_TYPE, false),
                            StatementAnnotation::Type { type_name } => (type_name.as_str(), false),
                            StatementAnnotation::Program { qualified_name } => {
                                (qualified_name.as_str(), false)
                            }
                        })
                        .unwrap_or_else(|| (VOID_TYPE, false));
                    let mut new_ctx = ctx.with_qualifier(qualifier.to_string());
                    new_ctx.constant = constant;
                    ctx = new_ctx;
                }

                //the last guy represents the type of the whole qualified expression
                if let Some(last) = elements.last() {
                    if let Some(annotation) = self.annotation_map.get_annotation(last).cloned() {
                        self.annotation_map.annotate(statement, annotation);
                    }
                }
            }
            AstStatement::ExpressionList { expressions, .. } => expressions
                .iter()
                .for_each(|e| self.visit_statement(ctx, e)),

            AstStatement::RangeStatement { start, end, .. } => {
                visit_all_statements!(self, ctx, start, end);
            }
            AstStatement::Assignment { left, right, .. } => {
                self.visit_statement(ctx, right);
                if let Some(lhs) = ctx.call {
                    //special context for left hand side
                    self.visit_statement(&ctx.with_pou(lhs), left);
                    // give a type hint that we want the right side to be stored in the left's type
                } else {
                    self.visit_statement(ctx, left);
                }
                self.update_right_hand_side_expected_type(left, right);
            }
            AstStatement::OutputAssignment { left, right, .. } => {
                visit_all_statements!(self, ctx, left, right);
                if let Some(lhs) = ctx.call {
                    //special context for left hand side
                    self.visit_statement(&ctx.with_pou(lhs), left);
                } else {
                    self.visit_statement(ctx, left);
                }
                self.update_right_hand_side_expected_type(left, right);
            }
            AstStatement::CallStatement {
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
                            StatementAnnotation::Function { qualified_name, .. } => {
                                Some(qualified_name.clone())
                            }
                            StatementAnnotation::Program { qualified_name } => {
                                Some(qualified_name.clone())
                            }
                            StatementAnnotation::Variable { resulting_type, .. } => {
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
                            _ => None,
                        })
                        .unwrap_or_else(|| VOID_TYPE.to_string());
                    let ctx = ctx.with_call(operator_qualifier.as_str());
                    //need to clone the qualifier string because of borrow checker :-( - //todo look into this
                    self.visit_statement(&ctx, s);

                    let parameters = ast::flatten_expression_list(s);
                    let all_members = self
                        .index
                        .get_container_members(operator_qualifier.as_str());
                    let members = all_members.iter().filter(|it| it.is_parameter());

                    for (i, m) in members.enumerate() {
                        if let Some(p) = parameters.get(i) {
                            if !matches!(p, AstStatement::Assignment { .. }) {
                                if let Some(effective_member_type) =
                                    self.index.find_effective_type(m.get_type_name())
                                {
                                    //update the type hint
                                    self.annotation_map.annotate_type_hint(
                                        p,
                                        StatementAnnotation::value(
                                            effective_member_type.get_name(),
                                        ),
                                    )
                                }
                            }
                        }
                    }
                }

                if let Some(StatementAnnotation::Function { return_type, .. }) =
                    self.annotation_map.get(operator)
                {
                    if let Some(return_type) = self.index.find_effective_type(return_type) {
                        self.annotation_map.annotate(
                            statement,
                            StatementAnnotation::value(return_type.get_name()),
                        );
                    }
                }
            }
            AstStatement::CastStatement {
                target, type_name, ..
            } => {
                //see if this type really exists
                let data_type = self.index.find_effective_type_info(type_name);
                if let Some(DataTypeInformation::Enum { name, .. }) = data_type {
                    //enum cast
                    self.visit_statement(&ctx.with_qualifier(name.to_string()), target);
                    //use the type of the target
                    let type_name = self
                        .annotation_map
                        .get_type_or_void(target, self.index)
                        .get_name();
                    self.annotation_map
                        .annotate(statement, StatementAnnotation::value(type_name));
                } else if let Some(t) = data_type {
                    //different cast
                    self.annotation_map
                        .annotate(statement, StatementAnnotation::value(t.get_name()));
                    //overwrite the existing annotation
                    self.annotation_map
                        .annotate(target, StatementAnnotation::value(t.get_name()))
                } else {
                    //unknown type? what should we do here?
                    self.visit_statement(ctx, target);
                }
            }
            _ => {
                self.visit_statement_literals(ctx, statement);
            }
        }
    }

    /// annotate a literal statement
    fn visit_statement_literals(&mut self, ctx: &VisitorContext, statement: &AstStatement) {
        match statement {
            AstStatement::LiteralBool { .. } => {
                self.annotation_map
                    .annotate(statement, StatementAnnotation::value(BOOL_TYPE));
            }

            AstStatement::LiteralString { is_wide, .. } => {
                let string_type_name = if *is_wide {
                    CONST_WSTRING_TYPE
                } else {
                    CONST_STRING_TYPE
                };
                self.annotation_map
                    .annotate(statement, StatementAnnotation::value(string_type_name));
            }
            AstStatement::LiteralInteger { value, .. } => {
                self.annotation_map.annotate(
                    statement,
                    StatementAnnotation::value(get_int_type_name_for(*value)),
                );
            }
            AstStatement::LiteralTime { .. } => self
                .annotation_map
                .annotate(statement, StatementAnnotation::value(TIME_TYPE)),
            AstStatement::LiteralTimeOfDay { .. } => {
                self.annotation_map
                    .annotate(statement, StatementAnnotation::value(TIME_OF_DAY_TYPE));
            }
            AstStatement::LiteralDate { .. } => {
                self.annotation_map
                    .annotate(statement, StatementAnnotation::value(DATE_TYPE));
            }
            AstStatement::LiteralDateAndTime { .. } => {
                self.annotation_map
                    .annotate(statement, StatementAnnotation::value(DATE_AND_TIME_TYPE));
            }
            AstStatement::LiteralReal { .. } => {
                //TODO when do we need a LREAL literal?
                self.annotation_map
                    .annotate(statement, StatementAnnotation::value(REAL_TYPE));
            }
            AstStatement::LiteralArray {
                elements: Some(elements),
                ..
            } => {
                self.visit_statement(ctx, elements.as_ref());
                //TODO as of yet we have no way to derive a name that reflects a fixed size array
            }
            AstStatement::MultipliedStatement { element, .. } => {
                self.visit_statement(ctx, element)
                //TODO as of yet we have no way to derive a name that reflects a fixed size array
            }

            _ => {}
        }
    }
}
fn find_implementation_annotation(name: &str, index: &Index) -> Option<StatementAnnotation> {
    index
        .find_implementation(name)
        .and_then(|it| match it.get_implementation_type() {
            ImplementationType::Program | &ImplementationType::Action => {
                Some(to_programm_annotation(it))
            }
            ImplementationType::Function | ImplementationType::Method => {
                Some(to_function_annotation(it, index))
            }
            ImplementationType::FunctionBlock => Some(to_type_annotation(name)),
            _ => None,
        })
}

fn to_type_annotation(name: &str) -> StatementAnnotation {
    StatementAnnotation::Type {
        type_name: name.into(),
    }
}

fn to_programm_annotation(it: &ImplementationIndexEntry) -> StatementAnnotation {
    StatementAnnotation::Program {
        qualified_name: it.get_call_name().into(),
    }
}

fn to_variable_annotation(
    v: &VariableIndexEntry,
    index: &Index,
    constant_override: bool,
) -> StatementAnnotation {
    let v_type = index.get_effective_type_by_name(v.get_type_name());

    //see if this is an auto-deref variable
    let (effective_type_name, is_auto_deref) = if let DataTypeInformation::Pointer {
        auto_deref: true,
        inner_type_name,
        ..
    } = v_type.get_type_information()
    {
        (inner_type_name.clone(), true)
    } else {
        (v_type.get_name().to_string(), false)
    };

    StatementAnnotation::Variable {
        qualified_name: v.get_qualified_name().into(),
        resulting_type: effective_type_name,
        constant: v.is_constant() || constant_override,
        is_auto_deref,
    }
}

fn to_function_annotation(it: &ImplementationIndexEntry, index: &Index) -> StatementAnnotation {
    StatementAnnotation::Function {
        qualified_name: it.get_call_name().into(),
        return_type: index
            .find_return_type(it.get_call_name())
            .map(|it| it.get_name())
            .unwrap_or(VOID_TYPE)
            .into(),
    }
}

fn get_int_type_name_for(value: i128) -> &'static str {
    if i32::MIN as i128 <= value && i32::MAX as i128 >= value {
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
        assert_eq!(get_int_type_name_for(i128::pow(2, 8) - 1), "DINT");
        assert_eq!(get_int_type_name_for(i128::pow(2, 8)), "DINT");
        assert_eq!(get_int_type_name_for(i128::pow(2, 16) - 1), "DINT");
        assert_eq!(get_int_type_name_for(i128::pow(2, 16)), "DINT");
        assert_eq!(get_int_type_name_for(i128::pow(2, 31) - 1), "DINT");
        assert_eq!(get_int_type_name_for(i128::pow(2, 31)), "LINT");
        assert_eq!(get_int_type_name_for(i128::pow(2, 32)), "LINT");
        assert_eq!(get_int_type_name_for(i64::MAX as i128), "LINT");
    }
}
