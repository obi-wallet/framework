use crate::Result;
use async_trait::async_trait;
use mpc_relay_protocol::SessionId;
use serde::Serialize;

/// Enumeration of available transports.
pub enum Transport {
    /// Relay websocket client.
    Relay(crate::Client),
    // NOTE: later we will add a Peer variant using
    // NOTE: as WebRTC data channel for communication
}

#[async_trait]
impl NetworkTransport for Transport {
    fn public_key(&self) -> &[u8] {
        match self {
            Transport::Relay(client) => client.public_key(),
        }
    }

    async fn connect(&mut self) -> Result<()> {
        match self {
            Transport::Relay(client) => client.connect().await,
        }
    }

    async fn connect_peer(
        &mut self,
        public_key: &[u8],
    ) -> Result<()> {
        match self {
            Transport::Relay(client) => {
                client.connect_peer(public_key).await
            }
        }
    }

    async fn send_json<S>(
        &mut self,
        public_key: &[u8],
        payload: &S,
        session_id: Option<SessionId>,
    ) -> Result<()>
    where
        S: Serialize + Send + Sync + ?Sized,
    {
        match self {
            Transport::Relay(client) => {
                client
                    .send_json(public_key, payload, session_id)
                    .await
            }
        }
    }

    async fn send_blob(
        &mut self,
        public_key: &[u8],
        payload: Vec<u8>,
        session_id: Option<SessionId>,
    ) -> Result<()> {
        match self {
            Transport::Relay(client) => {
                client
                    .send_blob(public_key, payload, session_id)
                    .await
            }
        }
    }

    async fn new_session(
        &mut self,
        participant_keys: Vec<Vec<u8>>,
    ) -> Result<()> {
        match self {
            Transport::Relay(client) => {
                client.new_session(participant_keys).await
            }
        }
    }

    async fn register_connection(
        &mut self,
        session_id: &SessionId,
        peer_key: &[u8],
    ) -> Result<()> {
        match self {
            Transport::Relay(client) => {
                client.register_connection(session_id, peer_key).await
            }
        }
    }

    async fn close_session(
        &mut self,
        session_id: SessionId,
    ) -> Result<()> {
        match self {
            Transport::Relay(client) => {
                client.close_session(session_id).await
            }
        }
    }

    async fn broadcast_json<S>(
        &mut self,
        session_id: &SessionId,
        recipient_public_keys: &[Vec<u8>],
        payload: &S,
    ) -> Result<()>
    where
        S: Serialize + Send + Sync + ?Sized,
    {
        match self {
            Transport::Relay(client) => {
                client
                    .broadcast_json(
                        session_id,
                        recipient_public_keys,
                        payload,
                    )
                    .await
            }
        }
    }

    async fn broadcast_blob(
        &mut self,
        session_id: &SessionId,
        recipient_public_keys: &[Vec<u8>],
        payload: Vec<u8>,
    ) -> Result<()> {
        match self {
            Transport::Relay(client) => {
                client
                    .broadcast_blob(
                        session_id,
                        recipient_public_keys,
                        payload,
                    )
                    .await
            }
        }
    }

    async fn close(&self) -> Result<()> {
        match self {
            Transport::Relay(client) => client.close().await,
        }
    }
}

/// Trait for network clients.
#[async_trait]
pub trait NetworkTransport {
    /// Public key for this client.
    fn public_key(&self) -> &[u8];

    /// Perform initial handshake with the server.
    async fn connect(&mut self) -> Result<()>;

    /// Handshake with a peer.
    ///
    /// Peer already exists error is returned if this
    /// client is already connecting to the peer.
    async fn connect_peer(&mut self, public_key: &[u8])
        -> Result<()>;

    /// Send a JSON message to a peer.
    async fn send_json<S>(
        &mut self,
        public_key: &[u8],
        payload: &S,
        session_id: Option<SessionId>,
    ) -> Result<()>
    where
        S: Serialize + Send + Sync + ?Sized;

    /// Send a binary message to a peer.
    async fn send_blob(
        &mut self,
        public_key: &[u8],
        payload: Vec<u8>,
        session_id: Option<SessionId>,
    ) -> Result<()>;

    /// Create a new session.
    ///
    /// Do not include the public key of the initiator for the new
    /// session; it is automatically included as the session owner.
    async fn new_session(
        &mut self,
        participant_keys: Vec<Vec<u8>>,
    ) -> Result<()>;

    /// Register a peer connection in a session.
    async fn register_connection(
        &mut self,
        session_id: &SessionId,
        peer_key: &[u8],
    ) -> Result<()>;

    /// Close a session.
    async fn close_session(
        &mut self,
        session_id: SessionId,
    ) -> Result<()>;

    /// Broadcast a JSON message in the context of a session.
    async fn broadcast_json<S>(
        &mut self,
        session_id: &SessionId,
        recipient_public_keys: &[Vec<u8>],
        payload: &S,
    ) -> Result<()>
    where
        S: Serialize + Send + Sync + ?Sized;

    /// Broadcast a binary message in the context of a session.
    async fn broadcast_blob(
        &mut self,
        session_id: &SessionId,
        recipient_public_keys: &[Vec<u8>],
        payload: Vec<u8>,
    ) -> Result<()>;

    /// Close the socket connection.
    async fn close(&self) -> Result<()>;
}
