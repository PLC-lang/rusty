// `lsp_types::Uri` wraps `fluent_uri::Uri`, which has internal Cell-based
// caching for parsed components. That makes clippy fire `mutable_key_type`
// when we use `Uri` as a HashMap/HashSet key — but the hash itself is
// stable for a given URI value (the cache holds derived data, not
// identity), so the lint is conservative here. We use `Uri` as a key
// throughout the publish path; allowing the lint module-wide is cleaner
// than scattering attribute annotations.
#![allow(clippy::mutable_key_type)]

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use anyhow::Context as _;
use crossbeam_channel::{select, Sender};
use lsp_server::{
    Connection, ErrorCode, IoThreads, Message, Notification, Request, RequestId, Response, ResponseError,
};
use lsp_types::{
    ClientCapabilities, DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, ExecuteCommandOptions, ExecuteCommandParams, FileChangeType, InitializeParams,
    InitializeResult, MessageType, PositionEncodingKind, PublishDiagnosticsParams, ServerCapabilities,
    ServerInfo, ShowMessageParams, TextDocumentSyncCapability, TextDocumentSyncKind, Unregistration,
    UnregistrationParams, Uri,
};

pub mod compile;
pub mod diagnostics;
pub mod document;
pub mod project;
pub mod watcher;

const REPARSE_PROJECT_COMMAND: &str = "rusty.reparseProject";

/// Runtime configuration for the server.
#[derive(Default, Debug, Clone)]
pub struct Settings {
    pub config_override: Option<PathBuf>,
}

pub struct ServerState {
    /// Encoding negotiated during the initialize handshake. Phases that
    /// exchange positions on the wire (diagnostics, hover, …) read this to
    /// decide how to convert their byte offsets.
    pub position_encoding: PositionEncodingKind,
    /// In-memory view of every editor-open buffer.
    pub documents: document::DocumentStore,
    /// Workspace folder root captured at `initialize`. Used by project
    /// discovery; `None` for clients that don't supply one (rare in
    /// practice — every modern editor sends at least `rootUri`).
    pub workspace_root: Option<PathBuf>,
    /// Resolved `plc.json` override path (CLI arg or
    /// `initializationOptions.plcConfigPath`). Takes precedence over
    /// discovery.
    pub plc_config_override: Option<PathBuf>,
    /// Resolved plc.json path (override or downward discovery). Cleared
    /// if a watched-files notification reports the file was deleted.
    pub plc_config_path: Option<PathBuf>,
    /// True when a compile request has been sent to the worker and no
    /// result has come back yet.
    pub compile_pending: bool,
    /// True when a compile trigger arrived while one was already pending.
    /// Honoured when the in-flight compile finishes — see decisions log D1.
    pub compile_dirty: bool,
    /// URIs we've published non-empty diagnostics for on the most recent
    /// compile. Used to send empty `publishDiagnostics` for files that
    /// previously had errors and now don't, so the editor clears them.
    pub published_uris: HashSet<Uri>,
    /// Request ID we used for the in-flight `client/registerCapability` or
    /// `client/unregisterCapability` send; used to recognise the matching
    /// Response when it comes back.
    pub pending_registration: Option<RequestId>,
    /// Counter for server-initiated request IDs (string-namespaced to
    /// avoid colliding with client-side IDs).
    pub next_request_id: u64,
}

