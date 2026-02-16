mod event;
mod grammar;
mod input;
mod lexed_str;
mod output;
mod parser;

#[macro_use]
mod syntax_kind;
mod shortcuts;
mod token_set;

pub use T_ as T;

pub use crate::{
    input::Input,
    lexed_str::LexedStr,
    output::{Output, Step},
    shortcuts::StrStep,
    syntax_kind::SyntaxKind,
};
pub(crate) use token_set::TokenSet;

/**
 * public API for parsing
 */
pub fn parse_event_list(input: &Input) -> Output {
    let mut p = parser::Parser::new(input);
    grammar::compilation_unit(&mut p);
    let events = p.finish();
    let res = event::process(events);
    res
}
