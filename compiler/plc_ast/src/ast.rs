// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use derive_more::TryInto;
use serde::{Deserialize, Serialize};

use crate::{
    control_statements::{
        AstControlStatement, CaseStatement, ConditionalBlock, ForLoopStatement, IfStatement, LoopStatement,
        ReturnStatement,
    },
    literals::{AstLiteral, StringValue},
    pre_processor,
    provider::IdProvider,
};

use plc_source::source_location::*;

pub type AstId = usize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GenericBinding {
    pub name: String,
    pub nature: TypeNature,
}

#[derive(PartialEq)]
pub struct Pou {
    pub name: String,
    pub kind: PouType,
    pub variable_blocks: Vec<VariableBlock>,
    pub return_type: Option<DataTypeDeclaration>,
    /// The SourceLocation of the whole POU
    pub location: SourceLocation,
    /// The SourceLocation of the POUs name
    pub name_location: SourceLocation,
    pub poly_mode: Option<PolymorphismMode>,
    pub generics: Vec<GenericBinding>,
    pub linkage: LinkageType,
    pub super_class: Option<String>,
    /// A list of interfaces this POU implements
    pub interfaces: Vec<InterfaceIdentifier>,
    pub is_const: bool,
}

#[derive(Debug, PartialEq)]
pub struct Interface {
    pub name: String,
    pub methods: Vec<Pou>,
    pub location: SourceLocation,
    pub location_name: SourceLocation,
}

/// Helper struct for [`Pou`] to get the location of the interface without relying on [`Interface`] which
/// only exists if the interface is actually defined. Mostly needed for user-friendly validation messages.
#[derive(Debug, PartialEq)]
pub struct InterfaceIdentifier {
    pub name: String,
    pub location: SourceLocation,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub name: String,
    pub name_location: SourceLocation,
    pub kind_parent: PouType,
    pub name_parent: String,
    pub name_parent_location: SourceLocation,
    pub datatype: DataTypeDeclaration,
    pub implementations: Vec<PropertyImplementation>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PropertyImplementation {
    pub kind: PropertyKind,
    pub variables: Vec<VariableBlock>,
    pub statements: Vec<AstNode>,
    pub location: SourceLocation,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PropertyKind {
    Get,
    Set,
}

impl std::fmt::Display for PropertyKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyKind::Get => write!(f, "get"),
            PropertyKind::Set => write!(f, "set"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PolymorphismMode {
    None,
    Abstract,
    Final,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "direction")]
pub enum HardwareAccessType {
    Input,
    Output,
    Memory,
    Global,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum DirectAccessType {
    Bit,
    Byte,
    Word,
    DWord,
    LWord,
    Template,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeNature {
    Any,
    Derived,
    Elementary,
    Magnitude,
    Num,
    Real,
    Int,
    Signed,
    Unsigned,
    Duration,
    Bit,
    Chars,
    String,
    Char,
    Date,
    __VLA,
}

impl TypeNature {
    pub fn derives_from(self, other: TypeNature) -> bool {
        if other == self {
            true
        } else {
            match self {
                TypeNature::Any => true,
                TypeNature::Derived => matches!(other, TypeNature::Any),
                TypeNature::__VLA => matches!(other, TypeNature::Any),
                TypeNature::Elementary => matches!(other, TypeNature::Any),
                TypeNature::Magnitude => matches!(other, TypeNature::Elementary | TypeNature::Any),
                TypeNature::Num => {
                    matches!(other, TypeNature::Magnitude | TypeNature::Elementary | TypeNature::Any)
                }
                TypeNature::Real => matches!(
                    other,
                    TypeNature::Num | TypeNature::Magnitude | TypeNature::Elementary | TypeNature::Any
                ),
                TypeNature::Int => matches!(
                    other,
                    TypeNature::Num | TypeNature::Magnitude | TypeNature::Elementary | TypeNature::Any
                ),
                TypeNature::Signed => matches!(
                    other,
                    TypeNature::Int
                        | TypeNature::Num
                        | TypeNature::Magnitude
                        | TypeNature::Elementary
                        | TypeNature::Any
                ),
                TypeNature::Unsigned => matches!(
                    other,
                    TypeNature::Int
                        | TypeNature::Num
                        | TypeNature::Magnitude
                        | TypeNature::Elementary
                        | TypeNature::Any
                ),
                TypeNature::Duration => {
                    matches!(other, TypeNature::Magnitude | TypeNature::Elementary | TypeNature::Any)
                }
                TypeNature::Bit => matches!(other, TypeNature::Elementary | TypeNature::Any),
                TypeNature::Chars => matches!(other, TypeNature::Elementary | TypeNature::Any),
                TypeNature::String => {
                    matches!(other, TypeNature::Chars | TypeNature::Elementary | TypeNature::Any)
                }
                TypeNature::Char => {
                    matches!(other, TypeNature::Chars | TypeNature::Elementary | TypeNature::Any)
                }
                TypeNature::Date => matches!(other, TypeNature::Elementary | TypeNature::Any),
            }
        }
    }

    pub fn is_numerical(&self) -> bool {
        self.derives_from(TypeNature::Num)
    }

    pub fn is_real(&self) -> bool {
        self.derives_from(TypeNature::Real)
    }

    pub fn is_bit(&self) -> bool {
        self.derives_from(TypeNature::Bit)
    }
}

impl Display for TypeNature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            TypeNature::Any => "ANY",
            TypeNature::Derived => "ANY_DERIVED",
            TypeNature::Elementary => "ANY_ELEMENTARY",
            TypeNature::Magnitude => "ANY_MAGNITUDE",
            TypeNature::Num => "ANY_NUMBER",
            TypeNature::Real => "ANY_REAL",
            TypeNature::Int => "ANY_INT",
            TypeNature::Signed => "ANY_SIGNED",
            TypeNature::Unsigned => "ANY_UNSIGNED",
            TypeNature::Duration => "ANY_DURATION",
            TypeNature::Bit => "ANY_BIT",
            TypeNature::Chars => "ANY_CHARS",
            TypeNature::String => "ANY_STRING",
            TypeNature::Char => "ANY_CHAR",
            TypeNature::Date => "ANY_DATE",
            TypeNature::__VLA => "__ANY_VLA",
        };
        write!(f, "{name}")
    }
}

impl DirectAccessType {
    /// Returns the size of the bitaccess result
    pub fn get_bit_width(&self) -> u64 {
        match self {
            DirectAccessType::Bit => 1,
            DirectAccessType::Byte => 8,
            DirectAccessType::Word => 16,
            DirectAccessType::DWord => 32,
            DirectAccessType::LWord => 64,
            DirectAccessType::Template => unimplemented!("Should not test for template width"),
        }
    }
}

impl Debug for Pou {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = f.debug_struct("POU");
        str.field("name", &self.name)
            .field("variable_blocks", &self.variable_blocks)
            .field("pou_type", &self.kind)
            .field("return_type", &self.return_type)
            .field("interfaces", &self.interfaces);
        if !self.generics.is_empty() {
            str.field("generics", &self.generics);
        }
        str.finish()
    }
}

