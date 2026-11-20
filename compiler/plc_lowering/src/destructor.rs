//! The destructor lowering module generates destructor functions (`__dtor`) for the types and
//! POUs of a compilation unit that need cleanup. It is the mirror of the initializer lowering.
//!
//! ## Which types need a destructor
//!
//! - A function block, class, or program that declares an `FB_EXIT` method.
//! - A type that owns an instance of such a type: as a member, a struct field, an array element,
//!   or through its base type.
//!
//! All other types get no destructor and no destructor calls. Pointers, references, and
//! `VAR_IN_OUT` variables do not own the instance they refer to.
//!
//! ## Destructor generation
//!
//! - The destructor `<TypeName>__dtor` calls the `FB_EXIT` method of the type, then the
//!   destructors of the members in reverse declaration order, then the destructor of the base type.
//! - The destructor of an array type calls the destructor of each element in ascending order.
//!
//! ## Global cleanup
//!
//! - Program instances and global variables are cleaned up in a per-module function called
//!   `__unit_<name>__dtor`, registered via LLVM's `.fini_array` for automatic execution when the
//!   module is unloaded. Its order is the reverse of the unit constructor.
//!
//! ## Stateless POUs
//!
//! - Local variables of functions and methods, `VAR_TEMP` variables of every POU, and the return
//!   value of a function are constructed on each call. Their destructors are called on each path
//!   that leaves the body: before every `RETURN` and at the end.
//!
//! ## Linkage behavior
//!
//! - Destructors follow the same linkage rules as constructors: types from included (`-i`) or
//!   external units get a declaration only, unless external constructors are generated.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use plc::{
    index::Index,
    lowering::helper::{
        create_call_statement, create_member_reference, get_unit_name, new_destructor, new_unit_destructor,
    },
    typesystem::DataTypeInformation,
};
use plc_ast::{
    ast::{
        AstFactory, AstNode, AstStatement, CompilationUnit, DataType, Implementation, LinkageType, Pou,
        UserTypeDeclaration, VariableBlock,
    },
    control_statements::ReturnStatement,
    mut_visitor::AstVisitorMut,
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::{
    initializer::{create_element_call_loop, is_injected_base_member},
    loops::helper as loop_helper,
};

const USER_DEFINED_DESTRUCTOR: &str = "FB_EXIT";

pub struct Destructor {
    id_provider: IdProvider,
    index: Rc<Index>,
    /// Flag to generate the destructors of external units
    generate_externals: bool,
    /// Remembers for each type that was looked at if it needs a destructor
    needs_destructor: RefCell<HashMap<String, bool>>,
}

/// A variable on the stack of a POU and the type whose destructor cleans it up
struct StackVariable {
    name: String,
    type_name: String,
}

/// Inserts the destructor calls of the stack variables on each path that leaves a body
struct ExitPathLowerer<'a> {
    id_provider: IdProvider,
    variables: &'a [StackVariable],
}

impl Destructor {
    pub fn new(id_provider: IdProvider, index: Rc<Index>, generate_externals: bool) -> Destructor {
        Destructor { id_provider, index, generate_externals, needs_destructor: RefCell::default() }
    }

    /// Adds a destructor for each POU and type of the unit that needs one, a destructor for the
    /// unit itself, and the destructor calls for stack variables to each body
    pub fn apply_destruction(&self, mut unit: CompilationUnit) -> CompilationUnit {
        // Destructors of the POUs and types in this unit
        let mut destructors = vec![];
        destructors.extend(unit.pous.iter().filter_map(|pou| self.create_pou_destructor(pou)));
        destructors.extend(unit.user_types.iter().filter_map(|it| self.create_type_destructor(it)));

        // Destructor of the unit
        destructors.extend(self.create_unit_destructor(&unit));

        // Destructor calls for stack variables
        for pou in &unit.pous {
            let variables = self.collect_stack_variables(pou);
            let implementation = unit.implementations.iter_mut().find(|it| it.name == pou.name);
            if let Some(implementation) = implementation.filter(|_| !variables.is_empty()) {
                ExitPathLowerer { id_provider: self.id_provider.clone(), variables: &variables }
                    .lower(implementation);
            }
        }

        for (pou, implementation) in destructors {
            unit.pous.push(pou);
            unit.implementations.push(implementation);
        }

        unit
    }

