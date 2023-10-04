// TODO: Remove
#![allow(dead_code)]
// #![feature(trait_upcasting)]

pub mod error;
mod extensions;
pub mod xml_parser;
pub(crate) mod model {
    pub mod action;
    pub mod block;
    pub mod body;
    pub mod connector;
    pub mod control;
    pub mod fbd;
    pub mod interface;
    pub mod pou;
    pub mod project;
    pub mod variables;
}
mod reader;
mod serializer;