impl Pou {
    pub fn get_return_name(&self) -> &str {
        Pou::calc_return_name(&self.name)
    }

    pub fn calc_return_name(pou_name: &str) -> &str {
        pou_name.split('.').last().unwrap_or_default()
    }

    pub fn is_aggregate(&self) -> bool {
        matches!(self.return_type, Some(DataTypeDeclaration::Aggregate { .. }))
    }

    pub fn is_generic(&self) -> bool {
        !self.generics.is_empty()
    }
}

#[derive(Debug, PartialEq)]
pub struct Implementation {
    pub name: String,
    pub type_name: String,
    pub linkage: LinkageType,
    pub pou_type: PouType,
    pub statements: Vec<AstNode>,
    pub location: SourceLocation,
    pub name_location: SourceLocation,
    pub overriding: bool,
    pub generic: bool,
    pub access: Option<AccessModifier>,
}

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash)]
pub enum LinkageType {
    Internal,
    External,
    BuiltIn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessModifier {
    Private,
    Public,
    Protected, // default
    Internal,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PouType {
    Program,
    Function,
    FunctionBlock,
    Action,
    Class,
    Method {
        /// The parent of this method, i.e. a function block, class or an interface
        parent: String,
        property: Option<String>,
    },
    Init,
    ProjectInit,
}

impl Display for PouType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PouType::Program => write!(f, "Program"),
            PouType::Function => write!(f, "Function"),
            PouType::FunctionBlock => write!(f, "FunctionBlock"),
            PouType::Action => write!(f, "Action"),
            PouType::Class => write!(f, "Class"),
            PouType::Method { .. } => write!(f, "Method"),
            PouType::Init => write!(f, "Init"),
            PouType::ProjectInit => write!(f, "ProjectInit"),
        }
    }
}

impl PouType {
    /// returns Some(owner_class) if this is a `Method` or otherwhise `None`
    pub fn get_optional_owner_class(&self) -> Option<String> {
        if let PouType::Method { parent, .. } = self {
            Some(parent.clone())
        } else {
            None
        }
    }

    pub fn is_function_or_init(&self) -> bool {
        matches!(self, PouType::Function | PouType::Init | PouType::ProjectInit)
    }

    pub fn is_stateful_pou(&self) -> bool {
        matches!(self, PouType::FunctionBlock | PouType::Program | PouType::Class)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConfigVariable {
    pub reference: AstNode,
    pub data_type: DataTypeDeclaration,
    pub address: AstNode,
    pub location: SourceLocation,
}

impl ConfigVariable {
    pub fn new(
        reference: AstNode,
        data_type: DataTypeDeclaration,
        address: AstNode,
        location: SourceLocation,
    ) -> Self {
        Self { reference, data_type, address, location }
    }
}

#[derive(Debug, PartialEq)]
pub struct CompilationUnit {
    pub global_vars: Vec<VariableBlock>,
    pub var_config: Vec<ConfigVariable>,
    /// List of POU definitions (signature and some additional metadata)
    pub units: Vec<Pou>,
    /// List of statements within a POU body
    pub implementations: Vec<Implementation>,
    pub interfaces: Vec<Interface>,
    pub user_types: Vec<UserTypeDeclaration>,
    pub file_name: String,
    pub properties: Vec<Property>,
}

impl CompilationUnit {
    pub fn new(file_name: &str) -> Self {
        CompilationUnit {
            global_vars: Vec::new(),
            var_config: Vec::new(),
            units: Vec::new(),
            implementations: Vec::new(),
            interfaces: Vec::new(),
            user_types: Vec::new(),
            file_name: file_name.to_string(),
            properties: Vec::new(),
        }
    }

    pub fn with_implementations(mut self, implementations: Vec<Implementation>) -> Self {
        self.implementations = implementations;
        self
    }

    /// imports all elements of the other CompilationUnit into this CompilationUnit
    ///
    /// this will import all global_vars, units, implementations and types. The imported
    /// structs are moved from the other unit into this unit
    /// # Arguments
    /// `other` the other CompilationUnit to import the elements from.
    pub fn import(&mut self, other: CompilationUnit) {
        self.global_vars.extend(other.global_vars);
        self.units.extend(other.units);
        self.implementations.extend(other.implementations);
        self.user_types.extend(other.user_types);
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum VariableBlockType {
    Local,
    Temp,
    Input(ArgumentProperty),
    Output,
    Global,
    InOut,
    External,
    /// Compiler internal
    Property,
}

impl Display for VariableBlockType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableBlockType::Local => write!(f, "Local"),
            VariableBlockType::Temp => write!(f, "Temp"),
            VariableBlockType::Input(_) => write!(f, "Input"),
            VariableBlockType::Output => write!(f, "Output"),
            VariableBlockType::Global => write!(f, "Global"),
            VariableBlockType::InOut => write!(f, "InOut"),
            VariableBlockType::External => write!(f, "External"),
            VariableBlockType::Property => write!(f, "Property"),
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum ArgumentProperty {
    ByVal,
    ByRef,
}

#[derive(PartialEq, Clone)]
pub struct VariableBlock {
    pub access: AccessModifier,
    pub constant: bool,
    pub retain: bool,
    pub variables: Vec<Variable>,
    pub variable_block_type: VariableBlockType,
    pub linkage: LinkageType,
    pub location: SourceLocation,
}

impl VariableBlock {
    pub fn with_block_type(mut self, block_type: VariableBlockType) -> Self {
        self.variable_block_type = block_type;
        self
    }

    pub fn with_variables(mut self, variables: Vec<Variable>) -> Self {
        self.variables = variables;
        self
    }

    /// Creates a new VariableBlock with the block type set to [`Property`]
    pub fn property(variables: Vec<Variable>) -> VariableBlock {
        VariableBlock {
            access: AccessModifier::Internal,
            constant: false,
            retain: false,
            variables,
            variable_block_type: VariableBlockType::Property,
            linkage: LinkageType::Internal,
            location: SourceLocation::internal(),
        }
    }
}

impl Default for VariableBlock {
    fn default() -> Self {
        VariableBlock {
            access: AccessModifier::Internal,
            constant: false,
            retain: false,
            variables: vec![],
            variable_block_type: VariableBlockType::Local,
            linkage: LinkageType::Internal,
            location: SourceLocation::internal(),
        }
    }
}

impl Debug for VariableBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariableBlock")
            .field("variables", &self.variables)
            .field("variable_block_type", &self.variable_block_type)
            .finish()
    }
}

#[derive(Clone)]
pub struct Variable {
    pub name: String,
    pub data_type_declaration: DataTypeDeclaration,
    pub initializer: Option<AstNode>,
    pub address: Option<AstNode>,
    pub location: SourceLocation,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.location == other.location
    }
}

impl Debug for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut var = f.debug_struct("Variable");
        var.field("name", &self.name).field("data_type", &self.data_type_declaration);
        if self.initializer.is_some() {
            var.field("initializer", &self.initializer);
        }
        if self.address.is_some() {
            var.field("address", &self.address);
        }
        var.finish()
    }
}

