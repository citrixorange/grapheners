use std::net::TcpStream;
use tungstenite::{connect, Message, WebSocket, stream::MaybeTlsStream };
use url::Url;
use crate::websocket::{interface::IWebSocket, errors::WebSocketError};
use serde_json::{json, Value};

pub struct Tungstenite {
    socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    url: String
}

impl Tungstenite {
    pub fn new(url: String) -> Self {
        Self {
            socket: None,
            url: url
        }
    }
}

impl IWebSocket for Tungstenite {

    fn connect(&mut self) -> Result<(), WebSocketError> {
        match connect(Url::parse(self.url.as_ref()).unwrap()) {
            Ok(result) => {
                let (socket, response) = result;
                let (_parts, body) = response.into_parts();
                let connection_msg = serde_json::to_string_pretty(&body).unwrap();
                println!("{}", connection_msg);
                self.socket = Some(socket);
                return Ok(());
            },
            Err(_err) => {
                return Err(WebSocketError::ConnectionError);
            }
        }
    }

    fn send(&mut self, msg:Value) -> Result<(), WebSocketError> {

        if let Some(socket) = self.socket.as_mut() {
            match socket.write_message(Message::Text(serde_json::to_string(&msg).expect("Failed to serialize JSON"))) {
                Ok(_) => {
                    return Ok(());
                }
                Err(_err) => {
                    return Err(WebSocketError::MessageSendError);
                }
            }
        } else {
            return Err(WebSocketError::NotConnected);
        }
    }

    fn receive(&mut self) -> Result<Value, WebSocketError> {

        if let Some(socket) = self.socket.as_mut() {
            match socket.read_message() {
                Ok(msg) => {
                    println!("Received: {}", msg);
                    let message_json = json!(msg.into_text().unwrap());
                    let req = json!({
                        "msg": message_json 
                    });
                    return Ok(req);
                }
                Err(_e) => {
                    println!("Error on receive messaging");
                    return Err(WebSocketError::MessageReceiveError)
                }
            }
        } else {
            return Err(WebSocketError::NotConnected);
        }


    }

    fn close(&mut self) -> Result<(), WebSocketError> {

        if let Some(socket) = self.socket.as_mut() {
            match socket.close(None) {
                Ok(_sucess) => {
                    self.socket = None;
                    return Ok(());
                }
                Err(_err) => {
                    return Err(WebSocketError::WebSocketNotClosed);
                }
            }
        } else {
            return Err(WebSocketError::NotConnected);
        }
    }
}
