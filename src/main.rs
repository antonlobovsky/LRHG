use serde::Deserialize;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
struct PriceData {
    #[serde(flatten)]
    prices: HashMap<String, HashMap<String, f64>>,
}

#[derive(Deserialize)]
struct OhlcData {
    prices: Vec<Vec<f64>>,  // [[timestamp, open, high, low, close, ...], ...]
}

fn calculate_adx(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Option<f64> {
    if highs.len() < period + 1 { return None; }

    let mut tr = Vec::new();
    let mut plus_dm = Vec::new();
    let mut minus_dm = Vec::new();

    for i in 1..highs.len() {
        let high_diff = highs[i] - highs[i-1];
        let low_diff = lows[i-1] - lows[i];

        let tr_val = f64::max(highs[i] - lows[i],
            f64::max((highs[i] - closes[i-1]).abs(),
                     (lows[i] - closes[i-1]).abs()));
        tr.push(tr_val);

        let plus = if high_diff > low_diff && high_diff > 0.0 { high_diff } else { 0.0 };
        let minus = if low_diff > high_diff && low_diff > 0.0 { low_diff } else { 0.0 };

        plus_dm.push(plus);
        minus_dm.push(minus);
    }

    let mut atr: Vec<f64> = vec![0.0; tr.len()];
    let mut plus_di: Vec<f64> = vec![0.0; tr.len()];
    let mut minus_di: Vec<f64> = vec![0.0; tr.len()];

    atr[period-2] = tr[..period].iter().sum::<f64>() / period as f64;
    plus_di[period-2] = 100.0 * (plus_dm[..period].iter().sum::<f64>() / period as f64) / (atr[period-2] + 1e-10);
    minus_di[period-2] = 100.0 * (minus_dm[..period].iter().sum::<f64>() / period as f64) / (atr[period-2] + 1e-10);

    for i in period-1..tr.len() {
        atr[i-1] = (atr[i-2] * (period as f64 - 1.0) + tr[i]) / period as f64;
        plus_di[i-1] = 100.0 * (((plus_di[i-2] / 100.0) * (period as f64 - 1.0) + plus_dm[i]) / period as f64) / (atr[i-1] + 1e-10);
        minus_di[i-1] = 100.0 * (((minus_di[i-2] / 100.0) * (period as f64 - 1.0) + minus_dm[i]) / period as f64) / (atr[i-1] + 1e-10);
    }

    let mut dx: Vec<f64> = vec![0.0; tr.len() - period + 1];
    for i in 0..dx.len() {
        let sum_di = plus_di[i + period - 2] + minus_di[i + period - 2];
        dx[i] = 100.0 * (plus_di[i + period - 2] - minus_di[i + period - 2]).abs() / (sum_di + 1e-10);
    }

    let mut adx = vec![0.0; dx.len()];
    adx[period-2] = dx[..period].iter().sum::<f64>() / period as f64;

    for i in period-1..dx.len() {
        adx[i-1] = (adx[i-2] * (period as f64 - 1.0) + dx[i]) / period as f64;
    }

    adx.last().copied()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Current prices
    let price_url = "https://api.coingecko.com/api/v3/simple/price";
    let params = [("ids", "bitcoin,cardano"), ("vs_currencies", "usd")];
    let price_resp = client.get(price_url).query(&params).send().await?.json::<PriceData>().await?;

    let btc_price = price_resp.prices.get("bitcoin").and_then(|m| m.get("usd")).unwrap_or(&0.0);
    let ada_price = price_resp.prices.get("cardano").and_then(|m| m.get("usd")).unwrap_or(&0.0);

    // Historical OHLC for ADX (daily, last 100 days)
    let mut btc_highs = Vec::new();
    let mut btc_lows = Vec::new();
    let mut btc_closes = Vec::new();

    let mut ada_highs = Vec::new();
    let mut ada_lows = Vec::new();
    let mut ada_closes = Vec::new();

    for (id, highs, lows, closes) in [("bitcoin", &mut btc_highs, &mut btc_lows, &mut btc_closes),
                                      ("cardano", &mut ada_highs, &mut ada_lows, &mut ada_closes)] {
        let ohlc_url = format!("https://api.coingecko.com/api/v3/coins/{}/ohlc?vs_currency=usd&days=90");
        let ohlc_resp = client.get(&ohlc_url.replace("{}", id)).send().await?.json::<OhlcData>().await?;
        for candle in ohlc_resp.prices {
            // candle: [time, open, high, low, close]
            highs.push(candle[2]);
            lows.push(candle[3]);
            closes.push(candle[4]);
        }
    }

    let btc_adx = calculate_adx(&btc_highs, &btc_lows, &btc_closes, 14).unwrap_or(0.0);
    let ada_adx = calculate_adx(&ada_highs, &ada_lows, &ada_closes, 14).unwrap_or(0.0);

    // Pretty table
    println!("┌────────────────────┬────────────────────┬──────────────┐");
    println!("│ Cryptocurrency     │ Price (USD)        │ ADX (14)     │");
    println!("├────────────────────┼────────────────────┼──────────────┤");
    println!("│ ₿ Bitcoin (BTC)    │ ${:>16,.2} │ {:>10.2} │", btc_price, btc_adx);
    println!("│ ₳ Cardano (ADA)    │ ${:>16.4} │ {:>10.2} │", ada_price, ada_adx);
    println!("└────────────────────┴────────────────────┴──────────────┘");

    // Interpretation
    println!("\nADX Interpretation:");
    println!("  < 20 → Weak trend (ranging)");
    println!("  20-25 → Developing trend");
    println!("  > 25 → Strong trend");

    Ok(())
}
