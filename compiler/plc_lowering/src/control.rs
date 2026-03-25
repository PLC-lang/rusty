use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit},
    control_statements::{ConditionalBlock, IfStatement, LoopStatement},
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

/// Public entry point for all control-flow desugaring passes.
/// Orchestrates sub-desugarers (while, and later for, repeat, etc.)
pub struct ControlDesugarer {
    ids: IdProvider,
}

/// Desugars `WHILE <cond> DO <body> END_WHILE` into:
/// ```st
/// WHILE TRUE DO
///   IF NOT <cond> THEN EXIT; END_IF
///   <body>
/// END_WHILE
/// ```
struct WhileDesugarer {
    ids: IdProvider,
}

/// Desugars `REPEAT <body> UNTIL <cond> END_REPEAT` into:
/// ```st
/// alloca __repeat_check_N : BOOL;
/// WHILE TRUE DO
///   IF __repeat_check_N THEN
///     IF <cond> THEN EXIT; END_IF
///   END_IF
///   __repeat_check_N := TRUE;
///   <body>
/// END_WHILE
/// ```
///
/// A synthetic boolean variable (defaulting to `FALSE`) skips the exit-condition
/// check on the first iteration, preserving the do-while "execute at least once"
/// semantics. On subsequent iterations the flag is `TRUE`, so the UNTIL condition
/// is evaluated normally. Keeping the original condition inside a nested `IF`
/// also ensures that later lowering passes can expand `<cond>` into helper
/// statements without accidentally evaluating it on the first iteration. This
/// approach is safe in the presence of `CONTINUE` (which jumps to the top of the
/// WHILE and re-evaluates the guard) and `EXIT` (which leaves the WHILE entirely).
///
/// Because a single REPEAT statement expands into multiple sibling statements
/// (alloca + while loop), this desugarer uses the drain-loop pattern from
/// `visit_statement_list` with a small per-statement expansion buffer (see
/// `src/lowering/polymorphism/dispatch/interface.rs` for the same technique).
struct RepeatDesugarer {
    ids: IdProvider,

    /// Tracks statements emitted before or instead of the current statement.
    expansion: StatementExpansion,

    /// Monotonic counter for generating unique flag variable names
    /// (`__repeat_check_0`, `__repeat_check_1`, ...).
    counter: usize,
}

/// Tracks synthesized nodes emitted while rewriting the current statement.
/// `preamble`, when set, is emitted before the original statement.
/// `replacement`, when set, replaces the original statement entirely.
struct StatementExpansion {
    preamble: Option<AstNode>,
    replacement: Option<AstNode>,
}

impl ControlDesugarer {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    /// Runs all control-flow desugaring passes over the given
    /// compilation units.
    pub fn desugar(&self, units: &mut [CompilationUnit]) {
        let mut while_desugarer = WhileDesugarer::new(self.ids.clone());
        let mut repeat_desugarer = RepeatDesugarer::new(self.ids.clone());

        for unit in units {
            while_desugarer.visit_compilation_unit(unit);
            repeat_desugarer.visit_compilation_unit(unit);
        }
    }
}

impl WhileDesugarer {
    fn new(ids: IdProvider) -> Self {
        Self { ids }
    }
}

impl AstVisitorMut for WhileDesugarer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self);
    }

    fn visit_while_statement(&mut self, stmt: &mut LoopStatement) {
        // Use the source condition's location for all synthesized nodes.
        let location = stmt.condition.get_location();

        // Replace `WHILE <cond>` with `WHILE TRUE` and keep the source condition.
        let transformed_condition = Box::new(helper::create_true_literal(&location, &mut self.ids));
        let source_condition = *std::mem::replace(&mut stmt.condition, transformed_condition);

        // Desugar nested while loops in the source body before wrapping it with the exit guard.
        let mut lowered_source_body = std::mem::take(&mut stmt.body);
        self.visit_statement_list(&mut lowered_source_body);

        // Prepend `IF NOT <cond> THEN EXIT; END_IF` to preserve while semantics.
        let transformed_body = {
            let guard_condition =
                AstFactory::create_not_expression(source_condition, location.clone(), self.ids.next_id());
            let exit_guard = helper::create_exit_guard(guard_condition, &location, &mut self.ids);

            // `IF NOT <cond> THEN EXIT; END_IF` followed by the lowered source body.
            std::iter::once(exit_guard).chain(lowered_source_body).collect::<Vec<_>>()
        };

        stmt.body = transformed_body;
    }
}

