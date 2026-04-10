#![allow(dead_code)]

use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Window detection error: {0}")]
    WindowDetection(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl AppError {
    pub fn session(msg: impl Into<String>) -> Self {
        AppError::Session(msg.into())
    }

    pub fn config(msg: impl Into<String>) -> Self {
        AppError::Config(msg.into())
    }
}
