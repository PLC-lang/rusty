use plc_ast::{
    ast::ReferenceAccess,
    literals::StringValue,
    visitor::{AstVisitor, Walker},
};

use crate::{
    index::{Index, VariableIndexEntry},
    resolver::{register_string_type, AnnotationMap, AnnotationMapImpl, StatementAnnotation, StringLiterals},
    typesystem::{
        get_bigger_type, DataType, DataTypeInformation, BOOL_TYPE, DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE,
        LINT_TYPE, REAL_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE, VOID_TYPE,
    },
};

#[derive(Clone)]
pub enum Scope {
    Type,
    Program,
    GlobalVariable,
    LocalVariable(String),
    Composite(Vec<Scope>),
    StaticallyCallable,
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

impl Scope {
    fn lookup(&self, identifier: &str, index: &Index) -> Option<StatementAnnotation> {
        match self {
            // lookup a type
            Scope::Type => {
                index.find_type(identifier).map(|dt| dt.get_type_information()).map(StatementAnnotation::from)
            }

            Scope::Program => {
                index.find_pou(identifier).filter(|p| p.is_program()).map(StatementAnnotation::from)
            }

            // lookup a global variable
            Scope::GlobalVariable => {
                index.find_global_variable(identifier).map(|v| to_variable_annotation(v, index, false))
            }

            // lookup a local variable inside a container
            Scope::LocalVariable(container_name) => index
                .find_member(container_name.as_str(), identifier)
                .map(|v| to_variable_annotation(v, index, false)),

            // lookup the identifier in an ordered list of scopes
            Scope::Composite(scopes) => {
                scopes.iter().filter_map(|scope| scope.lookup(identifier, index)).next()
            }

            // functions, programs, actions, methods
            Scope::StaticallyCallable => {
                index.find_pou_implementation(identifier).map(|i| match i.implementation_type {
                    crate::index::ImplementationType::Program => {
                        StatementAnnotation::Program { qualified_name: i.get_call_name().to_string() }
                    }
                    crate::index::ImplementationType::Function => {
                        let return_type = index
                            .find_return_type(i.get_type_name())
                            .map(|dt| dt.get_name())
                            .unwrap_or_else(|| VOID_TYPE)
                            .to_string();
                        StatementAnnotation::Function {
                            return_type,
                            qualified_name: i.call_name.to_string(),
                            call_name: None,
                        }
                    }
                    crate::index::ImplementationType::FunctionBlock => todo!(),
                    crate::index::ImplementationType::Action => todo!(),
                    crate::index::ImplementationType::Class => todo!(),
                    crate::index::ImplementationType::Method => todo!(),
                })
            }
        }
    }
}

pub enum ScopingStrategy {
    /// Indicates that this scope inherits symbols from
    /// it's parent scope (as reflected by the order of the ScopeStack)
    Hierarchical(Scope),
    /// Indicates that this scope does not inherit from
    /// its parent scope (as reflected by the order of the ScopeStack)
    /// e.g. `foo( x := a)`
    /// `x` has a strict LocalVariable("foo") scope,
    /// `a` has a pou-local scope and inherits from the global scope
    Strict(Scope),
}

pub struct ScopeStack {
    stack: Vec<ScopingStrategy>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack {
            stack: vec![ScopingStrategy::Strict(Scope::Composite(vec![
                Scope::GlobalVariable,
                Scope::StaticallyCallable,
            ]))],
        }
    }

    pub fn lookup(&self, identifier: &str, i: &Index) -> Option<StatementAnnotation> {
        for strategy in self.stack.iter().rev() {
            match strategy {
                ScopingStrategy::Hierarchical(scope) => {
                    if let Some(dti) = scope.lookup(identifier, i) {
                        return Some(dti);
                    }
                }

                // uses this scope, no fallback to parent scopes
                ScopingStrategy::Strict(scope) => return scope.lookup(identifier, i),
            }
        }
        None
    }

    pub fn run_with_scope(&mut self, scope: ScopingStrategy, f: impl Fn()) {
        self.push(scope);
        f();
        self.pop();
    }
}

