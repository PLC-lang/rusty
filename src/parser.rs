// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    ast::*,
    expect_token, lexer,
    lexer::{ParseSession, Token, Token::*},
    typesystem::DINT_TYPE,
    Diagnostic,
};

use self::{
    control_parser::parse_control_statement,
    expressions_parser::{parse_expression, parse_expression_list},
};

mod control_parser;
mod expressions_parser;

#[cfg(test)]
pub mod tests;
pub type ParsedAst = (CompilationUnit, Vec<Diagnostic>);

pub fn parse(mut lexer: ParseSession, lnk: LinkageType, file_name: &str) -> ParsedAst {
    let mut unit = CompilationUnit::new(file_name, NewLines::build(lexer.get_src()));

    let mut linkage = lnk;
    loop {
        match lexer.token {
            PropertyExternal => {
                linkage = LinkageType::External;
                lexer.advance();
                //Don't reset linkage
                continue;
            }
            KeywordVarGlobal => unit
                .global_vars
                .push(parse_variable_block(&mut lexer, linkage)),
            KeywordProgram | KeywordClass | KeywordFunction | KeywordFunctionBlock => {
                let params = match lexer.token {
                    KeywordProgram => (PouType::Program, KeywordEndProgram),
                    KeywordClass => (PouType::Class, KeywordEndClass),
                    KeywordFunction => (PouType::Function, KeywordEndFunction),
                    _ => (PouType::FunctionBlock, KeywordEndFunctionBlock),
                };

                let (mut pou, mut implementation) =
                    parse_pou(&mut lexer, params.0, linkage, params.1);

                unit.units.append(&mut pou);
                unit.implementations.append(&mut implementation);
            }
            KeywordAction => {
                if let Some(implementation) = parse_action(&mut lexer, linkage, None) {
                    unit.implementations.push(implementation);
                }
            }
            KeywordActions => {
                let last_pou = unit
                    .units
                    .last()
                    .map(|it| it.name.as_str())
                    .unwrap_or("__unknown__");
                let mut actions = parse_actions(&mut lexer, linkage, last_pou);
                unit.implementations.append(&mut actions);
            }
            KeywordType => {
                let unit_type = parse_type(&mut lexer);
                for utype in unit_type {
                    unit.types.push(utype);
                }
            }
            KeywordEndActions | End => return (unit, lexer.diagnostics),
            _ => {
                lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                    "StartKeyword",
                    lexer.slice(),
                    lexer.location(),
                ));
                lexer.advance();
            }
        };
        linkage = lnk;
    }
    //the match in the loop will always return
}

fn parse_actions(
    lexer: &mut ParseSession,
    linkage: LinkageType,
    default_container: &str,
) -> Vec<Implementation> {
    parse_any_in_region(lexer, vec![KeywordEndActions], |lexer| {
        lexer.advance();
        let container = if lexer.token == Identifier {
            lexer.slice_and_advance()
        } else {
            lexer.accept_diagnostic(Diagnostic::missing_action_container(lexer.location()));
            default_container.into()
        };
        let mut impls = vec![];

        //Go through each action
        while lexer.token != KeywordEndActions && !lexer.is_end_of_stream() {
            match lexer.token {
                KeywordAction => {
                    if let Some(implementation) = parse_action(lexer, linkage, Some(&container)) {
                        impls.push(implementation);
                    }
                }
                _ => {
                    lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                        "KeywordAction",
                        lexer.slice(),
                        lexer.location(),
                    ));
                    return impls;
                }
            }
        }
        impls
    })
}

