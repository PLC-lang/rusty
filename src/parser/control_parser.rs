use plc_ast::{
    ast::{AstFactory, AstNode},
    control_statements::{CaseStatement, ConditionalBlock, ForLoopStatement, IfStatement, LoopStatement},
};
use plc_diagnostics::diagnostics::Diagnostic;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    expect_token,
    lexer::Token::{self, *},
    parser::{parse_any_in_region_until_element_when, parse_body_in_region},
};

use super::recovery;
use super::ParseSession;
use super::{parse_expression, parse_reference, parse_statement};

pub fn parse_control_statement(lexer: &mut ParseSession) -> AstNode {
    match lexer.token {
        KeywordIf => parse_if_statement(lexer),
        KeywordFor => parse_for_statement(lexer),
        KeywordWhile => parse_while_statement(lexer),
        KeywordRepeat => parse_repeat_statement(lexer),
        KeywordCase => parse_case_statement(lexer),
        KeywordReturn => parse_return_statement(lexer),
        KeywordContinue => parse_continue_statement(lexer),
        KeywordExit => parse_exit_statement(lexer),
        _ => parse_statement(lexer),
    }
}

fn parse_return_statement(lexer: &mut ParseSession) -> AstNode {
    let location = lexer.location();
    lexer.advance();
    AstFactory::create_return_statement(None, location, lexer.next_id())
}

fn parse_exit_statement(lexer: &mut ParseSession) -> AstNode {
    let location = lexer.location();
    lexer.advance();
    AstFactory::create_exit_statement(location, lexer.next_id())
}

fn parse_continue_statement(lexer: &mut ParseSession) -> AstNode {
    let location = lexer.location();
    lexer.advance();
    AstFactory::create_continue_statement(location, lexer.next_id())
}

fn parse_if_statement(lexer: &mut ParseSession) -> AstNode {
    let start = lexer.range().start;
    lexer.advance(); //If
    let mut conditional_blocks = vec![];

    while lexer.last_token == KeywordElseIf || lexer.last_token == KeywordIf {
        let condition = parse_expression(lexer);
        lexer.try_consume_or_report(KeywordThen);

        let condition_block = ConditionalBlock {
            condition: Box::new(condition),
            body: parse_body_in_region(lexer, vec![KeywordEndIf, KeywordElseIf, KeywordElse]),
        };

        conditional_blocks.push(condition_block);
    }

    let mut else_block = Vec::new();

    if lexer.last_token == KeywordElse {
        else_block.append(&mut parse_body_in_region(lexer, vec![KeywordEndIf]));
    }

    let end = lexer.last_range.end;

    let stmt = IfStatement { blocks: conditional_blocks, else_block, end_location: lexer.last_location() };
    AstFactory::create_if_statement(
        stmt,
        lexer.source_range_factory.create_range(start..end),
        lexer.next_id(),
    )
}

fn parse_for_statement(lexer: &mut ParseSession) -> AstNode {
    let start = lexer.range().start;
    lexer.advance(); // FOR

    let counter_expression = parse_reference(lexer);
    expect_token!(
        lexer,
        KeywordAssignment,
        AstFactory::create_empty_statement(lexer.location(), lexer.next_id())
    );
    lexer.advance();

    let start_expression = parse_expression(lexer);
    if !lexer.try_consume(KeywordTo) {
        if lexer.token == Identifier && lexer.peek_token() == Identifier {
            lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                KeywordTo.to_string().as_str(),
                lexer.slice(),
                lexer.location(),
            ));
            return AstFactory::create_empty_statement(lexer.location(), lexer.next_id());
        }

        lexer.accept_diagnostic(Diagnostic::missing_token(KeywordTo.to_string().as_str(), lexer.location()));
    }
    let end_expression = parse_expression(lexer);

    let step = if lexer.token == KeywordBy {
        lexer.advance(); // BY
        Some(parse_expression(lexer))
    } else {
        None
    };

    lexer.try_consume_or_report(KeywordDo); // DO

    let stmt = ForLoopStatement {
        counter: Box::new(counter_expression),
        start: Box::new(start_expression),
        end: Box::new(end_expression),
        by_step: step.map(Box::new),
        body: parse_body_in_region(lexer, vec![KeywordEndFor]),
        end_location: lexer.last_location(),
    };
    AstFactory::create_for_loop(
        stmt,
        lexer.source_range_factory.create_range(start..lexer.last_range.end),
        lexer.next_id(),
    )
}

fn parse_while_statement(lexer: &mut ParseSession) -> AstNode {
    let start = lexer.range().start;
    lexer.advance(); //WHILE

    let condition = parse_expression(lexer);
    lexer.try_consume_or_report(KeywordDo);

    let stmt = LoopStatement {
        condition: Box::new(condition),
        body: parse_body_in_region(lexer, vec![KeywordEndWhile]),
        end_location: lexer.last_location(),
    };
    AstFactory::create_while_statement(
        stmt,
        lexer.source_range_factory.create_range(start..lexer.last_range.end),
        lexer.next_id(),
    )
}

