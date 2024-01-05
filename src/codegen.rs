// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::{
    cell::RefCell,
    ffi::CString,
    ops::Deref,
    path::{Path, PathBuf},
};

/// module to generate llvm intermediate representation for a CompilationUnit
use self::{
    debug::{Debug, DebugBuilderEnum},
    generators::{
        data_type_generator,
        llvm::{GlobalValueExt, Llvm},
        pou_generator::{self, PouGenerator},
        variable_generator::VariableGenerator,
    },
    llvm_index::LlvmTypedIndex,
};
use crate::{
    output::FormatOption,
    resolver::{AstAnnotations, Dependency, StringLiterals},
    DebugLevel, OptimizationLevel, Target,
};

use super::index::*;
use indexmap::IndexSet;
use inkwell::{
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    memory_buffer::MemoryBuffer,
    types::BasicType,
};
use inkwell::{
    module::Module,
    passes::{PassBuilderOptions, PassManager, PassManagerBuilder},
    targets::{CodeModel, FileType, InitializationConfig, RelocMode},
};
use plc_ast::ast::{CompilationUnit, LinkageType};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use rustc_llvm_coverage::{
    self,
    types::{CounterExpression, CounterMappingRegion},
};

mod debug;
pub(crate) mod generators;
mod llvm_index;
mod llvm_typesystem;
#[cfg(test)]
mod tests;
use rustc_llvm_coverage::*;
use types::*;

/// A wrapper around the LLVM context to allow passing it without exposing the inkwell dependencies
pub struct CodegenContext(Context);

impl CodegenContext {
    pub fn create() -> Self {
        CodegenContext(Context::create())
    }
}

impl Deref for CodegenContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// the codegen struct carries all dependencies required to generate
/// the IR code for a compilation unit
pub struct CodeGen<'ink> {
    /// the module represents a llvm compilation unit
    pub module: Module<'ink>,
    /// the debugging module creates debug information at appropriate locations
    pub debug: DebugBuilderEnum<'ink>,

    pub module_location: String,
}

pub struct GeneratedModule<'ink> {
    module: Module<'ink>,
    engine: RefCell<Option<ExecutionEngine<'ink>>>,
}

type MainFunction<T, U> = unsafe extern "C" fn(*mut T) -> U;
type MainEmptyFunction<U> = unsafe extern "C" fn() -> U;

impl<'ink> CodeGen<'ink> {
    /// constructs a new code-generator that generates CompilationUnits into a module with the given module_name
    pub fn new(
        context: &'ink CodegenContext,
        root: Option<&Path>,
        module_location: &str,
        optimization_level: OptimizationLevel,
        debug_level: DebugLevel,
    ) -> CodeGen<'ink> {
        let module = context.create_module(module_location);
        module.set_source_file_name(module_location);
        let debug = debug::DebugBuilderEnum::new(context, &module, root, optimization_level, debug_level);

        // let mut pm = PassManager::create(());

        // let pass_manager_builder = PassManagerBuilder::create();
        // pass_manager_builder.populate_module_pass_manager(&pm);

        // unsafe {
        //     rustc_llvm_coverage::LLVMRustAddInstrumentationPass(pm.as_mut_ptr());
        // }
        // let did_init = pm.initialize();
        // println!("Did init: {}", did_init);
        // let did_finalize = pm.finalize();
        // println!("Did finalize: {:?}", did_finalize);

