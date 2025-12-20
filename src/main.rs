use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

// Hyperliquid public endpoints
const INFO_URL: &str = "https://api.hyperliquid.xyz/info";
const WS_URL: &str = "wss://api.hyperliquid.xyz/ws";

#[derive(Serialize)]
struct CandleRequest {
    #[serde(rename = "type")]
    msg_type: String,
    req: CandleReq,
}

#[derive(Serialize)]
struct CandleReq {
    coin: String,
    interval: String,
    startTime: u64,
    endTime: u64,
}

#[derive(Deserialize, Debug)]
struct Candle {
    #[serde(rename = "t")]
    timestamp: u64,
    #[serde(rename = "o")]
    open: String,
    #[serde(rename = "h")]
    high: String,
    #[serde(rename = "l")]
    low: String,
    #[serde(rename = "c")]
    close: String,
    #[serde(rename = "v")]
    volume: String,
}

fn parse_price(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or(0.0)
}

async fn fetch_candles_rest(coin: &str, interval: &str, num_candles: usize) -> Vec<Candle> {
    let client = Client::new();

    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let interval_minutes: u64 = match interval {
        "1m" => 1,
        "5m" => 5,
        "15m" => 15,
        "1h" => 60,
        "4h" => 240,
        "1d" => 1440,
        _ => 5,
    };

    let start_time = end_time.saturating_sub(num_candles as u64 * interval_minutes * 60 * 1000);

    let payload = CandleRequest {
        msg_type: "candleSnapshot".to_string(),
        req: CandleReq {
            coin: coin.to_string(),
            interval: interval.to_string(),
            startTime: start_time,
            endTime: end_time,
        },
    };

    let response = client
        .post(INFO_URL)
        .json(&payload)
        .send()
        .await
        .ok()
        .and_then(|r| r.json::<Vec<Candle>>().ok());

    response.unwrap_or_default()
}

#[tokio::main]
async fn main() {
    println!("Fetching latest 100 x 5m candles for BTC from Hyperliquid...\n");

    let candles = fetch_candles_rest("BTC", "5m", 100).await;

    if candles.is_empty() {
        println!("Failed to fetch candles");
        return;
    }

    println!("Got {} candles (enough for ADX-14 and more)", candles.len());
    println!("┌────────────────────┬──────────┬──────────┬──────────┬──────────┐");
    println!("│ Time (UTC)         │ Open     │ High     │ Low      │ Close    │");
    println!("├────────────────────┼──────────┼──────────┼──────────┼──────────┤");

    // Show last 10 candles (most recent)
    for candle in candles.iter().rev().take(10) {
        let time = chrono::NaiveDateTime::from_timestamp_opt((candle.timestamp / 1000) as i64, 0)
            .unwrap_or_default()
            .format("%Y-%m-%d %H:%M");

        println!(
            "│ {} │ {:>8.2} │ {:>8.2} │ {:>8.2} │ {:>8.2} │",
            time,
            parse_price(&candle.open),
            parse_price(&candle.high),
            parse_price(&candle.low),
            parse_price(&candle.close)
        );
    }

    println!("└────────────────────┴──────────┴──────────┴──────────┴──────────┘");

    // Now you have candles → ready for ADX calculation!
    let highs: Vec<f64> = candles.iter().map(|c| parse_price(&c.high)).collect();
    let lows: Vec<f64> = candles.iter().map(|c| parse_price(&c.low)).collect();
    let closes: Vec<f64> = candles.iter().map(|c| parse_price(&c.close)).collect();

    println!("\nYou now have vectors for highs/lows/closes — plug into your ADX function!");
    println!("Example: highs.len() = {}", highs.len());
}
