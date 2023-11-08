use crate::websocket::{interface::IWebSocket, errors::WebSocketError};
use serde_json::{Value};

pub struct WebSocket<'a> {
    socket: &'a mut dyn IWebSocket
}

impl <'a> WebSocket<'a> {

    pub fn new(socket: &'a mut dyn IWebSocket) -> Self {
        Self { socket }
    }

    pub fn connect(&mut self) -> Result<(), WebSocketError> {
        return self.socket.connect();
    }

    pub fn send(&mut self, msg:Value) -> Result<(), WebSocketError> {
        return self.socket.send(msg);
    }

    pub fn receive(&mut self) -> Result<Value, WebSocketError> {
        return self.socket.receive();
    }

    pub fn close(&mut self) -> Result<(), WebSocketError> {
        return self.socket.close();
    }

}