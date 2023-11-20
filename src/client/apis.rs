use std::fmt;
use crate::config::config::CONFIG;

#[derive(Debug, Copy, Clone)]
pub enum GrapheneApi {
    Database(u8),
    Network(u8),
    History(u8),
    Crypto(u8),
    Custom(u8)
}


impl fmt::Display for GrapheneApi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GrapheneApi::Database(_) => write!(f,"GrapheneApi::Database"),
            GrapheneApi::Network(_) => write!(f,"GrapheneApi::Network"),
            GrapheneApi::History(_) => write!(f,"GrapheneApi::History"),
            GrapheneApi::Crypto(_) => write!(f,"GrapheneApi::Crypto"),
            GrapheneApi::Custom(_) => write!(f,"GrapheneApi::Custom"),
        }
    }
}

impl From<GrapheneApi> for u8 {
    fn from(api_id: GrapheneApi) -> u8 {
        match api_id {
            GrapheneApi::Database(id) => id as u8,
            GrapheneApi::Network(id) => id as u8,
            GrapheneApi::History(id) => id as u8,
            GrapheneApi::Crypto(id) => id as u8,
            GrapheneApi::Custom(id) => id as u8,
        }
    }
}

impl From<GrapheneApi> for String {
    fn from(api_id: GrapheneApi) -> String {
        match api_id {
            GrapheneApi::Database(_id) => String::from("database"),
            GrapheneApi::Network(_id) => String::from("network_broadcast"),
            GrapheneApi::History(_id) => String::from("history"),
            GrapheneApi::Crypto(_id) => String::from("crypto"),
            GrapheneApi::Custom(id) => CONFIG.get_custom_api_name(id),
        }
    }
}

pub struct GrapheneApis {
    database: Option<GrapheneApi>,
    network: Option<GrapheneApi>,
    history: Option<GrapheneApi>,
    crypto: Option<GrapheneApi>,
    custom: Option<Vec<GrapheneApi>>
}

impl GrapheneApis {
    pub fn new() -> Self {
        GrapheneApis {
            database: None,
            network: None,
            history: None,
            crypto: None,
            custom: None
        }
    }

    pub fn set_database_api(&mut self, id:u8) {
        self.database = Some(GrapheneApi::Database(id));
    }

    pub fn set_network_api(&mut self, id:u8) {
        self.network = Some(GrapheneApi::Network(id));
    }

    pub fn set_history_api(&mut self, id:u8) {
        self.history = Some(GrapheneApi::History(id));
    }

    pub fn set_crypto_api(&mut self, id:u8) {
        self.crypto = Some(GrapheneApi::Crypto(id));
    }

    pub fn set_custom_api(&mut self, ids:Vec<u8>) {
        self.custom = Some(ids.iter().map(|id| GrapheneApi::Custom(*id)).collect::<Vec<GrapheneApi>>());
    }

    pub fn get_database_api(&self) -> Option<u8> {
        if let Some(id) = self.database {
            return Some(id.into());
        } else {
            return None;
        }
    }

    pub fn get_network_api(&self) -> Option<u8> {
        if let Some(id) = self.network {
            return Some(id.into());
        } else {
            return None;
        }
    }

    pub fn get_history_api(&self) -> Option<u8> {
        if let Some(id) = self.history {
            return Some(id.into());
        } else {
            return None;
        }
    }

    pub fn get_crypto_api(&self) -> Option<u8> {
        if let Some(id) = self.crypto {
            return Some(id.into());
        } else {
            return None;
        }
    }

    pub fn get_custom_api(&self) -> Option<Vec<u8>> {
        if let Some(ids) = &self.custom {
            return Some(ids.iter().map(|id| u8::from(*id)).collect::<Vec<u8>>());
        } else {
            return None;
        }
    }

}


