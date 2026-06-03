//! Sidecar map from user-facing qualified variable names to the synthetic global
//! names (`__PI_…`, `__M_…`, `__G_…`) the pre-processor emits for hardware-bound
//! variables.
//!
//! Maps synthetic names to source-level identities.
//!
//! # Wire format
//!
//! The serialized output is a stability contract with downstream tools (live-monitoring
//! debuggers, IDE bridges) that parse DWARF and need to translate user-facing symbol
//! subscriptions into the mangled globals the codegen actually emits. The shape is:
//!
//! ```json
//! {
//!   "VariableMap": [
//!     {
//!       "name": "prg.foo.bar",
//!       "mangled_name": "__PI_1_2",
//!       "address": "%IX1.2",
//!       "direction": "Input",
//!       "access_type": "Bit"
//!     }
//!   ]
//! }
//! ```
//!
//! Stable keys and string values:
//! - `VariableMap` — the top-level wrapper.
//! - Per-entry keys: `name`, `mangled_name`, `address`, `direction`, `access_type`.
//! - `direction` values: `Input`, `Output`, `Memory`, `Global`.
//! - `access_type` values: `Bit`, `Byte`, `Word`, `DWord`, `LWord`, `Template`.
//!
//! These strings are produced by `#[derive(Serialize)]` on the local [`WireDirection`]
//! and [`WireAccess`] enums in this module rather than the AST types, so renaming a
//! variant in `plc_ast` cannot silently change the wire format. Adding a new variant
//! to either AST enum will fail to compile here via the exhaustive `From` impls.

use plc_ast::ast::{mangle_hw_name, AstStatement, DirectAccessType, HardwareAccessType};
use plc_diagnostics::diagnostics::Diagnostic;
use serde::Serialize;

use crate::{expression_path::ExpressionPath, index::Index, ConfigFormat};

/// One row in the emitted map. `name` is the qualified user-facing path;
/// `mangled_name` is the synthetic global as it appears in DWARF.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HwMapEntry {
    pub name: String,
    pub mangled_name: String,
    pub address: String,
    pub direction: WireDirection,
    pub access_type: WireAccess,
}

/// Wire-format enum for `direction`. Mirrors [`HardwareAccessType`] but lives here so
/// the JSON/TOML contract (see module-level docs) is owned by this module and not by
/// the AST.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum WireDirection {
    Input,
    Output,
    Memory,
    Global,
}

impl From<HardwareAccessType> for WireDirection {
    fn from(d: HardwareAccessType) -> Self {
        match d {
            HardwareAccessType::Input => Self::Input,
            HardwareAccessType::Output => Self::Output,
            HardwareAccessType::Memory => Self::Memory,
            HardwareAccessType::Global => Self::Global,
        }
    }
}

/// Wire-format enum for `access_type`. Mirrors [`DirectAccessType`] but lives here so
/// the JSON/TOML contract (see module-level docs) is owned by this module and not by
/// the AST.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum WireAccess {
    Bit,
    Byte,
    Word,
    DWord,
    LWord,
    Template,
}

