// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use serde::{Deserialize, Serialize};

use crate::{
    control_statements::{
        AstControlStatement, CaseStatement, ConditionalBlock, ForLoopStatement, IfStatement, LoopStatement,
    },
    literals::{AstLiteral, StringValue},
    pre_processor,
    provider::IdProvider,
};

use plc_source::source_location::*;

pub type AstId = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenericBinding {
    pub name: String,
    pub nature: TypeNature,
}

#[derive(PartialEq)]
pub struct Pou {
    pub name: String,
    pub variable_blocks: Vec<VariableBlock>,
    pub pou_type: PouType,
    pub return_type: Option<DataTypeDeclaration>,
    /// the SourceLocation of the whole POU
    pub location: SourceLocation,
    /// the SourceLocation of the POU's name
    pub name_location: SourceLocation,
    pub poly_mode: Option<PolymorphismMode>,
    pub generics: Vec<GenericBinding>,
    pub linkage: LinkageType,
    pub super_class: Option<String>,
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
            .field("pou_type", &self.pou_type)
            .field("return_type", &self.return_type);
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
}

#[derive(Debug, PartialEq)]
pub struct Implementation {
    pub name: String,
    pub type_name: String,
    pub linkage: LinkageType,
    pub pou_type: PouType,
    pub statements: Vec<AstStatement>,
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

#[derive(Debug, PartialEq, Eq)]
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
    Method { owner_class: String },
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
        }
    }
}

impl PouType {
    /// returns Some(owner_class) if this is a `Method` or otherwhise `None`
    pub fn get_optional_owner_class(&self) -> Option<String> {
        if let PouType::Method { owner_class } = self {
            Some(owner_class.clone())
        } else {
            None
        }
    }
}
/**
 * A datastructure that stores the location of newline characters of a string.
 * It also offers some useful methods to determine the line-number of an offset-location.
 */
#[derive(Debug, PartialEq, Eq)]
pub struct NewLines {
    line_breaks: Vec<usize>,
}

impl NewLines {
    pub fn build(str: &str) -> NewLines {
        let mut line_breaks = Vec::new();
        let mut total_offset: usize = 0;
        if !str.is_empty() {
            // Instead of using ´lines()´ we split at \n to preserve the offsets if a \r exists
            for l in str.split('\n') {
                total_offset += l.len() + 1;
                line_breaks.push(total_offset);
            }
        }
        NewLines { line_breaks }
    }

    ///
    /// returns the 0 based line-nr of the given offset-location
    ///
    pub fn get_line_nr(&self, offset: usize) -> u32 {
        (match self.line_breaks.binary_search(&offset) {
            //In case we hit an exact match, we just found the first character of a new line, we must add one to the result
            Ok(line) => line + 1,
            Err(line) => line,
        }) as u32
    }

    ///
    /// returns the 0 based column of the given offset-location
    ///
    pub fn get_column(&self, line: u32, offset: usize) -> u32 {
        (if line > 0 {
            self.line_breaks.get((line - 1) as usize).map(|l| offset - *l).unwrap_or(0)
        } else {
            offset
        }) as u32
    }
}

#[derive(Debug, PartialEq)]
pub struct CompilationUnit {
    pub global_vars: Vec<VariableBlock>,
    pub units: Vec<Pou>,
    pub implementations: Vec<Implementation>,
    pub user_types: Vec<UserTypeDeclaration>,
    pub file_name: String,
    pub new_lines: NewLines,
}

impl CompilationUnit {
    pub fn new(file_name: &str, new_lines: NewLines) -> Self {
        CompilationUnit {
            global_vars: Vec::new(),
            units: Vec::new(),
            implementations: Vec::new(),
            user_types: Vec::new(),
            file_name: file_name.to_string(),
            new_lines,
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
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum ArgumentProperty {
    ByVal,
    ByRef,
}

#[derive(PartialEq)]
pub struct VariableBlock {
    pub access: AccessModifier,
    pub constant: bool,
    pub retain: bool,
    pub variables: Vec<Variable>,
    pub variable_block_type: VariableBlockType,
    pub linkage: LinkageType,
    pub location: SourceLocation,
}

impl Debug for VariableBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariableBlock")
            .field("variables", &self.variables)
            .field("variable_block_type", &self.variable_block_type)
            .finish()
    }
}

#[derive(Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub data_type_declaration: DataTypeDeclaration,
    pub initializer: Option<AstStatement>,
    pub address: Option<AstStatement>,
    pub location: SourceLocation,
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
}

