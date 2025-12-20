use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use ta::indicators::{AverageDirectionalIndex, AverageTrueRange}; 
use ta::Next;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🟢 STARTING BTC ANALYSIS BOT ---");

    // 1. Connect to Hyperliquid
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;

    // 2. Fetch the current BTC price directly
    let all_mids = info_client.all_mids().await?;
    if let Some(price) = all_mids.get("BTC") {
        println!("🚀 LIVE BTC PRICE: ${}", price);
    }

    // 3. Fetch candles for math (15-minute timeframe)
    let candles = info_client
        .candles_snapshot("BTC".to_string(), "15m".to_string(), 0, 0)
        .await?;

    // 4. Initialize indicators with full names (Standard 14-period)
    let mut adx = AverageDirectionalIndex::new(14).unwrap();
    let mut atr = AverageTrueRange::new(14).unwrap();
    let (mut last_adx, mut last_atr) = (0.0, 0.0);

    // 5. Feed data into indicators
    for candle in candles.iter() {
        let h: f64 = candle.high.parse().unwrap_or(0.0);
        let l: f64 = candle.low.parse().unwrap_or(0.0);
        let c: f64 = candle.close.parse().unwrap_or(0.0);
        
        last_adx = adx.next((h, l, c));
        last_atr = atr.next((h, l, c));
    }

    println!("📊 BTC 15m ADX (Trend Strength): {:.2}", last_adx);
    println!("📊 BTC 15m ATR (Volatility): ${:.2}", last_atr);

    // 6. Final Execution Signal
    if last_adx > 25.0 {
        println!("🔥 SIGNAL: Strong trend detected. Bot is ready.");
    } else {
        println!("😴 SIGNAL: Market is quiet. Waiting for trend.");
    }

    Ok(())
}
