use plc_ast::ast::{DirectAccessType, HardwareAccessType};
use plc_diagnostics::diagnostics::Diagnostic;
use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

use crate::{expression_path::ExpressionPath, index::Index, ConfigFormat};

trait SerializeWithContext {
    fn serialize<S>(&self, ctx: &Index, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

impl<T> SerializeWithContext for Vec<T>
where
    T: SerializeWithContext,
{
    fn serialize<S>(&self, ctx: &Index, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut all = serializer.serialize_seq(Some(self.len()))?;
        for ele in self {
            all.serialize_element(&WithContext::new(ctx, ele))?;
        }
        all.end()
    }
}

pub struct HardwareConfiguration<'idx> {
    index: &'idx Index,
    hardware_binding: Vec<HardwareBinding<'idx>>,
}

struct WithContext<'a, T: SerializeWithContext> {
    context: &'a Index,
    element: &'a T,
}
impl<'a, T> WithContext<'a, T>
where
    T: SerializeWithContext,
{
    fn new(context: &'a Index, element: &'a T) -> Self {
        WithContext { context, element }
    }
}

impl<T> Serialize for WithContext<'_, T>
where
    T: SerializeWithContext,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.element.serialize(self.context, serializer)
    }
}

impl Serialize for HardwareConfiguration<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bindings: Vec<WithContext<HardwareBinding>> =
            self.hardware_binding.iter().map(|it| WithContext::new(self.index, it)).collect();
        let mut config = serializer.serialize_struct("Configuration", 1)?;
        config.serialize_field("HardwareConfiguration", &bindings)?;
        config.end()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HardwareBinding<'idx> {
    qualifed_name: ExpressionPath<'idx>,
    direction: HardwareAccessType,
    access_type: DirectAccessType,
    address: Vec<String>,
}
impl<'idx> HardwareBinding<'idx> {
    fn expand(&self, index: &'idx Index) -> Vec<ExpandedHardwareBinding> {
        let names = self.qualifed_name.expand(index);
        names
            .iter()
            .map(|it| ExpandedHardwareBinding {
                name: it.clone(),
                direction: self.direction,
                access_type: self.access_type,
                address: self.address.clone(),
            })
            .collect()
    }
}

#[derive(Serialize)]
struct ExpandedHardwareBinding {
    name: String,
    #[serde(flatten)]
    direction: HardwareAccessType,
    #[serde(flatten)]
    access_type: DirectAccessType,
    address: Vec<String>,
}

impl SerializeWithContext for HardwareBinding<'_> {
    fn serialize<S>(&self, ctx: &Index, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bindings = self.expand(ctx);
        let mut ser = serializer.serialize_seq(Some(bindings.len()))?;
        for binding in bindings {
            ser.serialize_element(&binding)?;
        }
        ser.end()
    }
}

/// Retrieves hardware bindings from all defined instances in the program
pub fn collect_hardware_configuration(index: &Index) -> Result<HardwareConfiguration, Diagnostic> {
    let conf: Result<Vec<HardwareBinding>, String> = index
        //Avoid arrays that are not representing structural types
        .find_instances()
        .filter(|(_, instance)| instance.has_hardware_binding())
        .map(|(name, instance)| {
            let binding = instance.get_hardware_binding().expect("Instance should have a binding");
            binding
                .entries
                .iter()
                .map(|it| index.get_const_expressions().get_constant_int_statement_value(it))
                .map(|it| it.map(|it| it.to_string()))
                .collect::<Result<Vec<String>, String>>()
                .map(|address| HardwareBinding {
                    qualifed_name: name,
                    access_type: binding.access,
                    address,
                    direction: binding.direction,
                })
        })
        .collect();

    conf.map(|hardware_binding| HardwareConfiguration { index, hardware_binding })
        .map_err(|message| Diagnostic::new(message).with_error_code("E002"))
}

