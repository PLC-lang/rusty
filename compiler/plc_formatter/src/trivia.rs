//! Recovers comments and blank-line information from the gaps between real tokens.
//!
//! The real lexer (`plc_lexer`) discards whitespace, comments, and pragmas entirely, with no
//! span retained anywhere (see `compiler/plc_lexer/src/lexer.rs`'s `parse_comments`/`parse_pragma`).
//! Since `lex::lex_tokens` drives the exact same tokenization, the gap between two adjacent real
//! tokens is known to contain only whitespace, well-formed comments, and pragmas - whatever the
//! real lexer would have skipped. This module re-scans just that gap text to recover it, without
//! touching `plc_lexer` itself.

use crate::lex::RawToken;
use crate::model::{Document, LeadingTrivia, LexedToken, Trivia};

enum GapItem {
    Newlines(u32),
    Comment(Trivia),
}

/// Mirrors `plc_lexer::lexer::parse_comments`'s nesting algorithm exactly, so that a comment this
/// module finds is always byte-for-byte what the real lexer would have consumed. `rest` must
/// start with `(*` or `/*`; the real lexer has already validated these are always well-formed
/// within a gap, so the "unterminated" fallback here is unreachable in practice.
fn scan_block_comment(rest: &str) -> usize {
    let (open, close) = match &rest[0..2] {
        "(*" => ('(', ')'),
        "/*" => ('/', '/'),
        _ => unreachable!("caller only passes block-comment openers"),
    };
    let mut unclosed = 1i32;
    let mut prev = ' ';
    let mut traversed = 0usize;
    for c in rest[2..].chars() {
        if c == '*' && prev == open {
            unclosed += 1;
            prev = ' ';
        } else if c == close && prev == '*' {
            unclosed -= 1;
            prev = c;
        } else {
            prev = c;
        }
        traversed += c.len_utf8();
        if unclosed == 0 {
            return 2 + traversed;
        }
    }
    2 + traversed
}

fn scan_gap_items(gap: &str) -> Vec<GapItem> {
    let mut items = Vec::new();
    let mut pending_newlines = 0u32;
    let mut i = 0usize;
    while i < gap.len() {
        let rest = &gap[i..];
        let c = rest.chars().next().expect("i < gap.len()");
        if c == '\r' {
            i += c.len_utf8();
        } else if c == '\n' {
            pending_newlines += 1;
            i += c.len_utf8();
        } else if c == ' ' || c == '\t' || c == '\x0c' {
            i += c.len_utf8();
        } else if rest.starts_with("//") {
            items.push(GapItem::Newlines(pending_newlines));
            pending_newlines = 0;
            let end = rest.find('\n').unwrap_or(rest.len());
            items.push(GapItem::Comment(Trivia::LineComment(rest[..end].to_string())));
            i += end;
        } else if rest.starts_with("/*") || rest.starts_with("(*") {
            items.push(GapItem::Newlines(pending_newlines));
            pending_newlines = 0;
            let len = scan_block_comment(rest);
            items.push(GapItem::Comment(Trivia::BlockComment(rest[..len].to_string())));
            i += len;
        } else if c == '{' {
            items.push(GapItem::Newlines(pending_newlines));
            pending_newlines = 0;
            let end = rest.find('}').map(|p| p + 1).unwrap_or(rest.len());
            items.push(GapItem::Comment(Trivia::Pragma(rest[..end].to_string())));
            i += end;
        } else {
            // Defensive only: a gap between two real tokens never contains anything else,
            // since the real lexer already validated this. Never panic on unexpected input.
            i += c.len_utf8();
        }
    }
    items.push(GapItem::Newlines(pending_newlines));
    items
}

