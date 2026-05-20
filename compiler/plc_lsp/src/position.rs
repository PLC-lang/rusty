//! Position → AST-node lookup for hover, goto-def, and references.
//!
//! Translates an LSP `(uri, position)` into a [`SymbolUnderCursor`] by:
//! 1. converting the LSP position into a `(path, byte_offset)` against
//!    the source content the worker captured during its last compile;
//! 2. walking the matching `CompilationUnit` via the existing
//!    [`plc_ast::visitor::AstVisitor`] to find the deepest `AstNode`
//!    whose `SourceLocation` span contains the byte offset;
//! 3. reading the annotator's [`StatementAnnotation`] for that node and
//!    translating it into a [`ResolvedSymbol`] via [`Index`] lookups.
//!
//! Design rationale (cf. `.baseline/lsp-plan-phases-7-10.md` §2.2, §2.4):
//!
//! - **No annotator extension.** Every context-sensitive form (member
//!   access, named call args, struct-literal initializers) resolves
//!   either from the deepest node's direct annotation or via an
//!   *ancestor* node's annotation that names the container — the field
//!   name then comes from the source slice. The position walker tracks
//!   the ancestor chain so the fallback has somewhere to look.
//!
//! - **Display vs identity (§3.5).** For display strings (`usage_text`,
//!   `qualified_name`) we slice the user's source text via
//!   [`GlobalContext::slice`]. For identity (`declaration_location`,
//!   reverse-index keys) we use `SourceLocation` directly. This defuses
//!   the preprocessor-renamed (`__INT_LITERAL_TYPE`) and synthesised-
//!   method (`PROPERTY foo` → `__get_foo`) classes of bugs uniformly.
//!
//! - **Three capture sites.** The walker catches three kinds of click:
//!   (1) `AstNode` hits for body expressions; (2) type-reference hits
//!   for `DataTypeDeclaration::Reference` (`x: myType`); (3)
//!   declaration-name hits for variable / POU / type names that aren't
//!   themselves `AstNode`s. The tightest hit wins.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use lsp_types::{Position, PositionEncodingKind, Uri};
use plc::index::Index;
use plc::resolver::{AnnotationMap, AstAnnotations, StatementAnnotation};
use plc_ast::ast::{AstId, AstNode, CompilationUnit, DataTypeDeclaration, Pou, Variable};
use plc_ast::visitor::{AstVisitor, Walker};
use plc_driver::pipelines::AnnotatedProject;
use plc_index::GlobalContext;
use plc_source::source_location::SourceLocation;

/// What the position lookup found at the cursor.
///
/// `usage_location` and `usage_text` are always populated when the
/// walker found *any* identifier under the cursor. `resolved` is `None`
/// when the annotator left the node unbound (typo, missing import,
/// mid-edit broken code) — handlers degrade gracefully (no hover, no
/// goto target).
pub struct SymbolUnderCursor {
    /// Literal byte range of the identifier under the cursor — the `c`
    /// in `a.b.c`, the `x` in `foo(x := 1)`.
    pub usage_location: SourceLocation,
    /// Exact source bytes under the cursor. Used by hover when the
    /// resolved declaration has a different name (preprocessor-renamed
    /// types, synthesised methods).
    pub usage_text: String,
    /// What the annotator resolved this identifier to.
    pub resolved: Option<ResolvedSymbol>,
}

/// The declaration the cursor points at, plus enough context for
/// hover / outline kind mapping.
pub struct ResolvedSymbol {
    /// Source of truth for goto-def and reverse-index lookups.
    pub declaration_location: SourceLocation,
    /// Drives hover format choice and outline `SymbolKind` mapping.
    pub kind: SymbolKind,
    /// Display-only: `"OtherFB.c"`, `"MyFB.in"`, `"MAIN.local_x"`,
    /// built from the resolution chain.
    pub qualified_name: String,
}

/// Classification of a resolved symbol — drives per-kind hover format
/// and the outline's `lsp_types::SymbolKind` mapping. Local to the LSP
/// crate (no compiler-side change).
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Pou,
    Variable,
    Type,
    /// Struct field or FB VAR encountered through a member access.
    Member,
    /// Function/program argument (named call parameter).
    Argument,
}

