#![allow(dead_code, unused_variables)]

// This FFI links to the static library built by `rustc_llvm`
//
// Function interface definitions are taken from [here](https://github.com/rust-lang/rust/blob/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/compiler/rustc_codegen_llvm/src/llvm/ffi.rs#L1864).
//
use libc::{c_char, c_uint, c_void, size_t};

use rustc_llvm::RustString;

use super::types::*;

#[repr(C)]
pub struct Module(c_void);

#[repr(C)]
pub struct Value(c_void);

#[link(name = "llvm-wrapper")]
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

    pub fn LLVMRustCoverageCreatePGOFuncNameVar(F: &Value, FuncName: *const c_char) -> &Value;
    pub fn LLVMRustCoverageHashCString(StrVal: *const c_char) -> u64;
    pub fn LLVMRustCoverageHashByteArray(Bytes: *const c_char, NumBytes: size_t) -> u64;

    #[allow(improper_ctypes)]
    pub fn LLVMRustCoverageWriteMapSectionNameToString(M: &Module, Str: &RustString);

    #[allow(improper_ctypes)]
    pub fn LLVMRustCoverageWriteFuncSectionNameToString(M: &Module, Str: &RustString);

    #[allow(improper_ctypes)]
    pub fn LLVMRustCoverageWriteMappingVarNameToString(Str: &RustString);

    pub fn LLVMRustCoverageMappingVersion() -> u32;
}
