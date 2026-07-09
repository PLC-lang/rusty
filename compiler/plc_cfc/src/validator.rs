// TODO: Some invalid graphs are currently caught by the IDE rather than us, e.g. incomplete
//       connector/continuation pairs. Eventually we want to validate these here as well, so we do not solely
//       rely on the IDE.

use ast::provider::IdProvider;
use plc::{lexer, parser::expressions_parser::parse_expression};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocationFactory;

use crate::{
    model::{FbdObject, Pou},
    resolver::{Object, Resolver},
};

struct Context<'a> {
    pou: &'a Pou,
    factory: &'a SourceLocationFactory,
    resolver: &'a Resolver,
}

pub fn validate(pou: &Pou, factory: &SourceLocationFactory, resolver: &Resolver) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let context = Context { pou, factory, resolver };

    diagnostics.extend(validate_connections(&context));
    diagnostics.extend(validate_evaluation_order(&context));
    diagnostics.extend(validate_variable_object(&context));

    diagnostics
}

/// Validates that every consumed connection points at a value some object actually produces, i.e. that a
/// consumer (a block input, a sink, a return condition) references a real producer rather than a dangling
/// wire. For example
/// ```text
///    localA  --(2)                 (nothing produces 999)
///                     result  --(999?)-->  (0)
/// ```
/// where the sink reads connection `999`, which no object produces — a stale `refConnectionPointOutId`, a
/// connector with no source feeding a continuation, or a connector/continuation cycle that leads back to
/// nothing.
///
/// Note: without this the dangling wire slips past the other checks and later panics the transpiler, whose
/// `resolve` treats an unknown connection as unreachable; reporting it here aborts compilation first.
fn validate_connections(ctx: &Context) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for op in ctx.pou.network().get_ordered_operations() {
        for id in op.get_connections_in() {
            if ctx.resolver.is_resolvable(id) {
                continue;
            }

            let message = format!(
                "Invalid connection, `{consumer}` references a value that no object in the network produces",
                consumer = op.get_name(),
            );

            diagnostics.push(
                Diagnostic::new(message).with_error_code("E081").with_location(helper::create_location(
                    ctx.factory,
                    op.get_global_id(),
                    op.get_priority(),
                )),
            );
        }
    }

    diagnostics
}

/// Validates the evaluation order such that a consumer must first produce a value before it can be consumed
/// by a consumer. Specifically a block must have an evaluation order less than its consumer. For example
/// ```text
/// +--- myFunc ---+  (1)
/// |        myFunc|----------->  result  (0)
/// +--------------+
///
/// (n) = evaluation priority; result runs first, no temp exists yet
/// ```
fn validate_evaluation_order(ctx: &Context) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for op in ctx.pou.network().get_ordered_operations() {
        for id in op.get_connections_in() {
            let id = ctx.resolver.resolve_alias(id);
            let Some(Object::BlockOutput(block, _)) = ctx.resolver.get(id) else {
                continue;
            };

            // Check if the consumer has a priority which would evaluate before the block can produce a value
            if op.get_priority().is_none_or(|opt| opt <= block.add_data.priority.unwrap()) {
                let message = format!(
                    "Invalid evaluation order, result of `{producer}` is consumed by `{consumer}` before it is being evaluated",
                    producer = block.instance_name.as_deref().unwrap_or(&block.type_name),
                    consumer = op.get_name(),
                );

                let diagnostic = Diagnostic::new(message)
                    .with_error_code("E142")
                    .with_location(helper::create_location(
                        ctx.factory,
                        op.get_global_id(),
                        op.get_priority(),
                    ))
                    .with_secondary_location(helper::create_location(
                        ctx.factory,
                        block.global_id,
                        block.add_data.priority,
                    ));

                diagnostics.push(diagnostic);
            }
        }
    }

    diagnostics
}

