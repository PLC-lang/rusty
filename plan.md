# Compatibility Profile System — Implementation Plan

## Overview

Add a `--profile` global CLI flag that lets users select a named compatibility profile (`standard`, `codesys`, ...) or provide a custom profile file (JSON/TOML). The profile controls compiler behavior across all phases: diagnostics, validation, lowering, codegen, and the type system.

The existing `--error-config` flag is deprecated and subsumed by the profile's `diagnostics` section.

## Design Principles

- **Behavior flags are flat, not per-phase.** A single flag like `short_circuit_bool_ops` may affect lowering, validation, and codegen simultaneously.
- **Forward-compatible deserialization.** Unknown fields in a profile file are silently skipped with a `log::debug!` message. This lets newer profile files work with older compilers.
- **Built-in profiles are starting points.** Users can export a built-in profile to JSON/TOML, customize it, and pass the file back via `--profile`.
- **Infrastructure first, behaviors later.** The profile system is built and wired through all phases before any specific behavior flags are implemented.

## File Format

```json
{
  "name": "codesys",
  "behaviors": {
    "short_circuit_bool_ops": false
  },
  "diagnostics": {
    "ignore": ["E015"],
    "warning": [],
    "error": []
  }
}
```

- `name`: optional human-readable label
- `behaviors`: flat map of behavior flags (all optional, all have defaults matching current CODESYS-compatible behavior)
- `diagnostics`: same format as today's `--error-config` JSON — maps severity levels to lists of error codes

---

## Phase 1: Profile Data Model & Loading

**Goal**: Define the `CompatibilityProfile` type, built-in profiles, and file loading logic. No wiring into the pipeline yet.

### Tasks

- [x] Create `compiler/plc_driver/src/profiles.rs`
  - Define `CompatibilityProfile` struct with `name: Option<String>`, `behaviors: BehaviorFlags`, `diagnostics: DiagnosticsConfiguration`
  - Define `BehaviorFlags` struct — initially empty (no flags yet), with `#[serde(flatten)]` or manual handling to capture and log unknown fields
  - Use `#[serde(default)]` on all sections so partial files work
  - Implement `CompatibilityProfile::standard()` and `CompatibilityProfile::codesys()` as built-in constructors
  - Implement `CompatibilityProfile::from_file(path: &Path) -> Result<Self>` for loading from JSON/TOML
  - Implement `CompatibilityProfile::from_name_or_path(value: &str) -> Result<Self>` — resolves named profiles or falls back to file path
  - Implement serialization (for the `config profile` export command)
- [x] Add `mod profiles;` to `compiler/plc_driver/src/lib.rs`
- [x] Add unit tests for profile loading:
  - Load a named profile (`standard`, `codesys`)
  - Load a profile from a JSON file
  - Load a partial profile (missing sections default correctly)
  - Load a profile with unknown behavior flags (skipped gracefully, debug logged)
  - Invalid file path errors cleanly

### Key Files

| File | Action |
|------|--------|
| `compiler/plc_driver/src/profiles.rs` | **New** |
| `compiler/plc_driver/src/lib.rs` | Add `mod profiles` |

### Context for Implementer

- `DiagnosticsConfiguration` is defined at `compiler/plc_diagnostics/src/diagnostics/diagnostics_registry.rs:87` — it's a `FxHashMap<Severity, Vec<String>>` with serde support. Reuse it as-is for the `diagnostics` field.
- The profile crate dependency chain: `plc_driver` already depends on `plc_diagnostics`, so importing `DiagnosticsConfiguration` is straightforward.
- For TOML support, check if `toml` is already a dependency; the `config` subcommand already has a `ConfigFormat` enum with `Json` and `Toml` variants (see `cli.rs:411`).

---

## Phase 2: CLI Flag & `--error-config` Compatibility

**Goal**: Add `--profile` as a global flag. Keep `--error-config` working as-is (no deprecation yet). Both flags resolve to a `CompatibilityProfile`.

### Tasks

- [x] Add `--profile` flag to `CompileParameters` in `compiler/plc_driver/src/cli.rs`
  - Global flag (`global = true`) so it works with `build`, `check`, `config`, `generate`, etc.
  - Type: `Option<String>` — accepts a profile name or file path
  - Help text: `"Set a compatibility profile (name or path to profile file)"`
