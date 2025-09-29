// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Mutex,
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
    DebugLevel, OnlineChange, OptimizationLevel, Target,
};

use super::index::*;

use inkwell::{
    builder::BuilderError,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    memory_buffer::MemoryBuffer,
    support::LLVMString,
    types::BasicType,
    values::BasicValue,
    AddressSpace,
};
use inkwell::{
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode},
    types::BasicTypeEnum,
};
use plc_ast::ast::{CompilationUnit, LinkageType};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{FileMarker, SourceLocation};

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
    /// Whether we are generating a hot-reloadable binary or not
    pub online_change: OnlineChange,

    pub module_location: String,
}

pub struct GeneratedModule<'ink> {
    module: Module<'ink>,
    location: PathBuf,
    engine: RefCell<Option<ExecutionEngine<'ink>>>,
}

type MainFunction<T, U> = unsafe extern "C" fn(*mut T) -> U;
type MainEmptyFunction<U> = unsafe extern "C" fn() -> U;

impl<'ink> CodeGen<'ink> {
    /// constructs a new code-generator that generates CompilationUnits into a module with the given module_name
    pub fn new(
        context: &'ink CodegenContext,
        root: Option<&Path>,
        file_marker: FileMarker,
        optimization_level: OptimizationLevel,
        debug_level: DebugLevel,
        online_change: OnlineChange,
        target: &Target,
    ) -> CodeGen<'ink> {
        let module_location = file_marker.get_name().unwrap_or_default();
        let module = context.create_module(module_location);
        module.set_source_file_name(module_location);

        // Initialize all targets
        let initialization_config = &InitializationConfig::default();
        inkwell::targets::Target::initialize_all(initialization_config);
        let triple = target.get_target_triple();

        // Create target from triple
        let target_obj =
            inkwell::targets::Target::from_triple(&triple).expect("Failed to create target from triple");

        // Create a target machine with default options
        let target_machine = target_obj
            .create_target_machine(
                &triple,
                "generic", // CPU features - generic for portability
                "",        // CPU features - empty string for default
                optimization_level.into(),
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .expect("Failed to create target machine");

        // Get the data layout from the target machine and set the module's data layout and triple
        let target_data = target_machine.get_target_data();
        module.set_data_layout(&target_data.get_data_layout());
        module.set_triple(&triple);

        let debug_level = if file_marker.is_internal() { DebugLevel::None } else { debug_level };
        let debug = debug::DebugBuilderEnum::new(context, &module, root, optimization_level, debug_level);
        CodeGen { module, debug, module_location: module_location.to_string(), online_change }
    }

    pub fn generate_llvm_index(
        &mut self,
        context: &'ink CodegenContext,
        annotations: &AstAnnotations,
        literals: &StringLiterals,
        dependencies: &FxIndexSet<Dependency>,
        global_index: &Index,
        got_layout: &Mutex<HashMap<String, u64>>,
    ) -> Result<LlvmTypedIndex<'ink>, CodegenError> {
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

        let mut variable_generator = VariableGenerator::new(
            &self.module,
            &llvm,
            global_index,
            annotations,
            &index,
            &mut self.debug,
            &self.online_change,
        );

        //Generate global variables
        let llvm_gv_index =
            variable_generator.generate_global_variables(dependencies, &self.module_location)?;
        index.merge(llvm_gv_index);

        // Build our GOT layout here. We need to find all the names for globals, programs, and
        // functions and assign them indices in the GOT, taking into account prior indices.
        let program_globals =
            global_index.get_program_instances().into_iter().fold(Vec::new(), |mut acc, p| {
                acc.push(p.get_name().to_owned());
                acc.push(p.get_qualified_name().to_owned());
                acc.push(format!("{}_instance", p.get_name()));
                acc
            });

        let functions = global_index.get_pous().values().filter_map(|p| match p {
            PouIndexEntry::Function { name, linkage: LinkageType::Internal, is_generated: false, .. }
            | PouIndexEntry::FunctionBlock { name, linkage: LinkageType::Internal, .. } => {
                Some(String::from(name))
            }
            _ => None,
        });
        let all_names = global_index
            .get_globals()
            .values()
            .map(VariableIndexEntry::get_qualified_name)
            .map(String::from)
            .chain(program_globals)
            .chain(functions)
            .map(|s| s.to_lowercase())
            .map(|s| (crate::index::get_initializer_name(&s), s))
            .fold(Vec::new(), |mut acc, (s, s1)| {
                acc.push(s);
                acc.push(s1);
                acc
            });

