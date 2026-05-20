//! Compile worker: long-lived thread that runs the compile pipeline
//! on demand, posting results back to the main thread.
//!
//! The "latest wins" scheduling lives on the main thread (see
//! `lib.rs`); this module just defines the message types and the
//! worker loop. Step 3.4 fills in the actual pipeline call inside
//! `run_compile`.

use std::collections::HashMap;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

use crossbeam_channel::{unbounded, Receiver, Sender};
use lsp_types::{DocumentSymbol, PositionEncodingKind};
use plc_diagnostics::cancellation::CancellationToken;
use plc_diagnostics::reporter::ResolvedDiagnostics;
use plc_driver::pipelines::AnnotatedProject;
use plc_index::GlobalContext;
use rustc_hash::FxHashMap;

use crate::outline;
use crate::reverse_index::ReverseIndex;

/// Snapshot of the inputs a compile needs. Built on the main thread and
/// shipped to the worker so the worker thread doesn't reach into
/// `ServerState`.
pub struct CompileSnapshot {
    pub plc_config_path: Option<PathBuf>,
    pub workspace_root: Option<PathBuf>,
    /// Map of project source paths → buffer contents for every open
    /// editor buffer. Files not present here are read from disk by the
    /// worker.
    pub open_buffers: HashMap<PathBuf, String>,
    /// Negotiated position encoding. Phase 4 uses it when mapping
    /// diagnostic ranges back to LSP coordinates.
    pub position_encoding: PositionEncodingKind,
    /// Per-compile cancellation token. The main thread keeps a clone
    /// on `ServerState.active_compile_token`; when a new trigger
    /// arrives mid-compile, the main thread calls `.cancel()` on its
    /// clone and the worker sees the flag flip at the next pipeline
    /// check point. See decisions log Q3.
    pub cancellation: CancellationToken,
}

pub struct CompileRequest {
    pub snapshot: CompileSnapshot,
}

/// What the worker sends back. `Cancelled` is a distinct variant
/// (rather than a flag on `CompileResult`) so the main thread can
/// match on it and skip the publish path without first inspecting a
/// half-populated result.
///
/// `CompileResult` is boxed: it now carries the full `AnnotatedProject`
/// (~kilobytes) so the `Done` variant would otherwise dwarf `Cancelled`
/// and trigger clippy's `large_enum_variant`.
pub enum CompileOutcome {
    Done(Box<CompileResult>),
    Cancelled,
}

pub struct CompileResult {
    pub diagnostics: Vec<ResolvedDiagnostics>,
    pub file_paths: FxHashMap<usize, String>,
    /// Top-level pipeline error (e.g. parse short-circuit), stringified
    /// here because the underlying `Diagnostic` type isn't `Send`-safe in
    /// every form and we don't need the structure on the main thread.
    pub error: Option<String>,
    /// Source content per file path, used by the diagnostics mapper to
    /// convert byte offsets to utf-16 code units when the negotiated
    /// position encoding is UTF-16. Populated only in that case to avoid
    /// the memory churn on the utf-8 happy path (helix, nvim). See
    /// decisions log D4.
    pub source_contents: HashMap<String, String>,
    /// Encoding the worker had at the time of compile; the mapper uses
    /// this to decide whether to walk the source for utf-16 conversion.
    pub position_encoding: PositionEncodingKind,
    /// Pre-computed `textDocument/documentSymbol` outline per source
    /// file path. Pre-computing on the worker (rather than holding the
    /// whole `AnnotatedProject` on the main thread) keeps the shipping
    /// surface small and `Send`-clean. Empty when the pipeline failed
    /// before annotate completed.
    pub document_symbols: HashMap<String, Vec<DocumentSymbol>>,
    /// The annotated project itself. Owned hand-off (not `Arc`): the
    /// main thread takes ownership and answers cursor-parameterised
    /// queries (hover, goto-def, references) by borrowing it. `Some`
    /// iff the worker's `annotate` stage returned `Ok` — validation
    /// severity does not gate the attach. `None` on parse fatal,
    /// lowering failure, cancellation, or any other pipeline error.
    pub annotated: Option<AnnotatedProject>,
    /// The compile-time `GlobalContext` paired with `annotated`.
    /// Cloned out of the pipeline before drop so the main thread can
    /// call `ctxt.slice(&location)` to recover the user's literal
    /// source text for display. `None` whenever `annotated` is.
    pub ctxt: Option<GlobalContext>,
    /// Pre-built reverse index (declaration → use sites) for
    /// `textDocument/references`. Built on the worker by walking
    /// `annotated.units` after annotation completes. `None` whenever
    /// `annotated` is.
    pub reverse_index: Option<ReverseIndex>,
}

