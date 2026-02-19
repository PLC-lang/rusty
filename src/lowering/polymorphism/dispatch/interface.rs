use plc_ast::{
    ast::{
        Allocation, AstFactory, AstNode, AstStatement, CallStatement, CompilationUnit, DataTypeDeclaration,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    resolver::{AnnotationMap, AnnotationMapImpl},
    typesystem::DataType,
};

const FATPOINTER_TYPE_NAME: &str = "__FATPOINTER";
const FATPOINTER_DATA_FIELD_NAME: &str = "data";
const FATPOINTER_TABLE_FIELD_NAME: &str = "table";

pub struct InterfaceDispatchLowerer<'a> {
    ids: IdProvider,
    index: &'a Index,
    annotations: &'a AnnotationMapImpl,

    // TODO: This might be obsolote if we decide to make the `__FATPOINTER` struct a builtin
    /// Do we need to generate the `__FATPOINTER` struct definition?
    needs_fatpointer_definition: bool,

    /// Are we in a call statement and if so in how many depths?
    call_depth: usize,

    /// Are we in an assignment and if so what is its left side?
    assignment_ctx: Option<Box<AstNode>>,

    /// Replacement statements for the current assignment (filled by `visit_reference_expr`).
    assignment_preamble: Vec<AstNode>,

    /// Preamble statements for the enclosing call (filled by `visit_reference_expr`,
    /// drained by `visit_call_statement`).
    call_preamble: Vec<AstNode>,

    /// Monotonic counter for generating unique alloca names.
    alloca_counter: u32,
}

impl<'a> InterfaceDispatchLowerer<'a> {
    pub fn new(ids: IdProvider, index: &'a Index, annotations: &'a AnnotationMapImpl) -> Self {
        Self {
            ids,
            index,
            annotations,
            needs_fatpointer_definition: false,
            call_depth: 0,
            assignment_ctx: None,
            assignment_preamble: Vec::new(),
            call_preamble: Vec::new(),
            alloca_counter: 0,
        }
    }

    pub fn lower(&mut self, units: &mut [CompilationUnit]) {
        for unit in &mut *units {
            self.visit_compilation_unit(unit);
        }

        if self.needs_fatpointer_definition {
            units[0].user_types.push(helper::create_fat_pointer_struct());
        }
    }

    /// Returns a unique name for a temporary fat pointer alloca (e.g. `__fatpointer_0`).
    fn next_fatpointer_alloca_name(&mut self) -> String {
        let n = self.alloca_counter;
        self.alloca_counter += 1;
        format!("__fatpointer_{n}")
    }
}

impl<'a> AstVisitorMut for InterfaceDispatchLowerer<'a> {
    /// Replace any datatype declaration that resolves to an interface type with a `__FATPOINTER`
    fn visit_data_type_declaration(&mut self, data_type_declaration: &mut DataTypeDeclaration) {
        if let DataTypeDeclaration::Reference { referenced_type, .. } = data_type_declaration {
            if self.index.find_effective_type_by_name(referenced_type).is_some_and(DataType::is_interface) {
                helper::replace_datatype_with_fatpointer(data_type_declaration);
                self.needs_fatpointer_definition = true;
            }
        }

        data_type_declaration.walk(self);
    }

    /// Walk into interface declarations so that interface method parameters with
    /// interface types also get rewritten to `__FATPOINTER` (matching the implementing POUs).
    fn visit_interface(&mut self, interface: &mut plc_ast::ast::Interface) {
        for method in &mut interface.methods {
            self.visit_pou(method);
        }
    }

    fn visit_assignment(&mut self, node: &mut plc_ast::ast::AstNode) {
        let AstStatement::Assignment(assignment) = &mut node.stmt else { unreachable!() };

        assignment.left.walk(self);

        // Save and restore `assignment_ctx` around the walk so that nested assignments
        // don't clobber the outer context. For example in `result := consumer(in := instance)`,
        // the inner named-parameter assignment `in := instance` would otherwise overwrite the
        // outer `result` context, causing `visit_call_statement` to misidentify which
        // assignment the preamble belongs to.
        let prev_ctx = self.assignment_ctx.take();
        self.assignment_ctx = Some(assignment.left.clone());
        assignment.right.walk(self);

        if !self.assignment_preamble.is_empty() {
            node.stmt = AstStatement::ExpressionList(std::mem::take(&mut self.assignment_preamble));
        }

        self.assignment_ctx = prev_ctx;
    }

