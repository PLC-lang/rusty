//! Probe for `completionItem/resolve`: request completion, take one
//! item that carries a `data` resolve tag, send it back via the
//! resolve method, assert the response carries the declaration's
//! docstring as markdown.

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

const MAIN_ST: &str = r#"PROGRAM main
VAR
    n : DINT;
END_VAR
    n :=
END_PROGRAM
"#;

const LIB_ST: &str = r#"(* Computes the answer to everything. *)
FUNCTION foo : DINT
    foo := 42;
END_FUNCTION
"#;

#[test]
fn resolve_attaches_docstring_to_focused_item() {
    let tmp = tempdir_with_sources();
    let main_uri = file_uri(&tmp, "main.st");
    let lib_uri = file_uri(&tmp, "lib.st");

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    initialize(&client_conn, &tmp);
    send_did_open(&client_conn, &main_uri, MAIN_ST);
    send_did_open(&client_conn, &lib_uri, LIB_ST);
    thread::sleep(Duration::from_secs(2));

    // Cursor on the empty assignment RHS — line 4 col 9 lands right after `:=`.
    let completion_pos = Position { line: 4, character: 9 };
    let items = completion(&client_conn, &main_uri, completion_pos);
    let items_arr = items.get("items").and_then(|v| v.as_array()).expect("items array");

    // Find the `foo` item the user would focus.
    let foo_item = items_arr
        .iter()
        .find(|i| i.get("label").and_then(|l| l.as_str()) == Some("foo"))
        .expect("foo POU completion item missing");
    // Pre-resolve the item must NOT carry documentation — that's the lazy
    // contract we're advertising via resolveProvider.
    assert!(
        foo_item.get("documentation").is_none(),
        "completion item should not carry docstring before resolve: {foo_item}"
    );
    assert!(foo_item.get("data").is_some(), "completion item missing resolve tag in data field");

    // Send the item back via completionItem/resolve. The server should
    // look up the declaration via the resolve tag and attach the doc.
    let resolved = request(&client_conn, "completionItem/resolve", foo_item);
    let doc =
        resolved.get("documentation").and_then(|d| d.get("value")).and_then(|v| v.as_str()).unwrap_or("");
    assert!(doc.contains("answer to everything"), "resolve response missing docstring; got {resolved}",);

    shutdown(&client_conn);
    server_thread.join().expect("server thread").expect("server returned error");
}

fn completion(client: &Connection, uri: &Uri, position: Position) -> Value {
    request(
        client,
        "textDocument/completion",
        &json!({
            "textDocument": { "uri": uri.as_str() },
            "position": { "line": position.line, "character": position.character },
            "context": { "triggerKind": 1 },
        }),
    )
}

fn tempdir_with_sources() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{"name":"probe","files":["*.st"]}"#).unwrap();
    fs::write(tmp.path().join("main.st"), MAIN_ST).unwrap();
    fs::write(tmp.path().join("lib.st"), LIB_ST).unwrap();
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
