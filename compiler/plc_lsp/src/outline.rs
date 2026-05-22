//! Build LSP `DocumentSymbol` trees from rusty AST units.
//!
//! Phase 7 of the LSP plan — file outline (`textDocument/documentSymbol`).
//! Walks a `CompilationUnit`'s POUs, user types, and global var blocks,
//! emits a *flat* tree (Q3 choice A): POUs/types at top level, members
//! as direct children. Section info (VAR_INPUT vs VAR_OUTPUT etc.)
//! goes into the `detail` field rather than via synthetic namespace
//! nodes — matches how other LSPs surface qualifiers like `public` /
//! `static`.
//!
//! Filtering: declarations whose `SourceLocation` is internal
//! (`<internal>` filename or empty span) are dropped. This catches
//! synthesised entries from lowering — except the PropertyLowerer's
//! `__get_X` / `__set_X` methods, which inherit real locations from
//! the user's `PROPERTY` statement; per the phase-7-10 design they
//! show as methods in the outline. Accepted as a known prototype
//! degradation.
//!
//! Per the "display vs identity" rule (see plan §3.5): names rendered
//! in the outline are read from declaration string fields. The
//! preprocessor-renaming concern that motivated the rule mostly
//! affects type *references* (not declarations themselves), so the
//! outline isn't the place that bites — but `data_type_declaration_label`
//! is the seam where we'd switch to a source-slice display if we ever
//! see a real-world mismatch.

// `lsp_types::DocumentSymbol::deprecated` is itself deprecated in the
// LSP spec (clients should use `tags` instead), but the struct still
// requires the field and lsp-types still includes it. We always pass
// `None`; the lint fires anyway because the struct field is annotated.
#![allow(deprecated)]

use std::collections::HashMap;

use lsp_types::{DocumentSymbol, PositionEncodingKind, Range, SymbolKind};
use plc_ast::ast::{
    ArgumentProperty, CompilationUnit, DataType, DataTypeDeclaration, Interface, Pou, PouType,
    UserTypeDeclaration, Variable, VariableBlockType,
};
use plc_source::source_location::{FileMarker, SourceLocation};

use crate::diagnostics::code_span_to_range;

