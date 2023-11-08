use serde_json::{Value};
use crate::websocket::errors::WebSocketError;

pub trait IWebSocket {
    fn connect(&mut self) -> Result<(), WebSocketError>;
    fn send(&mut self, msg: Value) -> Result<(), WebSocketError>;
    fn receive(&mut self) -> Result<Value, WebSocketError>;
    fn close(&mut self) -> Result<(), WebSocketError>;
    //listening
}