- [x] Add `get_compatibility_profile(&self) -> Result<CompatibilityProfile>` method on `CompileParameters`
  - If `--profile` is set: resolve via `CompatibilityProfile::from_name_or_path()`
  - Else if `--error-config` is set: load old format, wrap in a codesys-default `CompatibilityProfile` with the provided diagnostics overrides, and `log::trace!` that the error config is being converted to a profile
  - Else: return `CompatibilityProfile::codesys()` (default — matches current compiler behavior)
  - If both `--profile` and `--error-config` are provided, `--profile` takes precedence
- [x] Extend `ConfigOption` enum with a `Profile` variant
  - `plc config profile` — prints the resolved profile as JSON/TOML
  - Allows `plc config profile --profile codesys` to export a named profile
- [x] Add CLI tests:
  - `--profile standard` parses correctly
  - `--profile path/to/file.json` parses correctly
  - `--error-config` still works, converted to a codesys profile with trace log
  - `plc config profile` outputs valid JSON
  - `--profile` works with `build` and `check` subcommands

### Key Files

| File | Action |
|------|--------|
| `compiler/plc_driver/src/cli.rs` | Add `--profile` flag (~line 343, near `error_config`), add `ConfigOption::Profile`, add `get_compatibility_profile()` |

### Context for Implementer

- Look at how `--error-config` is defined (line 343) and `get_error_configuration()` (line 689) for the pattern to follow.
- The `ConfigOption` enum is at line 452. Add `Profile` variant.
- The `get_config_options()` method at line 649 extracts config subcommand options.
- `print_config_options()` in `pipelines.rs:303` handles the output — extend it for `ConfigOption::Profile`.

---

## Phase 3: Wire Profile Into the Pipeline

**Goal**: The resolved profile reaches all compiler phases — diagnostics, validation, lowering, and codegen.

### Tasks

- [x] Add `compatibility_profile` field to `GlobalContext` (`compiler/plc_index/src/lib.rs:21`)
  - Type: `Arc<CompatibilityProfile>` (cheap to clone, shared across phases)
  - Add `set_compatibility_profile()` and `compatibility_profile()` accessor
  - Default via `#[serde(skip)]` (defaults to `CompatibilityProfile::default()` = codesys)
- [x] Set the profile on `GlobalContext` during pipeline initialization
  - In `BuildPipeline::try_from(CompileParameters)` (`pipelines.rs:83`), resolve the profile and set it on `self.context`
- [x] Feed `profile.diagnostics` to the diagnostician
  - Replaced `get_error_configuration()` call with profile-based extraction in pipeline init
- [x] Add profile to `CompileOptions` (`compiler/plc_driver/src/lib.rs:45`)
  - Type: `Arc<CompatibilityProfile>`
  - Populated in `get_compile_options()` from `self.context.compatibility_profile()`
- [x] Thread profile to codegen
  - Passed through `CompileOptions` → `generate_module()` → `generate_llvm_index()` + `generate()`
  - Threaded through `PouGenerator` → `StatementCodeGenerator` → `ExpressionCodeGenerator`
  - Also threaded through `DataTypeGenerator`, `VariableGenerator`, and `generate_implementation_stubs`
- [x] Verify lowering participants can access the profile
  - Participants are constructed in `get_default_mut_participants()` using `self.context` (the `GlobalContext`)
  - `self.context.compatibility_profile()` returns `&Arc<CompatibilityProfile>`
  - Verified: access path works; actual usage comes in later phases
- [x] Verify validators can access the profile
  - `Validator::new(context: &GlobalContext)` already receives `GlobalContext`
  - `self.context.compatibility_profile()` is accessible
  - Verified: access path works

### Key Files