impl ServerState {
    pub fn new(
        position_encoding: PositionEncodingKind,
        workspace_root: Option<PathBuf>,
        plc_config_override: Option<PathBuf>,
    ) -> Self {
        let plc_config_path =
            project::resolve_plc_config_path(workspace_root.as_deref(), plc_config_override.as_deref());
        ServerState {
            position_encoding,
            documents: document::DocumentStore::new(),
            workspace_root,
            plc_config_override,
            plc_config_path,
            compile_pending: false,
            compile_dirty: false,
            published_uris: HashSet::new(),
            pending_registration: None,
            next_request_id: 0,
        }
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

pub fn run(settings: Settings) -> anyhow::Result<()> {
    let (connection, io_threads) = Connection::stdio();
    let result = serve(&connection, settings);
    finalize(result, io_threads)
}

pub fn serve(connection: &Connection, settings: Settings) -> anyhow::Result<()> {
    log::info!("plc-lsp starting; performing initialize handshake");

    let (init_id, init_params_val) = connection.initialize_start().context(
        "LSP initialize handshake failed waiting for the client's initialize request \
         (LSP messages must be framed as 'Content-Length: N\\r\\n\\r\\n<json>')",
    )?;
    let init_params: InitializeParams = serde_json::from_value(init_params_val)
        .context("client's initialize params did not match the LSP InitializeParams schema")?;

    let position_encoding = pick_position_encoding(&init_params.capabilities);
    log::info!("plc-lsp negotiated position encoding: {position_encoding:?}");

    let workspace_root = project::extract_workspace_root(&init_params);
    let init_options = init_params
        .initialization_options
        .as_ref()
        .and_then(|v| serde_json::from_value::<project::InitializationOptions>(v.clone()).ok())
        .unwrap_or_default();
    let plc_config_override =
        settings.config_override.clone().or_else(|| init_options.plc_config_path.map(PathBuf::from));

    if let Some(root) = &workspace_root {
        log::info!("plc-lsp workspace root: {root:?}");
    }
    if let Some(override_path) = &plc_config_override {
        log::info!("plc-lsp plc.json override: {override_path:?}");
    }

    let server_capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        position_encoding: Some(position_encoding.clone()),
        execute_command_provider: Some(ExecuteCommandOptions {
            commands: vec![REPARSE_PROJECT_COMMAND.to_string()],
            work_done_progress_options: Default::default(),
        }),
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

    let mut state = ServerState::new(position_encoding, workspace_root, plc_config_override);
    if let Some(path) = &state.plc_config_path {
        log::info!("plc-lsp resolved plc.json at {path:?}");
    } else {
        log::warn!("plc-lsp: no plc.json found; filesystem watching disabled");
        send_show_message(
            connection,
            MessageType::WARNING,
            "plc-lsp: no plc.json found under the workspace. Filesystem \
             watching is disabled; use the 'rusty.reparseProject' command \
             after external edits.",
        );
    }
    let worker = compile::CompileWorker::spawn();

    register_file_watchers(&mut state, connection);

    // Q11: initial compile as soon as we're initialized so the user
    // sees diagnostics before opening any file.
    trigger_compile(&mut state, &worker.compile_tx);

    let main_loop_result = main_loop(connection, &mut state, &worker);

    log::info!("plc-lsp shutting down");
    worker.join();
    main_loop_result
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

fn main_loop(
    connection: &Connection,
    state: &mut ServerState,
    worker: &compile::CompileWorker,
) -> anyhow::Result<()> {
    loop {
        select! {
            recv(connection.receiver) -> msg => {
                let Ok(msg) = msg else {
                    // Receiver disconnected — client closed stdin.
                    return Ok(());
                };
                match msg {
                    Message::Request(req) => {
                        if connection.handle_shutdown(&req)? {
                            return Ok(());
                        }
                        handle_request(state, connection, &worker.compile_tx, req);
                    }
                    Message::Notification(notif) => {
                        if !handle_notification(state, connection, &worker.compile_tx, notif)? {
                            return Ok(());
                        }
                    }
                    Message::Response(resp) => {
                        handle_response(state, connection, resp);
                    }
                }
            }
            recv(worker.result_rx) -> result => {
                let Ok(result) = result else {
                    log::error!("compile worker result channel closed unexpectedly");
                    return Ok(());
                };
                handle_compile_result(state, connection, &worker.compile_tx, result);
            }
        }
    }
}

fn handle_request(
    state: &mut ServerState,
    connection: &Connection,
    compile_tx: &Sender<compile::CompileRequest>,
    req: Request,
) {
    match req.method.as_str() {
        "workspace/executeCommand" => handle_execute_command(state, connection, compile_tx, req),
        _ => {
            log::debug!("unhandled request: method={} id={:?}", req.method, req.id);
            let response = Response {
                id: req.id.clone(),
                result: None,
                error: Some(ResponseError {
                    code: ErrorCode::MethodNotFound as i32,
                    message: format!("method '{}' not implemented", req.method),
                    data: None,
                }),
            };
            if let Err(e) = connection.sender.send(Message::Response(response)) {
                log::error!("failed to send response: {e}");
            }
        }
    }
}

fn handle_execute_command(
    state: &mut ServerState,
    connection: &Connection,
    compile_tx: &Sender<compile::CompileRequest>,
    req: Request,
) {
    let req_id = req.id.clone();
    let params: ExecuteCommandParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed ExecuteCommandParams: {e}"),
            );
            return;
        }
    };

    match params.command.as_str() {
        REPARSE_PROJECT_COMMAND => {
            log::info!("rusty.reparseProject invoked; triggering compile");
            trigger_compile(state, compile_tx);
            let response = Response { id: req_id, result: Some(serde_json::Value::Null), error: None };
            if let Err(e) = connection.sender.send(Message::Response(response)) {
                log::error!("failed to send executeCommand response: {e}");
            }
        }
        other => send_error_response(
            connection,
            req_id,
            ErrorCode::MethodNotFound,
            format!("unknown command '{other}'"),
        ),
    }
}

