use thiserror::Error;

/// Errors generated by the relay protocol.
#[derive(Debug, Error)]
pub enum Error {
    /// Error generated when the PEM encoding does not match
    /// the expected format.
    #[error("encoding in PEM is invalid")]
    BadKeypairPem,

    /// Error generated decoding the kind for an encoding is invalid.
    #[error("invalid encoding kind identifier {0}")]
    EncodingKind(u8),

    /// Error generated by input/output.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Error generated by the noise protocol library.
    #[error(transparent)]
    Snow(#[from] snow::error::Error),

    /// Error generated decoding PEM data.
    #[error(transparent)]
    Pem(#[from] pem::PemError),
}