/// Handles to the compile worker. Call `join` to stop it cleanly; if
/// dropped, `compile_tx` closes naturally and the worker exits in the
/// background.
pub struct CompileWorker {
    pub compile_tx: Sender<CompileRequest>,
    pub result_rx: Receiver<CompileOutcome>,
    handle: JoinHandle<()>,
}

impl CompileWorker {
    pub fn spawn() -> Self {
        let (compile_tx, compile_rx) = unbounded::<CompileRequest>();
        let (result_tx, result_rx) = unbounded::<CompileOutcome>();

        let handle = thread::spawn(move || worker_loop(compile_rx, result_tx));

        CompileWorker { compile_tx, result_rx, handle }
    }

    /// Stop the worker and wait for it to finish. Dropping `compile_tx`
    /// closes the inbound channel, which makes the worker loop exit.
    pub fn join(self) {
        drop(self.compile_tx);
        if let Err(panic) = self.handle.join() {
            log::error!("compile worker thread panicked: {panic:?}");
        }
    }
}

fn worker_loop(compile_rx: Receiver<CompileRequest>, result_tx: Sender<CompileOutcome>) {
    log::info!("compile worker started");
    while let Ok(req) = compile_rx.recv() {
        let outcome = run_compile(req);
        if result_tx.send(outcome).is_err() {
            // Main thread is gone; we should exit too.
            break;
        }
    }
    log::info!("compile worker exiting");
}

