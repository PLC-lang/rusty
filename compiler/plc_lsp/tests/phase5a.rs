//! Phase 5a end-to-end tests: `workspace/didChangeWatchedFiles`
//! triggers a recompile, and `workspace/executeCommand` for
//! `rusty.reparseProject` does the same on demand.

use std::fs;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::{
    DidChangeWatchedFilesParams, ExecuteCommandParams, FileChangeType, FileEvent, InitializeParams,
    PublishDiagnosticsParams, WorkspaceFolder,
};
use serde_json::json;
use tempfile::TempDir;

fn spawn_server() -> (Connection, JoinHandle<anyhow::Result<()>>) {
    let (server_conn, client_conn) = Connection::memory();
    let handle = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));
    (client_conn, handle)
}

fn handshake(client: &Connection, workspace: &TempDir) {
    let workspace_uri: lsp_types::Uri = format!("file://{}", workspace.path().display()).parse().unwrap();
    let params = InitializeParams {
        workspace_folders: Some(vec![WorkspaceFolder { uri: workspace_uri, name: "test".to_string() }]),
        ..Default::default()
    };
    client
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(1),
            method: "initialize".to_string(),
            params: serde_json::to_value(params).unwrap(),
        }))
        .unwrap();

    // Drain initialize response.
    drain_until_response(client, &RequestId::from(1));

    client
        .sender
        .send(Message::Notification(Notification { method: "initialized".to_string(), params: json!({}) }))
        .unwrap();
}

fn shutdown_and_join(client: &Connection, handle: JoinHandle<anyhow::Result<()>>) {
    client
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(999),
            method: "shutdown".to_string(),
            params: json!(null),
        }))
        .unwrap();
    drain_until_response(client, &RequestId::from(999));
    client
        .sender
        .send(Message::Notification(Notification { method: "exit".to_string(), params: json!(null) }))
        .unwrap();
    handle.join().expect("server thread panicked").expect("server returned error");
}

fn drain_until_response(client: &Connection, expected_id: &RequestId) -> lsp_server::Response {
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    loop {
        let remaining = deadline.checked_duration_since(std::time::Instant::now()).unwrap_or(Duration::ZERO);
        match client.receiver.recv_timeout(remaining) {
            Ok(Message::Response(r)) if r.id == *expected_id => return r,
            Ok(_) => {} // ignore other messages
            Err(_) => panic!("timed out waiting for response id={expected_id:?}"),
        }
    }
}

fn wait_for_publish(client: &Connection, timeout: Duration) -> PublishDiagnosticsParams {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        let remaining = deadline.checked_duration_since(std::time::Instant::now()).unwrap_or(Duration::ZERO);
        match client.receiver.recv_timeout(remaining) {
            Ok(Message::Notification(n)) if n.method == "textDocument/publishDiagnostics" => {
                return serde_json::from_value(n.params).unwrap();
            }
            Ok(_) => {}
            Err(_) => panic!("timed out waiting for publishDiagnostics"),
        }
    }
}

#[test]
fn watched_file_change_triggers_a_recompile() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("plc.json"), r#"{ "name": "phase5a", "files": ["*.st"] }"#).unwrap();
    // Start with the file already broken so the initial compile produces a
    // publishDiagnostics we can synchronise on.
    fs::write(tmp.path().join("main.st"), "PROGRAM main\n  initial_undefined := 1;\nEND_PROGRAM\n").unwrap();

    let (client, handle) = spawn_server();
    handshake(&client, &tmp);

    // Wait for the initial publish so we know the first compile has finished.
    let _initial = wait_for_publish(&client, Duration::from_secs(30));

    // Replace the file's contents with a *different* broken state, then notify.
    fs::write(tmp.path().join("main.st"), "PROGRAM main\n  changed_undefined := 1;\nEND_PROGRAM\n").unwrap();
    let main_uri: lsp_types::Uri =
        format!("file://{}", tmp.path().join("main.st").display()).parse().unwrap();
    let params = DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: main_uri.clone(), typ: FileChangeType::CHANGED }],
    };
    client
        .sender
        .send(Message::Notification(Notification {
            method: "workspace/didChangeWatchedFiles".to_string(),
            params: serde_json::to_value(params).unwrap(),
        }))
        .unwrap();

    let publish = wait_for_publish(&client, Duration::from_secs(30));
    assert!(
        publish.uri.as_str().ends_with("main.st"),
        "expected URI ending with main.st, got {:?}",
        publish.uri
    );
    assert!(
        !publish.diagnostics.is_empty(),
        "expected diagnostics after the file change introduced an error"
    );

    shutdown_and_join(&client, handle);
}

#[test]
fn execute_command_reparse_project_triggers_recompile() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("plc.json"), r#"{ "name": "phase5a_cmd", "files": ["*.st"] }"#).unwrap();
    fs::write(tmp.path().join("main.st"), "PROGRAM main\n  undefined_variable := 1;\nEND_PROGRAM\n").unwrap();

    let (client, handle) = spawn_server();
    handshake(&client, &tmp);

    // Initial compile produces diagnostics from the broken file.
    let initial = wait_for_publish(&client, Duration::from_secs(30));
    assert!(!initial.diagnostics.is_empty(), "initial compile should produce diagnostics");

    // Issue rusty.reparseProject via workspace/executeCommand.
    let exec_id = RequestId::from(42);
    let params = ExecuteCommandParams {
        command: "rusty.reparseProject".to_string(),
        arguments: vec![],
        work_done_progress_params: Default::default(),
    };
    client
        .sender
        .send(Message::Request(Request {
            id: exec_id.clone(),
            method: "workspace/executeCommand".to_string(),
            params: serde_json::to_value(params).unwrap(),
        }))
        .unwrap();

    // We expect (in some order): a publishDiagnostics with the same errors,
    // and a Response to the executeCommand.
    let mut got_publish = false;
    let mut got_response = false;
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    while !(got_publish && got_response) {
        let remaining = deadline.checked_duration_since(std::time::Instant::now()).unwrap_or(Duration::ZERO);
        let msg = client.receiver.recv_timeout(remaining).expect("timed out waiting for reparse outcome");
        match msg {
            Message::Response(r) if r.id == exec_id => {
                assert!(r.error.is_none(), "reparse command should not error: {:?}", r.error);
                got_response = true;
            }
            Message::Notification(n) if n.method == "textDocument/publishDiagnostics" => {
                let p: PublishDiagnosticsParams = serde_json::from_value(n.params).unwrap();
                assert!(
                    !p.diagnostics.is_empty(),
                    "re-publish should still carry diagnostics from the broken file"
                );
                got_publish = true;
            }
            _ => {} // drain other messages
        }
    }

    shutdown_and_join(&client, handle);
}