    /// Lowers interface method calls and wraps fat-pointer preambles around call statements.
    ///
    /// For interface method calls (`ref.foo(args)` where `ref` is interface-typed), the call
    /// is transformed into an indirect call through the itable:
    ///   `ref.foo(args)` → `__itable_IA#(ref.table^).foo^(ref.data, args)`
    ///
    /// The transformation is applied after walking children so that nested interface calls
    /// (e.g. `ref.foo(x := ref.bar())`) are lowered bottom-up.
    fn visit_call_statement(&mut self, node: &mut AstNode) {
        {
            let AstStatement::CallStatement(call) = &mut node.stmt else { unreachable!() };

            self.call_depth += 1;
            call.walk(self);
            self.call_depth -= 1; // XXX: saturating_sub might be safer, in theory we should not need it 🤞
        }

        // Check if this is a method call on an interface-typed variable (e.g. `ref.foo(...)`)
        // by inspecting the operator's base annotation (`ref` in `ref.foo(...)`)
        let interface_name = node
            .get_call_operator()
            .and_then(|op| op.get_base_ref_expr())
            .and_then(|base| self.annotations.get_type(base, self.index))
            .filter(|ty| ty.is_interface())
            .map(|ty| ty.get_name().to_string());

        // Lower: ref.foo(args) → __itable_IA#(ref.table^).foo^(ref.data, args)
        if let Some(interface_name) = interface_name {
            let AstStatement::CallStatement(CallStatement { operator, parameters }) = &mut node.stmt else {
                unreachable!()
            };

            // Step 1: Prepend `base.data` as the implicit first argument
            // ref.foo(args)  →  ref.foo(ref.data^, args)
            // NOTE: The deref (`^`) is required because `data` is a void-pointer field
            // containing the address of the concrete instance; we need to load that
            // pointer so the callee receives a `ptr` to the instance, not a `ptr*` to
            // the fat-pointer's data slot.
            {
                let base = operator.get_base_ref_expr().expect("interface call must have a base");
                let data_member = AstFactory::create_member_reference(
                    AstFactory::create_identifier(
                        FATPOINTER_DATA_FIELD_NAME,
                        SourceLocation::internal(),
                        self.ids.next_id(),
                    ),
                    Some(base.clone()),
                    self.ids.next_id(),
                );
                let data_ref = AstFactory::create_deref_reference(
                    data_member,
                    self.ids.next_id(),
                    SourceLocation::internal(),
                );

                match parameters {
                    None => {
                        parameters.replace(Box::new(data_ref));
                    }

                    Some(ref mut expr) => match &mut expr.stmt {
                        AstStatement::ExpressionList(expressions) => {
                            expressions.insert(0, data_ref);
                        }

                        _ => {
                            let mut expressions = Box::new(AstFactory::create_expression_list(
                                vec![data_ref, std::mem::take(expr)],
                                SourceLocation::internal(),
                                self.ids.next_id(),
                            ));

                            std::mem::swap(expr, &mut expressions);
                        }
                    },
                }
            }

            // Step 2: Replace the operator base with a dereferenced `.table` access
            // ref.foo  →  ref.table^.foo
            {
                let old_base = operator.get_base_ref_expr_mut().expect("interface call must have a base");
                let mut new_base = AstFactory::create_deref_reference(
                    AstFactory::create_member_reference(
                        AstFactory::create_identifier(
                            FATPOINTER_TABLE_FIELD_NAME,
                            SourceLocation::internal(),
                            self.ids.next_id(),
                        ),
                        Some(std::mem::take(old_base)),
                        self.ids.next_id(),
                    ),
                    self.ids.next_id(),
                    SourceLocation::internal(),
                );

                std::mem::swap(old_base, &mut new_base);
            }

            // Step 3: Cast the itable access to the concrete itable type
            // ref.table^.foo  →  __itable_IA#(ref.table^).foo
            {
                let old_base = operator.get_base_ref_expr_mut().expect("interface call must have a base");
                let old_base_paren = AstFactory::create_paren_expression(
                    std::mem::take(old_base),
                    SourceLocation::internal(),
                    self.ids.next_id(),
                );

                let mut new_base = AstFactory::create_cast_statement(
                    AstFactory::create_member_reference(
                        AstFactory::create_identifier(
                            format!("__itable_{interface_name}"),
                            SourceLocation::internal(),
                            self.ids.next_id(),
                        ),
                        None,
                        self.ids.next_id(),
                    ),
                    old_base_paren,
                    &SourceLocation::internal(),
                    self.ids.next_id(),
                );

                std::mem::swap(old_base, &mut new_base);
            }

            // Step 4: Dereference the function pointer
            // __itable_IA#(ref.table^).foo  →  __itable_IA#(ref.table^).foo^
            {
                let mut deref = AstFactory::create_deref_reference(
                    std::mem::take(operator.as_mut()),
                    self.ids.next_id(),
                    SourceLocation::internal(),
                );

                std::mem::swap(operator.as_mut(), &mut deref);
            }
        }

        // Wrap preamble + original call into an ExpressionList.
        //
        // When the call is the RHS of an assignment (e.g. `result := ref.foo(instance)`),
        // the preamble must be hoisted *above* the assignment so the result becomes:
        //   `alloca ..., tmp.data := ..., tmp.table := ..., result := ref.foo(tmp)`
        // rather than incorrectly nesting everything under the assignment:
        //   `result := alloca ..., tmp.data := ..., tmp.table := ..., ref.foo(tmp)`
        //
        // We detect this case via `assignment_ctx`: if set, the enclosing `visit_assignment`
        // is waiting for us. We consume the context, build `[preamble..., left := call]`,
        // and route it through `assignment_preamble` so `visit_assignment` can replace
        // the assignment node with the complete expression list.
        //
        // When there is no enclosing assignment (e.g. a bare `ref.foo(instance)` call),
        // we wrap the call node directly.
        if self.call_depth == 0 && !self.call_preamble.is_empty() {
            let mut statements = std::mem::take(&mut self.call_preamble);
            let original_call = std::mem::replace(
                node,
                AstNode {
                    stmt: AstStatement::EmptyStatement(plc_ast::ast::EmptyStatement {}),
                    id: self.ids.next_id(),
                    location: SourceLocation::internal(),
                    metadata: None,
                },
            );
            if let Some(left) = self.assignment_ctx.take() {
                statements.push(AstFactory::create_assignment(*left, original_call, self.ids.next_id()));
                self.assignment_preamble = statements;
            } else {
                statements.push(original_call);
                node.stmt = AstStatement::ExpressionList(statements);
            }
        }
    }

