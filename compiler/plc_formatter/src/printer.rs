//! Re-serializes a `Document` driven by `layout::compute_layout`'s per-token decisions back
//! into formatted source text.
//!
//! Printing is two passes. First, every token (and every standalone comment) is grouped into a
//! logical output line and rendered to text, recording the byte offset of its `:`/`:=` alignment
//! anchor if it has one (`render_code_line`). Second, if alignment is enabled, consecutive
//! eligible lines are padded so their anchors - or their trailing comments - line up in the same
//! column (`apply_*_alignment`). Only then is the final string assembled, so a line's anchor
//! offsets are always measured on its own already-padded text.

use std::ops::Range;

use plc_lexer::Token;

use crate::config::{FormatConfig, KeywordCase};
use crate::layout::{is_closer, is_opener, LayoutDecision};
use crate::model::{Document, LexedToken, Trivia};

// Token-level text rules

/// Keyword-like tokens whose text is a pure, case-insensitive word (every `#[token(..,
/// ignore(case))]` match in `compiler/plc_lexer/src/lexer/tokens.rs` that is not also a literal
/// carrying data, e.g. a date/time literal keeps its original casing in v1 since rewriting only
/// part of its text is more delicate than this MVP needs). Identifiers and built-in type names
/// such as `INT`/`BOOL` are plain `Token::Identifier`s to this lexer, not keywords, and are
/// therefore never touched here.
fn is_case_adjustable(tok: Token) -> bool {
    use Token::*;
    matches!(
        tok,
        KeywordProgram
            | KeywordClass
            | KeywordEndClass
            | KeywordExtends
            | KeywordImplements
            | KeywordInterface
            | KeywordEndInterface
            | KeywordVarInput
            | KeywordVarOutput
            | KeywordVar
            | KeywordVarConfig
            | KeywordAbstract
            | KeywordFinal
            | KeywordMethod
            | KeywordEndMethod
            | KeywordSuper
            | KeywordThis
            | KeywordPropertyGet
            | KeywordPropertySet
            | KeywordEndProperty
            | KeywordConstant
            | KeywordRetain
            | KeywordNonRetain
            | KeywordVarTemp
            | KeywordAccessPublic
            | KeywordAccessPrivate
            | KeywordAccessInternal
            | KeywordAccessProtected
            | KeywordOverride
            | KeywordVarGlobal
            | KeywordVarInOut
            | KeywordVarExternal
            | KeywordEndVar
            | KeywordEndProgram
            | KeywordFunction
            | KeywordEndFunction
            | KeywordFunctionBlock
            | KeywordEndFunctionBlock
            | KeywordType
            | KeywordStruct
            | KeywordEndType
            | KeywordEndStruct
            | KeywordActions
            | KeywordAction
            | KeywordEndAction
            | KeywordEndActions
            | KeywordIf
            | KeywordThen
            | KeywordElseIf
            | KeywordElse
            | KeywordEndIf
            | KeywordFor
            | KeywordTo
            | KeywordBy
            | KeywordDo
            | KeywordEndFor
            | KeywordWhile
            | KeywordEndWhile
            | KeywordRepeat
            | KeywordUntil
            | KeywordEndRepeat
            | KeywordCase
            | KeywordReturn
            | KeywordExit
            | KeywordContinue
            | KeywordPointer
            | KeywordFunctionPointer
            | KeywordRef
            | KeywordReferenceTo
            | KeywordArray
            | KeywordString
            | KeywordWideString
            | KeywordOf
            | KeywordAt
            | KeywordEndCase
            | OperatorModulo
            | OperatorAndThen
            | OperatorAnd
            | OperatorOrElse
            | OperatorOr
            | OperatorXor
            | OperatorNot
            | LiteralNull
            | LiteralTrue
            | LiteralFalse
    )
}

fn token_text(tok: &LexedToken, config: &FormatConfig) -> String {
    if is_case_adjustable(tok.token) {
        match config.keyword_case {
            KeywordCase::Upper => tok.text.to_uppercase(),
            KeywordCase::Lower => tok.text.to_lowercase(),
            KeywordCase::Preserve => tok.text.clone(),
        }
    } else {
        tok.text.clone()
    }
}

