//! Doc-comment extraction shared by hover and completion.
//!
//! Both handlers take a declaration's `SourceLocation` and a
//! `SymbolKind`, then need the contiguous comment block above that
//! declaration as markdown. The lookup wraps `TokenCache` (per-file
//! cached `lex_with_trivia` output) and `DocumentStore` (in-memory
//! editor buffers, with disk fallback) so the call site doesn't have
//! to reason about source acquisition.

use std::path::Path;

use lsp_types::{Documentation, MarkupContent, MarkupKind};
use plc::lexer::Token;
use plc_source::source_location::SourceLocation;

use crate::document::DocumentStore;
use crate::position::SymbolKind;
use crate::token_cache::TokenCache;
use crate::token_walk;

/// Tag attached to a `CompletionItem.data` field so the lazy
/// `completionItem/resolve` handler can find the declaration without
/// re-running the enumerator. `kind` tells the resolver which Index
/// lookup table to use; `qualified_name` is the key.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ResolveTag {
    /// Index key: `"FOO"` for a global / POU / type, `"Container.member"`
    /// for a variable inside a container.
    pub qualified_name: String,
    /// Mirrors `position::SymbolKind` for routing the index lookup.
    pub kind: String,
}

impl ResolveTag {
    pub fn for_variable(qualified_name: impl Into<String>) -> Self {
        Self { qualified_name: qualified_name.into(), kind: "variable".into() }
    }
    pub fn for_member(qualified_name: impl Into<String>) -> Self {
        Self { qualified_name: qualified_name.into(), kind: "member".into() }
    }
    pub fn for_argument(qualified_name: impl Into<String>) -> Self {
        Self { qualified_name: qualified_name.into(), kind: "argument".into() }
    }
    pub fn for_pou(qualified_name: impl Into<String>) -> Self {
        Self { qualified_name: qualified_name.into(), kind: "pou".into() }
    }
    pub fn for_type(qualified_name: impl Into<String>) -> Self {
        Self { qualified_name: qualified_name.into(), kind: "type".into() }
    }

    pub fn parsed_kind(&self) -> Option<SymbolKind> {
        match self.kind.as_str() {
            "variable" => Some(SymbolKind::Variable),
            "member" => Some(SymbolKind::Member),
            "argument" => Some(SymbolKind::Argument),
            "pou" => Some(SymbolKind::Pou),
            "type" => Some(SymbolKind::Type),
            _ => None,
        }
    }
}

/// Resolve the declaration `SourceLocation` for a `ResolveTag` against
/// the project Index. Returns `None` when the named entity has been
/// removed since the completion list was built.
pub fn lookup_location(index: &plc::index::Index, tag: &ResolveTag) -> Option<SourceLocation> {
    let kind = tag.parsed_kind()?;
    match kind {
        SymbolKind::Variable | SymbolKind::Member | SymbolKind::Argument => {
            let entry = match tag.qualified_name.rsplit_once('.') {
                Some((container, name)) => index.find_member(container, name),
                None => index.find_global_variable(&tag.qualified_name),
            }?;
            Some(entry.source_location.clone())
        }
        SymbolKind::Pou => index.find_pou(&tag.qualified_name).map(|p| p.get_location().clone()),
        SymbolKind::Type => index
            .find_type(&tag.qualified_name)
            .or_else(|| index.find_pou_type(&tag.qualified_name))
            .map(|t| t.location.clone()),
    }
}

/// Build a markdown `Documentation` value from a doc body, suitable for
/// assignment to `CompletionItem.documentation`.
pub fn as_markdown_documentation(body: String) -> Documentation {
    Documentation::MarkupContent(MarkupContent { kind: MarkupKind::Markdown, value: body })
}

/// Fetch the doc body attached to `location` for a symbol of `kind`.
/// Returns `None` if the declaration's file can't be read, no leading
/// comments attach, or the location is undefined.
pub fn fetch(
    cache: &mut TokenCache,
    documents: &DocumentStore,
    location: &SourceLocation,
    kind: SymbolKind,
) -> Option<String> {
    let file_name = location.get_file_name()?;
    let path = Path::new(file_name);
    let range = location.to_range()?;
    let source = source_for(documents, path)?;
    let tokens = cache.get_or_recompute(path, &source);
    token_walk::docstring_at(tokens.as_slice(), &source, range.start, decl_prefix_for(kind))
}

/// Return source text for `path`. In-memory editor buffer first (so
/// unsaved edits are reflected), disk fallback otherwise.
pub fn source_for(documents: &DocumentStore, path: &Path) -> Option<String> {
    if let Some(uri) = crate::project::path_to_file_uri(path) {
        if let Some(buf) = documents.get(&uri) {
            return Some(buf.content.clone());
        }
    }
    std::fs::read_to_string(path).ok()
}

/// Closure picking out keyword / attribute tokens that syntactically
/// precede a declaration's name on the same line — `docstring_at`
/// widens its anchor past these to find the comment above. Includes
/// `{external}` / `{ref}` / `{constant}` / `{sized}` so oscat-style
/// stdlib declarations attach correctly.
pub fn decl_prefix_for(kind: SymbolKind) -> impl Fn(&Token) -> bool {
    move |tok: &Token| match kind {
        SymbolKind::Pou => matches!(
            tok,
            Token::KeywordFunction
                | Token::KeywordFunctionBlock
                | Token::KeywordProgram
                | Token::KeywordClass
                | Token::KeywordInterface
                | Token::KeywordMethod
                | Token::KeywordAction
                | Token::KeywordActions
                | Token::KeywordPropertyGet
                | Token::KeywordPropertySet
                | Token::PropertyExternal
                | Token::PropertyByRef
                | Token::PropertyConstant
                | Token::PropertySized
        ),
        SymbolKind::Type => matches!(
            tok,
            Token::KeywordType
                | Token::PropertyExternal
                | Token::PropertyByRef
                | Token::PropertyConstant
                | Token::PropertySized
        ),
        SymbolKind::Variable | SymbolKind::Argument | SymbolKind::Member => false,
    }
}