fn parse_repeat_statement(lexer: &mut ParseSession) -> AstNode {
    let start = lexer.range().start;
    lexer.advance(); //REPEAT

    let body = parse_body_in_region(lexer, vec![KeywordUntil, KeywordEndRepeat]); //UNTIL
    let condition = if lexer.last_token == KeywordUntil {
        let condition = parse_expression(lexer);
        lexer.try_consume_or_report(KeywordEndRepeat);
        condition
    } else {
        AstFactory::create_empty_statement(lexer.location(), lexer.next_id())
    };

    let stmt = LoopStatement { condition: Box::new(condition), body, end_location: lexer.last_location() };
    AstFactory::create_repeat_statement(
        stmt,
        lexer.source_range_factory.create_range(start..lexer.last_range.end),
        lexer.next_id(),
    )
}

fn parse_case_statement(lexer: &mut ParseSession) -> AstNode {
    let start = lexer.range().start;
    lexer.advance(); // CASE

    let selector = parse_expression(lexer);

    lexer.try_consume_or_report(KeywordOf);

    let mut case_blocks = Vec::new();

    while !lexer.closes_open_region(&lexer.token) && !is_case_block_boundary(lexer) {
        let condition = parse_case_condition(lexer);
        let body = parse_case_selection_body(lexer);
        case_blocks.push(ConditionalBlock { condition, body });
    }

    let else_block = if lexer.try_consume(KeywordElse) {
        parse_body_in_region(lexer, vec![KeywordEndCase])
    } else {
        lexer.try_consume_or_report(KeywordEndCase);
        vec![]
    };

    let end = lexer.last_range.end;
    let stmt = CaseStatement {
        selector: Box::new(selector),
        case_blocks,
        else_block,
        end_location: lexer.last_location(),
    };
    AstFactory::create_case_statement(
        stmt,
        lexer.source_range_factory.create_range(start..end),
        lexer.next_id(),
    )
}

fn parse_case_condition(lexer: &mut ParseSession) -> Box<AstNode> {
    if lexer.try_consume(KeywordColon) {
        return Box::new(AstFactory::create_empty_statement(lexer.last_location(), lexer.next_id()));
    }

    let condition = parse_any_in_region_until_element_when(
        lexer,
        vec![KeywordColon],
        recovery::STATEMENT_BLOCK_BOUNDARY,
        None,
        is_case_condition_recovery_boundary,
        parse_expression,
    );

    Box::new(condition)
}

fn parse_case_selection_body(lexer: &mut ParseSession) -> Vec<AstNode> {
    let recovery_tokens =
        recovery::combine(&[KeywordEndCase, KeywordElse], recovery::STATEMENT_BLOCK_BOUNDARY);
    let mut statements = Vec::new();
    while !lexer.closes_open_region(&lexer.token)
        && !is_case_block_boundary(lexer)
        && lexer.token != KeywordColon
        && !is_case_selection_start(lexer)
    {
        let statement = match lexer.token {
            KeywordIf | KeywordFor | KeywordWhile | KeywordRepeat | KeywordCase | KeywordReturn
            | KeywordContinue | KeywordExit => parse_control_statement(lexer),
            _ => parse_case_body_statement(lexer, &recovery_tokens),
        };
        statements.push(statement);
    }

    statements
}

fn parse_case_body_statement(lexer: &mut ParseSession, recovery_tokens: &[Token]) -> AstNode {
    let result = parse_any_in_region_until_element_when(
        lexer,
        vec![KeywordSemicolon, KeywordColon],
        recovery_tokens,
        None,
        is_case_selection_start,
        parse_expression,
    );
    if lexer.last_token == KeywordColon {
        let location = result.location.span(&lexer.last_location());
        AstFactory::create_case_condition(result, location, lexer.next_id())
    } else {
        result
    }
}

fn is_case_condition_recovery_boundary(lexer: &ParseSession) -> bool {
    is_case_block_boundary(lexer) || is_case_selection_start(lexer) || is_statement_start(lexer)
}

fn is_case_block_boundary(lexer: &ParseSession) -> bool {
    lexer.token == KeywordEndCase
        || lexer.token == KeywordElse
        || recovery::STATEMENT_BLOCK_BOUNDARY.contains(&lexer.token)
}

fn is_case_selection_start(lexer: &ParseSession) -> bool {
    matches!(
        lexer.token,
        Identifier
            | KeywordPropertyGet
            | KeywordPropertySet
            | LiteralIntegerHex
            | LiteralIntegerOct
            | LiteralIntegerBin
            | LiteralInteger
            | LiteralNull
            | LiteralTrue
            | LiteralFalse
            | LiteralDate
            | LiteralDateAndTime
            | LiteralTimeOfDay
            | LiteralTime
            | LiteralString
            | LiteralWideString
            | TypeCastPrefix
            | KeywordParensOpen
    ) && lexer.token_appears_before(KeywordColon, &[KeywordSemicolon, KeywordEndCase, KeywordElse])
}

fn is_statement_start(lexer: &ParseSession) -> bool {
    matches!(
        lexer.token,
        Identifier
            | KeywordPropertyGet
            | KeywordPropertySet
            | KeywordIf
            | KeywordFor
            | KeywordWhile
            | KeywordRepeat
            | KeywordCase
            | KeywordReturn
            | KeywordContinue
            | KeywordExit
            | KeywordSuper
    )
}
