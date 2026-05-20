//! End-to-end probe for phases 8/9/10. Drives the LSP server against a
//! tempdir mirroring `~/git/lsp-editor-setup/examples` and prints the
//! responses to hover / definition / references at known cursor
//! positions. Used as a debugging aid when bug reports come in;
//! assertions are intentionally loose so behaviour changes surface as
//! snapshot diffs rather than red bars.
//!
//! Layout:
//!     plc.json   {"name":"probe","files":["*.st"]}
//!     test.st    program test
//!                VAR a: DINT := 0; str: myType; END_VAR
//!                a := myFunc(x := 2);
//!                a := myFunc(a);
//!                str.myInt := a;
//!                end_program
//!     myFunc.st  function myFunc : DINT VAR_INPUT x: DINT; END_VAR
//!                myFunc := x * 2; end_function
//!     myType.st  type myType : STRUCT myInt : INT; myReal : REAL;
//!                END_STRUCT end_type

#![allow(clippy::too_many_arguments)]

use std::fs;
use std::thread;
use std::time::Duration;

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::{
    DidOpenTextDocumentParams, GeneralClientCapabilities, InitializeParams, Position, PositionEncodingKind,
    PublishDiagnosticsParams, TextDocumentItem, Uri, WorkspaceFolder,
};
use serde_json::{json, Value};
use tempfile::TempDir;

const TEST_ST: &str = "program test\n\
VAR\n\
\ta: DINT := 0;\n\
\tstr: myType;\n\
END_VAR\n\
a := myFunc(x := 2);\n\
a := myFunc(a);\n\
str.myInt := a;\n\
end_program\n";

const MY_FUNC_ST: &str = "function myFunc : DINT\n\
VAR_INPUT\n\
    x: DINT; \n\
END_VAR\n\
\n\
myFunc := x * 2;\n\
\n\
end_function\n";

const MY_TYPE_ST: &str = "type myType : STRUCT\n\
    myInt : INT;\n\
    myReal : REAL;\n\
END_STRUCT\n\
end_type\n";

#[test]
fn probe_hover_definition_references() {
    let tmp = tempdir_with_examples();
    let test_uri = file_uri(&tmp, "test.st");
    let mytype_uri = file_uri(&tmp, "myType.st");

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    initialize(&client_conn, &tmp);
    send_did_open(&client_conn, &test_uri, TEST_ST);
    send_did_open(&client_conn, &mytype_uri, MY_TYPE_ST);

    // Wait for the publishDiagnostics that confirms a compile completed.
    wait_for_first_publish(&client_conn, Duration::from_secs(30));

    let probes: Vec<(&str, &Uri, Position)> = vec![
        // line / col is 0-based, character is UTF-8 since that's what the
        // test client negotiates below.
        ("DECL: `a` name in `VAR a: DINT`", &test_uri, Position { line: 2, character: 1 }),
        ("DECL: `str` name in VAR block", &test_uri, Position { line: 3, character: 1 }),
        ("DECL: `test` POU name", &test_uri, Position { line: 0, character: 8 }),
        ("DECL: `myType` type name (in myType.st)", &mytype_uri, Position { line: 0, character: 5 }),
        ("hover on `a` (LHS of `a := myFunc(x := 2)`)", &test_uri, Position { line: 5, character: 0 }),
        ("hover on `myFunc` in the call", &test_uri, Position { line: 5, character: 5 }),
        ("hover on `x` (named arg LHS)", &test_uri, Position { line: 5, character: 12 }),
        ("hover on `a` (positional arg in `myFunc(a)`)", &test_uri, Position { line: 6, character: 12 }),
        ("hover on `str` in `str.myInt`", &test_uri, Position { line: 7, character: 0 }),
        ("hover on `myInt` (member access)", &test_uri, Position { line: 7, character: 4 }),
        ("hover on `myType` in VAR declaration", &test_uri, Position { line: 3, character: 6 }),
    ];

    let mut out = String::new();
    for (label, uri, position) in &probes {
        let hover = request(&client_conn, "textDocument/hover", &hover_params(uri, *position));
        let definition = request(&client_conn, "textDocument/definition", &goto_params(uri, *position));
        let refs = request(&client_conn, "textDocument/references", &refs_params(uri, *position));
        out.push_str(&format!(
            "── {label} @ ({},{})\n  hover: {}\n  definition: {}\n  references: {}\n",
            position.line,
            position.character,
            summarise(&hover),
            summarise(&definition),
            summarise(&refs),
        ));
    }

    // Print so the test runner surfaces it on failure / `--nocapture`.
    eprintln!("\n=== cursor_lookup probe ===\n{out}");

    shutdown(&client_conn);
    server_thread.join().expect("server thread").expect("server returned error");
}

fn tempdir_with_examples() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{"name":"probe","files":["*.st"]}"#).unwrap();
    fs::write(tmp.path().join("test.st"), TEST_ST).unwrap();
    fs::write(tmp.path().join("myFunc.st"), MY_FUNC_ST).unwrap();
    fs::write(tmp.path().join("myType.st"), MY_TYPE_ST).unwrap();
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
    // Force UTF-8 so positions are byte offsets (simpler for the probe).
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

fn goto_params(uri: &Uri, position: Position) -> Value {
    json!({
        "textDocument": { "uri": uri.as_str() },
        "position": { "line": position.line, "character": position.character },
    })
}

fn refs_params(uri: &Uri, position: Position) -> Value {
    json!({
        "textDocument": { "uri": uri.as_str() },
        "position": { "line": position.line, "character": position.character },
        "context": { "includeDeclaration": true },
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

fn summarise(v: &Value) -> String {
    serde_json::to_string(v).unwrap_or_else(|_| "<serialise error>".into())
}

fn wait_for_first_publish(client: &Connection, timeout: Duration) -> PublishDiagnosticsParams {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        let remaining = deadline.checked_duration_since(std::time::Instant::now()).unwrap_or(Duration::ZERO);
        match client.receiver.recv_timeout(remaining) {
            Ok(Message::Notification(n)) if n.method == "textDocument/publishDiagnostics" => {
                return serde_json::from_value(n.params).expect("PublishDiagnosticsParams deserialise");
            }
            Ok(_) => {}
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
