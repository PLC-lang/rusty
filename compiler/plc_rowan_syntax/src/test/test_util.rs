use plc_rowan_parser::Parser;

use crate::{Parse, parsing};


#[macro_export]
macro_rules! expect_all {
    ($($opt:expr),+ $(,)?) => {
        {
            let mut missing = Vec::new();
            $(
                if $opt.is_none() {
                    missing.push(stringify!($opt));
                }
            )+
            if !missing.is_empty() {
                panic!("expected Some but got None for: {:?}", missing);
            }
        }
    };
}


pub fn parse_generic<T>(text: &str, fun: fn(&mut Parser)) -> Parse<T> {
    let lexed = plc_rowan_parser::LexedStr::new(text);
    let parser_input = lexed.to_input();
    let parser_output = plc_rowan_parser::parse_event_list_generic(&parser_input, fun);
    let (node, errors, _eof) = parsing::build_tree(lexed, parser_output);
    Parse::new(node, errors)
}