        CodeGen { module, debug, module_location: module_location.to_string() }
    }

    pub fn generate_llvm_index(
        &mut self,
        context: &'ink CodegenContext,
        annotations: &AstAnnotations,
        literals: &StringLiterals,
        dependencies: &IndexSet<Dependency>,
        global_index: &Index,
    ) -> Result<LlvmTypedIndex<'ink>, Diagnostic> {
        let llvm = Llvm::new(context, context.create_builder());
        let mut index = LlvmTypedIndex::default();
        //Generate types index, and any global variables associated with them.
        let llvm_type_index = data_type_generator::generate_data_types(
            &llvm,
            &mut self.debug,
            dependencies,
            global_index,
            annotations,
        )?;
        index.merge(llvm_type_index);

        let mut variable_generator =
            VariableGenerator::new(&self.module, &llvm, global_index, annotations, &index, &mut self.debug);

        //Generate global variables
        let llvm_gv_index =
            variable_generator.generate_global_variables(dependencies, &self.module_location)?;
        index.merge(llvm_gv_index);

        //Generate opaque functions for implementations and associate them with their types
        let llvm = Llvm::new(context, context.create_builder());
        let llvm_impl_index = pou_generator::generate_implementation_stubs(
            &self.module,
            llvm,
            dependencies,
            global_index,
            annotations,
            &index,
            &mut self.debug,
        )?;
        let llvm = Llvm::new(context, context.create_builder());
        index.merge(llvm_impl_index);
        let llvm_values_index = pou_generator::generate_global_constants_for_pou_members(
            &self.module,
            &llvm,
            dependencies,
            global_index,
            annotations,
            &index,
            &self.module_location,
        )?;
        index.merge(llvm_values_index);

        //Generate constants for string-literal
        //generate literals but first sort, so we get reproducable builds
        let mut utf08s = literals.utf08.iter().map(String::as_str).collect::<Vec<&str>>();
        utf08s.sort_unstable();
        for (idx, literal) in utf08s.into_iter().enumerate() {
            let len = literal.len() + 1;
            let data_type = llvm.context.i8_type().array_type(len as u32);
            let literal_variable = llvm.create_global_variable(
                &self.module,
                format!("utf08_literal_{idx}").as_str(),
                data_type.as_basic_type_enum(),
            );
            let initializer = llvm.create_const_utf8_string(literal, len)?;
            literal_variable.make_constant().make_private().set_initializer(&initializer);

            index.associate_utf08_literal(literal, literal_variable);
        }
        //generate literals but first sort, so we get reproducable builds
        let mut utf16s = literals.utf16.iter().map(String::as_str).collect::<Vec<&str>>();
        utf16s.sort_unstable();
        for (idx, literal) in utf16s.into_iter().enumerate() {
            let len = literal.len() + 1;
            let data_type = llvm.context.i16_type().array_type(len as u32);
            let literal_variable = llvm.create_global_variable(
                &self.module,
                format!("utf16_literal_{idx}").as_str(),
                data_type.as_basic_type_enum(),
            );
            let initializer = llvm.create_const_utf16_string(literal, literal.len() + 1)?;
            literal_variable.make_constant().make_private().set_initializer(&initializer);

            index.associate_utf16_literal(literal, literal_variable);
        }

        Ok(index)
    }

    /// generates all TYPEs, GLOBAL-sections and POUs of the given CompilationUnit
    pub fn generate(
        self,
        context: &'ink CodegenContext,
        unit: &CompilationUnit,
        annotations: &AstAnnotations,
        global_index: &Index,
        llvm_index: &LlvmTypedIndex,
    ) -> Result<GeneratedModule<'ink>, Diagnostic> {
        //generate all pous
        let llvm = Llvm::new(context, context.create_builder());
        let pou_generator = PouGenerator::new(llvm, global_index, annotations, llvm_index);

        //Generate the POU stubs in the first go to make sure they can be referenced.
        for implementation in &unit.implementations {
            //Don't generate external or generic functions
            if let Some(entry) = global_index.find_pou(implementation.name.as_str()) {
                if !entry.is_generic() && entry.get_linkage() != &LinkageType::External {
                    pou_generator.generate_implementation(implementation, &self.debug)?;
                }
            }
        }

        self.debug.finalize();
        log::debug!("{}", self.module.to_string());

        println!("Done generating POUs");
        println!("Cov mapping version: {}", rustc_llvm_coverage::mapping_version());
        println!("Hash string: {:#?}", rustc_llvm_coverage::hash_str("asdf"));
        let byte_string: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        println!("Hash bytes: {:#?}", rustc_llvm_coverage::hash_bytes(byte_string));

        let rust_string = rustc_llvm_coverage::types::RustString { bytes: RefCell::new(Vec::new()) };
        let filenames = vec![CString::new("test.c").unwrap()];
        rustc_llvm_coverage::write_filenames_section_to_buffer(&filenames, &rust_string);
        // print buffer
        // println!("Filenames: {:#?}", rust_string.bytes.borrow());
        // print buffer as hex string
        println!(
            "Filenames: {:#?}",
            rust_string
                .bytes
                .borrow()
                .iter()
                .map(|it| format!("{:02x}", it))
                .collect::<Vec<String>>()
                .join("")
        );

        // println!("{:#?}", llvm_index);
        // let prg_func: inkwell::values::FunctionValue<'_> =
        // llvm_index.find_associated_implementation("prg").expect("Unable to get prg");
        let prg_func = self.module.get_function("prg").expect("Unable to get prg");
        println!("prg: {:#?}", prg_func);
        let global_func_var = rustc_llvm_coverage::create_pgo_func_name_var(&prg_func);
        println!("global_func_var: {:#?}", global_func_var);

        let global_vars: Vec<_> = self.module.get_globals().collect();
        println!("global_vars: {:#?}", global_vars);
        //create write_mapping_to_buffer params
        //virtual_file_mapping
        let virtual_file_mapping: Vec<u32> = vec![0x0];

        let counter1 = Counter::counter_value_reference(CounterId::new(1));
        let counter2 = Counter::expression(ExpressionId::new(2));
        let counter3 = Counter::counter_value_reference(CounterId::new(3));

        // Creating a vector of CounterExpression instances
        let expressions: Vec<CounterExpression> = vec![
            CounterExpression::new(counter1, ExprKind::Add, counter2),
            CounterExpression::new(counter2, ExprKind::Subtract, counter3),
            // Add more CounterExpression instances as needed
        ];
        //mapping_regions
        let mapping_regions: Vec<CounterMappingRegion> = vec![
            CounterMappingRegion::code_region(counter1, 0, 0, 0, 3, 10),
            CounterMappingRegion::code_region(counter3, 0, 4, 0, 9, 10),
        ];
        //buffer
        let buffer = rustc_llvm_coverage::types::RustString { bytes: RefCell::new(Vec::new()) };
        write_mapping_to_buffer(virtual_file_mapping, expressions, mapping_regions, &buffer);
        //print the buffer
        //print the buffer
        println!(
            "Buffer: {:#?}",
            buffer.bytes.borrow().iter().map(|it| format!("{:02x}", it)).collect::<Vec<String>>().join("")
        );

        // cov mappping
        // let struct_type = context.opaque_struct_type("my_struct");
        let i32_type = context.i32_type();
        let i32_zero = i32_type.const_int(0, false);
        // TODO - generate this dynamically
        let i32_42 = i32_type.const_int(42, false);
        // TODO - replace this w/ cov mapping version
        let i32_5 = i32_type.const_int(5, false);

        let cov_data_header =
            context.const_struct(&[i32_zero.into(), i32_42.into(), i32_zero.into(), i32_5.into()], false);

        // TODO - generate this dynamically from cov mapping data
        let i8_type = context.i8_type();
        let i8_zero = i8_type.const_int(0, false);
        let cov_mapping_data = i8_type.const_array(&[i8_zero, i8_zero, i8_zero]);

        let cov_data_val = context.const_struct(&[cov_data_header.into(), cov_mapping_data.into()], false);

        rustc_llvm_coverage::save_cov_data_to_mod(&self.module, cov_data_val);

        // func record

        let i64_type = context.i64_type();
        let i64_zero = i64_type.const_int(0, false);
        let i64_one = i64_type.const_int(1, false);
        let func_mapping_data = i8_type.const_array(&[i8_zero, i8_zero, i8_zero]);
        let func_data_val = context.const_struct(
            &[i64_one.into(), i32_zero.into(), i64_zero.into(), i64_zero.into(), func_mapping_data.into()],
            false,
        );
        rustc_llvm_coverage::save_func_record_to_mod(&self.module, 0x1234, func_data_val, true);

        #[cfg(feature = "verify")]
        {
            self.module
                .verify()
                .map_err(|it| Diagnostic::GeneralError {
                    message: it.to_string(),
                    err_no: crate::diagnostics::ErrNo::codegen__general,
                })
                .map(|_| GeneratedModule { module: self.module, debug: self.debug })
        }

        #[cfg(not(feature = "verify"))]
        Ok(GeneratedModule { module: self.module, engine: RefCell::new(None) })
    }
}

