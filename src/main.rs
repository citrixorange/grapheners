use crate::websocket::service::WebSocket;
use crate::websocket::implementations::fast_websocket::FastWebsocketClient;
use crate::websocket::errors::WebSocketError;
use crate::client::client::GrapheneClient;

use serde_json::Value;

mod websocket;
mod commands;
mod client;
mod config;

pub async fn watch_account_cb_one(notice: Option<Value>) -> Result<(), WebSocketError> {
    println!("Heya 1");
    println!("{:?}", notice);
    return Ok(()); 
}

pub async fn watch_account_cb_two(notice: Option<Value>) -> Result<(), WebSocketError> {
    println!("Heya 2");
    println!("{:?}", notice);
    return Ok(()); 
}

#[tokio::main]
async fn main () {

    let mut concrete_ws = FastWebsocketClient::new(String::from("ws://127.0.0.1:8090"));
    
    let mut graphene_client = GrapheneClient::new();

    let tx_cb_call = graphene_client.create_callback_channel();

    concrete_ws.set_callback_channel(tx_cb_call);

    let mut ws_service = WebSocket::new(&mut concrete_ws);

    graphene_client.set_ws_connection(&mut ws_service);

    graphene_client.connect().await.expect("Failed to establish websocket connection with server");

    graphene_client.init().await.expect("Failed to Initialize Client...");

    let account_to_watch_one = String::from("1.2.7");

    let account_to_watch_two = String::from("1.3.6");

    let _ = graphene_client.subscribe_to_account(Box::new(|x| Box::pin(watch_account_cb_one(x))), vec![account_to_watch_one]).await;

    let _ = graphene_client.subscribe_to_account(Box::new(|x| Box::pin(watch_account_cb_two(x))), vec![account_to_watch_two]).await;

    loop{};

    //graphene_client.close().await.expect("Failed to close websocket connection");

}
