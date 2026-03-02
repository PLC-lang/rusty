// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use rustc_hash::{FxHashMap, FxHashSet};

use plc_util::convention::internal_type_name;

use crate::{
    ast::{
        flatten_expression_list, Assignment, AstFactory, AstNode, AstStatement, CompilationUnit,
        ConfigVariable, DataType, DataTypeDeclaration, Interface, LinkageType, Operator, Pou,
        UserTypeDeclaration, Variable, VariableBlock, VariableBlockType,
    },
    literals::AstLiteral,
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from,
};
use plc_source::source_location::SourceLocation;

/// Pre-processes a compilation unit, extracting inline type definitions,
/// initializing enum elements, and creating backing globals for hardware access.
pub fn pre_process(unit: &mut CompilationUnit, id_provider: IdProvider) {
    PreProcessor::new(id_provider).visit_compilation_unit(unit);
}

/// A mutable AST visitor that performs pre-processing transformations.
///
/// Follows the [`AstVisitorMut`] pattern (see also `InheritanceLowerer`, `PropertyLowerer`)
/// to walk the compilation unit and transform it in place:
///
/// - Extracts inline type definitions from variables into named [`UserTypeDeclaration`]s
/// - Generates types for generic POU parameters
/// - Creates backing global variables for IEC hardware access declarations (`AT %IX…`)
/// - Initializes enum elements with auto-incrementing values
struct PreProcessor {
    id_provider: IdProvider,
    /// Tracks which hardware-backing globals have been created to prevent duplicates.
    known_hw_globals: FxHashSet<String>,
    /// New types accumulated during the walk, flushed to `unit.user_types` afterward.
    new_types: Vec<UserTypeDeclaration>,
    /// Hardware-access backing globals accumulated during the walk.
    mangled_globals: Vec<Variable>,
    /// Name of the current containing scope (`"global"`, POU name, or struct name).
    current_container: String,
    /// Linkage of the current scope.
    current_linkage: LinkageType,
    /// Whether we are inside a POU (suppresses hardware-access processing for local variables).
    in_pou: bool,
}

impl PreProcessor {
    fn new(id_provider: IdProvider) -> Self {
        Self {
            id_provider,
            known_hw_globals: FxHashSet::default(),
            new_types: Vec::new(),
            mangled_globals: Vec::new(),
            current_container: "global".to_string(),
            current_linkage: LinkageType::Internal,
            in_pou: false,
        }
    }

    /// Registers a [`DataTypeDeclaration::Definition`] as a named user type in [`Self::new_types`].
    ///
    /// If `decl` is not a `Definition` (e.g. already a `Reference`), this is a no-op.
    fn register_extracted_type(&mut self, decl: DataTypeDeclaration, name: String, linkage: LinkageType) {
        if let DataTypeDeclaration::Definition { mut data_type, location, scope } = decl {
            data_type.set_name(name);
            self.new_types.push(UserTypeDeclaration {
                data_type: *data_type,
                initializer: None,
                location,
                scope,
                linkage,
            });
        }
    }

    /// Extracts the inner type of an array or pointer when it is an inline [`DataTypeDeclaration::Definition`],
    /// replacing it with a [`DataTypeDeclaration::Reference`] and pushing the extracted type to [`Self::new_types`].
    fn extract_nested_type(&mut self, name: &Option<String>, referenced_type: &mut Box<DataTypeDeclaration>) {
        if !should_generate_implicit(referenced_type) {
            return;
        }
        // Convention: nested extracted types are named `{container}_` (matches the old
        // `add_nested_datatypes` behaviour; see also plc_util/src/convention.rs).
        let container = name.as_deref().unwrap_or("undefined");
        let type_name = format!("{container}_");
        let type_ref = DataTypeDeclaration::Reference {
            referenced_type: type_name.clone(),
            location: SourceLocation::internal(),
        };
        let old = std::mem::replace(referenced_type.as_mut(), type_ref);
        self.register_extracted_type(old, type_name, self.current_linkage);
    }

