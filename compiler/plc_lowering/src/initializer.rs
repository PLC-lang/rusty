//! The initializer lowering module is responsible for adding initialization logic
//! to PLC AST nodes. This includes generating default values for variables, handling
//! constant expressions, and ensuring that all necessary initializations are present
//! before code generation. The module traverses the AST and modifies nodes as needed
//! to include initialization code, making sure that the resulting AST is ready for
//! further compilation stages.
//! Initialization logic is as follows:
//! - Every struct(and POU) has a constructor for fields with constant or pointer initializers.
//!    - The name for this constructor is `<StructName>__ctor`
//!    - The constructor is always public
//! - Variables of the struct are initialized by calling the constructor
//! - Global variables are initialized in a global constructor function called `__unit_<name>__ctor`
//!   - This function is called per module inside the static initialization code
//!   - The function is private to the module
//! - Stateless POUs (functions and methods) are initialized during their call.
//!     - Variables of a stateless POU of a struct type are initialized using the constructor call.
//! - External POUs and struct constructors are marked as `extern` and have no body, unless
//!   `--generate-external-constructors` is enabled for the build.
//! - Included units (`-i`) do not generate constructors by default.
//! - External variables are not re-initialized in the global constructor; they are assumed to be
//!   initialized externally.
//! - Built-in types and variables are not re-initialized in the global
//!   constructor.

use std::rc::Rc;

