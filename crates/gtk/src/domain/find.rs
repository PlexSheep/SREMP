use sremp_client::domain::known_identities::SharedContact;
use sremp_core::{chat::Chat, identity::ContactId};

use crate::domain::UiDomain;

impl UiDomain {
    pub(crate) fn find_contact(&self, key: &ContactId) -> Option<SharedContact> {
        todo!()
    }
    pub(crate) fn find_chat(&self, key: &ContactId) -> Option<&Chat> {
        todo!()
    }
}
