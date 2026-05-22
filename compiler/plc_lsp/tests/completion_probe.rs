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

// MAIN_ST line layout (0-based, matches Position.line in LSP). Designed
// so each P13.7 probe lands on a deliberate cursor position with the
// shape it exercises.
//
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
//  20:      points : ARRAY[1..3] OF Point;
//  21:      n : DINT;
//  22:      flag : BOOL;
//  23:  END_VAR
//  24:      n := p.x;
//  25:                              ← statement-position probe lands here
//  26:      FOR n := 1 TO          ← FOR-counter type-hint probe lands here
//  27:          ;                  (closes the FOR)
//  28:      END_FOR;
//  29:      WHILE                   ← WHILE-condition type-hint probe lands here
//  30:          DO
//  31:          ;
//  32:      END_WHILE;
//  33:      n := points[                ← array-index DINT-hint probe lands here
//  34:                                  (closes the index)
//  35:      ];
//  35:      n := points[1].             ← array-element member-access probe lands here
//  36:      ;
//  37:      n := compute(               ← call-site probe lands here (line 37, char 17)
//  38:      );
//  39:      n := n.                     ← scalar/no-members probe lands here (line 39, char 11)
//  40:      ;
//  41:  END_FUNCTION
//
// Type-position probe lands on line 12 ("    value : DINT;") char 11 —
// the byte right after the `:` in `value :`. Detector sees `:` as the
// preceding non-whitespace byte → TypePosition.
//
// Top-level probe lands at line 0, character 0 — file start. Detector
// walks back through (no) whitespace, lands at idx 0 → TopLevel.
const MAIN_ST: &str = "TYPE Point : STRUCT\n    x : DINT;\n    y : DINT;\n    name : STRING;\nEND_STRUCT END_TYPE\n\nVAR_GLOBAL\n    origin : Point;\nEND_VAR\n\nFUNCTION compute : DINT\nVAR_INPUT\n    value : DINT;\nEND_VAR\n    compute := value * 2;\nEND_FUNCTION\n\nFUNCTION main : DINT\nVAR\n    p : Point;\n    points : ARRAY[1..3] OF Point;\n    n : DINT;\n    flag : BOOL;\nEND_VAR\n    n := p.x;\n    \n    FOR n := 1 TO \n        ;\n    END_FOR;\n    WHILE \n        DO\n        ;\n    END_WHILE;\n    n := points[\n    ];\n    n := points[1].\n    ;\n    n := compute(\n    );\n    n := n.\n    ;\nEND_FUNCTION\n";

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
    // --- Case 1: Struct member access (`p.|`). Dot-trigger restricts
    //     output to members only — no keywords / locals / POUs.
    let dot_pos = Position { line: 24, character: 11 };
    let dot_items = completion(&client_conn, &main_uri, dot_pos, Some(2), Some("."));
    let dot_labels = labels_of(&dot_items);
    assert_eq!(dot_labels.len(), 3, "struct member access should emit exactly 3 fields: {dot_labels:?}");
    assert!(dot_labels.contains(&"x".to_string()));
    assert!(dot_labels.contains(&"y".to_string()));
    assert!(dot_labels.contains(&"name".to_string()));
    assert!(!dot_labels.contains(&"IF".to_string()), "dot trigger leaked keyword IF");
    assert!(!dot_labels.contains(&"main".to_string()), "dot trigger leaked POU name");

    // --- Case 2: Same position via ctrl-space — still member context.
    let invoked = completion(&client_conn, &main_uri, dot_pos, Some(1), None);
    assert!(labels_of(&invoked).contains(&"x".to_string()));

    // --- Case 3: Statement position (ctrl-space on empty body line).
    //     Previous non-whitespace byte is `;` → Statement. Emits
    //     locals + globals + POU names + statement keywords. The
    //     synthesised lowering POUs (`__unit_xxx__ctor`, `Point__ctor`)
    //     must be filtered out per the P13.7 noise pass.
    let stmt_items = completion(&client_conn, &main_uri, Position { line: 25, character: 4 }, Some(1), None);
    let stmt_labels = labels_of(&stmt_items);
    assert!(stmt_labels.contains(&"p".to_string()), "stmt: local p missing");
    assert!(stmt_labels.contains(&"points".to_string()), "stmt: local points missing");
    assert!(stmt_labels.contains(&"origin".to_string()), "stmt: global origin missing");
    assert!(stmt_labels.contains(&"compute".to_string()), "stmt: POU compute missing");
    assert!(stmt_labels.contains(&"IF".to_string()), "stmt: keyword IF missing");
    assert!(stmt_labels.contains(&"FOR".to_string()), "stmt: keyword FOR missing");
    for l in &stmt_labels {
        assert!(!l.contains("__ctor"), "stmt: generated __ctor POU leaked: {l}");
        assert!(!l.starts_with("__"), "stmt: synthesised __-prefixed POU leaked: {l}");
    }

    // --- Case 4: FOR counter type-hint. `FOR n := 1 TO ⎵` with `n :
    //     DINT`. Hint = DINT, so DINT-typed items get sortText with a
    //     leading `-` and float above BOOL-typed items in tier 0.
    let for_items = completion(&client_conn, &main_uri, Position { line: 26, character: 18 }, Some(1), None);
    let for_labels_with_sort = items_with_sort(&for_items);
    let n_sort = sort_of(&for_labels_with_sort, "n");
    let flag_sort = sort_of(&for_labels_with_sort, "flag");
    assert!(n_sort.is_some(), "FOR hint: local n missing");
    assert!(flag_sort.is_some(), "FOR hint: local flag missing");
    let n_sort = n_sort.unwrap();
    let flag_sort = flag_sort.unwrap();
    assert!(
        n_sort.starts_with('-'),
        "FOR DINT hint: n should have type-match discount; got sortText={n_sort:?}"
    );
    assert!(
        !flag_sort.starts_with('-'),
        "FOR DINT hint: flag (BOOL) should NOT have discount; got {flag_sort:?}"
    );

    // --- Case 5: WHILE condition type-hint. Hint = BOOL.
    let while_items =
        completion(&client_conn, &main_uri, Position { line: 29, character: 10 }, Some(1), None);
    let while_sorts = items_with_sort(&while_items);
    let flag_sort = sort_of(&while_sorts, "flag").expect("flag missing in WHILE hint");
    let n_sort = sort_of(&while_sorts, "n").expect("n missing in WHILE hint");
    assert!(flag_sort.starts_with('-'), "WHILE BOOL hint: flag should have discount; got {flag_sort:?}");
    assert!(!n_sort.starts_with('-'), "WHILE BOOL hint: n (DINT) should NOT have discount; got {n_sort:?}");

    // --- Case 6: Array index DINT-hint. `points[⎵`.
    let idx_items = completion(&client_conn, &main_uri, Position { line: 33, character: 16 }, Some(1), None);
    let idx_sorts = items_with_sort(&idx_items);
    let n_sort = sort_of(&idx_sorts, "n").expect("n missing in array-index hint");
    assert!(n_sort.starts_with('-'), "array-index DINT hint: n should have discount; got {n_sort:?}");

    // --- Case 7: Array-element member access. `points[1].⎵` — element
    //     type is Point, members `x`, `y`, `name` should all appear.
    let arr_mem_items =
        completion(&client_conn, &main_uri, Position { line: 35, character: 19 }, Some(2), Some("."));
    let arr_mem_labels = labels_of(&arr_mem_items);
    assert!(
        arr_mem_labels.contains(&"x".to_string()),
        "array-element member access: x missing in {arr_mem_labels:?}"
    );
    assert!(arr_mem_labels.contains(&"y".to_string()));
    assert!(arr_mem_labels.contains(&"name".to_string()));

    // --- Case 8 (Q7-5): Call site. `compute(⎵` — emits compute's
    //     parameters (value : DINT) as tier-0 named-arg candidates plus
    //     in-scope locals as tier-1 positional-arg candidates. The named
    //     arg's label carries the `:=` separator so the user sees the
    //     direction in the completion list before accepting.
    let call_items = completion(&client_conn, &main_uri, Position { line: 37, character: 17 }, Some(1), None);
    let call_labels = labels_of(&call_items);
    assert!(
        call_labels.contains(&"value :=".to_string()),
        "call site: compute's input param `value :=` missing in {call_labels:?}"
    );
    assert!(call_labels.contains(&"n".to_string()), "call site: caller local `n` missing");
    assert!(call_labels.contains(&"origin".to_string()), "call site: global `origin` missing");

    // --- Case 8b (L14 / L14b): Callee parameter shows the named-arg
    //     separator in BOTH the label (UX: user sees the direction) and
    //     the insert text. filterText stays bare so fuzzy matching still
    //     keys on `value`. Caller-scope items (locals, globals) get no
    //     separator — they're positional candidates.
    let value_item = call_items
        .get("items")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.iter().find(|i| i.get("label").and_then(|l| l.as_str()) == Some("value :=")))
        .expect("value's completion item should be present");
    let value_insert = value_item.get("insertText").and_then(|t| t.as_str()).expect("insertText");
    assert_eq!(
        value_insert, "value := ",
        "VAR_INPUT param insert should end with `:= `, got {value_insert:?}"
    );
    let value_filter = value_item.get("filterText").and_then(|t| t.as_str()).expect("filterText");
    assert_eq!(value_filter, "value", "filterText must stay bare for fuzzy matching, got {value_filter:?}");

    let n_insert = call_items
        .get("items")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.iter().find(|i| i.get("label").and_then(|l| l.as_str()) == Some("n")))
        .and_then(|i| i.get("insertText").and_then(|t| t.as_str()).map(String::from))
        .expect("n's insert_text should be present");
    assert_eq!(n_insert, "n", "caller-scope local should insert bare name, got {n_insert:?}");

    // --- Case 9 (Q7-3): Scalar/no-members. `n.⎵` where n is DINT —
    //     emits an empty list (DINT has no struct members, no methods).
    let scalar_items =
        completion(&client_conn, &main_uri, Position { line: 39, character: 11 }, Some(2), Some("."));
    let scalar_labels = labels_of(&scalar_items);
    assert!(
        scalar_labels.is_empty(),
        "scalar member access on DINT should emit empty list; got {scalar_labels:?}"
    );

    // --- Case 10 (Q7-10): Type-position. Cursor right after the `:` in
    //     `value : DINT;` (line 12 char 11). Emits only types — no
    //     locals, no statement keywords. Labels preserve original case
    //     by reading `DataType.name` (L8 fix); the SymbolMap key alone
    //     would yield lowercase.
    let type_items = completion(&client_conn, &main_uri, Position { line: 12, character: 11 }, Some(1), None);
    let type_labels = labels_of(&type_items);
    assert!(type_labels.contains(&"DINT".to_string()), "type pos: DINT missing in {type_labels:?}");
    assert!(type_labels.contains(&"BOOL".to_string()), "type pos: BOOL missing");
    assert!(type_labels.contains(&"Point".to_string()), "type pos: user type Point missing");
    assert!(!type_labels.contains(&"IF".to_string()), "type pos: leaked statement keyword");
    assert!(!type_labels.contains(&"p".to_string()), "type pos: leaked local `p`");
    for l in &type_labels {
        assert!(!l.starts_with("__") && !l.ends_with("__ctor"), "type pos: synthetic type leaked: {l}");
    }

    // --- Case 11 (Q7-11): Top-level. Cursor at file start. Emits
    //     POU-start + top-level declaration keywords only.
    let top_items = completion(&client_conn, &main_uri, Position { line: 0, character: 0 }, Some(1), None);
    let top_labels = labels_of(&top_items);
    assert!(top_labels.contains(&"PROGRAM".to_string()), "top-level: PROGRAM missing");
    assert!(top_labels.contains(&"FUNCTION".to_string()), "top-level: FUNCTION missing");
    assert!(top_labels.contains(&"FUNCTION_BLOCK".to_string()), "top-level: FUNCTION_BLOCK missing");
    assert!(top_labels.contains(&"TYPE".to_string()), "top-level: TYPE missing");
    assert!(top_labels.contains(&"VAR_GLOBAL".to_string()), "top-level: VAR_GLOBAL missing");
    assert!(!top_labels.contains(&"IF".to_string()), "top-level: leaked statement keyword IF");
    assert!(!top_labels.contains(&"main".to_string()), "top-level: leaked POU name `main`");

    eprintln!("\n=== completion_probe ===");
    eprintln!("Case 1: struct member access `p.|`  ({}): {:?}", dot_labels.len(), dot_labels);
    eprintln!("Case 3: statement-position  ({}):", stmt_labels.len());
    for l in stmt_labels.iter().take(20) {
        eprintln!("    {l}");
    }
    eprintln!(
        "Case 4: FOR DINT hint — n sortText {n_sort:?}, flag sortText {flag_sort:?}",
        n_sort = sort_of(&for_labels_with_sort, "n").unwrap(),
        flag_sort = sort_of(&for_labels_with_sort, "flag").unwrap()
    );
    eprintln!("Case 7: `points[1].|`  ({}): {:?}", arr_mem_labels.len(), arr_mem_labels);
    eprintln!("Case 8: call-site `compute(|`  ({}): {:?}", call_labels.len(), call_labels);
    eprintln!("Case 9: scalar `n.|` (no members)  ({})", scalar_labels.len());
    eprintln!(
        "Case 10: type-position  ({}): {:?}",
        type_labels.len(),
        &type_labels[..type_labels.len().min(15)]
    );
    eprintln!("Case 11: top-level  ({}): {:?}", top_labels.len(), top_labels);

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

/// Returns `(label, sortText)` for every item in the response. Used by
/// ranking-assertion probes that need to inspect the type-hint discount.
fn items_with_sort(response: &Value) -> Vec<(String, String)> {
    let items = response.get("items").and_then(|v| v.as_array()).or_else(|| response.as_array());
    items
        .map(|arr| {
            arr.iter()
                .filter_map(|i| {
                    let label = i.get("label").and_then(|l| l.as_str())?.to_string();
                    let sort = i.get("sortText").and_then(|s| s.as_str())?.to_string();
                    Some((label, sort))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn sort_of(items: &[(String, String)], label: &str) -> Option<String> {
    items.iter().find(|(l, _)| l == label).map(|(_, s)| s.clone())
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
