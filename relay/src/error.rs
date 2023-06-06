use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Error generated when a file is expected.
    #[error(r#"not a file "{0}""#)]
    NotFile(PathBuf),

    /// Error generated when the config server key file was not specified.
    #[error("server config requires path to a key file")]
    KeyFileRequired,

    /// Error generated when the config server key file was not found.
    #[error(r#"key file "{0}" not found"#)]
    KeyNotFound(PathBuf),

    /// Error generated when the PEM encoding does not match
    /// the expected format.
    #[error("encoding in PEM is invalid")]
    BadKeypairPem,

    /// Error generated when a file does not have a parent directory.
    #[error("no parent directory")]
    NoParentDir,

    /// Error generated when a client request does not receive a binary
    /// reply message.
    #[error("no parent directory")]
    BinaryReplyExpected,

    /// Error generated by input/output.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Error generated by the web server library.
    #[error(transparent)]
    Axum(#[from] axum::Error),

    /// Error generated by the noise protocol library.
    #[error(transparent)]
    Snow(#[from] snow::error::Error),

    /// Error generated serializing or deserializing JSON.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// Error generated parsing TOML.
    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    /// Error generated when a header value is invalid.
    #[error(transparent)]
    HeaderValue(#[from] axum::http::header::InvalidHeaderValue),

    /// Error generated decoding PEM data.
    #[error(transparent)]
    Pem(#[from] pem::PemError),

    /// Error generated by the client websocket library.
    #[error(transparent)]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error(transparent)]
    MpscSend(#[from] tokio::sync::mpsc::error::SendError<Vec<u8>>),

    #[error(transparent)]
    BroadcastSend(
        #[from] tokio::sync::broadcast::error::SendError<Vec<u8>>,
    ),
}