    /// Creates the destructor of a function block, class, or program: the user-defined
    /// destructor, then the members in reverse declaration order, then the base type
    fn create_pou_destructor(&self, pou: &Pou) -> Option<(Pou, Implementation)> {
        if !pou.is_stateful() || pou.is_generic() || !self.needs_destructor(&pou.name) {
            return None;
        }
        let linkage = self.linkage_of_destructor(pou.linkage)?;

        // User-defined destructor
        let mut statements = vec![];
        if declares_user_destructor(&self.index, &pou.name) {
            statements.push(self.create_user_destructor_call());
        }

        // Members
        let members = pou
            .variable_blocks
            .iter()
            .flat_map(|block| &block.variables)
            .filter(|variable| !is_injected_base_member(variable, Some(&pou.name), &self.index));
        for variable in members.rev() {
            statements.extend(self.create_member_destructor_call(&pou.name, variable.get_name()));
        }

        // Base type
        if let Some(base) = pou.super_class.as_ref().filter(|base| self.needs_destructor(&base.name)) {
            statements.push(self.create_destructor_call(
                &base.name,
                &format!("__{}", base.name),
                Some("self"),
            ));
        }

        Some(new_destructor(&pou.name, linkage, statements, self.id_provider.clone()))
    }

    /// Creates the destructor of a struct, array, or alias type
    fn create_type_destructor(&self, user_type: &UserTypeDeclaration) -> Option<(Pou, Implementation)> {
        let name = user_type.data_type.get_name()?;
        if user_type.data_type.is_generic() || !self.needs_destructor(name) {
            return None;
        }
        let linkage = self.linkage_of_destructor(user_type.linkage)?;

        let statements = match &user_type.data_type {
            // Fields in reverse declaration order
            DataType::StructType { variables, .. } => variables
                .iter()
                .rev()
                .filter_map(|variable| self.create_member_destructor_call(name, variable.get_name()))
                .collect(),

            // Elements in ascending order
            DataType::ArrayType { referenced_type, is_variable_length: false, .. } => {
                let element_type = referenced_type.get_referenced_type()?;
                create_element_call_loop(
                    &self.index,
                    &mut self.id_provider.clone(),
                    name,
                    &format!("{name}__dtor"),
                    &format!("{element_type}__dtor"),
                )?
            }

            // The type that the alias renames
            DataType::SubRangeType { referenced_type, bounds: None, .. } => {
                vec![self.create_destructor_call(referenced_type, "self", None)]
            }

            _ => return None,
        };

        Some(new_destructor(name, linkage, statements, self.id_provider.clone()))
    }

    /// Creates the destructor of the unit: the program instances and then the global variables,
    /// each in reverse declaration order
    fn create_unit_destructor(&self, unit: &CompilationUnit) -> Option<(Pou, Implementation)> {
        let should_generate = match unit.linkage {
            LinkageType::Internal => true,
            LinkageType::External => self.generate_externals,
            _ => false,
        };
        if !should_generate {
            return None;
        }

        // Program instances
        let mut statements = vec![];
        let programs = unit.pous.iter().filter(|pou| pou.is_program() && self.needs_destructor(&pou.name));
        for program in programs.rev() {
            statements.push(self.create_destructor_call(&program.name, program.get_return_name(), None));
        }

        // Global variables
        let globals = unit
            .global_vars
            .iter()
            .filter(|block| self.owns_global_variables(block))
            .flat_map(|block| &block.variables);
        for variable in globals.rev() {
            let Some(global) = self.index.find_global_variable(variable.get_name()) else { continue };
            if self.needs_destructor(global.get_type_name()) {
                statements.push(self.create_destructor_call(global.get_type_name(), global.get_name(), None));
            }
        }

        if statements.is_empty() {
            return None;
        }
        Some(new_unit_destructor(&get_unit_name(unit), statements, self.id_provider.clone()))
    }