/// Splits a gap's items into: comments trailing the previous token (same line, only possible
/// when there is a previous token), leading comments for what follows (each on its own line,
/// with the blank lines before each one), and the blank-line count immediately before what
/// follows.
fn structure(items: Vec<GapItem>, has_prev: bool) -> (Vec<Trivia>, Vec<LeadingTrivia>, u32) {
    let mut idx = 0;
    let mut trailing = Vec::new();
    if has_prev {
        while let (Some(GapItem::Newlines(0)), Some(GapItem::Comment(_))) =
            (items.get(idx), items.get(idx + 1))
        {
            if let Some(GapItem::Comment(c)) = items.get(idx + 1) {
                trailing.push(c.clone());
            }
            idx += 2;
        }
    }

    let mut leading = Vec::new();
    let mut pending_blank = 0u32;
    while idx < items.len() {
        match &items[idx] {
            GapItem::Newlines(n) => pending_blank = n.saturating_sub(1),
            GapItem::Comment(c) => {
                leading.push(LeadingTrivia { blank_lines_before: pending_blank, comment: c.clone() });
                pending_blank = 0;
            }
        }
        idx += 1;
    }
    (trailing, leading, pending_blank)
}

fn structure_gap(gap: &str, has_prev: bool) -> (Vec<Trivia>, Vec<LeadingTrivia>, u32) {
    structure(scan_gap_items(gap), has_prev)
}

/// Builds the full token+trivia `Document` for `source`, given the real-token stream already
/// produced by `lex::lex_tokens`.
pub fn build_document(source: &str, raw_tokens: Vec<RawToken>) -> Document {
    let mut tokens: Vec<LexedToken> = Vec::with_capacity(raw_tokens.len());
    let mut prev_end = 0usize;

    for raw in &raw_tokens {
        let gap = &source[prev_end..raw.range.start];
        let (trailing_for_prev, leading, blank_before) = structure_gap(gap, !tokens.is_empty());

        if let Some(last) = tokens.last_mut() {
            last.trailing_trivia = trailing_for_prev;
        }

        tokens.push(LexedToken {
            token: raw.token,
            text: source[raw.range.clone()].to_string(),
            range: raw.range.clone(),
            trailing_trivia: Vec::new(),
            leading_trivia: leading,
            blank_lines_before: blank_before,
            had_newline_before: gap.contains('\n'),
        });
        prev_end = raw.range.end;
    }

    let eof_gap = &source[prev_end..];
    let (trailing_for_last, trailing_doc, _blank_at_eof) = structure_gap(eof_gap, !tokens.is_empty());
    if let Some(last) = tokens.last_mut() {
        last.trailing_trivia = trailing_for_last;
    }

    Document { tokens, trailing_trivia: trailing_doc }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::lex_tokens;
    use plc_lexer::Token;

    fn document(source: &str) -> Document {
        build_document(source, lex_tokens(source).unwrap())
    }

    #[test]
    fn trailing_line_comment_attaches_to_previous_token() {
        let doc = document("a := 1; // hi\nb;");
        let semi = doc.tokens.iter().find(|t| t.token == Token::KeywordSemicolon).unwrap();
        assert_eq!(semi.trailing_trivia, vec![Trivia::LineComment("// hi".into())]);
    }

    #[test]
    fn leading_comment_attaches_to_next_token_with_blank_lines() {
        let doc = document("a := 1;\n\n// about b\nb := 2;");
        let b = &doc.tokens[4]; // a , := , 1 , ; , b
        assert_eq!(b.token, Token::Identifier);
        assert_eq!(
            b.leading_trivia,
            vec![LeadingTrivia { blank_lines_before: 1, comment: Trivia::LineComment("// about b".into()) }]
        );
        assert_eq!(b.blank_lines_before, 0);
    }

    #[test]
    fn nested_block_comment_is_preserved_verbatim() {
        let src = "a := 1; (* outer (* inner *) still outer *)\nb;";
        let doc = document(src);
        let semi = doc.tokens.iter().find(|t| t.token == Token::KeywordSemicolon).unwrap();
        assert_eq!(
            semi.trailing_trivia,
            vec![Trivia::BlockComment("(* outer (* inner *) still outer *)".into())]
        );
    }

    #[test]
    fn trailing_comments_after_last_token_land_on_document() {
        let doc = document("a := 1; // done\n");
        assert!(doc.trailing_trivia.is_empty());
        let semi = doc.tokens.last().unwrap();
        assert_eq!(semi.trailing_trivia, vec![Trivia::LineComment("// done".into())]);
    }
}
