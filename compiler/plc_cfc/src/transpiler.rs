use ast::ast::{AstFactory, AstNode, CompilationUnit, LinkageType};
use ast::provider::IdProvider;
use plc::lexer::lex_with_ids;
use plc::parser::expressions_parser;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};
use plc_source::{SourceCode, SourceContainer};

use crate::model::{
    self, Block, BodyContent, CommonObject, ConnectionPointIn, DataSink, FbdNetwork, FbdObject, Network, Pou,
};
use crate::resolver::{PinSource, Resolver};

pub fn transpile(source: &SourceCode, id_provider: IdProvider) -> Result<CompilationUnit, Vec<Diagnostic>> {
    Transpiler::new(source, id_provider).transpile()
}

struct Transpiler<'cfc> {
    source: &'cfc SourceCode,
    ids: IdProvider,
    locations: SourceLocationFactory,
    /// Lowering problems; collected rather than returned so that one broken statement
    /// does not hide the problems of its siblings
    diagnostics: Vec<Diagnostic>,
}

impl<'cfc> Transpiler<'cfc> {
    fn new(source: &'cfc SourceCode, ids: IdProvider) -> Self {
        Transpiler {
            source,
            ids,
            locations: SourceLocationFactory::for_source(source),
            diagnostics: Vec::new(),
        }
    }

    fn transpile(mut self) -> Result<CompilationUnit, Vec<Diagnostic>> {
        // First deserialize the XML content
        let deserialized = model::from_str(&self.source.source).map_err(|error| {
            let location = self.locations.create_file_only_location();
            let message = format!("Invalid CFC document: {error}");

            vec![Diagnostic::new(message).with_location(location)]
        })?;

        // Then deserialize the header within the XML content...
        let mut unit = self.parse_header(&deserialized)?;

        // ...and lower the graphical body into the statements of the header's (empty) implementation
        let statements = self.parse_body(&deserialized)?;
        if !self.diagnostics.is_empty() {
            return Err(self.diagnostics);
        }

        unit.implementations.first_mut().expect("validated by parse_header").statements = statements;
        Ok(unit)
    }

    /// Parses the CFC header content, i.e. the ST content with its variable declarations
    fn parse_header(&self, pou: &Pou) -> Result<CompilationUnit, Vec<Diagnostic>> {
        // The text declaration is missing a `END_*` keyword. To parse successfully we need to append it
        let raw = pou.get_header_content().ok_or(vec![Diagnostic::new("expected CFC header content")])?;
        let kw = match pou {
            model::Pou::Program(_) => "END_PROGRAM",
            model::Pou::FunctionBlock(_) => "END_FUNCTION_BLOCK",
            model::Pou::Function(_) => "END_FUNCTION",
        };
        let content = format!("{raw}\n{kw}");

        // Parse the post-processed header
        let lexer = lex_with_ids(&content, self.ids.clone(), self.locations.clone());
        let (unit, diagnostics) =
            plc::parser::parse(lexer, LinkageType::Internal, self.source.get_location_str());

        // Return early
        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        // Check if invalid; note that the parser always yields an implementation for a POU, even a
        // declaration-only one — that empty implementation is where the body's statements will land
        if unit.implementations.len() != 1 {
            return Err(vec![Diagnostic::new("CFC header must declare exactly one POU")]);
        }
        if !unit.implementations[0].statements.is_empty() {
            return Err(vec![Diagnostic::new("CFC header must not contain any statements")]);
        }

        Ok(unit)
    }

    /// Parses the actual CFC body, i.e. the XML content
    fn parse_body(&mut self, pou: &Pou) -> Result<Vec<AstNode>, Vec<Diagnostic>> {
        let network = self.single_network(pou)?;
        if network.common_objects.iter().any(|object| !matches!(object, CommonObject::Comment(_))) {
            unimplemented!("connector and continuation objects");
        }

        let resolver = Resolver::index(network);

        let mut statements = Vec::new();
        for object in &network.objects {
            match object {
                // foo := <expression of the connected producer>
                FbdObject::DataSink(sink) => statements.extend(self.lower_sink(sink, &resolver)),

                // A call nobody consumes is executed for its side effects
                FbdObject::Block(block) if is_unconsumed(block, &resolver) => {
                    statements.extend(self.lower_call(block, &resolver));
                }

                // Sources and consumed blocks are pulled in on demand while lowering their consumers
                FbdObject::DataSource(_) | FbdObject::Block(_) => (),

                FbdObject::Jump(_) | FbdObject::Return(_) | FbdObject::Unconnected(_) => {
                    unimplemented!("jump, return and unconnected objects")
                }
            }
        }

        Ok(statements)
    }

