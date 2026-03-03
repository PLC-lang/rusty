use std::path::PathBuf;
use std::process::Command;
use std::sync::Once;

use plc::codegen::CodegenContext;
use plc_driver::runner::compile_with_includes;
use plc_source::{Compilable, SourceCode, SourceContainer};

use libloading::{Library, Symbol};
use plc::OptimizationLevel;
use plc::Target;
use tempfile::TempDir;

pub struct CompiledModule {
    _temp_dir: TempDir,
    library: Library,
}

impl CompiledModule {
    pub fn get_function<T>(&'_ self, name: &str) -> Symbol<'_, T> {
        unsafe { self.library.get(name.as_bytes()).expect("Function not found") }
    }

    fn get_optional<T>(&'_ self, name: &str) -> Option<Symbol<'_, T>> {
        unsafe { self.library.get(name.as_bytes()).ok() }
    }

    pub fn run<T, U>(&self, name: &str, params: &mut T) -> U {
        let func: Symbol<extern "C" fn(*mut T) -> U> = self.get_function(name);
        func(params)
    }

    pub fn run_no_param<U>(&self, name: &str) -> U {
        let func: Symbol<extern "C" fn() -> U> = self.get_function(name);
        func()
    }

    pub fn mock_time_set_ns(&self, nanos: u64) -> bool {
        type FnType = extern "C" fn(u64);
        let Some(func) = self.get_optional::<FnType>("__mock_time_set_ns") else {
            return false;
        };
        func(nanos);
        true
    }

    pub fn mock_time_advance_ns(&self, nanos: u64) -> bool {
        type FnType = extern "C" fn(u64);
        let Some(func) = self.get_optional::<FnType>("__mock_time_advance_ns") else {
            return false;
        };
        func(nanos);
        true
    }

    pub fn mock_time_set_u32(&self, secs: u32) -> bool {
        type FnType = extern "C" fn(u32);
        let Some(func) = self.get_optional::<FnType>("__mock_time_set_u32") else {
            return false;
        };
        func(secs);
        true
    }

    pub fn mock_time_advance_u32(&self, secs: u32) -> bool {
        type FnType = extern "C" fn(u32);
        let Some(func) = self.get_optional::<FnType>("__mock_time_advance_u32") else {
            return false;
        };
        func(secs);
        true
    }
}

static BUILD_STDLIB_ONCE: Once = Once::new();

fn ensure_mock_time_stdlib_built() {
    BUILD_STDLIB_ONCE.call_once(|| {
        let mut build_cmd = Command::new("cargo");
        build_cmd.args(["build", "-p", "iec61131std", "--features", "mock_time"]);
        if !cfg!(debug_assertions) {
            build_cmd.arg("--release");
        }
        if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
            build_cmd.env("CARGO_TARGET_DIR", target_dir);
        }
        let build_status = build_cmd.status().expect("Failed to run cargo build for stdlib");
        assert!(build_status.success(), "Failed to build stdlib with mock_time");
    });
}

pub fn compile_and_load<T: Compilable>(context: &CodegenContext, source: T, includes: T) -> CompiledModule {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let obj_path = temp_dir.path().join("test.o");

    // Platform-specific shared library extension
    let so_path = if cfg!(target_os = "windows") {
        temp_dir.path().join("test.dll")
    } else if cfg!(target_os = "macos") {
        temp_dir.path().join("libtest.dylib")
    } else {
        temp_dir.path().join("libtest.so")
    };

    // Compile ST to object file
    let module = compile_with_includes(context, source, includes);
    let target = Target::default();

    module
        .persist_to_shared_pic_object(obj_path.clone(), &target, OptimizationLevel::None)
        .expect("Failed to compile to object");

    // Path to prebuilt stdlib (prefer test deps for mock_time-enabled builds)
    let target_dir = std::env::var("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().join("target")
    });
    let stdlib_profile_dir = target_dir.join(if cfg!(debug_assertions) { "debug" } else { "release" });

    ensure_mock_time_stdlib_built();

    let stdlib_path = if cfg!(target_os = "windows") {
        stdlib_profile_dir.join("iec61131std.lib")
    } else {
        stdlib_profile_dir.join("libiec61131std.a")
    };

    assert!(
        stdlib_path.exists(),
        "Missing stdlib artifact; run tests with the stdlib built (mock_time enabled)."
    );

    // Link using platform-specific commands
    let status = if cfg!(target_os = "windows") {
        Command::new("cl")
            .args([
                "/LD",
                obj_path.to_str().unwrap(),
                stdlib_path.to_str().unwrap(),
                &format!("/Fe:{}", so_path.to_str().unwrap()),
            ])
            .status()
            .expect("Failed to run cl.exe")
    } else if cfg!(target_os = "macos") {
        Command::new("cc")
            .args([
                "-dynamiclib",
                "-fPIC",
                "-o",
                so_path.to_str().unwrap(),
                obj_path.to_str().unwrap(),
                stdlib_path.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to run cc")
    } else {
        Command::new("cc")
            .args([
                "-shared",
                "-fPIC",
                "-o",
                so_path.to_str().unwrap(),
                obj_path.to_str().unwrap(),
                stdlib_path.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to run cc")
    };

    assert!(status.success(), "Linking failed");

    // Load - this automatically runs constructors!
    let library = unsafe { Library::new(&so_path).expect("Failed to load library") };

    CompiledModule { _temp_dir: temp_dir, library }
}

#[allow(unused_macros)] //This is actually used in subtests
macro_rules! add_std {
    ($src:expr, $($name:expr),* ) => {
        {
            let mut res = vec![$src.into()];
            $(
               res.push(crate::common::get_st_file($name));
            )*
            res
        }
    };
}

#[allow(unused_imports)] //This is actually used in subtests
pub(crate) use add_std;

#[macro_export]
macro_rules! assert_almost_eq {
    ($left:expr, $right:expr, $prec:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                let diff = (left_val - right_val).abs();

                if diff > $prec {
                    panic!(
                        "assertion failed: `(left == right)`\n      left: `{:?}`,\n     right: `{:?}`",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
}

/// Gets a file from the ST defined standard functions
#[allow(dead_code)]
pub fn get_st_file(name: &str) -> SourceCode {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("iec61131-st");
    data_path.push(name);

    assert!(data_path.exists());

    data_path.load_source(None).expect("Could not load source")
}

///
/// A Convenience method to compile and then run the given source
///
#[allow(dead_code)] //Not all test modules call the compile and run
pub fn compile_and_run<T, U, S: Compilable>(source: S, includes: S, params: &mut T) -> U {
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    module.run::<T, U>("main", params)
}

///
/// A Convenience method to compile and then run the given source with no parameters
///
#[allow(dead_code)] //Not all test modules call the compile and run
pub fn compile_and_run_no_params<T, S: Compilable>(source: S, includes: S) -> T {
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    module.run_no_param::<T>("main")
}

/// Helper function to create includes from standard library files
#[allow(dead_code)]
pub fn get_includes(files: &[&str]) -> Vec<SourceCode> {
    files.iter().map(|name| get_st_file(name)).collect()
}