/// Entry point. Returns `None` when no identifier sits under the cursor
/// (whitespace, comment, EOF) or when the URI doesn't belong to a unit
/// in the last successfully attached `AnnotatedProject`.
///
/// `source_contents` provides the per-file source text the worker
/// captured for utf-16 column conversion (cf. phase 4 / D4); we reuse
/// it to map the LSP position into a byte offset.
pub fn symbol_under_cursor(
    annotated: &AnnotatedProject,
    ctxt: &GlobalContext,
    uri: &Uri,
    position: Position,
    encoding: &PositionEncodingKind,
    source_contents: &HashMap<String, String>,
) -> Option<SymbolUnderCursor> {
    let (path, byte_offset) = position_to_byte_offset(uri, position, encoding, source_contents)?;
    let unit = find_unit(annotated, &path)?;

    let mut walker = PositionWalker {
        target_offset: byte_offset,
        deepest: None,
        type_ref_hit: None,
        decl_name_hit: None,
        ancestor_stack: Vec::new(),
        container_stack: Vec::new(),
    };
    unit.walk(&mut walker);

    // Tightest hit wins across the three capture sites.
    let ast_size = walker.deepest.as_ref().map(|d| d.span_size);
    let type_size = walker.type_ref_hit.as_ref().map(|t| t.span_size);
    let decl_size = walker.decl_name_hit.as_ref().map(|d| d.span_size());
    let tightest = [ast_size, type_size, decl_size].into_iter().flatten().min();

    // Declaration name hit (`a` in `VAR a : INT`, the POU name, a TYPE
    // name): synthesised from non-`AstNode` fields. Take it when it's
    // the tightest match — it always represents a true identifier
    // leaf, never a wrapper.
    if let Some(decl) = walker.decl_name_hit {
        if tightest == Some(decl.span_size()) {
            return Some(decl.into_symbol(annotated));
        }
    }

    // Type references in declarations (`x: myType`, function return
    // types, …) aren't reachable through `AstNode` traversal — the
    // referenced type name lives on `DataTypeDeclaration::Reference`
    // as a plain `String` + `SourceLocation`.
    if let Some(t) = walker.type_ref_hit {
        if tightest == Some(t.span_size) {
            let usage_text = ctxt.slice(&t.location);
            let resolved = annotated
                .index
                .find_type(&t.referenced_type)
                .or_else(|| annotated.index.find_pou_type(&t.referenced_type))
                .map(|ty| ResolvedSymbol {
                    declaration_location: ty.location.clone(),
                    kind: SymbolKind::Type,
                    qualified_name: t.referenced_type.clone(),
                });
            return Some(SymbolUnderCursor { usage_location: t.location, usage_text, resolved });
        }
    }

    let hit = walker.deepest?;
    let usage_text = ctxt.slice(&hit.location);
    let resolved = resolve_at(hit.id, &usage_text, &hit.ancestor_ids, annotated);
    Some(SymbolUnderCursor { usage_location: hit.location, usage_text, resolved })
}

/// Resolve a node ID to a `ResolvedSymbol`. Shared between the cursor-
/// lookup walker and the references reverse-index walker.
///
/// 1. Try the direct annotation on `node_id` — handles plain
///    identifiers, member access, function calls, type refs.
/// 2. Fall back to ancestor-context: walk outward looking for an
///    ancestor whose type hint names the container, then resolve the
///    member from the cursor's source slice. Handles named call args
///    (`foo(x := 1)`) and struct-literal initializer fields
///    (`(field := 1)`).
pub(crate) fn resolve_at(
    node_id: AstId,
    usage_text: &str,
    ancestor_ids: &[AstId],
    annotated: &AnnotatedProject,
) -> Option<ResolvedSymbol> {
    if let Some(annot) = annotated.annotations.get_with_id(node_id) {
        if let Some(resolved) = translate(annot, &annotated.index) {
            return Some(resolved);
        }
    }
    resolve_via_ancestor_chain(ancestor_ids, usage_text, annotated)
}

