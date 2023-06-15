#[cfg(test)]
mod tests {
    use crate::{
        deserializer::Parseable,
        reader::PeekableReader,
        serializer::{
            XBlock, XConnection, XConnectionPointIn, XExpression, XInOutVariables, XInputVariables,
            XOutVariable, XOutputVariables, XPosition, XRelPosition, XVariable,
        },
    };

    #[test]
    fn variable() {
        let content = XBlock::init("1", "bar")
            .with_input_variables(
                XInputVariables::new()
                    .with_variable(XVariable::init("a", false))
                    .with_variable(XVariable::init("b", false)),
            )
            .with_output_variables(
                XOutputVariables::new()
                    .with_variable(XVariable::init("c", true))
                    .with_variable(XVariable::init("d", true)),
            )
            .with_inout_variables(XInOutVariables::new().close())
            .serialize();

        let mut reader = PeekableReader::new(&content);
        insta::assert_debug_snapshot!(crate::model::block::Block::visit(&mut reader));
    }

    #[test]
    fn out_variable() {
        let content = XOutVariable::new()
            .with_attribute("localId", "7")
            .with_attribute("negated", "false")
            .with_attribute("executionOrderId", "2")
            .with_position(XPosition::new().close())
            .with_connection_point_in(
                XConnectionPointIn::new()
                    .with_rel_position(XRelPosition::init().close())
                    .with_connection(XConnection::new().with_attribute("refLocalId", "1").close()),
            )
            .with_expression(XExpression::new().with_data("foo"))
            .serialize();

        println!("{content}");
        let mut reader = PeekableReader::new(&content);
        insta::assert_debug_snapshot!(crate::model::variables::FunctionBlockVariable::visit(&mut reader));
    }
}