fn is_binary_operator(tok: Token) -> bool {
    use Token::*;
    matches!(
        tok,
        OperatorPlus
            | OperatorMinus
            | OperatorMultiplication
            | OperatorExponent
            | OperatorDivision
            | OperatorEqual
            | OperatorNotEqual
            | OperatorLess
            | OperatorGreater
            | OperatorLessOrEqual
            | OperatorGreaterOrEqual
            | OperatorAmp
            | OperatorModulo
            | OperatorAndThen
            | OperatorAnd
            | OperatorOrElse
            | OperatorOr
            | OperatorXor
    )
}

/// Whether a space belongs between two tokens staying on the same line. Unary vs. binary `+`/`-`
/// and the exact spacing around postfix `^` dereferences are not disambiguated without an AST;
/// this always treats them as binary/glued respectively, which is cosmetic-only and never
/// changes the formatted file's meaning.
fn needs_space_before(prev: Token, cur: Token, config: &FormatConfig) -> bool {
    use Token::*;
    if matches!(
        prev,
        KeywordParensOpen
            | KeywordSquareParensOpen
            | KeywordDot
            | KeywordDotDot
            | KeywordDotDotDot
            | TypeCastPrefix
    ) {
        return false;
    }
    if matches!(
        cur,
        KeywordParensClose
            | KeywordSquareParensClose
            | KeywordSemicolon
            | KeywordComma
            | KeywordDot
            | KeywordDotDot
            | KeywordDotDotDot
    ) {
        return false;
    }
    if prev == OperatorDeref || cur == OperatorDeref {
        return false;
    }
    if cur == KeywordColon {
        return config.space_before_colon;
    }
    if prev == KeywordColon {
        return config.space_after_colon;
    }
    if prev == KeywordComma {
        return config.space_after_comma;
    }
    if cur == KeywordAssignment || prev == KeywordAssignment {
        return config.space_around_assignment;
    }
    if is_binary_operator(cur) || is_binary_operator(prev) {
        return config.space_around_binary_operators;
    }
    true
}

// Logical lines

/// One output line: either a standalone comment, or a run of real tokens that stay on one line.
enum LineKind<'a> {
    Comment(&'a Trivia),
    Code(Vec<usize>),
}

struct Line<'a> {
    depth: u32,
    blank_lines_before: u32,
    kind: LineKind<'a>,
}

fn flush_code_line<'a>(lines: &mut Vec<Line<'a>>, current: &mut Option<(u32, u32, Vec<usize>)>) {
    if let Some((depth, blank_lines_before, idxs)) = current.take() {
        lines.push(Line { depth, blank_lines_before, kind: LineKind::Code(idxs) });
    }
}

/// Groups every real token (and every standalone comment) into the logical line it prints on.
/// Mirrors exactly which token starts a new line in the single-pass predecessor of this printer:
/// a token starts one when `decision.newline_before` is set, when it carries leading comments
/// (each comment is its own line, and the token itself always follows on a fresh one), or when
/// nothing has been opened yet.
fn build_lines<'a>(doc: &'a Document, decisions: &[LayoutDecision]) -> Vec<Line<'a>> {
    let mut lines = Vec::new();
    let mut current: Option<(u32, u32, Vec<usize>)> = None;

    for (i, (tok, decision)) in doc.tokens.iter().zip(decisions.iter()).enumerate() {
        if !tok.leading_trivia.is_empty() {
            flush_code_line(&mut lines, &mut current);
            for item in &tok.leading_trivia {
                lines.push(Line {
                    depth: decision.depth,
                    blank_lines_before: item.blank_lines_before,
                    kind: LineKind::Comment(&item.comment),
                });
            }
        }

        let starts_new_line = decision.newline_before || current.is_none() || !tok.leading_trivia.is_empty();
        if starts_new_line {
            flush_code_line(&mut lines, &mut current);
            current = Some((decision.depth, tok.blank_lines_before, vec![i]));
        } else {
            current.as_mut().expect("just ensured by starts_new_line").2.push(i);
        }
    }
    flush_code_line(&mut lines, &mut current);

    for item in &doc.trailing_trivia {
        lines.push(Line {
            depth: 0,
            blank_lines_before: item.blank_lines_before,
            kind: LineKind::Comment(&item.comment),
        });
    }
    lines
}

// Rendering a line to text, with alignment anchors

