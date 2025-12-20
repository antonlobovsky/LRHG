use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use ta::indicators::{AverageDirectionalIndex, AverageTrueRange}; // Both now correctly named
use ta::Next;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🟢 STARTING BTC VOLATILITY & TREND ENGINE ---");

    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;

    let candles = info_client
        .candles_snapshot("BTC".to_string(), "15m".to_string(), 0, 0)
        .await?;

    // 1. Initialize Indicators (14-period standard)
    let mut atr = AverageTrueRange::new(14).unwrap();
    let mut adx = AverageDirectionalIndex::new(14).unwrap();
    
    let mut current_atr = 0.0;
    let mut current_adx = 0.0;

    // 2. Feed data
    for candle in candles.iter() {
        let h: f64 = candle.high.parse().unwrap_or(0.0);
        let l: f64 = candle.low.parse().unwrap_or(0.0);
        let c: f64 = candle.close.parse().unwrap_or(0.0);
        
        current_atr = atr.next((h, l, c));
        current_adx = adx.next((h, l, c));
    }

    // 3. Print Results
    println!("📊 BTC 15m ATR: ${:.2}", current_atr);
    println!("📊 BTC 15m ADX: {:.2}", current_adx);

    // 4. Combined Strategy Logic
    if current_adx > 25.0 && current_atr > 50.0 {
        println!("🚀 SIGNAL: Strong trend WITH high volatility. Perfect for entry!");
    } else if current_adx < 20.0 {
        println!("😴 SIGNAL: Market is ranging. Do not enter trend trades.");
    }

    Ok(())
}
