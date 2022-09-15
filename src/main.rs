use binance_api::get_avg_price;

#[tokio::main]
async fn main() {
    let avg_price = get_avg_price().await;
    println!("avg price = {}", avg_price);
}
