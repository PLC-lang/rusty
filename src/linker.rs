// This file is based on code from the Mun Programming Language
// https://github.com/mun-lang/mun

use std::path::{Path, PathBuf};

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
    fn finalize(&mut self) -> Result<(), LinkerError>;
}

impl Linker {
    pub fn new(target: &str) -> Result<Linker, LinkerError> {
        let target_os = target.split('-').collect::<Vec<&str>>()[2];
        let linker = match target_os {
            "linux" => Ok(Box::new(LdLinker::new())),
            //"win32" | "windows" => Ok(Box::new(MsvcLinker::new())),
            _ => Err(LinkerError::Target(target_os.into())),
        }?;
        Ok(Linker {
            errors: Vec::default(),
            linker,
        })
    }

    /// Add an object file or static library to linker input
    pub fn add_obj<'a>(&'a mut self, file: &Path) -> &'a mut Self {
        if let Some(file) = self.get_str_from_path(file) {
            self.linker.add_obj(file);
        }
        self
    }

    /// Add a library seaBoxh path to look in for libraries
    pub fn add_lib_path<'a>(&'a mut self, path: &str) -> &'a mut Self {
        self.linker.add_lib_path(path);
        self
    }

    /// Add a library seaBoxh path to look in for libraries
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

    /// Check if the path is valid, log an error if it wasn't
    fn get_str_from_path<'a>(&mut self, path: &'a Path) -> Option<&'a str> {
        let filepath = path.to_str();
        if filepath.is_none() {
            self.errors.push(LinkerError::Path(path.into()));
        }
        filepath
    }
}

struct LdLinker {
    args: Vec<String>,
}

impl LdLinker {
    fn new() -> LdLinker {
        LdLinker {
            args: Vec::default(),
        }
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

    fn finalize(&mut self) -> Result<(), LinkerError> {
        println!("{:?}", self.args);
        lld_rs::link(lld_rs::LldFlavor::Elf, &self.args)
            .ok()
            .map_err(LinkerError::Link)
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

#[derive(Debug, PartialEq)]
pub enum LinkerError {
    /// Error emitted by the linker
    Link(String),

    /// Invalid target
    Target(String),

    /// Error in path conversion
    Path(PathBuf),
}

impl From<LinkerError> for String {
    fn from(error: LinkerError) -> Self {
        match error {
            LinkerError::Link(e) => e,
            LinkerError::Path(path) => {
                format!("path contains invalid UTF-8 characters: {}", path.display())
            }
            LinkerError::Target(tgt) => {
                format!("linker not available for target platform: {}", tgt)
            }
        }
    }
}

#[test]
fn creation_test() {
    let linker = Linker::new("x86_64-pc-linux-gnu").unwrap();
    assert_eq!(linker.linker.get_platform(), "Linux");

    if let Err(tgt) = Linker::new("x86_64-pc-redox-abc") {
        assert_eq!(tgt, LinkerError::Target("redox".into()));
    } else {
        panic!("Linker target should have returned an error!");
    }
}

#[test]
fn linker_error_test() {
    let msg = "error message";
    let link_err = LinkerError::Link(msg.into());
    assert_eq!(String::from(link_err), msg.to_string());

    let path = "/abc/def";
    let link_err = LinkerError::Path(path.into());
    assert_eq!(
        String::from(link_err),
        format!("path contains invalid UTF-8 characters: {}", path)
    );

    let target = "redox";
    let link_err = LinkerError::Target(target.into());
    assert_eq!(
        String::from(link_err),
        format!("linker not available for target platform: {}", target)
    );
}
