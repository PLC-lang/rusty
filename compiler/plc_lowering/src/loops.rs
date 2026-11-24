//! Canonical loop lowering for the Structured Text AST.
//!
//! This module rewrites all loop constructs into a canonical core based on `WHILE TRUE DO ... END_WHILE`.
//!
//! ## `WHILE`
//!
//! A `WHILE` loop checks its condition before each iteration:
//!
//! ```st
//! WHILE <cond> DO
//!     <body>
//! END_WHILE
//! ```
//!
//! It is rewritten to:
//!
//! ```st
//! WHILE TRUE DO
//!     IF NOT <cond> THEN
//!         EXIT;
//!     END_IF
//!
//!     <body>
//! END_WHILE
//! ```
//!
//! This preserves the original pre-test semantics while making the loop body
//! explicit.
//!
//! ## `REPEAT`
//!
//! A `REPEAT` loop executes its body first and checks the condition afterwards:
//!
//! ```st
//! REPEAT
//!     <body>
//! UNTIL <cond>
//! END_REPEAT
//! ```
//!
//! A naive lowering to:
//!
//! ```st
//! WHILE TRUE DO
//!     <body>
//!     IF <cond> THEN
//!         EXIT;
//!     END_IF
//! END_WHILE
//! ```
//!
//! is incorrect in the presence of `CONTINUE`, because a `CONTINUE` inside `<body>` would skip the condition
//! check entirely. To preserve post-test semantics, this module introduces a synthetic boolean temporary:
//!
//! ```st
//! alloca __ran_once_N : BOOL;
//! WHILE TRUE DO
//!     IF __ran_once_N THEN
//!         IF <cond> THEN
//!             EXIT;
//!         END_IF
//!     END_IF
//!
//!     __ran_once_N := TRUE;
//!     <body>
//! END_WHILE
//! ```
//!
//! The first iteration always executes the body. From the second iteration on, the original `UNTIL` condition
//! is checked before re-entering the body.
//!
//! ## `FOR`
//!
//! A counted loop:
//!
//! ```st
//! FOR <ctrl> := <init> TO <final> [BY <step>] DO
//!     <body>
//! END_FOR
//! ```
//!
//! is lowered to a canonical `WHILE TRUE` loop with three synthetic temporaries:
//!
//! - `__ran_once_N` tracks whether the first iteration has already happened
//! - `__is_incrementing_N` remembers whether the step is positive
//! - `__past_end_N` records whether the counter value of this pass is past the end bound
//!
//! ```st
//! alloca __ran_once_N : BOOL;
//! alloca __is_incrementing_N : BOOL;
//! alloca __past_end_N : BOOL;
//!
//! <ctrl> := <init>;
//! __is_incrementing_N := <step> > 0;
//!
//! WHILE TRUE DO
//!     IF __ran_once_N THEN
//!         IF __is_incrementing_N THEN
//!             __past_end_N := <ctrl> + <step> > <final>;
//!         ELSE
//!             __past_end_N := <ctrl> + <step> < <final>;
//!         END_IF
//!         <ctrl> := <ctrl> + <step>;
//!     ELSIF __is_incrementing_N THEN
//!         __past_end_N := <ctrl> > <final>;
//!     ELSE
//!         __past_end_N := <ctrl> < <final>;
//!     END_IF
//!     __ran_once_N := TRUE;
//!
//!     IF __past_end_N THEN
//!         EXIT;
//!     END_IF
//!
//!     <body>
//! END_WHILE
//! ```
//!
//! The bound check uses `<ctrl> + <step>` before the assignment truncates it to the counter's width, so a
//! narrow counter whose next value leaves its type still ends the loop. The sum has the type that the resolver
//! gives to the addition (at least 32 bits, unless the counter and the step are both bit strings). If the sum
//! does not fit that type, it wraps before the comparison, so a 32-bit or 64-bit counter, or a bit-string
//! counter with a bit-string step, can still loop without end at its limit. The counter is assigned on the last
//! iteration too, so it keeps the value that ended the loop.
//!
//! Every copy of `<ctrl>`, `<step>` and `<final>` gets fresh node ids, so the resolver types each use on its
//! own. The sum and its comparison are internal and report no conversion warnings; the first-iteration
//! comparisons get the location from the counter to the end value and report them.
//!
//! If `BY` is omitted, the step defaults to `1`. The step and end expressions stay in the lowered tree, so they
//! are evaluated again on every iteration. From the second iteration on, the step is evaluated twice, once for
//! the bound check and once for the update.
//!
//! ## Generated temporaries
//!
//! Compiler-generated names such as `__ran_once_N`, `__is_incrementing_N` and `__past_end_N` are unique per
//! desugared loop.
//! They are ordinary AST allocations used only to encode loop semantics explicitly for later stages.