///
/// parse a pou
/// # Arguments
///
/// * `lexer`       - the lexer
/// * `pou_type`    - the type of the pou currently parsed
/// * `linkage`     - internal, external ?
/// * `expected_end_token` - the token that ends this pou
///
fn parse_pou(
    lexer: &mut ParseSession,
    pou_type: PouType,
    linkage: LinkageType,
    expected_end_token: lexer::Token,
) -> (Vec<Pou>, Vec<Implementation>) {
    let start = lexer.range().start;
    lexer.advance(); //Consume ProgramKeyword
    let closing_tokens = vec![
        expected_end_token.clone(),
        KeywordEndAction,
        KeywordEndProgram,
        KeywordEndFunction,
        KeywordEndFunctionBlock,
    ];
    let pou = parse_any_in_region(lexer, closing_tokens.clone(), |lexer| {
        let poly_mode = match pou_type {
            PouType::Class | PouType::FunctionBlock | PouType::Method { .. } => {
                // classes and function blocks can be ABSTRACT, FINAL or neither.
                parse_polymorphism_mode(lexer, &pou_type)
            }
            _ => None,
        };

        let (name, name_location) =
            parse_identifier(lexer).unwrap_or_else(|| ("".to_string(), SourceRange::undefined())); // parse POU name

        let generics = parse_generics(lexer);

        with_scope(lexer, name.clone(), |lexer| {
            // TODO: Parse USING directives
            // TODO: Parse EXTENDS specifier
            // TODO: Parse IMPLEMENTS specifier

            let return_type = if pou_type != PouType::Class {
                // parse an optional return type
                parse_return_type(lexer, &pou_type)
            } else {
                // classes do not have a return type
                None
            };

            // parse variable declarations. note that var in/out/inout
            // blocks are not allowed inside of class declarations.
            let mut variable_blocks = vec![];
            let allowed_var_types = match pou_type {
                PouType::Class => vec![KeywordVar],
                _ => vec![
                    KeywordVar,
                    KeywordVarInput,
                    KeywordVarOutput,
                    KeywordVarInOut,
                    KeywordVarTemp,
                ],
            };
            while allowed_var_types.contains(&lexer.token) {
                variable_blocks.push(parse_variable_block(lexer, LinkageType::Internal));
            }

            let mut impl_pous = vec![];
            let mut implementations = vec![];
            if pou_type == PouType::Class || pou_type == PouType::FunctionBlock {
                // classes and function blocks can have methods. methods consist of a Pou part
                // and an implementation part. That's why we get another (Pou, Implementation)
                // tuple out of parse_method() that has to be added to the list of Pous and
                // implementations. Note that function blocks have to start with the method
                // declarations before their implementation.
                while lexer.token == KeywordMethod {
                    if let Some((pou, implementation)) = parse_method(lexer, &name, linkage) {
                        impl_pous.push(pou);
                        implementations.push(implementation);
                    }
                }
            }
            if pou_type != PouType::Class {
                // a class may not contain an implementation
                implementations.push(parse_implementation(
                    lexer,
                    linkage,
                    pou_type.clone(),
                    &name,
                    &name,
                    !generics.is_empty(),
                ));
            }
            let mut pous = vec![Pou {
                name,
                pou_type,
                variable_blocks,
                return_type,
                location: lexer.source_range_factory.create_range(start..lexer.range().end),
                name_location,
                poly_mode,
                generics,
                linkage,
            }];
            pous.append(&mut impl_pous);

            (pous, implementations)
        })
    });

    //check if we ended on the right end-keyword
    if closing_tokens.contains(&lexer.last_token) && lexer.last_token != expected_end_token {
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            format!("{:?}", expected_end_token).as_str(),
            lexer.slice_region(lexer.last_range.clone()),
            lexer.source_range_factory.create_range(lexer.last_range.clone()),
        ));
    }
    pou
}

fn parse_generics(lexer: &mut ParseSession) -> Vec<GenericBinding> {
    if lexer.allow(&Token::OperatorLess) {
        parse_any_in_region(lexer, vec![Token::OperatorGreater], |lexer| {
            let mut generics = vec![];
            loop {
                //identifier
                if let Some((name, _)) = parse_identifier(lexer) {
                    lexer.consume_or_report(Token::KeywordColon);

                    //Expect a type nature
                    if let Some(nature) =
                        parse_identifier(lexer).map(|(it, _)| parse_type_nature(lexer, &it))
                    {
                        generics.push(GenericBinding { name, nature });
                    }
                }

                if !lexer.allow(&Token::KeywordComma) || lexer.allow(&Token::OperatorGreater) {
                    break;
                }
            }

            generics
        })
    } else {
        vec![]
    }
}