fn handle_response(state: &mut ServerState, connection: &Connection, resp: Response) {
    if state.pending_registration.as_ref() == Some(&resp.id) {
        state.pending_registration = None;
        if let Some(err) = resp.error {
            log::warn!("client rejected file-watch registration: {} ({})", err.message, err.code);
            send_show_message(
                connection,
                MessageType::WARNING,
                "plc-lsp: client rejected file-watch registration. Use the \
                 'rusty.reparseProject' command after external edits.",
            );
        } else {
            log::info!("file-watch registration acknowledged by client");
        }
    } else {
        log::debug!("unexpected response from client (id={:?})", resp.id);
    }
}

/// Returns `false` when the notification should end the main loop
/// (an `exit` arriving here without prior shutdown is a protocol
/// violation, but we treat it as a clean termination request rather
/// than panicking).
fn handle_notification(
    state: &mut ServerState,
    connection: &Connection,
    compile_tx: &Sender<compile::CompileRequest>,
    notif: Notification,
) -> anyhow::Result<bool> {
    match notif.method.as_str() {
        "exit" => return Ok(false),
        "textDocument/didOpen" => handle_did_open(state, notif),
        "textDocument/didChange" => handle_did_change(state, notif),
        "textDocument/didSave" => handle_did_save(state, compile_tx, notif),
        "textDocument/didClose" => handle_did_close(state, notif),
        "workspace/didChangeWatchedFiles" => {
            handle_did_change_watched_files(state, connection, compile_tx, notif)
        }
        "$/setTrace" | "$/cancelRequest" => {
            log::debug!("{} ignored in phase 1", notif.method);
        }
        other => {
            log::debug!("unhandled notification: {other}");
        }
    }
    Ok(true)
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

fn handle_did_save(
    state: &mut ServerState,
    compile_tx: &Sender<compile::CompileRequest>,
    _notif: Notification,
) {
    trigger_compile(state, compile_tx);
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

fn handle_did_change_watched_files(
    state: &mut ServerState,
    connection: &Connection,
    compile_tx: &Sender<compile::CompileRequest>,
    notif: Notification,
) {
    let params: DidChangeWatchedFilesParams = match serde_json::from_value(notif.params) {
        Ok(p) => p,
        Err(e) => {
            log::warn!("workspace/didChangeWatchedFiles: malformed params: {e}");
            return;
        }
    };

    let mut any_source_change = false;
    let mut plc_json_changed = false;
    let mut plc_json_deleted = false;

    for event in &params.changes {
        let Some(path) = project::file_uri_to_path(&event.uri) else {
            log::debug!("watched-files: ignoring non-file URI {:?}", event.uri);
            continue;
        };

        let is_plc_json = state.plc_config_path.as_deref() == Some(path.as_path());
        if is_plc_json {
            match event.typ {
                FileChangeType::DELETED => plc_json_deleted = true,
                _ => plc_json_changed = true,
            }
            continue;
        }

        // Project source. Editor-owned buffer wins over disk events.
        if state.documents.get(&event.uri).is_some() {
            log::debug!("watched-files: ignoring change to open buffer {:?}", event.uri);
            continue;
        }
        any_source_change = true;
    }

    if plc_json_deleted {
        log::error!("plc.json was deleted; no further compiles will run until it returns");
        state.plc_config_path = None;
        // Don't re-register: with no plc.json there are no globs to watch.
        return;
    }

    if plc_json_changed {
        log::info!("plc.json changed; re-registering watchers + triggering compile");
        unregister_file_watchers(state, connection);
        register_file_watchers(state, connection);
        trigger_compile(state, compile_tx);
        return;
    }

    if any_source_change {
        trigger_compile(state, compile_tx);
    }
}

fn accept_language_id(language_id: &str) -> bool {
    matches!(language_id, "structured-text" | "st" | "iecst")
}

/// Send `client/registerCapability` for the file watcher set. No-op (with
/// a `window/showMessage` warning) when plc.json hasn't been resolved.
fn register_file_watchers(state: &mut ServerState, connection: &Connection) {
    let Some(config_path) = state.plc_config_path.clone() else {
        return; // Already showed a warning during serve() startup.
    };
    let globs = match watcher::extract_source_globs(&config_path) {
        Ok(g) => g,
        Err(e) => {
            log::warn!("failed to read plc.json globs for watcher: {e}");
            send_show_message(
                connection,
                MessageType::WARNING,
                "plc-lsp: failed to read plc.json file globs; filesystem \
                 watching may be partial. Use 'rusty.reparseProject' as a \
                 fallback.",
            );
            return;
        }
    };
    let params = watcher::build_registration(&config_path, &globs);
    let id = next_request_id(state);
    let request = Request {
        id: id.clone(),
        method: "client/registerCapability".to_string(),
        params: serde_json::to_value(params).expect("RegistrationParams must serialise"),
    };
    if let Err(e) = connection.sender.send(Message::Request(request)) {
        log::error!("failed to send registerCapability: {e}");
        return;
    }
    state.pending_registration = Some(id);
}

fn unregister_file_watchers(state: &mut ServerState, connection: &Connection) {
    let params = UnregistrationParams {
        unregisterations: vec![Unregistration {
            id: watcher::WATCHER_REGISTRATION_ID.to_string(),
            method: watcher::DID_CHANGE_WATCHED_FILES_METHOD.to_string(),
        }],
    };
    let id = next_request_id(state);
    let request = Request {
        id,
        method: "client/unregisterCapability".to_string(),
        params: serde_json::to_value(params).expect("UnregistrationParams must serialise"),
    };
    if let Err(e) = connection.sender.send(Message::Request(request)) {
        log::error!("failed to send unregisterCapability: {e}");
    }
}

fn next_request_id(state: &mut ServerState) -> RequestId {
    let id = state.next_request_id;
    state.next_request_id += 1;
    RequestId::from(format!("rusty.req.{id}"))
}

fn send_show_message(connection: &Connection, typ: MessageType, message: &str) {
    let params = ShowMessageParams { typ, message: message.to_string() };
    let notif = Notification {
        method: "window/showMessage".to_string(),
        params: serde_json::to_value(params).expect("ShowMessageParams must serialise"),
    };
    if let Err(e) = connection.sender.send(Message::Notification(notif)) {
        log::error!("failed to send window/showMessage: {e}");
    }
}

fn send_error_response(connection: &Connection, id: RequestId, code: ErrorCode, message: String) {
    let response =
        Response { id, result: None, error: Some(ResponseError { code: code as i32, message, data: None }) };
    if let Err(e) = connection.sender.send(Message::Response(response)) {
        log::error!("failed to send error response: {e}");
    }
}

/// Build a snapshot from current ServerState and send it to the worker.
/// If a compile is already in flight, just mark the state dirty so we
/// re-fire when the result arrives.
fn trigger_compile(state: &mut ServerState, compile_tx: &Sender<compile::CompileRequest>) {
    if state.compile_pending {
        state.compile_dirty = true;
        log::debug!("compile already pending; marking dirty");
        return;
    }
    let snapshot = build_snapshot(state);
    if compile_tx.send(compile::CompileRequest { snapshot }).is_err() {
        log::error!("compile worker channel closed; cannot send request");
        return;
    }
    state.compile_pending = true;
    state.compile_dirty = false;
}

fn handle_compile_result(
    state: &mut ServerState,
    connection: &Connection,
    compile_tx: &Sender<compile::CompileRequest>,
    result: compile::CompileResult,
) {
    state.compile_pending = false;
    if let Some(err) = &result.error {
        log::error!("compile pipeline error: {err}");
    }
    log::debug!("compile produced {} diagnostics", result.diagnostics.len());

    publish_diagnostics(state, connection, result);

    if state.compile_dirty {
        log::debug!("re-firing compile due to dirty state");
        state.compile_dirty = false;
        trigger_compile(state, compile_tx);
    }
}

fn publish_diagnostics(state: &mut ServerState, connection: &Connection, result: compile::CompileResult) {
    let grouped = diagnostics::map_collected(result.diagnostics, &result.file_paths);
    let new_uris: HashSet<Uri> = grouped.keys().cloned().collect();

    // Clear diagnostics for any URI we published last time but isn't in the new set.
    for stale in state.published_uris.difference(&new_uris) {
        send_diagnostics(connection, stale.clone(), Vec::new(), None);
    }

    // Publish the new set.
    for (uri, diags) in grouped {
        let version = state.documents.get(&uri).map(|b| b.version);
        send_diagnostics(connection, uri, diags, version);
    }

    state.published_uris = new_uris;
}

fn send_diagnostics(
    connection: &Connection,
    uri: Uri,
    diagnostics: Vec<lsp_types::Diagnostic>,
    version: Option<i32>,
) {
    let params = PublishDiagnosticsParams { uri, diagnostics, version };
    let notif = Notification {
        method: "textDocument/publishDiagnostics".to_string(),
        params: serde_json::to_value(params).expect("PublishDiagnosticsParams serialise must succeed"),
    };
    if let Err(e) = connection.sender.send(Message::Notification(notif)) {
        log::error!("failed to send publishDiagnostics: {e}");
    }
}

fn build_snapshot(state: &ServerState) -> compile::CompileSnapshot {
    let mut open_buffers: HashMap<PathBuf, String> = HashMap::new();
    for (uri, buf) in state.documents.iter() {
        if let Some(path) = project::file_uri_to_path(uri) {
            open_buffers.insert(path, buf.content.clone());
        }
    }
    compile::CompileSnapshot {
        plc_config_path: state.plc_config_path.clone(),
        workspace_root: state.workspace_root.clone(),
        open_buffers,
        position_encoding: state.position_encoding.clone(),
    }
}
