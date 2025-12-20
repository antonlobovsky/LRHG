use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use ta::indicators::AverageTrueRange; // Standard name in 'ta' crate
use ta::Next;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🟢 STARTING ATR VOLATILITY BOT ---");

    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;

    let candles = info_client
        .candles_snapshot("BTC".to_string(), "15m".to_string(), 0, 0)
        .await?;

    // Initialize ATR with 14 periods
    let mut atr = AverageTrueRange::new(14).unwrap();
    let mut current_atr = 0.0;

    for candle in candles.iter() {
        let h: f64 = candle.high.parse().unwrap_or(0.0);
        let l: f64 = candle.low.parse().unwrap_or(0.0);
        let c: f64 = candle.close.parse().unwrap_or(0.0);
        
        current_atr = atr.next((h, l, c));
    }

    println!("📊 Current BTC 15m ATR: {:.2}", current_atr);

    // ATR Expansion Logic: Is the market "expanding"?
    if current_atr > 50.0 { 
        println!("🚀 VOLATILITY EXPANDING: Market is highly active.");
    } else {
        println!("😴 VOLATILITY LOW: Market is compressing.");
    }

    Ok(())
}
