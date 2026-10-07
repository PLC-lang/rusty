//! This crate provides Rust bindings for LLVM functionalities that neither inkwell nor the C API expose.

use inkwell::debug_info::DebugInfoBuilder;
use inkwell::llvm_sys::prelude::LLVMBool;
use inkwell::llvm_sys::support::LLVMParseCommandLineOptions;
use inkwell::targets::TargetMachine;
use inkwell::values::{AsValueRef, FunctionValue, InstructionValue};
use std::ffi::{CString, c_int};
use std::sync::Mutex;

mod ffi {
    use inkwell::llvm_sys::prelude::{LLVMBool, LLVMDIBuilderRef, LLVMValueRef};
    use inkwell::llvm_sys::target_machine::LLVMTargetMachineRef;

    #[link(name = "llvm_wrapper")]
    unsafe extern "C" {
        pub fn setUseInitArray(tm: LLVMTargetMachineRef, use_init_array: LLVMBool);
        pub fn enableKeyInstructions(builder: LLVMDIBuilderRef, function: LLVMValueRef);
        pub fn setKeyInstruction(instruction: LLVMValueRef, group: u64, rank: u8);
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

pub trait DebugInfoBuilderExt<'ctx> {
    fn enable_key_instructions(&self, function: FunctionValue<'ctx>);
}

impl<'ctx> DebugInfoBuilderExt<'ctx> for DebugInfoBuilder<'ctx> {
    /// Replaces the function's subprogram by one that places `is_stmt` by Key Instructions (see
    /// [`InstructionValueExt::set_key_instruction`]). LLVM fixes this flag when it creates a
    /// subprogram, so call this before anything refers to the function's subprogram.
    fn enable_key_instructions(&self, function: FunctionValue<'ctx>) {
        unsafe { ffi::enableKeyInstructions(self.as_mut_ptr(), function.as_value_ref()) }
    }
}

pub trait InstructionValueExt {
    fn set_key_instruction(self, group: u64, rank: u8);
}

impl InstructionValueExt for InstructionValue<'_> {
    /// Makes this instruction a member of the Key Instructions atom `group`. A debugger stops only
    /// at the member of lowest `rank` (1 ranks highest) of each group. An instruction without a
    /// debug location stays unchanged.
    fn set_key_instruction(self, group: u64, rank: u8) {
        unsafe { ffi::setKeyInstruction(self.as_value_ref(), group, rank) }
    }
}

/// Sets an LLVM command-line option, for backend knobs that have no `TargetOptions`
/// field. The option is parsed as `-name=value` through LLVM's command-line parser,
/// which ignores options it does not know, such as the knob of a target that is not
/// built in.
///
/// The option registry is process-global and is read while a pass pipeline is built,
/// so an option must be set before any code generation starts. The parser itself is not
/// thread-safe, so concurrent calls are serialized.
pub fn set_llvm_option(name: &str, value: &str) {
    static PARSER: Mutex<()> = Mutex::new(());

    let Ok(option) = CString::new(format!("-{name}={value}")) else {
        return;
    };
    let argv = [c"plc".as_ptr(), option.as_ptr()];
    let _guard = PARSER.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    unsafe { LLVMParseCommandLineOptions(argv.len() as c_int, argv.as_ptr(), c"".as_ptr()) };
}
