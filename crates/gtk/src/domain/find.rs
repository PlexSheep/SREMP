use ed25519_dalek::VerifyingKey;
use sremp_core::{chat::Chat, identity::ContactIdentity};

use crate::domain::UiDomain;

impl UiDomain {
    pub(crate) fn find_contact(&self, key: &VerifyingKey) -> Option<ContactIdentity> {
        todo!()
    }
    pub(crate) fn find_chat(&self, key: &VerifyingKey) -> Option<&Chat> {
        todo!()
    }
}
