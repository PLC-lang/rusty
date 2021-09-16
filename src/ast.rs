// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::typesystem::DataTypeInformation;
use std::{
    fmt::{Debug, Display, Formatter, Result},
    iter,
    ops::Range,
    result, unimplemented,
};
mod pre_processor;

pub type AstId = usize;

#[derive(PartialEq)]
pub struct Pou {
    pub name: String,
    pub variable_blocks: Vec<VariableBlock>,
    pub pou_type: PouType,
    pub return_type: Option<DataTypeDeclaration>,
    pub location: SourceRange,
    pub poly_mode: Option<PolymorphismMode>,
}

#[derive(Debug, PartialEq)]
pub enum PolymorphismMode {
    None,
    Abstract,
    Final,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DirectAccess {
    Bit,
    Byte,
    Word,
    DWord,
}

impl DirectAccess {
    /// Returns true if the current index is in the range for the given type
    pub fn is_in_range(&self, index: u32, data_type: &DataTypeInformation) -> bool {
        self.to_bits(index) < data_type.get_size()
    }

    /// Returns the range from 0 for the given data type
    pub fn get_range(&self, data_type: &DataTypeInformation) -> Range<u32> {
        0..((data_type.get_size() / self.get_bit_witdh()) - 1)
    }

    /// Returns true if the direct access can be used for the given type
    pub fn is_compatible(&self, data_type: &DataTypeInformation) -> bool {
        data_type.get_size() > self.get_bit_witdh()
    }

    /// Returns the size of the bitaccess result
    pub fn get_bit_witdh(&self) -> u32 {
        match self {
            DirectAccess::Bit => 1,
            DirectAccess::Byte => 8,
            DirectAccess::Word => 16,
            DirectAccess::DWord => 32,
        }
    }

    /// Converts the given index to the apporpiate bit size
    pub fn to_bits(&self, index: u32) -> u32 {
        index * self.get_bit_witdh()
    }
}

impl Debug for Pou {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("POU")
            .field("name", &self.name)
            .field("variable_blocks", &self.variable_blocks)
            .field("pou_type", &self.pou_type)
            .field("return_type", &self.return_type)
            .finish()
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
    pub location: SourceRange,
    pub overriding: bool,
    pub access: Option<AccessModifier>,
}

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum LinkageType {
    Internal,
    External,
}

#[derive(Debug, PartialEq)]
pub enum AccessModifier {
    Private,
    Public,
    Protected, // default
    Internal,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PouType {
    Program,
    Function,
    FunctionBlock,
    Action,
    Class,
    Method { owner_class: String },
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

#[derive(Debug, PartialEq)]
pub struct CompilationUnit {
    pub global_vars: Vec<VariableBlock>,
    pub units: Vec<Pou>,
    pub implementations: Vec<Implementation>,
    pub types: Vec<UserTypeDeclaration>,
}

impl CompilationUnit {
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
        self.types.extend(other.types);
    }
}

impl Default for CompilationUnit {
    fn default() -> Self {
        CompilationUnit {
            global_vars: Vec::new(),
            units: Vec::new(),
            implementations: Vec::new(),
            types: Vec::new(),
        }
    }
}

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum VariableBlockType {
    Local,
    Temp,
    Input,
    Output,
    Global,
    InOut,
}

#[derive(PartialEq)]
pub struct VariableBlock {
    pub access: AccessModifier,
    pub constant: bool,
    pub retain: bool,
    pub variables: Vec<Variable>,
    pub variable_block_type: VariableBlockType,
}

impl Debug for VariableBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("VariableBlock")
            .field("variables", &self.variables)
            .field("variable_block_type", &self.variable_block_type)
            .finish()
    }
}

#[derive(Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub data_type: DataTypeDeclaration,
    pub initializer: Option<AstStatement>,
    pub location: SourceRange,
}

impl Debug for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.initializer.is_some() {
            f.debug_struct("Variable")
                .field("name", &self.name)
                .field("data_type", &self.data_type)
                .field("initializer", &self.initializer)
                .finish()
        } else {
            f.debug_struct("Variable")
                .field("name", &self.name)
                .field("data_type", &self.data_type)
                .finish()
        }
    }
}

