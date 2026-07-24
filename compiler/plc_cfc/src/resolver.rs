use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use plc_ast::ast::{AstFactory, AstNode, AstStatement};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};
use plc_source::SourceCode;

use plc::index::Index;

use crate::model::{self, FbdObject, Pin};
use crate::network::{Argument, Network, Statement, Temporary};
use crate::st;

pub struct Resolver<'index> {
    ids: IdProvider,
    factory: SourceLocationFactory,
    diagnostics: Vec<Diagnostic>,
    index: &'index Index,
}

enum Role {
    Source,
    Sink,
    Return,
    Jump,
    Label,
    Block,
    Connector,
    Continuation,
    Unconnected,
    Other,
}

// A plain variable/literal, or a block output (read as `instance.member`).
enum Source<'model> {
    Variable(&'model FbdObject),
    Output { block: &'model FbdObject, pin: &'model Pin },
}

// Everything the linking pass needs to know up front.
struct Survey<'model> {
    by_pin: HashMap<usize, &'model FbdObject>,
    block_output: HashMap<usize, &'model Pin>,
    connector_by_label: HashMap<&'model str, &'model FbdObject>,
    labels: HashSet<&'model str>,
    targets: HashSet<&'model str>,
    consumed: HashSet<usize>,
}

enum Trace<'model> {
    Reached(Source<'model>),
    DeadEnd(Broken<'model>),
    Unwired,
}

struct Broken<'model> {
    element: &'model FbdObject,
    reason: Reason,
}

enum Reason {
    OpenConnector,
    DanglingContinuation,
}

impl<'index> Resolver<'index> {
    pub fn new(ids: IdProvider, source: &SourceCode, index: &'index Index) -> Self {
        Self { ids, factory: SourceLocationFactory::for_source(source), diagnostics: Vec::new(), index }
    }