    fn visit_reference_expr(&mut self, node: &mut plc_ast::ast::AstNode) {
        let Some(ty) = self.annotations.get_type(node, self.index) else { return };
        let Some(ty_hint) = self.annotations.get_type_hint(node, self.index) else { return };

        if ty.is_interface() || !ty_hint.is_interface() {
            return;
        }

        // Call argument: `consumer(instance)` where `instance` is (potentially) a concrete POU
        // and the parameter expects an interface type.
        if self.call_depth > 0 {
            let interface_name = ty_hint.get_name();
            let pou_name = ty.get_name();
            let tmp_name = self.next_fatpointer_alloca_name();

            // Reference node for the temporary fat pointer
            let tmp_ref = AstFactory::create_member_reference(
                AstFactory::create_identifier(&tmp_name, SourceLocation::internal(), self.ids.next_id()),
                None,
                self.ids.next_id(),
            );

            // alloca __fatpointer_N : __FATPOINTER
            let alloca = AstNode {
                stmt: AstStatement::AllocationStatement(Allocation {
                    name: tmp_name.clone(),
                    reference_type: FATPOINTER_TYPE_NAME.to_string(),
                }),
                id: self.ids.next_id(),
                location: SourceLocation::internal(),
                metadata: None,
            };
            self.call_preamble.push(alloca);

            // __fatpointer_N.data := ADR(node)
            let assign_data = helper::create_fat_pointer_field_assignment(
                &mut self.ids,
                &tmp_ref,
                FATPOINTER_DATA_FIELD_NAME,
                node,
            );
            self.call_preamble.push(assign_data);

            // __fatpointer_N.table := ADR(__itable_<interface>_<pou>_instance)
            let itable_ref = AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    format!("__itable_{interface_name}_{pou_name}_instance"),
                    SourceLocation::internal(),
                    self.ids.next_id(),
                ),
                None,
                self.ids.next_id(),
            );
            let assign_table = helper::create_fat_pointer_field_assignment(
                &mut self.ids,
                &tmp_ref,
                FATPOINTER_TABLE_FIELD_NAME,
                &itable_ref,
            );
            self.call_preamble.push(assign_table);

