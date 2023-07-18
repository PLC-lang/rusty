// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::{
    cell::RefCell,
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
    diagnostics::Diagnostic,
    output::FormatOption,
    resolver::{AstAnnotations, Dependency, StringLiterals},
    DebugLevel, OptimizationLevel, Target,
};

use super::ast::*;
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
    passes::PassBuilderOptions,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode},
};

mod debug;
pub(crate) mod generators;
mod llvm_index;
mod llvm_typesystem;
#[cfg(test)]
mod tests;

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
                    pou_generator.generate_implementation(implementation, &self.debug, &unit.new_lines)?;
                }
            }
        }

        self.debug.finalize();
        log::debug!("{}", self.module.to_string());

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
        module.print_to_stderr();

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
            FormatOption::PIC | FormatOption::Static => {
                self.persist_to_shared_pic_object(output, target, optimization_level)
            }
            FormatOption::Shared => self.persist_to_shared_object(output, target, optimization_level),
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
                SourceRange::undefined(),
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
                Diagnostic::codegen_error("Cannot create target machine.", SourceRange::undefined())
            });

        //Make sure all parents exist
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        ////Run the passes
        machine
            .and_then(|it| {
                self.module
                    .run_passes(optimization_level.opt_params(), &it, PassBuilderOptions::create())
                    .map_err(|it| Diagnostic::llvm_error(output.to_str().unwrap_or_default(), &it))
                    .and_then(|_| {
                        it.write_to_file(&self.module, FileType::Object, output.as_path())
                            .map_err(|it| Diagnostic::llvm_error(output.to_str().unwrap_or_default(), &it))
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
            Err(Diagnostic::codegen_error("Could not write bitcode to file", SourceRange::undefined()))
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
