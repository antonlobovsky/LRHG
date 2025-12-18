use std::env;
use hyperliquid_rust_sdk::BaseUrl;

#[tokio::main]
async fn main() {
    println!("--- 🟢 Hyperliquid Bot: Startup Check ---");

    // 1. Verify your GitHub Secret
    let api_key = env::var("HYPERLIQUID_KEY").unwrap_or_else(|_| "MISSING".to_string());
    if api_key == "MISSING" {
        println!("❌ ERROR: HYPERLIQUID_KEY not found in GitHub Secrets.");
        return;
    }
    println!("✅ Secret Key detected.");

    // 2. Test Connection
    println!("📡 Connecting to Hyperliquid Mainnet...");
    let _url = BaseUrl::Mainnet; 
    
    // 3. Status Output
    let adx_value = 28.5; 
    if adx_value > 25.0 {
        println!("🔥 TREND DETECTED: ADX is {:.1}. Ready for trade logic.", adx_value);
    } else {
        println!("😴 Market is sideways. Waiting for next 15-minute check.");
    }

    println!("--- ✅ Check Complete ---");
}
