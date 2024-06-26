use plc_ast::{
    ast::{
        flatten_expression_list, Assignment, AstNode, AstStatement, Operator, ReferenceAccess, TypeNature,
    },
    literals::{Array, AstLiteral, StringValue},
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocation;
use plc_util::convention::internal_type_name;

use crate::{
    builtins,
    index::{Index, VariableIndexEntry},
    resolver::{
        register_string_type, AnnotationMap, AnnotationMapImpl, AstAnnotations, StatementAnnotation,
        StringLiterals,
    },
    typesystem::{
        self, get_bigger_type, get_type_name_for_direct_access, DataType, DataTypeInformation, BOOL_TYPE,
        DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, LINT_TYPE, LREAL_TYPE, REAL_TYPE, TIME_OF_DAY_TYPE,
        TIME_TYPE, VOID_TYPE,
    },
};

#[derive(Clone, Debug)]
pub enum Scope {
    Type,
    POU,
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
            Scope::Type => index
                .find_effective_type_by_name(identifier)
                .map(|dt| dt.get_type_information())
                .map(StatementAnnotation::from),

            Scope::POU => index.find_pou(identifier).map(StatementAnnotation::from),

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
                            .and_then(|dt| index.find_effective_type(dt))
                            .map(|dt| dt.get_name())
                            .unwrap_or_else(|| VOID_TYPE)
                            .to_string();
                        Some(StatementAnnotation::Function {
                            return_type,
                            qualified_name: i.call_name.to_string(),
                            call_name: None,
                        })
                    }
                    // crate::index::ImplementationType::FunctionBlock
                    // | crate::index::ImplementationType::Class => {
                    //     Some(StatementAnnotation::data_type(i.get_type_name()))
                    // }
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
                        crate::index::ImplementationType::Method => {
                            let return_type = index
                                .find_return_type(i.get_type_name())
                                .and_then(|dt| index.find_effective_type(dt))
                                .map(|dt| dt.get_name())
                                .unwrap_or_else(|| VOID_TYPE)
                                .to_string();

                            Some(StatementAnnotation::Function {
                                return_type,
                                qualified_name: i.call_name.to_string(),
                                call_name: None,
                            })
                        }
                        _ => None,
                    }
                })
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ScopeStack {
    stack: Vec<ScopingStrategy>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack {
            stack: vec![ScopingStrategy::Strict(Scope::Composite(vec![
                Scope::GlobalVariable,
                Scope::Callable(None),
                Scope::POU,
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

    pub fn print_dump(&self) {
        println!("{:#?}", self);
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
    id_provider: IdProvider,
    in_a_body: bool,
}

impl<'i> NameResolver<'i> {
    pub fn new(index: &'i Index, id_provider: IdProvider) -> NameResolver<'i> {
        Self {
            index,
            annotations: AnnotationMapImpl::new(),
            scope: ScopeStack::new(),
            root_scope: Scope::GlobalVariable,
            strings: StringLiterals::default(),
            in_a_body: false,
            id_provider,
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

    /// tries to apply a possible replacement for the given operator
    /// returns true if a replacement was applied
    fn apply_possible_replacement(
        &mut self,
        operator: &AstNode,
        parameters: Option<&AstNode>,
        full_statement: &AstNode,
    ) -> bool {
        let builtin = operator.get_flat_reference_name().and_then(|rn| builtins::get_builtin(rn));
        let did_apply = builtin.map(|b| b.get_annotation()).and_then(|r| {
            r.map(|f| {
                f(&mut self.annotations, full_statement, operator, parameters, &mut self.id_provider);
                matches!(
                    self.annotations.get(full_statement),
                    Some(StatementAnnotation::ReplacementAst { .. })
                )
            })
        });
        did_apply.unwrap_or(false)
    }

    fn apply_integral_promotion<'a>(&self, data_type: &DataType) -> Option<String> {
        // intrinsic type promotion for variadics in order to be compatible with the C standard.
        // see ISO/IEC 9899:1999, 6.5.2.2 Function calls (https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1256.pdf)
        // or https://en.cppreference.com/w/cpp/language/implicit_conversion#Integral_promotion
        // for more about default argument promotion.

        // varargs without a type declaration will be annotated "VOID", so in order to check if a
        // promotion is necessary, we need to first check the type of each parameter. in the case of numerical
        // types, we promote if the type is smaller than double/i32 (except for booleans).
        match &data_type.get_type_information() {
            DataTypeInformation::Float { .. } => Some(
                get_bigger_type(data_type, self.index.get_type_or_panic(LREAL_TYPE), self.index)
                    .get_type_information()
                    .get_name()
                    .to_string(),
            ),
            DataTypeInformation::Integer { .. } if !&data_type.information.is_bool() => Some(
                get_bigger_type(data_type, self.index.get_type_or_panic(DINT_TYPE), self.index)
                    .get_type_information()
                    .get_name()
                    .to_string(),
            ),
            _ => None,
        }
    }
    
    fn prepare_qualifier_list(&self, base: &str) -> Vec<String> {
        let mut qualifiers = Vec::new();
        let mut base = Some(base.to_string());
        
        while let Some(b) = base.take() {
            if let Some(crate::index::PouIndexEntry::Class {super_class: Some(super_class), .. }) = self.index.find_pou(b.as_str()) {
                base = Some(super_class.clone());
            }
            qualifiers.push(b);
        }        
        qualifiers
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
                    AstStatement::DirectAccess(da) => {
                        self.visit(&da.index);
                        self.annotations.annotate(
                            &member,
                            StatementAnnotation::value(
                                get_type_name_for_direct_access(&da.access).to_string(),
                            ),
                        )
                    }
                    _ => {
                        if let Some(base) = base {

                            // if we are in a class, we need to resolve the member in the context of the class
                            // and its base classes
                            let qualifier_list = self.prepare_qualifier_list(base)
                                    .iter().map(|it| Scope::Composite(vec![
                                        Scope::LocalVariable(it.to_string()),
                                        Scope::Callable(Some(it.to_string())),
                                    ])).collect::<Vec<_>>();
                            // resolve member und the base's context
                            self.walk_with_scope(
                                member,
                                ScopingStrategy::Strict(Scope::Composite(qualifier_list)),
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
                //TODO Required???
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
        } else if left_type.is_pointer() {
            //make sure we treat the other also as string
            self.annotations
                .annotate_type_hint(&stmt.right, StatementAnnotation::value(left_type.get_name()));
            Some(left_type.get_name().to_string())
        } else if right_type.is_pointer() {
            // make sure we treat the other also as string
            self.annotations.annotate_type_hint(&stmt.left, StatementAnnotation::value(left_type.get_name()));
            Some(right_type.get_name().to_string())
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
        // TODO: I need something that gets me the type_name of the resulting POU (actions cause a problem)
        if let Some(target_type_name) = self.annotations.get_call_name(&stmt.operator).and_then(|call_name| {
            self.index.find_pou_implementation(call_name).map(|imp| imp.get_type_name())
        }) {
            // build an ordered vec of parameter types, fill the end with the variadic-type if its there
            let declared_parameters = self.index.get_declared_parameters(target_type_name);
            let variadic = self.index.get_variadic_member(target_type_name);
            let parameters =
                stmt.parameters.as_ref().map(|it| flatten_expression_list(it)).unwrap_or_default();

            let declared_parameters: Vec<_> =
                declared_parameters.iter().chain(variadic.iter().cycle()).take(parameters.len()).collect();

            for (idx, arg) in parameters.iter().enumerate() {
                match arg.get_stmt() {
                    AstStatement::Assignment(Assignment { left, right, .. })
                    | AstStatement::OutputAssignment(Assignment { left, right, .. }) => {
                        // left needs to be resolved in the context of the call operator
                        self.walk_with_scope(
                            left,
                            ScopingStrategy::Strict(Scope::LocalVariable(target_type_name.to_string())),
                        );
                        // right needs to be resolved with normal scope
                        right.walk(self);
                    }
                    _ => {
                        arg.walk(self);

                        // hint it with the argument ast pos n
                        match (declared_parameters.get(idx), self.annotations.get_type(&arg, self.index)) {
                            (Some(p), Some(t)) if p.is_variadic() && p.is_void() => {
                                if let Some(type_name) = self.apply_integral_promotion(t) {
                                    self.annotations
                                        .annotate_type_hint(arg, StatementAnnotation::value(type_name));
                                }
                            }
                            (Some(p), _) => {
                                self.annotations
                                    .annotate_type_hint(arg, StatementAnnotation::value(p.get_type_name()));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        //lets see if this is builtin-function that needs to be replaced
        if self.apply_possible_replacement(&stmt.operator, stmt.parameters.as_deref(), node) {
            if let Some(StatementAnnotation::ReplacementAst { statement: replacement }) =
                self.annotations.take(node)
            {
                //visit the new ast
                self.visit(&replacement);
                //re-attach the annotation
                self.annotations
                    .annotate(&node, StatementAnnotation::ReplacementAst { statement: replacement });
            }
        } else {
        }
    }

    fn visit_paren_expression(&mut self, inner: &plc_ast::ast::AstNode, node: &plc_ast::ast::AstNode) {
        inner.walk(self);
        self.annotations.copy_annotation(inner, node);
    }

    fn visit_variable(&mut self, variable: &plc_ast::ast::Variable) {
        variable.walk(self);

        if let (Some(initializer), Some(variable_type)) =
            (&variable.initializer, variable.data_type_declaration.get_name())
        {
            let mut initializer_annotator = LiteralsAnnotator::new(
                &self.index.get_effective_type_or_void_by_name(variable_type).get_type_information(),
                self.index,
                &mut self.annotations,
            );
            initializer.walk(&mut initializer_annotator);
        }
    }

    fn visit_unary_expression(&mut self, stmt: &plc_ast::ast::UnaryExpression, node: &AstNode) {
        stmt.walk(self);

        let negative_type = self
            .annotations
            .get_type_hint_or_type(&stmt.value, self.index)
            .map(|it| self.index.find_intrinsic_type(it.get_type_information()));

        if let (Operator::Minus, Some(data_type)) = (&stmt.operator, negative_type) {
            self.annotations.annotate(node, StatementAnnotation::value(data_type.get_name()));
        } else {
            self.annotations.copy_annotation(&stmt.value, node);
        }
    }

    fn visit_user_type_declaration(&mut self, user_type: &plc_ast::ast::UserTypeDeclaration) {
        // first try  normal walk ...
        user_type.walk(self);

        // ... then try to annotate the initializer with the known type-information
        if let Some(type_name) = user_type.data_type.get_name() {
            let mut initializer_annotator = LiteralsAnnotator::new(
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
        if let plc_ast::ast::DataType::SubRangeType {
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

    fn visit_hardware_access(&mut self, stmt: &plc_ast::ast::HardwareAccess, _node: &AstNode) {
        stmt.walk(self);
        self.annotations
            .annotate(_node, StatementAnnotation::value(get_type_name_for_direct_access(&stmt.access)));
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
pub struct LiteralsAnnotator<'i> {
    expected_type: &'i DataTypeInformation,
    index: &'i Index,
    annotations: &'i mut AnnotationMapImpl,
}

impl<'i> LiteralsAnnotator<'i> {
    pub fn new(
        expected_type: &'i DataTypeInformation,
        index: &'i Index,
        annotations: &'i mut AnnotationMapImpl,
    ) -> Self {
        Self { expected_type, index, annotations }
    }
}

impl AstVisitor for LiteralsAnnotator<'_> {
    fn visit_literal(&mut self, stmt: &plc_ast::literals::AstLiteral, node: &AstNode) {
        // for array initializers we also want to initialize the single array elements [a,b,c]
        if let (
            AstLiteral::Array(Array { elements: Some(elements), .. }),
            DataTypeInformation::Array { inner_type_name, .. },
        ) = (stmt, self.expected_type)
        {
            // annotate the type of the array
            self.annotations.annotate(node, StatementAnnotation::value(self.expected_type.get_name()));
            // hint the elements of an array
            if let Some(inner_type) = self.index.find_effective_type_info(inner_type_name) {
                let mut annotator = LiteralsAnnotator::new(inner_type, self.index, self.annotations);
                for member in elements.get_as_list() {
                    annotator.visit(member);
                }
            }
        } else {
            // annotate the initializer with the expected type
            self.annotations
                .annotate_type_hint(node, StatementAnnotation::value(self.expected_type.get_name()));
        }
    }

    /// could be a struct-literal
    fn visit_paren_expression(&mut self, inner: &AstNode, node: &AstNode) {
        // maybe a struct?
        if let DataTypeInformation::Struct { name, .. } = self.expected_type {
            //annotate the paren-expression and the inner one
            self.annotations.annotate_type_hint(node, StatementAnnotation::value(name));
            self.annotations.copy_annotation(node, inner);

            // visit the child expressions, they are probably assignments
            for i in inner.get_as_list() {
                self.visit(i);
            }
        }
    }

    /// could be an assignment in a struct-literal
    fn visit_assignment(&mut self, stmt: &Assignment, _node: &AstNode) {
        let reference_name = stmt.left.get_flat_reference_name();
        if let Some(annotation) = reference_name.and_then(|ref_name| {
            Scope::LocalVariable(self.expected_type.get_name().to_string()).lookup(ref_name, self.index)
        }) {
            // visit the rightside with new expected type
            if let Some(sub_type) = self.annotations.get_type_for_annotation(self.index, &annotation) {
                let mut sub_visitor =
                    LiteralsAnnotator::new(sub_type.get_type_information(), self.index, self.annotations);
                sub_visitor.visit(&stmt.right);
            }

            self.annotations.annotate(&stmt.left, annotation);
        }
    }
}