    /// Collects the variables that are constructed on each call of the POU and that need a
    /// destructor, in the order of their destruction: the return value, then the locals and
    /// temporaries in reverse declaration order
    fn collect_stack_variables(&self, pou: &Pou) -> Vec<StackVariable> {
        if pou.is_generic() || pou.linkage.is_external_or_included() || pou.kind.is_generated() {
            return vec![];
        }

        // Return value
        let mut variables = vec![];
        let return_type = pou.return_type.as_ref().and_then(|it| it.get_name());
        if let Some(return_type) = return_type.filter(|it| !pou.is_stateful() && self.needs_destructor(it)) {
            variables.push(StackVariable {
                name: pou.get_return_name().to_string(),
                type_name: return_type.to_string(),
            });
        }

        // Locals and temporaries
        let blocks = pou
            .variable_blocks
            .iter()
            .filter(|block| !block.constant)
            .filter(|block| block.kind.is_temp() || (block.kind.is_local() && !pou.is_stateful()));
        for variable in blocks.flat_map(|block| &block.variables).rev() {
            let Some(member) = self.index.find_member(&pou.name, variable.get_name()) else { continue };
            if self.needs_destructor(member.get_type_name()) {
                variables.push(StackVariable {
                    name: member.get_name().to_string(),
                    type_name: member.get_type_name().to_string(),
                });
            }
        }

        variables
    }

    /// Returns the linkage of the destructor for a type with the given linkage, or `None` if the
    /// type gets no destructor. Included types and external types are declared only, unless the
    /// destructors of external units are generated.
    fn linkage_of_destructor(&self, linkage: LinkageType) -> Option<LinkageType> {
        match linkage {
            LinkageType::Internal => Some(LinkageType::Internal),
            LinkageType::External if self.generate_externals => Some(LinkageType::Internal),
            LinkageType::External | LinkageType::Include => Some(LinkageType::External),
            LinkageType::BuiltIn => None,
        }
    }

    /// Returns true if this unit cleans up the variables of the global block. Included globals
    /// belong to their library, and external globals to their module unless the destructors of
    /// external units are generated.
    fn owns_global_variables(&self, block: &VariableBlock) -> bool {
        !block.constant
            && match block.linkage {
                LinkageType::Include => false,
                LinkageType::External => self.generate_externals,
                _ => true,
            }
    }

    /// Returns true if an instance of the type must be cleaned up: the type declares `FB_EXIT`, or
    /// it owns an instance of a type that needs a destructor
    fn needs_destructor(&self, type_name: &str) -> bool {
        let key = type_name.to_lowercase();
        if let Some(known) = self.needs_destructor.borrow().get(&key) {
            return *known;
        }

        // An entry for the type itself ends the recursion for a type that contains itself
        self.needs_destructor.borrow_mut().insert(key.clone(), false);
        let needs_destructor = self.index.find_type(type_name).is_some_and(|data_type| {
            !data_type.linkage.is_built_in()
                && (declares_user_destructor(&self.index, type_name)
                    || match data_type.get_type_information() {
                        DataTypeInformation::Struct { members, .. } => members
                            .iter()
                            .filter(|member| !member.is_temp())
                            .any(|member| self.needs_destructor(member.get_type_name())),
                        DataTypeInformation::Array { inner_type_name, .. }
                        | DataTypeInformation::Alias { referenced_type: inner_type_name, .. } => {
                            self.needs_destructor(inner_type_name)
                        }
                        _ => false,
                    })
        });
        self.needs_destructor.borrow_mut().insert(key, needs_destructor);

        needs_destructor
    }

    /// Creates the destructor call for the member of a container, if the member is owned by
    /// the container and its type needs a destructor: `<type>__dtor(self.<member>)`
    fn create_member_destructor_call(&self, container: &str, member: &str) -> Option<AstNode> {
        let member = self.index.find_member(container, member).filter(|it| !it.is_temp())?;
        self.needs_destructor(member.get_type_name())
            .then(|| self.create_destructor_call(member.get_type_name(), member.get_name(), Some("self")))
    }

