#![allow(dead_code, unused_variables)]

// This FFI links to the static library built by `rustc_llvm`
//
// Function interface definitions are taken from [here](https://github.com/rust-lang/rust/blob/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/compiler/rustc_codegen_llvm/src/llvm/ffi.rs#L1864).
//
use libc::{c_char, c_uint, c_void, size_t};
use std::slice;

use super::types::*;
use llvm_sys::prelude::{LLVMModuleRef, LLVMPassManagerRef, LLVMValueRef};

/// Appending to a Rust string -- used by RawRustStringOstream.
#[no_mangle]
pub unsafe extern "C" fn LLVMRustStringWriteImpl(sr: &RustString, ptr: *const c_char, size: size_t) {
    let slice = slice::from_raw_parts(ptr as *const u8, size as usize);

    sr.bytes.borrow_mut().extend_from_slice(slice);
}

#[link(name = "llvm-wrapper", kind = "static")]
extern "C" {
    #[allow(improper_ctypes)]
    pub fn LLVMRustCoverageWriteFilenamesSectionToBuffer(
        Filenames: *const *const c_char,
        FilenamesLen: size_t,
        BufferOut: &RustString,
    );

    #[allow(improper_ctypes)]
    pub fn LLVMRustCoverageWriteMappingToBuffer(
        VirtualFileMappingIDs: *const c_uint,
        NumVirtualFileMappingIDs: c_uint,
        Expressions: *const CounterExpression,
        NumExpressions: c_uint,
        MappingRegions: *const CounterMappingRegion,
        NumMappingRegions: c_uint,
        BufferOut: &RustString,
    );

    pub fn LLVMRustCoverageCreatePGOFuncNameVar(F: LLVMValueRef, FuncName: *const c_char) -> LLVMValueRef;
    pub fn LLVMRustCoverageHashCString(StrVal: *const c_char) -> u64;
    pub fn LLVMRustCoverageHashByteArray(Bytes: *const c_char, NumBytes: size_t) -> u64;

    #[allow(improper_ctypes)]
    pub fn LLVMRustCoverageWriteMapSectionNameToString(M: LLVMModuleRef, Str: &RustString);

    #[allow(improper_ctypes)]
    pub fn LLVMRustCoverageWriteFuncSectionNameToString(M: LLVMModuleRef, Str: &RustString);

    #[allow(improper_ctypes)]
    pub fn LLVMRustCoverageWriteMappingVarNameToString(Str: &RustString);

    pub fn LLVMRustCoverageMappingVersion() -> u32;

    // pub fn LLVMRustAddInstrumentationPass(PM: LLVMPassManagerRef);
    // pub fn LLVMRustRunInstrumentationPass(M: LLVMModuleRef);
}
