//! This crate discribes the project management aspect of the build
//! It handles the creation of a `Project` by either parsing the `build_config` or from parameters
//! This crate is also responsible for `SourceCode`, that is how a source code is read from disk
//! and handled
mod build_config;
mod build_description_schema;
pub mod object;
pub mod project;
