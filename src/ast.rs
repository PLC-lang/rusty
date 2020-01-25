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
pub struct ConditionalBlock {
    pub condition: Box<Statement>,
    pub body: Vec<Statement>
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    // Literals
    LiteralNumber {
        value: String,
    },
    LiteralBool {
        value: bool,
    },
    // Expressions
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
    // Assignment
    Assignment {
        left: Box<Statement>,
        right: Box<Statement>,
    },
    // Control Statements
    IfStatement {
        blocks : Vec<ConditionalBlock>,
        else_block: Vec<Statement>,
    },
    ForLoopStatement {
        start: Box<Statement>,
        end: Box<Statement>,
        by_step: Option<Box<Statement>>,
        body: Vec<Statement>,
    },
    WhileLoopStatement {
        condition: Box<Statement>,
        body: Vec<Statement>,
    },
    RepeatLoopStatement {
        condition: Box<Statement>,
        body: Vec<Statement>,
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
