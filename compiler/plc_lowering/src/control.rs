use plc_ast::{
    ast::{AstFactory, CompilationUnit},
    control_statements::{ConditionalBlock, IfStatement, LoopStatement},
    literals::AstLiteral,
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

/// Public entry point for all control-flow desugaring passes.
/// Orchestrates sub-desugarers (while, and later for, repeat, etc.)
pub struct ControlDesugarer {
    id_provider: IdProvider,
}

/// Desugars `WHILE <cond> DO <body> END_WHILE` into:
/// ```st
/// WHILE TRUE DO
///   IF NOT <cond> THEN EXIT; END_IF
///   <body>
/// END_WHILE
/// ```
struct WhileDesugarer {
    id_provider: IdProvider,
}

impl ControlDesugarer {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { id_provider }
    }

    /// Runs all control-flow desugaring passes over the given
    /// compilation units.
    pub fn desugar(&self, units: &mut [CompilationUnit]) {
        let mut while_desugarer = WhileDesugarer { id_provider: self.id_provider.clone() };

        for unit in units {
            while_desugarer.visit_compilation_unit(unit);
        }

        // Future: ForDesugarer, RepeatDesugarer, etc.
    }
}

impl AstVisitorMut for WhileDesugarer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self);
    }

    fn visit_while_statement(&mut self, stmt: &mut LoopStatement) {
        // Use the original condition's location for all synthesized nodes.
        let location = stmt.condition.get_location();

        // Swap the original condition with `TRUE`, i.e. `WHILE <cond>` becomes `WHILE TRUE`.
        let original_condition = *std::mem::replace(
            &mut stmt.condition,
            Box::new(AstFactory::create_literal(
                AstLiteral::new_bool(true),
                location.clone(),
                self.id_provider.next_id(),
            )),
        );
        let original_body = std::mem::take(&mut stmt.body);

        // Negate the original condition, i.e. `<cond>` becomes `NOT <cond>`.
        let not_condition = AstFactory::create_not_expression(
            original_condition,
            location.clone(),
            self.id_provider.next_id(),
        );

        // Build the exit guard: `IF NOT <cond> THEN EXIT; END_IF`.
        let exit_stmt = AstFactory::create_exit_statement(
            location.clone(),
            self.id_provider.next_id(),
        );
        let guard = AstFactory::create_if_statement(
            IfStatement {
                blocks: vec![ConditionalBlock {
                    condition: Box::new(not_condition),
                    body: vec![exit_stmt],
                }],
                else_block: vec![],
                end_location: SourceLocation::internal(),
            },
            location,
            self.id_provider.next_id(),
        );

        // Prepend the guard to the original body and walk to desugar any nested while loops.
        stmt.body = std::iter::once(guard).chain(original_body).collect();
        self.visit_statement_list(&mut stmt.body);
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use plc_ast::{ast::LinkageType, provider::IdProvider, ser::AstSerializer};
    use plc_source::source_location::SourceLocationFactory;

    use super::ControlDesugarer;

    /// Parses ST source, applies the ControlDesugarer, and returns the
    /// formatted AST of the n-th statement in the first implementation.
    fn desugar_and_format(src: &str, stmt_index: usize) -> String {
        let id_provider = IdProvider::default();

        // Parse without any pipeline participants so we get an
        // un-lowered AST.
        let (mut unit, _diagnostics) = plc::parser::parse(
            plc::lexer::lex_with_ids(src, id_provider.clone(), SourceLocationFactory::internal(src)),
            LinkageType::Internal,
            "test.st",
        );

        // Apply our desugarer.
        let desugarer = ControlDesugarer::new(id_provider);
        desugarer.desugar(std::slice::from_mut(&mut unit));

        let stmt = &unit.implementations[0].statements[stmt_index];
        AstSerializer::format(stmt)
    }

    mod while_desugaring {
        use super::*;

        #[test]
        fn empty_endless_loop() {
            let src = r#"
                PROGRAM main
                    WHILE TRUE DO
                    END_WHILE
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format(src, 0), @r"
            WHILE TRUE DO
                IF NOT TRUE THEN
                    EXIT;
                END_IF
            END_WHILE
            ");
        }

        #[test]
        fn empty_while_with_condition() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                END_VAR
                    WHILE x > 5 DO
                    END_WHILE
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format(src, 0), @r"
            WHILE TRUE DO
                IF NOT x > 5 THEN
                    EXIT;
                END_IF
            END_WHILE
            ");
        }

        #[test]
        fn while_with_condition_and_body() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                END_VAR
                    WHILE x > 0 DO
                        x := x - 1;
                    END_WHILE
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format(src, 0), @r"
            WHILE TRUE DO
                IF NOT x > 0 THEN
                    EXIT;
                END_IF
                x := x - 1
            END_WHILE
            ");
        }

        #[test]
        fn while_with_if_in_body() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                    y : INT;
                END_VAR
                    WHILE x > 0 DO
                        IF x > 5 THEN
                            y := 1;
                        ELSE
                            y := 2;
                        END_IF
                        x := x - 1;
                    END_WHILE
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format(src, 0), @r"
            WHILE TRUE DO
                IF NOT x > 0 THEN
                    EXIT;
                END_IF
                IF x > 5 THEN
                    y := 1
                ELSE
                    y := 2
                END_IF
                x := x - 1
            END_WHILE
            ");
        }

        #[test]
        fn triple_nested_while() {
            let src = r#"
                PROGRAM main
                VAR
                    a : INT;
                    b : INT;
                    c : INT;
                END_VAR
                    WHILE a > 0 DO
                        WHILE b > 0 DO
                            WHILE c > 0 DO
                                c := c - 1;
                            END_WHILE
                            b := b - 1;
                        END_WHILE
                        a := a - 1;
                    END_WHILE
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format(src, 0), @r"
            WHILE TRUE DO
                IF NOT a > 0 THEN
                    EXIT;
                END_IF
                WHILE TRUE DO
                    IF NOT b > 0 THEN
                        EXIT;
                    END_IF
                    WHILE TRUE DO
                        IF NOT c > 0 THEN
                            EXIT;
                        END_IF
                        c := c - 1
                    END_WHILE
                    b := b - 1
                END_WHILE
                a := a - 1
            END_WHILE
            ");
        }

        #[test]
        fn while_inside_if_block() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                    flag : BOOL;
                END_VAR
                    IF flag THEN
                        WHILE x > 0 DO
                            x := x - 1;
                        END_WHILE
                    END_IF
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format(src, 0), @r"
            IF flag THEN
                WHILE TRUE DO
                    IF NOT x > 0 THEN
                        EXIT;
                    END_IF
                    x := x - 1
                END_WHILE
            END_IF
            ");
        }
    }
}