            // Replace the argument with a reference to __fatpointer_N
            *node = AstFactory::create_member_reference(
                AstFactory::create_identifier(&tmp_name, SourceLocation::internal(), self.ids.next_id()),
                None,
                self.ids.next_id(),
            );

            return;
        }

        // Boring assignment, something like `left := right`
        if self.call_depth == 0 {
            if let Some(left) = self.assignment_ctx.take() {
                let interface_name = ty_hint.get_name();
                let pou_name = ty.get_name();

                // left.data := ADR(right)
                let assign_data = helper::create_fat_pointer_field_assignment(
                    &mut self.ids,
                    &left,
                    FATPOINTER_DATA_FIELD_NAME,
                    node,
                );

                // left.table := ADR(__itable_<interface>_<pou>_instance)
                let itable_ref = AstFactory::create_member_reference(
                    AstFactory::create_identifier(
                        format!("__itable_{interface_name}_{pou_name}_instance"),
                        SourceLocation::internal(),
                        self.ids.next_id(),
                    ),
                    None,
                    self.ids.next_id(),
                );
                let assign_table = helper::create_fat_pointer_field_assignment(
                    &mut self.ids,
                    &left,
                    FATPOINTER_TABLE_FIELD_NAME,
                    &itable_ref,
                );

                self.assignment_preamble = vec![assign_data, assign_table];
            }
        }
    }
}

mod helper {
    use plc_ast::{
        ast::{
            AstFactory, AstNode, DataType, DataTypeDeclaration, LinkageType, UserTypeDeclaration, Variable,
        },
        provider::IdProvider,
    };
    use plc_source::source_location::SourceLocation;

    use crate::{
        lowering::polymorphism::dispatch::interface::{
            FATPOINTER_DATA_FIELD_NAME, FATPOINTER_TABLE_FIELD_NAME, FATPOINTER_TYPE_NAME,
        },
        typesystem::VOID_INTERNAL_NAME,
    };

    pub fn replace_datatype_with_fatpointer(type_decl: &mut DataTypeDeclaration) {
        *type_decl = DataTypeDeclaration::Reference {
            referenced_type: String::from(FATPOINTER_TYPE_NAME),
            location: type_decl.get_location(),
        };
    }