fn parse_type_nature(lexer: &mut ParseSession, nature: &str) -> TypeNature {
    match nature {
        "ANY" => TypeNature::Any,
        "ANY_DERIVED" => TypeNature::Derived,
        "ANY_ELEMENTARY" => TypeNature::Elementary,
        "ANY_MAGNITUDE" => TypeNature::Magnitude,
        "ANY_NUM" => TypeNature::Num,
        "ANY_REAL" => TypeNature::Real,
        "ANY_INT" => TypeNature::Int,
        "ANY_SIGNED" => TypeNature::Signed,
        "ANY_UNSIGNED" => TypeNature::Unsigned,
        "ANY_DURATION" => TypeNature::Duration,
        "ANY_BIT" => TypeNature::Bit,
        "ANY_CHARS" => TypeNature::Chars,
        "ANY_STRING" => TypeNature::String,
        "ANY_CHAR" => TypeNature::Char,
        "ANY_DATE" => TypeNature::Date,
        _ => {
            lexer.accept_diagnostic(Diagnostic::unknown_type_nature(nature, lexer.location()));
            TypeNature::Any
        }
    }
}

fn parse_polymorphism_mode(
    lexer: &mut ParseSession,
    pou_type: &PouType,
) -> Option<PolymorphismMode> {
    match pou_type {
        PouType::Class | PouType::FunctionBlock | PouType::Method { .. } => {
            Some(
                // See if the method/pou was declared FINAL or ABSTRACT
                if lexer.allow(&KeywordFinal) {
                    PolymorphismMode::Final
                } else if lexer.allow(&KeywordAbstract) {
                    PolymorphismMode::Abstract
                } else {
                    PolymorphismMode::None
                },
            )
        }
        _ => None,
    }
}

fn parse_return_type(lexer: &mut ParseSession, pou_type: &PouType) -> Option<DataTypeDeclaration> {
    let start_return_type = lexer.range().start;
    if lexer.allow(&KeywordColon) {
        if let Some((declaration, initializer)) = parse_data_type_definition(lexer, None) {
            if let Some(init) = initializer {
                lexer.accept_diagnostic(Diagnostic::unexpected_initializer_on_function_return(
                    init.get_location(),
                ));
            }

            if !matches!(pou_type, PouType::Function | PouType::Method { .. }) {
                lexer.accept_diagnostic(Diagnostic::return_type_not_supported(
                    pou_type,
                    lexer.source_range_factory.create_range(start_return_type..lexer.last_range.end),
                ));
            }

            if let DataTypeDeclaration::DataTypeDefinition { data_type, .. } = &declaration {
                if matches!(
                    data_type,
                    DataType::EnumType { .. } | DataType::StructType { .. }
                ) {
                    lexer.accept_diagnostic(Diagnostic::function_unsupported_return_type(
                        &declaration,
                    ))
                }
            }
            Some(declaration)
        } else {
            //missing return type
            lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                "Datatype",
                lexer.slice(),
                lexer.source_range_factory.create_range(lexer.range()),
            ));
            None
        }
    } else {
        None
    }
}

fn parse_method(
    lexer: &mut ParseSession,
    class_name: &str,
    linkage: LinkageType,
) -> Option<(Pou, Implementation)> {
    parse_any_in_region(lexer, vec![KeywordEndMethod], |lexer| {
        // Method declarations look like this:
        // METHOD [AccessModifier] [ABSTRACT|FINAL] [OVERRIDE] [: return_type]
        //    ...
        // END_METHOD

        let method_start = lexer.location().get_start();
        lexer.advance(); // eat METHOD keyword

        let access = Some(parse_access_modifier(lexer));
        let pou_type = PouType::Method {
            owner_class: class_name.into(),
        };
        let poly_mode = parse_polymorphism_mode(lexer, &pou_type);
        let overriding = lexer.allow(&KeywordOverride);
        let (name, name_location) = parse_identifier(lexer)?;
        let generics = parse_generics(lexer);
        let return_type = parse_return_type(lexer, &pou_type);

        let mut variable_blocks = vec![];
        while lexer.token == KeywordVar
            || lexer.token == KeywordVarInput
            || lexer.token == KeywordVarOutput
            || lexer.token == KeywordVarInOut
            || lexer.token == KeywordVarTemp
        {
            variable_blocks.push(parse_variable_block(lexer, LinkageType::Internal));
        }

        let call_name = format!("{}.{}", class_name, name);
        let implementation = parse_implementation(
            lexer,
            linkage,
            PouType::Method {
                owner_class: class_name.into(),
            },
            &call_name,
            &call_name,
            !generics.is_empty(),
        );

        // parse_implementation() will default-initialize the fields it
        // doesn't know. thus, we have to complete the information.
        let implementation = Implementation {
            overriding,
            access,
            ..implementation
        };

        let method_end = lexer.location().get_end();
        Some((
            Pou {
                name: call_name,
                pou_type,
                variable_blocks,
                return_type,
                location: lexer.source_range_factory.create_range(method_start..method_end),
                name_location,
                poly_mode,
                generics,
                linkage,
            },
            implementation,
        ))
    })
}

