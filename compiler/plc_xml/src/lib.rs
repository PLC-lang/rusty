// TODO: Remove
#![allow(dead_code)]

mod deserializer;
pub mod error;
pub mod xml_parser;
pub(crate) mod model {
    pub(crate) mod action;
    pub(crate) mod block;
    pub(crate) mod body;
    pub(crate) mod connector;
    pub(crate) mod control;
    pub(crate) mod fbd;
    pub(crate) mod interface;
    pub(crate) mod pou;
    pub(crate) mod variables;
}
mod reader;
mod serializer;

mod tests {
    mod variables;
    mod xml;
}
