use crate::websocket::service::WebSocket;
use crate::websocket::implementations::fast_websocket::FastWebsocketClient;
use crate::client::client::GrapheneClient;

mod websocket;
mod commands;
mod client;

#[tokio::main]
async fn main () {

    let mut concrete_ws = FastWebsocketClient::new(String::from("ws://127.0.0.1:8090"));
    let mut ws_service = WebSocket::new(&mut concrete_ws);
    let mut graphene_client = GrapheneClient::new(&mut ws_service);

    graphene_client.connect().await.expect("Failed to establish websocket connection with server");

    let result = graphene_client.chain_getter.get_chain_id().await.expect("Failed to fetch chain id from Blockchain Node Server");

    println!("{:?}", result);

    graphene_client.close().await.expect("Failed to close websocket connection");

}
