use std::error::{Error};
use std::fmt;

#[allow(unused_imports)]
use tungstenite::error::{Error as TungsteniteError, CapacityError};


#[derive(Debug, Copy, Clone)]
pub enum WebSocketError {
    ConnectionError,
    WebSocketNotClosed,
    MessageSendError,
    MessageReceiveError,
    NotConnected,
    
}

impl fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebSocketError::ConnectionError => write!(f,"Error on open a connection for Websocket Service"),
            WebSocketError::WebSocketNotClosed => write!(f,"Socket has not been closed successfully"),
            WebSocketError::MessageSendError => write!(f,"Error on Message Sending on Websocket Service"),
            WebSocketError::MessageReceiveError => write!(f,"Error on Message Receiving on Websocket Service"),
            WebSocketError::NotConnected => write!(f,"Websocket Connection not established"),
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
        }
    }
}