impl RepeatDesugarer {
    fn new(ids: IdProvider) -> Self {
        Self { ids, expansion: StatementExpansion::new(), counter: 0 }
    }
}

impl AstVisitorMut for RepeatDesugarer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self);
    }

    fn visit_statement_list(&mut self, stmts: &mut Vec<AstNode>) {
        // Save any expansion state accumulated by the parent context. Without
        // this, the drain loop below would clear it on its first iteration,
        // losing preamble or replacement generated outside this statement list.
        let saved_expansion = self.expansion.save();

        let original = std::mem::take(stmts);
        let mut output = Vec::with_capacity(original.len());

        for mut stmt in original {
            self.expansion.clear_current();

            stmt.walk(self);

            // Flush any accumulated expansion before or instead of the statement.
            self.expansion.flush_into(&mut output, stmt);
        }

        *stmts = output;

        // Restore the parent's expansion state so it gets flushed at the
        // correct level.
        self.expansion.restore(saved_expansion);
    }

    fn visit_repeat_statement(&mut self, stmt: &mut LoopStatement) {
        // Use the source UNTIL condition's location for all synthesized repeat nodes.
        let location = stmt.condition.get_location();
        let end_location = stmt.end_location.clone();

        // Reserve the skip-flag name before recursing so numbering follows source order.
        let flag_name = format!("__repeat_check_{}", self.counter);
        self.counter += 1;

        // Desugar nested repeat loops in the source body before wrapping it.
        let mut lowered_source_body = std::mem::take(&mut stmt.body);
        self.visit_statement_list(&mut lowered_source_body);

        // Move the source condition out of the loop so we can reuse it in the exit check.
        let source_condition = std::mem::take(&mut stmt.condition);

        // Build the `alloca __repeat_check_N : BOOL;` preamble.
        let preamble_alloca = helper::create_bool_alloca(&flag_name, &location, &mut self.ids);

        // Lower the body to the flag-gated exit check, flag assignment, then source body.
        let transformed_body = {
            // IF <cond> THEN EXIT END_IF
            let exit_guard = helper::create_exit_guard(*source_condition, &location, &mut self.ids);

            // IF __repeat_check_N THEN ... END_IF
            let flag_condition = Box::new(helper::create_ref(&flag_name, &location, &mut self.ids));
            let flag_guard = AstFactory::create_if_statement(
                IfStatement {
                    blocks: vec![ConditionalBlock { condition: flag_condition, body: vec![exit_guard] }],
                    else_block: vec![],
                    end_location: SourceLocation::internal(),
                },
                location.clone(),
                self.ids.next_id(),
            );

            // __repeat_check_N := TRUE;
            let flag_set = helper::create_bool_assignment(&flag_name, true, &location, &mut self.ids);

            // IF __repeat_check_N THEN
            //     IF <cond> THEN
            //         EXIT
            //     END_IF
            // END_IF
            //
            // __repeat_check := TRUE
            // <actual body>
            [flag_guard, flag_set].into_iter().chain(lowered_source_body).collect::<Vec<_>>()
        };

        // Replace `REPEAT` with `alloca ...; WHILE TRUE DO ... END_WHILE`.
        let replacement_loop =
            helper::create_while_true(transformed_body, &location, end_location, &mut self.ids);

        self.expansion.set_preamble(preamble_alloca);
        self.expansion.set_replacement(replacement_loop);
    }
}

impl StatementExpansion {
    fn new() -> Self {
        Self { preamble: None, replacement: None }
    }

    fn save(&mut self) -> Self {
        Self { preamble: self.preamble.take(), replacement: self.replacement.take() }
    }

    fn restore(&mut self, saved: Self) {
        *self = saved;
    }

    fn clear_current(&mut self) {
        self.preamble = None;
        self.replacement = None;
    }

