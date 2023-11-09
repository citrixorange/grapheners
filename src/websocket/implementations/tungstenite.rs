use std::net::TcpStream;
use tungstenite::{connect, Message, WebSocket, stream::MaybeTlsStream };
use url::Url;
use crate::websocket::{interface::IWebSocket, errors::WebSocketError};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::mpsc::{Sender, Receiver, channel};
use std::future::Future;
use std::pin::Pin;

pub struct Tungstenite {
    socket: Option<Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>>,
    sender_channel: Option<(Arc<TokioMutex<Sender<Value>>>,Arc<TokioMutex<Receiver<Value>>>)>,
    receiver_channel: Option<(Sender<Value>,Receiver<Value>)>,
    url: String
}

impl Tungstenite {
    pub fn new(url: String) -> Self {
        Self {
            socket: None,
            sender_channel: None,
            receiver_channel: None,
            url: url
        }
    }

    fn clone_sender_channel(&self) -> Option<(Arc<TokioMutex<Sender<Value>>>,Arc<TokioMutex<Receiver<Value>>>)> {
        if let Some((sender,receiver)) = &self.sender_channel {
            return Some((Arc::clone(&sender), Arc::clone(&receiver)));
        } else {
            return None;
        }
    }
}

impl IWebSocket for Tungstenite {

    fn connect(&mut self) -> Result<(), WebSocketError> {
        match connect(Url::parse(self.url.as_ref()).unwrap()) {
            Ok(result) => {
                let (socket, response) = result;
                let (_parts, body) = response.into_parts();
                let connection_msg = serde_json::to_string_pretty(&body).unwrap();
                println!("Connection Message:{}", connection_msg);

                let master_socket = Arc::new(Mutex::new(socket));
                let clone_socket = Arc::clone(&master_socket);

                let (tx, rx) = channel::<Value>(100);

                let rx_master = Arc::new(TokioMutex::new(rx));
                let rx_clone = Arc::clone(&rx_master);

                self.sender_channel = Some((Arc::new(TokioMutex::new(tx.clone())), rx_master));

                task::spawn(async move {

                    let socket = clone_socket;

                    loop {

                        let mut guard = rx_clone.lock().await;

                        if let Some(send_msg) = guard.recv().await {
                            println!("Received msg");
                            if let Ok(mut socket_handler) = socket.lock() {
                                match serde_json::to_string(&send_msg) {
                                    Ok(msg) => {
                                        let _ = socket_handler.write_message(Message::Text(msg));
                                        let _ = tx.send(json!(""));
                                    }
                                    Err(_err) => {
                                        let _ = tx.send(WebSocketError::MessageSendError.into());
                                    }
                                }
                            } else {
                                let _ = tx.send(WebSocketError::ErrorGetSocketLockFromSenderTask.into());
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

                    if let Ok(mut socket_handler) = socket.lock() {
                        loop {
                            match socket_handler.read_message() {
                                Ok(msg) => {
                                    println!("Received: {}", msg);
                                    let message_json = json!(msg.into_text().unwrap());
                                    let req = json!({
                                        "msg": message_json 
                                    });
                                    let _ = tx.send(req);
                                }
                                Err(_e) => {
                                    let _ = tx.send(WebSocketError::MessageReceiveError.into());
                                }
                            }
                        }
                    } else {
                        let _ = tx.send(WebSocketError::ErrorGetSocketLockFromReceiverTask.into());
                    };

                });

                self.socket = Some(master_socket);

                return Ok(());
            },
            Err(_err) => {
                return Err(WebSocketError::ConnectionError);
            }
        }
    }

    fn send(&mut self, msg:Value) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        
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

    fn close(&mut self) -> Result<(), WebSocketError> {

        let socket_clone:Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>;

        if let Some(socket) = &mut self.socket {
            socket_clone = Arc::clone(&socket);
        } else {
            return Err(WebSocketError::NotConnected);
        }

        let _x = if let Ok(mut socket_handler) = socket_clone.lock() {
            match socket_handler.close(None) {
                Ok(_sucess) => {
                    self.socket = None;
                    return Ok(());
                }
                Err(_err) => {
                    return Err(WebSocketError::WebSocketNotClosed);
                }
            }
        } else {
            return Err(WebSocketError::WebSocketNotClosed);
        };
    }
}