pub trait DiagnosticInfo {
    fn get_description(&self) -> String;
    fn get_location(&self) -> SourceLocation;
}

impl DiagnosticInfo for AstStatement {
    fn get_description(&self) -> String {
        format!("{self:?}")
    }

    fn get_location(&self) -> SourceLocation {
        self.get_location()
    }
}

#[derive(Clone, PartialEq)]
pub enum DataTypeDeclaration {
    DataTypeReference { referenced_type: String, location: SourceLocation },
    DataTypeDefinition { data_type: DataType, location: SourceLocation, scope: Option<String> },
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
        }
    }
}

impl DataTypeDeclaration {
    pub fn get_name(&self) -> Option<&str> {
        match self {
            DataTypeDeclaration::DataTypeReference { referenced_type, .. } => Some(referenced_type.as_str()),
            DataTypeDeclaration::DataTypeDefinition { data_type, .. } => data_type.get_name(),
        }
    }

    pub fn get_location(&self) -> SourceLocation {
        match self {
            DataTypeDeclaration::DataTypeReference { location, .. } => location.clone(),
            DataTypeDeclaration::DataTypeDefinition { location, .. } => location.clone(),
        }
    }

    pub fn get_referenced_type(&self) -> Option<String> {
        let DataTypeDeclaration::DataTypeReference {referenced_type, ..} = self else { return None };
        Some(referenced_type.to_owned())
    }
}

