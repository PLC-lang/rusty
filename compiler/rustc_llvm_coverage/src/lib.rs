// use rustc_llvm::RustString;

mod ffi;
pub mod types;

use types::*;

// use crate::abi::Abi;
// use crate::llvm;
// use libc::c_uint;

use std::ffi::CString;

// /*
//  * Many of the functions in this file have been adapted from the
//  * `rustc` implementation of LLVM code coverage.
//  *
//  * https://github.com/rust-lang/rust/blob/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/compiler/rustc_codegen_llvm/src/coverageinfo/mod.rs#L220-L221
//  */
// /// Calls llvm::createPGOFuncNameVar() with the given function instance's
// /// mangled function name. The LLVM API returns an llvm::GlobalVariable
// /// containing the function name, with the specific variable name and linkage
// /// required by LLVM InstrProf source-based coverage instrumentation. Use
// /// `bx.get_pgo_func_name_var()` to ensure the variable is only created once per
// /// `Instance`.
// // fn create_pgo_func_name_var<'ll, 'tcx>(
// //     cx: &CodegenCx<'ll, 'tcx>,
// //     instance: Instance<'tcx>,
// // ) -> &'ll llvm::Value {
// //     let mangled_fn_name =
// //         CString::new(cx.tcx.symbol_name(instance).name).expect("error converting function name to C string");
// //     let llfn = cx.get_fn(instance);
// //     unsafe { llvm::LLVMRustCoverageCreatePGOFuncNameVar(llfn, mangled_fn_name.as_ptr()) }
// // }

pub fn write_filenames_section_to_buffer<'a>(
    filenames: impl IntoIterator<Item = &'a CString>,
    buffer: &RustString,
) {
    let c_str_vec = filenames.into_iter().map(|cstring| cstring.as_ptr()).collect::<Vec<_>>();
    unsafe {
        ffi::LLVMRustCoverageWriteFilenamesSectionToBuffer(c_str_vec.as_ptr(), c_str_vec.len(), buffer);
    }
}

// pub(crate) fn write_mapping_to_buffer(
//     virtual_file_mapping: Vec<u32>,
//     expressions: Vec<CounterExpression>,
//     mapping_regions: Vec<CounterMappingRegion>,
//     buffer: &RustString,
// ) {
//     unsafe {
//         llvm::LLVMRustCoverageWriteMappingToBuffer(
//             virtual_file_mapping.as_ptr(),
//             virtual_file_mapping.len() as c_uint,
//             expressions.as_ptr(),
//             expressions.len() as c_uint,
//             mapping_regions.as_ptr(),
//             mapping_regions.len() as c_uint,
//             buffer,
//         );
//     }
// }

pub fn hash_str(strval: &str) -> u64 {
    let strval = CString::new(strval).expect("null error converting hashable str to C string");
    unsafe { ffi::LLVMRustCoverageHashCString(strval.as_ptr()) }
}

pub fn hash_bytes(bytes: Vec<u8>) -> u64 {
    unsafe { ffi::LLVMRustCoverageHashByteArray(bytes.as_ptr().cast(), bytes.len()) }
}

pub fn mapping_version() -> u32 {
    unsafe { ffi::LLVMRustCoverageMappingVersion() }
}

// pub(crate) fn save_cov_data_to_mod<'ll, 'tcx>(cx: &CodegenCx<'ll, 'tcx>, cov_data_val: &'ll llvm::Value) {
//     let covmap_var_name = llvm::build_string(|s| unsafe {
//         llvm::LLVMRustCoverageWriteMappingVarNameToString(s);
//     })
//     .expect("Rust Coverage Mapping var name failed UTF-8 conversion");
//     debug!("covmap var name: {:?}", covmap_var_name);

//     let covmap_section_name = llvm::build_string(|s| unsafe {
//         llvm::LLVMRustCoverageWriteMapSectionNameToString(cx.llmod, s);
//     })
//     .expect("Rust Coverage section name failed UTF-8 conversion");
//     debug!("covmap section name: {:?}", covmap_section_name);

//     let llglobal = llvm::add_global(cx.llmod, cx.val_ty(cov_data_val), &covmap_var_name);
//     llvm::set_initializer(llglobal, cov_data_val);
//     llvm::set_global_constant(llglobal, true);
//     llvm::set_linkage(llglobal, llvm::Linkage::PrivateLinkage);
//     llvm::set_section(llglobal, &covmap_section_name);
//     llvm::set_alignment(llglobal, VAR_ALIGN_BYTES);
//     cx.add_used_global(llglobal);
// }

// pub(crate) fn save_func_record_to_mod<'ll, 'tcx>(
//     cx: &CodegenCx<'ll, 'tcx>,
//     func_name_hash: u64,
//     func_record_val: &'ll llvm::Value,
//     is_used: bool,
// ) {
//     // Assign a name to the function record. This is used to merge duplicates.
//     //
//     // In LLVM, a "translation unit" (effectively, a `Crate` in Rust) can describe functions that
//     // are included-but-not-used. If (or when) Rust generates functions that are
//     // included-but-not-used, note that a dummy description for a function included-but-not-used
//     // in a Crate can be replaced by full description provided by a different Crate. The two kinds
//     // of descriptions play distinct roles in LLVM IR; therefore, assign them different names (by
//     // appending "u" to the end of the function record var name, to prevent `linkonce_odr` merging.
//     let func_record_var_name = format!("__covrec_{:X}{}", func_name_hash, if is_used { "u" } else { "" });
//     debug!("function record var name: {:?}", func_record_var_name);

//     let func_record_section_name = llvm::build_string(|s| unsafe {
//         llvm::LLVMRustCoverageWriteFuncSectionNameToString(cx.llmod, s);
//     })
//     .expect("Rust Coverage function record section name failed UTF-8 conversion");
//     debug!("function record section name: {:?}", func_record_section_name);

//     let llglobal = llvm::add_global(cx.llmod, cx.val_ty(func_record_val), &func_record_var_name);
//     llvm::set_initializer(llglobal, func_record_val);
//     llvm::set_global_constant(llglobal, true);
//     llvm::set_linkage(llglobal, llvm::Linkage::LinkOnceODRLinkage);
//     llvm::set_visibility(llglobal, llvm::Visibility::Hidden);
//     llvm::set_section(llglobal, &func_record_section_name);
//     llvm::set_alignment(llglobal, VAR_ALIGN_BYTES);
//     llvm::set_comdat(cx.llmod, llglobal, &func_record_var_name);
//     cx.add_used_global(llglobal);
// }