/// Validates that variable objects (sources and sinks) only contain a literals, reference expressions or any
/// combination of these two (unary, binary). That is `foo`, `foo + 1`, `!foo` and so on are valid. That
/// allows for graphs like
/// ```text
///                            +---- clamp ----+  (0)
///    localA + 5  ----------->| value    clamp|----------->  result  (1)
///    NOT enable  ----------->| bypass        |
///                            +---------------+
/// ```
/// where the addition and negation would otherwise each need a dedicated block (and priority) of
/// their own.
///
/// Note: Technically not something we must support (i.e. we could just throw an error if not a reference),
/// however it's a huge improvement for the user to just declare `5` or `foo + 5` for assignments, hence we
/// are a bit "laxer" here.
fn validate_variable_object(ctx: &Context) -> Vec<Diagnostic> {
    let ids = IdProvider::default();
    let mut diagnostics = Vec::new();

    for object in &ctx.pou.network().objects {
        let (identifier, global_id, priority) = match object {
            FbdObject::DataSource(source) => (&source.identifier, source.global_id, None),
            FbdObject::DataSink(sink) => (&sink.identifier, sink.global_id, sink.add_data.priority),
            _ => continue,
        };

        let expr = parse_expression(&mut lexer::lex_with_ids(identifier, ids.clone(), ctx.factory.clone()));

        // Note: whether a sink is a valid assignment target (`foo := ...` rather than `foo + 1 := ...`)
        // needs no validation here; the transpiled assignment goes through the regular pipeline
        // validation, which reports non-assignable targets as E050.
        if !helper::is_value_expression(&expr) {
            let message = format!(
                "Invalid expression `{identifier}` in variable, only literals, variable references and compositions of them are allowed"
            );

            diagnostics.push(
                Diagnostic::new(message).with_error_code("E143").with_location(helper::create_location(
                    ctx.factory,
                    global_id,
                    priority,
                )),
            );
        }
    }

    diagnostics
}

mod helper {
    use ast::ast::{AstNode, AstStatement};
    use ast::visitor::{AstVisitor, Walker};
    use plc_source::source_location::{SourceLocation, SourceLocationFactory};

    /// Returns true if the given expression is a value, i.e. built only from literals, variable references
    /// and compositions of them such as `foo + 1` or `-bar[i].baz`. Everything else — calls, assignments,
    /// expression lists, ranges and so on — is not a value and must not appear in a variable.
    pub(super) fn is_value_expression(expression: &AstNode) -> bool {
        struct Validator {
            valid: bool,
        }

        impl AstVisitor for Validator {
            fn visit(&mut self, node: &AstNode) {
                // The visitor does not descend into the elements of an array literal, so a call hiding in
                // one (e.g. `[foo()]`) would go unnoticed; array literals are rejected outright instead.
                self.valid &= !node.is_literal_array()
                    && matches!(
                        node.get_stmt(),
                        AstStatement::Literal(_)
                            | AstStatement::Identifier(_)
                            | AstStatement::HardwareAccess(_)
                            | AstStatement::DirectAccess(_)
                            | AstStatement::ReferenceExpr(_)
                            | AstStatement::BinaryExpression(_)
                            | AstStatement::UnaryExpression(_)
                            | AstStatement::ParenExpression(_)
                    );

                node.walk(self);
            }
        }

        let mut validator = Validator { valid: true };
        validator.visit(expression);
        validator.valid
    }

    pub(super) fn create_location(
        factory: &SourceLocationFactory,
        global_id: u64,
        priority: Option<u64>,
    ) -> SourceLocation {
        factory.create_block_location(global_id as usize, priority.map(|priority| priority as usize))
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use plc_diagnostics::diagnostician::Diagnostician;
    use plc_diagnostics::reporter::DiagnosticReporter;
    use plc_source::source_location::SourceLocationFactory;

    use crate::{model, resolver::Resolver};

    fn validate(xml: &str) -> String {
        let pou = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&pou);
        let diagnostics = super::validate(&pou, &SourceLocationFactory::internal(xml), &resolver);

        let mut reporter = Diagnostician::buffered();
        reporter.register_file("<internal>".to_string(), xml.to_string());
        reporter.handle(&diagnostics);
        reporter.buffer().expect("internal error with the buffered codespan reporter")
    }

