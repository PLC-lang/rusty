use crate::write_filenames_section_to_buffer;

use super::*;
use inkwell::context::Context;
use inkwell::intrinsics::Intrinsic;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use std::ffi::CString;

/// This represents a coverage mapping header that has been written to a module.
/// It is returned for debugging purposes and use with write_function_record.
pub struct WrittenCoverageMappingHeader {
    pub mapping_version: u32,
    pub filenames: Vec<String>,
    pub filenames_hash: u64,
}

/// filenames: In Coverage Mapping Version > 6, first filename must be the compilation directory
pub fn write_coverage_mapping_header<'ctx>(
    module: &Module<'ctx>,
    filenames: Vec<String>,
) -> WrittenCoverageMappingHeader {
    // Get context
    let context = module.get_context();

    // Convert filenames to CStrings
    let filenames_cstr = filenames.clone().into_iter().map(|f| CString::new(f).unwrap()).collect::<Vec<_>>();
    let mut encoded_filename_buffer = RustString::new();
    write_filenames_section_to_buffer(&filenames_cstr, &mut encoded_filename_buffer);
    let filenames_hash = hash_bytes(encoded_filename_buffer.bytes.borrow().to_vec());

    // Get values
    let mapping_version = mapping_version(); // versions are zero-indexed
    assert_eq!(mapping_version, 5, "Only mapping version 6 is supported");
    let encoded_filenames_len = encoded_filename_buffer.len();

    // Create mapping header types
    let i32_type = context.i32_type();
    let i32_zero = i32_type.const_int(0, false);
    let i32_cov_mapping_version = i32_type.const_int(mapping_version.into(), false);
    let i32_filenames_len = i32_type.const_int(encoded_filenames_len as u64, false);

    // See LLVM Code Coverage Specification for details on this data structure
    let cov_mapping_header = context.const_struct(
        &[
            // Value 1 : Always zero
            i32_zero.into(),
            // Value 2 : Len(encoded_filenames)
            i32_filenames_len.into(),
            // Value 3 : Always zero
            i32_zero.into(),
            // Value 4 : Mapping version
            i32_cov_mapping_version.into(),
        ],
        // https://github.com/rust-lang/rust/blob/e6707df0de337976dce7577e68fc57adcd5e4842/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs#L301
        false,
    );

    // Create filename value types
    let i8_type = context.i8_type();
    let i8_filename_array = i8_type.const_array(
        &encoded_filename_buffer
            .bytes
            .borrow()
            .iter()
            .map(|byte| i8_type.const_int(*byte as u64, false))
            .collect::<Vec<_>>(),
    );

    // Create structure
    let coverage_struct = context.const_struct(&[cov_mapping_header.into(), i8_filename_array.into()], false);

    // Write to module
    save_cov_data_to_mod(module, coverage_struct);

    // Return header
    WrittenCoverageMappingHeader { mapping_version, filenames, filenames_hash }
}

pub struct FunctionRecord {
    name: String,
    name_md5_hash: u64,
    structural_hash: u64,
    virtual_file_mapping: Vec<u32>,
    expressions: Vec<CounterExpression>,
    mapping_regions: Vec<CounterMappingRegion>,
    mapping_buffer: RustString,

    // A.k.a. hash of all filenames in module
    translation_unit_hash: u64,
    is_used: bool,
}

