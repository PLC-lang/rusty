//! Investigation probe: what does the lenient parser produce for typical
//! mid-typing inputs that an LSP completion handler would have to react to?
//!
//! `parser::parse` is the lenient core (always returns a `CompilationUnit`
//! plus a `Vec<Diagnostic>`). `parser::parse_file` wraps it with the
//! parse-time error gate that turns any error-severity diagnostic into
//! `Err(...)` and discards the partial AST. This probe goes straight to
//! the lenient core so we can see the partial AST that a future
//! `parse_file_lenient` would hand the LSP completion path.
//!
//! Run with:
//!   cargo test -p plc-compiler parser::tests::lenient_completion_probe -- --nocapture
//!
//! No assertions. Output goes to stderr; consume by piping to a file.
//! The probe is grouped by completion-context category so the output is
//! easy to skim alongside the category headings in
//! `.baseline/lsp-phase-13-investigation.md`.

use plc_ast::ast::{AstNode, LinkageType};
use plc_ast::provider::IdProvider;
use plc_ast::ser::AstSerializer;
use plc_ast::visitor::{AstVisitor, Walker};
use plc_diagnostics::diagnostician::Diagnostician;
use plc_source::SourceCode;

use crate::parser::parse_file_lenient;
use crate::resolver::{AnnotationMap, AnnotationMapImpl};
use crate::test_utils::tests::{annotate_with_ids, index_with_ids, parse};

/// Smoke test for `parse_file_lenient` (P13.1). Verifies the wrapper returns
/// the partial unit + diagnostics on broken input rather than `Err(...)` like
/// `parse_file` does.
#[test]
fn parse_file_lenient_returns_partial_unit_on_broken_input() {
    let source =
        SourceCode { source: "FUNCTION main : DINT\n    foo.\nEND_FUNCTION\n".to_string(), path: None };
    let mut diagnostician = Diagnostician::buffered();
    let (unit, errors) =
        parse_file_lenient(&source, LinkageType::Internal, IdProvider::default(), &mut diagnostician);
    assert_eq!(unit.pous.len(), 1, "expected one POU to survive the parse error");
    assert_eq!(unit.pous[0].name, "main");
    assert!(!errors.is_empty(), "expected at least one diagnostic for the broken expression");
}

fn category(label: &str) {
    eprintln!("\n\n############################################################");
    eprintln!("## {label}");
    eprintln!("############################################################");
}

fn probe(label: &str, src: &str) {
    eprintln!("\n========================================");
    eprintln!("== {label}");
    eprintln!("========================================");
    eprintln!("source:");
    for (i, line) in src.lines().enumerate() {
        eprintln!("  {i:>2} | {line}");
    }

    let (unit, diagnostics) = parse(src);

    eprintln!("\ndiagnostics ({}):", diagnostics.len());
    for d in &diagnostics {
        eprintln!("  - {}", d.get_message());
    }

    eprintln!("\nunit summary:");
    eprintln!("  pous            : {}", unit.pous.len());
    eprintln!("  implementations : {}", unit.implementations.len());
    eprintln!("  global_vars     : {}", unit.global_vars.len());
    eprintln!("  user_types      : {}", unit.user_types.len());

    for (i, pou) in unit.pous.iter().enumerate() {
        eprintln!("\npou[{i}]: {} (kind={:?})", pou.name, pou.kind);
        for vb in &pou.variable_blocks {
            eprintln!("  variable_block: {vb:#?}");
        }
    }

    for (i, ut) in unit.user_types.iter().enumerate() {
        eprintln!("\nuser_type[{i}]: {ut:#?}");
    }

    for (i, gv) in unit.global_vars.iter().enumerate() {
        eprintln!("\nglobal_var[{i}]: {gv:#?}");
    }

    for (i, impl_) in unit.implementations.iter().enumerate() {
        eprintln!("\nimplementation[{i}] for {} ({} stmts) — debug:", impl_.name, impl_.statements.len());
        for (j, stmt) in impl_.statements.iter().enumerate() {
            eprintln!("  stmt[{j}]:\n{stmt:#?}");
        }
        eprintln!("\nimplementation[{i}] — ast_serializer round-trip:");
        eprintln!("{}", AstSerializer::format_nodes(&impl_.statements));
    }
}

