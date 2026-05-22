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
    request::{GotoDeclarationParams, GotoDeclarationResponse},
    CallHierarchyIncomingCallsParams, CallHierarchyItem, CallHierarchyOutgoingCallsParams,
    CallHierarchyPrepareParams, CallHierarchyServerCapability, ClientCapabilities, CompletionList,
    CompletionOptions, CompletionParams, DeclarationCapability, DidChangeTextDocumentParams,
    DidChangeWatchedFilesParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DocumentFormattingParams, DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse,
    ExecuteCommandOptions, ExecuteCommandParams, FileChangeType, GotoDefinitionParams,
    GotoDefinitionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability, InitializeParams,
    InitializeResult, Location, MarkupContent, MarkupKind, MessageType, OneOf, Position,
    PositionEncodingKind, PrepareRenameResponse, PublishDiagnosticsParams, Range, ReferenceParams,
    RenameOptions, RenameParams, ServerCapabilities, ServerInfo, ShowMessageParams,
    TextDocumentPositionParams, TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Unregistration,
    UnregistrationParams, Uri,
};
use plc_diagnostics::cancellation::CancellationToken;

pub mod call_hierarchy;
pub mod code_lens;
pub mod compile;
pub mod completion;
pub mod diagnostics;
pub mod docstring;
pub mod document;
pub mod formatter;
pub mod hover_format;
pub mod inlay_hints;
pub mod interfaces;
pub mod outline;
pub mod position;
pub mod project;
pub mod rename;
pub mod reverse_index;
pub mod semantic_tokens;
pub mod token_cache;
pub mod token_walk;
pub mod watcher;

const REPARSE_PROJECT_COMMAND: &str = "rusty.reparseProject";