#[derive(PartialEq)]
pub struct UserTypeDeclaration {
    pub data_type: DataType,
    pub initializer: Option<AstStatement>,
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
        elements: AstStatement, //a single Ref, or an ExpressionList with Refs
    },
    SubRangeType {
        name: Option<String>,
        referenced_type: String,
        bounds: Option<AstStatement>,
    },
    ArrayType {
        name: Option<String>,
        bounds: AstStatement,
        referenced_type: Box<DataTypeDeclaration>,
        is_variable_length: bool,
    },
    PointerType {
        name: Option<String>,
        referenced_type: Box<DataTypeDeclaration>,
    },
    StringType {
        name: Option<String>,
        is_wide: bool, //WSTRING
        size: Option<AstStatement>,
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

#[derive(Clone, PartialEq, Debug)]
pub enum ReferenceAccess {
    /**
     * a, a.b
     */
    Member(Box<AstStatement>),
    /**
     * a[3]
     */
    Index(Box<AstStatement>),
    /**
     * Color#Red
     */
    Cast(Box<AstStatement>),
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
pub enum AstStatement {
    EmptyStatement {
        location: SourceLocation,
        id: AstId,
    },
    // a placeholder that indicates a default value of a datatype
    DefaultValue {
        location: SourceLocation,
        id: AstId,
    },
    // Literals
    Literal {
        kind: AstLiteral,
        location: SourceLocation,
        id: AstId,
    },

    CastStatement {
        target: Box<AstStatement>,
        type_name: String,
        location: SourceLocation,
        id: AstId,
    },
    MultipliedStatement {
        multiplier: u32,
        element: Box<AstStatement>,
        location: SourceLocation,
        id: AstId,
    },
    // Expressions
    ReferenceExpr {
        access: ReferenceAccess,
        base: Option<Box<AstStatement>>,
        id: AstId,
        location: SourceLocation,
    },
    Identifier {
        name: String,
        location: SourceLocation,
        id: AstId,
    },
    DirectAccess {
        access: DirectAccessType,
        index: Box<AstStatement>,
        location: SourceLocation,
        id: AstId,
    },
    HardwareAccess {
        direction: HardwareAccessType,
        access: DirectAccessType,
        address: Vec<AstStatement>,
        location: SourceLocation,
        id: AstId,
    },
    BinaryExpression {
        operator: Operator,
        left: Box<AstStatement>,
        right: Box<AstStatement>,
        id: AstId,
    },
    UnaryExpression {
        operator: Operator,
        value: Box<AstStatement>,
        location: SourceLocation,
        id: AstId,
    },
    ExpressionList {
        expressions: Vec<AstStatement>,
        id: AstId,
    },
    RangeStatement {
        id: AstId,
        start: Box<AstStatement>,
        end: Box<AstStatement>,
    },
    VlaRangeStatement {
        id: AstId,
    },
    // Assignment
    Assignment {
        left: Box<AstStatement>,
        right: Box<AstStatement>,
        id: AstId,
    },
    // OutputAssignment
    OutputAssignment {
        left: Box<AstStatement>,
        right: Box<AstStatement>,
        id: AstId,
    },
    //Call Statement
    CallStatement {
        operator: Box<AstStatement>,
        parameters: Box<Option<AstStatement>>,
        location: SourceLocation,
        id: AstId,
    },
    // Control Statements
    ControlStatement {
        kind: AstControlStatement,
        location: SourceLocation,
        id: AstId,
    },

    CaseCondition {
        condition: Box<AstStatement>,
        id: AstId,
    },
    ExitStatement {
        location: SourceLocation,
        id: AstId,
    },
    ContinueStatement {
        location: SourceLocation,
        id: AstId,
    },
    ReturnStatement {
        location: SourceLocation,
        id: AstId,
    },
}

impl Debug for AstStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AstStatement::EmptyStatement { .. } => f.debug_struct("EmptyStatement").finish(),
            AstStatement::DefaultValue { .. } => f.debug_struct("DefaultValue").finish(),
            AstStatement::Literal { kind, .. } => kind.fmt(f),
            AstStatement::Identifier { name, .. } => {
                f.debug_struct("Identifier").field("name", name).finish()
            }
            AstStatement::BinaryExpression { operator, left, right, .. } => f
                .debug_struct("BinaryExpression")
                .field("operator", operator)
                .field("left", left)
                .field("right", right)
                .finish(),
            AstStatement::UnaryExpression { operator, value, .. } => {
                f.debug_struct("UnaryExpression").field("operator", operator).field("value", value).finish()
            }
            AstStatement::ExpressionList { expressions, .. } => {
                f.debug_struct("ExpressionList").field("expressions", expressions).finish()
            }
            AstStatement::RangeStatement { start, end, .. } => {
                f.debug_struct("RangeStatement").field("start", start).field("end", end).finish()
            }
            AstStatement::VlaRangeStatement { .. } => f.debug_struct("VlaRangeStatement").finish(),
            AstStatement::Assignment { left, right, .. } => {
                f.debug_struct("Assignment").field("left", left).field("right", right).finish()
            }
            AstStatement::OutputAssignment { left, right, .. } => {
                f.debug_struct("OutputAssignment").field("left", left).field("right", right).finish()
            }
            AstStatement::CallStatement { operator, parameters, .. } => f
                .debug_struct("CallStatement")
                .field("operator", operator)
                .field("parameters", parameters)
                .finish(),
            AstStatement::ControlStatement {
                kind: AstControlStatement::If(IfStatement { blocks, else_block, .. }),
                ..
            } => {
                f.debug_struct("IfStatement").field("blocks", blocks).field("else_block", else_block).finish()
            }
            AstStatement::ControlStatement {
                kind:
                    AstControlStatement::ForLoop(ForLoopStatement { counter, start, end, by_step, body, .. }),
                ..
            } => f
                .debug_struct("ForLoopStatement")
                .field("counter", counter)
                .field("start", start)
                .field("end", end)
                .field("by_step", by_step)
                .field("body", body)
                .finish(),
            AstStatement::ControlStatement {
                kind: AstControlStatement::WhileLoop(LoopStatement { condition, body, .. }),
                ..
            } => f
                .debug_struct("WhileLoopStatement")
                .field("condition", condition)
                .field("body", body)
                .finish(),
            AstStatement::ControlStatement {
                kind: AstControlStatement::RepeatLoop(LoopStatement { condition, body, .. }),
                ..
            } => f
                .debug_struct("RepeatLoopStatement")
                .field("condition", condition)
                .field("body", body)
                .finish(),
            AstStatement::ControlStatement {
                kind: AstControlStatement::Case(CaseStatement { selector, case_blocks, else_block, .. }),
                ..
            } => f
                .debug_struct("CaseStatement")
                .field("selector", selector)
                .field("case_blocks", case_blocks)
                .field("else_block", else_block)
                .finish(),
            AstStatement::DirectAccess { access, index, .. } => {
                f.debug_struct("DirectAccess").field("access", access).field("index", index).finish()
            }
            AstStatement::HardwareAccess { direction, access, address, location, .. } => f
                .debug_struct("HardwareAccess")
                .field("direction", direction)
                .field("access", access)
                .field("address", address)
                .field("location", location)
                .finish(),
            AstStatement::MultipliedStatement { multiplier, element, .. } => f
                .debug_struct("MultipliedStatement")
                .field("multiplier", multiplier)
                .field("element", element)
                .finish(),
            AstStatement::CaseCondition { condition, .. } => {
                f.debug_struct("CaseCondition").field("condition", condition).finish()
            }
            AstStatement::ReturnStatement { .. } => f.debug_struct("ReturnStatement").finish(),
            AstStatement::ContinueStatement { .. } => f.debug_struct("ContinueStatement").finish(),
            AstStatement::ExitStatement { .. } => f.debug_struct("ExitStatement").finish(),
            AstStatement::CastStatement { target, type_name, .. } => {
                f.debug_struct("CastStatement").field("type_name", type_name).field("target", target).finish()
            }
            AstStatement::ReferenceExpr { access, base, .. } => {
                f.debug_struct("ReferenceExpr").field("kind", access).field("base", base).finish()
            }
        }
    }
}

