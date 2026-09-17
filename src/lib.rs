// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
//!A St&ructured Text LLVM Frontent
//!
//! RuSTy is an [`ST`] Compiler using LLVM
//!
//! # Features
//! ## Standard language support
//! Most of the [`IEC61131-3`] standard for ST and general programing is supported.
//! ## Native compilation
//! A (currently) single ST files into object code using LLVM.
//! A compiled object can be linked statically or dynamically linked
//!     with other programs using standard compiler linkers (ld, clang, gcc)
//! ## IR Output
//! An [`IR`] file can be generated from any given ST file in order to examin the generated LLVM IR code.
//! For a usage guide refer to the [User Documentation](../../)
//!
//! [`ST`]: https://en.wikipedia.org/wiki/Structured_text
//! [`IEC61131-3`]: https://en.wikipedia.org/wiki/IEC_61131-3
//! [`IR`]: https://llvm.org/docs/LangRef.html
use inkwell::targets::{self, TargetMachine, TargetTriple};

#[cfg(test)]
use validation::Validator;

pub mod builtins;
pub mod codegen;
mod datalayout;
pub mod expression_path;
pub mod hardware_binding;
pub mod hw_map;
pub mod index;
pub use plc_lexer as lexer;
pub use plc_lexer::expect_token;
pub mod linker;
pub mod lowering;
pub use plc_options::{
    output, ConfigFormat, DebugLevel, ErrorFormat, OnlineChange, OptimizationLevel, Target, Threads,
    DEFAULT_DWARF_VERSION, DEFAULT_GOT_LAYOUT_FILE,
};
pub mod parser;
pub mod resolver;
mod test_utils;

pub mod typesystem;
pub mod validation;
extern crate shell_words;

pub trait TargetExt {
    fn get_target_triple(&self) -> TargetTriple;
}

impl TargetExt for Target {
    fn get_target_triple(&self) -> TargetTriple {
        let res = match self {
            Target::System => TargetMachine::get_default_triple(),
            Target::Param { triple, .. } => TargetTriple::create(triple),
        };
        targets::TargetMachine::normalize_triple(&res)
    }
}

pub trait OptimizationLevelExt {
    fn codegen_level(&self) -> inkwell::OptimizationLevel;
}

impl OptimizationLevelExt for OptimizationLevel {
    /// The backend (`TargetMachine`) level backing this optimization level. `None` keeps
    /// the IR pipeline at O0 (see `opt_params`: no inlining, unrolling or other
    /// debug-hostile IR transforms) but raises the codegen level to `Less` so the
    /// machine pipeline runs; most importantly LLVM's stack coloring, which merges the
    /// lifetime-bracketed call temporaries and keeps string-heavy POU frames bounded.
    fn codegen_level(&self) -> inkwell::OptimizationLevel {
        match self {
            OptimizationLevel::None | OptimizationLevel::Less => inkwell::OptimizationLevel::Less,
            OptimizationLevel::Default => inkwell::OptimizationLevel::Default,
            OptimizationLevel::Aggressive => inkwell::OptimizationLevel::Aggressive,
        }
    }
}

#[macro_use]
#[cfg(test)]
extern crate pretty_assertions;
#[cfg(test)]
mod tests {
    mod adr;
}
