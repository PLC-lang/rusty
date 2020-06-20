#[derive(Debug, PartialEq)]
pub struct POU {
    pub name: String,
    pub variable_blocks: Vec<VariableBlock>,
    pub statements: Vec<Statement>,
    pub pou_type: PouType,
    pub return_type: Option<DataTypeDeclaration>,
}

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum PouType {
    Program,
    Function,
    FunctionBlock,
}

#[derive(Debug, PartialEq)]
pub struct CompilationUnit {
    pub global_vars: Vec<VariableBlock>,
    pub units: Vec<POU>,
    pub types: Vec<DataType>,
}


#[derive(Debug, Copy, PartialEq, Clone)]
pub enum VariableBlockType {
    Local,
    Input,
    Global,
}

#[derive(Debug, PartialEq)]
pub struct VariableBlock {
    pub variables: Vec<Variable>,
    pub variable_block_type: VariableBlockType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub data_type: DataTypeDeclaration,
}

impl Variable {
    pub fn replace_data_type_with_reference_to(&mut self, type_name : String) -> DataTypeDeclaration{
        let new_data_type = DataTypeDeclaration::DataTypeReference {referenced_type : type_name};
        let old_data_type = std::mem::replace(&mut self.data_type, new_data_type);
        old_data_type
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataTypeDeclaration {
    DataTypeReference {
        referenced_type: String,
    },
    DataTypeDefinition {
        data_type: DataType,    
    }
}

impl DataTypeDeclaration {
    pub fn get_name<'ctx> (&'ctx self) -> Option<&'ctx str> {
        match self {
            DataTypeDeclaration::DataTypeReference { referenced_type } => Some(referenced_type.as_str()),
            DataTypeDeclaration::DataTypeDefinition { data_type } => data_type.get_name(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
        referenced_type : String,
    }
}

impl DataType {
    pub fn set_name(&mut self, new_name : String) {
        match self {
            DataType::StructType {name , variables: _} => *name = Some(new_name),
            DataType::EnumType {name, elements: _} => *name = Some(new_name),
            DataType::SubRangeType {name, referenced_type: _} => *name = Some(new_name),
        }
    }

    pub fn get_name<'ctx>(&'ctx self) -> Option<&'ctx str> {
        match self {
            DataType::StructType {name, variables: _} => name.as_ref().map(|x| x.as_str()),
            DataType::EnumType {name, elements: _} => name.as_ref().map(|x| x.as_str()),
            DataType::SubRangeType {name, referenced_type: _} => name.as_ref().map(|x| x.as_str()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConditionalBlock {
    pub condition: Box<Statement>,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    // Literals
    LiteralInteger {
        value: String,
    },
    LiteralReal {
        value: String,
    },
    LiteralBool {
        value: bool,
    },
    LiteralString {
        value: String,
    },
    // Expressions
    Reference {
        elements : Vec<String>,
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
    //Call Statement
    CallStatement {
        operator: Box<Statement>,
        parameters: Box<Option<Statement>>,
    },
    // Control Statements
    IfStatement {
        blocks: Vec<ConditionalBlock>,
        else_block: Vec<Statement>,
    },
    ForLoopStatement {
        counter: Box<Statement>,
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
    },
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
