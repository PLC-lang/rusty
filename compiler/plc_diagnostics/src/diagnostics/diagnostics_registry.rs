/// Returns a diagnostics map with the error code, default severity and a description
macro_rules! add_diagnostic {
    ($($number:ident, $severity:expr, $desc:expr,)*) => {
        { let mut m : HashMap<&str, DiagnosticEntry> = HashMap::new();
            $(
                {
                    let code = stringify!($number);
                    m.insert(code, DiagnosticEntry{ code, severity: $severity, description: $desc});
                }
            )*
            m
        }}
}

use std::collections::HashMap;

use crate::diagnostician::DiagnosticAssessor;

use super::{
    Diagnostic,
    Severity::{self, Error, Info, Warning},
};

pub struct DiagnosticsRegistry(HashMap<&'static str, DiagnosticEntry>);

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
    fn new(map: HashMap<&'static str, DiagnosticEntry>) -> Self {
        DiagnosticsRegistry(map)
    }
}

impl DiagnosticAssessor for DiagnosticsRegistry {
    /// Assesses the diagnostic based on the current registered map. If no entry is found, the
    /// default diagnostic is returned
    fn assess(&self, d: &Diagnostic) -> Severity {
        self.0.get(d.get_type()).map(|it| it.severity).unwrap_or_else(|| {
            log::warn!("Unrecognized error code {}. Using default severity", d.get_type());
            Severity::default()
        })
    }
}

