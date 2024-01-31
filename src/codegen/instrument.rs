use super::LlvmTypedIndex;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::intrinsics::Intrinsic;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, GlobalValue};
use plc_ast::ast::{AstId, AstNode, AstStatement, CompilationUnit, Implementation, LinkageType};
use plc_ast::control_statements::AstControlStatement;
use plc_source::source_location::{CodeSpan, SourceLocation};
use rustc_llvm_coverage::types::{
    Counter, CounterExpression, CounterId, CounterMappingRegion, ExprKind, ExpressionId,
};
use rustc_llvm_coverage::*;
use std::collections::HashMap;
use std::ffi::CString;

pub struct CoverageInstrumentationBuilder<'ink> {
    context: &'ink Context,
    // module: &'ink Module<'ink>,
    files: Vec<String>,
    cov_mapping_header: Option<CoverageMappingHeader>,
    function_pgos: HashMap<String, (FunctionRecord, GlobalValue<'ink>)>,
    // TODO - better counter datastructures
    // ast_counter_lookup:
    // - if statements: map first body block -> true branch counter
    // - case statements: map first body block -> true branch counter
    // - for statements: map first body block -> true branch counter AND end block -> false branch counter
    // - while statements: map first body block -> true branch counter AND condition block -> false branch counter
    // - repeat statements: map first body block -> true branch counter AND condition block -> false branch counter
    ast_counter_lookup: HashMap<AstId, usize>,
}

/// Manages the creation of mapping regions for a given function
#[derive(Debug)]
struct MappingRegionGenerator {
    // TODO - verify that counter ids are PER function
    pub mapping_regions: Vec<CounterMappingRegion>,
    pub expressions: Vec<CounterExpression>,
    next_counter_id: u32,
    next_expression_id: u32,
    file_id: u32,
}

impl MappingRegionGenerator {
    pub fn new(file_id: u32) -> Self {
        Self {
            mapping_regions: Vec::new(),
            expressions: Vec::new(),
            next_counter_id: 0,
            next_expression_id: 0,
            file_id,
        }
    }

    /// Adds to internal index and returns for convenience
    pub fn add_code_mapping_region(&mut self, source: &SourceLocation) -> CounterMappingRegion {
        let (start_line, start_col, end_line, end_col) = source.get_start_end();

        let counter_id = self.next_counter_id;
        let counter = Counter::counter_value_reference(CounterId::new(counter_id));
        self.next_counter_id += 1;

        let mapping_region = CounterMappingRegion::code_region(
            counter,
            self.file_id,
            start_line.try_into().unwrap(),
            start_col.try_into().unwrap(),
            end_line.try_into().unwrap(),
            end_col.try_into().unwrap(),
        );
        self.mapping_regions.push(mapping_region.clone());

        mapping_region
    }

    // TODO - consolidate the two below functions
    /// Adds to internal index and returns for convenience
    /// Specific to if statements and case statements
    pub fn add_branch_mapping_region(
        &mut self,
        source: &SourceLocation,
        last_false_counter: Counter,
    ) -> CounterMappingRegion {
        let (start_line, start_col, end_line, end_col) = source.get_start_end();

        // Counts branch executions
        let counter_id = self.next_counter_id;
        let counter = Counter::counter_value_reference(CounterId::new(counter_id));
        self.next_counter_id += 1;

        // Count the branch skips (when cond evalutes to false using a_{n-1} - Counter)
        let false_counter_id = self.next_expression_id;
        let false_counter = Counter::expression(ExpressionId::new(false_counter_id));
        let false_counter_expression =
            CounterExpression::new(last_false_counter, ExprKind::Subtract, counter);
        self.expressions.push(false_counter_expression);
        self.next_expression_id += 1;

        let mapping_region = CounterMappingRegion::branch_region(
            counter,
            false_counter,
            self.file_id,
            start_line.try_into().unwrap(),
            start_col.try_into().unwrap(),
            end_line.try_into().unwrap(),
            end_col.try_into().unwrap(),
        );
        self.mapping_regions.push(mapping_region.clone());

        // Return the index of the counter id added
        mapping_region
    }