    fn set_preamble(&mut self, preamble: AstNode) {
        self.preamble = Some(preamble);
    }

    fn set_replacement(&mut self, replacement: AstNode) {
        self.replacement = Some(replacement);
    }

    fn flush_into(&mut self, output: &mut Vec<AstNode>, original: AstNode) {
        if let Some(preamble) = self.preamble.take() {
            output.push(preamble);
        }

        match self.replacement.take() {
            Some(replacement) => output.push(replacement),
            None => output.push(original),
        }
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

    pub(super) fn create_true_literal(location: &SourceLocation, ids: &mut IdProvider) -> AstNode {
        AstFactory::create_literal(AstLiteral::new_bool(true), location.clone(), ids.next_id())
    }

    pub(super) fn create_bool_alloca(name: &str, location: &SourceLocation, ids: &mut IdProvider) -> AstNode {
        AstNode {
            stmt: AstStatement::AllocationStatement(Allocation {
                name: name.to_string(),
                reference_type: "BOOL".to_string(),
            }),
            id: ids.next_id(),
            location: location.clone(),
            metadata: None,
        }
    }

    pub(super) fn create_ref(name: &str, location: &SourceLocation, ids: &mut IdProvider) -> AstNode {
        AstFactory::create_member_reference(
            AstFactory::create_identifier(name, location.clone(), ids.next_id()),
            None,
            ids.next_id(),
        )
    }

    pub(super) fn create_exit_guard(
        condition: AstNode,
        location: &SourceLocation,
        ids: &mut IdProvider,
    ) -> AstNode {
        let exit_stmt = AstFactory::create_exit_statement(location.clone(), ids.next_id());

        AstFactory::create_if_statement(
            IfStatement {
                blocks: vec![ConditionalBlock { condition: Box::new(condition), body: vec![exit_stmt] }],
                else_block: vec![],
                end_location: SourceLocation::internal(),
            },
            location.clone(),
            ids.next_id(),
        )
    }

    pub(super) fn create_bool_assignment(
        name: &str,
        value: bool,
        location: &SourceLocation,
        ids: &mut IdProvider,
    ) -> AstNode {
        AstFactory::create_assignment(
            create_ref(name, location, ids),
            AstFactory::create_literal(AstLiteral::new_bool(value), location.clone(), ids.next_id()),
            ids.next_id(),
        )
    }

    pub(super) fn create_while_true(
        body: Vec<AstNode>,
        location: &SourceLocation,
        end_location: SourceLocation,
        ids: &mut IdProvider,
    ) -> AstNode {
        AstFactory::create_while_statement(
            LoopStatement { condition: Box::new(create_true_literal(location, ids)), body, end_location },
            location.clone(),
            ids.next_id(),
        )
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
        let ids = IdProvider::default();

        // Parse without any pipeline participants so we get an
        // un-lowered AST.
        let (mut unit, _diagnostics) = plc::parser::parse(
            plc::lexer::lex_with_ids(src, ids.clone(), SourceLocationFactory::internal(src)),
            LinkageType::Internal,
            "test.st",
        );

        // Apply our desugarer.
        let desugarer = ControlDesugarer::new(ids);
        desugarer.desugar(std::slice::from_mut(&mut unit));

        let stmt = &unit.implementations[0].statements[stmt_index];
        AstSerializer::format(stmt)
    }

    /// Parses ST source, applies the ControlDesugarer, and returns the
    /// formatted AST of all statements in the first implementation.
    fn desugar_and_format_all(src: &str) -> String {
        let ids = IdProvider::default();

        let (mut unit, _diagnostics) = plc::parser::parse(
            plc::lexer::lex_with_ids(src, ids.clone(), SourceLocationFactory::internal(src)),
            LinkageType::Internal,
            "test.st",
        );

        let desugarer = ControlDesugarer::new(ids);
        desugarer.desugar(std::slice::from_mut(&mut unit));

        unit.implementations[0].statements.iter().map(AstSerializer::format).collect::<Vec<_>>().join("\n")
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

    mod repeat_desugaring {
        use super::*;

        #[test]
        fn empty() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                END_VAR
                    REPEAT
                    UNTIL x > 10
                    END_REPEAT
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format_all(src), @"
            alloca __repeat_check_0: BOOL
            WHILE TRUE DO
                IF __repeat_check_0 THEN
                    IF x > 10 THEN
                        EXIT;
                    END_IF
                END_IF
                __repeat_check_0 := TRUE
            END_WHILE
            ");
        }

        #[test]
        fn continue_only() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                END_VAR
                    REPEAT
                        IF TRUE THEN
                            CONTINUE;
                        END_IF
                    UNTIL x > 10
                    END_REPEAT
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format_all(src), @"
            alloca __repeat_check_0: BOOL
            WHILE TRUE DO
                IF __repeat_check_0 THEN
                    IF x > 10 THEN
                        EXIT;
                    END_IF
                END_IF
                __repeat_check_0 := TRUE
                IF TRUE THEN
                    CONTINUE;
                END_IF
            END_WHILE
            ");
        }

        #[test]
        fn call_in_until() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                END_VAR
                    REPEAT
                        IF TRUE THEN
                            CONTINUE;
                        END_IF
                    UNTIL someCondition()
                    END_REPEAT
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format_all(src), @"
            alloca __repeat_check_0: BOOL
            WHILE TRUE DO
                IF __repeat_check_0 THEN
                    IF someCondition() THEN
                        EXIT;
                    END_IF
                END_IF
                __repeat_check_0 := TRUE
                IF TRUE THEN
                    CONTINUE;
                END_IF
            END_WHILE
            ");
        }

