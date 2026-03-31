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
//!
//! # 3. For
//! Given
//! ```
//! FOR <ctrl> := <init> TO <final> [BY <steps>] DO
//!     <body>
//! END_FOR
//! ```
//! becomes
//! ```
//! alloca ran_once_N: bool;
//! alloca is_incrementing_N: bool;
//! <ctrl> := <init>;
//! is_incrementing_N := <step> > 0;
//!
//! WHILE TRUE DO
//!     IF ran_once_N THEN
//!         <ctrl> := <ctrl> + <step>;
//!     END_IF;
//!     ran_once_N := TRUE;
//!
//!     IF is_incrementing_N THEN
//!         IF <ctrl> > <final> THEN
//!             EXIT;
//!         END_IF;
//!     ELSE
//!         IF <ctrl> < <final> THEN
//!             EXIT;
//!         END_IF;
//!     END_IF;
//!
//!     <body>
//! END_WHILE
//! ```

use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit, Operator},
    control_statements::{ForLoopStatement, LoopStatement},
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

struct ForDesugarer {
    ids: IdProvider,
    counter: usize,

    /// Preamble statements followed by the replacement loop
    replacement: Option<Vec<AstNode>>,
}

impl LoopDesugarer {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn desugar(&self, units: &mut [CompilationUnit]) {
        let mut whiled = WhileDesugarer { ids: self.ids.clone() };
        let mut repeatd = RepeatDesugarer { ids: self.ids.clone(), counter: 0, replacement: None };
        let mut ford = ForDesugarer { ids: self.ids.clone(), counter: 0, replacement: None };

