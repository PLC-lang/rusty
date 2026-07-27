use plc_ast::ast::AstNode;
use plc_source::source_location::SourceLocation;

pub struct Network {
    pub statements: Vec<Statement>,
    pub temporaries: Vec<Temporary>,
}

pub struct Temporary {
    pub name: String,
    pub data_type: String,
    pub location: SourceLocation,
}

pub enum Argument {
    Input { parameter: String, value: Box<AstNode> },
    Output { parameter: String, capture: Option<String> },
}

pub enum Statement {
    Assignment { sink: AstNode, source: AstNode },
    Return { condition: AstNode, location: SourceLocation },
    Jump { condition: Option<AstNode>, target: String, location: SourceLocation },
    Label { name: String, location: SourceLocation },
    Call { target: String, arguments: Vec<Argument>, capture: Option<String>, location: SourceLocation },
}
