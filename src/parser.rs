// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    ast::*,
    expect_token, lexer,
    lexer::{ParseSession, Token, Token::*},
    Diagnostic,
};

use self::{control_parser::parse_control_statement, expressions_parser::parse_expression};

mod control_parser;
mod expressions_parser;

#[cfg(test)]
mod tests;
pub type ParsedAst = (CompilationUnit, Vec<Diagnostic>);

pub fn parse(mut lexer: ParseSession) -> ParsedAst {
    let mut unit = CompilationUnit::default();

    let mut linkage = LinkageType::Internal;
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
                .push(parse_variable_block(&mut lexer, VariableBlockType::Global)),
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
                if let Some(unit_type) = parse_type(&mut lexer) {
                    unit.types.push(unit_type);
                }
            }
            KeywordEndActions | End => return (unit, lexer.diagnostics),
            _ => {
                lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                    "StartKeyword".to_string(),
                    lexer.slice().to_string(),
                    lexer.location(),
                ));
                lexer.advance();
            }
        };
        linkage = LinkageType::Internal;
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
                        "KeywordAction".to_string(),
                        lexer.slice().to_string(),
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
            PouType::Class | PouType::FunctionBlock | PouType::Method => {
                // classes and function blocks can be ABSTRACT, FINAL or neither.
                parse_polymorphism_mode(lexer, pou_type)
            }
            _ => None,
        };

        let name = parse_identifier(lexer).unwrap_or_else(|| "".to_string()); // parse POU name

        // TODO: Parse USING directives
        // TODO: Parse EXTENDS specifier
        // TODO: Parse IMPLEMENTS specifier

        let return_type = if pou_type != PouType::Class {
            // parse an optional return type
            parse_return_type(lexer, pou_type)
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
            ],
        };
        while allowed_var_types.contains(&lexer.token) {
            variable_blocks.push(parse_variable_block(
                lexer,
                parse_variable_block_type(&lexer.token),
            ));
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
            implementations.push(parse_implementation(lexer, linkage, pou_type, &name, &name));
        }

        let mut pous = vec![Pou {
            name,
            pou_type,
            variable_blocks,
            return_type,
            location: SourceRange::new(start..lexer.range().end),
            poly_mode,
        }];
        pous.append(&mut impl_pous);

        (pous, implementations)
    });

    //check if we ended on the right end-keyword
    if closing_tokens.contains(&lexer.last_token) && lexer.last_token != expected_end_token {
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            format!("{:?}", expected_end_token),
            lexer.slice_region(lexer.last_range.clone()).into(),
            SourceRange::new(lexer.last_range.clone()),
        ));
    }
    pou
}

