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
//!
//! # 2. Repeat
//! Repeats are while-do loops, given
//! ```
//! REPEAT
//!     <body>
//! UNTIL <cond>
//! END_REPEAT
//! ```
//! we want to desugar into
//! ```
//! WHILE TRUE DO
//!     <body>
//!
//!     IF <cond> THEN
//!         EXIT
//!     END_IF
//! END_WHILE
//! ```
//! However, this will not work if the body contains a `CONTINUE` statement, as the `IF <cond>` check will be
//! skipped and the loop will run endlessly. As such the final desugared form is
//! ```
//! alloca ran_once_N: bool;
//! WHILE TRUE DO
//!     IF ran_once_N THEN
//!         IF <cond> THEN
//!             EXIT
//!         END_IF
//!     END_IF
//!
//!     ran_once_N := TRUE;
//!     <body>
//! END_WHILE
//! ```

use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit, Operator},
    control_statements::LoopStatement,
    literals::AstLiteral,
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

pub struct LoopDesugarer {
    ids: IdProvider,
}

struct WhileDesugarer {
    ids: IdProvider,
}

struct RepeatDesugarer {
    ids: IdProvider,
    counter: usize,

    /// A preamble and its replacement loop
    replacement: Option<(AstNode, AstNode)>,
}

impl LoopDesugarer {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn desugar(&self, units: &mut [CompilationUnit]) {
        let mut whiled = WhileDesugarer { ids: self.ids.clone() };
        let mut repeatd = RepeatDesugarer { ids: self.ids.clone(), counter: 0, replacement: None };

        for unit in units {
            whiled.visit_compilation_unit(unit);
            repeatd.visit_compilation_unit(unit);
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
        let negated_cond = AstFactory::create_unary_expression(
            Operator::Not,
            *prev_cond,
            prev_cond_loc.clone(),
            self.ids.next_id(),
        );
        let if_guard = helper::create_if_then_exit(self.ids.clone(), negated_cond, prev_cond_loc);

        // Prepend the body with the if guard
        stmt.body.insert(0, if_guard);
    }
}

impl AstVisitorMut for RepeatDesugarer {
    fn visit_statement_list(&mut self, stmts: &mut Vec<AstNode>) {
        let statements = std::mem::take(stmts);
        let mut new_statements = Vec::with_capacity(statements.len());

        for mut statement in statements {
            statement.walk(self);

            match self.replacement.take() {
                Some((alloca, while_loop)) => {
                    new_statements.push(alloca);
                    new_statements.push(while_loop);
                }

                None => new_statements.push(statement),
            };
        }

        *stmts = new_statements;
    }

    // TODO: Sourcelocations are wrong
    fn visit_repeat_loop_statement(&mut self, stmt: &mut LoopStatement) {
        // First, visit the body itself to desugar nested loops (if any)
        self.visit_statement_list(&mut stmt.body);

        // Take the repeat condition and body
        let cond = std::mem::take(&mut stmt.condition);
        let mut body = std::mem::take(&mut stmt.body);
        let cond_location = cond.location.clone();

        // Create a temporary variable to track first iteration
        let alloca_name = format!("ran_once_{}", self.counter);
        let alloca = helper::create_alloca(&mut self.ids, "BOOL", alloca_name.clone());
        self.counter += 1;

        // Create the reference identifier of temporary variable
        let ran_once_ref = AstFactory::create_member_reference(
            AstFactory::create_identifier(alloca_name, SourceLocation::internal(), self.ids.next_id()),
            None,
            self.ids.next_id(),
        );

        // Create the the if guards, that is
        // ```
        // IF ran_once_N THEN
        //     IF <cond> THEN
        //         EXIT
        //     END_IF
        // END_IF
        // ```
        let if_guard = helper::create_if_then(
            self.ids.clone(),
            ran_once_ref.clone(),
            vec![helper::create_if_then_exit(self.ids.clone(), *cond, cond_location.clone())],
            SourceLocation::internal(),
        );

        // Create the `ran_once_N := TRUE` before the actual body to flag iteration ran at least once
        let ran_once_assignment = AstFactory::create_assignment(
            ran_once_ref,
            AstFactory::create_literal(
                AstLiteral::Bool(true),
                SourceLocation::internal(),
                self.ids.next_id(),
            ),
            self.ids.next_id(),
        );

        // Prepend the if guard and `ran_once_N := TRUE` assignment before the actual body
        body.insert(0, if_guard);
        body.insert(1, ran_once_assignment);

        debug_assert!(self.replacement.is_none());
        self.replacement = Some((
            alloca,
            helper::create_while_true_loop(&mut self.ids, body, cond_location.clone(), cond_location),
        ));
    }
}

mod helper {
    use plc_ast::{
        ast::{Allocation, AstFactory, AstNode, AstStatement},
        control_statements::{ConditionalBlock, IfStatement, LoopStatement},
        literals::AstLiteral,
        provider::IdProvider,
    };
    use plc_source::source_location::SourceLocation;

