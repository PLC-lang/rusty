use crate::lexer::{lex_with_trivia, LspToken, Token, TriviaKind};

fn collect(source: &str) -> Vec<(&str, String)> {
    lex_with_trivia(source)
        .into_iter()
        .map(|t| match t {
            LspToken::Token(tok, r) => ("T", format!("{:?}@{}..{}", tok, r.start, r.end)),
            LspToken::Trivia(kind, r) => match kind {
                TriviaKind::Whitespace => ("W", format!("ws@{}..{}", r.start, r.end)),
                TriviaKind::LineComment => ("C", format!("line@{}..{}", r.start, r.end)),
                TriviaKind::BlockComment => ("C", format!("block@{}..{}", r.start, r.end)),
                TriviaKind::Pragma => ("P", format!("pragma@{}..{}", r.start, r.end)),
            },
        })
        .collect()
}

fn check_contiguous(source: &str) {
    let tokens = lex_with_trivia(source);
    let mut prev_end = 0;
    for tok in &tokens {
        let r = tok.range();
        assert_eq!(r.start, prev_end, "gap before {tok:?}");
        assert!(r.end <= source.len());
        prev_end = r.end;
    }
    assert_eq!(prev_end, source.len(), "trailing bytes not covered: {tokens:?}");
}

#[test]
fn empty_source_yields_no_tokens() {
    assert!(lex_with_trivia("").is_empty());
}

#[test]
fn whitespace_only_is_a_single_trivia_run() {
    let toks = collect("   \n\t\n");
    assert_eq!(toks, vec![("W", "ws@0..6".to_string())]);
}

#[test]
fn line_comment_to_eof_without_newline() {
    // Line comment runs to EOF when no trailing newline.
    let toks = collect("// hello");
    assert_eq!(toks, vec![("C", "line@0..8".to_string())]);
}

#[test]
fn line_comment_stops_at_newline_and_whitespace_follows() {
    let toks = collect("// hi\n  ");
    assert_eq!(toks, vec![("C", "line@0..5".to_string()), ("W", "ws@5..8".to_string())]);
}

#[test]
fn st_block_comment_simple() {
    let toks = collect("(* doc *)");
    assert_eq!(toks, vec![("C", "block@0..9".to_string())]);
}

#[test]
fn c_block_comment_simple() {
    let toks = collect("/* doc */");
    assert_eq!(toks, vec![("C", "block@0..9".to_string())]);
}

#[test]
fn st_block_comment_nested() {
    // (* outer (* inner *) still outer *)
    let src = "(* a (* b *) c *)";
    let toks = collect(src);
    assert_eq!(toks, vec![("C", format!("block@0..{}", src.len()))]);
}

#[test]
fn c_block_comment_nested() {
    let src = "/* a /* b */ c */";
    let toks = collect(src);
    assert_eq!(toks, vec![("C", format!("block@0..{}", src.len()))]);
}

#[test]
fn block_comments_do_not_cross_nest() {
    // A `/*` inside `(* ... *)` does not increase depth: the first `*)` closes.
    let src = "(* outer /* not nested *)";
    let toks = collect(src);
    assert_eq!(toks, vec![("C", format!("block@0..{}", src.len()))]);
}

#[test]
fn unknown_pragma_becomes_pragma_trivia() {
    // `{external}` is a recognised pragma token; `{random}` falls through the
    // catch-all and used to be silently skipped.
    let toks = collect("{random_thing}");
    assert_eq!(toks, vec![("P", "pragma@0..14".to_string())]);
}

#[test]
fn real_tokens_carry_their_kind_and_span() {
    let toks = lex_with_trivia("VAR");
    assert_eq!(toks.len(), 1);
    match &toks[0] {
        LspToken::Token(Token::KeywordVar, r) => assert_eq!(*r, 0..3),
        other => panic!("expected KeywordVar, got {other:?}"),
    }
}

#[test]
fn mixed_stream_covers_every_byte() {
    let src = "VAR x : INT; (* hello *) END_VAR\n";
    check_contiguous(src);
    let kinds: Vec<&str> = collect(src).into_iter().map(|(k, _)| k).collect();
    // Token, ws, Token, ws, Token, ws, Token, Token, ws, BlockComment, ws, Token, ws
    assert_eq!(kinds, vec!["T", "W", "T", "W", "T", "W", "T", "T", "W", "C", "W", "T", "W"]);
}

#[test]
fn trivia_before_first_token() {
    let src = "  VAR";
    let toks = collect(src);
    assert_eq!(toks, vec![("W", "ws@0..2".to_string()), ("T", "KeywordVar@2..5".to_string())]);
}

#[test]
fn trivia_after_last_token() {
    let src = "VAR  ";
    let toks = collect(src);
    assert_eq!(toks, vec![("T", "KeywordVar@0..3".to_string()), ("W", "ws@3..5".to_string())]);
}

#[test]
fn utf8_inside_block_comment_is_byte_safe() {
    // Multi-byte UTF-8 inside a comment must not corrupt span boundaries.
    let src = "(* héllo *)";
    check_contiguous(src);
    let toks = collect(src);
    assert_eq!(toks.len(), 1);
    assert!(matches!(toks[0].0, "C"));
}

#[test]
fn unterminated_block_comment_consumes_rest() {
    // Mirrors the existing lexer's behaviour: parse_comments returns Filter::Emit
    // when no closer is found, producing an Error token in the original stream.
    // Here we just want to confirm we don't panic and we cover all bytes.
    check_contiguous("(* unterminated");
    check_contiguous("/* unterminated");
}

#[test]
fn doc_comment_then_declaration() {
    // The scenario that motivates this whole module: a leading doc comment
    // followed by a declaration the LSP needs to associate with it.
    let src = "(* doc for foo *)\nFUNCTION foo : INT END_FUNCTION";
    check_contiguous(src);
    let toks = collect(src);
    assert_eq!(toks[0], ("C", "block@0..17".to_string()));
    assert_eq!(toks[1], ("W", "ws@17..18".to_string()));
    assert_eq!(toks[2], ("T", "KeywordFunction@18..26".to_string()));
}
