use crate::{
    ast::{
        Allocation, ArgumentProperty, Assignment, AstNode, AstStatement, AutoDerefType, BinaryExpression,
        CallStatement, CompilationUnit, ConfigVariable, DataType, DataTypeDeclaration, DefaultValue,
        DirectAccess, EmptyStatement, HardwareAccess, Implementation, Interface, JumpStatement,
        LabelStatement, MultipliedStatement, Pou, PropertyBlock, RangeStatement, ReferenceAccess,
        ReferenceExpr, UnaryExpression, UserTypeDeclaration, Variable, VariableBlock, VariableBlockType,
    },
    control_statements::{AstControlStatement, ReturnStatement},
    literals::AstLiteral,
    visitor::{AstVisitor, Walker},
};

pub struct AstSerializer<'a> {
    result: String,
    indent: usize,
    unit: Option<&'a CompilationUnit>,
    user_type_context: Option<&'a UserTypeDeclaration>,
    is_in_paren: bool,
}

impl AstSerializer<'_> {
    pub fn format(node: &AstNode) -> String {
        let mut serializer = AstSerializer {
            result: String::new(),
            indent: 0,
            unit: None,
            user_type_context: None,
            is_in_paren: false,
        };
        serializer.visit(node);

        serializer.result
    }

    pub fn format_nodes(nodes: &[AstNode]) -> String {
        let mut serializer = AstSerializer {
            result: String::new(),
            indent: 0,
            unit: None,
            user_type_context: None,
            is_in_paren: false,
        };

        let nodes = nodes.iter().filter(|node| !node.is_empty_statement());

        for (index, node) in nodes.enumerate() {
            if index > 0 {
                serializer.result.push('\n');
            }
            serializer.visit(node);

            // Expression lists push their own ';'
            if !node.is_expression_list() {
                serializer.result.push(';');
            }
        }

        serializer.result
    }

    pub fn format_variable_block(variable_block: &VariableBlock, unit: &CompilationUnit) -> String {
        let mut serializer = AstSerializer {
            result: String::new(),
            indent: 0,
            unit: Some(unit),
            user_type_context: None,
            is_in_paren: false,
        };
        serializer.visit_variable_block(variable_block);

        serializer.result
    }

    /// Serializes a list of statements, each on its own indented line.
    fn serialize_statement_list(&mut self, stmts: &[AstNode]) {
        self.indent += 1;
        for stmt in stmts {
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

    fn set_user_type_declaration_context(&mut self, type_name: &str) {
        let Some(unit) = self.unit else {
            panic!("cannot retrieve user type declaration without a compilation unit")
        };

        self.user_type_context =
            unit.user_types.iter().find(|it| it.data_type.get_name().is_some_and(|name| name == type_name));
    }
}

impl AstVisitor for AstSerializer<'_> {
    fn visit(&mut self, node: &AstNode) {
        node.walk(self)
    }

    fn visit_compilation_unit(&mut self, _: &CompilationUnit) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_implementation(&mut self, _: &Implementation) {
        unimplemented!("for now only interested in individual nodes located in a POU body")
    }

    fn visit_variable_block(&mut self, variable_block: &VariableBlock) {
        let var_start = match variable_block.kind {
            VariableBlockType::InOut => "VAR_IN_OUT",
            VariableBlockType::Input(ArgumentProperty::ByVal) => "VAR_INPUT",
            VariableBlockType::Input(ArgumentProperty::ByRef) => "VAR_INPUT {ref}",
            VariableBlockType::Output => "VAR_OUTPUT",
            VariableBlockType::Temp => "VAR_TEMP",
            VariableBlockType::Global => "VAR_GLOBAL",
            _ => "VAR",
        };
        let var_end: &str = "END_VAR";

        self.result.push_str(var_start);
        self.indent += 1;
        variable_block.variables.iter().for_each(|v| {
            self.push_indent();
            self.result.push_str(&format!("{} : ", v.name));
            v.walk(self);
            self.result.push(';');
        });
        self.indent -= 1;
        self.result.push_str(&format!("\n{var_end}"));
    }

    fn visit_variable(&mut self, variable: &Variable) {
        variable.data_type_declaration.walk(self);
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

    fn visit_data_type_declaration(&mut self, data_type_declaration: &DataTypeDeclaration) {
        match data_type_declaration {
            DataTypeDeclaration::Reference { referenced_type, .. } => {
                self.set_user_type_declaration_context(referenced_type);

                if let Some(user_type_declaration) = self.user_type_context {
                    self.visit_user_type_declaration(user_type_declaration);
                } else {
                    self.result.push_str(referenced_type);
                }
            }
            DataTypeDeclaration::Definition { data_type, .. } => {
                data_type.as_ref().walk(self);
            }
            DataTypeDeclaration::Aggregate { referenced_type, .. } => {
                self.result.push_str(referenced_type);
            }
        }
    }

    fn visit_user_type_declaration(&mut self, user_type_declaration: &UserTypeDeclaration) {
        self.visit_data_type(&user_type_declaration.data_type);
    }

    fn visit_data_type(&mut self, data_type: &DataType) {
        match data_type {
            DataType::PointerType { referenced_type, auto_deref, type_safe, .. } => {
                match auto_deref {
                    Some(AutoDerefType::Reference) => {
                        self.result.push_str("REFERENCE TO ");
                    }
                    // TODO: We also want to handle these cases at some point
                    Some(AutoDerefType::Alias) | Some(AutoDerefType::Default) => (),
                    _ => {
                        if *type_safe {
                            self.result.push_str("REF_TO ");
                        } else {
                            self.result.push_str("POINTER TO ");
                        }
                    }
                }

                self.visit_data_type_declaration(referenced_type.as_ref());
            }
            DataType::StructType { name: Some(name), .. } => {
                self.result.push_str(name);
            }
            // TODO: For now we aren't interested in non-pointer types, but this should be expanded
            _ => (),
        }
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
                self.is_in_paren = true;
                index.walk(self);
                self.is_in_paren = false;
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
        let len = stmt.iter().filter(|stmt| !stmt.is_empty_statement()).count();
        let stmt = stmt.iter().filter(|stmt| !stmt.is_empty_statement());
        if self.is_in_paren {
            for (i, node) in stmt.enumerate() {
                if i > 0 {
                    self.result.push_str(", ");
                }
                node.walk(self);
            }
        } else {
            for (i, node) in stmt.enumerate() {
                node.walk(self);
                self.result.push(';');
                if i != len - 1 {
                    self.push_indent();
                }
            }
        }
    }

    fn visit_paren_expression(&mut self, inner: &AstNode, _node: &AstNode) {
        self.result.push('(');
        self.is_in_paren = true;
        inner.walk(self);
        self.is_in_paren = false;
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
        self.is_in_paren = true;
        if let Some(opt) = stmt.parameters.as_ref() {
            opt.walk(self)
        }
        self.is_in_paren = false;
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
        self.result.push_str("RETURN");
        stmt.walk(self);
        self.result.push(';');
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