fn run_compile(req: CompileRequest) -> CompileOutcome {
    let snapshot = req.snapshot;
    let position_encoding = snapshot.position_encoding.clone();
    let cancellation = snapshot.cancellation.clone();

    // Early bail: the main thread may have cancelled before the
    // worker even pulled the request off the channel.
    if cancellation.is_cancelled() {
        return CompileOutcome::Cancelled;
    }

    let empty_result = || CompileResult {
        diagnostics: Vec::new(),
        file_paths: FxHashMap::default(),
        error: None,
        source_contents: HashMap::new(),
        position_encoding: position_encoding.clone(),
        document_symbols: HashMap::new(),
        annotated: None,
        ctxt: None,
        reverse_index: None,
    };

    let Some(config_path) = snapshot.plc_config_path.as_deref() else {
        log::warn!("compile: no plc.json available; skipping");
        return CompileOutcome::Done(Box::new(empty_result()));
    };

    log::info!("compile: starting with plc.json={config_path:?}");

    let project = match plc_project::project::Project::from_config(config_path) {
        Ok(p) => p,
        Err(e) => {
            return CompileOutcome::Done(Box::new(CompileResult {
                error: Some(format!("failed to load plc.json at {config_path:?}: {e}")),
                ..empty_result()
            }));
        }
    };

    let sources = build_sources(project.get_sources(), &snapshot.open_buffers);
    log::debug!("compile: built {} sources", sources.len());

    // For utf-16 we need the source text on the main thread to convert
    // byte-offset positions into utf-16 code units. For utf-8 we'd never
    // read it, so don't pay the memory cost.
    let source_contents: HashMap<String, String> = if position_encoding == PositionEncodingKind::UTF16 {
        sources
            .iter()
            .filter_map(|s| s.path.as_ref().map(|p| (p.to_string_lossy().into_owned(), s.source.clone())))
            .collect()
    } else {
        HashMap::new()
    };

    let reporter = plc_diagnostics::reporter::lsp::LspReporter::new();
    let handle = reporter.clone();
    let diagnostician = plc_diagnostics::diagnostician::Diagnostician::with_reporter(Box::new(reporter));

    let mut pipeline = match plc_driver::pipelines::BuildPipeline::from_sources(
        project.get_name(),
        sources,
        diagnostician,
    ) {
        Ok(p) => p,
        Err(e) => {
            return CompileOutcome::Done(Box::new(CompileResult {
                diagnostics: handle.take_collected(),
                file_paths: handle.file_paths(),
                error: Some(format!("from_sources failed: {e:?}")),
                source_contents,
                position_encoding,
                document_symbols: HashMap::new(),
                annotated: None,
                ctxt: None,
                reverse_index: None,
            }));
        }
    };

    // Install our per-compile cancellation token on the pipeline's
    // GlobalContext. Stages call `ctxt.cancellation().check()?` at
    // boundaries and short-circuit if the main thread flips the flag.
    pipeline.context.set_cancellation(cancellation.clone());

    pipeline.register_default_mut_participants();

    let stage_result = run_stages(&mut pipeline);
    let pipeline_error = stage_result.as_ref().err().map(|e| format!("{e:?}"));

    // Post-check: did the pipeline bail because we cancelled it? The
    // outcome enum from Q2 says cancelled → Cancelled variant; the
    // pipeline_error in that case would be a `Diagnostic::cancelled()`
    // sentinel, but rather than inspect the error we trust the token —
    // there's no race here (only the main thread cancels, only the
    // worker is reading right now).
    if cancellation.is_cancelled() {
        return CompileOutcome::Cancelled;
    }

    // Pre-compute the per-file outline now while the AnnotatedProject is
    // in scope. The outline is parameter-free per file, so we ship the
    // derived map instead of recomputing on each `documentSymbol`
    // request.
    let document_symbols = stage_result
        .as_ref()
        .ok()
        .map(|annotated| outline::build_outline_map(annotated, &position_encoding, &source_contents))
        .unwrap_or_default();

    // Attach iff `annotate` returned Ok — pair the project with a clone
    // of the pipeline's GlobalContext so the main thread can call
    // `ctxt.slice(&location)` for the source-text rule applied across
    // hover / goto / refs. The reverse-index is built here while the
    // annotated project is still in scope — same `Some`/`None` pairing.
    let (annotated, ctxt, reverse_index) = match stage_result {
        Ok(annotated) => {
            let ctxt = pipeline.context.clone();
            let reverse_index = ReverseIndex::build(&annotated, &ctxt);
            (Some(annotated), Some(ctxt), Some(reverse_index))
        }
        Err(_) => (None, None, None),
    };

    CompileOutcome::Done(Box::new(CompileResult {
        diagnostics: handle.take_collected(),
        file_paths: handle.file_paths(),
        error: pipeline_error,
        source_contents,
        position_encoding,
        document_symbols,
        annotated,
        ctxt,
        reverse_index,
    }))
}

fn run_stages(
    pipeline: &mut plc_driver::pipelines::BuildPipeline<plc_source::SourceCode>,
) -> Result<plc_driver::pipelines::AnnotatedProject, plc_diagnostics::diagnostics::Diagnostic> {
    use plc_driver::pipelines::Pipeline as _;
    let parsed = pipeline.parse()?;
    let indexed = pipeline.index(parsed)?;
    let annotated = pipeline.annotate(indexed)?;
    // `validate` takes &self; ignore its Result (existing carve-out
    // from phase 3 — validator failures don't gate the publish path).
    let _ = annotated.validate(&pipeline.context, &mut pipeline.diagnostician);
    Ok(annotated)
}

fn build_sources(
    project_paths: &[PathBuf],
    open_buffers: &HashMap<PathBuf, String>,
) -> Vec<plc_source::SourceCode> {
    project_paths
        .iter()
        .filter_map(|path| {
            let content = if let Some(buf) = open_buffers.get(path) {
                buf.clone()
            } else {
                match std::fs::read_to_string(path) {
                    Ok(s) => s,
                    Err(e) => {
                        log::warn!("compile: failed to read {path:?}: {e}");
                        return None;
                    }
                }
            };
            Some(plc_source::SourceCode { source: content, path: Some(path.clone()) })
        })
        .collect()
}