    /// Creates a call statement to the destructor of the given type: `<type_name>__dtor(<var_name>)`
    fn create_destructor_call(&self, type_name: &str, var_name: &str, base: Option<&str>) -> AstNode {
        create_call_statement(
            &format!("{type_name}__dtor"),
            var_name,
            base,
            self.id_provider.clone(),
            &SourceLocation::internal(),
        )
    }

    /// Creates the call to the user-defined destructor: `self.FB_EXIT()`
    fn create_user_destructor_call(&self) -> AstNode {
        let mut id_provider = self.id_provider.clone();
        let base = create_member_reference("self", id_provider.clone(), None);
        let operator = create_member_reference(USER_DEFINED_DESTRUCTOR, id_provider.clone(), Some(base));
        AstFactory::create_call_statement(operator, None, id_provider.next_id(), SourceLocation::internal())
    }
}

impl ExitPathLowerer<'_> {
    fn lower(&mut self, implementation: &mut Implementation) {
        self.visit_statement_list(&mut implementation.statements);
        let mut calls = self.create_destructor_calls();
        implementation.statements.append(&mut calls);
    }

    /// Creates the destructor calls for the stack variables, with new nodes for each call site
    fn create_destructor_calls(&self) -> Vec<AstNode> {
        self.variables
            .iter()
            .map(|variable| {
                create_call_statement(
                    &format!("{}__dtor", variable.type_name),
                    &variable.name,
                    None,
                    self.id_provider.clone(),
                    &SourceLocation::internal(),
                )
            })
            .collect()
    }
}

impl AstVisitorMut for ExitPathLowerer<'_> {
    fn visit_statement_list(&mut self, stmts: &mut Vec<AstNode>) {
        let mut lowered = Vec::with_capacity(stmts.len());

        for mut node in std::mem::take(stmts) {
            self.visit(&mut node);

            match node.stmt {
                // RETURN => <destructor calls>; RETURN
                AstStatement::ReturnStatement(ReturnStatement { condition: None }) => {
                    lowered.append(&mut self.create_destructor_calls());
                    lowered.push(node);
                }

                // RETURN <condition> => IF <condition> THEN <destructor calls>; RETURN; END_IF
                AstStatement::ReturnStatement(ReturnStatement { condition: Some(condition) }) => {
                    let mut body = self.create_destructor_calls();
                    body.push(AstFactory::create_return_statement(
                        None,
                        node.location.clone(),
                        self.id_provider.next_id(),
                    ));
                    lowered.push(loop_helper::create_if_then(
                        self.id_provider.clone(),
                        *condition,
                        body,
                        node.location,
                    ));
                }

                _ => lowered.push(node),
            }
        }

        *stmts = lowered;
    }
}

/// Returns true if the type declares the user-defined destructor `FB_EXIT`. The lookup is exact:
/// an inherited `FB_EXIT` is called by the destructor of the base type.
fn declares_user_destructor(index: &Index, type_name: &str) -> bool {
    index.find_pou(&format!("{type_name}.{USER_DEFINED_DESTRUCTOR}")).is_some()
}

#[cfg(test)]
mod tests {
    use plc::lowering::helper::create_member_reference;
    use plc_ast::{ast::AstFactory, mut_visitor::AstVisitorMut, provider::IdProvider};
    use plc_source::source_location::SourceLocation;

    use super::{ExitPathLowerer, StackVariable};

    #[test]
    fn conditional_return_is_lowered_to_a_branch_that_destructs_and_returns() {
        let mut id_provider = IdProvider::default();
        let condition = create_member_reference("done", id_provider.clone(), None);
        let mut statements = vec![AstFactory::create_return_statement(
            Some(condition),
            SourceLocation::internal(),
            id_provider.next_id(),
        )];
        let variables = [
            StackVariable { name: "second".into(), type_name: "Connection".into() },
            StackVariable { name: "first".into(), type_name: "Connection".into() },
        ];

        ExitPathLowerer { id_provider, variables: &variables }.visit_statement_list(&mut statements);

        let lowered = statements.iter().map(|it| it.as_string()).collect::<Vec<_>>().join("\n");
        insta::assert_snapshot!(lowered, @"
        IF done THEN
            Connection__dtor(second)
            Connection__dtor(first)
            RETURN
        END_IF
        ");
    }
}