/// Runtime configuration for the server. Currently only carries an
/// optional `plc.json` override path supplied via the CLI; the LSP
/// client can supply the same override via
/// `initializationOptions.plcConfigPath`.
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
    /// Cancellation handle for the in-flight compile (Some when
    /// `compile_pending` is true). On a new trigger arriving mid-compile
    /// we call `.cancel()` on this — the worker observes the flag at
    /// the next pipeline check point and bails with
    /// `CompileOutcome::Cancelled`. See phase-6 Q3.
    pub active_compile_token: Option<CancellationToken>,
    /// URIs we've published non-empty diagnostics for on the most recent
    /// compile. Used to send empty `publishDiagnostics` for files that
    /// previously had errors and now don't, so the editor clears them.
    pub published_uris: HashSet<Uri>,
    /// Per-URI outline (file symbols) from the last successfully
    /// attached compile. Replaced atomically on each attach; survives
    /// failed and cancelled compiles so queries answer from the
    /// last-known-good state.
    pub document_symbols: HashMap<Uri, Vec<DocumentSymbol>>,
    /// Result of the last `BuildPipeline::annotate(...)` that returned
    /// `Ok`. Owned (not `Arc`): the main thread is single-threaded so
    /// borrows from queries are uncontested. Cursor-parameterised
    /// queries (hover, goto-def, references) read from here.
    pub annotated: Option<plc_driver::pipelines::AnnotatedProject>,
    /// `GlobalContext` paired with `annotated`. Used by hover format
    /// and position lookup for the source-text display rule: calling
    /// `ctxt.slice(&location)` recovers the user's literal source for
    /// type expressions, defusing preprocessor/lowering rewrites.
    pub ctxt: Option<plc_index::GlobalContext>,
    /// Project-wide declaration → uses map, used to answer
    /// `textDocument/references`. Replaced atomically alongside
    /// `annotated` and `ctxt` on each successfully attached compile.
    pub reverse_index: Option<reverse_index::ReverseIndex>,
    /// Request ID we used for the in-flight `client/registerCapability` or
    /// `client/unregisterCapability` send; used to recognise the matching
    /// Response when it comes back.
    pub pending_registration: Option<RequestId>,
    /// Counter for server-initiated request IDs (string-namespaced to
    /// avoid colliding with client-side IDs).
    pub next_request_id: u64,
    /// Per-file cache of `lex_with_trivia` output. Keyed by path; entries
    /// hash-invalidate on content change. Consumed by completion and hover
    /// to walk tokens (and comments) around the cursor.
    pub token_cache: token_cache::TokenCache,
    /// Whether the client advertised `workspace.semanticTokens.refreshSupport`.
    /// After each successful compile we send `workspace/semanticTokens/refresh`
    /// to clients that support it so the editor re-paints with fresh
    /// annotations rather than caching the empty pre-annotate response.
    pub supports_semantic_tokens_refresh: bool,
    /// Mirror of the above for `workspace.inlayHint.refreshSupport`.
    pub supports_inlay_hint_refresh: bool,
    /// Mirror for `workspace.codeLens.refreshSupport`.
    pub supports_code_lens_refresh: bool,
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
            active_compile_token: None,
            published_uris: HashSet::new(),
            document_symbols: HashMap::new(),
            annotated: None,
            ctxt: None,
            reverse_index: None,
            pending_registration: None,
            next_request_id: 0,
            token_cache: token_cache::TokenCache::new(),
            supports_semantic_tokens_refresh: false,
            supports_inlay_hint_refresh: false,
            supports_code_lens_refresh: false,
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
        // Phase 7: file outline. We always answer from the cached
        // outline on `ServerState` — never recompile per request.
        document_symbol_provider: Some(OneOf::Left(true)),
        // Phase 8: hover. Answers from the cached AnnotatedProject on
        // `ServerState.annotated`; returns null when the cursor isn't
        // over a resolvable identifier or when no successful compile
        // has been attached yet.
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        // Phase 9: goto-def and goto-declaration. Collapsed for the
        // prototype — both endpoints return the same declaration
        // location. The distinction (interface declaration vs
        // implementing-FB body) is a refinement for post-phase-13.
        definition_provider: Some(OneOf::Left(true)),
        declaration_provider: Some(DeclarationCapability::Simple(true)),
        // Phase 10: find-references. Powered by a project-wide reverse
        // index built on the worker (`reverse_index::ReverseIndex`)
        // and cached on `ServerState`. Honours `context.includeDeclaration`.
        references_provider: Some(OneOf::Left(true)),
        // Phase 11: call hierarchy. `prepareCallHierarchy` returns an
        // item for callable POUs; `incomingCalls` filters the reverse
        // index to call sites grouped by container; `outgoingCalls`
        // walks the POU body once per request.
        call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
        // Phase 12: rename. `prepareRename` runs the same position
        // lookup as the other cursor features; `rename` emits a
        // `WorkspaceEdit` editing the declaration site + every
        // reverse-index entry. `prepare_provider: true` tells clients
        // to call prepareRename first.
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: Default::default(),
        })),
        // Phase 13: completion. Only `.` is advertised as an auto-trigger
        // per the grill decision (Q5/D17); everything else relies on the
        // user's explicit ctrl-space request. `resolve_provider: false` —
        // we send fully-populated CompletionItems and don't implement
        // Initial completion lists are returned without docstrings;
        // the docs come back via `completionItem/resolve` when the
        // editor focuses an item. Keeps the first response light when
        // the project has hundreds of entries.
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![".".to_string()]),
            resolve_provider: Some(true),
            ..Default::default()
        }),
        // F5: whole-document formatting (range/on-type left for later).
        document_formatting_provider: Some(OneOf::Left(true)),
        // H2: inlay hints — parameter-name tags above positional call
        // args. Resolution is computed eagerly per request; no separate
        // resolve roundtrip needed.
        inlay_hint_provider: Some(lsp_types::OneOf::Left(true)),
        // H3: code lenses for interface / implementation links. The
        // standard `editor.codeLens` master toggle is the per-user
        // escape hatch.
        code_lens_provider: Some(lsp_types::CodeLensOptions { resolve_provider: Some(false) }),
        // F6: server-side semantic highlighting refinement. We advertise
        // `full` only (no range, no delta) for the POC; the legend is
        // sourced from the semantic_tokens module so the wire format
        // and the collector can't drift apart.
        semantic_tokens_provider: Some(lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions(
            lsp_types::SemanticTokensOptions {
                work_done_progress_options: Default::default(),
                legend: lsp_types::SemanticTokensLegend {
                    token_types: semantic_tokens::TYPES
                        .iter()
                        .map(|s| lsp_types::SemanticTokenType::new(s))
                        .collect(),
                    token_modifiers: semantic_tokens::MODIFIERS
                        .iter()
                        .map(|s| lsp_types::SemanticTokenModifier::new(s))
                        .collect(),
                },
                range: Some(false),
                full: Some(lsp_types::SemanticTokensFullOptions::Bool(true)),
            },
        )),
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
    // Read refresh-support advertisements from the client capabilities
    // so we know which `workspace/<feature>/refresh` requests are safe
    // to send when annotated lands. vscode advertises all three;
    // helix/nvim builtin LSP varies.
    if let Some(ws) = init_params.capabilities.workspace.as_ref() {
        state.supports_semantic_tokens_refresh =
            ws.semantic_tokens.as_ref().and_then(|s| s.refresh_support).unwrap_or(false);
        state.supports_inlay_hint_refresh =
            ws.inlay_hint.as_ref().and_then(|s| s.refresh_support).unwrap_or(false);
        state.supports_code_lens_refresh =
            ws.code_lens.as_ref().and_then(|s| s.refresh_support).unwrap_or(false);
    }
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
            recv(worker.result_rx) -> outcome => {
                let Ok(outcome) = outcome else {
                    log::error!("compile worker result channel closed unexpectedly");
                    return Ok(());
                };
                handle_compile_outcome(state, connection, &worker.compile_tx, outcome);
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
        "textDocument/documentSymbol" => handle_document_symbol(state, connection, req),
        "textDocument/hover" => handle_hover(state, connection, req),
        "textDocument/definition" => handle_definition(state, connection, req),
        "textDocument/declaration" => handle_declaration(state, connection, req),
        "textDocument/references" => handle_references(state, connection, req),
        "textDocument/prepareCallHierarchy" => handle_prepare_call_hierarchy(state, connection, req),
        "callHierarchy/incomingCalls" => handle_incoming_calls(state, connection, req),
        "callHierarchy/outgoingCalls" => handle_outgoing_calls(state, connection, req),
        "textDocument/prepareRename" => handle_prepare_rename(state, connection, req),
        "textDocument/rename" => handle_rename(state, connection, req),
        "textDocument/completion" => handle_completion(state, connection, req),
        "completionItem/resolve" => handle_completion_item_resolve(state, connection, req),
        "textDocument/formatting" => handle_formatting(state, connection, req),
        "textDocument/semanticTokens/full" => handle_semantic_tokens_full(state, connection, req),
        "textDocument/inlayHint" => handle_inlay_hint(state, connection, req),
        "textDocument/codeLens" => handle_code_lens(state, connection, req),
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

/// Answer `textDocument/documentSymbol` from the cached outline on
/// `ServerState`. If the file hasn't been seen by a successful compile
/// yet, we return an empty list (the editor handles that gracefully —
/// shows "no symbols yet" rather than an error). Never recompiles per
/// request; see phase-7-10 plan §2.1.
fn handle_document_symbol(state: &ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: DocumentSymbolParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed DocumentSymbolParams: {e}"),
            );
            return;
        }
    };
    let symbols = state.document_symbols.get(&params.text_document.uri).cloned().unwrap_or_default();
    let result = DocumentSymbolResponse::Nested(symbols);
    let response = Response {
        id: req_id,
        result: Some(serde_json::to_value(result).expect("DocumentSymbolResponse must serialise")),
        error: None,
    };
    if let Err(e) = connection.sender.send(Message::Response(response)) {
        log::error!("failed to send documentSymbol response: {e}");
    }
}

