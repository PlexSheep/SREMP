use std::{collections::HashMap, fmt::Display, net::SocketAddr, sync::Arc};

use ed25519_dalek::VerifyingKey;
use sremp_core::{
    chat::{Chat, messages::SharedMessage},
    identity::{UserIdentity, format_key},
};

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum UiEvent {
    ConnectionEstablished(SocketAddr, VerifyingKey),
    ConnectionLost(SocketAddr, VerifyingKey),
    IncomingMessage(SocketAddr, VerifyingKey, SharedMessage),
    MessageSent(SocketAddr, VerifyingKey, SharedMessage),
    ConnectionReset(SocketAddr),
    ConnectionFailed(SocketAddr, String),
    ListenerStarted(SocketAddr),
    ListenerStopped,
    IdentitySet(Option<UserIdentity>),
    LoadInitialChats(HashMap<VerifyingKey, Chat>),
    ChatLoaded(Chat),
    ChatNotFound(VerifyingKey),
}

impl Display for UiEvent {
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
                Self::IdentitySet(id) => {
                    if let Some(id) = id {
                        format!(
                            "working copy of user identity was set to {} ({})",
                            format_key(&id.identity.public_key),
                            id.identity.username()
                        )
                    } else {
                        "working copy of user identity was set to nothing".to_string()
                    }
                }
                Self::ChatLoaded(chat) => format!(
                    "Chat with '{}' was loaded",
                    chat.contact().identity.username()
                ),
                Self::LoadInitialChats(chats) => format!("Loaded {} chats", chats.len()),
                Self::ChatNotFound(key) =>
                    format!("Chat with key {} does not exist", format_key(key)),
            }
        )
    }
}
