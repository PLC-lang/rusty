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
    KeywordSemicolon,

    #[token = ":="]
    KeywordAssignment,

    //Operators
    #[token = "+"]
    OperatorPlus,

    #[token = "-"]
    OperatorMinus,

    #[regex = r"[0-9]+(\.(0-9)+)?"]
    LiteralNumber,
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
    fn an_identifier_is_alphanumeric() {
        let lexer = super::lex("a12");
        assert_eq!(lexer.token, super::Token::Identifier);
    }

    #[test]
    fn an_identifier_can_start_with_underscore() {
        let lexer = super::lex("_a12");
        assert_eq!(lexer.token, super::Token::Identifier);
    }

    #[test]
    fn an_identifier_cannot_start_with_a_number() {
        let lexer = super::lex("2g12");
        assert_ne!(lexer.token, super::Token::Identifier);
    }

    #[test]
    fn a_colon_is_keyword_colon() {
        let lexer = super::lex(":");
        assert_eq!(lexer.token, super::Token::KeywordColon);
    }

    #[test]
    fn a_assignment_is_keword_assignment() {
        let lexer = super::lex(":=");
        assert_eq!(lexer.token, super::Token::KeywordAssignment);
    }

    #[test]
    fn a_semicolon_is_keyword_semicolon() {
        let lexer = super::lex(";");
        assert_eq!(lexer.token, super::Token::KeywordSemicolon);
    }

    #[test]
    fn operator_test() {
        let mut lexer = super::lex("+ -");
        assert_eq!(lexer.token, super::Token::OperatorPlus);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::OperatorMinus);
    }

    #[test]
    fn literals_test() {
        let mut lexer = super::lex("1 2.2 0123.0123 321");

        for x in 0..5 {
            print!("{}", x);
            assert_eq!(lexer.token, super::Token::LiteralNumber);
            lexer.advance();
        }
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
        assert_eq!(lexer.token, super::Token::KeywordSemicolon);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::KeywordEndVar);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::KeywordEndProgram);
    }
}
