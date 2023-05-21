#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,

    MissingAttributes,

    ReadEvent,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedEOF => write!(f, "{self:#?}"),
            Error::MissingAttributes => write!(f, "{self:#?}"),
            Error::ReadEvent => write!(f, "{self:#?}"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum VariableKind {
    Input,
    Output,
    InOut,
}

pub struct Block {
    local_id: i32,
    type_name: String,
    instance_name: String,
    execution_order: i32,
}
