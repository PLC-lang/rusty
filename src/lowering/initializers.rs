use crate::{
    index::{const_expressions::UnresolvableKind, get_init_fn_name, FxIndexMap, FxIndexSet},
    lowering::{create_call_statement, create_member_reference},
    resolver::const_evaluator::UnresolvableConstant,
};
use plc_ast::{
    ast::{
        AstFactory, AstId, AstNode, CompilationUnit, DataTypeDeclaration, Implementation, LinkageType, Pou,
        PouType, Variable, VariableBlock, VariableBlockType,
    },
    provider::IdProvider,
};
use plc_source::source_location::{FileMarker, SourceLocation};

use super::{create_assignments_from_initializer, InitVisitor};
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

impl InitVisitor {
    pub fn extend_ast(
        mut self,
        mut units: Vec<CompilationUnit>,
        init_symbol_name: &'static str,
    ) -> Vec<CompilationUnit> {
        let new_units = create_init_units(&self);

        if let Some(init_unit) = new_units.into_iter().reduce(|mut acc_unit, unit| {
            acc_unit.import(unit);
            acc_unit
        }) {
            units.push(init_unit);
        }

        if let Some(global_init) = create_init_wrapper_function(&mut self, init_symbol_name) {
            units.push(global_init);
        }

        units
    }
}

fn create_var_config_init(statements: Vec<AstNode>, mut id_provider: IdProvider) -> CompilationUnit {
    let loc = SourceLocation::internal_in_unit(Some(INIT_COMPILATION_UNIT));
    let pou = new_pou(VAR_CONFIG_INIT, id_provider.next_id(), vec![], PouType::Init, &loc); // this can probably just be internal
    let implementation = new_implementation(VAR_CONFIG_INIT, statements, PouType::Init, loc);
    new_unit(pou, implementation, INIT_COMPILATION_UNIT)
}

fn create_init_units(lowerer: &InitVisitor) -> Vec<CompilationUnit> {
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
        .chain(create_user_init_units(lowerer))
        .collect()
}

