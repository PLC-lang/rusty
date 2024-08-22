use crate::{
    index::{const_expressions::UnresolvableKind, get_init_fn_name, visitor, FxIndexMap, FxIndexSet, Index},
    resolver::{const_evaluator::UnresolvableConstant, TypeAnnotator},
};
use plc_ast::{
    ast::{
        pre_process, AstFactory, AstNode, CompilationUnit, DataTypeDeclaration, Implementation, LinkageType,
        Pou, PouType, Variable, VariableBlock, VariableBlockType,
    },
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use super::AstLowerer;
pub(crate) const GLOBAL_SCOPE: &str = "__global";

/// POUs and datatypes which require initialization via generated function call.
/// The key corresponds to the scope in which the initializers were encountered.
/// The value corresponds to the assignment data, with the key being the assigned variable name
/// and value being the initializer `AstNode`.
pub(crate) type Initializers = FxIndexMap<String, InitAssignments>;
type InitAssignments = FxIndexMap<String, Option<AstNode>>;

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
    // fn import(&mut self, other: Self);
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
                    Some(data.lhs.as_ref().unwrap_or(&data.target_type_name)),
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

    // fn import(&mut self, other: Self) {
    //     other.into_iter().for_each(|(scope, data)| {
    //         self.entry(scope).or_default().extend(data);
    //     });
    // }
}

impl AstLowerer {
    pub fn lower_init_functions(mut self, init_symbol_name: &str) -> Self {
        let res = create_init_units(&self);

        if let Some((mut init_index, init_unit)) =
            res.into_iter().reduce(|(mut acc_index, mut acc_unit), (index, unit)| {
                acc_unit.import(unit);
                acc_index.import(index);
                (acc_index, acc_unit)
            })
        {
            self.index.import(std::mem::take(&mut init_index));
            let (annotation, dependencies, literals) =
                TypeAnnotator::visit_unit(&self.index, &init_unit, self.ctx.id_provider.clone());

            self.units.push((init_unit, dependencies, literals));
            self.annotation_map.import(annotation);

            self.index.import(std::mem::take(&mut self.annotation_map.new_index));
        }

        if let Some((mut new_index, init_unit)) = create_init_wrapper_function(&self, init_symbol_name) {
            self.index.import(std::mem::take(&mut new_index));
            let (a, deps, literals) =
                TypeAnnotator::visit_unit(&self.index, &init_unit, self.ctx.id_provider.clone());
            self.units.push((init_unit, deps, literals));
            self.annotation_map.import(a);
        }

        self.index.import(std::mem::take(&mut self.annotation_map.new_index));

        self
    }
}