    pub fn resolve(mut self, network: &model::Network) -> (Network, Vec<Diagnostic>) {
        let mut survey = self.survey(network);
        survey.consumed = consumed(network, &survey);

        let mut statements = Vec::new();
        let mut temporaries = Vec::new();
        let mut broken = Vec::new();
        for object in network.elements() {
            match Role::from(object) {
                Role::Sink => match trace(consumes(object), &survey) {
                    // The traced source becomes the sink's assigned value.
                    Trace::Reached(source) => {
                        let location = self.factory.create_block_location(object.global_id);
                        let sink = self.expression(object, &location);
                        let source = self.value(&source, &location);

                        let statement = Statement::Assignment { sink, source };
                        statements.push((object.priority(), statement));
                    }

                    // A consumed chain that never reached a source; reported below.
                    Trace::DeadEnd(at) => broken.push(at),

                    // An unwired sink assigns nothing; drop it.
                    Trace::Unwired => {}
                },

                Role::Return => match trace(consumes(object), &survey) {
                    // The traced source guards the return.
                    Trace::Reached(source) => {
                        let location = self.factory.create_block_location(object.global_id);
                        let condition = self.condition(object, &source, &location);

                        let statement = Statement::Return { condition, location };
                        statements.push((object.priority(), statement));
                    }

                    // A consumed chain that never reached a source; reported below.
                    Trace::DeadEnd(at) => broken.push(at),

                    // A return without a condition is rejected; drop it.
                    Trace::Unwired => {
                        let location = self.factory.create_block_location(object.global_id);
                        self.diagnostics.push(Diagnostic::disconnected_return(location));
                    }
                },

                Role::Jump => {
                    let location = self.factory.create_block_location(object.global_id);
                    let target = object.target_label().unwrap_or_default().to_string();

                    // A jump to a name no label defines has nowhere to land.
                    if !survey.labels.contains(target.as_str()) {
                        self.diagnostics.push(Diagnostic::undefined_jump_target(&target, location.clone()));
                    }

                    match trace(consumes(object), &survey) {
                        // The traced source guards the jump.
                        Trace::Reached(source) => {
                            let condition = Some(self.condition(object, &source, &location));

                            let statement = Statement::Jump { condition, target, location };
                            statements.push((object.priority(), statement));
                        }

                        // A consumed chain that never reached a source; reported below.
                        Trace::DeadEnd(at) => broken.push(at),

                        // An unwired jump is legal but can never be taken; warn and keep it.
                        Trace::Unwired => {
                            self.diagnostics.push(Diagnostic::disconnected_jump(location.clone()));

                            let statement = Statement::Jump { condition: None, target, location };
                            statements.push((object.priority(), statement));
                        }
                    }
                }

                Role::Label => {
                    let location = self.factory.create_block_location(object.global_id);
                    let name = object.label().unwrap_or_default().to_string();

                    // A label no jump targets is dead routing; keep it, but flag it.
                    if !survey.targets.contains(name.as_str()) {
                        self.diagnostics.push(Diagnostic::unused_label(&name, location.clone()));
                    }

                    let statement = Statement::Label { name, location };
                    statements.push((object.priority(), statement));
                }

                // A callee the index doesn't know can't be classified; reject it.
                Role::Block if block::is_unknown(object, self.index) => {
                    let location = self.factory.create_block_location(object.global_id);
                    let name = object.type_name().unwrap_or_default();

                    self.diagnostics.push(Diagnostic::unknown_block_type(name, location));
                }

                // Stateless blocks (functions and sort of methods); temporaries for outputs
                Role::Block if block::is_stateless(object, self.index) => {
                    let location = self.factory.create_block_location(object.global_id);

                    let mut arguments = Vec::new();
                    for pin in block::inputs(object) {
                        let argument = match trace(pin.source_pin(), &survey) {
                            // The traced source becomes the pin's passed value.
                            Trace::Reached(source) => self.input(pin, &source, &location),

                            // A consumed chain that never reached a source; reported below.
                            Trace::DeadEnd(at) => {
                                broken.push(at);
                                self.empty_input(pin, &location)
                            }

                            // An unwired pin takes the callee's declared default.
                            Trace::Unwired => self.empty_input(pin, &location),
                        };

                        arguments.push(argument);
                    }

                    // Consumed outputs are captured into temporaries
                    let mut capture = None;
                    for pin in object.output_pins() {
                        let consumed = pin.output_pin().is_some_and(|id| survey.consumed.contains(&id));
                        let name = consumed.then(|| block::temp_name(object, pin));

                        if let Some(name) = &name {
                            if let Some(temporary) = self.temporary(name, object, pin, &location) {
                                temporaries.push((object.priority(), temporary));
                            }
                        }

                        match block::is_return_pin(object, pin) {
                            // The return value is received by assigning the call itself.
                            true => capture = name,

                            // Any other output is received inside the parentheses.
                            false => arguments.push(Argument::Output {
                                parameter: pin.parameter_name.clone(),
                                capture: name,
                            }),
                        }
                    }

                    let statement =
                        Statement::Call { target: block::call_target(object), arguments, capture, location };
                    statements.push((object.priority(), statement));
                }

                // A stateful call passes only its wired inputs; outputs read back as members.
                Role::Block => {
                    let location = self.factory.create_block_location(object.global_id);

                    let mut arguments = Vec::new();
                    for pin in block::inputs(object) {
                        match trace(pin.source_pin(), &survey) {
                            Trace::Reached(source) => arguments.push(self.input(pin, &source, &location)),

                            // A consumed chain that never reached a source; reported below.
                            Trace::DeadEnd(at) => broken.push(at),

                            // An unwired input passes nothing; the callee keeps last cycle's value.
                            Trace::Unwired => {}
                        }
                    }

                    let statement = Statement::Call {
                        target: block::call_target(object),
                        arguments,
                        capture: None,
                        location,
                    };
                    statements.push((object.priority(), statement));
                }

                // Placed but never wired; warn and ignore.
                Role::Unconnected => {
                    let location = self.factory.create_block_location(object.global_id);
                    let name = object.identifier().unwrap_or("<unnamed>");

                    self.diagnostics.push(Diagnostic::unconnected_element(name, location));
                }

                // Read-only (sources) or pure routing; no statements.
                Role::Source | Role::Connector | Role::Continuation | Role::Other => {}
            }
        }

        // Report a broken element once, no matter how many consumers hit it.
        broken.sort_by_key(|at| at.element.global_id);
        broken.dedup_by_key(|at| at.element.global_id);
        for at in broken {
            self.diagnostics.push(at.diagnostic(&self.factory));
        }

        // Statements and temporaries run in evaluation-priority order.
        statements.sort_by_key(|(priority, _)| priority.unwrap_or(usize::MAX));
        let statements = statements.into_iter().map(|(_, statement)| statement).collect();
        temporaries.sort_by_key(|(priority, _)| priority.unwrap_or(usize::MAX));
        let temporaries = temporaries.into_iter().map(|(_, temporary)| temporary).collect();

        (Network { statements, temporaries }, self.diagnostics)
    }

