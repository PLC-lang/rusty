//! Phase 1 smoke tests: drive the server through the full lifecycle
//! (initialize → initialized → shutdown → exit) using `Connection::memory()`
//! so no actual stdio or subprocess is involved.

use std::thread;

use insta::assert_snapshot;
use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::{GeneralClientCapabilities, InitializeParams, PositionEncodingKind};
use serde_json::json;

/// Runs the lifecycle handshake with the given initialize params and
/// returns the pretty-printed responses for snapshotting.
fn drive_lifecycle(params: InitializeParams) -> (String, String) {
    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    // initialize
    client_conn
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(1),
            method: "initialize".to_string(),
            params: serde_json::to_value(params).unwrap(),
        }))
        .unwrap();
    let initialize_response = pretty_response(&expect_response(&client_conn));

    // initialized
    client_conn
        .sender
        .send(Message::Notification(Notification { method: "initialized".to_string(), params: json!({}) }))
        .unwrap();

    // shutdown
    client_conn
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(2),
            method: "shutdown".to_string(),
            params: json!(null),
        }))
        .unwrap();
    let shutdown_response = pretty_response(&expect_response(&client_conn));

    // exit — handle_shutdown on the server blocks for this notification.
    client_conn
        .sender
        .send(Message::Notification(Notification { method: "exit".to_string(), params: json!(null) }))
        .unwrap();

    let server_result = server_thread.join().expect("server thread panicked");
    server_result.expect("server returned an error from a clean lifecycle");

    (initialize_response, shutdown_response)
}

#[test]
fn handshake_with_default_caps_falls_back_to_utf16() {
    // Default ClientCapabilities has no `general.position_encodings`, so the
    // server has nothing offered and must default to utf-16. This is the
    // vscode-languageclient v9 case.
    let (initialize, shutdown) = drive_lifecycle(InitializeParams::default());
    assert_snapshot!(initialize, @r#"
    {
      "id": 1,
      "result": {
        "capabilities": {
          "declarationProvider": true,
          "definitionProvider": true,
          "documentSymbolProvider": true,
          "executeCommandProvider": {
            "commands": [
              "rusty.reparseProject"
            ]
          },
          "hoverProvider": true,
          "positionEncoding": "utf-16",
          "referencesProvider": true,
          "textDocumentSync": 1
        },
        "serverInfo": {
          "name": "plc-lsp",
          "version": "<pkg-version>"
        }
      }
    }
    "#);
    assert_snapshot!(shutdown, @r#"
    {
      "id": 2,
      "result": null
    }
    "#);
}

#[test]
fn handshake_with_utf8_capable_client_negotiates_utf8() {
    // Client advertises utf-8 in general.positionEncodings — server prefers it.
    // This is the helix / nvim builtin-LSP case.
    let mut params = InitializeParams::default();
    params.capabilities.general = Some(GeneralClientCapabilities {
        position_encodings: Some(vec![PositionEncodingKind::UTF8]),
        ..Default::default()
    });

    let (initialize, _shutdown) = drive_lifecycle(params);
    assert_snapshot!(initialize, @r#"
    {
      "id": 1,
      "result": {
        "capabilities": {
          "declarationProvider": true,
          "definitionProvider": true,
          "documentSymbolProvider": true,
          "executeCommandProvider": {
            "commands": [
              "rusty.reparseProject"
            ]
          },
          "hoverProvider": true,
          "positionEncoding": "utf-8",
          "referencesProvider": true,
          "textDocumentSync": 1
        },
        "serverInfo": {
          "name": "plc-lsp",
          "version": "<pkg-version>"
        }
      }
    }
    "#);
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

/// Pretty-print a response as JSON, with the package version normalised so
/// snapshots don't churn on every version bump.
fn pretty_response(response: &lsp_server::Response) -> String {
    serde_json::to_string_pretty(response).unwrap().replace(env!("CARGO_PKG_VERSION"), "<pkg-version>")
}
