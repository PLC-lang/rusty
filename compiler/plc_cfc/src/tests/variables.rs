#[cfg(test)]
mod tests {
    use crate::{
        deserializer::Parseable,
        reader::PeekableReader,
        serializer::{
            Block, Connection, ConnectionPointIn, Expression, InOutVariables, InputVariables, OutVariable,
            OutputVariables, Position, RelPosition, Variable,
        },
    };

    #[test]
    fn variable() {
        let content = Block::init("1", "bar")
            .with_input_variables(
                InputVariables::new()
                    .with_variable(Variable::init("a", false))
                    .with_variable(Variable::init("b", false)),
            )
            .with_output_variables(
                OutputVariables::new()
                    .with_variable(Variable::init("c", true))
                    .with_variable(Variable::init("d", true)),
            )
            .with_inout_variables(InOutVariables::new().close())
            .serialize();

        let mut reader = PeekableReader::new(&content);
        insta::assert_debug_snapshot!(crate::model::block::Block::visit(&mut reader));
    }

    #[test]
    fn out_variable() {
        let content = OutVariable::new()
            .with_attribute("localId", "7")
            .with_attribute("negated", "false")
            .with_attribute("executionOrderId", "2")
            .with_position(Position::new().close())
            .with_connection_point_in(
                ConnectionPointIn::new()
                    .with_rel_position(RelPosition::init().close())
                    .with_connection(Connection::new().with_attribute("refLocalId", "1").close()),
            )
            .with_expression(Expression::new().with_data("foo"))
            .serialize();

        println!("{content}");
        let mut reader = PeekableReader::new(&content);
        insta::assert_debug_snapshot!(crate::model::variables::FunctionBlockVariable::visit(&mut reader));
    }
}
