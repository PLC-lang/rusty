//! # Control Statement Lowering
//!
//! This module handles the lowering of control statements in an effort to ensure that the control statements are
//! processed without any side-effects that the user might not expect.
//!
//! ## Current use-cases for lowering control statements
//! ### 1. else/if statements
//! Due to the existing way that the AST was lowered, if a function call returned an aggregate type
//! and that function call was part of the condition of an else/if, then that function would always be evaluated.
//! This would result in some cases where an operation in the function could process when the user never intended
//! it to.
//!
//! For example:
//! ```st
//! FUNCTION foo : STRING[10]
//!     VAR
//!         result : STRING[10];
//!     END_VAR
//!     VAR_IN_OUT
//!         counter : INT;
//!     END_VAR
//!     result := 'Hello';
//!     counter := counter + 1;
//!     foo := result;
//! END_FUNCTION
//!
//! FUNCTION main
//!     VAR
//!         counterVar: INT;
//!     END_VAR
//!
//!     counterVar := 0;
//!
//!     IF foo(counterVar) = 'Hello' THEN
//!         // Do nothing
//!     ELSIF foo(counterVar) = 'Goodbye' THEN
//!         // Do nothing
//!     END_IF
//! END_FUNCTION
//! ```
//!
//! In the above example, the `foo()` method would be called a second time and increment the `counter` variable to 2.
//! This is obviously incorrect as the statement evaluation should have ended after the IF returned true.
//!
//! Thus, the statement is now lowered to prevent this evaluation from occuring, the resulting lowered code
//! would look like this:
//! ```st
//!     ...
//!     IF foo(counterVar) = 'Hello' THEN
//!         // Do nothing
//!     ELSE
//!         IF foo(counterVar) = 'Goodbye' THEN
//!             // Do nothing
//!         END_IF
//!     END_IF
//!     ...
//! ```
use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit},
    control_statements::{AstControlStatement, ConditionalBlock, IfStatement},
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from_mut,
};
use plc_source::source_location::SourceLocation;

pub struct ControlStatementParticipant {
    pub ids: IdProvider,
}

impl ControlStatementParticipant {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn lower_control_statements(&mut self, units: &mut [CompilationUnit]) {
        let mut lowerer = ControlStatementLowerer { ids: self.ids.clone() };
        for unit in units {
            lowerer.visit_compilation_unit(unit);
        }
    }
}

pub struct ControlStatementLowerer {
    ids: IdProvider,
}

impl AstVisitorMut for ControlStatementLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self);
    }

    fn visit_control_statement(&mut self, node: &mut plc_ast::ast::AstNode) {
        let ctrl_stmt = try_from_mut!(node, AstControlStatement).expect("ControlStatement");
        if let AstControlStatement::If(stmt) = ctrl_stmt {
            if stmt.blocks.len() > 1 {
                let conditional_blocks = stmt.blocks.drain(0..1).collect();
                let else_block = self.extract_condition_and_append_new_else(
                    &mut stmt.blocks,
                    stmt.else_block.clone(),
                    stmt.end_location.clone(),
                );

                stmt.blocks = conditional_blocks;
                stmt.else_block = else_block;
            }

            self.walk_conditional_blocks(&mut stmt.blocks);
            self.steal_and_walk_list(&mut stmt.else_block);
        } else {
            ctrl_stmt.walk(self);
        }
    }
}

impl ControlStatementLowerer {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    fn steal_and_walk_list(&mut self, list: &mut Vec<AstNode>) {
        //Enter new scope
        let mut new_stmts = vec![];
        for stmt in list.drain(..) {
            new_stmts.push(self.map(stmt));
        }
        std::mem::swap(list, &mut new_stmts);
    }

    fn walk_conditional_blocks(&mut self, blocks: &mut Vec<ConditionalBlock>) {
        for b in blocks {
            b.condition.walk(self);
            self.steal_and_walk_list(&mut b.body);
        }
    }

