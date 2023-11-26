use std::rc::Rc;
use std::cell::RefCell;

use serde::{Deserialize};
use serde_json::{json};
use crate::websocket::{errors::WebSocketError, service::WebSocket};
use crate::client::apis::GrapheneApi;

#[derive(Debug, Deserialize)]
struct ChainWebsocketStringResponse {
    pub id: u8,
    pub result: String
}

#[derive(Debug, Deserialize)]
struct ChainWebsocketNumberResponse {
    pub id: u8,
    pub result: u8
}

pub struct ChainGetter<'a> {
    ws_service: Option<Rc<RefCell<&'a mut WebSocket<'a>>>>
} 

impl <'a>ChainGetter<'a> {

    pub fn new() -> Self {
        Self { ws_service: None }
    }

    pub fn set_ws_connection(&mut self, ws: Rc<RefCell<&'a mut WebSocket<'a>>>) {
        self.ws_service = Some(ws);
    }

    pub async fn get_chain_id(&mut self) -> Result<String, WebSocketError> {

        if let Some(ws) = &self.ws_service {

            let req = json!({
                "method": "call",
                "params": [0, "get_chain_id", []],
                "id": 1
            });
            
            let _ = ws.borrow_mut().send(req).await?;

            let result = ws.borrow_mut().receive().await?;

            let websocket_response:Result<ChainWebsocketStringResponse,serde_json::Error> = serde_json::from_value(result);

            if let Ok(response) = websocket_response {
                return Ok(response.result);
            } else {
                return Err(WebSocketError::MessageReceiveError);
            }

        } else {
            return Err(WebSocketError::NotConnected);
        }
        
    }

    pub async fn get_chain_api_id(&mut self, api: GrapheneApi) -> Result<u8, WebSocketError> {

        if let Some(ws) = &self.ws_service {

            let api_name: String = api.into();

            let req = json!({
                "method": "call",
                "params": [1, api_name.as_str(), []],
                "id": 1
            });
    
            let _ = ws.borrow_mut().send(req).await?;
    
            let result = ws.borrow_mut().receive().await?;
    
            let websocket_response:Result<ChainWebsocketNumberResponse,serde_json::Error> = serde_json::from_value(result);
    
            if let Ok(response) = websocket_response {
                return Ok(response.result);
            } else {
                return Err(WebSocketError::MessageReceiveError);
            }

        } else {
            return Err(WebSocketError::NotConnected);
        }
        
    }

    pub async fn get_chain_objects(&mut self, objects: Vec<String>) -> Result<(), WebSocketError> {

        if let Some(ws) = &self.ws_service {

            let req = json!({
                "method": "call",
                "params": [0, "get_objects", [objects]],
                "id": 1
            });
    
            let _ = ws.borrow_mut().send(req).await?;
    
            let _result = ws.borrow_mut().receive().await?;
    
            return Ok(());

        } else {
            return Err(WebSocketError::NotConnected);
        }

    }

}