        if self.online_change.is_enabled() {
            let got_entries = &mut *got_layout.lock().unwrap();

            let mut new_symbols = Vec::new();
            let mut new_got_entries = HashMap::new();
            let mut new_got = HashMap::new();

            for name in all_names {
                if let Some(idx) = got_entries.get(&name.to_string()) {
                    new_got_entries.insert(name.to_string(), *idx);
                    index.associate_got_index(&name, *idx)?;
                    new_got.insert(*idx, name.to_string());
                } else {
                    new_symbols.push(name.to_string());
                }
            }

            // Put any names that weren't there last time in any free space in the GOT.
            let mut idx: u64 = 0;
            for name in &new_symbols {
                while new_got.contains_key(&idx) {
                    idx += 1;
                }
                new_got_entries.insert(name.to_string(), idx);
                index.associate_got_index(name, idx)?;
                new_got.insert(idx, name.to_string());
            }

            // Construct our GOT as a new global array. We initialise this array in the loader code.
            let got_size: u32 = new_got
                .keys()
                .max()
                .map_or(0, |m| *m + 1)
                .try_into()
                .expect("the computed custom GOT size is too large");

            let ptr_ty = llvm.context.i8_type().ptr_type(AddressSpace::default());
            let empty_got = ptr_ty
                .const_array(vec![ptr_ty.const_null(); got_size as usize].as_slice())
                .as_basic_value_enum();
            let custom_got_ty =
                BasicTypeEnum::ArrayType(Llvm::get_array_type(BasicTypeEnum::PointerType(ptr_ty), got_size));

            let custom_got = llvm.create_global_variable(&self.module, "__custom_got", custom_got_ty);
            custom_got.set_linkage(inkwell::module::Linkage::WeakODR);
            custom_got.set_initial_value(Some(empty_got), custom_got_ty);

            *got_entries = new_got_entries;
        }

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
            &self.online_change,
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
        llvm_index: LlvmTypedIndex<'ink>,
    ) -> Result<GeneratedModule<'ink>, CodegenError> {
        //generate all pous
        let llvm = Llvm::new(context, context.create_builder());
        let pou_generator =
            PouGenerator::new(llvm, global_index, annotations, &llvm_index, &self.online_change);

        //Generate the POU stubs in the first go to make sure they can be referenced.
        for implementation in &unit.implementations {
            //Don't generate external or generic functions
            if let Some(entry) = global_index.find_pou(implementation.name.as_str()) {
                if !entry.is_generic() && entry.get_linkage() != &LinkageType::External {
                    pou_generator.generate_implementation(implementation, &self.debug)?;
                }
            }
        }

        let location = (&unit.file).into();

        self.debug.finalize();
        log::trace!("{}", self.module.to_string());

        #[cfg(feature = "verify")]
        {
            self.module.verify().map_err(|it| CodegenError::from(it)).map(|_| GeneratedModule {
                module: self.module,
                location,
                engine: RefCell::new(None),
            })
        }

        #[cfg(not(feature = "verify"))]
        Ok(GeneratedModule { module: self.module, location, engine: RefCell::new(None) })
    }
}

impl<'ink> GeneratedModule<'ink> {
    pub fn try_from_bitcode(context: &'ink CodegenContext, path: &Path) -> Result<Self, CodegenError> {
        let module = Module::parse_bitcode_from_path(path, context.deref())?;
        Ok(GeneratedModule { module, location: path.into(), engine: RefCell::new(None) })
    }

    pub fn from_memory(
        context: &'ink CodegenContext,
        buffer: &MemoryBuffer,
        path: &Path,
    ) -> Result<Self, CodegenError> {
        let module = Module::parse_bitcode_from_buffer(buffer, context.deref())?;
        Ok(GeneratedModule { module, location: path.into(), engine: RefCell::new(None) })
    }

    pub fn try_from_ir(context: &'ink CodegenContext, path: &Path) -> Result<Self, CodegenError> {
        let buffer = MemoryBuffer::create_from_file(path)?;
        let module = context.create_module_from_ir(buffer)?;

        log::trace!("{}", module.to_string());

        Ok(GeneratedModule { module, location: path.into(), engine: RefCell::new(None) })
    }

