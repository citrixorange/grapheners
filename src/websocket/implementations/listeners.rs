use fast_websocket_client::{client, OpCode};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Sender, Receiver};
use serde_json::{json, Value};
use std::time::Duration;

use crate::websocket::{
    errors::WebSocketError,
    implementations::message_channel::WebsocketCloseRequest
};

pub async fn spawn_sender_task(
    socket: Arc<Mutex<client::Online>>,
    mut rx: Receiver<Value>,
    tx: Sender<Value>,
) {

    loop {
        if let Some(send_msg) = rx.recv().await {
            let result_str: &str = &send_msg.to_string();
            println!("{}", result_str);

            match result_str.parse::<WebsocketCloseRequest>() {
                Ok(_close) => {
                    println!("WebSocket close has been requested.");
                    break;
                }
                Err(_e) => {}
            }

            if let Ok(_result) =
                tokio::time::timeout(Duration::from_secs(1), socket.lock().await.send_json(&send_msg))
                    .await
            {
                let _ = tx.send(json!("")).await;
            } else {
                let _ = tx.send(WebSocketError::MessageSendError.into()).await;
            }
        } else {
            let _ = tx.send(WebSocketError::ErrorSenderChannel.into()).await;
        }
    }

    println!("Closing Sender Task...");
}

pub async fn spawn_receiver_task(
    socket: Arc<Mutex<client::Online>>,
    mut rx: Receiver<Value>,
    tx: Sender<Value>,
) {
    
    loop {

        if let Ok(Some(result)) = tokio::time::timeout(Duration::from_millis(100), rx.recv()).await {
            let result_str: &str = &result.to_string();
            match result_str.parse::<WebsocketCloseRequest>() {
                Ok(_close) => {
                    println!("WebSocket close has been requested.");
                    break;
                },
                Err(_e) => {

                }
            }
        }

        let mut guard = socket.lock().await;

        let message = if let Ok(result) =
            tokio::time::timeout(Duration::from_secs(1), guard.receive_frame()).await
        {
            match result {
                Ok(message) => {
                    message
                },
                Err(e) => {
                    drop(guard);
                    break; // break the message loop then reconnect
                 }
            }
        } else {
            drop(guard);
            continue;
        };

        match message.opcode {
            OpCode::Text => {
                let msg_json = String::from_utf8_lossy(message.payload.as_ref()).to_string();
                println!("Received: {:?}", &msg_json);
                if let Ok(message_json) = serde_json::from_str(&msg_json) {
                    let _ = tx.send(message_json).await;
                } else {
                    let _ = tx.send(WebSocketError::MessageReceiveError.into()).await;
                }
            }

            OpCode::Close => {
                println!("Websocket Close Requested");
                break; // break the message loop then reconnect
            }

            _ => {
                println!("Error Unexpected Error");
            }
        }

    }

    println!("Closing Receiver Task...");
}