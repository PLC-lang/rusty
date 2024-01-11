use crate::write_filenames_section_to_buffer;

use super::*;
use inkwell::context::Context;
use inkwell::module::Module;
use std::ffi::CString;

pub struct FunctionRecord {
    name: String,
    structural_hash: u64,
    filenames_hash: u64,
    virtual_file_mapping: Vec<u32>,
    expressions: Vec<CounterExpression>,
    mapping_regions: Vec<CounterMappingRegion>,
}

pub fn write_coverage_mapping_header<'ctx>(module: &Module<'ctx>, filenames: Vec<String>) {
    // Get context
    let context = module.get_context();

    // Convert filenames to CStrings
    let filenames = filenames.into_iter().map(|f| CString::new(f).unwrap()).collect::<Vec<_>>();
    let mut encoded_filename_buffer = RustString::new();
    write_filenames_section_to_buffer(&filenames, &mut encoded_filename_buffer);

    // Get values
    let cov_mapping_version = mapping_version();
    let encoded_filenames_len = encoded_filename_buffer.len();

    // Create mapping header types
    let i32_type = context.i32_type();
    let i32_zero = i32_type.const_int(0, false);
    let i32_cov_mapping_version = i32_type.const_int(cov_mapping_version.into(), false);
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
}

pub fn write_function_record() {}
