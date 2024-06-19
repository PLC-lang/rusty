// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::hash::BuildHasherDefault;

use itertools::Itertools;
use rustc_hash::{FxHashSet, FxHasher};

use plc_ast::ast::{
    AstId, AstNode, AstStatement, DirectAccessType, GenericBinding, HardwareAccessType, LinkageType, PouType,
    TypeNature,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use plc_util::convention::qualified_name;

use crate::{
    builtins::{self, BuiltIn},
    datalayout::DataLayout,
    typesystem::{self, *},
};

use self::{
    const_expressions::{ConstExpressions, ConstId},
    instance_iterator::InstanceIterator,
    symbol::SymbolMap,
};

pub mod const_expressions;
mod instance_iterator;
pub mod symbol;
pub mod visitor;

#[cfg(test)]
mod tests;

/// Type alias for an IndexMap using the `fx` hashing algorithm, see https://github.com/rust-lang/rustc-hash
pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<FxHasher>>;

/// Type alias for a IndexSet using the `fx` hashing algorithm, see https://github.com/rust-lang/rustc-hash
pub type FxIndexSet<K> = indexmap::IndexSet<K, BuildHasherDefault<FxHasher>>;

/// A label represents a possible jump point in the source.
/// It can be referenced by jump elements in the same unit
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Label {
    pub id: AstId,
    pub name: String,
    pub location: SourceLocation,
}

impl From<&AstNode> for Label {
    fn from(value: &AstNode) -> Self {
        Label {
            id: value.get_id(),
            name: value.get_label_name().unwrap_or_default().to_string(),
            location: value.get_location(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct VariableIndexEntry {
    /// the name of this variable (e.g. 'x' for 'PLC_PRG.x')
    name: String,
    /// the fully qualified name of this variable (e.g. PLC_PRG.x)
    qualified_name: String,
    /// an optional initial value of this variable
    pub initial_value: Option<ConstId>,
    /// the type of variable
    pub argument_type: ArgumentType,
    /// true if this variable is a compile-time-constant
    is_constant: bool,
    /// the variable's datatype
    pub data_type_name: String,
    /// the index of the member-variable in it's container (e.g. struct). defautls to 0 (Single variables)
    location_in_parent: u32,
    /// Wether the variable is externally or internally available
    linkage: LinkageType,
    /// A binding to a hardware or external location
    binding: Option<HardwareBinding>,
    /// the location in the original source-file
    pub source_location: SourceLocation,
    /// Variadic information placeholder for the variable, if any
    varargs: Option<VarArgs>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HardwareBinding {
    /// Specifies if the binding is an In/Out or Memory binding
    pub direction: HardwareAccessType,
    /// The datatype (size) of the binding
    pub access: DirectAccessType,
    /// A list of entries that form this binding
    pub entries: Vec<ConstId>,
    /// The location in the original source-file
    pub location: SourceLocation,
}

impl HardwareBinding {
    fn from_statement(index: &mut Index, it: &AstNode, scope: Option<String>) -> Option<Self> {
        if let AstStatement::HardwareAccess(data) = it.get_stmt() {
            Some(HardwareBinding {
                access: data.access,
                direction: data.direction,
                entries: data
                    .address
                    .iter()
                    .map(|expr| {
                        index.constant_expressions.add_constant_expression(
                            expr.clone(),
                            typesystem::DINT_SIZE.to_string(),
                            scope.clone(),
                        )
                    })
                    .collect(),
                location: it.get_location(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct MemberInfo<'b> {
    container_name: &'b str,
    variable_name: &'b str,
    variable_linkage: ArgumentType,
    variable_type_name: &'b str,
    binding: Option<HardwareBinding>,
    is_constant: bool,
    varargs: Option<VarArgs>,
}

impl VariableIndexEntry {
    pub fn new(
        name: &str,
        qualified_name: &str,
        data_type_name: &str,
        argument_type: ArgumentType,
        location_in_parent: u32,
        source_location: SourceLocation,
    ) -> Self {
        VariableIndexEntry {
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            initial_value: None,
            argument_type,
            is_constant: false,
            data_type_name: data_type_name.to_string(),
            location_in_parent,
            linkage: LinkageType::Internal,
            binding: None,
            source_location,
            varargs: None,
        }
    }

    pub fn create_global(
        name: &str,
        qualified_name: &str,
        data_type_name: &str,
        source_location: SourceLocation,
    ) -> Self {
        VariableIndexEntry {
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            initial_value: None,
            argument_type: ArgumentType::ByVal(VariableType::Global),
            is_constant: false,
            data_type_name: data_type_name.to_string(),
            location_in_parent: 0,
            linkage: LinkageType::Internal,
            binding: None,
            source_location,
            varargs: None,
        }
    }

    pub fn set_linkage(mut self, linkage: LinkageType) -> Self {
        self.linkage = linkage;
        self
    }

    pub fn set_initial_value(mut self, initial_value: Option<ConstId>) -> Self {
        self.initial_value = initial_value;
        self
    }

    pub fn set_constant(mut self, is_constant: bool) -> Self {
        self.is_constant = is_constant;
        self
    }

    pub fn set_hardware_binding(mut self, binding: Option<HardwareBinding>) -> Self {
        self.binding = binding;
        self
    }

    pub fn set_varargs(mut self, varargs: Option<VarArgs>) -> Self {
        self.varargs = varargs;
        self
    }

    /// Creates a new VariableIndexEntry from the current entry with a new container and type
    /// This is used to create new entries from previously generic entries
    pub fn into_typed(&self, container: &str, new_type: &str) -> Self {
        let name = if self.is_return() { container } else { &self.name };
        let varargs = self.varargs.as_ref().map(|varargs| varargs.as_typed(new_type));

        VariableIndexEntry {
            name: name.to_string(),
            qualified_name: qualified_name(container, name),
            data_type_name: new_type.to_string(),
            varargs,
            ..self.to_owned()
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_qualified_name(&self) -> &str {
        self.qualified_name.as_str()
    }

    pub fn get_type_name(&self) -> &str {
        self.data_type_name.as_str()
    }

    pub fn get_location_in_parent(&self) -> u32 {
        self.location_in_parent
    }

    pub fn is_return(&self) -> bool {
        self.get_variable_type() == VariableType::Return
    }

    pub fn is_local(&self) -> bool {
        self.get_variable_type() == VariableType::Local
    }
    pub fn is_temp(&self) -> bool {
        self.get_variable_type() == VariableType::Temp
    }

    pub fn is_constant(&self) -> bool {
        self.is_constant
    }

    pub fn is_external(&self) -> bool {
        self.linkage == LinkageType::External
    }

    pub fn get_declaration_type(&self) -> ArgumentType {
        self.argument_type
    }

    pub fn get_linkage(&self) -> LinkageType {
        self.linkage
    }

    pub fn is_in_unit(&self, unit: &str) -> bool {
        self.source_location.is_in_unit(unit)
    }

    pub fn get_variable_type(&self) -> VariableType {
        self.argument_type.get_inner()
    }

    pub fn has_hardware_binding(&self) -> bool {
        self.binding.is_some()
    }

    pub fn get_hardware_binding(&self) -> Option<&HardwareBinding> {
        self.binding.as_ref()
    }

    pub fn is_parameter(&self) -> bool {
        matches!(self.get_variable_type(), VariableType::Input | VariableType::Output | VariableType::InOut)
    }

    /// returns `true` for `VAR_INPUT {ref}` and `VAR_IN_OUT`
    pub fn is_in_parameter_by_ref(&self) -> bool {
        matches!(
            self.get_declaration_type(),
            ArgumentType::ByRef(VariableType::Input) | ArgumentType::ByRef(VariableType::InOut)
        )
    }

    pub fn is_variadic(&self) -> bool {
        self.varargs.is_some()
    }

    pub fn get_varargs(&self) -> Option<&VarArgs> {
        self.varargs.as_ref()
    }

    fn has_parent(&self, context: &str) -> bool {
        let name = qualified_name(context, &self.name);
        self.qualified_name.eq_ignore_ascii_case(&name)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArgumentType {
    ByVal(VariableType),
    ByRef(VariableType),
}

impl ArgumentType {
    pub fn get_inner(&self) -> VariableType {
        match self {
            ArgumentType::ByVal(t) => *t,
            ArgumentType::ByRef(t) => *t,
        }
    }

    pub fn get_inner_ref(&self) -> &VariableType {
        match self {
            ArgumentType::ByRef(val) => val,
            ArgumentType::ByVal(val) => val,
        }
    }

    pub fn is_by_ref(&self) -> bool {
        matches!(self, ArgumentType::ByRef(..))
    }

    pub fn is_private(&self) -> bool {
        matches!(self.get_inner_ref(), VariableType::Temp | VariableType::Local)
    }

    pub fn is_input(&self) -> bool {
        matches!(self.get_inner_ref(), VariableType::Input)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum VariableType {
    Local, // functions have no locals; others: VAR-block
    Temp,  // for functions: VAR & VAR_TEMP; others: VAR_TEMP
    Input,
    Output,
    InOut,
    Global,
    Return,
}

impl VariableType {
    pub fn is_output(&self) -> bool {
        matches!(self, VariableType::Output)
    }
}

impl std::fmt::Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::Local => write!(f, "Local"),
            VariableType::Temp => write!(f, "Temp"),
            VariableType::Input => write!(f, "Input"),
            VariableType::Output => write!(f, "Output"),
            VariableType::InOut => write!(f, "InOut"),
            VariableType::Global => write!(f, "Global"),
            VariableType::Return => write!(f, "Return"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ImplementationType {
    Program,
    Function,
    FunctionBlock,
    Action,
    Class,
    Method,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImplementationIndexEntry {
    pub(crate) call_name: String,
    pub(crate) type_name: String,
    pub(crate) associated_class: Option<String>,
    pub(crate) implementation_type: ImplementationType,
    pub(crate) generic: bool,
    pub(crate) location: SourceLocation,
}

impl ImplementationIndexEntry {
    pub fn get_call_name(&self) -> &str {
        &self.call_name
    }
    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }
    pub fn get_associated_class_name(&self) -> Option<&String> {
        self.associated_class.as_ref()
    }
    pub fn get_implementation_type(&self) -> &ImplementationType {
        &self.implementation_type
    }

    pub fn is_generic(&self) -> bool {
        self.generic
    }

    pub fn get_location(&self) -> &SourceLocation {
        &self.location
    }

    pub fn is_in_unit(&self, unit: impl AsRef<str>) -> bool {
        self.get_location().is_in_unit(unit)
    }
}

impl From<&PouType> for ImplementationType {
    fn from(it: &PouType) -> Self {
        match it {
            PouType::Program => ImplementationType::Program,
            PouType::Function => ImplementationType::Function,
            PouType::FunctionBlock => ImplementationType::FunctionBlock,
            PouType::Action => ImplementationType::Action,
            PouType::Class => ImplementationType::Class,
            PouType::Method { .. } => ImplementationType::Method,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PouIndexEntry {
    Program {
        name: String,
        instance_struct_name: String,
        instance_variable: Box<VariableIndexEntry>,
        linkage: LinkageType,
        location: SourceLocation,
    },
    FunctionBlock {
        name: String,
        instance_struct_name: String,
        linkage: LinkageType,
        location: SourceLocation,
        super_class: Option<String>,
    },
    Function {
        name: String,
        return_type: String,
        generics: Vec<GenericBinding>,
        linkage: LinkageType,
        is_variadic: bool,
        location: SourceLocation,
        is_generated: bool, // true if this entry was added automatically (e.g. by generics)
    },
    Class {
        name: String,
        instance_struct_name: String,
        linkage: LinkageType,
        location: SourceLocation,
        super_class: Option<String>,
    },
    Method {
        name: String,
        parent_pou_name: String,
        return_type: String,
        instance_struct_name: String,
        linkage: LinkageType,
        location: SourceLocation,
    },
    Action {
        name: String,
        parent_pou_name: String,
        instance_struct_name: String,
        linkage: LinkageType,
        location: SourceLocation,
    },
}

impl PouIndexEntry {
    /// creates a new Program-PouIndexEntry
    /// # Arguments
    /// - `name` the name of the function
    /// - `instance_variable` the global instance-variable of the program
    pub fn create_program_entry(
        pou_name: &str,
        instance_variable: VariableIndexEntry,
        linkage: LinkageType,
        location: SourceLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::Program {
            name: pou_name.into(),
            instance_struct_name: pou_name.into(),
            instance_variable: Box::new(instance_variable),
            linkage,
            location,
        }
    }

    /// creates a new FunctionBlock-PouIndexEntry
    /// # Arguments
    /// - `name` the name of the FunctionBlock
    /// - `linkage` the linkage type of the pou
    pub fn create_function_block_entry(
        pou_name: &str,
        linkage: LinkageType,
        location: SourceLocation,
        super_class: Option<&str>,
    ) -> PouIndexEntry {
        PouIndexEntry::FunctionBlock {
            name: pou_name.into(),
            instance_struct_name: pou_name.into(),
            linkage,
            location,
            super_class: super_class.map(|s| s.to_owned()),
        }
    }

    /// creates a new Function-PouIndexEntry
    pub fn create_function_entry(
        name: &str,
        return_type: &str,
        generic_names: &[GenericBinding],
        linkage: LinkageType,
        is_variadic: bool,
        location: SourceLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::Function {
            name: name.into(),
            generics: generic_names.to_vec(),
            return_type: return_type.into(),
            linkage,
            is_variadic,
            location,
            is_generated: false,
        }
    }

    /// creates a new Function-PouIndexEntry generated by the compiler
    /// this will set the is_generated attribute to true.
    pub fn create_generated_function_entry(
        name: &str,
        return_type: &str,
        generic_names: &[GenericBinding],
        linkage: LinkageType,
        is_variadic: bool,
        location: SourceLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::Function {
            name: name.into(),
            generics: generic_names.to_vec(),
            return_type: return_type.into(),
            linkage,
            is_variadic,
            location,
            is_generated: true,
        }
    }

    /// creates a new Action-PouIndexEntry
    /// # Arguments
    /// - `name` the name of the action (without the pou-qualifier)
    /// - `pou_name` the name of the parent pou
    pub fn create_action_entry(
        qualified_name: &str,
        pou_name: &str,
        linkage: LinkageType,
        location: SourceLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::Action {
            name: qualified_name.into(),
            parent_pou_name: pou_name.into(),
            instance_struct_name: pou_name.into(),
            linkage,
            location,
        }
    }

    /// creates a new Class-PouIndexEntry
    /// # Arguments
    /// - `name` the name of the Class
    pub fn create_class_entry(
        pou_name: &str,
        linkage: LinkageType,
        location: SourceLocation,
        super_class: Option<String>,
    ) -> PouIndexEntry {
        PouIndexEntry::Class {
            name: pou_name.into(),
            instance_struct_name: pou_name.into(),
            linkage,
            location,
            super_class,
        }
    }

    /// creates a new Method-PouIndexEntry
    /// # Arguments
    /// - `name` the name of the method (without the pou-qualifier)
    /// - `return_type` the name of the method's return type
    /// - `owner_class` the name of the parent pou
    pub fn create_method_entry(
        name: &str,
        return_type: &str,
        owner_class: &str,
        linkage: LinkageType,
        location: SourceLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::Method {
            name: name.into(),
            parent_pou_name: owner_class.into(),
            instance_struct_name: name.into(),
            return_type: return_type.into(),
            linkage,
            location,
        }
    }

    /// returns the fully qualified name of this pou
    pub fn get_name(&self) -> &str {
        match self {
            PouIndexEntry::Program { name, .. }
            | PouIndexEntry::FunctionBlock { name, .. }
            | PouIndexEntry::Function { name, .. }
            | PouIndexEntry::Method { name, .. }
            | PouIndexEntry::Action { name, .. }
            | PouIndexEntry::Class { name, .. } => name,
        }
    }

    /// returns the super class of this pou if supported
    pub fn get_super_class(&self) -> Option<&str> {
        match self {
            PouIndexEntry::Class { super_class, .. } | PouIndexEntry::FunctionBlock { super_class, .. } => {
                super_class.as_deref()
            }
            _ => None,
        }
    }

    /// returns the name of the struct-type used to store the POUs state
    /// (interface-variables)
    pub fn get_instance_struct_type_name(&self) -> Option<&str> {
        match self {
            PouIndexEntry::Program { instance_struct_name, .. }
            | PouIndexEntry::FunctionBlock { instance_struct_name, .. }
            | PouIndexEntry::Method { instance_struct_name, .. }
            | PouIndexEntry::Action { instance_struct_name, .. }
            | PouIndexEntry::Class { instance_struct_name, .. } => Some(instance_struct_name.as_str()),
            _ => None, //functions have no struct type
        }
    }

    /// returns `Some(DataType)` associated with this pou or `None` if none is associated
    ///
    /// - `index` the index to fetch te DataType from
    pub fn find_instance_struct_type<'idx>(&self, index: &'idx Index) -> Option<&'idx DataType> {
        self.get_instance_struct_type_name().and_then(|it| index.type_index.find_pou_type(it))
    }

    /// returns the struct-datatype associated with this pou or `void` if none is associated
    ///
    /// - `index` the index to fetch te DataType from
    pub fn get_instance_struct_type_or_void<'idx>(&self, index: &'idx Index) -> &'idx DataType {
        self.find_instance_struct_type(index).unwrap_or_else(|| index.get_void_type())
    }

    /// returns the name of the POUs container
    /// it has no container (PROGRAM, FUNCTION, etc.)
    ///
    /// Actions and Methods return their host-POUs name
    pub fn get_container(&self) -> &str {
        match self {
            PouIndexEntry::Program { .. }
            | PouIndexEntry::FunctionBlock { .. }
            | PouIndexEntry::Class { .. }
            | PouIndexEntry::Function { .. } => self.get_name(),
            PouIndexEntry::Action { parent_pou_name, .. } | PouIndexEntry::Method { parent_pou_name, .. } => {
                parent_pou_name.as_str()
            }
        }
    }

    /// returns the ImplementationIndexEntry associated with this POU
    pub fn find_implementation<'idx>(&self, index: &'idx Index) -> Option<&'idx ImplementationIndexEntry> {
        index.find_implementation_by_name(self.get_name())
    }

    /// returns the linkage type of this pou
    pub fn get_linkage(&self) -> &LinkageType {
        match self {
            PouIndexEntry::Program { linkage, .. }
            | PouIndexEntry::FunctionBlock { linkage, .. }
            | PouIndexEntry::Function { linkage, .. }
            | PouIndexEntry::Method { linkage, .. }
            | PouIndexEntry::Action { linkage, .. }
            | PouIndexEntry::Class { linkage, .. } => linkage,
        }
    }

    /// returns true if this pou is a function with generic parameters, otherwise false
    pub fn is_generic(&self) -> bool {
        if let PouIndexEntry::Function { generics, .. } = self {
            !generics.is_empty()
        } else {
            false
        }
    }

    /// returns true if this pou has a variadic (last) parameter
    pub fn is_variadic(&self) -> bool {
        if let PouIndexEntry::Function { is_variadic, .. } = self {
            *is_variadic
        } else {
            false
        }
    }

    /// returns true if this pou is an action
    pub fn is_action(&self) -> bool {
        matches!(self, PouIndexEntry::Action { .. })
    }

    /// return true if this pou is a function
    pub fn is_function(&self) -> bool {
        matches!(self, PouIndexEntry::Function { .. })
    }

    pub fn is_void_function(&self) -> bool {
        match self {
            PouIndexEntry::Function { return_type, .. } => return_type.as_str() == VOID_TYPE,
            _ => false,
        }
    }

    pub fn is_function_block(&self) -> bool {
        matches!(self, PouIndexEntry::FunctionBlock { .. })
    }

    pub fn is_class(&self) -> bool {
        matches!(self, PouIndexEntry::Class { .. })
    }

    pub(crate) fn is_method(&self) -> bool {
        matches!(self, PouIndexEntry::Method { .. })
    }

    pub fn get_location(&self) -> &SourceLocation {
        match self {
            PouIndexEntry::Program { location, .. }
            | PouIndexEntry::FunctionBlock { location, .. }
            | PouIndexEntry::Function { location, .. }
            | PouIndexEntry::Method { location, .. }
            | PouIndexEntry::Action { location, .. }
            | PouIndexEntry::Class { location, .. } => location,
        }
    }

    fn is_auto_generated_function(&self) -> bool {
        matches!(self, PouIndexEntry::Function { is_generated: true, .. })
    }
}

/// the TypeIndex carries all types.
/// it is extracted into its seaprate struct so it can be
/// internally borrowed individually from the other maps
#[derive(Debug)]
pub struct TypeIndex {
    /// all types (structs, enums, type, POUs, etc.)
    types: SymbolMap<String, DataType>,
    pou_types: SymbolMap<String, DataType>,

    void_type: DataType,
}

impl Default for TypeIndex {
    fn default() -> Self {
        TypeIndex {
            types: SymbolMap::default(),
            pou_types: SymbolMap::default(),
            void_type: DataType {
                name: VOID_TYPE.into(),
                initial_value: None,
                information: DataTypeInformation::Void,
                nature: TypeNature::Any,
                location: SourceLocation::internal(),
            },
        }
    }
}

impl TypeIndex {
    pub fn find_type(&self, type_name: &str) -> Option<&DataType> {
        self.types.get(&type_name.to_lowercase()).or_else(|| self.find_pou_type(type_name))
    }

    pub fn find_pou_type(&self, type_name: &str) -> Option<&DataType> {
        self.pou_types.get(&type_name.to_lowercase())
    }

    pub fn find_effective_type_by_name(&self, type_name: &str) -> Option<&DataType> {
        self.find_type(type_name).and_then(|it| self.find_effective_type(it))
    }

    pub fn get_effective_type_by_name(&self, type_name: &str) -> &DataType {
        self.find_type(type_name).and_then(|it| self.find_effective_type(it)).unwrap_or(&self.void_type)
    }

    pub fn get_type(&self, type_name: &str) -> Result<&DataType, Diagnostic> {
        self.find_type(type_name)
            .ok_or_else(|| Diagnostic::unknown_type(type_name, SourceLocation::undefined()))
    }

    /// Retrieves the "Effective" type behind this datatype
    /// An effective type will be any end type i.e. Structs, Integers, Floats, String and Array
    pub fn find_effective_type<'ret>(&'ret self, data_type: &'ret DataType) -> Option<&'ret DataType> {
        match data_type.get_type_information() {
            DataTypeInformation::Alias { referenced_type, .. } => {
                self.find_type(referenced_type).and_then(|it| self.find_effective_type(it))
            }
            _ => Some(data_type),
        }
    }
}

/// The global index of the rusty-compiler
///
/// The index contains information about all referencable elements.
#[derive(Debug, Default)]
pub struct Index {
    /// all global variables
    global_variables: SymbolMap<String, VariableIndexEntry>,

    /// all struct initializers
    global_initializers: SymbolMap<String, VariableIndexEntry>,

    /// all enum-members with their names
    enum_global_variables: SymbolMap<String, VariableIndexEntry>,

    // all pous,
    pous: SymbolMap<String, PouIndexEntry>,

    /// all implementations
    // we keep an IndexMap for implementations since duplication issues regarding implementations
    // is handled by the `pous` SymbolMap
    implementations: FxIndexMap<String, ImplementationIndexEntry>,

    /// an index with all type-information
    pub type_index: TypeIndex,

    constant_expressions: ConstExpressions,

    /// Type layout for the target
    data_layout: DataLayout,

    /// The labels contained in each pou
    labels: FxIndexMap<String, SymbolMap<String, Label>>,
}

impl Index {
    /// imports all entries from the given index into the current index
    ///
    /// imports all global_variables, types and implementations
    /// # Arguments
    /// - `other` the other index. The elements are drained from the given index and moved
    /// into the current one
    pub fn import(&mut self, mut other: Index) {
        //global variables
        for (name, e) in other.global_variables.drain(..) {
            let entries = e
                .into_iter()
                .map(|it| self.transfer_constants(it, &mut other.constant_expressions))
                .collect::<Vec<_>>();
            self.global_variables.insert_many(name, entries);
        }

        //initializers
        for (name, elements) in other.global_initializers.drain(..) {
            let elements = elements
                .into_iter()
                .map(|e| self.transfer_constants(e, &mut other.constant_expressions))
                .collect::<Vec<_>>();
            self.global_initializers.insert_many(name, elements);
        }

        //types
        for (name, mut elements) in other.type_index.types.drain(..) {
            for mut e in elements.drain(..) {
                //avoid re-importing internally auto-generated types that we already know
                if !e.is_internal() || self.type_index.find_effective_type_by_name(name.as_str()).is_none() {
                    e.initial_value =
                        self.maybe_import_const_expr(&mut other.constant_expressions, &e.initial_value);

                    match &mut e.information {
                        //import constant expressions in array-type-definitions
                        DataTypeInformation::Array { dimensions, .. } => {
                            for d in dimensions.iter_mut() {
                                d.start_offset =
                                    self.import_type_size(&mut other.constant_expressions, &d.start_offset);
                                d.end_offset =
                                    self.import_type_size(&mut other.constant_expressions, &d.end_offset);
                            }
                        }
                        // import constant expressions in String-size defintions
                        DataTypeInformation::String { size, .. } => {
                            *size = self.import_type_size(&mut other.constant_expressions, size);
                        }
                        DataTypeInformation::Struct { members, .. } => {
                            let mut variables = members
                                .drain(..)
                                .map(|variable| {
                                    self.transfer_constants(variable, &mut other.constant_expressions)
                                })
                                .collect::<Vec<_>>();
                            members.append(&mut variables);
                        }
                        DataTypeInformation::Enum { variants, .. } => {
                            let mut variables = variants
                                .drain(..)
                                .map(|variable| {
                                    self.transfer_constants(variable, &mut other.constant_expressions)
                                })
                                .collect::<Vec<_>>();

                            for e in variables.iter() {
                                self.enum_global_variables.insert(e.get_name().to_lowercase(), e.clone());
                            }
                            variants.append(&mut variables);
                        }
                        _ => {}
                    }

                    self.type_index.types.insert(name.clone(), e)
                }
            }
        }

        //pou_types
        for (name, mut elements) in other.type_index.pou_types.drain(..) {
            elements.iter_mut().for_each(|e| {
                self.maybe_import_const_expr(&mut other.constant_expressions, &e.initial_value);

                if let DataTypeInformation::Struct { members, .. } = &mut e.information {
                    let mut variables = members
                        .drain(..)
                        .map(|variable| self.transfer_constants(variable, &mut other.constant_expressions))
                        .collect::<Vec<_>>();
                    members.append(&mut variables);
                }
            });
            self.type_index.pou_types.insert_many(name, elements)
        }

        //implementations
        self.implementations.extend(other.implementations);

        //pous
        for (name, elements) in other.pous.drain(..) {
            for ele in elements {
                // skip automatically generated pou's if they are already in the target index
                if !ele.is_auto_generated_function() || !self.pous.contains_key(&name) {
                    self.pous.insert(name.clone(), ele);
                }
            }
        }

        //labels
        self.labels.extend(other.labels);

        //Constant expressions are intentionally not imported
        // self.constant_expressions.import(other.constant_expressions)
    }

    fn transfer_constants(
        &mut self,
        mut variable: VariableIndexEntry,
        import_from: &mut ConstExpressions,
    ) -> VariableIndexEntry {
        variable.initial_value = self.maybe_import_const_expr(import_from, &variable.initial_value);

        let binding = if let Some(HardwareBinding { direction, access, entries, location }) =
            variable.get_hardware_binding()
        {
            let mut new_entries = vec![];
            for entry in entries {
                if let Some(e) = self.maybe_import_const_expr(import_from, &Some(*entry)) {
                    new_entries.push(e);
                }
            }
            Some(HardwareBinding {
                direction: *direction,
                access: *access,
                entries: new_entries,
                location: location.clone(),
            })
        } else {
            None
        };
        variable.set_hardware_binding(binding)
    }

    /// imports the corresponding const-expression (according to the given initializer-id) from the given ConstExpressions
    /// into self's const-expressions and returns the new Id
    fn maybe_import_const_expr(
        &mut self,
        import_from: &mut ConstExpressions,
        initializer_id: &Option<ConstId>,
    ) -> Option<ConstId> {
        initializer_id.as_ref().and_then(|it| import_from.clone(it)).map(|(init, target_type, scope)| {
            self.get_mut_const_expressions().add_constant_expression(init, target_type, scope)
        })
    }

    /// imports the corresponding TypeSize (according to the given initializer-id) from the given ConstExpressions
    /// into self's const-expressions and returns the new Id
    ///
    /// panics if the import fails (e.g. the given TypeSize::ConstExpression(id) does not exist in this Index)
    /// this problem would indicate a programming mistake
    fn import_type_size(&mut self, import_from: &mut ConstExpressions, type_size: &TypeSize) -> TypeSize {
        let ts = match type_size {
            TypeSize::LiteralInteger(_) => Some(*type_size),
            TypeSize::ConstExpression(id) => import_from
                .clone(id)
                .map(|(expr, target_type, scope)| {
                    self.get_mut_const_expressions().add_constant_expression(expr, target_type, scope)
                })
                .map(TypeSize::from_expression),
            TypeSize::Undetermined => Some(*type_size),
        };

        match ts {
            Some(it) => it,
            None => {
                unreachable!(
                    "requested type-size is not part of the given ConstExpressions. Cannot import '{:?}', from {:?}",
                    type_size, import_from
                );
            }
        }
    }

    /// returns the void-type
    /// the NULL-statement has a type of void.
    /// void cannot be casted to any other
    pub fn get_void_type(&self) -> &DataType {
        &self.type_index.void_type
    }

    /// returns the `VariableIndexEntry` of the global variable with the given name
    pub fn find_qualified_global_variable(
        &self,
        context: Option<&str>,
        name: &str,
    ) -> Option<&VariableIndexEntry> {
        self.global_variables
            .get_all(&name.to_lowercase())
            .or_else(|| self.enum_global_variables.get_all(&name.to_lowercase()))
            .and_then(|it| {
                if let Some(context) = context.filter(|it| !it.is_empty()) {
                    it.iter().find(|it| it.has_parent(context)).or_else(|| it.first())
                } else {
                    it.first()
                }
            })
    }

    /// returns the `VariableIndexEntry` of the global variable with the given name
    pub fn find_global_variable(&self, name: &str) -> Option<&VariableIndexEntry> {
        self.find_qualified_global_variable(None, name)
    }

    /// returns the `VariableIndexEntry` of the global initializer with the given name
    pub fn find_global_initializer(&self, name: &str) -> Option<&VariableIndexEntry> {
        self.global_initializers.get(&name.to_lowercase())
    }

    /// return the `VariableIndexEntry` with the qualified name: `container_name`.`variable_name`
    pub fn find_local_member(
        &self,
        container_name: &str,
        variable_name: &str,
    ) -> Option<&VariableIndexEntry> {
        self.type_index
            .find_type(container_name)
            .and_then(|it| it.find_member(variable_name))
            .or(self.find_enum_variant_in_pou(container_name, variable_name))
            .or(container_name
                .rfind('.')
                .map(|p| &container_name[..p])
                .and_then(|qualifier| self.find_member(qualifier, variable_name)))
    }

    /// Searches for variable name in the given container, if not found, attempts to search for it in super classes
    pub fn find_member(&self, container_name: &str, variable_name: &str) -> Option<&VariableIndexEntry> {
        // Find pou in index
        self.find_local_member(container_name, variable_name).or_else(|| {
            if let Some(class) = self.find_pou(container_name).and_then(|it| it.get_super_class()) {
                self.find_member(class, variable_name)
            } else {
                None
            }
        })
    }

    /// Searches for method names in the given container, if not found, attempts to search for it in super class
    pub fn find_method(&self, container_name: &str, method_name: &str) -> Option<&PouIndexEntry> {
        if let Some(local_method) = self.find_pou(&qualified_name(container_name, method_name)) {
            Some(local_method)
        } else if let Some(super_method) = self.find_pou(container_name).and_then(|it| it.get_super_class()) {
            self.find_method(super_method, method_name)
        } else {
            None
        }
    }

    /// return the `VariableIndexEntry` associated with the given fully qualified name using `.` as
    /// a delimiter. (e.g. "PLC_PRG.x", or "MyClass.MyMethod.x")
    pub fn find_fully_qualified_variable(&self, fully_qualified_name: &str) -> Option<&VariableIndexEntry> {
        let segments: Vec<&str> = fully_qualified_name.split('.').collect();
        let (q, segments) = if segments.len() > 1 {
            // the last segment is th ename, everything before ist qualifier
            // e.g. MyClass.MyMethod.x --> qualifier: "MyClass.MyMethod", name: "x"
            (Some(segments.iter().take(segments.len() - 1).join(".")), vec![*segments.last().unwrap()])
        } else {
            (None, segments)
        };
        self.find_variable(q.as_deref(), &segments[..])
    }

    // TODO: `node` could be a trait, "NodeName"?
    pub fn find_variable_ast(&self, context: Option<&str>, node: &AstNode) -> Option<&VariableIndexEntry> {
        if let Some(name) = node.get_flat_reference_name() {
            return self.find_variable(context, &[name]);
        }

        None
    }

    pub fn find_variable(&self, context: Option<&str>, segments: &[&str]) -> Option<&VariableIndexEntry> {
        if segments.is_empty() {
            return None;
        }

        //For the first element, if the context does not contain that element, it is possible that the element is also a global variable
        let init = match context {
            Some(context) => self
                .find_member(context, segments[0])
                .or_else(|| self.find_qualified_global_variable(Some(context), segments[0])),
            None => self.find_global_variable(segments[0]),
        };

        segments.iter().skip(1).fold(init, |context, current| {
            context.and_then(|context| self.find_member(&context.data_type_name, current))
        })
    }

    /// Returns the index entry of the enum variant or [`None`] if it does not exist.
    pub fn find_enum_variant(&self, name: &str, variant: &str) -> Option<&VariableIndexEntry> {
        self.type_index.find_type(name)?.find_member(variant)
    }

    /// Returns the index entry of the enum variant by its qualified name or [`None`] if it does not exist.
    pub fn find_enum_variant_by_qualified_name(&self, qualified_name: &str) -> Option<&VariableIndexEntry> {
        let (name, variant) = qualified_name.split('.').next_tuple()?;
        self.find_enum_variant(name, variant)
    }

    /// Returns all enum variants of the given variable.
    pub fn get_enum_variants_by_variable(&self, variable: &VariableIndexEntry) -> Vec<&VariableIndexEntry> {
        let Some(var) = self.type_index.find_type(&variable.data_type_name) else { return vec![] };
        let DataTypeInformation::Enum { variants, .. } = var.get_type_information() else { return vec![] };
        variants.iter().collect()
    }

    /// Tries to return an enum variant defined within a POU
    pub fn find_enum_variant_in_pou(&self, pou: &str, variant: &str) -> Option<&VariableIndexEntry> {
        self.get_enum_variants_in_pou(pou)
            .into_iter()
            .find(|it| it.name == variant)
            .or(self.find_enum_variant(pou, variant))
    }

    /// Returns all enum variants defined in the given POU
    pub fn get_enum_variants_in_pou(&self, pou: &str) -> Vec<&VariableIndexEntry> {
        let mut hs: FxHashSet<&VariableIndexEntry> = FxHashSet::default();
        for member in self.get_pou_members(pou) {
            if self.type_index.find_type(member.get_type_name()).is_some_and(|it| it.is_enum()) {
                hs.insert(member);
            }
        }

        hs.iter().flat_map(|variable| self.get_enum_variants_by_variable(variable)).collect()
    }

    /// returns all member variables of the given container (e.g. FUNCTION, PROGRAM, STRUCT, etc.)
    pub fn get_container_members(&self, container_name: &str) -> &[VariableIndexEntry] {
        self.type_index.find_type(container_name).map(|it| it.get_members()).unwrap_or_else(|| &[])
    }

    /// returns all member variables of the given POU (e.g. FUNCTION, PROGRAM, etc.)
    pub fn get_pou_members(&self, container_name: &str) -> &[VariableIndexEntry] {
        self.get_pou_types()
            .get(&container_name.to_lowercase())
            .map(|it| it.get_members())
            .unwrap_or_else(|| &[])
    }

    pub fn find_pou_type(&self, pou_name: &str) -> Option<&DataType> {
        self.get_pou_types().get(&pou_name.to_lowercase())
    }

    pub fn get_declared_parameters(&self, pou_name: &str) -> Vec<&VariableIndexEntry> {
        self.get_pou_members(pou_name)
            .iter()
            .filter(|it| it.is_parameter() && !it.is_variadic())
            .collect::<Vec<_>>()
    }

    pub fn has_variadic_parameter(&self, pou_name: &str) -> bool {
        self.get_pou_members(pou_name).iter().any(|member| member.is_parameter() && member.is_variadic())
    }

    /// returns some if the current index is a VAR_INPUT, VAR_IN_OUT or VAR_OUTPUT that is not a variadic argument
    /// In other words it returns some if the member variable at `index` of the given container is a possible parameter in
    /// the call to it
    pub fn get_declared_parameter(&self, pou_name: &str, index: u32) -> Option<&VariableIndexEntry> {
        self.type_index.find_pou_type(pou_name).and_then(|it| it.find_declared_parameter_by_location(index))
    }

    pub fn get_variadic_member(&self, pou_name: &str) -> Option<&VariableIndexEntry> {
        self.type_index.find_pou_type(pou_name).and_then(|it| it.find_variadic_member())
    }

    pub fn find_input_parameter(&self, pou_name: &str, index: u32) -> Option<&VariableIndexEntry> {
        self.get_pou_members(pou_name)
            .iter()
            .filter(|item| item.get_variable_type() == VariableType::Input)
            .find(|item| item.location_in_parent == index)
    }

    pub fn find_parameter(&self, pou_name: &str, index: u32) -> Option<&VariableIndexEntry> {
        self.get_pou_members(pou_name).iter().find(|item| item.location_in_parent == index)
    }

    /// returns the effective DataType of the type with the given name if it exists
    pub fn find_effective_type_by_name(&self, type_name: &str) -> Option<&DataType> {
        self.type_index.find_effective_type_by_name(type_name)
    }

    /// returns the effective DataType of the given type if it exists
    pub fn find_effective_type<'ret>(&'ret self, data_type: &'ret DataType) -> Option<&'ret DataType> {
        self.type_index.find_effective_type(data_type)
    }

    /// returns the effective DataType of the type with the given name or an Error
    pub fn get_effective_type_by_name(&self, type_name: &str) -> Result<&DataType, Diagnostic> {
        self.type_index
            .find_effective_type_by_name(type_name)
            .ok_or_else(|| Diagnostic::unknown_type(type_name, SourceLocation::undefined()))
    }

    /// returns the effective DataTypeInformation of the type with the given name if it exists
    pub fn find_effective_type_info(&self, type_name: &str) -> Option<&DataTypeInformation> {
        self.find_effective_type_by_name(type_name).map(DataType::get_type_information)
    }

    /// returns the effective type of the type with the with the given name or the
    /// void-type if the given name does not exist
    pub fn get_effective_type_or_void_by_name(&self, type_name: &str) -> &DataType {
        self.type_index.get_effective_type_by_name(type_name)
    }

    /// returns the intrinsic type of the type with the given name or the
    /// void-type if the given name does not exist
    /// returns the real type behind aliases and subRanges (while effective_types will only
    /// resolve aliases)
    pub fn get_intrinsic_type_by_name(&self, type_name: &str) -> &DataType {
        let effective_type = self.type_index.get_effective_type_by_name(type_name);

        match effective_type.get_type_information() {
            DataTypeInformation::SubRange { referenced_type, .. } => {
                self.get_intrinsic_type_by_name(referenced_type.as_str())
            }
            DataTypeInformation::Enum { referenced_type, .. } => {
                self.get_intrinsic_type_by_name(referenced_type)
            }
            _ => effective_type,
        }
    }

    pub fn get_type(&self, type_name: &str) -> Result<&DataType, Diagnostic> {
        self.type_index.get_type(type_name)
    }

    pub fn find_type(&self, type_name: &str) -> Option<&DataType> {
        self.type_index.find_type(type_name)
    }

    /// expect a built-in type
    /// This only returns types, not POUs as it is meant for builtins only
    pub fn get_type_or_panic(&self, type_name: &str) -> &DataType {
        self.get_types().get(&type_name.to_lowercase()).unwrap_or_else(|| panic!("{type_name} not found"))
    }

    pub fn get_initial_value(&self, id: &Option<ConstId>) -> Option<&AstNode> {
        self.get_const_expressions().maybe_get_constant_statement(id)
    }

    /// returns type aliased by Alias or SubRange
    fn get_aliased_target_type(&self, dt: &DataTypeInformation) -> Option<&DataType> {
        match dt {
            DataTypeInformation::SubRange { referenced_type, .. }
            | DataTypeInformation::Alias { referenced_type, .. } => {
                self.type_index.find_type(referenced_type)
            }
            _ => None,
        }
    }

    /// Returns the initioal value registered for the given data_type.
    /// If the given dataType has no initial value AND it is an Alias or SubRange (referencing another type)
    /// this method tries to obtain the default value from the referenced type.
    pub fn get_initial_value_for_type(&self, type_name: &str) -> Option<&AstNode> {
        let mut dt = self.type_index.find_type(type_name);
        let mut initial_value = dt.and_then(|it| it.initial_value);

        //check if we have no initial value AND this type is an alias to another type
        while initial_value.is_none()
            && matches!(
                dt.map(|it| &it.information),
                Some(DataTypeInformation::Alias { .. } | DataTypeInformation::SubRange { .. })
            )
        {
            //try to fetch initial value of the aliased type
            dt = dt.and_then(|it| self.get_aliased_target_type(&it.information));
            initial_value = dt.and_then(|it| it.initial_value);
        }
        self.get_initial_value(&initial_value)
    }

    pub fn find_return_variable(&self, pou_name: &str) -> Option<&VariableIndexEntry> {
        self.get_pou_types().get(&pou_name.to_lowercase()).and_then(|it| it.find_return_variable())
    }

    pub fn find_return_type(&self, pou_name: &str) -> Option<&DataType> {
        let variable = self.find_return_variable(pou_name);
        variable.and_then(|it| self.get_type(it.get_type_name()).ok())
    }

    pub fn get_type_information_or_void(&self, type_name: &str) -> &DataTypeInformation {
        self.find_effective_type_by_name(type_name)
            .map(|it| it.get_type_information())
            .unwrap_or_else(|| self.get_void_type().get_type_information())
    }

    /// Returns the map of types, should not be used to search for types --> see find_type
    pub fn get_types(&self) -> &SymbolMap<String, DataType> {
        &self.type_index.types
    }

    /// Returns the map of pou_types, should not be used to search for pou_types -->  see find_pou_type
    pub fn get_pou_types(&self) -> &SymbolMap<String, DataType> {
        &self.type_index.pou_types
    }

    /// Returns the map of globals, should not be used to search for globals -->  see find_global_variable
    pub fn get_globals(&self) -> &SymbolMap<String, VariableIndexEntry> {
        &self.global_variables
    }

    pub fn get_program_instances(&self) -> Vec<&VariableIndexEntry> {
        self.pous
            .values()
            .filter_map(|p| match p {
                PouIndexEntry::Program { instance_variable, .. } => Some(instance_variable.as_ref()),
                _ => None,
            })
            .collect()
    }

    /// Returns the map of pous, should not be used to search for pous -->  see find_pou
    pub fn get_pous(&self) -> &SymbolMap<String, PouIndexEntry> {
        &self.pous
    }

    pub fn get_global_initializers(&self) -> &SymbolMap<String, VariableIndexEntry> {
        &self.global_initializers
    }

    pub fn get_all_enum_variants(&self) -> Vec<&VariableIndexEntry> {
        self.enum_global_variables.values().collect()
    }

    pub fn get_implementations(&self) -> &FxIndexMap<String, ImplementationIndexEntry> {
        &self.implementations
    }

    pub fn register_implementation(
        &mut self,
        call_name: &str,
        type_name: &str,
        associated_class_name: Option<&String>,
        impl_type: ImplementationType,
        generic: bool,
        location: SourceLocation,
    ) {
        self.implementations.insert(
            call_name.to_lowercase(),
            ImplementationIndexEntry {
                call_name: call_name.into(),
                type_name: type_name.into(),
                associated_class: associated_class_name.map(|str| str.into()),
                implementation_type: impl_type,
                generic,
                location,
            },
        );
    }

    pub fn find_pou(&self, pou_name: &str) -> Option<&PouIndexEntry> {
        self.pous.get(&pou_name.to_lowercase())
    }

    pub fn register_program(&mut self, name: &str, location: SourceLocation, linkage: LinkageType) {
        let instance_variable =
            VariableIndexEntry::create_global(&format!("{}_instance", &name), name, name, location.clone()) // TODO: Naming convention (see plc_util/src/convention.rs)
                .set_linkage(linkage);
        // self.register_global_variable(name, instance_variable.clone());
        let entry = PouIndexEntry::create_program_entry(name, instance_variable, linkage, location);
        self.pous.insert(entry.get_name().to_lowercase(), entry);
    }

    pub fn register_pou(&mut self, entry: PouIndexEntry) {
        self.pous.insert(entry.get_name().to_lowercase(), entry);
    }

    pub fn find_implementation_by_name(&self, call_name: &str) -> Option<&ImplementationIndexEntry> {
        self.implementations.get(&call_name.to_lowercase())
    }

    pub fn find_pou_implementation(&self, pou_name: &str) -> Option<&ImplementationIndexEntry> {
        self.find_pou(pou_name).and_then(|it| it.find_implementation(self))
    }

    /// creates a member-variable of a container to be accessed in a qualified name.
    /// e.g. "POU.member", "StructName.member", etc.
    ///
    /// #Arguments
    /// * `container_name`- the name of hosting container (pou or struct)
    /// * `variable_name` - the name of the member variable
    /// * `variable_linkage` - the linkage-type of that variable (one of local, global, etc. )
    /// * `variable_type_name` - the variable's data type as a string
    /// * `initial_value` - the initial value as defined in the AST
    /// * `location` - the location (index) inside the container
    pub fn register_member_variable(
        &mut self,
        member_info: MemberInfo,
        initial_value: Option<ConstId>,
        source_location: SourceLocation,
        location: u32,
    ) -> VariableIndexEntry {
        let container_name = member_info.container_name;
        let variable_name = member_info.variable_name;
        let variable_type = member_info.variable_linkage;
        let data_type_name = member_info.variable_type_name;

        let qualified_name = qualified_name(container_name, variable_name);

        VariableIndexEntry::new(
            variable_name,
            &qualified_name,
            data_type_name,
            variable_type,
            location,
            source_location,
        )
        .set_constant(member_info.is_constant)
        .set_initial_value(initial_value)
        .set_hardware_binding(member_info.binding)
        .set_varargs(member_info.varargs)
    }

    pub fn register_enum_variant(
        &mut self,
        name: &str,
        variant: &str,
        initial_value: Option<ConstId>,
        source_location: SourceLocation,
    ) -> VariableIndexEntry {
        let qualified_name = qualified_name(name, variant);
        let entry = VariableIndexEntry::create_global(variant, &qualified_name, name, source_location)
            .set_constant(true)
            .set_initial_value(initial_value);

        self.enum_global_variables.insert(variant.to_lowercase(), entry.clone());
        entry
    }

    pub fn register_global_variable(&mut self, name: &str, variable: VariableIndexEntry) {
        self.global_variables.insert(name.to_lowercase(), variable);
    }

    pub fn register_global_initializer(&mut self, name: &str, variable: VariableIndexEntry) {
        self.global_initializers.insert(name.to_lowercase(), variable);
    }

    pub fn register_type(&mut self, datatype: DataType) {
        self.type_index.types.insert(datatype.get_name().to_lowercase(), datatype);
    }

    pub fn register_pou_type(&mut self, datatype: DataType) {
        self.type_index.pou_types.insert(datatype.get_name().to_lowercase(), datatype);
    }

    pub fn find_callable_instance_variable(
        &self,
        context: Option<&str>,
        reference: &[&str],
    ) -> Option<&VariableIndexEntry> {
        //look for a *callable* variable with that name
        self.find_variable(context, reference).filter(|v| {
            //callable means, there is an implementation associated with the variable's datatype
            self.find_implementation_by_name(&v.data_type_name).is_some()
        })
    }

    /// returns the mutable reference to all registered ConstExpressions
    pub fn get_mut_const_expressions(&mut self) -> &mut ConstExpressions {
        &mut self.constant_expressions
    }

    /// returns all registered ConstExpressions
    pub fn get_const_expressions(&self) -> &ConstExpressions {
        &self.constant_expressions
    }

    /// returns the intrinsic (built-in) type represented by the given type-information
    /// this will return the built-in type behind alias / range-types
    pub fn find_intrinsic_type<'idx>(
        &'idx self,
        initial_type: &'idx DataTypeInformation,
    ) -> &'idx DataTypeInformation {
        match initial_type {
            DataTypeInformation::SubRange { .. } | DataTypeInformation::Alias { .. } => {
                let inner_type_name = match initial_type {
                    DataTypeInformation::SubRange { referenced_type, .. } => referenced_type,
                    _ => initial_type.get_name(),
                };
                if let Some(inner_type) = self.find_effective_type_info(inner_type_name) {
                    self.find_intrinsic_type(inner_type)
                } else {
                    initial_type
                }
            }
            DataTypeInformation::Enum { referenced_type, .. } => {
                self.find_effective_type_info(referenced_type).unwrap_or(initial_type)
            }
            _ => initial_type,
        }
    }

    pub fn find_elementary_pointer_type<'idx>(
        &'idx self,
        initial_type: &'idx DataTypeInformation,
    ) -> &'idx DataTypeInformation {
        match initial_type {
            DataTypeInformation::Pointer { inner_type_name, .. } => {
                let inner_type = self
                    .find_effective_type_info(inner_type_name)
                    .map(|t| self.find_intrinsic_type(t))
                    .unwrap_or_else(|| initial_type);
                self.find_elementary_pointer_type(inner_type)
            }
            _ => initial_type,
        }
    }

    /// Creates an iterator over all instances in the index
    pub fn find_instances(&self) -> InstanceIterator {
        InstanceIterator::new(self)
    }

    /// Creates an iterator over all instances in the index
    /// The passed filter will stop the iterator from navigating deeper into variables that should not be considered
    /// To filter a variable from showing up at all, (not only its children), add a filter to the returned iterator
    pub fn filter_instances(
        &self,
        inner_filter: fn(&VariableIndexEntry, &Index) -> bool,
    ) -> InstanceIterator {
        InstanceIterator::with_filter(self, inner_filter)
    }

    /// If the provided name is a builtin function, returns it from the builtin index
    pub fn get_builtin_function(&self, name: &str) -> Option<&'_ BuiltIn> {
        //Find a type for that function, see if that type is builtin
        if let Some(PouIndexEntry::Function { linkage: LinkageType::BuiltIn, .. }) = self.find_pou(name) {
            builtins::get_builtin(name)
        } else {
            None
        }
    }

    pub fn get_type_layout(&self) -> &DataLayout {
        &self.data_layout
    }

    /// returns the implementation of the sub-range-check-function for a variable of the given dataType
    pub fn find_range_check_implementation_for(
        &self,
        range_type: &DataTypeInformation,
    ) -> Option<&ImplementationIndexEntry> {
        match range_type {
            DataTypeInformation::Integer { signed, size, .. } if *signed && *size <= 32 => {
                self.find_pou_implementation(RANGE_CHECK_S_FN)
            }
            DataTypeInformation::Integer { signed, size, .. } if *signed && *size > 32 => {
                self.find_pou_implementation(RANGE_CHECK_LS_FN)
            }
            DataTypeInformation::Integer { signed, size, .. } if !*signed && *size <= 32 => {
                self.find_pou_implementation(RANGE_CHECK_U_FN)
            }
            DataTypeInformation::Integer { signed, size, .. } if !*signed && *size > 32 => {
                self.find_pou_implementation(RANGE_CHECK_LU_FN)
            }
            DataTypeInformation::Alias { name, .. }
            | DataTypeInformation::SubRange { referenced_type: name, .. } => {
                //traverse to the primitive type
                self.find_effective_type_info(name)
                    .and_then(|info| self.find_range_check_implementation_for(info))
            }
            _ => None,
        }
    }

    /// Adds a label definition for the POU
    pub fn add_label(&mut self, pou_name: &str, label: Label) {
        let labels = self.labels.entry(pou_name.to_string()).or_default();
        labels.insert(label.name.clone(), label);
    }

    pub fn get_label(&self, pou_name: &str, label_name: &str) -> Option<&Label> {
        self.labels.get(pou_name).and_then(|it| it.get(label_name))
    }

    pub fn get_labels(&self, pou_name: &str) -> Option<&SymbolMap<String, Label>> {
        self.labels.get(pou_name)
    }
}

/// Returns a default initialization name for a variable or type
pub fn get_initializer_name(name: &str) -> String {
    format!("__{name}__init")
}