impl<'ink> GeneratedModule<'ink> {
    pub fn try_from_bitcode(context: &'ink CodegenContext, path: &Path) -> Result<Self, Diagnostic> {
        let module = Module::parse_bitcode_from_path(path, context.deref())?;
        Ok(GeneratedModule { module, engine: RefCell::new(None) })
    }

    pub fn try_from_ir(context: &'ink CodegenContext, path: &Path) -> Result<Self, Diagnostic> {
        let buffer = MemoryBuffer::create_from_file(path)?;
        let module = context.create_module_from_ir(buffer)?;

        log::debug!("{}", module.to_string());

        Ok(GeneratedModule { module, engine: RefCell::new(None) })
    }

    pub fn merge(self, other: GeneratedModule<'ink>) -> Result<Self, Diagnostic> {
        self.module.link_in_module(other.module)?;
        log::debug!("Merged: {}", self.module.to_string());

        Ok(self)
    }

    /// Persists the module into the disk based on output and target requirments
    /// If an object file should be generated, all optimizations will be executed on the object
    pub fn persist(
        &self,
        output_dir: Option<&Path>,
        output_name: &str,
        format: FormatOption,
        target: &Target,
        optimization_level: OptimizationLevel,
    ) -> Result<PathBuf, Diagnostic> {
        let output = Self::get_output_file(output_dir, output_name, target);
        //ensure output exists
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        match format {
            FormatOption::Object | FormatOption::Relocatable => {
                self.persist_as_static_obj(output, target, optimization_level)
            }
            FormatOption::PIC | FormatOption::Shared | FormatOption::Static => {
                self.persist_to_shared_pic_object(output, target, optimization_level)
            }
            FormatOption::NoPIC => self.persist_to_shared_object(output, target, optimization_level),
            FormatOption::Bitcode => self.persist_to_bitcode(output),
            FormatOption::IR => self.persist_to_ir(output),
        }
    }

