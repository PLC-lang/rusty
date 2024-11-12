
use plc_ast::ast::AutoDerefType;

use crate::{index::{Index, VariableIndexEntry}, typesystem::{DataTypeInformation, VOID_TYPE}};

use super::{StatementAnnotation};

#[derive(Clone, Debug)]
pub enum Scope {
    /// resolves names to types
    Types,
    /// resolves names to programs, functions, actions, methods
    POUs,
    /// resolves names to global variables
    GlobalVariables,
    /// resolves names to local variables, relative to the given container name
    LocalVariable(String),
    /// delegates to the children-scopes, first hit wins
    Composite(Vec<Scope>),
    /// resolves names to elements that can be called (programs, functions, actions, methods)
    Callable(Option<String>),
    // en empty scope
    Empty,
}

impl Scope {
    fn lookup(&self, identifier: &str, index: &Index) -> Option<StatementAnnotation> {
        match self {
            Scope::Empty => None,
            // lookup a type
            Scope::Types => index
                .find_effective_type_by_name(identifier)
                .map(|dt| dt.get_type_information())
                .map(StatementAnnotation::from),

            Scope::POUs => index.find_pou(identifier).map(StatementAnnotation::from),

            // lookup a global variable
            Scope::GlobalVariables => {
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
                Scope::GlobalVariables,
                Scope::Callable(None),
                Scope::POUs,
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
    pub fn push(&mut self, scope: ScopingStrategy) {
        self.stack.push(scope);
    }

    pub fn pop(&mut self) -> Option<ScopingStrategy> {
        self.stack.pop()
    }
}

fn to_variable_annotation(
    v: &VariableIndexEntry,
    index: &Index,
    constant_override: bool,
) -> StatementAnnotation {
    let v_type = index.get_effective_type_or_void_by_name(v.get_type_name());

    //see if this is an auto-deref variable
    let (effective_type_name, auto_deref) = match (v_type.get_type_information(), v.is_return()) {
        (_, true) if v_type.is_aggregate_type() => {
            // treat a return-aggregate variable like an auto-deref pointer since it got
            // passed by-ref
            (v_type.get_name().to_string(), Some(AutoDerefType::Default))
        }
        (DataTypeInformation::Pointer { inner_type_name, auto_deref , .. }, _) 
        => {
            if matches!(auto_deref, Some(AutoDerefType::Default) | Some(AutoDerefType::Alias) ) {
                // treat a pointer like an auto-deref pointer since it got
                // passed by-ref
                (inner_type_name.clone(), auto_deref.clone())
            } else {
                (v_type.get_name().to_string(), None)
            }
        }
        _ => (v_type.get_name().to_string(), None),
    };

    StatementAnnotation::Variable {
        qualified_name: v.get_qualified_name().into(),
        resulting_type: effective_type_name,
        constant: v.is_constant() || constant_override,
        argument_type: v.get_declaration_type(),
        auto_deref: auto_deref.map(|it| it.into()),
        

    }
}