    // `parameter := value`, negated by the pin's inversion bubble.
    fn input(&mut self, pin: &Pin, source: &Source, location: &SourceLocation) -> Argument {
        let value = self.value(source, location);
        let value = self.negate_if(value, location, pin.negated);

        Argument::Input { parameter: pin.parameter_name.clone(), value: Box::new(value) }
    }

    // `parameter := `; an empty argument for an unwired parameter.
    fn empty_input(&mut self, pin: &Pin, location: &SourceLocation) -> Argument {
        let value = AstFactory::create_empty_statement(location.clone(), self.ids.next_id());
        Argument::Input { parameter: pin.parameter_name.clone(), value: Box::new(value) }
    }

    fn temporary(
        &mut self,
        name: &str,
        block: &FbdObject,
        pin: &Pin,
        location: &SourceLocation,
    ) -> Option<Temporary> {
        let Some(data_type) = block::temp_type(block, pin, self.index) else {
            self.diagnostics.push(Diagnostic::undeclared_block_output(
                &pin.parameter_name,
                block.type_name().unwrap_or_default(),
                location.clone(),
            ));

            return None;
        };

        Some(Temporary { name: name.to_string(), data_type, location: location.clone() })
    }

    fn survey<'model>(&mut self, network: &'model model::Network) -> Survey<'model> {
        let mut by_pin = HashMap::new();
        let mut block_output = HashMap::new();
        let mut connector_by_label = HashMap::new();
        let mut labels = HashSet::new();
        let mut targets = HashSet::new();

        for object in network.elements() {
            // A label name claimed twice makes its jump target ambiguous.
            if matches!(Role::from(object), Role::Label) {
                if let Some(label) = object.label() {
                    if !labels.insert(label) {
                        let location = self.factory.create_block_location(object.global_id);
                        self.diagnostics.push(Diagnostic::duplicate_label(label, location));
                    }
                }
            }

            if matches!(Role::from(object), Role::Jump) {
                if let Some(target) = object.target_label() {
                    targets.insert(target);
                }
            }

            // Register every output pin an incoming wire could reference; a block exposes one per output parameter.
            if let Some(out) = &object.connection_out {
                by_pin.insert(out.id, object);
            }

            if matches!(Role::from(object), Role::Block) {
                for pin in object.output_pins() {
                    if let Some(id) = pin.output_pin() {
                        by_pin.insert(id, object);
                        block_output.insert(id, pin);
                    }
                }
            }

            if matches!(Role::from(object), Role::Connector) {
                if let Some(label) = object.label() {
                    match connector_by_label.entry(label) {
                        // The first connector to claim a label owns it.
                        Entry::Vacant(entry) => {
                            entry.insert(object);
                        }

                        // A later one reusing the label is a duplicate.
                        Entry::Occupied(_) => {
                            let location = self.factory.create_block_location(object.global_id);
                            self.diagnostics.push(Diagnostic::duplicate_connector(label, location));
                        }
                    }
                }
            }
        }

        Survey { by_pin, block_output, connector_by_label, labels, targets, consumed: HashSet::new() }
    }

    fn expression(&mut self, object: &FbdObject, location: &SourceLocation) -> AstNode {
        let node = self.parse(object, location);

        // Expressions other than literals and references defined in the text declaration of a CFC graph node
        // are unsupported. A variable node element should only contain a literal or reference, not a call
        // for example (calls must use a block element instead).
        if !is_supported(&node.stmt) {
            let location = self.factory.create_block_location(object.global_id);
            let text = object.identifier().unwrap_or_default();
            self.diagnostics.push(Diagnostic::unsupported_cfc_expression(text, location));
        }

        node
    }

    // The value a source produces: a vetted plain expression, or a block-output member read.
    fn value(&mut self, source: &Source, location: &SourceLocation) -> AstNode {
        match source {
            Source::Variable(object) => self.expression(object, location),
            Source::Output { block, pin } => self.member_read(block, pin, location),
        }
    }

    // Creates an AST node, wrapped in a NOT expression if the consumer is negated in the graph.
    fn condition(&mut self, consumer: &FbdObject, source: &Source, location: &SourceLocation) -> AstNode {
        // Unvetted; a condition may be any ST expression.
        let node = match source {
            Source::Variable(object) => self.parse(object, location),
            Source::Output { block, pin } => self.member_read(block, pin, location),
        };

        self.negate_if(node, location, consumer.negated())
    }

    fn member_read(&mut self, block: &FbdObject, pin: &Pin, location: &SourceLocation) -> AstNode {
        let mut node = st::parse_expression(&block::read_target(block, pin, self.index), self.ids.clone());
        node.location = location.clone();

        self.negate_if(node, location, pin.negated)
    }

    fn negate_if(&mut self, node: AstNode, location: &SourceLocation, negated: bool) -> AstNode {
        match negated {
            true => AstFactory::create_not_expression(node, location.clone(), self.ids.next_id()),
            false => node,
        }
    }

    fn parse(&mut self, object: &FbdObject, location: &SourceLocation) -> AstNode {
        let mut node = st::parse_expression(object.identifier().unwrap_or_default(), self.ids.clone());
        node.location = location.clone();
        node
    }
}

