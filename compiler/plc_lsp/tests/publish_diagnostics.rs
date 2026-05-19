//! End-to-end test for phase 4: a minimal tempdir project with a broken
//! source file produces a `textDocument/publishDiagnostics` notification
//! after the initial compile fires.

use std::fs;
use std::thread;
use std::time::Duration;

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::{InitializeParams, PublishDiagnosticsParams, WorkspaceFolder};
use serde_json::json;
use tempfile::tempdir;

#[test]
fn publishes_diagnostics_after_initial_compile() {
    let tmp = tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{ "name": "diag_test", "files": ["*.st"] }"#).unwrap();
    // Deliberately broken — references an undefined identifier so the
    // validator produces a diagnostic.
    fs::write(tmp.path().join("main.st"), "PROGRAM main\n  undefined_variable := 1;\nEND_PROGRAM\n").unwrap();

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    let workspace_uri: lsp_types::Uri = format!("file://{}", tmp.path().display()).parse().unwrap();
    let params = InitializeParams {
        workspace_folders: Some(vec![WorkspaceFolder { uri: workspace_uri, name: "test".to_string() }]),
        ..Default::default()
    };

    client_conn
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(1),
            method: "initialize".to_string(),
            params: serde_json::to_value(params).unwrap(),
        }))
        .unwrap();

    // Drain the initialize response.
    let _ = client_conn.receiver.recv().unwrap();

    client_conn
        .sender
        .send(Message::Notification(Notification { method: "initialized".to_string(), params: json!({}) }))
        .unwrap();

    let publish = wait_for_publish(&client_conn, Duration::from_secs(30));
    assert!(
        publish.uri.as_str().ends_with("main.st"),
        "expected URI to end with main.st, got {:?}",
        publish.uri
    );
    assert!(
        !publish.diagnostics.is_empty(),
        "expected at least one diagnostic, got: {:?}",
        publish.diagnostics
    );

    // Clean shutdown so the worker exits.
    client_conn
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(2),
            method: "shutdown".to_string(),
            params: json!(null),
        }))
        .unwrap();
    drain_until_response(&client_conn, 2);
    client_conn
        .sender
        .send(Message::Notification(Notification { method: "exit".to_string(), params: json!(null) }))
        .unwrap();

    server_thread.join().expect("server thread panicked").expect("server returned error");
}

fn wait_for_publish(client: &Connection, timeout: Duration) -> PublishDiagnosticsParams {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        let remaining = deadline.checked_duration_since(std::time::Instant::now()).unwrap_or(Duration::ZERO);
        match client.receiver.recv_timeout(remaining) {
            Ok(Message::Notification(n)) if n.method == "textDocument/publishDiagnostics" => {
                return serde_json::from_value(n.params).expect("PublishDiagnosticsParams deserialise");
            }
            Ok(_other) => {} // ignore other notifications while waiting
            Err(_) => panic!("timed out waiting for publishDiagnostics"),
        }
    }
}

fn drain_until_response(client: &Connection, expected_id: i32) {
    while let Ok(msg) = client.receiver.recv_timeout(Duration::from_secs(5)) {
        if let Message::Response(r) = msg {
            if r.id == RequestId::from(expected_id) {
                return;
            }
        }
    }
    panic!("didn't see response for id {expected_id}");
}
