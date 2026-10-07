use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use serde::{Deserialize, Serialize};

pub mod source_location;
/// Represents the type of source a SourceContainer holds
#[derive(Clone, Copy, Debug)]
pub enum SourceType {
    /// A normal text file, the content of the file could be parsed
    Text,

    /// An xml file, probably cfc
    Xml,

    /// Unknown type, probably a binary
    Unknown,
}

/// SourceContainers offer source-code to be compiled via the load_source function.
/// Furthermore it offers a location-String used when reporting diagnostics.
pub trait SourceContainer: Sync + Send {
    /// loads and returns the SourceEntry that contains the SourceCode and the path it was loaded from
    fn load_source(&self, encoding: Option<&'static Encoding>) -> Result<SourceCode, String>;
    /// returns the location of this source-container. Used when reporting diagnostics.
    fn get_location(&self) -> Option<&Path>;

    /// Returns the `SourceType` for the current container,
    /// by default everything is Text unless it has the extension for a known binary/object
    fn get_type(&self) -> SourceType {
        if let Some(ext) = self.get_location().and_then(|it| it.extension()) {
            match ext.to_str() {
                Some("o") | Some("so") | Some("exe") => SourceType::Unknown,
                //XXX: file ending vs first line? (<?xml ...)
                Some("cfc") | Some("fbd") | Some("xml") => SourceType::Xml,
                _ => SourceType::Text,
            }
        } else {
            SourceType::Text
        }
    }

    /// Returns a staticly available location for this source
    fn get_location_str(&self) -> &'static str {
        let s = self
            .get_location()
            .map(|it| it.to_string_lossy())
            .map(|it| it.to_string())
            .unwrap_or_else(|| "<internal>".into());
        Box::leak(s.into_boxed_str())
    }
}

/// The SourceCode unit is the smallest unit of compilation that can be passed to the compiler
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceCode {
    /// the source code to be compiled
    pub source: String,
    /// the location this code was loaded from
    pub path: Option<PathBuf>,
}

/// tests can provide a SourceCode directly
impl SourceContainer for SourceCode {
    fn load_source(&self, _: Option<&'static Encoding>) -> Result<SourceCode, String> {
        Ok(self.clone())
    }

    fn get_location(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

pub type BuildDescriptionSource = SourceCode;

///Extension trait to create sources with names from strs, used in tests
pub trait SourceCodeFactory {
    fn create_source(self, path: impl Into<PathBuf>) -> SourceCode;
}

impl SourceCodeFactory for &str {
    fn create_source(self, path: impl Into<PathBuf>) -> SourceCode {
        SourceCode::new(self, path)
    }
}

impl<T: AsRef<Path> + Sync + Send> SourceContainer for T {
    fn load_source(&self, encoding: Option<&'static Encoding>) -> Result<SourceCode, String> {
        let source_type = self.get_type();
        if matches!(source_type, SourceType::Text | SourceType::Xml) {
            let mut file = File::open(self).map_err(|err| err.to_string())?;
            let source = create_source_code(&mut file, encoding)?;

            Ok(SourceCode { source, path: Some(self.as_ref().to_owned()) })
        } else {
            Err(format!("{} is not a source file", &self.as_ref().to_string_lossy()))
        }
    }

    fn get_location(&self) -> Option<&Path> {
        Some(self.as_ref())
    }
}

pub fn create_source_code<T: Read>(
    reader: &mut T,
    encoding: Option<&'static Encoding>,
) -> Result<String, String> {
    let mut buffer = String::new();
    let mut decoder = DecodeReaderBytesBuilder::new().encoding(encoding).build(reader);
    decoder.read_to_string(&mut buffer).map_err(|err| format!("{err}"))?;
    Ok(buffer)
}

impl From<&str> for SourceCode {
    fn from(src: &str) -> Self {
        SourceCode { source: src.into(), path: Some("<internal>".into()) }
    }
}

impl From<String> for SourceCode {
    fn from(source: String) -> Self {
        SourceCode { source, path: Some("<internal>".into()) }
    }
}

impl SourceCode {
    pub fn new(source: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        SourceCode { source: source.into(), path: Some(path.into()) }
    }

    pub fn with_path(mut self, name: impl Into<PathBuf>) -> Self {
        self.path = Some(name.into());
        self
    }
}

pub trait Compilable {
    type T: SourceContainer;
    fn containers(self) -> Vec<Self::T>;
}

impl Compilable for &str {
    type T = SourceCode;
    fn containers(self) -> Vec<Self::T> {
        let code = Self::T::from(self);
        vec![code]
    }
}

impl Compilable for String {
    type T = SourceCode;
    fn containers(self) -> Vec<Self::T> {
        let code = self.into();
        vec![code]
    }
}

impl<S: SourceContainer> Compilable for Vec<S> {
    type T = S;
    fn containers(self) -> Vec<Self::T> {
        self
    }
}

impl Compilable for SourceCode {
    type T = Self;

    fn containers(self) -> Vec<Self::T> {
        vec![self]
    }
}

#[cfg(test)]
mod tests {
    use crate::create_source_code;

    #[test]
    fn windows_encoded_file_content_read() {
        let expected = r"PROGRAM ä
(* Cöment *)
END_PROGRAM
";
        let mut source = &b"\x50\x52\x4f\x47\x52\x41\x4d\x20\xe4\x0a\x28\x2a\x20\x43\xf6\x6d\x65\x6e\x74\x20\x2a\x29\x0a\x45\x4e\x44\x5f\x50\x52\x4f\x47\x52\x41\x4d\x0a"[..];
        // let read = std::io::Read()
        let source = create_source_code(&mut source, Some(encoding_rs::WINDOWS_1252)).unwrap();

        assert_eq!(expected, &source);
    }

    #[test]
    fn utf_16_encoded_file_content_read() {
        let expected = r"PROGRAM ä
(* Cömment *)
END_PROGRAM
";

        let mut source = &b"\xff\xfe\x50\x00\x52\x00\x4f\x00\x47\x00\x52\x00\x41\x00\x4d\x00\x20\x00\xe4\x00\x0a\x00\x28\x00\x2a\x00\x20\x00\x43\x00\xf6\x00\x6d\x00\x6d\x00\x65\x00\x6e\x00\x74\x00\x20\x00\x2a\x00\x29\x00\x0a\x00\x45\x00\x4e\x00\x44\x00\x5f\x00\x50\x00\x52\x00\x4f\x00\x47\x00\x52\x00\x41\x00\x4d\x00\x0a\x00" [..];

        let source = create_source_code(&mut source, None).unwrap();
        assert_eq!(expected, &source);
    }

    #[test]
    fn utf_8_encoded_file_content_read() {
        let expected = r"PROGRAM ä
(* Cöment *)
END_PROGRAM
";

        let mut source = &b"\x50\x52\x4f\x47\x52\x41\x4d\x20\xc3\xa4\x0a\x28\x2a\x20\x43\xc3\xb6\x6d\x65\x6e\x74\x20\x2a\x29\x0a\x45\x4e\x44\x5f\x50\x52\x4f\x47\x52\x41\x4d\x0a" [..];
        let source = create_source_code(&mut source, None).unwrap();
        assert_eq!(expected, &source);
    }
}
