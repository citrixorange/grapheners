use std::error::{Error};
use std::fmt;

#[allow(unused_imports)]
use tungstenite::error::{Error as TungsteniteError, CapacityError};


#[derive(Debug, Copy, Clone)]
pub enum WebSocketError {
    ConnectionError,
    WebSocketClosed,
    MessageSizeOutOfBounds,
    MessageReceiveError,
    GenericError
}

impl WebSocketError {
    pub fn handle_tungstenite_error(&self, error: TungsteniteError) -> WebSocketError {
        match error {
            TungsteniteError::AlreadyClosed => WebSocketError::WebSocketClosed,

            TungsteniteError::ConnectionClosed => WebSocketError::WebSocketClosed,

            #[allow(non_snake_case)]
            TungsteniteError::Capacity(_CapacityError) => WebSocketError::MessageSizeOutOfBounds,
            
            _ => WebSocketError::GenericError
        }
    }
}

impl fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebSocketError::ConnectionError => write!(f,"Error on open a connection for Websocket Service"),
            WebSocketError::WebSocketClosed =>  write!(f,"Socket has been closed on Websocket Service"),
            WebSocketError::MessageReceiveError => write!(f,"Error on Message Receiving on Websocket Service"),
            WebSocketError::MessageSizeOutOfBounds => write!(f,"Message too big sent on Websocket Service"),
            WebSocketError::GenericError => write!(f,"Generic Error on Websocket Service")
        }
    }
}

impl Error for WebSocketError {
    fn description(&self) -> &str {
        match self {
            WebSocketError::ConnectionError => "Error on open a connection for Websocket Service",
            WebSocketError::WebSocketClosed =>  "Socket has been closed on Websocket Service",
            WebSocketError::MessageReceiveError => "Error on Message Receiving on Websocket Service",
            WebSocketError::MessageSizeOutOfBounds => "Message too big sent on Websocket Service",
            WebSocketError::GenericError => "Generic Error on Websocket Service"
        }
    }
}