fn parse_access_modifier(lexer: &mut ParseSession) -> AccessModifier {
    if lexer.allow(&KeywordAccessPublic) {
        AccessModifier::Public
    } else if lexer.allow(&KeywordAccessPrivate) {
        AccessModifier::Private
    } else if lexer.allow(&KeywordAccessProtected) {
        AccessModifier::Protected
    } else if lexer.allow(&KeywordAccessInternal) {
        AccessModifier::Internal
    } else {
        AccessModifier::Protected
    }
}

/// parse identifier and advance if successful
/// returns the identifier as a String and the SourceRange of the parsed name
fn parse_identifier(lexer: &mut ParseSession) -> Option<(String, SourceRange)> {
    let pou_name = lexer.slice().to_string();
    if lexer.token == Identifier {
        lexer.advance();
        Some((pou_name, lexer.last_location()))
    } else {
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            "Identifier",
            pou_name.as_str(),
            lexer.location(),
        ));
        None
    }
}

fn parse_implementation(
    lexer: &mut ParseSession,
    linkage: LinkageType,
    pou_type: PouType,
    call_name: &str,
    type_name: &str,
    generic: bool,
) -> Implementation {
    let start = lexer.range().start;
    let statements = parse_body_standalone(lexer);
    Implementation {
        name: call_name.into(),
        type_name: type_name.into(),
        linkage,
        pou_type,
        statements,
        location: lexer.source_range_factory.create_range(start..lexer.range().end),
        overriding: false,
        generic,
        access: None,
    }
}

fn parse_action(
    lexer: &mut ParseSession,
    linkage: LinkageType,
    container: Option<&str>,
) -> Option<Implementation> {
    lexer.advance(); //Consume the Action keyword
    let closing_tokens = vec![
        KeywordEndAction,
        KeywordEndProgram,
        KeywordEndFunction,
        KeywordEndFunctionBlock,
    ];

    parse_any_in_region(lexer, closing_tokens.clone(), |lexer| {
        let name_or_container = lexer.slice_and_advance();

        let (container, name) = if let Some(container) = container {
            (container.into(), name_or_container)
        } else {
            expect_token!(lexer, KeywordDot, None);

            lexer.advance();

            expect_token!(lexer, Identifier, None);

            let name = lexer.slice_and_advance();
            (name_or_container, name)
        };
        let call_name = format!("{}.{}", &container, &name);

        let implementation = parse_implementation(
            lexer,
            linkage,
            PouType::Action,
            &call_name,
            &container,
            false,
        );
        //lets see if we ended on the right END_ keyword
        if closing_tokens.contains(&lexer.last_token) && lexer.last_token != KeywordEndAction {
            lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                format!("{:?}", KeywordEndAction).as_str(),
                lexer.slice(),
                lexer.location(),
            ))
        }
        Some(implementation)
    })
}

