use tokio::sync::mpsc::{Sender, Receiver, channel};
use std::str::FromStr;
use serde_json::{json, Value};
use std::time::Duration;
use crate::websocket::errors::WebSocketError;

pub struct MessageChannel {
    sender: Option<Sender<Value>>,
    receiver: Option<Receiver<Value>>,
}

impl MessageChannel {
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None
        }
    }

    pub fn create_channel(&mut self) -> (Sender<Value>,Receiver<Value>) {
        let (sender, rx) = channel::<Value>(100);
        let (tx, receiver) = channel::<Value>(100);
        self.sender = Some(sender);
        self.receiver = Some(receiver);
        return (tx,rx);
    }

    pub async fn send(&self, msg: Value) -> Result<(), WebSocketError> {
        if let Some(sender) = &self.sender {
            if let Ok(_res) = sender.send_timeout(msg, Duration::from_secs(1)).await {
                return Ok(());
            } else {
                return Err(WebSocketError::ErrorSenderChannel);
            }
        } else {
            return Err(WebSocketError::ErrorSenderChannel);
        }
    }

    pub async fn recv(&mut self) -> Result<Option<Value>, WebSocketError> {
        if let Some(receiver) = &mut self.receiver {
            if let Ok(Some(result)) = tokio::time::timeout(Duration::from_secs(3), receiver.recv()).await {
                return Ok(Some(result));
            } else {
                return Err(WebSocketError::ErrorReceiverChannel);
            }
        } else {
            return Err(WebSocketError::ErrorReceiverChannel);
        }
    }
}

pub enum WebsocketCloseRequest {
    Close
}

impl FromStr for WebsocketCloseRequest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "\"WebsocketCloseRequest::Close\"" {
            return Ok(WebsocketCloseRequest::Close);
        } else {
            return Err(());
        }
    }
}

impl From<WebsocketCloseRequest> for Value {
    fn from(request: WebsocketCloseRequest) -> Value {
        match request {
            WebsocketCloseRequest::Close => json!("WebsocketCloseRequest::Close"),
        }
    }
}