enum Rendered {
    Comment {
        depth: u32,
        blank_lines_before: u32,
        text: String,
    },
    Code {
        depth: u32,
        blank_lines_before: u32,
        text: String,
        in_var_block: bool,
        /// Byte offset in `text` right before a declaration's `:`, if this line is a `VAR`-block
        /// declaration. `None` outside a `VAR` block, even if a `:` appears (e.g. a `CASE` label).
        colon_offset: Option<usize>,
        /// Byte offset in `text` right before a top-level `:=`, whether this is a declaration's
        /// default value or a body statement's assignment - the two alignment passes tell them
        /// apart using `in_var_block`.
        assign_offset: Option<usize>,
        trailing_comment: Option<String>,
        comment_spacing: u32,
        is_opener_line: bool,
        is_closer_line: bool,
    },
}

/// Finds this line's `:` and `:=` alignment anchors: the first of each that appears outside any
/// `()`/`[]` nesting within the line. A declaration's default-value `:=` is always the first
/// top-level one after its `:`, and a body statement has only one top-level `:=` to begin with
/// (e.g. the named-argument `:=` in `foo(x := 1)` is nested inside the call's parens, at depth
/// 1, so it is correctly skipped).
fn find_anchors(doc: &Document, idxs: &[usize]) -> (Option<usize>, Option<usize>) {
    let mut depth = 0i32;
    let mut colon = None;
    let mut assign = None;
    for &i in idxs {
        match doc.tokens[i].token {
            Token::KeywordParensOpen | Token::KeywordSquareParensOpen => depth += 1,
            Token::KeywordParensClose | Token::KeywordSquareParensClose => depth -= 1,
            Token::KeywordColon if depth == 0 && colon.is_none() => colon = Some(i),
            Token::KeywordAssignment if depth == 0 && assign.is_none() => assign = Some(i),
            _ => {}
        }
    }
    (colon, assign)
}

fn render_code_line(
    doc: &Document,
    decisions: &[LayoutDecision],
    depth: u32,
    blank_lines_before: u32,
    idxs: &[usize],
    config: &FormatConfig,
) -> Rendered {
    let in_var_block = decisions[idxs[0]].in_var_block;
    let (colon_tok, assign_tok) = find_anchors(doc, idxs);

    let mut text = String::new();
    let mut colon_offset = None;
    let mut assign_offset = None;
    let mut prev: Option<Token> = None;

    for (pos, &i) in idxs.iter().enumerate() {
        let tok = &doc.tokens[i];
        if let Some(p) = prev {
            if needs_space_before(p, tok.token, config) {
                text.push(' ');
            }
        }
        if colon_tok == Some(i) {
            colon_offset = Some(text.len());
        }
        if assign_tok == Some(i) {
            assign_offset = Some(text.len());
        }
        text.push_str(&token_text(tok, config));

        // An inline comment on a token that is not the last one on the line stays glued where it
        // is, rather than becoming this line's (singular) aligned trailing comment.
        if pos + 1 != idxs.len() {
            for (j, trivia) in tok.trailing_trivia.iter().enumerate() {
                if j == 0 {
                    text.extend(std::iter::repeat_n(' ', config.trailing_comment_min_spacing as usize));
                } else {
                    text.push(' ');
                }
                text.push_str(trivia.text());
            }
        }
        prev = Some(tok.token);
    }

    let last = &doc.tokens[*idxs.last().expect("a code line always has at least one token")];
    let trailing_comment = (!last.trailing_trivia.is_empty())
        .then(|| last.trailing_trivia.iter().map(Trivia::text).collect::<Vec<_>>().join(" "));

    let first_token = doc.tokens[idxs[0]].token;
    Rendered::Code {
        depth,
        blank_lines_before,
        text,
        in_var_block,
        colon_offset: if in_var_block { colon_offset } else { None },
        assign_offset,
        trailing_comment,
        comment_spacing: config.trailing_comment_min_spacing,
        is_opener_line: is_opener(first_token),
        is_closer_line: is_closer(first_token),
    }
}

// Alignment

fn blank_before(line: &Rendered) -> u32 {
    match line {
        Rendered::Comment { blank_lines_before, .. } | Rendered::Code { blank_lines_before, .. } => {
            *blank_lines_before
        }
    }
}

/// Groups line indices into maximal runs of 2+ consecutive lines that all satisfy `qualifies`,
/// breaking a run at a blank line or at any line that does not qualify. A lone qualifying line
/// has nothing to align against, so it is dropped (not returned as a length-1 run).
fn qualifying_runs(lines: &[Rendered], qualifies: impl Fn(&Rendered) -> bool) -> Vec<Range<usize>> {
    let mut runs = Vec::new();
    let mut start: Option<usize> = None;
    for (i, line) in lines.iter().enumerate() {
        let q = qualifies(line);
        if !q || blank_before(line) > 0 {
            if let Some(s) = start.take() {
                runs.push(s..i);
            }
        }
        if q && start.is_none() {
            start = Some(i);
        }
    }
    if let Some(s) = start {
        runs.push(s..lines.len());
    }
    runs.into_iter().filter(|r| r.len() > 1).collect()
}

