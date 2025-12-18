use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use ta::indicators::AverageDirectionalIndex;
use ta::Next;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🟢 Fetching Live BTC 15m Data ---");

    // 1. Initialize client
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;

    // 2. Fetch candles (0, 0 gets the most recent data)
    let candles = info_client
        .candles_snapshot("BTC".to_string(), "15m".to_string(), 0, 0)
        .await?;

    println!("✅ Received {} candles.", candles.len());

    // 3. Initialize ADX with a 14-period window
    let mut adx = AverageDirectionalIndex::new(14).unwrap();
    let mut last_adx = 0.0;

    // 4. Feed High, Low, Close data into the ADX
    for candle in candles.iter() {
        let high: f64 = candle.high.parse().unwrap_or(0.0);
        let low: f64 = candle.low.parse().unwrap_or(0.0);
        let close: f64 = candle.close.parse().unwrap_or(0.0);
        
        last_adx = adx.next((high, low, close));
    }

    println!("📊 Current 15m ADX: {:.2}", last_adx);

    if last_adx > 25.0 {
        println!("🔥 TREND STRONG: Strategy ready to trade.");
    } else {
        println!("💤 TREND WEAK: Waiting for next 15m cycle.");
    }

    Ok(())
}