/// Answer `textDocument/hover` from the cached `AnnotatedProject` on
/// `ServerState`. Returns null when there's no successful compile yet,
/// when the cursor isn't over a resolvable identifier, or when the
/// resolved declaration has no displayable source (synthetic /
/// `<internal>` locations).
fn handle_hover(state: &mut ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: HoverParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed HoverParams: {e}"),
            );
            return;
        }
    };

    let hover = hover_for_position(state, &params);
    let response = Response {
        id: req_id,
        result: Some(serde_json::to_value(hover).expect("Hover must serialise")),
        error: None,
    };
    if let Err(e) = connection.sender.send(Message::Response(response)) {
        log::error!("failed to send hover response: {e}");
    }
}

/// Pure logic for the hover handler — separated so tests can drive it
/// without spinning up a `Connection`. Returns `None` when there's
/// nothing to show; callers serialise `null`, which is the LSP-correct
/// "no hover" reply. Takes `&mut ServerState` because the docstring
/// lookup populates the token cache for declarations in other files.
fn hover_for_position(state: &mut ServerState, params: &HoverParams) -> Option<Hover> {
    let position_encoding = state.position_encoding.clone();
    let pos = &params.text_document_position_params;

    let (symbol, body) = {
        let annotated = state.annotated.as_ref()?;
        let ctxt = state.ctxt.as_ref()?;
        let source_contents = build_source_contents(state);
        let symbol = position::symbol_under_cursor(
            annotated,
            ctxt,
            &pos.text_document.uri,
            pos.position,
            &position_encoding,
            &source_contents,
        )?;
        let body = hover_format::format_symbol(&symbol, &annotated.index)?;
        (symbol, body)
    };

    let range = diagnostics::code_span_to_range(symbol.usage_location.get_span(), &position_encoding, None);
    let mut body = body;
    if let Some(doc) = lookup_docstring(state, &symbol) {
        body.push_str("\n\n---\n\n");
        body.push_str(&doc);
    }

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: body }),
        range,
    })
}

/// Pull the doc body attached to the symbol's declaration site, if any.
/// Crosses files transparently: source comes from the in-memory buffer
/// when open, disk otherwise; tokens come from the per-path cache.
fn lookup_docstring(state: &mut ServerState, symbol: &position::SymbolUnderCursor) -> Option<String> {
    let resolved = symbol.resolved.as_ref()?;
    docstring::fetch(&mut state.token_cache, &state.documents, &resolved.declaration_location, resolved.kind)
}

/// Build a `path → source` map from the open editor buffers — used by
/// the position lookup to convert utf-16 columns into byte offsets.
///
/// We don't cache this on `ServerState` because it has to track every
/// `didChange` and the documents store is already authoritative for
/// that data. Building per-request is fine: hover/goto/refs are
/// interactive and the maps are small (a handful of open files).
/// Answer `textDocument/definition`. Builds a `SymbolUnderCursor` from
/// the cached project and returns `resolved.declaration_location` as
/// an LSP `Location`. Returns null when nothing under the cursor
/// resolves.
fn handle_definition(state: &ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: GotoDefinitionParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed GotoDefinitionParams: {e}"),
            );
            return;
        }
    };
    let result =
        goto_target(state, &params.text_document_position_params).map(GotoDefinitionResponse::Scalar);
    send_goto_response(connection, req_id, serde_json::to_value(result));
}

/// Answer `textDocument/declaration`. For a method that overrides an
/// interface signature, returns the *interface's* method site rather
/// than the FB's concrete method body — that's the LSP split between
/// declaration and definition. For everything else, falls back to the
/// same site `handle_definition` would return.
fn handle_declaration(state: &ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: GotoDeclarationParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed GotoDeclarationParams: {e}"),
            );
            return;
        }
    };
    let result = goto_declaration_target(state, &params.text_document_position_params)
        .map(GotoDeclarationResponse::Scalar);
    send_goto_response(connection, req_id, serde_json::to_value(result));
}

/// Like `goto_target` but consults the interface-implementation
/// linkage. When the cursor resolves to a method on an FB/Class that
/// implements an interface method, redirect to the interface's
/// declaration site.
fn goto_declaration_target(
    state: &ServerState,
    position: &lsp_types::TextDocumentPositionParams,
) -> Option<Location> {
    let annotated = state.annotated.as_ref()?;
    let ctxt = state.ctxt.as_ref()?;
    let source_contents = build_source_contents(state);

    let symbol = position::symbol_under_cursor(
        annotated,
        ctxt,
        &position.text_document.uri,
        position.position,
        &state.position_encoding,
        &source_contents,
    )?;
    let resolved = symbol.resolved?;

    // If the resolved symbol is a method whose container implements an
    // interface declaring the same method, prefer the interface site.
    let target_location = if matches!(resolved.kind, position::SymbolKind::Pou) {
        interfaces::interface_method_decl_for(&annotated.index, &resolved.qualified_name)
            .unwrap_or(resolved.declaration_location.clone())
    } else {
        resolved.declaration_location.clone()
    };

    let path = target_location.get_file_name()?;
    let uri = diagnostics::path_to_uri(path)?;
    let range = diagnostics::code_span_to_range(target_location.get_span(), &state.position_encoding, None)?;
    Some(Location { uri, range })
}

