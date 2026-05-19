//! Phase 1 smoke test: drive the server through the full lifecycle
//! (initialize → initialized → shutdown → exit) using `Connection::memory()`
//! so no actual stdio or subprocess is involved.

use std::thread;

use insta::assert_snapshot;
use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::InitializeParams;
use serde_json::json;

#[test]
fn lifecycle_initialize_then_shutdown() {
    let (server_conn, client_conn) = Connection::memory();

    // Server runs on a worker thread so we can drive both sides from one test.
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn));

    // --- initialize ---------------------------------------------------------
    let initialize_id = RequestId::from(1);
    client_conn
        .sender
        .send(Message::Request(Request {
            id: initialize_id.clone(),
            method: "initialize".to_string(),
            params: serde_json::to_value(InitializeParams::default()).unwrap(),
        }))
        .unwrap();

    let initialize_response = pretty_response(&expect_response(&client_conn));
    assert_snapshot!(initialize_response, @r#"
    {
      "id": 1,
      "result": {
        "capabilities": {
          "positionEncoding": "utf-8",
          "textDocumentSync": 1
        },
        "serverInfo": {
          "name": "plc-lsp",
          "version": "<pkg-version>"
        }
      }
    }
    "#);

    // --- initialized --------------------------------------------------------
    client_conn
        .sender
        .send(Message::Notification(Notification { method: "initialized".to_string(), params: json!({}) }))
        .unwrap();

    // --- shutdown -----------------------------------------------------------
    let shutdown_id = RequestId::from(2);
    client_conn
        .sender
        .send(Message::Request(Request {
            id: shutdown_id.clone(),
            method: "shutdown".to_string(),
            params: json!(null),
        }))
        .unwrap();

    let shutdown_response = pretty_response(&expect_response(&client_conn));
    assert_snapshot!(shutdown_response, @r#"
    {
      "id": 2,
      "result": null
    }
    "#);

    // --- exit ---------------------------------------------------------------
    // `handle_shutdown` on the server side blocks waiting for the exit
    // notification after replying to shutdown, so we have to send it for the
    // server to return cleanly.
    client_conn
        .sender
        .send(Message::Notification(Notification { method: "exit".to_string(), params: json!(null) }))
        .unwrap();

    let server_result = server_thread.join().expect("server thread panicked");
    server_result.expect("server returned an error from a clean lifecycle");
}

fn expect_response(client: &Connection) -> lsp_server::Response {
    match client.receiver.recv().expect("server closed channel before responding") {
        Message::Response(r) => r,
        other => panic!("expected a response, got {other:?}"),
    }
}

/// Pretty-print a response as JSON, with the package version normalised so
/// snapshots don't churn on every version bump.
fn pretty_response(response: &lsp_server::Response) -> String {
    serde_json::to_string_pretty(response).unwrap().replace(env!("CARGO_PKG_VERSION"), "<pkg-version>")
}