// TYPE ... END_TYPE
fn parse_type(lexer: &mut ParseSession) -> Vec<UserTypeDeclaration> {
    lexer.advance(); // consume the TYPE

    parse_any_in_region(lexer, vec![KeywordEndType], |lexer| {
        let mut declarations = vec![];
        while !lexer.closes_open_region(&lexer.token) {
            let start = lexer.location().get_start();
            let name = lexer.slice_and_advance();
            lexer.consume_or_report(KeywordColon);

            let result = parse_full_data_type_definition(lexer, Some(name));

            if let Some((DataTypeDeclaration::DataTypeDefinition { data_type, .. }, initializer)) =
                result
            {
                let end = lexer.last_range.end;
                declarations.push(UserTypeDeclaration {
                    data_type,
                    initializer,
                    location: (start..end).into(),
                    scope: lexer.scope.clone(),
                });
            }
        }
        declarations
    })
}

type DataTypeWithInitializer = (DataTypeDeclaration, Option<AstStatement>);

fn parse_full_data_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Option<DataTypeWithInitializer> {
    let end_keyword = if lexer.token == KeywordStruct {
        KeywordEndStruct
    } else {
        KeywordSemicolon
    };
    parse_any_in_region(lexer, vec![end_keyword], |lexer| {
        let sized = lexer.allow(&PropertySized);
        if lexer.allow(&KeywordDotDotDot) {
            Some((
                DataTypeDeclaration::DataTypeDefinition {
                    data_type: DataType::VarArgs {
                        referenced_type: None,
                        sized,
                    },
                    location: lexer.last_range.clone().into(),
                    scope: lexer.scope.clone(),
                },
                None,
            ))
        } else {
            parse_data_type_definition(lexer, name).map(|(type_def, initializer)| {
                if lexer.allow(&KeywordDotDotDot) {
                    (
                        DataTypeDeclaration::DataTypeDefinition {
                            data_type: DataType::VarArgs {
                                referenced_type: Some(Box::new(type_def)),
                                sized,
                            },
                            location: lexer.last_range.clone().into(),
                            scope: lexer.scope.clone(),
                        },
                        None,
                    )
                } else {
                    (type_def, initializer)
                }
            })
        }
    })
}

// TYPE xxx : 'STRUCT' | '(' | IDENTIFIER
fn parse_data_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Option<DataTypeWithInitializer> {
    let start = lexer.location().get_start();
    if lexer.allow(&KeywordStruct) {
        // Parse struct
        let variables = parse_variable_list(lexer);
        Some((
            DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::StructType { name, variables },
                location: (start..lexer.range().end).into(),
                scope: lexer.scope.clone(),
            },
            None,
        ))
    } else if lexer.allow(&KeywordArray) {
        parse_array_type_definition(lexer, name)
    } else if lexer.allow(&KeywordPointer) {
        let start_pos = lexer.last_range.start;
        //Report wrong keyword
        lexer.accept_diagnostic(Diagnostic::ImprovementSuggestion {
            message: "'POINTER TO' is not a standard keyword, use REF_TO instead".to_string(),
            range: lexer.last_location(),
        });
        if let Err(diag) = lexer.expect(KeywordTo) {
            lexer.accept_diagnostic(diag);
        } else {
            lexer.advance();
        }
        parse_pointer_definition(lexer, name, start_pos)
    } else if lexer.allow(&KeywordRef) {
        parse_pointer_definition(lexer, name, lexer.last_range.start)
    } else if lexer.allow(&KeywordParensOpen) {
        //enum without datatype
        parse_enum_type_definition(lexer, name)
    } else if lexer.token == KeywordString || lexer.token == KeywordWideString {
        parse_string_type_definition(lexer, name)
    } else if lexer.token == Identifier {
        parse_type_reference_type_definition(lexer, name)
    } else {
        //no datatype?
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            "DataTypeDefinition",
            format!("{:?}", lexer.token).as_str(),
            lexer.location(),
        ));
        None
    }
}

fn parse_pointer_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
    start_pos: usize,
) -> Option<(DataTypeDeclaration, Option<AstStatement>)> {
    parse_data_type_definition(lexer, None).map(|(decl, initializer)| {
        (
            DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::PointerType {
                    name,
                    referenced_type: Box::new(decl),
                },
                location: (start_pos..lexer.last_range.end).into(),
                scope: lexer.scope.clone(),
            },
            initializer,
        )
    })
}