| File | Action |
|------|--------|
| `compiler/plc_diagnostics/src/profiles.rs` | **New** — core profile types (moved from `plc_driver`) |
| `compiler/plc_diagnostics/src/lib.rs` | Add `pub mod profiles` |
| `compiler/plc_driver/src/profiles.rs` | Re-exports from `plc_diagnostics::profiles` + tests |
| `compiler/plc_index/src/lib.rs` | Add `compatibility_profile` to `GlobalContext` |
| `compiler/plc_driver/src/pipelines.rs` | Resolve profile at init, feed to diagnostician + CompileOptions |
| `compiler/plc_driver/src/lib.rs` | Add profile to `CompileOptions` |
| `src/codegen.rs` | Thread profile through `generate_llvm_index` + `generate` |
| `src/codegen/generators/pou_generator.rs` | Add profile to struct + constructors |
| `src/codegen/generators/statement_generator.rs` | Add profile to struct + constructors |
| `src/codegen/generators/expression_generator.rs` | Add profile to struct + constructors |
| `src/codegen/generators/data_type_generator.rs` | Add profile to `DataTypeGenerator` |
| `src/codegen/generators/variable_generator.rs` | Add profile to `VariableGenerator` |
| `src/test_utils.rs` | Pass default profile in test codegen helpers |

### Context for Implementer

- `GlobalContext` is in `compiler/plc_index/src/lib.rs:21`. It already has `error_fmt` and `generate_external_constructors` as compile-time config fields (with a TODO to clean this up).
- `CompileOptions` is in `compiler/plc_driver/src/lib.rs:45`.
- Codegen chain: `CodeGen::generate()` (codegen.rs:325) creates `PouGenerator` (line 337). `PouGenerator` creates `StatementCodeGenerator`. `StatementCodeGenerator::create_expr_generator()` creates `ExpressionCodeGenerator`.
- `ExpressionCodeGenerator` struct is at `expression_generator.rs:48` — add an `Arc<CompatibilityProfile>` field.
- All existing tests must continue to pass with the default `standard` profile.

---

## Phase 4: Book Documentation

**Goal**: Document the profile system in the book.

### Tasks

- [ ] Create `book/src/using_rusty/compatibility_profiles.md`
  - Explain what profiles are and why they exist
  - Document the `--profile` flag
  - Document built-in profiles (`standard`, `codesys`)
  - Document the file format with examples
  - Explain how to export and customize profiles (`plc config profile`)
  - Explain forward compatibility (unknown flags are skipped)
  - Section for each behavior flag (empty initially, grows as flags are added)
- [ ] Update `book/src/using_rusty/error_configuration.md`
  - Mention that `--error-config` is automatically converted to a profile internally
  - Point users to the new profile system as the recommended approach
  - Explain that their existing JSON works inside a profile's `diagnostics` section
- [ ] Update `book/src/SUMMARY.md`
  - Add `Compatibility Profiles` entry under `Using RuSTy`

### Key Files

| File | Action |
|------|--------|
| `book/src/using_rusty/compatibility_profiles.md` | **New** |
| `book/src/using_rusty/error_configuration.md` | Mention profile conversion, link to new docs |
| `book/src/SUMMARY.md` | Add new entry |

---

## Phase 5: First Behavior Flag — `short_circuit_bool_ops`

**Goal**: Implement the first concrete behavior toggle. When `false`, AND/OR evaluate both operands.

### Tasks

- [ ] Add `short_circuit_bool_ops: bool` to `BehaviorFlags` (default: `true`)
- [ ] Update built-in `codesys` profile to set `short_circuit_bool_ops: false`
- [ ] **Lowering** (approach A — pre-evaluate operands):
  - Create a new lowering participant or extend `ControlStatementParticipant` (`compiler/plc_lowering/src/control_statement.rs`)
  - When `short_circuit_bool_ops = false`, transform `a AND b` → `__tmp_a := a; __tmp_b := b; __tmp_a AND __tmp_b`
  - This guarantees both sides are evaluated regardless of codegen
  - Participant receives profile at construction from `GlobalContext`
- [ ] **Codegen** (approach B — eager evaluation):
  - In `generate_bool_binary_expression` (`expression_generator.rs:2664`), check `self.profile.behaviors.short_circuit_bool_ops`
  - If `false`, call new `generate_bool_eager_expression()` which evaluates both sides and applies `build_and`/`build_or` without branching
  - If `true`, call existing `generate_bool_short_circuit_expression()` (current behavior)
- [ ] **Validation** (optional info diagnostic):
  - When profile disables short-circuit, emit an info-level diagnostic at AND/OR usage sites (or a one-time note)
  - This is optional and can be deferred
- [ ] **Tests**:
  - Lit test: `--profile codesys` with AND/OR — verify both sides are evaluated (e.g. both sides call a function with side effects)
  - Lit test: default profile — verify short-circuit still works
  - Unit test: codegen IR differs between profiles
