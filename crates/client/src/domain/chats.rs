use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use sremp_core::{
    chat::{Chat, messages::SharedMessage},
    identity::ContactId,
};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Chats {
    inner: HashMap<ContactId, Chat>,
}

impl Chats {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_or_update(&mut self, id: ContactId) -> &mut Chat {
        self.entry(id).or_default()
    }

    pub fn add_message(&mut self, id: ContactId, msg: SharedMessage) {
        self.create_or_update(id).add_message(msg);
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
