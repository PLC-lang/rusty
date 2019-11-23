use logos::Lexer;
use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[end]
    End,
    #[error]
    Error,

    #[token = "PROGRAM"]
    KeywordProgram,

    #[token = "VAR"]
    KeywordVar,

    #[token = "END_VAR"]
    KeywordEndVar,

    #[token = "END_PROGRAM"]
    KeywordEndProgram,

    #[regex = r"[a-zA-Z_][a-zA-Z_0-9]*"]
    Identifier,

    #[token = ":"]
    KeywordColon,

    #[token = ";"]
    KeywordSemiColon,

    #[token = ":="]
    KeywordAssignment,
}

pub fn lex(source: &str) -> Lexer<Token, &str> {
    Token::lexer(source)
}

#[cfg(test)]
mod tests {

    #[test]
    fn program_is_a_keyword() {
        let lexer = super::lex("PROGRAM");
        assert_eq!(lexer.token, super::Token::KeywordProgram);
    }

    #[test]
    fn var_is_a_keyword() {
        let lexer = super::lex("VAR");
        assert_eq!(lexer.token, super::Token::KeywordVar);
    }

    #[test]
    fn endvar_is_a_keyword() {
        let lexer = super::lex("END_VAR");
        assert_eq!(lexer.token, super::Token::KeywordEndVar);
    }

    #[test]
    fn endprorgram_is_a_keyword() {
        let lexer = super::lex("END_PROGRAM");
        assert_eq!(lexer.token, super::Token::KeywordEndProgram);
    }

    #[test]
    fn hello_is_an_identifier() {
        let lexer = super::lex("hello");
        assert_eq!(lexer.token, super::Token::Identifier);
    }

    #[test]
    fn an_identifer_is_alphanumeric() {
        let lexer = super::lex("a12");
        assert_eq!(lexer.token, super::Token::Identifier);
    }

    #[test]
    fn an_identifer_can_start_with_underscore() {
        let lexer = super::lex("_a12");
        assert_eq!(lexer.token, super::Token::Identifier);
    }

    #[test]
    fn an_identifer_cannot_start_with_a_number() {
        let lexer = super::lex("2a12");
        assert_eq!(lexer.token, super::Token::Error);
    }

    #[test]
    fn a_colon_is_KeywordColon() {
        let lexer = super::lex(":");
        assert_eq!(lexer.token, super::Token::KeywordColon);
    }

    #[test]
    fn a_assignment_is_KewordAssignment() {
        let lexer = super::lex(":=");
        assert_eq!(lexer.token, super::Token::KeywordAssignment);
    }

    #[test]
    fn a_semicolon_is_KeywordSemiColon() {
        let lexer = super::lex(";");
        assert_eq!(lexer.token, super::Token::KeywordSemiColon);
    }

    #[test]
    fn a_full_program_generates_correct_token_sequence() {
        let mut lexer = super::lex(
            r"
        PROGRAM hello
        VAR
          a : INT;
        END_VAR
        END_PROGRAM
        ",
        );

        assert_eq!(lexer.token, super::Token::KeywordProgram);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::Identifier);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::KeywordVar);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::Identifier);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::KeywordColon);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::Identifier);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::KeywordSemiColon);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::KeywordEndVar);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::KeywordEndProgram);
    }

}
