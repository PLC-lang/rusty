use ast::ast::{AstFactory, AstNode};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::model::{
    control::{Control, ControlKind},
    fbd::{Node, NodeIndex},
};

use super::ParseSession;

impl<'xml> Control<'xml> {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> Result<AstNode, Diagnostic> {
        match self.kind {
            ControlKind::Jump => transform_jump(self, session, index),
            ControlKind::Label => transform_label(self, session),
            ControlKind::Return => transform_return(self, session, index),
        }
    }
}

fn get_connection(
    control: &Control,
    session: &ParseSession,
    index: &NodeIndex,
) -> Result<AstNode, Diagnostic> {
    let Some(ref_local_id) = control.ref_local_id else {
        let location =
            session.range_factory.create_block_location(control.local_id, control.execution_order_id);
        return Err(Diagnostic::new("Control statement has no connection")
            .with_error_code("E081")
            .with_location(location));
    };

    let Some(node) = index.get(&ref_local_id) else {
        let location = session.range_factory.create_block_location(ref_local_id, None);
        return Err(Diagnostic::new(format!(
            "Node {} is referencing a non-existing element with ID {ref_local_id}",
            control.local_id
        ))
        .with_error_code("E082")
        .with_location(location));
    };

    match node {
        Node::FunctionBlockVariable(variable) => Ok(variable.transform(session)),
        Node::Block(block) => Ok(block.transform(session, index)),

        _ => {
            let location_control =
                session.range_factory.create_block_location(control.local_id, control.execution_order_id);
            let location_other =
                session.range_factory.create_block_location(ref_local_id, node.get_exec_id());

            Err(Diagnostic::new("Unexpected relationship between nodes")
                .with_error_code("E083")
                .with_location(location_control.span(&location_other)))
        }
    }
}

fn transform_jump(
    control: &Control,
    session: &ParseSession,
    index: &NodeIndex,
) -> Result<AstNode, Diagnostic> {
    let location = session.range_factory.create_block_location(control.local_id, control.execution_order_id);
    let condition = get_connection(control, session, index)?;
    let condition = if control.negated { condition.negate(session.id_provider.clone()) } else { condition };

    let target = control
        .name
        .as_ref()
        .filter(|it| !it.is_empty())
        .map(|it| session.parse_expression(it.as_ref(), control.local_id, control.execution_order_id))
        .ok_or_else(|| Diagnostic::unnamed_control(location.clone()))?;

    Ok(AstFactory::create_jump_statement(Box::new(condition), Box::new(target), location, session.next_id()))
}

fn transform_label(control: &Control, session: &ParseSession) -> Result<AstNode, Diagnostic> {
    let location = session.range_factory.create_block_location(control.local_id, control.execution_order_id);

    control
        .name
        .as_ref()
        .filter(|it| !it.is_empty())
        .map(|it| AstFactory::create_label_statement(it.to_string(), location.clone(), session.next_id()))
        .ok_or_else(|| Diagnostic::unnamed_control(location))
}

fn transform_return(
    control: &Control,
    session: &ParseSession,
    index: &NodeIndex,
) -> Result<AstNode, Diagnostic> {
    let condition = get_connection(control, session, index)?;
    let condition = if control.negated { condition.negate(session.id_provider.clone()) } else { condition };

    Ok(AstFactory::create_return_statement(
        Some(condition),
        session.create_block_location(control.local_id, control.execution_order_id),
        session.next_id(),
    ))
}

#[cfg(test)]
mod tests {
    use ast::provider::IdProvider;
    use insta::assert_debug_snapshot;

    use crate::serializer::{SInVariable, SJump, SLabel, SOutVariable, SPou, SReturn};
    use crate::{
        model::control::Control,
        reader::{get_start_tag, Reader},
        xml_parser::{self, Parseable},
    };

