use std::rc::Rc;
use std::cell::RefCell;
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;

use crate::commands::getters::getters::ChainGetter;
use crate::commands::subscriptions::subscriptions::ChainSubscriptions;
use crate::WebSocket;
use crate::websocket::errors::WebSocketError;
use crate::websocket::implementations::message_channel::{MessageChannel, CallbackMessageChannel, WebsocketCloseRequest};
use crate::websocket::interface::SubscriptionCallback;
use crate::client::apis::{GrapheneApi, GrapheneApis};
use crate::websocket::implementations::listeners::spawn_callbacks_table;

pub struct GrapheneClient<'a> {
    ws_service: Option<Rc<RefCell<&'a mut WebSocket<'a>>>>,
    pub chain_getter: ChainGetter<'a>,
    chain_subscriptions: ChainSubscriptions<'a>,
    callback_id_counter: u64,
    callback_channel: Option<CallbackMessageChannel>,
    chain_id: Option<String>,
    apis: GrapheneApis
}

impl <'a>GrapheneClient<'a> {
    pub fn new() -> Self {
        Self {
            ws_service: None,
            chain_getter: ChainGetter::new(),
            chain_subscriptions: ChainSubscriptions::new(),
            callback_id_counter: 0,
            callback_channel: None,
            chain_id: None,
            apis: GrapheneApis::new()
        }
    }

    pub fn set_ws_connection(&mut self, ws: &'a mut WebSocket<'a>) {
        let websocket = Rc::new(RefCell::new(ws));
        self.chain_getter.set_ws_connection(Rc::clone(&websocket));
        self.chain_subscriptions.set_ws_connection(Rc::clone(&websocket));
        self.ws_service = Some(websocket);
    }

    pub async fn connect(&mut self) -> Result<(),WebSocketError> {
        if let Some(ws) = &self.ws_service {
            return ws.borrow_mut().connect().await;
        } else {
            return Err(WebSocketError::ConnectionError);
        }
    }

    pub async fn init(&mut self) -> Result<(),WebSocketError> {
        let _result = self.login(Some(String::from("init0")), Some(String::from("password"))).await?;
        self.chain_id = Some(self.chain_getter.get_chain_id().await?);
        self.apis.set_database_api(self.chain_getter.get_chain_api_id(GrapheneApi::Database(0)).await?);
        self.apis.set_network_api(self.chain_getter.get_chain_api_id(GrapheneApi::Network(0)).await?);
        self.apis.set_history_api(self.chain_getter.get_chain_api_id(GrapheneApi::History(0)).await?);
        self.apis.set_crypto_api(self.chain_getter.get_chain_api_id(GrapheneApi::Crypto(0)).await?);
        return Ok(());
    }

    pub fn create_callback_channel(&mut self) -> Sender<(u64, Value)>
    {

        let mut callback_channel = CallbackMessageChannel::new();

        let (
            rx_cb_register,
            rx_cb_unregister,
            rx_cb_call,
            tx_cb_call,
            rx_ws_close
        ) = callback_channel.create_channel();

        self.callback_channel = Some(callback_channel);

        tokio::task::spawn(async move {
            spawn_callbacks_table(
                rx_cb_register,
                rx_cb_unregister,
                rx_cb_call,
                rx_ws_close
            ).await;
        });

        return tx_cb_call;

    }

    pub async fn login(&mut self, username: Option<String>, password: Option<String>) -> Result<(),WebSocketError> {

        if let Some(ws) = &self.ws_service {

            let username_json: Value;
            let password_json: Value;
    
            if let Some(user_name) = username {
                username_json = json!(user_name);
            } else {
                username_json = json!("");
            }
    
            if let Some(pass) = password {
                password_json = json!(pass);
            } else {
                password_json = json!("");
            }
    
            let req = json!({
                "method": "call",
                "params": [1, "login", [username_json, password_json]],
                "id": 1
            });
    
            let _ = ws.borrow_mut().send(req).await?;

            let result = ws.borrow_mut().receive().await?;
    
            println!("Login Response: {:?}", result);
    
            return Ok(());

        } else {
            return Err(WebSocketError::ConnectionError);
        }

    }

    pub async fn subscribe_to_account(&mut self, cb: Box<SubscriptionCallback>, accounts: Vec<String>) -> Result<(), WebSocketError> {
        if let Ok(_res) = self.chain_subscriptions.get_full_accounts(self.callback_id_counter, accounts).await {
            if let Some(callback_channel) = &self.callback_channel {
                let _ = callback_channel.register_callback(self.callback_id_counter, cb).await;
                self.callback_id_counter += 1;
                return Ok(()); 
            } else {
                return Err(WebSocketError::SubscribingError);
            }   
        } else {
            return Err(WebSocketError::SubscribingError);
        }
    }

    pub async fn close(&mut self) -> Result<(),WebSocketError> {
        if let Some(ws) = &self.ws_service {
            return ws.borrow_mut().close().await;
        } else {
            return Err(WebSocketError::ConnectionError);
        }
    }
}