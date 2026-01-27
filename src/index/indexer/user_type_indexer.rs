use plc_ast::{
    ast::{
        flatten_expression_list, get_enum_element_name, Assignment, AstFactory, AstNode, AstStatement,
        AutoDerefType, DataType, DataTypeDeclaration, RangeStatement, TypeNature, UserTypeDeclaration,
        Variable,
    },
    literals::AstLiteral,
    visitor::{AstVisitor, Walker},
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::{
    index::{
        const_expressions::ConstId, ArgumentType, HardwareBinding, Index, MemberInfo, VariableIndexEntry,
        VariableType,
    },
    typesystem::{
        self, DataTypeInformation, Dimension, StringEncoding, StructSource, TypeSize, DEFAULT_STRING_LEN,
        DINT_TYPE,
    },
};

/// Indexer that registers all user-defined types in the index
pub struct UserTypeIndexer<'i, 't> {
    index: &'i mut Index,
    user_type: &'t UserTypeDeclaration,
    pending_initializer: Option<ConstId>,
}

impl<'i, 't> UserTypeIndexer<'i, 't> {
    pub fn new(index: &'i mut Index, user_type: &'t UserTypeDeclaration) -> Self {
        UserTypeIndexer { index, user_type, pending_initializer: None }
    }

    fn collect_initializer(&mut self, i: AstNode, target_type_name: String) -> ConstId {
        let scope = self.current_scope();
        let init =
            self.index.get_mut_const_expressions().add_constant_expression(i, target_type_name, scope, None);
        init
    }
}

impl AstVisitor for UserTypeIndexer<'_, '_> {
    fn visit_user_type_declaration(&mut self, user_type: &UserTypeDeclaration) {
        //handle inner types & initializers
        user_type.data_type.walk(self);
        if let Some(init_index) = user_type.initializer.clone().map(|i| {
            self.collect_initializer(i, user_type.data_type.get_name().map(|s| s.to_string()).unwrap())
            // XXX(mihr): is unwrapping fine here?
        }) {
            //TODO: without field?
            self.pending_initializer = Some(init_index);
        };

        self.visit_data_type(&user_type.data_type);
    }

    fn visit_data_type(&mut self, data_type: &DataType) {
        match &data_type {
            DataType::StructType { name: Some(name), variables } => {
                self.index_struct_type(name, variables, StructSource::OriginalDeclaration)
            }
            DataType::EnumType { name: Some(name), numeric_type, elements } => {
                self.index_enum_type(name, numeric_type, elements)
            }
            DataType::SubRangeType { name: Some(name), referenced_type, bounds } => {
                self.index_sub_range_type(name, referenced_type, bounds.as_ref())
            }
            DataType::ArrayType { name: Some(name), bounds, referenced_type, is_variable_length: false } => {
                self.index_array_type(name, bounds, referenced_type)
            }
            DataType::ArrayType { name: Some(name), bounds, referenced_type, is_variable_length: true } => {
                self.index_vla_array(name, bounds, referenced_type)
            }
            DataType::PointerType { name, referenced_type, auto_deref, type_safe, is_function } => {
                self.index_pointer_type(name, referenced_type, *auto_deref, *type_safe, *is_function)
            }
            DataType::StringType { name: Some(name), is_wide, size } => {
                self.index_string_type(name.as_ref(), *is_wide, size.as_ref())
            }
            DataType::VarArgs { .. } => {
                // do nothing
            }
            DataType::GenericType { name, generic_symbol, nature } => {
                self.index_generic_type(name, generic_symbol, nature)
            }
            _ => {}
        }
    }
}

impl UserTypeIndexer<'_, '_> {
    fn current_scope(&self) -> Option<String> {
        self.user_type.scope.clone()
    }

    fn index_vla_array(&mut self, name: &str, bounds: &AstNode, referenced_type: &DataTypeDeclaration) {
        let ndims = match bounds.get_stmt() {
            AstStatement::VlaRangeStatement => 1,
            AstStatement::ExpressionList(expressions) => expressions.len(),
            _ => unreachable!("not a bounds statement"),
        };

        let referenced_type = referenced_type.get_name().expect("named datatype").to_string();
        let struct_name = name.to_owned();

        let dummy_array_name = format!("{struct_name}_vla_{ndims}_{referenced_type}").to_lowercase();
        let member_array_name = format!("__ptr_to_{dummy_array_name}");
        let member_dimensions_name = format!("__bounds_{dummy_array_name}");

        // check the index if a dummy-array type matching the given VLA (eg. 1 dimension, type INT) already exists.
        // if we find a type, we can use references to the internal types. otherwise, register the array in the index
        // and declare internal member types.
        let (vla_arr_type_declaration, dim_arr_type_declaration) =
            if self.index.get_effective_type_by_name(&dummy_array_name).is_ok() {
                (
                    DataTypeDeclaration::Reference {
                        referenced_type: member_array_name,
                        location: SourceLocation::internal(),
                    },
                    DataTypeDeclaration::Reference {
                        referenced_type: member_dimensions_name,
                        location: SourceLocation::internal(),
                    },
                )
            } else {
                // register dummy array type so it can later be annotated as a type hint
                self.index.register_type(typesystem::DataType {
                    name: dummy_array_name.clone(),
                    initial_value: None,
                    information: DataTypeInformation::Array {
                        name: dummy_array_name.clone(),
                        inner_type_name: referenced_type.clone(),
                        // dummy dimensions that will never actually be used
                        dimensions: (0..ndims)
                            .map(|_| Dimension {
                                start_offset: TypeSize::Undetermined,
                                end_offset: TypeSize::Undetermined,
                            })
                            .collect::<Vec<_>>(),
                    },
                    nature: TypeNature::__VLA,
                    location: SourceLocation::internal(),
                });

                // define internal vla members
                (
                    DataTypeDeclaration::Definition {
                        data_type: Box::new(DataType::PointerType {
                            name: Some(member_array_name),
                            referenced_type: Box::new(DataTypeDeclaration::Reference {
                                referenced_type: dummy_array_name,
                                location: SourceLocation::internal(),
                            }),
                            auto_deref: None,
                            type_safe: true,
                            is_function: false,
                        }),
                        location: SourceLocation::internal(),
                        scope: None,
                    },
                    DataTypeDeclaration::Definition {
                        data_type: Box::new(DataType::ArrayType {
                            name: Some(member_dimensions_name),
                            bounds: AstNode::new(
                                AstStatement::ExpressionList(
                                    (0..ndims)
                                        .map(|_| {
                                            AstFactory::create_range_statement(
                                                AstNode::new_literal(
                                                    AstLiteral::new_integer(0),
                                                    0,
                                                    SourceLocation::internal(),
                                                ),
                                                AstNode::new_literal(
                                                    AstLiteral::new_integer(1),
                                                    0,
                                                    SourceLocation::internal(),
                                                ),
                                                0,
                                            )
                                        })
                                        .collect::<_>(),
                                ),
                                0,
                                SourceLocation::internal(),
                            ),
                            referenced_type: Box::new(DataTypeDeclaration::Reference {
                                referenced_type: DINT_TYPE.to_string(),
                                location: SourceLocation::internal(),
                            }),
                            is_variable_length: false,
                        }),
                        location: SourceLocation::internal(),
                        scope: None,
                    },
                )
            };

        // Create variable index entries for VLA struct members
        let variables = vec![
            // Pointer
            Variable {
                name: format!("struct_vla_{referenced_type}_{ndims}").to_lowercase(),
                data_type_declaration: vla_arr_type_declaration,
                initializer: None,
                address: None,
                location: SourceLocation::internal(),
            },
            // Dimensions Array
            Variable {
                name: "dimensions".to_string(),
                data_type_declaration: dim_arr_type_declaration,
                initializer: None,
                address: None,
                location: SourceLocation::internal(),
            },
        ];

        self.index_struct_type(
            &struct_name,
            &variables,
            StructSource::Internal(typesystem::InternalType::VariableLengthArray {
                inner_type_name: referenced_type,
                ndims,
            }),
        );
    }

    fn index_array_type(&mut self, name: &str, bounds: &AstNode, referenced_type: &DataTypeDeclaration) {
        let scope = self.current_scope();
        let dimensions: Result<Vec<Dimension>, Diagnostic> = bounds
            .get_as_list()
            .iter()
            .map(|it| match it.get_stmt() {
                AstStatement::RangeStatement(RangeStatement { start, end }) => {
                    let constants = self.index.get_mut_const_expressions();
                    Ok(Dimension {
                        start_offset: TypeSize::from_expression(constants.add_constant_expression(
                            *start.clone(),
                            typesystem::DINT_TYPE.to_string(),
                            scope.clone(),
                            None,
                        )),
                        end_offset: TypeSize::from_expression(constants.add_constant_expression(
                            *end.clone(),
                            typesystem::DINT_TYPE.to_string(),
                            scope.clone(),
                            None,
                        )),
                    })
                }

                _ => Err(Diagnostic::codegen_error("Invalid array definition: RangeStatement expected", *it)),
            })
            .collect();

        // TODO(mhasel, volsa): This unwrap will panic with `ARRAY[0..5, 5] OF DINT;`
        let dimensions = dimensions.unwrap();

        //TODO hmm we need to talk about all this unwrapping :-/
        let referenced_type_name = referenced_type.get_name().expect("named datatype");
        let information = DataTypeInformation::Array {
            name: name.to_string(),
            inner_type_name: referenced_type_name.to_string(),
            dimensions,
        };

        self.register_type(name, information, TypeNature::Any);
        let global_init_name = crate::index::get_initializer_name(name);

        // TODO unfortunately we cannot share const-expressions between multiple
        // index-entries
        let init = self
            .user_type
            .initializer
            .as_ref()
            .map(|i| self.collect_initializer(i.clone(), name.to_string()));
        if init.is_some() {
            let variable = VariableIndexEntry::create_global(
                global_init_name.as_str(),
                global_init_name.as_str(),
                name,
                self.user_type.location.clone(),
            )
            .set_constant(true)
            .set_initial_value(init);
            self.index.register_global_initializer(&global_init_name, variable);
        }
    }

    fn index_enum_type(&mut self, name: &str, numeric_type: &str, elements: &AstNode) {
        let mut variants = Vec::new();

        for ele in flatten_expression_list(elements) {
            let variant = get_enum_element_name(ele);
            if let AstStatement::Assignment(Assignment { right, .. }) = ele.get_stmt() {
                let scope = self.current_scope();
                let init = self.index.get_mut_const_expressions().add_constant_expression(
                    right.as_ref().clone(),
                    numeric_type.to_string(),
                    scope,
                    None,
                );

                variants.push(self.index.register_enum_variant(
                    name,
                    &variant,
                    Some(init),
                    ele.get_location(),
                ))
            } else {
                unreachable!("the preprocessor should have provided explicit assignments for enum values")
            }
        }

        let information = DataTypeInformation::Enum {
            name: name.to_owned(),
            variants,
            referenced_type: numeric_type.to_string(),
        };

        self.register_type(name, information, TypeNature::Int);
    }

    fn index_sub_range_type(&mut self, name: &str, referenced_type: &str, bounds: Option<&AstNode>) {
        let information = if let Some(AstStatement::RangeStatement(RangeStatement { start, end })) =
            bounds.as_ref().map(|it| it.get_stmt())
        {
            let scope = self.current_scope();
            let start_size = self.ast_node_to_type_size(start, &scope);
            let end_size = self.ast_node_to_type_size(end, &scope);

            DataTypeInformation::SubRange {
                name: name.into(),
                referenced_type: referenced_type.into(),
                sub_range: start_size..end_size,
            }
        } else {
            DataTypeInformation::Alias { name: name.into(), referenced_type: referenced_type.into() }
        };

        self.register_type(name, information, TypeNature::Int);
    }

    /// Converts an AstNode to a TypeSize, either as a literal or a const expression
    fn ast_node_to_type_size(&mut self, node: &AstNode, scope: &Option<String>) -> TypeSize {
        match &node.stmt {
            AstStatement::Literal(AstLiteral::Integer(value)) => TypeSize::from_literal(*value as i64),
            _ => TypeSize::from_expression(self.index.get_mut_const_expressions().add_constant_expression(
                node.clone(),
                DINT_TYPE.to_string(),
                scope.clone(),
                None,
            )),
        }
    }

    fn register_type(&mut self, name: &str, information: DataTypeInformation, nature: TypeNature) {
        self.index.register_type(typesystem::DataType {
            name: name.into(),
            initial_value: self.pending_initializer,
            information,
            nature,
            location: self.user_type.location.clone(),
        });
    }

    fn index_generic_type(&mut self, name: &str, generic_symbol: &str, nature: &TypeNature) {
        let information = DataTypeInformation::Generic {
            name: name.to_string(),
            generic_symbol: generic_symbol.to_string(),
            nature: *nature,
        };

        self.register_type(name, information, TypeNature::Any);
    }

    fn index_string_type(&mut self, name: &str, is_wide: bool, size: Option<&AstNode>) {
        let encoding = if is_wide { StringEncoding::Utf16 } else { StringEncoding::Utf8 };

        //TODO: handle the case where type_name is None
        let scope = self.current_scope();

        let size = match size {
            Some(AstNode { stmt: AstStatement::Literal(AstLiteral::Integer(value)), .. }) => {
                TypeSize::from_literal((value + 1) as i64)
            }
            Some(statement) => {
                // construct a "x + 1" expression because we need one additional character for \0 terminator
                let len_plus_1 = AstFactory::create_plus_one_expression(
                    statement.clone(),
                    statement.get_location(),
                    statement.get_id(),
                );

                TypeSize::from_expression(self.index.get_mut_const_expressions().add_constant_expression(
                    len_plus_1,
                    DINT_TYPE.to_string(),
                    scope,
                    None,
                ))
            }
            None => TypeSize::from_literal((DEFAULT_STRING_LEN + 1).into()),
        };
        let information = DataTypeInformation::String { size, encoding };
        self.register_type(name, information, TypeNature::String);

        //TODO: can we reuse this?
        if let Some(init) = self.pending_initializer {
            // register a global variable with the initial value to memcopy from
            let global_init_name = crate::index::get_initializer_name(name);
            let initializer_global = VariableIndexEntry::create_global(
                global_init_name.as_str(),
                global_init_name.as_str(),
                name,
                self.user_type
                    .initializer
                    .clone()
                    .map(|i| i.get_location())
                    .unwrap_or_else(|| self.user_type.location.clone()),
            )
            .set_constant(true)
            .set_initial_value(Some(init));
            self.index.register_global_initializer(global_init_name.as_str(), initializer_global);
        }
    }

    fn index_pointer_type(
        &mut self,
        name: &Option<String>,
        referenced_type: &DataTypeDeclaration,
        auto_deref: Option<AutoDerefType>,
        type_safe: bool,
        is_function: bool,
    ) {
        let inner_type_name = referenced_type.get_name().expect("named datatype");
        let name = name.as_deref().unwrap();
        let information = DataTypeInformation::Pointer {
            name: name.to_string(),
            inner_type_name: inner_type_name.into(),
            auto_deref,
            type_safe,
            is_function,
        };

        self.index.register_type(typesystem::DataType {
            name: name.to_string(),
            initial_value: self.pending_initializer,
            information,
            nature: TypeNature::Any,
            location: self.user_type.location.clone(),
        });
    }

    fn index_struct_type(&mut self, name: &str, variables: &[Variable], source: StructSource) {
        let scope = Some(name.to_string());
        let members = variables
            .iter()
            .enumerate()
            .map(|(count, var)| {
                // let inner_indexer = UserTypeIndexer::new(self.index, &var.data_type_declaration);
                self.visit_data_type_declaration(&var.data_type_declaration);

                let member_type = var.data_type_declaration.get_name().expect("named variable datatype");
                let init = self.index.get_mut_const_expressions().maybe_add_constant_expression(
                    var.initializer.clone(),
                    member_type,
                    scope.clone(),
                    Some(var.name.clone()),
                );

                let binding = var
                    .address
                    .as_ref()
                    .and_then(|it| HardwareBinding::from_statement(self.index, it, scope.clone()));

                self.index.create_member_variable(
                    MemberInfo {
                        container_name: name,
                        variable_name: &var.name,
                        variable_linkage: ArgumentType::ByVal(VariableType::Input), // struct members act like VAR_INPUT in terms of visibility
                        variable_type_name: member_type,
                        is_constant: false, //struct members are not constants //TODO thats probably not true (you can define a struct in an CONST-block?!)
                        is_var_external: false, // see above
                        binding,
                        varargs: None,
                    },
                    init,
                    var.location.clone(),
                    count as u32,
                )
            })
            .collect::<Vec<_>>();

        let nature = source.get_type_nature();
        let information = DataTypeInformation::Struct { name: name.to_owned(), members, source };

        self.register_type(name, information, nature);

        //Generate an initializer for the struct
        let global_struct_name = crate::index::get_initializer_name(name);
        let variable = VariableIndexEntry::create_global(
            &global_struct_name,
            &global_struct_name,
            name,
            self.user_type.location.clone(),
        )
        .set_initial_value(self.pending_initializer)
        .set_constant(true);

        self.index.register_global_initializer(&global_struct_name, variable);
    }
}