fn is_declaration(line: &Rendered) -> bool {
    matches!(line, Rendered::Code { in_var_block: true, colon_offset: Some(_), .. })
}

fn is_body_assignment(line: &Rendered) -> bool {
    matches!(line, Rendered::Code { in_var_block: false, assign_offset: Some(_), .. })
}

/// Aligns both the `:` and, among the lines that have a default value, the `:=` of every run of
/// consecutive `VAR`-block declarations - the colon first (every declaration in the run has
/// one), then the assignment (shifted by however much the colon step already padded that line).
/// Each step reads every line's current column before mutating any of them, since a line's
/// `let ... = &mut lines[i]` borrow cannot coexist with reading `lines[i]` again to compute it.
fn apply_declaration_alignment(lines: &mut [Rendered], config: &FormatConfig) {
    for run in qualifying_runs(lines, is_declaration) {
        let colon_target = run
            .clone()
            .map(|i| match &lines[i] {
                Rendered::Code { depth, colon_offset: Some(off), .. } => {
                    depth * config.indent_width + *off as u32
                }
                _ => unreachable!("is_declaration guarantees a colon_offset"),
            })
            .max()
            .unwrap_or(0);

        for i in run.clone() {
            let current = match &lines[i] {
                Rendered::Code { depth, colon_offset: Some(off), .. } => {
                    depth * config.indent_width + *off as u32
                }
                _ => continue,
            };
            let pad = (colon_target - current) as usize;
            if pad == 0 {
                continue;
            }
            if let Rendered::Code { text, colon_offset: Some(off), assign_offset, .. } = &mut lines[i] {
                text.insert_str(*off, &" ".repeat(pad));
                if let Some(a) = assign_offset {
                    *a += pad;
                }
            }
        }

        let assign_target = run
            .clone()
            .filter_map(|i| match &lines[i] {
                Rendered::Code { depth, assign_offset: Some(off), .. } => {
                    Some(depth * config.indent_width + *off as u32)
                }
                _ => None,
            })
            .max();
        let Some(assign_target) = assign_target else { continue };

        for i in run {
            let current = match &lines[i] {
                Rendered::Code { depth, assign_offset: Some(off), .. } => {
                    Some(depth * config.indent_width + *off as u32)
                }
                _ => None,
            };
            let Some(current) = current else { continue };
            let pad = (assign_target - current) as usize;
            if pad == 0 {
                continue;
            }
            if let Rendered::Code { text, assign_offset: Some(off), .. } = &mut lines[i] {
                text.insert_str(*off, &" ".repeat(pad));
            }
        }
    }
}

/// Aligns the `:=` of every run of consecutive simple assignment statements in program/function
/// bodies - logic code, as opposed to the declarations [`apply_declaration_alignment`] handles.
fn apply_assignment_alignment(lines: &mut [Rendered], config: &FormatConfig) {
    for run in qualifying_runs(lines, is_body_assignment) {
        let target = run
            .clone()
            .map(|i| match &lines[i] {
                Rendered::Code { depth, assign_offset: Some(off), .. } => {
                    depth * config.indent_width + *off as u32
                }
                _ => unreachable!("is_body_assignment guarantees an assign_offset"),
            })
            .max()
            .unwrap_or(0);

        for i in run {
            let current = match &lines[i] {
                Rendered::Code { depth, assign_offset: Some(off), .. } => {
                    depth * config.indent_width + *off as u32
                }
                _ => continue,
            };
            let pad = (target - current) as usize;
            if pad == 0 {
                continue;
            }
            if let Rendered::Code { text, assign_offset: Some(off), .. } = &mut lines[i] {
                text.insert_str(*off, &" ".repeat(pad));
            }
        }
    }
}