fn parse_polymorphism_mode(
    lexer: &mut ParseSession,
    pou_type: PouType,
) -> Option<PolymorphismMode> {
    match pou_type {
        PouType::Class | PouType::FunctionBlock | PouType::Method => {
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

fn parse_return_type(lexer: &mut ParseSession, pou_type: PouType) -> Option<DataTypeDeclaration> {
    let start_return_type = lexer.range().start;
    if lexer.allow(&KeywordColon) {
        if lexer.token == Identifier || lexer.token == KeywordString {
            if pou_type != PouType::Function && pou_type != PouType::Method {
                lexer.accept_diagnostic(Diagnostic::return_type_not_supported(
                    &pou_type,
                    SourceRange::new(start_return_type..lexer.range().end),
                ));
            }
            let referenced_type = lexer.slice_and_advance();
            Some(DataTypeDeclaration::DataTypeReference { referenced_type })
        } else {
            //missing return type
            lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                "Datatype".to_string(),
                lexer.slice().to_string(),
                SourceRange::new(lexer.range()),
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
        let poly_mode = parse_polymorphism_mode(lexer, PouType::Method);
        let overriding = lexer.allow(&KeywordOverride);
        let name = parse_identifier(lexer)?;
        let return_type = parse_return_type(lexer, PouType::Method);

        let mut variable_blocks = vec![];
        while lexer.token == KeywordVar
            || lexer.token == KeywordVarInput
            || lexer.token == KeywordVarOutput
            || lexer.token == KeywordVarInOut
            || lexer.token == KeywordVarTemp
        {
            variable_blocks.push(parse_variable_block(
                lexer,
                parse_variable_block_type(&lexer.token),
            ));
        }

        let call_name = format!("{}.{}", class_name, name);
        let implementation =
            parse_implementation(lexer, linkage, PouType::Class, &call_name, class_name);

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
                pou_type: PouType::Method,
                variable_blocks,
                return_type,
                location: SourceRange::new(method_start..method_end),
                poly_mode,
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
fn parse_identifier(lexer: &mut ParseSession) -> Option<String> {
    let pou_name = lexer.slice().to_string();
    if lexer.token == Identifier {
        lexer.advance();
        Some(pou_name)
    } else {
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            "Identifier".into(),
            pou_name,
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
) -> Implementation {
    let start = lexer.range().start;
    let statements = parse_body_standalone(lexer);
    Implementation {
        name: call_name.into(),
        type_name: type_name.into(),
        linkage,
        pou_type,
        statements,
        location: SourceRange::new(start..lexer.range().end),
        overriding: false,
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

        let implementation =
            parse_implementation(lexer, linkage, PouType::Action, &call_name, &container);
        //lets see if we ended on the right END_ keyword
        if closing_tokens.contains(&lexer.last_token) && lexer.last_token != KeywordEndAction {
            lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                format!("{:?}", KeywordEndAction),
                lexer.slice().into(),
                lexer.location(),
            ))
        }
        Some(implementation)
    })
}

// TYPE ... END_TYPE
fn parse_type(lexer: &mut ParseSession) -> Option<UserTypeDeclaration> {
    lexer.advance(); // consume the TYPE
    let name = lexer.slice_and_advance();
    lexer.consume_or_report(KeywordColon);

    let result = parse_full_data_type_definition(lexer, Some(name));
    if let Some((DataTypeDeclaration::DataTypeDefinition { data_type }, initializer)) = result {
        lexer.consume_or_report(KeywordEndType);
        Some(UserTypeDeclaration {
            data_type,
            initializer,
        })
    } else {
        None
    }
}

type DataTypeWithInitializer = (DataTypeDeclaration, Option<Statement>);

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
        if lexer.allow(&KeywordDotDotDot) {
            Some((
                DataTypeDeclaration::DataTypeDefinition {
                    data_type: DataType::VarArgs {
                        referenced_type: None,
                    },
                    location: lexer.last_range.clone().into(),
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
                            },
                            location: lexer.last_range.clone().into(),
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
    if lexer.allow(&KeywordStruct) {
        // Parse struct
        let variables = parse_variable_list(lexer);
        Some((
            DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::StructType { name, variables },
                location: (start..lexer.range().end).into(),
            },
            None,
        ))
    } else if lexer.allow(&KeywordArray) {
        parse_array_type_definition(lexer, name)
    } else if lexer.allow(&KeywordParensOpen) {
        parse_enum_type_definition(lexer, name)
    } else if lexer.token == KeywordString || lexer.token == KeywordWideString {
        parse_string_type_definition(lexer, name)
    } else if lexer.token == Identifier {
        parse_type_reference_type_definition(lexer, name)
    } else {
        //no datatype?
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            "DataTypeDefinition".into(),
            format!("{:?}", lexer.token),
            lexer.location(),
        ));
        None
    }
}

fn parse_type_reference_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Option<(DataTypeDeclaration, Option<Statement>)> {
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

    if name.is_some() || bounds.is_some() {
        let data_type = DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::SubRangeType {
                name,
                referenced_type,
                bounds,
            },
            location: (start..end).into(),
        };
        Some((data_type, initial_value))
    } else {
        Some((
            DataTypeDeclaration::DataTypeReference { referenced_type },
            initial_value,
        ))
    }
}

fn parse_string_size_expression(lexer: &mut ParseSession) -> Option<Statement> {
    let opening_token = lexer.token.clone();
    if lexer.allow(&KeywordSquareParensOpen) || lexer.allow(&KeywordParensOpen) {
        let opening_location = lexer.location().get_start();
        let closing_tokens = vec![KeywordSquareParensClose, KeywordParensClose];
        parse_any_in_region(lexer, closing_tokens, |lexer| {
            let size_expr = parse_expression(lexer);
            let error_range = SourceRange::new(opening_location..lexer.location().get_end());

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
) -> Option<(DataTypeDeclaration, Option<Statement>)> {
    let is_wide = lexer.token == KeywordWideString;
    lexer.advance();

    let size = parse_string_size_expression(lexer);

    Some((
        DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::StringType {
                name,
                is_wide,
                size,
            },
            location: (start..end).into(),
        },
        lexer
            .allow(&KeywordAssignment)
            .then(|| parse_expression(lexer)),
    ))
}

fn parse_enum_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Option<(DataTypeDeclaration, Option<Statement>)> {
    let elements = parse_any_in_region(lexer, vec![KeywordParensClose], |lexer| {
        // Parse Enum - we expect at least one element

        let mut elements = Vec::new();
        expect_token!(lexer, Identifier, None);
        elements.push(lexer.slice_and_advance());

        // parse additional elements separated by ','
        while lexer.allow(&KeywordComma) {
            expect_token!(lexer, Identifier, None);
            elements.push(lexer.slice_and_advance());
        }
        Some(elements)
    })?;

    Some((
        DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::EnumType { name, elements },
            location: (start..lexer.last_range.end).into(),
        },
        None,
    ))
}

fn parse_array_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Option<(DataTypeDeclaration, Option<Statement>)> {
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
        let end = reference.get_location().get_end();
        (
            DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::ArrayType {
                    name,
                    bounds: range,
                    referenced_type: Box::new(reference),
                },
                location: (start..end).into(),
            },
            initializer,
        )
    })
}

