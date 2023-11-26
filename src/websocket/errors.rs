use std::error::{Error};
use std::fmt;
use serde_json::{json, Value};
use std::str::FromStr;

#[allow(unused_imports)]
use tungstenite::error::{Error as TungsteniteError, CapacityError};


#[derive(Debug, Copy, Clone)]
pub enum WebSocketError {
    ConnectionError,
    WebSocketNotClosed,
    MessageSendError,
    MessageReceiveError,
    NotConnected,
    ErrorSenderChannel,
    ErrorReceiverChannel,
    SubscribingError
    
}

impl fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebSocketError::ConnectionError => write!(f,"Error on open a connection for Websocket Service"),
            WebSocketError::WebSocketNotClosed => write!(f,"Socket has not been closed successfully"),
            WebSocketError::MessageSendError => write!(f,"Error on Message Sending on Websocket Service"),
            WebSocketError::MessageReceiveError => write!(f,"Error on Message Receiving on Websocket Service"),
            WebSocketError::NotConnected => write!(f,"Websocket Connection not established"),
            WebSocketError::ErrorSenderChannel => write!(f,"Error receive message on Sender Task"),
            WebSocketError::ErrorReceiverChannel => write!(f,"Error sending message on Receiver Task"),
            WebSocketError::SubscribingError => write!(f,"Error on subscribing"),
        }
    }
}

impl Error for WebSocketError {
    fn description(&self) -> &str {
        match self {
            WebSocketError::ConnectionError => "Error on open a connection for Websocket Service",
            WebSocketError::WebSocketNotClosed =>  "Socket has not been closed successfully",
            WebSocketError::MessageSendError => "Error on Message Sending on Websocket Service",
            WebSocketError::MessageReceiveError => "Error on Message Receiving on Websocket Service",
            WebSocketError::NotConnected => "Websocket Connection not established",
            WebSocketError::ErrorSenderChannel => "Error receive message on Sender Task",
            WebSocketError::ErrorReceiverChannel => "Error sending message on Receiver Task",
            WebSocketError::SubscribingError => "Error on subscribing",
        }
    }
}

impl FromStr for WebSocketError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\"WebSocketError::ConnectionError\"" => Ok(WebSocketError::ConnectionError),
            "\"WebSocketError::WebSocketNotClosed\"" => Ok(WebSocketError::WebSocketNotClosed),
            "\"WebSocketError::MessageSendError\"" => Ok(WebSocketError::MessageSendError),
            "\"WebSocketError::MessageReceiveError\"" => Ok(WebSocketError::MessageReceiveError),
            "\"WebSocketError::NotConnected\"" => Ok(WebSocketError::NotConnected),
            "\"WebSocketError::ErrorSenderChannel\"" => Ok(WebSocketError::ErrorSenderChannel),
            "\"WebSocketError::ErrorReceiverChannel\"" => Ok(WebSocketError::ErrorReceiverChannel),
            "\"WebSocketError::SubscribingError\"" => Ok(WebSocketError::SubscribingError),
            _ => Err(())
        }
    }
}

impl From<WebSocketError> for Value {
    fn from(error: WebSocketError) -> Value {
        match error {
            WebSocketError::ConnectionError => json!("WebSocketError::ConnectionError"),
            WebSocketError::WebSocketNotClosed =>  json!("WebSocketError::WebSocketNotClosed"),
            WebSocketError::MessageSendError => json!("WebSocketError::MessageSendError"),
            WebSocketError::MessageReceiveError => json!("WebSocketError::MessageReceiveError"),
            WebSocketError::NotConnected => json!("WebSocketError::NotConnected"),
            WebSocketError::ErrorSenderChannel => json!("WebSocketError::ErrorSenderChannel"),
            WebSocketError::ErrorReceiverChannel => json!("WebSocketError::ErrorReceiverChannel"),
            WebSocketError::SubscribingError => json!("WebSocketError::SubscribingError"),
        }
    }
}