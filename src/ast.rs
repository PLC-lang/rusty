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
    pub data_type: String,
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
    OperatorEqual,
    OperatorNotEqual,
    Modulo,
}
