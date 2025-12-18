use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use ta::indicators::AverageDirectionalIndex;
use ta::Next;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🟢 Fetching Live BTC Data ---");

    // 1. Initialize the Hyperliquid Info Client for market data
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;

    // 2. Fetch the last 20 candles (15m interval)
    // snapshot(coin, interval, start_time, end_time)
    let candles = info_client
        .candles_snapshot("BTC".to_string(), "15m".to_string(), 0, 0)
        .await?;

    println!("✅ Received {} candles from Hyperliquid.", candles.len());

    // 3. Initialize ADX Indicator (standard 14-period)
    let mut adx = AverageDirectionalIndex::new(14).unwrap();
    let mut current_adx = 0.0;

    // Feed high, low, and close prices into the indicator
    for candle in candles.iter().take(20) {
        let high: f64 = candle.h.parse().unwrap_or(0.0);
        let low: f64 = candle.l.parse().unwrap_or(0.0);
        let close: f64 = candle.c.parse().unwrap_or(0.0);
        
        // AverageDirectionalIndex typically needs high/low/close data
        // For simple testing, we update it iteratively
        current_adx = adx.next((high, low, close));
    }

    println!("📊 Current 15m ADX: {:.2}", current_adx);

    // 4. Decision Logic
    if current_adx > 25.0 {
        println!("🔥 TREND DETECTED: Strong movement. Trading logic would trigger here.");
    } else {
        println!("😴 NO TREND: Market is sideways. Waiting 15 minutes.");
    }

    Ok(())
}
