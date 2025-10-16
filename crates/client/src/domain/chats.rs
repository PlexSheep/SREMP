use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use sremp_core::{
    chat::Chat,
    identity::{ContactId, ContactIdentity},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Chats {
    inner: HashMap<ContactId, Chat>,
}

impl Chats {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_or_update(&mut self, iden: &ContactIdentity) -> &mut Chat {
        let id = iden.id();
        self.entry(id).or_default()
    }
}

impl Deref for Chats {
    type Target = HashMap<ContactId, Chat>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Chats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
