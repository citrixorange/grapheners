use crate::websocket::{interface::IWebSocket, errors::WebSocketError};
use json::{JsonValue};

pub struct WebSocket<'a> {
    socket: &'a mut dyn IWebSocket
}

impl <'a> WebSocket<'a> {

    pub fn new(socket: &'a mut dyn IWebSocket) -> Self {
        Self { socket }
    }

    pub fn connect(&mut self) -> Result<bool, WebSocketError> {
        return self.socket.connect();
    }

    pub fn send(&mut self, msg:JsonValue) -> Result<bool, WebSocketError> {
        return self.socket.send(msg);
    }

    pub fn receive(&mut self) -> Result<JsonValue, WebSocketError> {
        return self.socket.receive();
    }

    pub fn close(&mut self) -> Result<bool, WebSocketError> {
        return self.socket.close();
    }

}