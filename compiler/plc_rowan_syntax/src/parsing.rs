//! Lexing, bridging to parser (which does the actual parsing) and
//! incremental reparsing.

use rowan::TextRange;

use crate::{SyntaxError, SyntaxTreeBuilder, syntax_node::GreenNode};

pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let lexed = plc_rowan_parser::LexedStr::new(text);
    let parser_input = lexed.to_input();
    let parser_output = plc_rowan_parser::parse_event_list(&parser_input);
    let (node, errors, _eof) = build_tree(lexed, parser_output);
    (node, errors)
}

pub(crate) fn build_tree(
    lexed: plc_rowan_parser::LexedStr<'_>,
    parser_output: plc_rowan_parser::Output,
) -> (GreenNode, Vec<SyntaxError>, bool) {
    let mut builder = SyntaxTreeBuilder::default();

    let is_eof = lexed.intersperse_trivia(&parser_output, &mut |step| match step {
        plc_rowan_parser::StrStep::Token { kind, text } => builder.token(kind, text),
        plc_rowan_parser::StrStep::Enter { kind } => builder.start_node(kind),
        plc_rowan_parser::StrStep::Exit => builder.finish_node(),
        plc_rowan_parser::StrStep::Error { msg, pos } => {
            builder.error(msg.to_owned(), pos.try_into().unwrap())
        }
    });

    let (node, mut errors) = builder.finish_raw();
    for (i, err) in lexed.errors() {
        let text_range = lexed.text_range(i);
        let text_range = TextRange::new(
            text_range.start.try_into().unwrap(),
            text_range.end.try_into().unwrap(),
        );
        errors.push(SyntaxError::new(err, text_range))
    }

    (node, errors, is_eof)
}
