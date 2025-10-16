use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::{Deref, DerefMut},
};

use crate::{
    identity::{ContactId, Identity},
    net::connection::Connection,
};

#[derive(Debug, Default)]
pub struct ActiveConnections {
    inner: HashMap<SocketAddr, ConnectionData>,
}

#[derive(Debug)]
pub struct ConnectionData {
    pub conn: Connection,
    pub iden: Identity,
}

impl ActiveConnections {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find_socket_addr_for_contact(&self, id: &ContactId) -> Option<SocketAddr> {
        let mut kv: Vec<(&SocketAddr, &ConnectionData)> = self.inner.iter().collect();
        kv.sort();
        let correct_idx = match kv.binary_search_by_key(id, |(_, v)| v.iden.id()) {
            Ok(idx) => idx,
            Err(_) => return None,
        };
        let conn = kv[correct_idx];
        Some(*conn.0)
    }
}

impl Deref for ActiveConnections {
    type Target = HashMap<SocketAddr, ConnectionData>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ActiveConnections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl PartialEq for ConnectionData {
    fn eq(&self, other: &Self) -> bool {
        self.iden == other.iden
    }
}

impl PartialOrd for ConnectionData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ConnectionData {}

impl Ord for ConnectionData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iden.id().cmp(&other.iden.id())
    }
}