    pub fn merge(self, other: GeneratedModule<'ink>) -> Result<Self, CodegenError> {
        self.module.link_in_module(other.module)?;
        log::trace!("Merged: {}", self.module.to_string());

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
    ) -> Result<PathBuf, CodegenError> {
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

    pub fn get_unit_location(&self) -> &Path {
        &self.location
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
    ) -> Result<PathBuf, CodegenError> {
        let initialization_config = &InitializationConfig::default();
        inkwell::targets::Target::initialize_all(initialization_config);

        let triple = target.get_target_triple();

        let target = inkwell::targets::Target::from_triple(&triple)?;
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
            .ok_or_else(|| CodegenError::new("Cannot create target machine.", SourceLocation::undefined()));

        //Make sure all parents exist
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        ////Run the passes
        machine
            .and_then(|it| {
                self.module
                    .run_passes(optimization_level.opt_params(), &it, PassBuilderOptions::create())
                    .map_err(Into::into)
                    .and_then(|_| {
                        it.write_to_file(&self.module, FileType::Object, output.as_path()).map_err(Into::into)
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
    /// * `target` - an optional llvm target triple; if not provided, the machine's triple will be used.
    pub fn persist_as_static_obj(
        &self,
        output: PathBuf,
        target: &Target,
        optimization_level: OptimizationLevel,
    ) -> Result<PathBuf, CodegenError> {
        self.persist_to_obj(output, RelocMode::Default, target, optimization_level)
    }

    /// Persists a given LLVM module to a shared postiion indepedent object and saves the output.
    ///
    /// # Arguments
    ///
    /// * `codegen` - The generated LLVM module to persist
    /// * `output` - the location on disk to save the output
    /// * `target` - an optional llvm target triple; if not provided, the machine's triple will be used.
    pub fn persist_to_shared_pic_object(
        &self,
        output: PathBuf,
        target: &Target,
        optimization_level: OptimizationLevel,
    ) -> Result<PathBuf, CodegenError> {
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
    ) -> Result<PathBuf, CodegenError> {
        self.persist_to_obj(output, RelocMode::DynamicNoPic, target, optimization_level)
    }

    ///
    /// Persists the given LLVM module into a bitcode file
    ///
    /// # Arguments
    ///
    /// * `codegen` - the genated LLVM module to persist
    /// * `output` - the location on disk to save the output
    pub fn persist_to_bitcode(&self, output: PathBuf) -> Result<PathBuf, CodegenError> {
        if self.module.write_bitcode_to_path(&output) {
            Ok(output)
        } else {
            Err(CodegenError::new("Could not write bitcode to file", SourceLocation::undefined()))
        }
    }

    pub fn to_in_memory_bitcode(&self) -> Result<MemoryBuffer, CodegenError> {
        Ok(self.module.write_bitcode_to_memory())
    }

    ///
    /// Persits the given LLVM module into LLVM IR and saves it to the given output location
    ///
    /// # Arguments
    ///
    /// * `codegen` - The generated LLVM module to be persisted
    /// * `output`  - The location to save the generated ir file
    pub fn persist_to_ir(&self, output: PathBuf) -> Result<PathBuf, CodegenError> {
        log::trace!("Output location: {}", output.to_string_lossy());
        log::trace!("{}", self.persist_to_string());

        self.module.print_to_file(&output).map_err(Into::into).map(|_| output)
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

    pub fn set_name(&self, name: &str) {
        self.module.set_name(name);
        self.module.set_source_file_name(name);
    }
}

#[derive(Debug)]
pub enum CodegenError {
    GenericError(String, SourceLocation),
    IoError(std::io::Error),
    DiagnosticError(Diagnostic),
    BuilderError(BuilderError),
}

impl From<BuilderError> for CodegenError {
    fn from(err: BuilderError) -> Self {
        let bt = std::backtrace::Backtrace::force_capture();
        eprintln!("LLVM Builder Error: {:?}\n{bt}", err);
        CodegenError::BuilderError(err)
    }
}

impl From<std::io::Error> for CodegenError {
    fn from(err: std::io::Error) -> Self {
        CodegenError::IoError(err)
    }
}

impl From<Diagnostic> for CodegenError {
    fn from(err: Diagnostic) -> Self {
        CodegenError::DiagnosticError(err)
    }
}

impl From<String> for CodegenError {
    fn from(err: String) -> Self {
        CodegenError::GenericError(err, SourceLocation::undefined())
    }
}

impl CodegenError {
    pub fn new<T, U>(msg: T, location: U) -> Self
    where
        T: Into<String>,
        U: Into<SourceLocation>,
    {
        CodegenError::GenericError(msg.into(), location.into())
    }

    fn missing_function(location: SourceLocation) -> Self {
        CodegenError::new("Cannot generate code outside of function context.", location)
    }
}

impl From<LLVMString> for CodegenError {
    fn from(err: LLVMString) -> Self {
        CodegenError::GenericError(err.to_string_lossy().to_string(), SourceLocation::undefined())
    }
}

impl Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodegenError::GenericError(err, location) => {
                write!(f, "{}", err)?;
                if !location.is_undefined() {
                    write!(f, " at {}", location)?;
                }
                Ok(())
            }
            CodegenError::DiagnosticError(err) => write!(f, "{}", err),
            CodegenError::BuilderError(err) => write!(f, "{}", err),
            CodegenError::IoError(err) => write!(f, "{}", err),
        }
    }
}

impl From<CodegenError> for Diagnostic {
    fn from(err: CodegenError) -> Self {
        if let CodegenError::DiagnosticError(diagnostic) = err {
            return diagnostic;
        }
        let bt = std::backtrace::Backtrace::force_capture();
        eprintln!("Codegen Error: {:?}\n{bt}", err);
        Diagnostic::new(format!("Builder error: {err}"))
            .with_error_code("E002")
            .with_internal_error(anyhow::anyhow!(err))
    }
}

impl std::error::Error for CodegenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CodegenError::GenericError(_, _) => None,
            CodegenError::BuilderError(err) => Some(err),
            CodegenError::DiagnosticError(err) => Some(err),
            CodegenError::IoError(err) => Some(err),
        }
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