use lazy_static::lazy_static;
lazy_static! {
    static ref DIAGNOSTICS: HashMap<&'static str, DiagnosticEntry> = add_diagnostic!(
    E001,
    Error,
    include_str!("./error_codes/E001.md"), //General Error
    E002,
    Error,
    include_str!("./error_codes/E002.md"), //General IO Error
    E003,
    Error,
    include_str!("./error_codes/E003.md"), //Parameter Error
    E004,
    Error,
    include_str!("./error_codes/E004.md"), //Duplicate Symbol
    E005,
    Error,
    include_str!("./error_codes/E005.md"), //Generic LLVM Error
    E006,
    Error,
    include_str!("./error_codes/E006.md"), //Missing Token
    E007,
    Error,
    include_str!("./error_codes/E007.md"), //Unexpected Token
    E008,
    Error,
    include_str!("./error_codes/E008.md"), //Invalid Range
    E009,
    Error,
    include_str!("./error_codes/E009.md"), //Mismatched Parantheses
    E010,
    Error,
    include_str!("./error_codes/E010.md"), //Invalid Time Literal
    E011,
    Error,
    include_str!("./error_codes/E011.md"), //Invalid Number
    E012,
    Error,
    include_str!("./error_codes/E012.md"), //Missing Case Condition
    E013,
    Warning,
    include_str!("./error_codes/E013.md"), //Keywords shoud contain underscores
    E014,
    Warning,
    include_str!("./error_codes/E014.md"), //Wrong parantheses type
    E015,
    Warning,
    include_str!("./error_codes/E015.md"), //Pointer is not standard
    E016,
    Warning,
    include_str!("./error_codes/E016.md"), //Return types cannot have a default value
    E017,
    Error,
    include_str!("./error_codes/E017.md"), //Classes cannot contain implementations
    E018,
    Error,
    include_str!("./error_codes/E018.md"), //Duplicate Label
    E019,
    Error,
    include_str!("./error_codes/E019.md"), //Classes cannot contain IN_OUT variables
    E020,
    Error,
    include_str!("./error_codes/E020.md"), //Classes cannot contain a return type
    E021,
    Error,
    include_str!("./error_codes/E021.md"), //POUs cannot be extended
    E022,
    Warning,
    include_str!("./error_codes/E022.md"), //Missing action container
    E023,
    Warning,
    include_str!("./error_codes/E023.md"), //Statement with no effect
    E024,
    Warning,
    include_str!("./error_codes/E024.md"), //Invalid pragma location
    E025,
    Error,
    include_str!("./error_codes/E025.md"),
    E026,
    Error,
    include_str!("./error_codes/E026.md"),
    E027,
    Error,
    include_str!("./error_codes/E027.md"),
    E028,
    Error,
    include_str!("./error_codes/E028.md"),
    E029,
    Error,
    include_str!("./error_codes/E029.md"),
    E030,
    Error,
    include_str!("./error_codes/E030.md"),
    E031,
    Error,
    include_str!("./error_codes/E031.md"),
    E032,
    Error,
    include_str!("./error_codes/E032.md"),
    E033,
    Error,
    include_str!("./error_codes/E033.md"),
    E034,
    Error,
    include_str!("./error_codes/E034.md"),
    E035,
    Error,
    include_str!("./error_codes/E035.md"),
    E036,
    Error,
    include_str!("./error_codes/E036.md"),
    E037,
    Error,
    include_str!("./error_codes/E037.md"),
    E038,
    Error,
    include_str!("./error_codes/E038.md"), //Missing type
    E039,
    Warning,
    include_str!("./error_codes/E039.md"),
    E040,
    Error,
    include_str!("./error_codes/E040.md"),
    E041,
    Error,
    include_str!("./error_codes/E041.md"),
    E042,
    Warning,
    include_str!("./error_codes/E042.md"), //Assignment to reference
    E043,
    Error,
    include_str!("./error_codes/E043.md"),
    E044,
    Error,
    include_str!("./error_codes/E044.md"),
    E045,
    Error,
    include_str!("./error_codes/E045.md"),
    E046,
    Error,
    include_str!("./error_codes/E046.md"),
    E047,
    Warning,
    include_str!("./error_codes/E047.md"), //VLAs are always by reference
    E048,
    Error,
    include_str!("./error_codes/E048.md"),
    E049,
    Error,
    include_str!("./error_codes/E049.md"),
    E050,
    Error,
    include_str!("./error_codes/E050.md"),
    E051,
    Error,
    include_str!("./error_codes/E051.md"),
    E052,
    Error,
    include_str!("./error_codes/E052.md"),
    E053,
    Error,
    include_str!("./error_codes/E053.md"),
    E054,
    Error,
    include_str!("./error_codes/E054.md"),
    E055,
    Error,
    include_str!("./error_codes/E055.md"),
    E056,
    Error,
    include_str!("./error_codes/E056.md"),
    E057,
    Error,
    include_str!("./error_codes/E057.md"),
    E058,
    Error,
    include_str!("./error_codes/E058.md"),
    E059,
    Error,
    include_str!("./error_codes/E059.md"),
    E060,
    Info,
    include_str!("./error_codes/E060.md"), //Variable direct access with %
    E061,
    Error,
    include_str!("./error_codes/E061.md"),
    E062,
    Error,
    include_str!("./error_codes/E062.md"),
    E063,
    Error,
    include_str!("./error_codes/E063.md"),
    E064,
    Error,
    include_str!("./error_codes/E064.md"),
    E065,
    Error,
    include_str!("./error_codes/E065.md"),
    E066,
    Error,
    include_str!("./error_codes/E066.md"),
    E067,
    Info,
    include_str!("./error_codes/E067.md"), //Implicit typecast
    E068,
    Error,
    include_str!("./error_codes/E068.md"),
    E069,
    Error,
    include_str!("./error_codes/E069.md"),
    E070,
    Error,
    include_str!("./error_codes/E070.md"),
    E071,
    Error,
    include_str!("./error_codes/E071.md"),
    E072,
    Error,
    include_str!("./error_codes/E072.md"),
    E073,
    Error,
    include_str!("./error_codes/E073.md"),
    E074,
    Error,
    include_str!("./error_codes/E074.md"),
    E075,
    Error,
    include_str!("./error_codes/E075.md"),
    E076,
    Error,
    include_str!("./error_codes/E076.md"),
    E077,
    Error,
    include_str!("./error_codes/E077.md"),
    E078,
    Error,
    include_str!("./error_codes/E078.md"),
    E079,
    Error,
    include_str!("./error_codes/E079.md"),
    E080,
    Error,
    include_str!("./error_codes/E080.md"),
    E081,
    Error,
    include_str!("./error_codes/E081.md"),
    E082,
    Error,
    include_str!("./error_codes/E082.md"),
    E083,
    Error,
    include_str!("./error_codes/E083.md"),
    E084,
    Error,
    include_str!("./error_codes/E084.md"),
    E085,
    Error,
    include_str!("./error_codes/E085.md"),
    E086,
    Error,
    include_str!("./error_codes/E086.md"),
    E087,
    Error,
    include_str!("./error_codes/E087.md"),
    E088,
    Error,
    include_str!("./error_codes/E088.md"),
    E089,
    Error,
    include_str!("./error_codes/E089.md"),
    E090,
    Warning,
    include_str!("./error_codes/E090.md"), //Incompatible reference Assignment
);
}
