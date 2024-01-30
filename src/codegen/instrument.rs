use super::LlvmTypedIndex;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::intrinsics::Intrinsic;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, GlobalValue};
use plc_ast::ast::{AstId, AstNode, AstStatement, CompilationUnit, Implementation, LinkageType};
use plc_ast::control_statements::AstControlStatement;
use plc_source::source_location::{CodeSpan, SourceLocation};
use rustc_llvm_coverage::types::{Counter, CounterId, CounterMappingRegion};
use rustc_llvm_coverage::*;
use std::collections::HashMap;
use std::ffi::CString;

pub struct CoverageInstrumentationBuilder<'ink> {
    context: &'ink Context,
    // module: &'ink Module<'ink>,
    files: Vec<String>,
    cov_mapping_header: Option<CoverageMappingHeader>,
    function_pgos: HashMap<String, (FunctionRecord, GlobalValue<'ink>)>,
    ast_counter_lookup: HashMap<AstId, usize>,
}

/// Manages the creation of mapping regions for a given function
#[derive(Debug)]
struct MappingRegionGenerator {
    // TODO - verify that counter ids are PER function
    pub mapping_regions: Vec<CounterMappingRegion>,
    next_counter_id: u32,
    file_id: u32,
}

impl MappingRegionGenerator {
    pub fn new(file_id: u32) -> Self {
        Self { mapping_regions: Vec::new(), next_counter_id: 0, file_id }
    }

    /// Returns the index of the counter id added
    pub fn add_mapping_region(&mut self, source: &SourceLocation) -> u32 {
        let (start_line, start_col, end_line, end_col) = source.get_start_end();
        let counter_id = self.next_counter_id;
        let counter = Counter::counter_value_reference(CounterId::new(counter_id));
        let mapping_region = CounterMappingRegion::code_region(
            counter,
            self.file_id,
            start_line.try_into().unwrap(),
            start_col.try_into().unwrap(),
            end_line.try_into().unwrap(),
            end_col.try_into().unwrap(),
        );
        self.mapping_regions.push(mapping_region);
        self.next_counter_id += 1;

        // Return the index of the counter id added
        counter_id
    }
}

impl<'ink> CoverageInstrumentationBuilder<'ink> {
    pub fn new(context: &'ink Context, /* module: &'ink Module<'ink>,*/ files: Vec<String>) -> Self {
        Self {
            context,
            // module,
            files,
            cov_mapping_header: None,
            function_pgos: HashMap::new(),
            ast_counter_lookup: HashMap::new(),
        }
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
            // Skip no-definition functions
            // TODO - investigate which functions don't have definitions and why
            if module.get_function(&implementation.name).is_none() {
                println!("Skipping undefined function: {}", &implementation.name);
                continue;
            }

            let func = self.generate_function_record(implementation);
            func.write_to_module(module);

            function_records.push(func);
        }

        // Loop through LLVM definitions, create PGO vars
        for function_record in function_records {
            let func_name = function_record.name.clone();

            let func = module
                .get_function(&func_name)
                .expect(&format!("Function not found in module: {}", func_name));

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

    pub fn emit_branch_increment<'ctx>(
        &self,
        builder: &Builder<'ink>,
        context: &Context,
        increment_intrinsic_func: &FunctionValue<'ink>,
        func_name: &str,
        ast_id: AstId,
    ) {
        let (func_record, func_pgo_var) = self.function_pgos.get(func_name).unwrap();

        let pgo_pointer = func_pgo_var.as_pointer_value();
        let num_counters = func_record.mapping_regions.len();

        let counter_index = match self.ast_counter_lookup.get(&ast_id) {
            Some(counter_index) => counter_index,
            None => {
                println!("Ast Not Registered: {} (from function {})", ast_id, func_name);
                return;
            }
        };

        rustc_llvm_coverage::emit_counter_increment_with_function(
            builder,
            context,
            increment_intrinsic_func,
            &pgo_pointer,
            func_record.structural_hash,
            num_counters.try_into().unwrap(),
            *counter_index as u64,
        );
    }

    pub fn finalize(&mut self, module: &Module<'ink>) {
        run_instrumentation_lowering_pass(module);
    }

    /// Internal function to generate for a function:
    /// - FunctionRecord
    /// - MappingRegions
    fn generate_function_record(&mut self, implementation: &Implementation) -> FunctionRecord {
        // Gather function information

        let func_name = implementation.name.clone();
        // TODO - hash strucutrally
        let struct_hash = rustc_llvm_coverage::interfaces::hash_str(&func_name);
        let func_filenames = vec![implementation.location.get_file_name().unwrap().to_string()];
        // TODO - use expression counters
        let expressions = Vec::new();
        // TODO - file mapping table
        let file_id = 1;

        // Map entire function
        let mut mapping_region_generator = MappingRegionGenerator::new(file_id);
        let func_ctr_id = mapping_region_generator.add_mapping_region(&implementation.location);
        assert!(func_ctr_id == 0);

        // DFS function statements
        self.generate_coverage_records(&implementation.statements, &mut mapping_region_generator);

        // TODO - determine if function is used
        let is_used = true;
        let written_coverage_header = &self.cov_mapping_header.as_mut().unwrap();

        FunctionRecord::new(
            func_name,
            struct_hash,
            func_filenames,
            expressions,
            mapping_region_generator.mapping_regions,
            is_used,
            &written_coverage_header,
        )
    }

    fn generate_coverage_records(
        &mut self,
        ast_node_list: &Vec<AstNode>,
        mapping_region_generator: &mut MappingRegionGenerator,
    ) {
        for ast_node in ast_node_list {
            // Only generate coverage records for control statements
            let control_statement = match &ast_node.stmt {
                AstStatement::ControlStatement(statement) => statement,
                _ => continue,
            };

            //
            match control_statement {
                AstControlStatement::If(statement) => {
                    // Loop through if/elif blocks
                    for block in &statement.blocks {
                        // Setup ast->id mapping, store region location
                        self.register_ast_list_as_region(&block.body, mapping_region_generator);
                        // Recurse
                        self.generate_coverage_records(&block.body, mapping_region_generator);
                    }

                    // Else block
                    self.register_ast_list_as_region(&statement.else_block, mapping_region_generator);
                    self.generate_coverage_records(&statement.else_block, mapping_region_generator);
                }
                AstControlStatement::ForLoop(statement) => (),
                AstControlStatement::WhileLoop(statement) => (),
                AstControlStatement::RepeatLoop(statement) => (),
                AstControlStatement::Case(statement) => (),
            }
        }
    }

    // TODO - find me a better name
    /// Registers a Vec<AstNode> as a region, spanning first and last
    fn register_ast_list_as_region(
        &mut self,
        ast_list: &Vec<AstNode>,
        mapping_region_generator: &mut MappingRegionGenerator,
    ) {
        if ast_list.is_empty() {
            return;
        }

        // Create a span from first_block -> last_block
        let first_block = ast_list.first().unwrap();
        let last_block = ast_list.last().unwrap();
        let span = first_block.location.span(&last_block.location);

        // Map the span, store the counter id in the lookup table (key at first_block.ast_id)
        let ctr_id = mapping_region_generator.add_mapping_region(&span);
        self.ast_counter_lookup.insert(first_block.id, ctr_id.try_into().unwrap());
    }

    pub fn get_increment_function(&self, module: &Module<'ink>) -> FunctionValue<'ink> {
        let increment_intrinsic = Intrinsic::find("llvm.instrprof.increment").unwrap();
        let increment_intrinsic_func = increment_intrinsic.get_declaration(module, &[]).unwrap();

        increment_intrinsic_func
    }
}