    fn get_output_file(output_dir: Option<&Path>, output_name: &str, target: &Target) -> PathBuf {
        let output_dir = output_dir.map(Path::to_path_buf).unwrap_or_else(|| PathBuf::from(""));
        let output = if let Some(name) = target.try_get_name() {
            output_dir.join(name).join(output_name)
        } else {
            output_dir.join(output_name)
        };
        output
    }

    ///
    /// Compiles the given source into an object file and saves it in output
    ///
    fn persist_to_obj(
        &self,
        output: PathBuf,
        reloc: RelocMode,
        target: &Target,
        optimization_level: OptimizationLevel,
    ) -> Result<PathBuf, Diagnostic> {
        let initialization_config = &InitializationConfig::default();
        inkwell::targets::Target::initialize_all(initialization_config);

        let triple = target.get_target_triple();

        let target = inkwell::targets::Target::from_triple(&triple).map_err(|it| {
            Diagnostic::codegen_error(
                &format!("Invalid target-tripple '{triple}' - {it:?}"),
                SourceLocation::undefined(),
            )
        })?;
        let machine = target
            .create_target_machine(
                &triple,
                //TODO : Add cpu features as optionals
                "generic", //TargetMachine::get_host_cpu_name().to_string().as_str(),
                "",        //TargetMachine::get_host_cpu_features().to_string().as_str(),
                optimization_level.into(),
                reloc,
                CodeModel::Default,
            )
            .ok_or_else(|| {
                Diagnostic::codegen_error("Cannot create target machine.", SourceLocation::undefined())
            });

        //Make sure all parents exist
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // Log passes
        println!("Optimization level: {:?}", optimization_level.opt_params());
        ////Run the passes
        machine
            .and_then(|it| {
                self.module
                    .run_passes(optimization_level.opt_params(), &it, PassBuilderOptions::create())
                    .map_err(|it| {
                        Diagnostic::llvm_error(output.to_str().unwrap_or_default(), &it.to_string())
                    })
                    .and_then(|_| {
                        it.write_to_file(&self.module, FileType::Object, output.as_path()).map_err(|it| {
                            Diagnostic::llvm_error(output.to_str().unwrap_or_default(), &it.to_string())
                        })
                    })
            })
            .map(|_| output)
    }

    /// Persists a given LLVM module to a static object and saves the output.
    ///
    /// # Arguments
    ///
    /// * `codegen` - The generated LLVM module to persist
    /// * `output` - the location on disk to save the output
    /// * `target` - an optional llvm target triple
    ///     If not provided, the machine's triple will be used.
    pub fn persist_as_static_obj(
        &self,
        output: PathBuf,
        target: &Target,
        optimization_level: OptimizationLevel,
    ) -> Result<PathBuf, Diagnostic> {
        self.persist_to_obj(output, RelocMode::Default, target, optimization_level)
    }

    /// Persists a given LLVM module to a shared postiion indepedent object and saves the output.
    ///
    /// # Arguments
    ///
    /// * `codegen` - The generated LLVM module to persist
    /// * `output` - the location on disk to save the output
    /// * `target` - an optional llvm target triple
    ///     If not provided, the machine's triple will be used.
    pub fn persist_to_shared_pic_object(
        &self,
        output: PathBuf,
        target: &Target,
        optimization_level: OptimizationLevel,
    ) -> Result<PathBuf, Diagnostic> {
        self.persist_to_obj(output, RelocMode::PIC, target, optimization_level)
    }

    /// Persists the given LLVM module to a dynamic non PIC object and saves the output.
    ///
    /// # Arguments
    ///
    /// * `codegen` - The generated LLVM module to persits
    /// * `output` - the location on disk to save the output
    /// * `target` - llvm target triple
    pub fn persist_to_shared_object(
        &self,
        output: PathBuf,
        target: &Target,
        optimization_level: OptimizationLevel,
    ) -> Result<PathBuf, Diagnostic> {
        self.persist_to_obj(output, RelocMode::DynamicNoPic, target, optimization_level)
    }