use plc::{
    index::{FxIndexMap, Index},
    lowering::helper::{
        create_assignment, create_assignment_with_index, create_assignments_from_initializer_with_index,
        create_call_statement, create_member_reference, create_ref_assignment,
        create_ref_assignment_with_index, extract_ref_call_argument, get_unit_name, new_constructor,
        new_unit_constructor,
    },
};
use plc_ast::{
    ast::{
        AstFactory, AstNode, AutoDerefType, CompilationUnit, DataType, DataTypeDeclaration, LinkageType,
        PouType, Variable, VariableBlockType,
    },
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocation;

#[derive(Debug, Clone)]
enum Body {
    Internal(Vec<AstNode>),
    External(Vec<AstNode>),
    None,
}

#[derive(Debug)]
pub struct Initializer {
    pub id_provider: IdProvider,
    index: Option<Rc<Index>>,
    /// Stateful constructor per POU/struct
    constructors: FxIndexMap<String, Body>,
    /// User defined constructor names per POU/datatype
    user_defined_constructors: FxIndexMap<String, String>,
    /// Constructors for temp and stack variables per POU
    stack_constructor: FxIndexMap<String, Body>,
    /// Global constructor statements
    global_constructor: Vec<AstNode>,
    /// Current context during AST traversal
    context: Context,
    /// Flag to mark external functions for generation
    generate_externals: bool,
}

#[derive(Default, Debug)]
struct Context {
    current_pou: Option<String>,
    current_datatype: Option<String>,
    current_variable_block: Option<VariableBlockType>,
}

impl Context {
    fn enter_pou(&mut self, pou: &str) {
        self.current_pou = Some(pou.to_string());
    }
    fn exit_pou(&mut self) {
        self.current_pou = None;
    }
    fn enter_datatype(&mut self, datatype: &str) {
        self.current_datatype = Some(datatype.to_string());
    }
    fn exit_datatype(&mut self) {
        self.current_datatype = None;
    }
    fn enter_variable_block(&mut self, block: &plc_ast::ast::VariableBlock) {
        self.current_variable_block = Some(block.kind);
    }
    fn exit_variable_block(&mut self) {
        self.current_variable_block = None;
    }
}

impl AstVisitor for Initializer {
    fn visit_pou(&mut self, pou: &plc_ast::ast::Pou) {
        self.context.enter_pou(pou.name.as_str());
        if pou.is_stateful() {
            self.user_defined_constructors.insert(pou.name.clone(), "FB_INIT".to_string());
        }
        // Skip generic POUs
        if pou.is_generic() {
            self.constructors.insert(pou.name.clone(), Body::None);
            self.context.exit_pou();
            return;
        }
        if pou.linkage == plc_ast::ast::LinkageType::BuiltIn {
            self.constructors.insert(pou.name.clone(), Body::None);
            self.context.exit_pou();
            return;
        }

        let constructor_body = self.constructor_body_for_linkage(pou.linkage);
        if pou.is_stateful() {
            self.constructors.insert(pou.name.clone(), constructor_body.clone());
        }
        self.stack_constructor.insert(pou.name.clone(), constructor_body);
        // If the POU has a base class, call the base constructor first
        if let Some(super_class) = &pou.super_class {
            let base_ctor_call = create_call_statement(
                &format!("{}__ctor", super_class.name),
                &format!("__{}", super_class.name),
                Some("self"),
                self.id_provider.clone(),
                &SourceLocation::internal(),
            );
            self.add_to_current_constructor(vec![base_ctor_call]);
        }
        pou.walk(self);
        // If the POU is a function block or a class, add an assignment to the vtable
        if pou.is_function_block() || pou.is_class() {
            let rhs = create_call_statement(
                "ADR",
                &format!("__vtable_{}_instance", pou.name),
                None,
                self.id_provider.clone(),
                &SourceLocation::internal(),
            );
            let vtable_assignment =
                create_assignment("__vtable", Some("self"), &rhs, self.id_provider.clone());
            self.add_to_current_constructor(vec![vtable_assignment]);
        }
        // call the user defined constructor here
        if let Some(user_defined_ctor_call) = self.get_user_defined_constructor_call(&pou.name, "self") {
            self.add_to_current_constructor(vec![user_defined_ctor_call]);
        }
        // If a program, add a constructor call to the global variables
        if pou.is_program() {
            let call = create_call_statement(
                &format!("{}__ctor", pou.name),
                pou.get_return_name(),
                None,
                self.id_provider.clone(),
                &SourceLocation::internal(),
            );
            self.global_constructor.push(call);
        }
        // If a stateless POU with a named return type, add constructor call for the return value
        if !pou.is_stateful() {
            if let Some(return_type) = &pou.return_type {
                if let Some(return_type_name) = return_type.get_name() {
                    if self.constructors.contains_key(return_type_name) {
                        let call = create_call_statement(
                            &format!("{}__ctor", return_type_name),
                            pou.get_return_name(),
                            None,
                            self.id_provider.clone(),
                            &SourceLocation::internal(),
                        );
                        self.add_to_current_stack_constructor(vec![call]);
                    }
                }
            }
        }
        self.context.exit_pou();
    }

    fn visit_variable_block(&mut self, block: &plc_ast::ast::VariableBlock) {
        // Skip constant blocks
        if block.constant {
            return;
        }
        self.context.enter_variable_block(block);
        block.walk(self);
        self.context.exit_variable_block();
    }

    fn visit_config_variable(&mut self, config_variable: &plc_ast::ast::ConfigVariable) {
        // VAR_CONFIG entries create assignments from the variable reference to its hardware address
        // For example: prog.instance1.foo := %IX1.2.1
        // These assignments are added to the global constructor
        let assignment = AstFactory::create_assignment(
            config_variable.reference.clone(),
            config_variable.address.clone(),
            self.id_provider.next_id(),
        );
        self.global_constructor.push(assignment);
    }

    fn visit_variable(&mut self, variable: &plc_ast::ast::Variable) {
        // grab the index
        let index = self.index.as_ref().expect("index is set at this stage");
        let variable_block_type =
            self.context.current_variable_block.expect("variable block is set at this stage");
        // Find if the parent is stateful
        let is_stateful = if let Some(pou_name) = &self.context.current_pou {
            index.find_pou(pou_name).is_some_and(|it| it.is_stateful())
        } else {
            true
        };
        let mut stmts = vec![];
        let base = Self::get_base_ident(&variable_block_type, is_stateful);
        if !variable_block_type.is_inout() {
            if let Some(constructor) = variable
                .data_type_declaration
                .get_referenced_type()
                .and_then(|it| self.get_constructor_call(it, base, variable.get_name()))
            {
                stmts.push(constructor);
            }
        }
        // Determine if we need to create an initializer
        // For alias/reference variables (AT x), if there's no explicit initializer, we use the AT target (address)
        // BUT only if the address is a simple identifier (not a hardware address like %I* or %QX1.2.1)
        let initializer_to_use = if variable.initializer.is_some() {
            variable.initializer.as_ref()
        } else if is_alias_or_reference_variable(variable, index)
            && variable.address.as_ref().is_some_and(is_simple_identifier_address)
        {
            // For alias/reference variables with simple identifier AT targets, use the address as implicit initializer
            variable.address.as_ref()
        } else {
            None
        };

        if let Some(initializer) = initializer_to_use {
            // Qualify the initializer with the POU name if needed
            let initializer = if let Some(initializer_name) = initializer.get_flat_reference_name() {
                if let Some(pou_name) = self.context.current_pou.as_ref() {
                    if self.index.as_ref().unwrap().find_member(pou_name, initializer_name).is_some() {
                        let base_ref =
                            base.map(|it| create_member_reference(it, self.id_provider.clone(), None));
                        create_member_reference(initializer_name, self.id_provider.clone(), base_ref)
                    } else {
                        initializer.to_owned()
                    }
                } else {
                    initializer.to_owned()
                }
            } else {
                initializer.to_owned()
            };
            // For alias variables (AT syntax) and REFERENCE TO variables, we need REF= assignment
            // to properly set up the reference/alias relationship
            let init_policy = InitLoweringPolicy::for_variable(variable, &initializer, index);
            match init_policy {
                InitLoweringPolicy::RefAssign(ref_rhs) => {
                    let assignment = create_ref_assignment_with_index(
                        variable.get_name(),
                        base,
                        &ref_rhs,
                        self.id_provider.clone(),
                        self.index.as_ref().map(|idx| idx.as_ref()),
                        self.context.current_pou.as_deref(),
                    );
                    stmts.push(assignment);
                }
                InitLoweringPolicy::StructDecompose => {
                    let decomposed = create_assignments_from_initializer_with_index(
                        variable.get_name(),
                        base,
                        &Some(initializer.clone()),
                        self.id_provider.clone(),
                        self.index.as_ref().map(|idx| idx.as_ref()),
                        self.context.current_pou.as_deref(),
                    );
                    stmts.extend(decomposed);
                }
                InitLoweringPolicy::DirectAssign => {
                    let assignment = create_assignment_with_index(
                        variable.get_name(),
                        base,
                        &initializer,
                        self.id_provider.clone(),
                        self.index.as_ref().map(|idx| idx.as_ref()),
                        self.context.current_pou.as_deref(),
                    );
                    stmts.push(assignment);
                }
            }
        }
        if variable_block_type.is_temp()
            || (variable_block_type.is_local() && !is_stateful)
            || variable_block_type.is_inout()
        {
            self.add_to_current_stack_constructor(stmts);
        } else {
            self.add_to_current_constructor(stmts);
        }
    }

    fn visit_user_type_declaration(&mut self, user_type: &plc_ast::ast::UserTypeDeclaration) {
        let name = user_type.data_type.get_name().expect("name is set at this stage");
        self.context.enter_datatype(name);
        if user_type.data_type.is_generic() {
            self.constructors.insert(name.to_string(), Body::None);
            self.context.exit_datatype();
            return;
        }
        // Skip creating constructors for VLA types - they're parameter types that don't need initialization
        let index = self.index.as_ref().expect("index is set at this stage");
        if index.get_type_information_or_void(name).is_vla() {
            self.constructors.insert(name.to_string(), Body::None);
            self.context.exit_datatype();
            return;
        }
        if user_type.linkage == plc_ast::ast::LinkageType::BuiltIn {
            self.constructors.insert(name.to_string(), Body::None);
            self.context.exit_datatype();
            return;
        }

        let constructor_body = self.constructor_body_for_linkage(user_type.linkage);
        self.constructors.insert(name.to_string(), constructor_body);

        // For alias types (typedefs), call the parent type's constructor first
        // This handles typedef chains like: mysubtype2 -> mysubtype -> mytypedefstruct
        if let DataType::SubRangeType { referenced_type, bounds: None, .. } = &user_type.data_type {
            // This is a typedef (alias) without bounds - call the parent constructor
            if self.constructors.contains_key(referenced_type) {
                let parent_ctor_call = create_call_statement(
                    &format!("{}__ctor", referenced_type),
                    "self",
                    None,
                    self.id_provider.clone(),
                    &SourceLocation::internal(),
                );
                self.add_to_current_constructor(vec![parent_ctor_call]);
            }
        }

        self.visit_data_type(&user_type.data_type);
        let mut stmts = vec![];
        // Explicitly add the initializer call here
        if let Some(initializer) = &user_type.initializer {
            let assignment = create_assignment("self", None, initializer, self.id_provider.clone());
            stmts.push(assignment);
        }
        self.add_to_current_constructor(stmts);
        self.context.exit_datatype();
    }

    fn visit_data_type(&mut self, data_type: &plc_ast::ast::DataType) {
        // Only structs get constructors
        if let plc_ast::ast::DataType::StructType { variables, .. } = data_type {
            let index = self.index.as_ref().expect("index is set at this stage");
            let mut constructor = vec![];
            for variable in variables.iter() {
                // First, check if the field's type has a constructor and generate a call if needed
                if let Some(type_name) = variable.data_type_declaration.get_referenced_type() {
                    if self.constructors.contains_key(type_name) {
                        let call = create_call_statement(
                            &format!("{}__ctor", type_name),
                            variable.get_name(),
                            Some("self"),
                            self.id_provider.clone(),
                            &SourceLocation::internal(),
                        );
                        constructor.push(call);
                    }
                }

                if let Some(initializer) = &variable.initializer {
                    let init_policy = InitLoweringPolicy::for_struct_field(variable, initializer, index);
                    match init_policy {
                        InitLoweringPolicy::RefAssign(ref_rhs) => {
                            let assignment = create_ref_assignment(
                                variable.get_name(),
                                Some("self"),
                                &ref_rhs,
                                self.id_provider.clone(),
                            );
                            constructor.push(assignment);
                        }
                        InitLoweringPolicy::StructDecompose => {
                            let decomposed = create_assignments_from_initializer_with_index(
                                variable.get_name(),
                                Some("self"),
                                &Some(initializer.clone()),
                                self.id_provider.clone(),
                                self.index.as_ref().map(|idx| idx.as_ref()),
                                self.context.current_pou.as_deref(),
                            );
                            constructor.extend(decomposed);
                        }
                        InitLoweringPolicy::DirectAssign => {
                            let assignment = create_assignment(
                                variable.get_name(),
                                Some("self"),
                                initializer,
                                self.id_provider.clone(),
                            );
                            constructor.push(assignment);
                        }
                    }
                }
            }
            self.add_to_current_constructor(constructor);
        }
    }
}

/// Policy describing how an initializer is lowered into constructor assignments.
///
/// - RefAssign: emit a `REF=` assignment for alias/reference variables or reference-typed fields.
/// - StructDecompose: decompose a struct literal into per-field assignments.
/// - DirectAssign: emit a standard `:=` assignment without decomposition.
#[derive(Debug, Clone)]
enum InitLoweringPolicy {
    RefAssign(Box<AstNode>),
    StructDecompose,
    DirectAssign,
}

impl InitLoweringPolicy {
    /// Determines the lowering policy for a variable initializer.
    ///
    /// Resolution order:
    /// 1) Alias/reference variables always use `REF=`. For REFERENCE TO, unwrap `REF(...)` if present.
    /// 2) Struct literals on struct types are decomposed into per-field assignments.
    /// 3) Everything else becomes a direct `:=` assignment.
    fn for_variable(variable: &Variable, initializer: &AstNode, index: &Index) -> Self {
        if is_alias_or_reference_variable(variable, index) {
            let is_reference_to = variable
                .data_type_declaration
                .get_referenced_type()
                .and_then(|tn| index.find_effective_type_info(tn))
                .is_some_and(|ti| ti.is_reference_to());

            let ref_rhs = if is_reference_to {
                extract_ref_call_argument(initializer).unwrap_or(initializer).clone()
            } else {
                initializer.clone()
            };

            return InitLoweringPolicy::RefAssign(Box::new(ref_rhs));
        }

        if is_struct_type(variable, index) && initializer.is_struct_literal_initializer() {
            InitLoweringPolicy::StructDecompose
        } else {
            InitLoweringPolicy::DirectAssign
        }
    }

    /// Determines the lowering policy for struct-field initializers inside constructors.
    ///
    /// Resolution order:
    /// 1) Struct literals on struct-typed fields are decomposed.
    /// 2) Reference-typed fields use `REF=` (unwrapping `REF(...)` if present).
    /// 3) Otherwise use direct `:=` assignment.
    fn for_struct_field(variable: &Variable, initializer: &AstNode, index: &Index) -> Self {
        if is_struct_type(variable, index) && initializer.is_struct_literal_initializer() {
            InitLoweringPolicy::StructDecompose
        } else if is_reference_type(variable, index) {
            let ref_rhs = extract_ref_call_argument(initializer).unwrap_or(initializer).clone();
            InitLoweringPolicy::RefAssign(Box::new(ref_rhs))
        } else {
            InitLoweringPolicy::DirectAssign
        }
    }
}

fn is_struct_type(variable: &Variable, index: &Index) -> bool {
    variable
        .data_type_declaration
        .get_referenced_type()
        .and_then(|tn| index.find_effective_type_info(tn))
        .is_some_and(|ti| ti.is_struct())
}

fn is_reference_type(variable: &Variable, index: &Index) -> bool {
    variable
        .data_type_declaration
        .get_referenced_type()
        .and_then(|tn| index.find_effective_type_info(tn))
        .is_some_and(|ti| ti.is_reference_to())
}

/// Checks if a variable is an alias or reference variable that needs ADR() wrapping
/// (declared with AT syntax, e.g., `px AT x : DINT`, or `REFERENCE TO ... REF= ...`)
fn is_alias_or_reference_variable(variable: &Variable, index: &Index) -> bool {
    // First try to check the inline definition (before pre-processing)
    if let DataTypeDeclaration::Definition { data_type, .. } = &variable.data_type_declaration {
        if let DataType::PointerType { auto_deref: Some(auto_deref), .. } = data_type.as_ref() {
            return matches!(auto_deref, AutoDerefType::Alias | AutoDerefType::Reference);
        }
    }

    // If the type was pre-processed to a reference, look it up in the index
    if let Some(type_name) = variable.data_type_declaration.get_referenced_type() {
        if let Some(type_info) = index.find_effective_type_info(type_name) {
            return type_info.is_alias() || type_info.is_reference_to();
        }
    }

    false
}

/// Checks if an address node is a simple identifier (not a hardware address like %I* or %QX1.2.1)
fn is_simple_identifier_address(address: &AstNode) -> bool {
    matches!(address.get_stmt(), plc_ast::ast::AstStatement::Identifier(_))
}

impl Initializer {
    pub fn new(id_provider: IdProvider, generate_externals: bool) -> Initializer {
        Initializer {
            id_provider,
            index: None,
            constructors: Default::default(),
            user_defined_constructors: Default::default(),
            stack_constructor: Default::default(),
            global_constructor: Default::default(),
            context: Default::default(),
            generate_externals,
        }
    }

    /// Visits the given unit and collects all initialization logic, then applies it to the unit
    /// Adds a new constructor for each struct/POU with initialization logic
    /// Adds constructor calls for stack variables to each function
    /// Adds a global constructor function with global variable initializations
    pub fn apply_initialization(mut self, mut unit: CompilationUnit, index: Rc<Index>) -> CompilationUnit {
        // Set the index
        self.index = Some(index.clone());
        // Pre-pass: register all constructors for all types before visiting variables
        // This ensures that when we encounter a member variable of a struct type during
        // traversal, we can generate a constructor call for it.
        self.pre_register_all_constructors(&unit);
        // Now do the main traversal to generate initialization logic
        self.visit_compilation_unit(&unit);
        self.index = None;
        // Add each constructor function to the unit as a new function
        for (name, body) in self.constructors {
            match body {
                Body::Internal(nodes) => {
                    let (pou, implementation) = new_constructor(
                        &name,
                        LinkageType::Internal,
                        PouType::Init,
                        nodes,
                        self.id_provider.clone(),
                    );
                    unit.pous.push(pou);
                    unit.implementations.push(implementation);
                }
                Body::External(nodes) => {
                    let (pou, implementation) = new_constructor(
                        &name,
                        LinkageType::External,
                        PouType::Init,
                        nodes,
                        self.id_provider.clone(),
                    );
                    unit.pous.push(pou);
                    unit.implementations.push(implementation);
                }
                Body::None => {}
            }
        }
        // Add the construction calls for stack variables to each function
        for (name, elements) in self.stack_constructor.into_iter() {
            if let Some(implementation) = unit.implementations.iter_mut().find(|it| it.name.as_str() == name)
            {
                match elements {
                    Body::Internal(nodes) => {
                        implementation.statements.splice(0..0, nodes);
                    }
                    Body::External(nodes) => {
                        implementation.statements.splice(0..0, nodes);
                    }
                    Body::None => {}
                }
            }
        }
        let should_generate_constructor = match unit.linkage {
            LinkageType::Internal => true,
            LinkageType::External if self.generate_externals => true,
            _ => false,
        };
        // Add a global constructor function with the global constructor calls
        if !self.global_constructor.is_empty() && should_generate_constructor {
            let unit_name = get_unit_name(&unit);
            let (pou, implementation) =
                new_unit_constructor(&unit_name, self.global_constructor, self.id_provider.clone());
            unit.pous.push(pou);
            unit.implementations.push(implementation);
        }
        unit
    }

    fn constructor_body_for_linkage(&self, linkage: LinkageType) -> Body {
        match linkage {
            LinkageType::Internal => Body::Internal(vec![]),
            LinkageType::External if self.generate_externals => Body::Internal(vec![]),
            LinkageType::External | LinkageType::Include => Body::External(vec![]),
            LinkageType::BuiltIn => Body::None,
        }
    }

    /// Pre-register all constructors for all POUs and struct types in the compilation unit.
    /// This ensures that when we visit member variables of a struct type, the constructor
    /// for that type has already been registered, so we can generate a constructor call.
    fn pre_register_all_constructors(&mut self, unit: &CompilationUnit) {
        // Register all POUs
        for pou in &unit.pous {
            self.pre_register_pou_constructor(pou);
        }
        // Register all user-defined types (structs)
        for user_type in &unit.user_types {
            if let Some(name) = user_type.data_type.get_name() {
                self.constructors.entry(name.to_string()).or_insert(Body::None);
            }
        }
    }

    /// Register a single POU's constructor in the constructors map
    fn pre_register_pou_constructor(&mut self, pou: &plc_ast::ast::Pou) {
        // Skip generic POUs
        if pou.is_generic() {
            self.constructors.insert(pou.name.clone(), Body::None);
            return;
        }

        // Skip built-in types
        if pou.linkage == LinkageType::BuiltIn {
            self.constructors.insert(pou.name.clone(), Body::None);
            return;
        }

        // Register the constructor based on linkage and whether it's stateful
        if pou.is_stateful() {
            self.constructors.entry(pou.name.clone()).or_insert(Body::None);
        }
    }

    fn add_to_current_stack_constructor(&mut self, node: Vec<AstNode>) {
        if node.is_empty() {
            return;
        }
        if let Some(current_pou) = self.context.current_pou.as_ref() {
            if let Some(Body::Internal(nodes) | Body::External(nodes)) =
                self.stack_constructor.get_mut(current_pou)
            {
                nodes.extend(node);
            } else {
                log::debug!(
                    "Dropping {} init statement(s) for `{current_pou}`: no stack constructor entry",
                    node.len()
                );
            }
        } else {
            log::debug!("Dropping {} init statement(s): no current POU in context", node.len());
        }
    }

    fn add_to_current_constructor(&mut self, node: Vec<AstNode>) {
        if node.is_empty() {
            return;
        }
        if let Some(current_struct) =
            self.context.current_pou.as_ref().or(self.context.current_datatype.as_ref())
        {
            if let Some(Body::Internal(nodes) | Body::External(nodes)) =
                self.constructors.get_mut(current_struct)
            {
                nodes.extend(node)
            } else {
                log::debug!(
                    "Dropping {} init statement(s) for `{current_struct}`: no constructor entry",
                    node.len()
                );
            }
        } else if self.context.current_variable_block.is_some_and(|it| it.is_global()) {
            // Global constructor
            self.global_constructor.extend(node);
        } else {
            log::debug!("Dropping {} init statement(s): no current POU/datatype in context", node.len());
        }
    }

    fn get_constructor_call(&self, type_name: &str, base: Option<&str>, var_name: &str) -> Option<AstNode> {
        let should_create_call = if self.constructors.contains_key(type_name) {
            true
        } else {
            self.index.as_ref().and_then(|idx| idx.find_pou(type_name)).is_some_and(|pou| pou.is_stateful())
        };

        if should_create_call {
            let call = create_call_statement(
                &format!("{}__ctor", type_name),
                var_name,
                base,
                self.id_provider.clone(),
                &SourceLocation::internal(),
            );
            Some(call)
        } else {
            None
        }
    }

    fn get_user_defined_constructor_call(&mut self, type_name: &str, var_name: &str) -> Option<AstNode> {
        if let Some(index) = self.index.as_ref() {
            // Search for the user defined constructor for the given struct
            if let Some(user_defined_ctor_name) = self.user_defined_constructors.get(type_name) {
                if let Some(_pou) = index.find_method(type_name, user_defined_ctor_name) {
                    // Create an explicit base reference (e.g., "self")
                    // Wrap in a ReferenceExpr with Member access for consistency
                    let base_ident = AstFactory::create_identifier(
                        var_name,
                        SourceLocation::internal(),
                        self.id_provider.next_id(),
                    );
                    let base =
                        AstFactory::create_member_reference(base_ident, None, self.id_provider.next_id());

                    // Create member reference with explicit base: member(FB_INIT) base(self)
                    let op =
                        create_member_reference(user_defined_ctor_name, self.id_provider.clone(), Some(base));
                    let call = AstFactory::create_call_statement(
                        op,
                        None,
                        self.id_provider.next_id(),
                        SourceLocation::internal(),
                    );
                    return Some(call);
                }
            }
        }
        None
    }

    fn get_base_ident(variable_block_type: &VariableBlockType, is_stateful: bool) -> Option<&str> {
        if variable_block_type.is_temp()
            || (variable_block_type.is_local() && !is_stateful)
            || variable_block_type.is_inout()
            || variable_block_type.is_global()
        {
            None
        } else {
            Some("self")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use plc::lowering::vtable::VirtualTableGenerator;
    use plc_ast::{ast::AstNode, visitor::AstVisitor};
    use plc_diagnostics::diagnostician::Diagnostician;
    use plc_driver::pipelines::{AnnotatedProject, BuildPipeline};
    use plc_source::SourceCode;

    use crate::initializer::Initializer;

    fn print_to_string(nodes: &[AstNode]) -> String {
        nodes.iter().map(|it| it.as_string()).collect::<Vec<_>>().join("\n")
    }

    fn print_body_to_string(body: &super::Body) -> String {
        match body {
            super::Body::Internal(nodes) => {
                if nodes.is_empty() {
                    "".into()
                } else {
                    format!(
                        r"intern:
{}",
                        print_to_string(nodes)
                    )
                }
            }
            super::Body::External(nodes) => {
                if nodes.is_empty() {
                    "".into()
                } else {
                    format!(
                        r"extern:
{}",
                        print_to_string(nodes)
                    )
                }
            }
            super::Body::None => "none".to_string(),
        }
    }

    fn parse_and_init(src: &str) -> Initializer {
        parse_and_init_internal(vec![src.into()], false)
    }

    fn parse_and_init_multi(sources: Vec<SourceCode>) -> Initializer {
        parse_and_init_internal(sources, false)
    }

    fn parse_and_init_internal(sources: Vec<SourceCode>, generate_externals: bool) -> Initializer {
        let diagnostician = Diagnostician::buffered();
        let mut pipeline = BuildPipeline::from_sources("test.st", sources, diagnostician).unwrap();
        pipeline.register_mut_participants(vec![Box::new(VirtualTableGenerator::new(
            pipeline.context.provider(),
            generate_externals,
        ))]);
        let AnnotatedProject { units, index, .. } = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), generate_externals);
        initializer.index = Some(Rc::new(index));
        for unit in units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        initializer.index = None;
        initializer
    }

    #[test]
    fn struct_gets_imlicit_initializer_and_constructor() {
        let src = r#"
        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : REAL := 3.14;
            c : BOOL := TRUE;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Expecting a function declaration: void MyStruct__ctor(MyStruct* self)
        // Expecting assignments inside the constructor: self->a = 5; self->b = 3.14; self->c = 1;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @r"
        intern:
        self.a := 5
        self.b := 3.14
        self.c := TRUE
        ");
    }

    #[test]
    fn nested_structs_get_initializers_and_constructors() {
        let src = r#"
        TYPE InnerStruct : STRUCT
            x : INT := 10;
            y : INT := 20;
        END_STRUCT
        END_TYPE

        TYPE OuterStruct : STRUCT
            inner : InnerStruct;
            z : REAL := 2.71;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for constructors
        // Expecting a function declaration: void InnerStruct__ctor(InnerStruct* self)
        // Expecting assignments inside the constructor: self->x = 10; self->y = 20;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct").unwrap()), @r"
        intern:
        self.x := 10
        self.y := 20
        ");
        // Expecting a function declaration: void OuterStruct__ctor(OuterStruct* self)
        // Expecting a call to InnerStruct__ctor(&self->inner);
        // Expecting an assignment: self->z = 2.71;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @"
        intern:
        InnerStruct__ctor(self.inner)
        self.z := 2.71
        ");
    }

    #[test]
    fn nested_structs_with_values_get_initializers_and_constructors() {
        let src = r#"
        TYPE InnerStruct : STRUCT
            x : INT := 10;
            y : INT := 20;
        END_STRUCT
        END_TYPE

        TYPE OuterStruct : STRUCT
            inner : InnerStruct := (x := 1, y := 2);
            inner2 : InnerStruct := (y := 3);
            z : REAL := 2.71;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for constructors
        // Expecting a function declaration: void InnerStruct__ctor(InnerStruct* self)
        // Expecting assignments inside the constructor: self->x = 10; self->y = 20;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct").unwrap()), @r"
        intern:
        self.x := 10
        self.y := 20
        ");
        // Expecting a function declaration: void OuterStruct__ctor(OuterStruct* self)
        // Expecting a call to InnerStruct__ctor(&self->inner);
        // Expecting a call to InnerStruct__ctor(&self->inner2);
        // Expecting assignments: self->inner.x = 1; self->inner.y = 2;
        // Expecting assignments: self->inner2.y = 3;
        // Expecting an assignment: self->z = 2.71;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @"
        intern:
        InnerStruct__ctor(self.inner)
        self.inner.x := 1
        self.inner.y := 2
        InnerStruct__ctor(self.inner2)
        self.inner2.y := 3
        self.z := 2.71
        ");
    }

    #[test]
    fn variable_with_pointer_initializer_is_added_to_constructor() {
        let src = r#"
        VAR_GLOBAL
            gVar : INT;
            myStructVar : MyStruct;
        END_VAR
        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : POINTER TO INT := ADR(gVar);
            c : BOOL := TRUE;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for constructor
        // Expecting a function declaration: void MyStruct__ctor(MyStruct* self)
        // Expecting assignments inside the constructor: self->a = 5; self->c = 1;
        // Expecting an assignment: self->b = &gVar;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @r"
        intern:
        self.a := 5
        self.b := ADR(gVar)
        self.c := TRUE
        ");
    }

    #[test]
    fn enum_default_values_in_struct() {
        let src = r#"
        TYPE MyEnum : (Option1, Option2, Option3) := Option3; END_TYPE
        TYPE MyStruct : STRUCT
            e : MyEnum := Option2;
            n : INT := 42;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyEnum").unwrap()), @r"
        intern:
        self := Option3
        ");
        // Expecting a function declaration: void MyStruct__ctor(MyStruct* self)
        // Expecting a call to MyEnum__ctor(&self->e);
        // Expecting an assignment: self->e = 1;
        // Expecting an assignment: self->n = 42;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @"
        intern:
        MyEnum__ctor(self.e)
        self.e := Option2
        self.n := 42
        ");
    }

    #[test]
    fn nested_struct_with_different_default_values() {
        let src = r#"
        VAR_GLOBAL
            gVar : INT := 10;
        END_VAR

        TYPE InnerStruct : STRUCT
            a : INT := 1;
            b : POINTER TO INT := ADR(gVar);
        END_STRUCT
        END_TYPE

        TYPE InnerStruct2 : STRUCT
            c : INT := 4;
            d : INT := 5;
            inner : InnerStruct := (a := 6);
            inner2 : InnerStruct := (b := ADR(gVar));
        END_STRUCT
        END_TYPE

        TYPE OuterStruct : STRUCT
            e : INT := 0;
            inner : InnerStruct2 := (a := 1, b := 2, inner := (a := 3));
            inner2 : InnerStruct2 := (d := 8, inner := (b := ADR(gVar)));
            inner3 : InnerStruct2 := (inner (a := 9));
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for constructors
        // Expecting a function declaration: void InnerStruct__ctor(InnerStruct* self)
        // Expecting assignments inside the constructor: self->a = 1; self->b = &gVar;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct").unwrap()), @r"
        intern:
        self.a := 1
        self.b := ADR(gVar)
        ");
        // Expecting a function declaration: void InnerStruct2__ctor(InnerStruct2* self)
        // Expecting a call to InnerStruct__ctor(&self->inner);
        // Expecting a call to InnerStruct__ctor(&self->inner2);
        // Expecting assignments: self->c = 4; self->d = 5;
        // Expecting assignments: self->inner.a = 6; self->inner2.b = &gvar;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct2").unwrap()), @"
        intern:
        self.c := 4
        self.d := 5
        InnerStruct__ctor(self.inner)
        self.inner.a := 6
        InnerStruct__ctor(self.inner2)
        self.inner2.b := ADR(gVar)
        ");
        // Expecting a function declaration: void OuterStruct__ctor(OuterStruct* self)
        // Expecting a call to InnerStruct2__ctor(&self->inner);
        // Expecting a call to InnerStruct2__ctor(&self->inner2);
        // Expecting a call to InnerStruct2__ctor(&self->inner3);
        // Expecting an assignment: self->e = 0;
        // Expecting assignments: self->inner.a = 1; self->inner.b = 2; self->inner.inner.a = 3;
        // Expecting assignments: self->inner2.d = 8; self->inner2.inner.b = &gVar;
        // Expecting assignments: self->inner3.inner.a = 9;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @"
        intern:
        self.e := 0
        InnerStruct2__ctor(self.inner)
        self.inner.a := 1
        self.inner.b := 2
        self.inner.inner.a := 3
        InnerStruct2__ctor(self.inner2)
        self.inner2.d := 8
        self.inner2.inner.b := ADR(gVar)
        InnerStruct2__ctor(self.inner3)
        self.inner3 := (inner(a := 9))
        ");
    }

    #[test]
    fn global_constructor_is_generated() {
        let src = r#"
        VAR_GLOBAL
            gVar1 : INT := 10;
            gVar2 : REAL;
            gStructVar : MyStruct;
        END_VAR

        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : POINTER TO INT := ADR(gVar1);
            c : BOOL := TRUE;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for global constructor
        // Expecting a call to MyStruct__ctor(&gStructVar);
        insta::assert_snapshot!(print_to_string(&initializer.global_constructor), @"
        gVar1 := 10
        MyStruct__ctor(gStructVar)
        ");
    }

    #[test]
    fn function_constructors_have_the_function_name() {
        let src = r#"
        FUNCTION MyFunction : INT
        VAR
            localStruct : MyStruct;
        END_VAR
        END_FUNCTION

        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : BOOL := TRUE;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for function constructor on the stack
        // Expecting a call to MyStruct__ctor(&localStruct);
        insta::assert_snapshot!(print_body_to_string(initializer.stack_constructor.get("MyFunction").unwrap()), @"
        intern:
        MyStruct__ctor(localStruct)
        ");
    }

    #[test]
    fn program_temp_variables_are_in_stack_constructor() {
        let src = r#"
        PROGRAM MyProgram
        VAR_TEMP
            tempStruct : MyStruct;
        END_VAR
        VAR
            localStruct : MyStruct;
        END_VAR
        END_PROGRAM

        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : BOOL := TRUE;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for program constructor on the stack
        // Expecting a call to MyStruct__ctor(&localStruct);
        insta::assert_snapshot!(print_body_to_string(initializer.stack_constructor.get("MyProgram").unwrap()), @"
        intern:
        MyStruct__ctor(tempStruct)
        ");
        // Check for program constructor
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyProgram").unwrap()), @"
        intern:
        MyStruct__ctor(self.localStruct)
        ");
    }

    #[test]
    fn programs_are_globals() {
        let src = r#"
        PROGRAM MyProgram
        VAR
        END_VAR
        END_PROGRAM
        "#;

        let initializer = parse_and_init(src);
        // Check for program constructor
        // Expecting a call to MyProgram__ctor(&progStruct);
        insta::assert_snapshot!(print_to_string(&initializer.global_constructor), @"MyProgram__ctor(MyProgram)");
    }

    #[test]
    fn external_structs_and_variables_are_not_initialized() {
        let src = r#"
        VAR_GLOBAL
            internalVar : MyExtStruct;
        END_VAR

        {external}
        VAR_GLOBAL
            extVar : MyExtStruct;
        END_VAR

        {external}
        TYPE MyExtStruct : STRUCT
            a : INT := 5;
            b : BOOL := TRUE;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for internal var assignment in global constructor
        // Expecting a call to MyExtStruct__ctor(&internalVar);
        // No call to MyExtStruct__ctor(&extVar);
        insta::assert_snapshot!(print_to_string(&initializer.global_constructor), @"
        MyExtStruct__ctor(internalVar)
        MyExtStruct__ctor(extVar)
        ");
        // Check that no constructor is generated for MyExtStruct
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyExtStruct").unwrap()), @r"
        extern:
        self.a := 5
        self.b := TRUE
        ");
    }

    #[test]
    fn function_block_fb_init_called_in_constructor() {
        let src = r#"
        FUNCTION_BLOCK MyFB
        VAR
            fbVar : INT;
        END_VAR
        METHOD FB_INIT : VOID
            fbVar := 1;
        END_METHOD
        END_FUNCTION_BLOCK
        "#;
        let initializer = parse_and_init(src);
        // Check for FB_INIT call in constructor
        // Expecting a call to MyFB.FB_INIT(&self);
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyFB").unwrap()), @"
        intern:
        __MyFB___vtable__ctor(self.__vtable)
        self.__vtable := ADR(__vtable_MyFB_instance)
        self.FB_INIT()
        ");
    }

    #[test]
    fn function_block_inheritance_chain_called_in_constructor() {
        let src = r#"
        FUNCTION_BLOCK MyBaseFB
        VAR
            baseVar : INT := 5;
        END_VAR
        METHOD baseMeth
        END_METHOD
        METHOD overrideMeth
        END_METHOD
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK MyFB EXTENDS MyBaseFB
        VAR
            fbVar : INT := 10;
        END_VAR
        METHOD FB_INIT : VOID
            fbVar := 1;
        END_METHOD
        METHOD overrideMeth
        END_METHOD
        METHOD localMeth
        END_METHOD
        END_FUNCTION_BLOCK
        "#;
        let initializer = parse_and_init(src);
        // Check that there's a constructor function for MyBaseFB
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyBaseFB").unwrap()), @"
        intern:
        __MyBaseFB___vtable__ctor(self.__vtable)
        self.baseVar := 5
        self.__vtable := ADR(__vtable_MyBaseFB_instance)
        ");
        // Check that the constructor for MyFB calls the base FB constructor
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyFB").unwrap()), @"
        intern:
        MyBaseFB__ctor(self.__MyBaseFB)
        self.fbVar := 10
        self.__vtable := ADR(__vtable_MyFB_instance)
        self.FB_INIT()
        ");
        // Check that there's a constructor function for the vtables
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("__vtable_MyBaseFB").unwrap()), @r"
        intern:
        self.__body := ADR(MyBaseFB)
        self.baseMeth := ADR(MyBaseFB.baseMeth)
        self.overrideMeth := ADR(MyBaseFB.overrideMeth)
        ");
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("__vtable_MyFB").unwrap()), @r"
        intern:
        self.__body := ADR(MyFB)
        self.baseMeth := ADR(MyBaseFB.baseMeth)
        self.overrideMeth := ADR(MyFB.overrideMeth)
        self.FB_INIT := ADR(MyFB.FB_INIT)
        self.localMeth := ADR(MyFB.localMeth)
        ");
    }

    #[test]
    fn reference_to_local_variable_is_qualified_in_initializer() {
        let src = r#"
        VAR_GLOBAL
            globalVar : INT;
        END_VAR

        FUNCTION_BLOCK foo
        VAR
            i : INT;
            pi: REF_TO INT := REF(i);
            pglobal: REF_TO INT := REF(globalVar);
        END_VAR
        END_FUNCTION_BLOCK
        "#;
        let initializer = parse_and_init(src);
        // Check that local references are qualified with 'self.' but global references are not
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("foo").unwrap()), @"
        intern:
        __foo___vtable__ctor(self.__vtable)
        __foo_pi__ctor(self.pi)
        self.pi := REF(self.i)
        __foo_pglobal__ctor(self.pglobal)
        self.pglobal := REF(globalVar)
        self.__vtable := ADR(__vtable_foo_instance)
        ");
    }

    /// Test that verifies constructor generation when base and child FBs are in separate files.
    /// This is similar to the lit test at tests/lit/multi/constructors/src
    ///
    /// Note: constructor names use the `__ctor` suffix (double underscore).
    /// BUG: The child constructor should call baseFb__ctor(self.__baseFb) to initialize
    /// the parent properly, but currently it only sets up the vtable and calls FB_INIT
    /// directly without initializing the base's fields (like the reference 'y').
    /// This causes unresolved references when the parent constructor logic is skipped.
    ///
    /// Expected child__ctor behavior:
    ///   baseFb__ctor(self.__baseFb)      // <-- MISSING: should call parent constructor first
    ///   self.__vtable := ADR(__vtable_child_instance)
    ///
    /// Current (buggy) child__ctor behavior:
    ///   self.__vtable := ADR(__vtable_child_instance)
    ///   self.FB_INIT()                  // calls FB_INIT but parent fields not initialized
    #[test]
    fn child_constructor_calls_parent_constructor_across_files() {
        let base: SourceCode = SourceCode::new(
            r#"
            FUNCTION_BLOCK baseFb
                VAR_OUTPUT
                    x : DINT;
                    y : REFERENCE TO DINT REF= x;
                END_VAR

                METHOD FB_INIT
                    THIS^.x := 10;
                    THIS^.y := THIS^.x + 1; //x and y are 11
                END_METHOD
            END_FUNCTION_BLOCK
            "#,
            "base.st",
        );

        let child: SourceCode = SourceCode::new(
            r#"
            FUNCTION_BLOCK child EXTENDS baseFb
                THIS^.x := This^.x + 5; // x is now 16
                THIS^.y := THIS^.y + 2; // y is now 18
            END_FUNCTION_BLOCK
            "#,
            "child.st",
        );

        let main: SourceCode = SourceCode::new(
            r#"
            VAR_GLOBAL
                childInst : child;
            END_VAR

            FUNCTION main : DINT
                childInst();
                main := childInst.y + childInst.x; // Should be 36 (18 + 18)
            END_FUNCTION
            "#,
            "main.st",
        );

        let initializer = parse_and_init_multi(vec![base, child, main]);

        // baseFb constructor properly initializes vtable, reference field 'y', and calls FB_INIT
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("baseFb").unwrap()), @"
        intern:
        __baseFb___vtable__ctor(self.__vtable)
        __baseFb_y__ctor(self.y)
        self.y REF= self.x
        self.__vtable := ADR(__vtable_baseFb_instance)
        self.FB_INIT()
        ");

        // child constructor calls baseFb__ctor first, then sets up its own vtable
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("child").unwrap()), @"
        intern:
        baseFb__ctor(self.__baseFb)
        self.__vtable := ADR(__vtable_child_instance)
        self.FB_INIT()
        ");
    }

    /// Test that alias variables (AT syntax) in method local variables are properly initialized
    /// with REF= assignment. This tests the case where a method has `px AT x : DINT` which should
    /// generate `px REF= x` in the stack constructor (REF= assigns the address without dereferencing).
    #[test]
    fn alias_variable_in_method_wrapped_in_adr() {
        let src = r#"
            FUNCTION_BLOCK foo
                METHOD bar
                    VAR
                        x : DINT;
                        px AT x : DINT;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK
            "#;

        let initializer = parse_and_init(src);

        // The method's stack constructor should initialize px with REF= x
        insta::assert_snapshot!(print_body_to_string(initializer.stack_constructor.get("foo.bar").unwrap()), @"
        intern:
        __foo.bar_px__ctor(px)
        px REF= x
        ");
    }

    /// Test that alias variables in function blocks are properly initialized with REF=
    #[test]
    fn alias_variable_in_function_block_wrapped_in_adr() {
        let src = r#"
            FUNCTION_BLOCK foo
            VAR
                x : DINT;
                px AT x : DINT;
            END_VAR
            END_FUNCTION_BLOCK
            "#;

        let initializer = parse_and_init(src);

        // The constructor should initialize px with REF= self.x
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("foo").unwrap()), @"
        intern:
        __foo___vtable__ctor(self.__vtable)
        __foo_px__ctor(self.px)
        self.px REF= self.x
        self.__vtable := ADR(__vtable_foo_instance)
        ");
    }

    /// Test that nested struct members are properly initialized with their constructors.
    /// When a struct contains a member of another struct type, the member's constructor
    /// should be called to ensure all nested members are initialized properly.
    #[test]
    fn nested_struct_member_initialization() {
        let src = r#"
            FUNCTION_BLOCK Inner
            VAR
                value : DINT := 42;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK Outer
            VAR
                inner : Inner;
            END_VAR
            END_FUNCTION_BLOCK
            "#;

        let initializer = parse_and_init(src);

        // Inner constructor should initialize its member
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("Inner").unwrap()), @"
        intern:
        __Inner___vtable__ctor(self.__vtable)
        self.value := 42
        self.__vtable := ADR(__vtable_Inner_instance)
        ");

        // Outer constructor should call Inner__ctor for the inner member, then initialize its own vtable
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("Outer").unwrap()), @"
        intern:
        __Outer___vtable__ctor(self.__vtable)
        Inner__ctor(self.inner)
        self.__vtable := ADR(__vtable_Outer_instance)
        ");
    }

    /// Test that reference variables in function blocks are properly initialized with REF= assignment
    /// and that the RHS references to local variables are qualified with 'self.'
    #[test]
    fn reference_variable_in_fb_local_reference_qualified() {
        let src = r#"
            FUNCTION_BLOCK baseFb
                VAR_OUTPUT
                    x : DINT;
                    y : REFERENCE TO DINT REF= x;
                END_VAR
            END_FUNCTION_BLOCK
            "#;

        let initializer = parse_and_init(src);

        // baseFb constructor should initialize reference 'y' with qualified reference 'self.x'
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("baseFb").unwrap()), @"
        intern:
        __baseFb___vtable__ctor(self.__vtable)
        __baseFb_y__ctor(self.y)
        self.y REF= self.x
        self.__vtable := ADR(__vtable_baseFb_instance)
        ");
    }

    #[test]
    fn reference_types_in_struct_field_initializers() {
        let src = r#"
        VAR_GLOBAL
            gInt : DINT;
        END_VAR

        TYPE RefStruct : STRUCT
            r : REFERENCE TO DINT := REF(gInt);
            rt : REF_TO DINT := REF(gInt);
            p : POINTER TO DINT := ADR(gInt);
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        let body = print_body_to_string(initializer.constructors.get("RefStruct").unwrap());

        assert!(body.contains("self.r REF= gInt"));
        assert!(!body.contains("self.r := REF(gInt)"));

        assert!(body.contains("self.rt := REF(gInt)"));
        assert!(!body.contains("self.rt REF= gInt"));

        assert!(body.contains("self.p := ADR(gInt)"));
        assert!(!body.contains("self.p REF= gInt"));
    }

    #[test]
    fn reference_types_in_fb_variable_initializers() {
        let src = r#"
        FUNCTION_BLOCK RefVars
        VAR
            x : DINT;
            r : REFERENCE TO DINT := REF(x);
            rt : REF_TO DINT := REF(x);
            p : POINTER TO DINT := ADR(x);
        END_VAR
        END_FUNCTION_BLOCK
        "#;

        let initializer = parse_and_init(src);
        let body = print_body_to_string(initializer.constructors.get("RefVars").unwrap());

        assert!(body.contains("self.r REF= self.x"));
        assert!(!body.contains("self.r := REF(self.x)"));

        assert!(body.contains("self.rt := REF(self.x)"));
        assert!(!body.contains("self.rt REF= self.x"));

        assert!(body.contains("self.p := ADR(self.x)"));
        assert!(!body.contains("self.p REF= self.x"));
    }

    #[test]
    fn alias_at_identifier_vs_hardware_address() {
        let src = r#"
        VAR_GLOBAL
            target : DINT;
            alias_ident AT target : DINT;
            alias_hw AT %IX1.2 : DINT;
        END_VAR
        "#;

        let initializer = parse_and_init(src);
        let body = print_to_string(&initializer.global_constructor);

        assert!(body.contains("alias_ident REF= target"));
        assert!(!body.contains("alias_hw REF= %IX1.2"));
    }

    #[test]
    fn struct_literal_qualifies_local_references_in_stateful_pou() {
        let src = r#"
        TYPE MyStruct : STRUCT
            field : DINT;
        END_STRUCT
        END_TYPE

        FUNCTION_BLOCK Foo
        VAR
            local : DINT;
            s : MyStruct := (field := local);
        END_VAR
        END_FUNCTION_BLOCK
        "#;

        let initializer = parse_and_init(src);
        let body = print_body_to_string(initializer.constructors.get("Foo").unwrap());

        assert!(body.contains("self.s.field := self.local"));
        assert!(!body.contains("self.s.field := local"));
    }

    #[test]
    fn ref_in_struct_literal_with_parens_uses_ref_assignment() {
        let src = r#"
        VAR_GLOBAL
            g : DINT;
            s : MyStruct := (r := (REF(g)));
        END_VAR

        TYPE MyStruct : STRUCT
            r : REFERENCE TO DINT;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        let body = print_to_string(&initializer.global_constructor);

        assert!(body.contains("s.r REF= g"));
    }
}
