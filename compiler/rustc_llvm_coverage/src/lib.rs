//! This library provides a Rust interface to LLVM's coverage mapping format.
//!
//! This module exists to provide intuitive and useful abstractions for
//! interacting with LLVM's coverage mapping functions. If you want to
//! interact directly with LLVM, use the [`interfaces`] or [`ffi`] modules.
//!
//!

pub mod ffi;
pub mod interfaces;
pub mod types;
use interfaces::create_pgo_func_name_var;
use interfaces::*;
use types::*;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::intrinsics::Intrinsic;
use inkwell::module::Module;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetTriple};
use inkwell::values::PointerValue;
use inkwell::OptimizationLevel;
use std::ffi::CString;

/// This represents a coverage mapping header that has been written to a module.
/// It is returned for debugging purposes and use with write_function_record.
pub struct CoverageMappingHeader {
    pub mapping_version: u32,
    pub filenames: Vec<String>,
    pub filenames_hash: u64,
    pub encoded_filename_buffer: RustString,
}

impl CoverageMappingHeader {
    pub fn new(filenames: Vec<String>) -> Self {
        // Get mapping version from LLVM
        let mapping_version = get_mapping_version(); // versions are zero-indexed
        assert_eq!(mapping_version, 5, "Only mapping version 6 is supported");

        // Convert filenames to CStrings
        let filenames_cstr =
            filenames.clone().into_iter().map(|f| CString::new(f).unwrap()).collect::<Vec<_>>();
        let mut encoded_filename_buffer = RustString::new();
        write_filenames_section_to_buffer(&filenames_cstr, &mut encoded_filename_buffer);

        // Calc file hash
        let filenames_hash = hash_bytes(encoded_filename_buffer.bytes.borrow().to_vec());

        CoverageMappingHeader { mapping_version, filenames, filenames_hash, encoded_filename_buffer }
    }

    /// filenames: In Coverage Mapping Version > 6, first filename must be the compilation directory
    pub fn write_coverage_mapping_header<'ctx>(&self, module: &Module<'ctx>) {
        // Get context
        let context = module.get_context();

        // Create mapping header types
        let i32_type = context.i32_type();
        let i32_zero = i32_type.const_int(0, false);
        let i32_cov_mapping_version = i32_type.const_int(self.mapping_version.into(), false);
        let i32_filenames_len = i32_type.const_int(self.encoded_filename_buffer.len() as u64, false);

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
            &self
                .encoded_filename_buffer
                .bytes
                .borrow()
                .iter()
                .map(|byte| i8_type.const_int(*byte as u64, false))
                .collect::<Vec<_>>(),
        );

        // Create structure
        let coverage_struct =
            context.const_struct(&[cov_mapping_header.into(), i8_filename_array.into()], false);

        // Write to module
        save_cov_data_to_mod(module, coverage_struct);
    }
}

pub struct FunctionRecord {
    pub name: String,
    pub name_md5_hash: u64,
    pub structural_hash: u64,
    pub virtual_file_mapping: Vec<u32>,
    pub expressions: Vec<CounterExpression>,
    pub mapping_regions: Vec<CounterMappingRegion>,
    pub mapping_buffer: RustString,

    // A.k.a. hash of all filenames in module
    pub translation_unit_hash: u64,
    pub is_used: bool,
}

impl FunctionRecord {
    /// TODO - Update to use a filename table, like
    /// https://github.com/rust-lang/rust/blob/e6707df0de337976dce7577e68fc57adcd5e4842/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs#L155-L194
    pub fn new(
        name: String,
        structural_hash: u64,
        // TODO - better names for these
        function_filenames: Vec<String>,
        expressions: Vec<CounterExpression>,
        mapping_regions: Vec<CounterMappingRegion>,
        is_used: bool,

        written_mapping_header: &CoverageMappingHeader,
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

/// This pass will not operate unless the module already has intrinsic calls.
/// See [here](https://github.com/llvm/llvm-project/blob/f28c006a5895fc0e329fe15fead81e37457cb1d1/llvm/lib/Transforms/Instrumentation/InstrProfiling.cpp#L539-L549) for why.
pub fn run_instrumentation_lowering_pass<'ctx>(module: &Module<'ctx>) {
    // Setup
    let initialization_config = &InitializationConfig::default();
    inkwell::targets::Target::initialize_all(initialization_config);

    // Architecture Specifics
    // Module.set_triple() is required because the pass needs to know it's compiling
    // to ELF [here](https://github.com/llvm/llvm-project/blob/cfa30fa4852275eed0c59b81b5d8088d3e55f778/llvm/lib/Transforms/Instrumentation/InstrProfiling.cpp#L1191-L1199).
    // TODO - pass this as a param
    let triple = TargetTriple::create("x86_64-pc-linux-gnu");
    module.set_triple(&triple);
    let target = Target::from_triple(&triple).unwrap();
    let machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::None,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    // Run pass (uses new pass manager)
    let _ = module.run_passes("instrprof", &machine, PassBuilderOptions::create());
}

/// Emits a increment counter call at the current builder position.
///
/// `pgo_function_var` is a pointer to the function's global name variable,
/// generated from [`create_pgo_func_name_var`].
///
/// TODO - verify the correctness of these lifetimes.
pub fn emit_counter_increment<'ink, 'ctx>(
    builder: &Builder<'ink>,
    module: &Module<'ctx>,
    pgo_function_var: &PointerValue<'ink>,
    structural_hash: u64,
    num_counters: u32,
    counter_idx: u64,
) {
    let context = module.get_context();
    let increment_intrinsic = Intrinsic::find("llvm.instrprof.increment").unwrap();
    let increment_intrinsic_func = increment_intrinsic.get_declaration(module, &[]).unwrap();

    // Create types
    let i64_type = context.i64_type();
    let i32_type = context.i32_type();

    let i64_hash = i64_type.const_int(structural_hash, false);
    let i32_num_counters = i32_type.const_int(num_counters.into(), false);
    let i64_counter_idx = i64_type.const_int(counter_idx, false);

    builder.build_call(
        increment_intrinsic_func,
        &[(*pgo_function_var).into(), i64_hash.into(), i32_num_counters.into(), i64_counter_idx.into()],
        "increment_call",
    );
}

// TODO
// - investigate codegen diffs for function/function blocks/programs
