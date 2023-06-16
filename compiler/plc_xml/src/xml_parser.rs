use plc::{
    ast::{
        AstStatement, CompilationUnit, Implementation, LinkageType, PouType as AstPouType, SourceRange,
        SourceRangeFactory,
    },
    diagnostics::{Diagnostic, Diagnostician},
    lexer::{self, IdProvider},
    parser::expressions_parser::parse_expression,
};

use crate::{
    deserializer::visit,
    model::{
        action::Action,
        pou::{Pou, PouType},
        project::Project,
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
    // create a new parse session
    let parser = CfcParseSession::new(source, location, id_provider, linkage);

    // transform the xml file to a data model
    let Ok(project) = visit(source) else {
        todo!("cfc errors need to be transformed into diagnostics")
    };

    // try to parse a declaration data field
    let Some((unit, diagnostics)) = parser.try_parse_declaration(linkage, &project) else {
        unimplemented!("XML schemas without text declarations are not yet supported")
    };

    // transform the data model into rusty AST statements and add them to the compilation unit
    (dbg!(unit.with_implementations(parser.parse_model(project))), diagnostics)
}

struct CfcParseSession<'parse> {
    source: &'parse str,
    id_provider: IdProvider,
    location: &'static str,
    linkage: LinkageType,
}

impl<'parse> CfcParseSession<'parse> {
    fn new(
        source: &'parse str,
        location: &'static str,
        id_provider: IdProvider,
        linkage: LinkageType,
    ) -> Self {
        CfcParseSession { source, id_provider, location, linkage }
    }

    fn build_range_factory(&self) -> SourceRangeFactory {
        SourceRangeFactory::for_file(self.location)
    }

    fn try_parse_declaration(
        &self,
        linkage: LinkageType,
        project: &Project,
    ) -> Option<(CompilationUnit, Vec<Diagnostic>)> {
        let Some(content) = project.pous
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
            lexer::lex_with_ids(content, self.id_provider.clone(), self.build_range_factory()),
            linkage,
            self.location,
        ))
    }

    fn parse_expression(&self, expr: &str) -> AstStatement {
        parse_expression(&mut lexer::lex_with_ids(expr, self.id_provider.clone(), self.build_range_factory()))
    }

    fn parse_model(&self, project: Project) -> Vec<Implementation> {
        let mut implementations = vec![];
        for pou in project.pous {
            // transform body
            implementations.push(pou.build_implementation(self.linkage));
            // transform actions
            pou.actions
                .iter()
                .for_each(|action| implementations.push(action.build_implementation(self.linkage)));
        }
        implementations
    }
}

trait Transformable {
    fn transform(&self) -> Vec<AstStatement>;
    fn build_implementation(&self, linkage: LinkageType) -> Implementation;
}

impl Transformable for Pou {
    fn transform(&self) -> Vec<AstStatement> {
        vec![]
    }

    // TODO: sourcerange
    fn build_implementation(&self, linkage: LinkageType) -> Implementation {
        Implementation {
            name: self.name.to_owned(),
            type_name: self.name.to_owned(),
            linkage,
            pou_type: self.pou_type.into(),
            statements: self.transform(),
            location: SourceRange::undefined(),
            name_location: SourceRange::undefined(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}

impl Transformable for Action {
    fn transform(&self) -> Vec<AstStatement> {
        todo!()
    }

    // TODO: sourcerange
    fn build_implementation(&self, linkage: LinkageType) -> Implementation {
        Implementation {
            name: self.name.to_owned(),
            type_name: self.type_name.to_owned(),
            linkage,
            pou_type: AstPouType::Action,
            statements: self.transform(),
            location: SourceRange::undefined(),
            name_location: SourceRange::undefined(),
            overriding: false,
            generic: false,
            access: None,
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