    pub fn create_fat_pointer_struct() -> UserTypeDeclaration {
        /// Creates a struct member of type `POINTER TO __VOID`.
        fn create_void_pointer_member(name: &str, location: &SourceLocation) -> Variable {
            Variable {
                name: name.to_string(),
                data_type_declaration: DataTypeDeclaration::Definition {
                    data_type: Box::new(DataType::PointerType {
                        name: None,
                        referenced_type: Box::new(DataTypeDeclaration::Reference {
                            referenced_type: VOID_INTERNAL_NAME.to_string(),
                            location: location.clone(),
                        }),
                        auto_deref: None,
                        type_safe: false,
                        is_function: false,
                    }),
                    location: location.clone(),
                    scope: None,
                },
                initializer: None,
                address: None,
                location: location.clone(),
            }
        }

        let location = SourceLocation::internal();

        UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some(String::from(FATPOINTER_TYPE_NAME)),
                variables: vec![
                    create_void_pointer_member(FATPOINTER_DATA_FIELD_NAME, &location),
                    create_void_pointer_member(FATPOINTER_TABLE_FIELD_NAME, &location),
                ],
            },
            initializer: None,
            location,
            scope: None,
            linkage: LinkageType::Internal,
        }
    }

    /// Creates `<base>.<field> := ADR(<target>)`.
    pub fn create_fat_pointer_field_assignment(
        ids: &mut IdProvider,
        base: &AstNode,
        field: &str,
        target: &AstNode,
    ) -> AstNode {
        let location = SourceLocation::internal();

        // LHS: <base>.<field>
        let lhs = AstFactory::create_member_reference(
            AstFactory::create_identifier(field, &location, ids.next_id()),
            Some(base.clone()),
            ids.next_id(),
        );

        // RHS: ADR(<target>)
        let rhs = AstFactory::create_call_statement(
            AstFactory::create_member_reference(
                AstFactory::create_identifier("ADR", &location, ids.next_id()),
                None,
                ids.next_id(),
            ),
            Some(target.clone()),
            ids.next_id(),
            &location,
        );

        AstFactory::create_assignment(lhs, rhs, ids.next_id())
    }
}

#[cfg(test)]
mod tests {
    use crate::lowering::polymorphism::dispatch::interface::{
        tests::helper::lower_and_serialize_statements, FATPOINTER_TYPE_NAME,
    };

    #[test]
    fn fatpointer_is_generated_on_demand() {
        // Initially, no POU makes use of a interface as a variable
        {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA
                END_FUNCTION_BLOCK
            "#;

            let annotated_unit = helper::lower(source);
            let mut user_types = annotated_unit.get_unit().user_types.iter();
            assert!(!user_types.any(|ty| ty.data_type.get_name().unwrap() == FATPOINTER_TYPE_NAME));
        }

        // However, if the interface is used as a variable, a fat-pointer MUST be generated
        {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA
                    VAR
                        localVariable: IA;
                    END_VAR
                END_FUNCTION_BLOCK
            "#;

            let annotated_unit = helper::lower(source);
            let mut user_types = annotated_unit.get_unit().user_types.iter();
            assert!(user_types.any(|ty| ty.data_type.get_name().unwrap() == FATPOINTER_TYPE_NAME));
        }
    }

