use fast_websocket_client::{client, connect};
use std::sync::Arc;
use tokio::task;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Sender, Receiver};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

use crate::websocket::{
    interface::IWebSocket, 
    errors::WebSocketError,
    implementations::{
        message_channel::{MessageChannel, WebsocketCloseRequest},
        listeners::{
            spawn_sender_task, 
            spawn_receiver_task
        }
    }
};

pub struct FastWebsocketClient {
    socket: Option<Arc<Mutex<client::Online>>>,
    sender_channel: Option<MessageChannel<Value>>,
    receiver_channel: Option<MessageChannel<Value>>,
    callback_channel: Option<Sender<(u64, Value)>>,
    url: String
}

impl FastWebsocketClient {
    pub fn new(url: String) -> Self {
        Self {
            socket: None,
            sender_channel: None,
            receiver_channel: None,
            callback_channel: None,
            url: url
        }
    }

    fn create_sender_channel(&mut self) -> (Sender<Value>,Receiver<Value>) {

        let mut channel = MessageChannel::<Value>::new();
        let (tx,rx) = channel.create_channel();
        self.sender_channel = Some(channel);
        return (tx, rx);
    }

    fn create_receiver_channel(&mut self) -> (Sender<Value>,Receiver<Value>) {

        let mut channel = MessageChannel::<Value>::new();
        let (tx,rx) = channel.create_channel();
        self.receiver_channel = Some(channel);
        return (tx, rx);
    }

    pub fn set_callback_channel(&mut self, channel: Sender<(u64, Value)>) {
        self.callback_channel = Some(channel);
    }

}

impl IWebSocket for FastWebsocketClient {

    fn connect(&mut self) -> Pin<Box<dyn Future<Output = Result<(), WebSocketError>> + '_>> {
        let future = async {
            match connect(&self.url).await {
                Ok(ws) => {

                    let master_socket = Arc::new(Mutex::new(ws));

                    let clone_socket = Arc::clone(&master_socket);
    
                    let (tx,rx) = self.create_sender_channel();

                    task::spawn(async move {

                        spawn_sender_task(clone_socket, rx, tx).await;
    
                    });

                    let clone_socket = Arc::clone(&master_socket);

                    let (tx,rx) = self.create_receiver_channel();

                    let tx_cb_call = self.callback_channel.take();
    
                    task::spawn(async move {
    
                        spawn_receiver_task(clone_socket, rx, tx, tx_cb_call).await;
    
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
                        Err(_message) => return Ok(msg)
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