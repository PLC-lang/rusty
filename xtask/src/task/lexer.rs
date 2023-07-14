use std::time::Instant;

use plc::{
    ast::SourceRangeFactory,
    lexer::{self, IdProvider},
};

use crate::reporter::DurationFormat;

use super::Task;

pub struct Lexer(pub &'static str);
impl Task for Lexer {
    fn get_name(&self) -> String {
        format!("lexer/{}", self.0)
    }

    fn execute(&self) -> anyhow::Result<std::time::Duration> {
        let content = std::fs::read_to_string(format!("./xtask/res/{}", self.0)).unwrap();
        let mut lexer = lexer::lex_with_ids(&content, IdProvider::default(), SourceRangeFactory::internal());

        let now = Instant::now();
        while !lexer.is_end_of_stream() {
            lexer.advance();
        }
        let elapsed = now.elapsed();

        assert_eq!(lexer.token, plc::lexer::Token::End);
        assert_eq!(lexer.last_range, 139145..139156);
        Ok(elapsed)
    }

    fn get_time_format(&self) -> crate::reporter::DurationFormat {
        DurationFormat::Micros
    }
}
