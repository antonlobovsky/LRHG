use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct PriceData {
    #[serde(flatten)]
    prices: HashMap<String, HashMap<String, f64>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/simple/price";
    
    let params = [
        ("ids", "bitcoin,cardano"),
        ("vs_currencies", "usd"),
    ];

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .query(&params)
        .send()
        .await?
        .json::<PriceData>()
        .await?;

    // Extract prices
    let btc_price = response.prices.get("bitcoin").and_then(|m| m.get("usd")).unwrap_or(&0.0);
    let ada_price = response.prices.get("cardano").and_then(|m| m.get("usd")).unwrap_or(&0.0);

    println!("₿ Bitcoin (BTC):  ${:.2}", btc_price);
    println!("₳ Cardano (ADA):  ${:.4}", ada_price);

    Ok(())
}
