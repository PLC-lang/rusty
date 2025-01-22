use std::collections::HashMap;

use plc_ast::{
    ast::{
        AccessModifier, ArgumentProperty, AstFactory, CompilationUnit, Implementation, LinkageType, Pou,
        PouType, Property, PropertyKind, Variable, VariableBlock, VariableBlockType,
    },
    mut_visitor::AstVisitorMut,
    provider::IdProvider,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use rustc_hash::FxHashMap;

pub struct PropertyDesugar {
    pub parents: HashMap<String, Vec<String>>,
    pub id_provider: IdProvider,
    pub diagnostics: Vec<Diagnostic>,
}

impl PropertyDesugar {
    pub fn new(id_provider: IdProvider) -> PropertyDesugar {
        PropertyDesugar { parents: HashMap::new(), id_provider, diagnostics: Vec::new() }
    }

    pub fn validate_units(units: &Vec<CompilationUnit>) -> Vec<Diagnostic> {
        // TODO: Move this to a dedicated ParticipantValidator struct
        //       max 1 block (get/set) per prop
        // validate
        let mut diagnostics = Vec::new();

        let mut present: FxHashMap<String, Vec<SourceLocation>> = FxHashMap::default();

        for unit in units {
            let mut get_blocks = 0;
            let mut set_blocks = 0;
            for property in &unit.properties {
                match present.get_mut(&format!("{}.{}", &property.name_parent, &property.name)) {
                    Some(value) => {
                        value.push(property.name_location.clone());
                    }

                    None => {
                        present.insert(
                            format!("{}.{}", &property.name_parent, &property.name),
                            vec![property.name_location.clone()],
                        );
                    }
                }

                if !matches!(property.parent_kind, PouType::FunctionBlock | PouType::Program) {
                    diagnostics.push(
                        Diagnostic::new("Property only allowed in FunctionBlock or Program")
                            .with_location(property.name_location.clone())
                            .with_error_code("E001"), // TODO: Update me
                    );
                }

                if property.implementations.is_empty() {
                    diagnostics.push(
                        Diagnostic::new("Property has neither a GET nor a SET block")
                            .with_location(property.name_location.clone())
                            .with_error_code("E001"), // TODO: Update me
                    );
                }

                for implementation in &property.implementations {
                    match implementation {
                        PropertyKind::Get { variables, .. } => {
                            get_blocks += 1;
                            Self::validate_block_type(&mut diagnostics, variables, &property.name_location);
                        }

                        PropertyKind::Set { variables, .. } => {
                            set_blocks += 1;
                            Self::validate_block_type(&mut diagnostics, variables, &property.name_location);
                        }
                    };
                }

                if get_blocks > 1 {
                    diagnostics.push(
                        Diagnostic::new("Property has more than one GET block")
                            .with_location(property.name_location.clone())
                            .with_error_code("E001"), // TODO: Update me
                    );
                }

                if set_blocks > 1 {
                    diagnostics.push(
                        Diagnostic::new("Property has more than one SET block")
                            .with_location(property.name_location.clone())
                            .with_error_code("E001"), // TODO: Update me
                    );
                }
            }

            // TODO: Make this more efficient, too many clones
            for (name, locations) in present.clone().into_iter().filter(|(_, value)| value.len() > 1) {
                diagnostics.push(
                    Diagnostic::new(format!("Duplicate symbol `{name}`",))
                        .with_location(locations[0].clone())
                        .with_secondary_locations(locations.into_iter().skip(1).collect())
                        .with_error_code("E001"), // TODO: Update me
                );
            }
        }

        diagnostics
    }

    fn validate_block_type(
        diagnostics: &mut Vec<Diagnostic>,
        variables: &[VariableBlock],
        location: &SourceLocation,
    ) {
        if variables.is_empty() {
            // Nothing to validate
            return;
        }

        if variables.iter().any(|block| block.variable_block_type != VariableBlockType::Local) {
            diagnostics.push(
                Diagnostic::new("Invalid variable block type, only blocks of type VAR are allowed")
                    .with_location(location)
                    .with_error_code("E001"),
            );
        }
    }
}

impl AstVisitorMut for PropertyDesugar {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        for property in &mut unit.properties {
            match self.parents.get_mut(&property.name_parent) {
                Some(values) => values.push(property.name.clone()),
                None => {
                    self.parents.insert(property.name_parent.clone(), vec![property.name.clone()]);
                }
            };

            let Property { name, name_parent, name_location, return_type, ref mut implementations, .. } =
                property;
            for implementation in implementations {
                let (kind, variable, statements, return_type) = match implementation {
                    PropertyKind::Get { variables, ref mut statements } => {
                        statements.push(AstFactory::create_assignment(
                            AstFactory::create_member_reference(
                                AstFactory::create_identifier(
                                    &format!("get_{name}"),
                                    SourceLocation::undefined(),
                                    self.id_provider.next_id(),
                                ),
                                None,
                                self.id_provider.next_id(),
                            ),
                            AstFactory::create_member_reference(
                                AstFactory::create_identifier(
                                    name,
                                    SourceLocation::undefined(),
                                    self.id_provider.next_id(),
                                ),
                                None,
                                self.id_provider.next_id(),
                            ),
                            self.id_provider.next_id(),
                        ));
                        ("get", variables, statements, Some(return_type.clone()))
                    }
                    PropertyKind::Set { variables, ref mut statements } => {
                        variables.push(VariableBlock {
                            access: AccessModifier::Internal,
                            constant: false,
                            retain: false,
                            variables: vec![Variable {
                                name: "__in".to_string(),
                                data_type_declaration: return_type.clone(),
                                initializer: None,
                                address: None,
                                location: SourceLocation::undefined(),
                            }],
                            variable_block_type: VariableBlockType::Input(ArgumentProperty::ByVal),
                            linkage: LinkageType::Internal,
                            location: SourceLocation::undefined(),
                        });
                        statements.insert(
                            0,
                            AstFactory::create_assignment(
                                AstFactory::create_member_reference(
                                    AstFactory::create_identifier(
                                        &name.clone(),
                                        SourceLocation::undefined(),
                                        self.id_provider.next_id(),
                                    ),
                                    None,
                                    self.id_provider.next_id(),
                                ),
                                AstFactory::create_member_reference(
                                    AstFactory::create_identifier(
                                        "__in",
                                        SourceLocation::undefined(),
                                        self.id_provider.next_id(),
                                    ),
                                    None,
                                    self.id_provider.next_id(),
                                ),
                                self.id_provider.next_id(),
                            ),
                        );

                        ("set", variables, statements, None)
                    }
                };

                let pou = Pou {
                    name: format!("{name_parent}.{kind}_{name}"),
                    kind: PouType::Method { parent: name_parent.to_string() },
                    variable_blocks: variable.to_vec(),
                    return_type,
                    location: SourceLocation::undefined(),
                    name_location: name_location.clone(),
                    poly_mode: None,
                    generics: Vec::new(),
                    linkage: LinkageType::Internal,
                    super_class: None,
                    interfaces: Vec::new(),
                    is_const: false,
                };

                let implementation = Implementation {
                    name: format!("{name_parent}.{kind}_{name}"),
                    type_name: format!("{name_parent}.{kind}_{name}"),
                    linkage: LinkageType::Internal,
                    pou_type: PouType::Method { parent: name_parent.to_string() },
                    statements: statements.to_vec(),
                    location: SourceLocation::undefined(),
                    name_location: name_location.clone(),
                    overriding: false,
                    generic: false,
                    access: None,
                };

                unit.units.push(pou);
                unit.implementations.push(implementation);
            }

            if let Some(parent) = unit.units.iter_mut().find(|pou| &pou.name == name_parent) {
                parent.variable_blocks.push(VariableBlock {
                    access: AccessModifier::Internal,
                    constant: false,
                    retain: false,
                    variables: vec![Variable {
                        name: name.clone(),
                        data_type_declaration: return_type.clone(),
                        initializer: None,
                        address: None,
                        location: name_location.clone(),
                    }],
                    variable_block_type: VariableBlockType::Property,
                    linkage: LinkageType::Internal,
                    location: SourceLocation::internal(),
                });
            } else {
                panic!("must exist")
            }
        }
    }
}