    pub fn add_loop_branch_mapping_region(
        &mut self,
        source: &SourceLocation,
        false_counter: Counter,
    ) -> CounterMappingRegion {
        let (start_line, start_col, end_line, end_col) = source.get_start_end();

        // Counts branch executions
        let counter_id = self.next_counter_id;
        let counter = Counter::counter_value_reference(CounterId::new(counter_id));
        self.next_counter_id += 1;

        let mapping_region = CounterMappingRegion::branch_region(
            counter,
            false_counter,
            self.file_id,
            start_line.try_into().unwrap(),
            start_col.try_into().unwrap(),
            end_line.try_into().unwrap(),
            end_col.try_into().unwrap(),
        );
        self.mapping_regions.push(mapping_region.clone());

        // Return the index of the counter id added
        mapping_region
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
                // TODO - figure out why this happens
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
        // TODO - file mapping table
        let file_id = 1;

        // Map entire function
        let mut mapping_region_generator = MappingRegionGenerator::new(file_id);
        let func_map_region = mapping_region_generator.add_code_mapping_region(&implementation.location);
        assert!(func_map_region.counter.id == 0);

        // DFS function statements
        self.generate_coverage_records(
            &implementation.statements,
            &mut mapping_region_generator,
            func_map_region.counter,
        );

        // TODO - determine if function is used
        let is_used = true;
        let written_coverage_header = &self.cov_mapping_header.as_mut().unwrap();

        FunctionRecord::new(
            func_name,
            struct_hash,
            func_filenames,
            mapping_region_generator.expressions,
            mapping_region_generator.mapping_regions,
            is_used,
            &written_coverage_header,
        )
    }

    /// DFS algorithm to parse AST and generate coverage records for all branching
    /// `parent_counter_id` is the counter id of the parent node, used for calculating "false" branches under if/case statements
    /// TODO - explain or diagram what's going on here with
    /// - last_false_counter: for chain branches
    /// - last_true_counter: for recursing
    fn generate_coverage_records(
        &mut self,
        ast_node_list: &Vec<AstNode>,
        mapping_region_generator: &mut MappingRegionGenerator,
        parent_counter: Counter,
    ) {
        for ast_node in ast_node_list {
            // Only generate coverage records for control statements
            let control_statement = match &ast_node.stmt {
                AstStatement::ControlStatement(statement) => statement,
                _ => continue,
            };

            // Track last counter (a_{n-1}) - useful for false branch calculations
            // Must be initialized to parent counter in first iteration
            let mut last_true_counter = parent_counter;
            let mut last_false_counter = parent_counter;

            //
            match control_statement {
                AstControlStatement::If(statement) => {
                    // Loop through if/elif blocks
                    for block in &statement.blocks {
                        // Setup ast->id mapping, store region location
                        (last_true_counter, last_false_counter) = self.register_ast_list_as_branch_region(
                            &block.body,
                            mapping_region_generator,
                            last_true_counter,
                            last_false_counter,
                        );
                        // Recurse into child blocks
                        self.generate_coverage_records(
                            &block.body,
                            mapping_region_generator,
                            last_true_counter,
                        );
                    }

                    // Else block ast->id mapping
                    (last_true_counter, last_false_counter) = self.register_ast_list_as_branch_region(
                        &statement.else_block,
                        mapping_region_generator,
                        last_true_counter,
                        last_false_counter,
                    );
                    // Recurse into child blocks
                    self.generate_coverage_records(
                        &statement.else_block,
                        mapping_region_generator,
                        last_true_counter,
                    );
                }
                AstControlStatement::ForLoop(statement) => {
                    // Loop through for loop body
                    (last_true_counter, last_false_counter) = self.register_ast_list_as_loop_branch_region(
                        &statement.body,
                        &statement.end,
                        mapping_region_generator,
                        last_true_counter,
                        last_false_counter,
                    );
                    self.generate_coverage_records(
                        &statement.body,
                        mapping_region_generator,
                        last_true_counter,
                    );
                }
                AstControlStatement::WhileLoop(statement) => {
                    // Loop through while loop body
                    (last_true_counter, last_false_counter) = self.register_ast_list_as_loop_branch_region(
                        &statement.body,
                        &statement.condition,
                        mapping_region_generator,
                        last_true_counter,
                        last_false_counter,
                    );
                    self.generate_coverage_records(
                        &statement.body,
                        mapping_region_generator,
                        last_true_counter,
                    );
                }
                AstControlStatement::RepeatLoop(statement) => {
                    // Loop through while loop body
                    (last_true_counter, last_false_counter) = self.register_ast_list_as_loop_branch_region(
                        &statement.body,
                        &statement.condition,
                        mapping_region_generator,
                        last_true_counter,
                        last_false_counter,
                    );
                    self.generate_coverage_records(
                        &statement.body,
                        mapping_region_generator,
                        last_true_counter,
                    );
                }
                AstControlStatement::Case(statement) => {
                    // Loop through case blocks
                    for block in &statement.case_blocks {
                        // Setup ast->id mapping, store region location
                        (last_true_counter, last_false_counter) = self.register_ast_list_as_branch_region(
                            &block.body,
                            mapping_region_generator,
                            last_true_counter,
                            last_false_counter,
                        );
                        // Recurse
                        self.generate_coverage_records(
                            &block.body,
                            mapping_region_generator,
                            last_true_counter,
                        );
                    }

                    // Else block
                    (last_true_counter, last_false_counter) = self.register_ast_list_as_branch_region(
                        &statement.else_block,
                        mapping_region_generator,
                        last_true_counter,
                        last_false_counter,
                    );
                    self.generate_coverage_records(
                        &statement.else_block,
                        mapping_region_generator,
                        last_true_counter,
                    );
                }
            }
        }
    }

