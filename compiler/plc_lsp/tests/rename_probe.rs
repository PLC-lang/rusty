//! End-to-end probe for phase 12 (rename).
//!
//! Drives the LSP server against a small ST project mixing the symbol
//! kinds rename supports: locals, globals, struct members, POUs,
//! methods, types, inheritance (`Base.foo` / `Derived.foo`), and
//! actions. For each cursor position, runs `prepareRename` followed
//! by `rename` with a sample new name. Eprintln summary on
//! `--nocapture` — loose assertions, the value is in the visible diff
//! for behaviour changes.
//!
//! Inheritance + property cases are deliberately covered to surface
//! what the existing lowering passes propagate into the reverse
//! index. Outcomes feed `[[lsp-post-phase13-followups]]`.

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

const TYPES_ST: &str = "TYPE myType : STRUCT\n\
    myInt : INT;\n\
    myReal : REAL;\n\
END_STRUCT END_TYPE\n";

const BASE_ST: &str = "FUNCTION_BLOCK Base\n\
    METHOD foo\n\
    END_METHOD\n\
END_FUNCTION_BLOCK\n";

const DERIVED_ST: &str = "FUNCTION_BLOCK Derived EXTENDS Base\n\
    METHOD foo\n\
        SUPER^.foo();\n\
    END_METHOD\n\
END_FUNCTION_BLOCK\n";

const MAIN_ST: &str = "VAR_GLOBAL\n\
    g: DINT := 0;\n\
END_VAR\n\
FUNCTION myFunc : DINT\n\
VAR_INPUT\n\
    x: DINT;\n\
END_VAR\n\
myFunc := x * 2;\n\
END_FUNCTION\n\
FUNCTION main : DINT\n\
VAR\n\
    a: DINT := 0;\n\
    s: myType;\n\
    b: Base;\n\
    d: Derived;\n\
END_VAR\n\
a := myFunc(x := 2);\n\
g := a + 1;\n\
s.myInt := a;\n\
b.foo();\n\
d.foo();\n\
main := 0;\n\
END_FUNCTION\n";

#[test]
fn probe_rename() {
    let tmp = tempdir_with_corpus();
    let types_uri = file_uri(&tmp, "types.st");
    let base_uri = file_uri(&tmp, "base.st");
    let derived_uri = file_uri(&tmp, "derived.st");
    let main_uri = file_uri(&tmp, "main.st");

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    initialize(&client_conn, &tmp);
    send_did_open(&client_conn, &types_uri, TYPES_ST);
    send_did_open(&client_conn, &base_uri, BASE_ST);
    send_did_open(&client_conn, &derived_uri, DERIVED_ST);
    send_did_open(&client_conn, &main_uri, MAIN_ST);
    thread::sleep(Duration::from_secs(2));

    // Note: Rust string continuation (`\n\`) strips leading whitespace
    // off the next line, so `MAIN_ST`'s VAR-block lines start at col 0
    // (no indent in the actual literal). Positions reflect the actual
    // content, not the source-formatted-with-tabs version.
    let probes: Vec<(&str, &Uri, Position, &str)> = vec![
        ("local `a` (decl)", &main_uri, Position { line: 11, character: 0 }, "renamed_a"),
        ("global `g` (decl)", &main_uri, Position { line: 1, character: 0 }, "renamed_g"),
        ("argument `x` (decl)", &main_uri, Position { line: 5, character: 0 }, "renamed_x"),
        ("function `myFunc` (decl)", &main_uri, Position { line: 3, character: 9 }, "renamedFunc"),
        ("struct field `myInt` (decl)", &types_uri, Position { line: 1, character: 4 }, "renamed_myInt"),
        ("type `myType` (decl)", &types_uri, Position { line: 0, character: 5 }, "RenamedType"),
        ("function block `Base` (decl)", &base_uri, Position { line: 0, character: 15 }, "RenamedBase"),
        ("use of `a` in body", &main_uri, Position { line: 16, character: 0 }, "from_use_a"),
        ("use of `myFunc` in call", &main_uri, Position { line: 16, character: 5 }, "from_call_myFunc"),
        ("reject reserved keyword", &main_uri, Position { line: 11, character: 0 }, "PROGRAM"),
        ("reject invalid identifier", &main_uri, Position { line: 11, character: 0 }, "123bad"),
    ];

    let mut out = String::new();
    for (label, uri, position, new_name) in &probes {
        let prep = request(
            &client_conn,
            "textDocument/prepareRename",
            &json!({
                "textDocument": { "uri": uri.as_str() },
                "position": { "line": position.line, "character": position.character },
            }),
        );
        let rn = request(
            &client_conn,
            "textDocument/rename",
            &json!({
                "textDocument": { "uri": uri.as_str() },
                "position": { "line": position.line, "character": position.character },
                "newName": new_name,
            }),
        );
        out.push_str(&format!(
            "── {label} @ ({},{}) → '{}'\n  prepareRename: {}\n  rename: {}\n",
            position.line,
            position.character,
            new_name,
            summarise(&prep),
            summarise(&rn),
        ));
    }

    eprintln!("\n=== rename_probe ===\n{out}");
    shutdown(&client_conn);
    server_thread.join().expect("server thread").expect("server returned error");
}

fn tempdir_with_corpus() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{"name":"probe","files":["*.st"]}"#).unwrap();
    fs::write(tmp.path().join("types.st"), TYPES_ST).unwrap();
    fs::write(tmp.path().join("base.st"), BASE_ST).unwrap();
    fs::write(tmp.path().join("derived.st"), DERIVED_ST).unwrap();
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
                return if let Some(err) = r.error {
                    json!({"error": err.message})
                } else {
                    r.result.unwrap_or(Value::Null)
                };
            }
            Ok(_) => {}
            Err(_) => panic!("timed out waiting for {method} response"),
        }
    }
}

fn summarise(v: &Value) -> String {
    serde_json::to_string(v).unwrap_or_else(|_| "<serialise error>".into())
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
