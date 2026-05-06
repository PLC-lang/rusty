// This file is based on code from the Mun Programming Language
// https://github.com/mun-lang/mun

use plc_diagnostics::diagnostics::Diagnostic;
use which::which;

use std::{
    error::Error,
    path::{Path, PathBuf},
    process::{Command, Stdio},
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
                LinkerType::Internal => Box::new(resolve_internal_linker(target)?),
                LinkerType::External(linker) => Box::new(resolve_external_linker(target, &linker)?),
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
    pub fn build_executable(&mut self, path: PathBuf) -> Result<PathBuf, LinkerError> {
        if let Some(file) = self.get_str_from_path(&path) {
            self.linker.build_executable(file);
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

    /// Configure the linker backend used by compiler-driver linkers (`cc`, `clang`, ...).
    ///
    /// For example, passing `mold` results in `-fuse-ld=mold` being forwarded to the driver.
    pub fn set_fuse_ld(&mut self, linker: &str) {
        self.linker.set_fuse_ld(linker);
    }

    /// Disable C runtime startup files (e.g. crt1/Scrt1) when supported by the active linker.
    pub fn set_no_crt(&mut self) {
        self.linker.set_no_crt();
    }

    /// Disable implicit/default C libraries (e.g. libc) when supported by the active linker.
    pub fn set_no_libc(&mut self) {
        self.linker.set_no_libc();
    }

    /// Add one raw linker argument.
    ///
    /// Driver linkers receive this through `-Xlinker`, direct linkers receive it unchanged.
    pub fn add_linker_arg(&mut self, arg: &str) {
        self.linker.add_linker_arg(arg.to_string());
    }

    /// Add a driver-level flag.
    ///
    /// Driver linkers receive this directly (e.g. `cc -no-pie`).
    /// Direct linkers receive the equivalent low-level flag when known.
    pub fn add_driver_flag(&mut self, flag: &str) {
        self.linker.add_driver_flag(flag.to_string());
    }
}

/// Resolve the internal linker in this order: `cc` -> `clang` -> `ld.lld` -> `ld`.
fn resolve_internal_linker(target: &str) -> Result<CcLinker, LinkerError> {
    log::debug!("Resolving internal linker for target `{target}`");

    // Prefer compiler drivers for correct default platform setup.
    if which("cc").is_ok() {
        log::trace!("Candidate linker available: cc");
        return resolve_driver_linker("cc", target).or_else(|err| {
            log::debug!("cc rejected for target `{target}`: {err:?}");
            if which("clang").is_ok() {
                log::trace!("Falling back to clang");
                resolve_driver_linker("clang", target).or_else(|err| {
                    log::debug!("clang rejected for target `{target}`: {err:?}");
                    resolve_direct_linker_fallback()
                })
            } else {
                log::trace!("clang unavailable, falling back to direct linker resolution");
                resolve_direct_linker_fallback()
            }
        });
    }

    if which("clang").is_ok() {
        log::trace!("cc unavailable, trying clang");
        return resolve_driver_linker("clang", target).or_else(|err| {
            log::debug!("clang rejected for target `{target}`: {err:?}");
            resolve_direct_linker_fallback()
        });
    }

    log::trace!("No compiler-driver linker found, using direct linker fallback");
    resolve_direct_linker_fallback()
}

/// Resolve an explicitly selected linker.
///
/// Driver-like linkers (e.g. `cc`, `clang`, `gcc`) get driver semantics,
/// all others are treated as direct linkers.
fn resolve_external_linker(target: &str, linker: &str) -> Result<CcLinker, LinkerError> {
    log::debug!("Resolving external linker `{linker}` for target `{target}`");
    if is_driver_linker(linker) {
        log::trace!("External linker `{linker}` detected as compiler-driver linker");
        resolve_driver_linker(linker, target)
    } else {
        log::trace!("External linker `{linker}` treated as direct linker");
        Ok(CcLinker::new(linker))
    }
}

/// Resolve direct-linker fallback in this order: `ld.lld` then `ld`.
fn resolve_direct_linker_fallback() -> Result<CcLinker, LinkerError> {
    if which("ld.lld").is_ok() {
        log::trace!("Selected direct linker fallback: ld.lld");
        Ok(CcLinker::new("ld.lld"))
    } else if which("ld").is_ok() {
        log::trace!("Selected direct linker fallback: ld");
        Ok(CcLinker::new("ld"))
    } else {
        log::debug!("No direct linker available (tried ld.lld, ld)");
        Err(LinkerError::Link("No usable linker found. Tried in order: cc, clang, ld.lld, ld".to_string()))
    }
}

/// Heuristic that classifies compiler-driver-like linker commands.
fn is_driver_linker(linker: &str) -> bool {
    let name =
        Path::new(linker).file_name().and_then(|it| it.to_str()).unwrap_or(linker).to_ascii_lowercase();

    let is_driver = matches!(name.as_str(), "cc" | "c++" | "gcc" | "g++" | "clang" | "clang++")
        || name.contains("clang")
        || name.ends_with("gcc");

    log::trace!("is_driver_linker(`{linker}`) => {is_driver}");
    is_driver
}

/// Resolve a compiler-driver linker and compute default pre-arguments.
fn resolve_driver_linker(linker: &str, target: &str) -> Result<CcLinker, LinkerError> {
    if which(linker).is_err() {
        log::debug!("Requested driver linker `{linker}` is not available on PATH");
        return Err(LinkerError::Link(format!("Linker not found: {linker}")));
    }

    let mut pre_args = default_driver_pre_args();
    let cross_target = !target_matches_host(target);
    log::trace!("Driver linker `{linker}` cross-target={cross_target} target=`{target}`");

    if supports_target_flag(linker, target, &pre_args) {
        pre_args.push(format!("--target={target}"));
        log::trace!("Driver linker `{linker}` supports --target; added target flag");
    } else if cross_target {
        log::debug!("Driver linker `{linker}` does not support required --target for `{target}`");
        return Err(LinkerError::Link(format!(
            "{linker} does not support '--target={target}' for cross-target linking"
        )));
    }

    log::debug!("Selected driver linker `{linker}` with pre-args: {}", pre_args.join(" "));
    Ok(CcLinker::new_driver(linker, pre_args))
}

/// Default pre-arguments for compiler drivers.
///
/// - Prefer lld backend when available.
fn default_driver_pre_args() -> Vec<String> {
    let mut args = Vec::new();
    if which("ld.lld").is_ok() {
        args.push("-fuse-ld=lld".to_string());
        log::trace!("Driver default pre-args include -fuse-ld=lld");
    } else {
        log::trace!("Driver default pre-args use system default linker backend");
    }
    args
}

/// Probe whether a driver linker can actually compile **and link** for `target`.
///
/// The probe includes `pre_args` (e.g. `-fuse-ld=lld`) so that it tests the same
/// linker backend the driver will actually use. We compile+link a minimal C
/// program *without* `-nostdlib` so the probe also verifies that the target's
/// sysroot (crt files, libc, etc.) is available — matching what a real link will
/// need.
fn supports_target_flag(linker: &str, target: &str, pre_args: &[String]) -> bool {
    use std::time::{Duration, Instant};

    let null_output = if cfg!(windows) { "NUL" } else { "/dev/null" };
    let probe_src = "int main(){return 0;}";
    let timeout = Duration::from_secs(10);

    let supported = Command::new(linker)
        .arg(format!("--target={target}"))
        .args(pre_args)
        .args(["-x", "c", "-o", null_output, "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            // Write the source then drop stdin so the child sees EOF and doesn't hang.
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(probe_src.as_bytes());
            }

            let start = Instant::now();
            loop {
                match child.try_wait()? {
                    Some(status) => return Ok(status.success()),
                    None if start.elapsed() >= timeout => {
                        log::debug!(
                            "supports_target_flag: probe timed out after {}s, killing child",
                            timeout.as_secs()
                        );
                        let _ = child.kill();
                        let _ = child.wait();
                        return Ok(false);
                    }
                    None => std::thread::sleep(Duration::from_millis(50)),
                }
            }
        })
        .unwrap_or(false);

    log::trace!(
        "supports_target_flag(linker=`{linker}`, target=`{target}`, pre_args={pre_args:?}) => {supported}"
    );
    supported
}

/// Best-effort host/target compatibility check used for linker-driver decisions.
fn target_matches_host(target: &str) -> bool {
    let parts: Vec<_> = target.split('-').collect();
    if parts.is_empty() {
        log::trace!("target_matches_host(`{target}`) => false (empty target triple)");
        return false;
    }

    let host_arch = std::env::consts::ARCH;
    let host_os_aliases: &[&str] = match std::env::consts::OS {
        "linux" => &["linux"],
        "macos" => &["darwin", "macos"],
        "windows" => &["windows", "win32"],
        other => &[other],
    };

    let arch_matches = parts.contains(&host_arch);
    let os_matches = parts.iter().any(|part| host_os_aliases.contains(part));
    let matches = arch_matches && os_matches;
    log::trace!(
        "target_matches_host(`{target}`) => {matches} (host_arch={host_arch}, host_os_aliases={:?})",
        host_os_aliases
    );
    matches
}

struct CcLinker {
    args: Vec<String>,
    linker: String,
    pre_args: Vec<String>,
    driver_mode: bool,
    no_crt: bool,
    no_libc: bool,
}

impl CcLinker {
    fn new(linker: &str) -> CcLinker {
        CcLinker {
            args: Vec::default(),
            linker: linker.to_string(),
            pre_args: Vec::default(),
            driver_mode: false,
            no_crt: false,
            no_libc: false,
        }
    }

    fn new_driver(linker: &str, pre_args: Vec<String>) -> CcLinker {
        CcLinker {
            args: Vec::default(),
            linker: linker.to_string(),
            pre_args,
            driver_mode: true,
            no_crt: false,
            no_libc: false,
        }
    }

    fn command_args(&self) -> Vec<String> {
        self.pre_args.iter().chain(self.args.iter()).cloned().collect()
    }
}

impl LinkerInterface for CcLinker {
    fn add_arg(&mut self, value: String) {
        self.args.push(value)
    }

    fn set_fuse_ld(&mut self, linker: &str) {
        if self.driver_mode {
            log::debug!("Applying custom driver backend linker: -fuse-ld={linker}");
            self.pre_args.retain(|it| !it.starts_with("-fuse-ld="));
            self.pre_args.push(format!("-fuse-ld={linker}"));
        } else {
            log::trace!("Ignoring set_fuse_ld for direct linker `{}`", self.linker);
        }
    }

    fn set_no_crt(&mut self) {
        log::debug!("no_crt enabled for linker `{}`", self.linker);
        self.no_crt = true;
    }

    fn set_no_libc(&mut self) {
        log::debug!("no_libc enabled for linker `{}`", self.linker);
        self.no_libc = true;
    }

    fn add_linker_arg(&mut self, arg: String) {
        if self.driver_mode {
            log::trace!("Forwarding linker arg via driver -Xlinker: {arg}");
            self.add_arg("-Xlinker".to_string());
            self.add_arg(arg);
        } else {
            log::trace!("Forwarding linker arg directly: {arg}");
            self.add_arg(arg);
        }
    }

    fn add_driver_flag(&mut self, flag: String) {
        if self.driver_mode {
            log::trace!("Adding driver-level flag: {flag}");
            self.add_arg(flag);
        } else {
            // Translate known driver flags to their direct-linker equivalents.
            let direct_flag = match flag.as_str() {
                "-no-pie" | "-nopie" => "--no-pie",
                "-pie" => "--pie",
                _ => {
                    log::trace!("Unknown driver flag `{flag}` passed to direct linker unchanged");
                    self.add_arg(flag);
                    return;
                }
            };
            log::trace!("Translating driver flag `{flag}` to direct linker flag `{direct_flag}`");
            self.add_arg(direct_flag.to_string());
        }
    }

    fn build_executable(&mut self, path: &str) {
        if self.driver_mode {
            if self.no_crt {
                log::trace!("Executable link: adding -nostartfiles");
                self.add_arg("-nostartfiles".into());
            }
            if self.no_libc {
                log::trace!("Executable link: adding -nodefaultlibs");
                self.add_arg("-nodefaultlibs".into());
            }
        }
        self.add_arg("-o".into());
        self.add_arg(path.into());
    }

    fn get_build_command(&self) -> Result<String, LinkerError> {
        let linker_location = which(&self.linker)
            .map_err(|e| LinkerError::Link(format!("{e} for linker: {}", &self.linker)))?;
        Ok(format!("{} {}", linker_location.to_string_lossy(), self.command_args().join(" ")))
    }

    fn finalize(&mut self) -> Result<(), LinkerError> {
        let linker_location = which(&self.linker)
            .map_err(|e| LinkerError::Link(format!("{e} for linker: {}", &self.linker)))?;

        log::debug!("Linker command : {}", self.get_build_command()?);

        let status = Command::new(linker_location).args(self.command_args()).status()?;
        if status.success() {
            Ok(())
        } else {
            Err(LinkerError::Link("An error occured during linking".to_string()))
        }
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
    fn set_fuse_ld(&mut self, _linker: &str) {}
    fn set_no_crt(&mut self) {}
    fn set_no_libc(&mut self) {}
    fn add_linker_arg(&mut self, arg: String) {
        self.add_arg(arg);
    }

    fn add_driver_flag(&mut self, flag: String) {
        self.add_arg(flag);
    }

    fn add_obj(&mut self, path: &str) {
        self.add_arg(path.into());
    }

    fn add_lib_path(&mut self, path: &str) {
        self.add_arg(format!("-L{path}"));
    }

    fn add_lib(&mut self, path: &str) {
        if path.contains('/') || path.contains('\\') {
            log::trace!("Library argument `{path}` treated as direct file input");
            self.add_obj(path);
        } else {
            log::trace!("Library argument `{path}` treated as -l lookup");
            self.add_arg(format!("-l{path}"));
        }
    }

    fn add_sysroot(&mut self, path: &str) {
        self.add_arg(format!("--sysroot={path}"));
    }

    fn build_shared_object(&mut self, path: &str) {
        self.add_arg("--shared".into());
        self.add_arg("-o".into());
        self.add_arg(path.into());
    }

    fn build_executable(&mut self, path: &str) {
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
    use crate::linker::{CcLinker, Linker, LinkerInterface, LinkerType};

    #[test]
    fn windows_target_triple_should_result_in_ok() {
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
            assert!(Linker::new(target, LinkerType::Internal).is_ok());
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

    #[test]
    fn linker_arg_is_forwarded_via_xlinker_for_driver_linkers() {
        let mut linker = CcLinker::new_driver("cc", Vec::new());
        linker.add_linker_arg("--no-undefined".to_string());
        assert_eq!(linker.command_args(), vec!["-Xlinker", "--no-undefined"]);
    }

    #[test]
    fn linker_arg_is_forwarded_raw_for_direct_linkers() {
        let mut linker = CcLinker::new("ld.lld");
        linker.add_linker_arg("--no-undefined".to_string());
        assert_eq!(linker.command_args(), vec!["--no-undefined"]);
    }

    #[test]
    fn driver_flag_is_added_directly_for_driver_linkers() {
        let mut linker = CcLinker::new_driver("cc", Vec::new());
        linker.add_driver_flag("-no-pie".to_string());
        assert_eq!(linker.command_args(), vec!["-no-pie"]);
    }

    #[test]
    fn driver_flag_is_translated_for_direct_linkers() {
        let mut linker = CcLinker::new("ld.lld");
        linker.add_driver_flag("-no-pie".to_string());
        assert_eq!(linker.command_args(), vec!["--no-pie"]);
    }

    #[test]
    fn add_lib_with_full_path_is_treated_as_direct_input() {
        let mut linker = CcLinker::new("ld.lld");
        linker.add_lib("/tmp/libfoo.so.1");
        assert_eq!(linker.command_args(), vec!["/tmp/libfoo.so.1"]);
    }

    #[test]
    fn add_lib_with_exact_name_is_preserved() {
        let mut linker = CcLinker::new("ld.lld");
        linker.add_lib(":libfoo.so.1");
        assert_eq!(linker.command_args(), vec!["-l:libfoo.so.1"]);
    }
}
