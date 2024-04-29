use crate::lib_sourcelocation::{SourceCode, SourceContainer};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

#[derive(Clone, Default)]
pub struct SourceLocationFactory {
    file: Option<&'static str>,
    newlines: NewLines,
}

impl SourceLocationFactory {
    /// constructs a SourceRangeFactory used for internally generated code (e.g. builtins)
    pub fn internal(src: &str) -> Self {
        SourceLocationFactory { file: None, newlines: NewLines::build(src) }
    }

    /// constructs a SourceRangeFactory used to construct SourceRanes that point into the given source
    pub fn for_source(source_code: &SourceCode) -> Self {
        SourceLocationFactory {
            file: Some(source_code.get_location_str()),
            newlines: NewLines::build(&source_code.source),
        }
    }

    /// creates a new SourceRange using the factory's file_name
    pub fn create_range(&self, range: core::ops::Range<usize>) -> SourceLocation {
        let start = TextLocation::from_offset(range.start, &self.newlines);
        let end = TextLocation::from_offset(range.end, &self.newlines);
        SourceLocation { span: CodeSpan::Range(start..end), file: self.file }
    }

    pub fn create_block_location(&self, local_id: usize, execution_order: Option<usize>) -> SourceLocation {
        SourceLocation {
            span: CodeSpan::Block { local_id, execution_order, inner_range: None },
            file: self.file,
        }
    }

    pub fn create_file_only_location(&self) -> SourceLocation {
        SourceLocation { span: CodeSpan::None, file: self.file }
    }

    pub fn create_range_to_end_of_line(&self, line: usize, column: usize) -> SourceLocation {
        let start = TextLocation::new(line, column, 0);
        let end = TextLocation::new(line, self.newlines.get_end_of_line(line), 0);
        SourceLocation { span: CodeSpan::Range(start..end), file: self.file }
    }
}

/// The location of a certain element in a text file

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextLocation {
    /// Line in the source code where this location points to
    line: usize,
    /// Column in the sourcecode where this location points to
    column: usize,
    /// Raw offset to this location from the start of the file
    offset: usize,
}

impl TextLocation {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        TextLocation { line, column, offset }
    }

    pub fn from_offset(offset: usize, newlines: &NewLines) -> Self {
        let line = newlines.get_line_nr(offset);
        let column = newlines.get_column(line, offset);
        TextLocation { line, column, offset }
    }
}

/// Represents the location of a code element in a source code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodeSpan {
    /// The location of a block in a diagram
    Block { local_id: usize, execution_order: Option<usize>, inner_range: Option<Range<usize>> },
    /// An element spanning multiple IDs
    Combined(Vec<CodeSpan>),
    /// A location inside a text
    Range(Range<TextLocation>),
    /// The location does not point anywhere in the file
    None,
}

impl Display for CodeSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeSpan::Block { .. } => write!(f, "Block {}", self.get_line()),
            CodeSpan::Combined(spans) => {
                write!(f, "{}", spans.iter().map(|it| it.to_string()).collect::<String>())
            }
            CodeSpan::Range(range) => write!(
                f,
                "{}:{}:{{{}:{}-{}:{}}}: ",
                range.start.line,
                range.start.column,
                range.start.line,
                range.start.column,
                range.end.line,
                range.end.column,
            ),
            CodeSpan::None => Ok(()),
        }
    }
}

impl CodeSpan {
    /// Creates a codespan from line, column and range info
    pub fn from_text_info(start: TextLocation, end: TextLocation) -> Self {
        CodeSpan::Range(start..end)
    }

    /// Gets the line representation for a source location
    /// If the location does not represent a line, the closest equivalent is returned
    // That is 0 for None and the ID for id/inner spans
    pub fn get_line(&self) -> usize {
        match self {
            Self::Range(range) => range.start.line,
            Self::Block { local_id, .. } => *local_id,
            _ => 0,
        }
    }

