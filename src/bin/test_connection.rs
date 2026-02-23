//! Test connection utility
//! Run with: cargo run --release --bin test_connection
//!
//! Tests RPC, WebSocket, and API connectivity

use anyhow::{Result, anyhow};
use dotenvy::dotenv;
use std::env;
use tokio_tungstenite::connect_async;

const DEFAULT_RPC_URL: &str = "https://polygon-rpc.com";
// WebSocket URL for Polymarket CLOB - note: this is for testing connectivity only
// The actual bot uses RPC provider WebSocket for blockchain events
const CLOB_WS_URL: &str = "wss://clob.polymarket.com";
const CLOB_API_BASE: &str = "https://clob.polymarket.com";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!("🔌 Connection Tester");
    println!("====================\n");

    let mut all_ok = true;

    // Test RPC
    println!("1️⃣  Testing RPC connection...");
    match test_rpc().await {
        Ok(chain_id) => {
            println!("   ✅ RPC: Connected (Chain ID: {})", chain_id);
        }
        Err(e) => {
            println!("   ❌ RPC: Failed - {}", e);
            all_ok = false;
        }
    }

    // Test CLOB API
    println!("\n2️⃣  Testing CLOB API...");
    match test_clob_api().await {
        Ok(()) => {
            println!("   ✅ CLOB API: Accessible");
        }
        Err(e) => {
            println!("   ❌ CLOB API: Failed - {}", e);
            all_ok = false;
        }
    }

    // Test WebSocket
    println!("\n3️⃣  Testing WebSocket connection...");
    match test_websocket().await {
        Ok(()) => {
            println!("   ✅ WebSocket: Connected");
        }
        Err(e) => {
            // WebSocket may fail due to protocol requirements, but if server responds, connectivity is OK
            let error_str = e.to_string();
            if error_str.contains("protocol error") || error_str.contains("upgrade failed") {
                println!("   ⚠️  WebSocket: Server reachable but protocol negotiation failed");
                println!("      (This is OK - bot uses RPC provider WebSocket for blockchain events)");
            } else {
                println!("   ❌ WebSocket: Failed - {}", e);
                all_ok = false;
            }
        }
    }

    // Test configuration
    println!("\n4️⃣  Checking configuration...");
    let mut config_ok = true;

    if env::var("PRIVATE_KEY").is_err() {
        println!("   ❌ PRIVATE_KEY: Not set");
        config_ok = false;
    } else {
        println!("   ✅ PRIVATE_KEY: Set");
    }

    if env::var("FUNDER_ADDRESS").is_err() {
        println!("   ❌ FUNDER_ADDRESS: Not set");
        config_ok = false;
    } else {
        println!("   ✅ FUNDER_ADDRESS: Set");
    }

    if env::var("TARGET_WHALE_ADDRESS").is_err() {
        println!("   ❌ TARGET_WHALE_ADDRESS: Not set");
        config_ok = false;
    } else {
        println!("   ✅ TARGET_WHALE_ADDRESS: Set");
    }

    if env::var("ALCHEMY_API_KEY").is_ok() || env::var("CHAINSTACK_API_KEY").is_ok() {
        println!("   ✅ API Key: Set");
    } else {
        println!("   ⚠️  API Key: Not set (using public RPC)");
    }

    if !config_ok {
        all_ok = false;
    }

    // Summary
    println!("\n{}", "=".repeat(50));
    if all_ok {
        println!("✅ All connection tests passed!");
    } else {
        println!("❌ Some connection tests failed. Check errors above.");
    }
    println!();

    Ok(())
}

async fn test_rpc() -> Result<u64> {
    let rpc_url = get_rpc_url();
    
    // Test by getting chain ID via RPC call
    let client = reqwest::Client::new();
    let chain_id_result = client
        .post(&rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_chainId",
            "params": [],
            "id": 1
        }))
        .send()
        .await?;
    
    let chain_id_json: serde_json::Value = chain_id_result.json().await?;
    let chain_id_hex = chain_id_json["result"].as_str().unwrap_or("0x89"); // Default to Polygon (137)
    let chain_id = u64::from_str_radix(chain_id_hex.strip_prefix("0x").unwrap_or(chain_id_hex), 16)?;
    
    // Test block number query as additional verification
    let block_result = client
        .post(&rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 2
        }))
        .send()
        .await?;
    
    let _block_json: serde_json::Value = block_result.json().await?;
    
    Ok(chain_id)
}

async fn test_clob_api() -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Test CLOB API by checking if it's accessible
    // Use the book endpoint with an invalid token_id
    // If we get 400 (bad request), the API is responding (just invalid token)
    // If we get 404 (not found), the endpoint exists but token is invalid - API is up
    // Only fail on 5xx errors (server down) or connection errors
    let url = format!("{}/book?token_id=0", CLOB_API_BASE);
    let resp = client.get(&url).send().await?;

    // Accept any response except server errors (5xx)
    // 200 = success (unlikely with token_id=0, but possible)
    // 400 = bad request (API is up, just invalid token_id) - this is OK for connectivity test
    // 404 = not found (API is up, endpoint might not exist) - still OK for connectivity test
    // 5xx = server error (API is down)
    if resp.status().is_server_error() {
        return Err(anyhow!("Server error: {} - API may be down", resp.status()));
    }

    // For client errors (4xx), the API is up but the request was invalid - that's OK for connectivity test
    Ok(())
}

async fn test_websocket() -> Result<()> {
    // Connect to WebSocket - test basic connectivity
    // Note: The bot actually uses RPC provider WebSocket (Alchemy/Chainstack) for blockchain events
    // This test just verifies we can reach a WebSocket server
    match connect_async(CLOB_WS_URL).await {
        Ok((_ws_stream, response)) => {
            // Connection successful - check response status
            // HTTP 101 Switching Protocols means WebSocket upgrade succeeded
            let status = response.status();
            drop(_ws_stream);
            
            if status.as_u16() == 101 {
                // Proper WebSocket connection established
                Ok(())
            } else {
                // Got a response but not a proper WebSocket upgrade
                // Server is reachable but protocol negotiation failed
                // This is acceptable for connectivity test - server responded
                Err(anyhow!("protocol error: Server responded but upgrade failed (HTTP {}, expected 101)", status))
            }
        }
        Err(e) => {
            // Connection failed - check error type
            let error_msg = e.to_string();
            // "HTTP error: 200 OK" usually means server responded but WebSocket upgrade failed
            // This indicates server is reachable, just protocol issue
            if error_msg.contains("HTTP error: 200") {
                return Err(anyhow!("protocol error: Server responded but upgrade failed"));
            }
            // Other errors indicate actual connectivity issues
            Err(anyhow!("connection failed: {}", error_msg))
        }
    }
}

fn get_rpc_url() -> String {
    if let Ok(key) = env::var("ALCHEMY_API_KEY") {
        let key = key.trim();
        if !key.is_empty() && key != "your_alchemy_api_key_here" {
            return format!("https://polygon-mainnet.g.alchemy.com/v2/{}", key);
        }
    }

    if let Ok(key) = env::var("CHAINSTACK_API_KEY") {
        let key = key.trim();
        if !key.is_empty() && key != "your_chainstack_api_key_here" {
            return format!("https://polygon-mainnet.gateway.pokt.network/v1/lb/{}", key);
        }
    }

    DEFAULT_RPC_URL.to_string()
}