/// Shared core: position lookup → optional `Location`. Returns `None`
/// when there's no cached project, the cursor isn't over an
/// identifier, or the identifier resolves to no declaration.
fn goto_target(state: &ServerState, position: &lsp_types::TextDocumentPositionParams) -> Option<Location> {
    let annotated = state.annotated.as_ref()?;
    let ctxt = state.ctxt.as_ref()?;
    let source_contents = build_source_contents(state);

    let symbol = position::symbol_under_cursor(
        annotated,
        ctxt,
        &position.text_document.uri,
        position.position,
        &state.position_encoding,
        &source_contents,
    )?;
    let resolved = symbol.resolved?;
    let path = resolved.declaration_location.get_file_name()?;
    let uri = diagnostics::path_to_uri(path)?;
    let range = diagnostics::code_span_to_range(
        resolved.declaration_location.get_span(),
        &state.position_encoding,
        None,
    )?;
    Some(Location { uri, range })
}

/// Answer `textDocument/references`. Resolves the cursor to a
/// declaration, looks up its uses in the cached reverse index, and
/// returns the list (optionally including the declaration itself).
/// Returns an empty list — not null — when there's nothing to surface,
/// because that's what most LSP clients expect for "no references."
fn handle_references(state: &ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: ReferenceParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed ReferenceParams: {e}"),
            );
            return;
        }
    };

    let locations = references_for_position(state, &params).unwrap_or_default();
    let response = Response {
        id: req_id,
        result: Some(serde_json::to_value(locations).expect("Vec<Location> must serialise")),
        error: None,
    };
    if let Err(e) = connection.sender.send(Message::Response(response)) {
        log::error!("failed to send references response: {e}");
    }
}

/// Pure-logic core: position lookup → reverse-index query → LSP
/// `Location`s. Returns `None` only when state is missing entirely;
/// "no references found" returns `Some(empty)` so the caller can ship
/// `[]` to the client.
fn references_for_position(state: &ServerState, params: &ReferenceParams) -> Option<Vec<Location>> {
    let annotated = state.annotated.as_ref()?;
    let ctxt = state.ctxt.as_ref()?;
    let reverse_index = state.reverse_index.as_ref()?;
    let source_contents = build_source_contents(state);

    let pos = &params.text_document_position;
    let symbol = position::symbol_under_cursor(
        annotated,
        ctxt,
        &pos.text_document.uri,
        pos.position,
        &state.position_encoding,
        &source_contents,
    )?;
    let resolved = symbol.resolved?;

    let mut out: Vec<Location> = Vec::new();
    if params.context.include_declaration {
        if let Some(loc) = source_location_to_lsp(&resolved.declaration_location, &state.position_encoding) {
            out.push(loc);
        }
    }
    for entry in reverse_index.lookup(&resolved.declaration_location) {
        if let Some(loc) = source_location_to_lsp(&entry.location, &state.position_encoding) {
            out.push(loc);
        }
    }
    // Interface method → also list every FB/Class implementation. The
    // user asked for find-references on an interface method to
    // surface both the impls and the call sites.
    if matches!(resolved.kind, position::SymbolKind::Pou) {
        for impl_loc in interfaces::implementations_of(&annotated.index, &resolved.qualified_name) {
            if let Some(loc) = source_location_to_lsp(&impl_loc, &state.position_encoding) {
                out.push(loc);
            }
        }
    }
    Some(out)
}

/// Answer `textDocument/prepareCallHierarchy`. Returns one item if the
/// cursor sits on a callable POU; null otherwise (Q6 strict).
fn handle_prepare_call_hierarchy(state: &ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: CallHierarchyPrepareParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed CallHierarchyPrepareParams: {e}"),
            );
            return;
        }
    };

    let result = prepare_call_hierarchy(state, &params).map(|item| vec![item]);
    send_json_response(connection, req_id, serde_json::to_value(result));
}

fn prepare_call_hierarchy(
    state: &ServerState,
    params: &CallHierarchyPrepareParams,
) -> Option<CallHierarchyItem> {
    let annotated = state.annotated.as_ref()?;
    let ctxt = state.ctxt.as_ref()?;
    let source_contents = build_source_contents(state);

    let pos = &params.text_document_position_params;
    let symbol = position::symbol_under_cursor(
        annotated,
        ctxt,
        &pos.text_document.uri,
        pos.position,
        &state.position_encoding,
        &source_contents,
    )?;
    let resolved = symbol.resolved?;
    call_hierarchy::item_for_symbol(annotated, &resolved, &state.position_encoding)
}

/// Answer `callHierarchy/incomingCalls`. The client passes back the
/// item we returned from prepare; we decode its qualified name, look
/// up the POU, filter the reverse index to call sites grouped by
/// container POU.
fn handle_incoming_calls(state: &ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: CallHierarchyIncomingCallsParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed CallHierarchyIncomingCallsParams: {e}"),
            );
            return;
        }
    };

    let calls = incoming_calls_for(state, &params.item).unwrap_or_default();
    send_json_response(connection, req_id, serde_json::to_value(calls));
}