use std::{
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit, Operator},
    control_statements::{ForLoopStatement, LoopStatement},
    mut_visitor::{clone_with_new_ids, AstVisitorMut, WalkerMut},
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
    counter: Rc<AtomicUsize>,

    /// A preamble and its replacement loop
    replacement: Option<(AstNode, AstNode)>,
}

struct ForDesugarer {
    ids: IdProvider,
    counter: Rc<AtomicUsize>,

    /// Preamble statements followed by the replacement loop
    replacement: Option<Vec<AstNode>>,
}

impl LoopDesugarer {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn desugar(&self, units: &mut [CompilationUnit]) {
        let counter = Rc::new(AtomicUsize::new(0));

        let mut whiled = WhileDesugarer { ids: self.ids.clone() };
        let mut repeatd =
            RepeatDesugarer { ids: self.ids.clone(), counter: counter.clone(), replacement: None };
        let mut ford = ForDesugarer { ids: self.ids.clone(), counter, replacement: None };

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

        // Preserve the original condition location so stepping can still land on the user-written
        // `WHILE <cond>` header, but make the canonical loop scaffolding internal.
        let prev_cond_loc = stmt.condition.location.clone();

        // Swap the condition with a synthetic literal `TRUE`.
        let prev_cond = std::mem::replace(
            &mut stmt.condition,
            Box::new(helper::create_internal_literal_true(&mut self.ids)),
        );

        // Hide the canonical backedge from source-level debugging.
        stmt.end_location = SourceLocation::internal();

        // Create a synthetic guard whose wrapper is internal, while the reused user condition keeps
        // its original location.
        let negated_cond =
            AstFactory::create_unary_expression(Operator::Not, *prev_cond, prev_cond_loc, self.ids.next_id());
        let if_guard = helper::create_if_then_exit(self.ids.clone(), negated_cond);

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
                Some((alloca, mut while_loop)) => {
                    while_loop.location = statement.location.clone();

                    new_statements.push(alloca);
                    new_statements.push(while_loop);
                }

                None => new_statements.push(statement),
            };
        }

        *stmts = new_statements;
    }

    fn visit_repeat_loop_statement(&mut self, stmt: &mut LoopStatement) {
        // First, visit the body itself to desugar nested loops (if any)
        self.visit_statement_list(&mut stmt.body);

        // Take the repeat condition and body.
        let cond = std::mem::take(&mut stmt.condition);
        let mut body = std::mem::take(&mut stmt.body);

        // Create a temporary variable to track first iteration.
        let counter = self.counter.fetch_add(1, Ordering::SeqCst);
        let (alloca, ran_once_ref) =
            helper::create_alloca(&mut self.ids, "BOOL", format!("__ran_once_{counter}"));

        // Only the user-authored `UNTIL <cond>` check should stay visible in debug info. The
        // surrounding gating `IF __ran_once_N THEN ...` is synthetic and therefore internal.
        let if_guard = helper::create_if_then(
            self.ids.clone(),
            ran_once_ref.clone(),
            vec![helper::create_if_then_exit(self.ids.clone(), *cond)],
            SourceLocation::internal(),
        );

        // Synthetic `__ran_once_N := TRUE` should not inherit any user source location.
        let ran_once_assignment = helper::create_internal_assignment(
            ran_once_ref,
            helper::create_internal_literal_true(&mut self.ids),
            &mut self.ids,
        );

        // Prepend the if guard and `__ran_once_N := TRUE` assignment before the actual body.
        body.insert(0, if_guard);
        body.insert(1, ran_once_assignment);

        debug_assert!(self.replacement.is_none());
        self.replacement = Some((alloca, helper::create_while_true_loop(&mut self.ids, body)));
    }
}

impl AstVisitorMut for ForDesugarer {
    fn visit_statement_list(&mut self, stmts: &mut Vec<AstNode>) {
        let statements = std::mem::take(stmts);
        let mut new_statements = Vec::with_capacity(statements.len());

        for mut statement in statements {
            statement.walk(self);

            match self.replacement.take() {
                Some(mut replacement) => {
                    if let Some(while_loop) = replacement.last_mut() {
                        while_loop.location = statement.location.clone();
                    }
                    new_statements.extend(replacement)
                }
                None => new_statements.push(statement),
            };
        }

        *stmts = new_statements;
    }