impl Variable {
    pub fn replace_data_type_with_reference_to(&mut self, type_name: String) -> DataTypeDeclaration {
        let new_data_type = DataTypeDeclaration::DataTypeReference {
            referenced_type: type_name,
            location: self.data_type_declaration.get_location(),
        };
        std::mem::replace(&mut self.data_type_declaration, new_data_type)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, PartialEq)]
pub enum DataTypeDeclaration {
    DataTypeReference { referenced_type: String, location: SourceLocation },
    DataTypeDefinition { data_type: DataType, location: SourceLocation, scope: Option<String> },
    Aggregate { referenced_type: String, location: SourceLocation },
}

impl Debug for DataTypeDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataTypeDeclaration::DataTypeReference { referenced_type, .. } => {
                f.debug_struct("DataTypeReference").field("referenced_type", referenced_type).finish()
            }
            DataTypeDeclaration::DataTypeDefinition { data_type, .. } => {
                f.debug_struct("DataTypeDefinition").field("data_type", data_type).finish()
            }
            DataTypeDeclaration::Aggregate { referenced_type, .. } => {
                f.debug_struct("Aggregate").field("referenced_type", referenced_type).finish()
            }
        }
    }
}

impl From<&DataTypeDeclaration> for SourceLocation {
    fn from(value: &DataTypeDeclaration) -> Self {
        value.get_location()
    }
}

impl DataTypeDeclaration {
    pub fn get_name(&self) -> Option<&str> {
        match self {
            Self::Aggregate { referenced_type, .. }
            | DataTypeDeclaration::DataTypeReference { referenced_type, .. } => {
                Some(referenced_type.as_str())
            }
            DataTypeDeclaration::DataTypeDefinition { data_type, .. } => data_type.get_name(),
        }
    }

    pub fn get_location(&self) -> SourceLocation {
        match self {
            DataTypeDeclaration::DataTypeReference { location, .. } => location.clone(),
            DataTypeDeclaration::DataTypeDefinition { location, .. } => location.clone(),
            Self::Aggregate { location, .. } => location.clone(),
        }
    }

    pub fn get_referenced_type(&self) -> Option<String> {
        let DataTypeDeclaration::DataTypeReference { referenced_type, .. } = self else { return None };
        Some(referenced_type.to_owned())
    }

    pub fn get_inner_pointer_ty(&self) -> Option<DataTypeDeclaration> {
        match self {
            DataTypeDeclaration::DataTypeReference { .. } => Some(self.clone()),

            DataTypeDeclaration::DataTypeDefinition { data_type, .. } => {
                if let DataType::PointerType { referenced_type, .. } = data_type {
                    return referenced_type.get_inner_pointer_ty();
                }

                None
            }
            Self::Aggregate { .. } => None,
        }
    }

    pub fn is_aggregate(&self) -> bool {
        matches!(self, Self::Aggregate { .. })
    }
}

#[derive(PartialEq)]
pub struct UserTypeDeclaration {
    pub data_type: DataType,
    pub initializer: Option<AstNode>,
    pub location: SourceLocation,
    /// stores the original scope for compiler-generated types
    pub scope: Option<String>,
}

