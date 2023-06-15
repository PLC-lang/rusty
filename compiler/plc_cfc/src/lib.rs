// TODO: Remove
#![allow(dead_code)]

pub mod cfc_parser;
mod deserializer;
pub mod error;
pub(crate) mod model {
    pub(crate) mod block;
    pub(crate) mod body;
    pub(crate) mod connector;
    pub(crate) mod control;
    pub(crate) mod fbd;
    pub(crate) mod pou;
    pub(crate) mod variables;
}
mod reader;
mod serializer;

mod tests {
    mod variables;
    mod xml;
}
