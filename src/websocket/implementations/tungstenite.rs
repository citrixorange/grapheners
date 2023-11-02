use std::net::TcpStream;
use tungstenite::{connect, Message, WebSocket, stream::MaybeTlsStream };
use url::Url;
use crate::websocket::{interface::IWebSocket, errors::WebSocketError};
use json::{JsonValue, object, stringify};
use serde_json;

pub struct Tungstenite {
    socket: Result<WebSocket<MaybeTlsStream<TcpStream>>, WebSocketError>,
    connection_msg: String
}

impl Tungstenite {
    pub fn new(url: String) -> Self {
        match connect(Url::parse(&url).unwrap()) {
            Ok(result) => {
                let (socket, response) = result;
                let (_parts, body) = response.into_parts();
                let connection_msg = serde_json::to_string_pretty(&body).unwrap();
                let socket = Ok(socket);
                Self { socket, connection_msg}
            },
            Err(_err) => {
                let socket = Err(WebSocketError::ConnectionError);
                let connection_msg = String::from("Cannot connect");
                Self { socket, connection_msg}
            }
        }
    }
}

impl IWebSocket for Tungstenite {

    fn connect(&mut self) -> Result<bool, WebSocketError> {
        println!("{}", self.connection_msg);
        match self.socket {
            Ok(_) => return Ok(true),
            Err(err) => return Err(err)
        }
    }

    fn send(&mut self, msg:JsonValue) -> Result<bool, WebSocketError> {

        match self.socket.as_mut().expect(&self.connection_msg).write_message(Message::Text(stringify(msg))) {
            Ok(_result) => return Ok(true),
            Err(err) => return Err(WebSocketError::GenericError.handle_tungstenite_error(err))
        }
    }

    fn receive(&mut self) -> Result<JsonValue, WebSocketError> {
        match self.socket.as_mut().expect(&self.connection_msg).read_message() {
            Ok(msg) => {
                println!("Received: {}", msg);
                let req = object!{
                    msg: msg.into_text().unwrap()
                };
                return Ok(req);
            }
            Err(_e) => {
                println!("Error on receive messaging");
                return Err(WebSocketError::MessageReceiveError)
            }
        }
    }

    fn close(&mut self) -> Result<bool, WebSocketError> {
        match self.socket.as_mut().expect(&self.connection_msg).close(None) {
            Ok(_sucess) => {
                return Ok(true);
            }
            Err(_err) => {
                return Err(WebSocketError::GenericError);
            }
        }
    }
}