/// Find the `CompilationUnit` whose file matches the cursor's path.
/// Compares the unit's `file: FileMarker` name against the canonicalised
/// path; we use a string-suffix match because plc.json paths can be
/// stored with different leading-segment conventions than the LSP URI's
/// canonical form.
fn find_unit<'a>(annotated: &'a AnnotatedProject, path: &Path) -> Option<&'a CompilationUnit> {
    let needle = path.to_string_lossy();
    annotated.units.iter().map(|au| au.get_unit()).find(|unit| {
        unit.file.get_name().map(|file| file == needle.as_ref() || needle.ends_with(file)).unwrap_or(false)
    })
}

/// Tracks the tightest hit the cursor sits inside, across three
/// capture sites:
///
/// 1. **Deepest `AstNode`** — body expressions, member access, call
///    operators, etc. Resolved via [`StatementAnnotation`] lookups.
/// 2. **Type references** (`x: myType`) — `String + SourceLocation`
///    on `DataTypeDeclaration::Reference`, captured via the
///    `visit_data_type_declaration` override.
/// 3. **Declaration names** (the `a` in `VAR a : INT`, a POU name, a
///    TYPE name) — `String + SourceLocation` on `Pou` / `Variable` /
///    `DataType`, captured via the corresponding visit overrides.
///    Without these, find-references on a declaration silently fails
///    because there's no `AstNode` under the cursor for the walker to
///    record.
struct PositionWalker {
    target_offset: usize,
    deepest: Option<DeepestHit>,
    type_ref_hit: Option<TypeRefHit>,
    decl_name_hit: Option<DeclNameHit>,
    ancestor_stack: Vec<AstId>,
    /// Current container path (POU / type name), pushed when we descend
    /// into a `Pou` or `UserTypeDeclaration` and popped on the way out.
    /// Used to build qualified names for declaration-name hits without
    /// needing a separate scope-resolution pass.
    container_stack: Vec<String>,
}

struct DeepestHit {
    id: AstId,
    location: SourceLocation,
    span_size: usize,
    /// AstIds of containing nodes (outermost-first). Mirrors the
    /// recursion stack at the time of the hit.
    ancestor_ids: Vec<AstId>,
}

struct TypeRefHit {
    referenced_type: String,
    location: SourceLocation,
    span_size: usize,
}

enum DeclNameHit {
    Pou { name: String, location: SourceLocation, span_size: usize },
    Variable { container: Option<String>, name: String, location: SourceLocation, span_size: usize },
    Type { name: String, location: SourceLocation, span_size: usize },
}

impl DeclNameHit {
    fn span_size(&self) -> usize {
        match self {
            DeclNameHit::Pou { span_size, .. }
            | DeclNameHit::Variable { span_size, .. }
            | DeclNameHit::Type { span_size, .. } => *span_size,
        }
    }

    fn into_symbol(self, annotated: &AnnotatedProject) -> SymbolUnderCursor {
        match self {
            DeclNameHit::Pou { name, location, .. } => {
                let usage_text = name.clone();
                let resolved = annotated.index.find_pou(&name).map(|pou| ResolvedSymbol {
                    declaration_location: pou.get_location().clone(),
                    kind: SymbolKind::Pou,
                    qualified_name: name.clone(),
                });
                SymbolUnderCursor { usage_location: location, usage_text, resolved }
            }
            DeclNameHit::Variable { container, name, location, .. } => {
                let usage_text = name.clone();
                let entry = match &container {
                    Some(c) => annotated.index.find_member(c, &name),
                    None => annotated.index.find_global_variable(&name),
                };
                let qualified_name = match &container {
                    Some(c) => format!("{c}.{name}"),
                    None => name.clone(),
                };
                let resolved = entry.map(|e| ResolvedSymbol {
                    declaration_location: e.source_location.clone(),
                    // Members of types render without a section header
                    // (see `hover_format`); locals get the section.
                    kind: if container.as_deref().and_then(|c| annotated.index.find_type(c)).is_some() {
                        SymbolKind::Member
                    } else {
                        SymbolKind::Variable
                    },
                    qualified_name,
                });
                SymbolUnderCursor { usage_location: location, usage_text, resolved }
            }
            DeclNameHit::Type { name, location, .. } => {
                let usage_text = name.clone();
                let resolved =
                    annotated.index.find_type(&name).or_else(|| annotated.index.find_pou_type(&name)).map(
                        |ty| ResolvedSymbol {
                            declaration_location: ty.location.clone(),
                            kind: SymbolKind::Type,
                            qualified_name: name.clone(),
                        },
                    );
                SymbolUnderCursor { usage_location: location, usage_text, resolved }
            }
        }
    }
}

