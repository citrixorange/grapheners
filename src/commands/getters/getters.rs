use std::rc::Rc;
use std::cell::RefCell;

use serde_json::{json, Value};
use crate::websocket::{errors::WebSocketError, service::WebSocket};

pub struct ChainGetter<'a> {
    ws_service: Rc<RefCell<&'a mut WebSocket<'a>>>
} 

impl <'a> ChainGetter<'a> {

    pub fn new(ws_service: Rc<RefCell<&'a mut WebSocket<'a>>>) -> Self {
        Self { ws_service }
    } 

    pub fn get_chain_id(&mut self) -> Result<Value, WebSocketError> {
        
        let req = json!({
            "method": "call",
            "params": [0, "get_chain_id", []],
            "id": 1
        });

        let _ = self.ws_service.borrow_mut().send(req);

        let result = self.ws_service.borrow_mut().receive();

        return result;
    }
}

