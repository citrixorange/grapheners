use std::rc::Rc;
use std::cell::RefCell;

use json::{object, JsonValue};
use crate::websocket::{errors::WebSocketError, service::WebSocket};

pub struct ChainGetter<'a> {
    ws_service: Rc<RefCell<&'a mut WebSocket<'a>>>
} 

impl <'a> ChainGetter<'a> {

    pub fn new(ws_service: Rc<RefCell<&'a mut WebSocket<'a>>>) -> Self {
        Self { ws_service }
    } 

    pub fn get_chain_id(&mut self) -> Result<JsonValue, WebSocketError> {
        
        let req = object!{
            method: "call",
            params: [0, "get_chain_id", []],
            id: 1
        };

        let ws = Rc::clone(&self.ws_service);

        let _ = ws.borrow_mut().send(req);

        let result = ws.borrow_mut().receive();

        return result;
    }
}

