//! # Retain Variable Lowering
//!
//! This module handles the lowering of `RETAIN` variables as defined in IEC 61131-3.
//! Retain variables persist their values across power cycles by being placed in a
//! dedicated `.retain` linker section, which a PLC runtime maps to non-volatile memory.
//!
//! The lowering pass handles three cases:
//!
//! ## 1. PROGRAM retain variables
//! Variables declared as `VAR RETAIN` inside a `PROGRAM` are extracted into global
//! retain variables and replaced with auto-dereferenced pointers. For example:
//!
//! ```st
//! PROGRAM Main
//!     VAR RETAIN
//!         counter : INT := 0;
//!     END_VAR
//! END_PROGRAM
//! ```
//!
//! Is transformed into:
//! - A global variable `__Main_counter__retain : INT := 0` in a retain block
//! - The original `counter` becomes an auto-deref pointer to `__Main_counter__retain`
//!
//! This indirection ensures the PROGRAM's instance struct contains pointers to the
//! retain globals, while the actual values live in the `.retain` section.
//!
//! ## 2. Global variables with transitive retain
//! If a global variable's type transitively contains retain members (e.g. an FB with
//! `VAR RETAIN`), the variable is moved to a retain block even if it was not explicitly
//! declared in a `VAR_GLOBAL RETAIN` block.
//!
//! ## 3. FUNCTION_BLOCK retain variables
//! Retain variables inside FBs stay in-place within the FB's struct definition. The
//! entire FB instance receives the `.retain` section treatment when it is instantiated
//! at the global or program level, handled by the transitive `should_retain()` check
//! during code generation.

use plc_ast::{
    ast::{
        AccessModifier, AstFactory, AutoDerefType, CompilationUnit, DataType, DataTypeDeclaration, Variable,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

pub struct RetainParticipant {
    pub ids: IdProvider,
}

impl RetainParticipant {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn lower_retain(&mut self, units: &mut [CompilationUnit], index: plc::index::Index) {
        let mut lowerer = RetainLowerer { ids: self.ids.clone(), index, context: Context::default() };
        for unit in units {
            lowerer.visit_compilation_unit(unit);
        }
    }
}

struct RetainLowerer {
    ids: IdProvider,
    index: plc::index::Index,
    context: Context,
}

#[derive(Debug, Default)]
struct Context {
    container_name: Option<String>,
    in_program: bool,
    retain_variables: Vec<Variable>,
}

impl AstVisitorMut for RetainLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self);
        // After visiting the compilation unit, add all retain variables to the global vars
        if !self.context.retain_variables.is_empty() {
            // Find an existing retain global variable block or create a new one if it doesn't exist
            unit.global_vars
                .iter_mut()
                .find(|block| block.retain)
                .map(|block| {
                    block.variables.append(&mut self.context.retain_variables);
                })
                .unwrap_or_else(|| {
                    let retain_block = plc_ast::ast::VariableBlock {
                        variables: self.context.retain_variables.drain(..).collect(),
                        kind: plc_ast::ast::VariableBlockType::Global,
                        constant: false,
                        retain: true,
                        linkage: plc_ast::ast::LinkageType::Internal,
                        location: SourceLocation::internal(),
                        access: AccessModifier::Public,
                    };
                    unit.global_vars.push(retain_block);
                });
        }
    }
    fn visit_pou(&mut self, pou: &mut plc_ast::ast::Pou) {
        self.context.in_program = matches!(pou.kind, plc_ast::ast::PouType::Program);
        self.context.container_name = Some(pou.name.clone());
        pou.walk(self);
        self.context.in_program = false;
        self.context.container_name = None;
    }

    fn visit_variable_block(&mut self, block: &mut plc_ast::ast::VariableBlock) {
        let variables = std::mem::take(&mut block.variables);
        //If the block is retain but we are in a program, mark the block as non-retain
        if block.retain && self.context.in_program {
            block.retain = false;
        }
        for variable in variables {
            let Some(variable_index) =
                self.index.find_variable(self.context.container_name.as_deref(), &[variable.get_name()])
            else {
                block.variables.push(variable);
                continue;
            };

            if !variable_index.should_retain(&self.index) {
                block.variables.push(variable);
                continue;
            }

            if self.context.in_program {
                let (old_variable, new_var) = self.replace_with_retain_variable(variable);
                self.context.retain_variables.push(new_var);
                block.variables.push(old_variable);
            } else if matches!(block.kind, plc_ast::ast::VariableBlockType::Global) && !block.retain {
                // Global variable in a non-retain block whose type transitively contains retain
                // members (e.g. an FB with VAR RETAIN). Move it to a retain block.
                self.context.retain_variables.push(variable);
            } else {
                // FB retain variables stay in-place within the FB's struct. The entire FB instance
                // gets placed in the .retain section when instantiated at the global/program level,
                // handled by the transitive should_retain() check during codegen.
                block.variables.push(variable);
            }
        }
    }
}

