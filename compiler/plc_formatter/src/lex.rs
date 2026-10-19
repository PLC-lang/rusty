//! Drives the real lexer directly (bypassing `plc_lexer::ParseSession`) to get a plain
//! `(Token, Range<usize>)` stream, with no diagnostics/id-provider plumbing attached.

use std::ops::Range;

use logos::Logos;
use plc_lexer::Token;

#[derive(Debug, Clone)]
pub struct RawToken {
    pub token: Token,
    pub range: Range<usize>,
}

/// The byte offset of an unrecoverable lex error (an unterminated comment/pragma, or
/// genuinely invalid input). The formatter must never guess past this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexError {
    pub offset: usize,
}

pub fn lex_tokens(source: &str) -> Result<Vec<RawToken>, LexError> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        let range = lexer.span();
        if token == Token::Error {
            return Err(LexError { offset: range.start });
        }
        tokens.push(RawToken { token, range });
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_real_tokens_only() {
        let tokens = lex_tokens("PROGRAM foo END_PROGRAM").unwrap();
        let kinds: Vec<_> = tokens.iter().map(|t| t.token).collect();
        assert_eq!(kinds, vec![Token::KeywordProgram, Token::Identifier, Token::KeywordEndProgram]);
    }

    #[test]
    fn comments_and_whitespace_are_not_emitted() {
        let tokens = lex_tokens("a /* hi */ := 1; // trailing\nb;").unwrap();
        let kinds: Vec<_> = tokens.iter().map(|t| t.token).collect();
        assert_eq!(
            kinds,
            vec![
                Token::Identifier,
                Token::KeywordAssignment,
                Token::LiteralInteger,
                Token::KeywordSemicolon,
                Token::Identifier,
                Token::KeywordSemicolon,
            ]
        );
    }

    #[test]
    fn unterminated_comment_is_a_lex_error() {
        let result = lex_tokens("a := 1; (* unterminated");
        assert!(result.is_err());
    }
}