    /// CFC bodies consist of exactly one FBD network — a single free-form canvas
    fn single_network<'pou>(&self, pou: &'pou Pou) -> Result<&'pou FbdNetwork, Vec<Diagnostic>> {
        let body = pou.main_body().ok_or(vec![Diagnostic::new("expected a CFC body")])?;
        let [BodyContent::Fbd(fbd)] = body.body_content.as_slice() else {
            return Err(vec![Diagnostic::new("expected exactly one FBD body")]);
        };
        let [Network::Fbd(network)] = fbd.networks.as_slice() else {
            return Err(vec![Diagnostic::new("expected exactly one network")]);
        };

        Ok(network)
    }

    /// `foo := <expression of the connected producer>`
    fn lower_sink(&mut self, sink: &DataSink, resolver: &Resolver) -> Option<AstNode> {
        let source = self.resolve_or_diagnose(
            sink.connection_point_in.as_ref(),
            resolver,
            &sink.identifier,
            sink.global_id,
        )?;
        let value = self.lower_source(source, resolver)?;
        let variable = self.parse_expression(&sink.identifier, sink.global_id);

        Some(AstFactory::create_assignment(variable, value, self.ids.next_id()))
    }

    /// The expression a resolved pin produces: a data source's identifier as-is, or the
    /// call producing the value — inlined into its consumer
    fn lower_source(&mut self, source: PinSource, resolver: &Resolver) -> Option<AstNode> {
        match source {
            PinSource::Data(data) => Some(self.parse_expression(&data.identifier, data.global_id)),
            PinSource::BlockOutput { block, .. } => self.lower_call(block, resolver),
        }
    }

    /// `myAdd(x := <expr>, y := <expr>)` — arguments are named to stay independent of
    /// the declaration's parameter order
    fn lower_call(&mut self, block: &Block, resolver: &Resolver) -> Option<AstNode> {
        let mut arguments = Vec::new();
        for pin in block.input_variables.iter().flat_map(|inputs| &inputs.variables) {
            let subject = format!("{}.{}", block.type_name, pin.parameter_name);
            let source = self.resolve_or_diagnose(
                pin.connection_point_in.as_ref(),
                resolver,
                &subject,
                block.global_id,
            )?;
            let value = self.lower_source(source, resolver)?;

            let location = self.block_location(block.global_id);
            let name = AstFactory::create_identifier(&pin.parameter_name, location, self.ids.next_id());
            arguments.push(AstFactory::create_assignment(name, value, self.ids.next_id()));
        }

        Some(AstFactory::create_call_to(
            block.type_name.clone(),
            arguments,
            self.ids.next_id(),
            self.ids.next_id(),
            &self.block_location(block.global_id),
        ))
    }

    /// Resolves an input pin, pushing a diagnostic for unconnected pins
    fn resolve_or_diagnose<'net>(
        &mut self,
        pin: Option<&ConnectionPointIn>,
        resolver: &Resolver<'net>,
        subject: &str,
        global_id: Option<u64>,
    ) -> Option<PinSource<'net>> {
        // Checked first because `resolve_input` only considers plain connections — a
        // feedback-wired pin must not be misreported as unconnected
        if pin.is_some_and(|pin| !pin.feedback_connections.is_empty()) {
            unimplemented!("feedback connections");
        }

        match resolver.resolve_input(pin) {
            Some(source) => Some(*source),
            None => {
                let message = format!("`{subject}` is unconnected");
                self.diagnostics.push(Diagnostic::new(message).with_location(self.block_location(global_id)));
                None
            }
        }
    }

    /// Parses a data source/sink identifier — a variable, literal or qualified
    /// reference — into an expression located at the carrying element
    fn parse_expression(&mut self, text: &str, global_id: Option<u64>) -> AstNode {
        let mut lexer = lex_with_ids(text, self.ids.clone(), self.locations.clone());
        let mut expression = expressions_parser::parse_expression(&mut lexer);
        expression.set_location(self.block_location(global_id));

        expression
    }

    /// Synthetic location of a graphical element, keyed on its `globalId`
    fn block_location(&self, global_id: Option<u64>) -> SourceLocation {
        debug_assert!(global_id.is_some(), "the IDE guarantees a globalId on every object");
        self.locations.create_block_location(global_id.unwrap_or_default() as usize, None)
    }
}

/// A block none of whose outputs are consumed; it is executed for its side effects only
fn is_unconsumed(block: &Block, resolver: &Resolver) -> bool {
    block
        .output_variables
        .iter()
        .flat_map(|outputs| &outputs.variables)
        .filter_map(|pin| pin.connection_point_out.as_ref())
        .all(|out| resolver.use_count(out.id) == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::ser::AstSerializer;

    const SIMPLE_FUNCTION_CALL: &str = include_str!("../fixtures/simple_function_call/myMain.cfc");

    #[test]
    fn transpiles_the_simple_function_call_fixture() {
        let source = SourceCode::new(SIMPLE_FUNCTION_CALL, "myMain.cfc");
        let unit = transpile(&source, IdProvider::default()).unwrap();

        insta::assert_snapshot!(AstSerializer::format_unit(&unit), @r"
        PROGRAM myMain
        VAR
            localA : INT := 10;
            localB : INT := 20;
            localResult : INT := 0;
        END_VAR
            localResult := myAdd(x := localA, y := localB);
        END_PROGRAM
        ");
    }
}