    pub fn get_line_plus_one(&self) -> usize {
        match self {
            Self::Range(range) => range.start.line + 1,
            Self::Block { local_id, .. } => *local_id,
            _ => 0,
        }
    }

    /// Gets the colmumn representation for a source location
    /// If the location does not represent a line, 0 is returned
    pub fn get_column(&self) -> usize {
        match self {
            Self::Range(range) => range.start.column,
            _ => 0,
        }
    }

    pub fn to_range(&self) -> Option<Range<usize>> {
        match self {
            CodeSpan::Range(range) => Some(range.start.offset..range.end.offset),
            CodeSpan::Block { inner_range, .. } => inner_range.clone(),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    span: CodeSpan,
    /// the name of the file if available. if there is no file available
    /// the source is probably internally generated by the compiler. (e.g.
    /// a automatically generated data_type)
    file: Option<&'static str>,
}

impl From<&SourceLocation> for SourceLocation {
    fn from(value: &SourceLocation) -> Self {
        value.clone()
    }
}

impl Debug for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("SourceLocation");
        f.field("span", &self.span);
        if self.file.filter(|name| *name != "<internal>").is_some() {
            f.field("file", &self.file);
        }
        f.finish()
    }
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.file.filter(|name| *name != "<internal>").is_some() {
            write!(f, "{}", self.get_file_name().unwrap())?;
        }
        write!(f, ":{}", self.span)
    }
}

impl SourceLocation {
    /// Constructs an undefined SourceRange with a 0..0 range and no filename
    pub fn undefined() -> SourceLocation {
        SourceLocation { span: CodeSpan::None, file: None }
    }

    /// Gets the line representation for a source location
    /// If the location does not represent a line, the closest equivalent is returned
    /// That is 0 for None and the ID for id/inner spans
    pub fn get_line(&self) -> usize {
        self.span.get_line()
    }

    /// Same as [`get_line`] but adds one to the line number if its of type [`CodeSpan::Range`].
    pub fn get_line_plus_one(&self) -> usize {
        self.span.get_line_plus_one()
    }

    /// Gets the colmumn representation for a source location
    /// If the location does not represent a line, 0 is returned
    pub fn get_column(&self) -> usize {
        self.span.get_column()
    }

    /// returns a new SourceRange that spans `this` and the `other` range.
    /// In other words this results in `self.start .. other.end`
    pub fn span(&self, other: &SourceLocation) -> SourceLocation {
        let span = match (&self.span, &other.span) {
            //ID -> ID = Combile
            (
                CodeSpan::Block { local_id, execution_order, inner_range },
                CodeSpan::Block { local_id: other, inner_range: other_range, .. },
            ) if local_id == other => {
                let inner_range = match (inner_range, other_range) {
                    (None, None) => None,
                    (Some(range), None) | (None, Some(range)) => Some(range.clone()),
                    (Some(start), Some(end)) => Some(start.start..end.end),
                };

                CodeSpan::Block { local_id: *local_id, execution_order: *execution_order, inner_range }
            }
            (CodeSpan::Block { .. }, CodeSpan::Block { .. }) => {
                CodeSpan::Combined(vec![self.span.clone(), other.span.clone()])
            }
            //Range -> Range = Range
            (CodeSpan::Range(start), CodeSpan::Range(end)) => CodeSpan::Range(start.start..end.end),
            //ID -> Range = InnerRange
            (CodeSpan::Block { local_id, execution_order, inner_range }, CodeSpan::Range(range))
            | (CodeSpan::Range(range), CodeSpan::Block { local_id, execution_order, inner_range }) => {
                CodeSpan::Block {
                    local_id: *local_id,
                    execution_order: *execution_order,
                    inner_range: inner_range
                        .as_ref()
                        .map(|it| it.start..range.end.offset)
                        .or_else(|| Some(range.start.offset..range.end.offset)),
                }
            }
            //None any -> None (unsupported)
            (CodeSpan::None, _) | (_, CodeSpan::None) => CodeSpan::None,
            (CodeSpan::Combined(inner), CodeSpan::Combined(other)) => {
                let mut inner = inner.clone();
                inner.extend_from_slice(other);
                CodeSpan::Combined(inner)
            }
            (CodeSpan::Combined(data), other) | (other, CodeSpan::Combined(data)) => {
                let mut data = data.clone();
                data.push(other.clone());
                CodeSpan::Combined(data)
            }
        };
        SourceLocation { span, file: self.get_file_name() }
    }

