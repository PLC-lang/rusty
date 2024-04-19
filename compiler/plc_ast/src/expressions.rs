use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::ast::AstNode;

#[derive(Clone, PartialEq)]
pub enum AstExpression {
    Reference(ReferenceExpr),
    Identifier(String),
    DirectAccess(DirectAccess),
    HardwareAccess(HardwareAccess),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    List(Vec<AstNode>),
    Parenthesized(Box<AstNode>),
    Range(RangeStatement),
    VlaRange,
}

macro_rules! reference_expr {
    ($kind:expr, $access:expr, $base:expr) => {
        AstExpression::Reference(ReferenceExpr{ access: $kind(Box::new($access)), base: $base.map(Box::new) })
    };
    ($kind:expr, $base:expr) => {
        AstExpression::Reference(ReferenceExpr{ access: $kind, base: Some(Box::new($base)) })
    };
}

macro_rules! binary_expression {
    ($left:expr, $right:expr, $operator:expr) => {
        AstExpression::Binary(BinaryExpression{
            operator: $operator,
            left: Box::new($left),
            right: Box::new($right),
        })
    };
}

macro_rules! unary_expression {
    ($value:expr, $operator:expr) => {
        AstExpression::Unary(UnaryExpression{
            operator: $operator,
            value: Box::new($value),
        })
    };
}

impl AstExpression {
    pub(crate) fn get_as_list(&self) -> Option<Vec<&AstNode>> {
        let AstExpression::List(list) = self else { return None };
        Some(list.iter().collect())
    }

    pub(crate) fn hardware_access(
        access: DirectAccessType,
        direction: HardwareAccessType,
        address: Vec<AstNode>,
    ) -> AstExpression {
        Self::HardwareAccess(HardwareAccess::new(access, direction, address))
    }

    pub(crate) fn list(expressions: Vec<AstNode>) -> AstExpression {
        Self::List(expressions)
    }

    pub(crate) fn parenthesized(expression: AstNode) -> AstExpression {
        Self::Parenthesized(Box::new(expression))
    }

    pub(crate) fn binary(left: AstNode, right: AstNode, operator: Operator) -> AstExpression {
        binary_expression!(left, right, operator)
    }

    pub(crate) fn or(left: AstNode, right: AstNode) -> AstExpression {
        binary_expression!(left, right, Operator::Or)
    }
    
    pub(crate) fn unary(value: AstNode, operator: Operator) -> AstExpression {
        unary_expression!(value, operator)
    }

    pub(crate) fn not(value: AstNode) -> AstExpression {
        unary_expression!(value, Operator::Not)
    }

    pub(crate) fn ident(name: impl Into<String>) -> AstExpression {
        Self::Identifier(name.into())
    }
    
    pub(crate) fn member_reference(member: AstNode, base: Option<AstNode>) -> AstExpression {
        reference_expr!(ReferenceAccess::Member, member, base)
    }
    
    pub(crate) fn index_reference(index: AstNode, base: Option<AstNode>) -> AstExpression {
        reference_expr!(ReferenceAccess::Index, index, base)
    }
    
    pub(crate) fn address_reference(base: AstNode) -> AstExpression {
        reference_expr!(ReferenceAccess::Address, base)
    }
    
    pub(crate) fn deref_reference(base: AstNode) -> AstExpression {
        reference_expr!(ReferenceAccess::Deref, base)
    }

    pub(crate) fn cast_reference(access: AstNode, base: AstNode) -> AstExpression {
        reference_expr!(ReferenceAccess::Cast, access, Some(base))
    }
    
    pub(crate) fn direct_access(access: DirectAccessType, index: AstNode) -> AstExpression {
        Self::DirectAccess(DirectAccess { access, index: Box::new(index) })
    }

    pub(crate) fn range(start: AstNode, end: AstNode) -> AstExpression {
        Self::Range(RangeStatement { start: Box::new(start), end: Box::new(end) })
    }
}

impl Debug for AstExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AstExpression::Reference(ReferenceExpr { access, base }) => {
                f.debug_struct("ReferenceExpr").field("kind", access).field("base", base).finish()
            }
            AstExpression::Identifier(name) => f.debug_struct("Identifier").field("name", name).finish(),
            AstExpression::DirectAccess(DirectAccess { access, index }) => {
                f.debug_struct("DirectAccess").field("access", access).field("index", index).finish()
            }
            AstExpression::HardwareAccess(HardwareAccess { direction, access, address }) => f
                .debug_struct("HardwareAccess")
                .field("direction", direction)
                .field("access", access)
                .field("address", address)
                // .field("location", &self.location) // why does only HW access have a location?
                .finish(),
            AstExpression::Binary(BinaryExpression { operator, left, right }) => f
                .debug_struct("BinaryExpression")
                .field("operator", operator)
                .field("left", left)
                .field("right", right)
                .finish(),
            AstExpression::Unary(UnaryExpression { operator, value }) => {
                f.debug_struct("UnaryExpression").field("operator", operator).field("value", value).finish()
            }
            AstExpression::List(expressions) => {
                f.debug_struct("ExpressionList").field("expressions", expressions).finish()
            }
            AstExpression::Parenthesized(expression) => {
                f.debug_struct("ParenExpression").field("expression", expression).finish()
            }
            AstExpression::Range(RangeStatement { start, end }) => {
                f.debug_struct("RangeStatement").field("start", start).field("end", end).finish()
            }
            AstExpression::VlaRange => f.debug_struct("VlaRangeStatement").finish(),
        }
    }
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

impl HardwareAccess {
    fn new(access: DirectAccessType, direction: HardwareAccessType, address: Vec<AstNode>) -> HardwareAccess {
        HardwareAccess { access, direction, address }
    }
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "direction")]
pub enum HardwareAccessType {
    Input,
    Output,
    Memory,
    Global,
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
