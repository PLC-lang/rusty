// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::{
    ast::{
        AstStatement, DirectAccessType, GenericBinding, HardwareAccessType, LinkageType, PouType,
        SourceRange, TypeNature,
    },
    builtins::{self, BuiltIn},
    datalayout::DataLayout,
    diagnostics::Diagnostic,
    typesystem::{self, *},
};
use indexmap::IndexMap;

use self::{
    const_expressions::{ConstExpressions, ConstId},
    instance_iterator::InstanceIterator,
    symbol::{SymbolLocation, SymbolMap},
};

pub mod const_expressions;
mod instance_iterator;
pub mod symbol;
#[cfg(test)]
mod tests;
pub mod visitor;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableIndexEntry {
    /// the name of this variable (e.g. 'x' for 'PLC_PRG.x')
    name: String,
    /// the fully qualified name of this variable (e.g. PLC_PRG.x)
    qualified_name: String,
    /// an optional initial value of this variable
    pub initial_value: Option<ConstId>,
    /// the type of variable
    pub variable_type: ArgumentType,
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
    pub source_location: SymbolLocation,
    /// Variadic information placeholder for the variable, if any
    varargs: Option<VarArgs>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HardwareBinding {
    /// Specifies if the binding is an In/Out or Memory binding
    pub direction: HardwareAccessType,
    /// The datatype (size) of the binding
    pub access: DirectAccessType,
    /// A list of entries that form this binding
    pub entries: Vec<ConstId>,
    /// The location in the original source-file
    pub location: SourceRange,
}

impl HardwareBinding {
    fn from_statement(index: &mut Index, it: &AstStatement, scope: Option<String>) -> Option<Self> {
        if let AstStatement::HardwareAccess {
            access,
            address,
            direction,
            location,
            ..
        } = it
        {
            Some(HardwareBinding {
                access: *access,
                direction: *direction,
                entries: address
                    .iter()
                    .map(|expr| {
                        index.constant_expressions.add_constant_expression(
                            expr.clone(),
                            typesystem::DINT_SIZE.to_string(),
                            scope.clone(),
                        )
                    })
                    .collect(),
                location: location.clone(),
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
        variable_type: ArgumentType,
        location_in_parent: u32,
        source_location: SymbolLocation,
    ) -> Self {
        VariableIndexEntry {
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            initial_value: None,
            variable_type,
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
        source_location: SymbolLocation,
    ) -> Self {
        VariableIndexEntry {
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            initial_value: None,
            variable_type: ArgumentType::ByVal(VariableType::Global),
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
        let name = if self.is_return() {
            container
        } else {
            &self.name
        };
        let varargs = self
            .varargs
            .as_ref()
            .map(|varargs| varargs.as_typed(new_type));

        VariableIndexEntry {
            name: name.to_string(),
            qualified_name: format!("{}.{}", container, name),
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

    pub fn get_variable_type(&self) -> VariableType {
        self.variable_type.get_variable_type()
    }

    pub fn get_declaration_type(&self) -> ArgumentType {
        self.variable_type
    }

    pub fn has_hardware_binding(&self) -> bool {
        self.binding.is_some()
    }

    pub fn get_hardware_binding(&self) -> Option<&HardwareBinding> {
        self.binding.as_ref()
    }

    pub fn is_parameter(&self) -> bool {
        matches!(
            self.get_variable_type(),
            VariableType::Input | VariableType::Output | VariableType::InOut
        )
    }

    pub fn is_variadic(&self) -> bool {
        self.varargs.is_some()
    }

    pub fn get_varargs(&self) -> Option<&VarArgs> {
        self.varargs.as_ref()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArgumentType {
    ByVal(VariableType),
    ByRef(VariableType),
}

impl ArgumentType {
    pub fn get_variable_type(&self) -> VariableType {
        match self {
            ArgumentType::ByVal(t) => *t,
            ArgumentType::ByRef(t) => *t,
        }
    }

    pub fn is_by_ref(&self) -> bool {
        matches!(self, ArgumentType::ByRef(..))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VariableType {
    Local, // functions have no locals; others: VAR-block
    Temp,  // for functions: VAR & VAR_TEMP; others: VAR_TEMP
    Input,
    Output,
    InOut,
    Global,
    Return,
}

/// information regarding a variable
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableInformation {
    /// the type of variable
    variable_type: VariableType,
    /// true if this variable is a compile-time-constant
    is_constant: bool,
    /// the variable's datatype
    data_type_name: String,
    /// the variable's qualifier, None for global variables
    qualifier: Option<String>,
    /// Location in the qualifier defautls to 0 (Single variables)
    location_in_parent: u32,
}

#[derive(Debug)]
pub enum DataTypeType {
    Scalar,        // built in types: INT, BOOL, WORD, ...
    Struct,        // Struct-DataType
    FunctionBlock, // a Functionblock instance
    AliasType,     // a Custom-Alias-dataType
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
    pub(crate) location: SymbolLocation,
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

    pub fn get_location(&self) -> &SymbolLocation {
        &self.location
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
        instance_variable: VariableIndexEntry,
        linkage: LinkageType,
        location: SymbolLocation,
    },
    FunctionBlock {
        name: String,
        instance_struct_name: String,
        linkage: LinkageType,
        location: SymbolLocation,
    },
    Function {
        name: String,
        return_type: String,
        instance_struct_name: String,
        generics: Vec<GenericBinding>,
        linkage: LinkageType,
        is_variadic: bool,
        location: SymbolLocation,
        is_generated: bool, // true if this entry was added automatically (e.g. by generics)
    },
    Class {
        name: String,
        instance_struct_name: String,
        linkage: LinkageType,
        location: SymbolLocation,
    },
    Method {
        name: String,
        parent_pou_name: String,
        return_type: String,
        instance_struct_name: String,
        linkage: LinkageType,
        location: SymbolLocation,
    },
    Action {
        name: String,
        parent_pou_name: String,
        instance_struct_name: String,
        linkage: LinkageType,
        location: SymbolLocation,
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
        location: SymbolLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::Program {
            name: pou_name.into(),
            instance_struct_name: pou_name.into(),
            instance_variable,
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
        location: SymbolLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::FunctionBlock {
            name: pou_name.into(),
            instance_struct_name: pou_name.into(),
            linkage,
            location,
        }
    }

    /// creates a new Function-PouIndexEntry
    pub fn create_function_entry(
        name: &str,
        return_type: &str,
        generic_names: &[GenericBinding],
        linkage: LinkageType,
        is_variadic: bool,
        location: SymbolLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::Function {
            name: name.into(),
            generics: generic_names.to_vec(),
            return_type: return_type.into(),
            instance_struct_name: name.into(),
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
        location: SymbolLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::Function {
            name: name.into(),
            generics: generic_names.to_vec(),
            return_type: return_type.into(),
            instance_struct_name: name.into(),
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
        location: SymbolLocation,
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
        location: SymbolLocation,
    ) -> PouIndexEntry {
        PouIndexEntry::Class {
            name: pou_name.into(),
            instance_struct_name: pou_name.into(),
            linkage,
            location,
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
        location: SymbolLocation,
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

    /// returns the name of the struct-type used to store the POUs state
    /// (interface-variables)
    pub fn get_instance_struct_type_name(&self) -> Option<&str> {
        match self {
            PouIndexEntry::Program {
                instance_struct_name,
                ..
            }
            | PouIndexEntry::FunctionBlock {
                instance_struct_name,
                ..
            }
            | PouIndexEntry::Method {
                instance_struct_name,
                ..
            }
            | PouIndexEntry::Action {
                instance_struct_name,
                ..
            }
            | PouIndexEntry::Class {
                instance_struct_name,
                ..
            } => Some(instance_struct_name.as_str()),
            _ => None, //functions have no struct type
        }
    }

    /// returns `Some(DataType)` associated with this pou or `Some` if none is associated
    ///
    /// - `index` the index to fetch te DataType from
    pub fn find_instance_struct_type<'idx>(&self, index: &'idx Index) -> Option<&'idx DataType> {
        self.get_instance_struct_type_name()
            .and_then(|it| index.type_index.find_pou_type(it))
    }

    /// returns the struct-datatype associated with this pou or `void` if none is associated
    ///
    /// - `index` the index to fetch te DataType from
    pub fn get_instance_struct_type<'idx>(&self, index: &'idx Index) -> &'idx DataType {
        self.find_instance_struct_type(index)
            .unwrap_or_else(|| index.get_void_type())
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
            PouIndexEntry::Action {
                parent_pou_name, ..
            }
            | PouIndexEntry::Method {
                parent_pou_name, ..
            } => parent_pou_name.as_str(),
        }
    }

    /// returns the ImplementationIndexEntry associated with this POU
    pub fn find_implementation<'idx>(
        &self,
        index: &'idx Index,
    ) -> Option<&'idx ImplementationIndexEntry> {
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

    pub fn is_function_block(&self) -> bool {
        matches!(self, PouIndexEntry::FunctionBlock { .. })
    }

    pub fn is_class(&self) -> bool {
        matches!(self, PouIndexEntry::Class { .. })
    }

    pub(crate) fn is_method(&self) -> bool {
        matches!(self, PouIndexEntry::Method { .. })
    }

    pub fn get_location(&self) -> &SymbolLocation {
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
        matches!(
            self,
            PouIndexEntry::Function {
                is_generated: true,
                ..
            }
        )
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
                location: SymbolLocation::internal(),
            },
        }
    }
}

impl TypeIndex {
    pub fn find_type(&self, type_name: &str) -> Option<&DataType> {
        self.types
            .get(&type_name.to_lowercase())
            .or_else(|| self.find_pou_type(type_name))
    }

    pub fn find_pou_type(&self, type_name: &str) -> Option<&DataType> {
        self.pou_types.get(&type_name.to_lowercase())
    }

    pub fn find_effective_type_by_name(&self, type_name: &str) -> Option<&DataType> {
        self.find_type(type_name)
            .and_then(|it| self.find_effective_type(it))
    }

    pub fn get_effective_type_by_name(&self, type_name: &str) -> &DataType {
        self.find_type(type_name)
            .and_then(|it| self.find_effective_type(it))
            .unwrap_or(&self.void_type)
    }

    pub fn get_type(&self, type_name: &str) -> Result<&DataType, Diagnostic> {
        self.find_type(type_name)
            .ok_or_else(|| Diagnostic::unknown_type(type_name, SourceRange::undefined()))
    }

    /// Retrieves the "Effective" type behind this datatype
    /// An effective type will be any end type i.e. Structs, Integers, Floats, String and Array
    pub fn find_effective_type<'ret>(
        &'ret self,
        data_type: &'ret DataType,
    ) -> Option<&'ret DataType> {
        match data_type.get_type_information() {
            DataTypeInformation::Alias {
                referenced_type, ..
            } => self
                .find_type(referenced_type)
                .and_then(|it| self.find_effective_type(it)),
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
    global_variables: SymbolMap<String, VariableIndexEntry>, // IndexMap<String, Vec<VariableIndexEntry>>,

    /// all struct initializers
    global_initializers: SymbolMap<String, VariableIndexEntry>,

    /// all enum-members with their names
    enum_global_variables: SymbolMap<String, VariableIndexEntry>,

    /// all enum-members with their qualified names <enum-type>.<element-name>
    enum_qualified_variables: SymbolMap<String, VariableIndexEntry>,

    /// all local variables, grouped by the POU's name
    member_variables: IndexMap<String, SymbolMap<String, VariableIndexEntry>>,

    // all pous,
    pous: SymbolMap<String, PouIndexEntry>,

    /// all implementations
    // we keep an IndexMap for implementations since duplication issues regarding implementations
    // is handled by the `pous` SymbolMap
    implementations: IndexMap<String, ImplementationIndexEntry>,

    /// an index with all type-information
    type_index: TypeIndex,

    constant_expressions: ConstExpressions,

    /// Type layout for the target
    data_layout: DataLayout,
}

impl Index {
    /// imports all entries from the given index into the current index
    ///
    /// imports all global_variables, member_variables, types and implementations
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

        //enmu_variables use the qualified variables since name conflicts will be overriden in the enum_global
        for (qualified_name, elements) in other.enum_qualified_variables.drain(..) {
            let elements = elements
                .into_iter()
                .map(|e| self.transfer_constants(e, &mut other.constant_expressions))
                .collect::<Vec<_>>();

            for e in elements.iter() {
                self.enum_global_variables
                    .insert(e.get_name().to_lowercase(), e.clone());
            }

            self.enum_qualified_variables
                .insert_many(qualified_name, elements);
        }

        //initializers
        for (name, elements) in other.global_initializers.drain(..) {
            let elements = elements
                .into_iter()
                .map(|e| self.transfer_constants(e, &mut other.constant_expressions))
                .collect::<Vec<_>>();
            self.global_initializers.insert_many(name, elements);
        }

        //member variables
        for (name, mut members) in other.member_variables.drain(..) {
            //enum qualified variables
            let mut new_members = SymbolMap::default();
            for (name, entries) in members.drain(..) {
                let entries = entries
                    .into_iter()
                    .map(|e| self.transfer_constants(e, &mut other.constant_expressions))
                    .collect::<Vec<_>>();
                new_members.insert_many(name, entries);
            }
            self.member_variables.insert(name, new_members);
        }

        //types
        for (name, mut elements) in other.type_index.types.drain(..) {
            for mut e in elements.drain(..) {
                //avoid re-importing internally auto-generated types that we already know
                if !e.is_internal()
                    || self
                        .type_index
                        .find_effective_type_by_name(name.as_str())
                        .is_none()
                {
                    e.initial_value = self
                        .maybe_import_const_expr(&mut other.constant_expressions, &e.initial_value);

                    match &mut e.information {
                        //import constant expressions in array-type-definitions
                        DataTypeInformation::Array { dimensions, .. } => {
                            for d in dimensions.iter_mut() {
                                d.start_offset = self.import_type_size(
                                    &mut other.constant_expressions,
                                    &d.start_offset,
                                );
                                d.end_offset = self.import_type_size(
                                    &mut other.constant_expressions,
                                    &d.end_offset,
                                );
                            }
                        }
                        // import constant expressions in String-size defintions
                        DataTypeInformation::String { size, .. } => {
                            *size = self.import_type_size(&mut other.constant_expressions, size);
                        }
                        _ => {}
                    }

                    self.type_index.types.insert(name.clone(), e)
                }
            }
        }

        //pou_types
        for (name, elements) in other.type_index.pou_types.drain(..) {
            elements.iter().for_each(|e| {
                self.maybe_import_const_expr(&mut other.constant_expressions, &e.initial_value);
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

        //Constant expressions are intentionally not imported
        // self.constant_expressions.import(other.constant_expressions)
    }

    fn transfer_constants(
        &mut self,
        mut variable: VariableIndexEntry,
        import_from: &mut ConstExpressions,
    ) -> VariableIndexEntry {
        variable.initial_value = self.maybe_import_const_expr(import_from, &variable.initial_value);

        let binding = if let Some(HardwareBinding {
            direction,
            access,
            entries,
            location,
        }) = variable.get_hardware_binding()
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
        initializer_id
            .as_ref()
            .and_then(|it| import_from.clone(it))
            .map(|(init, target_type, scope)| {
                self.get_mut_const_expressions()
                    .add_constant_expression(init, target_type, scope)
            })
    }

    /// imports the corresponding TypeSize (according to the given initializer-id) from the given ConstExpressions
    /// into self's const-expressions and returns the new Id
    ///
    /// panics if the import fails (e.g. the given TypeSize::ConstExpression(id) does not exist in this Index)
    /// this problem would indicate a programming mistake
    fn import_type_size(
        &mut self,
        import_from: &mut ConstExpressions,
        type_size: &TypeSize,
    ) -> TypeSize {
        let ts = match type_size {
            TypeSize::LiteralInteger(_) => Some(*type_size),
            TypeSize::ConstExpression(id) => import_from
                .clone(id)
                .map(|(expr, target_type, scope)| {
                    self.get_mut_const_expressions().add_constant_expression(
                        expr,
                        target_type,
                        scope,
                    )
                })
                .map(TypeSize::from_expression),
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
    pub fn find_global_variable(&self, name: &str) -> Option<&VariableIndexEntry> {
        self.global_variables
            .get(&name.to_lowercase())
            .or_else(|| self.enum_global_variables.get(&name.to_lowercase()))
    }

    /// returns the `VariableIndexEntry` of the global initializer with the given name
    pub fn find_global_initializer(&self, name: &str) -> Option<&VariableIndexEntry> {
        self.global_initializers.get(&name.to_lowercase())
    }

    /// return the `VariableIndexEntry` with the qualified name: `container_name`.`variable_name`
    pub fn find_member(
        &self,
        container_name: &str,
        variable_name: &str,
    ) -> Option<&VariableIndexEntry> {
        self.member_variables
            .get(&container_name.to_lowercase())
            .and_then(|map| map.get(&variable_name.to_lowercase()))
            .or_else(|| {
                //check qualifier
                container_name
                    .rfind('.')
                    .map(|p| &container_name[..p])
                    .and_then(|qualifier| self.find_member(qualifier, variable_name))
            })
    }

    /// return the `VariableIndexEntry` associated with the given fully qualified name using `.` as
    /// a delimiter. (e.g. "PLC_PRG.x", or "MyClass.MyMethod.x")
    pub fn find_fully_qualified_variable(
        &self,
        fully_qualified_name: &str,
    ) -> Option<&VariableIndexEntry> {
        let segments: Vec<&str> = fully_qualified_name.split('.').collect();
        let (q, segments) = if segments.len() > 1 {
            (
                Some(segments[0]),
                segments.iter().skip(1).copied().collect::<Vec<&str>>(),
            )
        } else {
            (None, segments)
        };
        self.find_variable(q, &segments[..])
    }

    pub fn find_variable(
        &self,
        context: Option<&str>,
        segments: &[&str],
    ) -> Option<&VariableIndexEntry> {
        if segments.is_empty() {
            return None;
        }
        //For the first element, if the context does not contain that element, it is possible that the element is also a global variable
        let init = match context {
            Some(context) => self
                .find_member(context, segments[0])
                .or_else(|| self.find_global_variable(segments[0])),
            None => self.find_global_variable(segments[0]),
        };
        segments
            .iter()
            .skip(1)
            .fold(Some((segments[0], init)), |accum, current| match accum {
                Some((_, Some(context))) => Some((
                    *current,
                    self.find_member(&context.data_type_name, *current),
                )),
                // The variable could be in a block that has no global variable (Function block)
                Some((name, None)) => Some((*current, self.find_member(name, *current))),
                None => Some((*current, self.find_global_variable(*current))),
            })
            .and_then(|(_, it)| it)
    }

    /// returns the index entry of the enum-element `element_name` of the enum-type `enum_name`
    /// or None if the requested Enum-Type or -Element does not exist
    pub fn find_enum_element(
        &self,
        enum_name: &str,
        element_name: &str,
    ) -> Option<&VariableIndexEntry> {
        self.enum_qualified_variables
            .get(&format!("{}.{}", enum_name, element_name).to_lowercase())
    }

    /// returns the index entry of the enum-element denoted by the given fully `qualified_name` (e.g. "Color.RED")
    /// or None if the requested Enum-Type or -Element does not exist
    pub fn find_qualified_enum_element(&self, qualified_name: &str) -> Option<&VariableIndexEntry> {
        self.enum_qualified_variables
            .get(&qualified_name.to_lowercase())
    }

    /// returns all member variables of the given container (e.g. FUNCTION, PROGRAM, STRUCT, etc.)
    pub fn get_container_members(&self, container_name: &str) -> Vec<&VariableIndexEntry> {
        self.member_variables
            .get(&container_name.to_lowercase())
            .map(|it| it.values().collect())
            .unwrap_or_else(Vec::new)
    }

    pub fn get_declared_parameters(&self, container_name: &str) -> Vec<&VariableIndexEntry> {
        self.member_variables
            .get(&container_name.to_lowercase())
            .map(|it| {
                it.values()
                    .filter(|it| it.is_parameter() && !it.is_variadic())
            })
            .map(|it| it.collect())
            .unwrap_or_else(Vec::new)
    }

    /// returns some if the current index is a VAR_INPUT, VAR_IN_OUT or VAR_OUTPUT that is not a variadic argument
    /// In other words it returns some if the member variable at `index` of the given container is a possible parameter in
    /// the call to it
    pub fn get_declared_parameter(
        &self,
        container_name: &str,
        index: u32,
    ) -> Option<&VariableIndexEntry> {
        self.member_variables
            .get(&container_name.to_lowercase())
            .and_then(|map| {
                map.values()
                    .filter(|item| item.is_parameter() && !item.is_variadic())
                    .find(|item| item.location_in_parent == index)
            })
    }

    pub fn get_variadic_member(&self, container_name: &str) -> Option<&VariableIndexEntry> {
        self.member_variables
            .get(&container_name.to_lowercase())
            .and_then(|map| map.values().find(|it| it.is_variadic()))
    }

    pub fn find_input_parameter(&self, pou_name: &str, index: u32) -> Option<&VariableIndexEntry> {
        self.member_variables
            .get(&pou_name.to_lowercase())
            .and_then(|map| {
                map.values()
                    .filter(|item| item.get_variable_type() == VariableType::Input)
                    .find(|item| item.location_in_parent == index)
            })
    }

    pub fn find_parameter(&self, pou_name: &str, index: u32) -> Option<&VariableIndexEntry> {
        self.member_variables
            .get(&pou_name.to_lowercase())
            .and_then(|map| map.values().find(|item| item.location_in_parent == index))
    }

    /// returns the effective DataType of the type with the given name if it exists
    pub fn find_effective_type_by_name(&self, type_name: &str) -> Option<&DataType> {
        self.type_index.find_effective_type_by_name(type_name)
    }

    /// returns the effective DataType of the given type if it exists
    pub fn find_effective_type<'ret>(
        &'ret self,
        data_type: &'ret DataType,
    ) -> Option<&'ret DataType> {
        self.type_index.find_effective_type(data_type)
    }

    /// returns the effective DataType of the type with the given name or an Error
    pub fn get_effective_type_by_name(&self, type_name: &str) -> Result<&DataType, Diagnostic> {
        self.type_index
            .find_effective_type_by_name(type_name)
            .ok_or_else(|| Diagnostic::unknown_type(type_name, SourceRange::undefined()))
    }

    /// returns the effective DataTypeInformation of the type with the given name if it exists
    pub fn find_effective_type_info(&self, type_name: &str) -> Option<&DataTypeInformation> {
        self.find_effective_type_by_name(type_name)
            .map(DataType::get_type_information)
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
            DataTypeInformation::SubRange {
                referenced_type, ..
            } => self.get_intrinsic_type_by_name(referenced_type.as_str()),
            DataTypeInformation::Enum {
                referenced_type, ..
            } => self.get_intrinsic_type_by_name(referenced_type),
            _ => effective_type,
        }
    }

    pub fn get_type(&self, type_name: &str) -> Result<&DataType, Diagnostic> {
        self.type_index.get_type(type_name)
    }

    /// expect a built-in type
    pub fn get_type_or_panic(&self, type_name: &str) -> &DataType {
        self.type_index
            .get_type(type_name)
            .unwrap_or_else(|_| panic!("{} not found", type_name))
    }

    pub fn get_initial_value(&self, id: &Option<ConstId>) -> Option<&AstStatement> {
        self.get_const_expressions()
            .maybe_get_constant_statement(id)
    }

    /// returns type aliased by Alias or SubRange    
    fn get_aliased_target_type(&self, dt: &DataTypeInformation) -> Option<&DataType> {
        match dt {
            DataTypeInformation::SubRange {
                referenced_type, ..
            }
            | DataTypeInformation::Alias {
                referenced_type, ..
            } => self.type_index.find_type(referenced_type),
            _ => None,
        }
    }

    /// Returns the initioal value registered for the given data_type.
    /// If the given dataType has no initial value AND it is an Alias or SubRange (referencing another type)
    /// this method tries to obtain the default value from the referenced type.
    pub fn get_initial_value_for_type(&self, type_name: &str) -> Option<&AstStatement> {
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
        let members = self.member_variables.get(&pou_name.to_lowercase());

        members.and_then(|members| {
            members
                .values()
                .find(|it| it.get_variable_type() == VariableType::Return)
        })
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
                PouIndexEntry::Program {
                    instance_variable, ..
                } => Some(instance_variable),
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

    pub fn get_members(&self, name: &str) -> Option<&SymbolMap<String, VariableIndexEntry>> {
        self.member_variables.get(&name.to_lowercase())
    }

    pub fn get_all_members_by_container(
        &self,
    ) -> &IndexMap<String, SymbolMap<String, VariableIndexEntry>> {
        &self.member_variables
    }

    pub fn get_global_qualified_enums(&self) -> &SymbolMap<String, VariableIndexEntry> {
        &self.enum_qualified_variables
    }

    pub fn get_implementations(&self) -> &IndexMap<String, ImplementationIndexEntry> {
        &self.implementations
    }

    pub fn register_implementation(
        &mut self,
        call_name: &str,
        type_name: &str,
        associated_class_name: Option<&String>,
        impl_type: ImplementationType,
        generic: bool,
        location: SymbolLocation,
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

    pub fn register_program(&mut self, name: &str, location: SymbolLocation, linkage: LinkageType) {
        let instance_variable = VariableIndexEntry::create_global(
            &format!("{}_instance", &name),
            name,
            name,
            location.clone(),
        )
        .set_linkage(linkage);
        // self.register_global_variable(name, instance_variable.clone());
        let entry = PouIndexEntry::create_program_entry(name, instance_variable, linkage, location);
        self.pous.insert(entry.get_name().to_lowercase(), entry);
    }

    pub fn register_pou(&mut self, entry: PouIndexEntry) {
        self.pous.insert(entry.get_name().to_lowercase(), entry);
    }

    pub fn find_implementation_by_name(
        &self,
        call_name: &str,
    ) -> Option<&ImplementationIndexEntry> {
        self.implementations.get(&call_name.to_lowercase())
    }

    pub fn find_pou_implementation(&self, pou_name: &str) -> Option<&ImplementationIndexEntry> {
        self.find_pou(pou_name)
            .and_then(|it| it.find_implementation(self))
    }

    /// registers a member-variable of a container to be accessed in a qualified name.
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
        source_location: SymbolLocation,
        location: u32,
    ) {
        let container_name = member_info.container_name;
        let variable_name = member_info.variable_name;
        let variable_type = member_info.variable_linkage;
        let data_type_name = member_info.variable_type_name;

        let qualified_name = format!("{}.{}", container_name, variable_name);

        let entry = VariableIndexEntry::new(
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
        .set_varargs(member_info.varargs);

        self.register_member_entry(container_name, entry);
    }
    pub fn register_member_entry(&mut self, container_name: &str, entry: VariableIndexEntry) {
        let key = container_name.to_lowercase();
        let members = self.member_variables.entry(key).or_default();
        members.insert(entry.name.to_lowercase(), entry);
    }

    pub fn register_enum_element(
        &mut self,
        element_name: &str,
        enum_type_name: &str,
        initial_value: Option<ConstId>,
        source_location: SymbolLocation,
    ) {
        let qualified_name = format!("{}.{}", enum_type_name, element_name);
        let entry = VariableIndexEntry::create_global(
            element_name,
            &qualified_name,
            enum_type_name,
            source_location,
        )
        .set_constant(true)
        .set_initial_value(initial_value);
        self.enum_global_variables
            .insert(element_name.to_lowercase(), entry.clone());

        self.enum_qualified_variables
            .insert(qualified_name.to_lowercase(), entry);
    }

    pub fn register_global_variable(&mut self, name: &str, variable: VariableIndexEntry) {
        self.global_variables.insert(name.to_lowercase(), variable);
    }

    pub fn register_global_initializer(&mut self, name: &str, variable: VariableIndexEntry) {
        self.global_initializers
            .insert(name.to_lowercase(), variable);
    }

    pub fn register_type(&mut self, datatype: DataType) {
        self.type_index
            .types
            .insert(datatype.get_name().to_lowercase(), datatype);
    }

    pub fn register_pou_type(&mut self, datatype: DataType) {
        self.type_index
            .pou_types
            .insert(datatype.get_name().to_lowercase(), datatype);
    }

    pub fn find_callable_instance_variable(
        &self,
        context: Option<&str>,
        reference: &[&str],
    ) -> Option<&VariableIndexEntry> {
        //look for a *callable* variable with that name
        self.find_variable(context, reference).filter(|v| {
            //callable means, there is an implementation associated with the variable's datatype
            self.find_implementation_by_name(&v.data_type_name)
                .is_some()
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
                    DataTypeInformation::SubRange {
                        referenced_type, ..
                    } => referenced_type,
                    _ => initial_type.get_name(),
                };
                if let Some(inner_type) = self.find_effective_type_info(inner_type_name) {
                    self.find_intrinsic_type(inner_type)
                } else {
                    initial_type
                }
            }
            DataTypeInformation::Enum {
                referenced_type, ..
            } => self
                .find_effective_type_info(referenced_type)
                .unwrap_or(initial_type),
            _ => initial_type,
        }
    }

    pub fn find_elementary_pointer_type<'idx>(
        &'idx self,
        initial_type: &'idx DataTypeInformation,
    ) -> &'idx DataTypeInformation {
        match initial_type {
            DataTypeInformation::Pointer {
                inner_type_name, ..
            } => {
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
        if let Some(PouIndexEntry::Function {
            linkage: LinkageType::BuiltIn,
            ..
        }) = self.find_pou(name)
        {
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
            | DataTypeInformation::SubRange {
                referenced_type: name,
                ..
            } => {
                //traverse to the primitive type
                self.find_effective_type_info(name)
                    .and_then(|info| self.find_range_check_implementation_for(info))
            }
            _ => None,
        }
    }
}

/// Returns a default initialization name for a variable or type
pub fn get_initializer_name(name: &str) -> String {
    format!("__{}__init", name)
}
impl VariableType {
    pub(crate) fn is_private(&self) -> bool {
        matches!(self, VariableType::Temp | VariableType::Local)
    }
}