#[test]
fn run_all_probes() {
    // --------------------------------------------------------------------
    category("A. Member access (cursor right after the dot)");
    // --------------------------------------------------------------------

    probe(
        "A1. foo.",
        r#"
FUNCTION main : DINT
VAR foo : Point; END_VAR
    foo.
END_FUNCTION
"#,
    );

    probe(
        "A2. foo.bar.",
        r#"
FUNCTION main : DINT
VAR foo : Outer; END_VAR
    foo.bar.
END_FUNCTION
"#,
    );

    probe(
        "A3. arr[1].",
        r#"
FUNCTION main : DINT
VAR arr : ARRAY[1..5] OF Point; END_VAR
    arr[1].
END_FUNCTION
"#,
    );

    probe(
        "A4. THIS^.",
        r#"
FUNCTION_BLOCK FB
VAR x : DINT; END_VAR
    THIS^.
END_FUNCTION_BLOCK
"#,
    );

    probe(
        "A5. ptr^.",
        r#"
FUNCTION main : DINT
VAR ptr : REF_TO Point; END_VAR
    ptr^.
END_FUNCTION
"#,
    );

    probe(
        "A6. foo.  followed by a valid statement   (does the broken expr poison the next stmt?)",
        r#"
FUNCTION main : DINT
VAR foo : Point; a : DINT; END_VAR
    foo.
    a := 1;
END_FUNCTION
"#,
    );

    // --------------------------------------------------------------------
    category("B. Call sites");
    // --------------------------------------------------------------------

    probe(
        "B6. other(",
        r#"
FUNCTION main : DINT
    other(
END_FUNCTION
"#,
    );

    probe(
        "B7. other(x,",
        r#"
FUNCTION main : DINT
VAR x : DINT; END_VAR
    other(x,
END_FUNCTION
"#,
    );

    probe(
        "B8. other(x :=",
        r#"
FUNCTION main : DINT
    other(x :=
END_FUNCTION
"#,
    );

    // --------------------------------------------------------------------
    category("C. Declarations");
    // --------------------------------------------------------------------

    probe(
        "C9. VAR x :   (then END_VAR)",
        r#"
FUNCTION main : DINT
VAR
    x :
END_VAR
END_FUNCTION
"#,
    );

    probe(
        "C10. VAR x : MyT   (partial type name)",
        r#"
FUNCTION main : DINT
VAR
    x : MyT
END_VAR
END_FUNCTION
"#,
    );

    probe(
        "C11. FUNCTION foo :   (missing return type)",
        r#"
FUNCTION foo :
VAR_INPUT x : DINT; END_VAR
END_FUNCTION
"#,
    );

    probe(
        "C12. FUNCTION_BLOCK B EXTENDS   (awaiting parent name)",
        r#"
FUNCTION_BLOCK B EXTENDS
VAR x : DINT; END_VAR
END_FUNCTION_BLOCK
"#,
    );

    // --------------------------------------------------------------------
    category("D. Control flow (control body closed, expression incomplete)");
    // --------------------------------------------------------------------

    probe(
        "D13. IF cond   (between cond and THEN; END_IF follows)",
        r#"
FUNCTION main : DINT
VAR cond : BOOL; END_VAR
    IF cond
    END_IF;
END_FUNCTION
"#,
    );

    probe(
        "D14. IF cond TH   (partial THEN keyword)",
        r#"
FUNCTION main : DINT
VAR cond : BOOL; END_VAR
    IF cond TH
END_FUNCTION
"#,
    );

    probe(
        "D15. IF cond THEN   (awaiting first body statement; END_IF follows)",
        r#"
FUNCTION main : DINT
VAR cond : BOOL; END_VAR
    IF cond THEN
    END_IF;
END_FUNCTION
"#,
    );

    probe(
        "D16. FOR i := 1 TO   (range upper bound missing)",
        r#"
FUNCTION main : DINT
VAR i : DINT; END_VAR
    FOR i := 1 TO
END_FUNCTION
"#,
    );

    probe(
        "D17. WHILE   (condition position)",
        r#"
FUNCTION main : DINT
    WHILE
END_FUNCTION
"#,
    );

    // --------------------------------------------------------------------
    category("E. Expressions / body");
    // --------------------------------------------------------------------

    probe(
        "E18. a := my   (RHS partial identifier)",
        r#"
FUNCTION main : DINT
VAR a : DINT; my_var : DINT; END_VAR
    a := my
END_FUNCTION
"#,
    );

    probe(
        "E19. a +   (binary RHS missing)",
        r#"
FUNCTION main : DINT
VAR a : DINT; b : DINT; END_VAR
    b := a +
END_FUNCTION
"#,
    );

    probe(
        "E20. arr[   (array index, empty)",
        r#"
FUNCTION main : DINT
VAR arr : ARRAY[1..5] OF DINT; END_VAR
    arr[
END_FUNCTION
"#,
    );

    // --------------------------------------------------------------------
    category("F. Unclosed POU (EOF mid-pou, no END_*)");
    // --------------------------------------------------------------------

    probe(
        "F21. foo. inside an unclosed FUNCTION (no END_FUNCTION)",
        r#"
FUNCTION main : DINT
VAR foo : Point; END_VAR
    foo.
"#,
    );

    probe(
        "F22. VAR x :   (no END_VAR, no END_FUNCTION)",
        r#"
FUNCTION main : DINT
VAR x :
"#,
    );

    probe(
        "F23. TYPE Foo : STRUCT   (no END_STRUCT, no END_TYPE)",
        r#"
TYPE Foo : STRUCT
    a : DINT;
"#,
    );

    probe(
        "F24. FUNCTION_BLOCK B with body but no END_FUNCTION_BLOCK",
        r#"
FUNCTION_BLOCK B
VAR x : DINT; END_VAR
    x := 1;
"#,
    );

    // --------------------------------------------------------------------
    category("G. VAR_GLOBAL — partial / unclosed");
    // --------------------------------------------------------------------

    probe(
        "G25. VAR_GLOBAL x :   (then END_VAR)",
        r#"
VAR_GLOBAL
    x :
END_VAR
"#,
    );

    probe(
        "G26. VAR_GLOBAL x : MyT   (partial type, then END_VAR)",
        r#"
VAR_GLOBAL
    x : MyT
END_VAR
"#,
    );

    probe(
        "G27. VAR_GLOBAL x : DINT;   (no END_VAR)",
        r#"
VAR_GLOBAL
    x : DINT;
"#,
    );

    // --------------------------------------------------------------------
    category("H. Multi-POU with exactly one unclosed");
    // --------------------------------------------------------------------

    probe(
        "H28. unclosed POU at TOP, two complete POUs after",
        r#"
FUNCTION a : DINT
    x := 1;
FUNCTION b : DINT
END_FUNCTION
FUNCTION c : DINT
END_FUNCTION
"#,
    );

    probe(
        "H29. complete - UNCLOSED - complete",
        r#"
FUNCTION a : DINT
END_FUNCTION
FUNCTION b : DINT
    x := 1;
FUNCTION c : DINT
END_FUNCTION
"#,
    );

    probe(
        "H30. two complete POUs, then unclosed POU at BOTTOM",
        r#"
FUNCTION a : DINT
END_FUNCTION
FUNCTION b : DINT
END_FUNCTION
FUNCTION c : DINT
    x := 1;
"#,
    );

    probe(
        "H31. FB with unclosed METHOD followed by a complete METHOD",
        r#"
FUNCTION_BLOCK FB
METHOD a : DINT
    x := 1;
METHOD b : DINT
END_METHOD
METHOD c : DINT
END_METHOD
END_FUNCTION_BLOCK
"#,
    );

    // --------------------------------------------------------------------
    category("I. Unclosed control flow (inside otherwise complete POU)");
    // --------------------------------------------------------------------

    probe(
        "I31. IF cond THEN x := 1;   (no END_IF)",
        r#"
FUNCTION main : DINT
VAR cond : BOOL; x : DINT; END_VAR
    IF cond THEN
        x := 1;
END_FUNCTION
"#,
    );

    probe(
        "I32. FOR i := 1 TO 10 DO x := i;   (no END_FOR)",
        r#"
FUNCTION main : DINT
VAR i : DINT; x : DINT; END_VAR
    FOR i := 1 TO 10 DO
        x := i;
END_FUNCTION
"#,
    );

    probe(
        "I33. WHILE cond DO x := 1;   (no END_WHILE)",
        r#"
FUNCTION main : DINT
VAR cond : BOOL; x : DINT; END_VAR
    WHILE cond DO
        x := 1;
END_FUNCTION
"#,
    );

    probe(
        "I34. CASE x OF 1: y := 1;   (no END_CASE)",
        r#"
FUNCTION main : DINT
VAR x : DINT; y : DINT; END_VAR
    CASE x OF
        1: y := 1;
END_FUNCTION
"#,
    );

    // --------------------------------------------------------------------
    category("J. Punctuation gotchas");
    // --------------------------------------------------------------------

    probe(
        "J35. Missing ':' after CASE value (1 y := 1; instead of 1: y := 1;)",
        r#"
FUNCTION main : DINT
VAR x : DINT; y : DINT; END_VAR
    CASE x OF
        1 y := 1;
    END_CASE;
END_FUNCTION
"#,
    );

    probe(
        "J36. Missing ':' after type name (TYPE Foo STRUCT ...)",
        r#"
TYPE Foo STRUCT
    a : DINT;
END_STRUCT END_TYPE
"#,
    );

    probe(
        "J37. Missing ':' in struct field (a DINT; instead of a : DINT;)",
        r#"
TYPE Foo : STRUCT
    a DINT;
    b : DINT;
END_STRUCT END_TYPE
"#,
    );
}

// ============================================================================
// Annotator probe — run a subset of the partial-AST cases through the full
// parse → pre_process → index → annotate path. Verify the annotator does NOT
// panic on EmptyStatement and friends. For surviving annotations, dump each
// AstNode's annotation entry. The user's hypothesis: annotations on
// EmptyStatement are effectively None (the annotator skips silently).
// ============================================================================

/// Visitor that prints every AstNode it visits, alongside the annotation that
/// the AnnotationMap holds for it (if any). Indents by depth to show
/// containment.
struct AnnotationDumpVisitor<'a> {
    depth: usize,
    annotations: &'a AnnotationMapImpl,
}

impl AstVisitor for AnnotationDumpVisitor<'_> {
    fn visit(&mut self, node: &AstNode) {
        let indent = "  ".repeat(self.depth + 1);
        let kind_brief = brief_stmt(node);
        let ann = self.annotations.get(node);
        eprintln!("{indent}{kind_brief}  ann={ann:?}");
        self.depth += 1;
        node.walk(self);
        self.depth -= 1;
    }
}

/// One-line summary of an AstNode's `AstStatement` variant + a hint about
/// content. Compact debug — no trailing newlines, no nested structure.
fn brief_stmt(node: &AstNode) -> String {
    use plc_ast::ast::AstStatement;
    match node.get_stmt() {
        AstStatement::EmptyStatement(_) => "EmptyStatement".to_string(),
        AstStatement::DefaultValue(_) => "DefaultValue".to_string(),
        AstStatement::Literal(lit) => format!("Literal({lit:?})"),
        AstStatement::Identifier(name) => format!("Identifier({name:?})"),
        AstStatement::ReferenceExpr(_) => "ReferenceExpr".to_string(),
        AstStatement::BinaryExpression(b) => format!("BinaryExpression({:?})", b.operator),
        AstStatement::UnaryExpression(u) => format!("UnaryExpression({:?})", u.operator),
        AstStatement::Assignment(_) => "Assignment".to_string(),
        AstStatement::OutputAssignment(_) => "OutputAssignment".to_string(),
        AstStatement::RefAssignment(_) => "RefAssignment".to_string(),
        AstStatement::CallStatement(_) => "CallStatement".to_string(),
        AstStatement::ControlStatement(_) => "ControlStatement".to_string(),
        AstStatement::ExpressionList(items) => format!("ExpressionList({} items)", items.len()),
        AstStatement::ParenExpression(_) => "ParenExpression".to_string(),
        AstStatement::RangeStatement(_) => "RangeStatement".to_string(),
        AstStatement::CaseCondition(_) => "CaseCondition".to_string(),
        AstStatement::ReturnStatement(_) => "ReturnStatement".to_string(),
        AstStatement::ExitStatement(_) => "ExitStatement".to_string(),
        AstStatement::ContinueStatement(_) => "ContinueStatement".to_string(),
        AstStatement::Super(_) => "Super".to_string(),
        AstStatement::This => "This".to_string(),
        AstStatement::DirectAccess(_) => "DirectAccess".to_string(),
        AstStatement::HardwareAccess(_) => "HardwareAccess".to_string(),
        AstStatement::MultipliedStatement(_) => "MultipliedStatement".to_string(),
        AstStatement::VlaRangeStatement => "VlaRangeStatement".to_string(),
        AstStatement::JumpStatement(_) => "JumpStatement".to_string(),
        AstStatement::LabelStatement(_) => "LabelStatement".to_string(),
        AstStatement::AllocationStatement(_) => "AllocationStatement".to_string(),
    }
}

fn annotator_probe(label: &str, src: &str) {
    eprintln!("\n========================================");
    eprintln!("== ANN {label}");
    eprintln!("========================================");
    eprintln!("source:");
    for (i, line) in src.lines().enumerate() {
        eprintln!("  {i:>2} | {line}");
    }

    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(src.to_string().as_str(), id_provider.clone());

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        annotate_with_ids(&unit, &mut index, id_provider.clone())
    }));

    match result {
        Err(_) => {
            eprintln!("\n*** ANNOTATOR PANICKED ***");
        }
        Ok(annotations) => {
            eprintln!("\nannotator did not panic.");

            for (i, impl_) in unit.implementations.iter().enumerate() {
                eprintln!(
                    "\nimpl[{i}] for {} ({} stmts) — annotated tree:",
                    impl_.name,
                    impl_.statements.len()
                );
                let mut visitor = AnnotationDumpVisitor { depth: 0, annotations: &annotations };
                for stmt in &impl_.statements {
                    visitor.visit(stmt);
                }
            }

            if !unit.user_types.is_empty() {
                eprintln!("\nuser_types: {}", unit.user_types.len());
            }
            if !unit.global_vars.is_empty() {
                eprintln!("\nglobal_vars: {} blocks", unit.global_vars.len());
            }
        }
    }
}