/// Build the outline for a single `CompilationUnit`. The caller is
/// responsible for matching by URI; the function itself doesn't know
/// about URIs (kept this way so the unit tests are trivial — no need
/// to construct LSP URIs to exercise the walker).
///
/// `source` is the file's source text, used for utf-16 column
/// conversion. Pass `None` on the utf-8 path or when source isn't
/// available; columns then fall back to raw byte offsets — slightly
/// off for non-ASCII content but better than dropping the outline.
pub fn document_symbols(
    unit: &CompilationUnit,
    encoding: &PositionEncodingKind,
    source: Option<&str>,
) -> Vec<DocumentSymbol> {
    // Per-kind buckets. Each emits at most one synthetic group node
    // (a `Namespace` symbol) that wraps the actual declarations as
    // children. Order of the `add_group` calls below sets the order
    // the editor shows the groups in.
    let mut programs = Vec::new();
    let mut functions = Vec::new();
    let mut function_blocks = Vec::new();
    let mut classes = Vec::new();
    let mut interfaces = Vec::new();
    let mut types = Vec::new();
    let mut globals = Vec::new();

    // ST puts methods at the top level of the AST with a `parent`
    // string, not nested in their FB's struct. First pass collects
    // methods keyed by parent name so we can attach them as children
    // below.
    let mut methods_by_parent: HashMap<String, Vec<&Pou>> = HashMap::new();
    for pou in &unit.pous {
        if is_internal(&pou.location) {
            continue;
        }
        if let PouType::Method { parent, .. } = &pou.kind {
            methods_by_parent.entry(parent.clone()).or_default().push(pou);
        }
    }

    for pou in &unit.pous {
        if is_internal(&pou.location) {
            continue;
        }
        let methods = methods_by_parent.remove(&pou.name).unwrap_or_default();
        let Some(symbol) = build_pou_symbol(pou, &methods, encoding, source) else {
            continue;
        };
        match pou.kind {
            PouType::Program => programs.push(symbol),
            PouType::Function => functions.push(symbol),
            PouType::FunctionBlock => function_blocks.push(symbol),
            PouType::Class => classes.push(symbol),
            // Method / Action / Init / ProjectInit are skipped at the
            // top level — methods are attached as children of their
            // parent above; the rest are synthesised or not user-facing.
            _ => {}
        }
    }

    for iface in &unit.interfaces {
        if is_internal(&iface.location) {
            continue;
        }
        if let Some(symbol) = build_interface_symbol(iface, encoding, source) {
            interfaces.push(symbol);
        }
    }

    for ty in &unit.user_types {
        if is_internal(&ty.location) {
            continue;
        }
        // `UserTypeDeclaration.scope: Option<String>` is documented as
        // "stores the original scope for compiler-generated types" —
        // a structural marker for synthesised types like `__main_points`,
        // `__vtable_FB`, etc. that share the user file's location. Skip
        // anything with a scope set; only user-declared types (where the
        // user wrote `TYPE ... END_TYPE`) have `scope: None`.
        if ty.scope.is_some() {
            continue;
        }
        if let Some(symbol) = build_type_symbol(ty, encoding, source) {
            types.push(symbol);
        }
    }

    for block in &unit.global_vars {
        if is_internal(&block.location) {
            continue;
        }
        for var in &block.variables {
            if is_internal(&var.location) {
                continue;
            }
            if let Some(symbol) = build_variable_symbol(var, &block.kind, encoding, source) {
                globals.push(symbol);
            }
        }
    }

    // Wrap each non-empty bucket in a synthetic `Namespace` group node.
    // Order chosen so the outline reads "what types exist → what
    // globals are visible → what POUs implement them".
    let mut out: Vec<DocumentSymbol> = Vec::new();
    add_group(&mut out, "Types", types);
    add_group(&mut out, "Globals", globals);
    add_group(&mut out, "Interfaces", interfaces);
    add_group(&mut out, "Programs", programs);
    add_group(&mut out, "Functions", functions);
    add_group(&mut out, "Function Blocks", function_blocks);
    add_group(&mut out, "Classes", classes);
    out
}

/// Push a synthetic parent `DocumentSymbol` of kind Namespace into
/// `out` wrapping `children`. No-op when `children` is empty so the
/// editor doesn't show vacant group rows. The wrapper's range spans
/// from the first child to the last so "go to symbol" on the group
/// itself jumps to a sensible place.
fn add_group(out: &mut Vec<DocumentSymbol>, name: &str, children: Vec<DocumentSymbol>) {
    if children.is_empty() {
        return;
    }
    let first_range = children.first().expect("non-empty after the check").range;
    let last_range = children.last().expect("non-empty after the check").range;
    out.push(DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind: SymbolKind::NAMESPACE,
        tags: None,
        #[allow(deprecated)]
        deprecated: None,
        range: Range { start: first_range.start, end: last_range.end },
        selection_range: first_range,
        children: Some(children),
    });
}

fn is_internal(location: &SourceLocation) -> bool {
    location.is_internal() || location.get_file_name().is_none()
}

fn build_pou_symbol(
    pou: &Pou,
    methods: &[&Pou],
    encoding: &PositionEncodingKind,
    source: Option<&str>,
) -> Option<DocumentSymbol> {
    let range = location_to_range(&pou.location, encoding, source)?;
    // name_location is the identifier span; if for some reason it's
    // empty (shouldn't happen for parsed POUs), fall back to the full
    // range so the editor still has something to click on.
    let selection_range = location_to_range(&pou.name_location, encoding, source).unwrap_or(range);

    let mut children = Vec::new();
    for block in &pou.variable_blocks {
        if is_internal(&block.location) {
            continue;
        }
        for var in &block.variables {
            if is_internal(&var.location) {
                continue;
            }
            if let Some(child) = build_variable_symbol(var, &block.kind, encoding, source) {
                children.push(child);
            }
        }
    }
    for method in methods {
        if let Some(child) = build_method_symbol(method, encoding, source) {
            children.push(child);
        }
    }

    Some(DocumentSymbol {
        name: pou.name.clone(),
        detail: pou_detail(pou),
        kind: pou_symbol_kind(&pou.kind),
        tags: None,
        // The `deprecated` field is deprecated in the LSP spec itself
        // (use the tags vec instead). Leave None.
        deprecated: None,
        range,
        selection_range,
        children: nonempty(children),
    })
}