impl ScopeStack {
    fn push(&mut self, scope: ScopingStrategy) {
        self.stack.push(scope);
    }

    fn pop(&mut self) -> Option<ScopingStrategy> {
        self.stack.pop()
    }
}

pub struct NameResolver<'i> {
    index: &'i Index,
    pub annotations: AnnotationMapImpl,
    pub strings: StringLiterals,

    scope: ScopeStack,

    root_scope: Scope,
    in_a_body: bool,
}

impl<'i> NameResolver<'i> {
    pub fn new(index: &'i Index) -> NameResolver<'i> {
        Self {
            index,
            annotations: AnnotationMapImpl::new(),
            scope: ScopeStack::new(),
            root_scope: Scope::GlobalVariable,
            strings: StringLiterals::default(),
            in_a_body: false,
        }
    }

    fn walk_with_scope<T>(&mut self, t: &T, scope: ScopingStrategy)
    where
        T: Walker,
    {
        self.scope.push(scope);
        t.walk(self);
        self.scope.pop();
    }
}

impl AstVisitor for NameResolver<'_> {
    fn visit_implementation(&mut self, implementation: &plc_ast::ast::Implementation) {
        self.in_a_body = true;
        self.walk_with_scope(
            implementation,
            ScopingStrategy::Hierarchical(Scope::LocalVariable(implementation.type_name.clone())),
        );
        self.in_a_body = false;
    }

    fn visit_reference_expr(&mut self, stmt: &plc_ast::ast::ReferenceExpr, node: &plc_ast::ast::AstNode) {
        let base = stmt.base.as_ref();

        match (base, &stmt.access) {
            (Some(base), ReferenceAccess::Cast(_)) => {
                self.walk_with_scope(base, ScopingStrategy::Strict(Scope::Type))
            }
            (Some(base), _) => base.walk(self),
            (None, _) => {}
        };

        //TODO think about cow<string> here
        let context = base.and_then(|b| self.annotations.get_type_name(b)).map(str::to_string);

        match (&stmt.access, context.as_ref()) {
            (ReferenceAccess::Member(member), base) => {
                if let Some(base) = base {
                    // resolve member und the base's context
                    self.scope.push(ScopingStrategy::Strict(Scope::LocalVariable(base.to_string())));
                }

                member.walk(self);
                self.annotations.copy_annotation(member, node);

                if base.is_some() {
                    self.scope.pop();
                }
            }
            (ReferenceAccess::Index(idx), _) => {
                // make sure we resolve from the root-scope
                self.walk_with_scope(idx, ScopingStrategy::Strict(self.root_scope.clone()));
                self.annotations.copy_annotation(idx, node);
            }
            (ReferenceAccess::Cast(target), Some(base)) => {
                if let Some(true) =
                    self.annotations.get_type(stmt.base.as_ref().unwrap(), &self.index).map(|it| it.is_enum())
                {
                    self.walk_with_scope(
                        target,
                        ScopingStrategy::Strict(Scope::LocalVariable(base.to_string())),
                    );
                } else {
                    target.walk(self);
                    self.annotations.annotate(target, StatementAnnotation::data_type(base));
                }
                if self.annotations.has_type_annotation(target) {
                    self.annotations.annotate(node, StatementAnnotation::data_type(base));
                }
            }
            (ReferenceAccess::Deref, Some(base)) => {
                if let Some(DataTypeInformation::Pointer { inner_type_name, auto_deref: false, .. }) =
                    self.index.find_type(base).map(DataType::get_type_information)
                {
                    self.annotations.annotate(node, StatementAnnotation::data_type(&inner_type_name));
                }
            }
            (ReferenceAccess::Address, Some(_base)) => {
                todo!("Address of operator not implemented yet")
            }
            _ => {}
        }
    }

    fn visit_identifier(&mut self, stmt: &str, node: &plc_ast::ast::AstNode) {
        if let Some(annotation) = self.scope.lookup(stmt, self.index) {
            self.annotations.annotate(node, annotation);
        }
    }

