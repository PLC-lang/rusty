// This file is based on code from the Mun Programming Language
// https://github.com/mun-lang/mun

use plc_diagnostics::diagnostics::Diagnostic;
use which::which;

use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

use std::sync::{Arc, Mutex};

pub struct Linker {
    errors: Vec<LinkerError>,
    linker: Box<dyn LinkerInterface>,
}

#[derive(Clone, Default, Debug)]
pub enum LinkerType {
    #[default]
    Internal,
    External(String),
    Test(MockLinker),
}

impl From<Option<&str>> for LinkerType {
    fn from(value: Option<&str>) -> Self {
        match value {
            None => LinkerType::Internal,
            Some(linker) => LinkerType::External(linker.to_string()),
        }
    }
}

impl Linker {
    pub fn new(target: &str, linker: LinkerType) -> Result<Linker, LinkerError> {
        Ok(Linker {
            errors: Vec::default(),
            linker: match linker {
                // TODO: Linker for Windows is missing, see also:
                // https://github.com/PLC-lang/rusty/pull/702/files#r1052446296
                LinkerType::Internal => {
                    let [platform, target_os] = target.split('-').collect::<Vec<&str>>()[1..=2] else {
                        return Err(LinkerError::Target(target.into()));
                    };
                    match (platform, target_os) {
                        (_, "win32") | (_, "windows") | ("win32", _) | ("windows", _) => Box::new(CcLinker::new("clang")), //only clang from llvm is supported in windows
                        (_, "darwin") => Box::new(CcLinker::new("clang")),
                        _ => Box::new(LdLinker::new()),
                    }
                }
                LinkerType::External(linker) => Box::new(CcLinker::new(&linker)),
                LinkerType::Test(linker) => Box::new(linker),
            },
        })
    }

    /// Add an object file or static library to linker input
    pub fn add_obj<'a>(&'a mut self, path: &str) -> &'a mut Self {
        self.linker.add_obj(path);
        self
    }

    /// Add a library seaBoxh path to look in for libraries
    pub fn add_lib_path<'a>(&'a mut self, path: &str) -> &'a mut Self {
        self.linker.add_lib_path(path);
        self
    }

    /// Add a library path to look in for libraries
    pub fn add_lib<'a>(&'a mut self, path: &str) -> &'a mut Self {
        self.linker.add_lib(path);
        self
    }

    /// Add path to system root
    pub fn add_sysroot<'a>(&'a mut self, path: &str) -> &'a mut Self {
        self.linker.add_sysroot(path);
        self
    }

    /// Set the output file and run the linker to generate a shared object
    pub fn build_shared_obj(&mut self, path: PathBuf) -> Result<PathBuf, LinkerError> {
        if let Some(file) = self.get_str_from_path(&path) {
            self.linker.build_shared_object(file);
            self.linker.finalize()?;
        }
        Ok(path)
    }

    /// Set the output file and run the linker to generate an executable
    pub fn build_exectuable(&mut self, path: PathBuf) -> Result<PathBuf, LinkerError> {
        if let Some(file) = self.get_str_from_path(&path) {
            self.linker.build_exectuable(file);
            self.linker.finalize()?;
        }
        Ok(path)
    }

    /// Set the output file and run the linker to generate a relocatable object for further linking
    pub fn build_relocatable(&mut self, path: PathBuf) -> Result<PathBuf, LinkerError> {
        if let Some(file) = self.get_str_from_path(&path) {
            self.linker.build_relocatable(file);
            self.linker.finalize()?;
        }
        Ok(path)
    }

    /// Check if the path is valid, log an error if it wasn't
    fn get_str_from_path<'a>(&mut self, path: &'a Path) -> Option<&'a str> {
        let filepath = path.to_str();
        if filepath.is_none() {
            self.errors.push(LinkerError::Path(path.into()));
        }
        filepath
    }

    pub fn set_linker_script(&mut self, script: String) {
        self.linker.add_arg("-T".to_string());
        self.linker.add_arg(script);
    }
}

struct CcLinker {
    args: Vec<String>,
    linker: String,
}

impl CcLinker {
    fn new(linker: &str) -> CcLinker {
        CcLinker { args: Vec::default(), linker: linker.to_string() }
    }
}

impl LinkerInterface for CcLinker {
    fn add_arg(&mut self, value: String) {
        self.args.push(value)
    }

    fn get_build_command(&self) -> Result<String, LinkerError> {
        let linker_location = which(&self.linker)
            .map_err(|e| LinkerError::Link(format!("{e} for linker: {}", &self.linker)))?;
        Ok(format!("{} {}", linker_location.to_string_lossy(), self.args.join(" ")))
    }

