// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// offers operations to generate global variables
use crate::{
    codegen::{debug::Debug, llvm_index::LlvmTypedIndex, llvm_typesystem::cast_if_needed},
    index::{get_initializer_name, Index, PouIndexEntry, VariableIndexEntry},
    resolver::{AnnotationMap, AstAnnotations, Dependency},
    ConfigFormat,
};
use indexmap::IndexSet;
use inkwell::{module::Module, types::BasicTypeEnum, values::GlobalValue};
use plc_ast::ast::LinkageType;
use plc_diagnostics::diagnostics::Diagnostic;
use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::path::Path;

use super::{
    data_type_generator::get_default_for,
    expression_generator::ExpressionCodeGenerator,
    llvm::{GlobalValueExt, Llvm},
    section_names,
};
use crate::codegen::debug::DebugBuilderEnum;
use crate::index::FxIndexSet;

pub fn read_got_layout(location: &str, format: ConfigFormat) -> Result<HashMap<String, u64>, Diagnostic> {
    if !Path::new(location).is_file() {
        // Assume if the file doesn't exist that there is no existing GOT layout yet. write_got_layout will handle
        // creating our file when we want to.
        return Ok(HashMap::new());
    }

    let s =
        read_to_string(location).map_err(|_| Diagnostic::new("GOT layout could not be read from file"))?;
    match format {
        ConfigFormat::JSON => serde_json::from_str(&s)
            .map_err(|_| Diagnostic::new("Could not deserialize GOT layout from JSON")),
        ConfigFormat::TOML => {
            toml::de::from_str(&s).map_err(|_| Diagnostic::new("Could not deserialize GOT layout from TOML"))
        }
    }
}

pub fn write_got_layout(
    got_entries: HashMap<String, u64>,
    location: &str,
    format: ConfigFormat,
) -> Result<(), Diagnostic> {
    let s = match format {
        ConfigFormat::JSON => serde_json::to_string(&got_entries)
            .map_err(|_| Diagnostic::new("Could not serialize GOT layout to JSON"))?,
        ConfigFormat::TOML => toml::ser::to_string(&got_entries)
            .map_err(|_| Diagnostic::new("Could not serialize GOT layout to TOML"))?,
    };

    write(location, s).map_err(|_| Diagnostic::new("GOT layout could not be written to file"))?;
    Ok(())
}

pub struct VariableGenerator<'ctx, 'b> {
    module: &'b Module<'ctx>,
    llvm: &'b Llvm<'ctx>,
    global_index: &'b Index,
    annotations: &'b AstAnnotations,
    types_index: &'b LlvmTypedIndex<'ctx>,
    debug: &'b mut DebugBuilderEnum<'ctx>,
    got_layout_file: Option<(String, ConfigFormat)>,
}

impl<'ctx, 'b> VariableGenerator<'ctx, 'b> {
    pub fn new(
        module: &'b Module<'ctx>,
        llvm: &'b Llvm<'ctx>,
        global_index: &'b Index,
        annotations: &'b AstAnnotations,
        types_index: &'b LlvmTypedIndex<'ctx>,
        debug: &'b mut DebugBuilderEnum<'ctx>,
        got_layout_file: Option<(String, ConfigFormat)>,
    ) -> Self {
        VariableGenerator { module, llvm, global_index, annotations, types_index, debug, got_layout_file }
    }

    pub fn generate_global_variables(
        &mut self,
        dependencies: &FxIndexSet<Dependency>,
        location: &'b str,
    ) -> Result<LlvmTypedIndex<'ctx>, Diagnostic> {
        let mut index = LlvmTypedIndex::default();

        let mut globals = vec![];
        dependencies.iter().for_each(|dep| {
            if let Some(dep) = match dep {
                Dependency::Datatype(name) => {
                    //Attempt to find a pou with that name
                    if let Some(PouIndexEntry::Program { instance_variable, .. }) =
                        self.global_index.find_pou(name).as_ref()
                    {
                        Some((name.as_str(), instance_variable.as_ref()))
                    } else {
                        None
                    }
                }
                Dependency::Variable(name) => {
                    self.global_index.find_fully_qualified_variable(name).map(|it| (name.as_str(), it))
                }
                Dependency::Call(_) => None,
            } {
                globals.push(dep);
            }

            if let Some(init) =
                self.global_index.find_global_initializer(&get_initializer_name(dep.get_name()))
            {
                globals.push((init.get_name(), init));
            }
        });