impl AstStatement {
    ///Returns the statement in a singleton list, or the contained statements if the statement is already a list
    pub fn get_as_list(&self) -> Vec<&AstStatement> {
        if let AstStatement::ExpressionList { expressions, .. } = self {
            expressions.iter().collect::<Vec<&AstStatement>>()
        } else {
            vec![self]
        }
    }
    pub fn get_location(&self) -> SourceLocation {
        match self {
            AstStatement::EmptyStatement { location, .. } => location.clone(),
            AstStatement::DefaultValue { location, .. } => location.clone(),
            AstStatement::Literal { location, .. } => location.clone(),
            AstStatement::Identifier { location, .. } => location.clone(),
            AstStatement::BinaryExpression { left, right, .. } => {
                let left_loc = left.get_location();
                let right_loc = right.get_location();
                left_loc.span(&right_loc)
            }
            AstStatement::UnaryExpression { location, .. } => location.clone(),
            AstStatement::ExpressionList { expressions, .. } => {
                let first =
                    expressions.first().map_or_else(SourceLocation::undefined, |it| it.get_location());
                let last = expressions.last().map_or_else(SourceLocation::undefined, |it| it.get_location());
                first.span(&last)
            }
            AstStatement::RangeStatement { start, end, .. } => {
                let start_loc = start.get_location();
                let end_loc = end.get_location();
                start_loc.span(&end_loc)
            }
            AstStatement::VlaRangeStatement { .. } => SourceLocation::undefined(), // internal type only
            AstStatement::Assignment { left, right, .. } => {
                let left_loc = left.get_location();
                let right_loc = right.get_location();
                left_loc.span(&right_loc)
            }
            AstStatement::OutputAssignment { left, right, .. } => {
                let left_loc = left.get_location();
                let right_loc = right.get_location();
                left_loc.span(&right_loc)
            }
            AstStatement::CallStatement { location, .. } => location.clone(),
            AstStatement::ControlStatement { location, .. } => location.clone(),
            AstStatement::DirectAccess { location, .. } => location.clone(),
            AstStatement::HardwareAccess { location, .. } => location.clone(),
            AstStatement::MultipliedStatement { location, .. } => location.clone(),
            AstStatement::CaseCondition { condition, .. } => condition.get_location(),
            AstStatement::ReturnStatement { location, .. } => location.clone(),
            AstStatement::ContinueStatement { location, .. } => location.clone(),
            AstStatement::ExitStatement { location, .. } => location.clone(),
            AstStatement::CastStatement { location, .. } => location.clone(),
            AstStatement::ReferenceExpr { location, .. } => location.clone(),
        }
    }

