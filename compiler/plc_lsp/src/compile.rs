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

fn run_compile(_req: CompileRequest) -> CompileResult {
    // Phase 3.4 fills this in.
    log::debug!("compile worker: run_compile invoked (no-op; pipeline lands in phase 3.4)");
    CompileResult { diagnostics: Vec::new(), file_paths: FxHashMap::default(), error: None }
}
