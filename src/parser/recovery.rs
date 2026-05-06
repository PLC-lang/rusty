use crate::lexer::{
    ParseSession,
    Token::{self, *},
};

#[derive(Clone, Copy)]
pub enum ElementStart {
    ConfigVariable,
    VariableDeclaration,
}

pub const TOP_LEVEL_START: &[Token] = &[
    PropertyExternal,
    PropertyConstant,
    KeywordInterface,
    KeywordVarGlobal,
    KeywordVarConfig,
    KeywordProgram,
    KeywordClass,
    KeywordFunction,
    KeywordFunctionBlock,
    KeywordAction,
    KeywordActions,
    KeywordType,
];

pub const MEMBER_START: &[Token] = &[PropertyConstant, KeywordMethod, KeywordPropertyGet, KeywordPropertySet];

pub const ACTION_START: &[Token] = &[KeywordAction, KeywordEndActions];

pub const ACTIONS_BLOCK_BOUNDARY: &[Token] = &[
    PropertyExternal,
    PropertyConstant,
    KeywordInterface,
    KeywordVarGlobal,
    KeywordVarConfig,
    KeywordProgram,
    KeywordClass,
    KeywordFunction,
    KeywordFunctionBlock,
    KeywordType,
];

pub const STATEMENT_BLOCK_BOUNDARY: &[Token] = &[
    PropertyExternal,
    PropertyConstant,
    KeywordInterface,
    KeywordVarGlobal,
    KeywordVarConfig,
    KeywordProgram,
    KeywordClass,
    KeywordFunction,
    KeywordFunctionBlock,
    KeywordAction,
    KeywordActions,
    KeywordType,
    KeywordEndProgram,
    KeywordEndFunction,
    KeywordEndFunctionBlock,
    KeywordEndMethod,
    KeywordEndProperty,
    KeywordEndAction,
    KeywordEndClass,
];

pub const VARIABLE_BLOCK_BOUNDARY: &[Token] = &[
    PropertyExternal,
    PropertyConstant,
    KeywordInterface,
    KeywordVarGlobal,
    KeywordVarConfig,
    KeywordProgram,
    KeywordClass,
    KeywordFunction,
    KeywordFunctionBlock,
    KeywordAction,
    KeywordActions,
    KeywordType,
    KeywordMethod,
    KeywordPropertyGet,
    KeywordPropertySet,
    KeywordEndProgram,
    KeywordEndFunction,
    KeywordEndFunctionBlock,
    KeywordEndMethod,
    KeywordEndProperty,
    KeywordEndAction,
    KeywordEndClass,
    KeywordEndInterface,
];

pub const EXPRESSION_REGION_BOUNDARY: &[Token] = &[
    KeywordSemicolon,
    PropertyExternal,
    PropertyConstant,
    KeywordInterface,
    KeywordVarGlobal,
    KeywordVarConfig,
    KeywordProgram,
    KeywordClass,
    KeywordFunction,
    KeywordFunctionBlock,
    KeywordAction,
    KeywordActions,
    KeywordType,
    KeywordEndProgram,
    KeywordEndFunction,
    KeywordEndFunctionBlock,
    KeywordEndMethod,
    KeywordEndProperty,
    KeywordEndAction,
    KeywordEndClass,
];

const CONFIG_VARIABLE_BOUNDARY: &[Token] = &[
    KeywordSemicolon,
    KeywordEndVar,
    PropertyExternal,
    PropertyConstant,
    KeywordInterface,
    KeywordVarGlobal,
    KeywordVarConfig,
    KeywordProgram,
    KeywordClass,
    KeywordFunction,
    KeywordFunctionBlock,
    KeywordAction,
    KeywordActions,
    KeywordType,
    KeywordEndProgram,
    KeywordEndFunction,
    KeywordEndFunctionBlock,
    KeywordEndMethod,
    KeywordEndProperty,
    KeywordEndAction,
    KeywordEndClass,
    End,
];

pub fn combine(primary: &[Token], secondary: &[Token]) -> Vec<Token> {
    let mut tokens = primary.to_vec();
    tokens.extend(secondary);
    tokens
}

pub fn at_element_start(lexer: &ParseSession, element_start: ElementStart) -> bool {
    match element_start {
        ElementStart::ConfigVariable => {
            lexer.token.is_identifier_like()
                && lexer.token_appears_before(KeywordAt, CONFIG_VARIABLE_BOUNDARY)
        }
        ElementStart::VariableDeclaration => {
            lexer.token.is_identifier_like() && matches!(lexer.peek_token(), KeywordColon)
        }
    }
}