fn build_interface_symbol(
    iface: &Interface,
    encoding: &PositionEncodingKind,
    source: Option<&str>,
) -> Option<DocumentSymbol> {
    let range = location_to_range(&iface.location, encoding, source)?;
    let selection_range = location_to_range(&iface.ident.location, encoding, source).unwrap_or(range);

    let mut children = Vec::new();
    for method in &iface.methods {
        if is_internal(&method.location) {
            continue;
        }
        if let Some(child) = build_method_symbol(method, encoding, source) {
            children.push(child);
        }
    }

    Some(DocumentSymbol {
        name: iface.ident.name.clone(),
        detail: Some("INTERFACE".to_string()),
        kind: SymbolKind::INTERFACE,
        tags: None,
        deprecated: None,
        range,
        selection_range,
        children: nonempty(children),
    })
}

fn build_method_symbol(
    method: &Pou,
    encoding: &PositionEncodingKind,
    source: Option<&str>,
) -> Option<DocumentSymbol> {
    let range = location_to_range(&method.location, encoding, source)?;
    let selection_range = location_to_range(&method.name_location, encoding, source).unwrap_or(range);

    let mut children = Vec::new();
    for block in &method.variable_blocks {
        if is_internal(&block.location) {
            continue;
        }
        for var in &block.variables {
            if is_internal(&var.location) {
                continue;
            }
            if let Some(child) = build_variable_symbol(var, &block.kind, encoding, source) {
                children.push(child);
            }
        }
    }

    Some(DocumentSymbol {
        name: method.name.clone(),
        detail: pou_detail(method),
        kind: SymbolKind::METHOD,
        tags: None,
        deprecated: None,
        range,
        selection_range,
        children: nonempty(children),
    })
}

fn build_variable_symbol(
    var: &Variable,
    section: &VariableBlockType,
    encoding: &PositionEncodingKind,
    source: Option<&str>,
) -> Option<DocumentSymbol> {
    let range = location_to_range(&var.location, encoding, source)?;
    let detail = format!(
        "{section} : {type_str}",
        section = variable_section_label(section),
        type_str = data_type_declaration_label(&var.data_type_declaration),
    );

    Some(DocumentSymbol {
        name: var.name.clone(),
        detail: Some(detail),
        kind: variable_symbol_kind(section),
        tags: None,
        deprecated: None,
        range,
        // Variables don't carry a separate name-only location in the
        // AST; full range serves both.
        selection_range: range,
        children: None,
    })
}

fn build_type_symbol(
    ty: &UserTypeDeclaration,
    encoding: &PositionEncodingKind,
    source: Option<&str>,
) -> Option<DocumentSymbol> {
    let range = location_to_range(&ty.location, encoding, source)?;
    let name = data_type_name(&ty.data_type)?;

    // Struct/enum members as children when there are any structural
    // children we can identify; everything else is a leaf.
    let children = match &ty.data_type {
        DataType::StructType { variables, .. } => {
            let mut entries = Vec::new();
            for var in variables {
                if is_internal(&var.location) {
                    continue;
                }
                let var_range = match location_to_range(&var.location, encoding, source) {
                    Some(r) => r,
                    None => continue,
                };
                let detail =
                    format!("Field : {ty}", ty = data_type_declaration_label(&var.data_type_declaration));
                entries.push(DocumentSymbol {
                    name: var.name.clone(),
                    detail: Some(detail),
                    kind: SymbolKind::FIELD,
                    tags: None,
                    deprecated: None,
                    range: var_range,
                    selection_range: var_range,
                    children: None,
                });
            }
            nonempty(entries)
        }
        _ => None,
    };

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: data_type_symbol_kind(&ty.data_type),
        tags: None,
        deprecated: None,
        range,
        selection_range: range,
        children,
    })
}

