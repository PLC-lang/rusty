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
//! - `__next_N` holds the next counter value
//!
//! ```st
//! alloca __ran_once_N : BOOL;
//! alloca __is_incrementing_N : BOOL;
//! alloca __next_N : LINT;
//!
//! <ctrl> := <init>;
//! __is_incrementing_N := <step> > 0;
//!
//! WHILE TRUE DO
//!     __next_N := <ctrl>;
//!     IF __ran_once_N THEN
//!         __next_N := __next_N + <step>;
//!         <ctrl> := __next_N;
//!     END_IF
//!     __ran_once_N := TRUE;
//!
//!     IF __is_incrementing_N THEN
//!         IF __next_N > <final> THEN
//!             EXIT;
//!         END_IF
//!     ELSE
//!         IF __next_N < <final> THEN
//!             EXIT;
//!         END_IF
//!     END_IF
//!
//!     <body>
//! END_WHILE
//! ```
//!
//! The step is evaluated once per iteration, and the exit test compares `__next_N`, not the counter that the
//! assignment truncates to its width. The counter's type is not known before annotation, so `__next_N` is
//! declared as `LINT`. After annotation and after property accesses are lowered to accessor calls,
//! [`NextValueTyper::type_next_values`] keeps `LINT` for a signed or unsigned integer or bit-string counter of
//! up to 32 bits, which makes the next value exact, and gives `__next_N` the counter's own intrinsic type
//! otherwise (64-bit integers, `CHAR`, date and time types, `REAL`, ...). Every copy of `<ctrl>`, `<step>`,
//! `<final>` and `__next_N` gets fresh node ids, so the resolver types each use on its own.
//!
//! If `BY` is omitted, the step defaults to `1`. The step and end expressions remain part of the lowered tree
//! and are therefore evaluated where needed by later passes, matching the compiler's current semantics.
//!
//! ## Generated temporaries
//!
//! Compiler-generated names such as `__ran_once_N`, `__is_incrementing_N` and `__next_N` are unique per
//! desugared loop.
//! They are ordinary AST allocations used only to encode loop semantics explicitly for later stages.

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use plc_ast::{
    ast::{Allocation, Assignment, AstFactory, AstId, AstNode, AstStatement, CompilationUnit, Operator},
    control_statements::{ForLoopStatement, LoopStatement},
    mut_visitor::{clone_with_new_ids, AstVisitorMut, WalkerMut},
    provider::IdProvider,
    visitor::AstVisitor,
};
use plc_source::source_location::SourceLocation;

/// The name prefix of the temporary that holds the next counter value of a `FOR` loop.
const NEXT_VALUE_PREFIX: &str = "__next_";

/// The type of the next-value temporary when the counter is an integer of up to 32 bits.
const NEXT_VALUE_WIDE_TYPE: &str = "LINT";

/// The ids of each `__next_N := <ctrl>` copy and of its `__next_N` allocation, shared by the desugarer that
/// records them and the typer that reads them.
type NextValues = Rc<RefCell<HashMap<AstId, AstId>>>;

pub struct LoopDesugarer {
    ids: IdProvider,
    next_values: NextValues,
}

/// Gives the `__next_N` temporaries of the desugared `FOR` loops their final types after annotation.
pub struct NextValueTyper {
    ids: IdProvider,
    next_values: NextValues,
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

    /// The ids of each `__next_N := <ctrl>` copy and of its `__next_N` allocation
    next_values: HashMap<AstId, AstId>,
}

/// Collects, by allocation id, the type of each next-value temporary that does not stay `LINT`.
struct NextValueTypeCollector<'a> {
    next_value_type: &'a dyn Fn(&AstNode) -> Option<String>,
    next_values: &'a HashMap<AstId, AstId>,
    types: HashMap<AstId, String>,
}

/// Sets the collected types on the next-value allocations.
struct NextValueTypeSetter {
    types: HashMap<AstId, String>,
}

