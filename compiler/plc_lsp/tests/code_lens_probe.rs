//! End-to-end probe for `textDocument/codeLens` (H3).
//!
//! Asserts:
//! - Above the interface's `aaa` method a lens reading "1 implementations".
//! - Above the FB's `aaa` method a lens reading "implements aaa".

use std::fs;
use std::thread;
use std::time::Duration;

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::{
    DidOpenTextDocumentParams, GeneralClientCapabilities, InitializeParams, PositionEncodingKind,
    TextDocumentItem, Uri, WorkspaceFolder,
};
use serde_json::{json, Value};
use tempfile::TempDir;

const MAIN_ST: &str = r#"INTERFACE I_Worker
    METHOD aaa : INT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK Worker IMPLEMENTS I_Worker
    METHOD aaa : INT
        aaa := 42;
    END_METHOD
END_FUNCTION_BLOCK
"#;

#[test]
fn code_lenses_emit_for_interface_and_implementation() {
    let tmp = tempdir_with_main();
    let main_uri = file_uri(&tmp, "main.st");

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    initialize(&client_conn, &tmp);
    send_did_open(&client_conn, &main_uri, MAIN_ST);
    thread::sleep(Duration::from_secs(2));

    let response = request(
        &client_conn,
        "textDocument/codeLens",
        &json!({ "textDocument": { "uri": main_uri.as_str() } }),
    );
    let lenses = response.as_array().expect("codeLens should return an array");

    // Find the "N implementations" lens — should sit above the interface's
    // `aaa` method at line 1.
    let iface_lens = lenses
        .iter()
        .find(|l| {
            l.get("command")
                .and_then(|c| c.get("title"))
                .and_then(|t| t.as_str())
                .map(|t| t.ends_with(" implementations"))
                .unwrap_or(false)
        })
        .unwrap_or_else(|| panic!("no `N implementations` lens: {response}"));
    let iface_title =
        iface_lens.get("command").and_then(|c| c.get("title")).and_then(|t| t.as_str()).unwrap();
    assert_eq!(iface_title, "1 implementations", "got {iface_title:?}");

    // Find the "implements aaa" lens — sits above the FB's `aaa` at line 6.
    let impl_lens = lenses
        .iter()
        .find(|l| {
            l.get("command")
                .and_then(|c| c.get("title"))
                .and_then(|t| t.as_str())
                .map(|t| t.starts_with("implements"))
                .unwrap_or(false)
        })
        .unwrap_or_else(|| panic!("no `implements ...` lens: {response}"));
    let impl_title = impl_lens.get("command").and_then(|c| c.get("title")).and_then(|t| t.as_str()).unwrap();
    assert_eq!(impl_title, "implements aaa", "got {impl_title:?}");

    shutdown(&client_conn);
    server_thread.join().expect("server thread").expect("server returned error");
}

// --- helpers ---

fn tempdir_with_main() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{"name":"probe","files":["*.st"]}"#).unwrap();
    fs::write(tmp.path().join("main.st"), MAIN_ST).unwrap();
    tmp
}

fn file_uri(tmp: &TempDir, name: &str) -> Uri {
    format!("file://{}/{name}", tmp.path().display()).parse().unwrap()
}

fn initialize(client: &Connection, tmp: &TempDir) {
    let workspace_uri: Uri = format!("file://{}", tmp.path().display()).parse().unwrap();
    let mut params = InitializeParams {
        workspace_folders: Some(vec![WorkspaceFolder { uri: workspace_uri, name: "probe".to_string() }]),
        ..Default::default()
    };
    params.capabilities.general = Some(GeneralClientCapabilities {
        position_encodings: Some(vec![PositionEncodingKind::UTF8]),
        ..Default::default()
    });
    client
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(1),
            method: "initialize".to_string(),
            params: serde_json::to_value(params).unwrap(),
        }))
        .unwrap();
    drain_until_response(client, 1);
    client
        .sender
        .send(Message::Notification(Notification { method: "initialized".to_string(), params: json!({}) }))
        .unwrap();
}

fn send_did_open(client: &Connection, uri: &Uri, text: &str) {
    let params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "st".to_string(),
            version: 1,
            text: text.to_string(),
        },
    };
    client
        .sender
        .send(Message::Notification(Notification {
            method: "textDocument/didOpen".to_string(),
            params: serde_json::to_value(params).unwrap(),
        }))
        .unwrap();
}

fn request(client: &Connection, method: &str, params: &Value) -> Value {
    static NEXT_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(100);
    let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    client
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(id),
            method: method.to_string(),
            params: params.clone(),
        }))
        .unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let remaining = deadline.checked_duration_since(std::time::Instant::now()).unwrap_or(Duration::ZERO);
        match client.receiver.recv_timeout(remaining) {
            Ok(Message::Response(r)) if r.id == RequestId::from(id) => {
                return r.result.unwrap_or(Value::Null);
            }
            Ok(_) => {}
            Err(_) => panic!("timed out waiting for {method} response"),
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

fn shutdown(client: &Connection) {
    client
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(999),
            method: "shutdown".to_string(),
            params: json!(null),
        }))
        .unwrap();
    drain_until_response(client, 999);
    client
        .sender
        .send(Message::Notification(Notification { method: "exit".to_string(), params: json!(null) }))
        .unwrap();
}
