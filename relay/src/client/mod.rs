#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod native;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub use native::{EventLoop, NativeClient, Notification};

/// Events dispatched by the client.
#[derive(Debug)]
pub enum Event {
    /// Event dispatched when a handshake with the server
    /// is completed.
    ServerConnected,

    /// Event dispatched when a handshake with a peer
    /// has been completed.
    PeerConnected {
        /// Peer identifier, hex-encoded public key.
        peer_id: String,
    },
}

/// Options used to create a new websocket client.
pub struct ClientOptions {
    /// Client static keypair.
    pub keypair: snow::Keypair,
    /// Public key for the server to connect to.
    pub server_public_key: Vec<u8>,
}