    fn visit_literal(&mut self, stmt: &plc_ast::literals::AstLiteral, node: &plc_ast::ast::AstNode) {
        let type_name = match stmt {
            plc_ast::literals::AstLiteral::Null => Some(VOID_TYPE),
            plc_ast::literals::AstLiteral::Integer(v) if *v > i32::MAX as i128 => Some(LINT_TYPE),
            plc_ast::literals::AstLiteral::Integer(_) => Some(DINT_TYPE),
            plc_ast::literals::AstLiteral::Date(_) => Some(DATE_TYPE),
            plc_ast::literals::AstLiteral::DateAndTime(_) => Some(DATE_AND_TIME_TYPE),
            plc_ast::literals::AstLiteral::TimeOfDay(_) => Some(TIME_OF_DAY_TYPE),
            plc_ast::literals::AstLiteral::Time(_) => Some(TIME_TYPE),
            plc_ast::literals::AstLiteral::Real(_) => Some(REAL_TYPE),
            plc_ast::literals::AstLiteral::Bool(_) => Some(BOOL_TYPE),
            plc_ast::literals::AstLiteral::String(StringValue { is_wide, value, .. }) => {
                //collect literals so we can generate global constants later
                match (self.in_a_body, *is_wide) {
                    (true, true) => self.strings.utf16.insert(value.to_string()),
                    (true, false) => self.strings.utf08.insert(value.to_string()),
                    _ => false,
                };
                Some(register_string_type(&mut self.annotations.new_index, *is_wide, value.len()))
            }
            plc_ast::literals::AstLiteral::Array(a) => {
                a.elements.walk(self);
                Some(VOID_TYPE)
            }
        };

        if let Some(type_name) = type_name.map(|t| t.to_string()) {
            self.annotations.annotate(node, StatementAnnotation::value(type_name));
        }
    }

    fn visit_binary_expression(
        &mut self,
        stmt: &plc_ast::ast::BinaryExpression,
        _node: &plc_ast::ast::AstNode,
    ) {
        stmt.walk(self);

        let left_type = self
            .annotations
            .get_type_hint_or_type(&stmt.left, self.index)
            .and_then(|it| self.index.find_effective_type(it))
            .unwrap_or_else(|| self.index.get_void_type());

        // TODO intrinsic

        let right_type = self
            .annotations
            .get_type_hint_or_type(&stmt.right, self.index)
            .and_then(|it| self.index.find_effective_type(it))
            .unwrap_or_else(|| self.index.get_void_type());

        let type_name = if stmt.operator.is_arithmetic_operator()
            && left_type.is_arithmetic()
            && right_type.is_arithmetic()
        {
            let bigger_type = if left_type.is_bool() && right_type.is_bool() {
                left_type
                //TODO Reuired???
            } else {
                let ty = if left_type.is_bit() && right_type.is_bit() {
                    right_type
                } else {
                    self.index.get_type_or_panic(DINT_TYPE)
                };
                get_bigger_type(get_bigger_type(left_type, right_type, self.index), ty, self.index)
            };
            Some(bigger_type.get_name().to_string()) //TODO tostring?
        } else if stmt.operator.is_binary_operator() && left_type.is_bool() && right_type.is_bool() {
            Some(BOOL_TYPE.to_string())
        } else if stmt.operator.is_binary_operator() && left_type.is_numerical() && right_type.is_numerical()
        {
            Some(get_bigger_type(left_type, right_type, &self.index).get_name().to_string())
        } else if stmt.operator.is_comparison_operator() {
            Some(BOOL_TYPE.to_string())
        } else {
            None
        };

        if let Some(tn) = type_name {
            self.annotations.annotate(_node, StatementAnnotation::value(tn));
        }
    }

    fn visit_call_statement(&mut self, stmt: &plc_ast::ast::CallStatement, node: &plc_ast::ast::AstNode) {
        stmt.walk(self);

        // annotate the whole statement with the resulting type
        if let Some(StatementAnnotation::Function { return_type, .. }) = self.annotations.get(&stmt.operator)
        {
            self.annotations.annotate(node, StatementAnnotation::value(return_type.to_string()));
        }
    }
}
