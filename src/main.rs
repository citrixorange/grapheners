use crate::websocket::service::WebSocket;
use crate::websocket::implementations::tungstenite::Tungstenite;
use crate::client::client::GrapheneClient;

mod websocket;
mod commands;
mod client;

#[tokio::main]
async fn main () {

    let mut concrete_ws = Tungstenite::new(String::from("wss://127.0.0.1:8090"));
    let mut ws_service = WebSocket::new(&mut concrete_ws);
    let mut graphene_client = GrapheneClient::new(&mut ws_service);

    let _ = graphene_client.connect();

    let result = graphene_client.chain_getter.get_chain_id().await;

    println!("{:?}", result.unwrap());

    let _ = graphene_client.close();

}
