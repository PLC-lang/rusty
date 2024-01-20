use super::LlvmTypedIndex;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, GlobalValue};
use plc_ast::ast::{CompilationUnit, LinkageType};
use plc_source::source_location::CodeSpan;
use rustc_llvm_coverage::types::{Counter, CounterId, CounterMappingRegion};
use rustc_llvm_coverage::*;
use std::collections::HashMap;
use std::ffi::CString;

pub struct CoverageInstrumentationBuilder<'ink> {
    context: &'ink Context,
    files: Vec<String>,
    cov_mapping_header: Option<CoverageMappingHeader>,
    function_pgos: HashMap<String, (FunctionRecord, GlobalValue<'ink>)>,
}

impl<'ink> CoverageInstrumentationBuilder<'ink> {
    pub fn new(context: &'ink Context, files: Vec<String>) -> Self {
        Self { context, files, cov_mapping_header: None, function_pgos: HashMap::new() }
    }

    pub fn initialize(&mut self, module: &Module<'ink>) {
        let cov_mapping_header = CoverageMappingHeader::new(self.files.clone());
        cov_mapping_header.write_coverage_mapping_header(module);
        self.cov_mapping_header = Some(cov_mapping_header);
    }

    pub fn create_function_records(
        &mut self,
        unit: &CompilationUnit,
        llvm_index: &LlvmTypedIndex,
        module: &Module<'ink>,
    ) {
        // Keep records
        let mut function_records = Vec::new();

        // Loop through functions in AST, create function records
        for implementation in &unit.implementations {
            // Skip non-internal functions (external links + built-ins)
            if implementation.linkage != LinkageType::Internal {
                continue;
            }

            let func_name = implementation.name.clone();
            // TODO - hash strucutrally
            let struct_hash = rustc_llvm_coverage::interfaces::hash_str(&func_name);
            // TODO - modify this?
            let func_filenames = vec![unit.file_name.clone()];
            // TODO - use expression counters
            let expressions = Vec::new();
            // TODO - file mapping table
            let file_id = 1;

            // TODO - loop through dfs
            let mut mapping_regions = Vec::new();
            {
                let (start_line, start_col, end_line, end_col) = implementation.location.get_start_end();
                let counter = Counter::counter_value_reference(CounterId::new(0));
                let mapping_region = CounterMappingRegion::code_region(
                    counter,
                    file_id,
                    start_line.try_into().unwrap(),
                    start_col.try_into().unwrap(),
                    end_line.try_into().unwrap(),
                    end_col.try_into().unwrap(),
                );
                mapping_regions.push(mapping_region);
            }

            let is_used = true;

            let written_coverage_header = &self.cov_mapping_header.as_mut().unwrap();

            let func = FunctionRecord::new(
                func_name,
                struct_hash,
                func_filenames,
                expressions,
                mapping_regions,
                is_used,
                &written_coverage_header,
            );

            func.write_to_module(module);

            function_records.push(func);
        }

        // Loop through LLVM definitions, create PGO vars
        for function_record in function_records {
            let func_name = function_record.name.clone();

            // TODO - decide whether or not this needs to come from the module
            // let func = llvm_index
            //     .find_associated_implementation(&func_name)
            //     .expect("Function not found in LLVM index");
            let func = module.get_function(&func_name).expect("Function not found in module!");

            let func_pgo = rustc_llvm_coverage::interfaces::create_pgo_func_name_var(&func);

            &self.function_pgos.insert(func_name, (function_record, func_pgo));
        }
    }

    pub fn emit_function_increment<'ctx>(
        &self,
        builder: &Builder<'ink>,
        module: &Module<'ctx>,
        func_name: &str,
        counter_index: u64,
    ) {
        let (func_record, func_pgo_var) = self.function_pgos.get(func_name).unwrap();

        // TODO - see if expressions need different num_counters
        let pgo_pointer = func_pgo_var.as_pointer_value();
        let num_counters = func_record.mapping_regions.len();

        rustc_llvm_coverage::emit_counter_increment(
            builder,
            module,
            &pgo_pointer,
            func_record.structural_hash,
            num_counters.try_into().unwrap(),
            counter_index,
        );
    }

    pub fn finalize(&mut self, module: &Module<'ink>) {
        run_instrumentation_lowering_pass(module);
    }
}