impl From<&FbdObject> for Role {
    fn from(object: &FbdObject) -> Self {
        match object.kind.as_str() {
            "ppx:DataSource" => Role::Source,
            "ppx:DataSink" => Role::Sink,
            "ppx:Return" => Role::Return,
            "bmx:CfcJump" => Role::Jump,
            "bmx:CfcLabel" => Role::Label,
            "ppx:Block" => Role::Block,
            "ppx:Connector" => Role::Connector,
            "ppx:Continuation" => Role::Continuation,
            "ppx:Unconnected" => Role::Unconnected,
            _ => Role::Other,
        }
    }
}

impl Broken<'_> {
    fn diagnostic(&self, factory: &SourceLocationFactory) -> Diagnostic {
        let location = factory.create_block_location(self.element.global_id);
        let label = self.element.label().unwrap_or_default();

        match self.reason {
            Reason::OpenConnector => Diagnostic::open_connector(label, location),
            Reason::DanglingContinuation => Diagnostic::dangling_continuation(label, location),
        }
    }
}

fn trace<'model>(start: Option<usize>, survey: &Survey<'model>) -> Trace<'model> {
    let Some(mut pin) = start else { return Trace::Unwired };
    let Some(mut element) = survey.by_pin.get(&pin).copied() else { return Trace::Unwired };

    // Trace through the connections until we hit a producer (cycle-guarded).
    let mut visited = HashSet::new();
    while matches!(Role::from(element), Role::Continuation) {
        let dangling = |element| Trace::DeadEnd(Broken { element, reason: Reason::DanglingContinuation });

        // An unlabeled continuation, or one revisiting a label (a cycle), dangles.
        let Some(label) = element.label() else { return dangling(element) };
        if !visited.insert(label) {
            return dangling(element);
        }

        // Resolve the label to the connector owning it; an unclaimed label dangles.
        let Some(connector) = survey.connector_by_label.get(label).copied() else {
            return dangling(element);
        };

        // Step onto whatever feeds that connector; an input-less one is open.
        let open = Broken { element: connector, reason: Reason::OpenConnector };
        let Some(wire) = consumes(connector) else { return Trace::DeadEnd(open) };
        let Some(producer) = survey.by_pin.get(&wire).copied() else { return Trace::DeadEnd(open) };

        pin = wire;
        element = producer;
    }

    // A landed wire is a block output pin (read as a member) or a plain source.
    match survey.block_output.get(&pin).copied() {
        Some(output) => Trace::Reached(Source::Output { block: element, pin: output }),
        None => Trace::Reached(Source::Variable(element)),
    }
}

