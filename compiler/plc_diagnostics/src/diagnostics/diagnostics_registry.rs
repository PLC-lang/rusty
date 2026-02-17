/// Returns a diagnostics map with the error code, default severity and a description
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::diagnostician::DiagnosticAssessor;

use super::{
    Diagnostic,
    Severity::{self, Error, Info, Warning},
};

macro_rules! add_diagnostic {
    ($($number:ident, $severity:expr, $desc:expr,)*) => {
        { let mut m : FxHashMap<&str, DiagnosticEntry> = FxHashMap::default();
            $(
                {
                    let code = stringify!($number);
                    m.insert(code, DiagnosticEntry{ code, severity: $severity, description: $desc});
                }
            )*
            m
        }}
}

pub struct DiagnosticsRegistry(FxHashMap<&'static str, DiagnosticEntry>);

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiagnosticEntry {
    code: &'static str,
    severity: Severity,
    description: &'static str,
}

impl Default for DiagnosticsRegistry {
    fn default() -> Self {
        Self::new(DIAGNOSTICS.clone())
    }
}

impl DiagnosticsRegistry {
    /// Creates an empty registry.
    fn new(map: FxHashMap<&'static str, DiagnosticEntry>) -> Self {
        DiagnosticsRegistry(map)
    }

    pub fn with_configuration(mut self, config: DiagnosticsConfiguration) -> Self {
        for (severity, codes) in config.0 {
            for code in &codes {
                if let Some(entry) = self.0.get_mut(code.as_str()) {
                    entry.severity = severity
                }
            }
        }
        self
    }
}

impl DiagnosticAssessor for DiagnosticsRegistry {
    /// Assesses the diagnostic based on the current registered map. If no entry is found, the
    /// default diagnostic is returned
    fn assess(&self, d: &Diagnostic) -> Severity {
        self.0.get(d.get_error_code()).map(|it| it.severity).unwrap_or_else(|| {
            log::warn!("Unrecognized error code {}. Using default severity", d.get_error_code());
            Severity::default()
        })
    }

    fn explain(&self, error: &str) -> String {
        let Some(info) = self.0.get(error) else {
            return format!("Unknown error {error}");
        };
        format!(
            r"Explanation for error {error}:
{}
",
            info.description
        )
    }

    fn get_diagnostic_configuration(&self) -> String {
        let config: DiagnosticsConfiguration = self.into();
        serde_json::ser::to_string(&config).expect("Cannot fail")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct DiagnosticsConfiguration(FxHashMap<Severity, Vec<String>>);

impl From<&DiagnosticsRegistry> for DiagnosticsConfiguration {
    fn from(registry: &DiagnosticsRegistry) -> Self {
        let mut res = DiagnosticsConfiguration::default();
        for val in registry.0.values() {
            let entry = res.0.entry(val.severity).or_default();
            entry.push(val.code.into());
        }
        res
    }
}

#[rustfmt::skip]
lazy_static! {
    pub static ref DIAGNOSTICS: FxHashMap<&'static str, DiagnosticEntry> = add_diagnostic!(
        E001,   Error,      include_str!("./error_codes/E001.md"), // General Error
        E002,   Error,      include_str!("./error_codes/E002.md"), // General IO Error
        E003,   Error,      include_str!("./error_codes/E003.md"), // Parameter Error
        E004,   Error,      include_str!("./error_codes/E004.md"), // Duplicate Symbol
        E005,   Error,      include_str!("./error_codes/E005.md"), // Generic LLVM Error
        E006,   Error,      include_str!("./error_codes/E006.md"), // Missing Token
        E007,   Error,      include_str!("./error_codes/E007.md"), // Unexpected Token
        E008,   Error,      include_str!("./error_codes/E008.md"), // Invalid Range
        E009,   Error,      include_str!("./error_codes/E009.md"), // Mismatched Parantheses
        E010,   Error,      include_str!("./error_codes/E010.md"), // Invalid Time Literal
        E011,   Error,      include_str!("./error_codes/E011.md"), // Invalid Number
        E012,   Error,      include_str!("./error_codes/E012.md"), // Missing Case Condition
        E013,   Warning,    include_str!("./error_codes/E013.md"), // Keywords shoud contain underscores
        E014,   Warning,    include_str!("./error_codes/E014.md"), // Wrong parantheses type
        E015,   Warning,    include_str!("./error_codes/E015.md"), // Pointer is not standard
        E016,   Warning,    include_str!("./error_codes/E016.md"), // Return types cannot have a default value
        E017,   Error,      include_str!("./error_codes/E017.md"), // Classes cannot contain implementations
        E018,   Error,      include_str!("./error_codes/E018.md"), // Duplicate Label
        E019,   Error,      include_str!("./error_codes/E019.md"), // Classes cannot contain IN_OUT variables
        E020,   Error,      include_str!("./error_codes/E020.md"), // Classes cannot contain a return type
        E021,   Error,      include_str!("./error_codes/E021.md"), // Re-declaration of variable
        E022,   Warning,    include_str!("./error_codes/E022.md"), // Missing action container
        E023,   Warning,    include_str!("./error_codes/E023.md"), // Statement with no effect
        E024,   Warning,    include_str!("./error_codes/E024.md"), // Invalid pragma location
        E025,   Error,      include_str!("./error_codes/E025.md"), // Missing return type
        E026,   Error,      include_str!("./error_codes/E026.md"),
        E027,   Error,      include_str!("./error_codes/E027.md"),
        E028,   Error,      include_str!("./error_codes/E028.md"),
        E029,   Error,      include_str!("./error_codes/E029.md"),
        E030,   Error,      include_str!("./error_codes/E030.md"),
        E031,   Error,      include_str!("./error_codes/E031.md"),
        E032,   Error,      include_str!("./error_codes/E032.md"),
        E033,   Error,      include_str!("./error_codes/E033.md"),
        E034,   Error,      include_str!("./error_codes/E034.md"),
        E035,   Error,      include_str!("./error_codes/E035.md"),
        E036,   Error,      include_str!("./error_codes/E036.md"),
        E037,   Error,      include_str!("./error_codes/E037.md"),
        E038,   Error,      include_str!("./error_codes/E038.md"), // Missing type
        E039,   Warning,    include_str!("./error_codes/E039.md"),
        E040,   Warning,    include_str!("./error_codes/E040.md"),
        E041,   Error,      include_str!("./error_codes/E041.md"),
        E042,   Warning,    include_str!("./error_codes/E042.md"), // Assignment to reference
        E043,   Error,      include_str!("./error_codes/E043.md"),
        E044,   Error,      include_str!("./error_codes/E044.md"),
        E045,   Error,      include_str!("./error_codes/E045.md"),
        E046,   Error,      include_str!("./error_codes/E046.md"),
        E047,   Warning,    include_str!("./error_codes/E047.md"), // VLAs are always by reference
        E048,   Error,      include_str!("./error_codes/E048.md"),
        E049,   Warning,    include_str!("./error_codes/E049.md"),
        E050,   Error,      include_str!("./error_codes/E050.md"),
        E051,   Error,      include_str!("./error_codes/E051.md"),
        E052,   Error,      include_str!("./error_codes/E052.md"),
        E053,   Error,      include_str!("./error_codes/E053.md"),
        E054,   Error,      include_str!("./error_codes/E054.md"),
        E055,   Error,      include_str!("./error_codes/E055.md"),
        E056,   Error,      include_str!("./error_codes/E056.md"),
        E057,   Error,      include_str!("./error_codes/E057.md"),
        E058,   Error,      include_str!("./error_codes/E058.md"),
        E059,   Error,      include_str!("./error_codes/E059.md"),
        E060,   Info,       include_str!("./error_codes/E060.md"), // Variable direct access with %
        E061,   Error,      include_str!("./error_codes/E061.md"),
        E062,   Error,      include_str!("./error_codes/E062.md"),
        E063,   Error,      include_str!("./error_codes/E063.md"),
        E064,   Error,      include_str!("./error_codes/E064.md"),
        E065,   Error,      include_str!("./error_codes/E065.md"),
        E066,   Error,      include_str!("./error_codes/E066.md"),
        E067,   Warning,    include_str!("./error_codes/E067.md"), // Implicit typecast
        E068,   Error,      include_str!("./error_codes/E068.md"),
        E069,   Error,      include_str!("./error_codes/E069.md"),
        E070,   Error,      include_str!("./error_codes/E070.md"),
        E071,   Error,      include_str!("./error_codes/E071.md"),
        E072,   Error,      include_str!("./error_codes/E072.md"),
        E073,   Error,      include_str!("./error_codes/E073.md"),
        E074,   Error,      include_str!("./error_codes/E074.md"),
        E075,   Error,      include_str!("./error_codes/E075.md"),
        E076,   Error,      include_str!("./error_codes/E076.md"),
        E077,   Error,      include_str!("./error_codes/E077.md"),
        E078,   Error,      include_str!("./error_codes/E078.md"),
        E079,   Error,      include_str!("./error_codes/E079.md"),
        E080,   Error,      include_str!("./error_codes/E080.md"),
        E081,   Error,      include_str!("./error_codes/E081.md"),
        E082,   Error,      include_str!("./error_codes/E082.md"),
        E083,   Error,      include_str!("./error_codes/E083.md"),
        E084,   Error,      include_str!("./error_codes/E084.md"),
        E085,   Error,      include_str!("./error_codes/E085.md"),
        E086,   Error,      include_str!("./error_codes/E086.md"),
        E087,   Error,      include_str!("./error_codes/E087.md"),
        E088,   Error,      include_str!("./error_codes/E088.md"),
        E089,   Error,      include_str!("./error_codes/E089.md"),
        E090,   Warning,    include_str!("./error_codes/E090.md"),  // Incompatible reference Assignment
        E091,   Warning,    include_str!("./error_codes/E091.md"),
        E092,   Info,       include_str!("./error_codes/E092.md"),
        E093,   Warning,    include_str!("./error_codes/E093.md"),
        E094,   Error,      include_str!("./error_codes/E094.md"),
        E095,   Error,      include_str!("./error_codes/E095.md"),  // Action call without `()`
        E096,   Warning,    include_str!("./error_codes/E096.md"),  // Integer Condition
        E097,   Error,      include_str!("./error_codes/E097.md"),  // Invalid Array Range
        E098,   Error,      include_str!("./error_codes/E098.md"),  // Invalid `REF=` assignment
        E099,   Error,      include_str!("./error_codes/E099.md"),  // Invalid `REFERENCE TO` declaration
        E100,   Error,      include_str!("./error_codes/E100.md"),  // Immutable variable address
        E101,   Error,      include_str!("./error_codes/E101.md"),  // Invalid VAR_CONFIG / Template Variable Declaration
        E102,   Error,      include_str!("./error_codes/E102.md"),  // Template variable without hardware binding
        E103,   Error,      include_str!("./error_codes/E103.md"),  // Immutable Hardware Binding
        E104,   Error,      include_str!("./error_codes/E104.md"),  // Config Variable With Incomplete Address
        E105,   Error,      include_str!("./error_codes/E105.md"),  // CONSTANT keyword in POU
        E106,   Warning,    include_str!("./error_codes/E106.md"),  // VAR_EXTERNAL have no effect
        E107,   Error,      include_str!("./error_codes/E107.md"),  // Missing configuration for template variable
        E108,   Error,      include_str!("./error_codes/E108.md"),  // Template variable is configured multiple times
        E109,   Error,      include_str!("./error_codes/E109.md"),  // Stateful pointer variable initialized with temporary value
        E110,   Error,      include_str!("./error_codes/E110.md"),  // Invalid POU Type for Interface Implementation
        E111,   Error,      include_str!("./error_codes/E111.md"),  // Duplicate interface methods with different signatures
        E112,   Error,      include_str!("./error_codes/E112.md"),  // Incomplete interface implementation
        E113,   Error,      include_str!("./error_codes/E113.md"),  // Interface default method implementation
        E114,   Error,      include_str!("./error_codes/E114.md"),  // Multiple extensions of same POU
        E115,   Error,      include_str!("./error_codes/E115.md"),  // Property in unsupported POU type
        E116,   Error,      include_str!("./error_codes/E116.md"),  // Property defined in unsupported variable block
        E117,   Error,      include_str!("./error_codes/E117.md"),  // Property with invalid number of GET and/or SET blocks
        E118,   Info,       include_str!("./error_codes/E118.md"),  // Follow-up error to 112
        E119,   Error,      include_str!("./error_codes/E119.md"),  // Invalid use of `SUPER` keyword
        E120,   Error,      include_str!("./error_codes/E120.md"),  // Invalid use of `THIS` keyword
        E121,   Error,      include_str!("./error_codes/E121.md"),  // Recursive type alias
        E122,   Error,      include_str!("./error_codes/E122.md"),  // Invalid enum base type
        E123,   Error,      include_str!("./error_codes/E123.md"),  // Division by zero
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        diagnostician::DiagnosticAssessor,
        diagnostics::{diagnostics_registry::DiagnosticsConfiguration, Diagnostic, Severity},
    };

    use super::DiagnosticsRegistry;

    #[test]
    fn deserialize_empty_json() {
        let error_config = "{}";

        let DiagnosticsConfiguration(configuration) = serde_json::de::from_str(error_config).unwrap();
        assert!(configuration.is_empty());
    }

    #[test]
    fn deserialize_json() {
        let error_config = r#"{
            "error": ["E001", "E002"],
            "warning": ["E003", "E004"],
            "info": ["E005"],
            "ignore": ["E010"]
        }"#;

        let DiagnosticsConfiguration(configuration) = serde_json::de::from_str(error_config).unwrap();
        assert_eq!(configuration.len(), 4);
        assert_eq!(configuration.get(&Severity::Error).unwrap(), &["E001", "E002"]);
        assert_eq!(configuration.get(&Severity::Warning).unwrap(), &["E003", "E004"]);
        assert_eq!(configuration.get(&Severity::Info).unwrap(), &["E005"]);
        assert_eq!(configuration.get(&Severity::Ignore).unwrap(), &["E010"]);
    }

    #[test]
    fn overridden_errors_are_assessed_correctly() {
        let error_config = r#"{
            "error": ["E090"],
            "warning": ["E001"],
            "info": ["E002"],
            "ignore": ["E003"]
        }"#;

        let configuration = serde_json::de::from_str(error_config).unwrap();
        let diagnostics_registry = DiagnosticsRegistry::default().with_configuration(configuration);

        let e090 = Diagnostic::new("Warning->Error").with_error_code("E090");
        let e001 = Diagnostic::new("Error->Warning").with_error_code("E001");
        let e002 = Diagnostic::new("Error->Info").with_error_code("E002");
        let e003 = Diagnostic::new("Error->Ignore").with_error_code("E003");

        assert_eq!(diagnostics_registry.assess(&e090), Severity::Error);
        assert_eq!(diagnostics_registry.assess(&e001), Severity::Warning);
        assert_eq!(diagnostics_registry.assess(&e002), Severity::Info);
        assert_eq!(diagnostics_registry.assess(&e003), Severity::Ignore);
    }
}
