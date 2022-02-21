// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use indexmap::IndexMap;

use crate::{
    ast::{
        AstStatement, DirectAccessType, HardwareAccessType, Implementation, LinkageType, PouType,
        SourceRange, TypeNature,
    },
    builtins,
    diagnostics::Diagnostic,
    lexer::IdProvider,
    typesystem::{self, *},
};

use self::{
    const_expressions::{ConstExpressions, ConstId},
    instance_iterator::InstanceIterator,
};

pub mod const_expressions;
mod instance_iterator;
#[cfg(test)]
mod tests;
pub mod visitor;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableIndexEntry {
    /// the name of this variable (e.g. 'x' for 'PLC_PRG.x')
    name: String,
    /// the fully qualified name of this variable (e.g. PLC_PRG.x)
    qualified_name: String,
    /// an optional initial value of this variable
    pub initial_value: Option<ConstId>,
    /// the type of variable
    pub variable_type: VariableType,
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
    pub source_location: SourceRange,
}

#[derive(Debug, PartialEq, Clone)]
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
    variable_linkage: VariableType,
    variable_type_name: &'b str,
    binding: Option<HardwareBinding>,
    is_constant: bool,
}

impl VariableIndexEntry {
    pub fn new(
        name: &str,
        qualified_name: &str,
        data_type_name: &str,
        variable_type: VariableType,
        location_in_parent: u32,
        source_location: SourceRange,
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
        }
    }

    pub fn create_global(
        name: &str,
        qualified_name: &str,
        data_type_name: &str,
        source_location: SourceRange,
    ) -> Self {
        VariableIndexEntry {
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            initial_value: None,
            variable_type: VariableType::Global,
            is_constant: false,
            data_type_name: data_type_name.to_string(),
            location_in_parent: 0,
            linkage: LinkageType::Internal,
            binding: None,
            source_location,
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

    /// Creates a new VariableIndexEntry from the current entry with a new container and type
    /// This is used to create new entries from previously generic entries
    pub fn into_typed(&self, container: &str, new_type: &str) -> Self {
        let name = if self.is_return() {
            container
        } else {
            &self.name
        };
        VariableIndexEntry {
            name: name.to_string(),
            qualified_name: format!("{}.{}", container, name),
            data_type_name: new_type.to_string(),
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
        self.variable_type == VariableType::Return
    }

    pub fn is_local(&self) -> bool {
        self.variable_type == VariableType::Local
    }
    pub fn is_temp(&self) -> bool {
        self.variable_type == VariableType::Temp
    }

    pub fn is_constant(&self) -> bool {
        self.is_constant
    }

    pub fn is_external(&self) -> bool {
        self.linkage == LinkageType::External
    }

    pub fn get_variable_type(&self) -> VariableType {
        self.variable_type
    }

    pub fn has_hardware_binding(&self) -> bool {
        self.binding.is_some()
    }

    pub fn get_hardware_binding(&self) -> Option<&HardwareBinding> {
        self.binding.as_ref()
    }

    pub(crate) fn is_parameter(&self) -> bool {
        let vt = self.get_variable_type();
        matches!(
            vt,
            VariableType::Input | VariableType::Output | VariableType::InOut
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VariableType {
    Local,
    Temp,
    Input,
    Output,
    InOut,
    Global,
    Return,
}

/// information regarding a variable
#[derive(Debug, PartialEq, Clone)]
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

#[derive(Clone, PartialEq, Debug)]
pub enum ImplementationType {
    Program,
    Function,
    FunctionBlock,
    Action,
    Class,
    Method,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplementationIndexEntry {
    pub(crate) call_name: String,
    pub(crate) type_name: String,
    pub(crate) associated_class: Option<String>,
    pub(crate) implementation_type: ImplementationType,
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
}

impl From<&Implementation> for ImplementationIndexEntry {
    fn from(implementation: &Implementation) -> Self {
        let pou_type = &implementation.pou_type;
        ImplementationIndexEntry {
            call_name: implementation.name.clone(),
            type_name: implementation.type_name.clone(),
            associated_class: pou_type.get_optional_owner_class(),
            implementation_type: pou_type.into(),
        }
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

/// the TypeIndex carries all types.
/// it is extracted into its seaprate struct so it can be
/// internally borrowed individually from the other maps
#[derive(Debug, PartialEq)]
pub struct TypeIndex {
    /// all types (structs, enums, type, POUs, etc.)
    types: IndexMap<String, DataType>,
    pou_types: IndexMap<String, DataType>,

    void_type: DataType,
}

impl Default for TypeIndex {
    fn default() -> Self {
        TypeIndex {
            types: IndexMap::new(),
            pou_types: IndexMap::new(),
            void_type: DataType {
                name: VOID_TYPE.into(),
                initial_value: None,
                information: DataTypeInformation::Void,
                nature: TypeNature::Any,
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
    global_variables: IndexMap<String, VariableIndexEntry>,

    /// all struct initializers
    global_initializers: IndexMap<String, VariableIndexEntry>,

    /// all enum-members with their names
    enum_global_variables: IndexMap<String, VariableIndexEntry>,

    /// all enum-members with their qualified names <enum-type>.<element-name>
    enum_qualified_variables: IndexMap<String, VariableIndexEntry>,

    /// all local variables, grouped by the POU's name
    member_variables: IndexMap<String, IndexMap<String, VariableIndexEntry>>,

    /// all implementations
    implementations: IndexMap<String, ImplementationIndexEntry>,

    /// an index with all type-information
    type_index: TypeIndex,

    constant_expressions: ConstExpressions,
}

impl Index {
    pub fn create_with_builtins(id_provider: IdProvider) -> Index {
        let (unit, _) = builtins::parse_built_ins(id_provider.clone());
        visitor::visit_index(Index::default(), &unit, id_provider)
    }

    /// imports all entries from the given index into the current index
    ///
    /// imports all global_variables, member_variables, types and implementations
    /// # Arguments
    /// - `other` the other index. The elements are drained from the given index and moved
    /// into the current one
    pub fn import(&mut self, mut other: Index) {
        //global variables
        for (name, mut e) in other.global_variables.drain(..) {
            e.initial_value =
                self.maybe_import_const_expr(&mut other.constant_expressions, &e.initial_value);
            self.global_variables.insert(name, e);
        }

        //enum_global_variables
        for (name, mut e) in other.enum_global_variables.drain(..) {
            e.initial_value =
                self.maybe_import_const_expr(&mut other.constant_expressions, &e.initial_value);
            self.enum_global_variables.insert(name, e.clone());
            self.enum_qualified_variables
                .insert(e.qualified_name.to_lowercase(), e);
        }

        //initializers
        for (name, mut e) in other.global_initializers.drain(..) {
            e.initial_value =
                self.maybe_import_const_expr(&mut other.constant_expressions, &e.initial_value);
            self.global_initializers.insert(name, e);
        }

        //member variables
        for (name, mut members) in other.member_variables.drain(..) {
            //enum qualified variables
            for (_, mut e) in members.iter_mut() {
                e.initial_value =
                    self.maybe_import_const_expr(&mut other.constant_expressions, &e.initial_value);
            }
            self.member_variables.insert(name, members);
        }

        //types
        for (name, mut e) in other.type_index.types.drain(..) {
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
                _ => {}
            }
            self.type_index.types.insert(name, e);
        }

        //pou_types
        for (name, mut e) in other.type_index.pou_types.drain(..) {
            e.initial_value =
                self.maybe_import_const_expr(&mut other.constant_expressions, &e.initial_value);
            self.type_index.pou_types.insert(name, e);
        }

        //implementations
        self.implementations.extend(other.implementations);

        //Constant expressions are intentionally not imported
        // self.constant_expressions.import(other.constant_expressions)
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
            .and_then(|it| import_from.remove(it))
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
                .remove(id)
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
        let first_var = segments[0];
        let mut result = match context {
            Some(context) => self
                .find_member(context, first_var)
                .or_else(|| self.find_global_variable(first_var)),
            None => self.find_global_variable(first_var),
        };
        for segment in segments.iter().skip(1) {
            result = match result {
                Some(context) => self.find_member(&context.data_type_name, segment),
                None => None,
            };
        }
        result
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

    /// returns all member variables of the given container (e.g. FUNCTION, PROGRAM, STRUCT, etc.)
    pub fn get_container_members(&self, container_name: &str) -> Vec<&VariableIndexEntry> {
        self.member_variables
            .get(&container_name.to_lowercase())
            .map(|it| it.values().collect())
            .unwrap_or_else(Vec::new)
    }

    /// returns true if the current index is a VAR_INPUT, VAR_IN_OUT or VAR_OUTPUT that is not a variadic argument
    /// In other words it returns whether the member variable at `index` of the given container is a possible parameter in
    /// call to it
    pub fn is_declared_parameter(&self, container_name: &str, index: u32) -> bool {
        self.member_variables
            .get(&container_name.to_lowercase())
            .and_then(|map| {
                map.values()
                    .filter(|item| {
                        item.variable_type == VariableType::Input
                            || item.variable_type == VariableType::InOut
                            || item.variable_type == VariableType::Output
                    })
                    .find(|item| item.location_in_parent == index)
            })
            .is_some()
    }

    pub fn find_input_parameter(&self, pou_name: &str, index: u32) -> Option<&VariableIndexEntry> {
        self.member_variables
            .get(&pou_name.to_lowercase())
            .and_then(|map| {
                map.values()
                    .filter(|item| item.variable_type == VariableType::Input)
                    .find(|item| item.location_in_parent == index)
            })
    }

    /// returns the effective DataType of the type with the given name if it exists
    pub fn find_effective_type(&self, type_name: &str) -> Option<&DataType> {
        self.type_index.find_effective_type_by_name(type_name)
    }

    /// returns the effective DataType of the type with the given name or an Error
    pub fn get_effective_type(&self, type_name: &str) -> Result<&DataType, Diagnostic> {
        self.type_index
            .find_effective_type_by_name(type_name)
            .ok_or_else(|| Diagnostic::unknown_type(type_name, SourceRange::undefined()))
    }

    /// returns the effective DataTypeInformation of the type with the given name if it exists
    pub fn find_effective_type_info(&self, type_name: &str) -> Option<&DataTypeInformation> {
        self.find_effective_type(type_name)
            .map(DataType::get_type_information)
    }

    /// returns the effective type of the type with the with the given name or the
    /// void-type if the given name does not exist
    pub fn get_effective_type_by_name(&self, type_name: &str) -> &DataType {
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

    pub fn find_return_variable(&self, pou_name: &str) -> Option<&VariableIndexEntry> {
        let members = self.member_variables.get(&pou_name.to_lowercase()); //.ok_or_else(||Diagnostic::unknown_type(pou_name, 0..0))?;
        if let Some(members) = members {
            for (_, variable) in members {
                if variable.variable_type == VariableType::Return {
                    return Some(variable);
                }
            }
        }
        None
    }

    pub fn find_return_type(&self, pou_name: &str) -> Option<&DataType> {
        let variable = self.find_return_variable(pou_name);
        variable.and_then(|it| self.get_type(it.get_type_name()).ok())
    }

    pub fn get_type_information_or_void(&self, type_name: &str) -> &DataTypeInformation {
        self.find_effective_type(type_name)
            .map(|it| it.get_type_information())
            .unwrap_or_else(|| self.get_void_type().get_type_information())
    }

    /// Returns a list of types, should not be used to search for types, just to react on them
    pub fn get_types(&self) -> &IndexMap<String, DataType> {
        &self.type_index.types
    }
    pub fn get_pou_types(&self) -> &IndexMap<String, DataType> {
        &self.type_index.pou_types
    }

    pub fn get_globals(&self) -> &IndexMap<String, VariableIndexEntry> {
        &self.global_variables
    }

    pub fn get_global_initializers(&self) -> &IndexMap<String, VariableIndexEntry> {
        &self.global_initializers
    }

    pub fn get_members(&self, name: &str) -> Option<&IndexMap<String, VariableIndexEntry>> {
        self.member_variables.get(name.to_lowercase().as_str())
    }

    pub fn get_global_qualified_enums(&self) -> &IndexMap<String, VariableIndexEntry> {
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
    ) {
        self.implementations.insert(
            call_name.to_lowercase(),
            ImplementationIndexEntry {
                call_name: call_name.into(),
                type_name: type_name.into(),
                associated_class: associated_class_name.map(|str| str.into()),
                implementation_type: impl_type,
            },
        );
    }

    pub fn find_implementation(&self, call_name: &str) -> Option<&ImplementationIndexEntry> {
        self.implementations.get(&call_name.to_lowercase())
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
        source_location: SourceRange,
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
        .set_hardware_binding(member_info.binding);

        self.register_member_entry(container_name, entry);
    }
    pub fn register_member_entry(&mut self, container_name: &str, entry: VariableIndexEntry) {
        let members = self
            .member_variables
            .entry(container_name.to_lowercase())
            .or_insert_with(IndexMap::new);
        members.insert(entry.name.to_lowercase(), entry);
    }

    pub fn register_enum_element(
        &mut self,
        element_name: &str,
        enum_type_name: &str,
        initial_value: Option<ConstId>,
        source_location: SourceRange,
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
            self.find_implementation(&v.data_type_name).is_some()
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
    /// this will return the built-in type behind alias and range-types
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

    pub fn is_builtin(&self, function: &str) -> bool {
        //Find a type for that function, see if that type is builtin
        self.find_effective_type_info(function)
            .map(DataTypeInformation::is_builtin)
            .unwrap_or_default()
    }
}

/// Returns a default initialization name for a variable or type
pub fn get_initializer_name(name: &str) -> String {
    format!("{}__init", name)
}
