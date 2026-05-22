//! ST formatter prototype.
//!
//! Two-pass formatter that operates on the `lex_with_trivia` token
//! stream. First pass identifies VAR / STRUCT blocks and computes the
//! tabular alignment widths for their declaration lines. Second pass
//! re-emits the document with normalised whitespace:
//!
//! - 4-space indent per nested block scope (FUNCTION, VAR, IF/FOR/WHILE,
//!   STRUCT, ...).
//! - Keywords UPPER-cased and emitted in their canonical form
//!   (`FUNCTIONBLOCK` → `FUNCTION_BLOCK`).
//! - Spaces around `:`, `:=`, `=>`; space after `,`; no space before `;`.
//! - Inside a VAR/STRUCT block, names padded so `:` lines up across
//!   declarations; type-expressions padded so `:=` lines up across the
//!   lines that have an initializer.
//! - Comments preserved as written; blank lines preserved but capped at
//!   one consecutive blank line.
//! - Trailing whitespace stripped from every line.
//!
//! Indent rules are deliberately simple — a stack of opener tokens
//! drives the depth. ELSE / ELSIF / case labels emit at the same depth
//! as their body for the prototype; sharper outdenting is a follow-up.

use std::ops::Range;

use plc::lexer::{lex_with_trivia, LspToken, Token, TriviaKind};

const INDENT: &str = "    ";

/// Format the document. Always succeeds; on a lex-only input even a
/// partial / parse-broken file will round-trip.
pub fn format_document(source: &str) -> String {
    let tokens = lex_with_trivia(source);
    let alignments = compute_block_alignments(&tokens, source);
    let mut state = Emitter::new(source, &alignments);
    state.emit_all(&tokens);
    state.finish(source)
}

// =====================================================================
// Pass 1 — block alignment widths
// =====================================================================

/// Per-block measurements used by the emitter to pad declarations into
/// columns. Keyed by the token index of the opening keyword
/// (VAR / VAR_INPUT / VAR_OUTPUT / VAR_IN_OUT / VAR_TEMP / VAR_GLOBAL /
/// VAR_CONFIG / VAR_EXTERNAL / STRUCT).
#[derive(Default, Debug, Clone)]
struct BlockAlignment {
    /// Max name+attribute width across all declarations in the block.
    /// Used to pad the column where `:` lands.
    name_width: usize,
    /// Max width of "name + padding + `:` + space + type-expression"
    /// across declarations that have an initializer. Used to pad the
    /// column where `:=` lands.
    type_col_end: usize,
}