    fn visit_for_loop_statement(&mut self, stmt: &mut ForLoopStatement) {
        // First, visit the body itself to desugar nested loops (if any)
        self.visit_statement_list(&mut stmt.body);

        // Take the for loop components and original body.
        let counter = *std::mem::take(&mut stmt.counter);
        let start = *std::mem::take(&mut stmt.start);
        let end = *std::mem::take(&mut stmt.end);
        let by_step = stmt.by_step.take().map(|step| *step);
        let mut body = std::mem::take(&mut stmt.body);

        // Create temporaries tracking whether the loop already ran, which comparison branch to use and
        // whether the counter moved past the end bound.
        let num = self.counter.fetch_add(1, Ordering::SeqCst);
        let (ran_once_alloca, ran_once_ref) =
            helper::create_alloca(&mut self.ids, "BOOL", format!("__ran_once_{num}"));
        let (is_incrementing_alloca, is_incrementing_ref) =
            helper::create_alloca(&mut self.ids, "BOOL", format!("__is_incrementing_{num}"));
        let (past_end_alloca, past_end_ref) =
            helper::create_alloca(&mut self.ids, "BOOL", format!("__past_end_{num}"));

        // Normalize the step expression so omitted `BY` becomes a literal `1`.
        let has_explicit_step = by_step.is_some();
        let step = by_step.unwrap_or_else(|| helper::create_literal_integer(&mut self.ids, 1));
        let zero = helper::create_literal_integer(&mut self.ids, 0);

        // From the second iteration on, compare `<ctrl> + <step>` with the end bound before the
        // assignment truncates the sum to the counter's width, then step the counter. The sum and its
        // comparison are internal, like the update, so they report no conversion warnings; the
        // first-iteration comparisons get the location from the counter to the end value and report
        // them. Every copy of the counter, step and end gets fresh ids, so the resolver types each use
        // on its own.
        let bound_location = counter.get_location().span(&end.get_location());
        let next_value = |ids: &mut IdProvider| {
            helper::create_internal_binary_expression(
                clone_with_new_ids(&counter, ids),
                Operator::Plus,
                clone_with_new_ids(&step, ids),
                ids,
            )
        };
        let (next_incrementing, next_decrementing) = (next_value(&mut self.ids), next_value(&mut self.ids));
        let next_bound_check = helper::create_if_then_else(
            self.ids.clone(),
            is_incrementing_ref.clone(),
            vec![helper::create_bound_check(
                past_end_ref.clone(),
                next_incrementing,
                Operator::Greater,
                clone_with_new_ids(&end, &mut self.ids),
                (bound_location.clone(), SourceLocation::internal()),
                &mut self.ids,
            )],
            vec![helper::create_bound_check(
                past_end_ref.clone(),
                next_decrementing,
                Operator::Less,
                clone_with_new_ids(&end, &mut self.ids),
                (bound_location.clone(), SourceLocation::internal()),
                &mut self.ids,
            )],
            SourceLocation::internal(),
        );
        let increment_assignment = helper::create_internal_assignment(
            clone_with_new_ids(&counter, &mut self.ids),
            next_value(&mut self.ids),
            &mut self.ids,
        );

        // The first iteration compares the start value itself.
        let first_incrementing_check = helper::create_bound_check(
            past_end_ref.clone(),
            clone_with_new_ids(&counter, &mut self.ids),
            Operator::Greater,
            clone_with_new_ids(&end, &mut self.ids),
            (bound_location.clone(), bound_location.clone()),
            &mut self.ids,
        );
        let first_decrementing_check = helper::create_bound_check(
            past_end_ref.clone(),
            clone_with_new_ids(&counter, &mut self.ids),
            Operator::Less,
            end,
            (bound_location.clone(), bound_location),
            &mut self.ids,
        );
        let bound_check = helper::create_if_chain(
            self.ids.clone(),
            vec![
                (ran_once_ref.clone(), vec![next_bound_check, increment_assignment]),
                (is_incrementing_ref.clone(), vec![first_incrementing_check]),
            ],
            vec![first_decrementing_check],
            SourceLocation::internal(),
        );

        // Mark that the loop has already executed once.
        let ran_once_assignment = helper::create_internal_assignment(
            ran_once_ref,
            helper::create_internal_literal_true(&mut self.ids),
            &mut self.ids,
        );

        // Compute the comparison direction once before entering the replacement loop.
        let is_incrementing_value = if has_explicit_step {
            helper::create_internal_binary_expression(
                clone_with_new_ids(&step, &mut self.ids),
                Operator::Greater,
                zero,
                &mut self.ids,
            )
        } else {
            helper::create_internal_literal_true(&mut self.ids)
        };
        let is_incrementing_assignment = helper::create_internal_assignment(
            is_incrementing_ref.clone(),
            is_incrementing_value,
            &mut self.ids,
        );

        // Exit once the counter has moved past the end bound for the chosen direction.
        let past_end_exit = helper::create_if_then_exit(self.ids.clone(), past_end_ref);

        // Prepend the desugared loop control flow ahead of the original body.
        body.insert(0, bound_check);
        body.insert(1, ran_once_assignment);
        body.insert(2, past_end_exit);

        debug_assert!(self.replacement.is_none());
        // Replace the original for loop with its preamble and a `WHILE TRUE` loop.
        self.replacement = Some(vec![
            ran_once_alloca,
            is_incrementing_alloca,
            past_end_alloca,
            AstFactory::create_assignment(counter, start, self.ids.next_id()),
            is_incrementing_assignment,
            helper::create_while_true_loop(&mut self.ids, body),
        ]);
    }
}