fn incoming_calls_for(
    state: &ServerState,
    item: &CallHierarchyItem,
) -> Option<Vec<lsp_types::CallHierarchyIncomingCall>> {
    let annotated = state.annotated.as_ref()?;
    let reverse_index = state.reverse_index.as_ref()?;
    let qualified_name = call_hierarchy::decode_item(item)?;
    let pou = annotated.index.find_pou(&qualified_name)?;
    Some(call_hierarchy::incoming_calls(
        annotated,
        reverse_index,
        pou.get_location(),
        &state.position_encoding,
    ))
}

/// Answer `callHierarchy/outgoingCalls`. Walks the POU's implementation
/// body, collects each call's callee, returns one entry per callee.
fn handle_outgoing_calls(state: &ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: CallHierarchyOutgoingCallsParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed CallHierarchyOutgoingCallsParams: {e}"),
            );
            return;
        }
    };

    let calls = outgoing_calls_for(state, &params.item).unwrap_or_default();
    send_json_response(connection, req_id, serde_json::to_value(calls));
}

fn outgoing_calls_for(
    state: &ServerState,
    item: &CallHierarchyItem,
) -> Option<Vec<lsp_types::CallHierarchyOutgoingCall>> {
    let annotated = state.annotated.as_ref()?;
    let qualified_name = call_hierarchy::decode_item(item)?;
    Some(call_hierarchy::outgoing_calls(annotated, &qualified_name, &state.position_encoding))
}

/// Answer `textDocument/prepareRename`. Returns the rename range +
/// placeholder name when the cursor sits on a renamable symbol;
/// null otherwise (Q6 strict).
fn handle_prepare_rename(state: &ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: TextDocumentPositionParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed TextDocumentPositionParams: {e}"),
            );
            return;
        }
    };

    let result = prepare_rename_for(state, &params);
    send_json_response(connection, req_id, serde_json::to_value(result));
}

fn prepare_rename_for(
    state: &ServerState,
    params: &TextDocumentPositionParams,
) -> Option<PrepareRenameResponse> {
    let annotated = state.annotated.as_ref()?;
    let ctxt = state.ctxt.as_ref()?;
    let source_contents = build_source_contents(state);
    let symbol = position::symbol_under_cursor(
        annotated,
        ctxt,
        &params.text_document.uri,
        params.position,
        &state.position_encoding,
        &source_contents,
    )?;
    rename::prepare_rename(&symbol, &state.position_encoding)
}

/// Answer `textDocument/rename`. Validates the new name + emits a
/// `WorkspaceEdit` listing the declaration site and every recorded
/// usage. On validation failure returns `InvalidRequest` so the editor
/// surfaces the message to the user.
fn handle_rename(state: &ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: RenameParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed RenameParams: {e}"),
            );
            return;
        }
    };

    let Some(annotated) = state.annotated.as_ref() else {
        send_error_response(
            connection,
            req_id,
            ErrorCode::InvalidRequest,
            "no successful compile yet".into(),
        );
        return;
    };
    let Some(ctxt) = state.ctxt.as_ref() else {
        send_error_response(connection, req_id, ErrorCode::InvalidRequest, "no compile context".into());
        return;
    };
    let Some(reverse_index) = state.reverse_index.as_ref() else {
        send_error_response(
            connection,
            req_id,
            ErrorCode::InvalidRequest,
            "no reverse index available".into(),
        );
        return;
    };
    let source_contents = build_source_contents(state);
    let Some(symbol) = position::symbol_under_cursor(
        annotated,
        ctxt,
        &params.text_document_position.text_document.uri,
        params.text_document_position.position,
        &state.position_encoding,
        &source_contents,
    ) else {
        send_error_response(
            connection,
            req_id,
            ErrorCode::InvalidRequest,
            "no symbol at this position".into(),
        );
        return;
    };

    match rename::rename_symbol(annotated, reverse_index, &symbol, &params.new_name, &state.position_encoding)
    {
        Ok(edit) => send_json_response(connection, req_id, serde_json::to_value(edit)),
        Err(msg) => send_error_response(connection, req_id, ErrorCode::InvalidRequest, msg),
    }
}

/// Answer `textDocument/completion`. Looks up the source for the cursor's
/// URI, fetches the cached `lex_with_trivia` tokens for that file, and
/// delegates context detection + item enumeration to
/// `completion::items_at`. Always returns a valid CompletionList; never
/// errors. Takes `&mut ServerState` because the token cache stores its
/// hash-indexed entries inside it.
fn handle_completion(state: &mut ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: CompletionParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed CompletionParams: {e}"),
            );
            return;
        }
    };

    let trigger = params.context.as_ref().map(|c| c.trigger_kind);
    let source_contents = build_source_contents(state);
    let result = match position::position_to_byte_offset(
        &params.text_document_position.text_document.uri,
        params.text_document_position.position,
        &state.position_encoding,
        &source_contents,
    ) {
        Some((path, byte_offset)) => {
            let path_str = path.to_string_lossy().into_owned();
            let source = source_contents.get(&path_str).cloned();
            match source {
                Some(src) => {
                    let tokens = state.token_cache.get_or_recompute(&path, &src);
                    completion::items_at(
                        tokens.as_slice(),
                        &src,
                        byte_offset,
                        trigger,
                        Some(path_str.as_str()),
                        state.annotated.as_ref(),
                    )
                }
                None => CompletionList { is_incomplete: false, items: vec![] },
            }
        }
        None => CompletionList { is_incomplete: false, items: vec![] },
    };

    send_json_response(connection, req_id, serde_json::to_value(result));
}