fn parse_type_reference_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Option<(DataTypeDeclaration, Option<AstStatement>)> {
    let start = lexer.location().get_start();
    //Subrange
    let referenced_type = lexer.slice_and_advance();

    let bounds = if lexer.allow(&KeywordParensOpen) {
        // INT (..) :=
        let bounds = parse_expression(lexer);
        expect_token!(lexer, KeywordParensClose, None);
        lexer.advance();
        Some(bounds)
    } else {
        None
    };

    let initial_value = if lexer.allow(&KeywordAssignment) {
        Some(parse_expression(lexer))
    } else {
        None
    };

    let end = lexer.last_range.end;
    if name.is_some() || bounds.is_some() {
        let data_type = match bounds {
            Some(AstStatement::ExpressionList { expressions, id }) => {
                //this is an enum
                DataTypeDeclaration::DataTypeDefinition {
                    data_type: DataType::EnumType {
                        name,
                        numeric_type: referenced_type,
                        elements: AstStatement::ExpressionList { expressions, id },
                    },
                    location: (start..end).into(),
                    scope: lexer.scope.clone(),
                }
            }
            Some(AstStatement::Reference { .. }) => {
                // a enum with just one element
                DataTypeDeclaration::DataTypeDefinition {
                    data_type: DataType::EnumType {
                        name,
                        numeric_type: referenced_type,
                        elements: bounds.unwrap(),
                    },
                    location: (start..end).into(),
                    scope: lexer.scope.clone(),
                }
            }
            _ => DataTypeDeclaration::DataTypeDefinition {
                //something else inside the brackets -> probably a subrange?
                data_type: DataType::SubRangeType {
                    name,
                    referenced_type,
                    bounds,
                },
                location: (start..end).into(),
                scope: lexer.scope.clone(),
            },
        };
        Some((data_type, initial_value))
    } else {
        Some((
            DataTypeDeclaration::DataTypeReference {
                referenced_type,
                location: (start..end).into(),
            },
            initial_value,
        ))
    }
}

fn parse_string_size_expression(lexer: &mut ParseSession) -> Option<AstStatement> {
    let opening_token = lexer.token.clone();
    if lexer.allow(&KeywordSquareParensOpen) || lexer.allow(&KeywordParensOpen) {
        let opening_location = lexer.location().get_start();
        let closing_tokens = vec![KeywordSquareParensClose, KeywordParensClose];
        parse_any_in_region(lexer, closing_tokens, |lexer| {
            let size_expr = parse_expression(lexer);
            let error_range = lexer.source_range_factory.create_range(opening_location..lexer.location().get_end());

            if (opening_token == KeywordParensOpen && lexer.token == KeywordSquareParensClose)
                || (opening_token == KeywordSquareParensOpen && lexer.token == KeywordParensClose)
            {
                lexer.accept_diagnostic(Diagnostic::ImprovementSuggestion {
                    message: "Mismatched types of parentheses around string size expression".into(),
                    range: error_range,
                });
            } else if opening_token == KeywordParensOpen || lexer.token == KeywordParensClose {
                lexer.accept_diagnostic(Diagnostic::ImprovementSuggestion {
                    message: "Unusual type of parentheses around string size expression, consider using square parentheses '[]'"
                        .into(),
                    range: error_range,
                });
            }

            Some(size_expr)
        })
    } else {
        None
    }
}

fn parse_string_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Option<(DataTypeDeclaration, Option<AstStatement>)> {
    let start = lexer.location().get_start();
    let is_wide = lexer.token == KeywordWideString;
    lexer.advance();

    let size = parse_string_size_expression(lexer);
    let end = lexer.last_range.end;

    Some((
        DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::StringType {
                name,
                is_wide,
                size,
            },
            location: (start..end).into(),
            scope: lexer.scope.clone(),
        },
        lexer
            .allow(&KeywordAssignment)
            .then(|| parse_expression(lexer)),
    ))
}

