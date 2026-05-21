//! Completion probe (phase 13 — P13.6). Drives the server with a real
//! project compile in a tempdir, sends `textDocument/completion`
//! requests at deliberate cursor positions, and asserts on the returned
//! item set per Q7-D: must_include / must_exclude / triggerKind
//! routing. Symbol enumeration is now wired up — the assertions exercise
//! the per-category logic in `completion.rs`.

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

// MAIN_ST line layout (0-based, matches Position.line in LSP):
//   0:  TYPE Point : STRUCT
//   1:      x : DINT;
//   2:      y : DINT;
//   3:      name : STRING;
//   4:  END_STRUCT END_TYPE
//   5:
//   6:  VAR_GLOBAL
//   7:      origin : Point;
//   8:  END_VAR
//   9:
//  10:  FUNCTION compute : DINT
//  11:  VAR_INPUT
//  12:      value : DINT;
//  13:  END_VAR
//  14:      compute := value * 2;
//  15:  END_FUNCTION
//  16:
//  17:  FUNCTION main : DINT
//  18:  VAR
//  19:      p : Point;
//  20:      n : DINT;
//  21:      flag : BOOL;
//  22:  END_VAR
//  23:      n := p.x;
//  24:                              ← statement-position probe lands here (line 24, char 4)
//  25:  END_FUNCTION
const MAIN_ST: &str = "TYPE Point : STRUCT\n    x : DINT;\n    y : DINT;\n    name : STRING;\nEND_STRUCT END_TYPE\n\nVAR_GLOBAL\n    origin : Point;\nEND_VAR\n\nFUNCTION compute : DINT\nVAR_INPUT\n    value : DINT;\nEND_VAR\n    compute := value * 2;\nEND_FUNCTION\n\nFUNCTION main : DINT\nVAR\n    p : Point;\n    n : DINT;\n    flag : BOOL;\nEND_VAR\n    n := p.x;\n    \nEND_FUNCTION\n";

#[test]
fn completion_probe() {
    let tmp = tempdir_with_main();
    let main_uri = file_uri(&tmp, "main.st");

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    initialize(&client_conn, &tmp);
    send_did_open(&client_conn, &main_uri, MAIN_ST);
    // Clean compiles don't emit publishDiagnostics (post-13 follow-up
    // item 6 — server only publishes on a delta). Sleep to let the
    // compile worker attach annotated to ServerState, matching the
    // workaround used by other probes (rename_probe etc.).
    thread::sleep(Duration::from_secs(2));

    // --- Member access: `p.` (inside main, after `n := p.x;` line we
    //     position the cursor right after `p.`). The buffer already has
    //     `p.x` on that line — we use character index immediately after
    //     the `.`. Member context, expects `x`, `y`, `name` from Point.
    let dot_pos = Position { line: 23, character: 11 };
    let dot_items = completion(&client_conn, &main_uri, dot_pos, Some(2), Some("."));
    let dot_labels = labels_of(&dot_items);
    assert!(dot_labels.contains(&"x".to_string()), "member access: x missing in {dot_labels:?}");
    assert!(dot_labels.contains(&"y".to_string()), "member access: y missing in {dot_labels:?}");
    assert!(dot_labels.contains(&"name".to_string()), "member access: name missing in {dot_labels:?}");
    // Dot-trigger routing: only members, no keywords / locals / POUs.
    assert!(!dot_labels.contains(&"IF".to_string()), "dot trigger leaked keyword IF");
    assert!(!dot_labels.contains(&"main".to_string()), "dot trigger leaked POU name");
    assert!(!dot_labels.contains(&"flag".to_string()), "dot trigger leaked local 'flag'");

    // --- Same position, but ctrl-space (Invoked). Detector still sees
    //     `.` immediately before the cursor → still member context.
    let invoked = completion(&client_conn, &main_uri, dot_pos, Some(1), None);
    let invoked_labels = labels_of(&invoked);
    assert!(invoked_labels.contains(&"x".to_string()));

    // --- Statement position (ctrl-space on the empty body line right
    //     after `n := p.x;`). The previous non-whitespace byte is the
    //     `;` ending that statement → Statement context. Emits locals,
    //     globals, POU names, statement keywords.
    let stmt_items = completion(&client_conn, &main_uri, Position { line: 24, character: 4 }, Some(1), None);
    let stmt_labels = labels_of(&stmt_items);
    assert!(stmt_labels.contains(&"p".to_string()), "stmt: local p missing");
    assert!(stmt_labels.contains(&"n".to_string()), "stmt: local n missing");
    assert!(stmt_labels.contains(&"flag".to_string()), "stmt: local flag missing");
    assert!(stmt_labels.contains(&"origin".to_string()), "stmt: global origin missing");
    assert!(stmt_labels.contains(&"compute".to_string()), "stmt: POU compute missing");
    assert!(stmt_labels.contains(&"IF".to_string()), "stmt: keyword IF missing");
    assert!(stmt_labels.contains(&"FOR".to_string()), "stmt: keyword FOR missing");

    eprintln!("\n=== completion_probe ===");
    eprintln!("member-access items @ p.|  ({}):", dot_labels.len());
    for l in &dot_labels {
        eprintln!("    {l}");
    }
    eprintln!("\nstatement items @ body-start  ({}):", stmt_labels.len());
    for l in stmt_labels.iter().take(20) {
        eprintln!("    {l}");
    }

    shutdown(&client_conn);
    server_thread.join().expect("server thread").expect("server returned error");
}

// --- helpers ---

fn tempdir_with_main() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{"name":"completion-probe","files":["*.st"]}"#).unwrap();
    fs::write(tmp.path().join("main.st"), MAIN_ST).unwrap();
    tmp
}

fn file_uri(tmp: &TempDir, name: &str) -> Uri {
    format!("file://{}/{name}", tmp.path().display()).parse().unwrap()
}

fn initialize(client: &Connection, tmp: &TempDir) {
    let workspace_uri: Uri = format!("file://{}", tmp.path().display()).parse().unwrap();
    let mut params = InitializeParams {
        workspace_folders: Some(vec![WorkspaceFolder {
            uri: workspace_uri,
            name: "completion-probe".to_string(),
        }]),
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

fn completion(
    client: &Connection,
    uri: &Uri,
    position: Position,
    trigger_kind: Option<u32>,
    trigger_char: Option<&str>,
) -> Value {
    static NEXT_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(100);
    let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let context = if let Some(kind) = trigger_kind {
        let mut c = json!({ "triggerKind": kind });
        if let Some(ch) = trigger_char {
            c["triggerCharacter"] = json!(ch);
        }
        Some(c)
    } else {
        None
    };
    let params = json!({
        "textDocument": { "uri": uri.as_str() },
        "position": { "line": position.line, "character": position.character },
        "context": context,
    });
    client
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(id),
            method: "textDocument/completion".to_string(),
            params,
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
            Err(_) => panic!("timed out waiting for completion response"),
        }
    }
}

fn labels_of(response: &Value) -> Vec<String> {
    // Response may be CompletionList (object with `items`) or a bare array.
    let items = response.get("items").and_then(|v| v.as_array()).or_else(|| response.as_array());
    items
        .map(|arr| {
            arr.iter().filter_map(|i| i.get("label").and_then(|l| l.as_str()).map(String::from)).collect()
        })
        .unwrap_or_default()
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