pub(crate) mod helper {
    use plc_ast::{
        ast::{Allocation, AstFactory, AstNode, AstStatement, Operator},
        control_statements::{ConditionalBlock, IfStatement, LoopStatement},
        literals::AstLiteral,
        provider::IdProvider,
    };
    use plc_source::source_location::SourceLocation;

    pub fn create_internal_literal_true(ids: &mut IdProvider) -> AstNode {
        AstFactory::create_literal(AstLiteral::Bool(true), SourceLocation::internal(), ids.next_id())
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
        ids: IdProvider,
        condition: AstNode,
        body: Vec<AstNode>,
        else_body: Vec<AstNode>,
        location: SourceLocation,
    ) -> AstNode {
        create_if_chain(ids, vec![(condition, body)], else_body, location)
    }

    /// Creates `IF <c1> THEN <b1> ELSIF <c2> THEN <b2> ... ELSE <else_body> END_IF`.
    pub fn create_if_chain(
        mut ids: IdProvider,
        blocks: Vec<(AstNode, Vec<AstNode>)>,
        else_body: Vec<AstNode>,
        location: SourceLocation,
    ) -> AstNode {
        let blocks = blocks
            .into_iter()
            .map(|(condition, body)| ConditionalBlock { condition: Box::new(condition), body })
            .collect();

        AstFactory::create_if_statement(
            IfStatement { blocks, else_block: else_body, end_location: location.clone() },
            location,
            ids.next_id(),
        )
    }

    /// Creates `<flag> := <value> <operator> <end>`. Codegen takes the debug location of an
    /// assignment from its left side, so the flag takes the header location and a debugger stops
    /// there for the bound check. The validator checks operand conversions only for a comparison
    /// that has a source location.
    pub fn create_bound_check(
        mut flag: AstNode,
        value: AstNode,
        operator: Operator,
        end: AstNode,
        (header_location, comparison_location): (SourceLocation, SourceLocation),
        ids: &mut IdProvider,
    ) -> AstNode {
        let mut comparison = AstFactory::create_binary_expression(value, operator, end, ids.next_id());
        comparison.location = comparison_location;
        flag.location = header_location;
        create_internal_assignment(flag, comparison, ids)
    }

    pub fn create_if_then_exit(mut ids: IdProvider, condition: AstNode) -> AstNode {
        let location = SourceLocation::internal();
        AstFactory::create_if_statement(
            IfStatement {
                blocks: vec![ConditionalBlock {
                    condition: Box::new(condition),
                    body: vec![AstFactory::create_exit_statement(SourceLocation::internal(), ids.next_id())],
                }],
                else_block: Vec::new(),
                end_location: location.clone(),
            },
            location,
            ids.next_id(),
        )
    }

    pub fn create_while_true_loop(ids: &mut IdProvider, body: Vec<AstNode>) -> AstNode {
        let statement = LoopStatement {
            condition: Box::new(create_internal_literal_true(ids)),
            body,
            end_location: SourceLocation::internal(),
        };

        AstFactory::create_while_statement(statement, SourceLocation::internal(), ids.next_id())
    }