#[test]
fn annotator_on_partial_ast() {
    eprintln!("\n\n############################################################");
    eprintln!("## ANNOTATOR PROBE — partial AST cases");
    eprintln!("############################################################");

    // A1 — EmptyStatement body. Hypothesis: annotator skips.
    annotator_probe(
        "A1. foo.  (member access collapses to EmptyStatement)",
        r#"
TYPE Point : STRUCT a : DINT; b : DINT; END_STRUCT END_TYPE
FUNCTION main : DINT
VAR foo : Point; END_VAR
    foo.
END_FUNCTION
"#,
    );

    // A6 — the smoking gun. AST shows clean foo.a := 1; — what does the
    // annotator do? Does it complain that Point doesn't have member `a`?
    annotator_probe(
        "A6. foo.  followed by valid stmt   (silent merge → foo.a := 1;)",
        r#"
TYPE Point : STRUCT a : DINT; b : DINT; END_STRUCT END_TYPE
FUNCTION main : DINT
VAR foo : Point; a : DINT; END_VAR
    foo.
    a := 1;
END_FUNCTION
"#,
    );

    // B6 — CallStatement with EmptyStatement parameters.
    annotator_probe(
        "B6. other(   (CallStatement, parameters=EmptyStatement)",
        r#"
FUNCTION other : DINT
VAR_INPUT x : DINT; END_VAR
END_FUNCTION
FUNCTION main : DINT
    other(
END_FUNCTION
"#,
    );

    // B8 — named-arg expression slot is EmptyStatement.
    annotator_probe(
        "B8. other(x :=   (Assignment.right = EmptyStatement)",
        r#"
FUNCTION other : DINT
VAR_INPUT x : DINT; END_VAR
END_FUNCTION
FUNCTION main : DINT
    other(x :=
END_FUNCTION
"#,
    );

    // D16 — ForLoopStatement with end: EmptyStatement.
    annotator_probe(
        "D16. FOR i := 1 TO   (range end = EmptyStatement)",
        r#"
FUNCTION main : DINT
VAR i : DINT; END_VAR
    FOR i := 1 TO
END_FUNCTION
"#,
    );

    // E20 — ReferenceExpr { kind: Index(EmptyStatement) }.
    annotator_probe(
        "E20. arr[   (Index payload = EmptyStatement)",
        r#"
FUNCTION main : DINT
VAR arr : ARRAY[1..5] OF DINT; END_VAR
    arr[
END_FUNCTION
"#,
    );

    // H29 — middle-unclosed POU: does annotator survive when one expected POU
    // is missing from the unit?
    annotator_probe(
        "H29. complete - UNCLOSED - complete   (middle POU swallowed)",
        r#"
FUNCTION a : DINT
END_FUNCTION
FUNCTION b : DINT
    x := 1;
FUNCTION c : DINT
END_FUNCTION
"#,
    );
}

// ============================================================================
// Lowering probe — run partial-AST cases through the FULL BuildPipeline
// (parse_lenient → index → annotate, with all 10 default mut_participants
// registered). The goal: confirm no lowering pass panics on partial AST
// before we commit further down the phase-13 path. If any do, we know which
// to harden.
// ============================================================================

fn lowering_probe(label: &str, src: &str) {
    use driver::pipelines::{BuildPipeline, Pipeline};
    use plc_diagnostics::diagnostics::Diagnostic;
    use plc_source::SourceCode;

    eprintln!("\n========================================");
    eprintln!("== LOW {label}");
    eprintln!("========================================");
    eprintln!("source:");
    for (i, line) in src.lines().enumerate() {
        eprintln!("  {i:>2} | {line}");
    }

    let sources = vec![SourceCode { source: src.to_string(), path: None }];
    let diagnostician = Diagnostician::buffered();

    let mut pipeline = match BuildPipeline::from_sources("lowering_probe", sources, diagnostician) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("\n*** from_sources failed: {e:?}");
            return;
        }
    };
    pipeline.register_default_mut_participants();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let parsed = pipeline.parse_lenient();
        let indexed = pipeline.index(parsed)?;
        let annotated = pipeline.annotate(indexed)?;
        Ok::<_, Diagnostic>(annotated)
    }));

    match result {
        Err(panic) => {
            let msg = panic
                .downcast_ref::<&str>()
                .map(|s| (*s).to_string())
                .or_else(|| panic.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<panic payload not a string>".to_string());
            eprintln!("\n*** LOWERING PANICKED: {msg}");
        }
        Ok(Err(e)) => {
            eprintln!("\nlowering returned Err (graceful): {}", e.get_message());
        }
        Ok(Ok(annotated)) => {
            eprintln!("\nlowering succeeded. {} units.", annotated.units.len());
        }
    }
}