impl PositionWalker {
    fn record_decl_name(&mut self, hit: DeclNameHit) {
        if self.decl_name_hit.as_ref().is_none_or(|d| hit.span_size() <= d.span_size()) {
            self.decl_name_hit = Some(hit);
        }
    }

    fn loc_contains(&self, loc: &SourceLocation) -> Option<usize> {
        let range = loc.to_range()?;
        if range.start <= self.target_offset && self.target_offset < range.end {
            Some(range.end - range.start)
        } else {
            None
        }
    }
}

impl AstVisitor for PositionWalker {
    fn visit_pou(&mut self, pou: &Pou) {
        if let Some(size) = self.loc_contains(&pou.name_location) {
            self.record_decl_name(DeclNameHit::Pou {
                name: pou.name.clone(),
                location: pou.name_location.clone(),
                span_size: size,
            });
        }
        self.container_stack.push(pou.name.clone());
        Walker::walk(pou, self);
        self.container_stack.pop();
    }

    fn visit_user_type_declaration(&mut self, ut: &plc_ast::ast::UserTypeDeclaration) {
        // The type's name lives on `DataType::*Type { name, .. }`. Each
        // variant carries it; `DataType::get_name` flattens.
        if let Some(name) = ut.data_type.get_name() {
            // The data_type doesn't expose a name location separately;
            // use its overall location as the click target. This is
            // wider than ideal (covers the whole inline definition)
            // but the tightest-wins arbitration in `symbol_under_cursor`
            // keeps a nested AstNode hit preferred when present.
            if let Some(size) = self.loc_contains(&ut.location) {
                self.record_decl_name(DeclNameHit::Type {
                    name: name.to_string(),
                    location: ut.location.clone(),
                    span_size: size,
                });
            }
            self.container_stack.push(name.to_string());
            Walker::walk(ut, self);
            self.container_stack.pop();
        } else {
            Walker::walk(ut, self);
        }
    }

    fn visit_variable(&mut self, variable: &Variable) {
        // `Variable.location` is the variable name's span (set by the
        // parser at the identifier token). When the cursor sits on it
        // we synthesise a hit pointing back at the declaration so
        // hover / goto-def / references all work from the declaration
        // site itself.
        if let Some(size) = self.loc_contains(&variable.location) {
            self.record_decl_name(DeclNameHit::Variable {
                container: self.container_stack.last().cloned(),
                name: variable.name.clone(),
                location: variable.location.clone(),
                span_size: size,
            });
        }
        Walker::walk(variable, self);
    }

    fn visit_data_type_declaration(&mut self, dt: &DataTypeDeclaration) {
        // `DataTypeDeclaration::Reference` carries a type name + its
        // own source location — that's what the user clicks on when
        // they navigate from a `VAR x : myType` declaration.
        if let DataTypeDeclaration::Reference { referenced_type, location } = dt {
            if let Some(range) = location.to_range() {
                if range.start <= self.target_offset && self.target_offset < range.end {
                    let size = range.end - range.start;
                    if self.type_ref_hit.as_ref().is_none_or(|t| size <= t.span_size) {
                        self.type_ref_hit = Some(TypeRefHit {
                            referenced_type: referenced_type.clone(),
                            location: location.clone(),
                            span_size: size,
                        });
                    }
                }
            }
        }
        // Default walk handles `Definition` variants (inline types).
        Walker::walk(dt, self);
    }

