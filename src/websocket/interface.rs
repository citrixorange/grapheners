use json::{JsonValue};
use crate::websocket::errors::WebSocketError;

pub trait IWebSocket {
    fn connect(&mut self) -> Result<bool, WebSocketError>;
    fn send(&mut self, msg: JsonValue) -> Result<bool, WebSocketError>;
    fn receive(&mut self) -> Result<JsonValue, WebSocketError>;
    fn close(&mut self) -> Result<bool, WebSocketError>;
    //listening
}