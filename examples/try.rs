use futures_util::StreamExt;
use std::env;
use std::error::Error;
use zerodha_tl::KiteConnect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    let kite = KiteConnect::new(api_key, access_token);

    let instruments = vec![256265, 408065];

    println!("Starting Backend Worker...");

    match kite.stream(&instruments).await {
        Ok(mut stream) => {
            println!("Worker attached to stream. Waiting for ticks...");

            // 3. Consume the stream
            //    This loop will run forever until the connection closes
            while let Some(tick) = stream.next().await {
                // This is where your business logic goes
                println!(">> Tick received: {:?}", tick);
            }

            println!("Stream ended (Connection closed).");
        }
        Err(e) => {
            eprintln!("Critical Worker Error: Failed to connect - {}", e);
        }
    }

    Ok(())
}
