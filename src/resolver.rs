// Copyright (c) 2021 Ghaith Hachem and Mathias Riede

//! Resolves (partial) expressions & statements and annotates the resulting types
//!
//! Recursively visits all statements and expressions of a `CompilationUnit` and
//! records all resulting types associated with the statement's id.

use rustc_hash::{FxHashMap, FxHashSet};
use std::{fmt::Debug, hash::Hash};

use plc_ast::{
    ast::{
        self, flatten_expression_list, Allocation, Assignment, AstFactory, AstId, AstNode, AstStatement,
        BinaryExpression, CompilationUnit, DataType, DataTypeDeclaration, DirectAccessType, Identifier,
        Interface, JumpStatement, Operator, Pou, PouType, ReferenceAccess, ReferenceExpr, TypeNature,
        UserTypeDeclaration, Variable,
    },
    control_statements::{AstControlStatement, ReturnStatement},
    literals::{Array, AstLiteral, StringValue},
    provider::IdProvider,
    try_from,
};
use plc_source::source_location::SourceLocation;
use plc_util::convention::internal_type_name;

use crate::index::{FxIndexMap, FxIndexSet, InterfaceIndexEntry};
use crate::typesystem::VOID_INTERNAL_NAME;
use crate::{
    builtins::{self, BuiltIn},
    index::{ArgumentType, Index, PouIndexEntry, VariableIndexEntry, VariableType},
    typesystem::{
        self, get_bigger_type, DataTypeInformation, InternalType, StringEncoding, StructSource, BOOL_TYPE,
        BYTE_TYPE, DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, DWORD_TYPE, LINT_TYPE, LREAL_TYPE, LWORD_TYPE,
        REAL_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE, VOID_TYPE, WORD_TYPE,
    },
};

