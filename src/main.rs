use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use ta::indicators::{AverageDirectionalIndex, AverageTrueRange}; // MUST use full names
use ta::Next;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🟢 STARTING BTC ANALYSIS BOT ---");

    // 1. Connect to Hyperliquid
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;

    // 2. Get Live Price (This is what you already have working)
    let all_mids = info_client.all_mids().await?;
    if let Some(price) = all_mids.get("BTC") {
        println!("🚀 LIVE BTC PRICE: ${}", price);
    }

    // 3. Get 15m Candles for the Math
    let candles = info_client
        .candles_snapshot("BTC".to_string(), "15m".to_string(), 0, 0)
        .await?;

    // 4. Initialize the Indicators (14-period is standard)
    let mut adx = AverageDirectionalIndex::new(14).unwrap();
    let mut atr = AverageTrueRange::new(14).unwrap();
    let (mut current_adx, mut current_atr) = (0.0, 0.0);

    // 5. Feed the candles into the indicators
    for candle in candles.iter() {
        // Hyperliquid prices are strings, we must parse them to numbers (f64)
        let h: f64 = candle.high.parse().unwrap_or(0.0);
        let l: f64 = candle.low.parse().unwrap_or(0.0);
        let c: f64 = candle.close.parse().unwrap_or(0.0);
        
        current_adx = adx.next((h, l, c));
        current_atr = atr.next((h, l, c));
    }

    // 6. Print the results
    println!("📊 BTC 15m ADX (Trend Strength): {:.2}", current_adx);
    println!("📊 BTC 15m ATR (Volatility): ${:.2}", current_atr);

    // 7. Trading Signal
    if current_adx > 25.0 {
        println!("🔥 SIGNAL: Strong Trend detected!");
    } else {
        println!("😴 SIGNAL: Market is sideways.");
    }

    Ok(())
}
