use serde_json::Value;
use crate::websocket::errors::WebSocketError;
use std::future::Future;
use std::pin::Pin;

pub type SubscriptionCallback = dyn Fn(Option<Value>) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + Send + Sync >> + Send + Sync;

pub trait IWebSocket {
    fn connect(&mut self) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>>;
    fn send(&mut self, msg: Value) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>>;
    fn receive(&mut self) -> Pin<Box<dyn Future<Output = Result<Value, WebSocketError>> + '_>>;
    fn close(&mut self) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>>;
}