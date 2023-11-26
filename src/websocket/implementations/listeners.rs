use fast_websocket_client::{client, OpCode};
use serde::de::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Sender, Receiver};
use serde_json::{json, Value};
use std::time::Duration;
use crate::config::config::CONFIG;
use crate::websocket::implementations::timeouts::Timeout;
use std::collections::HashMap;

use crate::websocket::{
    errors::WebSocketError,
    implementations::message_channel::WebsocketCloseRequest
};

use crate::websocket::interface::SubscriptionCallback;

use crate::commands::subscriptions::responses::ChainResponse;

pub async fn spawn_callbacks_table(
    mut rx_cb_register: Receiver<(u64, Box<SubscriptionCallback>)>,
    mut rx_cb_unregister: Receiver<u64>,
    mut rx_cb_call: Receiver<(u64, Value)>, 
    mut rx_ws_close: Receiver<WebsocketCloseRequest>
) {

    println!("Running Callback Table Task...");

    let mut callbacks:HashMap<u64, Box<SubscriptionCallback>> = HashMap::new();

    loop {

        if let Ok(Some(_request)) = tokio::time::timeout(
            Duration::from_millis(CONFIG.get_timeout(Timeout::WebsocketClose)), 
            rx_ws_close.recv()
        ).await {
            println!("WebSocket close has been requested.");
            break;
        }

        if let Ok(Some(cb)) = tokio::time::timeout(
            Duration::from_millis(CONFIG.get_timeout(Timeout::WebsocketClose)), 
            rx_cb_register.recv()
        ).await {
            println!("Registering Callback");
            callbacks.insert(cb.0, cb.1);
        }

        if let Ok(Some(cb_id)) = tokio::time::timeout(
            Duration::from_millis(CONFIG.get_timeout(Timeout::WebsocketClose)), 
            rx_cb_unregister.recv()
        ).await {
            let _cb = callbacks.remove(&cb_id);
        }

        if let Ok(Some(exec)) = tokio::time::timeout(
            Duration::from_millis(CONFIG.get_timeout(Timeout::WebsocketClose)), 
            rx_cb_call.recv()
        ).await {
            println!("Calling Callback");
            println!("cb_id: {}", exec.0);
            println!("cb_msg: {}", exec.1);
            if let Some(cb) = callbacks.get(&(exec.0)) {
                let _result = cb(Some(exec.1)).await;
            }
        }

    }

    println!("Closing Callback Table...");
}

pub async fn spawn_sender_task(
    socket: Arc<Mutex<client::Online>>,
    mut rx: Receiver<Value>,
    tx: Sender<Value>
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
                tokio::time::timeout(
                    Duration::from_millis(CONFIG.get_timeout(Timeout::WebSocketSend)),
                    socket.lock().await.send_json(&send_msg)
                ).await
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
    cb_channel: Option<Sender<(u64,Value)>>
) {
    
    loop {

        if let Ok(Some(result)) = tokio::time::timeout(
            Duration::from_millis(CONFIG.get_timeout(Timeout::WebsocketClose)), 
            rx.recv()
        ).await {
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
            tokio::time::timeout(
                Duration::from_millis(CONFIG.get_timeout(Timeout::WebSocketReceive)),
        guard.receive_frame()
            ).await
        {
            match result {
                Ok(message) => {
                    message
                },
                Err(_e) => {
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

                let subscription_response:Result<ChainResponse, serde_json::Error> = serde_json::from_str(&msg_json);

                if let Ok(subs_res) = subscription_response.and_then(|res| if res.is_subcription_response() == true { Ok(res) } else { Err(serde_json::Error::custom("Not subscription response")) }) {
                    if let Some((callback_id, callback_param)) = subs_res.get_parsed_cb_response() {
                        if let Some(tx_cb) = &cb_channel {
                            let _ = tx_cb.send((callback_id, callback_param)).await;
                        } else {
                            println!("No callback channel set for listening subscripition response");
                        }
                        
                    } else {
                        println!("Error parsing callback id from subscriptions response.");
                    }
                } else {
                    if let Ok(message_json) = serde_json::from_str(&msg_json) {
                        let _ = tx.send(message_json).await;
                    } else {
                        let _ = tx.send(WebSocketError::MessageReceiveError.into()).await;
                    }
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