    fn visit(&mut self, node: &AstNode) {
        // Synthetic wrappers from lowering can have `location.to_range()
        // == None` even when their children carry real source ranges.
        // CallStatement parameters in particular get rewrapped into a
        // location-less ExpressionList by a lowering participant. We
        // can't prune by range here, but the children might have real
        // locations — descend without recording the wrapper itself.
        let Some(range) = node.location.to_range() else {
            self.ancestor_stack.push(node.id);
            node.walk(self);
            self.ancestor_stack.pop();
            return;
        };
        if !(range.start <= self.target_offset && self.target_offset < range.end) {
            // Children are always contained in their parent's span;
            // pruning here avoids walking entire subtrees.
            return;
        }
        let size = range.end - range.start;

        // <= so an inner node with the same span (rare but possible for
        // wrapper nodes around their sole child) replaces the outer.
        if self.deepest.as_ref().is_none_or(|d| size <= d.span_size) {
            self.deepest = Some(DeepestHit {
                id: node.id,
                location: node.location.clone(),
                span_size: size,
                ancestor_ids: self.ancestor_stack.clone(),
            });
        }

        self.ancestor_stack.push(node.id);
        node.walk(self);
        self.ancestor_stack.pop();
    }
}

/// Translate the annotator's direct annotation on the deepest node.
///
/// Maps each `StatementAnnotation` variant that can name a declaration
/// to a `ResolvedSymbol` by asking the `Index` for the declaration's
/// `SourceLocation`. Variants the prototype doesn't surface (Value,
/// ReplacementAst, Label, Override, …) return `None` — handlers degrade
/// gracefully.
fn translate(annot: &StatementAnnotation, index: &Index) -> Option<ResolvedSymbol> {
    match annot {
        StatementAnnotation::Variable { qualified_name, .. } => {
            let entry = lookup_variable(index, qualified_name)?;
            Some(ResolvedSymbol {
                declaration_location: entry.source_location.clone(),
                kind: classify_variable(qualified_name),
                qualified_name: qualified_name.clone(),
            })
        }
        StatementAnnotation::Argument { pou, position, .. } => {
            let params = index.get_available_parameters(pou);
            let entry = params.get(*position).copied()?;
            Some(ResolvedSymbol {
                declaration_location: entry.source_location.clone(),
                kind: SymbolKind::Argument,
                qualified_name: format!("{pou}.{}", entry.get_name()),
            })
        }
        StatementAnnotation::Function { qualified_name, .. }
        | StatementAnnotation::Program { qualified_name } => {
            let pou = index.find_pou(qualified_name)?;
            Some(ResolvedSymbol {
                declaration_location: pou.get_location().clone(),
                kind: SymbolKind::Pou,
                qualified_name: qualified_name.clone(),
            })
        }
        StatementAnnotation::Type { type_name } => {
            let ty = index.find_type(type_name).or_else(|| index.find_pou_type(type_name))?;
            Some(ResolvedSymbol {
                declaration_location: ty.location.clone(),
                kind: SymbolKind::Type,
                qualified_name: type_name.clone(),
            })
        }
        _ => None,
    }
}

/// `qualified_name` is either `"GLOBAL_VAR"` or `"Container.member"`.
/// Split on `.` and ask the `Index` accordingly.
fn lookup_variable<'a>(index: &'a Index, qualified_name: &str) -> Option<&'a plc::index::VariableIndexEntry> {
    match qualified_name.rsplit_once('.') {
        Some((container, member)) => index.find_member(container, member),
        None => index.find_global_variable(qualified_name),
    }
}

/// Best-effort classification — the prototype distinguishes Variable
/// vs Member only by whether the qualified name has a container prefix.
/// Section (VAR vs VAR_INPUT vs VAR_GLOBAL …) is queryable from the
/// `VariableIndexEntry` but the hover format doesn't yet branch on it,
/// so we leave that refinement to phase 13 polish.
fn classify_variable(qualified_name: &str) -> SymbolKind {
    if qualified_name.contains('.') {
        SymbolKind::Member
    } else {
        SymbolKind::Variable
    }
}

