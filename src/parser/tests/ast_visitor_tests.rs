use plc_ast::{
    ast::LinkageType,
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocationFactory;

use crate::{lexer, parser};

#[derive(Default)]
struct IdentifierCollector {
    identifiers: Vec<String>,
}

impl AstVisitor for IdentifierCollector {
    fn visit_identifier(&mut self, stmt: &str, _node: &plc_ast::ast::AstNode) {
        self.identifiers.push(stmt.to_string());
    }
}

fn get_character_range(start: char, end: char) -> Vec<String> {
    (start as u8..=end as u8).map(|c| c as char).map(|c| c.to_string()).collect()
}

fn collect_identifiers(src: &str) -> IdentifierCollector {
    let id_provider = IdProvider::default();
    let (compilation_unit, _) = parser::parse(
        lexer::lex_with_ids(src, id_provider.clone(), SourceLocationFactory::internal(src)),
        LinkageType::Internal,
        "test.st",
    );

    let mut visitor = IdentifierCollector::default();

    for st in &compilation_unit.implementations[0].statements {
        visitor.visit(st);
    }
    visitor.identifiers.sort();
    visitor
}

#[test]
fn test_visit_arithmetic_expressions() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a;
            b;
            c := NOT d;
            e := f MOD g;
            h := (i / j / k - (l + m)) * n;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'n'), visitor.identifiers);
}

#[test]
fn test_visit_expression_list() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a,b,c;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'c'), visitor.identifiers);
}

#[test]
fn test_if_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            IF a THEN
                b := c;
            ELSIF d THEN
                e := f;
            ELSE
                g := h;
            END_IF;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'h'), visitor.identifiers);
}

#[test]
fn test_visit_for_loop_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            FOR a := b TO c BY d DO
                e;
                f;
            END_FOR;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'f'), visitor.identifiers);
}

#[test]
fn test_visit_while_loop_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            WHILE a < b DO
                c;
                d;
            END_WHILE;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'd'), visitor.identifiers);
}
#[test]
fn test_visit_repeat_loop_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            REPEAT
                a;
                b;
            UNTIL c > d;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'd'), visitor.identifiers);
}

#[test]
fn test_visit_case_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            CASE a OF
                b:
                    c;
                    d;
                e, f:
                    g;
                    h;
                ELSE
                    i;
                    j;
            END_CASE;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'j'), visitor.identifiers);
}

#[test]
fn test_visit_multiplied_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            3(a+b);
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);
}

#[test]
fn test_visist_array_expressions() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a[b];
            c[d,e+f];
            g[h+i][j+k];
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'k'), visitor.identifiers);
}

#[test]
fn test_visit_range_statement() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a..b;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);
}

#[test]
fn test_visit_assignment_expressions() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a := b;
            c => d;
            e =>;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'e'), visitor.identifiers);
}

#[test]
fn test_visit_call_statements() {
    let visitor = collect_identifiers(
        "
        PROGRAM prg
            a();
            b(c,d);
            e(f:=(g), h=>i);
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'i'), visitor.identifiers);
}

#[test]
fn test_visit_return_statement() {
    let visitor = collect_identifiers(
        "
        FUNCTION prg : INT
            RETURN a + b;
        END_PROGRAM",
    );
    assert_eq!(get_character_range('a', 'b'), visitor.identifiers);
}

struct AssignmentCounter {
    count: usize,
}

impl AstVisitor for AssignmentCounter {
    fn visit_assignment(&mut self, stmt: &plc_ast::ast::Assignment, _node: &plc_ast::ast::AstNode) {
        self.count += 1;
        stmt.walk(self)
    }

    fn visit_output_assignment(&mut self, stmt: &plc_ast::ast::Assignment, _node: &plc_ast::ast::AstNode) {
        self.count += 1;
        stmt.walk(self)
    }
}

#[test]
fn test_count_assignments() {
    let id_provider = IdProvider::default();
    let (compilation_unit, _) = parser::parse(
        lexer::lex_with_ids(
            "
            PROGRAM prg
                a := b;
                c => d;
                e := f;
                foo(a := baz(x := 2, z => 3));
            END_PROGRAM",
            id_provider.clone(),
            SourceLocationFactory::internal(""),
        ),
        LinkageType::Internal,
        "test.st",
    );

    let mut visitor = AssignmentCounter { count: 0 };

    for st in &compilation_unit.implementations[0].statements {
        visitor.visit(st);
    }
    assert_eq!(6, visitor.count);
}