impl Debug for UserTypeDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserTypeDeclaration")
            .field("data_type", &self.data_type)
            .field("initializer", &self.initializer)
            .field("scope", &self.scope)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    StructType {
        name: Option<String>, //maybe None for inline structs
        variables: Vec<Variable>,
    },
    EnumType {
        name: Option<String>, //maybe empty for inline enums
        numeric_type: String,
        elements: AstNode, //a single Ref, or an ExpressionList with Refs
    },
    SubRangeType {
        name: Option<String>,
        referenced_type: String,
        bounds: Option<AstNode>,
    },
    ArrayType {
        name: Option<String>,
        bounds: AstNode,
        referenced_type: Box<DataTypeDeclaration>,
        is_variable_length: bool,
    },
    PointerType {
        name: Option<String>,
        referenced_type: Box<DataTypeDeclaration>,
        auto_deref: Option<AutoDerefType>,
    },
    StringType {
        name: Option<String>,
        is_wide: bool, //WSTRING
        size: Option<AstNode>,
    },
    VarArgs {
        referenced_type: Option<Box<DataTypeDeclaration>>,
        sized: bool, //If the variadic has the sized property
    },
    GenericType {
        name: String,
        generic_symbol: String,
        nature: TypeNature,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoDerefType {
    /// A plain pointer variable with the auto-deref trait, e.g. VAR_IN_OUT or VAR_INPUT{ref} variables
    Default,

    /// An alias pointer variable, e.g. `foo AT bar : DINT`
    Alias,

    /// A reference pointer variable, e.g. `foo : REFERENCE TO DINT;`
    Reference,
}

impl DataType {
    pub fn set_name(&mut self, new_name: String) {
        match self {
            DataType::StructType { name, .. }
            | DataType::EnumType { name, .. }
            | DataType::SubRangeType { name, .. }
            | DataType::ArrayType { name, .. }
            | DataType::PointerType { name, .. }
            | DataType::StringType { name, .. } => *name = Some(new_name),
            DataType::GenericType { name, .. } => *name = new_name,
            DataType::VarArgs { .. } => {} //No names on varargs
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        match &self {
            DataType::StructType { name, .. }
            | DataType::EnumType { name, .. }
            | DataType::ArrayType { name, .. }
            | DataType::PointerType { name, .. }
            | DataType::StringType { name, .. }
            | DataType::SubRangeType { name, .. } => name.as_ref().map(|x| x.as_str()),
            DataType::GenericType { name, .. } => Some(name.as_str()),
            DataType::VarArgs { referenced_type, .. } => {
                referenced_type.as_ref().and_then(|it| DataTypeDeclaration::get_name(it.as_ref()))
            }
        }
    }

    //Attempts to replace the inner type with a reference. Returns the old type if replaceable
    pub fn replace_data_type_with_reference_to(
        &mut self,
        type_name: String,
        location: &SourceLocation,
    ) -> Option<DataTypeDeclaration> {
        match self {
            DataType::ArrayType { referenced_type, .. } | DataType::PointerType { referenced_type, .. } => {
                replace_reference(referenced_type, type_name, location)
            }
            _ => None,
        }
    }
}

fn replace_reference(
    referenced_type: &mut Box<DataTypeDeclaration>,
    type_name: String,
    location: &SourceLocation,
) -> Option<DataTypeDeclaration> {
    if let DataTypeDeclaration::DataTypeReference { .. } = **referenced_type {
        return None;
    }
    let new_data_type =
        DataTypeDeclaration::DataTypeReference { referenced_type: type_name, location: location.clone() };
    let old_data_type = std::mem::replace(referenced_type, Box::new(new_data_type));
    Some(*old_data_type)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReferenceAccess {
    /**
     * a, a.b
     */
    Member(Box<AstNode>),
    /**
     * a[3]
     */
    Index(Box<AstNode>),
    /**
     * Color#Red
     */
    Cast(Box<AstNode>),
    /**
     * a^
     */
    Deref,
    /**
     * &a
     */
    Address,
}

#[derive(Clone, PartialEq)]
pub struct AstNode {
    pub stmt: AstStatement,
    pub id: AstId,
    pub location: SourceLocation,
}

impl Default for AstNode {
    fn default() -> Self {
        AstFactory::create_empty_statement(SourceLocation::internal(), usize::MAX)
    }
}

#[derive(Debug, Clone, PartialEq, TryInto)]
#[try_into(ref, ref_mut, owned)]
pub enum AstStatement {
    EmptyStatement(EmptyStatement),

    // A placeholder which indicates a default value of a datatype
    DefaultValue(DefaultValue),

    // Literals
    Literal(AstLiteral),
    MultipliedStatement(MultipliedStatement),

    // Expressions
    ReferenceExpr(ReferenceExpr),
    Identifier(String),
    DirectAccess(DirectAccess),
    HardwareAccess(HardwareAccess),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    ExpressionList(Vec<AstNode>),
    ParenExpression(Box<AstNode>),
    RangeStatement(RangeStatement),
    VlaRangeStatement,

    // TODO: Merge these variants with a `kind` field?
    //       Update: Tried that, pattern matching becomes a pain in the ass; will probably be easier if we
    //               introduce a `get_inner` method to extract the enum variants or potentially wait for
    //               https://github.com/PLC-lang/rusty/pull/1221 to get merged
    // Assignments
    Assignment(Assignment),
    OutputAssignment(Assignment),
    RefAssignment(Assignment),

    CallStatement(CallStatement),

    // Control Statements
    ControlStatement(AstControlStatement),
    CaseCondition(Box<AstNode>),
    #[try_into(ignore)]
    ExitStatement(()),
    #[try_into(ignore)]
    ContinueStatement(()),
    ReturnStatement(ReturnStatement),
    JumpStatement(JumpStatement),
    LabelStatement(LabelStatement),
    AllocationStatement(Allocation),
}

#[macro_export]
/// A `try_from` convenience wrapper for `AstNode`, passed as the `ex:expr` argument.
/// Will try to return a reference to the variants inner type, specified via the `t:ty` parameter.
/// Converts the `try_from`-`Result` into an `Option`
macro_rules! try_from {
    () => { None };
    ($ex:expr, $t:ty) => {
        <&$t>::try_from($ex.get_stmt()).ok()
    };
    ($($ex:tt)*, $t:ty) => {
        try_from!($($ex)*, $t).ok()
    };
}

#[macro_export]
/// A `try_from` convenience wrapper for `AstNode`, passed as the `ex:expr` argument.
/// Will try to return a reference to the variants inner type, specified via the `t:ty` parameter.
/// Converts the `try_from`-`Result` into an `Option`
macro_rules! try_from_mut {
    () => { None };
    ($ex:expr, $t:ty) => {
        <&mut $t>::try_from($ex.get_stmt_mut()).ok()
    };
    ($($ex:tt)*, $t:ty) => {
        try_from_mut!($($ex)*, $t).ok()
    };
}

impl Debug for AstNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.stmt {
            AstStatement::EmptyStatement(..) => f.debug_struct("EmptyStatement").finish(),
            AstStatement::DefaultValue(..) => f.debug_struct("DefaultValue").finish(),
            AstStatement::Literal(literal) => literal.fmt(f),
            AstStatement::Identifier(name) => f.debug_struct("Identifier").field("name", name).finish(),
            AstStatement::BinaryExpression(BinaryExpression { operator, left, right }) => f
                .debug_struct("BinaryExpression")
                .field("operator", operator)
                .field("left", left)
                .field("right", right)
                .finish(),
            AstStatement::UnaryExpression(UnaryExpression { operator, value }) => {
                f.debug_struct("UnaryExpression").field("operator", operator).field("value", value).finish()
            }
            AstStatement::ExpressionList(expressions) => {
                f.debug_struct("ExpressionList").field("expressions", expressions).finish()
            }
            AstStatement::ParenExpression(expression) => {
                f.debug_struct("ParenExpression").field("expression", expression).finish()
            }
            AstStatement::RangeStatement(RangeStatement { start, end }) => {
                f.debug_struct("RangeStatement").field("start", start).field("end", end).finish()
            }
            AstStatement::VlaRangeStatement => f.debug_struct("VlaRangeStatement").finish(),
            AstStatement::Assignment(Assignment { left, right }) => {
                f.debug_struct("Assignment").field("left", left).field("right", right).finish()
            }
            AstStatement::OutputAssignment(Assignment { left, right }) => {
                f.debug_struct("OutputAssignment").field("left", left).field("right", right).finish()
            }
            AstStatement::RefAssignment(Assignment { left, right }) => {
                f.debug_struct("ReferenceAssignment").field("left", left).field("right", right).finish()
            }
            AstStatement::CallStatement(CallStatement { operator, parameters }) => f
                .debug_struct("CallStatement")
                .field("operator", operator)
                .field("parameters", parameters)
                .finish(),
            AstStatement::ControlStatement(
                AstControlStatement::If(IfStatement { blocks, else_block, .. }),
                ..,
            ) => {
                f.debug_struct("IfStatement").field("blocks", blocks).field("else_block", else_block).finish()
            }
            AstStatement::ControlStatement(
                AstControlStatement::ForLoop(ForLoopStatement { counter, start, end, by_step, body, .. }),
                ..,
            ) => f
                .debug_struct("ForLoopStatement")
                .field("counter", counter)
                .field("start", start)
                .field("end", end)
                .field("by_step", by_step)
                .field("body", body)
                .finish(),
            AstStatement::ControlStatement(
                AstControlStatement::WhileLoop(LoopStatement { condition, body, .. }),
                ..,
            ) => f
                .debug_struct("WhileLoopStatement")
                .field("condition", condition)
                .field("body", body)
                .finish(),
            AstStatement::ControlStatement(AstControlStatement::RepeatLoop(LoopStatement {
                condition,
                body,
                ..
            })) => f
                .debug_struct("RepeatLoopStatement")
                .field("condition", condition)
                .field("body", body)
                .finish(),
            AstStatement::ControlStatement(AstControlStatement::Case(CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            })) => f
                .debug_struct("CaseStatement")
                .field("selector", selector)
                .field("case_blocks", case_blocks)
                .field("else_block", else_block)
                .finish(),
            AstStatement::DirectAccess(DirectAccess { access, index }) => {
                f.debug_struct("DirectAccess").field("access", access).field("index", index).finish()
            }
            AstStatement::HardwareAccess(HardwareAccess { direction, access, address }) => f
                .debug_struct("HardwareAccess")
                .field("direction", direction)
                .field("access", access)
                .field("address", address)
                .field("location", &self.location)
                .finish(),
            AstStatement::MultipliedStatement(MultipliedStatement { multiplier, element }, ..) => f
                .debug_struct("MultipliedStatement")
                .field("multiplier", multiplier)
                .field("element", element)
                .finish(),
            AstStatement::CaseCondition(condition) => {
                f.debug_struct("CaseCondition").field("condition", condition).finish()
            }
            AstStatement::ReturnStatement(ReturnStatement { condition }) => {
                f.debug_struct("ReturnStatement").field("condition", condition).finish()
            }
            AstStatement::ContinueStatement(..) => f.debug_struct("ContinueStatement").finish(),
            AstStatement::ExitStatement(..) => f.debug_struct("ExitStatement").finish(),
            AstStatement::ReferenceExpr(ReferenceExpr { access, base }) => {
                f.debug_struct("ReferenceExpr").field("kind", access).field("base", base).finish()
            }
            AstStatement::JumpStatement(JumpStatement { condition, target, .. }) => {
                f.debug_struct("JumpStatement").field("condition", condition).field("target", target).finish()
            }
            AstStatement::LabelStatement(LabelStatement { name, .. }) => {
                f.debug_struct("LabelStatement").field("name", name).finish()
            }
            AstStatement::AllocationStatement(Allocation { name, reference_type }) => f
                .debug_struct("Allocation")
                .field("name", name)
                .field("reference_type", reference_type)
                .finish(),
        }
    }
}

