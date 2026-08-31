//! This crate provides Rust bindings for LLVM Target Machine functionalities.

use inkwell::llvm_sys::prelude::LLVMBool;
use inkwell::targets::TargetMachine;
use std::ffi::CString;

mod ffi {
    use inkwell::llvm_sys::prelude::LLVMBool;
    use inkwell::llvm_sys::target_machine::LLVMTargetMachineRef;
    use std::ffi::c_char;

    #[link(name = "llvm_wrapper")]
    unsafe extern "C" {
        pub fn setUseInitArray(tm: LLVMTargetMachineRef, use_init_array: LLVMBool);
        pub fn setLLVMOption(name: *const c_char, value: *const c_char) -> LLVMBool;
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

/// Sets a registered LLVM command-line option, for backend knobs that have no
/// `TargetOptions` field and no C-API entry point. Returns `false` if LLVM does not
/// know the option or rejects the value, which is how a renamed or removed option in
/// a future LLVM surfaces.
///
/// The option registry is process-global and is read while a pass pipeline is built,
/// so an option must be set before any code generation starts.
pub fn set_llvm_option(name: &str, value: &str) -> bool {
    let (Ok(name), Ok(value)) = (CString::new(name), CString::new(value)) else {
        return false;
    };
    unsafe { ffi::setLLVMOption(name.as_ptr(), value.as_ptr()) != 0 }
}

#[cfg(test)]
mod tests {
    use super::set_llvm_option;

    #[test]
    fn tail_merging_option_is_known_to_this_llvm() {
        assert!(set_llvm_option("enable-tail-merge", "false"));
    }

    #[test]
    fn unknown_option_is_rejected() {
        assert!(!set_llvm_option("plc-no-such-llvm-option", "false"));
    }
}