impl Variable {
    pub fn replace_data_type_with_reference_to(
        &mut self,
        type_name: String,
    ) -> DataTypeDeclaration {
        let new_data_type = DataTypeDeclaration::DataTypeReference {
            referenced_type: type_name,
            location: self.data_type.get_location(),
        };
        std::mem::replace(&mut self.data_type, new_data_type)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceRange {
    range: core::ops::Range<usize>,
}

impl SourceRange {
    pub fn new(range: core::ops::Range<usize>) -> SourceRange {
        SourceRange { range }
    }

    pub fn undefined() -> SourceRange {
        SourceRange { range: 0..0 }
    }

    pub fn get_start(&self) -> usize {
        self.range.start
    }

    pub fn get_end(&self) -> usize {
        self.range.end
    }

    pub fn sub_range(&self, start: usize, len: usize) -> SourceRange {
        SourceRange::new((self.get_start() + start)..(self.get_start() + len))
    }

    pub fn to_range(&self) -> Range<usize> {
        self.range.clone()
    }
}

impl From<std::ops::Range<usize>> for SourceRange {
    fn from(range: std::ops::Range<usize>) -> SourceRange {
        SourceRange::new(range)
    }
}

#[derive(Clone, PartialEq)]
pub enum DataTypeDeclaration {
    DataTypeReference {
        referenced_type: String,
        location: SourceRange,
    },
    DataTypeDefinition {
        data_type: DataType,
        location: SourceRange,
    },
}

impl Debug for DataTypeDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DataTypeDeclaration::DataTypeReference {
                referenced_type, ..
            } => f
                .debug_struct("DataTypeReference")
                .field("referenced_type", referenced_type)
                .finish(),
            DataTypeDeclaration::DataTypeDefinition { data_type, .. } => f
                .debug_struct("DataTypeDefinition")
                .field("data_type", data_type)
                .finish(),
        }
    }
}

impl DataTypeDeclaration {
    pub fn get_name(&self) -> Option<&str> {
        match self {
            DataTypeDeclaration::DataTypeReference {
                referenced_type, ..
            } => Some(referenced_type.as_str()),
            DataTypeDeclaration::DataTypeDefinition { data_type, .. } => data_type.get_name(),
        }
    }

    pub fn get_location(&self) -> SourceRange {
        match self {
            DataTypeDeclaration::DataTypeReference { location, .. } => location.clone(),
            DataTypeDeclaration::DataTypeDefinition { location, .. } => location.clone(),
        }
    }
}

#[derive(PartialEq)]
pub struct UserTypeDeclaration {
    pub data_type: DataType,
    pub initializer: Option<AstStatement>,
    pub location: SourceRange,
}

impl Debug for UserTypeDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("UserTypeDeclaration")
            .field("data_type", &self.data_type)
            .field("initializer", &self.initializer)
            .finish()
    }
}

#[derive(Clone, PartialEq)]
pub enum DataType {
    StructType {
        name: Option<String>, //maybe None for inline structs
        variables: Vec<Variable>,
    },
    EnumType {
        name: Option<String>, //maybe empty for inline enums
        elements: Vec<String>,
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
    },
}

impl Debug for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DataType::StructType { name, variables } => f
                .debug_struct("StructType")
                .field("name", name)
                .field("variables", variables)
                .finish(),
            DataType::EnumType { name, elements } => f
                .debug_struct("EnumType")
                .field("name", name)
                .field("elements", elements)
                .finish(),
            DataType::SubRangeType {
                name,
                referenced_type,
                bounds,
            } => f
                .debug_struct("SubRangeType")
                .field("name", name)
                .field("referenced_type", referenced_type)
                .field("bounds", bounds)
                .finish(),
            DataType::ArrayType {
                name,
                bounds,
                referenced_type,
            } => f
                .debug_struct("ArrayType")
                .field("name", name)
                .field("bounds", bounds)
                .field("referenced_type", referenced_type)
                .finish(),
            DataType::PointerType {
                name,
                referenced_type,
            } => f
                .debug_struct("PointerType")
                .field("name", name)
                .field("referenced_type", referenced_type)
                .finish(),
            DataType::StringType {
                name,
                is_wide,
                size,
            } => f
                .debug_struct("StringType")
                .field("name", name)
                .field("is_wide", is_wide)
                .field("size", size)
                .finish(),
            DataType::VarArgs { referenced_type } => f
                .debug_struct("VarArgs")
                .field("referenced_type", referenced_type)
                .finish(),
        }
    }
}