        for unit in units {
            whiled.visit_compilation_unit(unit);
            repeatd.visit_compilation_unit(unit);
            ford.visit_compilation_unit(unit);
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
        let (alloca, ran_once_ref) =
            helper::create_alloca(&mut self.ids, "BOOL", format!("ran_once_{}", self.counter));
        self.counter += 1;

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

impl AstVisitorMut for ForDesugarer {
    fn visit_statement_list(&mut self, stmts: &mut Vec<AstNode>) {
        let statements = std::mem::take(stmts);
        let mut new_statements = Vec::with_capacity(statements.len());

        for mut statement in statements {
            statement.walk(self);

            match self.replacement.take() {
                Some(replacement) => new_statements.extend(replacement),
                None => new_statements.push(statement),
            };
        }

        *stmts = new_statements;
    }

    // TODO: Sourcelocations are wrong
    fn visit_for_loop_statement(&mut self, stmt: &mut ForLoopStatement) {
        // First, visit the body itself to desugar nested loops (if any)
        self.visit_statement_list(&mut stmt.body);

        // Take the for loop components and original body.
        let counter = *std::mem::take(&mut stmt.counter);
        let start = *std::mem::take(&mut stmt.start);
        let end = *std::mem::take(&mut stmt.end);
        let by_step = stmt.by_step.take().map(|step| *step);
        let mut body = std::mem::take(&mut stmt.body);
        let loop_location = start.location.span(&stmt.end_location);

        // Create temporaries tracking whether the loop already ran and which comparison branch to use.
        let (ran_once_alloca, ran_once_ref) =
            helper::create_alloca(&mut self.ids, "BOOL", format!("ran_once_{}", self.counter));
        let (is_incrementing_alloca, is_incrementing_ref) =
            helper::create_alloca(&mut self.ids, "BOOL", format!("is_incrementing_{}", self.counter));
        self.counter += 1;

        // Normalize the step expression so omitted `BY` becomes a literal `1`.
        let has_explicit_step = by_step.is_some();
        let step = by_step.unwrap_or_else(|| helper::create_literal_integer(&mut self.ids, 1));
        let zero = helper::create_literal_integer(&mut self.ids, 0);

        // Increment the counter at the top of every iteration after the first one.
        let increment_assignment = AstFactory::create_assignment(
            counter.clone(),
            AstFactory::create_binary_expression(
                counter.clone(),
                Operator::Plus,
                step.clone(),
                self.ids.next_id(),
            ),
            self.ids.next_id(),
        );
        let increment_guard = helper::create_if_then(
            self.ids.clone(),
            ran_once_ref.clone(),
            vec![increment_assignment],
            SourceLocation::internal(),
        );

        // Mark that the loop has already executed once.
        let ran_once_assignment = AstFactory::create_assignment(
            ran_once_ref,
            AstFactory::create_literal(
                AstLiteral::Bool(true),
                SourceLocation::internal(),
                self.ids.next_id(),
            ),
            self.ids.next_id(),
        );

        // Compute the comparison direction once before entering the replacement loop.
        let is_incrementing_value = if has_explicit_step {
            AstFactory::create_binary_expression(step.clone(), Operator::Greater, zero, self.ids.next_id())
        } else {
            helper::create_literal_true(&mut self.ids, SourceLocation::internal())
        };
        let is_incrementing_assignment = AstFactory::create_assignment(
            is_incrementing_ref.clone(),
            is_incrementing_value,
            self.ids.next_id(),
        );

        // Exit once the counter has moved past the end bound for the chosen direction.
        let incrementing_exit = helper::create_if_then_exit(
            self.ids.clone(),
            AstFactory::create_binary_expression(
                counter.clone(),
                Operator::Greater,
                end.clone(),
                self.ids.next_id(),
            ),
            loop_location.clone(),
        );
        let decrementing_exit = helper::create_if_then_exit(
            self.ids.clone(),
            AstFactory::create_binary_expression(counter.clone(), Operator::Less, end, self.ids.next_id()),
            loop_location.clone(),
        );
        let bound_guard = helper::create_if_then_else(
            self.ids.clone(),
            is_incrementing_ref,
            vec![incrementing_exit],
            vec![decrementing_exit],
            SourceLocation::internal(),
        );

        // Prepend the desugared loop control flow ahead of the original body.
        body.insert(0, increment_guard);
        body.insert(1, ran_once_assignment);
        body.insert(2, bound_guard);

        debug_assert!(self.replacement.is_none());
        // Replace the original for loop with its preamble and a `WHILE TRUE` loop.
        self.replacement = Some(vec![
            ran_once_alloca,
            is_incrementing_alloca,
            AstFactory::create_assignment(counter, start, self.ids.next_id()),
            is_incrementing_assignment,
            helper::create_while_true_loop(&mut self.ids, body, loop_location.clone(), loop_location),
        ]);
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

    pub fn create_if_then_else(
        mut ids: IdProvider,
        condition: AstNode,
        body: Vec<AstNode>,
        else_body: Vec<AstNode>,
        location: SourceLocation,
    ) -> AstNode {
        AstFactory::create_if_statement(
            IfStatement {
                blocks: vec![ConditionalBlock { condition: Box::new(condition), body }],
                else_block: else_body,
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
        _location: SourceLocation,
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

    pub fn create_literal_integer(ids: &mut IdProvider, value: i128) -> AstNode {
        AstFactory::create_literal(AstLiteral::Integer(value), SourceLocation::internal(), ids.next_id())
    }

    pub fn create_alloca(ids: &mut IdProvider, ty: &str, name: String) -> (AstNode, AstNode) {
        let alloca = AstNode {
            stmt: AstStatement::AllocationStatement(Allocation {
                name: name.to_string(),
                reference_type: String::from(ty),
            }),
            id: ids.next_id(),
            location: SourceLocation::internal(),
            metadata: None,
        };
        let ident = AstFactory::create_member_reference(
            AstFactory::create_identifier(name, SourceLocation::internal(), ids.next_id()),
            None,
            ids.next_id(),
        );

        (alloca, ident)
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

    mod for_loops {
        #[test]
        fn default_step() {
            let source = r#"
                FUNCTION main
                    VAR
                        i : INT;
                        a, b : INT;
                    END_VAR

                    FOR i := 0 TO 10 DO
                        a := b;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := 0
            is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + 1
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > 10 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < 10 THEN
                        EXIT;
                    END_IF
                END_IF
                a := b
            END_WHILE
            ");
        }

        #[test]
        fn explicit_step_and_continue() {
            let source = r#"
                FUNCTION main
                    VAR
                        i, step, max, a, b : INT;
                    END_VAR

                    FOR i := a TO max BY step DO
                        b := a;
                        CONTINUE;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := a
            is_incrementing_0 := step > 0
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + step
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > max THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < max THEN
                        EXIT;
                    END_IF
                END_IF
                b := a
                CONTINUE;
                
            END_WHILE
            ");
        }

        #[test]
        fn zero_step() {
            let source = r#"
                FUNCTION main
                    VAR
                        i : INT;
                    END_VAR

                    FOR i := 1 TO 3 BY 0 DO
                        CONTINUE;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := 1
            is_incrementing_0 := 0 > 0
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + 0
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > 3 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < 3 THEN
                        EXIT;
                    END_IF
                END_IF
                CONTINUE;
                
            END_WHILE
            ");
        }

        #[test]
        fn explicit_positive_step_no_iteration_when_start_exceeds_end() {
            let source = r#"
                FUNCTION main
                    VAR
                        i : INT;
                    END_VAR

                    FOR i := 5 TO 1 BY 2 DO
                        EXIT;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := 5
            is_incrementing_0 := 2 > 0
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + 2
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > 1 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < 1 THEN
                        EXIT;
                    END_IF
                END_IF
                EXIT;
                
            END_WHILE
            ");
        }

        #[test]
        fn explicit_negative_step_no_iteration_when_start_is_below_end() {
            let source = r#"
                FUNCTION main
                    VAR
                        i : INT;
                    END_VAR

                    FOR i := 1 TO 5 BY -1 DO
                        EXIT;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := 1
            is_incrementing_0 := -1 > 0
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + -1
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > 5 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < 5 THEN
                        EXIT;
                    END_IF
                END_IF
                EXIT;
                
            END_WHILE
            ");
        }

        #[test]
        fn equal_bounds() {
            let source = r#"
                FUNCTION main
                    VAR
                        i, a : INT;
                    END_VAR

                    FOR i := 4 TO 4 DO
                        a := i;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := 4
            is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + 1
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > 4 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < 4 THEN
                        EXIT;
                    END_IF
                END_IF
                a := i
            END_WHILE
            ");
        }

        #[test]
        fn mutable_end_expression() {
            let source = r#"
                FUNCTION main
                    VAR
                        i, max : INT;
                    END_VAR

                    FOR i := 0 TO max DO
                        max := max - 1;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := 0
            is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + 1
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > max THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < max THEN
                        EXIT;
                    END_IF
                END_IF
                max := max - 1
            END_WHILE
            ");
        }

        #[test]
        fn mutable_step_expression() {
            let source = r#"
                FUNCTION main
                    VAR
                        i, step : INT;
                    END_VAR

                    FOR i := 0 TO 10 BY step DO
                        step := step + 1;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := 0
            is_incrementing_0 := step > 0
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + step
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > 10 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < 10 THEN
                        EXIT;
                    END_IF
                END_IF
                step := step + 1
            END_WHILE
            ");
        }

        #[test]
        fn continue_in_nested_if() {
            let source = r#"
                FUNCTION main
                    VAR
                        i, a, b, c, d : INT;
                    END_VAR

                    FOR i := a TO b DO
                        IF c > d THEN
                            CONTINUE;
                        END_IF
                        c := i;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := a
            is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + 1
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > b THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < b THEN
                        EXIT;
                    END_IF
                END_IF
                IF c > d THEN
                    CONTINUE;
                    
                END_IF
                c := i
            END_WHILE
            ");
        }

        #[test]
        fn exit_in_body() {
            let source = r#"
                FUNCTION main
                    VAR
                        i, a, b : INT;
                    END_VAR

                    FOR i := a TO b DO
                        EXIT;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_0: BOOL
            alloca is_incrementing_0: BOOL
            i := a
            is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF ran_once_0 THEN
                    i := i + 1
                END_IF
                ran_once_0 := TRUE
                IF is_incrementing_0 THEN
                    IF i > b THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < b THEN
                        EXIT;
                    END_IF
                END_IF
                EXIT;
                
            END_WHILE
            ");
        }

        #[test]
        fn nested_with_explicit_steps() {
            let source = r#"
                FUNCTION main
                    VAR
                        i, j, start, finish, outer_step, inner_step, a : INT;
                    END_VAR

                    FOR i := start TO finish BY outer_step DO
                        FOR j := 10 TO 0 BY inner_step DO
                            a := j;
                        END_FOR
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_1: BOOL
            alloca is_incrementing_1: BOOL
            i := start
            is_incrementing_1 := outer_step > 0
            WHILE TRUE DO
                IF ran_once_1 THEN
                    i := i + outer_step
                END_IF
                ran_once_1 := TRUE
                IF is_incrementing_1 THEN
                    IF i > finish THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < finish THEN
                        EXIT;
                    END_IF
                END_IF
                alloca ran_once_0: BOOL
                alloca is_incrementing_0: BOOL
                j := 10
                is_incrementing_0 := inner_step > 0
                WHILE TRUE DO
                    IF ran_once_0 THEN
                        j := j + inner_step
                    END_IF
                    ran_once_0 := TRUE
                    IF is_incrementing_0 THEN
                        IF j > 0 THEN
                            EXIT;
                        END_IF
                    ELSE
                        IF j < 0 THEN
                            EXIT;
                        END_IF
                    END_IF
                    a := j
                END_WHILE
            END_WHILE
            ");
        }

        #[test]
        fn nested() {
            let source = r#"
                FUNCTION main
                    VAR
                        i, j, a, b, c : INT;
                    END_VAR

                    FOR i := a TO b BY c DO
                        FOR j := 0 TO 2 DO
                            a := j;
                        END_FOR
                        b := i;
                    END_FOR
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @"
            alloca ran_once_1: BOOL
            alloca is_incrementing_1: BOOL
            i := a
            is_incrementing_1 := c > 0
            WHILE TRUE DO
                IF ran_once_1 THEN
                    i := i + c
                END_IF
                ran_once_1 := TRUE
                IF is_incrementing_1 THEN
                    IF i > b THEN
                        EXIT;
                    END_IF
                ELSE
                    IF i < b THEN
                        EXIT;
                    END_IF
                END_IF
                alloca ran_once_0: BOOL
                alloca is_incrementing_0: BOOL
                j := 0
                is_incrementing_0 := TRUE
                WHILE TRUE DO
                    IF ran_once_0 THEN
                        j := j + 1
                    END_IF
                    ran_once_0 := TRUE
                    IF is_incrementing_0 THEN
                        IF j > 2 THEN
                            EXIT;
                        END_IF
                    ELSE
                        IF j < 2 THEN
                            EXIT;
                        END_IF
                    END_IF
                    a := j
                END_WHILE
                b := i
            END_WHILE
            ");
        }
    }
}
