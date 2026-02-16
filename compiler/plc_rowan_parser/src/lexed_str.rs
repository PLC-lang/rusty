use std::ops::Range;

use logos::Logos;

use crate::SyntaxKind;

// Re-export Token from the main lexer module
pub use rusty::lexer::Token;

/// Lexer that wraps the logos-based Token lexer and converts tokens to SyntaxKind
struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from source text
    pub fn new(source: &'a str) -> Self {
        Self {
            inner: Token::lexer(source),
        }
    }

    /// Get the current token's text slice
    pub fn slice(&self) -> &'a str {
        self.inner.slice()
    }

    /// Get the current token's byte range
    pub fn span(&self) -> std::ops::Range<u32> {
        let span = self.inner.span();
        span.start as u32..span.end as u32  //TODO: is it worth? move to usize?
    }

    /// Advance to the next token and return its SyntaxKind
    pub fn next_token(&mut self) -> Option<SyntaxKind> {
        self.inner.next().map(token_to_syntax_kind)
    }

    pub fn next(&mut self) -> Option<(SyntaxKind, std::ops::Range<u32>)> {
        self.inner.next().map(|token| (token_to_syntax_kind(token), self.span()))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, std::ops::Range<u32>);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

/// Convert a Token to its corresponding SyntaxKind
fn token_to_syntax_kind(token: Token) -> SyntaxKind {
    match token {
        // Error token
        Token::Error => SyntaxKind::ERROR,

        // Properties - no direct mapping, treating as identifiers or error
        Token::PropertyExternal => todo!("PropertyExternal mapping"),
        Token::PropertyByRef => todo!("PropertyByRef mapping"),
        Token::PropertyConstant => todo!("PropertyConstant mapping"),
        Token::PropertySized => todo!("PropertySized mapping"),

        // POU keywords
        Token::KeywordProgram => SyntaxKind::POU_TYPE,
        Token::KeywordFunction => SyntaxKind::POU_TYPE,
        Token::KeywordEndFunction => SyntaxKind::POU_END_KEYWORD,
        Token::KeywordFunctionBlock => SyntaxKind::POU_TYPE,
        Token::KeywordEndFunctionBlock => SyntaxKind::POU_END_KEYWORD,
        Token::KeywordEndProgram => SyntaxKind::POU_END_KEYWORD,

        // Class/OOP keywords - no direct mapping in generated.rs
        Token::KeywordClass => todo!("KeywordClass mapping"),
        Token::KeywordEndClass => todo!("KeywordEndClass mapping"),
        Token::KeywordExtends => todo!("KeywordExtends mapping"),
        Token::KeywordImplements => todo!("KeywordImplements mapping"),
        Token::KeywordInterface => todo!("KeywordInterface mapping"),
        Token::KeywordEndInterface => todo!("KeywordEndInterface mapping"),
        Token::KeywordAbstract => todo!("KeywordAbstract mapping"),
        Token::KeywordFinal => todo!("KeywordFinal mapping"),
        Token::KeywordMethod => todo!("KeywordMethod mapping"),
        Token::KeywordEndMethod => todo!("KeywordEndMethod mapping"),
        Token::KeywordSuper => todo!("KeywordSuper mapping"),
        Token::KeywordThis => todo!("KeywordThis mapping"),
        Token::KeywordProperty => todo!("KeywordProperty mapping"),
        Token::KeywordEndProperty => todo!("KeywordEndProperty mapping"),
        Token::KeywordGet => todo!("KeywordGet mapping"),
        Token::KeywordEndGet => todo!("KeywordEndGet mapping"),
        Token::KeywordSet => todo!("KeywordSet mapping"),
        Token::KeywordEndSet => todo!("KeywordEndSet mapping"),
        Token::KeywordOverride => todo!("KeywordOverride mapping"),

        // Variable block keywords
        Token::KeywordVar => SyntaxKind::VAR_DECLARATION_TYPE,
        Token::KeywordVarInput => SyntaxKind::VAR_DECLARATION_TYPE,
        Token::KeywordVarOutput => SyntaxKind::VAR_DECLARATION_TYPE,
        Token::KeywordVarInOut => SyntaxKind::VAR_DECLARATION_TYPE,
        Token::KeywordVarTemp => SyntaxKind::VAR_DECLARATION_TYPE,
        Token::KeywordVarGlobal => SyntaxKind::VAR_DECLARATION_TYPE,
        Token::KeywordEndVar => SyntaxKind::END_VAR_KW,

        // Variable qualifiers
        Token::KeywordConstant => SyntaxKind::CONSTANT_KW,
        Token::KeywordRetain => SyntaxKind::RETAIN_KW,
        Token::KeywordNonRetain => SyntaxKind::NON_RETAIN_KW,
        Token::KeywordAt => SyntaxKind::AT_KW,

        // Other variable keywords
        Token::KeywordVarConfig => todo!("KeywordVarConfig mapping"),
        Token::KeywordVarExternal => todo!("KeywordVarExternal mapping"),

        // Access modifiers - no direct mapping
        Token::KeywordAccessPublic => todo!("KeywordAccessPublic mapping"),
        Token::KeywordAccessPrivate => todo!("KeywordAccessPrivate mapping"),
        Token::KeywordAccessInternal => todo!("KeywordAccessInternal mapping"),
        Token::KeywordAccessProtected => todo!("KeywordAccessProtected mapping"),

        // Type keywords
        Token::KeywordType => todo!("KeywordType mapping"),
        Token::KeywordStruct => todo!("KeywordStruct mapping"),
        Token::KeywordEndType => todo!("KeywordEndType mapping"),
        Token::KeywordEndStruct => todo!("KeywordEndStruct mapping"),

        // Action keywords - no direct mapping
        Token::KeywordActions => todo!("KeywordActions mapping"),
        Token::KeywordAction => todo!("KeywordAction mapping"),
        Token::KeywordEndAction => todo!("KeywordEndAction mapping"),
        Token::KeywordEndActions => todo!("KeywordEndActions mapping"),

        // Punctuation
        Token::KeywordColon => SyntaxKind::COLON,
        Token::KeywordSemicolon => SyntaxKind::SEMICOLON,
        Token::KeywordAssignment => SyntaxKind::ASSIGN,
        Token::KeywordOutputAssignment => SyntaxKind::FAT_ARROW,
        Token::KeywordReferenceAssignment => todo!("KeywordReferenceAssignment REF= mapping"),
        Token::KeywordParensOpen => SyntaxKind::L_PAREN,
        Token::KeywordParensClose => SyntaxKind::R_PAREN,
        Token::KeywordSquareParensOpen => SyntaxKind::L_BRACK,
        Token::KeywordSquareParensClose => SyntaxKind::R_BRACK,
        Token::KeywordComma => SyntaxKind::COMMA,
        Token::KeywordDotDotDot => SyntaxKind::DOT3,
        Token::KeywordDotDot => SyntaxKind::DOT2,
        Token::KeywordDot => SyntaxKind::DOT,

        // Control flow keywords - no direct mapping for most
        Token::KeywordIf => todo!("KeywordIf mapping"),
        Token::KeywordThen => todo!("KeywordThen mapping"),
        Token::KeywordElseIf => todo!("KeywordElseIf mapping"),
        Token::KeywordElse => todo!("KeywordElse mapping"),
        Token::KeywordEndIf => todo!("KeywordEndIf mapping"),
        Token::KeywordFor => todo!("KeywordFor mapping"),
        Token::KeywordTo => todo!("KeywordTo mapping"),
        Token::KeywordBy => todo!("KeywordBy mapping"),
        Token::KeywordDo => todo!("KeywordDo mapping"),
        Token::KeywordEndFor => todo!("KeywordEndFor mapping"),
        Token::KeywordWhile => todo!("KeywordWhile mapping"),
        Token::KeywordEndWhile => todo!("KeywordEndWhile mapping"),
        Token::KeywordRepeat => todo!("KeywordRepeat mapping"),
        Token::KeywordUntil => todo!("KeywordUntil mapping"),
        Token::KeywordEndRepeat => todo!("KeywordEndRepeat mapping"),
        Token::KeywordCase => todo!("KeywordCase mapping"),
        Token::KeywordEndCase => todo!("KeywordEndCase mapping"),
        Token::KeywordReturn => todo!("KeywordReturn mapping"),
        Token::KeywordExit => todo!("KeywordExit mapping"),
        Token::KeywordContinue => todo!("KeywordContinue mapping"),

        // Type keywords
        Token::KeywordPointer => todo!("KeywordPointer mapping"),
        Token::KeywordFunctionPointer => todo!("KeywordFunctionPointer mapping"),
        Token::KeywordRef => todo!("KeywordRef mapping"),
        Token::KeywordReferenceTo => todo!("KeywordReferenceTo mapping"),
        Token::KeywordArray => todo!("KeywordArray mapping"),
        Token::KeywordString => todo!("KeywordString mapping"),
        Token::KeywordWideString => todo!("KeywordWideString mapping"),
        Token::KeywordOf => todo!("KeywordOf mapping"),

        // Operators
        Token::OperatorPlus => SyntaxKind::PLUS,
        Token::OperatorMinus => SyntaxKind::MINUS,
        Token::OperatorMultiplication => SyntaxKind::STAR,
        Token::OperatorExponent => todo!("OperatorExponent ** mapping"),
        Token::OperatorDivision => SyntaxKind::SLASH,
        Token::OperatorEqual => SyntaxKind::EQ,
        Token::OperatorNotEqual => SyntaxKind::NEQ,
        Token::OperatorLess => SyntaxKind::L_ANGLE,
        Token::OperatorGreater => SyntaxKind::R_ANGLE,
        Token::OperatorLessOrEqual => SyntaxKind::LTEQ,
        Token::OperatorGreaterOrEqual => SyntaxKind::GTEQ,
        Token::OperatorAmp => SyntaxKind::AMP,
        Token::OperatorDeref => SyntaxKind::CARET,
        Token::OperatorModulo => SyntaxKind::PERCENT,
        Token::OperatorAnd => SyntaxKind::AMP2,
        Token::OperatorOr => SyntaxKind::PIPE2,
        Token::OperatorXor => todo!("OperatorXor mapping"),
        Token::OperatorNot => SyntaxKind::BANG,

        // Identifiers
        Token::Identifier => SyntaxKind::IDENT,

        // Literals - integers
        Token::LiteralIntegerHex => SyntaxKind::INT_NUMBER,
        Token::LiteralIntegerOct => SyntaxKind::INT_NUMBER,
        Token::LiteralIntegerBin => SyntaxKind::INT_NUMBER,
        Token::LiteralInteger => SyntaxKind::INT_NUMBER,

        // Literals - boolean
        Token::LiteralTrue => SyntaxKind::BOOL_LITERAL,
        Token::LiteralFalse => SyntaxKind::BOOL_LITERAL,
        Token::LiteralNull => todo!("LiteralNull mapping"),

        // Literals - time/date
        Token::LiteralDate => todo!("LiteralDate mapping"),
        Token::LiteralDateAndTime => todo!("LiteralDateAndTime mapping"),
        Token::LiteralTimeOfDay => todo!("LiteralTimeOfDay mapping"),
        Token::LiteralTime => todo!("LiteralTime mapping"),

        // Hardware access
        Token::DirectAccess(_) => todo!("DirectAccess mapping"),
        Token::HardwareAccess(_) => todo!("HardwareAccess mapping"),

        // String literals
        Token::LiteralString => SyntaxKind::STRING_LITERAL,
        Token::LiteralWideString => SyntaxKind::STRING_LITERAL,

        // Type cast prefix
        Token::TypeCastPrefix => todo!("TypeCastPrefix mapping"),

        // End token
        Token::End => SyntaxKind::EOF,
    }
}