    /// Initializes enum elements with auto-incrementing values.
    ///
    /// Elements without an explicit initializer receive `previous + 1` (or `0` for the first).
    fn initialize_enum_elements(&mut self, enum_name: &mut str, original_elements: &mut AstNode) {
        let mut last_name: Option<String> = None;

        fn extract_flat_ref_name(statement: &AstNode) -> &str {
            statement.get_flat_reference_name().expect("expected assignment")
        }

        let id_provider = &mut self.id_provider;

        let initialized_enum_elements = flatten_expression_list(original_elements)
            .iter()
            .map(|it| {
                try_from!(it, Assignment).map_or_else(
                    || (extract_flat_ref_name(it), None, it.get_location()),
                    |Assignment { left, right }| {
                        (extract_flat_ref_name(left.as_ref()), Some(*right.clone()), it.get_location())
                    },
                )
            })
            .map(|(element_name, initializer, location)| {
                let enum_literal = initializer.unwrap_or_else(|| {
                    build_enum_initializer(&last_name, &location, id_provider, enum_name)
                });
                last_name = Some(element_name.to_string());
                AstFactory::create_assignment(
                    AstFactory::create_member_reference(
                        AstFactory::create_identifier(element_name, &location, id_provider.next_id()),
                        None,
                        id_provider.next_id(),
                    ),
                    enum_literal,
                    id_provider.next_id(),
                )
            })
            .collect::<Vec<AstNode>>();

        if !initialized_enum_elements.is_empty() {
            let start_loc = initialized_enum_elements.first().expect("non empty vec").get_location();
            let end_loc = initialized_enum_elements.iter().last().expect("non empty vec").get_location();
            let expression = AstFactory::create_expression_list(
                initialized_enum_elements,
                start_loc.span(&end_loc),
                id_provider.next_id(),
            );
            *original_elements = expression;
        }
    }

    /// Creates backing globals for `VAR_CONFIG` hardware addresses.
    fn process_var_config(&mut self, unit: &CompilationUnit) {
        for ConfigVariable { data_type, address, .. } in &unit.var_config {
            let AstStatement::HardwareAccess(hardware) = &address.stmt else {
                unreachable!("Must be parsed as hardware access")
            };

            if hardware.is_template() {
                continue;
            }

            let name = hardware.get_mangled_variable_name();
            if !self.known_hw_globals.insert(name.clone()) {
                continue;
            }

            self.mangled_globals.push(Variable {
                name,
                data_type_declaration: data_type.get_inner_pointer_ty().unwrap_or(data_type.clone()),
                initializer: None,
                address: None,
                location: address.get_location(),
            });
        }
    }
}

impl AstVisitorMut for PreProcessor {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        // Seed known hardware globals from existing globals so re-runs don't produce duplicates.
        self.known_hw_globals = unit
            .global_vars
            .iter()
            .flat_map(|block| &block.variables)
            .filter(|var| {
                var.name.starts_with("__PI_")
                    || var.name.starts_with("__M_")
                    || var.name.starts_with("__G_")
            })
            .map(|var| var.name.clone())
            .collect();

        // Walk the compilation unit: globals → user_types → pous → implementations → interfaces
        unit.walk(self);

        // Process VAR_CONFIG variables (not part of the default walk)
        self.process_var_config(unit);

        // Fixup: process any newly generated types that themselves need processing
        // (e.g. an extracted struct whose members have inline types, or a nested array).
        // The loop converges because each iteration only generates simpler (flatter) types.
        while !self.new_types.is_empty() {
            let mut batch = std::mem::take(&mut self.new_types);
            for ut in &mut batch {
                self.visit_user_type_declaration(ut);
            }
            unit.user_types.extend(batch);
        }

