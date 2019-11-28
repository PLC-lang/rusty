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
    fn pou_tokens() {
        let mut lexer = super::lex("PROGRAM END_PROGRAM");
        assert_eq!(lexer.token, super::Token::KeywordProgram);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::KeywordEndProgram);
    }

    #[test]
    fn var_tokens() {
        let mut lexer = super::lex("VAR END_VAR");
        assert_eq!(lexer.token, super::Token::KeywordVar);
        lexer.advance();
        assert_eq!(lexer.token, super::Token::KeywordEndVar);
    }

    #[test]
    fn hello_is_an_identifier() {
        let mut lexer = super::lex("hello a12 _a12");
        assert_eq!(lexer.token, super::Token::Identifier, "{}", lexer.slice());
        lexer.advance();
        assert_eq!(lexer.token, super::Token::Identifier, "{}", lexer.slice());
        lexer.advance();
        assert_eq!(lexer.token, super::Token::Identifier, "{}", lexer.slice());
        lexer.advance();
    }

    #[test]
    fn an_identifier_cannot_start_with_a_number() {
        let lexer = super::lex("2g12");
        assert_ne!(lexer.token, super::Token::Identifier);
    }

    #[test]
    fn punctuations() {
        let lexer = super::lex(":");
        assert_eq!(lexer.token, super::Token::KeywordColon, "{}", lexer.slice());
        let lexer = super::lex(";");
        assert_eq!(
            lexer.token,
            super::Token::KeywordSemicolon,
            "{}",
            lexer.slice()
        );
    }

    #[test]
    fn a_assignment_is_keword_assignment() {
        let lexer = super::lex(":=");
        assert_eq!(lexer.token, super::Token::KeywordAssignment);
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
