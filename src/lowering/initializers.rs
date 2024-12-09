use crate::{
    index::{const_expressions::UnresolvableKind, get_init_fn_name, FxIndexMap, FxIndexSet},
    lowering::{create_assignment_if_necessary, create_call_statement, create_member_reference},
    resolver::const_evaluator::UnresolvableConstant,
};
use plc_ast::ast::{
    AstFactory, AstNode, CompilationUnit, DataTypeDeclaration, Implementation, LinkageType, Pou, PouType,
    Variable, VariableBlock, VariableBlockType,
};
use plc_source::source_location::SourceLocation;

use super::AstLowerer;
pub(crate) const GLOBAL_SCOPE: &str = "__global";
const INIT_COMPILATION_UNIT: &str = "__initializers";
const VAR_CONFIG_INIT: &str = "__init___var_config";

/// POUs and datatypes which require initialization via generated function call.
/// The key corresponds to the scope in which the initializers were encountered.
/// The value corresponds to the assignment data, with the key being the assigned variable name
/// and value being the initializer `AstNode`.
pub(crate) type Initializers = FxIndexMap<String, InitAssignments>;
pub(crate) type InitAssignments = FxIndexMap<String, Option<AstNode>>;

pub(crate) trait Init<'lwr>
where
    Self: Sized + Default,
{
    fn new(candidates: &'lwr [UnresolvableConstant]) -> Self;
    /// Inserts an initializer only if no entry exists for the given variable
    fn maybe_insert_initializer(
        &mut self,
        container_name: &str,
        var_name: Option<&str>,
        initializer: &Option<AstNode>,
    );
    /// Inserts an initializer for the given variable. Will update existing values.
    fn insert_initializer(
        &mut self,
        container_name: &str,
        var_name: Option<&str>,
        initializer: &Option<AstNode>,
    );
}

impl<'lwr> Init<'lwr> for Initializers {
    fn new(candidates: &'lwr [UnresolvableConstant]) -> Self {
        let mut assignments = Self::default();
        candidates
            .iter()
            .filter_map(|it| {
                if let Some(UnresolvableKind::Address(init)) = &it.kind {
                    // assume all initializers without scope/not in a container are global variables for now. type-defs are separated later
                    Some((init.scope.clone().unwrap_or(GLOBAL_SCOPE.to_string()), init))
                } else {
                    None
                }
            })
            .for_each(|(scope, data)| {
                assignments.maybe_insert_initializer(
                    &scope,
                    data.lhs.as_ref().or(data.target_type_name.as_ref()).map(|it| it.as_str()),
                    &data.initializer,
                );
            });

        assignments
    }

    fn maybe_insert_initializer(
        &mut self,
        container_name: &str,
        var_name: Option<&str>,
        initializer: &Option<AstNode>,
    ) {
        let assignments = self.entry(container_name.to_string()).or_default();
        let Some(var_name) = var_name else {
            return;
        };

        // don't overwrite existing values
        if assignments.contains_key(var_name) {
            return;
        }

        assignments.insert(var_name.to_string(), initializer.clone());
    }

    fn insert_initializer(
        &mut self,
        container_name: &str,
        var_name: Option<&str>,
        initializer: &Option<AstNode>,
    ) {
        let assignments = self.entry(container_name.to_string()).or_default();
        let Some(var_name) = var_name else {
            return;
        };
        assignments.insert(var_name.to_string(), initializer.clone());
    }
}

impl AstLowerer {
    pub fn lower_init_functions(mut self, init_symbol_name: &str) -> Self {
        let units = create_init_units(&self);

        if let Some(init_unit) = units.into_iter().reduce(|mut acc_unit, unit| {
            acc_unit.import(unit);
            acc_unit
        }) {
            self.units.push(init_unit);
        }

        if let Some(global_init) = create_init_wrapper_function(&mut self, init_symbol_name) {
            self.units.push(global_init);
        }

        self
    }
}

fn create_var_config_init(statements: Vec<AstNode>) -> CompilationUnit {
    let loc = SourceLocation::internal_in_unit(Some(INIT_COMPILATION_UNIT));
    let pou = new_pou(VAR_CONFIG_INIT, vec![], PouType::Init, &loc); // this can probably just be internal
    let implementation = new_implementation(VAR_CONFIG_INIT, statements, PouType::Init, &loc);
    new_unit(pou, implementation, INIT_COMPILATION_UNIT)
}

fn create_init_units(lowerer: &AstLowerer) -> Vec<CompilationUnit> {
    let lookup = lowerer.unresolved_initializers.keys().map(|it| it.as_str()).collect::<FxIndexSet<_>>();
    lowerer
        .unresolved_initializers
        .iter()
        .filter_map(|(container, init)| {
            // globals will be initialized in the `__init` body
            if container == GLOBAL_SCOPE {
                return None;
            }

            create_init_unit(lowerer, container, init, &lookup)
        })
        .collect()
}

fn create_init_unit(
    lowerer: &AstLowerer,
    container_name: &str,
    assignments: &InitAssignments,
    all_init_units: &FxIndexSet<&str>,
) -> Option<CompilationUnit> {
    let id_provider = &lowerer.ctxt.id_provider;
    let init_fn_name = get_init_fn_name(container_name);
    let (is_stateless, location) = lowerer
        .index
        .find_pou(container_name)
        .map(|it| (it.is_function() || it.is_method(), it.get_location()))
        .unwrap_or_else(|| (false, &lowerer.index.get_type_or_panic(container_name).location));

    if is_stateless {
        // functions do not get their own init-functions -
        // initialization-statements will be added to the function body instead
        return None;
    };

    let (param, ident) = (
        vec![VariableBlock::default().with_block_type(VariableBlockType::InOut).with_variables(vec![
            Variable {
                name: "self".into(),
                data_type_declaration: DataTypeDeclaration::DataTypeReference {
                    referenced_type: container_name.to_string(),
                    location: location.clone(),
                },
                initializer: None,
                address: None,
                location: location.clone(),
            },
        ])],
        "self".to_string(),
    );

    let init_pou = new_pou(&init_fn_name, param, PouType::Init, location);

    let mut statements = assignments
        .iter()
        .filter_map(|(lhs_name, initializer)| {
            create_assignment_if_necessary(lhs_name, Some(&ident), initializer, id_provider.clone())
        })
        .collect::<Vec<_>>();

    let member_init_calls = lowerer
        .index
        .get_container_members(container_name)
        .iter()
        .filter_map(|member| {
            let member_type_name = member.get_type_name();
            let type_name = lowerer
                .index
                .get_effective_type_by_name(member_type_name)
                .map(|it| it.get_type_information().get_name())
                .unwrap_or(member_type_name);
            let call_name = get_init_fn_name(type_name);
            // TODO: support temp accessors && external declarations
            if !member.is_temp() && all_init_units.contains(type_name) {
                Some(create_call_statement(
                    &call_name,
                    member.get_name(),
                    Some("self"),
                    id_provider.clone(),
                    location,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    statements.extend(member_init_calls);
    let implementation = new_implementation(&init_fn_name, statements, PouType::Init, location);

    Some(new_unit(init_pou, implementation, INIT_COMPILATION_UNIT))
}

fn create_init_wrapper_function(lowerer: &mut AstLowerer, init_symbol_name: &str) -> Option<CompilationUnit> {
    let skip_var_config = lowerer.var_config_initializers.is_empty();
    if skip_var_config && lowerer.unresolved_initializers.is_empty() {
        return None;
    };

    let mut id_provider = lowerer.ctxt.id_provider.clone();
    let init_pou = new_pou(init_symbol_name, vec![], PouType::ProjectInit, &SourceLocation::internal());

    let global_instances = if let Some(global_instances) =
        lowerer.unresolved_initializers.get(GLOBAL_SCOPE).map(|it| {
            it.keys().filter_map(|var_name| {
                lowerer.index.find_variable(None, &[var_name]).and_then(|it| {
                    lowerer.index.find_effective_type_by_name(it.get_type_name()).and_then(|dt| {
                        let name = dt.get_type_information().get_name();
                        if dt.get_type_information().is_struct() {
                            Some((get_init_fn_name(name), var_name))
                        } else {
                            None
                        }
                    })
                })
            })
        }) {
        global_instances.collect::<Vec<_>>()
    } else {
        vec![]
    };

    let programs = lowerer.unresolved_initializers.iter().filter_map(|(scope, _)| {
        if lowerer.index.find_pou(scope).is_some_and(|pou| pou.is_program()) {
            Some((get_init_fn_name(scope), scope))
        } else {
            None
        }
    });

    let mut assignments = if let Some(stmts) = lowerer.unresolved_initializers.get(GLOBAL_SCOPE) {
        stmts
            .iter()
            .filter_map(|(var_name, initializer)| {
                create_assignment_if_necessary(var_name, None, initializer, id_provider.clone())
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };
    let calls = programs
        .chain(global_instances)
        .map(|(fn_name, param)| {
            let op = create_member_reference(&fn_name, id_provider.clone(), None);
            let param = create_member_reference(param, id_provider.clone(), None);
            AstFactory::create_call_statement(
                op,
                Some(param),
                id_provider.next_id(),
                SourceLocation::internal(),
            )
        })
        .collect::<Vec<_>>();

    assignments.extend(calls);

    if !skip_var_config {
        assignments.push(AstFactory::create_call_statement(
            create_member_reference(VAR_CONFIG_INIT, id_provider.clone(), None),
            None,
            id_provider.next_id(),
            SourceLocation::internal(),
        ));
    };

    let implementation =
        new_implementation(init_symbol_name, assignments, PouType::ProjectInit, &SourceLocation::internal());
    let mut global_init = new_unit(init_pou, implementation, init_symbol_name);

    if skip_var_config {
        return Some(global_init);
    };

    let var_config_init = create_var_config_init(std::mem::take(&mut lowerer.var_config_initializers));
    global_init.import(var_config_init);
    Some(global_init)
}

fn new_pou(
    name: &str,
    variable_blocks: Vec<VariableBlock>,
    pou_type: PouType,
    location: &SourceLocation,
) -> Pou {
    Pou {
        name: name.into(),
        variable_blocks,
        pou_type,
        return_type: None,
        location: location.clone(),
        name_location: location.to_owned(),
        poly_mode: None,
        generics: vec![],
        linkage: LinkageType::Internal,
        super_class: None,
        interfaces: vec![],
        is_const: false,
    }
}

fn new_implementation(
    name: &str,
    statements: Vec<AstNode>,
    pou_type: PouType,
    location: &SourceLocation,
) -> Implementation {
    Implementation {
        name: name.into(),
        type_name: name.into(),
        linkage: LinkageType::Internal,
        pou_type,
        statements,
        location: location.clone(),
        name_location: location.to_owned(),
        overriding: false,
        generic: false,
        access: None,
    }
}

fn new_unit(pou: Pou, implementation: Implementation, file_name: &str) -> CompilationUnit {
    CompilationUnit {
        global_vars: vec![],
        var_config: Default::default(),
        units: vec![pou],
        implementations: vec![implementation],
        interfaces: vec![],
        user_types: vec![],
        file_name: file_name.into(),
    }
}
