use std::rc::Rc;
use std::cell::RefCell;
use json::{object};

use crate::commands::getters::getters::ChainGetter;
use crate::WebSocket;

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

    pub fn connect(&mut self) {

        let req = object!{
            method: "call",
            params: [1, "login", ["",""]],
            id: 1
        };

        let ws = Rc::clone(&self.ws_service);

        let _ = ws.borrow_mut().send(req);

        let _result = ws.borrow_mut().receive();

        let req = object!{
            method: "call",
            params: [1, "database", []],
            id: 2
        };

        let ws = Rc::clone(&self.ws_service);

        let _ = ws.borrow_mut().send(req);

        let _result = ws.borrow_mut().receive();
    }
}