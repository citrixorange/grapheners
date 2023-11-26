use serde_json::Value;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChainResponse {
    method: String,
    params: Vec<Value>
}

impl ChainResponse {
    pub fn is_subcription_response(&self) -> bool {
        let method = &self.method;
        return method == "notice";
    }

    fn get_callback_id(&self) -> Option<u64> {

        if let Some(callback_id) = self.params.get(0).and_then(|cb_id| if let Some(id) = cb_id.as_u64() { Some(id) } else { None }) {
            return Some(callback_id);
        } else {
            return None;
        }

    }

    fn get_body_response(&self) -> Option<Value> {
        if let Some(body) = self.params.get(1) {
            return Some(body.clone());
        } else {
            return None;
        }
    }

    pub fn get_parsed_cb_response(&self) -> Option<(u64,Value)> {
        let cb_id:u64;
        let cb_response_body: Value;

        if let Some(id) = self.get_callback_id() {
            cb_id = id;
        } else {
            return None;
        }

        if let Some(body) = self.get_body_response() {
            cb_response_body = body;
        } else {
            return None;
        }

        return Some((cb_id,cb_response_body));

    }
}