impl FunctionRecord {
    pub fn new(
        name: String,
        structural_hash: u64,
        // TODO - better names for these
        function_filenames: Vec<String>,
        expressions: Vec<CounterExpression>,
        mapping_regions: Vec<CounterMappingRegion>,
        is_used: bool,

        written_mapping_header: &WrittenCoverageMappingHeader,
    ) -> Self {
        let name_md5_hash = hash_str(&name);

        // Get indexes of function filenames in module file list
        // TODO - hoist this into rusty
        let mut virtual_file_mapping = Vec::new();
        for filename in function_filenames {
            let filename_idx = written_mapping_header
                .filenames
                .iter()
                .position(|f| f == &filename)
                .expect("Unable to find function filename in module files");
            virtual_file_mapping.push(filename_idx.try_into().unwrap());
        }

        // Write mapping to buffer
        let mut mapping_buffer = RustString::new();
        write_mapping_to_buffer(
            virtual_file_mapping.clone(),
            expressions.clone(),
            mapping_regions.clone(),
            &mut mapping_buffer,
        );

        FunctionRecord {
            name,
            name_md5_hash,
            structural_hash,
            virtual_file_mapping,
            expressions,
            is_used,
            mapping_regions,
            mapping_buffer,
            translation_unit_hash: written_mapping_header.filenames_hash,
        }
    }

    pub fn write_to_module<'ctx>(&self, module: &Module<'ctx>) {
        // Get context
        let context = module.get_context();

        // Create types
        let i64_type = context.i64_type();
        let i32_type = context.i32_type();
        let i8_type = context.i8_type();

        // Create values
        let i64_name_md5_hash = i64_type.const_int(self.name_md5_hash, false);
        let i32_mapping_len = i32_type.const_int(self.mapping_buffer.len() as u64, false);
        let i64_structural_hash = i64_type.const_int(self.structural_hash, false);
        let i64_translation_unit_hash = i64_type.const_int(self.translation_unit_hash, false);

        // Build mapping array
        let i8_mapping_array = i8_type.const_array(
            &self
                .mapping_buffer
                .bytes
                .borrow()
                .iter()
                .map(|byte| i8_type.const_int(*byte as u64, false))
                .collect::<Vec<_>>(),
        );

        // Create structure
        let function_record_struct = context.const_struct(
            &[
                i64_name_md5_hash.into(),
                i32_mapping_len.into(),
                i64_structural_hash.into(),
                i64_translation_unit_hash.into(),
                i8_mapping_array.into(),
            ],
            // https://github.com/rust-lang/rust/blob/e6707df0de337976dce7577e68fc57adcd5e4842/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs#L311
            true,
        );

        save_func_record_to_mod(&module, self.name_md5_hash, function_record_struct, self.is_used);
    }
}
/// Why the pass isn't working yet:
///
pub fn run_legacy_coverage_pass<'ctx>(module: &Module<'ctx>) {
    // Add the intrinsic
    let context = module.get_context();
    // let increment_intrinsic = Intrinsic::find("llvm.instrprof.increment").unwrap();
    // println!("intrinsic: {:?}", increment_intrinsic);
    // let definition = increment_intrinsic.get_declaration(&module, &[]).unwrap();
    // println!("definition: {:?}", definition);

    // Run the pass
    let pm = PassManager::create(());
    unsafe {
        // Switch between new and legacy pass managers with the following:
        LLVMRustAddInstrumentationPass(pm.as_mut_ptr());
        // LLVMRustRunInstrumentationPass(module.as_mut_ptr());
    }
    let did_run = pm.run_on(module);
    println!("Did run: {}", did_run);
    // let did_init = pm.initialize();
    // println!("Did init: {}", did_init);
    // let did_finalize = pm.finalize();
    // println!("Did finalize: {:?}", did_finalize);
}

// TODO
// - why the pass isn't working yet: https://github.com/llvm/llvm-project/blob/f28c006a5895fc0e329fe15fead81e37457cb1d1/llvm/lib/Transforms/Instrumentation/InstrProfiling.cpp#L539-L549
// - pass pgo func var to incr (after creating) - https://github.com/rust-lang/rust/blob/174e73a3f6df6f96ab453493796e461164dea94a/compiler/rustc_codegen_llvm/src/coverageinfo/mod.rs#L59-L74
// - this pass is what generates increment calls - https://github.com/rust-lang/rust/blob/1.64.0/compiler/rustc_mir_transform/src/coverage/mod.rs
// - call invoking in normal inkwell pipeline, through -instrprof
// - investigate codegen diffs for function/function blocks/programs
