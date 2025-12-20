use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use ta::indicators::AverageDirectionalIndex;
use ta::Next;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🟢 STARTING HYPERLIQUID BOT ---");

    // 1. Setup Client
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;

    // 2. Fetch BTC Candles
    let candles = info_client
        .candles_snapshot("BTC".to_string(), "15m".to_string(), 0, 0)
        .await?;

    println!("✅ Market Data Synced. Processing {} candles...", candles.len());

    // 3. Initialize ADX (14-period)
    let mut adx = AverageDirectionalIndex::new(14).unwrap();
    let mut current_adx = 0.0;

    // 4. Feed data into indicator (Fixed field names)
    for candle in candles.iter() {
        let h: f64 = candle.high.parse().unwrap_or(0.0);
        let l: f64 = candle.low.parse().unwrap_or(0.0);
        let c: f64 = candle.close.parse().unwrap_or(0.0);
        
        current_adx = adx.next((h, l, c));
    }

    println!("📊 Current BTC 15m ADX: {:.2}", current_adx);

    // 5. Strategy Check
    if current_adx > 25.0 {
        println!("🔥 TREND DETECTED: Market is moving.");
    } else {
        println!("😴 NO TREND: ADX below 25.");
    }

    Ok(())
}