/// Aligns trailing comments across any run of adjacent lines (regardless of what the lines
/// otherwise look like - declarations, statements, standalone comments), broken only by a blank
/// line. Unlike the other two passes this needs 2+ lines that actually *have* a trailing
/// comment, not 2+ lines of the same kind, so it does its own grouping instead of
/// `qualifying_runs`.
fn apply_trailing_comment_alignment(lines: &mut [Rendered], config: &FormatConfig) {
    let mut blocks: Vec<Range<usize>> = Vec::new();
    let mut start = 0usize;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if blank_before(line) > 0 {
            blocks.push(start..i);
            start = i;
        }
    }
    if !lines.is_empty() {
        blocks.push(start..lines.len());
    }

    for block in blocks {
        let eligible: Vec<usize> = block
            .filter(|&i| matches!(&lines[i], Rendered::Code { trailing_comment: Some(_), .. }))
            .collect();
        if eligible.len() < 2 {
            continue;
        }

        let target = eligible
            .iter()
            .map(|&i| match &lines[i] {
                Rendered::Code { depth, text, .. } => depth * config.indent_width + text.len() as u32,
                _ => unreachable!("filtered to Code lines above"),
            })
            .max()
            .unwrap_or(0);

        for i in eligible {
            let current = match &lines[i] {
                Rendered::Code { depth, text, .. } => depth * config.indent_width + text.len() as u32,
                _ => continue,
            };
            let extra = target - current;
            if let Rendered::Code { comment_spacing, .. } = &mut lines[i] {
                *comment_spacing = config.trailing_comment_min_spacing + extra;
            }
        }
    }
}

// Final assembly

fn emit_indent(out: &mut String, depth: u32, config: &FormatConfig) {
    if config.use_tab {
        out.extend(std::iter::repeat_n('\t', depth as usize));
    } else {
        out.extend(std::iter::repeat_n(' ', (depth * config.indent_width) as usize));
    }
}

/// Ends the current line (unless `out` is still empty, i.e. this is the very first line - a
/// formatted file never starts with blank lines), emits up to `max_consecutive_empty_lines`
/// blank lines, then indents to `depth`.
fn start_new_line(out: &mut String, blank_lines: u32, depth: u32, config: &FormatConfig) {
    if out.is_empty() {
        emit_indent(out, depth, config);
        return;
    }
    out.push('\n');
    for _ in 0..blank_lines.min(config.max_consecutive_empty_lines) {
        out.push('\n');
    }
    emit_indent(out, depth, config);
}

