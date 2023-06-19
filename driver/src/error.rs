use thiserror::Error;

/// Errors generated by the driver.
#[derive(Debug, Error)]
pub enum Error {
    /// Error when the session identifier for an incoming message
    /// does not match the session identifier assigned to the bridge.
    #[error("session identifier mismatch for incoming message")]
    SessionIdMismatch,

    /// Error generated when a session identifier is required.
    #[error("session identifier required")]
    SessionIdRequired,

    #[cfg(feature = "gg20")]
    /// GG20 driver errors.
    #[error(transparent)]
    GG20(#[from] crate::gg20::Error),

    /// Client library errors.
    #[error(transparent)]
    Client(#[from] mpc_relay_client::Error),
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl From<Error> for wasm_bindgen::JsValue {
    fn from(value: Error) -> Self {
        let s = value.to_string();
        wasm_bindgen::JsValue::from_str(&s)
    }
}
