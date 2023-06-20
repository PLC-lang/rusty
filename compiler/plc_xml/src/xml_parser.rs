use indexmap::IndexMap;
use plc::{
    ast::{
        AstId, AstStatement, CompilationUnit, Implementation, LinkageType, Operator, PouType as AstPouType,
        SourceRange, SourceRangeFactory,
    },
    diagnostics::{Diagnostic, Diagnostician},
    lexer::{self, IdProvider},
    parser::expressions_parser::parse_expression,
};

use crate::{
    deserializer::visit,
    model::{
        action::Action,
        block::Block,
        fbd::{FunctionBlockDiagram, Node, NodeId, NodeIndex},
        pou::{Pou, PouType},
        project::Project,
        variables::{BlockVariable, FunctionBlockVariable},
    },
};

pub fn parse_file(
    source: &str,
    location: &'static str,
    linkage: LinkageType,
    id_provider: IdProvider,
    diagnostician: &mut Diagnostician,
) -> CompilationUnit {
    let (unit, errors) = parse(source, location, linkage, id_provider);
    //Register the source file with the diagnostician
    diagnostician.register_file(location.to_string(), source.to_string());
    diagnostician.handle(errors);
    unit
}

fn parse(
    source: &str,
    location: &'static str,
    linkage: LinkageType,
    id_provider: IdProvider,
) -> (CompilationUnit, Vec<Diagnostic>) {
    // transform the xml file to a data model
    let Ok(project) = visit(source) else {
        todo!("cfc errors need to be transformed into diagnostics")
    };

    // create a new parse session
    let mut parser = ParseSession::new(&project, location, id_provider, linkage);

    // try to parse a declaration data field
    let Some((unit, diagnostics)) = parser.try_parse_declaration() else {
        unimplemented!("XML schemas without text declarations are not yet supported")
    };

    // transform the data model into rusty AST statements and add them to the compilation unit
    (unit.with_implementations(parser.parse_model()), diagnostics)
}

struct ParseSession<'parse> {
    project: &'parse Project,
    id_provider: IdProvider,
    linkage: LinkageType,
    file_name: &'static str,
    range_factory: SourceRangeFactory,
    references: IndexMap<NodeId, Vec<NodeId>>,
}

impl<'parse> ParseSession<'parse> {
    fn new(
        project: &'parse Project,
        file_name: &'static str,
        id_provider: IdProvider,
        linkage: LinkageType,
    ) -> Self {
        ParseSession {
            project,
            id_provider,
            linkage,
            file_name,
            range_factory: SourceRangeFactory::for_file(file_name),
            references: IndexMap::new(),
        }
    }

    fn try_parse_declaration(&self) -> Option<(CompilationUnit, Vec<Diagnostic>)> {
        let Some(content) = self.project.pous
            .first()
            .and_then(|it|
                it.interface
                    .as_ref()
                    .and_then(|it|
                        it.get_data_content()
                    )
        ) else {
            return None
        };

        //TODO: if our ST parser returns a diagnostic here, we might not have a text declaration and need to rely on the XML to provide us with
        // the necessary data. for now, we will assume to always have a text declaration
        Some(plc::parser::parse(
            lexer::lex_with_ids(
                content,
                self.id_provider.clone(),
                SourceRangeFactory::for_file(self.file_name),
            ),
            self.linkage,
            self.file_name,
        ))
    }

    fn parse_expression(&self, expr: &str) -> AstStatement {
        parse_expression(&mut lexer::lex_with_ids(
            html_escape::decode_html_entities_to_string(expr, &mut String::new()),
            self.id_provider.clone(),
            SourceRangeFactory::for_file(self.file_name),
        ))
    }

    fn parse_model(&mut self) -> Vec<Implementation> {
        let mut implementations = vec![];
        for pou in &self.project.pous {
            // transform body
            implementations.push(pou.build_implementation(self));
            // transform actions
            pou.actions.iter().for_each(|action| implementations.push(action.build_implementation(self)));
        }
        implementations
    }

    fn next_id(&mut self) -> AstId {
        self.id_provider.next_id()
    }

    fn create_range(&self, range: core::ops::Range<usize>) -> SourceRange {
        self.range_factory.create_range(range)
    }

    fn get_referencing_ids(&self, id: NodeId) -> Vec<&NodeId> {
        self.references.iter().filter(|(_, v)| v.contains(&id)).map(|(k, _)| k).collect()
    }
}