pub struct LexedStr<'a> {
    text: &'a str,
    kind: Vec<SyntaxKind>,
    start: Vec<Range<u32>>,
    error: Vec<LexError>,
}

struct LexError {
    msg: String,
    token: u32,
}

impl<'a> LexedStr<'a> {
    pub fn new(text: &'a str) -> LexedStr<'a> {
        let mut lexed_str = Self{
            text,
            kind: Vec::new(),
            start: Vec::new(),
            error: Vec::new(),
        };

        for (k, r) in Lexer::new(text) {
            lexed_str.push(k, r);
        }
        lexed_str.push(SyntaxKind::EOF, lexed_str.text_range(lexed_str.len() - 1));
        lexed_str
    }

    pub fn as_str(&self) -> &str {
        self.text
    }

    pub fn len(&self) -> usize {
        self.kind.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self, i: usize) -> SyntaxKind {
        assert!(i < self.len());
        self.kind[i]
    }

    pub fn text(&self, i: usize) -> &str {
        self.range_text(i..i)
    }

    pub fn range_text(&self, r: Range<usize>) -> &str {
        assert!(r.start <= r.end && r.end <= self.len());
        let lo = self.start[r.start].start as usize;
        let hi = self.start[r.end].end as usize;
        &self.text[lo..hi]
    }

    // Naming is hard.
    pub fn text_range(&self, i: usize) -> Range<u32> {
        assert!(i < self.len());
        self.start[i].clone()
    }
    pub fn text_start(&self, i: usize) -> usize {
        assert!(i <= self.len());
        self.start[i].start as usize
    }
    pub fn text_len(&self, i: usize) -> usize {
        assert!(i < self.len());
        let r = self.text_range(i);
        (r.end - r.start) as usize
    }

    pub fn error(&self, i: usize) -> Option<&str> {
        assert!(i < self.len());
        let err = self.error.binary_search_by_key(&(i as u32), |i| i.token).ok()?;
        Some(self.error[err].msg.as_str())
    }

    pub fn errors(&self) -> impl Iterator<Item = (usize, &str)> + '_ {
        self.error.iter().map(|it| (it.token as usize, it.msg.as_str()))
    }

