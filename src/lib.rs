use serde::Deserialize;

#[derive(Deserialize)]
struct PriceData {
    price: String,
}

pub async fn get_avg_price() -> f64 {
    let url = "https://api.binance.com/api/v3/trades?limit=100&symbol=BTCUSDT";

    let resp = reqwest::get(url)
        .await
        .expect("Cannot reach server")
        .text()
        .await
        .expect("Unable to get response contents");
    let price_data = parse_json(resp).expect("Cannot parse json");
    calc_avg(price_data)
}

fn calc_avg(prices: Vec<f64>) -> f64 {
    prices.iter().sum::<f64>() as f64 / prices.len() as f64
}

fn parse_json(raw_json: String) -> Result<Vec<f64>, serde_json::Error> {
    let parsed: Vec<PriceData> = serde_json::from_str(&raw_json)?;
    let mut prices: Vec<f64> = vec![];
    parsed.iter().for_each(|price| {
        prices.push(
            price
                .price
                .parse::<f64>()
                .expect("Cannot parse string price data to float"),
        );
    });
    Ok(prices)
}