impl DataType {
    pub fn set_name(&mut self, new_name: String) {
        match self {
            DataType::StructType { name, variables: _ } => *name = Some(new_name),
            DataType::EnumType { name, elements: _ } => *name = Some(new_name),
            DataType::SubRangeType { name, .. } => *name = Some(new_name),
            DataType::ArrayType { name, .. } => *name = Some(new_name),
            DataType::PointerType { name, .. } => *name = Some(new_name),
            DataType::StringType { name, .. } => *name = Some(new_name),
            DataType::VarArgs { .. } => {} //No names on varargs
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        match self {
            DataType::StructType { name, variables: _ } => name.as_ref().map(|x| x.as_str()),
            DataType::EnumType { name, elements: _ } => name.as_ref().map(|x| x.as_str()),
            DataType::ArrayType { name, .. } => name.as_ref().map(|x| x.as_str()),
            DataType::PointerType { name, .. } => name.as_ref().map(|x| x.as_str()),
            DataType::StringType { name, .. } => name.as_ref().map(|x| x.as_str()),
            DataType::SubRangeType { name, .. } => name.as_ref().map(|x| x.as_str()),
            DataType::VarArgs { .. } => None,
        }
    }

    //Attempts to replace the inner type with a reference. Returns the old type if replaceable
    pub fn replace_data_type_with_reference_to(
        &mut self,
        type_name: String,
        location: &SourceRange,
    ) -> Option<DataTypeDeclaration> {
        match self {
            DataType::ArrayType {
                referenced_type, ..
            } => replace_reference(referenced_type, type_name, location),
            DataType::PointerType {
                referenced_type, ..
            } => replace_reference(referenced_type, type_name, location),
            _ => None,
        }
    }
}

fn replace_reference(
    referenced_type: &mut Box<DataTypeDeclaration>,
    type_name: String,
    location: &SourceRange,
) -> Option<DataTypeDeclaration> {
    if let DataTypeDeclaration::DataTypeReference { .. } = **referenced_type {
        return None;
    }
    let new_data_type = DataTypeDeclaration::DataTypeReference {
        referenced_type: type_name,
        location: location.clone(),
    };
    let old_data_type = std::mem::replace(referenced_type, Box::new(new_data_type));
    Some(*old_data_type)
}

#[derive(Clone, PartialEq)]
pub struct ConditionalBlock {
    pub condition: Box<AstStatement>,
    pub body: Vec<AstStatement>,
}

impl Debug for ConditionalBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ConditionalBlock")
            .field("condition", &self.condition)
            .field("body", &self.body)
            .finish()
    }
}