impl LoopDesugarer {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids, next_values: NextValues::default() }
    }

    /// Returns the typer for the `__next_N` temporaries that this desugarer creates.
    pub fn next_value_typer(&self) -> NextValueTyper {
        NextValueTyper { ids: self.ids.clone(), next_values: self.next_values.clone() }
    }

    pub fn desugar(&self, units: &mut [CompilationUnit]) {
        let counter = Rc::new(AtomicUsize::new(0));

        let mut whiled = WhileDesugarer { ids: self.ids.clone() };
        let mut repeatd =
            RepeatDesugarer { ids: self.ids.clone(), counter: counter.clone(), replacement: None };
        let mut ford =
            ForDesugarer { ids: self.ids.clone(), counter, replacement: None, next_values: HashMap::new() };

        for unit in units {
            whiled.visit_compilation_unit(unit);
            repeatd.visit_compilation_unit(unit);
            ford.visit_compilation_unit(unit);
        }

        *self.next_values.borrow_mut() = ford.next_values;
    }
}

impl NextValueTyper {
    pub fn provider(&self) -> IdProvider {
        self.ids.clone()
    }

    /// Gives each `__next_N` temporary its final type. `next_value_type` maps the annotated counter in the
    /// copy `__next_N := <ctrl>` to the new type of `__next_N`, or to `None` if it stays `LINT` (see
    /// `plc::lowering::helper::for_loop_next_value_type`). A property counter must already be lowered to
    /// its accessor calls, because a property reference has no type annotation. Returns whether a
    /// temporary changed its type, in which case the units must be annotated again.
    pub fn type_next_values<'u>(
        &self,
        units: impl IntoIterator<Item = &'u mut CompilationUnit>,
        next_value_type: &dyn Fn(&AstNode) -> Option<String>,
    ) -> bool {
        let mut units = units.into_iter().collect::<Vec<_>>();

        let next_values = self.next_values.borrow();
        let mut collector =
            NextValueTypeCollector { next_value_type, next_values: &next_values, types: HashMap::new() };
        for unit in &units {
            collector.visit_compilation_unit(unit);
        }

        if collector.types.is_empty() {
            return false;
        }

        let mut setter = NextValueTypeSetter { types: collector.types };
        for unit in &mut units {
            setter.visit_compilation_unit(unit);
        }

        true
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

        // Create temporaries tracking whether the loop already ran, which comparison branch to use and the
        // next counter value.
        let num = self.counter.fetch_add(1, Ordering::SeqCst);
        let (ran_once_alloca, ran_once_ref) =
            helper::create_alloca(&mut self.ids, "BOOL", format!("__ran_once_{num}"));
        let (is_incrementing_alloca, is_incrementing_ref) =
            helper::create_alloca(&mut self.ids, "BOOL", format!("__is_incrementing_{num}"));
        let (next_alloca, next_ref) =
            helper::create_alloca(&mut self.ids, NEXT_VALUE_WIDE_TYPE, format!("{NEXT_VALUE_PREFIX}{num}"));

        // Normalize the step expression so omitted `BY` becomes a literal `1`.
        let has_explicit_step = by_step.is_some();
        let step = by_step.unwrap_or_else(|| helper::create_literal_integer(&mut self.ids, 1));
        let zero = helper::create_literal_integer(&mut self.ids, 0);

        // Copy the counter to `__next_N` at the top of every iteration. After the first one, add the step
        // to the copy and store it in the counter, which can truncate it. Every copy of the counter, step,
        // end and `__next_N` gets fresh ids, so the resolver types each use on its own.
        let next_copy = helper::create_internal_assignment(
            clone_with_new_ids(&next_ref, &mut self.ids),
            clone_with_new_ids(&counter, &mut self.ids),
            &mut self.ids,
        );
        self.next_values.insert(next_copy.get_id(), next_alloca.get_id());

        let next_sum = helper::create_internal_binary_expression(
            clone_with_new_ids(&next_ref, &mut self.ids),
            Operator::Plus,
            clone_with_new_ids(&step, &mut self.ids),
            &mut self.ids,
        );
        let increment_guard = helper::create_if_then(
            self.ids.clone(),
            ran_once_ref.clone(),
            vec![
                helper::create_internal_assignment(
                    clone_with_new_ids(&next_ref, &mut self.ids),
                    next_sum,
                    &mut self.ids,
                ),
                helper::create_internal_assignment(
                    clone_with_new_ids(&counter, &mut self.ids),
                    clone_with_new_ids(&next_ref, &mut self.ids),
                    &mut self.ids,
                ),
            ],
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
            helper::create_internal_binary_expression(step, Operator::Greater, zero, &mut self.ids)
        } else {
            helper::create_internal_literal_true(&mut self.ids)
        };
        let is_incrementing_assignment = helper::create_internal_assignment(
            is_incrementing_ref.clone(),
            is_incrementing_value,
            &mut self.ids,
        );

        // Exit once the next value has moved past the end bound for the chosen direction. The comparisons
        // point at the user-written `FOR <ctrl> := ... TO <end>` header, and `__next_N` in them at the
        // counter, so their conversion warnings point at the source.
        let bound_location = counter.get_location().span(&end.get_location());
        let mut compared_next = clone_with_new_ids(&next_ref, &mut self.ids);
        compared_next.location = counter.get_location();
        let incrementing_exit = helper::create_if_then_exit(
            self.ids.clone(),
            helper::create_bound_comparison(
                clone_with_new_ids(&compared_next, &mut self.ids),
                Operator::Greater,
                clone_with_new_ids(&end, &mut self.ids),
                bound_location.clone(),
                &mut self.ids,
            ),
        );
        let decrementing_exit = helper::create_if_then_exit(
            self.ids.clone(),
            helper::create_bound_comparison(
                compared_next,
                Operator::Less,
                end,
                bound_location,
                &mut self.ids,
            ),
        );
        let bound_guard = helper::create_if_then_else(
            self.ids.clone(),
            is_incrementing_ref,
            vec![incrementing_exit],
            vec![decrementing_exit],
            SourceLocation::internal(),
        );

        // Prepend the desugared loop control flow ahead of the original body.
        body.insert(0, next_copy);
        body.insert(1, increment_guard);
        body.insert(2, ran_once_assignment);
        body.insert(3, bound_guard);

        debug_assert!(self.replacement.is_none());
        // Replace the original for loop with its preamble and a `WHILE TRUE` loop.
        self.replacement = Some(vec![
            ran_once_alloca,
            is_incrementing_alloca,
            next_alloca,
            AstFactory::create_assignment(counter, start, self.ids.next_id()),
            is_incrementing_assignment,
            helper::create_while_true_loop(&mut self.ids, body),
        ]);
    }
}

