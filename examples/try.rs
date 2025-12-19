use futures_util::StreamExt;
use std::env;
use std::error::Error;
use zerodha_tl::{KiteConnect, config::StreamConfig, models::Mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let api_key = env::var("KITE_API_KEY").expect("KITE_API_KEY must be set");
    let access_token = env::var("KITE_ACCESS_TOKEN").expect("KITE_ACCESS_TOKEN must be set");

    let kite = KiteConnect::new(api_key, access_token);

    // Instrument Tokens: Nifty 50 (256265), Infosys (408065)
    let instruments = vec![256265, 408065];

    // Default is LTP, but here we explicitly request 'Full' (includes Market Depth)
    let config = StreamConfig::new(instruments).mode(Mode::Full);

    println!("Starting Backend Worker...");

    match kite.stream(config).await {
        Ok(mut stream) => {
            println!("Worker attached to stream. Waiting for ticks...");

            while let Some(tick) = stream.next().await {
                println!(">> Tick received: {:?}", tick);

                if let Some(depth) = &tick.bids {
                    println!("   Top Bid: {:?}", depth.first());
                }
            }

            println!("Stream ended (Connection closed).");
        }
        Err(e) => {
            eprintln!("Critical Worker Error: Failed to connect - {}", e);
        }
    }

    Ok(())
}
