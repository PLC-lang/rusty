//! Temporal literal parsing on top of [`plc_literals`].
//!
//! The lexer has already matched the token shape, so these functions strip the type prefix, hand
//! the body to the shared parser and turn its errors into diagnostics.

use plc_ast::{
    ast::AstNode,
    literals::{AstLiteral, Time},
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_lexer::ParseSession;
use plc_literals::{CalendarError, DurationError, Leniency};
use plc_source::source_location::SourceLocation;

/// Returns the body of a temporal literal token after the `#`.
fn literal_body(token: &str) -> &str {
    &token[token.find('#').unwrap_or_default() + 1..]
}

fn report_calendar_error(
    lexer: &mut ParseSession,
    body: &str,
    error: CalendarError,
    location: &SourceLocation,
) {
    let message = match error {
        CalendarError::Overflow { start, end } => format!("Failed to parse number {}", &body[start..end]),
        CalendarError::MissingField | CalendarError::InvalidCharacter => {
            format!("Invalid date/time literal {body}")
        }
    };
    lexer.accept_diagnostic(Diagnostic::new(message).with_error_code("E011").with_location(location));
}

pub fn parse_literal_date_and_time(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    let token = lexer.slice_and_advance().to_string();
    let body = literal_body(&token);

    let date_time = match plc_literals::parse_date_and_time(body) {
        Ok(date_time) => date_time,
        Err(error) => {
            report_calendar_error(lexer, body, error, &location);
            return None;
        }
    };
    let plc_literals::DateAndTime { date, time } = date_time;
    Some(AstNode::new_literal(
        AstLiteral::new_date_and_time(
            date.year, date.month, date.day, time.hour, time.min, time.sec, time.nano,
        ),
        lexer.next_id(),
        location,
    ))
}

pub fn parse_literal_date(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    let token = lexer.slice_and_advance().to_string();
    let body = literal_body(&token);

    let date = match plc_literals::parse_date(body) {
        Ok(date) => date,
        Err(error) => {
            report_calendar_error(lexer, body, error, &location);
            return None;
        }
    };
    Some(AstNode::new_literal(
        AstLiteral::new_date(date.year, date.month, date.day),
        lexer.next_id(),
        location,
    ))
}

pub fn parse_literal_time_of_day(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    let token = lexer.slice_and_advance().to_string();
    let body = literal_body(&token);

    let time = match plc_literals::parse_time_of_day(body) {
        Ok(time) => time,
        Err(error) => {
            report_calendar_error(lexer, body, error, &location);
            return None;
        }
    };
    Some(AstNode::new_literal(
        AstLiteral::new_time_of_day(time.hour, time.min, time.sec, time.nano),
        lexer.next_id(),
        location,
    ))
}

pub fn parse_literal_time(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    let token = lexer.slice_and_advance().to_string();
    let body = literal_body(&token);

    let duration = match plc_literals::parse_duration(body, Leniency::STRICT) {
        Ok(duration) => duration,
        // a magnitude beyond u128 nanoseconds saturates, and the validator reports it as out of range
        Err(DurationError::Overflow) => {
            plc_literals::Duration { negative: body.starts_with('-'), nanos: u128::MAX }
        }
        Err(error) => {
            let message = match error {
                DurationError::SegmentOutOfOrder => {
                    "Invalid TIME Literal: segments out of order, use d-h-m-s-ms".to_string()
                }
                DurationError::DuplicateSegment => {
                    "Invalid TIME Literal: segments must be unique".to_string()
                }
                DurationError::UnknownUnit { start, end } => {
                    format!("Invalid TIME Literal: illegal unit '{}'", &body[start..end])
                }
                DurationError::MissingUnit => {
                    "Invalid TIME Literal: Missing unit (d|h|m|s|ms|us|ns)".to_string()
                }
                DurationError::Empty
                | DurationError::MissingDigits
                | DurationError::InvalidCharacter
                | DurationError::Overflow => "Invalid TIME Literal: Cannot parse segment.".to_string(),
            };
            lexer
                .accept_diagnostic(Diagnostic::new(message).with_error_code("E010").with_location(&location));
            return None;
        }
    };
    Some(AstNode::new_literal(
        AstLiteral::Time(Time { nanos: duration.nanos, negative: duration.negative }),
        lexer.next_id(),
        location,
    ))
}