    fn push(&mut self, kind: SyntaxKind, range: Range<u32>) {
        self.kind.push(kind);
        self.start.push(range);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_keywords() {
        let source = "PROGRAM END_PROGRAM FUNCTION END_FUNCTION FUNCTION_BLOCK END_FUNCTION_BLOCK";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next_token(), Some(SyntaxKind::POU_TYPE));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::POU_END_KEYWORD));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::POU_TYPE));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::POU_END_KEYWORD));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::POU_TYPE));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::POU_END_KEYWORD));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_punctuation() {
        let source = "()[];:,.";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next_token(), Some(SyntaxKind::L_PAREN));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::R_PAREN));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::L_BRACK));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::R_BRACK));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::SEMICOLON));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::COLON));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::COMMA));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::DOT));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_operators() {
        let source = "+ - * / = < >";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next_token(), Some(SyntaxKind::PLUS));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::MINUS));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::STAR));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::SLASH));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::EQ));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::L_ANGLE));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::R_ANGLE));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_literals() {
        let source = "123 TRUE FALSE 'string'";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next_token(), Some(SyntaxKind::INT_NUMBER));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::BOOL_LITERAL));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::BOOL_LITERAL));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::STRING_LITERAL));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_var_blocks() {
        let source = "VAR VAR_INPUT VAR_OUTPUT END_VAR";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next_token(), Some(SyntaxKind::VAR_DECLARATION_TYPE));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::VAR_DECLARATION_TYPE));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::VAR_DECLARATION_TYPE));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::END_VAR_KW));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_identifiers() {
        let source = "myVar _test var123";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next_token(), Some(SyntaxKind::NAME));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::NAME));
        assert_eq!(lexer.next_token(), Some(SyntaxKind::NAME));
        assert_eq!(lexer.next_token(), None);
    }
}
