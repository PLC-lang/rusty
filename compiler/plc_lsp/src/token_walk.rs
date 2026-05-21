//! Cursor-aware walks over the `Vec<LspToken>` produced by
//! `plc::lexer::lex_with_trivia`.
//!
//! All operations are stateless on a `&[LspToken]` slice: callers pass a
//! token index in and get another index out. There is no internal mutable
//! cursor — this avoids subtle reuse bugs where one handler's walk leaks
//! into another's.
//!
//! Position lookup is `O(log n)` via `partition_point`; the token vector
//! is laid out in source order with contiguous spans (the lexer guarantees
//! no gaps), so binary search on `range().end` finds the token whose span
//! contains a byte offset.

use std::ops::Range;

use plc::lexer::{LspToken, Token, TriviaKind};

/// Look up the doc body adjacent to a declaration at `decl_offset` in
/// `source`. Returns the concatenation of every contiguous leading
/// comment (block or line), with markers stripped and individual
/// comments separated by a blank line for markdown paragraph breaks.
///
/// `is_decl_prefix` lets the caller widen the attachment anchor past
/// keywords that syntactically precede the name (e.g. `FUNCTION` before
/// a POU's identifier) — a doc comment sits above the `FUNCTION` line,
/// not above the identifier itself. Return `true` for keywords that are
/// part of the declaration's syntactic prefix; the walker skips them
/// and then scans for leading comments.
///
/// Returns `None` when no leading comments attach to the declaration.
pub fn docstring_at(
    tokens: &[LspToken],
    source: &str,
    decl_offset: usize,
    is_decl_prefix: impl Fn(&Token) -> bool,
) -> Option<String> {
    let walk = TokenWalk::new(tokens);
    let mut anchor = walk.token_at(decl_offset)?;
    // Widen the anchor past declaration-prefix keywords so a comment
    // above `FUNCTION foo` attaches to foo (not to FUNCTION).
    while let Some(prev) = walk.prev_real(anchor) {
        match walk.token_kind(prev) {
            Some(t) if is_decl_prefix(t) => anchor = prev,
            _ => break,
        }
    }
    let leading = walk.leading_comments(anchor);
    if leading.is_empty() {
        return None;
    }
    let mut body = String::new();
    for &idx in &leading {
        let range = walk.range_at(idx)?;
        let stripped = strip_comment_markers(&source[range.clone()]);
        if stripped.is_empty() {
            continue;
        }
        if !body.is_empty() {
            body.push_str("\n\n");
        }
        body.push_str(&stripped);
    }
    if body.is_empty() {
        None
    } else {
        Some(body)
    }
}

/// Trim the comment delimiters off a single comment span. Recognises
/// `// …`, `(* … *)`, and `/* … */`. The interior text is trimmed of
/// leading/trailing whitespace; embedded newlines are preserved.
fn strip_comment_markers(raw: &str) -> String {
    let s = raw.trim();
    if let Some(rest) = s.strip_prefix("//") {
        return rest.trim().to_string();
    }
    if let Some(rest) = s.strip_prefix("(*") {
        if let Some(inner) = rest.strip_suffix("*)") {
            return inner.trim().to_string();
        }
        return rest.trim().to_string();
    }
    if let Some(rest) = s.strip_prefix("/*") {
        if let Some(inner) = rest.strip_suffix("*/") {
            return inner.trim().to_string();
        }
        return rest.trim().to_string();
    }
    s.to_string()
}

/// Stateless walker over a slice of `LspToken`. Cheap to construct.
pub struct TokenWalk<'a> {
    tokens: &'a [LspToken],
}

impl<'a> TokenWalk<'a> {
    pub fn new(tokens: &'a [LspToken]) -> Self {
        Self { tokens }
    }