    // TODO - find me a better name
    /// Registers a Vec<AstNode> as a region, spanning first and last
    /// Returns the true and false counters for DFS
    fn register_ast_list_as_branch_region(
        &mut self,
        ast_list: &Vec<AstNode>,
        mapping_region_generator: &mut MappingRegionGenerator,
        last_true_counter: Counter,
        last_false_counter: Counter,
    ) -> (Counter, Counter) {
        if ast_list.is_empty() {
            return (last_true_counter, last_false_counter);
        }

        // Create a span from first_block -> last_block
        let first_block = ast_list.first().unwrap();
        let last_block = ast_list.last().unwrap();
        let span = first_block.location.span(&last_block.location);

        // Map the span, store the counter id in the lookup table (key at first_block.ast_id)
        let mapping_region = mapping_region_generator.add_branch_mapping_region(&span, last_false_counter);
        self.ast_counter_lookup.insert(first_block.id, mapping_region.counter.id.try_into().unwrap());

        (mapping_region.counter, mapping_region.false_counter)
    }

    // TODO - find a better name
    fn register_ast_list_as_loop_branch_region(
        &mut self,
        loop_body_ast_list: &Vec<AstNode>,
        loop_condition_ast: &AstNode,
        mapping_region_generator: &mut MappingRegionGenerator,
        last_true_counter: Counter,
        last_false_counter: Counter,
    ) -> (Counter, Counter) {
        if loop_body_ast_list.is_empty() {
            return (last_true_counter, last_false_counter);
        }

        // Create a counter for the condition ast
        // this is a temporary hack, because false_counter will not actually create
        // a counter that doesn't exist, only reference once
        let condition_mapping_region =
            mapping_region_generator.add_code_mapping_region(&loop_condition_ast.location);

        // Create a span from first_block -> last_block
        let first_block = loop_body_ast_list.first().unwrap();
        let last_block = loop_body_ast_list.last().unwrap();
        let span = first_block.location.span(&last_block.location);

        // Map the span, store the counter id in the lookup table (key at first_block.ast_id)
        let mapping_region =
            mapping_region_generator.add_loop_branch_mapping_region(&span, condition_mapping_region.counter);

        // Map loop body -> true branch counter
        // Map loop condition -> false branch counter
        self.ast_counter_lookup.insert(first_block.id, mapping_region.counter.id.try_into().unwrap());
        self.ast_counter_lookup
            .insert(loop_condition_ast.id, mapping_region.false_counter.id.try_into().unwrap());

        (mapping_region.counter, mapping_region.false_counter)
    }

    pub fn get_increment_function(&self, module: &Module<'ink>) -> FunctionValue<'ink> {
        let increment_intrinsic = Intrinsic::find("llvm.instrprof.increment").unwrap();
        let increment_intrinsic_func = increment_intrinsic.get_declaration(module, &[]).unwrap();

        increment_intrinsic_func
    }
}