impl RetainLowerer {
    /// Replaces a retain variable in a program with a global retain variable and replaces the original variable with an auto reference to the global variable
    fn replace_with_retain_variable(&mut self, mut variable: Variable) -> (Variable, Variable) {
        let new_name = format!(
            "__{}_{}__retain",
            self.context.container_name.as_deref().unwrap_or_default(),
            variable.get_name()
        );
        // Create a global variable called __<pou_name>_<var_name> and move the initializer and datatype to the global variable
        let new_var = Variable {
            name: new_name,
            data_type_declaration: variable.data_type_declaration.clone(),
            initializer: variable.initializer.take(),
            location: variable.location.clone(),
            address: None,
        };
        variable.data_type_declaration = DataTypeDeclaration::Definition {
            data_type: Box::new(DataType::PointerType {
                name: None,
                referenced_type: Box::new(variable.data_type_declaration.clone()),
                auto_deref: Some(AutoDerefType::Alias),
                type_safe: true,
                is_function: false,
            }),
            location: variable.data_type_declaration.get_location(),
            scope: self.context.container_name.clone(),
        };
        variable.initializer = Some(AstFactory::create_member_reference(
            AstFactory::create_identifier(&new_var.name, variable.location.clone(), self.ids.next_id()),
            None,
            self.ids.next_id(),
        ));
        (variable, new_var)
    }
}

#[cfg(test)]
mod tests {
    use plc_ast::ast::{CompilationUnit, DataTypeDeclaration, Variable, VariableBlock};
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

    /// Finds the first global variable block with `retain: true` in a compilation unit.
    fn find_retain_block(unit: &CompilationUnit) -> &VariableBlock {
        unit.global_vars.iter().find(|b| b.retain).expect("expected a global retain block")
    }