fn is_supported(statement: &AstStatement) -> bool {
    match statement {
        // See through parentheses, e.g. `(foo)` or `((5))`.
        AstStatement::ParenExpression(inner) => is_supported(&inner.stmt),
        AstStatement::Literal(_) | AstStatement::ReferenceExpr(_) => true,
        _ => false,
    }
}

// Block related helpers
mod block {
    use plc::index::Index;

    use super::{FbdObject, Pin};

    pub(super) fn is_unknown(block: &FbdObject, index: &Index) -> bool {
        block.type_name().and_then(|name| index.find_pou(name)).is_none()
    }

    pub(super) fn is_stateless(block: &FbdObject, index: &Index) -> bool {
        block.instance_name.is_none()
            && block.type_name().and_then(|name| index.find_pou(name)).is_some_and(|pou| pou.is_function())
    }

    pub(super) fn is_return_pin(block: &FbdObject, pin: &Pin) -> bool {
        block.type_name() == Some(pin.parameter_name.as_str())
    }

    pub(super) fn temp_name(block: &FbdObject, pin: &Pin) -> String {
        format!("__out_{}_{}", pin.parameter_name, block.global_id)
    }

    pub(super) fn inputs(block: &FbdObject) -> impl Iterator<Item = &Pin> {
        block.input_pins().iter().chain(block.inout_pins())
    }

    // The captured temporary (stateless) or instance member (stateful).
    pub(super) fn read_target(block: &FbdObject, pin: &Pin, index: &Index) -> String {
        match is_stateless(block, index) {
            true => temp_name(block, pin),
            false => format!("{}.{}", instance(block).unwrap_or_default(), pin.parameter_name),
        }
    }

    // The output's declared type; `None` for a pin the callee doesn't declare.
    // Outputs pass by reference, so unwrap the auto-deref pointer's inner type.
    pub(super) fn temp_type(block: &FbdObject, pin: &Pin, index: &Index) -> Option<String> {
        let function = block.type_name().unwrap_or_default();
        let variable = match is_return_pin(block, pin) {
            true => index.find_return_variable(function),
            false => index.find_member(function, &pin.parameter_name),
        }?;

        let information = index.get_type_information_or_void(variable.get_type_name());
        Some(information.get_inner_name().to_string())
    }

    // The call operator: `inst`, or `inst.act` for an action.
    pub(super) fn call_target(block: &FbdObject) -> String {
        let instance = instance(block).unwrap_or_default();
        match action(block) {
            Some(action) => format!("{instance}.{action}"),
            None => instance.to_string(),
        }
    }

    // What outputs are read through: the declared instance, or the owner POU.
    fn instance(block: &FbdObject) -> Option<&str> {
        block.instance_name.as_deref().or_else(|| owner(block))
    }

    // Everything before an action suffix (`MyFb.act` -> `MyFb`).
    fn owner(block: &FbdObject) -> Option<&str> {
        block.type_name().map(|name| name.rsplit_once('.').map_or(name, |(owner, _)| owner))
    }

    // The suffix of a qualified `typeName` (`owner.action`), if any.
    fn action(block: &FbdObject) -> Option<&str> {
        block.type_name().and_then(|name| name.rsplit_once('.').map(|(_, action)| action))
    }
}

// Which block output pins some consumer actually reads.
fn consumed(network: &model::Network, survey: &Survey) -> HashSet<usize> {
    let mut consumed = HashSet::new();
    let mut record = |wire: Option<usize>| {
        if let Trace::Reached(Source::Output { pin, .. }) = trace(wire, survey) {
            if let Some(id) = pin.output_pin() {
                consumed.insert(id);
            }
        }
    };

    for object in network.elements() {
        match Role::from(object) {
            Role::Sink | Role::Return | Role::Jump => record(consumes(object)),
            Role::Block => {
                for pin in block::inputs(object) {
                    record(pin.source_pin());
                }
            }
            _ => {}
        }
    }

    consumed
}

