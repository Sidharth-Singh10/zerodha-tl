use futures_util::{SinkExt, Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{debug, error, info, instrument, trace};

use crate::{config::StreamConfig, utils::parse_binary};

pub mod config;
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
    #[instrument(skip(self), name = "kite_stream")]
    pub async fn stream(
        &self,
        config: StreamConfig,
    ) -> Result<impl Stream<Item = crate::models::Tick>, Box<dyn std::error::Error>> {
        let url = format!(
            "wss://ws.kite.trade/?api_key={}&access_token={}",
            self.api_key.trim(),
            self.access_token.trim()
        );

        debug!("Attempting to connect to Kite Ticker...");
        let (ws_stream, _) = connect_async(&url).await?;
        info!("Connected to Kite Ticker WebSocket");

        let (mut write, mut read) = ws_stream.split();

        // Subscribe
        let sub_msg = serde_json::json!({
            "a": "subscribe",
            "v": config.instruments
        });

        write.send(Message::Text(sub_msg.to_string())).await?;

        // Set Mode to Full (market depth + quotes) , FiX: Need flexibilty here
        let mode_msg = serde_json::json!({
            "a": "mode",
            "v": [config.mode, config.instruments]
        });
        write.send(Message::Text(mode_msg.to_string())).await?;

        info!(
            instruments = ?config.instruments,
            mode = ?config.mode,
            "Subscribed to instruments"
        );

        // Create a channel to bridge the background task and the user
        let (tx, rx) = mpsc::channel(1024);

        // Spawn the Read Loop in the background
        tokio::spawn(async move {
            // moving write into the task to keep the connection alive.. Hack ????
            let _write_guard = write;

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Binary(bin)) => {
                        // Ignore heartbeat (1 byte)
                        if bin.len() > 1 {
                            let ticks = parse_binary(&bin);
                            for tick in ticks {
                                trace!(?tick, "Received tick");

                                // Send to user. If receiver is dropped, stop loop.
                                if tx.send(tick).await.is_err() {
                                    debug!("User dropped the stream, closing connection.");
                                    return;
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("Connection closed by server.");
                        break;
                    }
                    Err(e) => error!(%e, "WebSocket Error encountered"),
                    _ => {}
                }
            }
        });

        Ok(ReceiverStream::new(rx))
    }
}