fn create_init_unit(
    lowerer: &InitVisitor,
    container_name: &str,
    assignments: &InitAssignments,
    all_init_units: &FxIndexSet<&str>,
) -> Option<CompilationUnit> {
    let mut id_provider = lowerer.ctxt.id_provider.clone();
    let init_fn_name = get_init_fn_name(container_name);
    log::trace!("creating {init_fn_name}");
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

    let location = location.clone().into_internal();

    let (self_param, self_ident) = (
        vec![VariableBlock::default().with_block_type(VariableBlockType::InOut).with_variables(vec![
            Variable {
                name: "self".into(),
                data_type_declaration: DataTypeDeclaration::Reference {
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

    let init_pou = new_pou(&init_fn_name, id_provider.next_id(), self_param, PouType::Init, &location);

    let mut statements = Vec::new();

    if let Some(initializer) = create_vtable_initializer(lowerer, &mut id_provider, container_name) {
        statements.push(initializer);
    }

    for (var_name, initializer) in assignments {
        if initializer.as_ref().is_some_and(|opt| !opt.is_literal_array()) {
            let initializers = create_assignments_from_initializer(
                var_name,
                Some(&self_ident),
                initializer,
                id_provider.clone(),
            );
            statements.extend(initializers);
        }
    }

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
                    &location,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let statements = [member_init_calls, statements].concat();
    let implementation = new_implementation(&init_fn_name, statements, PouType::Init, location);

    Some(new_unit(init_pou, implementation, INIT_COMPILATION_UNIT))
}

fn create_user_init_units(lowerer: &InitVisitor) -> Vec<CompilationUnit> {
    lowerer
        .user_inits
        .iter()
        .map(|(container_name, has_fb_init)| {
            let location = SourceLocation::internal_in_unit(Some(INIT_COMPILATION_UNIT));
            let mut id_provider = lowerer.ctxt.get_id_provider();
            let param = vec![VariableBlock::default()
                .with_block_type(VariableBlockType::InOut)
                .with_variables(vec![Variable {
                    name: "self".into(),
                    data_type_declaration: DataTypeDeclaration::Reference {
                        referenced_type: container_name.to_string(),
                        location: location.clone(),
                    },
                    initializer: None,
                    address: None,
                    location: location.clone(),
                }])];

            let fn_name = get_user_init_fn_name(container_name);
            let init_pou = new_pou(&fn_name, id_provider.next_id(), param, PouType::Init, &location);

            let mut statements = lowerer
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
                    let call_name = get_user_init_fn_name(type_name);
                    if !member.is_temp() && lowerer.user_inits.contains_key(type_name) {
                        Some(create_call_statement(
                            &call_name,
                            member.get_name(),
                            Some("self"),
                            id_provider.clone(),
                            &location,
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if *has_fb_init {
                let base = create_member_reference("self", id_provider.clone(), None);
                let op = create_member_reference("fb_init", id_provider.clone(), Some(base));
                let call_statement =
                    AstFactory::create_call_statement(op, None, id_provider.next_id(), location.clone());
                statements.push(call_statement);
            }
            let implementation = new_implementation(&fn_name, statements, PouType::Init, location);

            new_unit(init_pou, implementation, INIT_COMPILATION_UNIT)
        })
        .collect()
}

fn create_init_wrapper_function(
    lowerer: &mut InitVisitor,
    init_symbol_name: &'static str,
) -> Option<CompilationUnit> {
    let skip_var_config = lowerer.var_config_initializers.is_empty();
    if skip_var_config && lowerer.unresolved_initializers.is_empty() {
        return None;
    };

    let mut id_provider = lowerer.ctxt.id_provider.clone();
    let init_pou = new_pou(
        init_symbol_name,
        id_provider.next_id(),
        vec![],
        PouType::ProjectInit,
        &SourceLocation::internal(),
    );

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

    let mut statements = Vec::new();
    if let Some(statement) = lowerer.unresolved_initializers.get(GLOBAL_SCOPE) {
        for (var_name, initializer) in statement {
            if initializer.as_ref().is_some_and(|opt| !opt.is_literal_array()) {
                let res =
                    create_assignments_from_initializer(var_name, None, initializer, id_provider.clone());
                statements.extend(res);
            }
        }
    }

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

    if !skip_var_config {
        statements.push(AstFactory::create_call_statement(
            create_member_reference(VAR_CONFIG_INIT, id_provider.clone(), None),
            None,
            id_provider.next_id(),
            SourceLocation::internal(),
        ));
    };

    let user_init_calls = get_global_user_init_statements(lowerer);
    let statements = [calls, statements, user_init_calls].concat();
    let implementation =
        new_implementation(init_symbol_name, statements, PouType::ProjectInit, SourceLocation::internal());
    let mut global_init = new_unit(init_pou, implementation, init_symbol_name);

    if skip_var_config {
        return Some(global_init);
    };

    let var_config_init =
        create_var_config_init(std::mem::take(&mut lowerer.var_config_initializers), id_provider.clone());
    global_init.import(var_config_init);
    Some(global_init)
}

fn get_global_user_init_statements(lowerer: &InitVisitor) -> Vec<AstNode> {
    let global_instances = if let Some(global_instances) =
        lowerer.unresolved_initializers.get(GLOBAL_SCOPE).map(|it| {
            it.keys().filter_map(|var_name| {
                lowerer.index.find_variable(None, &[var_name]).and_then(|it| {
                    lowerer.index.find_effective_type_by_name(it.get_type_name()).and_then(|dt| {
                        let name = dt.get_type_information().get_name();
                        if lowerer.user_inits.contains_key(name) {
                            Some((get_user_init_fn_name(name), var_name))
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
            Some((get_user_init_fn_name(scope), scope))
        } else {
            None
        }
    });
    let mut id_provider = lowerer.ctxt.id_provider.clone();
    programs
        .chain(global_instances)
        .map(|(fn_name, param)| {
            let op = create_member_reference(&fn_name, lowerer.ctxt.id_provider.clone(), None);
            let param = create_member_reference(param, lowerer.ctxt.id_provider.clone(), None);
            AstFactory::create_call_statement(
                op,
                Some(param),
                id_provider.next_id(),
                SourceLocation::internal(),
            )
        })
        .collect::<Vec<_>>()
}

fn new_pou(
    name: &str,
    id: AstId,
    variable_blocks: Vec<VariableBlock>,
    kind: PouType,
    location: &SourceLocation,
) -> Pou {
    Pou {
        name: name.into(),
        id,
        variable_blocks,
        kind,
        return_type: None,
        location: location.clone(),
        name_location: location.to_owned(),
        poly_mode: None,
        generics: vec![],
        linkage: LinkageType::Internal,
        super_class: None,
        interfaces: vec![],
        properties: vec![],
        is_const: false,
    }
}

fn new_implementation(
    name: &str,
    statements: Vec<AstNode>,
    pou_type: PouType,
    location: SourceLocation,
) -> Implementation {
    Implementation {
        name: name.into(),
        type_name: name.into(),
        linkage: LinkageType::Internal,
        pou_type,
        statements,
        location: location.clone(),
        name_location: location.clone(),
        end_location: location,
        overriding: false,
        generic: false,
        access: None,
    }
}

fn new_unit(pou: Pou, implementation: Implementation, file_name: &'static str) -> CompilationUnit {
    CompilationUnit {
        global_vars: vec![],
        var_config: Default::default(),
        pous: vec![pou],
        implementations: vec![implementation],
        interfaces: vec![],
        user_types: vec![],
        file: FileMarker::Internal(file_name),
    }
}

pub(super) fn get_user_init_fn_name(type_name: &str) -> String {
    format!("__user_init_{}", type_name)
}

/// Creates an assignment of form `self.__vtable := ADR(__vtable_<pou_name>_instance)`. This is required to
/// initialize the virtual table of a class or function block. For more information refere to
/// [`crate::lowering::vtable`] and [`crate::lowering::polymorphism`].
fn create_vtable_initializer(lowerer: &InitVisitor, ids: &mut IdProvider, pou_name: &str) -> Option<AstNode> {
    let pou = lowerer.index.find_pou(pou_name)?;
    if !(pou.is_class() || pou.is_function_block()) {
        return None;
    }

    // self.__vtable
    let lhs = AstFactory::create_member_reference(
        AstFactory::create_identifier("__vtable", SourceLocation::internal(), ids.next_id()),
        Some(AstFactory::create_member_reference(
            AstFactory::create_identifier("self", SourceLocation::internal(), ids.next_id()),
            None,
            ids.next_id(),
        )),
        ids.next_id(),
    );

    // ADR(__vtable_<pou_name>_instance)
    let rhs = AstFactory::create_call_statement(
        AstFactory::create_member_reference(
            AstFactory::create_identifier("ADR", SourceLocation::internal(), ids.next_id()),
            None,
            ids.next_id(),
        ),
        Some(AstFactory::create_member_reference(
            AstFactory::create_identifier(
                format!("__vtable_{pou_name}_instance"),
                SourceLocation::internal(),
                ids.next_id(),
            ),
            None,
            ids.next_id(),
        )),
        ids.next_id(),
        SourceLocation::internal(),
    );

    // self.__vtable := ADR(__vtable_<pou_name>_instance)
    let assignment = AstFactory::create_assignment(lhs, rhs, ids.next_id());
    log::debug!("created virtual table initializer: {}", assignment.as_string());

    Some(assignment)
}

#[cfg(test)]
mod tests {
    use test_utils::parse_and_validate_buffered_ast;

    #[test]
    fn usertype_todo_better_name_00() {
        let src = r#"
        TYPE StructA:
            STRUCT
                value: DINT := 10;
            END_STRUCT
        END_TYPE
        "#;

        let units = parse_and_validate_buffered_ast(src);
        assert_eq!(units[1].implementations[0].name, "__init_structa");
        insta::assert_debug_snapshot!(units[1].implementations[0].statements, @r#"
        [
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "self",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 10,
                },
            },
        ]
        "#);
    }

    #[test]
    fn usertype_todo_better_name_01() {
        let src = r#"
        VAR_GLOBAL
            globalValue: DINT := 30;
        END_VAR

        TYPE StructA:
            STRUCT
                value: DINT := 10;
                instanceB: structB := (value := 20, valueRef := REF(globalValue));
            END_STRUCT
        END_TYPE

        TYPE StructB:
            STRUCT
                value: DINT;
                valueRef: REF_TO DINT;
            END_STRUCT
        END_TYPE
        "#;

        let units = parse_and_validate_buffered_ast(src);
        assert_eq!(units[1].implementations[0].name, "__init_structa");
        insta::assert_debug_snapshot!(units[1].implementations[0].statements, @r#"
        [
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__init_structb",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "instanceB",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "self",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceB",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "self",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 20,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "valueRef",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceB",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "self",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "REF",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "globalValue",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "self",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 10,
                },
            },
        ]
        "#);
    }

    #[test]
    fn usertype_todo_better_name_02() {
        let src = r#"
        VAR_GLOBAL
            globalValue: DINT := 40;
        END_VAR

        TYPE StructA:
            STRUCT
                value: DINT := 10;
                instanceB: StructB := (value := 20, instanceC := (value := 30, valueRef := REF(globalValue)));
            END_STRUCT
        END_TYPE

        TYPE StructB:
            STRUCT
                value: DINT;
                instanceC: StructC;
            END_STRUCT
        END_TYPE

        TYPE StructC:
            STRUCT
                value: DINT;
                valueRef: REF_TO DINT;
            END_STRUCT
        END_TYPE
        "#;

        let units = parse_and_validate_buffered_ast(src);
        assert_eq!(units[1].implementations[0].name, "__init_structa");
        insta::assert_debug_snapshot!(units[1].implementations[0].statements, @r#"
        [
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__init_structb",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "instanceB",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "self",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceB",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "self",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 20,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceC",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "instanceB",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "self",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 30,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "valueRef",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceC",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "instanceB",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "self",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                right: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "REF",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "globalValue",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "self",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 10,
                },
            },
        ]
        "#);
    }

    #[test]
    fn global_struct_with_simple_ref_initializer() {
        let src = r#"
        VAR_GLOBAL
            globalValue: DINT := 10;
            globalStructA: StructA := (value := REF(globalValue));
        END_VAR

        TYPE StructA:
            STRUCT
                value: REF_TO DINT;
            END_STRUCT
        END_TYPE
        "#;

        let units = parse_and_validate_buffered_ast(src);
        assert_eq!(units[2].implementations[0].name, "__init___TestProject");
        insta::assert_debug_snapshot!(units[2].implementations[0].statements, @r#"
        [
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__init_structa",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalStructA",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "globalStructA",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "REF",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "globalValue",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__user_init_StructA",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalStructA",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn global_struct_with_nested_struct_initializer() {
        let src = r#"
        VAR_GLOBAL
            globalValue: DINT := 30;
            globalStructA: StructA := (value := 10, instanceB := (value := 20, valueRef := REF(globalValue)));
        END_VAR

        TYPE StructA:
            STRUCT
                value: DINT;
                instanceB: structB;
            END_STRUCT
        END_TYPE

        TYPE StructB:
            STRUCT
                value: DINT;
                valueRef: REF_TO DINT;
            END_STRUCT
        END_TYPE
        "#;

        let units = parse_and_validate_buffered_ast(src);
        assert_eq!(units[2].implementations[0].name, "__init___TestProject");
        insta::assert_debug_snapshot!(units[2].implementations[0].statements, @r#"
        [
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__init_structa",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalStructA",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "globalStructA",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 10,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceB",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "globalStructA",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 20,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "valueRef",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceB",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "globalStructA",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "REF",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "globalValue",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__user_init_StructA",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalStructA",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn global_struct_with_deeply_nested_struct_initializer() {
        let src = r#"
        VAR_GLOBAL
            globalValue: DINT := 30;
            globalStructA: StructA := (value := 10, instanceB := (value := 20, instanceC := (value := 30, valueRef := REF(globalValue))));
        END_VAR

        TYPE StructA:
            STRUCT
                value: DINT;
                instanceB: StructB;
            END_STRUCT
        END_TYPE

        TYPE StructB:
            STRUCT
                value: DINT;
                instanceC: StructC;
            END_STRUCT
        END_TYPE

        TYPE StructC:
            STRUCT
                value: DINT;
                valueRef: REF_TO DINT;
            END_STRUCT
        END_TYPE
        "#;

        let units = parse_and_validate_buffered_ast(src);
        assert_eq!(units[2].implementations[0].name, "__init___TestProject");
        insta::assert_debug_snapshot!(units[2].implementations[0].statements, @r#"
        [
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__init_structa",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalStructA",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "globalStructA",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 10,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceB",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "globalStructA",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 20,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "value",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceC",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "instanceB",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "globalStructA",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 30,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "valueRef",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "instanceC",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "instanceB",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "globalStructA",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
                right: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "REF",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "globalValue",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__user_init_StructA",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalStructA",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    #[ignore = "Does not work yet, because `a := b := 1` is not flagged as `Unresolvable::Address`"]
    fn global_struct_with_integer_assignment_initializer() {
        let src = r#"
        VAR_GLOBAL
            globalA: MyStruct2 := (a := (b := 1));
        END_VAR

        TYPE MyStruct2:
            STRUCT
                a: MyStruct;
            END_STRUCT
        END_TYPE

        TYPE MyStruct:
            STRUCT
                b: DINT;
            END_STRUCT
        END_TYPE
        "#;

        let units = parse_and_validate_buffered_ast(src);
        assert_eq!(units[2].implementations[0].name, "__init___TestProject");
        insta::assert_debug_snapshot!(units[2].implementations[0].statements, @r###"
        [
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__init_mystruct2",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalA",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__init_mystruct2",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalB",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__user_init_MyStruct2",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalA",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__user_init_MyStruct2",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "globalB",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ]
        "###);
    }

    #[test]
    fn virtual_table_initialized() {
        let src = r#"
        FUNCTION_BLOCK A
            METHOD bar
                // printf('A::bar$N');
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK B EXTENDS A
            METHOD bar
                // printf('B::bar$N');
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK C EXTENDS B
            METHOD bar
                // printf('C::bar$N');
            END_METHOD
        END_FUNCTION_BLOCK
        "#;

        let units = parse_and_validate_buffered_ast(src);
        let unit_unit = &units
            .iter()
            .find(|unit| unit.file.get_name().is_some_and(|name| name == "__initializers"))
            .unwrap();

        let unit_init_a = unit_unit.implementations.iter().find(|it| it.name == "__init_a").unwrap();
        let stmts = unit_init_a.statements.iter().map(|statement| statement.as_string()).collect::<Vec<_>>();
        insta::assert_debug_snapshot!(stmts, @r#"
        [
            "self.__vtable := ADR(__vtable_A_instance)",
        ]
        "#);

        let unit_init_b = unit_unit.implementations.iter().find(|it| it.name == "__init_b").unwrap();
        let stmts = unit_init_b.statements.iter().map(|statement| statement.as_string()).collect::<Vec<_>>();
        insta::assert_debug_snapshot!(stmts, @r#"
        [
            "__init_a(self.__A)",
            "self.__A.__vtable := ADR(__vtable_B_instance)",
        ]
        "#);

        let unit_init_c = unit_unit.implementations.iter().find(|it| it.name == "__init_c").unwrap();
        let stmts = unit_init_c.statements.iter().map(|statement| statement.as_string()).collect::<Vec<_>>();
        insta::assert_debug_snapshot!(stmts, @r#"
        [
            "__init_b(self.__B)",
            "self.__B.__A.__vtable := ADR(__vtable_C_instance)",
        ]
        "#);
    }
}