#[derive(Clone, PartialEq)]
pub enum AstStatement {
    EmptyStatement {
        location: SourceRange,
        id: AstId,
    },
    // Literals
    LiteralInteger {
        value: i128,
        location: SourceRange,
        id: AstId,
    },
    LiteralDate {
        year: i32,
        month: u32,
        day: u32,
        location: SourceRange,
        id: AstId,
    },
    LiteralDateAndTime {
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
        location: SourceRange,
        id: AstId,
    },
    LiteralTimeOfDay {
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
        location: SourceRange,
        id: AstId,
    },
    LiteralTime {
        day: f64,
        hour: f64,
        min: f64,
        sec: f64,
        milli: f64,
        micro: f64,
        nano: u32,
        negative: bool,
        location: SourceRange,
        id: AstId,
    },
    LiteralReal {
        value: String,
        location: SourceRange,
        id: AstId,
    },
    LiteralBool {
        value: bool,
        location: SourceRange,
        id: AstId,
    },
    LiteralString {
        value: String,
        is_wide: bool,
        location: SourceRange,
        id: AstId,
    },
    LiteralArray {
        elements: Option<Box<AstStatement>>, // expression-list
        location: SourceRange,
        id: AstId,
    },
    CastStatement {
        target: Box<AstStatement>,
        type_name: String,
        location: SourceRange,
        id: AstId,
    },
    MultipliedStatement {
        multiplier: u32,
        element: Box<AstStatement>,
        location: SourceRange,
        id: AstId,
    },
    // Expressions
    QualifiedReference {
        elements: Vec<AstStatement>,
        id: AstId,
    },
    Reference {
        name: String,
        location: SourceRange,
        id: AstId,
    },
    ArrayAccess {
        reference: Box<AstStatement>,
        access: Box<AstStatement>,
        id: AstId,
    },
    PointerAccess {
        reference: Box<AstStatement>,
        id: AstId,
    },
    DirectAccess {
        access: DirectAccess,
        index: u32,
        location: SourceRange,
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
        location: SourceRange,
        id: AstId,
    },
    ExpressionList {
        expressions: Vec<AstStatement>,
        id: AstId,
    },
    RangeStatement {
        start: Box<AstStatement>,
        end: Box<AstStatement>,
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
        location: SourceRange,
        id: AstId,
    },
    // Control Statements
    IfStatement {
        blocks: Vec<ConditionalBlock>,
        else_block: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    },
    ForLoopStatement {
        counter: Box<AstStatement>,
        start: Box<AstStatement>,
        end: Box<AstStatement>,
        by_step: Option<Box<AstStatement>>,
        body: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    },
    WhileLoopStatement {
        condition: Box<AstStatement>,
        body: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    },
    RepeatLoopStatement {
        condition: Box<AstStatement>,
        body: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    },
    CaseStatement {
        selector: Box<AstStatement>,
        case_blocks: Vec<ConditionalBlock>,
        else_block: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    },
    CaseCondition {
        condition: Box<AstStatement>,
        id: AstId,
    },
    ExitStatement {
        location: SourceRange,
        id: AstId,
    },
    ContinueStatement {
        location: SourceRange,
        id: AstId,
    },
    ReturnStatement {
        location: SourceRange,
        id: AstId,
    },
    LiteralNull {
        location: SourceRange,
        id: AstId,
    },
}

