use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum ChainGetters {
    ChainId,
    DatabaseApiId,
    NetworkApiId,
    HistoryApiId,
    CryptoApiId
}

impl fmt::Display for ChainGetters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChainGetters::ChainId => write!(f,"ChainGetters::ChainId"),
            ChainGetters::DatabaseApiId => write!(f,"ChainGetters::DatabaseApiId"),
            ChainGetters::NetworkApiId => write!(f,"ChainGetters::NetworkApiId"),
            ChainGetters::HistoryApiId => write!(f,"ChainGetters::HistoryApiId"),
            ChainGetters::CryptoApiId => write!(f,"ChainGetters::CryptoApiId"),
        }
    }
}

