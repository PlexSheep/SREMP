use std::net::SocketAddr;

use crate::{chat::messages::SharedMessage, domain::NetworkDomain, identity::ContactIdentity};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod messages;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Chat {
    messages: Vec<SharedMessage>,
}

impl Chat {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn latest_timestamp(&self) -> Option<DateTime<Utc>> {
        Some(self.messages.last()?.meta().time_received)
    }

    pub fn messages(&self) -> &[SharedMessage] {
        &self.messages
    }

    pub fn add_message(&mut self, msg: impl Into<SharedMessage>) {
        self.messages.push(msg.into());
        self.sort();
    }

    fn sort(&mut self) {
        self.messages.sort_by_key(|m| m.meta().time_received);
    }
}

impl NetworkDomain {
    pub fn find_socket_addr_for_contact(&self, iden: &ContactIdentity) -> Option<SocketAddr> {
        self.active_connections
            .find_socket_addr_for_contact(&iden.id())
    }
}