impl Debug for AstStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AstStatement::EmptyStatement { .. } => f.debug_struct("EmptyStatement").finish(),
            AstStatement::LiteralNull { .. } => f.debug_struct("LiteralNull").finish(),
            AstStatement::LiteralInteger { value, .. } => f
                .debug_struct("LiteralInteger")
                .field("value", value)
                .finish(),
            AstStatement::LiteralDate {
                year, month, day, ..
            } => f
                .debug_struct("LiteralDate")
                .field("year", year)
                .field("month", month)
                .field("day", day)
                .finish(),
            AstStatement::LiteralDateAndTime {
                year,
                month,
                day,
                hour,
                min,
                sec,
                milli,
                ..
            } => f
                .debug_struct("LiteralDateAndTime")
                .field("year", year)
                .field("month", month)
                .field("day", day)
                .field("hour", hour)
                .field("min", min)
                .field("sec", sec)
                .field("milli", milli)
                .finish(),
            AstStatement::LiteralTimeOfDay {
                hour,
                min,
                sec,
                milli,
                ..
            } => f
                .debug_struct("LiteralTimeOfDay")
                .field("hour", hour)
                .field("min", min)
                .field("sec", sec)
                .field("milli", milli)
                .finish(),
            AstStatement::LiteralTime {
                day,
                hour,
                min,
                sec,
                milli,
                micro,
                nano,
                negative,
                ..
            } => f
                .debug_struct("LiteralTime")
                .field("day", day)
                .field("hour", hour)
                .field("min", min)
                .field("sec", sec)
                .field("milli", milli)
                .field("micro", micro)
                .field("nano", nano)
                .field("negative", negative)
                .finish(),
            AstStatement::LiteralReal { value, .. } => {
                f.debug_struct("LiteralReal").field("value", value).finish()
            }
            AstStatement::LiteralBool { value, .. } => {
                f.debug_struct("LiteralBool").field("value", value).finish()
            }
            AstStatement::LiteralString { value, is_wide, .. } => f
                .debug_struct("LiteralString")
                .field("value", value)
                .field("is_wide", is_wide)
                .finish(),
            AstStatement::LiteralArray { elements, .. } => f
                .debug_struct("LiteralArray")
                .field("elements", elements)
                .finish(),
            AstStatement::Reference { name, .. } => {
                f.debug_struct("Reference").field("name", name).finish()
            }
            AstStatement::QualifiedReference { elements, .. } => f
                .debug_struct("QualifiedReference")
                .field("elements", elements)
                .finish(),
            AstStatement::BinaryExpression {
                operator,
                left,
                right,
                ..
            } => f
                .debug_struct("BinaryExpression")
                .field("operator", operator)
                .field("left", left)
                .field("right", right)
                .finish(),
            AstStatement::UnaryExpression {
                operator, value, ..
            } => f
                .debug_struct("UnaryExpression")
                .field("operator", operator)
                .field("value", value)
                .finish(),
            AstStatement::ExpressionList { expressions, .. } => f
                .debug_struct("ExpressionList")
                .field("expressions", expressions)
                .finish(),
            AstStatement::RangeStatement { start, end, .. } => f
                .debug_struct("RangeStatement")
                .field("start", start)
                .field("end", end)
                .finish(),
            AstStatement::Assignment { left, right, .. } => f
                .debug_struct("Assignment")
                .field("left", left)
                .field("right", right)
                .finish(),
            AstStatement::OutputAssignment { left, right, .. } => f
                .debug_struct("OutputAssignment")
                .field("left", left)
                .field("right", right)
                .finish(),
            AstStatement::CallStatement {
                operator,
                parameters,
                ..
            } => f
                .debug_struct("CallStatement")
                .field("operator", operator)
                .field("parameters", parameters)
                .finish(),
            AstStatement::IfStatement {
                blocks, else_block, ..
            } => f
                .debug_struct("IfStatement")
                .field("blocks", blocks)
                .field("else_block", else_block)
                .finish(),
            AstStatement::ForLoopStatement {
                counter,
                start,
                end,
                by_step,
                body,
                ..
            } => f
                .debug_struct("ForLoopStatement")
                .field("counter", counter)
                .field("start", start)
                .field("end", end)
                .field("by_step", by_step)
                .field("body", body)
                .finish(),
            AstStatement::WhileLoopStatement {
                condition, body, ..
            } => f
                .debug_struct("WhileLoopStatement")
                .field("condition", condition)
                .field("body", body)
                .finish(),
            AstStatement::RepeatLoopStatement {
                condition, body, ..
            } => f
                .debug_struct("RepeatLoopStatement")
                .field("condition", condition)
                .field("body", body)
                .finish(),
            AstStatement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => f
                .debug_struct("CaseStatement")
                .field("selector", selector)
                .field("case_blocks", case_blocks)
                .field("else_block", else_block)
                .finish(),
            AstStatement::ArrayAccess {
                reference, access, ..
            } => f
                .debug_struct("ArrayAccess")
                .field("reference", reference)
                .field("access", access)
                .finish(),
            AstStatement::PointerAccess { reference, .. } => f
                .debug_struct("PointerAccess")
                .field("reference", reference)
                .finish(),
            AstStatement::DirectAccess { access, index, .. } => f
                .debug_struct("DirectAccess")
                .field("access", access)
                .field("index", index)
                .finish(),
            AstStatement::MultipliedStatement {
                multiplier,
                element,
                ..
            } => f
                .debug_struct("MultipliedStatement")
                .field("multiplier", multiplier)
                .field("element", element)
                .finish(),
            AstStatement::CaseCondition { condition, .. } => f
                .debug_struct("CaseCondition")
                .field("condition", condition)
                .finish(),
            AstStatement::ReturnStatement { .. } => f.debug_struct("ReturnStatement").finish(),
            AstStatement::ContinueStatement { .. } => f.debug_struct("ContinueStatement").finish(),
            AstStatement::ExitStatement { .. } => f.debug_struct("ExitStatement").finish(),
            AstStatement::CastStatement {
                target, type_name, ..
            } => f
                .debug_struct("CastStatement")
                .field("type_name", type_name)
                .field("target", target)
                .finish(),
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
    pub fn get_location(&self) -> SourceRange {
        match self {
            AstStatement::EmptyStatement { location, .. } => location.clone(),
            AstStatement::LiteralNull { location, .. } => location.clone(),
            AstStatement::LiteralInteger { location, .. } => location.clone(),
            AstStatement::LiteralDate { location, .. } => location.clone(),
            AstStatement::LiteralDateAndTime { location, .. } => location.clone(),
            AstStatement::LiteralTimeOfDay { location, .. } => location.clone(),
            AstStatement::LiteralTime { location, .. } => location.clone(),
            AstStatement::LiteralReal { location, .. } => location.clone(),
            AstStatement::LiteralBool { location, .. } => location.clone(),
            AstStatement::LiteralString { location, .. } => location.clone(),
            AstStatement::LiteralArray { location, .. } => location.clone(),
            AstStatement::Reference { location, .. } => location.clone(),
            AstStatement::QualifiedReference { elements, .. } => {
                let first = elements
                    .first()
                    .map_or_else(SourceRange::undefined, |it| it.get_location());
                let last = elements
                    .last()
                    .map_or_else(SourceRange::undefined, |it| it.get_location());
                SourceRange::new(first.get_start()..last.get_end())
            }
            AstStatement::BinaryExpression { left, right, .. } => {
                let left_loc = left.get_location();
                let right_loc = right.get_location();
                SourceRange::new(left_loc.range.start..right_loc.range.end)
            }
            AstStatement::UnaryExpression { location, .. } => location.clone(),
            AstStatement::ExpressionList { expressions, .. } => {
                let first = expressions
                    .first()
                    .map_or_else(SourceRange::undefined, |it| it.get_location());
                let last = expressions
                    .last()
                    .map_or_else(SourceRange::undefined, |it| it.get_location());
                SourceRange::new(first.get_start()..last.get_end())
            }
            AstStatement::RangeStatement { start, end, .. } => {
                let start_loc = start.get_location();
                let end_loc = end.get_location();
                SourceRange::new(start_loc.range.start..end_loc.range.end)
            }
            AstStatement::Assignment { left, right, .. } => {
                let left_loc = left.get_location();
                let right_loc = right.get_location();
                SourceRange::new(left_loc.range.start..right_loc.range.end)
            }
            AstStatement::OutputAssignment { left, right, .. } => {
                let left_loc = left.get_location();
                let right_loc = right.get_location();
                SourceRange::new(left_loc.range.start..right_loc.range.end)
            }
            AstStatement::CallStatement { location, .. } => location.clone(),
            AstStatement::IfStatement { location, .. } => location.clone(),
            AstStatement::ForLoopStatement { location, .. } => location.clone(),
            AstStatement::WhileLoopStatement { location, .. } => location.clone(),
            AstStatement::RepeatLoopStatement { location, .. } => location.clone(),
            AstStatement::CaseStatement { location, .. } => location.clone(),
            AstStatement::ArrayAccess {
                reference, access, ..
            } => {
                let reference_loc = reference.get_location();
                let access_loc = access.get_location();
                SourceRange::new(reference_loc.range.start..access_loc.range.end)
            }
            AstStatement::PointerAccess { reference, .. } => reference.get_location(),
            AstStatement::DirectAccess { location, .. } => location.clone(),
            AstStatement::MultipliedStatement { location, .. } => location.clone(),
            AstStatement::CaseCondition { condition, .. } => condition.get_location(),
            AstStatement::ReturnStatement { location, .. } => location.clone(),
            AstStatement::ContinueStatement { location, .. } => location.clone(),
            AstStatement::ExitStatement { location, .. } => location.clone(),
            AstStatement::CastStatement { location, .. } => location.clone(),
        }
    }

    pub fn get_id(&self) -> AstId {
        match self {
            AstStatement::EmptyStatement { id, .. } => *id,
            AstStatement::LiteralNull { id, .. } => *id,
            AstStatement::LiteralInteger { id, .. } => *id,
            AstStatement::LiteralDate { id, .. } => *id,
            AstStatement::LiteralDateAndTime { id, .. } => *id,
            AstStatement::LiteralTimeOfDay { id, .. } => *id,
            AstStatement::LiteralTime { id, .. } => *id,
            AstStatement::LiteralReal { id, .. } => *id,
            AstStatement::LiteralBool { id, .. } => *id,
            AstStatement::LiteralString { id, .. } => *id,
            AstStatement::LiteralArray { id, .. } => *id,
            AstStatement::MultipliedStatement { id, .. } => *id,
            AstStatement::QualifiedReference { id, .. } => *id,
            AstStatement::Reference { id, .. } => *id,
            AstStatement::ArrayAccess { id, .. } => *id,
            AstStatement::PointerAccess { id, .. } => *id,
            AstStatement::DirectAccess { id, .. } => *id,
            AstStatement::BinaryExpression { id, .. } => *id,
            AstStatement::UnaryExpression { id, .. } => *id,
            AstStatement::ExpressionList { id, .. } => *id,
            AstStatement::RangeStatement { id, .. } => *id,
            AstStatement::Assignment { id, .. } => *id,
            AstStatement::OutputAssignment { id, .. } => *id,
            AstStatement::CallStatement { id, .. } => *id,
            AstStatement::IfStatement { id, .. } => *id,
            AstStatement::ForLoopStatement { id, .. } => *id,
            AstStatement::WhileLoopStatement { id, .. } => *id,
            AstStatement::RepeatLoopStatement { id, .. } => *id,
            AstStatement::CaseStatement { id, .. } => *id,
            AstStatement::CaseCondition { id, .. } => *id,
            AstStatement::ReturnStatement { id, .. } => *id,
            AstStatement::ContinueStatement { id, .. } => *id,
            AstStatement::ExitStatement { id, .. } => *id,
            AstStatement::CastStatement { id, .. } => *id,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiplication,
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
    Address,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let symbol = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiplication => "*",
            Operator::Division => "/",
            Operator::Equal => "=",
            _ => unimplemented!(),
        };
        f.write_str(symbol)
    }
}