pub fn print(doc: &Document, decisions: &[LayoutDecision], config: &FormatConfig) -> String {
    let lines = build_lines(doc, decisions);
    let mut rendered: Vec<Rendered> = lines
        .iter()
        .map(|line| match &line.kind {
            LineKind::Comment(trivia) => Rendered::Comment {
                depth: line.depth,
                blank_lines_before: line.blank_lines_before,
                text: trivia.text().to_string(),
            },
            LineKind::Code(idxs) => {
                render_code_line(doc, decisions, line.depth, line.blank_lines_before, idxs, config)
            }
        })
        .collect();

    if config.align_consecutive_declarations {
        apply_declaration_alignment(&mut rendered, config);
    }
    if config.align_consecutive_assignments {
        apply_assignment_alignment(&mut rendered, config);
    }
    if config.align_trailing_comments {
        apply_trailing_comment_alignment(&mut rendered, config);
    }

    let mut out = String::new();
    let mut prev_was_opener = false;
    for line in &rendered {
        match line {
            Rendered::Comment { depth, blank_lines_before, text } => {
                start_new_line(&mut out, *blank_lines_before, *depth, config);
                out.push_str(text);
            }
            Rendered::Code {
                depth,
                blank_lines_before,
                text,
                trailing_comment,
                comment_spacing,
                is_opener_line,
                is_closer_line,
                ..
            } => {
                let mut blank = *blank_lines_before;
                if config.blank_line_after_block_open && prev_was_opener {
                    blank = blank.max(1);
                }
                if config.blank_line_before_block_close && *is_closer_line {
                    blank = blank.max(1);
                }
                start_new_line(&mut out, blank, *depth, config);
                out.push_str(text);
                if let Some(comment) = trailing_comment {
                    out.extend(std::iter::repeat_n(' ', *comment_spacing as usize));
                    out.push_str(comment);
                }
                prev_was_opener = *is_opener_line;
            }
        }
    }

    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{layout::compute_layout, lex::lex_tokens, trivia::build_document};
    use pretty_assertions::assert_eq;

    fn format(source: &str, config: &FormatConfig) -> String {
        let doc = build_document(source, lex_tokens(source).unwrap());
        let decisions = compute_layout(&doc.tokens).unwrap();
        print(&doc, &decisions, config)
    }

    #[test]
    fn reindents_and_normalizes_spacing() {
        let out = format("IF   x>0 THEN y:=1;ELSE y  :=   2  ;END_IF", &FormatConfig::default());
        assert_eq!(out, "IF x > 0 THEN\n    y := 1;\nELSE\n    y := 2;\nEND_IF\n");
    }

    #[test]
    fn lowercase_keyword_case_only_touches_keywords() {
        let mut config = FormatConfig::default();
        config.keyword_case = KeywordCase::Lower;
        let out = format("IF MyVar THEN END_IF", &config);
        assert_eq!(out, "if MyVar then\nend_if\n");
    }

    #[test]
    fn trailing_comment_stays_on_the_same_line() {
        let out = format("a := 1; // hi\n", &FormatConfig::default());
        assert_eq!(out, "a := 1;  // hi\n");
    }

    #[test]
    fn leading_comment_gets_its_own_indented_line() {
        let out = format("VAR\n// about x\nx : INT;\nEND_VAR\n", &FormatConfig::default());
        assert_eq!(out, "VAR\n    // about x\n    x: INT;\nEND_VAR\n");
    }

    #[test]
    fn is_idempotent() {
        let config = FormatConfig::default();
        let once = format("IF   x>0 THEN y:=1;ELSE y  :=   2  ;END_IF", &config);
        let twice = format(&once, &config);
        assert_eq!(once, twice);
    }

    #[test]
    fn aligns_consecutive_declarations_by_colon_and_assignment() {
        let mut config = FormatConfig::default();
        config.align_consecutive_declarations = true;
        let out = format("VAR\nx:INT;\nlongName:BOOL:=TRUE;\nEND_VAR\n", &config);
        // Both colons land at the same absolute column (12): `longName` is the widest name and
        // keeps its own natural (unpadded, `space_before_colon: false`) position; `x` is padded
        // out to match it. `:=` only has `longName`'s own line to align against.
        assert_eq!(out, "VAR\n    x       : INT;\n    longName: BOOL := TRUE;\nEND_VAR\n");
    }

    #[test]
    fn declaration_alignment_resets_across_a_blank_line() {
        let mut config = FormatConfig::default();
        config.align_consecutive_declarations = true;
        let out = format("VAR\nx:INT;\n\nlongName:BOOL;\nEND_VAR\n", &config);
        assert_eq!(out, "VAR\n    x: INT;\n\n    longName: BOOL;\nEND_VAR\n");
    }

    #[test]
    fn aligns_consecutive_body_assignments_but_not_declarations() {
        let mut config = FormatConfig::default();
        config.align_consecutive_assignments = true;
        let out = format("VAR\nx:INT;\nlongName:INT;\nEND_VAR\nx:=1;\nlongName:=2;\n", &config);
        assert_eq!(out, "VAR\n    x: INT;\n    longName: INT;\nEND_VAR\nx        := 1;\nlongName := 2;\n");
    }

    #[test]
    fn aligns_trailing_comments_across_declarations_and_code_alike() {
        let mut config = FormatConfig::default();
        config.align_trailing_comments = true;
        let out = format("VAR\nx:INT; // short\nEND_VAR\nlongStmt:=1; // long one\n", &config);
        // Trailing-comment alignment only resets at a blank line, so it spans the `END_VAR`
        // boundary here: `x: INT;`'s comment is pushed out to the same absolute column (14) as
        // `longStmt := 1;`'s, even though one line is a declaration and the other is code.
        assert_eq!(out, "VAR\n    x: INT;     // short\nEND_VAR\nlongStmt := 1;  // long one\n");
    }

    #[test]
    fn lone_declaration_in_a_run_is_not_padded() {
        let mut config = FormatConfig::default();
        config.align_consecutive_declarations = true;
        let out = format("VAR\nx:INT;\nEND_VAR\n", &config);
        assert_eq!(out, "VAR\n    x: INT;\nEND_VAR\n");
    }

    #[test]
    fn alignment_is_idempotent() {
        let mut config = FormatConfig::default();
        config.align_consecutive_declarations = true;
        config.align_consecutive_assignments = true;
        config.align_trailing_comments = true;
        let source = "VAR\nx:INT; // a\nlongName:BOOL:=TRUE; // b\nEND_VAR\nx:=1;\nlongName:=2; // c\n";
        let once = format(source, &config);
        let twice = format(&once, &config);
        assert_eq!(once, twice);
    }
}
