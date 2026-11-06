//! A pure token-stream stack machine that decides, for each token, its indentation depth and
//! whether it starts a new line. No AST is built: every ST block keyword has a disjoint,
//! unambiguous closer (verified against `compiler/plc_lexer/src/lexer/tokens.rs`), so the block
//! structure can be tracked with a simple stack instead of a parse tree.
//!
//! Outside of the structurally-forced break points below (block open/close, `;`, the dedent
//! markers), the MVP engine preserves whatever line breaks already exist in the source
//! (`LexedToken::had_newline_before`) rather than reflowing statement-internal expressions.
//! Column-limit line-wrapping is deferred to a later version; `Frame::Paren`/`Frame::Bracket`
//! below exist already as the seam that work will plug into.

use crate::model::LexedToken;
use plc_lexer::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutDecision {
    pub depth: u32,
    pub newline_before: bool,
    /// Whether this token sits inside a `VAR`-family block (any of its variants), as opposed to
    /// executable code. Declaration alignment only applies here; assignment alignment never does.
    pub in_var_block: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutError {
    /// A closing keyword/bracket with nothing on the stack to match it.
    UnmatchedCloser { index: usize },
    /// A block left open at end-of-file.
    UnmatchedOpener,
}

enum Frame {
    Block { opener: Token, owner_depth: u32 },
    Paren,
    Bracket,
}

pub(crate) fn is_opener(tok: Token) -> bool {
    use Token::*;
    matches!(
        tok,
        KeywordProgram
            | KeywordFunction
            | KeywordFunctionBlock
            | KeywordClass
            | KeywordMethod
            | KeywordInterface
            | KeywordPropertyGet
            | KeywordPropertySet
            | KeywordType
            | KeywordStruct
            | KeywordActions
            | KeywordAction
            | KeywordIf
            | KeywordFor
            | KeywordWhile
            | KeywordRepeat
            | KeywordCase
            | KeywordVar
            | KeywordVarInput
            | KeywordVarOutput
            | KeywordVarInOut
            | KeywordVarTemp
            | KeywordVarGlobal
            | KeywordVarConfig
            | KeywordVarExternal
    )
}

/// Whether `tok` opens one of the `VAR*` block variants specifically (a subset of [`is_opener`]).
fn is_var_opener(tok: Token) -> bool {
    use Token::*;
    matches!(
        tok,
        KeywordVar
            | KeywordVarInput
            | KeywordVarOutput
            | KeywordVarInOut
            | KeywordVarTemp
            | KeywordVarGlobal
            | KeywordVarConfig
            | KeywordVarExternal
    )
}

pub(crate) fn is_closer(tok: Token) -> bool {
    use Token::*;
    matches!(
        tok,
        KeywordEndProgram
            | KeywordEndFunction
            | KeywordEndFunctionBlock
            | KeywordEndClass
            | KeywordEndMethod
            | KeywordEndInterface
            | KeywordEndProperty
            | KeywordEndType
            | KeywordEndStruct
            | KeywordEndActions
            | KeywordEndAction
            | KeywordEndIf
            | KeywordEndFor
            | KeywordEndWhile
            | KeywordEndRepeat
            | KeywordEndCase
            | KeywordEndVar
    )
}

/// Whether `closer` is the keyword that ends the block `opener` started. Every `VAR*` variant
/// shares the single `END_VAR` closer; every other pair is 1:1.
fn closes(opener: Token, closer: Token) -> bool {
    use Token::*;
    matches!(
        (opener, closer),
        (KeywordProgram, KeywordEndProgram)
            | (KeywordFunction, KeywordEndFunction)
            | (KeywordFunctionBlock, KeywordEndFunctionBlock)
            | (KeywordClass, KeywordEndClass)
            | (KeywordMethod, KeywordEndMethod)
            | (KeywordInterface, KeywordEndInterface)
            | (KeywordPropertyGet, KeywordEndProperty)
            | (KeywordPropertySet, KeywordEndProperty)
            | (KeywordType, KeywordEndType)
            | (KeywordStruct, KeywordEndStruct)
            | (KeywordActions, KeywordEndActions)
            | (KeywordAction, KeywordEndAction)
            | (KeywordIf, KeywordEndIf)
            | (KeywordFor, KeywordEndFor)
            | (KeywordWhile, KeywordEndWhile)
            | (KeywordRepeat, KeywordEndRepeat)
            | (KeywordCase, KeywordEndCase)
            | (KeywordVar, KeywordEndVar)
            | (KeywordVarInput, KeywordEndVar)
            | (KeywordVarOutput, KeywordEndVar)
            | (KeywordVarInOut, KeywordEndVar)
            | (KeywordVarTemp, KeywordEndVar)
            | (KeywordVarGlobal, KeywordEndVar)
            | (KeywordVarConfig, KeywordEndVar)
            | (KeywordVarExternal, KeywordEndVar)
    )
}

/// `ELSIF`/`ELSE` (inside `IF`) and `UNTIL` (inside `REPEAT`) dedent to the depth of the block
/// they belong to, rather than starting a new frame. `ELSE` inside `CASE` is different: it is
/// just another branch at the body's own depth, not dedented, since `KeywordElse` is shared
/// between `IF` and `CASE` but means something structurally different in each.
fn dedent_target(tok: Token, stack: &[Frame]) -> Option<u32> {
    let Some(Frame::Block { opener, owner_depth }) = stack.last() else { return None };
    match (tok, opener) {
        (Token::KeywordElseIf | Token::KeywordElse, Token::KeywordIf) => Some(*owner_depth),
        (Token::KeywordElse, Token::KeywordCase) => Some(*owner_depth + 1),
        (Token::KeywordUntil, Token::KeywordRepeat) => Some(*owner_depth),
        _ => None,
    }
}

/// Keywords whose body starts on the line after them, even if the source had it inline: the
/// token that follows always starts a new line at the current depth. `PROGRAM`/`FUNCTION`/...
/// are deliberately not here: their header (name, optional `: return type`) can continue on the
/// same line, and with no AST there is no reliable token-only way to tell where that header
/// ends, so the body-starts-on-its-own-line convention for those headers relies on the source
/// already having a line break there (the common case in practice). `ELSE` is here (unlike
/// `ELSIF`, whose own condition continues on the same line, ended by its own `THEN`) because it
/// has no header content of its own before its body, in both the `IF` and `CASE` contexts.
fn forces_break_after(tok: Token) -> bool {
    use Token::*;
    matches!(
        tok,
        KeywordVar
            | KeywordVarInput
            | KeywordVarOutput
            | KeywordVarInOut
            | KeywordVarTemp
            | KeywordVarGlobal
            | KeywordVarConfig
            | KeywordVarExternal
            | KeywordThen
            | KeywordElse
            | KeywordDo
            | KeywordOf
            | KeywordRepeat
            | KeywordStruct
            | KeywordActions
            | KeywordSemicolon
    )
}

pub fn compute_layout(tokens: &[LexedToken]) -> Result<Vec<LayoutDecision>, LayoutError> {
    let mut stack: Vec<Frame> = Vec::new();
    let mut depth: u32 = 0;
    let mut decisions = Vec::with_capacity(tokens.len());
    let mut force_break_after_prev = false;

    for (i, lt) in tokens.iter().enumerate() {
        let tok = lt.token;
        let dedent = dedent_target(tok, &stack);

        let (print_depth, should_pop) = if is_closer(tok) {
            match stack.last() {
                Some(Frame::Block { opener, owner_depth }) if closes(*opener, tok) => (*owner_depth, true),
                _ => return Err(LayoutError::UnmatchedCloser { index: i }),
            }
        } else if let Some(target) = dedent {
            (target, false)
        } else {
            (depth, false)
        };

        let forced_before = is_closer(tok) || is_opener(tok) || dedent.is_some();
        let newline_before = forced_before || force_break_after_prev || lt.had_newline_before;
        let in_var_block =
            matches!(stack.last(), Some(Frame::Block { opener, .. }) if is_var_opener(*opener));
        decisions.push(LayoutDecision { depth: print_depth, newline_before, in_var_block });

        if should_pop {
            stack.pop();
            depth -= 1;
        }
        if is_opener(tok) {
            stack.push(Frame::Block { opener: tok, owner_depth: depth });
            depth += 1;
        }

        match tok {
            Token::KeywordParensOpen => stack.push(Frame::Paren),
            Token::KeywordSquareParensOpen => stack.push(Frame::Bracket),
            Token::KeywordParensClose => match stack.last() {
                Some(Frame::Paren) => {
                    stack.pop();
                }
                _ => return Err(LayoutError::UnmatchedCloser { index: i }),
            },
            Token::KeywordSquareParensClose => match stack.last() {
                Some(Frame::Bracket) => {
                    stack.pop();
                }
                _ => return Err(LayoutError::UnmatchedCloser { index: i }),
            },
            _ => {}
        }

        force_break_after_prev = forces_break_after(tok);
    }

    if !stack.is_empty() {
        return Err(LayoutError::UnmatchedOpener);
    }

    Ok(decisions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lex::lex_tokens, trivia::build_document};

    fn layout(source: &str) -> Vec<LayoutDecision> {
        let doc = build_document(source, lex_tokens(source).unwrap());
        compute_layout(&doc.tokens).unwrap()
    }

    /// `depth` only matters where `newline_before` is true (that's the only time the printer
    /// indents); elsewhere its value is unconstrained, so only line-starting tokens are checked.
    fn line_starts(decisions: &[LayoutDecision]) -> Vec<(usize, u32)> {
        decisions.iter().enumerate().filter(|(_, d)| d.newline_before).map(|(i, d)| (i, d.depth)).collect()
    }

    #[test]
    fn if_then_else_end_if_depths() {
        let decisions = layout("IF x THEN\ny:=1;\nELSE\ny:=2;\nEND_IF\n");
        // IF(0) opens at 0; y(3) body at 1; ELSE(7) dedents to 0; y(8) body at 1; END_IF(12) at 0.
        assert_eq!(line_starts(&decisions), vec![(0, 0), (3, 1), (7, 0), (8, 1), (12, 0)]);
    }

    #[test]
    fn var_block_indents_members() {
        let decisions = layout("VAR\nx:INT;\nEND_VAR\n");
        let depths: Vec<u32> = decisions.iter().map(|d| d.depth).collect();
        // VAR, x, :, INT, ;, END_VAR
        assert_eq!(depths, vec![0, 1, 1, 1, 1, 0]);
    }

    #[test]
    fn semicolon_forces_newline_even_on_one_source_line() {
        let decisions = layout("a:=1;b:=2;");
        let newlines: Vec<bool> = decisions.iter().map(|d| d.newline_before).collect();
        // a, :=, 1, ;, b, :=, 2, ;
        assert_eq!(newlines, vec![false, false, false, false, true, false, false, false]);
    }

    #[test]
    fn unmatched_end_if_is_an_error() {
        let doc = build_document("END_IF", lex_tokens("END_IF").unwrap());
        assert_eq!(compute_layout(&doc.tokens), Err(LayoutError::UnmatchedCloser { index: 0 }));
    }

    #[test]
    fn unclosed_if_is_an_error() {
        let doc = build_document("IF x THEN y:=1;", lex_tokens("IF x THEN y:=1;").unwrap());
        assert_eq!(compute_layout(&doc.tokens), Err(LayoutError::UnmatchedOpener));
    }

    #[test]
    fn case_else_is_not_dedented_like_if_else() {
        let decisions = layout("CASE x OF\n1:y:=1;\nELSE\ny:=2;\nEND_CASE\n");
        // CASE(0) opens at 0; label "1"(3) body at 1; ELSE(9) stays at body depth 1 (not
        // dedented, unlike IF's ELSE); y(10) body at 1; END_CASE(14) at 0.
        assert_eq!(line_starts(&decisions), vec![(0, 0), (3, 1), (9, 1), (10, 1), (14, 0)]);
    }
}