    #[test]
    fn simple_return() {
        let content = SReturn::id(1).with_execution_id(2).connect(3).negate(false).serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Control::visit(&mut reader, tag).unwrap());
    }

    #[test]
    fn simple_negated_return() {
        let content = SReturn::id(1).with_execution_id(2).connect(3).negate(true).serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Control::visit(&mut reader, tag).unwrap());
    }

    #[test]
    fn jump_to_label() {
        let declaration = "PROGRAM program_0 VAR x : BOOL := 0; END_VAR";
        let content = SPou::init("program_0", "program", declaration).with_fbd(vec![
            &SInVariable::id(0).with_expression("x"),
            &SLabel::id(1).with_name("lbl").with_execution_id(0),
            &SJump::id(2).with_name("lbl").with_execution_id(1).connect(0),
            &SOutVariable::id(3).with_execution_id(2).with_expression("x").connect(4),
            &SInVariable::id(4).with_expression("FALSE"),
        ]);

        assert_debug_snapshot!(xml_parser::visit(&content.serialize()));
    }

    #[test]
    fn unconnected_jump_to_label() {
        let content = SJump::id(1).with_execution_id(2).serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Control::visit(&mut reader, tag).unwrap());
    }

    #[test]
    fn negated_jump() {
        let declaration = "PROGRAM program_0 VAR x : BOOL := 0; END_VAR";
        let content = SPou::init("program_0", "program", declaration).with_fbd(vec![
            &SInVariable::id(0).with_expression("x"),
            &SLabel::id(1).with_name("lbl").with_execution_id(0),
            &SJump::id(2).with_name("lbl").with_execution_id(1).connect(0).negate(),
            &SOutVariable::id(3).with_execution_id(2).with_expression("x").connect(4),
            &SInVariable::id(4).with_expression("FALSE"),
        ]);

        assert_debug_snapshot!(xml_parser::visit(&content.serialize()));
    }

    #[test]
    fn negated_jump_ast() {
        let declaration = "PROGRAM program_0 VAR x : BOOL := 0; END_VAR";
        let content = SPou::init("program_0", "program", declaration)
            .with_fbd(vec![
                &SInVariable::id(0).with_expression("x"),
                &SLabel::id(1).with_name("lbl").with_execution_id(0),
                &SJump::id(2).with_name("lbl").with_execution_id(1).connect(0).negate(),
                &SOutVariable::id(3).with_execution_id(2).with_expression("x").connect(4),
                &SInVariable::id(4).with_expression("FALSE"),
            ])
            .serialize();

        let (ast, diagnostics) =
            xml_parser::parse(&content.into(), ast::ast::LinkageType::Internal, IdProvider::default());
        assert_debug_snapshot!(ast);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn label_parsed() {
        let content = SLabel::id(1).with_execution_id(2).serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Control::visit(&mut reader, tag).unwrap());
    }

    #[test]
    fn jump_and_label_converted_to_ast() {
        let declaration = "PROGRAM program_0 VAR x : BOOL := 0; END_VAR";
        let content = SPou::init("program_0", "program", declaration)
            .with_fbd(vec![
                &SInVariable::id(0).with_expression("x"),
                &SLabel::id(1).with_name("lbl").with_execution_id(0),
                &SJump::id(2).with_name("lbl").with_execution_id(1).connect(0),
                &SOutVariable::id(3).with_execution_id(2).with_expression("x").connect(4),
                &SInVariable::id(4).with_expression("FALSE"),
            ])
            .serialize();

        let (ast, diagnostics) =
            xml_parser::parse(&content.into(), ast::ast::LinkageType::Internal, IdProvider::default());
        assert_debug_snapshot!(ast);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn unconnected_jump_generated_as_empty_statement() {
        let declaration = "PROGRAM program_0 VAR x : BOOL := 0; END_VAR";
        let content = SPou::init("program_0", "program", declaration)
            .with_fbd(vec![
                &SInVariable::id(0).with_expression("x"),
                &SLabel::id(1).with_name("lbl").with_execution_id(0),
                &SJump::id(2).with_name("lbl").with_execution_id(1),
                &SOutVariable::id(3).with_execution_id(2).with_expression("x").connect(4),
                &SInVariable::id(4).with_expression("FALSE"),
            ])
            .serialize();

        let (ast, diagnostics) =
            xml_parser::parse(&content.into(), ast::ast::LinkageType::Internal, IdProvider::default());
        assert_debug_snapshot!(ast);
        assert_debug_snapshot!(diagnostics);
    }

    #[test]
    fn unnamed_controls() {
        let content = SPou::init("program_0", "program", "PROGRAM program_0")
            .with_fbd(vec![
                &SInVariable::id(0).with_expression("x"),
                &SLabel::id(1).with_execution_id(0),
                &SJump::id(2).with_execution_id(1).connect(0),
                &SOutVariable::id(3).with_execution_id(2).with_expression("x").connect(4),
                &SInVariable::id(4).with_expression("FALSE"),
            ])
            .serialize();

        let (ast, diagnostics) =
            xml_parser::parse(&content.into(), ast::ast::LinkageType::Internal, IdProvider::default());
        assert_debug_snapshot!(ast);
        assert_debug_snapshot!(diagnostics);
    }
}