impl From<&AstNode> for SourceLocation {
    fn from(value: &AstNode) -> Self {
        value.get_location()
    }
}

impl AstNode {
    ///Returns the statement in a singleton list, or the contained statements if the statement is already a list
    pub fn get_as_list(&self) -> Vec<&AstNode> {
        try_from!(self, Vec<AstNode>).map(|it| it.iter().collect()).unwrap_or(vec![self])
    }

    pub fn get_location(&self) -> SourceLocation {
        self.location.clone()
    }

    pub fn set_location(&mut self, location: SourceLocation) {
        self.location = location;
    }

    pub fn get_id(&self) -> AstId {
        self.id
    }

    pub fn get_stmt(&self) -> &AstStatement {
        &self.stmt
    }

    pub fn get_stmt_mut(&mut self) -> &mut AstStatement {
        &mut self.stmt
    }

    /// Similar to [`AstNode::get_stmt`] with the exception of peeling parenthesized expressions.
    /// For example if called on `((1))` this function would return a [`AstStatement::Literal`] ignoring the
    /// parenthesized expressions altogether.
    pub fn get_stmt_peeled(&self) -> &AstStatement {
        match &self.stmt {
            AstStatement::ParenExpression(expr) => expr.get_stmt_peeled(),
            _ => &self.stmt,
        }
    }

    /// Returns true if the current statement has a direct access.
    pub fn has_direct_access(&self) -> bool {
        match &self.stmt {
            AstStatement::ReferenceExpr(
                ReferenceExpr { access: ReferenceAccess::Member(reference), base },
                ..,
            )
            | AstStatement::ReferenceExpr(
                ReferenceExpr { access: ReferenceAccess::Cast(reference), base },
                ..,
            ) => {
                reference.has_direct_access()
                    || base.as_ref().map(|it| it.has_direct_access()).unwrap_or(false)
            }
            AstStatement::DirectAccess(..) => true,
            _ => false,
        }
    }

    /// returns true if this AST Statement is a literal or reference that can be
    /// prefixed with a type-cast (e.g. INT#23)
    pub fn is_cast_prefix_eligible(&self) -> bool {
        // TODO: figure out a better name for this...
        match &self.stmt {
            AstStatement::Literal(kind, ..) => kind.is_cast_prefix_eligible(),
            AstStatement::Identifier(..) => true,
            _ => false,
        }
    }

    /// Returns true if the current statement is a flat reference (e.g. `a`)
    pub fn is_flat_reference(&self) -> bool {
        self.get_flat_reference_name().is_some()
    }

    /// Returns the reference-name if this is a flat reference like `a`, or None if this is no flat reference
    pub fn get_flat_reference_name(&self) -> Option<&str> {
        match &self.stmt {
            AstStatement::ReferenceExpr(
                ReferenceExpr { access: ReferenceAccess::Member(reference), .. },
                ..,
            ) => reference.as_ref().get_flat_reference_name(),
            AstStatement::Identifier(name, ..) => Some(name),
            _ => None,
        }
    }

    pub fn get_parent_name_of_reference(&self) -> Option<&str> {
        if let AstStatement::ReferenceExpr(ReferenceExpr { base: Some(base), .. }, ..) = &self.stmt {
            base.as_ref().get_flat_reference_name()
        } else {
            None
        }
    }

    pub fn get_label_name(&self) -> Option<&str> {
        match &self.stmt {
            AstStatement::LabelStatement(LabelStatement { name, .. }) => Some(name.as_str()),
            _ => None,
        }
    }

    pub fn is_empty_statement(&self) -> bool {
        matches!(self.stmt, AstStatement::EmptyStatement(..))
    }

    pub fn is_assignment(&self) -> bool {
        matches!(self.stmt, AstStatement::Assignment(..))
    }

    pub fn is_output_assignment(&self) -> bool {
        matches!(self.stmt, AstStatement::OutputAssignment(..))
    }

    pub fn is_reference(&self) -> bool {
        matches!(self.stmt, AstStatement::ReferenceExpr(..))
    }

    pub fn is_call(&self) -> bool {
        matches!(self.stmt, AstStatement::CallStatement(..))
    }

    pub fn is_hardware_access(&self) -> bool {
        matches!(self.stmt, AstStatement::HardwareAccess(..))
    }

