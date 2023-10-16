
use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::str::FromStr;

use lsp_types::notification::{PublishDiagnostics, Notification, DidChangeTextDocument, DidSaveTextDocument};
use lsp_types::{OneOf, DiagnosticServerCapabilities, PublishDiagnosticsParams, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentContentChangeEvent, Location, Range, Url, Position};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use plc_diagnostics::reporter::ResolvedDiagnostics;
use plc_driver::cli::CompileParameters;
use plc_driver::pipelines::AnnotatedProject;
use plc_source::source_location::{CodeSpan, TextLocation};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(Default::default())),
        text_document_sync : Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    eprintln!("starting example main loop");
    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                eprintln!("got request: {req:?}");
                 match cast::<GotoDefinition>(req) {
                    Ok((id, params)) => {
                        eprintln!("got gotoDefinition request #{id}: {params:?}");
                        
                        if let Ok(project) = plc_driver::parse_and_annotate("plc", vec![params.text_document_position_params.text_document.uri.path()]) {
                            //Element under cursor
                            if let Some(location) = project.annotations.get_declaration_location(&project.index,
                            params.text_document_position_params.position.line,
                            params.text_document_position_params.position.character) {
                                let start = Position::new(location.get_line() as u32, location.get_column() as u32);
                                let range = Range::new(start, start);
                                let url = location.get_file_name().map(|it| Url::from_str(it)).unwrap().unwrap();
                            let result = Some(GotoDefinitionResponse::Scalar(Location::new(url, range)));
                            let result = serde_json::to_value(&result).unwrap();
                            let resp = Response { id, result: Some(result), error: None };
                            connection.sender.send(Message::Response(resp))?;

                            } else {

                            }

                        } else {
                            let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                            let result = serde_json::to_value(&result).unwrap();
                            let resp = Response { id, result: Some(result), error: None };
                            connection.sender.send(Message::Response(resp))?;
                        };
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                eprintln!("got notification: {not:?}");
                match cast_notification::<DidSaveTextDocument>(not) {
                    Ok(param) => {
                        let diagnostics : Rc<RefCell<Vec<ResolvedDiagnostics>>> = Default::default();
                        let reporter = LspDiagnostisReporter {
                            diagnostics: diagnostics.clone()
                        };
                        let diagnostician = plc_diagnostics::diagnostician::Diagnostician {
                            reporter: Box::new(reporter),
                            ..Default::default()
                        };
                        connection.sender.send(Message::Notification(lsp_server::Notification::new(PublishDiagnostics::METHOD.to_string(), PublishDiagnosticsParams {
                                        uri: param.text_document.uri.clone(),
                                        diagnostics: vec![],
                                        version: None, //Some(param.text_document.version),
                                    }))).unwrap();
                        let compiler_params = CompileParameters::parse(&["--check", param.text_document.uri.path()]).unwrap();
                        let _ = plc_driver::compile_with_diagnostician(compiler_params, diagnostician);
                        let diagnostics = diagnostics.as_ref().borrow().iter().map(|diag| {
                            if let CodeSpan::Range(text_location) = &diag.main_location.span {
                                let start = lsp_types::Position::new(text_location.start.line as u32, text_location.start.column as u32);
                                let end = lsp_types::Position::new(text_location.end.line as u32, text_location.start.column as u32);
                                let range = lsp_types::Range::new(start, end);
                                lsp_types::Diagnostic::new_simple(range, diag.message.to_string())
                            } else {
                                lsp_types::Diagnostic::new_simple(lsp_types::Range { start: lsp_types::Position { line: 0, character: 0 }, end: lsp_types::Position { line: 0, character: 0 } }, diag.message.clone())
                            }

                        }).collect::<Vec<_>>();
                        connection.sender.send(Message::Notification(lsp_server::Notification::new(PublishDiagnostics::METHOD.to_string(), PublishDiagnosticsParams {
                                        uri: param.text_document.uri,
                                        diagnostics,
                                        version: None,
                                    }))).unwrap();
                    },
                    Err(_) => {}
                }
                
            }
        }
    }
    eprintln!("Done");
    Ok(())
}

struct LspDiagnostisReporter {
   diagnostics: Rc<RefCell<Vec<ResolvedDiagnostics>>>,
}

impl plc_diagnostics::reporter::DiagnosticReporter for LspDiagnostisReporter {
    fn report(&mut self, diagnostics: &[plc_diagnostics::reporter::ResolvedDiagnostics]) {
        self.diagnostics.borrow_mut().extend_from_slice(diagnostics)
        // self.connection.sender.send(Message::Notification(lsp_server::Notification::new(PublishDiagnostics::METHOD.to_string(), PublishDiagnosticsParams {
        //             uri: todo!(),
        //             diagnostics: todo!(),
        //             version: todo!(),
        //         }))).unwrap();
    }

    fn register(&mut self, path: String, src: String) -> usize {
        0
    }
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}


fn cast_notification<N>(not : lsp_server::Notification) -> Result<N::Params, ExtractError<lsp_server::Notification>> 
where 
    N : Notification,
    N::Params: serde::de::DeserializeOwned {
    not.extract(N::METHOD)
}