impl From<DirectAccessType> for WireAccess {
    fn from(a: DirectAccessType) -> Self {
        match a {
            DirectAccessType::Bit => Self::Bit,
            DirectAccessType::Byte => Self::Byte,
            DirectAccessType::Word => Self::Word,
            DirectAccessType::DWord => Self::DWord,
            DirectAccessType::LWord => Self::LWord,
            DirectAccessType::Template => Self::Template,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HwMap {
    #[serde(rename = "VariableMap")]
    pub entries: Vec<HwMapEntry>,
}

/// Walks the index and produces one entry per (instance, hardware address) pair.
/// Two passes that are disjoint by construction (and thus the `(name, mangled)`
/// dedup set across them is a defensive belt — see notes below):
/// - Pass A: instances with direct hardware bindings (`AT %IX0.0`, `AT %QW2.5`, …).
///   Templates (`AT %I*`) are skipped — they are resolved in pass B.
/// - Pass B: `VAR_CONFIG` entries, which resolve templated FB members to a concrete
///   address. The reference is already instance-qualified by the user. The synthetic
///   global the pre-processor produces for these has `binding: None`, so the same
///   instance cannot show up in pass A.
pub fn collect_hw_map(index: &Index) -> Result<HwMap, Diagnostic> {
    let mut entries = Vec::new();
    // Keyed on (qualified name, mangled global). Pass A and pass B do not overlap by
    // design (see fn-level doc); this set guards against a future regression where a
    // single instance gains both a direct binding and a VAR_CONFIG entry.
    let mut seen = rustc_hash::FxHashSet::<(String, String)>::default();

    // Pass A — direct hardware bindings on indexed instances.
    for (path, variable) in index.find_instances() {
        let Some(binding) = variable.get_hardware_binding() else { continue };
        if matches!(binding.access, DirectAccessType::Template) {
            continue;
        }
        let address_ints = binding
            .entries
            .iter()
            .map(|id| index.get_const_expressions().get_constant_int_statement_value(id))
            .collect::<Result<Vec<i128>, _>>()
            .map_err(|msg| Diagnostic::new(msg).with_error_code("E002"))?;

        let mangled = mangle_hw_name(binding.direction, &address_ints);
        let address_literal = render_address(binding.direction, binding.access, &address_ints);

        for name in path.expand(index) {
            if seen.insert((name.clone(), mangled.clone())) {
                entries.push(HwMapEntry {
                    name,
                    mangled_name: mangled.clone(),
                    address: address_literal.clone(),
                    direction: binding.direction.into(),
                    access_type: binding.access.into(),
                });
            }
        }
    }

    // Pass B — VAR_CONFIG resolutions.
    for var_config in index.get_config_variables() {
        let AstStatement::HardwareAccess(hw) = &var_config.address.stmt else { continue };
        if matches!(hw.access, DirectAccessType::Template) {
            continue;
        }
        // The parser only accepts integer literals in `AT %…` slots, so any non-literal
        // would be a parser/validation contract violation — surface it instead of
        // silently skipping the entry.
        let address_ints = hw
            .address
            .iter()
            .map(|node| {
                node.get_literal_integer_value().ok_or_else(|| {
                    Diagnostic::new("VAR_CONFIG hardware address contains non-literal part")
                        .with_error_code("E002")
                        .with_location(&var_config.location)
                })
            })
            .collect::<Result<Vec<i128>, _>>()?;
        if address_ints.is_empty() {
            continue;
        }
        let mangled = mangle_hw_name(hw.direction, &address_ints);
        let address_literal = render_address(hw.direction, hw.access, &address_ints);

        let path = match ExpressionPath::try_from(var_config) {
            Ok(p) => p,
            Err(_) => continue, // malformed reference; validation will have flagged it.
        };
        for name in path.expand(index) {
            if seen.insert((name.clone(), mangled.clone())) {
                entries.push(HwMapEntry {
                    name,
                    mangled_name: mangled.clone(),
                    address: address_literal.clone(),
                    direction: hw.direction.into(),
                    access_type: hw.access.into(),
                });
            }
        }
    }

    Ok(HwMap { entries })
}

pub fn serialize_hw_map(map: &HwMap, format: ConfigFormat) -> Result<String, Diagnostic> {
    // A serializer failure here is a contract violation between this module and serde —
    // every field is a plain string or a derived enum. Surface it as an internal error.
    match format {
        ConfigFormat::JSON => serde_json::to_string_pretty(map).map_err(|e| {
            Diagnostic::new(e.to_string()).with_error_code("E002").with_internal_error(e.into())
        }),
        ConfigFormat::TOML => toml::ser::to_string_pretty(map).map_err(|e| {
            Diagnostic::new(e.to_string()).with_error_code("E002").with_internal_error(e.into())
        }),
    }
}

/// Reconstructs the source-form literal (`%IX1.2.3.4`) from its parts. Mirrors the
/// lexer's parsing rule in [src/lexer.rs] (direction letter at slice[1],
/// access letter at slice[2]).
fn render_address(direction: HardwareAccessType, access: DirectAccessType, address: &[i128]) -> String {
    let dir = match direction {
        HardwareAccessType::Input => 'I',
        HardwareAccessType::Output => 'Q',
        HardwareAccessType::Memory => 'M',
        HardwareAccessType::Global => 'G',
    };
    let acc = match access {
        DirectAccessType::Bit => 'X',
        DirectAccessType::Byte => 'B',
        DirectAccessType::Word => 'W',
        DirectAccessType::DWord => 'D',
        DirectAccessType::LWord => 'L',
        DirectAccessType::Template => '*',
    };
    let joined = address.iter().map(ToString::to_string).collect::<Vec<_>>().join(".");
    format!("%{dir}{acc}{joined}")
}

#[cfg(test)]
mod tests {
    use super::{collect_hw_map, serialize_hw_map};
    use crate::{test_utils::tests::index, ConfigFormat};

    #[test]
    fn direct_global_binding() {
        let (_, idx) = index(
            "
        VAR_GLOBAL
            a AT %IX0.0 : BOOL;
            b AT %QW2.5 : WORD;
        END_VAR",
        );
        let map = collect_hw_map(&idx).unwrap();
        let names: Vec<&str> = map.entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"a"));
        assert!(names.contains(&"b"));
        let b = map.entries.iter().find(|e| e.name == "b").unwrap();
        assert_eq!(b.mangled_name, "__PI_2_5");
        assert_eq!(b.address, "%QW2.5");
    }

    #[test]
    fn shared_address_two_user_vars() {
        // `b, c AT %QW2.5` declares two user-facing names that share one synthetic global.
        let (_, idx) = index(
            "
        VAR_GLOBAL
            b,c AT %QW2.5 : WORD;
        END_VAR",
        );
        let map = collect_hw_map(&idx).unwrap();
        let entries: Vec<_> = map.entries.iter().filter(|e| e.name == "b" || e.name == "c").collect();
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().all(|e| e.mangled_name == "__PI_2_5"));
    }

