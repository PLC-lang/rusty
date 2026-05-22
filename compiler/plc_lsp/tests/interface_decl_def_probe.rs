//! End-to-end probe for the interface declaration / FB definition
//! split (H1).
//!
//! Layout:
//!
//!     INTERFACE I_Worker
//!         METHOD aaa : INT ... END_METHOD
//!     END_INTERFACE
//!
//!     FUNCTION_BLOCK Worker IMPLEMENTS I_Worker
//!         ... aaa body ...
//!     END_FUNCTION_BLOCK
//!
//! Assertions:
//! - cursor on `aaa` in `Worker.aaa` (the impl) → `goto-declaration`
//!   jumps to the interface's `aaa` decl line.
//! - cursor on `aaa` in `Worker.aaa` (the impl) → `goto-definition`
//!   stays on the impl.
//! - find-references on the interface's `aaa` includes Worker's
//!   `aaa` (the implementation site).

use std::fs;
use std::thread;
use std::time::Duration;

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::{
    DidOpenTextDocumentParams, GeneralClientCapabilities, InitializeParams, Position, PositionEncodingKind,
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
fn interface_method_decl_def_split() {
    let tmp = tempdir_with_main();
    let main_uri = file_uri(&tmp, "main.st");

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    initialize(&client_conn, &tmp);
    send_did_open(&client_conn, &main_uri, MAIN_ST);
    thread::sleep(Duration::from_secs(2));

    // Cursor on `aaa` inside the FB's METHOD aaa header (line 6 col 11).
    let impl_aaa = Position { line: 6, character: 11 };

    // goto-declaration → interface line. The interface's `aaa` is on
    // line 1 col 11.
    let decl_resp = request(&client_conn, "textDocument/declaration", &goto_params(&main_uri, impl_aaa));
    let decl_line = decl_resp
        .get("range")
        .and_then(|r| r.get("start"))
        .and_then(|s| s.get("line"))
        .and_then(|v| v.as_u64());
    assert_eq!(
        decl_line,
        Some(1),
        "goto-declaration should land on the interface's aaa (line 1); got {decl_resp}"
    );

    // goto-definition stays on the FB's method — same as the cursor's line.
    let def_resp = request(&client_conn, "textDocument/definition", &goto_params(&main_uri, impl_aaa));
    let def_line = def_resp
        .get("range")
        .and_then(|r| r.get("start"))
        .and_then(|s| s.get("line"))
        .and_then(|v| v.as_u64());
    assert_eq!(def_line, Some(6), "goto-definition should stay on the FB's aaa (line 6); got {def_resp}");

    // find-references on the interface's aaa. Cursor on the interface's
    // method name at line 1 col 11. Expect the Worker.aaa impl in the
    // results.
    let iface_aaa = Position { line: 1, character: 11 };
    let refs_resp = request(
        &client_conn,
        "textDocument/references",
        &json!({
            "textDocument": { "uri": main_uri.as_str() },
            "position": { "line": iface_aaa.line, "character": iface_aaa.character },
            "context": { "includeDeclaration": false },
        }),
    );
    let refs = refs_resp.as_array().expect("references should be an array");
    let any_impl = refs.iter().any(|r| {
        r.get("range").and_then(|r| r.get("start")).and_then(|s| s.get("line")).and_then(|v| v.as_u64())
            == Some(6)
    });
    assert!(any_impl, "references list should include Worker.aaa impl at line 6; got {refs_resp}");

    shutdown(&client_conn);
    server_thread.join().expect("server thread").expect("server returned error");
}

// --- helpers (lifted from other probes) ---

fn tempdir_with_main() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{"name":"probe","files":["*.st"]}"#).unwrap();
    fs::write(tmp.path().join("main.st"), MAIN_ST).unwrap();
    tmp
}

fn file_uri(tmp: &TempDir, name: &str) -> Uri {
    format!("file://{}/{name}", tmp.path().display()).parse().unwrap()
}

fn goto_params(uri: &Uri, position: Position) -> Value {
    json!({
        "textDocument": { "uri": uri.as_str() },
        "position": { "line": position.line, "character": position.character },
    })
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
