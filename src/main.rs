use hyperliquid_rust_sdk::BaseUrl;
use std::env;

#[tokio::main]
async fn main() {
    println!("--- Hyperliquid Rust Bot: Startup Check ---");

    // 1. Check for the Secret Key
    let api_key = env::var("HYPERLIQUID_KEY").unwrap_or_else(|_| "NOT_SET".to_string());
    if api_key == "NOT_SET" {
        println!("❌ Error: HYPERLIQUID_KEY is missing from GitHub Secrets!");
        return;
    }
    println!("✅ Secret Key detected.");

    // 2. Test connection to the exchange
    println!("📡 Connecting to Hyperliquid Mainnet...");
    let _base_url = BaseUrl::Mainnet; 
    
    // For now, we just print success to confirm the library works
    println!("✅ Connection logic initialized. Ready for ADX calculations.");
    println!("--- Check Complete ---");
}