impl AstVisitor for NextValueTypeCollector<'_> {
    fn visit_assignment(&mut self, stmt: &Assignment, node: &AstNode) {
        // Only the copy `__next_N := <ctrl>` that the desugarer generated tells the counter's type.
        let Some(allocation) = self.next_values.get(&node.get_id()) else { return };
        if let Some(next_type) = (self.next_value_type)(&stmt.right) {
            self.types.insert(*allocation, next_type);
        }
    }
}

impl AstVisitorMut for NextValueTypeSetter {
    fn visit_allocation(&mut self, node: &mut AstNode) {
        let Some(next_type) = self.types.get(&node.id) else { return };
        if let AstStatement::AllocationStatement(Allocation { reference_type, .. }) = &mut node.stmt {
            *reference_type = next_type.clone();
        }
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

    /// Creates `<left> <operator> <right>` with the given location, so the validator checks its operand
    /// conversions and a debugger stops on that location.
    pub fn create_bound_comparison(
        left: AstNode,
        operator: Operator,
        right: AstNode,
        location: SourceLocation,
        ids: &mut IdProvider,
    ) -> AstNode {
        let mut comparison = AstFactory::create_binary_expression(left, operator, right, ids.next_id());
        comparison.location = location;
        comparison
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
            alloca __next_0: LINT
            i := 0
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + 1
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > 10 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < 10 THEN
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __next_0: LINT
            i := a
            __is_incrementing_0 := step > 0
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + step
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > max THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < max THEN
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __next_0: LINT
            i := 1
            __is_incrementing_0 := 0 > 0
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + 0
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > 3 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < 3 THEN
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __next_0: LINT
            i := 5
            __is_incrementing_0 := 2 > 0
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + 2
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > 1 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < 1 THEN
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __next_0: LINT
            i := 1
            __is_incrementing_0 := -1 > 0
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + -1
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > 5 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < 5 THEN
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __next_0: LINT
            i := 4
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + 1
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > 4 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < 4 THEN
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __next_0: LINT
            i := 0
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + 1
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > max THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < max THEN
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __next_0: LINT
            i := 0
            __is_incrementing_0 := step > 0
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + step
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > 10 THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < 10 THEN
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __next_0: LINT
            i := a
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + 1
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > b THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < b THEN
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
            alloca __ran_once_0: BOOL
            alloca __is_incrementing_0: BOOL
            alloca __next_0: LINT
            i := a
            __is_incrementing_0 := TRUE
            WHILE TRUE DO
                __next_0 := i
                IF __ran_once_0 THEN
                    __next_0 := __next_0 + 1
                    i := __next_0
                END_IF
                __ran_once_0 := TRUE
                IF __is_incrementing_0 THEN
                    IF __next_0 > b THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_0 < b THEN
                        EXIT;
                    END_IF
                END_IF
                EXIT;
                
            END_WHILE
            ");
        }

        #[test]
        fn next_value_is_lint_for_integers_up_to_32_bits_and_the_counter_type_else() {
            let source = r#"
                TYPE Small : INT(0..10); END_TYPE
                TYPE Color : (Red, Green, Blue); END_TYPE
                TYPE Big : ULINT; END_TYPE

                FUNCTION main
                    VAR
                        small : Small;
                        color : Color;
                        b : BOOL;
                        big : Big;
                        c : CHAR;
                        t : TIME;
                    END_VAR

                    FOR small := 0 TO 1 DO END_FOR
                    FOR color := Red TO Blue DO END_FOR
                    FOR b := FALSE TO TRUE DO END_FOR
                    FOR big := 0 TO 1 DO END_FOR
                    FOR c := 'a' TO 'b' DO END_FOR
                    FOR t := T#0s TO T#1s BY T#1s DO END_FOR
                END_FUNCTION
            "#;

            let serialized = super::serialize(source);
            let allocations =
                serialized.lines().filter(|line| line.starts_with("alloca __next_")).collect::<Vec<_>>();
            insta::assert_snapshot!(allocations.join("\n"), @"
            alloca __next_0: LINT
            alloca __next_1: LINT
            alloca __next_2: LINT
            alloca __next_3: ULINT
            alloca __next_4: CHAR
            alloca __next_5: TIME
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
            alloca __next_1: LINT
            i := start
            __is_incrementing_1 := outer_step > 0
            WHILE TRUE DO
                __next_1 := i
                IF __ran_once_1 THEN
                    __next_1 := __next_1 + outer_step
                    i := __next_1
                END_IF
                __ran_once_1 := TRUE
                IF __is_incrementing_1 THEN
                    IF __next_1 > finish THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_1 < finish THEN
                        EXIT;
                    END_IF
                END_IF
                alloca __ran_once_0: BOOL
                alloca __is_incrementing_0: BOOL
                alloca __next_0: LINT
                j := 10
                __is_incrementing_0 := inner_step > 0
                WHILE TRUE DO
                    __next_0 := j
                    IF __ran_once_0 THEN
                        __next_0 := __next_0 + inner_step
                        j := __next_0
                    END_IF
                    __ran_once_0 := TRUE
                    IF __is_incrementing_0 THEN
                        IF __next_0 > 0 THEN
                            EXIT;
                        END_IF
                    ELSE
                        IF __next_0 < 0 THEN
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
            alloca __ran_once_1: BOOL
            alloca __is_incrementing_1: BOOL
            alloca __next_1: LINT
            i := a
            __is_incrementing_1 := c > 0
            WHILE TRUE DO
                __next_1 := i
                IF __ran_once_1 THEN
                    __next_1 := __next_1 + c
                    i := __next_1
                END_IF
                __ran_once_1 := TRUE
                IF __is_incrementing_1 THEN
                    IF __next_1 > b THEN
                        EXIT;
                    END_IF
                ELSE
                    IF __next_1 < b THEN
                        EXIT;
                    END_IF
                END_IF
                alloca __ran_once_0: BOOL
                alloca __is_incrementing_0: BOOL
                alloca __next_0: LINT
                j := 0
                __is_incrementing_0 := TRUE
                WHILE TRUE DO
                    __next_0 := j
                    IF __ran_once_0 THEN
                        __next_0 := __next_0 + 1
                        j := __next_0
                    END_IF
                    __ran_once_0 := TRUE
                    IF __is_incrementing_0 THEN
                        IF __next_0 > 2 THEN
                            EXIT;
                        END_IF
                    ELSE
                        IF __next_0 < 2 THEN
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