fn parse_enum_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Option<(DataTypeDeclaration, Option<AstStatement>)> {
    let start = lexer.last_range.start;
    let elements = parse_any_in_region(lexer, vec![KeywordParensClose], |lexer| {
        // Parse Enum - we expect at least one element
        let elements = parse_expression_list(lexer);
        Some(elements)
    })?;

    Some((
        DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::EnumType {
                name,
                elements,
                numeric_type: DINT_TYPE.to_string(),
            },
            location: (start..lexer.last_range.end).into(),
            scope: lexer.scope.clone(),
        },
        None,
    ))
}

fn parse_array_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Option<(DataTypeDeclaration, Option<AstStatement>)> {
    let start = lexer.last_range.start;
    let range = parse_any_in_region(lexer, vec![KeywordOf], |lexer| {
        // Parse Array range

        expect_token!(lexer, KeywordSquareParensOpen, None);
        lexer.advance();

        let range_statement = parse_expression(lexer);

        expect_token!(lexer, KeywordSquareParensClose, None);
        lexer.advance();

        Some(range_statement)
    })?;

    let inner_type_defintion = parse_data_type_definition(lexer, None);
    inner_type_defintion.map(|(reference, initializer)| {
        let location = lexer.source_range_factory.create_range(start..reference.get_location().get_end());
        (
            DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::ArrayType {
                    name,
                    bounds: range,
                    referenced_type: Box::new(reference),
                },
                location,
                scope: lexer.scope.clone(),
            },
            initializer,
        )
    })
}

/// parse a body and recovers until the given `end_keywords`
fn parse_body_in_region(lexer: &mut ParseSession, end_keywords: Vec<Token>) -> Vec<AstStatement> {
    parse_any_in_region(lexer, end_keywords, parse_body_standalone)
}

fn parse_body_standalone(lexer: &mut ParseSession) -> Vec<AstStatement> {
    let mut statements = Vec::new();
    while !lexer.closes_open_region(&lexer.token) {
        statements.push(parse_control(lexer));
    }
    statements
}

/// parses a statement ending with a ';'
fn parse_statement(lexer: &mut ParseSession) -> AstStatement {
    let result = parse_any_in_region(lexer, vec![KeywordSemicolon, KeywordColon], |lexer| {
        parse_expression(lexer)
    });
    if lexer.last_token == KeywordColon {
        AstStatement::CaseCondition {
            condition: Box::new(result),
            id: lexer.next_id(),
        }
    } else {
        result
    }
}

pub fn with_scope<T, F: FnOnce(&mut ParseSession) -> T>(
    lexer: &mut ParseSession,
    scope: String,
    parse_fn: F,
) -> T {
    lexer.scope = Some(scope);
    let result = parse_fn(lexer);
    lexer.scope = None;
    result
}

pub fn parse_any_in_region<T, F: FnOnce(&mut ParseSession) -> T>(
    lexer: &mut ParseSession,
    closing_tokens: Vec<Token>,
    parse_fn: F,
) -> T {
    lexer.enter_region(closing_tokens);
    let result = parse_fn(lexer);

    // try to recover by eating everything until
    // we believe the parser is able to continue
    lexer.recover_until_close();
    lexer.close_region();

    result
}

fn parse_reference(lexer: &mut ParseSession) -> AstStatement {
    match expressions_parser::parse_qualified_reference(lexer) {
        Ok(statement) => statement,
        Err(diagnostic) => {
            let statement = AstStatement::EmptyStatement {
                location: diagnostic.get_location(),
                id: lexer.next_id(),
            };
            lexer.accept_diagnostic(diagnostic);
            statement
        }
    }
}

fn parse_control(lexer: &mut ParseSession) -> AstStatement {
    parse_control_statement(lexer)
}

fn parse_variable_block_type(lexer: &mut ParseSession) -> VariableBlockType {
    let block_type = lexer.token.clone();
    //Consume the type token
    lexer.advance();
    let argument_property = if lexer.allow(&PropertyByRef) {
        //Report a diagnostic if blocktype is incompatible
        if !matches!(block_type, KeywordVarInput) {
            lexer.accept_diagnostic(Diagnostic::invalid_pragma_location(
                "Only VAR_INPUT support by ref properties",
                lexer.location(),
            ))
        }
        ArgumentProperty::ByRef
    } else {
        ArgumentProperty::ByVal
    };
    match block_type {
        KeywordVar => VariableBlockType::Local,
        KeywordVarTemp => VariableBlockType::Temp,
        KeywordVarInput => VariableBlockType::Input(argument_property),
        KeywordVarOutput => VariableBlockType::Output,
        KeywordVarGlobal => VariableBlockType::Global,
        KeywordVarInOut => VariableBlockType::InOut,
        _ => VariableBlockType::Local,
    }
}