    pub fn is_array_access(&self) -> bool {
        matches!(
            self.stmt,
            AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Index(_), .. }, ..)
        )
    }

    pub fn is_pointer_access(&self) -> bool {
        matches!(
            self.stmt,
            AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Deref, .. }, ..)
        )
    }

    pub fn is_paren(&self) -> bool {
        matches!(self.stmt, AstStatement::ParenExpression { .. })
    }

    pub fn is_expression_list(&self) -> bool {
        matches!(self.stmt, AstStatement::ExpressionList { .. })
    }

    pub fn can_be_assigned_to(&self) -> bool {
        self.has_direct_access()
            || self.is_flat_reference()
            || self.is_reference()
            || self.is_array_access()
            || self.is_pointer_access()
            || self.is_hardware_access()
    }

    pub fn new(stmt: AstStatement, id: AstId, location: SourceLocation) -> AstNode {
        AstNode { stmt, id, location }
    }

    pub fn new_literal(kind: AstLiteral, id: AstId, location: SourceLocation) -> AstNode {
        AstNode::new(AstStatement::Literal(kind), id, location)
    }

    pub fn new_integer(value: i128, id: AstId, location: SourceLocation) -> AstNode {
        AstNode::new(AstStatement::Literal(AstLiteral::Integer(value)), id, location)
    }

    pub fn new_real(value: String, id: AstId, location: SourceLocation) -> AstNode {
        AstNode::new(AstStatement::Literal(AstLiteral::Real(value)), id, location)
    }

    pub fn new_string(
        value: impl Into<String>,
        is_wide: bool,
        id: AstId,
        location: SourceLocation,
    ) -> AstNode {
        AstNode::new(
            AstStatement::Literal(AstLiteral::String(StringValue { value: value.into(), is_wide })),
            id,
            location,
        )
    }

    /// Returns true if the given token is an integer or float and zero.
    pub fn is_zero(&self) -> bool {
        try_from!(self, AstLiteral).is_some_and(|it| it.is_zero())
    }

    pub fn is_binary_expression(&self) -> bool {
        matches!(self.stmt, AstStatement::BinaryExpression(..))
    }

    pub fn is_literal_array(&self) -> bool {
        matches!(self.stmt, AstStatement::Literal(AstLiteral::Array(..), ..))
    }

    pub fn is_literal(&self) -> bool {
        matches!(self.stmt, AstStatement::Literal(..))
    }

    pub fn is_literal_integer(&self) -> bool {
        matches!(self.stmt, AstStatement::Literal(AstLiteral::Integer(..), ..))
    }

    pub fn get_literal_integer_value(&self) -> Option<i128> {
        try_from!(self, AstLiteral).map(|it| it.get_literal_integer_value()).unwrap_or_default()
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.stmt, AstStatement::Identifier(..))
    }

    pub fn is_default_value(&self) -> bool {
        matches!(self.stmt, AstStatement::DefaultValue { .. })
    }

    /// Negates the given element by adding it to a not expression
    pub fn negate(self: AstNode, mut id_provider: IdProvider) -> AstNode {
        let location = self.get_location();
        AstFactory::create_not_expression(self, location, id_provider.next_id())
    }

    pub fn is_template(&self) -> bool {
        matches!(
            self.stmt,
            AstStatement::HardwareAccess(HardwareAccess { access: DirectAccessType::Template, .. })
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Multiplication,
    Exponentiation,
    Division,
    Equal,
    NotEqual,
    Modulo,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Not,
    And,
    Or,
    Xor,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiplication => "*",
            Operator::Division => "/",
            Operator::Equal => "=",
            Operator::Modulo => "MOD",
            Operator::Exponentiation => "**",
            _ => unimplemented!(),
        };
        f.write_str(symbol)
    }
}

/// enum_elements should be the statement between then enum's brackets ( )
/// e.g. x : ( this, that, etc)
pub fn get_enum_element_names(enum_elements: &AstNode) -> Vec<String> {
    flatten_expression_list(enum_elements)
        .into_iter()
        .filter(|it| matches!(it.stmt, AstStatement::Identifier(..) | AstStatement::Assignment(..)))
        .map(get_enum_element_name)
        .collect()
}

/// expects a Reference or an Assignment
pub fn get_enum_element_name(enum_element: &AstNode) -> String {
    match &enum_element.stmt {
        AstStatement::Identifier(name, ..) => name.to_string(),
        AstStatement::Assignment(Assignment { left, .. }, ..) => left
            .get_flat_reference_name()
            .map(|it| it.to_string())
            .expect("left of assignment not a reference"),
        _ => {
            unreachable!("expected {:?} to be a Reference or Assignment", enum_element);
        }
    }
}

/// flattens expression-lists and MultipliedStatements into a vec of statements.
/// It can also handle nested structures like 2(3(4,5))
pub fn flatten_expression_list(list: &AstNode) -> Vec<&AstNode> {
    match &list.stmt {
        AstStatement::ExpressionList(expressions, ..) => {
            expressions.iter().by_ref().flat_map(flatten_expression_list).collect()
        }
        AstStatement::MultipliedStatement(MultipliedStatement { multiplier, element }, ..) => {
            std::iter::repeat(flatten_expression_list(element)).take(*multiplier as usize).flatten().collect()
        }
        AstStatement::ParenExpression(expression) => flatten_expression_list(expression),
        _ => vec![list],
    }
}

pub fn steal_expression_list(list: &mut AstNode) -> Vec<AstNode> {
    match &mut list.stmt {
        AstStatement::ExpressionList(expressions, ..) => std::mem::take(expressions),
        AstStatement::ParenExpression(expression) => steal_expression_list(expression),
        _ => {
            let node = std::mem::take(list);
            vec![node]
        }
    }
}

pub fn pre_process(unit: &mut CompilationUnit, id_provider: IdProvider) {
    pre_processor::pre_process(unit, id_provider)
}
impl Operator {
    /// returns true, if this operator is a comparison operator,
    /// resulting in a bool value
    /// (=, <>, >, <, >=, <=)
    pub fn is_comparison_operator(&self) -> bool {
        matches!(
            self,
            Operator::Equal
                | Operator::NotEqual
                | Operator::Less
                | Operator::Greater
                | Operator::LessOrEqual
                | Operator::GreaterOrEqual
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ArgumentProperty, PouType, VariableBlockType};

    #[test]
    fn display_pou() {
        assert_eq!(PouType::Program.to_string(), "Program");
        assert_eq!(PouType::Function.to_string(), "Function");
        assert_eq!(PouType::FunctionBlock.to_string(), "FunctionBlock");
        assert_eq!(PouType::Action.to_string(), "Action");
        assert_eq!(PouType::Class.to_string(), "Class");
        assert_eq!(PouType::Method { parent: String::new(), property: None }.to_string(), "Method");
    }

    #[test]
    fn display_variable_block_type() {
        assert_eq!(VariableBlockType::Local.to_string(), "Local");
        assert_eq!(VariableBlockType::Temp.to_string(), "Temp");
        assert_eq!(VariableBlockType::Input(ArgumentProperty::ByVal).to_string(), "Input");
        assert_eq!(VariableBlockType::Input(ArgumentProperty::ByRef).to_string(), "Input");
        assert_eq!(VariableBlockType::Output.to_string(), "Output");
        assert_eq!(VariableBlockType::Global.to_string(), "Global");
        assert_eq!(VariableBlockType::InOut.to_string(), "InOut");
    }
}

pub struct AstFactory {}

impl AstFactory {
    pub fn create_empty_statement(location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::EmptyStatement(EmptyStatement {}), location, id }
        // AstStatement::EmptyStatement (  EmptyStatement {}, location, id }
    }

