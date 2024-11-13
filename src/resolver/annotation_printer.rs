use std::io::Write;

use plc_ast::{ast::AstStatement, visitor::{AstVisitor, Walker}};

use super::{AnnotationMap, AnnotationMapImpl, StatementAnnotation};

pub struct AnnotationPrinter<'i> {
    src: &'i str,
    annotations: &'i AnnotationMapImpl,
    lvl: usize,
    text: Vec<u8>,
}
const EMPTY: &str = "-";

impl AnnotationPrinter<'_> {
    pub fn print<W>(src: &str, annotations: &AnnotationMapImpl, w: &W) -> String
    where
        W: Walker,
    {
        let mut printer = AnnotationPrinter { src, annotations, lvl: 0, text: Vec::new() };
        w.walk(&mut printer);

        String::from_utf8(printer.text).unwrap()
    }

    fn get_type_name(&self, a: Option<&StatementAnnotation>) -> String {
        if let Some(a) = a {
            match a {
                StatementAnnotation::Function { qualified_name, .. } => {
                    format!("{qualified_name} (Function)")
                }
                StatementAnnotation::Program { qualified_name } => format!("{qualified_name} (Program)"),
                StatementAnnotation::Variable { resulting_type, qualified_name, .. } => {
                    format!("{resulting_type} (Variable {qualified_name})")
                }
                _ => self.annotations.get_type_name_for_annotation(a).unwrap_or(EMPTY).to_string(),
            }
        } else {
            EMPTY.to_string()
        }
    }
}

fn should_skip(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::Identifier(_) => true,
        _ => false,
    }
}

impl AstVisitor for AnnotationPrinter<'_> {
    fn visit(&mut self, node: &plc_ast::ast::AstNode) {
        if should_skip(node.get_stmt()) {
            return;
        }

        let type_name = self.get_type_name(self.annotations.get(&node)).to_string();
        let hint_name = self
            .annotations
            .get_hint(&node)
            .map(|it| format!(", hint: {}", self.get_type_name(Some(it))))
            .unwrap_or_default();

        let stmt = &self.src[node.get_location().to_range().unwrap()];
        let indent = self.lvl * 4;

        writeln!(&mut self.text, "{:indent$}{} [<{}>{}]{}", "", stmt, type_name, hint_name, "")
            .unwrap();

        self.lvl += 1;
        node.walk(self);
        self.lvl -= 1;
    }
}