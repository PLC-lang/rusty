use ast::{
    ast::{AstId, AstStatement, CompilationUnit, Implementation, LinkageType, PouType as AstPouType},
    provider::IdProvider,
};
use plc::{lexer, parser::expressions_parser::parse_expression};
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};

use plc_source::{
    source_location::{SourceLocation, SourceLocationFactory},
    SourceCode, SourceContainer,
};
use quick_xml::events::Event;

use crate::{
    error::Error,
    model::{pou::PouType, project::Project},
    reader::PeekableReader,
};

mod action;
mod block;
mod fbd;
mod pou;
#[cfg(test)]
mod tests;
mod variables;

pub(crate) trait Parseable {
    type Item;
    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error>;
}

pub(crate) fn visit(content: &str) -> Result<Project, Error> {
    let mut reader = PeekableReader::new(content);
    loop {
        match reader.peek()? {
            Event::Start(tag) if tag.name().as_ref() == b"pou" => return Project::pou_entry(&mut reader),
            Event::Start(tag) if tag.name().as_ref() == b"project" => return Project::visit(&mut reader),
            Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"pou"])),
            _ => reader.consume()?,
        }
    }
}

pub fn parse_file(
    source: SourceCode,
    linkage: LinkageType,
    id_provider: IdProvider,
    diagnostician: &mut Diagnostician,
) -> CompilationUnit {
    let (unit, errors) = parse(&source, linkage, id_provider);
    //Register the source file with the diagnostician
    diagnostician.register_file(source.get_location_str().to_string(), source.source.to_string());
    diagnostician.handle(errors);
    unit
}

fn parse(
    source: &SourceCode,
    linkage: LinkageType,
    id_provider: IdProvider,
) -> (CompilationUnit, Vec<Diagnostic>) {
    // transform the xml file to a data model.
    // XXX: consecutive call-statements are nested in a single ast-statement. this will be broken up with temporary variables in the future
    let Ok(project) = visit(&source.source) else {
        todo!("cfc errors need to be transformed into diagnostics")
    };

    // create a new parse session
    let source_location_factory = SourceLocationFactory::for_source(source);
    let parser =
        ParseSession::new(&project, source.get_location_str(), id_provider, linkage, source_location_factory);

    // try to parse a declaration data field
    let Some((unit, diagnostics)) = parser.try_parse_declaration() else {
        unimplemented!("XML schemas without text declarations are not yet supported")
    };

    // transform the data model into rusty AST statements and add them to the compilation unit
    (unit.with_implementations(parser.parse_model()), diagnostics)
}

pub(crate) struct ParseSession<'parse> {
    project: &'parse Project,
    id_provider: IdProvider,
    linkage: LinkageType,
    file_name: &'static str,
    range_factory: SourceLocationFactory,
}

impl<'parse> ParseSession<'parse> {
    fn new(
        project: &'parse Project,
        file_name: &'static str,
        id_provider: IdProvider,
        linkage: LinkageType,
        range_factory: SourceLocationFactory,
    ) -> Self {
        ParseSession { project, id_provider, linkage, file_name, range_factory }
    }

    /// parse the compilation unit from the addData field
    fn try_parse_declaration(&self) -> Option<(CompilationUnit, Vec<Diagnostic>)> {
        let Some(content) = self
            .project
            .pous
            .first()
            .and_then(|it| it.interface.as_ref().and_then(|it| it.get_data_content()))
        else {
            return None;
        };

        //TODO: if our ST parser returns a diagnostic here, we might not have a text declaration and need to rely on the XML to provide us with
        // the necessary data. for now, we will assume to always have a text declaration
        Some(plc::parser::parse(
            lexer::lex_with_ids(content, self.id_provider.clone(), self.range_factory.clone()),
            self.linkage,
            self.file_name,
        ))
    }

    fn parse_expression(&self, expr: &str, local_id: usize, execution_order: Option<usize>) -> AstStatement {
        let exp = parse_expression(&mut lexer::lex_with_ids(
            html_escape::decode_html_entities_to_string(expr, &mut String::new()),
            self.id_provider.clone(),
            self.range_factory.clone(),
        ));
        let loc = exp.get_location();
        exp.set_location(self.range_factory.create_block_location(local_id, execution_order).span(&loc))
    }

    fn parse_model(&self) -> Vec<Implementation> {
        let mut implementations = vec![];
        for pou in &self.project.pous {
            // transform body
            implementations.push(pou.build_implementation(self));
            // transform actions
            pou.actions.iter().for_each(|action| implementations.push(action.build_implementation(self)));
        }
        implementations
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