#[test]
fn lowering_on_partial_ast() {
    eprintln!("\n\n############################################################");
    eprintln!("## LOWERING PROBE — partial AST cases run through full pipeline");
    eprintln!("############################################################");

    lowering_probe(
        "A1. foo.   (Member(EmptyStatement), base preserved)",
        r#"
TYPE Point : STRUCT a : DINT; b : DINT; END_STRUCT END_TYPE
FUNCTION main : DINT
VAR foo : Point; END_VAR
    foo.
END_FUNCTION
"#,
    );

    lowering_probe(
        "A6. foo. followed by valid stmt   (silent merge → foo.a := 1;)",
        r#"
TYPE Point : STRUCT a : DINT; b : DINT; END_STRUCT END_TYPE
FUNCTION main : DINT
VAR foo : Point; a : DINT; END_VAR
    foo.
    a := 1;
END_FUNCTION
"#,
    );

    lowering_probe(
        "B6. other(   (CallStatement with EmptyStatement parameters)",
        r#"
FUNCTION other : DINT
VAR_INPUT x : DINT; END_VAR
END_FUNCTION
FUNCTION main : DINT
    other(
END_FUNCTION
"#,
    );

    lowering_probe(
        "B8. other(x :=   (Assignment.right = EmptyStatement)",
        r#"
FUNCTION other : DINT
VAR_INPUT x : DINT; END_VAR
END_FUNCTION
FUNCTION main : DINT
    other(x :=
END_FUNCTION
"#,
    );

    lowering_probe(
        "D16. FOR i := 1 TO   (range end = EmptyStatement)",
        r#"
FUNCTION main : DINT
VAR i : DINT; END_VAR
    FOR i := 1 TO
END_FUNCTION
"#,
    );

    lowering_probe(
        "E20. arr[   (Index payload = EmptyStatement)",
        r#"
FUNCTION main : DINT
VAR arr : ARRAY[1..5] OF DINT; END_VAR
    arr[
END_FUNCTION
"#,
    );

    lowering_probe(
        "H29. complete - UNCLOSED - complete   (middle POU swallowed)",
        r#"
FUNCTION a : DINT
END_FUNCTION
FUNCTION b : DINT
    x := 1;
FUNCTION c : DINT
END_FUNCTION
"#,
    );

    lowering_probe(
        "PropertyLowerer surface — PROPERTY GET/SET on a partially-typed FB body",
        r#"
FUNCTION_BLOCK fbTest
VAR _prop : DINT; END_VAR

PROPERTY_GET myProp : DINT
    myProp := _prop;
END_PROPERTY

PROPERTY_SET myProp : DINT
    _prop := myProp;
END_PROPERTY

myProp := 10;
foo.
END_FUNCTION_BLOCK
"#,
    );

    lowering_probe(
        "InheritanceLowerer surface — Derived FB calls SUPER^.foo, mid-typing",
        r#"
FUNCTION_BLOCK Base
METHOD foo : DINT
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Derived EXTENDS Base
METHOD foo : DINT
    SUPER^.
END_METHOD
END_FUNCTION_BLOCK
"#,
    );

    lowering_probe(
        "C9. VAR x :   (Bucket 2 — placeholder Variable with empty type)",
        r#"
FUNCTION main : DINT
VAR
    x :
END_VAR
END_FUNCTION
"#,
    );

    lowering_probe(
        "G25. VAR_GLOBAL x :   (Bucket 2 at global scope)",
        r#"
VAR_GLOBAL
    x :
END_VAR

FUNCTION main : DINT
END_FUNCTION
"#,
    );

    lowering_probe(
        "H31. FB with unclosed METHOD before complete METHOD",
        r#"
FUNCTION_BLOCK FB
METHOD a : DINT
    x := 1;
METHOD b : DINT
END_METHOD
METHOD c : DINT
END_METHOD
END_FUNCTION_BLOCK
"#,
    );
}
