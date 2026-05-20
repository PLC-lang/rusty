//! Phase 2 integration test: drive document notifications through the
//! server and verify the lifecycle remains clean.
//!
//! State correctness lives in the document.rs unit tests; this file
//! exercises the dispatch + (de)serialisation path that the unit tests
//! can't reach.

use std::thread;

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::InitializeParams;
use serde_json::json;

#[test]
fn document_notifications_survive_through_to_shutdown() {
    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    // --- initialize + initialized ---
    client_conn
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(1),
            method: "initialize".to_string(),
            params: serde_json::to_value(InitializeParams::default()).unwrap(),
        }))
        .unwrap();
    expect_response(&client_conn);
    notification(&client_conn, "initialized", json!({}));

    // didOpen with an accepted language_id.
    notification(
        &client_conn,
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": "file:///plc/main.st",
                "languageId": "structured-text",
                "version": 1,
                "text": "PROGRAM main; END_PROGRAM"
            }
        }),
    );

    // didOpen with a rejected language_id — should be silently dropped by
    // the filter, no panic, no state change.
    notification(
        &client_conn,
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": "file:///some/foo.rs",
                "languageId": "rust",
                "version": 1,
                "text": "fn main() {}"
            }
        }),
    );

    // didChange (Full sync — single change with no range).
    notification(
        &client_conn,
        "textDocument/didChange",
        json!({
            "textDocument": { "uri": "file:///plc/main.st", "version": 2 },
            "contentChanges": [{ "text": "PROGRAM main_v2; END_PROGRAM" }]
        }),
    );

    // didSave — phase 2 no-op, but the dispatch arm must still match.
    notification(
        &client_conn,
        "textDocument/didSave",
        json!({ "textDocument": { "uri": "file:///plc/main.st" } }),
    );

    // didClose — drops the buffer.
    notification(
        &client_conn,
        "textDocument/didClose",
        json!({ "textDocument": { "uri": "file:///plc/main.st" } }),
    );

    // Malformed didOpen — server should log a warning and continue, not crash.
    notification(&client_conn, "textDocument/didOpen", json!({ "garbage": true }));

    // --- shutdown + exit ---
    client_conn
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(2),
            method: "shutdown".to_string(),
            params: json!(null),
        }))
        .unwrap();
    expect_response(&client_conn);
    notification(&client_conn, "exit", json!(null));

    let result = server_thread.join().expect("server thread panicked");
    result.expect("server returned an error from a clean lifecycle with document traffic");
}

fn notification(client: &Connection, method: &str, params: serde_json::Value) {
    client.sender.send(Message::Notification(Notification { method: method.to_string(), params })).unwrap();
}

fn expect_response(client: &Connection) -> lsp_server::Response {
    // Drain server-initiated notifications / requests (e.g. window/showMessage,
    // client/registerCapability) while waiting for the response we asked for.
    loop {
        if let Message::Response(r) = client.receiver.recv().expect("server closed channel before responding")
        {
            return r;
        }
    }
}
