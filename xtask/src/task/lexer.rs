use std::time::Instant;

use plc::lexer;
use plc_ast::provider::IdProvider;
use plc_source::source_location::SourceLocationFactory;

use crate::reporter::DurationWrapper;

use super::Task;

pub struct Lexer(pub &'static str);
impl Task for Lexer {
    fn get_name(&self) -> String {
        format!("lexer/{}", self.0)
    }

    fn execute(&self) -> anyhow::Result<std::time::Duration> {
        let content = std::fs::read_to_string(format!("./xtask/res/{}", self.0)).unwrap();
        let mut lexer =
            lexer::lex_with_ids(&content, IdProvider::default(), SourceLocationFactory::internal(&content));

        let now = Instant::now();
        while !lexer.is_end_of_stream() {
            lexer.advance();
        }
        let elapsed = now.elapsed();

        debug_assert_eq!(lexer.token, plc::lexer::Token::End);
        debug_assert!(lexer.last_range.end >= 143100);
        Ok(elapsed)
    }

    fn get_wrapped(&self, duration: std::time::Duration) -> DurationWrapper {
        DurationWrapper::Micros(duration)
    }
}
