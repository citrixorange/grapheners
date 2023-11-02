use json::{JsonValue};
use crate::websocket::errors::WebSocketError;

pub trait IWebSocket {
    fn connect(&mut self) -> Result<(), WebSocketError>;
    fn send(&mut self, msg: JsonValue) -> Result<(), WebSocketError>;
    fn receive(&mut self) -> Result<JsonValue, WebSocketError>;
    fn close(&mut self) -> Result<(), WebSocketError>;
    //listening
}