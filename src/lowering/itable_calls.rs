//! Lowering of method calls on interface-typed variables into indirect calls through interface tables
//!
//! This module is the interface-polymorphism counterpart to [`crate::lowering::polymorphism`] (which
//! handles class/FB inheritance-based dispatch via virtual tables). It transforms method calls on
//! interface-typed variables into itable lookups and indirect calls.
//!
//! The transformations performed are:
//!
//! # 1. Fat Pointer Type Generation
//! When at least one interface-typed variable is encountered, a `__FATPOINTER` struct is generated
//! containing two void pointers:
//! - `data`: points to the concrete POU instance
//! - `table`: points to the correct itable instance for the (interface, POU) pair
//!
//! This struct is only generated when needed (i.e., when there are interface-typed variables).
//!
//! # 2. Variable type replacement
//! Interface-typed variable declarations (e.g. `myVar : IFoo`) are replaced with `__FATPOINTER`.
//!
//! # 3. Assignment rewriting
//! Assignments to interface-typed variables are rewritten to populate the fat pointer fields:
//! - `.data := ADR(instance)`
//! - `.table := ADR(__itable_<Interface>_<POU>_instance)`
//!
//! # 4. Method call transformation
//! Method calls on interface variables are lowered using the same 4-step pattern as
//! [`crate::lowering::polymorphism`], but accessing the itable instead of the fat pointer's `.table` field:
//! 1. Add the data pointer as the first argument
//! 2. Access the itable through the fat pointer's `.table` field
//! 3. Cast the itable to the concrete interface table type
//! 4. Dereference the function pointer for the indirect call

use plc_ast::{
    ast::{
        Assignment, AstFactory, AstNode, AstStatement, CallStatement, CompilationUnit, DataType,
        ReferenceAccess, ReferenceExpr, UserTypeDeclaration,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    lowering::itable::helper,
    resolver::{AnnotationMap, AnnotationMapImpl, StatementAnnotation},
};

pub struct InterfaceCallLowerer {
    pub ids: IdProvider,
    pub index: Option<Index>,
    pub annotations: Option<AnnotationMapImpl>,
    /// Tracks whether we've encountered at least one interface-typed variable
    has_interface_variable: bool,
}

impl InterfaceCallLowerer {
    pub fn new(ids: IdProvider) -> InterfaceCallLowerer {
        InterfaceCallLowerer { ids, index: None, annotations: None, has_interface_variable: false }
    }

    pub fn lower_units(&mut self, units: &mut [CompilationUnit]) {
        for unit in units.iter_mut() {
            self.visit_compilation_unit(unit);
        }

        // Generate __FATPOINTER once if any unit had interface-typed variables
        if self.has_interface_variable {
            if let Some(first_unit) = units.first_mut() {
                let unit_file = first_unit.file.get_name();
                first_unit.user_types.push(self.generate_fat_pointer(unit_file));
            }
        }
    }

    /// Generates the `__FATPOINTER` struct type used to represent interface-typed variables.
    ///
    /// The struct contains two void pointers:
    /// - `data`: points to the concrete POU instance
    /// - `table`: points to the itable for the (interface, POU) pair
    fn generate_fat_pointer(&self, unit_file: Option<&'static str>) -> UserTypeDeclaration {
        let location = SourceLocation::internal_in_unit(unit_file);
        UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some("__FATPOINTER".to_string()),
                variables: vec![
                    helper::create_void_pointer_variable("data", &location),
                    helper::create_void_pointer_variable("table", &location),
                ],
            },
            initializer: None,
            location,
            scope: None,
        }
    }

    /// Attempts to expand an interface assignment into two fat pointer field assignments.
    ///
    /// Given `reference := instance` where `reference` is interface-typed, produces:
    /// - `reference.data := ADR(instance)`
    /// - `reference.table := ADR(__itable_<Interface>_<POU>_instance)`
    fn try_expand_interface_assignment(&mut self, node: &AstNode) -> Option<Vec<AstNode>> {
        let AstStatement::Assignment(Assignment { left, right }) = &node.stmt else {
            return None;
        };

        let index = self.index.as_ref()?;
        let annotations = self.annotations.as_ref()?;

        // Check if the left-hand side is interface-typed
        let left_type = annotations.get_type(left, index)?;
        if !left_type.get_type_information().is_interface() {
            return None;
        }
        let interface_name = left_type.get_type_information().get_name().to_string();

        // Get the concrete POU name from the right-hand side annotation
        let right_annotation = annotations.get(right)?;
        let pou_name = match right_annotation {
            StatementAnnotation::Variable { resulting_type, .. } => resulting_type.clone(),
            _ => return None,
        };

        let loc = SourceLocation::internal();

        // Build: reference.data := ADR(instance)
        let data_member = AstFactory::create_member_reference(
            AstFactory::create_identifier("data", &loc, self.ids.next_id()),
            Some(left.as_ref().clone()),
            self.ids.next_id(),
        );
        let adr_instance =
            AstFactory::create_call_to_with_ids("ADR", vec![right.as_ref().clone()], &loc, self.ids.clone());
        // Consume the IDs used by create_call_to_with_ids (it uses 4 IDs)
        for _ in 0..4 {
            self.ids.next_id();
        }
        let data_assignment = AstFactory::create_assignment(data_member, adr_instance, self.ids.next_id());

        // Build: reference.table := ADR(__itable_<Interface>_<POU>_instance)
        let table_member = AstFactory::create_member_reference(
            AstFactory::create_identifier("table", &loc, self.ids.next_id()),
            Some(left.as_ref().clone()),
            self.ids.next_id(),
        );
        let itable_instance_name = helper::get_itable_instance_name(&interface_name, &pou_name);
        let adr_itable = AstFactory::create_call_to_with_ids(
            "ADR",
            vec![AstFactory::create_member_reference(
                AstFactory::create_identifier(&itable_instance_name, &loc, self.ids.next_id()),
                None,
                self.ids.next_id(),
            )],
            &loc,
            self.ids.clone(),
        );
        for _ in 0..4 {
            self.ids.next_id();
        }
        let table_assignment = AstFactory::create_assignment(table_member, adr_itable, self.ids.next_id());

        Some(vec![data_assignment, table_assignment])
    }
}