    fn finalize(&mut self) -> Result<(), LinkerError> {
        let linker_location = which(&self.linker)
            .map_err(|e| LinkerError::Link(format!("{e} for linker: {}", &self.linker)))?;

        log::debug!("Linker command : {}", self.get_build_command()?);

        let status = Command::new(linker_location).args(&self.args).status()?;
        if status.success() {
            Ok(())
        } else {
            Err(LinkerError::Link("An error occured during linking".to_string()))
        }
    }
}

struct LdLinker {
    args: Vec<String>,
}

impl LdLinker {
    fn new() -> LdLinker {
        LdLinker { args: Vec::default() }
    }
}

impl LinkerInterface for LdLinker {
    fn add_arg(&mut self, value: String) {
        self.args.push(value)
    }

    fn get_build_command(&self) -> Result<String, LinkerError> {
        Ok(format!("ld.lld {}", self.args.join(" ")))
    }

    fn finalize(&mut self) -> Result<(), LinkerError> {
        log::debug!("Linker arguments : {}", self.get_build_command()?);
        lld_rs::link(lld_rs::LldFlavor::Elf, &self.args).ok().map_err(LinkerError::Link)
    }
}

#[derive(Clone, Debug)]
pub struct MockLinker {
    pub args: Arc<Mutex<Vec<String>>>,
}

impl LinkerInterface for MockLinker {
    fn add_arg(&mut self, value: String) {
        self.args.lock().unwrap().push(value)
    }

    fn get_build_command(&self) -> Result<String, LinkerError> {
        Ok(format!("ld.lld {}", self.args.lock()?.join(" ")))
    }

    fn finalize(&mut self) -> Result<(), LinkerError> {
        println!("Test Executing build command {}", self.get_build_command()?);
        Ok(())
    }
}

trait LinkerInterface {
    fn add_arg(&mut self, value: String);
    fn get_build_command(&self) -> Result<String, LinkerError>;
    fn finalize(&mut self) -> Result<(), LinkerError>;

    fn add_obj(&mut self, path: &str) {
        self.add_arg(path.into());
    }

    fn add_lib_path(&mut self, path: &str) {
        self.add_arg(format!("-L{path}"));
    }

    fn add_lib(&mut self, path: &str) {
        self.add_arg(format!("-l{path}"));
    }

    fn add_sysroot(&mut self, path: &str) {
        self.add_arg(format!("--sysroot={path}"));
    }

    fn build_shared_object(&mut self, path: &str) {
        self.add_arg("--shared".into());
        self.add_arg("-o".into());
        self.add_arg(path.into());
    }

    fn build_exectuable(&mut self, path: &str) {
        self.add_arg("-o".into());
        self.add_arg(path.into());
    }

    fn build_relocatable(&mut self, path: &str) {
        self.add_arg("-r".into()); // equivalent to --relocatable
        self.add_arg("-o".into());
        self.add_arg(path.into());
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LinkerError {
    /// Error emitted by the linker
    Link(String),

    /// Invalid target
    Target(String),

    /// Error in path conversion
    Path(PathBuf),
}

//TODO: This should be of type error, or we should be using anyhow/thiserror here
impl From<LinkerError> for Diagnostic {
    fn from(error: LinkerError) -> Self {
        match error {
            LinkerError::Link(e) => {
                Diagnostic::new(format!("An error occurred during linking: {e}")).with_error_code("E077")
            }
            LinkerError::Path(path) => {
                Diagnostic::new(format!("path contains invalid UTF-8 characters: {}", path.display()))
                    .with_error_code("E077")
            }
            LinkerError::Target(tgt) => {
                Diagnostic::new(format!("linker not available for target platform: {tgt}"))
                    .with_error_code("E077")
            }
        }
    }
}

impl<T: Error> From<T> for LinkerError {
    fn from(e: T) -> Self {
        LinkerError::Link(e.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::linker::{Linker, LinkerType};

    #[test]
    fn windows_target_triple_should_result_in_error() {
        for target in &[
            "x86_64-pc-windows-gnu",
            "x86_64-pc-win32-gnu",
            "x86_64-windows-gnu",
            "x86_64-win32-gnu",
            "aarch64-pc-windows-gnu",
            "aarch64-pc-win32-gnu",
            "aarch64-windows-gnu",
            "aarch64-win32-gnu",
            "i686-pc-windows-gnu",
            "i686-pc-win32-gnu",
            "i686-windows-gnu",
            "i686-win32-gnu",
        ] {
            assert!(Linker::new(target, LinkerType::Internal).is_err());
        }
    }

    #[test]
    fn non_windows_target_triple_should_result_in_ok() {
        for target in
            &["x86_64-linux-gnu", "x86_64-pc-linux-gnu", "x86_64-unknown-linux-gnu", "aarch64-apple-darwin"]
        {
            assert!(Linker::new(target, LinkerType::Internal).is_ok());
        }
    }
}
