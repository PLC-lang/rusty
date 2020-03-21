#[derive(Debug, PartialEq)]
pub struct POU {
    pub name: String,
    pub variable_blocks: Vec<VariableBlock>,
    pub statements: Vec<Statement>,
    pub pou_type: PouType,
    pub return_type: Option<Type>,
}

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum PouType {
    Program,
    Function,
    FunctionBlock,
}

#[derive(Debug, PartialEq)]
pub struct CompilationUnit {
    pub units: Vec<POU>,
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

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum Type {
    Primitive (PrimitiveType),
    Custom,
}

#[derive(Debug, Copy, PartialEq, Clone)]
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
    LiteralInteger {
        value: String,
    },
    LiteralReal {
        value : String,
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
    ExpressionList {
        expressions: Vec<Statement>,
    },
    RangeStatement {
        start: Box<Statement>,
        end: Box<Statement>,
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
        counter : Box<Statement>,
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
    },
    CaseStatement {
        selector: Box<Statement>,
        case_blocks: Vec<ConditionalBlock>,
        else_block: Vec<Statement>,
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
