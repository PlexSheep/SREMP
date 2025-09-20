use std::{fmt::Display, net::SocketAddr, sync::Arc};

use ed25519_dalek::VerifyingKey;

use crate::identity::{UserIdentity, format_key};

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum NetworkCommand {
    Connect(SocketAddr),
    Disconnect(SocketAddr),
    SendMessage(SocketAddr, VerifyingKey, Arc<Vec<u8>>),
    /// Associated [SocketAddr] is the local addres on which to listen, not a remote address
    StartListener(SocketAddr),
    StopListener,
    SetIdentity(Option<UserIdentity>),
}

impl Display for NetworkCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Connect(addr) => format!("Connect to {addr}"),
                Self::Disconnect(addr) => format!("Disconnect from {addr}"),
                Self::SendMessage(addr, id, _msg) =>
                    format!("Send Message to {addr}: {}", format_key(id)),
                Self::StartListener(addr) =>
                    format!("Start listening for incoming connection on {addr}"),
                Self::StopListener => "Stop listening for incoming connections".to_string(),
                Self::SetIdentity(id) => {
                    if let Some(id) = id {
                        format!(
                            "Set working copy of user identity to {} ({})",
                            format_key(&id.identity.public_key),
                            id.identity.username()
                        )
                    } else {
                        "Set working copy of user identity to <None>".to_string()
                    }
                }
            }
        )
    }
}
