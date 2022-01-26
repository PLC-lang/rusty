use serde::Serialize;

use crate::{
    ast::{DirectAccessType, HardwareAccessType},
    diagnostics::Diagnostic,
    index::Index,
    qualifed_name::QualifiedName,
    ConfigFormat,
};

#[derive(Debug, Serialize, PartialEq)]
pub struct HardwareConfiguration<'idx> {
    qualifed_name: QualifiedName<'idx>,
    direction: HardwareAccessType,
    access_type: DirectAccessType,
    address: Vec<String>,
}

/// Retrieves hardware bindings from all defined instances in the program
pub fn collect_hardware_configuration(index: &Index) -> Result<Vec<HardwareConfiguration>, String> {
    index
        .find_instances()
        .filter(|(_, instance)| !instance.is_constant())
        .filter(|(_, instance)| {
            //Allow arrays and structs through
            // let dt = index.find_effective_type_info(instance.get_type_name()).filter(|it| it.is_array() && it.is_struct());
            // dt.is_some() ||
            instance.has_hardware_binding()
        })
        .map(|(name, instance)| {
            let binding = instance
                .get_hardware_binding()
                .expect("Instance should have a binding");
            binding
                .entries
                .iter()
                .map(|it| {
                    index
                        .get_const_expressions()
                        .get_constant_int_statement_value(it)
                })
                .map(|it| it.map(|it| it.to_string()))
                .collect::<Result<Vec<String>, String>>()
                .map(|address| HardwareConfiguration {
                    qualifed_name: name,
                    access_type: binding.access,
                    address,
                    direction: binding.direction,
                })
        })
        .collect()
}

pub fn write_hardware_configuration(
    config: Vec<HardwareConfiguration>,
    format: ConfigFormat,
    target: &str,
) -> Result<(), Diagnostic> {
    todo!("Write config")
}

#[cfg(test)]
mod tests {
    use crate::test_utils::tests::index;

    use super::collect_hardware_configuration;

    #[test]
    fn hardware_collected_gv() {
        let (_, index) = index(
            "
        VAR_GLOBAL
            a AT %I*: DWORD;
            b,c AT %QW2.5: WORD;
            d AT %Q* : ARRAY[0..10] OF DWORD;
        END_VAR",
        );
        let config = collect_hardware_configuration(&index);
        insta::assert_debug_snapshot!(config);
    }

    #[test]
    fn hardware_collected_program() {
        let (_, index) = index(
            "
        PROGRAM prg
        VAR
            a AT %I*: DWORD;
            b,c AT %QW2.5: WORD;
            d AT %Q* : ARRAY[0..10] OF DWORD;
        END_VAR
        END_PROGRAM",
        );
        let config = collect_hardware_configuration(&index);
        insta::assert_debug_snapshot!(config);
    }
    #[test]
    fn hardware_collected_fb() {
        let (_, index) = index(
            "
        FUNCTION_BLOCK fb
        VAR
            a AT %I*: DWORD;
            b,c AT %QW2.5: WORD;
            d AT %Q* : ARRAY[0..10] OF DWORD;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL
            gFb : fb;
            aFb : ARRAY[0..2] OF fb;
        END_VAR",
        );
        let config = collect_hardware_configuration(&index);
        insta::assert_debug_snapshot!(config);
    }
}
