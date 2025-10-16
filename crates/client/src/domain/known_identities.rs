use std::{
    collections::{HashMap, hash_map::Entry},
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use sremp_core::identity::{ContactId, ContactIdentity, Identity, Trust};

use crate::error::ClientResult;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KnownIdentities {
    inner: HashMap<ContactId, ContactIdentity>,
}

impl KnownIdentities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_or_update(&mut self, iden: &Identity) -> ClientResult<&ContactIdentity> {
        let id = iden.id();
        match self.inner.entry(id) {
            Entry::Occupied(mut entry) => {
                let contact = entry.get_mut();
                contact.seen();
            }
            Entry::Vacant(entry) => {
                let contact = ContactIdentity::from_peer_identity(iden.clone(), Trust::Unknown)?;
                entry.insert(contact);
            }
        };
        Ok(&self.inner[&id])
    }
}

impl Deref for KnownIdentities {
    type Target = HashMap<ContactId, ContactIdentity>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for KnownIdentities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
