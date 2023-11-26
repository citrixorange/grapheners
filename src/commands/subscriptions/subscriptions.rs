use std::rc::Rc;
use std::cell::RefCell;

use serde_json::{json};
use crate::websocket::{errors::WebSocketError, service::WebSocket};

pub struct ChainSubscriptions<'a> {
    ws_service: Option<Rc<RefCell<&'a mut WebSocket<'a>>>>
} 

impl <'a>ChainSubscriptions<'a> {

    pub fn new() -> Self {
        Self {
            ws_service: None
        }
    }

    pub fn set_ws_connection(&mut self, ws: Rc<RefCell<&'a mut WebSocket<'a>>>) {
        self.ws_service = Some(ws);
    }

    pub async fn get_full_accounts(&mut self, cb_id: u64, accounts: Vec<String>) -> Result<(), WebSocketError> {

        if let Some(ws) = &self.ws_service {

            let req = json!({
                "method": "call",
                "params": [0, "set_subscribe_callback", [cb_id, true]],
                "id": 1
            });
    
            let _ = ws.borrow_mut().send(req).await?;

            let req = json!({
                "method": "call",
                "params": [0, "get_objects", [accounts]],
                "id": 1
            });
    
            let _ = ws.borrow_mut().send(req).await?;
    
            return Ok(());

        } else {
            return Err(WebSocketError::NotConnected);
        }
        
    }

}