    #[test]
    fn fb_array_instances_expand() {
        let (_, idx) = index(
            "
        FUNCTION_BLOCK fb
        VAR
            a AT %IX1.2 : BOOL;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL
            arr : ARRAY[0..2] OF fb;
        END_VAR",
        );
        let map = collect_hw_map(&idx).unwrap();
        let mut names: Vec<&str> =
            map.entries.iter().filter(|e| e.name.starts_with("arr")).map(|e| e.name.as_str()).collect();
        names.sort();
        assert_eq!(names, vec!["arr[0].a", "arr[1].a", "arr[2].a"]);
        assert!(map
            .entries
            .iter()
            .filter(|e| e.name.starts_with("arr"))
            .all(|e| e.mangled_name == "__PI_1_2"));
    }

    #[test]
    fn template_direct_bindings_are_skipped() {
        let (_, idx) = index(
            "
        VAR_GLOBAL
            a AT %I* : DWORD;
        END_VAR",
        );
        let map = collect_hw_map(&idx).unwrap();
        assert!(map.entries.iter().all(|e| e.name != "a"));
    }

    #[test]
    fn var_config_resolves_template_to_mangled_global() {
        let (_, idx) = index(
            "
        FUNCTION_BLOCK fb
        VAR
            bar AT %I* : BOOL;
        END_VAR
        END_FUNCTION_BLOCK
        PROGRAM prg
        VAR
            foo : fb;
        END_VAR
        END_PROGRAM
        VAR_CONFIG
            prg.foo.bar AT %IX1.2.3.4 : BOOL;
        END_VAR",
        );
        let map = collect_hw_map(&idx).unwrap();
        let entry = map
            .entries
            .iter()
            .find(|e| e.name == "prg.foo.bar")
            .expect("VAR_CONFIG resolution should appear in the map");
        assert_eq!(entry.mangled_name, "__PI_1_2_3_4");
        assert_eq!(entry.address, "%IX1.2.3.4");
    }

