//! This crates handles diagnostics, from creation to reporting.

pub mod diagnostics;
pub mod reporter {
    pub mod clang;
    pub mod codespan;
    pub mod null;
}
pub mod errno; // TODO: Make this crate-private?
