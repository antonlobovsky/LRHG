use hyperliquid_rust_sdk::{BaseUrl, InfoClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🟢 STARTING BTC PRICE TICKER ---");

    // 1. Initialize the client to talk to Hyperliquid
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;

    // 2. Fetch the "All Mids" snapshot (all current prices)
    let all_mids = info_client.all_mids().await?;

    // 3. Extract BTC price from the map
    if let Some(price) = all_mids.get("BTC") {
        println!("🚀 LIVE BTC PRICE: ${}", price);
        
        // Simple logic: Is it a "round number" day?
        let price_f64: f64 = price.parse().unwrap_or(0.0);
        if price_f64 > 100000.0 {
            println!("🌕 BTC is in the six-figures!");
        }
    } else {
        println!("❌ Could not find BTC ticker.");
    }

    Ok(())
}