impl Pou {
    fn transform(&self, session: &mut ParseSession) -> Vec<AstStatement> {
        let Some(fbd) = &self.body.function_block_diagram else {
            // empty body
            return vec![]
        };

        let statements = fbd.transform(session);

        #[cfg(feature = "debug")]
        println!("{statements:#?}");

        statements
    }

    // TODO: sourcerange
    fn build_implementation(&self, session: &mut ParseSession) -> Implementation {
        Implementation {
            name: self.name.to_owned(),
            type_name: self.name.to_owned(),
            linkage: session.linkage,
            pou_type: self.pou_type.into(),
            statements: self.transform(session),
            location: SourceRange::undefined(),
            name_location: SourceRange::undefined(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}

impl FunctionBlockDiagram {
    fn transform(&self, session: &mut ParseSession) -> Vec<AstStatement> {
        // self.build_reference_table(session);
        let mut ast_association = IndexMap::new();
        self.nodes.iter().for_each(|(id, _)| self.transform_node(*id, session, &mut ast_association));
        ast_association
            .into_iter()
            .filter(|(k, _)| self.nodes.get(k).is_some_and(|it| it.get_exec_id().is_some()))
            .map(|(_, v)| v)
            .collect()
    }

    fn transform_node(
        &self,
        id: NodeId,
        session: &mut ParseSession,
        ast_association: &mut IndexMap<NodeId, AstStatement>,
    ) {
        let Some(current_node) = self.nodes.get(&id) else {
            unreachable!()
        };
        match current_node {
            Node::Block(block) => block.transform(session, &self.nodes, ast_association),
            Node::FunctionBlockVariable(var) => {
                let stmt = var.transform(session);

                // if we are not being assigned to, we can return here
                let Some(ref_id) = var.ref_local_id else {
                    ast_association.insert(id, stmt);
                    return;
                };

                let rhs = if let Some(rhs) = ast_association.remove(&ref_id) {
                    rhs
                } else {
                    // that is awkward
                    self.transform_node(ref_id, session, ast_association);
                    let Some(entry) = ast_association.remove(&ref_id) else {
                        return;
                    };
                    entry
                };

                ast_association.insert(
                    id,
                    AstStatement::Assignment {
                        left: Box::new(stmt),
                        right: Box::new(rhs),
                        id: session.id_provider.next_id(),
                    },
                );
            }
            Node::Control(_) => todo!(),
            Node::Connector(_) => todo!(),
        }
    }

    fn build_reference_table(&self, session: &mut ParseSession) {
        session.references.clear();
        let _ = self
            .nodes
            .iter()
            .map(|(id, node)| session.references.insert(*id, node.get_ref_ids()))
            .collect::<Vec<_>>();
    }
}

impl Action {
    fn transform(&self, session: &mut ParseSession) -> Vec<AstStatement> {
        todo!()
    }

    // TODO: sourcerange
    fn build_implementation(&self, session: &mut ParseSession) -> Implementation {
        Implementation {
            name: self.name.to_owned(),
            type_name: self.type_name.to_owned(),
            linkage: session.linkage,
            pou_type: AstPouType::Action,
            statements: self.transform(session),
            location: SourceRange::undefined(),
            name_location: SourceRange::undefined(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}

impl FunctionBlockVariable {
    fn transform(&self, session: &mut ParseSession) -> AstStatement {
        let stmt = if self.negated {
            let ident = session.parse_expression(&self.expression);
            let location = ident.get_location();
            AstStatement::UnaryExpression {
                operator: Operator::Not,
                value: Box::new(ident),
                location,
                id: session.id_provider.next_id(),
            }
        } else {
            session.parse_expression(&self.expression)
        };

        stmt
    }
}

impl Block {
    fn transform(
        &self,
        session: &mut ParseSession,
        index: &NodeIndex,
        ast_association: &mut IndexMap<usize, AstStatement>,
    ) {
        let operator = Box::new(AstStatement::Reference {
            name: self.type_name.clone(),
            location: SourceRange::undefined(),
            id: session.next_id(),
        });

        let parameters = if self.variables.len() > 0 {
            Box::new(Some(AstStatement::ExpressionList {
                expressions: self.variables.iter().filter_map(|var| var.transform(session, index)).collect(),
                id: session.next_id(),
            }))
        } else {
            Box::new(None)
        };

        ast_association.insert(
            self.local_id,
            AstStatement::CallStatement {
                operator,
                parameters,
                location: SourceRange::undefined(),
                id: session.next_id(),
            },
        );
    }
}

impl BlockVariable {
    fn transform(&self, session: &mut ParseSession, index: &NodeIndex) -> Option<AstStatement> {
        let Some(ref_id) = self.ref_local_id else {
            // param not provided/passed
            return None
        };

        let Some(ref_node) = index.get(&ref_id) else {
            unreachable!()
        };

        match ref_node {
            Node::Block(_) => {
                // let name = block.instance_name.as_ref().unwrap_or(&block.type_name).as_str();
                // dbg!(Some(session.parse_expression(name)))
                None
            }
            // result assignment happens here
            Node::FunctionBlockVariable(var) => Some(var.transform(session)),
            Node::Control(_) => todo!(),
            Node::Connector(_) => todo!(),
        }
    }
}

trait CompilationUnitExt {
    fn with_implementations(self, implementations: Vec<Implementation>) -> Self;
}

impl CompilationUnitExt for CompilationUnit {
    fn with_implementations(self, implementations: Vec<Implementation>) -> Self {
        CompilationUnit {
            global_vars: self.global_vars,
            units: self.units,
            implementations,
            user_types: self.user_types,
            file_name: self.file_name,
            new_lines: self.new_lines,
        }
    }
}

// XXX: that seems redundant.. we only need our own enum because we impl Display
impl From<PouType> for AstPouType {
    fn from(value: PouType) -> Self {
        match value {
            PouType::Program => AstPouType::Program,
            PouType::Function => AstPouType::Function,
            PouType::FunctionBlock => AstPouType::FunctionBlock,
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{deserializer, xml_parser::ASSIGNMENT_A_B};

    #[test]
    fn variable_assignment() {
        let pou = crate::deserializer::visit(ASSIGNMENT_A_B).unwrap();
        assert_debug_snapshot!(pou);
    }

    #[test]
    fn model_is_sorted_by_execution_order() {
        let src = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="thistimereallyeasy" pouType="program">
            <interface>
                <localVars/>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <textDeclaration>
                            <content>
        PROGRAM thistimereallyeasy
        VAR
            a, b, c, d : DINT;
        END_VAR
                            </content>
                        </textDeclaration>
                    </data>
                </addData>
            </interface>
            <body>
                <FBD>
                    <inVariable localId="1" height="20" width="80" negated="false">
                        <position x="410" y="130"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>a</expression>
                    </inVariable>
                    <outVariable localId="2" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                        <position x="550" y="70"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>b</expression>
                    </outVariable>
                    <outVariable localId="3" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                        <position x="550" y="130"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>c</expression>
                    </outVariable>
                    <outVariable localId="4" height="20" width="80" executionOrderId="1" negated="false" storage="none">
                        <position x="550" y="190"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>d</expression>
                    </outVariable>
                </FBD>
            </body>
        </pou>
        "#;

        assert_debug_snapshot!(deserializer::visit(src).unwrap());
    }

    #[test]
    fn expression_can_be_parsed() {
        let expression = "a + b * 3";
        // let expected = AstStatement::BinaryExpression {
        //     operator: Operator::Plus,
        //     left: AstStatement::Reference { name: "a".to_string(), location: (0..1).into(), id: () },
        //     right: todo!(),
        //     id: todo!(),
        // };
        // dbg!(parse_cfc_expression(expression));
    }

    // #[test]
    // fn declaration_can_be_parsed() {
    //     deserializer::visit(ASSIGNMENT_A_B).unwrap().parse_declaration();
    // }
}

const ASSIGNMENT_A_B: &str = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="thistimereallyeasy" pouType="program">
            <interface>
                <localVars/>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <textDeclaration>
                            <content>
        PROGRAM thistimereallyeasy
        VAR
            a, b : DINT;
        END_VAR
                            </content>
                        </textDeclaration>
                    </data>
                </addData>
            </interface>
            <body>
                <FBD>
                    <outVariable localId="2" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                        <position x="550" y="130"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>b</expression>
                    </outVariable>
                    <inVariable localId="1" height="20" width="80" negated="false">
                    <position x="410" y="130"/>
                    <connectionPointOut>
                        <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>a</expression>
                </inVariable>
                </FBD>
            </body>
        </pou>
"#;
