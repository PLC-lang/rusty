use crate::{
    ast::{
        Allocation, Assignment, AstNode, AstStatement, BinaryExpression, CallStatement, CompilationUnit,
        ConfigVariable, DataType, DataTypeDeclaration, DefaultValue, DirectAccess, EmptyStatement,
        HardwareAccess, Implementation, Interface, JumpStatement, LabelStatement, MultipliedStatement, Pou,
        PropertyBlock, RangeStatement, ReferenceAccess, ReferenceExpr, UnaryExpression, UserTypeDeclaration,
        Variable, VariableBlock,
    },
    control_statements::{AstControlStatement, ReturnStatement},
    literals::AstLiteral,
    visitor::{AstVisitor, Walker},
};

pub struct AstSerializer {
    result: String,
    indent: usize,
}

impl AstSerializer {
    pub fn format(node: &AstNode) -> String {
        let mut serializer = AstSerializer { result: String::new(), indent: 0 };
        serializer.visit(node);

        serializer.result
    }

    /// Serializes a list of statements, each on its own indented line.
    fn serialize_statement_list(&mut self, stmts: &[AstNode]) {
        self.indent += 1;
        for stmt in stmts {
            if matches!(stmt.get_stmt(), AstStatement::EmptyStatement(..)) {
                continue;
            }
            self.push_indent();
            stmt.walk(self);
        }
        self.indent -= 1;
    }

    /// Pushes a newline followed by the current indentation.
    fn push_indent(&mut self) {
        self.result.push('\n');
        for _ in 0..self.indent {
            self.result.push_str("    ");
        }
    }
}

impl AstVisitor for AstSerializer {
    fn visit(&mut self, node: &AstNode) {
        node.walk(self)
    }

    fn visit_compilation_unit(&mut self, _: &CompilationUnit) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_implementation(&mut self, _: &Implementation) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_variable_block(&mut self, _: &VariableBlock) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_variable(&mut self, _: &Variable) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_config_variable(&mut self, _: &ConfigVariable) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_interface(&mut self, _: &Interface) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_property(&mut self, _: &PropertyBlock) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_enum_element(&mut self, element: &AstNode) {
        element.walk(self);
    }

    fn visit_data_type_declaration(&mut self, _: &DataTypeDeclaration) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_user_type_declaration(&mut self, _: &UserTypeDeclaration) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_data_type(&mut self, _: &DataType) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_pou(&mut self, _: &Pou) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_empty_statement(&mut self, _stmt: &EmptyStatement, _node: &AstNode) {}

    fn visit_default_value(&mut self, _stmt: &DefaultValue, _node: &AstNode) {}

    fn visit_literal(&mut self, stmt: &AstLiteral, _node: &AstNode) {
        use crate::literals::AstLiteral;
        match stmt {
            AstLiteral::Integer(value) => self.result.push_str(&value.to_string()),
            AstLiteral::Real(value) => self.result.push_str(value),
            AstLiteral::Bool(value) => self.result.push_str(&value.to_string().to_uppercase()),
            AstLiteral::String(string_value) => {
                if string_value.is_wide {
                    self.result.push_str(&format!("\"{}\"", string_value.value));
                } else {
                    self.result.push_str(&format!("'{}'", string_value.value));
                }
            }
            AstLiteral::Null => self.result.push_str("NULL"),
            _ => stmt.walk(self), // Let other literals use their default walking behavior
        }
    }

