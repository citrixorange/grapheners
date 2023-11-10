use fast_websocket_client::{client, connect, OpCode};
use std::sync::Arc;
use tokio::task;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Sender, Receiver, channel};
use serde_json::{json, Value};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use std::str::FromStr;
use crate::websocket::{interface::IWebSocket, errors::WebSocketError};

struct MessageChannel {
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

enum WebsocketCloseRequest {
    Close
}

impl FromStr for WebsocketCloseRequest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WebsocketCloseRequest::Close" => Ok(WebsocketCloseRequest::Close),
            _ => Err(())
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

pub struct FastWebsocketClient {
    socket: Option<Arc<Mutex<client::Online>>>,
    sender_channel: Option<MessageChannel>,
    receiver_channel: Option<MessageChannel>,
    url: String
}

impl FastWebsocketClient {
    pub fn new(url: String) -> Self {
        Self {
            socket: None,
            sender_channel: None,
            receiver_channel: None,
            url: url
        }
    }

    fn create_sender_channel(&mut self) -> (Sender<Value>,Receiver<Value>) {

        let mut channel = MessageChannel::new();
        let (tx,rx) = channel.create_channel();
        self.sender_channel = Some(channel);
        return (tx, rx);
    }

    fn create_receiver_channel(&mut self) -> (Sender<Value>,Receiver<Value>) {

        let mut channel = MessageChannel::new();
        let (tx,rx) = channel.create_channel();
        self.receiver_channel = Some(channel);
        return (tx, rx);
    }


}

impl IWebSocket for FastWebsocketClient {

    fn connect(&mut self) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        let future = async {
            match connect(&self.url).await {
                Ok(ws) => {

                    let master_socket = Arc::new(Mutex::new(ws));
                    let clone_socket = Arc::clone(&master_socket);
    
                    let (tx,mut rx) = self.create_sender_channel();

                    task::spawn(async move {

                        let socket = clone_socket;
    
                        loop {
    
                            if let Some(send_msg) = rx.recv().await {

                                match send_msg.to_string().parse::<WebsocketCloseRequest>() {
                                    Ok(_close) => {
                                        println!("WebSocket close has been requested.");
                                        println!("Closing Sender Task...");
                                        break;
                                    },
                                    Err(_e) => {

                                    }
                                }

                                if let Ok(_result) = tokio::time::timeout(Duration::from_secs(1), socket.lock().await.send_json(&send_msg)).await {
                                    let _ = tx.send(json!("")).await;
                                } else {
                                    let _ = tx.send(WebSocketError::MessageSendError.into()).await;
                                }

                            } else {
                                let _ = tx.send(WebSocketError::ErrorSenderChannel.into()).await;
                            }
    
                        }
    
                    });

                    let clone_socket = Arc::clone(&master_socket);

                    let (tx,mut rx) = self.create_receiver_channel();
    
                    task::spawn(async move {
    
                        let socket = clone_socket;
    
                        loop {

                            if let Ok(Some(result)) = tokio::time::timeout(Duration::from_millis(100), rx.recv()).await {
                                match result.to_string().parse::<WebsocketCloseRequest>() {
                                    Ok(_close) => {
                                        println!("WebSocket close has been requested.");
                                        println!("Closing Sender Task...");
                                        break;
                                    },
                                    Err(_e) => {

                                    }
                                }
                            }

                            let mut guard = socket.lock().await;

                            let message = if let Ok(result) =
                                tokio::time::timeout(Duration::from_secs(1), guard.receive_frame()).await
                            {
                                match result {
                                    Ok(message) => {
                                        message
                                    },
                                    Err(e) => {
                                        eprintln!("Reconnecting from an Error: {e:?}");
                                        drop(guard);
                                        break; // break the message loop then reconnect
                                     }
                                }
                            } else {
                                drop(guard);
                                continue;
                            };

                            match message.opcode {
                                OpCode::Text => {
                                    println!("Received: {:?}", String::from_utf8_lossy(message.payload.as_ref()));
                                    let message_json = json!(String::from_utf8_lossy(message.payload.as_ref()));
                                    let req = json!({
                                        "msg": message_json 
                                    });
                                    let _ = tx.send(req).await;
                                }

                                OpCode::Close => {
                                    println!("Error Close Websocket Requested");
                                }

                                _ => {
                                    println!("Error Unexpected Error");
                                }
                            }

                        }
    
                    });
    
                    self.socket = Some(master_socket);

                }

                Err(e) => {
                    println!("Not Connected: {:?}", e);
                    return Err(WebSocketError::ConnectionError);
                }
            }

            return Ok(());
        };
        return Box::pin(future);
    }

    fn send(&mut self, msg: Value) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        let future = async {
            if let Some(channel) = &mut self.sender_channel {
                if let Ok(_sent) = channel.send(msg).await {
                    if let Ok(Some(msg)) = channel.recv().await {
                        let _deserialized_error: WebSocketError = match msg.to_string().parse() {
                            Ok(error) => return Err(error),
                            Err(_msg) => {
                                return Ok(());
                            }
                        };
                    } else {
                        return Err(WebSocketError::ErrorSenderChannel);
                    }
                } else {
                    return Err(WebSocketError::MessageSendError);
                }
            } else {
                return Err(WebSocketError::ConnectionError);
            }
        };
        
        return Box::pin(future);
    }

    fn receive(&mut self) -> Pin<Box<dyn Future<Output = Result<Value, WebSocketError>> + '_>> {
        let future = async {
            if let Some(channel) = &mut self.receiver_channel {
                if let Ok(Some(msg)) = channel.recv().await {
                    let _deserialized_error: WebSocketError = match msg.to_string().parse() {
                        Ok(error) => return Err(error),
                        Err(_message) => {
                            return Ok(msg);
                        }
                    };
                } else {
                    return Err(WebSocketError::MessageReceiveError);
                }
            } else {
                return Err(WebSocketError::ConnectionError);
            }
        };

        return Box::pin(future);
    }

    fn close(&mut self) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {

        let future = async {

            if let Some(channel) = &mut self.sender_channel {
                if let Ok(_sent) = channel.send(WebsocketCloseRequest::Close.into()).await {

                } else {
                    return Err(WebSocketError::MessageSendError);
                }
            } else {
                return Err(WebSocketError::MessageSendError);
            }

            let socket_clone:Arc<Mutex<client::Online>>;

            if let Some(socket) = &mut self.socket {
                socket_clone = Arc::clone(&socket);
            } else {
                return Err(WebSocketError::NotConnected);
            }
    
            let _x = match socket_clone.lock().await.send_close(&[]).await {
                Ok(_sucess) => {
                    self.socket = None;
                    return Ok(());
                }
                Err(_err) => {
                    return Err(WebSocketError::WebSocketNotClosed);
                }
            };
        };

        return Box::pin(future);
    }

}