pub fn generate_hardware_configuration(
    config: &HardwareConfiguration,
    format: ConfigFormat,
) -> Result<String, Diagnostic> {
    match format {
        ConfigFormat::JSON => serde_json::to_string_pretty(&config).map_err(|e| {
            Diagnostic::new(e.to_string()).with_error_code("E002").with_internal_error(e.into())
        }),
        ConfigFormat::TOML => toml::ser::to_string_pretty(&config).map_err(|e| {
            Diagnostic::new(e.to_string()).with_error_code("E002").with_internal_error(e.into())
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        hardware_binding::{collect_hardware_configuration, generate_hardware_configuration},
        test_utils::tests::index,
        ConfigFormat,
    };

    #[test]
    fn hardware_collected_gv() {
        let (_, index) = index(
            "
        VAR_GLOBAL
            a AT %I*: DWORD;
            b,c AT %QW2.5: WORD;
            d AT %Q* : ARRAY[0..10] OF DWORD;
            x AT %Q* : INT;
            y AT %QW1 : INT;
            z AT %IW1.2 : INT;
        END_VAR",
        );
        let config = collect_hardware_configuration(&index).unwrap().hardware_binding;
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
            x AT %Q* : INT;
            y AT %QW1 : INT;
            z AT %IW1.2 : INT;
        END_VAR
        END_PROGRAM",
        );
        let config = collect_hardware_configuration(&index).unwrap().hardware_binding;
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
            x AT %Q* : INT;
            y AT %QW1 : INT;
            z AT %IW1.2 : INT;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL
            gFb : fb;
            aFb : ARRAY[0..2] OF fb;
        END_VAR",
        );
        let config = collect_hardware_configuration(&index).unwrap().hardware_binding;
        insta::assert_debug_snapshot!(config);
    }

    #[test]
    fn hardware_printed_fb_no_arrays() {
        let (_, index) = index(
            "
        FUNCTION_BLOCK fb
        VAR
            a AT %I*: DWORD;
            b,c AT %QW2.5: WORD;
            x AT %Q* : INT;
            y AT %QW1 : INT;
            z AT %IW1.2 : INT;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL
            gFb : fb;
            aFb : ARRAY[0..2] OF fb;
            aFb2 : ARRAY[0..2,1..2] OF fb;
            aFb3 : ARRAY[0..2] OF ARRAY[1..2] OF fb;
        END_VAR",
        );
        let config = collect_hardware_configuration(&index).unwrap();
        let res = generate_hardware_configuration(&config, ConfigFormat::JSON).unwrap();
        insta::assert_snapshot!(res);
        let res = generate_hardware_configuration(&config, ConfigFormat::TOML).unwrap();
        insta::assert_snapshot!(res);
    }

    #[test]
    fn hardware_printed_fb() {
        let (_, index) = index(
            "
        FUNCTION_BLOCK fb
        VAR
            a AT %I*: DWORD;
            b,c AT %QW2.5: WORD;
            d AT %Q* : ARRAY[0..1] OF DWORD;
            e AT %Q* : ARRAY[0..1,1..2] OF DWORD;
            f AT %Q* : ARRAY[0..1] OF ARRAY[1..2] OF DWORD;
            x AT %Q* : INT;
            y AT %QW1 : INT;
            z AT %IW1.2 : INT;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL
            gFb : fb;
            aFb : ARRAY[0..2] OF fb;
            aFb2 : ARRAY[0..2,1..2] OF fb;
            aFb3 : ARRAY[0..2] OF ARRAY[1..2] OF fb;
        END_VAR",
        );
        let config = collect_hardware_configuration(&index).unwrap();
        let res = generate_hardware_configuration(&config, ConfigFormat::JSON).unwrap();
        insta::assert_snapshot!(res);
        let res = generate_hardware_configuration(&config, ConfigFormat::TOML).unwrap();
        insta::assert_snapshot!(res);
    }
}
