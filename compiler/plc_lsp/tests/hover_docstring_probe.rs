//! End-to-end probe for hover docstrings sourced from the comment above
//! a declaration, including the cross-file case (declaration in a
//! different source than the cursor).
//!
//! Layout:
//!     plc.json   {"name":"probe","files":["*.st"]}
//!     main.st    PROGRAM main with a VAR of type Widget
//!     other.st   (* doc *)\nTYPE Widget : STRUCT … END_STRUCT END_TYPE
//!
//! Asserts:
//!     - Hover on `Widget` in main.st renders the docstring from other.st.
//!     - Hover on the declaration site in other.st also renders it.

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

// Raw strings so leading whitespace survives — feedback_raw_strings_for_test_fixtures.
const MAIN_ST: &str = r#"VAR_GLOBAL CONSTANT
    gc : DINT := 42;
END_VAR

VAR_GLOBAL RETAIN
    state : DINT;
END_VAR

PROGRAM main
VAR
    p : Widget;
    n : DINT;
END_VAR
    n := foo();
    n := gc;
    state := state + 1;
END_PROGRAM
"#;

const OTHER_ST: &str = r#"(* A reusable widget with a single counter. *)
TYPE Widget : STRUCT
    count : DINT;
END_STRUCT END_TYPE

(* The famous answer. *)
FUNCTION foo : DINT
    foo := 42;
END_FUNCTION

(* Converts DWORD to REAL *)
{external}
FUNCTION DWORD_TO_REAL : REAL
VAR_INPUT
    in : DWORD;
END_VAR
END_FUNCTION
"#;

#[test]
fn cross_file_hover_includes_docstring() {
    let tmp = tempdir_with_sources();
    let main_uri = file_uri(&tmp, "main.st");
    let other_uri = file_uri(&tmp, "other.st");

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    initialize(&client_conn, &tmp);
    send_did_open(&client_conn, &main_uri, MAIN_ST);
    send_did_open(&client_conn, &other_uri, OTHER_ST);
    // Clean compiles publish only "clear" notifications; wait a bit for the
    // worker to attach AnnotatedProject to ServerState.
    thread::sleep(Duration::from_secs(2));

    // Hover at the usage site in main.st. Line 10 col 8 lands on `Widget`
    // in `p : Widget;` — the type reference. The hover handler resolves it
    // to the type's declaration in other.st and the docstring lookup
    // crosses the file boundary via the token cache.
    let usage_pos = Position { line: 10, character: 8 };
    let usage_response = request(&client_conn, "textDocument/hover", &hover_params(&main_uri, usage_pos));
    let usage_markdown = extract_markdown(&usage_response);
    assert!(
        usage_markdown.contains("A reusable widget"),
        "cross-file hover at usage site missing docstring; response: {usage_response}",
    );
    assert!(
        usage_markdown.contains("---"),
        "expected horizontal rule between signature and docs; got {usage_markdown:?}"
    );

    // Hover at the call site `foo()` in main.st (line 13 col 9 = `f` of foo).
    // Should resolve to the function declaration in other.st and surface
    // the docstring.
    let call_pos = Position { line: 13, character: 9 };
    let call_response = request(&client_conn, "textDocument/hover", &hover_params(&main_uri, call_pos));
    let call_markdown = extract_markdown(&call_response);
    assert!(
        call_markdown.contains("famous answer"),
        "cross-file hover at call site missing docstring; response: {call_response}",
    );

    // Hover at the use of the constant global `gc` (line 14 col 9 = `g`).
    let gc_pos = Position { line: 14, character: 9 };
    let gc_response = request(&client_conn, "textDocument/hover", &hover_params(&main_uri, gc_pos));
    let gc_markdown = extract_markdown(&gc_response);
    assert!(
        gc_markdown.contains("VAR_GLOBAL CONSTANT"),
        "constant global hover should say `VAR_GLOBAL CONSTANT`; got {gc_markdown:?}",
    );

    // Hover at the use of the RETAIN global `state` (line 15 col 4 = `s`).
    // Section keyword must reflect the RETAIN modifier.
    let state_pos = Position { line: 15, character: 4 };
    let state_response = request(&client_conn, "textDocument/hover", &hover_params(&main_uri, state_pos));
    let state_markdown = extract_markdown(&state_response);
    assert!(
        state_markdown.contains("VAR_GLOBAL RETAIN"),
        "retain global hover should say `VAR_GLOBAL RETAIN`; got {state_markdown:?}",
    );

    // Hover at the `DWORD_TO_REAL` declaration in other.st (the
    // stdlib-style shape: doc comment then `{external}` then FUNCTION).
    // `{external}` is a real Token::PropertyExternal, not Pragma trivia;
    // the prefix walker must widen past it so the comment above attaches.
    // Line 12 col 9 = `D` of DWORD_TO_REAL in OTHER_ST (line 11 is `{external}`).
    let stdlib_pos = Position { line: 12, character: 9 };
    let stdlib_response = request(&client_conn, "textDocument/hover", &hover_params(&other_uri, stdlib_pos));
    let stdlib_markdown = extract_markdown(&stdlib_response);
    assert!(
        stdlib_markdown.contains("Converts DWORD to REAL"),
        "stdlib-style hover missing docstring across {{external}}; response: {stdlib_response}",
    );

    // Hover at the declaration site in other.st. Line 1 col 5 = `W` of Widget.
    let decl_pos = Position { line: 1, character: 5 };
    let decl_response = request(&client_conn, "textDocument/hover", &hover_params(&other_uri, decl_pos));
    let decl_markdown = extract_markdown(&decl_response);
    assert!(
        decl_markdown.contains("A reusable widget"),
        "hover at declaration site missing docstring; response: {decl_response}",
    );

    shutdown(&client_conn);
    server_thread.join().expect("server thread").expect("server returned error");
}

fn extract_markdown(response: &Value) -> String {
    response.get("contents").and_then(|c| c.get("value")).and_then(|v| v.as_str()).unwrap_or("").to_string()
}

fn tempdir_with_sources() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{"name":"probe","files":["*.st"]}"#).unwrap();
    fs::write(tmp.path().join("main.st"), MAIN_ST).unwrap();
    fs::write(tmp.path().join("other.st"), OTHER_ST).unwrap();
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

fn hover_params(uri: &Uri, position: Position) -> Value {
    json!({
        "textDocument": { "uri": uri.as_str() },
        "position": { "line": position.line, "character": position.character },
    })
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