        for (name, variable) in &globals {
            let linkage =
                if !variable.is_in_unit(location) { LinkageType::External } else { variable.get_linkage() };
            let global_variable = self.generate_global_variable(variable, linkage).map_err(|err| {
                match err.get_error_code() {
                    //If we encounter a missing function or an invalid reference, we wrap it in a more generic issue
                    "E072" | "E048" => {
                        Diagnostic::new(format!("Cannot generate literal initializer for `{name}`."))
                            .with_error_code("E041")
                            .with_sub_diagnostic(err)
                    }
                    _ => err,
                }
            })?;
            index.associate_global(name, global_variable)?;
            //generate debug info
            self.debug.create_global_variable(
                variable.get_qualified_name(),
                &variable.data_type_name,
                global_variable,
                &variable.source_location,
            );
        }

        if let Some((location, format)) = &self.got_layout_file {
            let got_entries = read_got_layout(location.as_str(), *format)?;
            let mut new_globals = Vec::new();
            let mut new_got_entries = HashMap::new();
            let mut new_got = HashMap::new();

            for (name, _) in &globals {
                if let Some(idx) = got_entries.get(&name.to_string()) {
                    new_got_entries.insert(name.to_string(), *idx);
                    new_got.insert(*idx, name.to_string());
                } else {
                    new_globals.push(name.to_string());
                }
            }

            // Put any globals that weren't there last time in any free space in the GOT.
            let mut idx: u64 = 0;
            for name in &new_globals {
                while new_got.contains_key(&idx) {
                    idx += 1;
                }
                new_got_entries.insert(name.to_string(), idx);
                new_got.insert(idx, name.to_string());
            }

            // Now we can write new_got_entries back out to a file.
            write_got_layout(new_got_entries, location.as_str(), *format)?;

            // Construct our GOT as a new global array. We initialise this array in the loader code.
            let got_size = new_got.keys().max().map_or(0, |m| *m + 1);
            let _got = self.llvm.create_global_variable(
                self.module,
                "__custom_got",
                BasicTypeEnum::ArrayType(Llvm::get_array_type(
                    BasicTypeEnum::PointerType(self.llvm.context.i8_type().ptr_type(0.into())),
                    got_size.try_into().expect("the computed custom GOT size is too large"),
                )),
            );
        }

        Ok(index)
    }

    /// convenience function to generates a global variable for the given variable
    ///
    /// - `module` the module to generate the variable into
    /// - `llvm` the struct used to generate IR-code
    /// - `index` the global symbol table, the global variable will be registerd as a new symbol
    /// - `global_variable` the variable to generate
    fn generate_global_variable(
        &self,
        global_variable: &VariableIndexEntry,
        linkage: LinkageType,
    ) -> Result<GlobalValue<'ctx>, Diagnostic> {
        let type_name = global_variable.get_type_name();
        let variable_type = self.types_index.get_associated_type(type_name)?;

        let name = if self.global_index.get_type_information_or_void(type_name).is_enum() {
            global_variable.get_qualified_name()
        } else {
            global_variable.get_name()
        };

        let mut global_ir_variable = self.llvm.create_global_variable(self.module, name, variable_type);
        if linkage == LinkageType::External {
            global_ir_variable = global_ir_variable.make_external();
        } else {
            let initial_value = if let Some(initializer) = self
                .global_index
                .get_const_expressions()
                .maybe_get_constant_statement(&global_variable.initial_value)
            {
                let expr_generator = ExpressionCodeGenerator::new_context_free(
                    self.llvm,
                    self.global_index,
                    self.annotations,
                    self.types_index,
                );

                //see if this value was compile-time evaluated ...
                if let Some(value) =
                    self.types_index.find_constant_value(global_variable.get_qualified_name())
                {
                    Some(value)
                } else {
                    let value = expr_generator.generate_expression(initializer)?;
                    let target_type = self.global_index.get_effective_type_or_void_by_name(type_name);
                    let value_type = self.annotations.get_type_or_void(initializer, self.global_index);
                    Some(cast_if_needed!(expr_generator, target_type, value_type, value, None))
                }
            } else {
                None
            };

            let initial_value = initial_value
                // 2nd try: find an associated default value for the declared type
                .or_else(|| self.types_index.find_associated_initial_value(type_name))
                // 3rd try: get the compiler's default for the given type (zero-initializer)
                .or_else(|| self.types_index.find_associated_type(type_name).map(get_default_for));
            global_ir_variable.set_initial_value(initial_value, variable_type);
            if global_variable.is_constant() {
                global_ir_variable = global_ir_variable.make_constant();
                if initial_value.is_none() {
                    return Err(Diagnostic::codegen_error(
                        "Cannot generate uninitialized constant",
                        global_variable.source_location.clone(),
                    ));
                }
            }
        }

        let section = section_mangler::SectionMangler::variable(
            global_variable.get_name(),
            section_names::mangle_type(
                self.global_index,
                self.global_index.get_effective_type_by_name(global_variable.get_type_name())?,
            )?,
        )
        .mangle();

        global_ir_variable.set_section(Some(&section));

        Ok(global_ir_variable)
    }
}
