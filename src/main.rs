use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use ta::indicators::AverageDirectionalIndex;
use ta::Next;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🟢 Fetching Live BTC Data ---");

    // 1. Setup the client for Mainnet
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;

    // 2. Fetch the last 20 candles (15-minute timeframe)
    let candles = info_client
        .candles_snapshot("BTC".to_string(), "15m".to_string(), 0, 0)
        .await?;

    println!("✅ Received {} candles.", candles.len());

    // 3. Setup ADX (Standard 14-period)
    let mut adx = AverageDirectionalIndex::new(14).unwrap();
    let mut current_adx = 0.0;

    // 4. Feed candles into indicator
    for candle in candles.iter() {
        let h: f64 = candle.h.parse().unwrap_or(0.0);
        let l: f64 = candle.l.parse().unwrap_or(0.0);
        let c: f64 = candle.c.parse().unwrap_or(0.0);
        
        current_adx = adx.next((h, l, c));
    }

    println!("📊 Current 15m ADX: {:.2}", current_adx);

    if current_adx > 25.0 {
        println!("🔥 TREND DETECTED: Market is moving strongly.");
    } else {
        println!("😴 NO TREND: Market is sideways.");
    }

    Ok(())
}
