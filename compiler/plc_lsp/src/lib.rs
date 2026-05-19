use anyhow::Context as _;
use lsp_server::{Connection, IoThreads, Message, Notification, Request, Response, ResponseError};
use lsp_types::{
    ClientCapabilities, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    InitializeParams, InitializeResult, PositionEncodingKind, ServerCapabilities, ServerInfo,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};

pub mod document;

pub struct ServerState {
    /// Encoding negotiated during the initialize handshake. Phases that
    /// exchange positions on the wire (diagnostics, hover, …) read this to
    /// decide how to convert their byte offsets.
    pub position_encoding: PositionEncodingKind,
    /// In-memory view of every editor-open buffer.
    pub documents: document::DocumentStore,
}

impl ServerState {
    pub fn new(position_encoding: PositionEncodingKind) -> Self {
        ServerState { position_encoding, documents: document::DocumentStore::new() }
    }
}

/// Pick the position encoding from what the client offers. We prefer UTF-8
/// (no byte-offset conversion) but fall back to UTF-16 when the client
/// doesn't advertise utf-8 — notably `vscode-languageclient` v9 only ever
/// advertises utf-16, and per LSP the server must pick from the client's
/// offered list.
fn pick_position_encoding(client_capabilities: &ClientCapabilities) -> PositionEncodingKind {
    let offered = client_capabilities.general.as_ref().and_then(|g| g.position_encodings.as_ref());
    match offered {
        Some(encodings) if encodings.contains(&PositionEncodingKind::UTF8) => PositionEncodingKind::UTF8,
        _ => PositionEncodingKind::UTF16,
    }
}

pub fn run() -> anyhow::Result<()> {
    let (connection, io_threads) = Connection::stdio();
    let result = serve(&connection);
    finalize(result, io_threads)
}

pub fn serve(connection: &Connection) -> anyhow::Result<()> {
    log::info!("plc-lsp starting; performing initialize handshake");

    let (init_id, init_params_val) = connection.initialize_start().context(
        "LSP initialize handshake failed waiting for the client's initialize request \
         (LSP messages must be framed as 'Content-Length: N\\r\\n\\r\\n<json>')",
    )?;
    let init_params: InitializeParams = serde_json::from_value(init_params_val)
        .context("client's initialize params did not match the LSP InitializeParams schema")?;

    let position_encoding = pick_position_encoding(&init_params.capabilities);
    log::info!("plc-lsp negotiated position encoding: {position_encoding:?}");

    let server_capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        position_encoding: Some(position_encoding.clone()),
        ..Default::default()
    };

    let init_result = InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(ServerInfo {
            name: "plc-lsp".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
    };
    connection
        .initialize_finish(init_id, serde_json::to_value(init_result)?)
        .context("sending initialize response and waiting for 'initialized' notification")?;

    log::info!("plc-lsp initialized; entering main loop");

    let mut state = ServerState::new(position_encoding);
    main_loop(connection, &mut state)?;

    log::info!("plc-lsp shutting down");
    Ok(())
}

/// Combine the server result with the I/O-thread result.
/// When the server errors with a "disconnected channel" style message, the real
/// cause usually sits in the I/O thread (e.g., a framing error on stdin). We
/// join the threads here and surface their error as additional context.
fn finalize(server_result: anyhow::Result<()>, io_threads: IoThreads) -> anyhow::Result<()> {
    match (server_result, io_threads.join()) {
        (Ok(()), Ok(())) => Ok(()),
        (Ok(()), Err(io_err)) => Err(anyhow::Error::new(io_err).context("I/O thread error")),
        (Err(e), Ok(())) => Err(e),
        (Err(e), Err(io_err)) => Err(e.context(format!("I/O thread also reported: {io_err}"))),
    }
}

fn main_loop(connection: &Connection, state: &mut ServerState) -> anyhow::Result<()> {
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                handle_request(state, connection, req);
            }
            Message::Notification(notif) => {
                handle_notification(state, notif)?;
            }
            Message::Response(_) => {
                log::debug!("unexpected response from client");
            }
        }
    }
    Ok(())
}

fn handle_request(_state: &mut ServerState, connection: &Connection, req: Request) {
    log::debug!("unhandled request: method={} id={:?}", req.method, req.id);
    let response = Response {
        id: req.id,
        result: None,
        error: Some(ResponseError {
            code: lsp_server::ErrorCode::MethodNotFound as i32,
            message: format!("method '{}' not implemented", req.method),
            data: None,
        }),
    };
    if let Err(e) = connection.sender.send(Message::Response(response)) {
        log::error!("failed to send response: {e}");
    }
}

fn handle_notification(state: &mut ServerState, notif: Notification) -> anyhow::Result<()> {
    match notif.method.as_str() {
        "exit" => anyhow::bail!("exit notification without prior shutdown"),
        "textDocument/didOpen" => handle_did_open(state, notif),
        "textDocument/didChange" => handle_did_change(state, notif),
        "textDocument/didSave" => handle_did_save(state, notif),
        "textDocument/didClose" => handle_did_close(state, notif),
        "$/setTrace" | "$/cancelRequest" => {
            log::debug!("{} ignored in phase 1", notif.method);
        }
        other => {
            log::debug!("unhandled notification: {other}");
        }
    }
    Ok(())
}

fn handle_did_open(state: &mut ServerState, notif: Notification) {
    let params: DidOpenTextDocumentParams = match serde_json::from_value(notif.params) {
        Ok(p) => p,
        Err(e) => {
            log::warn!("textDocument/didOpen: malformed params: {e}");
            return;
        }
    };
    let item = params.text_document;
    if !accept_language_id(&item.language_id) {
        log::debug!("ignoring didOpen: unsupported language_id={:?}", item.language_id);
        return;
    }
    state.documents.open(item.uri, item.language_id, item.version, item.text);
}

fn handle_did_change(state: &mut ServerState, notif: Notification) {
    let params: DidChangeTextDocumentParams = match serde_json::from_value(notif.params) {
        Ok(p) => p,
        Err(e) => {
            log::warn!("textDocument/didChange: malformed params: {e}");
            return;
        }
    };
    let id = params.text_document;

    let Some(change) = params.content_changes.into_iter().next() else {
        log::warn!("textDocument/didChange: no content_changes");
        return;
    };
    if change.range.is_some() {
        log::warn!("textDocument/didChange: incremental change received but server advertised Full sync");
        return;
    }

    state.documents.change(&id.uri, id.version, change.text);
}

fn handle_did_save(_state: &mut ServerState, _notif: Notification) {
    // Phase 2: no-op. Phase 3 turns this into the compile trigger.
}

fn handle_did_close(state: &mut ServerState, notif: Notification) {
    let params: DidCloseTextDocumentParams = match serde_json::from_value(notif.params) {
        Ok(p) => p,
        Err(e) => {
            log::warn!("textDocument/didClose: malformed params: {e}");
            return;
        }
    };
    state.documents.close(&params.text_document.uri);
}

fn accept_language_id(language_id: &str) -> bool {
    matches!(language_id, "structured-text" | "st" | "iecst")
}
