use warp::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use crate::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;

pub async fn handle_connection(ws: WebSocket, tx: Arc<Mutex<broadcast::Sender<String>>>) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let mut rx = tx.lock().unwrap().subscribe();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if ws_sender.send( Message::text(msg)).await.is_err() {
                println!("mauvaise  reception ");
            }
        }
    });
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(message) => {
                if let Ok(text) = message.to_str() {
                    tx.lock().unwrap().send(text.to_string()).expect("Failed to broadcast message");
                }
            },
            Err(e) => break,
        }
    }
}