    fn extract_condition_and_append_new_else(
        &mut self,
        conditional_blocks: &mut Vec<ConditionalBlock>,
        else_block: Vec<AstNode>,
        location: SourceLocation,
    ) -> Vec<AstNode> {
        let new_conditional_blocks: Vec<ConditionalBlock>;
        let new_else_block: Vec<AstNode>;

        if conditional_blocks.len() > 1 {
            new_conditional_blocks = conditional_blocks.drain(0..1).collect();
            new_else_block =
                self.extract_condition_and_append_new_else(conditional_blocks, else_block, location.clone());
        } else {
            new_conditional_blocks = std::mem::take(conditional_blocks);
            new_else_block = else_block;
        }

        let stmt = IfStatement {
            blocks: new_conditional_blocks,
            else_block: new_else_block,
            end_location: location.clone(),
        };

        let node = AstFactory::create_if_statement(stmt, location.clone(), self.ids.next_id());
        vec![node]
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

    fn serialize(source: impl Into<SourceCode>) -> String {
        let (_, project) = parse_and_annotate("unit-test", vec![source.into()]).unwrap();
        let unit = project.units[0].get_unit();

        unit.implementations
            .iter()
            .find(|implementation| implementation.name == "mainProg")
            .expect("mainProg implementation should exist")
            .statements
            .iter()
            .map(|statement| statement.as_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn elseif_is_lowered_to_else_with_nested_if() {
        let source = r#"
            PROGRAM mainProg
            VAR
                val : INT;
                cVar : CHAR;
            END_VAR

            val := 5;
            cVar := '';

            IF val = 3 THEN
                // Fizz
                cVar := 'f';
            ELSIF val = 5 THEN
                // Buzz
                cVar := 'b';
            ELSE
                cVar := 'x';
            END_IF
            END_PROGRAM
            "#;

        assert_snapshot!(serialize(source), @"
        val := 5
        cVar := ''
        IF val = 3 THEN
            cVar := 'f'
        ELSE
            IF val = 5 THEN
                cVar := 'b'
            ELSE
                cVar := 'x'
            END_IF
        END_IF
        ");
    }

    #[test]
    fn elseif_is_lowered_to_else_with_nested_if_even_if_no_else_is_present() {
        let source = r#"
            PROGRAM mainProg
            VAR
                val : INT;
                cVar : CHAR;
            END_VAR

            val := 5;
            cVar := '';

            IF val = 3 THEN
                // Fizz
                cVar := 'f';
            ELSIF val = 5 THEN
                // Buzz
                cVar := 'b';
            END_IF
            END_PROGRAM
            "#;

        assert_snapshot!(serialize(source), @"
        val := 5
        cVar := ''
        IF val = 3 THEN
            cVar := 'f'
        ELSE
            IF val = 5 THEN
                cVar := 'b'
            END_IF
        END_IF
        ");
    }

    #[test]
    fn elseif_is_lowered_to_else_with_nested_if_when_prenested_in_if() {
        let source = r#"
            PROGRAM mainProg
            VAR
                val : INT;
                cVar : CHAR;
            END_VAR

            val := 5;
            cVar := '';

            IF val = 4 THEN
                cVar := 'a';
            ELSE
                IF val = 3 THEN
                    cVar := 'f';
                ELSIF val = 5 THEN
                    cVar := 'b';
                ELSE
                    cVar := 'x';
                END_IF
            END_IF
            END_PROGRAM
            "#;

        assert_snapshot!(serialize(source), @"
        val := 5
        cVar := ''
        IF val = 4 THEN
            cVar := 'a'
        ELSE
            IF val = 3 THEN
                cVar := 'f'
            ELSE
                IF val = 5 THEN
                    cVar := 'b'
                ELSE
                    cVar := 'x'
                END_IF
            END_IF
        END_IF
        ");
    }

    #[test]
    fn elseif_is_lowered_to_else_with_nested_if_inside_for_loop() {
        let source = r#"
            PROGRAM mainProg
            VAR
                i : INT;
                val : INT;
                cVar : CHAR;
            END_VAR

            val := 5;
            cVar := '';

            FOR i := 0 TO 10 DO
                IF val = 3 THEN
                    cVar := 'f';
                ELSIF val = 5 THEN
                    cVar := 'b';
                ELSE
                    cVar := 'x';
                END_IF
            END_FOR
            END_PROGRAM
            "#;

        assert_snapshot!(serialize(source), @"
        val := 5
        cVar := ''
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
            IF val = 3 THEN
                cVar := 'f'
            ELSE
                IF val = 5 THEN
                    cVar := 'b'
                ELSE
                    cVar := 'x'
                END_IF
            END_IF
        END_WHILE
        ");
    }

    #[test]
    fn elseif_is_lowered_to_else_with_nested_if_inside_while_loop() {
        let source = r#"
            PROGRAM mainProg
            VAR
                i : INT;
                breakOut: INT;
                val : INT;
                cVar : CHAR;
                someCon : BOOL;
            END_VAR

            val := 5;
            cVar := '';
            someCon := TRUE;
            breakOut := 0;

            WHILE someCon DO
                IF val = 3 THEN
                    cVar := 'f';
                    someCon := FALSE;
                ELSIF val = 5 THEN
                    cVar := 'b';
                    someCon := FALSE;
                ELSE
                    cVar := 'x';
                    IF breakOut = 10 THEN
                        someCon := FALSE;
                    END_IF
                    breakOut := breakOut + 1;
                END_IF
            END_WHILE
            END_PROGRAM
            "#;

        assert_snapshot!(serialize(source), @"
        val := 5
        cVar := ''
        someCon := TRUE
        breakOut := 0
        WHILE TRUE DO
            IF NOT someCon THEN
                EXIT;
            END_IF
            IF val = 3 THEN
                cVar := 'f'
                someCon := FALSE
            ELSE
                IF val = 5 THEN
                    cVar := 'b'
                    someCon := FALSE
                ELSE
                    cVar := 'x'
                    IF breakOut = 10 THEN
                        someCon := FALSE
                    END_IF
                    breakOut := breakOut + 1
                END_IF
            END_IF
        END_WHILE
        ");
    }
}