        #[test]
        fn call_with_or_in_until() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                END_VAR
                    REPEAT
                        IF TRUE THEN
                            CONTINUE;
                        END_IF
                    UNTIL someCondition() OR anotherCondition()
                    END_REPEAT
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format_all(src), @"
            alloca __repeat_check_0: BOOL
            WHILE TRUE DO
                IF __repeat_check_0 THEN
                    IF someCondition() OR anotherCondition() THEN
                        EXIT;
                    END_IF
                END_IF
                __repeat_check_0 := TRUE
                IF TRUE THEN
                    CONTINUE;
                END_IF
            END_WHILE
            ");
        }

        #[test]
        fn inside_if_block() {
            let src = r#"
                PROGRAM main
                VAR
                    flag : BOOL;
                    x : INT;
                END_VAR
                    IF flag THEN
                        REPEAT
                            x := x + 1;
                        UNTIL x > 3
                        END_REPEAT
                    END_IF
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format(src, 0), @r"
            IF flag THEN
                alloca __repeat_check_0: BOOL
                WHILE TRUE DO
                    IF __repeat_check_0 THEN
                        IF x > 3 THEN
                            EXIT;
                        END_IF
                    END_IF
                    __repeat_check_0 := TRUE
                    x := x + 1
                END_WHILE
            END_IF
            ");
        }

        #[test]
        fn while_with_repeat_in_body() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                    y : INT;
                END_VAR
                    WHILE x > 0 DO
                        REPEAT
                            y := y + 1;
                        UNTIL y > 3
                        END_REPEAT
                    END_WHILE
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format(src, 0), @r"
            WHILE TRUE DO
                IF NOT x > 0 THEN
                    EXIT;
                END_IF
                alloca __repeat_check_0: BOOL
                WHILE TRUE DO
                    IF __repeat_check_0 THEN
                        IF y > 3 THEN
                            EXIT;
                        END_IF
                    END_IF
                    __repeat_check_0 := TRUE
                    y := y + 1
                END_WHILE
            END_WHILE
            ");
        }

        #[test]
        fn sibling_repeats() {
            let src = r#"
                PROGRAM main
                VAR
                    x : INT;
                    y : INT;
                END_VAR
                    REPEAT
                        x := x + 1;
                    UNTIL x > 3
                    END_REPEAT

                    REPEAT
                        y := y + 1;
                    UNTIL y > 4
                    END_REPEAT
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format_all(src), @"
            alloca __repeat_check_0: BOOL
            WHILE TRUE DO
                IF __repeat_check_0 THEN
                    IF x > 3 THEN
                        EXIT;
                    END_IF
                END_IF
                __repeat_check_0 := TRUE
                x := x + 1
            END_WHILE
            alloca __repeat_check_1: BOOL
            WHILE TRUE DO
                IF __repeat_check_1 THEN
                    IF y > 4 THEN
                        EXIT;
                    END_IF
                END_IF
                __repeat_check_1 := TRUE
                y := y + 1
            END_WHILE
            ");
        }

        #[test]
        fn nesting() {
            let src = r#"
                PROGRAM main
                VAR
                    x, y, z : INT;
                END_VAR
                    REPEAT
                        // Level 1
                        x := x + 1;
                        IF X = 5 THEN
                            CONTINUE;
                        END_IF

                        // Level 2
                        REPEAT
                            y := x + y + 1;
                            IF Y = 10 THEN
                                CONTINUE;
                            END_IF
                        UNTIL x * y = 20
                        END_REPEAT
                    UNTIL x = 10
                    END_REPEAT
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format_all(src), @"
            alloca __repeat_check_0: BOOL
            WHILE TRUE DO
                IF __repeat_check_0 THEN
                    IF x = 10 THEN
                        EXIT;
                    END_IF
                END_IF
                __repeat_check_0 := TRUE
                x := x + 1
                IF X = 5 THEN
                    CONTINUE;
                END_IF
                alloca __repeat_check_1: BOOL
                WHILE TRUE DO
                    IF __repeat_check_1 THEN
                        IF x * y = 20 THEN
                            EXIT;
                        END_IF
                    END_IF
                    __repeat_check_1 := TRUE
                    y := x + y + 1
                    IF Y = 10 THEN
                        CONTINUE;
                    END_IF
                END_WHILE
            END_WHILE
            ");
        }

        #[test]
        fn deep_nesting() {
            let src = r#"
                PROGRAM main
                VAR
                    x, y, z : INT;
                END_VAR
                    REPEAT
                        // Level 1
                        x := x + 1;
                        IF X = 5 THEN
                            CONTINUE;
                        END_IF

                        // Level 2
                        REPEAT
                            y := x + y + 1;
                            IF Y = 10 THEN
                                CONTINUE;
                            END_IF

                            // Level 3
                            REPEAT
                                doWork();
                            UNTIL someCondition() AND x * y * z = 30
                            END_REPEAT
                        UNTIL x * y = 20
                        END_REPEAT

                        // Also Level 2
                        REPEAT
                            callExactlyOnce();
                        UNTIL TRUE
                        END_REPEAT
                    UNTIL x = 10
                    END_REPEAT
                END_PROGRAM
            "#;

            assert_snapshot!(desugar_and_format_all(src), @"
            alloca __repeat_check_0: BOOL
            WHILE TRUE DO
                IF __repeat_check_0 THEN
                    IF x = 10 THEN
                        EXIT;
                    END_IF
                END_IF
                __repeat_check_0 := TRUE
                x := x + 1
                IF X = 5 THEN
                    CONTINUE;
                END_IF
                alloca __repeat_check_1: BOOL
                WHILE TRUE DO
                    IF __repeat_check_1 THEN
                        IF x * y = 20 THEN
                            EXIT;
                        END_IF
                    END_IF
                    __repeat_check_1 := TRUE
                    y := x + y + 1
                    IF Y = 10 THEN
                        CONTINUE;
                    END_IF
                    alloca __repeat_check_2: BOOL
                    WHILE TRUE DO
                        IF __repeat_check_2 THEN
                            IF someCondition() AND x * y * z = 30 THEN
                                EXIT;
                            END_IF
                        END_IF
                        __repeat_check_2 := TRUE
                        doWork()
                    END_WHILE
                END_WHILE
                alloca __repeat_check_3: BOOL
                WHILE TRUE DO
                    IF __repeat_check_3 THEN
                        IF TRUE THEN
                            EXIT;
                        END_IF
                    END_IF
                    __repeat_check_3 := TRUE
                    callExactlyOnce()
                END_WHILE
            END_WHILE
            ");
        }
    }
}