fn consumes(consumer: &FbdObject) -> Option<usize> {
    consumer
        .connection_in
        .as_ref()
        .and_then(|it| it.connections.first())
        .map(|connection| connection.ref_out_id)
}

#[cfg(test)]
mod tests {
    use plc_ast::provider::IdProvider;
    use plc_ast::ser::AstSerializer;

    use crate::model::Pou;
    use crate::network::{Argument, Network, Statement};
    use crate::test_utils::fixture_source;

    fn resolve_project(fixture: &str) -> String {
        let source = fixture_source(fixture);
        let pou = Pou::parse(&source.source).unwrap();
        let ids = IdProvider::default();

        let (interface, _) = crate::st::parse_interface(&pou, &source, ids.clone());
        let index = crate::test_utils::fixture_index(fixture, &interface);

        let (network, _) = super::Resolver::new(ids, &source, &index).resolve(pou.content().network());
        render(&network)
    }

    fn render(network: &Network) -> String {
        network
            .statements
            .iter()
            .map(|statement| match statement {
                Statement::Assignment { sink, source } => {
                    format!("{} := {}", AstSerializer::format(sink), AstSerializer::format(source))
                }
                Statement::Return { condition, .. } => {
                    format!("RETURN {}", AstSerializer::format(condition))
                }
                Statement::Jump { condition: Some(condition), target, .. } => {
                    format!("JMP {target} IF {}", AstSerializer::format(condition))
                }
                Statement::Jump { condition: None, target, .. } => {
                    format!("JMP {target} <disconnected>")
                }
                Statement::Label { name, .. } => format!("LABEL {name}"),
                Statement::Call { target, arguments, capture, .. } => {
                    let arguments = arguments
                        .iter()
                        .map(|argument| match argument {
                            Argument::Input { parameter, value } => {
                                format!("{parameter} := {}", AstSerializer::format(value))
                            }
                            Argument::Output { parameter, capture: Some(capture) } => {
                                format!("{parameter} => {capture}")
                            }
                            Argument::Output { parameter, capture: None } => format!("{parameter} => "),
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    match capture {
                        Some(capture) => format!("{capture} := {target}({arguments})"),
                        None => format!("{target}({arguments})"),
                    }
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    mod variables {
        use super::resolve_project;
        use crate::test_utils::diagnostics;

        #[test]
        fn simple_assignment() {
            insta::assert_snapshot!(resolve_project("variables/valid/simple_assignment"), @"bar := foo");
        }

        #[test]
        fn reciprocal_assignment() {
            insta::assert_snapshot!(resolve_project("variables/valid/reciprocal_assignment"), @r"
            bar := foo
            foo := bar");
        }

        #[test]
        fn fan_out() {
            insta::assert_snapshot!(resolve_project("variables/valid/fan_out"), @r"
            bar := foo
            baz := foo");
        }

        #[test]
        fn literal_assignment() {
            insta::assert_snapshot!(resolve_project("variables/valid/literal_assignment"), @"foo := 5");
        }

        #[test]
        fn indexed_assignment() {
            insta::assert_snapshot!(resolve_project("variables/valid/indexed_assignment"), @"values[1] := source");
        }

        #[test]
        fn unconnected_variables() {
            insta::assert_snapshot!(resolve_project("variables/valid/unconnected_variables"), @"bar := foo");
            insta::assert_snapshot!(diagnostics("variables/valid/unconnected_variables"), @r"
            warning[E084]: Element `foo` is unconnected and will be ignored
             = unconnected_variables.cfc: Block 1

            warning[E084]: Element `bar` is unconnected and will be ignored
             = unconnected_variables.cfc: Block 4
            ");
        }
    }

    mod connectors {
        use super::resolve_project;
        use crate::test_utils::diagnostics;

        #[test]
        fn assignment() {
            insta::assert_snapshot!(resolve_project("connectors/valid/assignment"), @"bar := foo");
        }

        #[test]
        fn fan_out() {
            insta::assert_snapshot!(resolve_project("connectors/valid/fan_out"), @r"
            bar := foo
            baz := foo");
        }

        #[test]
        fn chain() {
            insta::assert_snapshot!(resolve_project("connectors/valid/chain"), @"bar := foo");
        }

        #[test]
        fn unused_is_quiet() {
            insta::assert_snapshot!(diagnostics("connectors/valid/unused"), @"");
        }
    }

    mod returns {
        use super::resolve_project;

        #[test]
        fn conditional_return() {
            insta::assert_snapshot!(resolve_project("returns/valid/conditional_return"), @"RETURN myCondition");
        }

        #[test]
        fn negated_return() {
            insta::assert_snapshot!(resolve_project("returns/valid/negated_return"), @"RETURN NOT myCondition");
        }
    }

    mod jumps {
        use super::resolve_project;
        use crate::test_utils::diagnostics;

        #[test]
        fn conditional_jump() {
            insta::assert_snapshot!(resolve_project("jumps/valid/conditional_jump"), @r"
            JMP skipAssignment IF myCondition
            y := x
            LABEL skipAssignment");
        }

        #[test]
        fn negated_jump() {
            insta::assert_snapshot!(resolve_project("jumps/valid/negated_jump"), @r"
            JMP skipAssignment IF NOT myCondition
            y := x
            LABEL skipAssignment");
        }

        #[test]
        fn disconnected_jump() {
            insta::assert_snapshot!(resolve_project("jumps/valid/disconnected_jump"), @r"
            JMP skipAssignment <disconnected>
            y := x
            LABEL skipAssignment");
            insta::assert_snapshot!(diagnostics("jumps/valid/disconnected_jump"), @r"
            warning[E145]: Jump element is not connected to a condition and can never be taken
             = disconnected_jump.cfc: Block 1
            ");
        }

        #[test]
        fn unused_label() {
            insta::assert_snapshot!(resolve_project("jumps/valid/unused_label"), @r"
            y := x
            LABEL orphan");
            insta::assert_snapshot!(diagnostics("jumps/valid/unused_label"), @r"
            warning[E143]: Label `orphan` is not referenced by any jump
             = unused_label.cfc: Block 4
            ");
        }

        #[test]
        fn scrambled() {
            insta::assert_snapshot!(resolve_project("jumps/valid/scrambled"), @r"
            JMP mid IF g1
            a := x
            JMP end IF g2
            LABEL mid
            b := x
            JMP end IF g3
            c := x
            LABEL end");
        }

        #[test]
        fn backward_jump() {
            insta::assert_snapshot!(resolve_project("jumps/valid/backward_jump"), @r"
            LABEL top
            x := i
            JMP top IF cond");
        }
    }

    mod blocks {
        use super::resolve_project;

        #[test]
        fn block_route() {
            insta::assert_snapshot!(resolve_project("connectors/valid/block_route"), @r"
            counter(in := seed)
            result := counter.out");
        }

        #[test]
        fn function_void() {
            insta::assert_snapshot!(resolve_project("blocks/valid/function_void"), @r"
            myVoid(in := localIn, out => __out_out_3)
            localOut := __out_out_3");
        }

        #[test]
        fn program_call() {
            insta::assert_snapshot!(resolve_project("blocks/valid/program_call"), @r"
            counter(in := countIn)
            countOut := counter.out");
        }

        #[test]
        fn program_chain() {
            insta::assert_snapshot!(resolve_project("blocks/valid/program_chain"), @r"
            counter(in := seed)
            doubler(in := counter.out)
            result := doubler.out");
        }

        #[test]
        fn program_feedback() {
            insta::assert_snapshot!(resolve_project("blocks/valid/program_feedback"), @"counter(in := counter.out)");
        }

        #[test]
        fn program_negated() {
            insta::assert_snapshot!(resolve_project("blocks/valid/program_negated"), @r"
            program_0(in := NOT localIn, inout := NOT localInOut)
            localOut := NOT program_0.out");
        }

        #[test]
        fn fb_call() {
            insta::assert_snapshot!(resolve_project("blocks/valid/fb_call"), @r"
            inst(in := localIn)
            localOut := inst.out");
        }

        #[test]
        fn fb_instances() {
            insta::assert_snapshot!(resolve_project("blocks/valid/fb_instances"), @r"
            a(in := seed)
            b(in := a.out)
            result := b.out");
        }

        #[test]
        fn action_fb() {
            insta::assert_snapshot!(resolve_project("blocks/valid/action_fb"), @r"
            inst.increment(in := localIn)
            localOut := inst.out");
        }

        #[test]
        fn action_program() {
            insta::assert_snapshot!(resolve_project("blocks/valid/action_program"), @r"
            P.bump(step := localIn)
            localOut := P.out");
        }
    }

    mod validations {
        use super::resolve_project;
        use crate::test_utils::{diagnostics, transpile_project};

        #[test]
        fn call_expression() {
            insta::assert_snapshot!(diagnostics("variables/invalid/call_expression"), @r"
            error[E083]: Unsupported CFC expression: `MAX(foo, bar)`
             = call_expression.cfc: Block 1
            ");
        }

        #[test]
        fn binary_expression() {
            insta::assert_snapshot!(diagnostics("variables/invalid/binary_expression"), @r"
            error[E083]: Unsupported CFC expression: `foo + 1`
             = binary_expression.cfc: Block 1
            ");
        }

        #[test]
        fn duplicate_connector() {
            insta::assert_snapshot!(transpile_project("connectors/invalid/duplicate_connector").unwrap_err(), @r"
            error[E081]: Connector `x` is already defined
             = duplicate_connector.cfc: Block 10
            ");
        }

        #[test]
        fn without_source() {
            insta::assert_snapshot!(transpile_project("connectors/invalid/without_source").unwrap_err(), @r"
            error[E086]: Connector `x` has no incoming connection
             = without_source.cfc: Block 5
            ");
        }

        #[test]
        fn dangling_continuation() {
            insta::assert_snapshot!(transpile_project("connectors/invalid/dangling_continuation").unwrap_err(), @r"
            error[E082]: Continuation `x` has no matching connector
             = dangling_continuation.cfc: Block 6
            ");
        }

        #[test]
        fn connector_cycle() {
            insta::assert_snapshot!(transpile_project("connectors/invalid/connector_cycle").unwrap_err(), @r"
            error[E082]: Continuation `x` has no matching connector
             = connector_cycle.cfc: Block 6
            ");
        }

        #[test]
        fn disconnected_return() {
            insta::assert_snapshot!(resolve_project("returns/invalid/disconnected_return"), @"RETURN myCondition");
            insta::assert_snapshot!(transpile_project("returns/invalid/disconnected_return").unwrap_err(), @r"
            error[E085]: Return element is not connected to a condition
             = disconnected_return.cfc: Block 4
            ");
        }

        #[test]
        fn undefined_jump_target() {
            insta::assert_snapshot!(transpile_project("jumps/invalid/undefined_jump_target").unwrap_err(), @r"
            error[E142]: Jump refers to undefined label `missing`
             = undefined_jump_target.cfc: Block 3
            ");
        }

        #[test]
        fn duplicate_label() {
            insta::assert_snapshot!(transpile_project("jumps/invalid/duplicate_label").unwrap_err(), @r"
            error[E144]: Label `dup` is already defined
             = duplicate_label.cfc: Block 5
            ");
        }

        #[test]
        fn unknown_type() {
            insta::assert_snapshot!(transpile_project("blocks/invalid/unknown_type").unwrap_err(), @r"
            error[E146]: Block `counter` refers to an undeclared POU
             = unknown_type.cfc: Block 3
            ");
        }

        #[test]
        fn function_stale_output() {
            insta::assert_snapshot!(transpile_project("blocks/invalid/function_stale_output").unwrap_err(), @r"
            error[E147]: Output `oldDoubled` is not declared by `myAdd`
             = function_stale_output.cfc: Block 5
            ");
        }
    }
}
