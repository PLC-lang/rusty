use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

pub struct SourceLocationFactory {
    file: Option<&'static str>,
}

impl SourceLocationFactory {
    /// constructs a SourceRangeFactory used for internally generated code (e.g. builtins)
    pub fn internal() -> Self {
        SourceLocationFactory { file: None }
    }

    /// constructs a SourceRangeFactory used to construct SourceRanes that point into the given file_name
    pub fn for_file(file_name: &'static str) -> Self {
        SourceLocationFactory { file: Some(file_name) }
    }

    /// creates a new SourceRange using the factory's file_name
    pub fn create_range(&self, range: core::ops::Range<usize>) -> SourceLocation {
        SourceLocation { span: CodeSpan::Range(range), file: self.file }
    }

    pub fn create_id_location(&self, id: usize) -> SourceLocation {
        SourceLocation { span: CodeSpan::Id(id), file: self.file }
    }

    pub fn create_file_only_location(&self) -> SourceLocation {
        SourceLocation { span: CodeSpan::None, file: self.file }
    }
}

/// Represents the location of a code element in a source code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodeSpan {
    /// The ID of the element in a sourcefile
    Id(usize),
    /// the start and end offset in the source-file
    Range(Range<usize>),
    /// The location does not point anywhere in the file
    None,
}
impl CodeSpan {
    pub fn get_end(&self) -> usize {
        match self {
            CodeSpan::Id(id) => *id,
            CodeSpan::Range(range) => range.end,
            CodeSpan::None => 0,
        }
    }

    pub fn get_start(&self) -> usize {
        match self {
            CodeSpan::Id(id) => *id,
            CodeSpan::Range(range) => range.start,
            CodeSpan::None => 0,
        }
    }

    pub fn to_range(&self) -> Option<Range<usize>> {
        match self {
            CodeSpan::Range(range) => Some(range.clone()),
            CodeSpan::Id(_) | CodeSpan::None => None,
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

impl Debug for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("SourceLocation");
        f.field("span", &self.span);
        if self.file.is_some() {
            f.field("file", &self.file);
        }
        f.finish()
    }
}

impl SourceLocation {
    /// Constructs a new SourceRange with the given range and filename
    pub fn in_file_ranged(range: core::ops::Range<usize>, file_name: &'static str) -> SourceLocation {
        SourceLocation { span: CodeSpan::Range(range), file: Some(file_name) }
    }

    /// Constructs a new SourceRange without the file_name attribute
    pub fn without_file_ranged(range: core::ops::Range<usize>) -> SourceLocation {
        SourceLocation { span: CodeSpan::Range(range), file: None }
    }

    /// Constructs an undefined SourceRange with a 0..0 range and no filename
    pub fn undefined() -> SourceLocation {
        SourceLocation { span: CodeSpan::None, file: None }
    }

    /// returns the start-offset of this source-range if it is a range
    /// If the location is an id, returns the ID
    pub fn get_start(&self) -> usize {
        self.span.get_start()
    }

    /// returns the end-offset of this source-location if it is a range
    /// If the location is an id, returns the ID
    pub fn get_end(&self) -> usize {
        self.span.get_end()
    }

    /// returns a new SourceRange that spans `this` and the `other` range.
    /// In other words this results in `self.start .. other.end`
    pub fn span(&self, other: &SourceLocation) -> SourceLocation {
        let range = match (&self.span, &other.span) {
            (CodeSpan::Id(start), CodeSpan::Id(end)) => Some(*start..*end),
            (CodeSpan::Id(start), CodeSpan::Range(range)) => Some(*start..range.end),
            (CodeSpan::Id(id), CodeSpan::None) | (CodeSpan::None, CodeSpan::Id(id)) => Some(*id..*id),
            (CodeSpan::Range(range), CodeSpan::Id(end)) => Some(range.start..*end),
            (CodeSpan::Range(start), CodeSpan::Range(end)) => Some(start.start..end.end),
            (CodeSpan::Range(range), CodeSpan::None) | (CodeSpan::None, CodeSpan::Range(range)) => {
                Some(range.clone())
            }
            (CodeSpan::None, CodeSpan::None) => None,
        };
        let span = range.map(CodeSpan::Range).unwrap_or(CodeSpan::None);
        SourceLocation { span, file: self.get_file_name() }
    }

    /// converts this SourceRange into a Range
    pub fn to_range(&self) -> Option<Range<usize>> {
        self.span.to_range()
    }

    pub fn get_file_name(&self) -> Option<&'static str> {
        self.file
    }

    /// returns true if this SourceRange points to an undefined location.
    /// see `SourceRange::undefined()`
    pub fn is_undefined(&self) -> bool {
        self.file.is_none() && self.span == CodeSpan::None
    }

    pub fn get_span(&self) -> &CodeSpan {
        &self.span
    }
}

impl From<std::ops::Range<usize>> for SourceLocation {
    fn from(range: std::ops::Range<usize>) -> SourceLocation {
        SourceLocation::without_file_ranged(range)
    }
}