    pub fn get_id(&self) -> AstId {
        match self {
            AstStatement::EmptyStatement { id, .. } => *id,
            AstStatement::DefaultValue { id, .. } => *id,
            AstStatement::Literal { id, .. } => *id,
            AstStatement::MultipliedStatement { id, .. } => *id,
            AstStatement::Identifier { id, .. } => *id,
            AstStatement::DirectAccess { id, .. } => *id,
            AstStatement::HardwareAccess { id, .. } => *id,
            AstStatement::BinaryExpression { id, .. } => *id,
            AstStatement::UnaryExpression { id, .. } => *id,
            AstStatement::ExpressionList { id, .. } => *id,
            AstStatement::RangeStatement { id, .. } => *id,
            AstStatement::VlaRangeStatement { id, .. } => *id,
            AstStatement::Assignment { id, .. } => *id,
            AstStatement::OutputAssignment { id, .. } => *id,
            AstStatement::CallStatement { id, .. } => *id,
            AstStatement::ControlStatement { id, .. } => *id,
            AstStatement::CaseCondition { id, .. } => *id,
            AstStatement::ReturnStatement { id, .. } => *id,
            AstStatement::ContinueStatement { id, .. } => *id,
            AstStatement::ExitStatement { id, .. } => *id,
            AstStatement::CastStatement { id, .. } => *id,
            AstStatement::ReferenceExpr { id, .. } => *id,
        }
    }

    /// Returns true if the current statement has a direct access.
    pub fn has_direct_access(&self) -> bool {
        match self {
            AstStatement::ReferenceExpr { access: ReferenceAccess::Member(reference), base, .. }
            | AstStatement::ReferenceExpr { access: ReferenceAccess::Cast(reference), base, .. } => {
                reference.has_direct_access()
                    || base.as_ref().map(|it| it.has_direct_access()).unwrap_or(false)
            }
            AstStatement::DirectAccess { .. } => true,
            _ => false,
        }
    }

    /// returns true if this AST Statement is a literal or reference that can be
    /// prefixed with a type-cast (e.g. INT#23)
    pub fn is_cast_prefix_eligible(&self) -> bool {
        // TODO: figure out a better name for this...
        match self {
            AstStatement::Literal { kind, .. } => kind.is_cast_prefix_eligible(),
            AstStatement::Identifier { .. } => true,
            _ => false,
        }
    }

    /// Returns true if the current statement is a flat reference (e.g. `a`)
    pub fn is_flat_reference(&self) -> bool {
        matches!(self, AstStatement::Identifier { .. }) || {
            if let AstStatement::ReferenceExpr {
                access: ReferenceAccess::Member(reference),
                base: None,
                ..
            } = self
            {
                matches!(reference.as_ref(), AstStatement::Identifier { .. })
            } else {
                false
            }
        }
    }

    /// Returns the reference-name if this is a flat reference like `a`, or None if this is no flat reference
    pub fn get_flat_reference_name(&self) -> Option<&str> {
        match self {
            AstStatement::ReferenceExpr { access: ReferenceAccess::Member(reference), .. } => {
                if let AstStatement::Identifier { name, .. } = reference.as_ref() {
                    Some(name)
                } else {
                    None
                }
            }
            AstStatement::Identifier { name, .. } => Some(name),
            _ => None,
        }
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, AstStatement::ReferenceExpr { .. })
    }

    pub fn is_hardware_access(&self) -> bool {
        matches!(self, AstStatement::HardwareAccess { .. })
    }

    pub fn is_array_access(&self) -> bool {
        matches!(self, AstStatement::ReferenceExpr { access: ReferenceAccess::Index(_), .. })
    }

    pub fn is_pointer_access(&self) -> bool {
        matches!(self, AstStatement::ReferenceExpr { access: ReferenceAccess::Deref, .. })
    }

    pub fn can_be_assigned_to(&self) -> bool {
        self.has_direct_access()
            || self.is_flat_reference()
            || self.is_reference()
            || self.is_array_access()
            || self.is_pointer_access()
            || self.is_hardware_access()
    }

    pub fn new_literal(kind: AstLiteral, id: AstId, location: SourceLocation) -> Self {
        AstStatement::Literal { kind, id, location }
    }

    pub fn new_integer(value: i128, id: AstId, location: SourceLocation) -> Self {
        AstStatement::Literal { kind: AstLiteral::Integer(value), location, id }
    }

    pub fn new_real(value: String, id: AstId, location: SourceLocation) -> Self {
        AstStatement::Literal { kind: AstLiteral::Real(value), location, id }
    }

    pub fn new_string(value: impl Into<String>, is_wide: bool, id: AstId, location: SourceLocation) -> Self {
        AstStatement::Literal {
            kind: AstLiteral::String(StringValue { value: value.into(), is_wide }),
            location,
            id,
        }
    }

    /// Returns true if the given token is an integer or float and zero.
    pub fn is_zero(&self) -> bool {
        match self {
            AstStatement::Literal { kind, .. } => match kind {
                AstLiteral::Integer(0) => true,
                AstLiteral::Real(val) => val == "0" || val == "0.0",
                _ => false,
            },

            _ => false,
        }
    }

    pub fn is_binary_expression(&self) -> bool {
        matches!(self, AstStatement::BinaryExpression { .. })
    }

    pub fn is_literal_array(&self) -> bool {
        matches!(self, AstStatement::Literal { kind: AstLiteral::Array(..), .. })
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, AstStatement::Literal { .. })
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
pub fn get_enum_element_names(enum_elements: &AstStatement) -> Vec<String> {
    flatten_expression_list(enum_elements)
        .into_iter()
        .filter(|it| matches!(it, AstStatement::Identifier { .. } | AstStatement::Assignment { .. }))
        .map(get_enum_element_name)
        .collect()
}