/// flattens expression-lists and MultipliedStatements into a vec of statements.
/// It can also handle nested structures like 2(3(4,5))
pub fn flatten_expression_list(condition: &AstStatement) -> Vec<&AstStatement> {
    match condition {
        AstStatement::ExpressionList { expressions, .. } => expressions
            .iter()
            .by_ref()
            .flat_map(|statement| flatten_expression_list(statement))
            .collect(),
        AstStatement::MultipliedStatement {
            multiplier,
            element,
            ..
        } => iter::repeat(flatten_expression_list(element))
            .take(*multiplier as usize)
            .flatten()
            .collect(),
        _ => vec![condition],
    }
}

pub fn pre_process(unit: &mut CompilationUnit) {
    pre_processor::pre_process(unit)
}

/// extracts the compile-time value of the given statement.
/// returns an error if no value can be derived at compile-time
pub fn extract_value(s: &AstStatement) -> result::Result<String, String> {
    match s {
        AstStatement::UnaryExpression {
            operator, value, ..
        } => extract_value(value).map(|result| format!("{}{}", operator, result)),
        AstStatement::LiteralInteger { value, .. } => Ok(value.to_string()),
        //TODO constants
        _ => Err("Unsupported Statement. Cannot evaluate expression.".to_string()),
    }
}

/// evaluate the given statemetn as i128
pub fn evaluate_constant_int(s: &AstStatement) -> result::Result<i128, String> {
    //TODO give early return for literalInteger (I think the negative-number issue is solved now)
    let value = extract_value(s);
    value.map(|v| v.parse().unwrap_or(0))
}