pub mod const_evaluator;
pub mod generics;

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
#[derive(Clone, Default)]
pub struct VisitorContext<'s> {
    pub id_provider: IdProvider,

    /// the type_name of the context for a reference (e.g. `a.b` where `a`'s type is the context of `b`)
    qualifier: Option<&'s str>,

    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU`'s body)
    pou: Option<&'s str>,

    /// special context of the left-hand-side of an assignment in call statements
    /// Inside the left hand side of an assignment is in the context of the call's POU
    /// `foo(a := a)` actually means: `foo(foo.a := POU.a)`
    lhs: Option<&'s str>,

    /// Set to true when dealing with a member reference at the left-hand side of an assignment. This is
    /// needed to correctly annotate the type of the property we're dealing with, i.e. a get- or set-accessor.
    property_set: bool,

    /// true if the expression passed a constant-variable on the way
    /// e.g. true for `x` if x is declared in a constant block
    /// e.g. true for `a.b.c` if either a,b or c is declared in a constant block
    constant: bool,

    /// true if the visitor entered a body (so no declarations)
    in_body: bool,

    /// true if the visitor entered a control statement
    in_control: bool,

    // what's the current strategy for resolving
    resolve_strategy: Vec<ResolvingStrategy>,
}

impl<'s> VisitorContext<'s> {
    /// returns a copy of the current context and changes the `current_qualifier` to the given qualifier
    fn with_qualifier(&self, qualifier: &'s str) -> VisitorContext<'s> {
        let mut ctx = self.clone();
        ctx.qualifier = Some(qualifier);
        ctx.constant = false;
        ctx
    }

    /// returns a copy of the current context and changes the `current_pou` to the given pou
    pub fn with_pou(&self, pou: &'s str) -> VisitorContext<'s> {
        let mut ctx = self.clone();
        ctx.pou = Some(pou);
        ctx.constant = false;
        ctx
    }

    /// returns a copy of the current context and changes the `lhs` to the given identifier
    fn with_lhs(&self, lhs: &'s str) -> VisitorContext<'s> {
        let mut ctx = self.clone();
        ctx.lhs = Some(lhs);
        ctx.constant = false;
        ctx
    }

    /// returns a copy of the current context and changes the `is_call` to true
    fn with_const(&self, const_state: bool) -> VisitorContext<'s> {
        let mut ctx = self.clone();
        ctx.constant = const_state;
        ctx
    }

    fn with_property_set(&self, is_member: bool) -> VisitorContext<'s> {
        let mut ctx = self.clone();
        ctx.property_set = is_member;
        ctx
    }

    // returns a copy of the current context and sets the in_body field to true
    fn enter_body(&self) -> Self {
        let mut ctx = self.clone();
        ctx.in_body = true;
        ctx
    }

    fn enter_control(&self) -> Self {
        let mut ctx = self.clone();
        ctx.in_control = true;
        ctx
    }

    // returns a copy of the current context and sets the resolve_strategy field to the given strategies
    pub fn with_resolving_strategy(&self, resolve_strategy: Vec<ResolvingStrategy>) -> Self {
        let mut ctx = self.clone();
        ctx.in_body = true;
        ctx.resolve_strategy = resolve_strategy;
        ctx
    }

    fn with_property_strategy(&self) -> Self {
        let mut ctx = self.clone();
        ctx.resolve_strategy.push(ResolvingStrategy::Property);
        ctx
    }

    fn is_in_a_body(&self) -> bool {
        self.in_body
    }
}

pub struct TypeAnnotator<'i> {
    pub(crate) index: &'i Index,
    pub annotation_map: AnnotationMapImpl,
    string_literals: StringLiterals,
    dependencies: FxIndexSet<Dependency>,
    /// A map containing every jump encountered in a file, and the label of where this jump should
    /// point. This is later used to annotate all jumps after the initial visit is done.
    jumps_to_annotate: FxHashMap<String, FxHashMap<String, Vec<AstId>>>,
    // Scope to search for variables in
    scopes: Scopes,
}

impl TypeAnnotator<'_> {
    pub fn annotate_with_id(&mut self, id: AstId, annotation: StatementAnnotation) {
        match &annotation {
            StatementAnnotation::Function { return_type, qualified_name, call_name, .. } => {
                let name = call_name.as_ref().unwrap_or(qualified_name);
                self.dependencies.insert(Dependency::Call(name.to_string()));
                self.dependencies.extend(self.get_datatype_dependencies(name, FxIndexSet::default()));
                self.dependencies.extend(self.get_datatype_dependencies(return_type, FxIndexSet::default()));
            }
            StatementAnnotation::Program { qualified_name } => {
                self.dependencies.insert(Dependency::Call(qualified_name.to_string()));
                self.dependencies
                    .extend(self.get_datatype_dependencies(qualified_name, FxIndexSet::default()));
            }
            StatementAnnotation::Variable {
                resulting_type,
                qualified_name,
                argument_type,
                auto_deref,
                ..
            } => {
                if matches!(argument_type.get_inner(), VariableType::Global) {
                    match auto_deref {
                        Some(AutoDerefType::Alias(inner)) | Some(AutoDerefType::Reference(inner)) => {
                            self.dependencies.insert(Dependency::Datatype(inner.to_owned()));
                        }
                        _ => (),
                    };
                    self.dependencies
                        .extend(self.get_datatype_dependencies(resulting_type, FxIndexSet::default()));
                    self.dependencies.insert(Dependency::Variable(qualified_name.to_string()));
                }
            }
            StatementAnnotation::Value { resulting_type } => {
                self.dependencies.insert(Dependency::Datatype(resulting_type.to_string()));
            }
            _ => (),
        };
        self.annotation_map.annotate_with_id(id, annotation);
    }

    pub fn annotate(&mut self, s: &AstNode, annotation: StatementAnnotation) {
        self.annotate_with_id(s.get_id(), annotation);
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

    pub fn annotate_arguments(&mut self, operator: &AstNode, arguments_node: &AstNode, ctx: &VisitorContext) {
        self.visit_statement(ctx, arguments_node);
        let arguments = flatten_expression_list(arguments_node);

        let pou_name = {
            let name = self.get_call_name(operator);
            let implementation = self.index.find_implementation_by_name(&name);
            implementation.map(|it| it.get_type_name().to_string()).unwrap_or(name)
        };

        let generics = if arguments.iter().any(|arg| arg.is_assignment() | arg.is_output_assignment()) {
            self.annotate_arguments_named(&pou_name, arguments)
        } else {
            self.annotate_arguments_positional(&pou_name, operator, arguments)
        };

        self.update_generic_call_statement(generics, &pou_name, operator, arguments_node, ctx.to_owned());
    }

    fn annotate_arguments_named(
        &mut self,
        pou_name: &str,
        arguments: Vec<&AstNode>,
    ) -> FxHashMap<String, Vec<String>> {
        let mut generics_candidates = FxHashMap::<String, Vec<String>>::default();

        for argument in arguments {
            let Some(var_name) = argument.get_assignment_identifier() else {
                continue;
            };

            let Some((parameter, depth)) =
                TypeAnnotator::find_pou_member_and_depth(self.index, pou_name, var_name)
            else {
                continue;
            };

            if let Some((key, candidate)) = self.get_generic_candidate(parameter.get_type_name(), argument) {
                generics_candidates.entry(key.to_string()).or_default().push(candidate.to_string());
                continue;
            }

            self.annotate_argument(
                parameter.get_qualifier().expect("parameter must have a qualifier"),
                argument,
                parameter.get_type_name(),
                depth,
                parameter.get_location_in_parent() as usize,
            );
        }

        generics_candidates
    }

    fn annotate_arguments_positional(
        &mut self,
        pou_name: &str,
        operator: &AstNode,
        arguments: Vec<&AstNode>,
    ) -> FxHashMap<String, Vec<String>> {
        let mut generic_candidates: FxHashMap<String, Vec<String>> = FxHashMap::default();
        let mut positional_candidates = Vec::new();

        let mut arguments = if self.annotation_map.get(operator).is_some_and(|opt| opt.is_fnptr()) {
            // When dealing with a function pointer (which are only supported in the context of methods and
            // direct function block calls), the first argument will be a instance of the POU, e.g.
            // `fnPtrToMyFbEcho^(instanceFb)`, hence we must skip the first argument as otherwise the
            // remaining arguments will receive an incorrect type hint. Again, for example assume we have
            // `fnPtrToMyFbEcho^(instanceFb, 'stringValue', 5)` and we do not skip the first argument. Then,
            // `instanceFB` will have a type-hint of "STRING" and `stringValue` will have a type-hint on
            // `DINT`. This then results in an error in the codegen. Somewhat "ugly" I have to admit and a
            // better approach would be to lower method calls from `fbInstance.echo('stringValue', 5)` to
            // `fbInstance.echo(fbInstance, 'stringValue', 5)` but this has to do for now
            arguments[1..].iter()
        } else {
            arguments.iter()
        };

        // Zip the parameters together with the arguments, then link the correct type information to them.
        for (parameter, argument) in self.index.get_available_parameters(pou_name).iter().zip(&mut arguments)
        {
            let parameter_type_name = parameter.get_type_name();

            match self.get_generic_candidate(parameter_type_name, argument) {
                Some((key, candidate)) => {
                    generic_candidates.entry(key.to_string()).or_default().push(candidate.to_string());
                }

                None => {
                    let candidate =
                        (argument, parameter_type_name.to_string(), parameter.get_location_in_parent());
                    positional_candidates.push(candidate);
                }
            }
        }

        // When dealing with variadic arguments, the previous zip will not have consumed all arguments because
        // potentially we have more arguments than parameters. In that case, check if we are dealing with a
        // variadic argument and if so, iterate over the remaining arguments.
        if let Some(vararg) = self.index.get_variadic_member(pou_name) {
            for argument in arguments {
                if let Some((key, candidate)) = self.get_generic_candidate(vararg.get_type_name(), argument) {
                    generic_candidates.entry(key.to_string()).or_default().push(candidate.to_string());
                    continue;
                }

                let type_name = self.get_vararg_type_name(argument, vararg);
                positional_candidates.push((argument, type_name, vararg.get_location_in_parent()));
            }
        }

        for (argument, type_name, position) in positional_candidates {
            self.annotate_argument(pou_name, argument, &type_name, 0, position as usize);
        }

        generic_candidates
    }

    fn get_vararg_type_name(&mut self, argument: &&AstNode, vararg: &VariableIndexEntry) -> String {
        // intrinsic type promotion for variadics in order to be compatible with the C standard.
        // see ISO/IEC 9899:1999, 6.5.2.2 Function calls (https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1256.pdf)
        // or https://en.cppreference.com/w/cpp/language/implicit_conversion#Integral_promotion
        // for more about default argument promotion.
        //
        // varargs without a type declaration will be annotated "VOID", so in order to check if a
        // promotion is necessary, we need to first check the type of each parameter. in the case of numerical
        // types, we promote if the type is smaller than double/i32 (except for booleans).

        let type_name = match self.annotation_map.get_type(argument, self.index) {
            Some(data_type) => match &data_type.information {
                DataTypeInformation::Float { .. } => {
                    get_bigger_type(data_type, self.index.get_type_or_panic(LREAL_TYPE), self.index)
                        .get_name()
                }
                DataTypeInformation::Integer { .. }
                    if !data_type.information.is_bool() && !data_type.information.is_character() =>
                {
                    get_bigger_type(data_type, self.index.get_type_or_panic(DINT_TYPE), self.index).get_name()
                }
                // Enum types need to be promoted based on their underlying integer type
                DataTypeInformation::Enum { referenced_type, .. } => self
                    .index
                    .get_effective_type_by_name(referenced_type)
                    .ok()
                    .filter(|dt| {
                        let info = dt.get_type_information();
                        info.is_int() && !(info.is_bool() || info.is_character())
                    })
                    .map(|enum_base_type| {
                        get_bigger_type(enum_base_type, self.index.get_type_or_panic(DINT_TYPE), self.index)
                            .get_name()
                    })
                    .unwrap_or(vararg.get_type_name()),

                _ => vararg.get_type_name(),
            },

            None => vararg.get_type_name(),
        };

        type_name.to_string()
    }

    /// Finds a member in the specified POU, traversing the inheritance chain if necessary. Returns the
    /// [`VariableIndexEntry`] along with the inheritance depth from the given POU to where the member
    /// was declared.
    fn find_pou_member_and_depth<'a>(
        index: &'a Index,
        pou: &str,
        name: &str,
    ) -> Option<(&'a VariableIndexEntry, usize)> {
        fn find<'a>(index: &'a Index, pou: &str, name: &str) -> Option<&'a VariableIndexEntry> {
            index.find_type(pou).and_then(|pou| pou.find_member(name))
        }

        // Check if the POU has the member locally
        if let Some(entry) = find(index, pou, name) {
            return Some((entry, 0));
        }

        // ..and if not walk the inheritance chain and re-try
        let mut depth = 1;
        let mut current_pou = pou;

        while let Some(parent) = index.find_pou(current_pou).and_then(PouIndexEntry::get_super_class) {
            if let Some(entry) = find(index, parent, name) {
                return Some((entry, depth));
            }

            depth += 1;
            current_pou = parent;
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum AutoDerefType {
    #[default]
    Default,
    Alias(String),
    Reference(String),
}

impl AutoDerefType {
    pub fn get_inner(&self) -> Option<String> {
        match self {
            AutoDerefType::Default => None,
            AutoDerefType::Alias(inner) | AutoDerefType::Reference(inner) => Some(inner.to_owned()),
        }
    }
}

impl From<ast::AutoDerefType> for AutoDerefType {
    fn from(value: ast::AutoDerefType) -> Self {
        match value {
            ast::AutoDerefType::Default => AutoDerefType::Default,
            ast::AutoDerefType::Alias => AutoDerefType::Alias(String::new()),
            ast::AutoDerefType::Reference => AutoDerefType::Reference(String::new()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum StatementAnnotation {
    /// an expression that resolves to a certain type (e.g. `a + b` --> `INT`)
    Value {
        resulting_type: String,
    },
    /// An argument of a call statement
    Argument {
        /// The resulting type of this argument
        resulting_type: String,

        /// The position of the parameter within its declared POU.
        position: usize,

        /// Inheritance depth from parameter declaration to calling context.
        ///
        /// Given an inheritance chain `A <- B <- C` and `instanceC(inA := 1, inB := 2, inC := 3)`:
        /// - `inA := 1` will have a depth of 2 (declared in grandparent A)
        /// - `inB := 2` will have a depth of 1 (declared in parent B)
        /// - `inC := 3` will have a depth of 0 (declared in C itself)
        depth: usize,

        /// The POU name where this arguments parameter is declared, which may differ from the actual POU
        /// being called.
        ///
        /// Given an inheritance chain `A <- B <- C` and `instanceC(inA := 1, inB := 2, inC := 3)`:
        /// - `inA := 1` will have a POU name of `A`
        /// - `inB := 2` will have a POU name of `B`
        /// - `inC := 3` will have a POU name of `C`
        pou: String,
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
        /// denotes wheter this variable has the auto-deref trait and if so what type
        auto_deref: Option<AutoDerefType>,
    },
    /// a reference to a function
    Function {
        /// The defined return type of the function
        return_type: String,
        /// The defined qualified name of the function
        qualified_name: String,
        /// Original name before generic resolution
        generic_name: Option<String>,
        /// The call name of the function iff it differs from the qualified name (generics)
        call_name: Option<String>,
    },
    /// A pointer to a function, mostly needed for polymorphism where function calls are handled indirectly
    /// with a virtual table.
    FunctionPointer {
        /// The return type name of the function pointer
        return_type: String,

        // XXX: In classical function pointers such information is not neccessary, e.g. in C you would have
        //      `<return type> (*<function pointer name>)(<comma seperated arg types>)`, however in ST I
        //      __think__ it might be neccessary because of named arguments? Obviously this limits the use
        //      case of function pointers because you'd always reference a concrete function rather than some
        //      generic function such as `DINT(STRING, INT)`. If we ever decide to support function pointers
        //      for real, we probably want to include another syntax similar to C with the limitation of not
        //      supporting named arguments.
        /// The name of the referenced function, e.g. `MyFb.myMethod` in `POINTER TO MyFb.MyMethod := ADR(...)`
        qualified_name: String,
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
    /// a reference to a label in a POU
    Label {
        name: String,
    },
    /// A method override
    Override {
        // The qualified name of all definitions of this method in interfaces or base classes
        definitions: Vec<MethodDeclarationType>,
    },
    Super {
        // Name of the super class (EXTENDS)
        name: String,
        // name of all interfaces implemented by the class
        interfaces: Vec<String>,
    },
    MethodDeclarations {
        declarations: FxHashMap<String, Vec<MethodDeclarationType>>,
    },
    Property {
        // TODO: Is this neccesary?
        name: String,
    },
    #[default]
    None,
}

type QualifiedName = String;

#[derive(Debug, Hash, Clone, PartialEq)]
pub enum MethodDeclarationType {
    Abstract(QualifiedName),
    Concrete(QualifiedName),
}

impl MethodDeclarationType {
    pub fn abstract_method(name: &str) -> Self {
        MethodDeclarationType::Abstract(name.into())
    }

    pub fn concrete_method(name: &str) -> Self {
        MethodDeclarationType::Concrete(name.into())
    }

    pub fn is_abstract(&self) -> bool {
        matches!(self, Self::Abstract(_))
    }

    pub fn is_concrete(&self) -> bool {
        matches!(self, Self::Concrete(_))
    }

    pub fn get_qualified_name(&self) -> &str {
        match self {
            MethodDeclarationType::Abstract(name) | MethodDeclarationType::Concrete(name) => name,
        }
    }

    pub fn get_qualifier(&self) -> &str {
        match self {
            MethodDeclarationType::Abstract(name) | MethodDeclarationType::Concrete(name) => {
                name.rsplit_once('.').map(|(qualifier, _)| qualifier).unwrap_or(name)
            }
        }
    }

    pub fn get_flat_name(&self) -> &str {
        match self {
            MethodDeclarationType::Abstract(name) | MethodDeclarationType::Concrete(name) => {
                name.rsplit_once('.').map(|(_, flat_name)| flat_name).unwrap_or(name)
            }
        }
    }
}

trait InheritanceAnnotationConverter {
    /// Returns a [`StatementAnnotation::MethodDeclarations`] for all method declarations of this entry,
    /// including all inherited methods.
    fn get_method_declarations_annotation(&self, index: &Index) -> Option<StatementAnnotation> {
        let mut declarations = FxHashMap::<String, Vec<MethodDeclarationType>>::default();
        self.get_method_declaration_types(index).into_iter().for_each(|method| {
            declarations.entry(method.get_flat_name().to_string()).or_default().push(method)
        });
        (!declarations.is_empty()).then_some(StatementAnnotation::MethodDeclarations { declarations })
    }

    /// Returns a [`StatementAnnotation::Override`] for all method overrides of this entry,
    /// including all inherited, overridden methods.
    fn get_method_overrides_annotation(&self, index: &Index) -> Option<StatementAnnotation> {
        // TODO: lazy inheritance iterator
        let definitions = self.get_method_overrides(index);
        (!definitions.is_empty()).then_some(StatementAnnotation::create_override(definitions))
    }

    /// Returns all method declaration-types (abstract/concrete) of this entry, including all inherited methods.
    fn get_method_declaration_types(&self, index: &Index) -> Vec<MethodDeclarationType>;

    /// Returns all overridden method declaration-types (abstract/concrete) of this entry, including all inherited methods.
    fn get_method_overrides(&self, index: &Index) -> Vec<MethodDeclarationType>;
}

impl InheritanceAnnotationConverter for InterfaceIndexEntry {
    fn get_method_declaration_types(&self, index: &Index) -> Vec<MethodDeclarationType> {
        self.get_methods(index)
            .iter()
            .map(|method| MethodDeclarationType::abstract_method(method.get_name()))
            .collect()
    }

    fn get_method_overrides(&self, index: &Index) -> Vec<MethodDeclarationType> {
        let derived_methods = self.get_derived_methods(index);
        self.methods
            .iter()
            .filter(|method_name| {
                derived_methods.iter().any(|derived_method| {
                    derived_method.get_call_name()
                        == method_name.rsplit_once(".").map(|(_, it)| it).unwrap_or_default()
                })
            })
            .map(|method_name| MethodDeclarationType::abstract_method(method_name))
            .collect()
    }
}

impl InheritanceAnnotationConverter for PouIndexEntry {
    fn get_method_declaration_types(&self, index: &Index) -> Vec<MethodDeclarationType> {
        match self {
            PouIndexEntry::Program { name, .. }
            | PouIndexEntry::FunctionBlock { name, .. }
            | PouIndexEntry::Class { name, .. } => {
                //Find all declared methods
                index
                    .get_methods(name)
                    .into_iter()
                    .map(|it| MethodDeclarationType::concrete_method(it.get_name()))
                    .chain(self.get_interfaces().iter().flat_map(|ident| {
                        index
                            .find_interface(ident)
                            .into_iter()
                            .flat_map(|interface| interface.get_method_declaration_types(index))
                    }))
                    .collect()
            }
            _ => vec![],
        }
    }

    fn get_method_overrides(&self, index: &Index) -> Vec<MethodDeclarationType> {
        let PouIndexEntry::Method { ref parent_name, declaration_kind: kind, .. } = self else {
            return vec![];
        };

        let mut overrides = vec![];
        let method_name = self.get_call_name();

        // Annotate as concrete override if a super-class also defines this method
        if let Some(inherited_method) = index
            .find_pou(parent_name)
            .and_then(|pou| pou.get_super_class())
            .and_then(|super_name| index.find_method(super_name, method_name))
        {
            overrides.push(MethodDeclarationType::concrete_method(inherited_method.get_name()));
        };

        let interfaces = match kind {
            ast::DeclarationKind::Abstract => index
                .find_interface(parent_name)
                .map(|it| it.get_extensions().iter().map(|it| it.name.as_str()).collect()),
            ast::DeclarationKind::Concrete => index.find_pou(parent_name).map(|it| it.get_interfaces()),
        }
        .unwrap_or_default();
        // Annotate all implemented methods which were inherited from interfaces
        interfaces.iter().for_each(|interface| {
            if let Some(interface) = index.find_interface(interface) {
                interface
                    .get_methods(index)
                    .iter()
                    .filter(|it| it.get_call_name() == method_name)
                    .for_each(|it| overrides.push(MethodDeclarationType::abstract_method(it.get_name())));
            }
        });

        overrides
    }
}

impl StatementAnnotation {
    /// Constructs a new [`StatementAnnotation::Value`] with the given type name
    pub fn value(type_name: impl Into<String>) -> Self {
        StatementAnnotation::Value { resulting_type: type_name.into() }
    }

    /// Constructs a new [`StatementAnnotation::FunctionPointer`] with the qualified and return type name
    pub fn fnptr<T, U>(qualified_name: T, return_type_name: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        StatementAnnotation::FunctionPointer {
            return_type: return_type_name.into(),
            qualified_name: qualified_name.into(),
        }
    }

    pub fn create_override(definitions: Vec<MethodDeclarationType>) -> Self {
        StatementAnnotation::Override { definitions }
    }

    pub fn with_generic_name(self, generic_name: &str) -> Self {
        match self {
            StatementAnnotation::Function { return_type, qualified_name, call_name, .. } => {
                StatementAnnotation::Function {
                    return_type,
                    qualified_name,
                    generic_name: Some(generic_name.to_string()),
                    call_name,
                }
            }
            _ => self,
        }
    }

    pub fn is_const(&self) -> bool {
        match self {
            StatementAnnotation::Variable { constant, .. } => *constant,
            _ => false,
        }
    }

    pub fn is_alias(&self) -> bool {
        matches!(self, StatementAnnotation::Variable { auto_deref: Some(AutoDerefType::Alias(_)), .. })
    }

    pub fn is_reference_to(&self) -> bool {
        matches!(self, StatementAnnotation::Variable { auto_deref: Some(AutoDerefType::Reference(_)), .. })
    }

    pub fn is_auto_deref(&self) -> bool {
        matches!(self, StatementAnnotation::Variable { auto_deref: Some(_), .. })
    }

    pub fn data_type(type_name: &str) -> Self {
        StatementAnnotation::Type { type_name: type_name.into() }
    }

    pub fn is_property(&self) -> bool {
        matches!(self, StatementAnnotation::Property { .. })
    }

    pub fn qualified_name(&self) -> Option<&str> {
        match self {
            StatementAnnotation::Variable { qualified_name, .. }
            | StatementAnnotation::Function { qualified_name, .. }
            | StatementAnnotation::FunctionPointer { qualified_name, .. }
            | StatementAnnotation::Program { qualified_name } => Some(qualified_name.as_str()),

            _ => None,
        }
    }

    pub fn is_fnptr(&self) -> bool {
        matches!(self, StatementAnnotation::FunctionPointer { .. })
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
                generic_name: None,
                call_name: None,
            },
            PouIndexEntry::Class { name, .. } => {
                StatementAnnotation::Program { qualified_name: name.to_string() }
            }
            PouIndexEntry::Method { name, return_type, .. } => StatementAnnotation::Function {
                return_type: return_type.to_string(),
                qualified_name: name.to_string(),
                generic_name: None,
                call_name: None,
            },
            PouIndexEntry::Action { name, .. } => {
                StatementAnnotation::Program { qualified_name: name.to_string() }
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Dependency {
    Datatype(String),
    Call(String),
    Variable(String),
}

impl Dependency {
    pub fn get_name(&self) -> &str {
        match self {
            Dependency::Datatype(name) | Dependency::Call(name) | Dependency::Variable(name) => name,
        }
    }
}

pub trait AnnotationMap {
    fn get(&self, s: &AstNode) -> Option<&StatementAnnotation> {
        self.get_with_id(s.get_id())
    }

    fn get_with_id(&self, id: AstId) -> Option<&StatementAnnotation>;

    fn get_hint(&self, s: &AstNode) -> Option<&StatementAnnotation> {
        self.get_hint_with_id(s.get_id())
    }

    fn get_hint_with_id(&self, id: AstId) -> Option<&StatementAnnotation>;

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
            StatementAnnotation::Argument { resulting_type, .. } => Some(resulting_type.as_str()),
            StatementAnnotation::Variable { resulting_type, .. } => Some(resulting_type.as_str()),
            StatementAnnotation::ReplacementAst { statement } => self
                .get_hint(statement)
                .or_else(|| self.get(statement))
                .and_then(|it| self.get_type_name_for_annotation(it)),
            StatementAnnotation::Type { type_name } => Some(type_name),
            StatementAnnotation::Program { qualified_name }
            | StatementAnnotation::Super { name: qualified_name, .. }
            | StatementAnnotation::Function { qualified_name, .. }
            | StatementAnnotation::FunctionPointer { qualified_name, .. } => Some(qualified_name),
            StatementAnnotation::Label { .. }
            | StatementAnnotation::Override { .. }
            | StatementAnnotation::MethodDeclarations { .. }
            | StatementAnnotation::Property { .. }
            | StatementAnnotation::None => None,
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
            Some(StatementAnnotation::Function { qualified_name, .. })
            | Some(StatementAnnotation::Variable { qualified_name, .. })
            | Some(StatementAnnotation::Program { qualified_name, .. }) => Some(qualified_name.as_str()),

            _ => self.get_call_name(s),
        }
    }

    fn has_type_annotation(&self, s: &AstNode) -> bool;

    fn get_generic_nature(&self, s: &AstNode) -> Option<&TypeNature>;

    fn import(&mut self, other: AnnotationMapImpl);
}

#[derive(Debug, Default)]
pub struct AstAnnotations {
    pub annotation_map: AnnotationMapImpl,
    bool_id: AstId,

    bool_annotation: StatementAnnotation,
}

impl AnnotationMap for AstAnnotations {
    fn get_with_id(&self, id: AstId) -> Option<&StatementAnnotation> {
        if id == self.bool_id {
            Some(&self.bool_annotation)
        } else {
            self.annotation_map.get_with_id(id)
        }
    }

    fn get_hint_with_id(&self, id: AstId) -> Option<&StatementAnnotation> {
        if id == self.bool_id {
            Some(&self.bool_annotation)
        } else {
            self.annotation_map.get_hint_with_id(id)
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

    fn import(&mut self, other: AnnotationMapImpl) {
        self.annotation_map.import(other);
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
    type_map: FxIndexMap<AstId, StatementAnnotation>,

    /// maps a statement to the target-type it should eventually resolve to
    /// example:
    /// x : BYTE := 1;  //1's actual type is DINT, 1's target type is BYTE
    /// x : INT := 1;   //1's actual type is DINT, 1's target type is INT
    type_hint_map: FxIndexMap<AstId, StatementAnnotation>,

    /// A map from a call to the generic function name of that call
    generic_nature_map: FxIndexMap<AstId, TypeNature>,

    /// maps a function to a statement
    ///
    /// currently used for `SubRange`check functions
    /// these functions are not called directly and are therefore maped to the corresponding statement here
    /// example:
    /// FUNCTION CheckRangeUnsigned : UDINT
    /// ...
    /// x : BYTE(0..100);
    /// x := 10; // a call to `CheckRangeUnsigned` is maped to `10`
    hidden_function_calls: FxIndexMap<AstId, AstNode>,

    // An index of newly created types
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

    pub fn annotate_with_id(&mut self, id: AstId, annotation: StatementAnnotation) {
        self.type_map.insert(id, annotation);
    }

    pub fn annotate_type_hint(&mut self, s: &AstNode, annotation: StatementAnnotation) {
        self.type_hint_map.insert(s.get_id(), annotation);
    }

    pub fn annotate_type_hint_with_id(&mut self, id: AstId, annotation: StatementAnnotation) {
        self.type_hint_map.insert(id, annotation);
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
    fn get_with_id(&self, id: AstId) -> Option<&StatementAnnotation> {
        self.type_map.get(&id)
    }

    fn get_hint_with_id(&self, id: AstId) -> Option<&StatementAnnotation> {
        self.type_hint_map.get(&id)
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

    fn import(&mut self, other: AnnotationMapImpl) {
        self.import(other);
    }
}

#[derive(Default, Debug)]
pub struct StringLiterals {
    pub utf08: FxHashSet<String>,
    pub utf16: FxHashSet<String>,
}

impl StringLiterals {
    pub fn import(&mut self, other: StringLiterals) {
        self.utf08.extend(other.utf08);
        self.utf16.extend(other.utf16);
    }
}

impl<'i> TypeAnnotator<'i> {
    /// constructs a new TypeAnnotater that works with the given index for type-lookups
    pub fn new(index: &'i Index) -> TypeAnnotator<'i> {
        TypeAnnotator {
            annotation_map: AnnotationMapImpl::new(),
            index,
            dependencies: FxIndexSet::default(),
            string_literals: StringLiterals { utf08: FxHashSet::default(), utf16: FxHashSet::default() },
            jumps_to_annotate: FxHashMap::default(),
            scopes: Scopes::global(),
        }
    }

    /// annotates the given AST elements with the type-name resulting for the statements/expressions.
    /// Returns an AnnotationMap with the resulting types for all visited Statements. See `AnnotationMap`
    pub fn visit_unit(
        index: &Index,
        unit: &'i CompilationUnit,
        id_provider: IdProvider,
    ) -> (AnnotationMapImpl, FxIndexSet<Dependency>, StringLiterals) {
        let mut visitor = TypeAnnotator::new(index);
        let ctx = &VisitorContext {
            id_provider,
            resolve_strategy: ResolvingStrategy::default_scopes(),
            ..Default::default()
        };

        for global_variable in unit.global_vars.iter().flat_map(|it| it.variables.iter()) {
            visitor.dependencies.insert(Dependency::Variable(global_variable.name.to_string()));
            visitor.visit_variable(ctx, global_variable);
        }

        for pou in &unit.pous {
            visitor.visit_pou(ctx, pou);
        }

        for interface in &unit.interfaces {
            visitor.visit_interface(interface);
        }

        for t in &unit.user_types {
            visitor.visit_user_type_declaration(t, ctx);
        }

        let body_ctx = ctx.enter_body();
        for i in &unit.implementations {
            visitor.dependencies.extend(visitor.get_datatype_dependencies(&i.name, FxIndexSet::default()));
            i.statements.iter().for_each(|s| visitor.visit_statement(&body_ctx.with_pou(i.name.as_str()), s));
        }

        for config_variable in &unit.var_config {
            visitor.visit_statement(ctx, &config_variable.reference);
        }

        // enum initializers may have been introduced by the visitor (indexer)
        // so we should try to resolve and type-annotate them here as well
        for enum_element in index
            .get_all_enum_variants()
            .iter()
            .filter(|it| it.is_in_unit(unit.file.get_name().unwrap_or_default()))
        {
            //Add to dependency map
            visitor.dependencies.insert(Dependency::Variable(enum_element.get_qualified_name().to_string()));
            if let Some((Some(statement), scope)) =
                enum_element.initial_value.map(|i| index.get_const_expressions().find_expression(&i))
            {
                if let Some(scope) = scope {
                    visitor.visit_statement(&ctx.with_pou(scope), statement);
                } else {
                    visitor.visit_statement(ctx, statement);
                }
            }
        }

        //Labels have been added to the index, annotate jumps with their appropriate labels
        for (pou, jumps) in visitor.jumps_to_annotate {
            for (label, nodes) in jumps {
                for node in nodes {
                    if let Some(label) = visitor.annotation_map.new_index.get_label(&pou, &label) {
                        visitor
                            .annotation_map
                            .type_map
                            .insert(node, StatementAnnotation::Label { name: label.name.clone() });
                    }
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

                // Update the type-hint for the initializer
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
        if let Some(Identifier { name, .. }) = &pou.super_class {
            self.dependencies.insert(Dependency::Datatype(name.to_string()));
        }
        self.annotate_pou(pou);
        let pou_ctx = ctx.with_pou(pou.name.as_str());
        for block in &pou.variable_blocks {
            for variable in &block.variables {
                self.visit_variable(&pou_ctx, variable);
            }
        }
    }

    fn visit_interface(&mut self, interface: &Interface) {
        self.annotate_interface(interface);
    }

    fn annotate_pou(&mut self, pou: &Pou) {
        if let Some(pou_entry) = self.index.find_pou(&pou.name) {
            if pou_entry.is_method() {
                self.annotate_method(pou_entry, pou.id);
            } else {
                let Some(annotation) = pou_entry.get_method_declarations_annotation(self.index) else {
                    return;
                };

                self.annotation_map.annotate_with_id(pou.id, annotation);
            }
        }
    }

    fn annotate_method(&mut self, method: &PouIndexEntry, id: AstId) {
        let Some(parent_pou_name) = method.get_parent_pou_name() else { return };

        if method.get_declaration_kind().is_some_and(|it| it.is_concrete()) {
            self.dependencies.insert(Dependency::Datatype(parent_pou_name.into()));
        }

        // If the method is overridden, annotate the method with every inherited method it overrides
        if let Some(annotation) = method.get_method_overrides_annotation(self.index) {
            self.annotation_map.annotate_with_id(id, annotation);
        };
    }

    fn annotate_interface(&mut self, interface: &Interface) {
        let Some(itf) = self.index.find_interface(&interface.ident.name) else { return };

        // annotate overrides of each method declared in the interface
        interface.methods.iter().for_each(|method| {
            self.annotate_pou(method);
        });

        // annotate all methods (declared and derived) in the interface
        if let Some(annotation) = itf.get_method_declarations_annotation(self.index) {
            self.annotation_map.annotate_with_id(interface.id, annotation);
        };
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
        if let AstStatement::ParenExpression(expr) = &right_side.stmt {
            self.update_right_hand_side_expected_type(ctx, annotated_left_side, expr);
            self.inherit_annotations(right_side, expr);
        }

        if let Some(expected_type) = self.annotation_map.get_type(annotated_left_side, self.index).cloned() {
            // for assignments on SubRanges check if there are range type check functions
            if let DataTypeInformation::SubRange { sub_range, referenced_type, .. } =
                expected_type.get_type_information()
            {
                if let Some(statement) = sub_range
                    .start
                    .to_ast_node(self.index, &ctx.id_provider)
                    .zip(sub_range.end.to_ast_node(self.index, &ctx.id_provider))
                    .and_then(|(start, end)| {
                        // Annotate the range bounds with the backing type of the subrange
                        // so they are correctly typed when passed to the check function
                        let backing_type = self.index.get_type(referenced_type).ok()?;
                        self.annotation_map
                            .annotate_type_hint(&start, StatementAnnotation::value(backing_type.get_name()));
                        self.annotation_map
                            .annotate_type_hint(&end, StatementAnnotation::value(backing_type.get_name()));
                        Some(start..end)
                    })
                    .and_then(|range| {
                        self.index
                            .find_range_check_implementation_for(expected_type.get_type_information())
                            .map(|f| {
                                AstFactory::create_call_to_check_function_ast(
                                    f.get_call_name(),
                                    right_side.clone(),
                                    range,
                                    &annotated_left_side.get_location(),
                                    ctx.id_provider.clone(),
                                )
                            })
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
    pub fn update_expected_types(&mut self, expected_type: &typesystem::DataType, statement: &AstNode) {
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
            AstStatement::ParenExpression(expr) => {
                self.update_expected_types(expected_type, expr);
                self.inherit_annotations(statement, expr);
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
                .find_variable(ctx.qualifier.or(ctx.pou), &[variable.name.as_str()])
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

                self.type_hint_for_variable_initializer(
                    initializer,
                    expected_type,
                    &ctx.with_lhs(&variable.name),
                );
            }
        }
    }

    fn type_hint_for_variable_initializer(
        &mut self,
        initializer: &AstNode,
        variable_ty: &typesystem::DataType,
        ctx: &VisitorContext,
    ) {
        if let AstStatement::ParenExpression(expr) = &initializer.stmt {
            self.type_hint_for_variable_initializer(expr, variable_ty, ctx);
            self.inherit_annotations(initializer, expr);
            return;
        }

        self.annotation_map.annotate_type_hint(initializer, StatementAnnotation::value(&variable_ty.name));
        self.update_expected_types(variable_ty, initializer);

        self.type_hint_for_array_of_structs(variable_ty, initializer, ctx);
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
                let ctx = ctx.with_qualifier(inner_data_type.get_name()).with_lhs(inner_data_type.get_name());

                if !inner_data_type.get_type_information().is_struct() {
                    return;
                }

                match statement.get_stmt() {
                    AstStatement::Literal(AstLiteral::Array(array)) => match array.elements() {
                        Some(elements) if elements.is_expression_list() || elements.is_paren() => {
                            self.type_hint_for_array_of_structs(expected_type, elements, &ctx)
                        }

                        _ => (),
                    },

                    AstStatement::ParenExpression(expression) => {
                        self.type_hint_for_array_of_structs(expected_type, expression, &ctx);
                        self.inherit_annotations(statement, expression);
                    }

                    AstStatement::ExpressionList(expressions) => {
                        let name = inner_data_type.get_name();
                        let hint = StatementAnnotation::value(name);

                        for expression in expressions {
                            self.annotation_map.annotate_type_hint(expression, hint.clone());

                            self.visit_statement(&ctx, expression);
                            self.type_hint_for_array_of_structs(expected_type, expression, &ctx);
                        }

                        // annotate the expression list as well
                        self.annotation_map.annotate_type_hint(statement, hint);
                    }

                    AstStatement::Assignment(Assignment { left, right, .. }) if left.is_reference() => {
                        if let AstStatement::Literal(AstLiteral::Array(array)) = right.as_ref().get_stmt() {
                            let Some(elements) = array.elements() else { return };

                            if let Some(datatype) = self.annotation_map.get_type(left, self.index).cloned() {
                                self.type_hint_for_array_of_structs(&datatype, elements, &ctx);
                            }
                        }

                        // https://github.com/PLC-lang/rusty/issues/1019
                        if inner_data_type.information.is_struct() {
                            let name = inner_data_type.get_name();
                            let hint = StatementAnnotation::value(name);
                            self.annotation_map.annotate_type_hint(statement, hint);
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
            let deps = self.get_datatype_dependencies(name, FxIndexSet::default());
            self.dependencies.extend(deps);
        }
        if let DataTypeDeclaration::Definition { data_type, .. } = declaration {
            self.visit_data_type(ctx, data_type);
        }
    }

    fn get_datatype_dependencies(
        &self,
        datatype_name: &str,
        mut resolved: FxIndexSet<Dependency>,
    ) -> FxIndexSet<Dependency> {
        let Some(datatype) = self
            .index
            .find_type(datatype_name)
            .or_else(|| self.annotation_map.new_index.find_type(datatype_name))
        else {
            return resolved;
        };

        // Return if the datatype has already been visited
        if !resolved.insert(Dependency::Datatype(datatype.get_name().to_string())) {
            return resolved;
        };

        match datatype.get_type_information() {
            DataTypeInformation::Struct { members, source, .. } => {
                for member in members {
                    resolved = self.get_datatype_dependencies(member.get_type_name(), resolved);
                }

                match source {
                    StructSource::Pou(PouType::Class | PouType::FunctionBlock) => {
                        // While the members of a struct such as a class or function block are visited
                        // recursively, the vtable itself is declared as a VOID pointer. Thus, when landing
                        // in the pointer variant branch of this function, a datatype dependency of VOID will
                        // be returned. However, the actual vtable is a struct with all the function pointers
                        // of a POU. Therefore, we need to explicitly visit the `__vtable_...` datatype here.
                        let name = format!("__vtable_{}", datatype.get_name());
                        self.get_datatype_dependencies(&name, resolved)
                    }
                    StructSource::Pou(PouType::Method { parent, .. }) => {
                        self.get_datatype_dependencies(parent, resolved)
                    }
                    _ => resolved,
                }
            }
            DataTypeInformation::Array { inner_type_name, .. }
            | DataTypeInformation::Pointer { inner_type_name, .. } => {
                resolved.insert(Dependency::Datatype(datatype.get_type_information().get_name().to_string()));
                self.get_datatype_dependencies(inner_type_name, resolved)
            }
            _ => {
                let dt_info = self.index.get_intrinsic_type_information(datatype.get_type_information());
                self.get_datatype_dependencies(dt_info.get_name(), resolved)
            }
        }
    }

    fn visit_data_type(&mut self, ctx: &VisitorContext, data_type: &DataType) {
        if let Some(name) = data_type.get_name() {
            self.dependencies.insert(Dependency::Datatype(name.to_string()));
        }
        match data_type {
            DataType::StructType { name: Some(name), variables, .. } => {
                let ctx = ctx.with_qualifier(name);
                variables.iter().for_each(|v| self.visit_variable(&ctx, v))
            }
            DataType::ArrayType { referenced_type, .. } => {
                self.visit_data_type_declaration(ctx, referenced_type)
            }
            DataType::VarArgs { referenced_type: Some(referenced_type), .. } => {
                self.visit_data_type_declaration(ctx, referenced_type.as_ref())
            }
            DataType::SubRangeType { referenced_type, bounds, .. } => {
                if let Some(bounds) = bounds {
                    if let Some(expected_type) = self.index.find_effective_type_by_name(referenced_type) {
                        self.visit_statement(ctx, bounds);
                        self.update_expected_types(expected_type, bounds);
                    }
                }
                self.dependencies
                    .extend(self.get_datatype_dependencies(referenced_type, FxIndexSet::default()));
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

    /// This function is only really useful for [`AstStatement::ParenExpression`] where we would
    /// like to annotate the parenthese itself with whatever annotation the inner expression got.
    /// For example ((1 + 2)), `1 + 2` => DINT, but also `(...)` => DINT and `((...)))` => DINT
    fn inherit_annotations(&mut self, paren: &AstNode, inner: &AstNode) {
        if let Some(annotation) = self.annotation_map.get_type(inner, self.index) {
            self.annotate(paren, StatementAnnotation::value(&annotation.name))
        }

        if let Some(annotation) = self.annotation_map.get_type_hint(inner, self.index) {
            self.annotation_map.annotate_type_hint(paren, StatementAnnotation::value(&annotation.name))
        }
    }

    /// annotate a control statement
    fn visit_statement_control(&mut self, ctx: &VisitorContext, statement: &AstNode) {
        match statement.get_stmt() {
            AstStatement::ParenExpression(expr) => {
                self.visit_statement(ctx, expr);
                self.inherit_annotations(statement, expr);
            }
            AstStatement::ControlStatement(control) => {
                match control {
                    AstControlStatement::If(stmt) => {
                        stmt.blocks.iter().for_each(|b| {
                            self.visit_statement(&ctx.enter_control(), b.condition.as_ref());
                            b.body.iter().for_each(|s| self.visit_statement(ctx, s));
                        });
                        stmt.else_block.iter().for_each(|e| self.visit_statement(ctx, e));
                    }
                    AstControlStatement::ForLoop(stmt) => {
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
                    AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
                        self.visit_statement(&ctx.enter_control(), &stmt.condition);
                        stmt.body.iter().for_each(|s| self.visit_statement(ctx, s));
                    }
                    AstControlStatement::Case(stmt) => {
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
                }
            }

            AstStatement::CaseCondition(condition, ..) => self.visit_statement(ctx, condition),
            _ => self.visit_statement_expression(ctx, statement),
        }
    }

    /// annotate an expression statement
    fn visit_statement_expression(&mut self, ctx: &VisitorContext, statement: &AstNode) {
        match statement.get_stmt() {
            AstStatement::This => {
                let name = match ctx.pou.and_then(|name| self.index.find_pou(name)) {
                    Some(PouIndexEntry::FunctionBlock { name, .. }) => name,
                    Some(
                        PouIndexEntry::Method { parent_name: name, .. }
                        | PouIndexEntry::Action { parent_name: name, .. },
                    ) if self.index.find_pou(name).is_some_and(|it| it.is_function_block()) => name,
                    _ => return,
                };
                let ptr_name = format!("{}.__THIS", name);
                if self
                    .index
                    .find_type(&ptr_name)
                    .or_else(|| self.annotation_map.new_index.find_type(&ptr_name))
                    .is_none()
                {
                    let information = DataTypeInformation::Pointer {
                        name: ptr_name.clone(),
                        inner_type_name: name.to_string(),
                        auto_deref: None,
                        type_safe: true,
                        is_function: false, // TODO(vosa): In general false, but THIS^() isn't?
                    };
                    let dt = crate::typesystem::DataType {
                        name: ptr_name.clone(),
                        initial_value: None,
                        information,
                        nature: TypeNature::Any,
                        location: SourceLocation::internal(),
                    };
                    self.annotation_map.new_index.register_type(dt);
                }
                self.annotate(statement, StatementAnnotation::value(ptr_name));
            }
            AstStatement::DirectAccess(data, ..) => {
                let ctx = VisitorContext { qualifier: None, ..ctx.clone() };
                visit_all_statements!(self, &ctx, &data.index);
                let access_type = get_direct_access_type(&data.access);
                self.annotate(statement, StatementAnnotation::value(access_type));
            }
            AstStatement::HardwareAccess(..) => {
                if let Some(annotation) = self.resolve_reference_expression(statement, None, ctx) {
                    self.annotate(statement, annotation.clone());
                }
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

                        let target_name = if data.operator.is_comparison_operator() {
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
                    self.annotate(statement, StatementAnnotation::value(statement_type.clone()));

                    // https://github.com/PLC-lang/rusty/issues/939: We rely on type-hints in order
                    // to identify `=` operations that have no effect (e.g. `foo = bar;`) hence
                    // type-hint the conditions of control statements to eliminate false-positives.
                    if ctx.in_control {
                        self.annotation_map
                            .annotate_type_hint(statement, StatementAnnotation::value(statement_type))
                    }
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
                    self.annotate(statement, StatementAnnotation::value(statement_type));
                }
            }

            AstStatement::ExpressionList(expressions, ..) => {
                expressions.iter().for_each(|e| self.visit_statement(ctx, e))
            }

            AstStatement::RangeStatement(data, ..) => {
                visit_all_statements!(self, ctx, &data.start, &data.end);
            }
            AstStatement::Assignment(data, ..) | AstStatement::RefAssignment(data, ..) => {
                self.visit_statement(&ctx.enter_control(), &data.right);

                // if the LHS of the assignment is a member access, we need to update the context - when trying to resolve
                // a property, this means it must be a setter, not a getter
                let ctx = ctx.with_property_set(data.left.is_member_access());

                if let Some(lhs) = ctx.lhs {
                    //special context for left hand side
                    self.visit_statement(&ctx.with_pou(lhs).with_lhs(lhs), &data.left);
                } else {
                    self.visit_statement(&ctx, &data.left);
                }

                // give a type hint that we want the right side to be stored in the left's type
                self.update_right_hand_side_expected_type(&ctx, &data.left, &data.right);
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
            AstStatement::ReferenceExpr(data, ..) => {
                self.visit_reference_expr(&data.access, data.base.as_deref(), statement, ctx);
            }
            AstStatement::ReturnStatement(ReturnStatement { condition }) => {
                if let Some(condition) = condition {
                    self.visit_statement(ctx, condition)
                }
            }
            AstStatement::LabelStatement(..) => {
                if let Some(pou) = ctx.pou {
                    self.annotation_map.new_index.add_label(pou, statement.into());
                }
            }
            AstStatement::JumpStatement(JumpStatement { condition, target }) => {
                self.visit_statement(ctx, condition);
                if let Some((name, pou)) = target.get_flat_reference_name().zip(ctx.pou) {
                    let pou = self.jumps_to_annotate.entry(pou.to_string()).or_default();
                    let jumps = pou.entry(name.to_string()).or_default();
                    jumps.push(statement.get_id());
                }
            }
            AstStatement::AllocationStatement(Allocation { name, reference_type }) => {
                let qualified_name =
                    if let Some(pou) = ctx.pou { format!("{}.{}", pou, name) } else { name.to_string() };
                self.scopes.enter(Scope::Local(Box::new(VariableIndexEntry::new(
                    name,
                    &qualified_name,
                    reference_type,
                    ArgumentType::ByVal(VariableType::Temp),
                    u32::MAX,
                    statement.get_location(),
                ))))
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
                let new_ctx = &new_ctx.as_ref().unwrap_or(ctx).with_property_strategy();

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

            (ReferenceAccess::Global(node), _) => {
                let ctx = ctx.with_resolving_strategy(vec![ResolvingStrategy::Global]);
                if let Some(annotation) = self.resolve_reference_expression(node, None, &ctx) {
                    // Annotate both the identifier and the statement, i.e. `.foo` and `foo`
                    self.annotate(stmt, annotation.clone());
                    self.annotate(node, annotation);
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
                        &ctx.with_resolving_strategy(vec![ResolvingStrategy::Variable]),
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
            (ReferenceAccess::Index(index), base) => {
                self.visit_statement(ctx, index);
                let Some(base) = base else { return };
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
                if let Some(DataTypeInformation::Pointer {
                    inner_type_name,
                    auto_deref: None,
                    is_function,
                    ..
                }) = base
                    .map(|base| self.annotation_map.get_type_or_void(base, self.index))
                    .map(|it| it.get_type_information())
                {
                    if let Some(inner_type) = self
                        .index
                        .find_effective_type_by_name(inner_type_name)
                        .or(self.annotation_map.new_index.find_effective_type_by_name(inner_type_name))
                    {
                        if ctx
                            .resolve_strategy
                            .first()
                            .is_some_and(|opt| *opt == ResolvingStrategy::FunctionsOnly)
                        {
                            // We might be dealing with a function pointer, e.g. `ptr^(...)`
                            if let Some(pou) = self.index.find_pou(&inner_type.name) {
                                // TODO(vosa): THIS^(), needs to be handled in the polymorphism PR but is
                                // ignored here (!stmt.is_this_deref())
                                if *is_function
                                    && (pou.is_method() | pou.is_function_block() && !stmt.is_this_deref())
                                {
                                    let name = pou.get_name();
                                    let return_type = pou.get_return_type().unwrap_or(VOID_INTERNAL_NAME);

                                    self.annotate(stmt, StatementAnnotation::fnptr(name, return_type));
                                    return;
                                }
                            }
                        }

                        self.annotate(stmt, StatementAnnotation::value(inner_type.get_name()))
                    }
                }
            }
            (ReferenceAccess::Address, _) => {
                if let Some(inner_type) = base
                    .map(|base| self.annotation_map.get_type_or_void(base, self.index).get_name().to_string())
                {
                    let ptr_type = add_pointer_type(&mut self.annotation_map.new_index, inner_type, true);
                    self.annotate(stmt, StatementAnnotation::value(ptr_type))
                }
            }

            _ => (),
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
                .find_map(|scope| scope.resolve_name(name, qualifier, self.index, ctx, &self.scopes)),

            AstStatement::ReferenceExpr(_) => {
                self.visit_statement(ctx, reference);
                self.annotation_map.get(reference).cloned()
            }

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

            AstStatement::HardwareAccess(data, ..) => {
                let name = data.get_mangled_variable_name();
                ctx.resolve_strategy.iter().find_map(|strategy| {
                    strategy.resolve_name(&name, qualifier, self.index, ctx, &self.scopes)
                })
            }

            AstStatement::ParenExpression(expr) => self.resolve_reference_expression(expr, qualifier, ctx),
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
        if let DataTypeInformation::Pointer { inner_type_name, auto_deref: kind, .. } = &self
            .index
            .get_effective_type_or_void_by_name(
                members.first().expect("internal VLA struct ALWAYS has this member").get_type_name(),
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
                auto_deref: kind.map(|it| it.into()),
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
        self.visit_statement(
            &ctx.with_resolving_strategy(ResolvingStrategy::call_operator_scopes()),
            operator,
        );
        let operator_qualifier = self.get_call_name(operator);
        //Use the context without the is_call =true
        //TODO why do we start a lhs context here???
        let ctx = ctx.with_lhs(operator_qualifier.as_str());
        if let Some(parameters) = parameters_stmt {
            self.visit_statement(&ctx, parameters);
        };

        if let Some(annotation) = builtins::get_builtin(&operator_qualifier).and_then(BuiltIn::get_annotation)
        {
            annotation(self, statement, operator, parameters_stmt, ctx.to_owned());
        } else if let Some(arguments) = parameters_stmt {
            //This is skipped for builtins that provide their own annotation-logic
            self.annotate_arguments(operator, arguments, &ctx);
        };

        match self.annotation_map.get(operator) {
            Some(StatementAnnotation::Function { return_type, .. })
            | Some(StatementAnnotation::FunctionPointer { return_type, .. }) => {
                if let Some(return_type) = self
                    .index
                    .find_effective_type_by_name(return_type)
                    .or_else(|| self.annotation_map.new_index.find_effective_type_by_name(return_type))
                {
                    if let Some(StatementAnnotation::ReplacementAst { .. }) =
                        self.annotation_map.get(statement)
                    {
                        // if we have a replacement ast, we do not need to annotate the function return type as it would
                        // overwrite the replacement ast
                        return;
                    }
                    self.annotate(statement, StatementAnnotation::value(return_type.get_name()));
                } else {
                    // Assuming this is a VOID function if no annotation is present
                    self.annotate(statement, StatementAnnotation::value(VOID_INTERNAL_NAME));
                }
            }

            _ => (),
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
                StatementAnnotation::FunctionPointer { qualified_name, .. } => {
                    Some(qualified_name.clone())
                }
                StatementAnnotation::Program { qualified_name } => Some(qualified_name.clone()),
                StatementAnnotation::Variable { resulting_type, .. } => {
                    self.index
                        .find_pou(resulting_type.as_str())
                        .filter(|it| matches!(it, PouIndexEntry::FunctionBlock { .. } | PouIndexEntry::Program { .. }))
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

    fn annotate_argument(
        &mut self,
        pou_name: &str,
        argument: &AstNode,
        type_name: &str,
        depth: usize,
        position: usize,
    ) {
        if let Some(resulting_type) = self.index.find_effective_type_by_name(type_name) {
            let annotation = StatementAnnotation::Argument {
                resulting_type: resulting_type.get_name().to_string(),
                position,
                depth,
                pou: pou_name.to_string(),
            };

            self.annotation_map.annotate_type_hint(argument, annotation);
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
                        self.annotate(statement, StatementAnnotation::value(string_type_name));

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
                        // XXX: if we have a float literal in an initializer (lhs) context, we need to see if the context expects a double or float type.
                        // This is due to `get_real_type_name_for` always returning `REAL_TYPE` for values within f32 range which leads to incorrect typing
                        // during initialization of double values.
                        let type_name = ctx
                            .lhs
                            .as_ref()
                            .and_then(|lhs| {
                                self.index
                                    .find_effective_type_by_name(lhs)
                                    .filter(|it| it.get_type_information().is_float())
                            })
                            .map(typesystem::DataType::get_name)
                            .unwrap_or_else(|| get_real_type_name_for(value));

                        self.annotate(statement, StatementAnnotation::value(type_name));
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
pub(crate) fn add_pointer_type(index: &mut Index, inner_type_name: String, type_safe: bool) -> String {
    let new_type_name = internal_type_name("POINTER_TO_", inner_type_name.as_str());

    if index.find_effective_type_by_name(new_type_name.as_str()).is_none() {
        index.register_type(crate::typesystem::DataType {
            name: new_type_name.clone(),
            initial_value: None,
            nature: TypeNature::Any,
            information: DataTypeInformation::Pointer {
                name: new_type_name.clone(),
                inner_type_name,
                auto_deref: None,
                type_safe,
                is_function: false,
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
            generic_name: None,
            call_name: None,
        }),
        PouIndexEntry::FunctionBlock { name, .. } => {
            Some(StatementAnnotation::Type { type_name: name.into() })
        }
        PouIndexEntry::Action { name, parent_name, .. } => match index.find_pou(parent_name) {
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
    let v_type = index.get_effective_type_or_void_by_name(v.get_type_name());

    //see if this is an auto-deref variable
    let (effective_type_name, kind) = match (v_type.get_type_information(), v.is_return()) {
        (_, true) if v_type.is_aggregate_type() => {
            // treat a return-aggregate variable like an auto-deref pointer since it got
            // passed by-ref
            let kind =
                v_type.get_type_information().get_auto_deref_type().map(|it| it.into()).unwrap_or_default();
            (v_type.get_name().to_string(), Some(kind))
        }
        (DataTypeInformation::Pointer { inner_type_name, auto_deref: Some(deref), name, .. }, _) => {
            // real auto-deref pointer
            let kind = match deref {
                ast::AutoDerefType::Default => AutoDerefType::Default,
                ast::AutoDerefType::Alias => AutoDerefType::Alias(name.to_owned()),
                ast::AutoDerefType::Reference => AutoDerefType::Reference(name.to_owned()),
            };

            (inner_type_name.clone(), Some(kind))
        }
        _ => (v_type.get_name().to_string(), None),
    };

    StatementAnnotation::Variable {
        qualified_name: v.get_qualified_name().into(),
        resulting_type: effective_type_name,
        constant: v.is_constant() || constant_override,
        argument_type: v.get_declaration_type(),
        auto_deref: kind,
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
    if parsed.is_infinite() {
        return LREAL_TYPE;
    }

    REAL_TYPE
}

#[derive(Clone, Debug)]
struct Scopes(Vec<Scope>);

impl Scopes {
    pub fn find_member<'idx>(
        &'idx self,
        index: &'idx Index,
        container_name: &str,
        variable_name: &str,
    ) -> Option<&'idx VariableIndexEntry> {
        self.0.iter().filter_map(|scope| scope.find_member(index, container_name, variable_name)).next()
    }

    pub fn enter(&mut self, scope: Scope) {
        self.0.push(scope)
    }

    //TODO: this is not accurate, we need to pop much more than the the top scope
    #[allow(dead_code)]
    pub fn exit(&mut self) -> Option<Scope> {
        self.0.pop()
    }

    fn global() -> Scopes {
        Scopes(vec![Scope::Global])
    }
}

#[derive(Clone, Debug)]
enum Scope {
    // Global scope relying on an index to find elements
    Global,
    // Local scope declared inline (alloca)
    Local(Box<VariableIndexEntry>),
    // A block scope like the body of an if or for
    #[allow(dead_code)]
    Block,
}

impl Scope {
    pub fn find_member<'idx>(
        &'idx self,
        index: &'idx Index,
        container_name: &str,
        variable_name: &str,
    ) -> Option<&'idx VariableIndexEntry> {
        match self {
            Scope::Global => index.find_member(container_name, variable_name),
            Scope::Local(entry) => {
                if entry.get_name() == variable_name {
                    Some(entry)
                } else {
                    None
                }
            }
            Scope::Block => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResolvingStrategy {
    /// try to resolve a variable
    Variable,

    /// try to resolve a global variable
    Global,

    /// try to resolve a POU
    POU,

    /// try to resolve a DataType
    DataType,

    /// only consider EnumTypes
    EnumTypeOnly,

    /// only consider functions
    FunctionsOnly,

    /// also consider references to properties before they are lowered to actual method calls
    Property,
}

impl ResolvingStrategy {
    /// scopes that can be used for general references. Will resolve to local/global
    /// variables, Pous or datatypes
    pub fn default_scopes() -> Vec<Self> {
        vec![ResolvingStrategy::Variable, ResolvingStrategy::POU, ResolvingStrategy::DataType]
    }

    /// scopes intended for call-statement-operators
    fn call_operator_scopes() -> Vec<ResolvingStrategy> {
        let mut strategy = vec![ResolvingStrategy::FunctionsOnly];
        strategy.extend(Self::default_scopes());
        strategy
    }

    fn resolve_property(
        index: &Index,
        name: &str,
        qualifier: &str,
        property_set: bool,
    ) -> Option<StatementAnnotation> {
        let accessor = if property_set { "set" } else { "get" };
        let name = format!("__{accessor}_{name}");

        // if our current context is a method or action, we need to look for the property in the parent
        let qualifier =
            index.find_pou(qualifier).and_then(|pou| pou.get_parent_pou_name()).unwrap_or(qualifier);

        index.find_method(qualifier, &name).map(|_| StatementAnnotation::Property { name })
    }

    /// tries to resolve the given name using the reprsented scope
    /// - `name` the name to resolve
    /// - `qualifier` an optional qualifier to prefix to the name,
    ///   if the qualifier is present, this method only resolves to targets with this qualifier
    /// - `index` the index to perform the lookups on
    fn resolve_name(
        &self,
        name: &str,
        qualifier: Option<&str>,
        index: &Index,
        ctx: &VisitorContext,
        scopes: &Scopes,
    ) -> Option<StatementAnnotation> {
        match self {
            // try to resolve the name as a variable
            ResolvingStrategy::Variable => {
                if let Some(qualifier) = qualifier {
                    // look for variable, enum with name "qualifier.name"
                    scopes
                        // instance.foo; // Resolver -> referencexpr ->
                        .find_member(index, qualifier, name)
                        .or_else(|| index.find_enum_variant(qualifier, name))
                        .map(|it| to_variable_annotation(it, index, it.is_constant() || ctx.constant))
                } else {
                    // look for member variable with name "pou.name"
                    // then try fopr a global variable called "name"
                    ctx.pou
                        .and_then(|pou| scopes.find_member(index, pou, name))
                        .or_else(|| index.find_global_variable(name))
                        .map(|g| to_variable_annotation(g, index, g.is_constant()))
                }
            }
            ResolvingStrategy::Global => {
                index.find_global_variable(name).map(|g| to_variable_annotation(g, index, g.is_constant()))
            }
            // try to resolve the name as POU/Action/Method
            ResolvingStrategy::POU => {
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
                                //Use the type name of the pou in case we are resolving a
                                //neighboring action
                                index.find_pou(pou).map(|pou| pou.get_container())
                                .and_then(|pou|ResolvingStrategy::POU.resolve_name(name, Some(pou), index, ctx, scopes)))
                    })
                }
            }
            // try to resolve the name as a datatype
            ResolvingStrategy::DataType => {
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
            ResolvingStrategy::EnumTypeOnly => {
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
            ResolvingStrategy::FunctionsOnly => {
                if qualifier.is_none() {
                    // look for function with name "name"
                    index.find_pou(name).filter(|it| it.is_function()).map(|pou| pou.into())
                } else {
                    // there are no qualified functions
                    None
                }
            }
            // try to resolve this name as a property
            ResolvingStrategy::Property => qualifier
                .or(ctx.pou)
                .and_then(|qualifier| Self::resolve_property(index, name, qualifier, ctx.property_set)),
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
    let Some(&AstLiteral::String(StringValue { ref value, is_wide })) = try_from!(literal, AstLiteral) else {
        return;
    };
    match (cast_type.get_type_information(), is_wide) {
        (DataTypeInformation::String { encoding: StringEncoding::Utf8, .. }, true)
        | (DataTypeInformation::String { encoding: StringEncoding::Utf16, .. }, false) => {
            // re-register the string-literal in the opposite encoding
            if is_wide {
                literals.utf08.insert(value.into());
            } else {
                literals.utf16.insert(value.into());
            }
        }
        _ => (),
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
