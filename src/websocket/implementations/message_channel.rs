use tokio::sync::mpsc::{Sender, Receiver, channel};
use std::str::FromStr;
use std::future::Future;
use std::pin::Pin;
use serde_json::{json, Value};
use std::time::Duration;
use crate::websocket::errors::WebSocketError;
use crate::websocket::interface::SubscriptionCallback;

pub struct MessageChannel<T> {
    sender: Option<Sender<T>>,
    receiver: Option<Receiver<T>>,
}

impl <T>MessageChannel<T> {
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None
        }
    }

    pub fn create_channel(&mut self) -> (Sender<T>,Receiver<T>) {
        let (sender, rx) = channel::<T>(100);
        let (tx, receiver) = channel::<T>(100);
        self.sender = Some(sender);
        self.receiver = Some(receiver);
        return (tx,rx);
    }

    pub async fn send(&self, msg: T) -> Result<(), WebSocketError> {
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

    pub async fn recv(&mut self) -> Result<Option<T>, WebSocketError> {
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

pub struct CallbackMessageChannel {
    cb_register: Option<Sender<(u64, Box<SubscriptionCallback>)>>,
    cb_unregister: Option<Sender<u64>>,
    ws_close: Option<Sender<WebsocketCloseRequest>>
}

impl CallbackMessageChannel {

    pub fn new() -> Self
    {
        Self {
            cb_register: None,
            cb_unregister: None,
            ws_close: None
        }
    }

    pub fn create_channel(&mut self) -> (
        Receiver<(u64, Box<SubscriptionCallback>)>,
        Receiver<u64>,
        Receiver<(u64, Value)>,
        Sender<(u64, Value)>,
        Receiver<WebsocketCloseRequest>
    ) {

        let (tx_cb_register,rx_cb_register) = channel::<(u64, Box<SubscriptionCallback>)>(100);

        let (tx_cb_unregister,rx_cb_unregister) = channel::<u64>(100);

        let (tx_ws_close,rx_ws_close) = channel::<WebsocketCloseRequest>(100);

        let (tx_cb_call,rx_cb_call) = channel::<(u64, Value)>(100);

        self.cb_register = Some(tx_cb_register);
        self.cb_unregister = Some(tx_cb_unregister);
        self.ws_close = Some(tx_ws_close);

        return (
            rx_cb_register,
            rx_cb_unregister,
            rx_cb_call,
            tx_cb_call,
            rx_ws_close
        );
    }

    pub fn register_callback(&self, cb_id: u64, callback: Box<SubscriptionCallback>) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        
        let future = async move {
            if let Some(channel) = &self.cb_register {
                if let Ok(_res) = channel.send((cb_id, callback)).await {
                    return Ok(());
                } else {
                    return Err(WebSocketError::SubscribingError);
                }
            } else {
                return Err(WebSocketError::SubscribingError)
            }
        };
        
        return Box::pin(future);   
    }

    pub fn unregister_callback(&self, cb_id: u64) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {

        let future = async move {
            if let Some(channel) = &self.cb_unregister {
                if let Ok(_res) = channel.send(cb_id).await {
                    return Ok(());
                } else {
                    return Err(WebSocketError::SubscribingError);
                }
            } else {
                return Err(WebSocketError::SubscribingError)
            }
        };
        
        return Box::pin(future);   
    }

    pub fn drop_callback_list(&self) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {

        let future = async move {
            if let Some(channel) = &self.ws_close {
                if let Ok(_res) = channel.send(WebsocketCloseRequest::Close).await {
                    return Ok(());
                } else {
                    return Err(WebSocketError::SubscribingError);
                }
            } else {
                return Err(WebSocketError::SubscribingError)
            }
        };
        
        return Box::pin(future);   
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