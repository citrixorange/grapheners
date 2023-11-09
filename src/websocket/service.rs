use crate::websocket::{interface::IWebSocket, errors::WebSocketError};
use serde_json::{Value};
use std::future::Future;
use std::pin::Pin;

pub struct WebSocket<'a> {
    socket: &'a mut dyn IWebSocket
}

impl <'a> WebSocket<'a> {

    pub fn new(socket: &'a mut dyn IWebSocket) -> Self {
        Self { socket }
    }

    pub fn connect(&mut self) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        return self.socket.connect();
    }

    pub fn send(&mut self, msg:Value) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        return self.socket.send(msg);
    }

    pub fn receive(&mut self) -> Pin<Box<dyn Future<Output = Result<Value, WebSocketError>> + '_>> {
        return self.socket.receive();
    }

    pub fn close(&mut self) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        return self.socket.close();
    }

}