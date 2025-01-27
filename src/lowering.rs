use crate::{
    index::{get_init_fn_name, Index, PouIndexEntry, VariableIndexEntry},
    resolver::const_evaluator::UnresolvableConstant,
};
use initializers::{Init, InitAssignments, Initializers, GLOBAL_SCOPE};
use plc_ast::{
    ast::{
        AstFactory, AstNode, AstStatement, CallStatement, CompilationUnit, ConfigVariable, DataType,
        LinkageType, PouType,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

pub mod calls;
mod initializers;
pub mod property;
pub mod validator;

pub struct InitVisitor {
    index: Index,
    unresolved_initializers: Initializers,
    var_config_initializers: Vec<AstNode>,
    ctxt: Context,
}

impl InitVisitor {
    pub fn visit(
        mut units: Vec<CompilationUnit>,
        index: Index,
        unresolvables: Vec<UnresolvableConstant>,
        id_provider: IdProvider,
        init_symbol_name: &str,
    ) -> Vec<CompilationUnit> {
        let mut visitor = Self::new(index, unresolvables, id_provider);
        // visit all units
        units.iter_mut().for_each(|unit| {
            visitor.visit_compilation_unit(unit);
        });

        visitor.extend_ast(units, init_symbol_name)
    }

    fn new(
        index: Index,
        unresolved_initializers: Vec<UnresolvableConstant>,
        id_provider: IdProvider,
    ) -> Self {
        Self {
            index,
            unresolved_initializers: Initializers::new(&unresolved_initializers),
            var_config_initializers: vec![],
            ctxt: Context::new(id_provider),
        }
    }

    fn walk_with_scope<T>(&mut self, t: &mut T, pou_name: Option<impl Into<String>>)
    where
        T: WalkerMut,
    {
        let old = self.ctxt.scope(pou_name.map(Into::into));
        t.walk(self);
        self.ctxt.scope(old);
    }

    fn update_initializer(&mut self, variable: &mut plc_ast::ast::Variable) {
        // flat references to stateful pou-local variables need to have a qualifier added, so they can be resolved in the init functions
        let scope = self.ctxt.get_scope().as_ref().map(|it| it.as_str()).unwrap_or(GLOBAL_SCOPE);
        let needs_qualifier = |flat_ref| {
            let rhs = self.index.find_member(scope, flat_ref);
            let lhs = self.index.find_member(scope, variable.get_name());
            let Some(pou) = self.index.find_pou(scope) else { return Ok(false) };
            if !(pou.is_function() || pou.is_method())
                && lhs.is_some_and(|it| !it.is_temp())
                && rhs.is_some_and(|it| it.is_temp())
            {
                // Unable to initialize a stateful member variable with an address of a temporary value since it doesn't exist at the time of initialization
                // On top of that, even if we were to initialize it, it would lead to a dangling pointer/potential use-after-free
                return Err(AstFactory::create_empty_statement(
                    SourceLocation::internal(),
                    self.ctxt.get_id_provider().next_id(),
                ));
            }
            Ok(match pou {
                        PouIndexEntry::Program { .. }
                        | PouIndexEntry::FunctionBlock { .. }
                        | PouIndexEntry::Class { .. }
                            // we only want to add qualifiers to local, non-temporary variables
                            if rhs.is_some_and(|it| !it.is_temp()) && lhs.is_some_and(|it| !it.is_temp())=>
                        {
                            true
                        }
                        _ => false,
                    })
        };

        if let Some(initializer) = variable.initializer.as_ref() {
            let type_name =
                variable.data_type_declaration.get_name().expect("Must have a type at this point");
            let data_type = self.index.get_effective_type_or_void_by_name(type_name).get_type_information();
            if !data_type.is_pointer() {
                return;
            }

            let updated_initializer = match &initializer.get_stmt() {
                // no call-statement in the initializer, so something like `a AT b` or `a : REFERENCE TO ... REF= b`
                AstStatement::ReferenceExpr(_) => {
                    initializer.get_flat_reference_name().and_then(|flat_ref| {
                        needs_qualifier(flat_ref).map_or_else(Option::Some, |q| {
                            q.then_some("self")
                                .map(|it| create_member_reference(it, self.ctxt.get_id_provider(), None))
                                .and_then(|base| {
                                    initializer.get_flat_reference_name().map(|it| {
                                        create_member_reference(it, self.ctxt.get_id_provider(), Some(base))
                                    })
                                })
                        })
                    })
                }
                // we found a call-statement, must be `a : REF_TO ... := REF(b) | ADR(b)`
                AstStatement::CallStatement(CallStatement { operator, parameters }) => parameters
                    .as_ref()
                    .and_then(|it| it.as_ref().get_flat_reference_name())
                    .and_then(|flat_ref| {
                        let op = operator.as_ref().get_flat_reference_name()?;
                        needs_qualifier(flat_ref).map_or_else(Option::Some, |q| {
                            q.then(|| {
                                create_call_statement(
                                    op,
                                    flat_ref,
                                    Some("self"),
                                    self.ctxt.id_provider.clone(),
                                    &initializer.location,
                                )
                            })
                        })
                    }),
                _ => return,
            };

            self.unresolved_initializers.insert_initializer(
                scope,
                Some(&variable.name),
                &updated_initializer.or(Some(initializer.clone())),
            );
        }
    }

    fn add_init_statements(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        let predicate = |var: &VariableIndexEntry| {
            var.is_temp()
                || var.is_var_external()
                || (matches!(implementation.pou_type, PouType::Function | PouType::Method { .. })
                    && var.is_local())
        };
        let strip_temporaries = |inits: &mut InitAssignments| {
            let mut temps = InitAssignments::default();
            let ids = inits
                .iter()
                .filter(|(id, _)| self.index.find_member(&implementation.name, id).is_some_and(predicate))
                .map(|(id, _)| id.to_owned())
                .collect::<Vec<_>>();

            for id in ids {
                if let Some(init) = inits.swap_remove(&id) {
                    temps.insert(id.to_owned(), init);
                };
            }

            temps
        };

        // remove all initializers for the current implementation/pou
        // XXX: this changes the order of init-statements in init-pous. but since assignments should still be ordered before call-statements,
        // this should be fine => edge cases?
        let Some(mut inits) = self.unresolved_initializers.swap_remove(&implementation.name) else {
            return;
        };

        // remove all non-stateful variable entries
        let temps = strip_temporaries(&mut inits);
        // re-enter the remaining initializers
        self.unresolved_initializers.insert(implementation.name.to_owned(), inits);

        // collect simple assignments
        let assignments = temps.into_iter().filter_map(|(lhs, init)| {
            init.as_ref().map(|it| {
                let lhs_ty = self
                    .index
                    .find_member(&implementation.name, &lhs)
                    .map(|it| {
                        self.index
                            .get_effective_type_or_void_by_name(it.get_type_name())
                            .get_type_information()
                    })
                    .unwrap();
                if lhs_ty.is_reference_to() || lhs_ty.is_alias() {
                    // XXX: ignore REF_TO for temp variables since they can be generated regularly
                    let rhs = if let AstStatement::CallStatement(CallStatement {
                        parameters: Some(parameter),
                        ..
                    }) = it.get_stmt()
                    {
                        parameter
                    } else {
                        it
                    };
                    // `REFERENCE TO` assignments in a POU body are automatically dereferenced and require the `REF=` operator to assign a pointer instead.
                    create_ref_assignment(&lhs, None, rhs, self.ctxt.get_id_provider())
                } else {
                    create_assignment(&lhs, None, it, self.ctxt.get_id_provider())
                }
            })
        });

        // collect necessary call statements to init-functions
        let delegated_calls = self
            .index
            .get_pou_members(&implementation.name)
            .iter()
            .filter(|var| predicate(var))
            .filter_map(|var| {
                let dti =
                    self.index.get_effective_type_or_void_by_name(var.get_type_name()).get_type_information();
                if dti.is_struct() {
                    Some(create_call_statement(
                        &get_init_fn_name(dti.get_name()),
                        var.get_name(),
                        None,
                        self.ctxt.get_id_provider(),
                        &implementation.name_location,
                    ))
                } else {
                    None
                }
            });

        let stmts = assignments
            .chain(delegated_calls)
            .chain(std::mem::take(&mut implementation.statements))
            .collect::<Vec<_>>();
        implementation.statements = stmts;
    }

    /// Updates the scope and initialized variable for struct types. Adds entries for each encountered struct
    /// (this includes POU-structs, i.e. programs, ...) to the initializer map if no entry is present
    fn update_struct_initializers(&mut self, user_type: &mut plc_ast::ast::UserTypeDeclaration) {
        let effective_type =
            user_type.data_type.get_name().and_then(|it| self.index.find_effective_type_by_name(it));
        if let DataType::StructType { .. } = &user_type.data_type {
            let Some(ty) = effective_type else {
                return user_type.walk(self);
            };
            let name = ty.get_name();

            let member_inits = self
                .index
                .get_container_members(name)
                .iter()
                .filter_map(|var| {
                    // struct member initializers don't have a qualifier/scope while evaluated in `const_evaluator.rs` and are registered as globals under their data-type name;
                    // look for member initializers for this struct in the global initializers, remove them and add new entries with the correct qualifier and left-hand-side
                    self.unresolved_initializers
                        .get_mut(GLOBAL_SCOPE)
                        .and_then(|it| it.swap_remove(var.get_type_name()))
                        .map(|node| (var.get_name(), node))
                })
                .collect::<Vec<_>>();

            for (lhs, init) in member_inits {
                // update struct member initializers
                self.unresolved_initializers.maybe_insert_initializer(name, Some(lhs), &init);
            }
            // add container to keys if not already present
            self.unresolved_initializers.maybe_insert_initializer(name, None, &user_type.initializer);
        }
    }

    fn maybe_add_global_instance_initializer(&mut self, variable: &plc_ast::ast::Variable) {
        let Some(global) = self.index.find_global_variable(variable.get_name()) else {
            return;
        };

        let info =
            self.index.get_effective_type_or_void_by_name(global.get_type_name()).get_type_information();

        if !info.is_struct()
            || self
                .index
                .find_pou(info.get_name())
                .is_some_and(|it| it.get_linkage() == &LinkageType::External)
        {
            return;
        }

        self.unresolved_initializers.maybe_insert_initializer(GLOBAL_SCOPE, Some(variable.get_name()), &None);
    }

    fn collect_var_config_assignments(&mut self, var_config: &[ConfigVariable]) {
        let assignments = var_config.iter().map(|var| {
            AstFactory::create_assignment(var.reference.clone(), var.address.clone(), self.ctxt.next_id())
        });
        self.var_config_initializers.extend(assignments);
    }
}

impl AstVisitorMut for InitVisitor {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        self.collect_var_config_assignments(&unit.var_config);
        unit.walk(self)
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        self.add_init_statements(implementation);
        implementation.walk(self);
    }

    fn visit_variable(&mut self, variable: &mut plc_ast::ast::Variable) {
        self.maybe_add_global_instance_initializer(variable);
        self.update_initializer(variable);
        variable.walk(self);
    }

    fn visit_user_type_declaration(&mut self, user_type: &mut plc_ast::ast::UserTypeDeclaration) {
        self.update_struct_initializers(user_type);
        user_type.walk(self);
    }

    fn visit_data_type(&mut self, data_type: &mut DataType) {
        if matches!(data_type, plc_ast::ast::DataType::StructType { .. }) {
            self.walk_with_scope(data_type, data_type.get_name().map(ToOwned::to_owned))
        } else {
            data_type.walk(self)
        }
    }

    fn visit_pou(&mut self, pou: &mut plc_ast::ast::Pou) {
        if !matches!(pou.linkage, LinkageType::External | LinkageType::BuiltIn) {
            self.unresolved_initializers.maybe_insert_initializer(&pou.name, None, &None);
        }

        self.walk_with_scope(pou, Some(&pou.name.to_owned()));
    }
}

#[derive(Clone, Default)]
struct Context {
    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU` body or `STRUCT.x` if `x` is a member of `STRUCT`)
    scope: Option<String>,

    pub id_provider: IdProvider,
}

// TODO: use &str with lifetimes, requires loads of changes to the visitor/walker traits
impl Context {
    fn new(id_provider: IdProvider) -> Self {
        Self { scope: None, id_provider }
    }

    /// updates the context's scope and returns the previous value
    fn scope(&mut self, pou: Option<String>) -> Option<String> {
        std::mem::replace(&mut self.scope, pou)
    }

    fn get_scope(&self) -> &Option<String> {
        &self.scope
    }

    fn get_id_provider(&self) -> IdProvider {
        self.id_provider.clone()
    }

    fn next_id(&mut self) -> usize {
        self.id_provider.next_id()
    }
}

fn create_member_reference_with_location(
    ident: &str,
    mut id_provider: IdProvider,
    base: Option<AstNode>,
    location: SourceLocation,
) -> AstNode {
    AstFactory::create_member_reference(
        AstFactory::create_identifier(ident, location, id_provider.next_id()),
        base,
        id_provider.next_id(),
    )
}

fn create_member_reference(ident: &str, id_provider: IdProvider, base: Option<AstNode>) -> AstNode {
    create_member_reference_with_location(ident, id_provider, base, SourceLocation::internal())
}

fn create_assignment_if_necessary(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &Option<AstNode>,
    mut id_provider: IdProvider,
) -> Option<AstNode> {
    let lhs = create_member_reference(
        lhs_ident,
        id_provider.clone(),
        base_ident.map(|id| create_member_reference(id, id_provider.clone(), None)),
    );
    rhs.as_ref().map(|node| AstFactory::create_assignment(lhs, node.to_owned(), id_provider.next_id()))
}

fn create_ref_assignment(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &AstNode,
    mut id_provider: IdProvider,
) -> AstNode {
    let lhs = create_member_reference(
        lhs_ident,
        id_provider.clone(),
        base_ident.map(|id| create_member_reference(id, id_provider.clone(), None)),
    );
    AstFactory::create_ref_assignment(lhs, rhs.to_owned(), id_provider.next_id())
}

fn create_assignment(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &AstNode,
    mut id_provider: IdProvider,
) -> AstNode {
    let lhs = create_member_reference(
        lhs_ident,
        id_provider.clone(),
        base_ident.map(|id| create_member_reference(id, id_provider.clone(), None)),
    );
    AstFactory::create_assignment(lhs, rhs.to_owned(), id_provider.next_id())
}

fn create_call_statement(
    operator: &str,
    member_id: &str,
    base_id: Option<&str>,
    mut id_provider: IdProvider,
    location: &SourceLocation,
) -> AstNode {
    let op = create_member_reference(operator, id_provider.clone(), None);
    let param = create_member_reference(
        member_id,
        id_provider.clone(),
        base_id.map(|it| create_member_reference(it, id_provider.clone(), None)),
    );
    AstFactory::create_call_statement(op, Some(param), id_provider.next_id(), location.clone())
}
