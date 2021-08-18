// This file is based on code from the Mun Programming Language
// https://github.com/mun-lang/mun

use std::path::{Path, PathBuf};

pub enum LinkerError {
    /// Error emitted by the linker
    LinkError(String),

    /// Invalid target
    TargetError(String),

    /// Error in path conversion
    PathError(PathBuf),
}

impl From<LinkerError> for String {
    fn from(error: LinkerError) -> Self {
        match error {
            LinkerError::LinkError(e) => format!("{}", e),
            LinkerError::PathError(path) => format!(
                "path contains invalid UTF-8 characters: {}",
                path.display()
            ),
            LinkerError::TargetError(tgt) => format!(
                "invalid target: {}", tgt
            )
        }
    }
}

#[test]
fn linker_error_test() {
    let msg = "error message";
    let link_err = LinkerError::LinkError(msg.into());
    assert_eq!(
        String::from(link_err),
        msg.into()
    );

    let path = "/abc/def";
    let link_err = LinkerError::PathError(path.into());
    assert_eq!(
        String::from(link_err),
        format!("path contains invalid UTF-8 characters: {}", path)
    );
}

pub fn create_with_target(target: &str) -> Result<Box<dyn Linker>, LinkerError> {
    let target_os = target.split('-').collect::<Vec<&str>>()[2];
    match target_os {
        "linux" => Ok(Box::new(LdLinker::new())),
        "win32" => Ok(Box::new(MsvcLinker::new())),
        _ => Err(LinkerError::TargetError(
            format!("invalid target platform: {}", target_os)
        )),
    }
}

#[test]
fn creation_test() {
    let linker = create_with_target("x86_64-pc-linux-gnu");
    assert_eq!(linker.unwrap().get_platform(), "linux".to_string());

    let linker = create_with_target("x86_64-pc-win32-gnu");
    assert_eq!(linker.unwrap().get_platform(), "win32".to_string());

    let linker = create_with_target("x86_64-pc-foo-gnu");
    if let Ok(..) = linker {
        panic!("Expected create_with_target() to fail");
    }
}

pub trait Linker {
    fn link_with_libc(&mut self);
    fn get_platform(&self) -> String;
    fn add_object(&mut self, path: &Path);
    fn build_shared_object(&mut self, path: &Path);
    fn build_exectuable(&mut self, path: &Path);
    fn finalize(&mut self) -> Result<(), LinkerError>;
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
    fn get_platform(&self) -> String {
        "linux".into()
    }

    fn link_with_libc(&mut self) {
        self.args.push("-L.".into());
        self.args.push("-lc".into());
    }

    fn add_object(&mut self, path: &Path) {
        let path_str = path.to_str().unwrap();
        self.args.push(path_str.to_owned());

    }

    fn build_shared_object(&mut self, path: &Path) {
        let path_str = path.to_str().unwrap();

        // Link as dynamic library
        self.args.push("--shared".to_owned());

        // Specify output path
        self.args.push("-o".to_owned());
        self.args.push(path_str.to_owned());

    }

    fn build_exectuable(&mut self, path: &Path) {
        let path_str = path.to_str().unwrap();

        // Specify output path
        self.args.push("-o".to_owned());
        self.args.push(path_str.to_owned());


    }

    fn finalize(&mut self) -> Result<(), LinkerError> {
        mun_lld::link(mun_lld::LldFlavor::Elf, &self.args)
            .ok()
            .map_err(LinkerError::LinkError)
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
    fn get_platform(&self) -> String {
        "win32".into()
    }

    fn link_with_libc(&mut self) {
        // Not sure how this is called?
        //self.args.push("libc.lib".into());
    }

    fn add_object(&mut self, path: &Path) {
        let path_str = path.to_str().unwrap();
        self.args.push(path_str.to_owned());

    }

    fn build_shared_object(&mut self, path: &Path) {
        let dll_path_str = path.to_str().unwrap();
        let dll_lib_path_str = path.to_str().unwrap();

        self.args.push("/DLL".to_owned());
        self.args.push("/NOENTRY".to_owned());
        self.args.push(format!("/IMPLIB:{}", dll_lib_path_str));
        self.args.push(format!("/OUT:{}", dll_path_str));

    }

    fn build_exectuable(&mut self, path: &Path) {
        let path_str = path.to_str().unwrap();

        // Specify output path
        self.args.push(format!("/OUT:{}", path_str.to_owned()));
    }

    fn finalize(&mut self) -> Result<(), LinkerError> {
        mun_lld::link(mun_lld::LldFlavor::Coff, &self.args)
            .ok()
            .map_err(LinkerError::LinkError)
    }
}

// #[test]
// fn linux_linker_test() {
//     let mut linker = LdLinker::new();
//     linker.link_with_libc();
//     linker.add_object(Path::new("test.o")).unwrap();
//     // linker
//     //     .build_shared_object(Path::new("test.so"))
//     //     .unwrap_or(());
//     assert_eq!(
//         linker.args,
//         vec!["-L.", "-lc", /* "test.o", "--shared", "-o", "test.so" */ ]
//     );

//     let mut linker = LdLinker::new();
//     linker.add_object(Path::new("test.o")).unwrap();
//     // linker.build_exectuable(Path::new("test")).unwrap_or(());
//     assert_eq!(linker.args, vec!["test.o", /* "-o", "test" */ ]);
// }

// #[test]
// fn windows_linker_test() {
//     let mut linker = MsvcLinker::new();
//     linker.link_with_libc();
//     linker.add_object(Path::new("test.obj")).unwrap();
//     // linker
//     //     .build_shared_object(Path::new("test.dll"))
//     //     .unwrap_or(());
//     assert_eq!(
//         linker.args,
//         vec![
//             "test.obj",
// /*            "/DLL",
//             "/NOENTRY",
//             "/IMPLIB:test.dll",
//             "/OUT:test.dll" */
//         ]
//     );

//     let mut linker = MsvcLinker::new();
//     linker.add_object(Path::new("test.obj")).unwrap();
//     //linker.build_exectuable(Path::new("test.exe")).unwrap_or(());
//     assert_eq!(linker.args, vec!["test.obj", /* "/OUT:test.exe" */]);
// }