fn pou_symbol_kind(kind: &PouType) -> SymbolKind {
    // Mapping rationale: PROGRAM and FUNCTION are stateless executables
    // — `Function` reads cleanly. FUNCTION_BLOCK and CLASS are stateful
    // with members — `Class` reads cleanly. METHOD is invoked on a
    // class — `Method`. ACTION is closer to a procedure on a parent FB
    // — `Function` again. Init/ProjectInit don't normally surface but
    // map to `Constructor` for the edge case.
    match kind {
        PouType::Program | PouType::Function | PouType::Action => SymbolKind::FUNCTION,
        PouType::FunctionBlock | PouType::Class => SymbolKind::CLASS,
        PouType::Method { .. } => SymbolKind::METHOD,
        PouType::Init | PouType::ProjectInit => SymbolKind::CONSTRUCTOR,
    }
}

fn variable_symbol_kind(section: &VariableBlockType) -> SymbolKind {
    // VAR_INPUT/OUTPUT/IN_OUT are parameters in the LSP `Field` sense
    // (they're parts of the surrounding POU's interface). VAR / VAR_TEMP
    // / VAR_GLOBAL / VAR_EXTERNAL are storage — `Variable`.
    match section {
        VariableBlockType::Input(_) | VariableBlockType::Output | VariableBlockType::InOut => {
            SymbolKind::FIELD
        }
        _ => SymbolKind::VARIABLE,
    }
}

fn data_type_symbol_kind(ty: &DataType) -> SymbolKind {
    match ty {
        DataType::StructType { .. } => SymbolKind::STRUCT,
        DataType::EnumType { .. } => SymbolKind::ENUM,
        DataType::ArrayType { .. } => SymbolKind::ARRAY,
        // Everything else is "this is a type alias / scalar wrapper"
        // — TYPE_PARAMETER is the least-bad LSP kind for that.
        _ => SymbolKind::TYPE_PARAMETER,
    }
}

fn data_type_name(ty: &DataType) -> Option<String> {
    match ty {
        DataType::StructType { name, .. }
        | DataType::EnumType { name, .. }
        | DataType::ArrayType { name, .. }
        | DataType::SubRangeType { name, .. }
        | DataType::PointerType { name, .. }
        | DataType::StringType { name, .. } => name.clone(),
        DataType::GenericType { name, .. } => Some(name.clone()),
        _ => None,
    }
}

fn variable_section_label(section: &VariableBlockType) -> &'static str {
    match section {
        VariableBlockType::Local => "VAR",
        VariableBlockType::Temp => "VAR_TEMP",
        VariableBlockType::Input(ArgumentProperty::ByVal) => "VAR_INPUT",
        VariableBlockType::Input(ArgumentProperty::ByRef) => "VAR_INPUT {ref}",
        VariableBlockType::Output => "VAR_OUTPUT",
        VariableBlockType::Global => "VAR_GLOBAL",
        VariableBlockType::InOut => "VAR_IN_OUT",
        VariableBlockType::External => "VAR_EXTERNAL",
    }
}

fn data_type_declaration_label(decl: &DataTypeDeclaration) -> String {
    match decl {
        DataTypeDeclaration::Reference { referenced_type, .. }
        | DataTypeDeclaration::Aggregate { referenced_type, .. } => referenced_type.clone(),
        DataTypeDeclaration::Definition { data_type, .. } => match data_type.as_ref() {
            DataType::StructType { name: Some(n), .. }
            | DataType::EnumType { name: Some(n), .. }
            | DataType::ArrayType { name: Some(n), .. }
            | DataType::PointerType { name: Some(n), .. }
            | DataType::StringType { name: Some(n), .. } => n.clone(),
            DataType::SubRangeType { referenced_type, .. } => referenced_type.clone(),
            _ => "<anonymous type>".to_string(),
        },
    }
}

