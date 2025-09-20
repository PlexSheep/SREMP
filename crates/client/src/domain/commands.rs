use std::{fmt::Display, net::SocketAddr};

use ed25519_dalek::VerifyingKey;
use sremp_core::{
    chat::messages::SharedMessage,
    identity::{UserIdentity, format_key},
};

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum UiCommand {
    SetIdentity(Option<UserIdentity>),
    SendMessage(VerifyingKey, SharedMessage),
    StartListener(SocketAddr),
    StopListener,
    Connect(SocketAddr),
    Disconnect(SocketAddr),
    LoadChat(VerifyingKey),
}

impl Display for UiCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Connect(addr) => format!("Connect to {addr}"),
                Self::Disconnect(addr) => format!("Disconnect from {addr}"),
                Self::SendMessage(id, _msg) => format!("Send Message to {}", format_key(id)),
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
                Self::LoadChat(id) => format!("Load chat for contact {}, id any", format_key(id)),
            }
        )
    }
}
