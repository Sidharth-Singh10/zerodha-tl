use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::utils::parse_binary;

pub mod models;
mod utils;
pub struct KiteConnect {
    api_key: String,
    access_token: String,
}

impl KiteConnect {
    pub fn new(api_key: String, access_token: String) -> Self {
        Self {
            api_key,
            access_token,
        }
    }

    // match this to a socket connection
    pub async fn stream(&self, instruments: &[u32]) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "wss://ws.kite.trade/?api_key={}&access_token={}",
            self.api_key.trim(),
            self.access_token.trim()
        );

        println!("[*] Connecting to Kite Ticker...");
        let (ws_stream, _) = connect_async(&url).await?;
        println!("[+] Connected!");

        let (mut write, mut read) = ws_stream.split();

        // Subscribe
        let sub_msg = serde_json::json!({
            "a": "subscribe",
            "v": instruments
        });
        write.send(Message::Text(sub_msg.to_string())).await?;

        // Set Mode to Full (market depth + quotes) , FiX: Need flexibilty here
        let mode_msg = serde_json::json!({
            "a": "mode",
            "v": ["full", instruments]
        });
        write.send(Message::Text(mode_msg.to_string())).await?;

        println!("[+] Subscribed to instruments: {:?}", instruments);

        // Read Loop
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Binary(bin)) => {
                    // Ignore heartbeat (1 byte)
                    if bin.len() > 1 {
                        let ticks = parse_binary(&bin);
                        for tick in ticks {
                            // fix 
                            println!("{:?}", tick);
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    println!("Connection closed by server.");
                    break;
                }
                Err(e) => eprintln!("WebSocket Error: {}", e),
                _ => {}
            }
        }

        Ok(())
    }
}