    /// Finds a variable by name in a given list of variable blocks.
    fn find_variable_in_blocks<'a>(blocks: &'a [VariableBlock], name: &str) -> &'a Variable {
        blocks
            .iter()
            .flat_map(|b| &b.variables)
            .find(|v| v.get_name() == name)
            .unwrap_or_else(|| panic!("expected variable '{name}' in blocks"))
    }

    /// Finds a POU by name in a compilation unit and returns a variable from it.
    fn find_pou_variable<'a>(unit: &'a CompilationUnit, pou_name: &str, var_name: &str) -> &'a Variable {
        let pou = unit
            .pous
            .iter()
            .find(|p| p.name == pou_name)
            .unwrap_or_else(|| panic!("POU '{pou_name}' not found"));
        find_variable_in_blocks(&pou.variable_blocks, var_name)
    }

    /// Returns the referenced type name from a DataTypeDeclaration, if it is a Reference variant.
    fn referenced_type_name(decl: &DataTypeDeclaration) -> &str {
        match decl {
            DataTypeDeclaration::Reference { referenced_type, .. } => referenced_type,
            other => panic!("expected DataTypeDeclaration::Reference, got {other:?}"),
        }
    }

    #[test]
    fn test_retain_lowering_on_program() {
        let source: SourceCode = r#"
        PROGRAM Test
        VAR RETAIN
            x: INT := 5;
        END_VAR
        END_PROGRAM
        "#
        .into();

        let (_, project) =
            parse_and_annotate("test", vec![source]).expect("Failed to parse compilation unit");
        let unit = project.units[0].get_unit();

        // A global retain block should exist with the extracted retain variable
        let retain_block = find_retain_block(unit);
        assert_eq!(retain_block.variables.len(), 1);
        let retain_var = &retain_block.variables[0];
        assert_eq!(retain_var.get_name(), "__Test_x__retain");
        assert_eq!(referenced_type_name(&retain_var.data_type_declaration), "INT");
        assert!(retain_var.initializer.is_some(), "retain variable should carry the original initializer");

        // The program's variable should be replaced with an auto-deref pointer
        let pou_var = find_pou_variable(unit, "Test", "x");
        // The type should now reference the generated pointer type, not INT directly
        assert_eq!(referenced_type_name(&pou_var.data_type_declaration), "__Test_x");
        assert!(pou_var.initializer.is_some(), "program variable should have a reference initializer");

        // The program's variable block should no longer be marked retain
        let test_pou = unit.pous.iter().find(|p| p.name == "Test").unwrap();
        assert!(
            test_pou.variable_blocks.iter().all(|b| !b.retain),
            "program variable blocks should not be retain after lowering"
        );
    }

    #[test]
    fn test_retain_lowering_on_program_nested() {
        let source: SourceCode = r#"
        FUNCTION_BLOCK FB
        VAR RETAIN
            a: INT := 5;
        END_VAR
        END_FUNCTION_BLOCK
        PROGRAM Test
        VAR
            x: FB;
        END_VAR
        END_PROGRAM
        "#
        .into();

        let (_, project) =
            parse_and_annotate("test", vec![source]).expect("Failed to parse compilation unit");
        let unit = project.units[0].get_unit();

        // The FB instance should be extracted to a global retain variable
        let retain_block = find_retain_block(unit);
        assert_eq!(retain_block.variables.len(), 1);
        let retain_var = &retain_block.variables[0];
        assert_eq!(retain_var.get_name(), "__Test_x__retain");
        assert_eq!(referenced_type_name(&retain_var.data_type_declaration), "FB");

        // The FB's own retain block should remain intact (retain stays in-place for FBs)
        let fb_pou = unit.pous.iter().find(|p| p.name == "FB").unwrap();
        let fb_retain_block =
            fb_pou.variable_blocks.iter().find(|b| b.retain).expect("FB should still have a retain block");
        let fb_retain_blocks = [fb_retain_block.clone()];
        let fb_var = find_variable_in_blocks(&fb_retain_blocks, "a");
        assert_eq!(referenced_type_name(&fb_var.data_type_declaration), "INT");

        // The program's variable should be replaced with an auto-deref pointer to the retain global
        let pou_var = find_pou_variable(unit, "Test", "x");
        assert_eq!(referenced_type_name(&pou_var.data_type_declaration), "__Test_x");
        assert!(pou_var.initializer.is_some(), "program variable should have a reference initializer");
    }

    #[test]
    fn test_retain_in_global_nested_should_move_to_retain_block() {
        let source: SourceCode = r#"
        FUNCTION_BLOCK FB
        VAR RETAIN
            a: INT := 5;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL RETAIN
            explicit_retain: FB;
            y : INT;
        END_VAR
        VAR_GLOBAL
            implicit_retain: FB;
            x : INT;
        END_VAR
        "#
        .into();

        let (_, project) =
            parse_and_annotate("test", vec![source]).expect("Failed to parse compilation unit");
        let unit = project.units[0].get_unit();

        // The retain block should contain the explicitly retained variables AND the
        // implicitly retained one (implicit_retain is FB which transitively contains retain members)
        let retain_block = find_retain_block(unit);
        let retain_var_names: Vec<&str> = retain_block.variables.iter().map(|v| v.get_name()).collect();
        assert!(retain_var_names.contains(&"explicit_retain"), "explicit_retain should be in retain block");
        assert!(
            retain_var_names.contains(&"y"),
            "y should stay in retain block (declared in VAR_GLOBAL RETAIN)"
        );
        assert!(
            retain_var_names.contains(&"implicit_retain"),
            "implicit_retain should be moved to retain block (transitive retain)"
        );

        // The non-retain global block should only contain x (a plain INT with no retain)
        let non_retain_globals: Vec<&str> = unit
            .global_vars
            .iter()
            .filter(|b| !b.retain)
            .flat_map(|b| &b.variables)
            .filter(|v| !v.get_name().starts_with("__")) // exclude compiler-generated variables
            .map(|v| v.get_name())
            .collect();
        assert_eq!(non_retain_globals, vec!["x"], "only x should remain in the non-retain global block");
    }
}
