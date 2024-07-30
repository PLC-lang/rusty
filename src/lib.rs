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
use std::convert::Infallible;
use std::str::FromStr;

use clap::clap_derive::ArgEnum;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use inkwell::targets::{self, TargetMachine, TargetTriple};

#[cfg(test)]
use resolver::TypeAnnotator;
#[cfg(test)]
use validation::Validator;

pub mod builtins;
pub mod codegen;
mod datalayout;
pub mod expression_path;
pub mod hardware_binding;
pub mod index;
pub mod lexer;
pub mod linker;
pub mod output;
pub mod parser;
pub mod resolver;
mod test_utils;

pub mod typesystem;
pub mod validation;
extern crate shell_words;

pub const DEFAULT_DWARF_VERSION: usize = 5;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Target {
    System,
    Param { triple: String, sysroot: Option<String> },
}

impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let target = String::deserialize(deserializer)?;
        Ok(target.into())
    }
}

impl Target {
    pub fn new(triple: String, sysroot: Option<String>) -> Target {
        Target::Param { triple, sysroot }
    }

    pub fn with_sysroot(self, sysroot: Option<String>) -> Target {
        match self {
            Self::Param { triple, .. } => Target::Param { triple, sysroot },
            _ => self,
        }
    }

    pub fn get_target_triple(&self) -> TargetTriple {
        let res = match self {
            Target::System => TargetMachine::get_default_triple(),
            Target::Param { triple, .. } => TargetTriple::create(triple),
        };
        targets::TargetMachine::normalize_triple(&res)
    }

    pub fn try_get_name(&self) -> Option<&str> {
        match self {
            Target::System => None,
            Target::Param { triple, .. } => Some(triple.as_str()),
        }
    }

    pub fn get_sysroot(&self) -> Option<&str> {
        match self {
            Target::Param { sysroot, .. } => sysroot.as_deref(),
            _ => None,
        }
    }

    pub fn append_to(&self, location: &Path) -> PathBuf {
        match self {
            Target::System => location.to_path_buf(),
            Target::Param { triple, .. } => location.join(triple),
        }
    }
}

impl<T> From<T> for Target
where
    T: core::ops::Deref<Target = str>,
{
    fn from(it: T) -> Self {
        Target::new(it.to_string(), None)
    }
}

impl FromStr for Target {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Target::from(s))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, ArgEnum)]
pub enum ConfigFormat {
    JSON,
    TOML,
}

impl FromStr for ConfigFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(ConfigFormat::JSON),
            "toml" => Ok(ConfigFormat::TOML),
            _ => Err(format!("Invalid option {s}")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ArgEnum, Serialize, Deserialize, Default)]
pub enum ErrorFormat {
    #[default]
    Rich,
    Clang,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Threads {
    Full,
    Fix(usize),
    #[default]
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Serialize, Deserialize, Default)]
pub enum OptimizationLevel {
    None,
    Less,
    #[default]
    Default,
    Aggressive,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebugLevel {
    #[default]
    None,
    VariablesOnly(usize),
    Full(usize),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OnlineChange {
    Enabled,
    Disabled,
}

impl From<OptimizationLevel> for inkwell::OptimizationLevel {
    fn from(val: OptimizationLevel) -> Self {
        match val {
            OptimizationLevel::None => inkwell::OptimizationLevel::None,
            OptimizationLevel::Less => inkwell::OptimizationLevel::Less,
            OptimizationLevel::Default => inkwell::OptimizationLevel::Default,
            OptimizationLevel::Aggressive => inkwell::OptimizationLevel::Aggressive,
        }
    }
}

impl OptimizationLevel {
    fn opt_params(&self) -> &str {
        match self {
            OptimizationLevel::None => "default<O0>",
            OptimizationLevel::Less => "default<O1>",
            OptimizationLevel::Default => "default<O2>",
            OptimizationLevel::Aggressive => "default<O3>",
        }
    }

    fn is_optimized(&self) -> bool {
        !matches!(self, OptimizationLevel::None)
    }
}

#[macro_use]
#[cfg(test)]
extern crate pretty_assertions;
#[cfg(test)]
mod tests {
    mod adr;
}
