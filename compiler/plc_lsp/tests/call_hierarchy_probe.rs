//! End-to-end probe for phase 11 (call hierarchy).
//!
//! Drives the LSP server against a small ST project covering the
//! shapes we want call hierarchy to handle:
//! - Direct function calls.
//! - Function-block instance invocation.
//! - Method calls (`instance.method()`).
//! - `THIS^.method()` from inside a method body.
//! - `SUPER^()` from a derived FB.
//! - `ACTION` declarations + their call sites.
//! - Inheritance: `Base` with method `foo`, `Derived EXTENDS Base`
//!   overriding `foo`. Inspects whether lowering propagates enough
//!   info into the reverse index for cross-inheritance call hierarchy.
//!
//! Loose assertions — eprintln summary on `--nocapture`. Behaviour
//! drift surfaces as visible diff.

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

// Mirrors the inheritance + SUPER^/THIS^ shape from
// `tests/lit/single/polymorphism/super_call.st`.
const BASE_ST: &str = "FUNCTION_BLOCK Base\n\
    METHOD foo\n\
    END_METHOD\n\
END_FUNCTION_BLOCK\n";

const DERIVED_ST: &str = "FUNCTION_BLOCK Derived EXTENDS Base\n\
    METHOD foo\n\
        SUPER^.foo();\n\
        SUPER^();\n\
    END_METHOD\n\
    METHOD callOwn\n\
        THIS^.foo();\n\
    END_METHOD\n\
END_FUNCTION_BLOCK\n";

// Actions live in a top-level ACTIONS block, NOT inside the FB body
// (cf. tests/lit/single/actions/action_call_with_var_in_out.st).
const ACTION_ST: &str = "FUNCTION_BLOCK Worker\n\
    VAR\n\
        counter : INT;\n\
    END_VAR\n\
END_FUNCTION_BLOCK\n\
\n\
ACTIONS Worker\n\
    ACTION step\n\
        counter := counter + 1;\n\
    END_ACTION\n\
END_ACTIONS\n";

const MAIN_ST: &str = "FUNCTION main : INT\n\
VAR\n\
    b : Base;\n\
    d : Derived;\n\
    w : Worker;\n\
END_VAR\n\
b.foo();\n\
d.foo();\n\
d.callOwn();\n\
w.step();\n\
main := 0;\n\
END_FUNCTION\n";

#[test]
fn probe_call_hierarchy() {
    let tmp = tempdir_with_corpus();
    let main_uri = file_uri(&tmp, "main.st");
    let derived_uri = file_uri(&tmp, "derived.st");
    let base_uri = file_uri(&tmp, "base.st");
    let action_uri = file_uri(&tmp, "action.st");

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    initialize(&client_conn, &tmp);
    // Position lookup needs the file open to compute byte offsets from
    // LSP positions; open everything we'll probe.
    send_did_open(&client_conn, &main_uri, MAIN_ST);
    send_did_open(&client_conn, &derived_uri, DERIVED_ST);
    send_did_open(&client_conn, &base_uri, BASE_ST);
    send_did_open(&client_conn, &action_uri, ACTION_ST);
    // A clean compile produces no `publishDiagnostics`, so we can't
    // wait on that. The corpus deliberately has no errors. Sleep
    // briefly to let the first compile complete; the worker is in-
    // process so 2s is conservative.
    thread::sleep(Duration::from_secs(2));

    let probes: Vec<(&str, &Uri, Position)> = vec![
        // Base.st: line 1 is "    METHOD foo", cursor on `foo` at col 11
        ("Base.foo decl", &base_uri, Position { line: 1, character: 11 }),
        // Derived.st: line 1 is "    METHOD foo", cursor on `foo` at col 11
        ("Derived.foo decl", &derived_uri, Position { line: 1, character: 11 }),
        // Action.st: line 6 is "ACTIONS Worker", line 7 is "    ACTION step"; cursor on `step`
        ("Worker.step action decl", &action_uri, Position { line: 7, character: 11 }),
        // main.st: line 0 is "FUNCTION main : INT", cursor on `main` at col 9
        ("main function decl", &main_uri, Position { line: 0, character: 9 }),
        // main.st: line 6 is "b.foo();", cursor on `foo` at col 2
        ("b.foo() call site (in main)", &main_uri, Position { line: 6, character: 2 }),
        ("d.foo() call site (in main)", &main_uri, Position { line: 7, character: 2 }),
        // derived.st line 2 is "        SUPER^.foo();", cursor on `foo` at col 15
        ("SUPER^.foo() in Derived.foo", &derived_uri, Position { line: 2, character: 15 }),
        // derived.st line 6 is "        THIS^.foo();" (callOwn body), cursor on `foo` at col 14
        ("THIS^.foo() in Derived.callOwn", &derived_uri, Position { line: 6, character: 14 }),
    ];

    let mut out = String::new();
    for (label, uri, position) in &probes {
        let item =
            request(&client_conn, "textDocument/prepareCallHierarchy", &prepare_params(uri, *position));
        out.push_str(&format!(
            "── {label} @ ({},{})\n  prepareCallHierarchy: {}\n",
            position.line,
            position.character,
            summarise(&item),
        ));

        // If prepare returned an item, drill into incoming + outgoing.
        if let Some(items) = item.as_array() {
            if let Some(first) = items.first() {
                let incoming =
                    request(&client_conn, "callHierarchy/incomingCalls", &json!({ "item": first }));
                let outgoing =
                    request(&client_conn, "callHierarchy/outgoingCalls", &json!({ "item": first }));
                out.push_str(&format!("  incomingCalls: {}\n", summarise(&incoming)));
                out.push_str(&format!("  outgoingCalls: {}\n", summarise(&outgoing)));
            }
        }
    }

    eprintln!("\n=== call_hierarchy_probe ===\n{out}");
    shutdown(&client_conn);
    server_thread.join().expect("server thread").expect("server returned error");
}

fn tempdir_with_corpus() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{"name":"probe","files":["*.st"]}"#).unwrap();
    fs::write(tmp.path().join("base.st"), BASE_ST).unwrap();
    fs::write(tmp.path().join("derived.st"), DERIVED_ST).unwrap();
    fs::write(tmp.path().join("action.st"), ACTION_ST).unwrap();
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

fn prepare_params(uri: &Uri, position: Position) -> Value {
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
