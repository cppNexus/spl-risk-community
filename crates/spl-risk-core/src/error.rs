use thiserror::Error;

#[derive(Error, Debug)]
pub enum RiskError {
    #[error("RPC request failed: {0}")]
    RpcError(String),
    
    #[error("Invalid token account: {0}")]
    InvalidToken(String),
    
    #[error("Not an SPL token")]
    NotSplToken,
    
    #[error("Timeout exceeded")]
    Timeout,
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Data parsing error: {0}")]
    ParseError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[cfg(feature = "lp-analysis")]
    #[error("LP analysis failed: {0}")]
    LpAnalysisError(String),
}


impl From<serde_json::Error> for RiskError {
    fn from(err: serde_json::Error) -> Self {
        RiskError::ParseError(err.to_string())
    }
}