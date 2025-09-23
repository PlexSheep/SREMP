use std::{fmt::Display, net::SocketAddr, sync::Arc};

use ed25519_dalek::VerifyingKey;

use crate::{error::CoreError, identity::format_key};

#[derive(Debug)]
pub enum NetworkEvent {
    ConnectionEstablished(SocketAddr, VerifyingKey),
    ConnectionLost(SocketAddr, VerifyingKey),
    IncomingMessage(SocketAddr, VerifyingKey, Arc<Vec<u8>>),
    MessageSent(SocketAddr, VerifyingKey, Arc<Vec<u8>>),
    ConnectionReset(SocketAddr),
    ConnectionFailed(SocketAddr, String),
    ListenerStarted(SocketAddr),
    ListenerFailed(CoreError),
    ListenerStopped,
}

impl Display for NetworkEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ConnectionEstablished(addr, key) =>
                    format!("Connection established with {addr} ({})", format_key(key)),
                Self::ConnectionLost(addr, key) =>
                    format!("Peer {addr} ({}) has disconnected", format_key(key)),
                Self::IncomingMessage(addr, key, _msg) =>
                    format!("Message received from {addr} ({})", format_key(key)),
                Self::MessageSent(addr, key, _msg) =>
                    format!("Message sent to {addr} ({})", format_key(key)),
                Self::ConnectionFailed(addr, reason) =>
                    format!("Connection to {addr} attempt was aborted: {reason}"),
                Self::ListenerStarted(addr) =>
                    format!("Listener for incoming connection was started on {addr}"),
                Self::ListenerStopped => "Listener for incoming connection was stopped".to_string(),
                Self::ConnectionReset(addr) =>
                    format!("Bad connection awards from {addr} was aborted",),
                Self::ListenerFailed(err) => format!("Listener failed: {err}"),
            }
        )
    }
}