    fn visit_multiplied_statement(&mut self, stmt: &MultipliedStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    fn visit_reference_expr(&mut self, stmt: &ReferenceExpr, _node: &AstNode) {
        if let Some(base) = &stmt.base {
            base.walk(self);
        }

        match &stmt.access {
            ReferenceAccess::Global(reference) => {
                self.result.push('.');
                reference.walk(self);
            }
            ReferenceAccess::Member(reference) => {
                if stmt.base.is_some() {
                    self.result.push('.');
                }
                reference.walk(self);
            }
            ReferenceAccess::Index(index) => {
                self.result.push('[');
                index.walk(self);
                self.result.push(']');
            }
            ReferenceAccess::Cast(reference) => {
                self.result.push('#');
                reference.walk(self);
            }
            ReferenceAccess::Deref => {
                self.result.push('^');
            }
            ReferenceAccess::Address => {
                self.result.insert_str(0, "ADR(");
                self.result.push(')');
            }
        }
    }

    fn visit_identifier(&mut self, stmt: &str, _node: &AstNode) {
        self.result.push_str(stmt);
    }

    fn visit_direct_access(&mut self, stmt: &DirectAccess, _node: &AstNode) {
        stmt.walk(self)
    }

    fn visit_hardware_access(&mut self, stmt: &HardwareAccess, _node: &AstNode) {
        stmt.walk(self)
    }

    fn visit_binary_expression(&mut self, stmt: &BinaryExpression, _node: &AstNode) {
        stmt.left.walk(self);
        self.result.push(' ');
        self.result.push_str(&stmt.operator.to_string());
        self.result.push(' ');
        stmt.right.walk(self);
    }

    fn visit_unary_expression(&mut self, stmt: &UnaryExpression, _node: &AstNode) {
        let op = stmt.operator.to_string();
        self.result.push_str(&op);
        // Word-based operators (NOT, MINUS as identifier) need a trailing space.
        if op.chars().next().is_some_and(|c| c.is_alphabetic()) {
            self.result.push(' ');
        }
        stmt.value.walk(self);
    }

    fn visit_expression_list(&mut self, stmt: &Vec<AstNode>, _node: &AstNode) {
        for (i, node) in stmt.iter().enumerate() {
            if i > 0 {
                self.result.push_str(", ");
            }
            node.walk(self);
        }
    }

    fn visit_paren_expression(&mut self, inner: &AstNode, _node: &AstNode) {
        self.result.push('(');
        inner.walk(self);
        self.result.push(')');
    }

    fn visit_range_statement(&mut self, stmt: &RangeStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    fn visit_vla_range_statement(&mut self, _node: &AstNode) {}

    fn visit_assignment(&mut self, stmt: &Assignment, _node: &AstNode) {
        stmt.left.walk(self);
        self.result.push_str(" := ");
        stmt.right.walk(self);
    }

    fn visit_output_assignment(&mut self, stmt: &Assignment, _node: &AstNode) {
        stmt.left.walk(self);
        self.result.push_str(" => ");
        stmt.right.walk(self);
    }

    fn visit_ref_assignment(&mut self, stmt: &Assignment, _node: &AstNode) {
        stmt.left.walk(self);
        self.result.push_str(" REF= ");
        stmt.right.walk(self);
    }

    fn visit_call_statement(&mut self, stmt: &CallStatement, _node: &AstNode) {
        stmt.operator.walk(self);
        self.result.push('(');
        if let Some(opt) = stmt.parameters.as_ref() {
            opt.walk(self)
        }
        self.result.push(')');
    }

    fn visit_control_statement(&mut self, stmt: &AstControlStatement, _node: &AstNode) {
        match stmt {
            AstControlStatement::If(if_stmt) => {
                for (i, block) in if_stmt.blocks.iter().enumerate() {
                    if i == 0 {
                        self.result.push_str("IF ");
                    } else {
                        self.push_indent();
                        self.result.push_str("ELSIF ");
                    }
                    block.condition.walk(self);
                    self.result.push_str(" THEN");
                    self.serialize_statement_list(&block.body);
                }
                if !if_stmt.else_block.is_empty() {
                    self.push_indent();
                    self.result.push_str("ELSE");
                    self.serialize_statement_list(&if_stmt.else_block);
                }
                self.push_indent();
                self.result.push_str("END_IF");
            }
            AstControlStatement::ForLoop(for_stmt) => {
                self.result.push_str("FOR ");
                for_stmt.counter.walk(self);
                self.result.push_str(" := ");
                for_stmt.start.walk(self);
                self.result.push_str(" TO ");
                for_stmt.end.walk(self);
                if let Some(step) = &for_stmt.by_step {
                    self.result.push_str(" BY ");
                    step.walk(self);
                }
                self.result.push_str(" DO");
                self.serialize_statement_list(&for_stmt.body);
                self.push_indent();
                self.result.push_str("END_FOR");
            }
            AstControlStatement::WhileLoop(loop_stmt) => {
                self.result.push_str("WHILE ");
                loop_stmt.condition.walk(self);
                self.result.push_str(" DO");
                self.serialize_statement_list(&loop_stmt.body);
                self.push_indent();
                self.result.push_str("END_WHILE");
            }
            AstControlStatement::RepeatLoop(loop_stmt) => {
                self.result.push_str("REPEAT");
                self.serialize_statement_list(&loop_stmt.body);
                self.push_indent();
                self.result.push_str("UNTIL ");
                loop_stmt.condition.walk(self);
                self.push_indent();
                self.result.push_str("END_REPEAT");
            }
            AstControlStatement::Case(case_stmt) => {
                self.result.push_str("CASE ");
                case_stmt.selector.walk(self);
                self.result.push_str(" OF");
                self.indent += 1;
                for block in &case_stmt.case_blocks {
                    self.push_indent();
                    block.condition.walk(self);
                    self.result.push(':');
                    self.serialize_statement_list(&block.body);
                }
                if !case_stmt.else_block.is_empty() {
                    self.push_indent();
                    self.result.push_str("ELSE");
                    self.serialize_statement_list(&case_stmt.else_block);
                }
                self.indent -= 1;
                self.push_indent();
                self.result.push_str("END_CASE");
            }
        }
    }

    fn visit_case_condition(&mut self, child: &AstNode, _node: &AstNode) {
        child.walk(self)
    }

    fn visit_exit_statement(&mut self, _node: &AstNode) {
        self.result.push_str("EXIT;");
    }

    fn visit_continue_statement(&mut self, _node: &AstNode) {
        self.result.push_str("CONTINUE;");
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    fn visit_jump_statement(&mut self, stmt: &JumpStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    fn visit_label_statement(&mut self, _stmt: &LabelStatement, _node: &AstNode) {}

    fn visit_allocation(&mut self, stmt: &Allocation, _node: &AstNode) {
        self.result.push_str(&format!("alloca {}: {}", stmt.name, stmt.reference_type));
    }

    fn visit_super(&mut self, _stmt: &AstStatement, _node: &AstNode) {
        self.result.push_str("SUPER");
    }

    fn visit_this(&mut self, _stmt: &AstStatement, _node: &AstNode) {
        self.result.push_str("THIS");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::AstFactory;
    use crate::literals::{AstLiteral, StringValue};
    use plc_source::source_location::SourceLocation;

    #[test]
    fn expression_list() {
        let function_name = AstFactory::create_identifier("foo", SourceLocation::undefined(), 0);
        let expressions = vec![
            AstFactory::create_literal(AstLiteral::Integer(1), SourceLocation::undefined(), 1),
            AstFactory::create_literal(
                AstLiteral::String(StringValue { value: "two".to_string(), is_wide: false }),
                SourceLocation::undefined(),
                2,
            ),
            AstFactory::create_literal(AstLiteral::Integer(3), SourceLocation::undefined(), 3),
            AstFactory::create_literal(
                AstLiteral::String(StringValue { value: "four".to_string(), is_wide: false }),
                SourceLocation::undefined(),
                4,
            ),
        ];
        let expression_list = AstFactory::create_expression_list(expressions, SourceLocation::undefined(), 5);
        let call = AstFactory::create_call_statement(
            function_name,
            Some(expression_list),
            6,
            SourceLocation::undefined(),
        );

        let result = AstSerializer::format(&call);
        assert_eq!(result, "foo(1, 'two', 3, 'four')");
    }
}
