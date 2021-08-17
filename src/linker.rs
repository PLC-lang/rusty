// This file is based on code from the Mun Programming Language
// https://github.com/mun-lang/mun

use crate::compile_error::CompileError;
use std::fmt;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinkerError {
    /// Error emitted by the linker
    LinkError(String),

    /// Error in path conversion
    PathError(PathBuf),
}

impl LinkerError {
    fn to_compile_error(&self) -> CompileError {
        CompileError::LinkerError {
            reason: format!("{}", self),
        }
    }
}

impl fmt::Display for LinkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            LinkerError::LinkError(e) => write!(f, "{}", e),
            LinkerError::PathError(path) => write!(
                f,
                "path contains invalid UTF-8 characters: {}",
                path.display()
            ),
        }
    }
}

pub fn create_with_target(target: &str) -> Result<Box<dyn Linker>, CompileError> {
    let target_os = target.split('-').collect::<Vec<&str>>()[2];
    match target_os {
        "linux" => Ok(Box::new(LdLinker::new())),
        "win32" => Ok(Box::new(MsvcLinker::new())),
        _ => Err(CompileError::LinkerError {
            reason: format!("invalid target platform: {}", target_os),
        }),
    }
}

pub trait Linker {
    fn link_with_libc(&mut self);
    fn add_object(&mut self, path: &Path) -> Result<(), CompileError>;
    fn build_shared_object(&mut self, path: &Path) -> Result<(), CompileError>;
    fn build_exectuable(&mut self, path: &Path) -> Result<(), CompileError>;
    fn finalize(&mut self) -> Result<(), CompileError>;
}

struct LdLinker {
    args: Vec<String>,
}

impl LdLinker {
    fn new() -> Self {
        LdLinker {
            args: Vec::default(),
        }
    }
}

impl Linker for LdLinker {
    fn link_with_libc(&mut self) {
        self.args.push("-L.".into());
        self.args.push("-lc".into());
    }

    fn add_object(&mut self, path: &Path) -> Result<(), CompileError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| LinkerError::PathError(path.to_owned()).to_compile_error())?
            .to_owned();
        self.args.push(path_str);
        Ok(())
    }

    fn build_shared_object(&mut self, path: &Path) -> Result<(), CompileError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| LinkerError::PathError(path.to_owned()).to_compile_error())?;

        // Link as dynamic library
        self.args.push("--shared".to_owned());

        // Specify output path
        self.args.push("-o".to_owned());
        self.args.push(path_str.to_owned());

        self.finalize()
    }

    fn build_exectuable(&mut self, path: &Path) -> Result<(), CompileError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| LinkerError::PathError(path.to_owned()).to_compile_error())?;

        // Specify output path
        self.args.push("-o".to_owned());
        self.args.push(path_str.to_owned());

        self.finalize()
    }

    fn finalize(&mut self) -> Result<(), CompileError> {
        mun_lld::link(mun_lld::LldFlavor::Elf, &self.args)
            .ok()
            .map_err(LinkerError::LinkError)
            .map_err(|error| error.to_compile_error())
    }
}
struct MsvcLinker {
    args: Vec<String>,
}

impl MsvcLinker {
    fn new() -> Self {
        MsvcLinker {
            args: Vec::default(),
        }
    }
}

impl Linker for MsvcLinker {
    fn link_with_libc(&mut self) {
        // Not sure how this is called?
        //self.args.push("libc.lib".into());
    }

    fn add_object(&mut self, path: &Path) -> Result<(), CompileError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| LinkerError::PathError(path.to_owned()).to_compile_error())?
            .to_owned();
        self.args.push(path_str);
        Ok(())
    }

    fn build_shared_object(&mut self, path: &Path) -> Result<(), CompileError> {
        let dll_path_str = path
            .to_str()
            .ok_or_else(|| LinkerError::PathError(path.to_owned()).to_compile_error())?;

        let dll_lib_path_str = path
            .to_str()
            .ok_or_else(|| LinkerError::PathError(path.to_owned()).to_compile_error())?;

        self.args.push("/DLL".to_owned());
        self.args.push("/NOENTRY".to_owned());
        self.args.push(format!("/IMPLIB:{}", dll_lib_path_str));
        self.args.push(format!("/OUT:{}", dll_path_str));

        self.finalize()
    }

    fn build_exectuable(&mut self, path: &Path) -> Result<(), CompileError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| LinkerError::PathError(path.to_owned()).to_compile_error())?;

        // Specify output path
        self.args.push(format!("/OUT:{}", path_str.to_owned()));

        self.finalize()
    }

    fn finalize(&mut self) -> Result<(), CompileError> {
        mun_lld::link(mun_lld::LldFlavor::Coff, &self.args)
            .ok()
            .map_err(LinkerError::LinkError)
            .map_err(|error| error.to_compile_error())
    }
}