    #[test]
    fn memory_and_global_directions_are_mangled_correctly() {
        // Pins the `__M_*` and `__G_*` prefix contracts. If the synthetic-global naming
        // scheme ever changes, this fails loudly and reminds the author to update the
        // runtime that consumes this map.
        let (_, idx) = index(
            "
        VAR_GLOBAL
            mem AT %MX7.8 : BOOL;
            glob AT %GX0.1 : BOOL;
        END_VAR",
        );
        let map = collect_hw_map(&idx).unwrap();
        let mem = map.entries.iter().find(|e| e.name == "mem").expect("memory entry missing");
        assert_eq!(mem.mangled_name, "__M_7_8");
        assert_eq!(mem.address, "%MX7.8");
        let glob = map.entries.iter().find(|e| e.name == "glob").expect("global entry missing");
        assert_eq!(glob.mangled_name, "__G_0_1");
        assert_eq!(glob.address, "%GX0.1");
    }

    #[test]
    fn struct_hardware_members_are_collected() {
        // `process_struct_hardware_variables` is a separate code path from the GVL/POU
        // pre-processing. The synthetic global it creates is a sibling of the struct
        // member, so the entry should appear via the qualified instance path.
        let (_, idx) = index(
            "
        TYPE my_struct : STRUCT
            c AT %IX1.2 : BOOL;
        END_STRUCT END_TYPE
        VAR_GLOBAL
            inst : my_struct;
        END_VAR",
        );
        let map = collect_hw_map(&idx).unwrap();
        let entry = map.entries.iter().find(|e| e.name == "inst.c").expect("inst.c missing");
        assert_eq!(entry.mangled_name, "__PI_1_2");
    }