fn compute_block_alignments(tokens: &[LspToken], source: &str) -> Vec<Option<BlockAlignment>> {
    let mut out: Vec<Option<BlockAlignment>> = vec![None; tokens.len()];
    let mut i = 0;
    while i < tokens.len() {
        if let LspToken::Token(t, _) = &tokens[i] {
            if is_decl_block_opener(t) {
                let (alignment, end_idx) = scan_block(tokens, source, i);
                out[i] = Some(alignment);
                i = end_idx;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn is_decl_block_opener(t: &Token) -> bool {
    matches!(
        t,
        Token::KeywordVar
            | Token::KeywordVarInput
            | Token::KeywordVarOutput
            | Token::KeywordVarInOut
            | Token::KeywordVarTemp
            | Token::KeywordVarGlobal
            | Token::KeywordVarConfig
            | Token::KeywordVarExternal
            | Token::KeywordStruct
    )
}

fn is_decl_block_closer(t: &Token) -> bool {
    matches!(t, Token::KeywordEndVar | Token::KeywordEndStruct)
}

/// Walk the tokens of one declaration block starting at `opener_idx`
/// (which points to a VAR* or STRUCT keyword). Collects per-line
/// measurements and returns the aggregate plus the index *past* the
/// matching END_VAR / END_STRUCT. Lines that don't look like
/// `name … : type [:= init];` are ignored (won't be tabularised but
/// also don't throw the alignment off).
fn scan_block(tokens: &[LspToken], source: &str, opener_idx: usize) -> (BlockAlignment, usize) {
    let mut align = BlockAlignment::default();
    let mut i = opener_idx + 1;
    while i < tokens.len() {
        match &tokens[i] {
            LspToken::Token(t, _) if is_decl_block_closer(t) => {
                return (align, i + 1);
            }
            LspToken::Token(t, _) if is_decl_block_opener(t) => {
                // Nested block (STRUCT inside VAR is rare but possible).
                // Skip past it without measuring nested lines against the
                // outer block.
                let (_, after) = scan_block(tokens, source, i);
                i = after;
                continue;
            }
            _ => {}
        }
        // Try to recognise a declaration line starting at i.
        if let Some(decl) = scan_decl_line(tokens, source, i) {
            align.name_width = align.name_width.max(decl.name_width);
            if decl.has_initializer {
                let projected = decl.name_width.max(align.name_width) + 3 + decl.type_width;
                align.type_col_end = align.type_col_end.max(projected);
            }
            i = decl.next_idx;
        } else {
            i += 1;
        }
    }
    (align, i)
}

#[derive(Debug)]
struct DeclMeasurement {
    name_width: usize,
    type_width: usize,
    has_initializer: bool,
    next_idx: usize,
}

/// Measure a single declaration line if `start` is the head of one.
/// Recognises:
///
/// - `NAME [, NAME ...] : TYPE_EXPR ;`
/// - `NAME [, NAME ...] : TYPE_EXPR := INITIALIZER ;`
///
/// Returns the rendered widths assuming the formatter's spacing rules,
/// or `None` if the lookahead doesn't match. Comments and pragmas in
/// the middle of the line are tolerated — they're trivia and don't
/// affect the columns.
fn scan_decl_line(tokens: &[LspToken], source: &str, start: usize) -> Option<DeclMeasurement> {
    // The name segment is one or more comma-separated identifiers
    // (e.g. `a, b, c : INT;`). We measure the rendered width of the
    // whole comma list — that's the column the `:` will sit in.
    let mut idx = start;
    let mut name_buf = String::new();
    let mut expecting_ident = true;
    loop {
        idx = skip_trivia(tokens, idx);
        if idx >= tokens.len() {
            return None;
        }
        match &tokens[idx] {
            LspToken::Token(Token::Identifier, range) if expecting_ident => {
                name_buf.push_str(&source[range.clone()]);
                expecting_ident = false;
                idx += 1;
            }
            LspToken::Token(Token::KeywordComma, _) if !expecting_ident => {
                name_buf.push_str(", ");
                expecting_ident = true;
                idx += 1;
            }
            LspToken::Token(Token::KeywordColon, _) if !expecting_ident => {
                idx += 1;
                break;
            }
            _ => return None,
        }
    }

    // Type expression: everything from `:` to `:=` or `;`, rendered as
    // a single line. Treat it as opaque text — we only need its visual
    // width. Includes balanced brackets for `ARRAY[..] OF ...`.
    let type_start = idx;
    let type_end;
    let mut has_initializer = false;
    let mut bracket_depth: i32 = 0;
    loop {
        idx = skip_trivia(tokens, idx);
        if idx >= tokens.len() {
            return None;
        }
        match &tokens[idx] {
            LspToken::Token(Token::KeywordAssignment, _) if bracket_depth == 0 => {
                has_initializer = true;
                type_end = idx;
                break;
            }
            LspToken::Token(Token::KeywordSemicolon, _) if bracket_depth == 0 => {
                type_end = idx;
                break;
            }
            LspToken::Token(Token::KeywordSquareParensOpen, _) => {
                bracket_depth += 1;
                idx += 1;
            }
            LspToken::Token(Token::KeywordSquareParensClose, _) => {
                bracket_depth -= 1;
                idx += 1;
            }
            LspToken::Token(_, _) => idx += 1,
            LspToken::Trivia(..) => unreachable!("skip_trivia"),
        }
    }

    let type_width = rendered_width(tokens, source, type_start..type_end);

    // Skip to end of line (semicolon).
    while idx < tokens.len() {
        match &tokens[idx] {
            LspToken::Token(Token::KeywordSemicolon, _) => {
                idx += 1;
                break;
            }
            _ => idx += 1,
        }
    }

    Some(DeclMeasurement { name_width: name_buf.chars().count(), type_width, has_initializer, next_idx: idx })
}

fn skip_trivia(tokens: &[LspToken], mut idx: usize) -> usize {
    while idx < tokens.len() && matches!(tokens[idx], LspToken::Trivia(..)) {
        idx += 1;
    }
    idx
}

/// Compute the visual width of a token range as it would be re-emitted
/// (canonical keyword form + minimal spacing). Used by the alignment
/// pass to size the `:` and `:=` columns.
fn rendered_width(tokens: &[LspToken], source: &str, range: Range<usize>) -> usize {
    let mut sim = SpacingSim::new();
    for tok in &tokens[range] {
        if let LspToken::Token(t, r) = tok {
            sim.push(t, &source[r.clone()]);
        }
    }
    sim.width
}

/// Tiny re-implementation of the emitter's spacing logic that only
/// tracks the running width (no buffer). Keeps the alignment pass in
/// sync with the actual emit pass without committing to a String.
struct SpacingSim {
    prev: Option<Token>,
    width: usize,
}

impl SpacingSim {
    fn new() -> Self {
        Self { prev: None, width: 0 }
    }

    fn push(&mut self, t: &Token, text: &str) {
        if let Some(prev) = &self.prev {
            if needs_space_between(prev, t) {
                self.width += 1;
            }
        }
        self.width += canonical_text(t, text).chars().count();
        self.prev = Some(*t);
    }
}

// =====================================================================
// Pass 2 — emit
// =====================================================================

struct Emitter<'a> {
    source: &'a str,
    alignments: &'a [Option<BlockAlignment>],
    out: String,
    indent_depth: usize,
    /// Currently inside one or more nested VAR/STRUCT blocks; the top
    /// of the stack is the active alignment (None for blocks we
    /// couldn't measure).
    align_stack: Vec<Option<BlockAlignment>>,
    /// True after the last byte emitted was a newline — the next real
    /// token should be preceded by indent.
    at_line_start: bool,
    /// Token kind most recently emitted, used for spacing decisions.
    prev_kind: Option<Token>,
    /// Number of blank lines since the last real token. Capped at 1 at
    /// emit time.
    blank_lines: usize,
    /// Count of `\n` characters seen in trivia since the last real
    /// token; distinguishes "still on same line" from "newline pending".
    newlines_pending: usize,
}

impl<'a> Emitter<'a> {
    fn new(source: &'a str, alignments: &'a [Option<BlockAlignment>]) -> Self {
        Self {
            source,
            alignments,
            out: String::new(),
            indent_depth: 0,
            align_stack: Vec::new(),
            at_line_start: true,
            prev_kind: None,
            blank_lines: 0,
            newlines_pending: 0,
        }
    }

    fn emit_all(&mut self, tokens: &[LspToken]) {
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                LspToken::Trivia(kind, r) => {
                    self.absorb_trivia(*kind, r.clone());
                    i += 1;
                }
                LspToken::Token(t, r) => {
                    // Handle indent changes triggered by closers BEFORE
                    // emitting the closer keyword, so e.g. `END_VAR` lands
                    // at the parent indent rather than the body's.
                    if is_dedent_token(t) && self.indent_depth > 0 {
                        self.indent_depth -= 1;
                    }
                    if is_decl_block_closer(t) {
                        self.align_stack.pop();
                    }

                    self.flush_pending_newlines();
                    let starting_new_line = self.at_line_start;
                    if self.at_line_start {
                        self.write_indent();
                        self.at_line_start = false;
                    }

                    // Tabular alignment: when inside a VAR / STRUCT block
                    // and we're about to emit `:` or `:=`, pad to the
                    // appropriate column so the symbol lines up with the
                    // longest declaration in the block. Otherwise apply
                    // the default needs_space_between rule — but never
                    // right after the indent, since the indent already
                    // separates this token from whatever came before.
                    let mut emitted_aligned = false;
                    if let Some(Some(align)) = self.align_stack.last() {
                        let indent_width = self.indent_depth * INDENT.len();
                        let current_col = self.current_column_in_line();
                        match t {
                            Token::KeywordColon => {
                                let target = indent_width + align.name_width;
                                let pad = target.saturating_sub(current_col);
                                for _ in 0..pad {
                                    self.out.push(' ');
                                }
                                self.out.push(' ');
                                emitted_aligned = true;
                            }
                            Token::KeywordAssignment if align.type_col_end > 0 => {
                                let target = indent_width + align.type_col_end;
                                let pad = target.saturating_sub(current_col);
                                for _ in 0..pad {
                                    self.out.push(' ');
                                }
                                self.out.push(' ');
                                emitted_aligned = true;
                            }
                            _ => {}
                        }
                    }
                    if !emitted_aligned && !starting_new_line {
                        if let Some(prev) = self.prev_kind.as_ref() {
                            if needs_space_between(prev, t) {
                                self.out.push(' ');
                            }
                        }
                    }

                    let text = canonical_text(t, &self.source[r.clone()]);
                    self.out.push_str(&text);
                    self.prev_kind = Some(*t);

                    // Push indent / alignment for openers AFTER
                    // emitting them — body of the block starts on the
                    // next line at depth+1.
                    if is_decl_block_opener(t) {
                        self.align_stack.push(self.alignments[i].as_ref().cloned());
                    }
                    if is_indent_token(t) {
                        self.indent_depth += 1;
                    }
                    i += 1;
                }
            }
        }
    }

    /// Trivia drives newline / blank-line bookkeeping. Comments are
    /// emitted verbatim with their original content, at the current
    /// indent level if they sit at line start. Pragmas (raw `{…}` blob)
    /// pass through untouched.
    fn absorb_trivia(&mut self, kind: TriviaKind, range: Range<usize>) {
        let text = &self.source[range];
        match kind {
            TriviaKind::Whitespace => {
                let newlines = text.bytes().filter(|&b| b == b'\n').count();
                if newlines > 0 {
                    self.newlines_pending = self.newlines_pending.saturating_add(newlines);
                    self.at_line_start = true;
                }
            }
            TriviaKind::LineComment | TriviaKind::BlockComment | TriviaKind::Pragma => {
                self.flush_pending_newlines();
                if self.at_line_start {
                    self.write_indent();
                    self.at_line_start = false;
                } else {
                    // Inline trivia: leave one space between the
                    // previous emit and the comment / pragma.
                    self.out.push(' ');
                }
                self.out.push_str(text);
                // Block comments / pragmas may contain newlines —
                // detect and flag.
                if text.contains('\n') {
                    self.at_line_start = true;
                }
            }
        }
    }

    fn flush_pending_newlines(&mut self) {
        if self.newlines_pending == 0 {
            return;
        }
        // Cap at 1 blank line (i.e. up to two consecutive newlines).
        let to_emit = self.newlines_pending.min(2);
        // Strip trailing whitespace from the line we're closing.
        while self.out.ends_with(' ') || self.out.ends_with('\t') {
            self.out.pop();
        }
        for _ in 0..to_emit {
            self.out.push('\n');
        }
        self.newlines_pending = 0;
        if to_emit > 1 {
            self.blank_lines = 1;
        } else {
            self.blank_lines = 0;
        }
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent_depth {
            self.out.push_str(INDENT);
        }
    }

    /// Column (visual char count) since the last `\n` in `self.out`.
    /// Used by the tabular padding so we know how many spaces to add
    /// before `:` / `:=`.
    fn current_column_in_line(&self) -> usize {
        match self.out.rfind('\n') {
            Some(nl) => self.out[nl + 1..].chars().count(),
            None => self.out.chars().count(),
        }
    }

    fn finish(mut self, source: &str) -> String {
        self.flush_pending_newlines();
        // Strip trailing whitespace on the final line.
        while self.out.ends_with(' ') || self.out.ends_with('\t') {
            self.out.pop();
        }
        // Preserve the original file's trailing-newline policy.
        let source_ends_nl = source.ends_with('\n');
        if source_ends_nl && !self.out.ends_with('\n') {
            self.out.push('\n');
        } else if !source_ends_nl {
            while self.out.ends_with('\n') {
                self.out.pop();
            }
        }
        self.out
    }
}

// =====================================================================
// Token classification + spacing rules
// =====================================================================

/// Tokens whose appearance INCREASES the indent depth for following
/// lines. Indent is bumped after emitting the token.
fn is_indent_token(t: &Token) -> bool {
    matches!(
        t,
        Token::KeywordVar
            | Token::KeywordVarInput
            | Token::KeywordVarOutput
            | Token::KeywordVarInOut
            | Token::KeywordVarTemp
            | Token::KeywordVarGlobal
            | Token::KeywordVarConfig
            | Token::KeywordVarExternal
            | Token::KeywordStruct
            | Token::KeywordFunction
            | Token::KeywordFunctionBlock
            | Token::KeywordProgram
            | Token::KeywordClass
            | Token::KeywordInterface
            | Token::KeywordMethod
            | Token::KeywordAction
            | Token::KeywordActions
            | Token::KeywordPropertyGet
            | Token::KeywordPropertySet
            | Token::KeywordType
            | Token::KeywordIf
            | Token::KeywordFor
            | Token::KeywordWhile
            | Token::KeywordRepeat
            | Token::KeywordCase
    )
}

/// Tokens that DECREASE the indent depth. Dedent fires BEFORE
/// emitting the token so the keyword lands at the parent's indent.
fn is_dedent_token(t: &Token) -> bool {
    matches!(
        t,
        Token::KeywordEndVar
            | Token::KeywordEndStruct
            | Token::KeywordEndFunction
            | Token::KeywordEndFunctionBlock
            | Token::KeywordEndProgram
            | Token::KeywordEndClass
            | Token::KeywordEndInterface
            | Token::KeywordEndMethod
            | Token::KeywordEndAction
            | Token::KeywordEndActions
            | Token::KeywordEndProperty
            | Token::KeywordEndType
            | Token::KeywordEndIf
            | Token::KeywordEndFor
            | Token::KeywordEndWhile
            | Token::KeywordEndRepeat
            | Token::KeywordEndCase
            | Token::KeywordUntil
    )
}

/// Canonical re-emit text for a token. Keywords go UPPER and use the
/// underscored form (`FUNCTION_BLOCK` over `FUNCTIONBLOCK`); operators
/// emit their punctuation symbol literally; identifiers / literals
/// pass through unchanged from the source.
fn canonical_text(t: &Token, source_slice: &str) -> String {
    match t {
        Token::Identifier => source_slice.to_string(),

        Token::KeywordProgram => "PROGRAM".into(),
        Token::KeywordEndProgram => "END_PROGRAM".into(),
        Token::KeywordFunction => "FUNCTION".into(),
        Token::KeywordEndFunction => "END_FUNCTION".into(),
        Token::KeywordFunctionBlock => "FUNCTION_BLOCK".into(),
        Token::KeywordEndFunctionBlock => "END_FUNCTION_BLOCK".into(),
        Token::KeywordClass => "CLASS".into(),
        Token::KeywordEndClass => "END_CLASS".into(),
        Token::KeywordInterface => "INTERFACE".into(),
        Token::KeywordEndInterface => "END_INTERFACE".into(),
        Token::KeywordMethod => "METHOD".into(),
        Token::KeywordEndMethod => "END_METHOD".into(),
        Token::KeywordAction => "ACTION".into(),
        Token::KeywordEndAction => "END_ACTION".into(),
        Token::KeywordActions => "ACTIONS".into(),
        Token::KeywordEndActions => "END_ACTIONS".into(),
        Token::KeywordPropertyGet => "PROPERTY_GET".into(),
        Token::KeywordPropertySet => "PROPERTY_SET".into(),
        Token::KeywordEndProperty => "END_PROPERTY".into(),

        Token::KeywordVar => "VAR".into(),
        Token::KeywordVarInput => "VAR_INPUT".into(),
        Token::KeywordVarOutput => "VAR_OUTPUT".into(),
        Token::KeywordVarInOut => "VAR_IN_OUT".into(),
        Token::KeywordVarTemp => "VAR_TEMP".into(),
        Token::KeywordVarGlobal => "VAR_GLOBAL".into(),
        Token::KeywordVarConfig => "VAR_CONFIG".into(),
        Token::KeywordVarExternal => "VAR_EXTERNAL".into(),
        Token::KeywordEndVar => "END_VAR".into(),

        Token::KeywordType => "TYPE".into(),
        Token::KeywordEndType => "END_TYPE".into(),
        Token::KeywordStruct => "STRUCT".into(),
        Token::KeywordEndStruct => "END_STRUCT".into(),

        Token::KeywordIf => "IF".into(),
        Token::KeywordThen => "THEN".into(),
        Token::KeywordElse => "ELSE".into(),
        Token::KeywordElseIf => "ELSIF".into(),
        Token::KeywordEndIf => "END_IF".into(),
        Token::KeywordFor => "FOR".into(),
        Token::KeywordTo => "TO".into(),
        Token::KeywordBy => "BY".into(),
        Token::KeywordDo => "DO".into(),
        Token::KeywordEndFor => "END_FOR".into(),
        Token::KeywordWhile => "WHILE".into(),
        Token::KeywordEndWhile => "END_WHILE".into(),
        Token::KeywordRepeat => "REPEAT".into(),
        Token::KeywordUntil => "UNTIL".into(),
        Token::KeywordEndRepeat => "END_REPEAT".into(),
        Token::KeywordCase => "CASE".into(),
        Token::KeywordOf => "OF".into(),
        Token::KeywordEndCase => "END_CASE".into(),
        Token::KeywordReturn => "RETURN".into(),
        Token::KeywordExit => "EXIT".into(),
        Token::KeywordContinue => "CONTINUE".into(),
        Token::KeywordExtends => "EXTENDS".into(),
        Token::KeywordImplements => "IMPLEMENTS".into(),
        Token::KeywordConstant => "CONSTANT".into(),
        Token::KeywordRetain => "RETAIN".into(),
        Token::KeywordNonRetain => "NON_RETAIN".into(),
        Token::KeywordAbstract => "ABSTRACT".into(),
        Token::KeywordFinal => "FINAL".into(),
        Token::KeywordSuper => "SUPER".into(),
        Token::KeywordThis => "THIS".into(),
        Token::KeywordOverride => "OVERRIDE".into(),
        Token::KeywordAccessPublic => "PUBLIC".into(),
        Token::KeywordAccessPrivate => "PRIVATE".into(),
        Token::KeywordAccessInternal => "INTERNAL".into(),
        Token::KeywordAccessProtected => "PROTECTED".into(),
        Token::KeywordRef => "REF_TO".into(),
        Token::KeywordReferenceTo => "REFERENCE TO".into(),
        Token::KeywordArray => "ARRAY".into(),
        Token::KeywordString => "STRING".into(),
        Token::KeywordWideString => "WSTRING".into(),
        Token::KeywordPointer => "POINTER".into(),
        Token::KeywordAt => "AT".into(),

        // Punctuation / operators emit their literal symbol.
        _ => source_slice.to_string(),
    }
}

/// Whether a single space should appear between two adjacent real
/// tokens. Errs on the side of inserting a space for keyword-keyword
/// or keyword-identifier adjacencies; punctuation rules are explicit.
fn needs_space_between(prev: &Token, next: &Token) -> bool {
    // No space inside `arr[idx]`, `foo()`, `^`, around dots.
    if matches!(
        next,
        Token::KeywordSquareParensClose
            | Token::KeywordParensClose
            | Token::KeywordSemicolon
            | Token::KeywordComma
            | Token::KeywordDot
            | Token::KeywordDotDot
            | Token::KeywordDotDotDot
            | Token::OperatorDeref
    ) {
        return false;
    }
    if matches!(
        prev,
        Token::KeywordSquareParensOpen | Token::KeywordParensOpen | Token::KeywordDot | Token::OperatorDeref
    ) {
        return false;
    }
    // `(` / `[` after an identifier is a call / index — no space.
    if matches!(prev, Token::Identifier)
        && matches!(next, Token::KeywordParensOpen | Token::KeywordSquareParensOpen)
    {
        return false;
    }
    // Default: insert one space between adjacent tokens.
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercase_keyword_normalisation() {
        // Lexer keywords are uppercased to canonical form. Type names
        // like `INT` are Identifier tokens, not keywords; the formatter
        // preserves user-written case for identifiers (a separate
        // "uppercase elementary types" feature would be additive).
        let src = "function foo : INT\nend_function\n";
        let out = format_document(src);
        assert!(out.contains("FUNCTION foo : INT"), "got: {out:?}");
        assert!(out.contains("END_FUNCTION"), "got: {out:?}");
    }

    #[test]
    fn canonical_function_block_form() {
        let src = "FUNCTIONBLOCK FB\nENDFUNCTIONBLOCK\n";
        let out = format_document(src);
        assert!(out.contains("FUNCTION_BLOCK FB"), "got: {out:?}");
        assert!(out.contains("END_FUNCTION_BLOCK"), "got: {out:?}");
    }

    #[test]
    fn tabular_var_block_aligns_colons() {
        let src = "VAR\n\
            x : INT;\n\
            counter : DINT;\n\
            name : STRING;\n\
            END_VAR\n";
        let out = format_document(src);
        // All three colons should land at the same column.
        let cols: Vec<usize> = out
            .lines()
            .filter(|l| l.contains(':') && !l.contains("VAR"))
            .map(|l| l.find(':').unwrap())
            .collect();
        assert!(!cols.is_empty(), "no decl lines in output:\n{out}");
        assert!(cols.iter().all(|&c| c == cols[0]), "colons not aligned at same column: {cols:?}\n{out}");
    }

    #[test]
    fn tabular_var_block_aligns_assignment() {
        let src = "VAR\nx : INT := 0;\ncount : DINT := 1;\nflag : BOOL := TRUE;\nEND_VAR\n";
        let out = format_document(src);
        let lines: Vec<&str> = out.lines().filter(|l| l.contains(":=")).collect();
        let cols: Vec<usize> = lines.iter().map(|l| l.find(":=").unwrap()).collect();
        assert!(cols.iter().all(|&c| c == cols[0]), "`:=` not aligned: {cols:?}\n{out}");
    }

    #[test]
    fn indent_for_function_body() {
        let src = "FUNCTION foo : INT\nVAR\nx : INT;\nEND_VAR\nfoo := x;\nEND_FUNCTION\n";
        let out = format_document(src);
        // `VAR` should be indented one level under FUNCTION, `x : INT;`
        // one more under VAR.
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines[0], "FUNCTION foo : INT");
        assert_eq!(lines[1], "    VAR");
        assert!(lines[2].starts_with("        x"), "decl indent wrong: {:?}", lines[2]);
        assert_eq!(lines[3], "    END_VAR");
        assert!(lines[4].starts_with("    foo"), "body indent wrong: {:?}", lines[4]);
        assert_eq!(lines[5], "END_FUNCTION");
    }

    #[test]
    fn comments_preserved_at_line_start() {
        let src = "(* doc *)\nFUNCTION foo : INT\nfoo := 1;\nEND_FUNCTION\n";
        let out = format_document(src);
        assert!(out.contains("(* doc *)"), "doc comment lost: {out}");
        assert!(
            out.find("(* doc *)").unwrap() < out.find("FUNCTION").unwrap(),
            "comment order wrong:\n{out}"
        );
    }

    #[test]
    fn blank_lines_capped_at_one() {
        let src = "VAR\nx : INT;\n\n\n\ny : INT;\nEND_VAR\n";
        let out = format_document(src);
        // Should not contain three consecutive newlines (i.e. > 1 blank line).
        assert!(!out.contains("\n\n\n"), "blank lines not capped:\n{out:?}");
        // Should still contain the gap (one blank line).
        assert!(out.contains("\n\n"), "blank line collapsed entirely:\n{out:?}");
    }

    #[test]
    fn trailing_whitespace_stripped() {
        let src = "FUNCTION foo : INT   \n   x := 1;   \nEND_FUNCTION   \n";
        let out = format_document(src);
        for line in out.lines() {
            assert!(!line.ends_with(' '), "line ends with space: {line:?}\n{out}");
        }
    }
}
