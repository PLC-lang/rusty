use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::clap_derive::ArgEnum;
use serde::{Deserialize, Serialize};

pub mod output;

pub const DEFAULT_DWARF_VERSION: usize = 5;
pub const DEFAULT_GOT_LAYOUT_FILE: &str = "online_change_got.json";

#[derive(Default, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Target {
    #[default]
    System,
    Param {
        triple: String,
        sysroot: Option<String>,
    },
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

impl OptimizationLevel {
    pub fn opt_params(&self) -> &str {
        match self {
            OptimizationLevel::None => "default<O0>",
            OptimizationLevel::Less => "default<O1>",
            OptimizationLevel::Default => "default<O2>",
            OptimizationLevel::Aggressive => "default<O3>",
        }
    }

    pub fn is_optimized(&self) -> bool {
        !matches!(self, OptimizationLevel::None)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebugLevel {
    #[default]
    None,
    VariablesOnly(usize),
    Full(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OnlineChange {
    Enabled { file_name: String, format: ConfigFormat },
    Disabled,
}

impl OnlineChange {
    pub fn is_enabled(&self) -> bool {
        matches!(self, OnlineChange::Enabled { .. })
    }
}
