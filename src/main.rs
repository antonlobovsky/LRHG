// Feed data into the ADX indicator
    for candle in candles.iter() {
        // SDK uses 'high', 'low', and 'close'
        let high: f64 = candle.high.parse().unwrap_or(0.0);
        let low: f64 = candle.low.parse().unwrap_or(0.0);
        let close: f64 = candle.close.parse().unwrap_or(0.0);
        
        last_adx = adx.next((high, low, close));
    }