    pub fn create_internal_binary_expression(
        left: AstNode,
        operator: Operator,
        right: AstNode,
        ids: &mut IdProvider,
    ) -> AstNode {
        let mut expr = AstFactory::create_binary_expression(left, operator, right, ids.next_id());
        expr.location = SourceLocation::internal();
        expr
    }

    pub fn create_internal_assignment(left: AstNode, right: AstNode, ids: &mut IdProvider) -> AstNode {
        let mut assignment = AstFactory::create_assignment(left, right, ids.next_id());
        assignment.location = SourceLocation::internal();
        assignment
    }

    pub fn create_literal_integer(ids: &mut IdProvider, value: i128) -> AstNode {
        AstFactory::create_literal(AstLiteral::Integer(value), SourceLocation::internal(), ids.next_id())
    }

    pub fn create_alloca(ids: &mut IdProvider, ty: &str, name: String) -> (AstNode, AstNode) {
        let alloca = AstNode {
            stmt: AstStatement::AllocationStatement(Allocation {
                name: name.to_string(),
                reference_type: String::from(ty),
                // Loop bookkeeping values (`__ran_once_*` etc.) cross iterations; the
                // slot must live for the whole function.
                statement_scoped: false,
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
        fn while_true_exit() {
            let source = r#"
                FUNCTION main
                    WHILE TRUE DO
                        EXIT;
                    END_WHILE
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @r"
            WHILE TRUE DO
                IF NOT TRUE THEN
                    EXIT;
                END_IF
                EXIT;
                
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

            insta::assert_snapshot!(super::serialize(source), @r"
            alloca __ran_once_0: BOOL
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF 1 > 2 THEN
                        EXIT;
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                a := b
                b := c
                c := a
            END_WHILE
            ");
        }

        #[test]
        fn condition_is_checked_on_second_iteration() {
            let source = r#"
                FUNCTION main
                    VAR
                        x : BOOL;
                    END_VAR

                    REPEAT
                        x;
                    UNTIL x
                    END_REPEAT
                END_FUNCTION
            "#;

            insta::assert_snapshot!(super::serialize(source), @r"
            alloca __ran_once_0: BOOL
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF x THEN
                        EXIT;
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                x
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

            insta::assert_snapshot!(super::serialize(source), @r"
            alloca __ran_once_0: BOOL
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF 1 > 2 AND 2 < 3 THEN
                        EXIT;
                    END_IF
                END_IF
                __ran_once_0 := TRUE
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

            insta::assert_snapshot!(super::serialize(source), @r"
            alloca __ran_once_0: BOOL
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF (1 > 2 AND 2 < 3) OR 3 = 4 THEN
                        EXIT;
                    END_IF
                END_IF
                __ran_once_0 := TRUE
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

            insta::assert_snapshot!(super::serialize(source), @r"
            alloca __ran_once_1: BOOL
            WHILE TRUE DO
                IF __ran_once_1 THEN
                    IF 1 > 2 THEN
                        EXIT;
                    END_IF
                END_IF
                __ran_once_1 := TRUE
                a := b
                alloca __ran_once_0: BOOL
                WHILE TRUE DO
                    IF __ran_once_0 THEN
                        IF 3 < 4 THEN
                            EXIT;
                        END_IF
                    END_IF
                    __ran_once_0 := TRUE
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := 0
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + 1 > 10
                    ELSE
                        __past_end_0 := i + 1 < 10
                    END_IF
                    i := i + 1
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > 10
                    ELSE
                        __past_end_0 := i < 10
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := a
            __is_incrementing_0 := step > 0
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + step > max
                    ELSE
                        __past_end_0 := i + step < max
                    END_IF
                    i := i + step
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > max
                    ELSE
                        __past_end_0 := i < max
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := 1
            __is_incrementing_0 := 0 > 0
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + 0 > 3
                    ELSE
                        __past_end_0 := i + 0 < 3
                    END_IF
                    i := i + 0
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > 3
                    ELSE
                        __past_end_0 := i < 3
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := 5
            __is_incrementing_0 := 2 > 0
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + 2 > 1
                    ELSE
                        __past_end_0 := i + 2 < 1
                    END_IF
                    i := i + 2
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > 1
                    ELSE
                        __past_end_0 := i < 1
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := 1
            __is_incrementing_0 := -1 > 0
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + -1 > 5
                    ELSE
                        __past_end_0 := i + -1 < 5
                    END_IF
                    i := i + -1
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > 5
                    ELSE
                        __past_end_0 := i < 5
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := 4
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + 1 > 4
                    ELSE
                        __past_end_0 := i + 1 < 4
                    END_IF
                    i := i + 1
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > 4
                    ELSE
                        __past_end_0 := i < 4
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := 0
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + 1 > max
                    ELSE
                        __past_end_0 := i + 1 < max
                    END_IF
                    i := i + 1
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > max
                    ELSE
                        __past_end_0 := i < max
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := 0
            __is_incrementing_0 := step > 0
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + step > 10
                    ELSE
                        __past_end_0 := i + step < 10
                    END_IF
                    i := i + step
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > 10
                    ELSE
                        __past_end_0 := i < 10
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := a
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + 1 > b
                    ELSE
                        __past_end_0 := i + 1 < b
                    END_IF
                    i := i + 1
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > b
                    ELSE
                        __past_end_0 := i < b
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __past_end_0: BOOL
            i := a
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                IF __ran_once_0 THEN
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i + 1 > b
                    ELSE
                        __past_end_0 := i + 1 < b
                    END_IF
                    i := i + 1
                ELSE
                    IF __is_incrementing_0 THEN
                        __past_end_0 := i > b
                    ELSE
                        __past_end_0 := i < b
                    END_IF
                END_IF
                __ran_once_0 := TRUE
                IF __past_end_0 THEN
                    EXIT;
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
            alloca __ran_once_1: BOOL
            alloca __is_incrementing_1: BOOL
            alloca __past_end_1: BOOL
            i := start
            __is_incrementing_1 := outer_step > 0
            WHILE TRUE DO
                IF __ran_once_1 THEN
                    IF __is_incrementing_1 THEN
                        __past_end_1 := i + outer_step > finish
                    ELSE
                        __past_end_1 := i + outer_step < finish
                    END_IF
                    i := i + outer_step
                ELSE
                    IF __is_incrementing_1 THEN
                        __past_end_1 := i > finish
                    ELSE
                        __past_end_1 := i < finish
                    END_IF
                END_IF
                __ran_once_1 := TRUE
                IF __past_end_1 THEN
                    EXIT;
                END_IF
                alloca __ran_once_0: BOOL
                alloca __is_incrementing_0: BOOL
                alloca __past_end_0: BOOL
                j := 10
                __is_incrementing_0 := inner_step > 0
                WHILE TRUE DO
                    IF __ran_once_0 THEN
                        IF __is_incrementing_0 THEN
                            __past_end_0 := j + inner_step > 0
                        ELSE
                            __past_end_0 := j + inner_step < 0
                        END_IF
                        j := j + inner_step
                    ELSE
                        IF __is_incrementing_0 THEN
                            __past_end_0 := j > 0
                        ELSE
                            __past_end_0 := j < 0
                        END_IF
                    END_IF
                    __ran_once_0 := TRUE
                    IF __past_end_0 THEN
                        EXIT;
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
            alloca __ran_once_1: BOOL
            alloca __is_incrementing_1: BOOL
            alloca __past_end_1: BOOL
            i := a
            __is_incrementing_1 := c > 0
            WHILE TRUE DO
                IF __ran_once_1 THEN
                    IF __is_incrementing_1 THEN
                        __past_end_1 := i + c > b
                    ELSE
                        __past_end_1 := i + c < b
                    END_IF
                    i := i + c
                ELSE
                    IF __is_incrementing_1 THEN
                        __past_end_1 := i > b
                    ELSE
                        __past_end_1 := i < b
                    END_IF
                END_IF
                __ran_once_1 := TRUE
                IF __past_end_1 THEN
                    EXIT;
                END_IF
                alloca __ran_once_0: BOOL
                alloca __is_incrementing_0: BOOL
                alloca __past_end_0: BOOL
                j := 0
                __is_incrementing_0 := TRUE
                WHILE TRUE DO
                    IF __ran_once_0 THEN
                        IF __is_incrementing_0 THEN
                            __past_end_0 := j + 1 > 2
                        ELSE
                            __past_end_0 := j + 1 < 2
                        END_IF
                        j := j + 1
                    ELSE
                        IF __is_incrementing_0 THEN
                            __past_end_0 := j > 2
                        ELSE
                            __past_end_0 := j < 2
                        END_IF
                    END_IF
                    __ran_once_0 := TRUE
                    IF __past_end_0 THEN
                        EXIT;
                    END_IF
                    a := j
                END_WHILE
                b := i
            END_WHILE
            ");
        }
    }
}
