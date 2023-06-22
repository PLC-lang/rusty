use indexmap::IndexMap;
use plc::{
    ast::{
        AstId, AstStatement, CompilationUnit, Implementation, LinkageType, PouType as AstPouType,
        SourceRange, SourceRangeFactory,
    },
    diagnostics::{Diagnostic, Diagnostician},
    lexer::{self, IdProvider},
    parser::expressions_parser::parse_expression,
};

use crate::{
    deserializer::visit,
    model::{fbd::NodeId, pou::PouType, project::Project},
};

mod action;
mod block;
mod fbd;
mod pou;
mod tests;
mod variables;

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
    // transform the xml file to a data model
    let Ok(project) = visit(source).map(|proj| proj.with_temp_vars()) else {
        todo!("cfc errors need to be transformed into diagnostics")
    };

    // create a new parse session
    let mut parser = ParseSession::new(&project, location, id_provider, linkage);

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
    range_factory: SourceRangeFactory,
}

impl<'parse> ParseSession<'parse> {
    fn new(
        project: &'parse Project,
        file_name: &'static str,
        id_provider: IdProvider,
        linkage: LinkageType,
    ) -> Self {
        ParseSession {
            project,
            id_provider,
            linkage,
            file_name,
            range_factory: SourceRangeFactory::for_file(file_name),
        }
    }

    /// parse the compilation unit from the addData field
    fn try_parse_declaration(&self) -> Option<(CompilationUnit, Vec<Diagnostic>)> {
        let Some(content) = self.project.pous
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
            lexer::lex_with_ids(
                content,
                self.id_provider.clone(),
                SourceRangeFactory::for_file(self.file_name),
            ),
            self.linkage,
            self.file_name,
        ))
    }

    fn parse_expression(&self, expr: &str) -> AstStatement {
        parse_expression(&mut lexer::lex_with_ids(
            html_escape::decode_html_entities_to_string(expr, &mut String::new()),
            self.id_provider.clone(),
            SourceRangeFactory::for_file(self.file_name),
        ))
    }

    fn parse_model(&mut self) -> Vec<Implementation> {
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

    fn create_range(&self, range: core::ops::Range<usize>) -> SourceRange {
        self.range_factory.create_range(range)
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
