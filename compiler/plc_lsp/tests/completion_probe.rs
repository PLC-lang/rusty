//! Completion probe (phase 13 — P13.5 skeleton). Drives the server
//! through `textDocument/completion` with both trigger kinds (Invoked +
//! TriggerCharacter ".") and verifies the dispatch + serialisation
//! round-trips cleanly. P13.5 returns empty lists; P13.6 will fill in
//! enumeration and turn this into the Q7-D structured-assertion probe
//! with the 15 fixture cases.

use std::thread;

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::{CompletionResponse, InitializeParams};
use serde_json::json;

#[test]
fn completion_dispatch_round_trips_both_trigger_kinds() {
    let (server_conn, client_conn) = Connection::memory();
    let server_thread = thread::spawn(move || plc_lsp::serve(&server_conn, plc_lsp::Settings::default()));

    // initialize + initialized
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

    // didOpen — a deliberate mid-typing buffer with `foo.` at the cursor
    // line. P13.5 doesn't enumerate items, but the byte-heuristic context
    // detector still fires and we log the detected category.
    notification(
        &client_conn,
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": "file:///plc/main.st",
                "languageId": "structured-text",
                "version": 1,
                "text": "FUNCTION main : DINT\nVAR foo : Point; END_VAR\n    foo.\nEND_FUNCTION\n"
            }
        }),
    );

    // Ctrl-space (Invoked) at the cursor position right after `foo.`.
    let completion_request = |id: i32, trigger_kind: u32, trigger_char: Option<&str>| {
        let mut context = json!({ "triggerKind": trigger_kind });
        if let Some(ch) = trigger_char {
            context["triggerCharacter"] = json!(ch);
        }
        client_conn
            .sender
            .send(Message::Request(Request {
                id: RequestId::from(id),
                method: "textDocument/completion".to_string(),
                params: json!({
                    "textDocument": { "uri": "file:///plc/main.st" },
                    "position": { "line": 2, "character": 8 },
                    "context": context,
                }),
            }))
            .unwrap();
        expect_response(&client_conn)
    };

    // triggerKind = Invoked (1) — ctrl-space.
    let invoked_response = completion_request(2, 1, None);
    let invoked: CompletionResponse =
        serde_json::from_value(invoked_response.result.expect("invoked completion must return result"))
            .expect("response must deserialise as CompletionResponse");
    let items = match invoked {
        CompletionResponse::Array(items) => items,
        CompletionResponse::List(list) => {
            assert!(!list.is_incomplete, "P13.5 skeleton must return is_incomplete=false");
            list.items
        }
    };
    assert!(items.is_empty(), "P13.5 skeleton emits empty list");

    // triggerKind = TriggerCharacter (2) with "." — auto-fire on dot.
    let trigger_response = completion_request(3, 2, Some("."));
    let trigger: CompletionResponse =
        serde_json::from_value(trigger_response.result.expect("trigger completion must return result"))
            .expect("response must deserialise as CompletionResponse");
    let items = match trigger {
        CompletionResponse::Array(items) => items,
        CompletionResponse::List(list) => {
            assert!(!list.is_incomplete, "P13.5 skeleton must return is_incomplete=false");
            list.items
        }
    };
    assert!(items.is_empty(), "P13.5 skeleton emits empty list on dot trigger too");

    // shutdown + exit
    client_conn
        .sender
        .send(Message::Request(Request {
            id: RequestId::from(99),
            method: "shutdown".to_string(),
            params: json!(null),
        }))
        .unwrap();
    expect_response(&client_conn);
    notification(&client_conn, "exit", json!(null));

    let result = server_thread.join().expect("server thread panicked");
    result.expect("server returned an error from a clean lifecycle with completion traffic");
}

fn notification(client: &Connection, method: &str, params: serde_json::Value) {
    client.sender.send(Message::Notification(Notification { method: method.to_string(), params })).unwrap();
}

fn expect_response(client: &Connection) -> lsp_server::Response {
    loop {
        if let Message::Response(r) = client.receiver.recv().expect("server closed channel before responding")
        {
            return r;
        }
    }
}
