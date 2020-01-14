pub type Return<T> = Option<T>;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub name: String,
    pub variable_blocks: Vec<VariableBlock>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct CompilationUnit {
    pub units: Vec<Program>,
}

#[derive(Debug, PartialEq)]
pub struct VariableBlock {
    pub variables: Vec<Variable>,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub data_type: Type,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Primitive (PrimitiveType),
    Custom,
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    Int,
    Bool,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    LiteralNumber {
        value: String,
    },
    Reference {
        name: String,
    },
    BinaryExpression {
        operator: Operator,
    left: Box<Statement>,
    right: Box<Statement>,
    },
    UnaryExpression {
        operator: Operator,
        value: Box<Statement>,
    },
    Assignment {
        left: Box<Statement>,
        right: Box<Statement>,
    }
}

#[derive(Debug, PartialEq)]
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
}
