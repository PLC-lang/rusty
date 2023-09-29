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
        return Err(Diagnostic::empty_control_statement(location));
    };

    let Some(node) = index.get(&ref_local_id) else {
        let location = session.range_factory.create_block_location(ref_local_id, None);
        return Err(Diagnostic::undefined_node(control.local_id, ref_local_id, location));
    };

    match node {
        Node::FunctionBlockVariable(variable) => Ok(variable.transform(session)),
        Node::Block(block) => Ok(block.transform(session, index)),

        _ => {
            let location_control =
                session.range_factory.create_block_location(control.local_id, control.execution_order_id);
            let location_other =
                session.range_factory.create_block_location(ref_local_id, node.get_exec_id());

            Err(Diagnostic::unexpected_nodes(location_control.span(&location_other)))
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

    use crate::{
        model::control::Control,
        reader::{get_start_tag, Reader},
        serializer::{XAddData, XConnectionPointIn, XJump, XLabel, XReturn},
        xml_parser::{self, Parseable},
    };

    #[test]
    fn simple_return() {
        let content = XReturn::new()
            .with_local_id("1")
            .with_execution_order_id("2")
            .with_connection_point_in(XConnectionPointIn::with_ref("3"))
            .with_add_data(XAddData::negated(false))
            .serialize();

        let reader = &mut Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Control::visit(reader, tag).unwrap());
    }

    #[test]
    fn simple_negated_return() {
        let content = XReturn::new()
            .with_local_id("1")
            .with_execution_order_id("2")
            .with_connection_point_in(XConnectionPointIn::with_ref("3"))
            .with_add_data(XAddData::negated(true))
            .serialize();

        let reader = &mut Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Control::visit(reader, tag).unwrap());
    }

    #[test]
    fn jump_to_label() {
        let content = r###"
            <?xml version="1.0" encoding="UTF-8"?>
            <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
                <interface>
                    <localVars/>
                    <addData>
                        <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                            <textDeclaration>
                                <content>
                                    PROGRAM program_0
                                    VAR&#xd;
                                        x: BOOL := 0;&#xd;
                                    END_VAR
                                </content>
                            </textDeclaration>
                        </data>
                    </addData>
                </interface>
                <body>
                    <FBD>
                        <inVariable localId="0" height="20" width="80" negated="false">
                            <position x="160" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>x</expression>
                        </inVariable>
                        <label localId="1" height="20" width="80" label="lbl" executionOrderId="0">
                            <position x="570" y="50"/>
                        </label>
                        <jump localId="2" height="20" width="80" label="lbl" executionOrderId="1">
                            <position x="320" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="0"/>
                            </connectionPointIn>
                            <addData>
                                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                                    <negated value="false"/>
                                </data>
                            </addData>
                        </jump>
                        <outVariable localId="3" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                            <position x="790" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="4"/>
                            </connectionPointIn>
                            <expression>x</expression>
                        </outVariable>
                        <inVariable localId="4" height="20" width="80" negated="false">
                            <position x="640" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>FALSE</expression>
                        </inVariable>
                    </FBD>
                </body>
            </pou>
        "###;

        assert_debug_snapshot!(xml_parser::visit(content));
    }

    #[test]
    fn unconnected_jump_to_label() {
        let content = XJump::new().with_local_id("1").with_execution_order_id("2").serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Control::visit(&mut reader, tag).unwrap());
    }

    #[test]
    fn negated_jump() {
        let content = r###"
            <?xml version="1.0" encoding="UTF-8"?>
            <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
                <interface>
                    <localVars/>
                    <addData>
                        <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                            <textDeclaration>
                                <content>
                                    PROGRAM program_0
                                    VAR&#xd;
                                        x: BOOL := 0;&#xd;
                                    END_VAR
                                </content>
                            </textDeclaration>
                        </data>
                    </addData>
                </interface>
                <body>
                    <FBD>
                        <inVariable localId="0" height="20" width="80" negated="false">
                            <position x="160" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>x</expression>
                        </inVariable>
                        <label localId="1" height="20" width="80" label="lbl" executionOrderId="0">
                            <position x="570" y="50"/>
                        </label>
                        <jump localId="2" height="20" width="80" label="lbl" executionOrderId="1">
                            <position x="320" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="0"/>
                            </connectionPointIn>
                            <addData>
                                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                                    <negated value="true"/>
                                </data>
                            </addData>
                        </jump>
                        <outVariable localId="3" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                            <position x="790" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="4"/>
                            </connectionPointIn>
                            <expression>x</expression>
                        </outVariable>
                        <inVariable localId="4" height="20" width="80" negated="false">
                            <position x="640" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>FALSE</expression>
                        </inVariable>
                    </FBD>
                </body>
            </pou>
        "###;

        assert_debug_snapshot!(xml_parser::visit(content));
    }

    #[test]
    fn label_parsed() {
        let content = XLabel::new().with_local_id("1").with_execution_order_id("2").serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Control::visit(&mut reader, tag).unwrap());
    }

    #[test]
    fn jump_and_label_converted_to_ast() {
        let content = r###"
            <?xml version="1.0" encoding="UTF-8"?>
            <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
                <interface>
                    <localVars/>
                    <addData>
                        <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                            <textDeclaration>
                                <content>
                                    PROGRAM program_0
                                    VAR
                                        x: BOOL := 0;
                                    END_VAR
                                </content>
                            </textDeclaration>
                        </data>
                    </addData>
                </interface>
                <body>
                    <FBD>
                        <inVariable localId="0" height="20" width="80" negated="false">
                            <position x="160" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>x</expression>
                        </inVariable>
                        <label localId="1" height="20" width="80" label="lbl" executionOrderId="0">
                            <position x="570" y="50"/>
                        </label>
                        <jump localId="2" height="20" width="80" label="lbl" executionOrderId="1">
                            <position x="320" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="0"/>
                            </connectionPointIn>
                            <addData>
                                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                                    <negated value="false"/>
                                </data>
                            </addData>
                        </jump>
                        <outVariable localId="3" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                            <position x="790" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="4"/>
                            </connectionPointIn>
                            <expression>x</expression>
                        </outVariable>
                        <inVariable localId="4" height="20" width="80" negated="false">
                            <position x="640" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>FALSE</expression>
                        </inVariable>
                    </FBD>
                </body>
            </pou>
        "###.to_string();

        let (ast, diagnostics) =
            xml_parser::parse(&content.into(), ast::ast::LinkageType::Internal, IdProvider::default());
        assert_debug_snapshot!(ast);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn unconnected_jump_generated_as_empty_statement() {
        let content = r###"
            <?xml version="1.0" encoding="UTF-8"?>
            <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
                <interface>
                    <localVars/>
                    <addData>
                        <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                            <textDeclaration>
                                <content>
                                    PROGRAM program_0
                                    VAR
                                        x: BOOL := 0;
                                    END_VAR
                                </content>
                            </textDeclaration>
                        </data>
                    </addData>
                </interface>
                <body>
                    <FBD>
                        <inVariable localId="0" height="20" width="80" negated="false">
                            <position x="160" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>x</expression>
                        </inVariable>
                        <label localId="1" height="20" width="80" label="lbl" executionOrderId="0">
                            <position x="570" y="50"/>
                        </label>
                        <jump localId="2" height="20" width="80" label="lbl" executionOrderId="1">
                            <position x="320" y="110"/>
                            <addData>
                                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                                    <negated value="false"/>
                                </data>
                            </addData>
                        </jump>
                        <outVariable localId="3" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                            <position x="790" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="4"/>
                            </connectionPointIn>
                            <expression>x</expression>
                        </outVariable>
                        <inVariable localId="4" height="20" width="80" negated="false">
                            <position x="640" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>FALSE</expression>
                        </inVariable>
                    </FBD>
                </body>
            </pou>
        "###.to_string();

        let (ast, diagnostics) =
            xml_parser::parse(&content.into(), ast::ast::LinkageType::Internal, IdProvider::default());
        assert_debug_snapshot!(ast);
        assert_debug_snapshot!(diagnostics);
    }

    #[test]
    fn negated_jump_ast() {
        let content = r###"
            <?xml version="1.0" encoding="UTF-8"?>
            <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
                <interface>
                    <localVars/>
                    <addData>
                        <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                            <textDeclaration>
                                <content>
                                    PROGRAM program_0
                                    VAR
                                        x: BOOL := 0;
                                    END_VAR
                                </content>
                            </textDeclaration>
                        </data>
                    </addData>
                </interface>
                <body>
                    <FBD>
                        <inVariable localId="0" height="20" width="80" negated="false">
                            <position x="160" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>x</expression>
                        </inVariable>
                        <label localId="1" height="20" width="80" label="lbl" executionOrderId="0">
                            <position x="570" y="50"/>
                        </label>
                        <jump localId="2" height="20" width="80" label="lbl" executionOrderId="1">
                            <position x="320" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="0"/>
                            </connectionPointIn>
                            <addData>
                                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                                    <negated value="true"/>
                                </data>
                            </addData>
                        </jump>
                        <outVariable localId="3" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                            <position x="790" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="4"/>
                            </connectionPointIn>
                            <expression>x</expression>
                        </outVariable>
                        <inVariable localId="4" height="20" width="80" negated="false">
                            <position x="640" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>FALSE</expression>
                        </inVariable>
                    </FBD>
                </body>
            </pou>
        "###.to_string();

        let (ast, diagnostics) =
            xml_parser::parse(&content.into(), ast::ast::LinkageType::Internal, IdProvider::default());
        assert_debug_snapshot!(ast);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn unnamed_controls() {
        let content = r###"
            <?xml version="1.0" encoding="UTF-8"?>
            <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
                <interface>
                    <localVars/>
                    <addData>
                        <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                            <textDeclaration>
                                <content>
                                    PROGRAM program_0
                                </content>
                            </textDeclaration>
                        </data>
                    </addData>
                </interface>
                <body>
                    <FBD>
                        <inVariable localId="0" height="20" width="80" negated="false">
                            <position x="160" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>x</expression>
                        </inVariable>
                        <label localId="1" height="20" width="80" label="" executionOrderId="0">
                            <position x="570" y="50"/>
                        </label>
                        <jump localId="2" height="20" width="80" label="" executionOrderId="1">
                            <position x="320" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="0"/>
                            </connectionPointIn>
                            <addData>
                                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                                    <negated value="false"/>
                                </data>
                            </addData>
                        </jump>
                        <outVariable localId="3" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                            <position x="790" y="110"/>
                            <connectionPointIn>
                                <relPosition x="0" y="10"/>
                                <connection refLocalId="4"/>
                            </connectionPointIn>
                            <expression>x</expression>
                        </outVariable>
                        <inVariable localId="4" height="20" width="80" negated="false">
                            <position x="640" y="110"/>
                            <connectionPointOut>
                                <relPosition x="80" y="10"/>
                            </connectionPointOut>
                            <expression>FALSE</expression>
                        </inVariable>
                    </FBD>
                </body>
            </pou>
        "###.to_string();

        let (ast, diagnostics) =
            xml_parser::parse(&content.into(), ast::ast::LinkageType::Internal, IdProvider::default());
        assert_debug_snapshot!(ast);
        assert_debug_snapshot!(diagnostics);
    }
}