    pub fn tokens(&self) -> &'a [LspToken] {
        self.tokens
    }

    /// Index of the token whose span contains `byte_offset`.
    ///
    /// Boundary policy: when `byte_offset == span.end` of token A and
    /// `byte_offset == span.start` of token B (i.e. the cursor sits
    /// exactly between two tokens), the token starting AT that offset
    /// wins. This matches how editors render the cursor: it appears
    /// "before" the next character.
    ///
    /// Returns `None` if `byte_offset` is past the last token's end.
    pub fn token_at(&self, byte_offset: usize) -> Option<usize> {
        if self.tokens.is_empty() {
            return None;
        }
        let idx = self.tokens.partition_point(|t| t.range().end <= byte_offset);
        if idx >= self.tokens.len() {
            return None;
        }
        let r = self.tokens[idx].range();
        if byte_offset >= r.start {
            Some(idx)
        } else {
            None
        }
    }

    /// Index of the last real `Token` whose span ends at or before
    /// `byte_offset`. Skips trivia. Use this when the cursor sits at
    /// end-of-source (no token covers it) and you need the previous real
    /// token — e.g. `foo.|` where the cursor is past the dot.
    pub fn prev_real_before(&self, byte_offset: usize) -> Option<usize> {
        let mut i = self.tokens.partition_point(|t| t.range().end <= byte_offset);
        while i > 0 {
            i -= 1;
            if matches!(self.tokens[i], LspToken::Token(..)) {
                return Some(i);
            }
        }
        None
    }

    /// Walk LEFT from `index` (exclusive), returning the first real-token
    /// index found. Trivia is skipped.
    pub fn prev_real(&self, index: usize) -> Option<usize> {
        let mut i = index;
        while i > 0 {
            i -= 1;
            if matches!(self.tokens[i], LspToken::Token(..)) {
                return Some(i);
            }
        }
        None
    }

    /// Walk RIGHT from `index` (exclusive), returning the first real-token
    /// index found. Trivia is skipped.
    pub fn next_real(&self, index: usize) -> Option<usize> {
        let mut i = index + 1;
        while i < self.tokens.len() {
            if matches!(self.tokens[i], LspToken::Token(..)) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// Extract the `Token` enum at `index`, or `None` if the index points
    /// at trivia or is out of bounds.
    pub fn token_kind(&self, index: usize) -> Option<&Token> {
        match self.tokens.get(index)? {
            LspToken::Token(t, _) => Some(t),
            LspToken::Trivia(..) => None,
        }
    }

    /// Range of the token at `index`, or `None` if out of bounds.
    pub fn range_at(&self, index: usize) -> Option<&Range<usize>> {
        self.tokens.get(index).map(|t| t.range())
    }

    /// Comment trivia indices immediately preceding `index`, in source order.
    ///
    /// Strict-contiguous semantics: scanning LEFT from `index - 1`, accept
    /// `Whitespace` (blank lines included) and comment trivia; stop on the
    /// first real `Token` or `Pragma`. The returned indices point to
    /// `LineComment` / `BlockComment` items, in left-to-right source order.
    pub fn leading_comments(&self, index: usize) -> Vec<usize> {
        let mut comments = Vec::new();
        if index == 0 {
            return comments;
        }
        let mut i = index;
        while i > 0 {
            i -= 1;
            match &self.tokens[i] {
                LspToken::Trivia(TriviaKind::LineComment | TriviaKind::BlockComment, _) => {
                    comments.push(i);
                }
                LspToken::Trivia(TriviaKind::Whitespace, _) => {
                    // pass-through; blank lines are still "contiguous"
                }
                _ => break,
            }
        }
        comments.reverse();
        comments
    }

    /// Comment trivia indices immediately following `index`, in source order.
    ///
    /// Mirror of `leading_comments`: scan RIGHT from `index + 1`, allow
    /// whitespace + comments, stop at the first real token or pragma.
    pub fn trailing_comments(&self, index: usize) -> Vec<usize> {
        let mut comments = Vec::new();
        let mut i = index + 1;
        while i < self.tokens.len() {
            match &self.tokens[i] {
                LspToken::Trivia(TriviaKind::LineComment | TriviaKind::BlockComment, _) => {
                    comments.push(i);
                }
                LspToken::Trivia(TriviaKind::Whitespace, _) => {}
                _ => break,
            }
            i += 1;
        }
        comments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plc::lexer::lex_with_trivia;

    fn walk(src: &str) -> (Vec<LspToken>, ()) {
        (lex_with_trivia(src), ())
    }

    #[test]
    fn empty_slice_returns_none() {
        let walk = TokenWalk::new(&[]);
        assert_eq!(walk.token_at(0), None);
        assert_eq!(walk.prev_real_before(0), None);
        assert_eq!(walk.prev_real(0), None);
        assert_eq!(walk.next_real(0), None);
    }

    #[test]
    fn token_at_inside_span() {
        // "VAR x" — KeywordVar 0..3, ws 3..4, Identifier 4..5
        let (tokens, _) = walk("VAR x");
        let w = TokenWalk::new(&tokens);
        assert_eq!(w.token_at(0), Some(0)); // 'V'
        assert_eq!(w.token_at(2), Some(0)); // 'R'
        assert_eq!(w.token_at(3), Some(1)); // boundary: ws starts here
        assert_eq!(w.token_at(4), Some(2)); // 'x'
        assert_eq!(w.token_at(5), None); // past end
    }

    #[test]
    fn prev_real_before_cursor_at_end_of_source() {
        // "foo." — Identifier 0..3, OperatorDot 3..4. Cursor at 4.
        let (tokens, _) = walk("foo.");
        let w = TokenWalk::new(&tokens);
        assert_eq!(w.token_at(4), None);
        let idx = w.prev_real_before(4).expect("dot should be returned");
        assert_eq!(w.range_at(idx).unwrap(), &(3..4));
    }

    #[test]
    fn prev_real_skips_whitespace_and_comments() {
        // "foo (* doc *)  bar" — Identifier, ws, BlockComment, ws, Identifier
        let (tokens, _) = walk("foo (* doc *)  bar");
        let w = TokenWalk::new(&tokens);
        // Find index of 'bar'
        let bar = w.token_at(15).expect("bar token");
        let prev = w.prev_real(bar).expect("foo token");
        assert_eq!(w.range_at(prev).unwrap(), &(0..3));
    }

    #[test]
    fn next_real_skips_whitespace_and_comments() {
        let (tokens, _) = walk("foo (* doc *)  bar");
        let w = TokenWalk::new(&tokens);
        let foo = w.token_at(0).expect("foo token");
        let next = w.next_real(foo).expect("bar token");
        assert_eq!(w.range_at(next).unwrap(), &(15..18));
    }

    #[test]
    fn leading_comments_attach_through_blank_line() {
        let src = "(* doc *)\n\nFUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let w = TokenWalk::new(&tokens);
        let func = w.token_at(11).expect("FUNCTION token");
        let leading = w.leading_comments(func);
        assert_eq!(leading.len(), 1);
        assert_eq!(w.range_at(leading[0]).unwrap(), &(0..9));
    }

    #[test]
    fn leading_comments_stop_at_real_token() {
        // Two functions, doc above the second. Should not pick up the
        // END_FUNCTION above as part of foo's leading comment chain.
        let src = "FUNCTION a : INT END_FUNCTION\n(* foo doc *)\nFUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let w = TokenWalk::new(&tokens);
        // Find 'foo' declaration token by its byte offset
        let foo_offset = src.find("FUNCTION foo").unwrap();
        let func = w.token_at(foo_offset).expect("FUNCTION foo token");
        let leading = w.leading_comments(func);
        assert_eq!(leading.len(), 1);
        let comment_range = w.range_at(leading[0]).unwrap();
        let comment_text = &src[comment_range.clone()];
        assert_eq!(comment_text, "(* foo doc *)");
    }

    #[test]
    fn leading_comments_chain_multiple_block_comments() {
        let src = "(* part 1 *)\n(* part 2 *)\nFUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let w = TokenWalk::new(&tokens);
        let func_offset = src.find("FUNCTION").unwrap();
        let func = w.token_at(func_offset).unwrap();
        let leading = w.leading_comments(func);
        assert_eq!(leading.len(), 2);
        // Source order
        assert_eq!(&src[w.range_at(leading[0]).unwrap().clone()], "(* part 1 *)");
        assert_eq!(&src[w.range_at(leading[1]).unwrap().clone()], "(* part 2 *)");
    }

    #[test]
    fn leading_comments_break_on_pragma() {
        let src = "(* doc *)\n{unknown_pragma}\nFUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let w = TokenWalk::new(&tokens);
        let func_offset = src.find("FUNCTION").unwrap();
        let func = w.token_at(func_offset).unwrap();
        let leading = w.leading_comments(func);
        assert_eq!(leading.len(), 0, "pragma must break attachment");
    }

    #[test]
    fn trailing_comments_pick_up_inline_doc() {
        let src = "FUNCTION foo : INT END_FUNCTION (* trailing *)";
        let (tokens, _) = walk(src);
        let w = TokenWalk::new(&tokens);
        let end_offset = src.find("END_FUNCTION").unwrap();
        let end = w.token_at(end_offset).unwrap();
        let trailing = w.trailing_comments(end);
        assert_eq!(trailing.len(), 1);
        assert_eq!(&src[w.range_at(trailing[0]).unwrap().clone()], "(* trailing *)");
    }

    #[test]
    fn line_comments_attach_too() {
        let src = "// short doc\nFUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let w = TokenWalk::new(&tokens);
        let func_offset = src.find("FUNCTION").unwrap();
        let func = w.token_at(func_offset).unwrap();
        let leading = w.leading_comments(func);
        assert_eq!(leading.len(), 1);
        assert_eq!(&src[w.range_at(leading[0]).unwrap().clone()], "// short doc");
    }

    /// Closure used by the docstring tests: treat POU-introducing keywords
    /// as part of the declaration prefix so a comment above `FUNCTION foo`
    /// attaches to `foo`.
    fn pou_prefix(tok: &Token) -> bool {
        matches!(tok, Token::KeywordFunction | Token::KeywordFunctionBlock | Token::KeywordProgram)
    }

    /// Closure for tests that anchor directly on a name (variables, members):
    /// no keyword is part of the prefix.
    fn no_prefix(_tok: &Token) -> bool {
        false
    }

    #[test]
    fn docstring_strips_st_block_markers() {
        let src = "(* Computes the running average *)\nFUNCTION moving_avg : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        // Anchor on the POU's name to mirror how SymbolUnderCursor's
        // declaration_location points at the identifier, not the keyword.
        let offset = src.find("moving_avg").unwrap();
        let doc = docstring_at(&tokens, src, offset, pou_prefix).expect("doc body");
        assert_eq!(doc, "Computes the running average");
    }

    #[test]
    fn docstring_strips_c_block_markers() {
        let src = "/* C-style docs */\nFUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let offset = src.find("foo").unwrap();
        let doc = docstring_at(&tokens, src, offset, pou_prefix).expect("doc body");
        assert_eq!(doc, "C-style docs");
    }

    #[test]
    fn docstring_strips_line_markers() {
        let src = "// short doc\nFUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let offset = src.find("foo").unwrap();
        let doc = docstring_at(&tokens, src, offset, pou_prefix).expect("doc body");
        assert_eq!(doc, "short doc");
    }

    #[test]
    fn docstring_joins_multiple_blocks_with_blank_line() {
        let src = "(* line one *)\n(* line two *)\nFUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let offset = src.find("foo").unwrap();
        let doc = docstring_at(&tokens, src, offset, pou_prefix).expect("doc body");
        assert_eq!(doc, "line one\n\nline two");
    }

    #[test]
    fn docstring_returns_none_when_no_leading_comments() {
        let src = "FUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let offset = src.find("foo").unwrap();
        assert!(docstring_at(&tokens, src, offset, pou_prefix).is_none());
    }

    #[test]
    fn docstring_preserves_internal_newlines() {
        let src = "(* line one\n   line two *)\nFUNCTION foo : INT END_FUNCTION";
        let (tokens, _) = walk(src);
        let offset = src.find("foo").unwrap();
        let doc = docstring_at(&tokens, src, offset, pou_prefix).expect("doc body");
        assert!(doc.contains("line one"));
        assert!(doc.contains("line two"));
    }

    #[test]
    fn docstring_anchors_on_variable_name_directly() {
        // Variables don't need keyword walk-back: the comment sits right
        // above the identifier.
        let src = "VAR\n(* width of buffer *)\nlen : INT;\nEND_VAR";
        let (tokens, _) = walk(src);
        let offset = src.find("len").unwrap();
        let doc = docstring_at(&tokens, src, offset, no_prefix).expect("doc body");
        assert_eq!(doc, "width of buffer");
    }
}