fn create_init_units(lowerer: &AstLowerer) -> Vec<(Index, CompilationUnit)> {
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
) -> Option<(Index, CompilationUnit)> {
    let mut id_provider = lowerer.ctx.id_provider.clone();
    enum InitFnType {
        StatefulPou,
        Function,
        Struct,
    }

    let init_fn_name = get_init_fn_name(container_name);
    let (init_type, location) = lowerer
        .index
        .find_pou(container_name)
        .map(|it| {
            let ty = if it.is_function() { InitFnType::Function } else { InitFnType::StatefulPou };
            (ty, it.get_location())
        })
        .unwrap_or_else(|| (InitFnType::Struct, &lowerer.index.get_type_or_panic(container_name).location));

    if matches!(init_type, InitFnType::Function) {
        return None; // TODO: handle functions
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

    let init_pou = Pou {
        name: init_fn_name.clone(),
        variable_blocks: param,
        pou_type: PouType::Function,
        return_type: None,
        location: location.clone(),
        name_location: location.clone(),
        poly_mode: None,
        generics: vec![],
        linkage: LinkageType::Internal,
        super_class: None,
    };

    let mut statements = assignments
        .iter()
        .filter_map(|(lhs_name, initializer)| {
            let lhs = create_member_reference(
                lhs_name,
                id_provider.clone(),
                Some(create_member_reference(&ident, id_provider.clone(), None)),
            );
            initializer
                .as_ref()
                .map(|node| AstFactory::create_assignment(lhs, node.to_owned(), id_provider.next_id()))
        })
        .collect::<Vec<_>>();

    let member_init_calls = lowerer
        .index
        .get_container_members(container_name)
        .iter()
        .filter_map(|member| {
            let type_name = member.get_type_name();
            let call_name = get_init_fn_name(type_name);
            // TODO: support temp accessors && external declarations
            if !member.is_temp() && all_init_units.contains(type_name) {
                let op = create_member_reference(&call_name, id_provider.clone(), None);
                let param = create_member_reference(
                    member.get_name(),
                    id_provider.clone(),
                    Some(create_member_reference("self", id_provider.clone(), None)),
                );
                Some(AstFactory::create_call_statement(
                    op,
                    Some(param),
                    id_provider.next_id(),
                    location.clone(),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    statements.extend(member_init_calls);
    let implementation = Implementation {
        name: init_fn_name.clone(),
        type_name: init_fn_name.clone(),
        linkage: LinkageType::Internal,
        pou_type: PouType::Function,
        statements,
        location: location.clone(),
        name_location: location.clone(),
        overriding: false,
        generic: false,
        access: None,
    };

    let mut new_unit = CompilationUnit {
        global_vars: vec![],
        units: vec![init_pou],
        implementations: vec![implementation],
        user_types: vec![],
        file_name: "__initializers".into(),
    };

    pre_process(&mut new_unit, id_provider.clone());
    let mut new_index = visitor::visit(&new_unit);
    new_index.register_init_function(container_name);
    Some((new_index, new_unit))
}

fn create_init_wrapper_function(
    lowerer: &AstLowerer,
    init_symbol_name: &str,
) -> Option<(Index, CompilationUnit)> {
    if lowerer.unresolved_initializers.is_empty() {
        return None;
    }

    let mut id_provider = lowerer.ctx.id_provider.clone();
    let init_pou = Pou {
        name: init_symbol_name.into(),
        variable_blocks: vec![],
        pou_type: PouType::Function,
        return_type: None,
        location: SourceLocation::internal(),
        name_location: SourceLocation::internal(),
        poly_mode: None,
        generics: vec![],
        linkage: LinkageType::Internal,
        super_class: None,
    };

    let init_functions = lowerer
        .unresolved_initializers
        .iter()
        .filter_map(|(scope, _)| {
            if lowerer.index.find_pou(scope).is_some_and(|pou| pou.is_program()) {
                let init_fn_name = get_init_fn_name(scope);
                Some((init_fn_name, scope))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut globals = if let Some(stmts) = lowerer.unresolved_initializers.get(GLOBAL_SCOPE) {
        stmts
            .iter()
            .filter_map(|(var_name, initializer)| {
                initializer.as_ref().map(|it| {
                    let global = create_member_reference(var_name, id_provider.clone(), None);
                    AstFactory::create_assignment(global, it.clone(), id_provider.next_id())
                })
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };
    let body = init_functions
        .iter()
        .map(|(fn_name, param)| {
            let op = create_member_reference(fn_name, id_provider.clone(), None);
            let param = create_member_reference(param, id_provider.clone(), None);
            AstFactory::create_call_statement(
                op,
                Some(param),
                id_provider.next_id(),
                SourceLocation::internal(),
            )
        })
        .collect::<Vec<_>>();

    globals.extend(body);
    let implementation = Implementation {
        name: init_symbol_name.into(),
        type_name: init_symbol_name.into(),
        linkage: LinkageType::Internal,
        pou_type: PouType::Function,
        statements: globals,
        location: SourceLocation::internal(),
        name_location: SourceLocation::internal(),
        overriding: false,
        generic: false,
        access: None,
    };

    let mut init_unit = CompilationUnit {
        global_vars: vec![],
        units: vec![init_pou],
        implementations: vec![implementation],
        user_types: vec![],
        file_name: init_symbol_name.into(),
    };

    pre_process(&mut init_unit, id_provider.clone());
    let new_index = visitor::visit(&init_unit);

    Some((new_index, init_unit))
}

fn create_member_reference(name: &str, mut id_provider: IdProvider, base: Option<AstNode>) -> AstNode {
    AstFactory::create_member_reference(
        AstFactory::create_identifier(name, SourceLocation::internal(), id_provider.next_id()),
        base,
        id_provider.next_id(),
    )
}
