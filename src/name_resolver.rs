use log::LevelFilter;
use plc_ast::{
    ast::{
        flatten_expression_list, Assignment, AstNode, AstStatement, DataType, ReferenceAccess, TypeNature,
    },
    literals::{Array, AstLiteral, StringValue},
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocation;
use plc_util::convention::internal_type_name;

use crate::{
    index::{Index, VariableIndexEntry},
    resolver::{register_string_type, AnnotationMap, AnnotationMapImpl, StatementAnnotation, StringLiterals},
    typesystem::{
        self, get_bigger_type, get_type_name_for_direct_access, DataTypeInformation, BOOL_TYPE,
        DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, LINT_TYPE, REAL_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE,
        VOID_TYPE,
    },
};

#[derive(Clone)]
pub enum Scope {
    Type,
    Program,
    GlobalVariable,
    LocalVariable(String),
    Composite(Vec<Scope>),
    Callable(Option<String>),
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

            Scope::Program => index.find_pou(identifier).map(StatementAnnotation::from),

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
            Scope::Callable(None) => {
                index.find_pou_implementation(identifier).and_then(|i| match i.implementation_type {
                    crate::index::ImplementationType::Program | crate::index::ImplementationType::Action => {
                        Some(StatementAnnotation::Program { qualified_name: i.get_call_name().to_string() })
                    }
                    crate::index::ImplementationType::Function => {
                        let return_type = index
                            .find_return_type(i.get_type_name())
                            .map(|dt| dt.get_name())
                            .unwrap_or_else(|| VOID_TYPE)
                            .to_string();
                        Some(StatementAnnotation::Function {
                            return_type,
                            qualified_name: i.call_name.to_string(),
                            call_name: None,
                        })
                    }
                    crate::index::ImplementationType::FunctionBlock
                    | crate::index::ImplementationType::Class => {
                        Some(StatementAnnotation::data_type(i.get_type_name()))
                    }
                    // crate::index::ImplementationType::Method => todo!(),
                    _ => None,
                })
            }

            // functions, programs, actions, methods
            Scope::Callable(Some(qualifier)) => {
                //TODO improve!
                let qualified_name = format!("{qualifier}.{identifier}");
                index.find_pou_implementation(qualified_name.as_str()).and_then(|i| {
                    match i.implementation_type {
                        crate::index::ImplementationType::Action => Some(StatementAnnotation::Program {
                            qualified_name: i.get_call_name().to_string(),
                        }),
                        crate::index::ImplementationType::Method => todo!(),
                        _ => None,
                    }
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
                Scope::Callable(None),
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
            ScopingStrategy::Hierarchical(Scope::Composite(vec![
                Scope::LocalVariable(implementation.type_name.clone()),
                Scope::Callable(Some(implementation.type_name.clone())),
            ])),
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
        let base_annotation = base.and_then(|b| self.annotations.get(b));

        let is_constant = base_annotation.map(|a| a.is_const()).unwrap_or(false);
        let context = base_annotation
            .and_then(|a| self.annotations.get_type_name_for_annotation(a))
            .map(str::to_string);

        match (&stmt.access, context.as_ref()) {
            (ReferenceAccess::Member(member), base) => {
                match member.get_stmt() {
                    AstStatement::Literal(AstLiteral::Integer(_)) => {
                        // BIT-Access
                        self.annotations.annotate(&member, StatementAnnotation::value(BOOL_TYPE.to_string()))
                    }
                    AstStatement::DirectAccess(da) => self.annotations.annotate(
                        &member,
                        StatementAnnotation::value(get_type_name_for_direct_access(&da.access).to_string()),
                    ),
                    _ => {
                        if let Some(base) = base {
                            // resolve member und the base's context
                            self.walk_with_scope(
                                member,
                                ScopingStrategy::Strict(Scope::Composite(vec![
                                    Scope::LocalVariable(base.to_string()),
                                    Scope::Callable(Some(base.to_string())),
                                ])),
                            );
                        } else {
                            member.walk(self);
                        }
                    }
                }

                self.annotations.copy_annotation(member, node);
            }
            (ReferenceAccess::Index(idx), Some(base)) => {
                // make sure we resolve from the root-scope
                self.walk_with_scope(idx, ScopingStrategy::Strict(self.root_scope.clone()));

                // the array-access turns this expression into the array's inner type
                if let Some(inner_type_name) = self
                    .index
                    .find_effective_type_info(base.as_str())
                    .and_then(|t| t.get_inner_array_type_name())
                    .and_then(|it| self.index.find_effective_type_by_name(it).map(|it| it.get_name()))
                //TODO why effective again?
                {
                    self.annotations.annotate(node, StatementAnnotation::value(inner_type_name))
                }
            }
            (ReferenceAccess::Cast(target), Some(base)) => {
                if let Some(true) =
                    self.annotations.get_type(stmt.base.as_ref().unwrap(), self.index).map(|it| it.is_enum())
                {
                    self.walk_with_scope(
                        target,
                        ScopingStrategy::Strict(Scope::LocalVariable(base.to_string())),
                    );
                } else {
                    target.walk(self);
                }
                if self.annotations.has_type_annotation(target) {
                    self.annotations.annotate(node, StatementAnnotation::data_type(base));
                }
            }
            (ReferenceAccess::Deref, Some(base)) => {
                if let Some(DataTypeInformation::Pointer { inner_type_name, auto_deref: false, .. }) =
                    self.index.find_type(base).map(typesystem::DataType::get_type_information)
                {
                    self.index.find_effective_type_by_name(inner_type_name).inspect(|effective_inner_type| {
                        self.annotations
                            .annotate(node, StatementAnnotation::data_type(effective_inner_type.get_name()))
                    });
                }
            }
            (ReferenceAccess::Address, Some(_base)) => {
                if let Some(inner_type) = base
                    .map(|base| self.annotations.get_type_or_void(base, self.index).get_name().to_string())
                {
                    let ptr_type = add_pointer_type(&mut self.annotations.new_index, inner_type);
                    self.annotations.annotate(node, StatementAnnotation::value(ptr_type))
                }
            }
            _ => {}
        }

        // if the parent reference is constant, we are also constant
        if is_constant {
            self.annotations.make_constant(node)
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
        // prioritize functions here
        self.walk_with_scope(&stmt.operator, ScopingStrategy::Hierarchical(Scope::Callable(None)));
        // annotate the whole statement with the resulting type
        let call_target = self.annotations.get(&stmt.operator);
        if let Some(StatementAnnotation::Function { return_type, .. }) = call_target {
            self.annotations.annotate(node, StatementAnnotation::value(return_type));
        }

        //annotate the parameters with the right hints
        // TODO: I need something that gets me the type_name of the resulting POU
        if let Some(target_type_name) = self.annotations.get_call_name(&stmt.operator).map(str::to_string) {
            let declared_parameters = self.index.get_declared_parameters(target_type_name.as_str());

            for (idx, arg) in stmt
                .parameters
                .as_ref()
                .map(|it| flatten_expression_list(it))
                .unwrap_or_default()
                .iter()
                .enumerate()
            {
                if let AstStatement::Assignment(Assignment { left, right, .. }) = arg.get_stmt() {
                    // left needs to be resolved in the context of the call operator
                    self.walk_with_scope(
                        left,
                        ScopingStrategy::Strict(Scope::LocalVariable(target_type_name.to_string())),
                    );
                    // right needs to be resolved with normal scope
                    right.walk(self);
                } else {
                    arg.walk(self);

                    // hint it with the argument ast pos n
                    // TODO: move to the hinter???
                    if let Some(declared_parameter) = declared_parameters.get(idx) {
                        self.annotations.annotate_type_hint(
                            arg,
                            StatementAnnotation::value(declared_parameter.get_type_name()),
                        );
                    }
                }
            }
        }
    }

    fn visit_paren_expression(&mut self, inner: &plc_ast::ast::AstNode, node: &plc_ast::ast::AstNode) {
        inner.walk(self);
        self.annotations.copy_annotation(inner, node)
    }

    fn visit_variable(&mut self, variable: &plc_ast::ast::Variable) {
        variable.walk(self);

        if let (Some(initializer), Some(variable_type)) =
            (&variable.initializer, variable.data_type_declaration.get_name())
        {
            let mut initializer_annotator = InitializerAnnotator::new(
                &self.index.get_effective_type_or_void_by_name(variable_type).get_type_information(),
                self.index,
                &mut self.annotations,
            );
            initializer.walk(&mut initializer_annotator);
        }
    }

    fn visit_user_type_declaration(&mut self, user_type: &plc_ast::ast::UserTypeDeclaration) {
        self.visit_data_type(&user_type.data_type);

        if let Some(type_name) = user_type.data_type.get_name() {
            let mut initializer_annotator = InitializerAnnotator::new(
                self.index.get_intrinsic_type_by_name(type_name).get_type_information(),
                &self.index,
                &mut self.annotations,
            );
            user_type.initializer.as_ref().inspect(|it| it.walk(&mut initializer_annotator));
        }
    }

    fn visit_data_type(&mut self, data_type: &plc_ast::ast::DataType) {
        data_type.walk(self);

        // hint the range limits with the original type
        // INT(0..100) --> 0 and 100 should be hinted with INT
        if let DataType::SubRangeType {
            bounds: Some(AstNode { stmt: AstStatement::RangeStatement(range), .. }),
            name: Some(name),
            ..
        } = data_type
        {
            let type_name = self.index.get_intrinsic_type_by_name(name).get_name();
            self.annotations.annotate_type_hint(&range.start, StatementAnnotation::value(type_name));
            self.annotations.annotate_type_hint(&range.end, StatementAnnotation::value(type_name));
        }
    }
}

//TODO find better place
/// adds a pointer to the given inner_type to the given index and return's its name
fn add_pointer_type(index: &mut Index, inner_type_name: String) -> String {
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

/// this anotator is used to create the type-annotations on initializers
/// Note that it assumes that it only ever visits initializers!
struct InitializerAnnotator<'i> {
    expected_type: &'i DataTypeInformation,
    index: &'i Index,
    annotations: &'i mut AnnotationMapImpl,
}

impl<'i> InitializerAnnotator<'i> {
    pub fn new(
        expected_type: &'i DataTypeInformation,
        index: &'i Index,
        annotations: &'i mut AnnotationMapImpl,
    ) -> Self {
        Self { expected_type, index, annotations }
    }
}

impl AstVisitor for InitializerAnnotator<'_> {
    fn visit_literal(&mut self, stmt: &plc_ast::literals::AstLiteral, node: &AstNode) {
        // annotate the initializer with the expected type
        self.annotations.annotate_type_hint(node, StatementAnnotation::value(self.expected_type.get_name()));

        // for array initializers we also want to initialize the single array elements [a,b,c]
        if let (
            AstLiteral::Array(Array { elements: Some(elements), .. }),
            DataTypeInformation::Array { inner_type_name, .. },
        ) = (stmt, self.expected_type)
        {
            // hint the elements of an array
            if let Some(inner_type) = self.index.find_effective_type_info(inner_type_name) {
                let mut annotator = InitializerAnnotator::new(inner_type, self.index, self.annotations);
                for member in elements.get_as_list() {
                    annotator.visit(member);
                }
            }
        }
    }

    fn visit_paren_expression(&mut self, inner: &AstNode, node: &AstNode) {
        inner.walk(self);
        // maybe a struct?
        if let DataTypeInformation::Struct { name, .. } = self.expected_type {
            //annotate the paren-expression and the inner one
            self.annotations.annotate_type_hint(node, StatementAnnotation::value(name));
            self.annotations.copy_annotation(node, inner);
        }
    }

    fn visit_assignment(&mut self, stmt: &Assignment, _node: &AstNode) {
        stmt.walk(self);
        if let Some(annotation) = stmt.left.get_flat_reference_name().and_then(|ref_name| {
            Scope::LocalVariable(self.expected_type.get_name().to_string()).lookup(ref_name, self.index)
        }) {
            // visit the rightside with new expected type
            if let Some(sub_type) = self.annotations.get_type_for_annotation(self.index, &annotation) {
                let mut sub_visitor =
                    InitializerAnnotator::new(sub_type.get_type_information(), self.index, self.annotations);
                sub_visitor.visit(&stmt.right);
            }

            self.annotations.annotate(&stmt.left, annotation);
        }
    }
}
