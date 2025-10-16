use std::{
    collections::{HashMap, hash_map::Entry},
    ops::{Deref, DerefMut},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use sremp_core::{
    identity::{ContactId, ContactIdentity, Identity, Trust},
    ser_helper::*,
};

use crate::error::ClientResult;

pub type SharedContact = Arc<ContactIdentity>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KnownIdentities {
    #[serde(serialize_with = "ser_arc_hm", deserialize_with = "deser_arc_hm")]
    inner: HashMap<ContactId, SharedContact>,
}

impl KnownIdentities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_or_update(&mut self, iden: &Identity) -> ClientResult<SharedContact> {
        let id = iden.id();
        match self.inner.entry(id.clone()) {
            Entry::Occupied(mut entry) => {
                let mut contact: ContactIdentity = (**entry.get()).clone();
                contact.seen();
                entry.insert(contact.into());
            }
            Entry::Vacant(entry) => {
                let contact = ContactIdentity::from_peer_identity(iden.clone(), Trust::Unknown)?;
                entry.insert(contact.into());
            }
        };
        Ok(self.inner[&id].clone())
    }
}

impl Deref for KnownIdentities {
    type Target = HashMap<ContactId, SharedContact>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for KnownIdentities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
