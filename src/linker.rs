// This file is based on code from the Mun Programming Language
// https://github.com/mun-lang/mun

use which::which;

use crate::diagnostics::Diagnostic;
use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

pub struct Linker {
    errors: Vec<LinkerError>,
    linker: Box<dyn LinkerInterface>,
}

trait LinkerInterface {
    fn get_platform(&self) -> String;
    fn add_obj(&mut self, path: &str);
    fn add_lib(&mut self, path: &str);
    fn add_lib_path(&mut self, path: &str);
    fn add_sysroot(&mut self, path: &str);
    fn build_shared_object(&mut self, path: &str);
    fn build_exectuable(&mut self, path: &str);
    fn build_relocatable(&mut self, path: &str);
    fn finalize(&mut self) -> Result<(), LinkerError>;
}

impl Linker {
    pub fn new(target: &str, linker: Option<&str>) -> Result<Linker, LinkerError> {
        let target_os = target.split('-').collect::<Vec<&str>>()[2];
        let linker: Box<dyn LinkerInterface> = if let Some(linker) = linker {
            Box::new(CcLinker::new(linker))
        } else {
            match target_os {
                "linux" | "gnu" => Ok(Box::new(LdLinker::new())),
                // "win32" | "windows" => Ok(Box::new(CcLinker::new("clang".to_string()))),
                _ => Err(LinkerError::Target(target_os.into())),
            }?
        };
        Ok(Linker { errors: Vec::default(), linker })
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
    pub fn build_shared_obj(&mut self, path: &Path) -> Result<(), LinkerError> {
        if let Some(file) = self.get_str_from_path(path) {
            self.linker.build_shared_object(file);
            self.linker.finalize()?;
        }
        Ok(())
    }

    /// Set the output file and run the linker to generate an executable
    pub fn build_exectuable(&mut self, path: &Path) -> Result<(), LinkerError> {
        if let Some(file) = self.get_str_from_path(path) {
            self.linker.build_exectuable(file);
            self.linker.finalize()?;
        }
        Ok(())
    }

    /// Set the output file and run the linker to generate a relocatable object for further linking
    pub fn build_relocatable(&mut self, path: &Path) -> Result<(), LinkerError> {
        if let Some(file) = self.get_str_from_path(path) {
            self.linker.build_relocatable(file);
            self.linker.finalize()?;
        }
        Ok(())
    }

    /// Check if the path is valid, log an error if it wasn't
    fn get_str_from_path<'a>(&mut self, path: &'a Path) -> Option<&'a str> {
        let filepath = path.to_str();
        if filepath.is_none() {
            self.errors.push(LinkerError::Path(path.into()));
        }
        filepath
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
    fn get_platform(&self) -> String {
        "Linux".into()
    }

    fn add_obj(&mut self, path: &str) {
        self.args.push(path.into());
    }

    fn add_lib_path(&mut self, path: &str) {
        self.args.push(format!("-L{}", path));
    }

    fn add_lib(&mut self, path: &str) {
        self.args.push(format!("-l{}", path));
    }

    fn add_sysroot(&mut self, path: &str) {
        self.args.push(format!("--sysroot={}", path));
    }

    fn build_shared_object(&mut self, path: &str) {
        self.args.push("--shared".into());
        self.args.push("-o".into());
        self.args.push(path.into());
    }

    fn build_exectuable(&mut self, path: &str) {
        self.args.push("-o".into());
        self.args.push(path.into());
    }

    fn build_relocatable(&mut self, path: &str) {
        self.args.push("-relocatable".into());
        self.args.push("-o".into());
        self.args.push(path.into());
    }

    fn finalize(&mut self) -> Result<(), LinkerError> {
        let linker_location = which(&self.linker)
            .map_err(|e| LinkerError::Link(format!("{} for linker: {}", e, &self.linker)))?;

        #[cfg(feature = "debug")]
        println!("Linker command : {} {}", linker_location.to_string_lossy(), self.args.join(" "));

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
    fn get_platform(&self) -> String {
        "Linux".into()
    }

    fn add_obj(&mut self, path: &str) {
        self.args.push(path.into());
    }

    fn add_lib_path(&mut self, path: &str) {
        self.args.push(format!("-L{}", path));
    }

    fn add_lib(&mut self, path: &str) {
        self.args.push(format!("-l{}", path));
    }

    fn add_sysroot(&mut self, path: &str) {
        self.args.push(format!("--sysroot={}", path));
    }

    fn build_shared_object(&mut self, path: &str) {
        self.args.push("--shared".into());
        self.args.push("-o".into());
        self.args.push(path.into());
    }

    fn build_exectuable(&mut self, path: &str) {
        self.args.push("-o".into());
        self.args.push(path.into());
    }

    fn build_relocatable(&mut self, path: &str) {
        self.args.push("-relocatable".into());
        self.args.push("-o".into());
        self.args.push(path.into());
    }

    fn finalize(&mut self) -> Result<(), LinkerError> {
        #[cfg(feature = "debug")]
        println!("Linker arguments : {}", self.args.join(" "));

        lld_rs::link(lld_rs::LldFlavor::Elf, &self.args).ok().map_err(LinkerError::Link)
    }
}

/* TODO: Implement Windows linker

struct MsvcLinker {
    args: Vec<String>,
}

impl LinkerInterface for MsvcLinker {
    fn get_platform(&self) -> String {

    }

    fn add_obj(&mut self, path: &str) {

    }

    fn add_lib_path(&mut self, path: &str) {

    }

    fn build_shared_object(&mut self, path: &Path) {

    }

    fn build_exectuable(&mut self, path: &Path) {

    }

    fn finalize(&mut self) -> Result<(), LinkerError>{

    }
}*/

#[derive(Debug, PartialEq, Eq)]
pub enum LinkerError {
    /// Error emitted by the linker
    Link(String),

    /// Invalid target
    Target(String),

    /// Error in path conversion
    Path(PathBuf),
}

impl From<LinkerError> for Diagnostic {
    fn from(error: LinkerError) -> Self {
        match error {
            LinkerError::Link(e) => Diagnostic::link_error(&e),
            LinkerError::Path(path) => {
                Diagnostic::link_error(&format!("path contains invalid UTF-8 characters: {}", path.display()))
            }
            LinkerError::Target(tgt) => {
                Diagnostic::link_error(&format!("linker not available for target platform: {}", tgt))
            }
        }
    }
}

impl<T: Error> From<T> for LinkerError {
    fn from(e: T) -> Self {
        LinkerError::Link(e.to_string())
    }
}

#[test]
fn creation_test() {
    let linker = Linker::new("x86_64-pc-linux-gnu", None).unwrap();
    assert_eq!(linker.linker.get_platform(), "Linux");

    if let Err(tgt) = Linker::new("x86_64-pc-redox-abc", None) {
        assert_eq!(tgt, LinkerError::Target("redox".into()));
    } else {
        panic!("Linker target should have returned an error!");
    }
}