    /// converts this SourceRange into a Range
    pub fn to_range(&self) -> Option<Range<usize>> {
        self.span.to_range()
    }

    pub fn get_file_name(&self) -> Option<&'static str> {
        self.file.filter(|it| *it != "<internal>")
    }

    /// returns true if this SourceRange points to an undefined location.
    /// see `SourceRange::undefined()`
    pub fn is_undefined(&self) -> bool {
        self.file.is_none() && self.span == CodeSpan::None
    }

    pub fn get_span(&self) -> &CodeSpan {
        &self.span
    }

    pub fn replace_with(&mut self, new_location: SourceLocation) {
        self.span = new_location.span;
        self.file = new_location.file;
    }
    /// creates a SymbolLocation with undefined source_range used for
    /// symbols that are created by the compiler on-the-fly.
    pub fn internal() -> Self {
        Self::undefined()
    }

    pub fn is_internal(&self) -> bool {
        self.is_undefined()
    }

    pub fn is_in_unit(&self, unit: impl AsRef<str>) -> bool {
        if let Some(filename) = self.get_file_name() {
            filename == unit.as_ref()
        } else {
            //Fallback, if no file is defined all files are local
            true
        }
    }
}

/**
 * A datastructure that stores the location of newline characters of a string.
 * It also offers some useful methods to determine the line-number of an offset-location.
 */
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct NewLines {
    line_breaks: Vec<usize>,
}

impl NewLines {
    pub fn build(str: &str) -> NewLines {
        let mut line_breaks = Vec::new();
        let mut total_offset: usize = 0;
        if !str.is_empty() {
            // Instead of using ´lines()´ we split at \n to preserve the offsets if a \r exists
            for l in str.split('\n') {
                total_offset += l.len() + 1;
                line_breaks.push(total_offset);
            }
        }
        NewLines { line_breaks }
    }

    ///
    /// returns the 0 based line-nr of the given offset-location
    ///
    pub fn get_line_nr(&self, offset: usize) -> usize {
        match self.line_breaks.binary_search(&offset) {
            //In case we hit an exact match, we just found the first character of a new line, we must add one to the result
            Ok(line) => line + 1,
            Err(line) => line,
        }
    }

    ///
    /// returns the 0 based column of the given offset-location
    ///
    pub fn get_column(&self, line: usize, offset: usize) -> usize {
        if line > 0 {
            self.line_breaks.get(line - 1).map(|l| offset - *l).unwrap_or(0)
        } else {
            offset
        }
    }