/// parse a body and recovers until the given `end_keywords`
fn parse_body_in_region(lexer: &mut ParseSession, end_keywords: Vec<Token>) -> Vec<Statement> {
    parse_any_in_region(lexer, end_keywords, |lexer| parse_body_standalone(lexer))
}

fn parse_body_standalone(lexer: &mut ParseSession) -> Vec<Statement> {
    let mut statements = Vec::new();
    while !lexer.closes_open_region(&lexer.token) {
        statements.push(parse_control(lexer));
    }
    statements
}

/// parses a statement ending with a ';'
fn parse_statement(lexer: &mut ParseSession) -> Statement {
    let result = parse_any_in_region(lexer, vec![KeywordSemicolon, KeywordColon], |lexer| {
        parse_expression(lexer)
    });
    if lexer.last_token == KeywordColon {
        Statement::CaseCondition {
            condition: Box::new(result),
            id: lexer.next_id(),
        }
    } else {
        result
    }
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

fn parse_reference(lexer: &mut ParseSession) -> Statement {
    match expressions_parser::parse_qualified_reference(lexer) {
        Ok(statement) => statement,
        Err(diagnostic) => {
            let statement = Statement::EmptyStatement {
                location: diagnostic.get_location(),
                id: lexer.next_id(),
            };
            lexer.accept_diagnostic(diagnostic);
            statement
        }
    }
}

fn parse_control(lexer: &mut ParseSession) -> Statement {
    parse_control_statement(lexer)
}

fn parse_variable_block_type(block_type: &Token) -> VariableBlockType {
    match block_type {
        KeywordVar => VariableBlockType::Local,
        KeywordVarTemp => VariableBlockType::Temp,
        KeywordVarInput => VariableBlockType::Input,
        KeywordVarOutput => VariableBlockType::Output,
        KeywordVarGlobal => VariableBlockType::Global,
        KeywordVarInOut => VariableBlockType::InOut,
        _ => VariableBlockType::Local,
    }
}

fn parse_variable_block(
    lexer: &mut ParseSession,
    variable_block_type: VariableBlockType,
) -> VariableBlock {
    //Consume the type keyword
    lexer.advance();

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

        if lexer.token == KeywordColon {
            break;
        }

        if !lexer.allow(&KeywordComma) {
            let next_token_start = lexer.location().get_start();
            lexer.accept_diagnostic(Diagnostic::missing_token(
                format!("{:?} or {:?}", KeywordColon, KeywordComma),
                SourceRange::new(identifier_end..next_token_start),
            ));
        }
    }

    // colon has to come before the data type
    if !lexer.allow(&KeywordColon) {
        lexer.accept_diagnostic(Diagnostic::missing_token(
            format!("{:?}", KeywordColon),
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
            });
        }
    }
    variables
}
