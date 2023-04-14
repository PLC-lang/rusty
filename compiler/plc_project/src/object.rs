use std::path::{Path, PathBuf};

use plc::Target;

#[derive(Debug, Clone)]
pub struct Object {
    path: PathBuf,
    target: Target,
    format: ObjectFormat,
}

/// Representation of a binary file
#[derive(Debug, Clone, Copy)]
pub enum ObjectFormat {
    /// Archive file containing several object files, used for static linking
    Archive,
    /// Shared object or DLL, used to link to other objects
    Shared,
    /// An executable file
    Executable,
    /// An LLVM Bitcode generated file (".bc")
    Bitcode,
    /// An LLVM IR generated file (".ll")
    IR,
    /// Default non specific representation, this is typically the ".o" file
    Object,
    /// Unknown type
    Unknown,
}

impl From<PathBuf> for Object {
    fn from(path: PathBuf) -> Self {
        let format = match path.extension().and_then(|it| it.to_str()) {
            Some("o") => ObjectFormat::Object,
            Some("bc") => ObjectFormat::Bitcode,
            Some("ir") => ObjectFormat::IR,
            Some("so") => ObjectFormat::Shared,
            Some("a") => ObjectFormat::Archive,
            Some(_) => ObjectFormat::Unknown,
            None => ObjectFormat::Executable,
        };
        Object { path, target: Target::System, format }
    }
}

impl Object {
    pub fn with_target(mut self, target: &Target) -> Self {
        self.target = target.clone();
        self
    }
    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn get_target(&self) -> &Target {
        &self.target
    }
}