/// expects a Reference or an Assignment
pub fn get_enum_element_name(enum_element: &AstStatement) -> String {
    match enum_element {
        AstStatement::Identifier { name, .. } => name.to_string(),
        AstStatement::Assignment { left, .. } => left
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
pub fn flatten_expression_list(list: &AstStatement) -> Vec<&AstStatement> {
    match list {
        AstStatement::ExpressionList { expressions, .. } => {
            expressions.iter().by_ref().flat_map(flatten_expression_list).collect()
        }
        AstStatement::MultipliedStatement { multiplier, element, .. } => {
            std::iter::repeat(flatten_expression_list(element)).take(*multiplier as usize).flatten().collect()
        }
        _ => vec![list],
    }
}

pub fn pre_process(unit: &mut CompilationUnit, id_provider: IdProvider) {
    pre_processor::pre_process(unit, id_provider)
}
impl Operator {
    /// returns true, if this operator results in a bool value
    pub fn is_bool_type(&self) -> bool {
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

    /// returns true, if this operator is a comparison operator
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
        assert_eq!(PouType::Method { owner_class: "...".to_string() }.to_string(), "Method");
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
    pub fn empty_statement(location: SourceLocation, id: AstId) -> AstStatement {
        AstStatement::EmptyStatement { location, id }
    }

    /// creates a new if-statement
    pub fn create_if_statement(
        blocks: Vec<ConditionalBlock>,
        else_block: Vec<AstStatement>,
        location: SourceLocation,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::If(IfStatement { blocks, else_block }),
            location,
            id,
        }
    }

    ///  creates a new for loop statement
    pub fn create_for_loop(
        counter: AstStatement,
        start: AstStatement,
        end: AstStatement,
        by_step: Option<AstStatement>,
        body: Vec<AstStatement>,
        location: SourceLocation,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::ForLoop(ForLoopStatement {
                counter: Box::new(counter),
                start: Box::new(start),
                end: Box::new(end),
                by_step: by_step.map(Box::new),
                body,
            }),
            location,
            id,
        }
    }

    /// creates a new while statement
    pub fn create_while_statement(
        condition: AstStatement,
        body: Vec<AstStatement>,
        location: SourceLocation,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::WhileLoop(LoopStatement { condition: Box::new(condition), body }),
            id,
            location,
        }
    }

    /// creates a new repeat-statement
    pub fn create_repeat_statement(
        condition: AstStatement,
        body: Vec<AstStatement>,
        location: SourceLocation,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::RepeatLoop(LoopStatement { condition: Box::new(condition), body }),
            id,
            location,
        }
    }

    /// creates a new case-statement
    pub fn create_case_statement(
        selector: AstStatement,
        case_blocks: Vec<ConditionalBlock>,
        else_block: Vec<AstStatement>,
        location: SourceLocation,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::Case(CaseStatement {
                selector: Box::new(selector),
                case_blocks,
                else_block,
            }),
            id,
            location,
        }
    }

    /// creates an or-expression
    pub fn create_or_expression(left: AstStatement, right: AstStatement) -> AstStatement {
        AstStatement::BinaryExpression {
            id: left.get_id(),
            left: Box::new(left),
            right: Box::new(right),
            operator: Operator::Or,
        }
    }

    /// creates a not-expression
    pub fn create_not_expression(operator: AstStatement, location: SourceLocation) -> AstStatement {
        AstStatement::UnaryExpression {
            id: operator.get_id(),
            value: Box::new(operator),
            location,
            operator: Operator::Not,
        }
    }

    /// creates a new Identifier
    pub fn create_identifier(name: &str, location: &SourceLocation, id: AstId) -> AstStatement {
        AstStatement::Identifier { id, location: location.clone(), name: name.to_string() }
    }

    pub fn create_member_reference(
        member: AstStatement,
        base: Option<AstStatement>,
        id: AstId,
    ) -> AstStatement {
        let location = base
            .as_ref()
            .map(|it| it.get_location().span(&member.get_location()))
            .unwrap_or_else(|| member.get_location());
        AstStatement::ReferenceExpr {
            access: ReferenceAccess::Member(Box::new(member)),
            base: base.map(Box::new),
            id,
            location,
        }
    }

    pub fn create_index_reference(
        index: AstStatement,
        base: Option<AstStatement>,
        id: AstId,
        location: SourceLocation,
    ) -> AstStatement {
        AstStatement::ReferenceExpr {
            access: ReferenceAccess::Index(Box::new(index)),
            base: base.map(Box::new),
            id,
            location,
        }
    }

    pub fn create_address_of_reference(
        base: AstStatement,
        id: AstId,
        location: SourceLocation,
    ) -> AstStatement {
        AstStatement::ReferenceExpr {
            access: ReferenceAccess::Address,
            base: Some(Box::new(base)),
            id,
            location,
        }
    }

    pub fn create_deref_reference(base: AstStatement, id: AstId, location: SourceLocation) -> AstStatement {
        AstStatement::ReferenceExpr {
            access: ReferenceAccess::Deref,
            base: Some(Box::new(base)),
            id,
            location,
        }
    }

    pub fn create_direct_access(
        access: DirectAccessType,
        index: AstStatement,
        id: AstId,
        location: SourceLocation,
    ) -> AstStatement {
        AstStatement::DirectAccess { access, index: Box::new(index), location, id }
    }

    /// creates a new binary statement
    pub fn create_binary_expression(
        left: AstStatement,
        operator: Operator,
        right: AstStatement,
        id: AstId,
    ) -> AstStatement {
        AstStatement::BinaryExpression { id, left: Box::new(left), operator, right: Box::new(right) }
    }

    /// creates a new cast statement
    pub fn create_cast_statement(
        type_name: AstStatement,
        stmt: AstStatement,
        location: &SourceLocation,
        id: AstId,
    ) -> AstStatement {
        let new_location = (location.get_start()..stmt.get_location().get_end()).into();
        AstStatement::ReferenceExpr {
            access: ReferenceAccess::Cast(Box::new(stmt)),
            base: Some(Box::new(type_name)),
            id,
            location: new_location,
        }
    }

    /// creates a new call statement to the given function and parameters
    pub fn create_call_to(
        function_name: String,
        parameters: Vec<AstStatement>,
        id: usize,
        parameter_list_id: usize,
        location: &SourceLocation,
    ) -> AstStatement {
        AstStatement::CallStatement {
            operator: Box::new(AstFactory::create_member_reference(
                AstFactory::create_identifier(&function_name, location, id),
                None,
                id,
            )),
            parameters: Box::new(Some(AstStatement::ExpressionList {
                expressions: parameters,
                id: parameter_list_id,
            })),
            location: location.clone(),
            id,
        }
    }

    pub fn create_call_to_with_ids(
        function_name: &str,
        parameters: Vec<AstStatement>,
        location: &SourceLocation,
        mut id_provider: IdProvider,
    ) -> AstStatement {
        AstStatement::CallStatement {
            operator: Box::new(AstFactory::create_member_reference(
                AstFactory::create_identifier(function_name, location, id_provider.next_id()),
                None,
                id_provider.next_id(),
            )),
            parameters: Box::new(Some(AstStatement::ExpressionList {
                expressions: parameters,
                id: id_provider.next_id(),
            })),
            location: location.clone(),
            id: id_provider.next_id(),
        }
    }

    pub fn create_call_to_check_function_ast(
        check_function_name: &str,
        parameter: AstStatement,
        sub_range: Range<AstStatement>,
        location: &SourceLocation,
        id_provider: IdProvider,
    ) -> AstStatement {
        AstFactory::create_call_to_with_ids(
            check_function_name,
            vec![parameter, sub_range.start, sub_range.end],
            location,
            id_provider,
        )
    }
}