    pub fn create_literal_true(ids: &mut IdProvider, location: SourceLocation) -> AstNode {
        AstFactory::create_literal(AstLiteral::Bool(true), location, ids.next_id())
    }

    pub fn create_if_then(
        mut ids: IdProvider,
        condition: AstNode,
        body: Vec<AstNode>,
        location: SourceLocation,
    ) -> AstNode {
        AstFactory::create_if_statement(
            IfStatement {
                blocks: vec![ConditionalBlock { condition: Box::new(condition), body }],
                else_block: Vec::new(),
                end_location: location.clone(),
            },
            location,
            ids.next_id(),
        )
    }

    pub fn create_if_then_exit(mut ids: IdProvider, condition: AstNode, location: SourceLocation) -> AstNode {
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

    pub fn create_while_true_loop(
        ids: &mut IdProvider,
        body: Vec<AstNode>,
        cond_location: SourceLocation,
        location: SourceLocation,
    ) -> AstNode {
        let statement = LoopStatement {
            condition: Box::new(AstFactory::create_literal(
                AstLiteral::Bool(true),
                cond_location,
                ids.next_id(),
            )),
            body,
            end_location: SourceLocation::internal(), // TODO: What location do we use here?
        };

        // TODO: Also what location here?
        AstFactory::create_while_statement(statement, SourceLocation::internal(), ids.next_id())
    }

    pub fn create_alloca(ids: &mut IdProvider, ty: &str, name: String) -> AstNode {
        AstNode {
            stmt: AstStatement::AllocationStatement(Allocation {
                name: name.to_string(),
                reference_type: String::from(ty),
            }),
            id: ids.next_id(),
            location: SourceLocation::internal(),
            metadata: None,
        }
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

                        WHILE 3 < 4 DO
                            c := a;
                        END_WHILE

                        b := c;
                    END_WHILE
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            WHILE TRUE DO
                IF NOT 1 > 2 THEN
                    EXIT;
                END_IF
                a := b
                WHILE TRUE DO
                    IF NOT 3 < 4 THEN
                        EXIT;
                    END_IF
                    c := a
                END_WHILE
                b := c
            END_WHILE
            ");
        }
    }

    mod repeat_loops {
        #[test]
        fn empty() {
            let source = r#"
                FUNCTION main
                    VAR
                        a, b, c : INT;
                    END_VAR

                    REPEAT
                        a := b;
                        b := c;
                        c := a;
                    UNTIL 1 > 2
                    END_REPEAT
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            WHILE TRUE DO
                IF ran_once_0 THEN
                    IF 1 > 2 THEN
                        EXIT;
                    END_IF
                END_IF
                ran_once_0 := TRUE
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

                    REPEAT
                        a := b;
                        b := c;
                        c := a;
                    UNTIL 1 > 2 AND 2 < 3
                    END_REPEAT
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            WHILE TRUE DO
                IF ran_once_0 THEN
                    IF 1 > 2 AND 2 < 3 THEN
                        EXIT;
                    END_IF
                END_IF
                ran_once_0 := TRUE
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

                    REPEAT
                        a := b;
                        b := c;
                        c := a;
                    UNTIL (1 > 2 AND 2 < 3) OR 3 = 4
                    END_REPEAT
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            WHILE TRUE DO
                IF ran_once_0 THEN
                    IF (1 > 2 AND 2 < 3) OR 3 = 4 THEN
                        EXIT;
                    END_IF
                END_IF
                ran_once_0 := TRUE
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

                    REPEAT
                        a := b;
                        REPEAT
                            b := c;
                        UNTIL 3 < 4
                        END_REPEAT
                        c := a;
                    UNTIL 1 > 2
                    END_REPEAT
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_1: BOOL
            WHILE TRUE DO
                IF ran_once_1 THEN
                    IF 1 > 2 THEN
                        EXIT;
                    END_IF
                END_IF
                ran_once_1 := TRUE
                a := b
                alloca ran_once_0: BOOL
                WHILE TRUE DO
                    IF ran_once_0 THEN
                        IF 3 < 4 THEN
                            EXIT;
                        END_IF
                    END_IF
                    ran_once_0 := TRUE
                    b := c
                END_WHILE
                c := a
            END_WHILE
            ");
        }
    }

    mod for_loops {}
}