fn parse_variable_block(lexer: &mut ParseSession, linkage: LinkageType) -> VariableBlock {
    let location = lexer.location();
    let variable_block_type = parse_variable_block_type(lexer);

    let constant = lexer.allow(&KeywordConstant);

    let retain = lexer.allow(&KeywordRetain);
    lexer.allow(&KeywordNonRetain);

    let access = parse_access_modifier(lexer);

    let variables = parse_any_in_region(lexer, vec![KeywordEndVar], |lexer| {
        parse_variable_list(lexer)
    });
    VariableBlock {
        access,
        constant,
        retain,
        variables,
        variable_block_type,
        linkage,
        location,
    }
}

fn parse_variable_list(lexer: &mut ParseSession) -> Vec<Variable> {
    let mut variables = vec![];
    while lexer.token == Identifier {
        let mut line_vars = parse_variable_line(lexer);
        variables.append(&mut line_vars);
    }
    variables
}

fn parse_variable_line(lexer: &mut ParseSession) -> Vec<Variable> {
    // read in a comma separated list of variable names
    let mut var_names: Vec<(String, SourceRange)> = vec![];
    while lexer.token == Identifier {
        let location = lexer.location();
        let identifier_end = location.get_end();
        var_names.push((lexer.slice_and_advance(), location));

        if lexer.token == KeywordColon || lexer.token == KeywordAt {
            break;
        }

        if !lexer.allow(&KeywordComma) {
            let next_token_start = lexer.location().get_start();
            lexer.accept_diagnostic(Diagnostic::missing_token(
                format!("{:?} or {:?}", KeywordColon, KeywordComma).as_str(),
                lexer.source_range_factory.create_range(identifier_end..next_token_start),
            ));
        }
    }

    //See if there's an AT keyword
    let address = if lexer.allow(&KeywordAt) {
        let start_location = lexer.last_range.start;
        //Look for a hardware address
        if let HardwareAccess((direction, access_type)) = lexer.token {
            lexer.advance();
            //Folowed by an integer
            if access_type == DirectAccessType::Template || lexer.token == LiteralInteger {
                let mut address = vec![];
                if lexer.token == LiteralInteger {
                    loop {
                        let int = expressions_parser::parse_strict_literal_integer(lexer);
                        match int {
                            Ok(statement) => address.push(statement),
                            Err(err) => {
                                lexer.accept_diagnostic(err);
                                break;
                            }
                        }
                        if !lexer.allow(&KeywordDot) {
                            break;
                        }
                    }
                }
                Some(AstStatement::HardwareAccess {
                    access: access_type,
                    direction,
                    address,
                    location: (start_location..lexer.last_range.end).into(),
                    id: lexer.next_id(),
                })
            } else {
                lexer.accept_diagnostic(Diagnostic::missing_token(
                    "LiteralInteger",
                    lexer.location(),
                ));
                None
            }
        } else {
            lexer.accept_diagnostic(Diagnostic::missing_token(
                "Hardware Access",
                lexer.location(),
            ));
            None
        }
    } else {
        None
    };

    // colon has to come before the data type
    if !lexer.allow(&KeywordColon) {
        lexer.accept_diagnostic(Diagnostic::missing_token(
            format!("{:?}", KeywordColon).as_str(),
            lexer.location(),
        ));
    }

    // create variables with the same data type for each of the names
    let mut variables = vec![];
    if let Some((data_type, initializer)) = parse_full_data_type_definition(lexer, None) {
        for (name, location) in var_names {
            variables.push(Variable {
                name,
                data_type: data_type.clone(),
                location,
                initializer: initializer.clone(),
                address: address.clone(),
            });
        }
    }
    variables
}