/// Lazy second-pass enrichment for `textDocument/completion` items.
/// VS Code / Helix call this when the user focuses an item; the
/// `data` field carries a `ResolveTag` set by `make_*_item` that
/// lets us look up the declaration and attach the docstring without
/// re-running the enumerator.
fn handle_completion_item_resolve(state: &mut ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let mut item: lsp_types::CompletionItem = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed CompletionItem: {e}"),
            );
            return;
        }
    };

    // Best-effort: docstring lookup needs the project + token cache;
    // if either is unavailable, ship the item back unchanged.
    let doc = item
        .data
        .as_ref()
        .and_then(|data| serde_json::from_value::<docstring::ResolveTag>(data.clone()).ok())
        .and_then(|tag| {
            let annotated = state.annotated.as_ref()?;
            let kind = tag.parsed_kind()?;
            let location = docstring::lookup_location(&annotated.index, &tag)?;
            // Split borrows: documents is read-only, token_cache is mutable.
            let documents = &state.documents;
            docstring::fetch(&mut state.token_cache, documents, &location, kind)
        });
    if let Some(body) = doc {
        item.documentation = Some(docstring::as_markdown_documentation(body));
    }

    send_json_response(connection, req_id, serde_json::to_value(item));
}

/// `textDocument/formatting`: re-emit the document with normalised
/// whitespace + indent + keyword case + tabular VAR / STRUCT
/// alignment. Returns a single whole-document `TextEdit` replacing
/// the buffer. Falls back to no-op (empty edit list) if the URI isn't
/// open in the document store — the LSP spec says we may return an
/// empty list when no formatting is available.
fn handle_formatting(state: &mut ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: DocumentFormattingParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed DocumentFormattingParams: {e}"),
            );
            return;
        }
    };

    let edits: Vec<TextEdit> = match state.documents.get(&params.text_document.uri) {
        Some(buf) => {
            let formatted = formatter::format_document(&buf.content);
            if formatted == buf.content {
                vec![]
            } else {
                // Replace the entire document with one TextEdit. The
                // end position is computed from the source's line count —
                // LSP wants a `Range`, and using {u32::MAX, 0} is a
                // common idiom for "to end of file" but spec-pedantic
                // clients prefer the real bound.
                let (end_line, end_char) = end_position(&buf.content);
                vec![TextEdit {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: end_line, character: end_char },
                    },
                    new_text: formatted,
                }]
            }
        }
        None => vec![],
    };

    send_json_response(connection, req_id, serde_json::to_value(edits));
}

/// LSP Position for the end of the document (line is 0-based; character
/// counts utf-16 code units in our negotiated encoding, but for ASCII
/// content — which formatter output always is — utf-8 byte count and
/// utf-16 unit count are equal).
fn end_position(source: &str) -> (u32, u32) {
    let mut line: u32 = 0;
    let mut last_line_chars: u32 = 0;
    for ch in source.chars() {
        if ch == '\n' {
            line += 1;
            last_line_chars = 0;
        } else {
            last_line_chars += 1;
        }
    }
    (line, last_line_chars)
}

/// `textDocument/semanticTokens/full`: emit token-kind tags for every
/// identifier in the document. Used by the editor's theme to colour
/// `foo` differently depending on whether it resolves to a function /
/// variable / type / etc. — refinement on top of the TextMate grammar
/// that serhioromano's vscode-st provides.
fn handle_semantic_tokens_full(state: &mut ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: lsp_types::SemanticTokensParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed SemanticTokensParams: {e}"),
            );
            return;
        }
    };

    // Need annotated project + the file's source to produce tokens.
    // Both are best-effort: if the compile hasn't attached yet, return
    // empty (the editor will request again on the next edit).
    let result: lsp_types::SemanticTokensResult =
        match (state.annotated.as_ref(), state.documents.get(&params.text_document.uri)) {
            (Some(annotated), Some(buf)) => {
                let Some(path) = project::file_uri_to_path(&params.text_document.uri) else {
                    send_json_response(
                        connection,
                        req_id,
                        serde_json::to_value(lsp_types::SemanticTokens::default()),
                    );
                    return;
                };
                let tokens = semantic_tokens::semantic_tokens_for_file(
                    annotated,
                    &path,
                    &buf.content,
                    &state.position_encoding,
                );
                lsp_types::SemanticTokensResult::Tokens(tokens)
            }
            _ => lsp_types::SemanticTokensResult::Tokens(lsp_types::SemanticTokens::default()),
        };

    send_json_response(connection, req_id, serde_json::to_value(result));
}

/// `textDocument/inlayHint`: emit parameter-name hints for positional
/// call arguments. Doesn't honour the request's `range` field for the
/// POC — we always return hints for the full file. The editor will
/// clip them to the visible viewport client-side.
fn handle_inlay_hint(state: &mut ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: lsp_types::InlayHintParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed InlayHintParams: {e}"),
            );
            return;
        }
    };

    let result: Vec<lsp_types::InlayHint> =
        match (state.annotated.as_ref(), state.documents.get(&params.text_document.uri)) {
            (Some(annotated), Some(buf)) => {
                let Some(path) = project::file_uri_to_path(&params.text_document.uri) else {
                    send_json_response(
                        connection,
                        req_id,
                        serde_json::to_value(Vec::<lsp_types::InlayHint>::new()),
                    );
                    return;
                };
                inlay_hints::inlay_hints_for_file(annotated, &path, &buf.content, &state.position_encoding)
            }
            _ => Vec::new(),
        };

    send_json_response(connection, req_id, serde_json::to_value(result));
}