        // Flush all accumulated hardware-access backing globals
        update_generated_globals(unit, std::mem::take(&mut self.mangled_globals));
    }

    fn visit_pou(&mut self, pou: &mut Pou) {
        let prev_container = std::mem::replace(&mut self.current_container, pou.name.clone());
        let prev_linkage = std::mem::replace(&mut self.current_linkage, pou.linkage);
        let prev_in_pou = std::mem::replace(&mut self.in_pou, true);

        // Generate types for generic parameters and replace generic names in variable types
        if !pou.generics.is_empty() {
            let mut generics = FxHashMap::default();
            for binding in &pou.generics {
                let new_name = format!("__{}__{}", pou.name, binding.name); // TODO: Naming convention (see plc_util/src/convention.rs)
                self.new_types.push(UserTypeDeclaration {
                    data_type: DataType::GenericType {
                        name: new_name.clone(),
                        generic_symbol: binding.name.clone(),
                        nature: binding.nature,
                    },
                    initializer: None,
                    scope: Some(pou.name.clone()),
                    location: pou.location.clone(),
                    linkage: pou.linkage,
                });
                generics.insert(binding.name.clone(), new_name);
            }
            for var in pou.variable_blocks.iter_mut().flat_map(|it| it.variables.iter_mut()) {
                replace_generic_type_name(&mut var.data_type_declaration, &generics);
            }
            if let Some(rt) = pou.return_type.as_mut() {
                replace_generic_type_name(rt, &generics);
            }
        }

        // Walk variable blocks → visit_variable handles implicit type extraction
        for block in &mut pou.variable_blocks {
            self.visit_variable_block(block);
        }

        // Extract implicit return type
        if let Some(return_type) = &pou.return_type {
            if should_generate_implicit(return_type) {
                let type_name = format!("__{}_return", &pou.name);
                let type_ref = DataTypeDeclaration::Reference {
                    referenced_type: type_name.clone(),
                    location: return_type.get_location(),
                };
                if let Some(old) = pou.return_type.replace(type_ref) {
                    self.register_extracted_type(old, type_name, pou.linkage);
                }
            }
        }

        self.current_container = prev_container;
        self.current_linkage = prev_linkage;
        self.in_pou = prev_in_pou;
    }

    fn visit_interface(&mut self, interface: &mut Interface) {
        for method in &mut interface.methods {
            self.visit_pou(method);
        }
    }

    fn visit_variable_block(&mut self, block: &mut VariableBlock) {
        // For non-POU blocks (globals), take linkage from the block itself
        if !self.in_pou {
            self.current_linkage = block.linkage;
        }
        block.walk(self);
    }

    fn visit_variable(&mut self, variable: &mut Variable) {
        // Capture inner pointer type before any replacement
        let ref_ty = variable.data_type_declaration.get_inner_pointer_ty();

        // Extract inline type definitions into named types
        if should_generate_implicit(&variable.data_type_declaration) {
            let new_type_name =
                internal_type_name(&format!("{}_", &self.current_container), &variable.name);
            let old = variable.replace_data_type_with_reference_to(new_type_name.clone());
            self.register_extracted_type(old, new_type_name, self.current_linkage);
        }

        // Create backing globals for hardware-access variables (globals and struct members only).
        // POU-local variables and template addresses (%I* : DWORD) are skipped.
        if self.in_pou {
            return;
        }
        let Some(ref node) = variable.address else { return };
        let AstStatement::HardwareAccess(hardware) = &node.stmt else { return };
        if hardware.is_template() {
            return;
        }

        let name = hardware.get_mangled_variable_name();
        variable.initializer = Some(AstFactory::create_member_reference(
            AstFactory::create_identifier(
                &name,
                SourceLocation::internal(),
                self.id_provider.next_id(),
            ),
            None,
            self.id_provider.next_id(),
        ));

        if self.known_hw_globals.insert(name.clone()) {
            self.mangled_globals.push(Variable {
                name,
                data_type_declaration: ref_ty.unwrap_or(variable.data_type_declaration.clone()),
                initializer: None,
                address: None,
                location: node.location.clone(),
            });
        }
    }

    fn visit_user_type_declaration(&mut self, user_type: &mut UserTypeDeclaration) {
        // Set context for struct member processing
        if let DataType::StructType { name, .. } = &user_type.data_type {
            self.current_container = name.as_deref().unwrap_or("undefined").to_string();
        }
        self.current_linkage = user_type.linkage;

        // Walk into data_type → visit_data_type dispatches per variant
        user_type.walk(self);
    }

    fn visit_data_type(&mut self, data_type: &mut DataType) {
        match data_type {
            // Extract nested inline types from arrays and pointers before walking children.
            DataType::ArrayType { name, referenced_type, .. }
            | DataType::PointerType { name, referenced_type, .. } => {
                self.extract_nested_type(name, referenced_type);
            }

            // Empty enum: normalize to empty expression list (no walk needed).
            DataType::EnumType { elements, .. }
                if matches!(elements.stmt, AstStatement::EmptyStatement { .. }) =>
            {
                elements.stmt = AstStatement::ExpressionList(vec![]);
                return;
            }

            // Named non-empty enum: generate auto-incrementing initializers (no walk needed).
            DataType::EnumType { elements, name: Some(enum_name), .. } => {
                self.initialize_enum_elements(enum_name, elements);
                return;
            }

            _ => {}
        }

        // Walk children: struct members → visit_variable, array bounds, etc.
        data_type.walk(self);
    }
}

