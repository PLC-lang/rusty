use std::collections::HashMap;

use ast::{
    ast::{AstId, AstNode, CompilationUnit, Implementation, LinkageType, PouType as AstPouType},
    provider::IdProvider,
};
use plc::{lexer, parser::expressions_parser::parse_expression};
use plc_diagnostics::{
    diagnostician::Diagnostician,
    diagnostics::{Diagnostic, Severity},
};

use ast::lib_sourcelocation::{SourceCode, SourceContainer};
use ast::source_location::{SourceLocation, SourceLocationFactory};
use quick_xml::events::{attributes::Attributes, BytesStart, Event};

use crate::{
    error::Error,
    extensions::TryToString,
    model::{
        pou::{Pou, PouType},
        project::Project,
    },
    reader::Reader,
};

mod action;
mod block;
mod control;
mod fbd;
mod pou;
#[cfg(test)]
mod tests;
mod variables;

pub(crate) fn get_attributes(attributes: Attributes) -> Result<HashMap<String, String>, Error> {
    attributes
        .flatten()
        .map(|it| Ok((it.key.try_to_string()?, it.value.try_to_string()?)))
        .collect::<Result<HashMap<_, _>, Error>>()
}

pub(crate) trait Parseable
where
    Self: Sized,
{
    fn visit(reader: &mut Reader, tag: Option<BytesStart>) -> Result<Self, Error>;
}

pub(crate) fn visit(content: &str) -> Result<Project, Error> {
    let mut reader = Reader::new(content);
    reader.trim_text(true).expand_empty_elements(true);
    let mut project = Project::default();

    loop {
        match reader.read_event()? {
            Event::Start(tag) if tag.name().as_ref() == b"pou" => {
                project.pous.push(Pou::visit(&mut reader, Some(tag))?)
            }
            Event::Start(tag) if tag.name().as_ref() == b"project" || tag.name().as_ref() == b"pous" => {
                todo!("Project support comming in #977")
            }
            Event::End(tag) if tag.name().as_ref() == b"project" || tag.name().as_ref() == b"pous" => break,
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(project)
}

pub fn parse_file(
    source: &SourceCode,
    linkage: LinkageType,
    id_provider: IdProvider,
    diagnostician: &mut Diagnostician,
) -> Result<CompilationUnit, Diagnostic> {
    let (unit, errors) = parse(source, linkage, id_provider);
    //Register the source file with the diagnostician
    diagnostician.register_file(source.get_location_str().to_string(), source.source.clone()); // TODO: Remove clone here, generally passing the GlobalContext instead of the actual source here or in the handle method should be sufficient
    if diagnostician.handle(&errors) == Severity::Error {
        Err(Diagnostic::new("Compilation aborted due to parse errors"))
    } else {
        Ok(unit)
    }
}

fn parse(
    source: &SourceCode,
    linkage: LinkageType,
    id_provider: IdProvider,
) -> (CompilationUnit, Vec<Diagnostic>) {
    let source_location_factory = SourceLocationFactory::for_source(source);
    // Transform the xml file to a data model.
    // XXX: consecutive call-statements are nested in a single ast-statement. this will be broken up with temporary variables in the future
    let mut project = match visit(&source.source) {
        Ok(project) => project,
        Err(why) => todo!("cfc errors need to be transformed into diagnostics; {why:?}"),
    };

    let mut diagnostics = vec![];
    let _ = project.desugar(&source_location_factory).map_err(|e| diagnostics.extend(e));

    // Create a new parse session
    let parser =
        ParseSession::new(&project, source.get_location_str(), id_provider, linkage, source_location_factory);
    // Parse the declaration data field
    let Some((unit, declaration_diagnostics)) = parser.try_parse_declaration() else {
        unimplemented!("XML schemas without text declarations are not yet supported")
    };
    diagnostics.extend(declaration_diagnostics);

    // Transform the data-model into an AST
    let (implementations, parser_diagnostics) = parser.parse_model();
    diagnostics.extend(parser_diagnostics);

    (unit.with_implementations(implementations), diagnostics)
}

pub(crate) struct ParseSession<'parse, 'xml> {
    project: &'parse Project<'xml>,
    id_provider: IdProvider,
    linkage: LinkageType,
    file_name: &'static str,
    range_factory: SourceLocationFactory,
    diagnostics: Vec<Diagnostic>,
}

impl<'parse, 'xml> ParseSession<'parse, 'xml> {
    fn new(
        project: &'parse Project<'xml>,
        file_name: &'static str,
        id_provider: IdProvider,
        linkage: LinkageType,
        range_factory: SourceLocationFactory,
    ) -> Self {
        ParseSession { project, id_provider, linkage, file_name, range_factory, diagnostics: Vec::new() }
    }

    /// parse the compilation unit from the addData field
    fn try_parse_declaration(&self) -> Option<(CompilationUnit, Vec<Diagnostic>)> {
        let content = self
            .project
            .pous
            .first()
            .and_then(|it| it.interface.as_ref().and_then(|it| it.get_data_content()))?;

        //TODO: if our ST parser returns a diagnostic here, we might not have a text declaration and need to rely on the XML to provide us with
        // the necessary data. for now, we will assume to always have a text declaration
        Some(plc::parser::parse(
            lexer::lex_with_ids(content, self.id_provider.clone(), self.range_factory.clone()),
            self.linkage,
            self.file_name,
        ))
    }

    fn parse_expression(&self, expr: &str, local_id: usize, execution_order: Option<usize>) -> AstNode {
        let mut exp = parse_expression(&mut lexer::lex_with_ids(
            html_escape::decode_html_entities_to_string(expr, &mut String::new()),
            self.id_provider.clone(),
            self.range_factory.clone(),
        ));
        let loc = exp.get_location();
        exp.set_location(self.range_factory.create_block_location(local_id, execution_order).span(&loc));
        exp
    }

    fn parse_model(mut self) -> (Vec<Implementation>, Vec<Diagnostic>) {
        let mut implementations = vec![];
        for pou in &self.project.pous {
            // transform body
            implementations.push(pou.build_implementation(&mut self));
            // transform actions
            pou.actions
                .iter()
                .for_each(|action| implementations.push(action.build_implementation(&mut self)));
        }

        (implementations, self.diagnostics)
    }

    fn next_id(&self) -> AstId {
        self.id_provider.clone().next_id()
    }

    fn create_range(&self, range: core::ops::Range<usize>) -> SourceLocation {
        self.range_factory.create_range(range)
    }

    fn create_block_location(&self, local_id: usize, execution_order: Option<usize>) -> SourceLocation {
        self.range_factory.create_block_location(local_id, execution_order)
    }

    fn create_file_only_location(&self) -> SourceLocation {
        self.range_factory.create_file_only_location()
    }
}

impl From<PouType> for AstPouType {
    fn from(value: PouType) -> Self {
        match value {
            PouType::Program => AstPouType::Program,
            PouType::Function => AstPouType::Function,
            PouType::FunctionBlock => AstPouType::FunctionBlock,
        }
    }
}