/// `textDocument/codeLens`: interface implementation lenses (H3).
/// Computes lazily-resolveable lenses for the entire file — we don't
/// implement codeLens/resolve since each lens already carries its
/// title and command in the initial response.
fn handle_code_lens(state: &mut ServerState, connection: &Connection, req: Request) {
    let req_id = req.id.clone();
    let params: lsp_types::CodeLensParams = match serde_json::from_value(req.params) {
        Ok(p) => p,
        Err(e) => {
            send_error_response(
                connection,
                req_id,
                ErrorCode::InvalidParams,
                format!("malformed CodeLensParams: {e}"),
            );
            return;
        }
    };

    let result: Vec<lsp_types::CodeLens> = match state.annotated.as_ref() {
        Some(annotated) => {
            let Some(path) = project::file_uri_to_path(&params.text_document.uri) else {
                send_json_response(
                    connection,
                    req_id,
                    serde_json::to_value(Vec::<lsp_types::CodeLens>::new()),
                );
                return;
            };
            code_lens::code_lenses_for_file(annotated, &path, &state.position_encoding)
        }
        None => Vec::new(),
    };

    send_json_response(connection, req_id, serde_json::to_value(result));
}

fn send_json_response(
    connection: &Connection,
    req_id: RequestId,
    body: serde_json::Result<serde_json::Value>,
) {
    let response = match body {
        Ok(value) => Response { id: req_id, result: Some(value), error: None },
        Err(e) => {
            log::error!("failed to serialise response: {e}");
            return;
        }
    };
    if let Err(e) = connection.sender.send(Message::Response(response)) {
        log::error!("failed to send response: {e}");
    }
}

/// Convert an internal `SourceLocation` into an LSP `Location`.
/// Returns `None` when the location lacks a usable file path or range
/// (synthetic / `<internal>` / undefined).
fn source_location_to_lsp(
    location: &plc_source::source_location::SourceLocation,
    encoding: &PositionEncodingKind,
) -> Option<Location> {
    let path = location.get_file_name()?;
    let uri = diagnostics::path_to_uri(path)?;
    let range = diagnostics::code_span_to_range(location.get_span(), encoding, None)?;
    Some(Location { uri, range })
}

fn send_goto_response(
    connection: &Connection,
    req_id: RequestId,
    body: serde_json::Result<serde_json::Value>,
) {
    let response = match body {
        Ok(value) => Response { id: req_id, result: Some(value), error: None },
        Err(e) => {
            log::error!("goto response failed to serialise: {e}");
            return;
        }
    };
    if let Err(e) = connection.sender.send(Message::Response(response)) {
        log::error!("failed to send goto response: {e}");
    }
}

