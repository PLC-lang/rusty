use crate::{
    index::{get_init_fn_name, Index, PouIndexEntry, VariableIndexEntry},
    resolver::const_evaluator::UnresolvableConstant,
};
use initializers::{get_user_init_fn_name, Init, InitAssignments, Initializers, GLOBAL_SCOPE};
use plc_ast::{
    ast::{
        Assignment, AstFactory, AstNode, AstStatement, CallStatement, CompilationUnit, ConfigVariable,
        DataType, LinkageType, PouType, ReferenceExpr,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;
use rustc_hash::FxHashMap;

pub mod calls;
mod initializers;
pub mod property;

pub struct InitVisitor {
    index: Index,
    unresolved_initializers: Initializers,
    var_config_initializers: Vec<AstNode>,
    user_inits: FxHashMap<String, bool>,
    ctxt: Context,
}

impl InitVisitor {
    pub fn visit(
        mut units: Vec<CompilationUnit>,
        index: Index,
        unresolvables: Vec<UnresolvableConstant>,
        id_provider: IdProvider,
        init_symbol_name: &'static str,
    ) -> Vec<CompilationUnit> {
        let mut visitor = Self::new(index, unresolvables, id_provider);
        // before visiting, we need to collect all candidates for user-defined init functions
        units.iter_mut().for_each(|unit| {
            visitor.collect_user_init_candidates(unit);
        });
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
            user_inits: FxHashMap::default(),
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

    fn collect_user_init_candidates(&mut self, unit: &mut CompilationUnit) {
        // collect all candidates for user-defined init functions
        for pou in unit.pous.iter().filter(|it| matches!(it.kind, PouType::FunctionBlock | PouType::Program))
        {
            // add the POU to potential `FB_INIT` candidates
            self.user_inits
                .insert(pou.name.to_owned(), self.index.find_method(&pou.name, "FB_INIT").is_some());
        }

        for user_type in
            unit.user_types.iter_mut().filter(|it| matches!(it.data_type, DataType::StructType { .. }))
        {
            // add the struct to potential `STRUCT_INIT` candidates
            if let Some(name) = user_type.data_type.get_name() {
                self.user_inits.insert(name.to_string(), false);
            };
        }
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

        // collect necessary call statements to init-functions and user-defined init-functions
        let mut implicit_calls = Vec::new();
        let mut user_init_calls = Vec::new();
        self.index.get_pou_members(&implementation.name).iter().filter(|var| predicate(var)).for_each(
            |var| {
                let dti =
                    self.index.get_effective_type_or_void_by_name(var.get_type_name()).get_type_information();
                let is_external = self
                    .index
                    .find_pou(dti.get_name())
                    .is_some_and(|it| it.get_linkage() == &LinkageType::External);
                if dti.is_struct() && !is_external {
                    implicit_calls.push(create_call_statement(
                        &get_init_fn_name(dti.get_name()),
                        var.get_name(),
                        None,
                        self.ctxt.get_id_provider(),
                        &implementation.name_location,
                    ));
                }
                if self.user_inits.contains_key(dti.get_name()) {
                    user_init_calls.push(create_call_statement(
                        &get_user_init_fn_name(dti.get_name()),
                        var.get_name(),
                        None,
                        self.ctxt.get_id_provider(),
                        &implementation.name_location,
                    ));
                }
            },
        );

        let stmts = assignments
            .chain(implicit_calls)
            .chain(user_init_calls)
            .chain(std::mem::take(&mut implementation.statements))
            .collect::<Vec<_>>();
        implementation.statements = stmts;
    }

    /// Updates the scope and initialized variable for struct types. Adds entries for each encountered struct
    /// (this includes POU-structs, i.e. programs, ...) to the initializer map if no entry is present
    fn update_struct_initializers(&mut self, user_type: &mut plc_ast::ast::UserTypeDeclaration) {
        let effective_type =
            user_type.data_type.get_name().and_then(|it| self.index.find_effective_type_by_name(it));
        if let DataType::StructType { ref mut variables, .. } = &mut user_type.data_type {
            let Some(ty) = effective_type else {
                return user_type.walk(self);
            };
            let name = ty.get_name();

            for variable in variables {
                self.unresolved_initializers.maybe_insert_initializer(
                    name,
                    Some(&variable.name),
                    &variable.initializer,
                );

                // XXX: Very duct-tapey but essentially we now have two initializers, one in the struct datatype
                // definition itself (`DataType::StructType { initializer: Some(<arena id>), ... }`) and one in the
                // `__init_*` function now. The former is unresolvable because it has the raw initializer, e.g.
                // `foo := (a := (b := (c := REF(...))))` whereas the latter is resolvable because it yields something
                // like `foo.a.b.c := REF(...)`. Thus we remove the initializer from the struct datatype definition
                // as the codegen would otherwise fail at generating them and result in a `Cannot generate values for..`
                // Literals and references are ignored however, since they are resolvable and / or constant.
                if variable.initializer.as_ref().is_some_and(|opt| !opt.is_literal() && !opt.is_reference()) {
                    variable.initializer = None;
                }
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

pub fn create_member_reference_with_location(
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

/// Takes some expression such as `bar := (baz := (qux := ADR(val)), baz2 := (qux := ADR(val)))` returning all final
/// assignment paths such as [`bar.baz.qux := ADR(val)`, `bar.baz2.qux := ADR(val)`].
fn create_assignment_paths(node: &AstNode, id_provider: IdProvider) -> Vec<Vec<AstNode>> {
    match node.get_stmt() {
        AstStatement::Assignment(Assignment { left, right }) => {
            let mut result = create_assignment_paths(right, id_provider.clone());
            for inner in result.iter_mut() {
                inner.insert(0, left.as_ref().clone());
            }
            result
        }
        AstStatement::ExpressionList(nodes) => {
            let mut result = vec![];
            for node in nodes {
                let inner = create_assignment_paths(node, id_provider.clone());
                result.extend(inner);
            }
            result
        }
        AstStatement::ParenExpression(node) => create_assignment_paths(node, id_provider),
        _ => vec![vec![node.clone()]],
    }
}

/// Takes some expression such as `foo : FooStruct := (bar := (baz := (qux := ADR(val)), baz2 := (qux := ADR(val))));`
/// and returns assignments of form [`foo.bar.baz.qux := ADR(val)`, `foo.bar.baz2.qux := ADR(val)`].
fn create_assignments_from_initializer(
    var_ident: &str,
    self_ident: Option<&str>,
    rhs: &Option<AstNode>,
    mut id_provider: IdProvider,
) -> Vec<AstNode> {
    let Some(initializer) = rhs else {
        return Vec::new();
    };

    let mut result = vec![];
    for mut path in create_assignment_paths(initializer, id_provider.clone()) {
        path.insert(0, create_member_reference(var_ident, id_provider.clone(), None));
        if self_ident.is_some() {
            path.insert(0, create_member_reference("self", id_provider.clone(), None));
        }

        let right = path.pop().expect("must have at least one node in the path");
        let mut left = path.pop().expect("must have at least one node in the path");

        for node in path.into_iter().rev() {
            insert_base_node(&mut left, node);
        }

        result.push(AstFactory::create_assignment(left, right, id_provider.next_id()));
    }

    result
}

/// Inserts a new base node into the member reference chain. For example a call such as `insert_base_node("b.c", a")`
/// will yield `a.b.c`.
fn insert_base_node(member: &mut AstNode, new_base: AstNode) {
    match &mut member.stmt {
        AstStatement::ReferenceExpr(ReferenceExpr { base, .. }) => match base {
            Some(inner) => insert_base_node(inner, new_base),
            None => {
                // We hit the end of the chain, simply replace the base (which must be None) with the new one
                base.replace(Box::new(new_base));
            }
        },

        _ => panic!("invalid function call, expected a member reference"),
    }
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

pub fn create_call_statement(
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
