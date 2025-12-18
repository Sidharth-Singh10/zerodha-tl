// examples/backend_worker.rs
use std::env;
use std::error::Error;
use zerodha_tl::KiteConnect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    let kite = KiteConnect::new(api_key, access_token);

    let instruments = vec![256265, 408065];

    println!("Starting Backend Worker...");

    if let Err(e) = kite.stream(&instruments).await {
        eprintln!("Critical Worker Error: {}", e);
    }

    Ok(())
}