    pub fn create_return_statement(
        condition: Option<AstNode>,
        location: SourceLocation,
        id: AstId,
    ) -> AstNode {
        let condition = condition.map(Box::new);
        AstNode { stmt: AstStatement::ReturnStatement(ReturnStatement { condition }), location, id }
    }

    pub fn create_exit_statement(location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::ExitStatement(()), location, id }
    }

    pub fn create_continue_statement(location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::ContinueStatement(()), location, id }
    }

    pub fn create_case_condition(result: AstNode, location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::CaseCondition(Box::new(result)), id, location }
    }

    pub fn create_vla_range_statement(location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::VlaRangeStatement, id, location }
    }

    pub fn create_literal(kind: AstLiteral, location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::Literal(kind), id, location }
    }

    pub fn create_hardware_access(
        access: DirectAccessType,
        direction: HardwareAccessType,
        address: Vec<AstNode>,
        location: SourceLocation,
        id: usize,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::HardwareAccess(HardwareAccess { access, direction, address }),
            location,
            id,
        }
    }

    pub fn create_default_value(location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::DefaultValue(DefaultValue {}), location, id }
    }

    pub fn create_expression_list(expressions: Vec<AstNode>, location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::ExpressionList(expressions), location, id }
    }

    pub fn create_paren_expression(expression: AstNode, location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::ParenExpression(Box::new(expression)), location, id }
    }

    /// creates a new if-statement
    pub fn create_if_statement(
        blocks: Vec<ConditionalBlock>,
        else_block: Vec<AstNode>,
        location: SourceLocation,
        id: AstId,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::ControlStatement(AstControlStatement::If(IfStatement { blocks, else_block })),
            location,
            id,
        }
    }

    ///  creates a new for loop statement
    pub fn create_for_loop(
        counter: AstNode,
        start: AstNode,
        end: AstNode,
        by_step: Option<AstNode>,
        body: Vec<AstNode>,
        location: SourceLocation,
        id: AstId,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::ControlStatement(AstControlStatement::ForLoop(ForLoopStatement {
                counter: Box::new(counter),
                start: Box::new(start),
                end: Box::new(end),
                by_step: by_step.map(Box::new),
                body,
            })),
            location,
            id,
        }
    }

    /// creates a new while statement
    pub fn create_while_statement(
        condition: AstNode,
        body: Vec<AstNode>,
        location: SourceLocation,
        id: AstId,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::ControlStatement(AstControlStatement::WhileLoop(LoopStatement {
                condition: Box::new(condition),
                body,
            })),
            id,
            location,
        }
    }

    /// creates a new repeat-statement
    pub fn create_repeat_statement(
        condition: AstNode,
        body: Vec<AstNode>,
        location: SourceLocation,
        id: AstId,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::ControlStatement(AstControlStatement::RepeatLoop(LoopStatement {
                condition: Box::new(condition),
                body,
            })),
            id,
            location,
        }
    }

    /// creates a new case-statement
    pub fn create_case_statement(
        selector: AstNode,
        case_blocks: Vec<ConditionalBlock>,
        else_block: Vec<AstNode>,
        location: SourceLocation,
        id: AstId,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::ControlStatement(AstControlStatement::Case(CaseStatement {
                selector: Box::new(selector),
                case_blocks,
                else_block,
            })),
            id,
            location,
        }
    }

    /// creates an or-expression
    pub fn create_or_expression(left: AstNode, right: AstNode) -> AstNode {
        let id = left.get_id();
        let location = left.get_location().span(&right.get_location());
        AstNode {
            stmt: AstStatement::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator: Operator::Or,
            }),
            id,
            location,
        }
    }

    /// creates a not-expression
    pub fn create_not_expression(operator: AstNode, location: SourceLocation, id: usize) -> AstNode {
        AstNode {
            stmt: AstStatement::UnaryExpression(UnaryExpression {
                value: Box::new(operator),
                operator: Operator::Not,
            }),
            id,
            location,
        }
    }

    /// creates a new Identifier
    pub fn create_identifier<T, U>(name: T, location: U, id: AstId) -> AstNode
    where
        T: Into<String>,
        U: Into<SourceLocation>,
    {
        AstNode::new(AstStatement::Identifier(name.into()), id, location.into())
    }

    pub fn create_unary_expression(
        operator: Operator,
        value: AstNode,
        location: SourceLocation,
        id: AstId,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::UnaryExpression(UnaryExpression { operator, value: Box::new(value) }),
            location,
            id,
        }
    }

    pub fn create_assignment(left: AstNode, right: AstNode, id: AstId) -> AstNode {
        let location = left.location.span(&right.location);
        AstNode {
            stmt: AstStatement::Assignment(Assignment { left: Box::new(left), right: Box::new(right) }),
            id,
            location,
        }
    }

    pub fn create_output_assignment(left: AstNode, right: AstNode, id: AstId) -> AstNode {
        let location = left.location.span(&right.location);
        AstNode::new(
            AstStatement::OutputAssignment(Assignment { left: Box::new(left), right: Box::new(right) }),
            id,
            location,
        )
    }

    // TODO: Merge `create_assignment`, `create_output_assignment` and `create_ref_assignment`
    //       once the the Assignment AstStatements have been merged and a `kind` field is available
    //       I.e. something like `AstStatement::Assignment { data, kind: AssignmentKind { Normal, Output, Reference } }
    //       and then fn create_assignment(kind: AssignmentKind, ...)
    pub fn create_ref_assignment(left: AstNode, right: AstNode, id: AstId) -> AstNode {
        let location = left.location.span(&right.location);
        AstNode::new(
            AstStatement::RefAssignment(Assignment { left: Box::new(left), right: Box::new(right) }),
            id,
            location,
        )
    }

    pub fn create_member_reference(member: AstNode, base: Option<AstNode>, id: AstId) -> AstNode {
        let location = base
            .as_ref()
            .map(|it| it.get_location().span(&member.get_location()))
            .unwrap_or_else(|| member.get_location());
        AstNode {
            stmt: AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Member(Box::new(member)),
                base: base.map(Box::new),
            }),
            id,
            location,
        }
    }

    pub fn create_index_reference(
        index: AstNode,
        base: Option<AstNode>,
        id: AstId,
        location: SourceLocation,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Index(Box::new(index)),
                base: base.map(Box::new),
            }),
            id,
            location,
        }
    }

    pub fn create_address_of_reference(base: AstNode, id: AstId, location: SourceLocation) -> AstNode {
        AstNode {
            stmt: AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Address,
                base: Some(Box::new(base)),
            }),
            id,
            location,
        }
    }

    pub fn create_deref_reference(base: AstNode, id: AstId, location: SourceLocation) -> AstNode {
        AstNode {
            stmt: AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Deref,
                base: Some(Box::new(base)),
            }),
            id,
            location,
        }
    }

    pub fn create_direct_access(
        access: DirectAccessType,
        index: AstNode,
        id: AstId,
        location: SourceLocation,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::DirectAccess(DirectAccess { access, index: Box::new(index) }),
            location,
            id,
        }
    }

    /// creates a new binary statement
    pub fn create_binary_expression(left: AstNode, operator: Operator, right: AstNode, id: AstId) -> AstNode {
        let location = left.location.span(&right.location);
        AstNode {
            stmt: AstStatement::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }),
            id,
            location,
        }
    }

    /// creates a new cast statement
    pub fn create_cast_statement(
        type_name: AstNode,
        stmt: AstNode,
        location: &SourceLocation,
        id: AstId,
    ) -> AstNode {
        let new_location = location.span(&stmt.get_location());
        AstNode {
            stmt: AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Cast(Box::new(stmt)),
                base: Some(Box::new(type_name)),
            }),
            id,
            location: new_location,
        }
    }

    pub fn create_call_statement<T>(
        operator: AstNode,
        parameters: Option<AstNode>,
        id: usize,
        location: T,
    ) -> AstNode
    where
        T: Into<SourceLocation>,
    {
        AstNode {
            stmt: AstStatement::CallStatement(CallStatement {
                operator: Box::new(operator),
                parameters: parameters.map(Box::new),
            }),
            location: location.into(),
            id,
        }
    }

    /// creates a new call statement to the given function and parameters
    pub fn create_call_to(
        function_name: String,
        parameters: Vec<AstNode>,
        id: usize,
        parameter_list_id: usize,
        location: &SourceLocation,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::CallStatement(CallStatement {
                operator: Box::new(AstFactory::create_member_reference(
                    AstFactory::create_identifier(&function_name, location, id),
                    None,
                    id,
                )),
                parameters: Some(Box::new(AstNode::new(
                    AstStatement::ExpressionList(parameters),
                    parameter_list_id,
                    SourceLocation::internal(), //TODO: get real location
                ))),
            }),
            location: location.clone(),
            id,
        }
    }

    pub fn create_multiplied_statement(
        multiplier: u32,
        element: AstNode,
        location: SourceLocation,
        id: AstId,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::MultipliedStatement(MultipliedStatement {
                multiplier,
                element: Box::new(element),
            }),
            location,
            id,
        }
    }

    pub fn create_range_statement(start: AstNode, end: AstNode, id: AstId) -> AstNode {
        let location = start.location.span(&end.location);
        let data = RangeStatement { start: Box::new(start), end: Box::new(end) };
        AstNode { stmt: AstStatement::RangeStatement(data), id, location }
    }

    pub fn create_call_to_with_ids(
        function_name: &str,
        parameters: Vec<AstNode>,
        location: &SourceLocation,
        mut id_provider: IdProvider,
    ) -> AstNode {
        AstNode {
            stmt: AstStatement::CallStatement(CallStatement {
                operator: Box::new(AstFactory::create_member_reference(
                    AstFactory::create_identifier(function_name, location, id_provider.next_id()),
                    None,
                    id_provider.next_id(),
                )),
                parameters: Some(Box::new(AstFactory::create_expression_list(
                    parameters,
                    SourceLocation::internal(),
                    id_provider.next_id(),
                ))),
            }),
            location: location.clone(),
            id: id_provider.next_id(),
        }
    }

    pub fn create_call_to_check_function_ast(
        check_function_name: &str,
        parameter: AstNode,
        sub_range: Range<AstNode>,
        location: &SourceLocation,
        id_provider: IdProvider,
    ) -> AstNode {
        AstFactory::create_call_to_with_ids(
            check_function_name,
            vec![parameter, sub_range.start, sub_range.end],
            location,
            id_provider,
        )
    }

    pub fn create_jump_statement(
        condition: Box<AstNode>,
        target: Box<AstNode>,
        location: SourceLocation,
        id: AstId,
    ) -> AstNode {
        AstNode { stmt: AstStatement::JumpStatement(JumpStatement { condition, target }), location, id }
    }

    pub fn create_label_statement(name: String, location: SourceLocation, id: AstId) -> AstNode {
        AstNode { stmt: AstStatement::LabelStatement(LabelStatement { name }), location, id }
    }

    pub fn create_plus_one_expression(value: AstNode, location: SourceLocation, id: AstId) -> AstNode {
        let one = AstFactory::create_literal(AstLiteral::Integer(1), location.clone(), id);
        AstFactory::create_binary_expression(value, Operator::Plus, one, id)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct EmptyStatement {}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultValue {}

#[derive(Debug, Clone, PartialEq)]
pub struct CastStatement {
    pub target: Box<AstNode>,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultipliedStatement {
    pub multiplier: u32,
    pub element: Box<AstNode>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceExpr {
    pub access: ReferenceAccess,
    pub base: Option<Box<AstNode>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectAccess {
    pub access: DirectAccessType,
    pub index: Box<AstNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HardwareAccess {
    pub direction: HardwareAccessType,
    pub access: DirectAccessType,
    pub address: Vec<AstNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub operator: Operator,
    pub left: Box<AstNode>,
    pub right: Box<AstNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub operator: Operator,
    pub value: Box<AstNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeStatement {
    pub start: Box<AstNode>,
    pub end: Box<AstNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub left: Box<AstNode>,
    pub right: Box<AstNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallStatement {
    pub operator: Box<AstNode>,
    pub parameters: Option<Box<AstNode>>,
}

/// Represents a conditional jump from current location to a specified label
#[derive(Debug, Clone, PartialEq)]
pub struct JumpStatement {
    /// The condition based on which the current statement will perform a jump
    pub condition: Box<AstNode>,
    /// The target location (Label) the statement will jump to
    pub target: Box<AstNode>,
}

/// Represents a location in code that could be jumbed to
#[derive(Debug, Clone, PartialEq)]
pub struct LabelStatement {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Allocation {
    pub name: String,
    pub reference_type: String,
}

impl HardwareAccess {
    pub fn get_mangled_variable_name(&self) -> String {
        let HardwareAccess { direction, address, .. } = self;
        let direction = match direction {
            HardwareAccessType::Input | HardwareAccessType::Output => "PI",
            HardwareAccessType::Memory => "M",
            HardwareAccessType::Global => "G",
        };
        format!(
            "__{direction}_{}",
            address
                .iter()
                .flat_map(|node| node.get_literal_integer_value())
                .map(|val| val.to_string())
                .collect::<Vec<_>>()
                .join("_")
        )
    }

    pub fn is_template(&self) -> bool {
        matches!(self.access, DirectAccessType::Template)
    }
}