    #[test]
    fn no_hardware_vars_yields_empty_map() {
        let (_, idx) = index(
            "
        VAR_GLOBAL
            counter : INT;
        END_VAR
        PROGRAM prg
        VAR
            x : INT;
        END_VAR
        END_PROGRAM",
        );
        let map = collect_hw_map(&idx).unwrap();
        assert!(map.entries.is_empty(), "expected empty map, got {:?}", map.entries);
        insta::assert_snapshot!(serialize_hw_map(&map, ConfigFormat::JSON).unwrap(), @r#"
        {
          "VariableMap": []
        }
        "#);
    }

    #[test]
    fn json_and_toml_output_snapshots() {
        let (_, idx) = index(
            "
        FUNCTION_BLOCK fb
        VAR
            a AT %IX0.0 : BOOL;
            b AT %QW2.5 : WORD;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL
            inst : fb;
            arr  : ARRAY[0..1] OF fb;
        END_VAR",
        );
        let map = collect_hw_map(&idx).unwrap();
        insta::assert_snapshot!(serialize_hw_map(&map, ConfigFormat::JSON).unwrap(), @r#"
        {
          "VariableMap": [
            {
              "name": "inst.a",
              "mangled_name": "__PI_0_0",
              "address": "%IX0.0",
              "direction": "Input",
              "access_type": "Bit"
            },
            {
              "name": "inst.b",
              "mangled_name": "__PI_2_5",
              "address": "%QW2.5",
              "direction": "Output",
              "access_type": "Word"
            },
            {
              "name": "arr[0].a",
              "mangled_name": "__PI_0_0",
              "address": "%IX0.0",
              "direction": "Input",
              "access_type": "Bit"
            },
            {
              "name": "arr[1].a",
              "mangled_name": "__PI_0_0",
              "address": "%IX0.0",
              "direction": "Input",
              "access_type": "Bit"
            },
            {
              "name": "arr[0].b",
              "mangled_name": "__PI_2_5",
              "address": "%QW2.5",
              "direction": "Output",
              "access_type": "Word"
            },
            {
              "name": "arr[1].b",
              "mangled_name": "__PI_2_5",
              "address": "%QW2.5",
              "direction": "Output",
              "access_type": "Word"
            }
          ]
        }
        "#);
        insta::assert_snapshot!(serialize_hw_map(&map, ConfigFormat::TOML).unwrap(), @r#"
        [[VariableMap]]
        name = "inst.a"
        mangled_name = "__PI_0_0"
        address = "%IX0.0"
        direction = "Input"
        access_type = "Bit"

        [[VariableMap]]
        name = "inst.b"
        mangled_name = "__PI_2_5"
        address = "%QW2.5"
        direction = "Output"
        access_type = "Word"

        [[VariableMap]]
        name = "arr[0].a"
        mangled_name = "__PI_0_0"
        address = "%IX0.0"
        direction = "Input"
        access_type = "Bit"

        [[VariableMap]]
        name = "arr[1].a"
        mangled_name = "__PI_0_0"
        address = "%IX0.0"
        direction = "Input"
        access_type = "Bit"

        [[VariableMap]]
        name = "arr[0].b"
        mangled_name = "__PI_2_5"
        address = "%QW2.5"
        direction = "Output"
        access_type = "Word"

        [[VariableMap]]
        name = "arr[1].b"
        mangled_name = "__PI_2_5"
        address = "%QW2.5"
        direction = "Output"
        access_type = "Word"
        "#);
    }

    #[test]
    fn var_config_output_snapshots() {
        // Mixes pass A (direct memory binding) with pass B (per-instance VAR_CONFIG
        // resolutions of templated FB members). Each FB instance gets its own pair
        // of resolved hardware addresses; the synthetic `__PI_*` / `__M_*` names
        // they map to are pinned by these snapshots.
        let (_, idx) = index(
            "
        FUNCTION_BLOCK valve
        VAR
            open_sensor AT %I* : BOOL;
            position    AT %Q* : WORD;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM prg
        VAR
            intake         : valve;
            outflow        : valve;
            pressure_alarm AT %MX5.0 : BOOL;
        END_VAR
        END_PROGRAM

        VAR_CONFIG
            prg.intake.open_sensor  AT %IX0.1 : BOOL;
            prg.intake.position     AT %QW1.0 : WORD;
            prg.outflow.open_sensor AT %IX0.2 : BOOL;
            prg.outflow.position    AT %QW1.1 : WORD;
        END_VAR",
        );
        let map = collect_hw_map(&idx).unwrap();
        insta::assert_snapshot!(serialize_hw_map(&map, ConfigFormat::JSON).unwrap(), @r#"
        {
          "VariableMap": [
            {
              "name": "prg.pressure_alarm",
              "mangled_name": "__M_5_0",
              "address": "%MX5.0",
              "direction": "Memory",
              "access_type": "Bit"
            },
            {
              "name": "prg.intake.open_sensor",
              "mangled_name": "__PI_0_1",
              "address": "%IX0.1",
              "direction": "Input",
              "access_type": "Bit"
            },
            {
              "name": "prg.intake.position",
              "mangled_name": "__PI_1_0",
              "address": "%QW1.0",
              "direction": "Output",
              "access_type": "Word"
            },
            {
              "name": "prg.outflow.open_sensor",
              "mangled_name": "__PI_0_2",
              "address": "%IX0.2",
              "direction": "Input",
              "access_type": "Bit"
            },
            {
              "name": "prg.outflow.position",
              "mangled_name": "__PI_1_1",
              "address": "%QW1.1",
              "direction": "Output",
              "access_type": "Word"
            }
          ]
        }
        "#);
        insta::assert_snapshot!(serialize_hw_map(&map, ConfigFormat::TOML).unwrap(), @r#"
        [[VariableMap]]
        name = "prg.pressure_alarm"
        mangled_name = "__M_5_0"
        address = "%MX5.0"
        direction = "Memory"
        access_type = "Bit"

        [[VariableMap]]
        name = "prg.intake.open_sensor"
        mangled_name = "__PI_0_1"
        address = "%IX0.1"
        direction = "Input"
        access_type = "Bit"

        [[VariableMap]]
        name = "prg.intake.position"
        mangled_name = "__PI_1_0"
        address = "%QW1.0"
        direction = "Output"
        access_type = "Word"

        [[VariableMap]]
        name = "prg.outflow.open_sensor"
        mangled_name = "__PI_0_2"
        address = "%IX0.2"
        direction = "Input"
        access_type = "Bit"

        [[VariableMap]]
        name = "prg.outflow.position"
        mangled_name = "__PI_1_1"
        address = "%QW1.1"
        direction = "Output"
        access_type = "Word"
        "#);
    }
}