fn build_source_contents(state: &ServerState) -> HashMap<String, String> {
    state
        .documents
        .iter()
        .filter_map(|(uri, buf)| {
            project::file_uri_to_path(uri).map(|p| (p.to_string_lossy().into_owned(), buf.content.clone()))
        })
        .collect()
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
        "textDocument/didOpen" => handle_did_open(state, compile_tx, notif),
        "textDocument/didChange" => handle_did_change(state, compile_tx, notif),
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

fn handle_did_open(
    state: &mut ServerState,
    compile_tx: &Sender<compile::CompileRequest>,
    notif: Notification,
) {
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
    // Trigger a recompile so the buffer (which may differ from disk for
    // freshly-opened editors that show unsaved state) feeds the next
    // query. Latest-wins scheduling cancels any in-flight compile.
    trigger_compile(state, compile_tx);
}

fn handle_did_change(
    state: &mut ServerState,
    compile_tx: &Sender<compile::CompileRequest>,
    notif: Notification,
) {
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
    // Trigger a recompile against the latest buffer so completion /
    // hover / find-references / diagnostics see fresh state on the next
    // query. Latest-wins scheduling cancels any in-flight compile;
    // back-to-back keystrokes only run the final compile to completion.
    //
    // P13.7 placeholder: the original design (post-13 follow-up item 14)
    // was "trigger a fresh compile on explicit user actions" — completion
    // / rename / call-hierarchy block until the compile finishes. That
    // requires response-queuing on the main thread; deferred until after
    // phase-13 lands. For now we accept the per-keystroke cost as the
    // cancellation token keeps it manageable.
    trigger_compile(state, compile_tx);
}

fn handle_did_save(
    state: &mut ServerState,
    compile_tx: &Sender<compile::CompileRequest>,
    notif: Notification,
) {
    // Clear the dirty flag on the saved buffer so a subsequent
    // watched-files event for the same URI knows the buffer matches disk.
    // See decisions log D7.
    if let Ok(params) = serde_json::from_value::<lsp_types::DidSaveTextDocumentParams>(notif.params) {
        state.documents.mark_saved(&params.text_document.uri);
    }
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
    if let Some(path) = project::file_uri_to_path(&params.text_document.uri) {
        state.token_cache.invalidate(&path);
    }
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

        // Project source. We always recompile on a watched-files event,
        // even when the buffer is open. The LSP can't reliably tell an
        // editor-driven didChange (user typing) from an external-reload
        // didChange (the client refreshing a clean buffer to match disk),
        // so the dirty-aware variant tried earlier misfires whenever the
        // client emits didChange before the watched-files event. The
        // cost is at most one extra compile per user save (when both
        // didSave and the OS-level watcher fire); the trade-off lets
        // external edits always rebuild. See decisions log D7.
        if state.documents.get(&event.uri).is_some() {
            log::debug!("watched-files: change to open buffer {:?}; recompiling", event.uri);
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

/// Ping the client to drop its cached responses for views that depend
/// on `state.annotated`. Sent after each compile attaches a fresh
/// AnnotatedProject; without this the editor keeps showing the stale
/// (or empty) results from before annotation finished.
///
/// Each refresh is a server-initiated request. We don't wait for the
/// response — the client will send fresh feature requests on its own
/// schedule. Only fire to clients that advertised refresh support;
/// older clients ignore unknown methods, but sending the request still
/// burns an id roundtrip.
fn refresh_annotated_views(state: &mut ServerState, connection: &Connection) {
    if state.supports_semantic_tokens_refresh {
        send_workspace_refresh(state, connection, "workspace/semanticTokens/refresh");
    }
    if state.supports_inlay_hint_refresh {
        send_workspace_refresh(state, connection, "workspace/inlayHint/refresh");
    }
    if state.supports_code_lens_refresh {
        send_workspace_refresh(state, connection, "workspace/codeLens/refresh");
    }
}

fn send_workspace_refresh(state: &mut ServerState, connection: &Connection, method: &str) {
    let id = next_request_id(state);
    let request = Request { id, method: method.to_string(), params: serde_json::Value::Null };
    if let Err(e) = connection.sender.send(Message::Request(request)) {
        log::warn!("failed to send {method}: {e}");
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
/// If a compile is already in flight, cancel it (so the worker can bail
/// at the next pipeline check point) and mark the state dirty so the
/// next compile fires when the in-flight one returns its
/// `CompileOutcome::Cancelled`. See phase-6 Q3.
fn trigger_compile(state: &mut ServerState, compile_tx: &Sender<compile::CompileRequest>) {
    if state.compile_pending {
        if let Some(token) = &state.active_compile_token {
            token.cancel();
            log::debug!("compile already pending; cancelling in-flight + marking dirty");
        } else {
            log::debug!("compile already pending (no token?); marking dirty");
        }
        state.compile_dirty = true;
        return;
    }
    let cancellation = CancellationToken::new();
    let snapshot = build_snapshot(state, cancellation.clone());
    if compile_tx.send(compile::CompileRequest { snapshot }).is_err() {
        log::error!("compile worker channel closed; cannot send request");
        return;
    }
    state.active_compile_token = Some(cancellation);
    state.compile_pending = true;
    state.compile_dirty = false;
}

fn handle_compile_outcome(
    state: &mut ServerState,
    connection: &Connection,
    compile_tx: &Sender<compile::CompileRequest>,
    outcome: compile::CompileOutcome,
) {
    state.compile_pending = false;
    state.active_compile_token = None;

    match outcome {
        compile::CompileOutcome::Cancelled => {
            log::debug!("compile cancelled; skipping publish");
            // No diagnostics to publish; the next trigger (queued via
            // compile_dirty by whoever cancelled us) will produce fresh
            // results from the latest state.
        }
        compile::CompileOutcome::Done(mut result) => {
            if let Some(err) = &result.error {
                log::error!("compile pipeline error: {err}");
            }
            log::debug!("compile produced {} diagnostics", result.diagnostics.len());
            // Cache the per-URI outline before consuming the result for
            // diagnostics publishing. Replace only on a successfully
            // attached compile (`document_symbols` is empty when the
            // pipeline didn't reach annotate); failed compiles preserve
            // the last-good outline so the user keeps seeing structure
            // while they're fixing a broken file.
            if !result.document_symbols.is_empty() {
                let mut by_uri: HashMap<Uri, Vec<DocumentSymbol>> = HashMap::new();
                for (path, syms) in &result.document_symbols {
                    if let Some(uri) = diagnostics::path_to_uri(path) {
                        by_uri.insert(uri, syms.clone());
                    }
                }
                state.document_symbols = by_uri;
            }
            // Owned hand-off: replace the cached project + context only
            // when the worker actually produced them. `None` here means
            // the compile didn't reach a queryable state — the previous
            // attachment continues to serve hover / goto / references.
            let annotated_attached = result.annotated.is_some();
            if let Some(annotated) = result.annotated.take() {
                state.annotated = Some(annotated);
            }
            if let Some(ctxt) = result.ctxt.take() {
                state.ctxt = Some(ctxt);
            }
            if let Some(ri) = result.reverse_index.take() {
                state.reverse_index = Some(ri);
            }
            publish_diagnostics(state, connection, *result);
            // Ping the client to re-request the views that depend on
            // annotated. Without this the editor caches the empty
            // pre-compile response and only refreshes on the next edit.
            if annotated_attached {
                refresh_annotated_views(state, connection);
            }
        }
    }

    if state.compile_dirty {
        log::debug!("re-firing compile due to dirty state");
        state.compile_dirty = false;
        trigger_compile(state, compile_tx);
    }
}

fn publish_diagnostics(state: &mut ServerState, connection: &Connection, result: compile::CompileResult) {
    let grouped = diagnostics::map_collected(
        result.diagnostics,
        &result.file_paths,
        &result.position_encoding,
        &result.source_contents,
    );
    let new_uris: HashSet<Uri> = grouped.keys().cloned().collect();

    // Build the "expected to be empty" set: every project URI the worker
    // touched that doesn't have diagnostics this round. We track this via
    // `result.project_paths` so a fresh LSP session can clear stale
    // diagnostics the editor was still showing from a *previous* session
    // — `state.published_uris` is empty on restart and can't drive that
    // by itself (L4 fix).
    let project_uris: HashSet<Uri> =
        result.project_paths.iter().filter_map(|p| diagnostics::path_to_uri(p.to_str()?)).collect();
    let mut clear_set: HashSet<Uri> = state.published_uris.union(&project_uris).cloned().collect();
    for uri in &new_uris {
        clear_set.remove(uri);
    }
    for stale in &clear_set {
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

fn build_snapshot(state: &ServerState, cancellation: CancellationToken) -> compile::CompileSnapshot {
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
        cancellation,
    }
}
