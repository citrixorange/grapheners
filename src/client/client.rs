use std::rc::Rc;
use std::cell::RefCell;
use serde_json::{json, Value};

use crate::commands::getters::getters::ChainGetter;
use crate::WebSocket;
use crate::websocket::errors::WebSocketError;

pub struct GrapheneClient<'a> {
    ws_service: Rc<RefCell<&'a mut WebSocket<'a>>>,
    pub chain_getter: ChainGetter<'a>
}

impl <'a> GrapheneClient<'a> {
    pub fn new(ws_service: &'a mut WebSocket<'a>) -> Self {
        let a = Rc::new(RefCell::new(ws_service));

        Self {
            ws_service: Rc::clone(&a),
            chain_getter: ChainGetter::new(Rc::clone(&a))
        }
    }

    pub async fn connect(&mut self) -> Result<(),WebSocketError> {
        return self.ws_service.borrow_mut().connect().await;
    }

    pub async fn login(&mut self, username: Option<String>, password: Option<String>) -> Result<(),WebSocketError> {

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

        self.ws_service.borrow_mut().send(req).await?;
        let result = self.ws_service.borrow_mut().receive().await?;

        println!("Login Response: {:?}", result);

        return Ok(());
    }

    pub async fn close(&mut self) -> Result<(),WebSocketError> {
        return self.ws_service.borrow_mut().close().await;
    }
}