    ///
    /// returns the 0 based column of end-of-line character for the given line
    pub fn get_end_of_line(&self, line: usize) -> usize {
        self.line_breaks.get(line).copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::source_location::{NewLines, TextLocation};
    use insta::assert_debug_snapshot;

    use super::{CodeSpan, SourceLocation};

    #[test]
    fn new_lines_test_empty_string() {
        let text = "";
        let nl = NewLines::build(text);

        assert_eq!(nl.get_line_nr(0), 0);
        assert_eq!(nl.get_line_nr(1), 0);
        assert_eq!(nl.get_line_nr(2), 0);
        assert_eq!(nl.get_line_nr(3), 0);
    }

    #[test]
    fn new_lines_test_three_lines_with_crlf() {
        let text = "A\r\nB\r\nC";
        let nl = NewLines::build(text);
        assert_eq!(nl.get_line_nr(text.find('A').unwrap()), 0);
        assert_eq!(nl.get_line_nr(text.find('B').unwrap()), 1);
        assert_eq!(nl.get_line_nr(text.find('C').unwrap()), 2);
    }

    #[test]
    fn new_lines_test_three_lines_with_lf() {
        let text = "A\nB\nC";
        let nl = NewLines::build(text);
        assert_eq!(nl.get_line_nr(text.find('A').unwrap()), 0);
        assert_eq!(nl.get_line_nr(text.find('B').unwrap()), 1);
        assert_eq!(nl.get_line_nr(text.find('C').unwrap()), 2);
    }

    #[test]
    fn new_lines_test_three_long_lines_with_lf() {
        let text = "xxxx A xxxx

    xxxx B xxxx

    xxxx C xxxxx";
        let nl = NewLines::build(text);
        assert_eq!(nl.get_line_nr(text.find('A').unwrap()), 0);
        assert_eq!(nl.get_line_nr(text.find('B').unwrap()), 2);
        assert_eq!(nl.get_line_nr(text.find('C').unwrap()), 4);
    }

    #[test]
    fn new_lines_and_columns_test() {
        let text = "xxxx A xxxx

    xxxx B xxxx

    xxxx C xxxxx";
        let nl = NewLines::build(text);
        assert_eq!(nl.get_column(0, text.find('A').unwrap()), 5);
        assert_eq!(nl.get_column(2, text.find('B').unwrap()), 9);
        assert_eq!(nl.get_column(4, text.find('C').unwrap()), 9);
    }

    #[test]
    fn span_two_blocks() {
        let loc1 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 1, execution_order: Some(1), inner_range: None },
        };
        let loc2 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 2, execution_order: Some(1), inner_range: None },
        };
        assert_debug_snapshot!(loc1.span(&loc2));
    }

    #[test]
    fn span_two_blocks_with_same_id() {
        let loc1 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 1, execution_order: Some(1), inner_range: None },
        };
        let loc2 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 1, execution_order: Some(1), inner_range: None },
        };
        assert_debug_snapshot!(loc1.span(&loc2));
    }

    #[test]
    fn span_id_and_range() {
        let loc1 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 1, execution_order: Some(1), inner_range: None },
        };
        let loc2 = SourceLocation {
            file: None,
            span: CodeSpan::from_text_info(TextLocation::new(1, 0, 0), TextLocation::new(2, 0, 4)),
        };
        assert_debug_snapshot!(loc1.span(&loc2));
    }

    #[test]
    fn span_two_ranges() {
        let loc1 = SourceLocation {
            file: None,
            span: CodeSpan::from_text_info(TextLocation::new(1, 0, 0), TextLocation::new(2, 0, 4)),
        };
        let loc2 = SourceLocation {
            file: None,
            span: CodeSpan::from_text_info(TextLocation::new(1, 0, 0), TextLocation::new(5, 30, 10)),
        };
        assert_debug_snapshot!(loc1.span(&loc2));
    }

    #[test]
    fn span_combined() {
        let loc1 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 1, execution_order: Some(1), inner_range: None },
        };
        let loc2 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 2, execution_order: Some(2), inner_range: None },
        };
        let loc3 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 3, execution_order: Some(3), inner_range: None },
        };
        let loc4 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 4, execution_order: Some(4), inner_range: None },
        };
        assert_debug_snapshot!(loc1.span(&loc2).span(&loc3.span(&loc4)));
    }

    #[test]
    fn span_none() {
        let loc1 = SourceLocation {
            file: None,
            span: CodeSpan::Block { local_id: 1, execution_order: Some(1), inner_range: None },
        };
        let loc2 = SourceLocation { file: None, span: CodeSpan::None };
        assert_debug_snapshot!(loc1.span(&loc2));
    }
}
