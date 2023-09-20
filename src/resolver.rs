// Copyright (c) 2021 Ghaith Hachem and Mathias Rieder

//! Resolves (partial) expressions & statements and annotates the resulting types
//!
//! Recursively visits all statements and expressions of a `CompilationUnit` and
//! records all resulting types associated with the statement's id.

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use indexmap::{IndexMap, IndexSet};
use plc_ast::{
    ast::{
        self, flatten_expression_list, Assignment, AstFactory, AstId, AstNode, AstStatement,
        BinaryExpression, CastStatement, CompilationUnit, DataType, DataTypeDeclaration, DirectAccessType,
        Operator, Pou, ReferenceAccess, ReferenceExpr, TypeNature, UserTypeDeclaration, Variable,
    },
    control_statements::{AstControlStatement, ReturnStatement},
    literals::{Array, AstLiteral, StringValue},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;
use plc_util::convention::internal_type_name;

pub mod const_evaluator;
pub mod generics;

use crate::{
    builtins::{self, BuiltIn},
    index::{ArgumentType, Index, PouIndexEntry, VariableIndexEntry, VariableType},
    typesystem::{
        self, get_bigger_type, DataTypeInformation, InternalType, StringEncoding, StructSource, BOOL_TYPE,
        BYTE_TYPE, DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, DWORD_TYPE, LINT_TYPE, LREAL_TYPE, LWORD_TYPE,
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
pub struct VisitorContext<'s> {
    /// the type_name of the context for a reference (e.g. `a.b` where `a`'s type is the context of `b`)
    qualifier: Option<String>,
    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU`'s body)
    pou: Option<&'s str>,
    /// special context of the left-hand-side of an assignment in call statements
    /// Inside the left hand side of an assignment is in the context of the call's POU
    /// `foo(a := a)` actually means: `foo(foo.a := POU.a)`
    lhs: Option<&'s str>,

    /// true if the expression passed a constant-variable on the way
    /// e.g. true for `x` if x is declared in a constant block
    /// e.g. true for `a.b.c` if either a,b or c is declared in a constant block
    constant: bool,

    /// true the visitor entered a body (so no declarations)
    in_body: bool,

    id_provider: IdProvider,

    // what's the current strategy for resolving
    resolve_strategy: Vec<ResolvingScope>,
}

impl<'s> VisitorContext<'s> {
    /// returns a copy of the current context and changes the `current_qualifier` to the given qualifier
    fn with_qualifier(&self, qualifier: String) -> VisitorContext<'s> {
        VisitorContext {
            pou: self.pou,
            qualifier: Some(qualifier),
            lhs: self.lhs,
            constant: false,
            in_body: self.in_body,
            id_provider: self.id_provider.clone(),
            resolve_strategy: self.resolve_strategy.clone(),
        }
    }

    /// returns a copy of the current context and changes the `current_pou` to the given pou
    fn with_pou(&self, pou: &'s str) -> VisitorContext<'s> {
        VisitorContext {
            pou: Some(pou),
            qualifier: self.qualifier.clone(),
            lhs: self.lhs,
            constant: false,
            in_body: self.in_body,
            id_provider: self.id_provider.clone(),
            resolve_strategy: self.resolve_strategy.clone(),
        }
    }

    /// returns a copy of the current context and changes the `lhs_pou` to the given pou
    fn with_lhs(&self, lhs_pou: &'s str) -> VisitorContext<'s> {
        VisitorContext {
            pou: self.pou,
            qualifier: self.qualifier.clone(),
            lhs: Some(lhs_pou),
            constant: false,
            in_body: self.in_body,
            id_provider: self.id_provider.clone(),
            resolve_strategy: self.resolve_strategy.clone(),
        }
    }

    /// returns a copy of the current context and changes the `is_call` to true
    fn with_const(&self, const_state: bool) -> VisitorContext<'s> {
        VisitorContext {
            pou: self.pou,
            qualifier: self.qualifier.clone(),
            lhs: self.lhs,
            constant: const_state,
            in_body: self.in_body,
            id_provider: self.id_provider.clone(),
            resolve_strategy: self.resolve_strategy.clone(),
        }
    }

    // returns a copy of the current context and sets the in_body field to true
    fn enter_body(&self) -> Self {
        VisitorContext {
            pou: self.pou,
            qualifier: self.qualifier.clone(),
            lhs: self.lhs,
            constant: self.constant,
            in_body: true,
            id_provider: self.id_provider.clone(),
            resolve_strategy: self.resolve_strategy.clone(),
        }
    }

    // returns a copy of the current context and sets the resolve_strategy field to the given strategies
    fn with_resolving_strategy(&self, resolve_strategy: Vec<ResolvingScope>) -> Self {
        VisitorContext {
            pou: self.pou,
            qualifier: self.qualifier.clone(),
            lhs: self.lhs,
            constant: self.constant,
            in_body: true,
            id_provider: self.id_provider.clone(),
            resolve_strategy,
        }
    }

    fn is_in_a_body(&self) -> bool {
        self.in_body
    }
}

pub struct TypeAnnotator<'i> {
    pub(crate) index: &'i Index,
    pub(crate) annotation_map: AnnotationMapImpl,
    string_literals: StringLiterals,
    dependencies: IndexSet<Dependency>,
}

impl TypeAnnotator<'_> {
    pub fn annotate(&mut self, s: &AstNode, annotation: StatementAnnotation) {
        match &annotation {
            StatementAnnotation::Function { return_type, qualified_name, call_name } => {
                let name = call_name.as_ref().unwrap_or(qualified_name);
                self.dependencies.insert(Dependency::Call(name.to_string()));
                self.dependencies.extend(self.get_datatype_dependencies(name, IndexSet::new()));
                self.dependencies.extend(self.get_datatype_dependencies(return_type, IndexSet::new()));
            }
            StatementAnnotation::Program { qualified_name } => {
                self.dependencies.insert(Dependency::Call(qualified_name.to_string()));
                self.dependencies.extend(self.get_datatype_dependencies(qualified_name, IndexSet::new()));
            }
            StatementAnnotation::Variable { resulting_type, qualified_name, argument_type, .. } => {
                if matches!(argument_type.get_inner(), VariableType::Global) {
                    self.dependencies.extend(self.get_datatype_dependencies(resulting_type, IndexSet::new()));
                    self.dependencies.insert(Dependency::Variable(qualified_name.to_string()));
                }
            }
            StatementAnnotation::Value { resulting_type } => {
                if let Some(dt) = self
                    .index
                    .find_type(resulting_type)
                    .or_else(|| self.annotation_map.new_index.find_type(resulting_type))
                {
                    self.dependencies.insert(Dependency::Datatype(dt.get_name().to_string()));
                }
            }
            _ => (),
        };
        self.annotation_map.annotate(s, annotation);
    }

    fn visit_compare_statement(&mut self, ctx: &VisitorContext, statement: &AstNode) {
        let AstStatement::BinaryExpression(BinaryExpression { operator, left, right }) = statement.get_stmt()
        else {
            return;
        };
        let mut ctx = ctx.clone();
        let call_statement = match operator {
            // a <> b expression is handled as Not(Equal(a,b))
            Operator::NotEqual => AstFactory::create_not_expression(
                self.create_typed_compare_call_statement(&mut ctx, &Operator::Equal, left, right, statement),
                statement.get_location(),
                ctx.id_provider.next_id(),
            ),
            // a <= b expression is handled as a = b OR a < b
            Operator::LessOrEqual => AstFactory::create_or_expression(
                self.create_typed_compare_call_statement(&mut ctx, &Operator::Equal, left, right, statement),
                self.create_typed_compare_call_statement(&mut ctx, &Operator::Less, left, right, statement),
            ),
            // a >= b expression is handled as a = b OR a > b
            Operator::GreaterOrEqual => AstFactory::create_or_expression(
                self.create_typed_compare_call_statement(&mut ctx, &Operator::Equal, left, right, statement),
                self.create_typed_compare_call_statement(
                    &mut ctx,
                    &Operator::Greater,
                    left,
                    right,
                    statement,
                ),
            ),
            _ => self.create_typed_compare_call_statement(&mut ctx, operator, left, right, statement),
        };
        self.visit_statement(&ctx, &call_statement);
        self.update_expected_types(self.index.get_type_or_panic(typesystem::BOOL_TYPE), &call_statement);
        self.annotate(statement, StatementAnnotation::ReplacementAst { statement: call_statement });
        self.update_expected_types(self.index.get_type_or_panic(typesystem::BOOL_TYPE), statement);
    }

    /// tries to call one of the EQUAL_XXX, LESS_XXX, GREATER_XXX functions for the
    /// given type (of left). The given operator has to be a comparison-operator
    fn create_typed_compare_call_statement(
        &self,
        ctx: &mut VisitorContext,
        operator: &Operator,
        left: &AstNode,
        right: &AstNode,
        statement: &AstNode,
    ) -> AstNode {
        let left_type = self
            .annotation_map
            .get_type_hint(left, self.index)
            .unwrap_or_else(|| self.annotation_map.get_type_or_void(left, self.index));
        let cmp_function_name = crate::typesystem::get_equals_function_name_for(
            left_type.get_type_information().get_name(),
            operator,
        );

        cmp_function_name
            .map(|name| {
                AstFactory::create_call_to(
                    name,
                    vec![left.clone(), right.clone()],
                    ctx.id_provider.next_id(),
                    left.get_id(),
                    &statement.get_location(),
                )
            })
            .unwrap_or(AstFactory::create_empty_statement(
                statement.get_location(),
                ctx.id_provider.next_id(),
            ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementAnnotation {
    /// an expression that resolves to a certain type (e.g. `a + b` --> `INT`)
    Value {
        resulting_type: String,
    },
    /// a reference that resolves to a declared variable (e.g. `a` --> `PLC_PROGRAM.a`)
    Variable {
        /// the name of the variable's type (e.g. `"INT"`)
        resulting_type: String,
        /// the fully qualified name of this variable (e.g. `"MyFB.a"`)
        qualified_name: String,
        /// denotes whether this variable is declared as a constant
        constant: bool,
        /// denotes the variable type of this variable, hence whether it is an input, output, etc.
        argument_type: ArgumentType,
        /// denotes whether this variable-reference should be automatically dereferenced when accessed
        is_auto_deref: bool,
    },
    /// a reference to a function
    Function {
        /// The defined return type of the function
        return_type: String,
        /// The defined qualified name of the function
        qualified_name: String,
        /// The call name of the function iff it defers from the qualified name (generics)
        call_name: Option<String>,
    },
    /// a reference to a type (e.g. `INT`)
    Type {
        type_name: String,
    },
    /// a reference to a program call or reference (e.g. `PLC_PRG`)
    Program {
        qualified_name: String,
    },
    ReplacementAst {
        statement: AstNode,
    },
}

impl StatementAnnotation {
    /// constructs a new StatementAnnotation::Value with the given type_name
    /// this is a convinience method to take a &str and clones it itself
    pub fn value(type_name: &str) -> Self {
        StatementAnnotation::new_value(type_name.to_string())
    }

    /// constructs a new StatementAnnotation::Value with the given type_name
    pub fn new_value(type_name: String) -> Self {
        StatementAnnotation::Value { resulting_type: type_name }
    }

    pub fn is_const(&self) -> bool {
        match self {
            StatementAnnotation::Variable { constant, .. } => *constant,
            _ => false,
        }
    }

    pub fn data_type(type_name: &str) -> Self {
        StatementAnnotation::Type { type_name: type_name.into() }
    }
}

impl From<&PouIndexEntry> for StatementAnnotation {
    fn from(e: &PouIndexEntry) -> Self {
        match e {
            PouIndexEntry::Program { name, .. } => {
                StatementAnnotation::Program { qualified_name: name.to_string() }
            }
            PouIndexEntry::FunctionBlock { name, .. } => {
                StatementAnnotation::Type { type_name: name.to_string() }
            }
            PouIndexEntry::Function { name, return_type, .. } => StatementAnnotation::Function {
                return_type: return_type.to_string(),
                qualified_name: name.to_string(),
                call_name: None,
            },
            PouIndexEntry::Class { name, .. } => {
                StatementAnnotation::Program { qualified_name: name.to_string() }
            }
            PouIndexEntry::Method { name, return_type, .. } => StatementAnnotation::Function {
                return_type: return_type.to_string(),
                qualified_name: name.to_string(),
                call_name: None,
            },
            PouIndexEntry::Action { name, .. } => {
                StatementAnnotation::Program { qualified_name: name.to_string() }
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
//TODO: Maybe refactor to struct
pub enum Dependency {
    Datatype(String),
    Call(String),
    Variable(String),
}

impl Dependency {
    pub fn get_name(&self) -> &str {
        match self {
            Dependency::Datatype(name) | Dependency::Call(name) | Dependency::Variable(name) => name.as_str(),
        }
    }
}

pub trait AnnotationMap {
    fn get(&self, s: &AstNode) -> Option<&StatementAnnotation>;

    fn get_hint(&self, s: &AstNode) -> Option<&StatementAnnotation>;

    fn get_hidden_function_call(&self, s: &AstNode) -> Option<&AstNode>;

    fn get_type_or_void<'i>(&'i self, s: &AstNode, index: &'i Index) -> &'i typesystem::DataType {
        self.get_type(s, index).unwrap_or_else(|| index.get_void_type())
    }

    fn get_hint_or_void<'i>(&'i self, s: &AstNode, index: &'i Index) -> &'i typesystem::DataType {
        self.get_type_hint(s, index).unwrap_or_else(|| index.get_void_type())
    }

    fn get_type_hint<'i>(&self, s: &AstNode, index: &'i Index) -> Option<&'i typesystem::DataType> {
        self.get_hint(s).and_then(|it| self.get_type_for_annotation(index, it))
    }

    fn get_type<'i>(&'i self, s: &AstNode, index: &'i Index) -> Option<&'i typesystem::DataType> {
        self.get(s).and_then(|it| self.get_type_for_annotation(index, it))
    }

    fn get_type_for_annotation<'a>(
        &self,
        index: &'a Index,
        annotation: &StatementAnnotation,
    ) -> Option<&'a typesystem::DataType> {
        self.get_type_name_for_annotation(annotation).and_then(|type_name| index.get_type(type_name).ok())
    }

    fn get_type_name_for_annotation<'a>(&'a self, annotation: &'a StatementAnnotation) -> Option<&'a str> {
        match annotation {
            StatementAnnotation::Value { resulting_type } => Some(resulting_type.as_str()),
            StatementAnnotation::Variable { resulting_type, .. } => Some(resulting_type.as_str()),
            StatementAnnotation::ReplacementAst { statement } => self
                .get_hint(statement)
                .or_else(|| self.get(statement))
                .and_then(|it| self.get_type_name_for_annotation(it)),
            StatementAnnotation::Program { qualified_name } => Some(qualified_name.as_str()),
            StatementAnnotation::Function { .. } => None,
            StatementAnnotation::Type { type_name } => Some(type_name),
        }
    }

    /// returns the name of the callable that is refered by the given statemt
    /// or none if this thing may not be callable
    fn get_call_name(&self, s: &AstNode) -> Option<&str> {
        match self.get(s) {
            Some(StatementAnnotation::Function { qualified_name, call_name, .. }) => {
                call_name.as_ref().map(String::as_str).or(Some(qualified_name.as_str()))
            }
            Some(StatementAnnotation::Program { qualified_name }) => Some(qualified_name.as_str()),
            Some(StatementAnnotation::Variable { resulting_type, .. }) => Some(resulting_type.as_str()),
            // this is used for call statements on array access
            Some(StatementAnnotation::Value { resulting_type }) => Some(resulting_type.as_str()),
            _ => None,
        }
    }

    fn get_qualified_name(&self, s: &AstNode) -> Option<&str> {
        match self.get(s) {
            Some(StatementAnnotation::Function { qualified_name, .. }) => Some(qualified_name.as_str()),
            _ => self.get_call_name(s),
        }
    }

    fn has_type_annotation(&self, s: &AstNode) -> bool;

    fn get_generic_nature(&self, s: &AstNode) -> Option<&TypeNature>;
}

#[derive(Debug)]
pub struct AstAnnotations {
    annotation_map: AnnotationMapImpl,
    bool_id: AstId,

    bool_annotation: StatementAnnotation,
}

impl AnnotationMap for AstAnnotations {
    fn get(&self, s: &AstNode) -> Option<&StatementAnnotation> {
        if s.get_id() == self.bool_id {
            Some(&self.bool_annotation)
        } else {
            self.annotation_map.get(s)
        }
    }

    fn get_hint(&self, s: &AstNode) -> Option<&StatementAnnotation> {
        if s.get_id() == self.bool_id {
            Some(&self.bool_annotation)
        } else {
            self.annotation_map.get_hint(s)
        }
    }

    fn get_hidden_function_call(&self, s: &AstNode) -> Option<&AstNode> {
        self.annotation_map.get_hidden_function_call(s)
    }

    fn has_type_annotation(&self, s: &AstNode) -> bool {
        self.annotation_map.has_type_annotation(s)
    }

    fn get_generic_nature(&self, s: &AstNode) -> Option<&TypeNature> {
        self.annotation_map.get_generic_nature(s)
    }
}

impl AstAnnotations {
    pub fn new(annotation_map: AnnotationMapImpl, bool_id: AstId) -> Self {
        AstAnnotations { annotation_map, bool_id, bool_annotation: StatementAnnotation::value(BOOL_TYPE) }
    }

    pub fn get_bool_id(&self) -> AstId {
        self.bool_id
    }
}

#[derive(Default, Debug)]
pub struct AnnotationMapImpl {
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

    /// A map from a call to the generic function name of that call
    generic_nature_map: IndexMap<AstId, TypeNature>,

    /// maps a function to a statement
    ///
    /// currently used for `SubRange`check functions
    /// these functions are not called directly and are therefore maped to the corresponding statement here
    /// example:
    /// FUNCTION CheckRangeUnsigned : UDINT
    /// ...
    /// x : BYTE(0..100);
    /// x := 10; // a call to `CheckRangeUnsigned` is maped to `10`
    hidden_function_calls: IndexMap<AstId, AstNode>,

    //An index of newly created types
    pub new_index: Index,
}

impl AnnotationMapImpl {
    /// creates a new empty AnnotationMap
    pub fn new() -> Self {
        Default::default()
    }

    pub fn import(&mut self, other: AnnotationMapImpl) {
        self.type_map.extend(other.type_map);
        self.type_hint_map.extend(other.type_hint_map);
        self.hidden_function_calls.extend(other.hidden_function_calls);
        self.new_index.import(other.new_index);
    }

    /// annotates the given statement (using it's `get_id()`) with the given type-name
    pub fn annotate(&mut self, s: &AstNode, annotation: StatementAnnotation) {
        self.type_map.insert(s.get_id(), annotation);
    }

    pub fn annotate_type_hint(&mut self, s: &AstNode, annotation: StatementAnnotation) {
        self.type_hint_map.insert(s.get_id(), annotation);
    }

    /// annotates the given statement s with the call-statement f so codegen can generate
    /// a hidden call f instead of generating s
    pub fn annotate_hidden_function_call(&mut self, s: &AstNode, f: AstNode) {
        self.hidden_function_calls.insert(s.get_id(), f);
    }

    /// Annotates the ast statement with its original generic nature
    pub fn add_generic_nature(&mut self, s: &AstNode, nature: TypeNature) {
        self.generic_nature_map.insert(s.get_id(), nature);
    }
}

impl AnnotationMap for AnnotationMapImpl {
    fn get(&self, s: &AstNode) -> Option<&StatementAnnotation> {
        self.type_map.get(&s.get_id())
    }

    fn get_hint(&self, s: &AstNode) -> Option<&StatementAnnotation> {
        self.type_hint_map.get(&s.get_id())
    }

    /// returns the function call previously annoted on s via annotate_hidden_function_call(...)
    fn get_hidden_function_call(&self, s: &AstNode) -> Option<&AstNode> {
        self.hidden_function_calls.get(&s.get_id())
    }

    fn get_type<'i>(&'i self, s: &AstNode, index: &'i Index) -> Option<&'i typesystem::DataType> {
        self.get(s).and_then(|it| {
            self.get_type_for_annotation(index, it)
                .or_else(|| self.get_type_for_annotation(&self.new_index, it))
        })
    }

    fn has_type_annotation(&self, s: &AstNode) -> bool {
        self.type_map.contains_key(&s.get_id())
    }

    fn get_generic_nature(&self, s: &AstNode) -> Option<&TypeNature> {
        self.generic_nature_map.get(&s.get_id())
    }
}

#[derive(Default)]
pub struct StringLiterals {
    pub utf08: HashSet<String>,
    pub utf16: HashSet<String>,
}

impl StringLiterals {
    pub fn import(&mut self, other: StringLiterals) {
        self.utf08.extend(other.utf08);
        self.utf16.extend(other.utf16);
    }
}

impl<'i> TypeAnnotator<'i> {
    /// constructs a new TypeAnnotater that works with the given index for type-lookups
    fn new(index: &'i Index) -> TypeAnnotator<'i> {
        TypeAnnotator {
            annotation_map: AnnotationMapImpl::new(),
            index,
            dependencies: IndexSet::new(),
            string_literals: StringLiterals { utf08: HashSet::new(), utf16: HashSet::new() },
        }
    }

    /// annotates the given AST elements with the type-name resulting for the statements/expressions.
    /// Returns an AnnotationMap with the resulting types for all visited Statements. See `AnnotationMap`
    pub fn visit_unit(
        index: &Index,
        unit: &'i CompilationUnit,
        id_provider: IdProvider,
    ) -> (AnnotationMapImpl, IndexSet<Dependency>, StringLiterals) {
        let mut visitor = TypeAnnotator::new(index);
        let ctx = &VisitorContext {
            pou: None,
            qualifier: None,
            lhs: None,
            constant: false,
            in_body: false,
            id_provider,
            resolve_strategy: ResolvingScope::default_scopes(),
        };

        for global_variable in unit.global_vars.iter().flat_map(|it| it.variables.iter()) {
            visitor.dependencies.insert(Dependency::Variable(global_variable.name.to_string()));
            visitor.visit_variable(ctx, global_variable);
        }

        for pou in &unit.units {
            visitor.visit_pou(ctx, pou);
        }

        for t in &unit.user_types {
            visitor.visit_user_type_declaration(t, ctx);
        }

        let body_ctx = ctx.enter_body();
        for i in &unit.implementations {
            visitor.dependencies.extend(visitor.get_datatype_dependencies(&i.name, IndexSet::new()));
            i.statements.iter().for_each(|s| visitor.visit_statement(&body_ctx.with_pou(i.name.as_str()), s));
        }

        // enum initializers may have been introduced by the visitor (indexer)
        // so we should try to resolve and type-annotate them here as well
        for enum_element in
            index.get_global_qualified_enums().values().filter(|it| it.is_in_unit(&unit.file_name))
        {
            //Add to dependency map
            visitor.dependencies.insert(Dependency::Variable(enum_element.get_qualified_name().to_string()));
            if let Some((Some(statement), scope)) =
                enum_element.initial_value.map(|i| index.get_const_expressions().find_expression(&i))
            {
                if visitor.annotation_map.get(statement).is_none() {
                    panic!("new expression we did not visit yet")
                }

                if let Some(scope) = scope {
                    visitor.visit_statement(&ctx.with_pou(scope), statement);
                } else {
                    visitor.visit_statement(ctx, statement);
                }
            }
        }

        (visitor.annotation_map, visitor.dependencies, visitor.string_literals)
    }

    fn visit_user_type_declaration(&mut self, user_data_type: &UserTypeDeclaration, ctx: &VisitorContext) {
        self.visit_data_type(ctx, &user_data_type.data_type);
        if let Some(name) = user_data_type.data_type.get_name() {
            let ctx = &ctx.with_pou(name);
            if let Some((initializer, name)) =
                user_data_type.initializer.as_ref().zip(user_data_type.data_type.get_name())
            {
                self.visit_statement(ctx, initializer);

                //update the type-hint for the initializer
                if let Some(right_type) = self.index.find_effective_type_by_name(name) {
                    self.update_expected_types(right_type, initializer);
                }
            }
        } else {
            unreachable!("datatype without a name");
        }
    }

    fn visit_pou(&mut self, ctx: &VisitorContext, pou: &'i Pou) {
        self.dependencies.insert(Dependency::Datatype(pou.name.clone()));
        //TODO dependency on super class
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
        ctx: &VisitorContext,
        annotated_left_side: &AstNode,
        right_side: &AstNode,
    ) {
        if let Some(expected_type) = self.annotation_map.get_type(annotated_left_side, self.index).cloned() {
            // for assignments on SubRanges check if there are range type check functions
            if let DataTypeInformation::SubRange { sub_range, .. } = expected_type.get_type_information() {
                if let Some(statement) = self
                    .index
                    .find_range_check_implementation_for(expected_type.get_type_information())
                    .map(|f| {
                        AstFactory::create_call_to_check_function_ast(
                            f.get_call_name(),
                            right_side.clone(),
                            sub_range.clone(),
                            &annotated_left_side.get_location(),
                            ctx.id_provider.clone(),
                        )
                    })
                {
                    self.visit_call_statement(&statement, ctx);
                    self.update_right_hand_side(&expected_type, &statement);
                    self.annotation_map.annotate_hidden_function_call(right_side, statement);
                } else {
                    self.update_right_hand_side(&expected_type, right_side);
                }
            } else {
                self.update_right_hand_side(&expected_type, right_side);
            }
        }
    }

    fn update_right_hand_side(&mut self, expected_type: &typesystem::DataType, right_side: &AstNode) {
        //annotate the right-hand side as a whole
        self.annotation_map
            .annotate_type_hint(right_side, StatementAnnotation::value(expected_type.get_name()));

        //dive into the right hand side
        self.update_expected_types(expected_type, right_side);
    }

    /// updates the expected types of statements on the right side of an assignment
    /// e.g. x : ARRAY [0..1] OF BYTE := [2,3];
    fn update_expected_types(&mut self, expected_type: &typesystem::DataType, statement: &AstNode) {
        //see if we need to dive into it
        match statement.get_stmt() {
            AstStatement::Literal(AstLiteral::Array(Array { elements: Some(elements) }), ..) => {
                //annotate the literal-array itself
                self.annotation_map
                    .annotate_type_hint(statement, StatementAnnotation::value(expected_type.get_name()));
                //TODO exprssionList and MultipliedExpressions are a mess!
                if matches!(
                    elements.get_stmt(),
                    AstStatement::ExpressionList(..) | AstStatement::MultipliedStatement(..)
                ) {
                    self.annotation_map
                        .annotate_type_hint(elements, StatementAnnotation::value(expected_type.get_name()));
                }
                //annotate the array's member elements with the array's inner type
                if let DataTypeInformation::Array { inner_type_name, .. } =
                    expected_type.get_type_information()
                {
                    if let Some(inner_type) = self.index.find_effective_type_by_name(inner_type_name) {
                        self.update_expected_types(inner_type, elements);
                    }
                }
            }
            AstStatement::Assignment(Assignment { left, right }, ..) => {
                // struct initialization (left := right)
                // find out left's type and update a type hint for right
                if let (
                    typesystem::DataTypeInformation::Struct { name: qualifier, .. },
                    Some(variable_name),
                ) = (expected_type.get_type_information(), left.as_ref().get_flat_reference_name())
                {
                    if let Some(v) = self.index.find_member(qualifier, variable_name) {
                        if let Some(target_type) = self.index.find_effective_type_by_name(v.get_type_name()) {
                            self.annotate(left.as_ref(), to_variable_annotation(v, self.index, false));
                            self.annotation_map.annotate_type_hint(
                                right.as_ref(),
                                StatementAnnotation::value(v.get_type_name()),
                            );
                            self.update_expected_types(target_type, right);
                        }
                    }
                }
            }
            AstStatement::MultipliedStatement(data, ..) => {
                // n(elements)
                //annotate the type to all multiplied elements
                for ele in AstNode::get_as_list(&data.element) {
                    self.update_expected_types(expected_type, ele);
                }
            }
            AstStatement::ExpressionList(expressions, ..) => {
                //annotate the type to all elements
                for ele in expressions {
                    self.update_expected_types(expected_type, ele);
                }
            }
            AstStatement::RangeStatement(data, ..) => {
                self.update_expected_types(expected_type, &data.start);
                self.update_expected_types(expected_type, &data.end);
            }
            AstStatement::Literal(AstLiteral::Integer { .. }, ..) => {
                //special case -> promote a literal-Integer directly, not via type-hint
                // (avoid later cast)
                if expected_type.get_type_information().is_float() {
                    self.annotate(statement, StatementAnnotation::value(expected_type.get_name()))
                } else if let DataTypeInformation::Array { inner_type_name, .. } =
                    expected_type.get_type_information()
                {
                    self.annotation_map
                        .annotate_type_hint(statement, StatementAnnotation::value(inner_type_name))
                } else {
                    //annotate the statement, whatever it is
                    self.annotation_map
                        .annotate_type_hint(statement, StatementAnnotation::value(expected_type.get_name()))
                }
            }
            AstStatement::Literal(AstLiteral::String { .. }, ..) | AstStatement::BinaryExpression { .. } => {
                // needed if we try to initialize an array with an expression-list
                // without we would annotate a false type this would leed to an error in expression_generator
                if let DataTypeInformation::Array { inner_type_name, .. } =
                    expected_type.get_type_information()
                {
                    self.annotation_map
                        .annotate_type_hint(statement, StatementAnnotation::value(inner_type_name))
                } else {
                    //annotate the statement, whatever it is
                    self.annotation_map
                        .annotate_type_hint(statement, StatementAnnotation::value(expected_type.get_name()))
                }
            }

            _ => {
                //annotate the statement, whatever it is
                self.annotation_map
                    .annotate_type_hint(statement, StatementAnnotation::value(expected_type.get_name()))
            }
        }
    }

    fn visit_variable(&mut self, ctx: &VisitorContext, variable: &Variable) {
        self.visit_data_type_declaration(ctx, &variable.data_type_declaration);
        if let Some(initializer) = variable.initializer.as_ref() {
            // annotate a type-hint for the initializer, it should be the same type as the variable
            // e.g. x : BYTE := 7 + 3;  --> 7+3 should be cast into a byte
            if let Some(expected_type) = self
                .index
                .find_variable(ctx.qualifier.as_deref().or(ctx.pou), &[variable.name.as_str()])
                .and_then(|ve| self.index.find_effective_type_by_name(ve.get_type_name()))
            {
                //Create a new context with the left operator being the target variable type, and the
                //right side being the local context
                let ctx = ctx.with_lhs(expected_type.get_name());

                if initializer.is_default_value() {
                    // the default-placeholder must be annotated with the correct type,
                    // it will be replaced by the appropriate literal later
                    self.annotate(initializer, StatementAnnotation::value(expected_type.get_name()));
                } else {
                    self.visit_statement(&ctx, initializer);
                }

                self.annotation_map
                    .annotate_type_hint(initializer, StatementAnnotation::value(expected_type.get_name()));
                self.update_expected_types(expected_type, initializer);

                self.type_hint_for_array_of_structs(expected_type, initializer, &ctx);
            }
        }
    }

    fn type_hint_for_array_of_structs(
        &mut self,
        expected_type: &typesystem::DataType,
        statement: &AstNode,
        ctx: &VisitorContext,
    ) {
        match expected_type.get_type_information() {
            DataTypeInformation::Array { inner_type_name, .. } => {
                let inner_data_type = self.index.get_effective_type_or_void_by_name(inner_type_name);
                // TODO this seems wrong
                let ctx = ctx
                    .with_qualifier(inner_data_type.get_name().to_string())
                    .with_lhs(inner_data_type.get_name());

                if !inner_data_type.get_type_information().is_struct() {
                    return;
                }

                match statement.get_stmt() {
                    AstStatement::Literal(AstLiteral::Array(array)) => match array.elements() {
                        Some(elements) if elements.is_expression_list() => {
                            self.type_hint_for_array_of_structs(expected_type, elements, &ctx)
                        }

                        _ => (),
                    },

                    AstStatement::ExpressionList(expressions) => {
                        for expression in expressions {
                            // annotate with the arrays inner_type
                            let name = inner_data_type.get_name().to_string();
                            let hint = StatementAnnotation::Value { resulting_type: name };
                            self.annotation_map.annotate_type_hint(expression, hint);

                            self.visit_statement(&ctx, expression);
                            self.type_hint_for_array_of_structs(expected_type, expression, &ctx);
                        }
                    }

                    AstStatement::Assignment(Assignment { left, right, .. }) if left.is_reference() => {
                        let AstStatement::Literal(AstLiteral::Array(array)) = right.as_ref().get_stmt()
                        else {
                            return;
                        };
                        let Some(elements) = array.elements() else { return };

                        if let Some(datatype) = self.annotation_map.get_type(left, self.index).cloned() {
                            self.type_hint_for_array_of_structs(&datatype, elements, &ctx);
                        }
                    }

                    _ => (),
                }
            }

            // We _should_ only land here when the variable itself isn't defined as an array (e.g. `foo : STRUCT1`).
            // The initializer of that variable might have an array of struct defined however, hence check it's elements.
            DataTypeInformation::Struct { members, .. } => {
                let flattened = ast::flatten_expression_list(statement);
                for (idx, member) in members.iter().enumerate() {
                    let data_type = self.index.get_effective_type_or_void_by_name(member.get_type_name());
                    if data_type.is_array() {
                        let Some(AstStatement::Assignment(data)) = flattened.get(idx).map(|it| it.get_stmt())
                        else {
                            continue;
                        };
                        self.type_hint_for_array_of_structs(data_type, &data.right, ctx);
                    }
                }
            }

            _ => (),
        }
    }

    fn visit_data_type_declaration(&mut self, ctx: &VisitorContext, declaration: &DataTypeDeclaration) {
        if let Some(name) = declaration.get_name() {
            let deps = self.get_datatype_dependencies(name, IndexSet::new());
            self.dependencies.extend(deps);
        }
        if let DataTypeDeclaration::DataTypeDefinition { data_type, .. } = declaration {
            self.visit_data_type(ctx, data_type);
        }
    }

    fn get_datatype_dependencies(
        &self,
        datatype_name: &str,
        resolved: IndexSet<Dependency>,
    ) -> IndexSet<Dependency> {
        let mut resolved_names = resolved;
        let Some(datatype) = self
            .index
            .find_type(datatype_name)
            .or_else(|| self.annotation_map.new_index.find_type(datatype_name))
        else {
            return resolved_names;
        };
        if resolved_names.insert(Dependency::Datatype(datatype.get_name().to_string())) {
            match datatype.get_type_information() {
                DataTypeInformation::Struct { members, .. } => {
                    for member in members {
                        resolved_names =
                            self.get_datatype_dependencies(member.get_type_name(), resolved_names);
                    }
                    resolved_names
                }
                DataTypeInformation::Array { inner_type_name, .. }
                | DataTypeInformation::Pointer { inner_type_name, .. } => {
                    self.get_datatype_dependencies(inner_type_name, resolved_names)
                }
                _ => {
                    let name = self.index.find_intrinsic_type(datatype.get_type_information()).get_name();
                    self.get_datatype_dependencies(name, resolved_names)
                }
            }
        } else {
            resolved_names
        }
    }

    fn visit_data_type(&mut self, ctx: &VisitorContext, data_type: &DataType) {
        if let Some(name) = data_type.get_name() {
            self.dependencies.insert(Dependency::Datatype(name.to_string()));
        }
        match data_type {
            DataType::StructType { name: Some(name), variables, .. } => {
                let ctx = ctx.with_qualifier(name.clone());
                variables.iter().for_each(|v| self.visit_variable(&ctx, v))
            }
            DataType::ArrayType { referenced_type, .. } => {
                self.visit_data_type_declaration(ctx, referenced_type)
            }
            DataType::VarArgs { referenced_type: Some(referenced_type), .. } => {
                self.visit_data_type_declaration(ctx, referenced_type.as_ref())
            }
            DataType::SubRangeType { referenced_type, bounds: Some(bounds), .. } => {
                if let Some(expected_type) = self.index.find_effective_type_by_name(referenced_type) {
                    self.visit_statement(ctx, bounds);
                    self.update_expected_types(expected_type, bounds);
                }
            }
            DataType::EnumType { elements, name, .. } => {
                let ctx = name.as_ref().map(|n| ctx.with_lhs(n)).unwrap_or(ctx.clone());
                self.visit_statement(&ctx, elements);
            }
            DataType::PointerType { referenced_type, .. } => {
                self.visit_data_type_declaration(ctx, referenced_type.as_ref())
            }
            _ => {}
        }
    }

    pub fn visit_statement(&mut self, ctx: &VisitorContext, statement: &AstNode) {
        self.visit_statement_control(ctx, statement);
    }

    /// annotate a control statement
    fn visit_statement_control(&mut self, ctx: &VisitorContext, statement: &AstNode) {
        match statement.get_stmt() {
            AstStatement::ControlStatement(AstControlStatement::If(stmt), ..) => {
                stmt.blocks.iter().for_each(|b| {
                    self.visit_statement(ctx, b.condition.as_ref());
                    b.body.iter().for_each(|s| self.visit_statement(ctx, s));
                });
                stmt.else_block.iter().for_each(|e| self.visit_statement(ctx, e));
            }
            AstStatement::ControlStatement(AstControlStatement::ForLoop(stmt), ..) => {
                visit_all_statements!(self, ctx, &stmt.counter, &stmt.start, &stmt.end);
                if let Some(by_step) = &stmt.by_step {
                    self.visit_statement(ctx, by_step);
                }
                //Hint annotate start, end and step with the counter's real type
                if let Some(type_name) = self
                    .annotation_map
                    .get_type(&stmt.counter, self.index)
                    .map(typesystem::DataType::get_name)
                {
                    let annotation = StatementAnnotation::value(type_name);
                    self.annotation_map.annotate_type_hint(&stmt.start, annotation.clone());
                    self.annotation_map.annotate_type_hint(&stmt.end, annotation.clone());
                    if let Some(by_step) = &stmt.by_step {
                        self.annotation_map.annotate_type_hint(by_step, annotation);
                    }
                }
                stmt.body.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            AstStatement::ControlStatement(AstControlStatement::WhileLoop(stmt), ..)
            | AstStatement::ControlStatement(AstControlStatement::RepeatLoop(stmt), ..) => {
                self.visit_statement(ctx, &stmt.condition);
                stmt.body.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            AstStatement::ControlStatement(AstControlStatement::Case(stmt), ..) => {
                self.visit_statement(ctx, &stmt.selector);
                let selector_type = self.annotation_map.get_type(&stmt.selector, self.index).cloned();
                stmt.case_blocks.iter().for_each(|b| {
                    self.visit_statement(ctx, b.condition.as_ref());
                    if let Some(selector_type) = &selector_type {
                        self.update_expected_types(selector_type, b.condition.as_ref());
                    }
                    b.body.iter().for_each(|s| self.visit_statement(ctx, s));
                });
                stmt.else_block.iter().for_each(|s| self.visit_statement(ctx, s));
            }
            AstStatement::CaseCondition(condition, ..) => self.visit_statement(ctx, condition),
            _ => {
                self.visit_statement_expression(ctx, statement);
            }
        }
    }

    /// annotate an expression statement
    fn visit_statement_expression(&mut self, ctx: &VisitorContext, statement: &AstNode) {
        match statement.get_stmt() {
            AstStatement::DirectAccess(data, ..) => {
                let ctx = VisitorContext { qualifier: None, ..ctx.clone() };
                visit_all_statements!(self, &ctx, &data.index);
                let access_type = get_direct_access_type(&data.access);
                self.annotate(statement, StatementAnnotation::Value { resulting_type: access_type.into() });
            }
            AstStatement::HardwareAccess(data, ..) => {
                let access_type = get_direct_access_type(&data.access);
                self.annotate(statement, StatementAnnotation::Value { resulting_type: access_type.into() });
            }
            AstStatement::BinaryExpression(data, ..) => {
                visit_all_statements!(self, ctx, &data.left, &data.right);
                let statement_type = {
                    let left_type = self
                        .annotation_map
                        .get_type_hint(&data.left, self.index)
                        .or_else(|| self.annotation_map.get_type(&data.left, self.index))
                        .and_then(|it| self.index.find_effective_type(it))
                        .unwrap_or_else(|| self.index.get_void_type());
                    // do not use for is_pointer() check
                    let l_intrinsic_type =
                        self.index.get_intrinsic_type_by_name(left_type.get_name()).get_type_information();
                    let right_type = self
                        .annotation_map
                        .get_type_hint(&data.right, self.index)
                        .or_else(|| self.annotation_map.get_type(&data.right, self.index))
                        .and_then(|it| self.index.find_effective_type(it))
                        .unwrap_or_else(|| self.index.get_void_type());
                    // do not use for is_pointer() check
                    let r_intrinsic_type =
                        self.index.get_intrinsic_type_by_name(right_type.get_name()).get_type_information();

                    if l_intrinsic_type.is_numerical() && r_intrinsic_type.is_numerical() {
                        let bigger_type = if l_intrinsic_type.is_bool() && r_intrinsic_type.is_bool() {
                            left_type
                        } else {
                            let ty = if left_type.is_bit() && right_type.is_bit() {
                                right_type
                            } else {
                                self.index.get_type_or_panic(DINT_TYPE)
                            };
                            get_bigger_type(
                                get_bigger_type(left_type, right_type, self.index),
                                ty,
                                self.index,
                            )
                        };

                        let target_name = if data.operator.is_bool_type() {
                            BOOL_TYPE.to_string()
                        } else {
                            bigger_type.get_name().to_string()
                        };

                        let bigger_is_left = bigger_type != right_type;
                        let bigger_is_right = bigger_type != left_type;

                        if bigger_is_left || bigger_is_right {
                            // if these types are different we need to update the 'other' type's annotation
                            let bigger_type = bigger_type.clone(); // clone here, so we release the borrow on self
                            if bigger_is_right {
                                self.update_expected_types(&bigger_type, &data.left);
                            }
                            if bigger_is_left {
                                self.update_expected_types(&bigger_type, &data.right);
                            }
                        }

                        Some(target_name)
                    } else if left_type.get_type_information().is_pointer()
                        || right_type.get_type_information().is_pointer()
                    {
                        // get the target type of the binary expression
                        let target_type = if data.operator.is_comparison_operator() {
                            // compare instructions result in BOOL
                            // to generate valid IR code if a pointer is beeing compared to an integer
                            // we need to cast the int to the pointers size
                            if !left_type.get_type_information().is_pointer() {
                                let left_type = left_type.clone(); // clone here, so we release the borrow on self
                                self.annotate_to_pointer_size_if_necessary(&left_type, &data.left);
                            } else if !right_type.get_type_information().is_pointer() {
                                let right_type = right_type.clone(); // clone here, so we release the borrow on self
                                self.annotate_to_pointer_size_if_necessary(&right_type, &data.right);
                            }
                            BOOL_TYPE
                        } else if left_type.get_type_information().is_pointer() {
                            left_type.get_name()
                        } else {
                            right_type.get_name()
                        };
                        Some(target_type.to_string())
                    } else if data.operator.is_comparison_operator() {
                        //Annotate as the function call to XXX_EQUALS/LESS/GREATER..
                        self.visit_compare_statement(ctx, statement);
                        None
                    } else {
                        None
                    }
                };

                if let Some(statement_type) = statement_type {
                    self.annotate(statement, StatementAnnotation::new_value(statement_type));
                }
            }
            AstStatement::UnaryExpression(data, ..) => {
                self.visit_statement(ctx, &data.value);

                let statement_type = if data.operator == Operator::Minus {
                    let inner_type =
                        self.annotation_map.get_type_or_void(&data.value, self.index).get_type_information();

                    //keep the same type but switch to signed
                    typesystem::get_signed_type(inner_type, self.index).map(|it| it.get_name().to_string())
                } else {
                    let inner_type = self
                        .annotation_map
                        .get_type_or_void(&data.value, self.index)
                        .get_type_information()
                        .get_name()
                        .to_string();

                    Some(inner_type)
                };

                if let Some(statement_type) = statement_type {
                    self.annotate(statement, StatementAnnotation::new_value(statement_type));
                }
            }

            AstStatement::ExpressionList(expressions, ..) => {
                expressions.iter().for_each(|e| self.visit_statement(ctx, e))
            }

            AstStatement::RangeStatement(data, ..) => {
                visit_all_statements!(self, ctx, &data.start, &data.end);
            }
            AstStatement::Assignment(data, ..) => {
                self.visit_statement(ctx, &data.right);
                if let Some(lhs) = ctx.lhs {
                    //special context for left hand side
                    self.visit_statement(&ctx.with_pou(lhs).with_lhs(lhs), &data.left);
                } else {
                    self.visit_statement(ctx, &data.left);
                }
                // give a type hint that we want the right side to be stored in the left's type
                self.update_right_hand_side_expected_type(ctx, &data.left, &data.right);
            }
            AstStatement::OutputAssignment(data, ..) => {
                visit_all_statements!(self, ctx, &data.left, &data.right);
                if let Some(lhs) = ctx.lhs {
                    //special context for left hand side
                    self.visit_statement(&ctx.with_pou(lhs), &data.left);
                } else {
                    self.visit_statement(ctx, &data.left);
                }
                self.update_right_hand_side_expected_type(ctx, &data.left, &data.right);
            }
            AstStatement::CallStatement(..) => {
                self.visit_call_statement(statement, ctx);
            }
            AstStatement::CastStatement(CastStatement { target, type_name }, ..) => {
                //see if this type really exists
                let data_type = self.index.find_effective_type_info(type_name);
                let statement_to_annotation = if let Some(DataTypeInformation::Enum { name, .. }) = data_type
                {
                    //enum cast
                    self.visit_statement(&ctx.with_qualifier(name.to_string()), target);
                    //use the type of the target
                    let type_name = self.annotation_map.get_type_or_void(target, self.index).get_name();
                    vec![(statement, type_name.to_string())]
                } else if let Some(t) = data_type {
                    // special handling for unlucky casted-strings where caste-type does not match the literal encoding
                    // ´STRING#"abc"´ or ´WSTRING#'abc'´
                    match (t, target.as_ref().get_stmt()) {
                        (
                            DataTypeInformation::String { encoding: StringEncoding::Utf8, .. },
                            AstStatement::Literal(AstLiteral::String(StringValue {
                                value,
                                is_wide: is_wide @ true,
                            })),
                        )
                        | (
                            DataTypeInformation::String { encoding: StringEncoding::Utf16, .. },
                            AstStatement::Literal(AstLiteral::String(StringValue {
                                value,
                                is_wide: is_wide @ false,
                            })),
                        ) => {
                            // visit the target-statement as if the programmer used the correct quotes to prevent
                            // a utf16 literal-global-variable that needs to be casted back to utf8 or vice versa
                            self.visit_statement(
                                ctx,
                                &AstNode::new_literal(
                                    AstLiteral::new_string(value.clone(), !is_wide),
                                    target.get_id(),
                                    target.get_location(),
                                ),
                            );
                        }
                        _ => {}
                    }
                    vec![(statement, t.get_name().to_string()), (target, t.get_name().to_string())]
                } else {
                    //unknown type? what should we do here?
                    self.visit_statement(ctx, target);
                    vec![]
                };
                for (stmt, annotation) in statement_to_annotation {
                    self.annotate(stmt, StatementAnnotation::new_value(annotation));
                }
            }
            AstStatement::ReferenceExpr(data, ..) => {
                self.visit_reference_expr(&data.access, data.base.as_deref(), statement, ctx);
            }
            AstStatement::ReturnStatement(ReturnStatement { condition }) => {
                if let Some(condition) = condition {
                    self.visit_statement(ctx, condition)
                }
            }
            _ => {
                self.visit_statement_literals(ctx, statement);
            }
        }
    }

    fn visit_reference_expr(
        &mut self,
        access: &ast::ReferenceAccess,
        base: Option<&AstNode>,
        stmt: &AstNode,
        ctx: &VisitorContext,
    ) {
        // first resolve base
        if let Some(base) = base {
            self.visit_statement(ctx, base);
        };

        match (
            access,
            base.and_then(|it| {
                self.annotation_map.get_type(it, self.index).map(|it| it.get_name().to_string())
            }),
        ) {
            (ReferenceAccess::Member(reference), qualifier) => {
                // uppdate the context's const field
                let new_ctx = base.map(|base| ctx.with_const(self.is_const_reference(base, ctx)));
                let new_ctx = new_ctx.as_ref().unwrap_or(ctx);

                if let Some(annotation) =
                    self.resolve_reference_expression(reference.as_ref(), qualifier.as_deref(), new_ctx)
                {
                    self.annotate(stmt, annotation.clone());
                    self.annotate(reference, annotation);
                    // if this was a vla, we update a typehint
                    if self.annotation_map.get_type(stmt, self.index).filter(|it| it.is_vla()).is_some() {
                        self.annotate_vla_hint(new_ctx, stmt);
                    }
                }
            }
            (ReferenceAccess::Cast(target), Some(qualifier)) => {
                // STRING#"abc"
                //  base
                if let Some(base_type) = base.and_then(|base| self.annotation_map.get_type(base, self.index))
                {
                    // if base is an enum, we need to look for members of this specific enum
                    let optional_enum_qualifier = Some(qualifier.as_str()).filter(|_| base_type.is_enum());
                    if ctx.is_in_a_body() {
                        accept_cast_string_literal(&mut self.string_literals, base_type, target);
                    }

                    if let Some(annotation) = self.resolve_reference_expression(
                        target,
                        optional_enum_qualifier,
                        &ctx.with_resolving_strategy(vec![ResolvingScope::Variable]),
                    ) {
                        self.annotate(target.as_ref(), annotation);
                        self.annotate(stmt, StatementAnnotation::value(qualifier.as_str()));

                        if let AstStatement::Literal(..) = target.get_stmt() {
                            // treate casted literals as the casted type
                            self.annotate(target.as_ref(), StatementAnnotation::value(qualifier.as_str()));
                        }
                    }
                }
            }
            (ReferenceAccess::Index(index), Some(base)) => {
                self.visit_statement(ctx, index);
                if let Some(inner_type) = self
                    .index
                    .find_effective_type_info(base.as_str())
                    .and_then(|t| t.get_inner_array_type_name())
                    .and_then(|it| self.index.find_effective_type_by_name(it).map(|it| it.get_name()))
                {
                    self.annotate(stmt, StatementAnnotation::value(inner_type))
                }
            }
            (ReferenceAccess::Deref, _) => {
                if let Some(DataTypeInformation::Pointer { inner_type_name, auto_deref: false, .. }) = base
                    .map(|base| self.annotation_map.get_type_or_void(base, self.index))
                    .map(|it| it.get_type_information())
                {
                    if let Some(inner_type) = self
                        .index
                        .find_effective_type_by_name(inner_type_name)
                        .or(self.annotation_map.new_index.find_effective_type_by_name(inner_type_name))
                    {
                        self.annotate(stmt, StatementAnnotation::value(inner_type.get_name()))
                    }
                }
            }
            (ReferenceAccess::Address, _) => {
                if let Some(inner_type) = base
                    .map(|base| self.annotation_map.get_type_or_void(base, self.index).get_name().to_string())
                {
                    let ptr_type = add_pointer_type(&mut self.annotation_map.new_index, inner_type);
                    self.annotate(stmt, StatementAnnotation::new_value(ptr_type))
                }
            }
            _ => {}
        }
    }

    fn is_const_reference(&self, stmt: &AstNode, ctx: &VisitorContext<'_>) -> bool {
        self.annotation_map
            .get(stmt)
            .map(|it| it.is_const())
            .filter(|it| *it != ctx.constant)
            .unwrap_or(false)
    }

    /// resolves the given reference, under the optional qualifier and returns the resulting
    /// Statement annotation if one can be derived. This method the annotation!
    fn resolve_reference_expression(
        &mut self,
        reference: &AstNode,
        qualifier: Option<&str>,
        ctx: &VisitorContext<'_>,
    ) -> Option<StatementAnnotation> {
        match reference.get_stmt() {
            AstStatement::Identifier(name, ..) => ctx
                .resolve_strategy
                .iter()
                .find_map(|scope| scope.resolve_name(name, qualifier, self.index, ctx)),

            AstStatement::Literal(..) => {
                self.visit_statement_literals(ctx, reference);
                let literal_annotation = self.annotation_map.get(reference).cloned(); // return what we just annotated //TODO not elegant, we need to clone
                if let Some((base_type, literal_type)) =
                    qualifier.and_then(|base| self.index.find_type(base)).zip(
                        literal_annotation
                            .as_ref()
                            .and_then(|a| self.annotation_map.get_type_for_annotation(self.index, a)),
                    )
                {
                    // see if this was casted
                    if base_type != literal_type {
                        return Some(StatementAnnotation::value(base_type.get_name()));
                    }
                }
                literal_annotation
            }

            AstStatement::DirectAccess(data, ..) if qualifier.is_some() => {
                // x.%X1 - bit access
                self.visit_statement(ctx, data.index.as_ref());
                Some(StatementAnnotation::value(get_direct_access_type(&data.access)))
            }
            _ => None,
        }
    }

    /// annotates the vla-statement it with a type hint
    /// referencing the contained array. This is needed to simplify codegen and validation.
    fn annotate_vla_hint(&mut self, ctx: &VisitorContext, statement: &AstNode) {
        let DataTypeInformation::Struct {
            source: StructSource::Internal(InternalType::VariableLengthArray { .. }),
            members,
            ..
        } = self.annotation_map.get_type_or_void(statement, self.index).get_type_information()
        else {
            unreachable!("expected a vla reference, but got {statement:#?}");
        };
        if let DataTypeInformation::Pointer { inner_type_name, .. } = &self
            .index
            .get_effective_type_or_void_by_name(
                members.get(0).expect("internal VLA struct ALWAYS has this member").get_type_name(),
            )
            .get_type_information()
        {
            let Some(qualified_name) = self.annotation_map.get_qualified_name(statement) else {
                unreachable!("VLAs are defined within POUs, such that the qualified name *must* exist")
            };

            let Some(pou) = ctx.pou else { unreachable!("VLA not allowed outside of POUs") };

            let name = if let AstStatement::Identifier(name, ..) = statement.get_stmt() {
                name.as_str()
            } else {
                statement.get_flat_reference_name().expect("must be a reference to a VLA")
            };

            let Some(argument_type) = self
                .index
                .get_pou_members(pou)
                .iter()
                .filter(|it| it.get_name() == name)
                .map(|it| it.get_declaration_type())
                .next()
            else {
                unreachable!()
            };

            let hint_annotation = StatementAnnotation::Variable {
                resulting_type: inner_type_name.to_owned(),
                qualified_name: qualified_name.to_string(),
                constant: false,
                argument_type,
                is_auto_deref: false,
            };
            self.annotation_map.annotate_type_hint(statement, hint_annotation)
        }
    }

    fn visit_call_statement(&mut self, statement: &AstNode, ctx: &VisitorContext) {
        let (operator, parameters_stmt) = if let AstStatement::CallStatement(data, ..) = statement.get_stmt()
        {
            (data.operator.as_ref(), data.parameters.as_deref())
        } else {
            unreachable!("Always a call statement");
        };
        // #604 needed for recursive function calls
        self.visit_statement(&ctx.with_resolving_strategy(ResolvingScope::call_operator_scopes()), operator);
        let operator_qualifier = self.get_call_name(operator);
        //Use the context without the is_call =true
        //TODO why do we start a lhs context here???
        let ctx = ctx.with_lhs(operator_qualifier.as_str());
        let parameters = if let Some(parameters) = parameters_stmt {
            self.visit_statement(&ctx, parameters);
            flatten_expression_list(parameters)
        } else {
            vec![]
        };
        if let Some(annotation) = builtins::get_builtin(&operator_qualifier).and_then(BuiltIn::get_annotation)
        {
            annotation(self, operator, parameters_stmt, ctx.to_owned())
        } else {
            //If builtin, skip this
            let mut generics_candidates: HashMap<String, Vec<String>> = HashMap::new();
            let mut params = vec![];
            let mut parameters = parameters.into_iter();

            // If we are dealing with an action call statement, we need to get the declared parameters from the parent POU in order
            // to annotate them with the correct type hint.
            let operator_qualifier = self
                .index
                .find_implementation_by_name(&operator_qualifier)
                .map(|it| it.get_type_name())
                .unwrap_or(operator_qualifier.as_str());

            for m in self.index.get_declared_parameters(operator_qualifier).into_iter() {
                if let Some(p) = parameters.next() {
                    let type_name = m.get_type_name();
                    if let Some((key, candidate)) =
                        TypeAnnotator::get_generic_candidate(self.index, &self.annotation_map, type_name, p)
                    {
                        generics_candidates
                            .entry(key.to_string())
                            .or_insert_with(std::vec::Vec::new)
                            .push(candidate.to_string())
                    } else {
                        params.push((p, type_name.to_string()))
                    }
                }
            }
            //We possibly did not consume all parameters, see if the variadic arguments are derivable

            match self.index.find_pou(operator_qualifier) {
                Some(pou) if pou.is_variadic() => {
                    //get variadic argument type, if it is generic, update the generic candidates
                    if let Some(type_name) =
                        self.index.get_variadic_member(pou.get_name()).map(VariableIndexEntry::get_type_name)
                    {
                        for parameter in parameters {
                            if let Some((key, candidate)) = TypeAnnotator::get_generic_candidate(
                                self.index,
                                &self.annotation_map,
                                type_name,
                                parameter,
                            ) {
                                generics_candidates
                                    .entry(key.to_string())
                                    .or_insert_with(std::vec::Vec::new)
                                    .push(candidate.to_string())
                            } else {
                                // intrinsic type promotion for variadics in order to be compatible with the C standard.
                                // see ISO/IEC 9899:1999, 6.5.2.2 Function calls (https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1256.pdf)
                                // or https://en.cppreference.com/w/cpp/language/implicit_conversion#Integral_promotion
                                // for more about default argument promotion.

                                // varargs without a type declaration will be annotated "VOID", so in order to check if a
                                // promotion is necessary, we need to first check the type of each parameter. in the case of numerical
                                // types, we promote if the type is smaller than double/i32 (except for booleans).
                                let type_name = if let Some(data_type) =
                                    self.annotation_map.get_type(parameter, self.index)
                                {
                                    match &data_type.information {
                                        DataTypeInformation::Float { .. } => get_bigger_type(
                                            data_type,
                                            self.index.get_type_or_panic(LREAL_TYPE),
                                            self.index,
                                        )
                                        .get_name(),
                                        DataTypeInformation::Integer { .. }
                                            if !&data_type.information.is_bool() =>
                                        {
                                            get_bigger_type(
                                                data_type,
                                                self.index.get_type_or_panic(DINT_TYPE),
                                                self.index,
                                            )
                                            .get_name()
                                        }
                                        _ => type_name,
                                    }
                                } else {
                                    // default to original type in case no type could be found
                                    // and let the validator handle situations that might lead here
                                    type_name
                                };

                                params.push((parameter, type_name.to_string()));
                            }
                        }
                    }
                }
                _ => {}
            }
            for (p, name) in params {
                self.annotate_parameters(p, &name);
            }
            //Attempt to resolve the generic signature here
            self.update_generic_call_statement(
                generics_candidates,
                operator_qualifier,
                operator,
                parameters_stmt,
                ctx.to_owned(),
            );
        }
        if let Some(StatementAnnotation::Function { return_type, .. }) = self.annotation_map.get(operator) {
            if let Some(return_type) = self
                .index
                .find_effective_type_by_name(return_type)
                .or_else(|| self.annotation_map.new_index.find_effective_type_by_name(return_type))
            {
                self.annotate(statement, StatementAnnotation::value(return_type.get_name()));
            }
        }
    }

    fn get_call_name(&mut self, operator: &AstNode) -> String {
        let operator_qualifier = self
            .annotation_map
            .get(operator)
            .and_then(|it| match it {
                StatementAnnotation::Function { qualified_name, call_name, .. } => {
                    call_name.as_ref().cloned().or_else(|| Some(qualified_name.clone()))
                }
                StatementAnnotation::Program { qualified_name } => Some(qualified_name.clone()),
                StatementAnnotation::Variable { resulting_type, .. } => {
                    //lets see if this is a FB
                    self.index
                        .find_pou(resulting_type.as_str())
                        .filter(|it| matches!(it, PouIndexEntry::FunctionBlock { .. }))
                        .map(|it| it.get_name().to_string())
                }
                // call statements on array access "arr[1]()" will return a StatementAnnotation::Value
                StatementAnnotation::Value { resulting_type } => {
                    // make sure we come from an array or function_block access
                    match operator.get_stmt() {
                        AstStatement::ReferenceExpr ( ReferenceExpr{access: ReferenceAccess::Index(_), ..},.. ) => Some(resulting_type.clone()),
                        AstStatement::ReferenceExpr ( ReferenceExpr{access: ReferenceAccess::Deref, ..}, .. ) =>
                        // AstStatement::ArrayAccess { .. } => Some(resulting_type.clone()),
                        // AstStatement::PointerAccess { .. } => {
                            self.index.find_pou(resulting_type.as_str()).map(|it| it.get_name().to_string()),
                        // }
                        _ => None,
                    }
                }
                _ => None,
            })
            .unwrap_or_else(|| VOID_TYPE.to_string());
        operator_qualifier
    }

    pub(crate) fn annotate_parameters(&mut self, p: &AstNode, type_name: &str) {
        if !matches!(p.get_stmt(), AstStatement::Assignment(..) | AstStatement::OutputAssignment(..)) {
            if let Some(effective_member_type) = self.index.find_effective_type_by_name(type_name) {
                //update the type hint
                self.annotation_map
                    .annotate_type_hint(p, StatementAnnotation::value(effective_member_type.get_name()))
            }
        }
    }

    /// annotate a literal statement
    fn visit_statement_literals(&mut self, ctx: &VisitorContext, statement: &AstNode) {
        match statement.get_stmt() {
            AstStatement::Literal(kind, ..) => {
                match kind {
                    AstLiteral::Bool { .. } => {
                        self.annotate(statement, StatementAnnotation::value(BOOL_TYPE));
                    }

                    AstLiteral::String(StringValue { is_wide, value, .. }) => {
                        let string_type_name =
                            register_string_type(&mut self.annotation_map.new_index, *is_wide, value.len());
                        self.annotate(statement, StatementAnnotation::new_value(string_type_name));

                        //collect literals so we can generate global constants later
                        if ctx.is_in_a_body() {
                            if *is_wide {
                                self.string_literals.utf16.insert(value.to_string());
                            } else {
                                self.string_literals.utf08.insert(value.to_string());
                            }
                        }
                    }
                    AstLiteral::Integer(value) => {
                        self.annotate(statement, StatementAnnotation::value(get_int_type_name_for(*value)));
                    }
                    AstLiteral::Time { .. } => {
                        self.annotate(statement, StatementAnnotation::value(TIME_TYPE))
                    }
                    AstLiteral::TimeOfDay { .. } => {
                        self.annotate(statement, StatementAnnotation::value(TIME_OF_DAY_TYPE));
                    }
                    AstLiteral::Date { .. } => {
                        self.annotate(statement, StatementAnnotation::value(DATE_TYPE));
                    }
                    AstLiteral::DateAndTime { .. } => {
                        self.annotate(statement, StatementAnnotation::value(DATE_AND_TIME_TYPE));
                    }
                    AstLiteral::Real(value) => {
                        self.annotate(statement, StatementAnnotation::value(get_real_type_name_for(value)));
                    }
                    AstLiteral::Array(Array { elements: Some(elements), .. }) => {
                        self.visit_statement(ctx, elements.as_ref());
                        //TODO as of yet we have no way to derive a name that reflects a fixed size array
                    }
                    _ => {} // ignore literalNull, arrays (they are covered earlier)
                }
            }
            AstStatement::MultipliedStatement(data, ..) => {
                self.visit_statement(ctx, &data.element)
                //TODO as of yet we have no way to derive a name that reflects a fixed size array
            }
            _ => {}
        }
    }

    fn annotate_to_pointer_size_if_necessary(
        &mut self,
        value_type: &typesystem::DataType,
        statement: &AstNode,
    ) {
        // pointer size is 64Bits matching LINT
        // therefore get the bigger type of current and LINT to check if cast is necessary
        let bigger_type = get_bigger_type(value_type, self.index.get_type_or_panic(LINT_TYPE), self.index);
        if bigger_type != value_type {
            let bigger_type = bigger_type.clone();
            self.update_expected_types(&bigger_type, statement);
        }
    }
}

fn get_direct_access_type(access: &DirectAccessType) -> &'static str {
    match access {
        DirectAccessType::Bit => BOOL_TYPE,
        DirectAccessType::Byte => BYTE_TYPE,
        DirectAccessType::Word => WORD_TYPE,
        DirectAccessType::DWord => DWORD_TYPE,
        DirectAccessType::LWord => LWORD_TYPE,
        DirectAccessType::Template => VOID_TYPE,
    }
}

/// adds a string-type to the given index and returns it's name
fn register_string_type(index: &mut Index, is_wide: bool, len: usize) -> String {
    let prefix = if is_wide { "WSTRING_" } else { "STRING_" };
    let new_type_name = internal_type_name(prefix, len.to_string().as_str());

    if index.find_effective_type_by_name(new_type_name.as_str()).is_none() {
        index.register_type(crate::typesystem::DataType {
            name: new_type_name.clone(),
            initial_value: None,
            nature: TypeNature::String,
            information: crate::typesystem::DataTypeInformation::String {
                encoding: if is_wide { StringEncoding::Utf16 } else { StringEncoding::Utf8 },
                size: typesystem::TypeSize::LiteralInteger(len as i64 + 1),
            },
            location: SourceLocation::internal(),
        });
    }
    new_type_name
}

/// adds a pointer to the given inner_type to the given index and return's its name
pub(crate) fn add_pointer_type(index: &mut Index, inner_type_name: String) -> String {
    let new_type_name = internal_type_name("POINTER_TO_", inner_type_name.as_str());

    if index.find_effective_type_by_name(new_type_name.as_str()).is_none() {
        index.register_type(crate::typesystem::DataType {
            name: new_type_name.clone(),
            initial_value: None,
            nature: TypeNature::Any,
            information: crate::typesystem::DataTypeInformation::Pointer {
                auto_deref: false,
                inner_type_name,
                name: new_type_name.clone(),
            },
            location: SourceLocation::internal(),
        });
    }
    new_type_name
}

fn to_pou_annotation(p: &PouIndexEntry, index: &Index) -> Option<StatementAnnotation> {
    match p {
        PouIndexEntry::Program { name, .. } => {
            Some(StatementAnnotation::Program { qualified_name: name.into() })
        }
        PouIndexEntry::Function { name, return_type, .. } => Some(StatementAnnotation::Function {
            return_type: return_type.into(),
            qualified_name: name.into(),
            call_name: None,
        }),
        PouIndexEntry::FunctionBlock { name, .. } => {
            Some(StatementAnnotation::Type { type_name: name.into() })
        }
        PouIndexEntry::Action { name, parent_pou_name, .. } => match index.find_pou(parent_pou_name) {
            Some(PouIndexEntry::Program { .. }) => {
                Some(StatementAnnotation::Program { qualified_name: name.into() })
            }
            _ => None,
        },
        _ => None,
    }
}

fn to_variable_annotation(
    v: &VariableIndexEntry,
    index: &Index,
    constant_override: bool,
) -> StatementAnnotation {
    const AUTO_DEREF: bool = true;
    const NO_DEREF: bool = false;
    let v_type = index.get_effective_type_or_void_by_name(v.get_type_name());

    //see if this is an auto-deref variable
    let (effective_type_name, is_auto_deref) = match (v_type.get_type_information(), v.is_return()) {
        (_, true) if v_type.is_aggregate_type() => {
            // treat a return-aggregate variable like an auto-deref pointer since it got
            // passed by-ref
            (v_type.get_name().to_string(), AUTO_DEREF)
        }
        (DataTypeInformation::Pointer { inner_type_name, auto_deref: true, .. }, _) => {
            // real auto-deref pointer
            (inner_type_name.clone(), AUTO_DEREF)
        }
        _ => (v_type.get_name().to_string(), NO_DEREF),
    };

    StatementAnnotation::Variable {
        qualified_name: v.get_qualified_name().into(),
        resulting_type: effective_type_name,
        constant: v.is_constant() || constant_override,
        argument_type: v.get_declaration_type(),
        is_auto_deref,
    }
}

fn get_int_type_name_for(value: i128) -> &'static str {
    if i32::MIN as i128 <= value && i32::MAX as i128 >= value {
        DINT_TYPE
    } else {
        LINT_TYPE
    }
}

fn get_real_type_name_for(value: &str) -> &'static str {
    let parsed = value.parse::<f32>().unwrap_or(f32::INFINITY);
    if parsed == f32::INFINITY || parsed == f32::NEG_INFINITY {
        return LREAL_TYPE;
    }

    REAL_TYPE
}

#[derive(Clone)]
pub enum ResolvingScope {
    Variable,      //try to resolve a variable
    POU,           //try to resolve a POU
    DataType,      //try to resolve a DataType
    EnumTypeOnly,  //only consider EnumTypes
    FunctionsOnly, //only consider functions
}

impl ResolvingScope {
    /// scopes that can be used for general references. Will resolve to local/global
    /// variables, Pous or datatypes
    pub fn default_scopes() -> Vec<Self> {
        vec![ResolvingScope::Variable, ResolvingScope::POU, ResolvingScope::DataType]
    }

    /// scopes intended for call-statement-operators
    fn call_operator_scopes() -> Vec<ResolvingScope> {
        let mut strategy = vec![ResolvingScope::FunctionsOnly];
        strategy.extend(Self::default_scopes());
        strategy
    }

    /// tries to resolve the given name using the reprsented scope
    /// - `name` the name to resolve
    /// - `qualifier` an optional qualifier to prefix to the name,
    ///     if the qualifier is present, this method only resolves to targets with this qualifier
    /// - `index` the index to perform the lookups on
    fn resolve_name(
        &self,
        name: &str,
        qualifier: Option<&str>,
        index: &Index,
        ctx: &VisitorContext,
    ) -> Option<StatementAnnotation> {
        match self {
            // try to resolve the name as a variable
            ResolvingScope::Variable => {
                if let Some(qualifier) = qualifier {
                    // look for variable, enum with name "qualifier.name"
                    index
                        .find_member(qualifier, name)
                        .or_else(|| index.find_qualified_enum_element(format!("{qualifier}.{name}").as_str()))
                        .map(|it| to_variable_annotation(it, index, it.is_constant() || ctx.constant))
                } else {
                    // look for member variable with name "pou.name"
                    // then try fopr a global variable called "name"
                    ctx.pou
                        .and_then(|pou| index.find_member(pou, name))
                        .or_else(|| index.find_global_variable(name))
                        .map(|g| to_variable_annotation(g, index, g.is_constant()))
                }
            }
            // try to resolve the name as POU/Action/Method
            ResolvingScope::POU => {
                if let Some(qualifier) = qualifier {
                    // look for Pou/Action with name "qualifier.name"
                    index
                        .find_pou(format!("{qualifier}.{name}").as_str())
                        .or_else(|| index.find_method(qualifier, name))
                        .map(|action| action.into())
                } else {
                    // look for Pou with name "name"
                    index.find_pou(name).and_then(|pou| to_pou_annotation(pou, index)).or_else(|| {
                        ctx.pou.and_then(|pou|
                                // retry with local pou as qualifier
                                ResolvingScope::POU.resolve_name(name, Some(pou), index, ctx))
                    })
                }
            }
            // try to resolve the name as a datatype
            ResolvingScope::DataType => {
                if qualifier.is_none() {
                    // look for datatype with name "name"
                    index
                        .find_type(name)
                        .map(|data_type| StatementAnnotation::data_type(data_type.get_name()))
                } else {
                    // there are no qualified types
                    None
                }
            }
            // try to resolve the name as an enum-type
            ResolvingScope::EnumTypeOnly => {
                if qualifier.is_none() {
                    // look for enum-tyoe with name "name"
                    index
                        .find_type(name)
                        .filter(|it| it.is_enum())
                        .map(|enum_type| StatementAnnotation::data_type(enum_type.get_name()))
                } else {
                    // there are no qualified types
                    None
                }
            }
            // try to resolve this name as a function
            ResolvingScope::FunctionsOnly => {
                if qualifier.is_none() {
                    // look for function with name "name"
                    index.find_pou(name).filter(|it| it.is_function()).map(|pou| pou.into())
                } else {
                    // there are no qualified functions
                    None
                }
            }
        }
    }
}

/// registers the resulting string-literal if the given literal is a String with a different encoding than what is
/// requested from given cast_type (e.g. STRING#"i am utf16", or WSTRING#'i am utf8')
fn accept_cast_string_literal(
    literals: &mut StringLiterals,
    cast_type: &typesystem::DataType,
    literal: &AstNode,
) {
    // check if we need to register an additional string-literal
    match (cast_type.get_type_information(), literal.get_stmt()) {
        (
            DataTypeInformation::String { encoding: StringEncoding::Utf8, .. },
            AstStatement::Literal(AstLiteral::String(StringValue { value, is_wide: is_wide @ true })),
        )
        | (
            DataTypeInformation::String { encoding: StringEncoding::Utf16, .. },
            AstStatement::Literal(AstLiteral::String(StringValue { value, is_wide: is_wide @ false })),
        ) => {
            // re-register the string-literal in the opposite encoding
            if *is_wide {
                literals.utf08.insert(value.to_string());
            } else {
                literals.utf16.insert(value.to_string());
            }
        }
        _ => {
            //ignore
        }
    }
}

#[cfg(test)]
mod resolver_tests {
    use super::{get_int_type_name_for, get_real_type_name_for};

    #[test]
    fn correct_int_type_names_for_numbers() {
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

    #[test]
    fn correct_real_type_names_for_numbers() {
        assert_eq!(get_real_type_name_for(&f32::MIN.to_string()), "REAL");
        assert_eq!(get_real_type_name_for(&f32::MAX.to_string()), "REAL");
        assert_eq!(get_real_type_name_for(&f64::MIN.to_string()), "LREAL");
        assert_eq!(get_real_type_name_for(&f64::MAX.to_string()), "LREAL");

        // f32 under- and overflows (MIN == -3.40282347E+38, MAX == 3.40282347E+38)
        assert_eq!(get_real_type_name_for(" 3.50282347E+38"), "LREAL");
        assert_eq!(get_real_type_name_for("-3.50282347E+38"), "LREAL");
        assert_eq!(get_real_type_name_for(" 3.40282347E+39"), "LREAL");
        assert_eq!(get_real_type_name_for("-3.40282347E+39"), "LREAL");

        // f64 under- and overflows (MIN == -1.7976931348623157E+308, MAX == 1.7976931348623157E+308)
        assert_eq!(get_real_type_name_for(" 1.8976931348623157E+308"), "LREAL");
        assert_eq!(get_real_type_name_for("-1.8976931348623157E+308"), "LREAL");
        assert_eq!(get_real_type_name_for(" 1.7976931348623157E+309"), "LREAL");
        assert_eq!(get_real_type_name_for("-1.7976931348623157E+309"), "LREAL");
    }
}
