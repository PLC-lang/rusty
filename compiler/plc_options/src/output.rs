use serde::{Deserialize, Serialize};

/// The desired output format / link mode for a compilation.
///
/// This determines the **kind** of artifact produced (object, shared lib, executable, etc.).
/// Relocation behavior (PIC vs non-PIC) is controlled separately via [`RelocationPreference`].
#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum FormatOption {
    /// Compile-only: emit a single object file, no linking.
    Object,
    /// Link into a static executable.
    #[default]
    Static,
    /// **Deprecated** — equivalent to `Shared` with `RelocationPreference::Pic`.
    /// Retained for backward compatibility with `--pic`.
    PIC,
    /// Link into a shared library (`.so` / `.dylib`).
    Shared,
    /// **Deprecated** — equivalent to `Shared` with `RelocationPreference::NoPic`.
    /// Retained for backward compatibility with `--no-pic`.
    NoPIC,
    /// Partial (relocatable) link: combine multiple objects into one (linker `-r`).
    Relocatable,
    /// Emit LLVM bitcode (`.bc`).
    Bitcode,
    /// Emit LLVM IR text (`.ll`).
    IR,
}

impl FormatOption {
    /// Returns `true` if this format requires invoking the linker.
    pub fn should_link(self) -> bool {
        matches!(
            self,
            FormatOption::Static
                | FormatOption::Shared
                | FormatOption::PIC
                | FormatOption::NoPIC
                | FormatOption::Relocatable
        )
    }
}

/// Controls the relocation model used during code generation.
///
/// This is orthogonal to [`FormatOption`] — you can request PIC code for any output kind.
///
/// | Preference | Object files | Shared libraries | Executables |
/// |---|---|---|---|
/// | `Default` | platform default | PIC | platform default (often PIE) |
/// | `Pic` | PIC relocations | PIC | PIC (PIE-compatible) |
/// | `NoPic` | non-PIC relocations | non-PIC (may fail on some targets) | non-PIE (`-no-pie`) |
///
/// Corresponds to CLI flags `--fpic` and `--fno-pic`.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum RelocationPreference {
    /// Use the platform / format default relocation model.
    #[default]
    Default,
    /// Generate position-independent code (`--fpic`).
    Pic,
    /// Generate non-position-independent code (`--fno-pic`).
    NoPic,
}
