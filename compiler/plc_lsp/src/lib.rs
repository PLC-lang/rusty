use anyhow::Context as _;
use lsp_server::{Connection, IoThreads, Message, Notification, Request, Response, ResponseError};
use lsp_types::{
    ClientCapabilities, InitializeParams, InitializeResult, PositionEncodingKind, ServerCapabilities,
    ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
};

pub struct ServerState {
    /// Encoding negotiated during the initialize handshake. Phases that
    /// exchange positions on the wire (diagnostics, hover, …) read this to
    /// decide how to convert their byte offsets.
    pub position_encoding: PositionEncodingKind,
}

impl ServerState {
    pub fn new(position_encoding: PositionEncodingKind) -> Self {
        ServerState { position_encoding }
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

fn handle_notification(_state: &mut ServerState, notif: Notification) -> anyhow::Result<()> {
    match notif.method.as_str() {
        "exit" => anyhow::bail!("exit notification without prior shutdown"),
        "$/setTrace" | "$/cancelRequest" => {
            log::debug!("{} ignored in phase 1", notif.method);
        }
        other => {
            log::debug!("unhandled notification: {other}");
        }
    }
    Ok(())
}
