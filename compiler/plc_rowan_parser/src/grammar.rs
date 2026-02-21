/// the actual parsing
///
/// this module contains general purpose parsing to emit events
/// it also contains the entry-point for the grammar, which is the `compilation_unit` function
///
/// the more specific parsing for the grammar is in the `grammar` sub-module, which contains functions for each rule in the grammar
pub mod pou_grammar;
pub mod var_decl_grammar;
pub mod statement_grammar;

use crate::SyntaxKind::{self, *};
use crate::T;
use crate::parser::Parser;

pub fn compilation_unit(p: &mut Parser) {
    let m = p.start();

    while (p.at(POU_START_KEYWORD_KW)) {
        pou_grammar::pou(p);
    }

    m.complete(p, crate::SyntaxKind::COMPILATION_UNIT);
}

// general purpose parsing rules
pub(crate) fn name(p: &mut Parser) {
    let m = p.start();
    p.expect(IDENT);
    m.complete(p, NAME);
}

pub(crate) fn name_ref(p: &mut Parser) {
    let m = p.start();
    p.expect(IDENT);
    m.complete(p, NAME_REF);
}

#[cfg(test)]
mod tests {

    use std::io::BufWriter;
    use std::io::Write;
    use std::ops::Range;

    use crate::event;
    use crate::lexed_str::LexedStr;
    use crate::parser::{Parser, parse};
    use crate::{Input, Output, Step};

    /// Helper function to parse input string and return the syntax tree output
    pub(crate) fn parse_with(input: &LexedStr, entry_point: fn(&mut Parser)) -> Output {
        // Parse the input
        let events = parse(&input.to_input(), entry_point);

        // Convert events to output
        event::process(events)
    }

    /// Format the output as a debug tree string
    pub(crate) fn format_tree(output: &Output, input_src: &LexedStr) -> String {

        let mut buf = BufWriter::new(Vec::new());
        let mut errors = Vec::new();
        let mut indent = String::new();
        let mut depth = 0;
        let mut len = 0;
        let mut i = 0;

        for step in output.iter() {
            match step {
                Step::Token { kind, .. } => {
                    assert!(depth > 0);
                    let text = input_src.text(i);
                    len += text.len();
                    writeln!(buf, "{indent}{kind:?} {text:?}").unwrap();
                    i += 1;
                }
                Step::Enter { kind } => {
                    assert!(depth > 0 || len == 0);
                    depth += 1;
                    writeln!(buf, "{indent}{kind:?}").unwrap();
                    indent.push_str("  ");
                }
                Step::Exit => {
                    assert!(depth > 0);
                    depth -= 1;
                    indent.pop();
                    indent.pop();
                }

                Step::Error { msg } => {
                    assert!(depth > 0);
                    let Range { start, end } = input_src.text_range(i);
                    let text = input_src.text(i);
                    errors.push(format!("error {start}-{end}: {msg}: near {text}\n"));
                }
                _ => unreachable!(),
            }
        }
        // let text = input_src.as_str();
        // assert_eq!(
        //     len,
        //     text.len(),
        //     "didn't parse all text.\nParsed:\n{}\n\nAll:\n{}\n",
        //     &text[..len],
        //     text
        // );

        // for (token, msg) in lexed.errors() {
        //     let pos = lexed.text_start(token);
        //     errors.push(format!("error {pos}: {msg}\n"));
        // }

        let has_errors = !errors.is_empty();

        if has_errors {
            writeln!(buf, "").unwrap();
            writeln!(buf, "Errors:").unwrap();
            for e in errors {
                writeln!(buf, "  {e}").unwrap();
            }
        }

        let formatted_tree = String::from_utf8(buf.into_inner().unwrap()).unwrap();
        formatted_tree
    }
}