    ///
    /// Persists the given LLVM module into a bitcode file
    ///
    /// # Arguments
    ///
    /// * `codegen` - the genated LLVM module to persist
    /// * `output` - the location on disk to save the output
    pub fn persist_to_bitcode(&self, output: PathBuf) -> Result<PathBuf, Diagnostic> {
        if self.module.write_bitcode_to_path(&output) {
            Ok(output)
        } else {
            Err(Diagnostic::codegen_error("Could not write bitcode to file", SourceLocation::undefined()))
        }
    }

    ///
    /// Persits the given LLVM module into LLVM IR and saves it to the given output location
    ///
    /// # Arguments
    ///
    /// * `codegen` - The generated LLVM module to be persisted
    /// * `output`  - The location to save the generated ir file
    pub fn persist_to_ir(&self, output: PathBuf) -> Result<PathBuf, Diagnostic> {
        log::debug!("Output location: {}", output.to_string_lossy());
        log::debug!("{}", self.persist_to_string());

        println!("Writing to IR");

        // let pm = PassManager::create(());
        // unsafe {
        //     rustc_llvm_coverage::LLVMRustAddInstrumentationPass(pm.as_mut_ptr());
        // }
        // let did_run = pm.run_on(&self.module);
        // println!("Did run: {}", did_run);
        // let did_init = pm.initialize();
        // println!("Did init: {}", did_init);
        // let did_finalize = pm.finalize();
        // println!("Did finalize: {:?}", did_finalize);

        unsafe {
            rustc_llvm_coverage::LLVMRustRunInstrumentationPass(self.module.as_mut_ptr());
        }

        self.module
            .print_to_file(&output)
            .map_err(|err| {
                Diagnostic::io_write_error(output.to_str().unwrap_or_default(), err.to_string().as_str())
            })
            .map(|_| output)
    }

    ///
    /// Persists the given module to a string
    ///
    pub fn persist_to_string(&self) -> String {
        self.module.to_string()
    }

    ///
    /// Prints the content of the module to the stderr
    ///
    pub fn print_to_stderr(&self) {
        self.module.print_to_stderr();
    }

    ///
    /// Runs the function given by `name` inside the compiled module.
    /// Returns the value returned by calling the function
    ///
    pub fn run<T, U>(&self, name: &str, params: &mut T) -> U {
        let engine = self.get_execution_engine();

        unsafe {
            let main: JitFunction<MainFunction<T, U>> = engine.get_function(name).unwrap();
            let main_t_ptr = &mut *params as *mut _;

            main.call(main_t_ptr)
        }
    }

    ///
    /// Runs the function given by `name` inside the compiled module.
    /// Returns the value returned by calling the function
    ///
    pub fn run_no_param<U>(&self, name: &str) -> U {
        let engine = self.get_execution_engine();
        unsafe {
            let main: JitFunction<MainEmptyFunction<U>> = engine.get_function(name).unwrap();
            main.call()
        }
    }

    pub fn add_global_function_mapping(&self, function_name: &str, local_function: usize) {
        let engine = self.get_execution_engine();
        if let Some(function) = self.module.get_function(function_name) {
            engine.add_global_mapping(&function, local_function);
        } else {
            log::debug!("Function {} does not exist", function_name);
        }
    }

    pub fn add_global_variable_mapping(&self, global_name: &str, local_var: usize) {
        let engine = self.get_execution_engine();
        if let Some(function) = self.module.get_global(global_name) {
            engine.add_global_mapping(&function, local_var);
        } else {
            log::debug!("Global {} does not exist", global_name);
        }
    }

    fn get_execution_engine(&self) -> ExecutionEngine<'ink> {
        if let Some(engine) = self.engine.borrow().as_ref() {
            return engine.clone();
        }
        //Create engine
        let engine = self.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap();
        *self.engine.borrow_mut() = Some(engine.clone());
        engine
    }
}

#[cfg(test)]
mod casting_big_numbers {
    #[test]
    fn casting_between_i128_and_u64() {
        let n: i128 = u64::MAX as i128;
        let nn: u64 = n as u64;
        assert_eq!(0xFFFF_FFFF_FFFF_FFFF_u64, nn);

        let n: i128 = i64::MAX as i128;
        let nn: u64 = n as u64;
        assert_eq!(0x7FFF_FFFF_FFFF_FFFF_u64, nn);
    }
}