impl AstVisitorMut for InterfaceCallLowerer {
    fn visit_variable(&mut self, variable: &mut plc_ast::ast::Variable) {
        let Some(type_name) = variable.data_type_declaration.get_referenced_type() else {
            return;
        };

        let Some(index) = self.index.as_ref() else {
            return;
        };

        if index.find_interface(&type_name).is_some() {
            variable.replace_data_type_with_reference_to("__FATPOINTER".to_string());
            self.has_interface_variable = true;
        }
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        if implementation.location.is_internal() {
            return;
        }

        let mut new_statements = Vec::with_capacity(implementation.statements.len());
        for mut statement in std::mem::take(&mut implementation.statements) {
            if let Some(expanded) = self.try_expand_interface_assignment(&statement) {
                new_statements.extend(expanded);
            } else {
                statement.walk(self);
                new_statements.push(statement);
            }
        }
        implementation.statements = new_statements;
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let AstStatement::CallStatement(CallStatement { operator, parameters }) = &mut node.stmt else {
            unreachable!();
        };

        // Walk parameters first — they may contain nested interface calls
        if let Some(ref mut parameters) = parameters {
            parameters.walk(self);
        }

        // Check if this is a method call on an interface-typed variable
        let AstStatement::ReferenceExpr(ReferenceExpr {
            access: ReferenceAccess::Member(method),
            base: Some(base),
        }) = &operator.stmt
        else {
            return;
        };

        let index = match self.index.as_ref() {
            Some(idx) => idx,
            None => return,
        };
        let annotations = match self.annotations.as_ref() {
            Some(ann) => ann,
            None => return,
        };

        // Check if the base is interface-typed
        let Some(base_type) = annotations.get_type(base, index) else {
            return;
        };
        if !base_type.get_type_information().is_interface() {
            return;
        }
        let interface_name = base_type.get_type_information().get_name().to_string();

        // Extract the method name from the member access
        let method_name = match &method.stmt {
            AstStatement::Identifier(name) => name.clone(),
            _ => return,
        };

        let loc = SourceLocation::internal();

        // Build the new operator: __itable_<Iface>#(reference.table^).method^
        //
        // Step 1: reference.table
        let table_access = AstFactory::create_member_reference(
            AstFactory::create_identifier("table", &loc, self.ids.next_id()),
            Some(base.as_ref().clone()),
            self.ids.next_id(),
        );

        // Step 2: reference.table^
        let table_deref = AstFactory::create_deref_reference(table_access, self.ids.next_id(), loc.clone());

        // Step 3: (reference.table^)
        let table_paren = AstFactory::create_paren_expression(table_deref, loc.clone(), self.ids.next_id());

        // Step 4: __itable_<Iface>#(reference.table^)
        let itable_name = helper::get_itable_name(&interface_name);
        let cast = AstFactory::create_cast_statement(
            AstFactory::create_member_reference(
                AstFactory::create_identifier(&itable_name, &loc, self.ids.next_id()),
                None,
                self.ids.next_id(),
            ),
            table_paren,
            &loc,
            self.ids.next_id(),
        );

        // Step 5: __itable_<Iface>#(reference.table^).method
        let method_access = AstFactory::create_member_reference(
            AstFactory::create_identifier(&method_name, &loc, self.ids.next_id()),
            Some(cast),
            self.ids.next_id(),
        );

        // Step 6: __itable_<Iface>#(reference.table^).method^
        let new_operator = AstFactory::create_deref_reference(method_access, self.ids.next_id(), loc.clone());

        // Build the first argument: reference.data (a pointer, not dereferenced)
        let data_access = AstFactory::create_member_reference(
            AstFactory::create_identifier("data", &loc, self.ids.next_id()),
            Some(base.as_ref().clone()),
            self.ids.next_id(),
        );

        // Replace the operator
        let AstStatement::CallStatement(CallStatement { operator, parameters }) = &mut node.stmt else {
            unreachable!();
        };
        *operator = Box::new(new_operator);

        // Prepend data_access (pointer) as the first argument
        match parameters {
            None => {
                *parameters = Some(Box::new(data_access));
            }
            Some(ref mut expr) => match &mut expr.stmt {
                AstStatement::ExpressionList(expressions) => {
                    expressions.insert(0, data_access);
                }
                _ => {
                    let mut new_expr = Box::new(AstFactory::create_expression_list(
                        vec![data_access, std::mem::take(expr)],
                        loc,
                        self.ids.next_id(),
                    ));
                    std::mem::swap(expr, &mut new_expr);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lowering::itable_calls::tests::helper::{
        get_fat_pointer, get_impl_statements, get_pou_variables, lower_interface_calls,
    };

    #[test]
    fn fat_pointer_is_not_generated_if_no_variable_makes_use_of_interface_type() {
        let source = r#"
            INTERFACE A
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS A
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceA: FbA;
                END_VAR
            END_FUNCTION
        "#;

        let units = lower_interface_calls(source);
        let fat_pointer = get_fat_pointer(&units);

        assert!(fat_pointer.is_none(), "__FATPOINTER should NOT be generated");
    }

    #[test]
    fn fat_pointer_is_generated_if_variable_makes_use_of_interface_type() {
        let source = r#"
            INTERFACE A
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS A
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceA: FbA;
                    referenceA: A; // This should trigger the creation of a __FATPOINTER struct
                END_VAR
            END_FUNCTION
        "#;

        let units = lower_interface_calls(source);
        let fat_pointer = get_fat_pointer(&units);

        assert!(fat_pointer.is_some(), "__FATPOINTER should be generated");
        insta::assert_snapshot!(fat_pointer.unwrap(), @r"
        __FATPOINTER {
            data: POINTER TO __VOID;
            table: POINTER TO __VOID;
        }
        ");
    }

    #[test]
    fn interface_type_is_replaced_by_fat_pointer() {
        let source = r#"
            INTERFACE A
            END_INTERFACE

            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceA: FbA;
                    referenceA: A; // This should become `referenceA: __FATPOINTER`
                END_VAR
            END_FUNCTION
        "#;

        let units = lower_interface_calls(source);
        let variables = get_pou_variables(&units, &["main"]);

        insta::assert_snapshot!(variables, @r"
        // Variables in main
        instanceA: FbA
        referenceA: __FATPOINTER
        ");
    }

    #[test]
    fn interface_variable_assignment_is_patched() {
        let source = r#"
            INTERFACE A
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS A
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceA: FbA;
                    referenceA: A;
                END_VAR

                referenceA := instanceA;
            END_FUNCTION
        "#;

        let units = lower_interface_calls(source);
        let statements = get_impl_statements(&units, &["main"]);

        insta::assert_snapshot!(statements, @r"
        // Statements in main
        referenceA.data := ADR(instanceA)
        referenceA.table := ADR(__itable_A_FbA_instance)
        ");
    }

    #[test]
    fn interface_variable_call_is_patched() {
        let source = r#"
            INTERFACE A
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS A
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceA: FbA;
                    referenceA: A; // This should become `referenceA: __FATPOINTER`
                END_VAR

                referenceA := instanceA;
                referenceA.foo();
            END_FUNCTION
        "#;

        let units = lower_interface_calls(source);
        let statements = get_impl_statements(&units, &["main"]);

        insta::assert_snapshot!(statements, @r"
        // Statements in main
        referenceA.data := ADR(instanceA)
        referenceA.table := ADR(__itable_A_FbA_instance)
        __itable_A#(referenceA.table^).foo^(referenceA.data)
        ");
    }

    #[test]
    fn aggregate_return_type() {
        todo!("make sure alloca works");
    }

    mod helper {
        use plc_ast::ast::{CompilationUnit, DataType, DataTypeDeclaration};
        use plc_ast::provider::IdProvider;

        use crate::{
            lowering::itable_calls::InterfaceCallLowerer,
            test_utils::tests::{annotate_with_ids, index_with_ids},
        };

        /// Lowers source code through the interface call lowerer, returning the transformed units.
        pub fn lower_interface_calls(source: &str) -> Vec<CompilationUnit> {
            let ids = IdProvider::default();
            let (unit, mut index) = index_with_ids(source, ids.clone());
            let annotations = annotate_with_ids(&unit, &mut index, ids.clone());

            let mut lowerer = InterfaceCallLowerer::new(ids);
            lowerer.index = Some(index);
            lowerer.annotations = Some(annotations);

            let mut units = vec![unit];
            lowerer.lower_units(&mut units);
            units
        }

        /// Returns the serialized __FATPOINTER definition, or None if not generated.
        pub fn get_fat_pointer(units: &[CompilationUnit]) -> Option<String> {
            units.iter().find_map(|unit| {
                unit.user_types
                    .iter()
                    .find(|ty| ty.data_type.get_name() == Some("__FATPOINTER"))
                    .map(|ty| serialize_struct_type(&ty.data_type))
            })
        }

        /// Returns serialized variable declarations for the specified POUs.
        ///
        /// Format:
        /// ```text
        /// // Variables in <POU1>
        /// var1: Type1
        /// var2: Type2
        ///
        /// // Variables in <POU2>
        /// var3: Type3
        /// ```
        pub fn get_pou_variables(units: &[CompilationUnit], pou_names: &[&str]) -> String {
            let sections: Vec<String> = pou_names
                .iter()
                .map(|pou_name| {
                    let variables = units
                        .iter()
                        .find_map(|unit| unit.pous.iter().find(|pou| &pou.name == pou_name))
                        .map(|pou| {
                            pou.variable_blocks
                                .iter()
                                .flat_map(|block| &block.variables)
                                .map(|var| {
                                    let type_name = serialize_data_type_decl(&var.data_type_declaration);
                                    format!("{}: {type_name}", var.name)
                                })
                                .collect::<Vec<_>>()
                                .join("\n")
                        })
                        .unwrap_or_default();

                    format!("// Variables in {pou_name}\n{variables}")
                })
                .collect();

            sections.join("\n\n")
        }

        /// Returns serialized statements for the specified implementations.
        ///
        /// Format:
        /// ```text
        /// // Statements in <Impl1>
        /// statement1
        /// statement2
        ///
        /// // Statements in <Impl2>
        /// statement3
        /// ```
        pub fn get_impl_statements(units: &[CompilationUnit], impl_names: &[&str]) -> String {
            let sections: Vec<String> = impl_names
                .iter()
                .map(|impl_name| {
                    let statements = units
                        .iter()
                        .find_map(|unit| unit.implementations.iter().find(|imp| &imp.name == impl_name))
                        .map(|imp| {
                            imp.statements.iter().map(|node| node.as_string()).collect::<Vec<_>>().join("\n")
                        })
                        .unwrap_or_default();

                    format!("// Statements in {impl_name}\n{statements}")
                })
                .collect();

            sections.join("\n\n")
        }

        /// Serializes a struct DataType to a readable string.
        fn serialize_struct_type(data_type: &DataType) -> String {
            let DataType::StructType { name, variables } = data_type else {
                return format!("{:?}", data_type);
            };

            let name = name.as_deref().unwrap_or("<anonymous>");
            let fields: Vec<_> = variables
                .iter()
                .map(|var| {
                    format!("    {}: {};", var.name, serialize_data_type_decl(&var.data_type_declaration))
                })
                .collect();

            format!("{name} {{\n{}\n}}", fields.join("\n"))
        }

        /// Serializes a DataTypeDeclaration to a compact string.
        fn serialize_data_type_decl(decl: &DataTypeDeclaration) -> String {
            match decl {
                DataTypeDeclaration::Reference { referenced_type, .. } => referenced_type.clone(),
                DataTypeDeclaration::Definition { data_type, .. } => match data_type.as_ref() {
                    DataType::PointerType { referenced_type, .. } => {
                        format!("POINTER TO {}", serialize_data_type_decl(referenced_type))
                    }
                    other => format!("{:?}", other),
                },
                other => format!("{:?}", other),
            }
        }
    }
}