    #[test]
    fn fatpointer_replaces_function_return_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION foo: IA
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous, @r#"
        [
            POU {
                name: "foo",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "foo",
                                data_type: DataTypeReference {
                                    referenced_type: "__FATPOINTER",
                                },
                            },
                        ],
                        variable_block_type: InOut,
                    },
                ],
                pou_type: Function,
                return_type: Some(
                    Aggregate {
                        referenced_type: "__FATPOINTER",
                    },
                ),
                interfaces: [],
                properties: [],
            },
        ]
        "#);
    }

    #[test]
    fn fatpointer_replaces_interface_variable_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION main
                VAR
                    localVariable: IA;
                END_VAR

                VAR_INPUT
                    inVariable: IA;
                END_VAR

                VAR_OUTPUT
                    outVariable: IA;
                END_VAR

                VAR_IN_OUT
                    inOutVariable: IA;
                END_VAR
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous[0].variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Local,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Input(
                    ByVal,
                ),
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "outVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Output,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inOutVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: InOut,
            },
        ]
        "#);
    }

    #[test]
    fn fatpointer_replaces_interface_variable_aliased_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            TYPE
                AliasedIA: IA;
            END_TYPE

            FUNCTION main
                VAR
                    localVariable: AliasedIA;
                END_VAR

                VAR_INPUT
                    inVariable: AliasedIA;
                END_VAR

                VAR_OUTPUT
                    outVariable: AliasedIA;
                END_VAR

                VAR_IN_OUT
                    inOutVariable: AliasedIA;
                END_VAR
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous[0].variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Local,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Input(
                    ByVal,
                ),
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "outVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Output,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inOutVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: InOut,
            },
        ]
        "#);
    }

    #[test]
    fn fatpointer_replaces_interface_variable_array_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION main
                VAR
                    localVariable: ARRAY[1..2] OF IA;
                    localVariableNested: ARRAY[1..2] OF ARRAY[3..4] OF IA;
                    localVariableNestedNested: ARRAY[1..2] OF ARRAY[3..4] OF ARRAY[5..6] OF IA;
                END_VAR
            END_FUNCTION
        "#;

        // TODO: snapshot needs to resolve to inner-most type, I want to see __FATPOINTER here
        // Put differently, the replacement of these datatypes works, but the snapshot doesn't reflect that
        // because it does currently not resolve these internal `__main_...` types.
        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous[0].variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__main_localVariable",
                        },
                    },
                    Variable {
                        name: "localVariableNested",
                        data_type: DataTypeReference {
                            referenced_type: "__main_localVariableNested",
                        },
                    },
                    Variable {
                        name: "localVariableNestedNested",
                        data_type: DataTypeReference {
                            referenced_type: "__main_localVariableNestedNested",
                        },
                    },
                ],
                variable_block_type: Local,
            },
        ]
        "#);
    }

    #[test]
    fn assignments_expand() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                VAR
                    localInstance: FbA;
                    localReference: IA;
                END_VAR

                VAR_INPUT
                    in: IA;
                    instance: FbA;
                END_VAR

                in := instance;
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR

                reference := instance;
                instance.localReference := instance.localInstance;
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main", "FbA"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "instance.localReference.data := ADR(instance.localInstance), instance.localReference.table := ADR(__itable_IA_FbA_instance)",
            "// Statements in FbA",
            "in.data := ADR(instance), in.table := ADR(__itable_IA_FbA_instance)",
        ]
        "#)
    }

    #[test]
    fn array_assignments_expand() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceA: FbA;
                    instanceB: FbB;
                    references: ARRAY[1..2] OF IA;
                END_VAR

                references[1] := instanceA;
                references[2] := instanceB;
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instanceA)",
            "__init_fbb(instanceB)",
            "__user_init_FbA(instanceA)",
            "__user_init_FbB(instanceB)",
            "references[1].data := ADR(instanceA), references[1].table := ADR(__itable_IA_FbA_instance)",
            "references[2].data := ADR(instanceB), references[2].table := ADR(__itable_IA_FbB_instance)",
        ]
        "#)
    }

    #[test]
    fn call_argument_is_expanded() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance: FbA;
                END_VAR

                consumer(instance);
                consumer(in := instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), consumer(__fatpointer_0)",
            "alloca __fatpointer_1: __FATPOINTER, __fatpointer_1.data := ADR(instance), __fatpointer_1.table := ADR(__itable_IA_FbA_instance), consumer(in := __fatpointer_1)",
        ]
        "#);
    }

    #[test]
    fn call_arguments_are_expanded() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    inOne, inTwo, inThree: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instanceA: FbA;
                    instanceB: FbB;
                    instanceC: FbC;
                END_VAR

                consumer(instanceA, instanceB, instanceC);
                consumer(inOne := instanceA, inTwo := instanceB, inThree := instanceC);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instanceA)",
            "__init_fbb(instanceB)",
            "__init_fbc(instanceC)",
            "__user_init_FbA(instanceA)",
            "__user_init_FbB(instanceB)",
            "__user_init_FbC(instanceC)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instanceA), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), alloca __fatpointer_1: __FATPOINTER, __fatpointer_1.data := ADR(instanceB), __fatpointer_1.table := ADR(__itable_IA_FbB_instance), alloca __fatpointer_2: __FATPOINTER, __fatpointer_2.data := ADR(instanceC), __fatpointer_2.table := ADR(__itable_IA_FbC_instance), consumer(__fatpointer_0, __fatpointer_1, __fatpointer_2)",
            "alloca __fatpointer_3: __FATPOINTER, __fatpointer_3.data := ADR(instanceA), __fatpointer_3.table := ADR(__itable_IA_FbA_instance), alloca __fatpointer_4: __FATPOINTER, __fatpointer_4.data := ADR(instanceB), __fatpointer_4.table := ADR(__itable_IA_FbB_instance), alloca __fatpointer_5: __FATPOINTER, __fatpointer_5.data := ADR(instanceC), __fatpointer_5.table := ADR(__itable_IA_FbC_instance), consumer(inOne := __fatpointer_3, inTwo := __fatpointer_4, inThree := __fatpointer_5)",
        ]
        "#);
    }

    #[test]
    fn nesting_single_depth_wrapping() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION producer : DINT
                VAR_INPUT
                    ref: IA;
                END_VAR
            END_FUNCTION

            FUNCTION consumer
                VAR_INPUT
                    value: DINT;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance: FbA;
                END_VAR

                consumer(producer(instance));
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), consumer(producer(__fatpointer_0))",
        ]
        "#);
    }

    #[test]
    fn nesting_multi_depth_wrapping_with_mixed_args() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION inner : DINT
                VAR_INPUT
                    ref: IA;
                END_VAR
            END_FUNCTION

            FUNCTION middle : DINT
                VAR_INPUT
                    a: DINT;
                    b: IA;
                END_VAR
            END_FUNCTION

            FUNCTION outer
                VAR_INPUT
                    a: DINT;
                    b: IA;
                    c: DINT;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    x: FbA;
                    y: FbA;
                    z: FbA;
                END_VAR

                outer(middle(inner(x), y), z, 42);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(x)",
            "__init_fba(y)",
            "__init_fba(z)",
            "__user_init_FbA(x)",
            "__user_init_FbA(y)",
            "__user_init_FbA(z)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(x), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), alloca __fatpointer_1: __FATPOINTER, __fatpointer_1.data := ADR(y), __fatpointer_1.table := ADR(__itable_IA_FbA_instance), alloca __fatpointer_2: __FATPOINTER, __fatpointer_2.data := ADR(z), __fatpointer_2.table := ADR(__itable_IA_FbA_instance), outer(middle(inner(__fatpointer_0), __fatpointer_1), __fatpointer_2, 42)",
        ]
        "#);
    }

    #[test]
    fn interface_method_call_simple() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR

                reference := instance;
                reference.foo();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "__itable_IA#(reference.table^).foo^(reference.data^)",
        ]
        "#);
    }

    #[test]
    fn interface_method_call_with_arguments() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                    VAR_INPUT
                        a: DINT;
                        b: DINT;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                    VAR_INPUT
                        a: DINT;
                        b: DINT;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR

                reference := instance;
                reference.foo(1, 2);
                reference.foo(a := 10, b := 20);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "__itable_IA#(reference.table^).foo^(reference.data^, 1, 2)",
            "__itable_IA#(reference.table^).foo^(reference.data^, a := 10, b := 20)",
        ]
        "#);
    }

    #[test]
    fn interface_method_call_nested() {
        let source = r#"
            INTERFACE IA
                METHOD foo : DINT
                    VAR_INPUT
                        x: DINT;
                    END_VAR
                END_METHOD

                METHOD bar : DINT
                END_METHOD

                METHOD baz
                    VAR_INPUT
                        a: DINT;
                        b: DINT;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : DINT
                    VAR_INPUT
                        x: DINT;
                    END_VAR
                END_METHOD

                METHOD bar : DINT
                END_METHOD

                METHOD baz
                    VAR_INPUT
                        a: DINT;
                        b: DINT;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR

                reference := instance;
                reference.baz(reference.foo(reference.bar()), 42);
                reference.baz(a := reference.foo(x := reference.bar()), b := 42);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "__itable_IA#(reference.table^).baz^(reference.data^, __itable_IA#(reference.table^).foo^(reference.data^, __itable_IA#(reference.table^).bar^(reference.data^)), 42)",
            "__itable_IA#(reference.table^).baz^(reference.data^, a := __itable_IA#(reference.table^).foo^(reference.data^, x := __itable_IA#(reference.table^).bar^(reference.data^)), b := 42)",
        ]
        "#);
    }

    #[test]
    fn interface_method_call_with_aggregate_return() {
        let source = r#"
            INTERFACE IA
                METHOD foo : STRING
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : STRING
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                    result: STRING;
                END_VAR

                reference := instance;
                result := reference.foo();
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "alloca __0: STRING, __itable_IA#(reference.table^).foo^(reference.data^, __0), result := __0",
        ]
        "#);
    }

    #[test]
    #[ignore = "TODO: currently incorrect, instance is not lowered to temporary alloca because of string return type"]
    fn interface_method_call_with_aggregate_return_and_interface_argument() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer : STRING
                VAR_INPUT
                    reference: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance: FbA;
                    result: STRING;
                END_VAR

                result := consumer(instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "alloca __consumer0: STRING, consumer(__consumer0, instance), result := __consumer0",
        ]
        "#);
    }

    #[test]
    #[ignore = "TODO: Parse is unable to call result of another function"]
    fn nesting_chained_method_calls_with_wrapping() {
        let source = r#"
            INTERFACE IA
                METHOD transform : IA
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD transform : IA
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION producer : IA
                VAR_INPUT
                    ref: IA;
                END_VAR
            END_FUNCTION

            FUNCTION consumer
                VAR_INPUT
                    a: IA;
                    b: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    x: FbA;
                    y: FbA;
                    z: FbA;
                END_VAR

                consumer(a := producer(x).transform(producer(y)), b := z);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#""#);
    }

    #[test]
    fn assignment_on_call_with_interface_argument() {
        let source = r#"
            INTERFACE IA
                METHOD foo : DINT
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : DINT
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                    result: DINT;
                END_VAR

                reference := instance;
                result := reference.foo(instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), result := __itable_IA#(reference.table^).foo^(reference.data^, __fatpointer_0)",
        ]
        "#);
    }

    #[test]
    fn assignment_on_call_with_named_interface_argument() {
        let source = r#"
            INTERFACE IA
                METHOD foo : DINT
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo : DINT
                    VAR_INPUT
                        ref: IA;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                    result: DINT;
                END_VAR

                reference := instance;
                result := reference.foo(ref := instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "alloca __fatpointer_0: __FATPOINTER, __fatpointer_0.data := ADR(instance), __fatpointer_0.table := ADR(__itable_IA_FbA_instance), result := __itable_IA#(reference.table^).foo^(reference.data^, ref := __fatpointer_0)",
        ]
        "#);
    }

    mod helper {
        use driver::{parse_and_annotate, pipelines::AnnotatedProject, pipelines::AnnotatedUnit};
        use plc_source::SourceCode;

        pub fn lower(source: impl Into<SourceCode>) -> AnnotatedUnit {
            let (_, mut project): (_, AnnotatedProject) =
                parse_and_annotate("unit-test", vec![source.into()]).unwrap();

            // (project.index, project.units.remove(0))
            project.units.remove(0)
        }

        pub fn lower_and_serialize_statements(source: impl Into<SourceCode>, pous: &[&str]) -> Vec<String> {
            let (_, project) = parse_and_annotate("unit-test", vec![source.into()]).unwrap();
            let unit = project.units[0].get_unit();

            let mut result = Vec::new();
            for pou in pous {
                result.push(format!("// Statements in {pou}"));
                let statements = &unit.implementations.iter().find(|it| &it.name == pou).unwrap().statements;
                let statements_str =
                    statements.iter().map(|statement| statement.as_string()).collect::<Vec<_>>();

                result.extend(statements_str);
            }

            result
        }
    }
}
