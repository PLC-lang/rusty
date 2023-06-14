use std::collections::HashMap;

use crate::{
    deserializer,
    model::{
        fbd::Node,
        variables::{FunctionBlockVariable, VariableKind},
    },
};

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::cfc_parser::ASSIGNMENT_A_B;

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

        // let pou = crate::deserializer::visit(src).unwrap().sort_by_execution_order();

        // assert_debug_snapshot!(pou);
    }
}

#[test]
fn aaaa() {
    let mut deser = deserializer::visit(ASSIGNMENT_A_B).unwrap();
    dbg!(&deser.body.function_block_diagram);
    let mut fbd = deser.body.function_block_diagram;
    // let Node::FunctionBlockVariable(var) = fbd.nodes.get(&1).unwrap() else { unreachable!() };

    dbg!(&fbd);
    fbd.nodes.sort_by(|_, b, _, d| b.partial_cmp(d).unwrap());
    dbg!(&fbd);

    // assert_eq!(
    //     var,
    //     &FunctionBlockVariable::new(
    //         HashMap::from([
    //             ("localId".to_string(), "1".to_string()),
    //             ("negated".to_string(), "false".to_string()),
    //             ("expression".to_string(), "a".to_string())
    //         ]),
    //         VariableKind::Input
    //     )
    //     .unwrap()
    // );
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
                    <inVariable localId="1" height="20" width="80" negated="false">
                        <position x="410" y="130"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>a</expression>
                    </inVariable>
                    <outVariable localId="4" height="20" width="80" executionOrderId="3" negated="false" storage="none">
                        <position x="550" y="130"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="2"/>
                        </connectionPointIn>
                        <expression>b</expression>
                    </outVariable>
                    <outVariable localId="2" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                        <position x="550" y="130"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>b</expression>
                    </outVariable>
                </FBD>
            </body>
        </pou>
"#;