- [ ] **Book**: Add `short_circuit_bool_ops` documentation to the behavior flags section in `compatibility_profiles.md`

### Context for Implementer

- `generate_bool_binary_expression` is at `expression_generator.rs:2664`. Lines 2671-2672 unconditionally call `generate_bool_short_circuit_expression`.
- `generate_bool_short_circuit_expression` is at line 2716. It uses LLVM conditional branches + phi nodes.
- The eager version is simpler: evaluate both sides into i1 values, then `builder.build_and(lhs, rhs)` or `builder.build_or(lhs, rhs)`.
- For the lowering approach, look at `ControlStatementParticipant` (`compiler/plc_lowering/src/control_statement.rs`) as a model — it already transforms conditional expressions by extracting sub-expressions into temporaries.

---

## Future Phases (not yet planned in detail)

- Additional behavior flags (implicit widening, type sizes, string defaults, etc.)
- Per-flag documentation in the book
- Profile inheritance (a custom profile extending a named one)
- Potential build config integration (profile specified in `plc.json` build file)

---

## Discussion: Profile Crate Placement

The `CompatibilityProfile` and `BehaviorFlags` types currently live in `plc_diagnostics`. This was a pragmatic choice: all crates that need the profile (`plc_index`, `plc_lowering`, the main `plc` crate, `plc_driver`) already depend on `plc_diagnostics`, and the profile struct uses `DiagnosticsConfiguration` which is defined there.

However, `plc_diagnostics` is semantically about error/warning reporting — profiles are a broader compiler configuration concern. Options to consider post-implementation:

1. **Rename `plc_diagnostics`** to something broader (e.g. `plc_core`, `plc_config`) — this reflects that it already contains configuration types (`DiagnosticsConfiguration`, `Severity`) alongside reporting. Disruptive but accurate.

2. **Create a new `plc_config` crate** — holds `CompatibilityProfile`, `BehaviorFlags`, and potentially `DiagnosticsConfiguration` (which is profile-adjacent). Both `plc_diagnostics` and `plc_driver` would depend on it. This is the cleanest separation but adds a crate.

3. **Move profiles to `plc_util`** — the existing shared utility crate. Would need `serde` and `log` as deps and a way to reference `DiagnosticsConfiguration` (either by moving it too, or making the diagnostics field generic/optional at the core level).

4. **Keep in `plc_diagnostics`** — accept the naming mismatch as pragmatic. The crate already has non-diagnostic types and this avoids churn.

**Decision**: defer until after Phase 5 (first behavior flag), then reassess based on how many crates actually import the profile types and whether the naming friction causes confusion.

---

## Reference: Key Source Locations

| Component | File | Line |
|-----------|------|------|
| CLI flags | `compiler/plc_driver/src/cli.rs` | `CompileParameters` at 38, `--error-config` at 343 |
| `CompileOptions` | `compiler/plc_driver/src/lib.rs` | 45 |
| `GlobalContext` | `compiler/plc_index/src/lib.rs` | 21 |
| Pipeline init | `compiler/plc_driver/src/pipelines.rs` | `TryFrom` at 83, error config at 103, participants at 335 |
| `DiagnosticsConfiguration` | `compiler/plc_diagnostics/src/diagnostics/diagnostics_registry.rs` | 87 |
| `DiagnosticsRegistry::with_configuration` | same file | 47 |
| Validator | `src/validation.rs` | 104, receives `&GlobalContext` |
| Lowering participants | `compiler/plc_lowering/src/` | Various |
| `CodeGen::new` | `src/codegen.rs` | 99 |
| `CodeGen::generate` | `src/codegen.rs` | 325 |
| `PouGenerator::new` | `src/codegen/generators/pou_generator.rs` | ~336 |
| `ExpressionCodeGenerator` | `src/codegen/generators/expression_generator.rs` | struct at 48, bool expr at 2664 |
| `generate_bool_short_circuit_expression` | same file | 2716 |
| Config subcommand | `compiler/plc_driver/src/cli.rs` | `ConfigOption` at 452 |
| `print_config_options` | `compiler/plc_driver/src/pipelines.rs` | 303 |
| Book summary | `book/src/SUMMARY.md` | - |
| Error config docs | `book/src/using_rusty/error_configuration.md` | - |
