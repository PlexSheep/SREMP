use std::{collections::HashMap, fmt::Display, net::SocketAddr, sync::Arc};

use sremp_core::{
    chat::{Chat, messages::SharedMessage},
    identity::{ContactId, UserIdentity},
};

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum UiEvent {
    ConnectionEstablished(SocketAddr, ContactId),
    ConnectionLost(SocketAddr, ContactId),
    IncomingMessage(SocketAddr, ContactId, SharedMessage),
    MessageSent(SocketAddr, ContactId, SharedMessage),
    ConnectionReset(SocketAddr),
    ConnectionFailed(SocketAddr, String),
    ListenerStarted(SocketAddr),
    ListenerStopped,
    IdentitySet(Option<Arc<UserIdentity>>),
    LoadInitialChats(HashMap<ContactId, Chat>),
    ChatLoaded(Chat),
    ChatNotFound(ContactId),
}

impl Display for UiEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ConnectionEstablished(addr, id) =>
                    format!("Connection established with {addr} ({id})"),
                Self::ConnectionLost(addr, id) => format!("Peer {addr} ({id}) has disconnected"),
                Self::IncomingMessage(addr, id, _msg) =>
                    format!("Message received from {addr} ({id})"),
                Self::MessageSent(addr, id, _msg) => format!("Message sent to {addr} ({id})"),
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
                            &id.identity.id(),
                            id.identity.username()
                        )
                    } else {
                        "working copy of user identity was set to nothing".to_string()
                    }
                }
                Self::ChatLoaded(chat) => format!(
                    "Chat was loaded (last msg at {:?})",
                    chat.messages().last().map(|m| m.meta().time_received)
                ),
                Self::LoadInitialChats(chats) => format!("Loaded {} chats", chats.len()),
                Self::ChatNotFound(id) => format!("Chat with id {id} does not exist"),
            }
        )
    }
}
