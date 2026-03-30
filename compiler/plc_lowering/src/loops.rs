//! Module canonicalizing loops into while loops.
//!
//! This module is responsible for desugaring loops into `WHILE TRUE (* ... *) END_WHILE` constructs.
//!
//! # 1. While
//! While loops are the easiest candidate to desugar. We simply take the condition and move it into the body
//! with a prepended if-guard to break out of the loop if the condition is false. That is given
//! ```
//! WHILE <cond> DO
//!     <body>
//! END_WHILE
//! ```
//! we transform it into
//! ```
//! WHILE TRUE DO
//!     IF NOT <cond> THEN
//!         EXIT;
//!     END_IF
//!
//!    <body>
//! END_WHILE
//! ```

use plc_ast::{
    ast::{AstFactory, CompilationUnit, Operator},
    control_statements::LoopStatement,
    mut_visitor::AstVisitorMut,
    provider::IdProvider,
};

pub struct LoopDesugarer {
    ids: IdProvider,
}

struct WhileDesugarer {
    ids: IdProvider,
}

impl LoopDesugarer {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn desugar(&self, units: &mut [CompilationUnit]) {
        let mut whiled = WhileDesugarer { ids: self.ids.clone() };

        for unit in units {
            whiled.visit_compilation_unit(unit);
        }
    }
}

impl AstVisitorMut for WhileDesugarer {
    fn visit_while_loop_statement(&mut self, stmt: &mut LoopStatement) {
        // First, visit the body itself to desugar nested loops (if any)
        self.visit_statement_list(&mut stmt.body);

        // Clone the previous conditions location for the new loop condition and if guard
        let prev_cond_loc = stmt.condition.location.clone();

        // Swap the condition with a literal `TRUE`
        let prev_cond = std::mem::replace(
            &mut stmt.condition,
            Box::new(helper::create_literal_true(&mut self.ids, prev_cond_loc.clone())),
        );

        // Create a negated if guard to break from the loop
        // Note: We want internal locations here, given these statements are invisible to the user
        let negated_cond = AstFactory::create_unary_expression(
            Operator::Not,
            *prev_cond,
            prev_cond_loc.clone(),
            self.ids.next_id(),
        );
        let if_guard = helper::create_if_then_exit_statement(&mut self.ids, negated_cond, prev_cond_loc);

        // Prepend the body with the if guard
        stmt.body.insert(0, if_guard);
    }
}

mod helper {
    use plc_ast::{
        ast::{AstFactory, AstNode},
        control_statements::{ConditionalBlock, IfStatement},
        literals::AstLiteral,
        provider::IdProvider,
    };
    use plc_source::source_location::SourceLocation;

    pub fn create_literal_true(ids: &mut IdProvider, location: SourceLocation) -> AstNode {
        AstFactory::create_literal(AstLiteral::Bool(true), location, ids.next_id())
    }

    pub fn create_if_then_exit_statement(
        ids: &mut IdProvider,
        condition: AstNode,
        location: SourceLocation,
    ) -> AstNode {
        AstFactory::create_if_statement(
            IfStatement {
                blocks: vec![ConditionalBlock {
                    condition: Box::new(condition),
                    body: vec![AstFactory::create_exit_statement(location.clone(), ids.next_id())],
                }],
                else_block: Vec::new(),
                end_location: location.clone(),
            },
            location,
            ids.next_id(),
        )
    }
}

#[cfg(test)]
mod tests {
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

    fn serialize(source: impl Into<SourceCode>) -> String {
        let (_, project) = parse_and_annotate("unit-test", vec![source.into()]).unwrap();
        let unit = project.units[0].get_unit();

        let mut result = Vec::new();
        let statements = &unit.implementations.iter().find(|it| &it.name == "main").unwrap().statements;

        for statement in statements {
            result.push(statement.as_string());
        }

        result.join("\n")
    }

    mod while_loops {
        #[test]
        fn empty() {
            let source = r#"
                FUNCTION main
                    VAR
                        a, b, c : INT;
                    END_VAR

                    WHILE 1 > 2 DO
                        a := b;
                        b := c;
                        c := a;
                    END_WHILE
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            WHILE TRUE DO
                IF NOT 1 > 2 THEN
                    EXIT;
                END_IF
                a := b
                b := c
                c := a
            END_WHILE
            ");
        }

        #[test]
        fn and_condition() {
            let source = r#"
                FUNCTION main
                    VAR
                        a, b, c : INT;
                    END_VAR

                    WHILE 1 > 2 AND 2 < 3 DO
                        a := b;
                        b := c;
                        c := a;
                    END_WHILE
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            WHILE TRUE DO
                IF NOT 1 > 2 AND 2 < 3 THEN
                    EXIT;
                END_IF
                a := b
                b := c
                c := a
            END_WHILE
            ");
        }

        #[test]
        fn and_or_condition() {
            let source = r#"
                FUNCTION main
                    VAR
                        a, b, c : INT;
                    END_VAR

                    WHILE (1 > 2 AND 2 < 3) OR 3 = 4 DO
                        a := b;
                        b := c;
                        c := a;
                    END_WHILE
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            WHILE TRUE DO
                IF NOT (1 > 2 AND 2 < 3) OR 3 = 4 THEN
                    EXIT;
                END_IF
                a := b
                b := c
                c := a
            END_WHILE
            ");
        }

        #[test]
        fn nested() {
            let source = r#"
                FUNCTION main
                    VAR
                        a, b, c : INT;
                    END_VAR

                    WHILE 1 > 2 DO
                        a := b;
                        b := c;

                        WHILE 3 < 4 DO
                            c := a;
                        END_WHILE
                    END_WHILE
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            WHILE TRUE DO
                IF NOT 1 > 2 THEN
                    EXIT;
                END_IF
                a := b
                b := c
                WHILE TRUE DO
                    IF NOT 3 < 4 THEN
                        EXIT;
                    END_IF
                    c := a
                END_WHILE
            END_WHILE
            ");
        }
    }

    mod repeat_loops {}

    mod for_loops {}
}