    #[test]
    fn sink_consumes_result_too_early() {
        //    +-- alwaysFive --+ (1)
        //    |      alwaysFive|--(2)-->  result  (0)
        //    +----------------+
        //
        //    (n)   evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/invalid/evaluation_order/sink.cfc");
        assert_snapshot!(validate(xml), @r"
        error[E142]: Invalid evaluation order, result of `alwaysFive` is consumed by `result` before it is being evaluated
         = at <internal>:block 3
         = see also <internal>:block 1
        ");
    }

    #[test]
    fn conditional_return_consumes_result_too_early() {
        //    +--- isReady ----+ (1)
        //    |         isReady|--(2)-->| RETURN |  (0)
        //    +----------------+
        let xml = include_str!("../fixtures/invalid/evaluation_order/conditional_return.cfc");
        assert_snapshot!(validate(xml), @r"
        error[E142]: Invalid evaluation order, result of `isReady` is consumed by `conditional return` before it is being evaluated
         = at <internal>:block 3
         = see also <internal>:block 1
        ");
    }

    #[test]
    fn block_argument_consumes_result_too_early() {
        //    +-- alwaysFive --+ (1)      +---- square ----+ (0)
        //    |      alwaysFive|--(2)---->| x        square|--(4)-->  result  (2)
        //    +----------------+          +----------------+
        let xml = include_str!("../fixtures/invalid/evaluation_order/block_argument.cfc");
        assert_snapshot!(validate(xml), @r"
        error[E142]: Invalid evaluation order, result of `alwaysFive` is consumed by `square` before it is being evaluated
         = at <internal>:block 3
         = see also <internal>:block 1
        ");
    }

    #[test]
    fn aliased_sink_consumes_result_too_early() {
        //    +-- alwaysFive --+ (1)
        //    |      alwaysFive|--(2)-->[ Connector "relay" ]
        //    +----------------+
        //
        //                       [ Continuation "relay" ]--(5)-->  result  (0)
        let xml = include_str!("../fixtures/invalid/evaluation_order/alias.cfc");
        assert_snapshot!(validate(xml), @r"
        error[E142]: Invalid evaluation order, result of `alwaysFive` is consumed by `result` before it is being evaluated
         = at <internal>:block 6
         = see also <internal>:block 1
        ");
    }

    #[test]
    fn call_in_source_variable() {
        //    conjure() + 5  --(2)-->  result  (0)
        //
        //    (n)   evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/invalid/call_in_variable/source.cfc");
        assert_snapshot!(validate(xml), @r"
        error[E143]: Invalid expression `conjure() + 5` in variable, only literals, variable references and compositions of them are allowed
         = at <internal>:block 1
        ");
    }

    #[test]
    fn call_in_sink_variable() {
        //    localA  --(2)-->  drain()  (0)
        let xml = include_str!("../fixtures/invalid/call_in_variable/sink.cfc");
        assert_snapshot!(validate(xml), @r"
        error[E143]: Invalid expression `drain()` in variable, only literals, variable references and compositions of them are allowed
         = at <internal>:block 3
        ");
    }

    #[test]
    fn value_expressions_in_variables_are_valid() {
        //    localA + 5  ----------->  result   (0)
        //
        //    (0)  evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/expression_source/mainProgram.cfc");
        assert_snapshot!(validate(xml), @"");
    }

    #[test]
    fn dangling_connection_is_reported() {
        //    localA  --(2)      result  --(999?)-->  (nothing produces 999)
        //
        //    the sink references id 999, which no object produces
        let xml = include_str!("../fixtures/invalid/dangling_connection/mainProgram.cfc");
        assert_snapshot!(validate(xml), @r"
        error[E081]: Invalid connection, `result` references a value that no object in the network produces
         = at <internal>:block 3
        ");
    }

    #[test]
    fn connector_continuation_cycle_is_reported() {
        //    [Cont y]-->[Conn x]   [Cont x]-->[Conn y]   [Cont x]--(10)-->  result  (0)
        //
        //    the connector/continuation pairs feed each other, so the sink's wire (10) resolves to no producer
        let xml = include_str!("../fixtures/invalid/connector_continuation_cycle/mainProgram.cfc");
        assert_snapshot!(validate(xml), @r"
        error[E081]: Invalid connection, `result` references a value that no object in the network produces
         = at <internal>:block 5
        ");
    }

    #[test]
    fn connector_without_source_is_reported() {
        //    (no source)-->[ Connector "relay" ]
        //
        //    [ Continuation "relay" ]--(10)-->  result  (0)
        //
        //    the connector carries no incoming wire, so the continuation feeding the sink resolves to nothing
        let xml = include_str!("../fixtures/invalid/connector_without_source/mainProgram.cfc");
        assert_snapshot!(validate(xml), @r"
        error[E081]: Invalid connection, `result` references a value that no object in the network produces
         = at <internal>:block 3
        ");
    }
}