/// §2.4 ancestor-context fallback for shapes where the deepest node
/// has no direct annotation but an ancestor's annotation names the
/// container.
///
/// Currently handles struct-literal initializer fields: `(field := 1)`
/// inside an initializer expression. The enclosing `ParenExpression`
/// carries a type-hint annotation pointing at the struct type; the
/// field name comes from the source slice the user typed.
fn resolve_via_ancestor_chain(
    ancestor_ids: &[AstId],
    usage_text: &str,
    annotated: &AnnotatedProject,
) -> Option<ResolvedSymbol> {
    // Walk outward (innermost ancestor first). The first ancestor
    // whose type hint matches a known shape wins.
    for ancestor_id in ancestor_ids.iter().rev() {
        let hint = annotated.annotations.get_hint_with_id(*ancestor_id);

        // Named call arg: `foo(x := 1)`. The annotator stores
        // `Argument { pou, position }` as a *type hint* on the whole
        // `x := 1` Assignment node — the inner Identifier `x` carries
        // no direct annotation. Recover the parameter by indexing the
        // POU's parameter list.
        if let Some(StatementAnnotation::Argument { pou, position, .. }) = hint {
            let params = annotated.index.get_available_parameters(pou);
            if let Some(entry) = params.get(*position).copied() {
                return Some(ResolvedSymbol {
                    declaration_location: entry.source_location.clone(),
                    kind: SymbolKind::Argument,
                    qualified_name: format!("{pou}.{}", entry.get_name()),
                });
            }
        }

        // Struct-literal initializer field: `(field := 1)`. The
        // enclosing `ParenExpression` carries a type hint pointing at
        // the struct type; the field name comes from the source slice.
        if let Some(StatementAnnotation::Type { type_name }) = hint {
            if let Some(entry) = annotated.index.find_member(type_name, usage_text) {
                return Some(ResolvedSymbol {
                    declaration_location: entry.source_location.clone(),
                    kind: SymbolKind::Member,
                    qualified_name: format!("{type_name}.{usage_text}"),
                });
            }
        }
    }
    None
}

/// Convert an LSP `(uri, position)` into `(path, byte_offset)` against
/// the worker-captured source. Returns `None` when the URI doesn't map
/// to a file path or the position falls outside the captured content.
///
/// Symmetric counterpart to `diagnostics::code_span_to_range` —
/// reuses the same utf-16 ↔ byte-offset arithmetic in the inverse
/// direction.
fn position_to_byte_offset(
    uri: &Uri,
    position: Position,
    encoding: &PositionEncodingKind,
    source_contents: &HashMap<String, String>,
) -> Option<(PathBuf, usize)> {
    let path = crate::project::file_uri_to_path(uri)?;
    let source = source_contents.get(path.to_string_lossy().as_ref())?;

    let line = source.lines().nth(position.line as usize)?;
    let byte_in_line = if encoding == &PositionEncodingKind::UTF16 {
        utf16_units_to_byte_offset(line, position.character)?
    } else {
        position.character as usize
    };

    let line_start: usize = source
        .lines()
        .take(position.line as usize)
        .map(|l| l.len() + 1) // +1 for newline; tolerates CRLF approximation
        .sum();
    Some((path, line_start + byte_in_line))
}

/// Inverse of `diagnostics::byte_offset_to_utf16_units`: walk the line
/// counting utf-16 code units until we've consumed `n_units`, return
/// the byte offset at that point. Falls back to a raw cast for
/// pathological inputs (mirrors the diagnostic-side fallback).
fn utf16_units_to_byte_offset(line: &str, n_units: u32) -> Option<usize> {
    let mut consumed: u32 = 0;
    for (byte_idx, ch) in line.char_indices() {
        if consumed >= n_units {
            return Some(byte_idx);
        }
        consumed += ch.len_utf16() as u32;
    }
    // Position past end of line — clamp to line end.
    Some(line.len())
}

// Keep the import honest even when annotation lookups go through
// the `AnnotationMap` trait above.
const _: fn(&AstAnnotations) -> Option<&StatementAnnotation> = |a| a.get_with_id(0);
