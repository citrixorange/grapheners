use fast_websocket_client::{client, connect, OpCode};
use std::sync::Arc;
use tokio::task;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Sender, Receiver, channel};
use serde_json::{json, Value};
use std::future::Future;
use std::pin::Pin;
use crate::websocket::{interface::IWebSocket, errors::WebSocketError};

pub struct FastWebsocketClient {
    socket: Option<Arc<Mutex<client::Online>>>,
    sender_channel: Option<(Arc<Mutex<Sender<Value>>>,Arc<Mutex<Receiver<Value>>>)>,
    receiver_channel: Option<(Sender<Value>,Receiver<Value>)>,
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

    fn clone_sender_channel(&self) -> Option<(Arc<Mutex<Sender<Value>>>,Arc<Mutex<Receiver<Value>>>)> {
        if let Some((sender,receiver)) = &self.sender_channel {
            return Some((Arc::clone(&sender), Arc::clone(&receiver)));
        } else {
            return None;
        }
    }
}

impl IWebSocket for FastWebsocketClient {

    fn connect(&mut self) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        let future = async {
            match connect(&self.url).await {
                Ok(ws) => {

                    let master_socket = Arc::new(Mutex::new(ws));
                    let clone_socket = Arc::clone(&master_socket);
    
                    let (tx, rx) = channel::<Value>(100);
    
                    let rx_master = Arc::new(Mutex::new(rx));
                    let rx_clone = Arc::clone(&rx_master);
    
                    self.sender_channel = Some((Arc::new(Mutex::new(tx.clone())), rx_master));

                    task::spawn(async move {

                        let socket = clone_socket;
    
                        loop {
    
                            let mut guard = rx_clone.lock().await;
    
                            if let Some(send_msg) = guard.recv().await {

                                match socket.lock().await.send_json(&send_msg).await {
                                    Ok(()) => {
                                        let _ = tx.send(json!(""));
                                    }

                                    Err(_e) => {
                                        let _ = tx.send(WebSocketError::MessageSendError.into());
                                    }
                                }

                            } else {
                                let _ = tx.send(WebSocketError::ErrorSenderChannel.into());
                            }
    
                        }
    
                    });

                    let clone_socket = Arc::clone(&master_socket);

                    let (tx, rx) = channel::<Value>(100);
    
                    self.receiver_channel = Some((tx.clone(), rx));
    
                    task::spawn(async move {
    
                        let socket = clone_socket;
    
                        loop {
                            match socket.lock().await.receive_frame().await {
                                Ok(msg) => {

                                    match msg.opcode {
                                        OpCode::Text => {
                                            println!("Received: {:?}", String::from_utf8_lossy(msg.payload.as_ref()));
                                            let message_json = json!(String::from_utf8_lossy(msg.payload.as_ref()));
                                            let req = json!({
                                                "msg": message_json 
                                            });
                                            let _ = tx.send(req);
                                        }

                                        OpCode::Close => {

                                        }

                                        _ => {

                                        }
                                    }
                                }
                                Err(_e) => {
                                    let _ = tx.send(WebSocketError::MessageReceiveError.into());
                                }
                            }
                        }
    
                    });
    
                    self.socket = Some(master_socket);

                }

                Err(_e) => {
                    return Err(WebSocketError::ConnectionError);
                }
            }

            return Ok(());
        };
        return Box::pin(future);
    }

    fn send(&mut self, msg: Value) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        let future = async {
            if let Some((sender, receiver)) = &self.clone_sender_channel() {
                if let Ok(_sent) = sender.lock().await.send(msg).await {
                    if let Some(msg) = receiver.lock().await.recv().await {
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
            if let Some((_sender, receiver)) = &mut self.receiver_channel {
                if let Some(msg) = receiver.recv().await {
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
            let socket_clone:Arc<Mutex<client::Online>>;

            if let Some(socket) = &mut self.socket {
                socket_clone = Arc::clone(&socket);
            } else {
                return Err(WebSocketError::NotConnected);
            }
    
            match socket_clone.lock().await.send_close(&[]).await {
                Ok(_sucess) => {
                    self.socket = None;
                    return Ok(());
                }
                Err(_err) => {
                    return Err(WebSocketError::WebSocketNotClosed);
                }
            }
        };

        return Box::pin(future);
    }

}