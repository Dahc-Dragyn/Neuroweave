use tokio::io::{self, AsyncBufReadExt, BufReader};
use serde::Deserialize;
use serde_json::json;
use std::io::Write;
use crate::ipc::IntentPayload;
use crate::router::evaluate_intent;
use crate::dispatcher::dispatch;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    method: String,
    params: Option<serde_json::Value>,
}


pub async fn run_mcp_server() {
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();

    loop {
        eprintln!("[Neuroweave MCP] Daemon listening for Stdio JSON-RPC payload...");
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                
                eprintln!("[Neuroweave MCP] Payload received. Routing to Intent Gate...");
                
                // Parse incoming request
                match serde_json::from_str::<JsonRpcRequest>(trimmed) {
                    Ok(req) => {
                        handle_rpc_request(req).await;
                    }
                    Err(err) => {
                        eprintln!("[Neuroweave MCP] JSON parsing error for payload: {}. Error: {}", trimmed, err);
                        let err_response = json!({
                            "jsonrpc": "2.0",
                            "error": {
                                "code": -32700,
                                "message": "Parse error"
                            },
                            "id": null
                        });
                        println!("{}", err_response);
                        std::io::stdout().flush().expect("Failed to flush stdout buffer");
                        eprintln!("[Neuroweave MCP] Response flushed to stdout.");
                    }
                }
            }
            Err(e) => {
                eprintln!("[Neuroweave MCP] Stdin read error: {}", e);
                break;
            }
        }
    }
}

async fn handle_rpc_request(req: JsonRpcRequest) {
    // If it's a notification (no ID), do not respond to prevent JSON-RPC spec violations
    let is_notification = req.id.is_none();
    let req_id = req.id.unwrap_or(serde_json::Value::Null);

    match req.method.as_str() {
        "initialize" => {
            let response = json!({
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "neuroweave_basal_ganglia",
                        "version": "1.0.0"
                    }
                }
            });
            println!("{}", response);
            std::io::stdout().flush().expect("Failed to flush stdout buffer");
            eprintln!("[Neuroweave MCP] Response flushed to stdout.");
        }
        "notifications/initialized" | "initialized" => {
            eprintln!("[Neuroweave MCP] Client initialized successfully.");
        }
        "tools/list" => {
            let response = json!({
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {
                    "tools": [
                        {
                            "name": "neuroweave_intercept",
                            "description": "Evaluates and intercepts development intents for potential local zero-cost subroutine execution.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": { "type": "string" },
                                    "intent_hash": { "type": "string" },
                                    "raw_prompt": { "type": "string" }
                                },
                                "required": ["session_id", "intent_hash", "raw_prompt"]
                            }
                        }
                    ]
                }
            });
            println!("{}", response);
            std::io::stdout().flush().expect("Failed to flush stdout buffer");
            eprintln!("[Neuroweave MCP] Response flushed to stdout.");
        }
        "tools/call" => {
            if let Some(params) = req.params {
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                if tool_name == "neuroweave_intercept" {
                    if let Some(args) = params.get("arguments") {
                        if let Ok(payload) = serde_json::from_value::<IntentPayload>(args.clone()) {
                            let profile = evaluate_intent(&payload);
                            match dispatch(&profile, &payload).await {
                                Ok(output) => {
                                    let content = json!([
                                        {
                                            "type": "text",
                                            "text": serde_json::to_string(&output).unwrap()
                                        }
                                    ]);
                                    let response = json!({
                                        "jsonrpc": "2.0",
                                        "id": req_id,
                                        "result": {
                                            "content": content
                                        }
                                    });
                                    println!("{}", response);
                                    std::io::stdout().flush().expect("Failed to flush stdout buffer");
                                    eprintln!("[Neuroweave MCP] Response flushed to stdout.");
                                }
                                Err(e) => {
                                    send_error(req_id, -32603, &format!("Dispatch error: {}", e));
                                }
                            }
                        } else {
                            send_error(req_id, -32602, "Invalid arguments for neuroweave_intercept");
                        }
                    } else {
                        send_error(req_id, -32602, "Arguments missing");
                    }
                } else {
                    send_error(req_id, -32601, &format!("Method not found: {}", tool_name));
                }
            } else {
                send_error(req_id, -32602, "Params missing");
            }
        }
        _ => {
            if is_notification {
                eprintln!("[Neuroweave MCP] Ignored unknown notification: {}", req.method);
            } else {
                send_error(req_id, -32601, &format!("Method not found: {}", req.method));
            }
        }
    }
}

fn send_error(id: serde_json::Value, code: i32, message: &str) {
    let response = json!({
        "jsonrpc": "2.0",
        "error": {
            "code": code,
            "message": message
        },
        "id": id
    });
    println!("{}", response);
    std::io::stdout().flush().expect("Failed to flush stdout buffer");
    eprintln!("[Neuroweave MCP] Response flushed to stdout.");
}
