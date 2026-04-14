use std::path::Path;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::diagnostics::diagnostics_registry::DiagnosticsConfiguration;

/// A compatibility profile that controls compiler behavior across all phases.
///
/// Profiles define behavior flags that may affect lowering, validation, codegen,
/// and the type system. They also include diagnostics severity overrides.
///
/// Unknown fields in the `behaviors` section are silently skipped for forward
/// compatibility — newer profile files can be used with older compilers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityProfile {
    /// Optional human-readable name for this profile.
    #[serde(default)]
    pub name: Option<String>,

    /// Flat set of behavior flags. Each flag may affect multiple compiler phases.
    #[serde(default)]
    pub behaviors: BehaviorFlags,

    /// Diagnostics severity overrides.
    #[serde(default)]
    pub diagnostics: DiagnosticsConfiguration,
}

/// Behavior flags that control compiler semantics across phases.
///
/// All fields are optional with defaults that match current (CODESYS-compatible) behavior.
/// Unknown fields are silently ignored during deserialization for forward compatibility.
// NOTE: We intentionally do NOT use `#[serde(deny_unknown_fields)]` here.
// Unknown fields are skipped by serde's default behavior, which provides
// forward compatibility — newer profile files work with older compilers.
// When adding new flags, add them with `#[serde(default = "...")]` to
// ensure profiles that don't mention the flag get the current default.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BehaviorFlags {
    // No flags yet — this struct will grow as behaviors are implemented.
    // Each flag should:
    //   1. Have a `#[serde(default = "...")]` attribute
    //   2. Default to current (CODESYS-compatible) behavior
    //   3. Be documented in book/src/using_rusty/compatibility_profiles.md
}

/// Well-known profile names.
pub const PROFILE_CODESYS: &str = "codesys";
pub const PROFILE_STANDARD: &str = "standard";

impl CompatibilityProfile {
    /// Returns the default CODESYS-compatible profile.
    /// This matches the compiler's current behavior.
    pub fn codesys() -> Self {
        CompatibilityProfile {
            name: Some(PROFILE_CODESYS.to_string()),
            behaviors: BehaviorFlags::default(),
            diagnostics: DiagnosticsConfiguration::default(),
        }
    }

    /// Returns the IEC 61131-3 strict-standard profile.
    pub fn standard() -> Self {
        CompatibilityProfile {
            name: Some(PROFILE_STANDARD.to_string()),
            behaviors: BehaviorFlags::default(),
            diagnostics: DiagnosticsConfiguration::default(),
        }
    }

    /// Resolves a profile from a name or file path.
    ///
    /// If `value` matches a built-in profile name (`codesys`, `standard`),
    /// the built-in profile is returned. Otherwise, `value` is treated as
    /// a file path and the profile is loaded from disk.
    pub fn from_name_or_path(value: &str) -> Result<Self> {
        match value {
            PROFILE_CODESYS => Ok(Self::codesys()),
            PROFILE_STANDARD => Ok(Self::standard()),
            path => Self::from_file(Path::new(path)),
        }
    }

    /// Loads a profile from a JSON or TOML file.
    ///
    /// The format is detected from the file extension:
    /// - `.json` → JSON
    /// - `.toml` → TOML
    /// - anything else → attempts JSON first, then TOML
    pub fn from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            bail!("{} does not exist", path.display());
        }

        let content = std::fs::read_to_string(path)?;
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match extension {
            "json" => Self::from_json(&content),
            "toml" => Self::from_toml(&content),
            _ => {
                // Try JSON first, fall back to TOML
                Self::from_json(&content).or_else(|_| Self::from_toml(&content))
            }
        }
    }

    /// Deserializes a profile from a JSON string.
    pub fn from_json(content: &str) -> Result<Self> {
        let profile: CompatibilityProfile = serde_json::from_str(content)?;
        if let Some(name) = &profile.name {
            log::debug!("Loaded profile '{name}' from JSON");
        }
        Ok(profile)
    }

    /// Deserializes a profile from a TOML string.
    pub fn from_toml(content: &str) -> Result<Self> {
        let profile: CompatibilityProfile = toml::from_str(content)?;
        if let Some(name) = &profile.name {
            log::debug!("Loaded profile '{name}' from TOML");
        }
        Ok(profile)
    }

    /// Serializes the profile to a JSON string.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Serializes the profile to a TOML string.
    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string(self)?)
    }

    /// Creates a profile from a legacy `DiagnosticsConfiguration` (for `--error-config` compat).
    ///
    /// All non-diagnostic settings default to codesys behavior.
    pub fn from_diagnostics_configuration(config: DiagnosticsConfiguration) -> Self {
        log::trace!("Converting --error-config diagnostics configuration to a compatibility profile");
        let mut profile = Self::codesys();
        profile.name = None;
        profile.diagnostics = config;
        profile
    }
}

impl Default for CompatibilityProfile {
    fn default() -> Self {
        Self::codesys()
    }
}