fn pou_detail(pou: &Pou) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(super_class) = &pou.super_class {
        parts.push(format!("EXTENDS {}", super_class.name));
    }
    if !pou.interfaces.is_empty() {
        let names: Vec<&str> = pou.interfaces.iter().map(|i| i.name.as_str()).collect();
        parts.push(format!("IMPLEMENTS {}", names.join(", ")));
    }
    if let Some(return_type) = &pou.return_type {
        parts.push(format!(": {}", data_type_declaration_label(return_type)));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn location_to_range(
    location: &SourceLocation,
    encoding: &PositionEncodingKind,
    source: Option<&str>,
) -> Option<Range> {
    code_span_to_range(location.get_span(), encoding, source)
}

fn nonempty<T>(v: Vec<T>) -> Option<Vec<T>> {
    if v.is_empty() {
        None
    } else {
        Some(v)
    }
}

/// Iterate `AnnotatedProject.units` and produce a per-file outline map
/// keyed by the unit's source file path. Used by the compile worker to
/// pre-compute outlines before shipping them across to the main thread
/// (which holds them on `ServerState` and answers `documentSymbol`
/// requests from there).
pub fn build_outline_map(
    annotated: &plc_driver::pipelines::AnnotatedProject,
    encoding: &PositionEncodingKind,
    source_contents: &HashMap<String, String>,
) -> HashMap<String, Vec<DocumentSymbol>> {
    let mut map: HashMap<String, Vec<DocumentSymbol>> = HashMap::new();
    for au in &annotated.units {
        let unit = au.get_unit();
        let Some(file) = unit_file_path(unit) else {
            continue;
        };
        let source = source_contents.get(&file).map(String::as_str);
        let symbols = document_symbols(unit, encoding, source);
        if symbols.is_empty() {
            continue;
        }
        map.entry(file).or_default().extend(symbols);
    }
    map
}

fn unit_file_path(unit: &CompilationUnit) -> Option<String> {
    match unit.file {
        FileMarker::File(s) => Some(s.to_string()),
        // Internal / Undefined units never show in the outline.
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plc_ast::ast::{
        AccessModifier, ArgumentProperty, CompilationUnit, DataType, DataTypeDeclaration, DeclarationKind,
        Identifier, LinkageType, Pou, PouType, UserTypeDeclaration, Variable, VariableBlock,
        VariableBlockType,
    };
    use plc_source::source_location::{FileMarker, SourceLocationFactory};
    use plc_source::SourceCode;
    use std::path::PathBuf;

    /// Test harness: a long stretch of dummy source backing a
    /// `SourceLocationFactory`, so tests can produce real (non-internal)
    /// `SourceLocation`s via byte-offset ranges. Line/column numbers
    /// are computed by the factory but we don't assert on them in the
    /// structural tests below — only that the locations are *real*
    /// (so the internal-filter doesn't drop them).
    struct Harness {
        factory: SourceLocationFactory,
    }

    impl Harness {
        fn new() -> Self {
            // 4 KiB of newlines and spaces is plenty for any test range
            // we construct below; the exact content doesn't matter.
            let source_text = " \n".repeat(2048);
            let source = SourceCode { source: source_text, path: Some(PathBuf::from("a.st")) };
            Self { factory: SourceLocationFactory::for_source(&source) }
        }

        fn loc(&self, start: usize, end: usize) -> plc_source::source_location::SourceLocation {
            self.factory.create_range(start..end)
        }

        fn pou(&self, name: &str, kind: PouType, range_start: usize) -> Pou {
            Pou {
                id: 0,
                name: name.to_string(),
                kind,
                variable_blocks: vec![],
                return_type: None,
                location: self.loc(range_start, range_start + 50),
                name_location: self.loc(range_start, range_start + name.len()),
                poly_mode: None,
                generics: vec![],
                linkage: LinkageType::Internal,
                super_class: None,
                is_const: false,
                interfaces: vec![],
                properties: vec![],
            }
        }

        fn var(&self, name: &str, range_start: usize) -> Variable {
            Variable {
                name: name.to_string(),
                data_type_declaration: DataTypeDeclaration::Reference {
                    referenced_type: "INT".to_string(),
                    location: self.loc(range_start + 10, range_start + 13),
                },
                initializer: None,
                address: None,
                location: self.loc(range_start, range_start + name.len()),
            }
        }

        fn block(&self, kind: VariableBlockType, vars: Vec<Variable>, range_start: usize) -> VariableBlock {
            VariableBlock {
                access: AccessModifier::Protected,
                constant: false,
                retain: false,
                variables: vars,
                kind,
                linkage: LinkageType::Internal,
                location: self.loc(range_start, range_start + 30),
            }
        }

        fn empty_unit(&self) -> CompilationUnit {
            CompilationUnit {
                global_vars: vec![],
                var_config: vec![],
                pous: vec![],
                implementations: vec![],
                interfaces: vec![],
                user_types: vec![],
                file: FileMarker::File("a.st"),
                linkage: LinkageType::Internal,
            }
        }
    }

    #[test]
    fn empty_unit_yields_no_symbols() {
        let h = Harness::new();
        let unit = h.empty_unit();
        let syms = document_symbols(&unit, &PositionEncodingKind::UTF8, None);
        assert!(syms.is_empty());
    }

    /// Locate the synthetic group named `group_name` in the flat
    /// top-level symbol list and return its children. Lets the
    /// per-shape tests keep asserting on the actual declarations
    /// without restating the group wrapper each time.
    fn group<'a>(syms: &'a [DocumentSymbol], group_name: &str) -> &'a [DocumentSymbol] {
        let g = syms
            .iter()
            .find(|s| s.name == group_name)
            .unwrap_or_else(|| panic!("missing group {group_name:?}; got {syms:?}"));
        g.children.as_deref().unwrap_or(&[])
    }

    #[test]
    fn single_program_with_local_var() {
        let h = Harness::new();
        let mut unit = h.empty_unit();
        let mut pou = h.pou("Main", PouType::Program, 0);
        pou.variable_blocks.push(h.block(VariableBlockType::Local, vec![h.var("x", 100)], 100));
        unit.pous.push(pou);

        let syms = document_symbols(&unit, &PositionEncodingKind::UTF8, None);
        let programs = group(&syms, "Programs");
        assert_eq!(programs.len(), 1);
        let main = &programs[0];
        assert_eq!(main.name, "Main");
        assert_eq!(main.kind, SymbolKind::FUNCTION); // PROGRAM
        let children = main.children.as_ref().expect("Main should have a child");
        assert_eq!(children.len(), 1);
        let x = &children[0];
        assert_eq!(x.name, "x");
        assert_eq!(x.kind, SymbolKind::VARIABLE);
        assert_eq!(x.detail.as_deref(), Some("VAR : INT"));
    }

    #[test]
    fn fb_with_input_output_var() {
        let h = Harness::new();
        let mut unit = h.empty_unit();
        let mut fb = h.pou("MyFB", PouType::FunctionBlock, 0);
        fb.variable_blocks.push(h.block(
            VariableBlockType::Input(ArgumentProperty::ByVal),
            vec![h.var("in", 100)],
            100,
        ));
        fb.variable_blocks.push(h.block(VariableBlockType::Output, vec![h.var("out", 200)], 200));
        unit.pous.push(fb);

        let syms = document_symbols(&unit, &PositionEncodingKind::UTF8, None);
        let fbs = group(&syms, "Function Blocks");
        assert_eq!(fbs.len(), 1);
        let fb_sym = &fbs[0];
        assert_eq!(fb_sym.kind, SymbolKind::CLASS); // FUNCTION_BLOCK → Class
        let children = fb_sym.children.as_ref().expect("MyFB should have children");
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].name, "in");
        assert_eq!(children[0].kind, SymbolKind::FIELD);
        assert_eq!(children[0].detail.as_deref(), Some("VAR_INPUT : INT"));
        assert_eq!(children[1].name, "out");
        assert_eq!(children[1].detail.as_deref(), Some("VAR_OUTPUT : INT"));
    }

    #[test]
    fn method_nests_under_parent_fb() {
        let h = Harness::new();
        let mut unit = h.empty_unit();
        let fb = h.pou("MyFB", PouType::FunctionBlock, 0);
        let method = h.pou(
            "doStuff",
            PouType::Method {
                parent: "MyFB".to_string(),
                property: None,
                declaration_kind: DeclarationKind::Concrete,
            },
            500,
        );
        unit.pous.push(fb);
        unit.pous.push(method);

        let syms = document_symbols(&unit, &PositionEncodingKind::UTF8, None);
        let fbs = group(&syms, "Function Blocks");
        assert_eq!(fbs.len(), 1, "method should not show as top-level");
        let fb_sym = &fbs[0];
        let children = fb_sym.children.as_ref().expect("MyFB should have a child");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "doStuff");
        assert_eq!(children[0].kind, SymbolKind::METHOD);
    }

    #[test]
    fn pou_with_extends_emits_detail() {
        let h = Harness::new();
        let mut unit = h.empty_unit();
        let mut fb = h.pou("Child", PouType::FunctionBlock, 0);
        fb.super_class = Some(Identifier { name: "Parent".to_string(), location: h.loc(30, 36) });
        unit.pous.push(fb);

        let syms = document_symbols(&unit, &PositionEncodingKind::UTF8, None);
        let fbs = group(&syms, "Function Blocks");
        assert_eq!(fbs[0].detail.as_deref(), Some("EXTENDS Parent"));
    }

    #[test]
    fn struct_type_with_fields() {
        let h = Harness::new();
        let mut unit = h.empty_unit();
        unit.user_types.push(UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some("Point".to_string()),
                variables: vec![h.var("x", 100), h.var("y", 150)],
            },
            initializer: None,
            location: h.loc(0, 200),
            scope: None,
            linkage: LinkageType::Internal,
        });

        let syms = document_symbols(&unit, &PositionEncodingKind::UTF8, None);
        let types = group(&syms, "Types");
        assert_eq!(types.len(), 1);
        let point = &types[0];
        assert_eq!(point.name, "Point");
        assert_eq!(point.kind, SymbolKind::STRUCT);
        let fields = point.children.as_ref().expect("struct should have fields");
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "x");
        assert_eq!(fields[0].kind, SymbolKind::FIELD);
        assert_eq!(fields[0].detail.as_deref(), Some("Field : INT"));
    }

    #[test]
    fn global_vars_emit_as_top_level() {
        let h = Harness::new();
        let mut unit = h.empty_unit();
        unit.global_vars.push(h.block(VariableBlockType::Global, vec![h.var("g", 100)], 0));

        let syms = document_symbols(&unit, &PositionEncodingKind::UTF8, None);
        let globals = group(&syms, "Globals");
        assert_eq!(globals.len(), 1);
        assert_eq!(globals[0].name, "g");
        assert_eq!(globals[0].kind, SymbolKind::VARIABLE);
        assert_eq!(globals[0].detail.as_deref(), Some("VAR_GLOBAL : INT"));
    }

    #[test]
    fn internal_location_filtered_out() {
        let h = Harness::new();
        let mut unit = h.empty_unit();
        let mut pou = h.pou("Synth", PouType::Program, 0);
        // Flip the location to internal — should be filtered.
        pou.location = pou.location.into_internal();
        unit.pous.push(pou);

        let syms = document_symbols(&unit, &PositionEncodingKind::UTF8, None);
        assert!(syms.is_empty(), "internal POU should be filtered");
    }
}
