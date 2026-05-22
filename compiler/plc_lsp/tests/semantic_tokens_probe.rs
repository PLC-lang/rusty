//! End-to-end probe for `textDocument/semanticTokens/full`. Drives the
//! server with a small project, requests semantic tokens, decodes the
//! flat integer array, and asserts the kind tags match expectations:
//!
//! - a POU name on its declaration is tagged `function` / `class`.
//! - a `VAR` local is tagged `variable`.
//! - a `VAR_INPUT` is tagged `parameter`.
//! - a `CONSTANT` global gets the `readonly` modifier bit set.
//!
//! The probe doesn't reach into the legend by index — it deserialises
//! the InitializeResult's legend and resolves names → indices at
//! runtime, so the assertions stay legible if the legend grows.

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

const MAIN_ST: &str = r#"VAR_GLOBAL CONSTANT
    PI : LREAL := 3.14;
END_VAR

FUNCTION_BLOCK Worker
VAR_INPUT
    speed : DINT;
END_VAR
VAR
    counter : DINT;
END_VAR
    counter := counter + speed;
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    w : Worker;
END_VAR
    main := 0;
END_FUNCTION
"#;

#[test]
fn semantic_tokens_full_tags_identifiers_by_kind() {
    let tmp = tempdir_with_main();
    let main_uri = file_uri(&tmp, "main.st");

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    let init = initialize(&client_conn, &tmp);
    let legend = extract_legend(&init);

    send_did_open(&client_conn, &main_uri, MAIN_ST);
    thread::sleep(Duration::from_secs(2));

    let response = request(
        &client_conn,
        "textDocument/semanticTokens/full",
        &json!({ "textDocument": { "uri": main_uri.as_str() } }),
    );
    let data = response.get("data").and_then(|v| v.as_array()).expect("data array missing").clone();
    assert!(!data.is_empty(), "semanticTokens returned no entries");
    let tagged = decode(&data, &legend, MAIN_ST);

    // Spot-check a handful of declarations and uses.
    expect_tag(&tagged, "PI", &["variable"], &["readonly"]);
    expect_tag(&tagged, "Worker", &["class"], &[]);
    expect_tag(&tagged, "main", &["function"], &[]);
    // VAR_INPUT parameter inside FUNCTION_BLOCK Worker.
    expect_tag(&tagged, "speed", &["parameter"], &[]);
    expect_tag(&tagged, "counter", &["variable"], &[]);

    shutdown(&client_conn);
    server_thread.join().expect("server thread").expect("server returned error");
}

// --------------------------------------------------------------------
// Decode helpers
// --------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Tagged {
    text: String,
    type_name: String,
    modifiers: Vec<String>,
}

struct Legend {
    types: Vec<String>,
    modifiers: Vec<String>,
}

fn extract_legend(init: &Value) -> Legend {
    let caps = init.get("result").and_then(|r| r.get("capabilities")).expect("capabilities missing");
    let prov = caps.get("semanticTokensProvider").expect("semanticTokensProvider not advertised");
    let legend = prov.get("legend").expect("legend missing");
    let types: Vec<String> = legend
        .get("tokenTypes")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let modifiers: Vec<String> = legend
        .get("tokenModifiers")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    Legend { types, modifiers }
}

fn decode(data: &[Value], legend: &Legend, source: &str) -> Vec<Tagged> {
    let lines: Vec<&str> = source.lines().collect();
    let mut out: Vec<Tagged> = Vec::new();
    let mut line: u32 = 0;
    let mut col: u32 = 0;
    let mut i = 0;
    while i + 5 <= data.len() {
        let dl = data[i].as_u64().unwrap_or(0) as u32;
        let ds = data[i + 1].as_u64().unwrap_or(0) as u32;
        let len = data[i + 2].as_u64().unwrap_or(0) as u32;
        let ty_idx = data[i + 3].as_u64().unwrap_or(0) as usize;
        let mods_bits = data[i + 4].as_u64().unwrap_or(0) as u32;
        if dl > 0 {
            line += dl;
            col = ds;
        } else {
            col += ds;
        }
        let type_name = legend.types.get(ty_idx).cloned().unwrap_or_else(|| format!("?{ty_idx}"));
        let mut modifiers = Vec::new();
        for (bit, name) in legend.modifiers.iter().enumerate() {
            if mods_bits & (1 << bit) != 0 {
                modifiers.push(name.clone());
            }
        }
        let text = lines
            .get(line as usize)
            .and_then(|l| l.get(col as usize..(col + len) as usize))
            .unwrap_or("")
            .to_string();
        out.push(Tagged { text, type_name, modifiers });
        i += 5;
    }
    out
}

fn expect_tag(tagged: &[Tagged], text: &str, allowed_types: &[&str], required_modifiers: &[&str]) {
    let hits: Vec<&Tagged> = tagged.iter().filter(|t| t.text == text).collect();
    assert!(!hits.is_empty(), "no semantic-token entry for {text:?}. Tagged: {tagged:?}");
    let any_match = hits.iter().any(|h| {
        allowed_types.contains(&h.type_name.as_str())
            && required_modifiers.iter().all(|m| h.modifiers.iter().any(|hm| hm == m))
    });
    assert!(
        any_match,
        "no {text:?} entry matched type in {allowed_types:?} with modifiers {required_modifiers:?}; got {hits:?}"
    );
}

// --------------------------------------------------------------------
// LSP client boilerplate
// --------------------------------------------------------------------

fn tempdir_with_main() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{"name":"probe","files":["*.st"]}"#).unwrap();
    fs::write(tmp.path().join("main.st"), MAIN_ST).unwrap();
    tmp
}

fn file_uri(tmp: &TempDir, name: &str) -> Uri {
    format!("file://{}/{name}", tmp.path().display()).parse().unwrap()
}

fn initialize(client: &Connection, tmp: &TempDir) -> Value {
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
    let init = drain_until_response(client, 1);
    client
        .sender
        .send(Message::Notification(Notification { method: "initialized".to_string(), params: json!({}) }))
        .unwrap();
    init
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

fn drain_until_response(client: &Connection, expected_id: i32) -> Value {
    while let Ok(msg) = client.receiver.recv_timeout(Duration::from_secs(5)) {
        if let Message::Response(r) = msg {
            if r.id == RequestId::from(expected_id) {
                return serde_json::to_value(r).unwrap_or(Value::Null);
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
