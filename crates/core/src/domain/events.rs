use std::{fmt::Display, net::SocketAddr, sync::Arc};

use crate::{
    error::CoreError,
    identity::{ContactId, Identity},
};

#[derive(Debug)]
pub enum NetworkEvent {
    ConnectionEstablished(SocketAddr, Arc<Identity>),
    ConnectionLost(SocketAddr, ContactId),
    IncomingMessage(SocketAddr, ContactId, Arc<Vec<u8>>),
    MessageSent(SocketAddr, ContactId, Arc<Vec<u8>>),
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
                Self::ConnectionEstablished(addr, iden) =>
                    format!("Connection established with {addr} ({})", iden.id()),
                Self::ConnectionLost(addr, key) =>
                    format!("Peer {addr} ({key}) has disconnected"),
                Self::IncomingMessage(addr, key, _msg) =>
                    format!("Message received from {addr} ({key})"),
                Self::MessageSent(addr, key, _msg) => format!("Message sent to {addr} ({key})"),
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
