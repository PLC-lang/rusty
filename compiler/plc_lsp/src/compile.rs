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
use lsp_types::PositionEncodingKind;
use plc_diagnostics::reporter::ResolvedDiagnostics;
use rustc_hash::FxHashMap;

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
}

pub struct CompileRequest {
    pub snapshot: CompileSnapshot,
}

pub struct CompileResult {
    pub diagnostics: Vec<ResolvedDiagnostics>,
    pub file_paths: FxHashMap<usize, String>,
    /// Top-level pipeline error (e.g. parse short-circuit), stringified
    /// here because the underlying `Diagnostic` type isn't `Send`-safe in
    /// every form and we don't need the structure on the main thread.
    pub error: Option<String>,
}

/// Handles to the compile worker. Call `join` to stop it cleanly; if
/// dropped, `compile_tx` closes naturally and the worker exits in the
/// background.
pub struct CompileWorker {
    pub compile_tx: Sender<CompileRequest>,
    pub result_rx: Receiver<CompileResult>,
    handle: JoinHandle<()>,
}

impl CompileWorker {
    pub fn spawn() -> Self {
        let (compile_tx, compile_rx) = unbounded::<CompileRequest>();
        let (result_tx, result_rx) = unbounded::<CompileResult>();

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

fn worker_loop(compile_rx: Receiver<CompileRequest>, result_tx: Sender<CompileResult>) {
    log::info!("compile worker started");
    while let Ok(req) = compile_rx.recv() {
        let result = run_compile(req);
        if result_tx.send(result).is_err() {
            // Main thread is gone; we should exit too.
            break;
        }
    }
    log::info!("compile worker exiting");
}

fn run_compile(req: CompileRequest) -> CompileResult {
    let snapshot = req.snapshot;

    let Some(config_path) = snapshot.plc_config_path.as_deref() else {
        log::warn!("compile: no plc.json available; skipping");
        return CompileResult { diagnostics: Vec::new(), file_paths: FxHashMap::default(), error: None };
    };

    log::info!("compile: starting with plc.json={config_path:?}");

    let project = match plc_project::project::Project::from_config(config_path) {
        Ok(p) => p,
        Err(e) => {
            return CompileResult {
                diagnostics: Vec::new(),
                file_paths: FxHashMap::default(),
                error: Some(format!("failed to load plc.json at {config_path:?}: {e}")),
            };
        }
    };

    let sources = build_sources(project.get_sources(), &snapshot.open_buffers);
    log::debug!("compile: built {} sources", sources.len());

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
            return CompileResult {
                diagnostics: handle.take_collected(),
                file_paths: handle.file_paths(),
                error: Some(format!("from_sources failed: {e:?}")),
            };
        }
    };

    pipeline.register_default_mut_participants();

    let pipeline_error = run_stages(&mut pipeline).err().map(|e| format!("{e:?}"));

    CompileResult {
        diagnostics: handle.take_collected(),
        file_paths: handle.file_paths(),
        error: pipeline_error,
    }
}

fn run_stages(
    pipeline: &mut plc_driver::pipelines::BuildPipeline<plc_source::SourceCode>,
) -> Result<(), plc_diagnostics::diagnostics::Diagnostic> {
    use plc_driver::pipelines::Pipeline as _;
    let parsed = pipeline.parse()?;
    let indexed = pipeline.index(parsed)?;
    let annotated = pipeline.annotate(indexed)?;
    let _ = annotated.validate(&pipeline.context, &mut pipeline.diagnostician);
    Ok(())
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
