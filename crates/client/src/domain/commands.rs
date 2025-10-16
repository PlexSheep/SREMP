use std::{fmt::Display, net::SocketAddr};

use sremp_core::{
    chat::messages::SharedMessage,
    identity::{ContactId, UserIdentity},
};

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum UiCommand {
    SetIdentity(Option<UserIdentity>),
    SendMessage(ContactId, SharedMessage),
    StartListener(SocketAddr),
    StopListener,
    Connect(SocketAddr),
    Disconnect(SocketAddr),
    LoadChat(ContactId),
}

impl Display for UiCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Connect(addr) => format!("Connect to {addr}"),
                Self::Disconnect(addr) => format!("Disconnect from {addr}"),
                Self::SendMessage(id, _msg) => format!("Send Message to {}", id),
                Self::StartListener(addr) =>
                    format!("Start listening for incoming connection on {addr}"),
                Self::StopListener => "Stop listening for incoming connections".to_string(),
                Self::SetIdentity(id) => {
                    if let Some(id) = id {
                        format!(
                            "Set working copy of user identity to {} ({})",
                            id.identity.id(),
                            id.identity.username()
                        )
                    } else {
                        "Set working copy of user identity to <None>".to_string()
                    }
                }
                Self::LoadChat(id) => format!("Load chat for contact {}, id any", id),
            }
        )
    }
}
