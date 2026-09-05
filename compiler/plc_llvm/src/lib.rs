//! This crate provides Rust bindings for LLVM Target Machine functionalities.

use inkwell::llvm_sys::prelude::LLVMBool;
use inkwell::llvm_sys::support::LLVMParseCommandLineOptions;
use inkwell::targets::TargetMachine;
use std::ffi::{CString, c_int};

mod ffi {
    use inkwell::llvm_sys::prelude::LLVMBool;
    use inkwell::llvm_sys::target_machine::LLVMTargetMachineRef;

    #[link(name = "llvm_wrapper")]
    unsafe extern "C" {
        pub fn setUseInitArray(tm: LLVMTargetMachineRef, use_init_array: LLVMBool);
    }
}

pub trait TargetMachineExt {
    fn use_init_array(&mut self, use_init_array: bool);
}

impl TargetMachineExt for TargetMachine {
    /// Set whether to use `.init_array` and `.fini_array` sections for global
    /// constructors and destructors instead of the legacy `.ctors` and `.dtors`
    /// sections.
    ///
    /// By default, LLVM uses `.ctors` and `.dtors` sections. This method allows
    /// changing that behavior.
    fn use_init_array(&mut self, use_init_array: bool) {
        let tm = self.as_mut_ptr();
        unsafe {
            ffi::setUseInitArray(tm, if use_init_array { 1 } else { 0 } as LLVMBool);
        }
    }
}

/// Sets an LLVM command-line option, for backend knobs that have no `TargetOptions`
/// field. The option is parsed as `-name=value` through LLVM's command-line parser,
/// which ignores options it does not know, such as the knob of a target that is not
/// built in.
///
/// The option registry is process-global and is read while a pass pipeline is built,
/// so an option must be set before any code generation starts.
pub fn set_llvm_option(name: &str, value: &str) {
    let Ok(option) = CString::new(format!("-{name}={value}")) else {
        return;
    };
    let argv = [c"plc".as_ptr(), option.as_ptr()];
    unsafe { LLVMParseCommandLineOptions(argv.len() as c_int, argv.as_ptr(), c"".as_ptr()) };
}
