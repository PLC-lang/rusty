use logos::Logos;

use plc::{
    ast::{AstStatement, CompilationUnit, LinkageType, SourceRange, SourceRangeFactory},
    diagnostics::{Diagnostic, Diagnostician},
    lexer::{self, IdProvider},
    parser::expressions_parser::parse_expression,
};

use crate::{deserializer::visit, model::pou::Pou};

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

fn parse<'source>(
    source: &'source str,
    location: &'static str,
    linkage: LinkageType,
    id_provider: IdProvider,
) -> (CompilationUnit, Vec<Diagnostic>) {
    let parser = CfcParser::new(source, location, id_provider);

    // parse the declaration data field
    let (unit, diagnostics) = parser.parse_declaration(linkage);

    let ast = parser.transform();

    (unit, diagnostics)
}

struct CfcParser<'parse> {
    source: &'parse str,
    id_provider: IdProvider,
    location: &'static str,
    model: Pou,
}

impl<'parse> CfcParser<'parse> {
    fn new(source: &'parse str, location: &'static str, id_provider: IdProvider) -> Self {
        let Ok(model) = visit(source) else {
            unimplemented!("cfc errors need to be transformed into diagnostics")
        };
        CfcParser { source, id_provider, location, model }
    }

    fn parse_declaration(&self, linkage: LinkageType) -> (CompilationUnit, Vec<Diagnostic>) {
        plc::parser::parse(
            lexer::lex_with_ids(
                &self.model.declaration,
                self.id_provider.clone(),
                self.build_range_factory(),
            ),
            linkage,
            self.location,
        )
    }

    fn parse_expression(&self, expr: &str) -> AstStatement {
        parse_expression(&mut lexer::lex_with_ids(expr, self.id_provider.clone(), self.build_range_factory()))
    }

    fn transform(&self) -> AstStatement {
        AstStatement::EmptyStatement { location: SourceRange::undefined(), id: 0 }
    }

    fn build_range_factory(&self) -> SourceRangeFactory {
        SourceRangeFactory::for_file(self.location)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use plc::ast::{AstStatement, Operator};

    use crate::{cfc_parser::ASSIGNMENT_A_B, deserializer, serializer};

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