// --- Standalone helpers ---

fn build_enum_initializer(
    last_name: &Option<String>,
    location: &SourceLocation,
    id_provider: &mut IdProvider,
    enum_name: &mut str,
) -> AstNode {
    if let Some(last_element) = last_name.as_ref() {
        // generate a `enum#last + 1` statement
        let enum_ref = AstFactory::create_identifier(last_element, location, id_provider.next_id());
        let type_element = AstFactory::create_member_reference(
            AstFactory::create_identifier(enum_name, location, id_provider.next_id()),
            None,
            id_provider.next_id(),
        );
        AstFactory::create_binary_expression(
            AstFactory::create_cast_statement(type_element, enum_ref, location, id_provider.next_id()),
            Operator::Plus,
            AstNode::new_literal(AstLiteral::new_integer(1), id_provider.next_id(), location.clone()),
            id_provider.next_id(),
        )
    } else {
        AstNode::new_literal(AstLiteral::new_integer(0), id_provider.next_id(), location.clone())
    }
}

fn should_generate_implicit(datatype: &DataTypeDeclaration) -> bool {
    match datatype {
        DataTypeDeclaration::Reference { .. } | DataTypeDeclaration::Aggregate { .. } => false,
        DataTypeDeclaration::Definition { data_type, .. }
            if matches!(data_type.as_ref(), DataType::VarArgs { .. }) =>
        {
            false
        }
        DataTypeDeclaration::Definition { .. } => true,
    }
}

fn replace_generic_type_name(dt: &mut DataTypeDeclaration, generics: &FxHashMap<String, String>) {
    match dt {
        DataTypeDeclaration::Definition { data_type, .. } => match data_type.as_mut() {
            DataType::ArrayType { referenced_type, .. }
            | DataType::PointerType { referenced_type, .. }
            | DataType::VarArgs { referenced_type: Some(referenced_type), .. } => {
                replace_generic_type_name(referenced_type.as_mut(), generics)
            }
            _ => {}
        },
        DataTypeDeclaration::Reference { referenced_type, .. } => {
            if let Some(type_name) = generics.get(referenced_type) {
                referenced_type.clone_from(type_name);
            }
        }
        DataTypeDeclaration::Aggregate { .. } => {}
    }
}

fn update_generated_globals(unit: &mut CompilationUnit, mangled_globals: Vec<Variable>) {
    if mangled_globals.is_empty() {
        return;
    }
    let mut block = if let Some(index) = unit
        .global_vars
        .iter()
        .position(|block| block.kind == VariableBlockType::Global && block.location.is_builtin_internal())
    {
        unit.global_vars.remove(index)
    } else {
        VariableBlock::default().with_block_type(VariableBlockType::Global)
    };
    for var in mangled_globals {
        if !block.variables.contains(&var) {
            block.variables.push(var);
        }
    }

    unit.global_vars.push(block);
}
