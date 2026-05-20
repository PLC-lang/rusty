//! End-to-end test for phase 7: after the initial compile, requesting
//! `textDocument/documentSymbol` returns the file's outline.

use std::fs;
use std::thread;
use std::time::Duration;

use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    DocumentSymbolParams, DocumentSymbolResponse, InitializeParams, PartialResultParams, SymbolKind,
    TextDocumentIdentifier, Uri, WorkDoneProgressParams, WorkspaceFolder,
};
use serde_json::json;
use tempfile::tempdir;

#[test]
fn document_symbol_returns_outline_after_initial_compile() {
    let tmp = tempdir().expect("tempdir");
    fs::write(tmp.path().join("plc.json"), r#"{ "name": "outline_test", "files": ["*.st"] }"#).unwrap();
    // Deliberately includes an undefined-reference inside the POU body
    // so the validator publishes a diagnostic — that's our signal the
    // compile completed and the outline cache is populated. The POUs
    // still parse cleanly so the outline contains the structural
    // entries we assert on below.
    fs::write(
        tmp.path().join("main.st"),
        "PROGRAM Main\n  VAR x : INT; END_VAR\n  undefined_ref := 1;\nEND_PROGRAM\n\n\
         FUNCTION_BLOCK MyFB\n  VAR_INPUT in : INT; END_VAR\n  VAR_OUTPUT out : INT; END_VAR\nEND_FUNCTION_BLOCK\n",
    )
    .unwrap();

    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    let workspace_uri: Uri = format!("file://{}", tmp.path().display()).parse().unwrap();
    let init = InitializeParams {
        workspace_folders: Some(vec![WorkspaceFolder { uri: workspace_uri, name: "test".to_string() }]),
        ..Default::default()
    };

    send_request(&client_conn, 1, "initialize", serde_json::to_value(init).unwrap());
    let _initialize_resp = wait_for_response(&client_conn, 1);

    client_conn
        .sender
        .send(Message::Notification(Notification { method: "initialized".to_string(), params: json!({}) }))
        .unwrap();

    // Wait for the initial compile to publish — that's our signal the
    // outline cache is populated.
    wait_for_publish(&client_conn, Duration::from_secs(30));

    let main_uri: Uri = format!("file://{}/main.st", tmp.path().display()).parse().unwrap();
    let params = DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri: main_uri },
        partial_result_params: PartialResultParams::default(),
        work_done_progress_params: WorkDoneProgressParams::default(),
    };
    send_request(&client_conn, 2, "textDocument/documentSymbol", serde_json::to_value(params).unwrap());
    let resp = wait_for_response(&client_conn, 2);

    let result: DocumentSymbolResponse =
        serde_json::from_value(resp.result.expect("documentSymbol response should carry a result"))
            .expect("DocumentSymbolResponse should deserialise");

    let DocumentSymbolResponse::Nested(symbols) = result else {
        panic!("expected nested DocumentSymbolResponse, got {result:?}");
    };

    let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Main"), "Main should appear: got {names:?}");
    assert!(names.contains(&"MyFB"), "MyFB should appear: got {names:?}");

    let main = symbols.iter().find(|s| s.name == "Main").unwrap();
    let main_children: Vec<&str> =
        main.children.as_ref().map(|c| c.iter().map(|s| s.name.as_str()).collect()).unwrap_or_default();
    assert!(main_children.contains(&"x"), "Main's children should include x: got {main_children:?}");

    let fb = symbols.iter().find(|s| s.name == "MyFB").unwrap();
    assert_eq!(fb.kind, SymbolKind::CLASS, "MyFB should map to Class kind");
    let fb_children: Vec<&str> =
        fb.children.as_ref().map(|c| c.iter().map(|s| s.name.as_str()).collect()).unwrap_or_default();
    assert!(fb_children.contains(&"in"), "MyFB's children should include in: got {fb_children:?}");
    assert!(fb_children.contains(&"out"), "MyFB's children should include out: got {fb_children:?}");

    // Clean shutdown so the worker exits.
    send_request(&client_conn, 3, "shutdown", json!(null));
    let _ = wait_for_response(&client_conn, 3);
    client_conn
        .sender
        .send(Message::Notification(Notification { method: "exit".to_string(), params: json!(null) }))
        .unwrap();

    server_thread.join().expect("server thread panicked").expect("server returned error");
}

fn send_request(client: &Connection, id: i32, method: &str, params: serde_json::Value) {
    client
        .sender
        .send(Message::Request(Request { id: RequestId::from(id), method: method.to_string(), params }))
        .unwrap();
}

fn wait_for_response(client: &Connection, expected_id: i32) -> Response {
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    loop {
        let remaining = deadline.checked_duration_since(std::time::Instant::now()).unwrap_or(Duration::ZERO);
        match client.receiver.recv_timeout(remaining) {
            Ok(Message::Response(r)) if r.id == RequestId::from(expected_id) => return r,
            Ok(_other) => {} // drain notifications / unrelated responses
            Err(_) => panic!("timed out waiting for response id={expected_id}"),
        }
    }
}

fn wait_for_publish(client: &Connection, timeout: Duration) {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        let remaining = deadline.checked_duration_since(std::time::Instant::now()).unwrap_or(Duration::ZERO);
        match client.receiver.recv_timeout(remaining) {
            Ok(Message::Notification(n)) if n.method == "textDocument/publishDiagnostics" => return,
            Ok(_other) => {}
            Err(_) => panic!("timed out waiting for initial publishDiagnostics"),
        }
    }
}
