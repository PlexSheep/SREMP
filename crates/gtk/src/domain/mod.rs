use std::{collections::HashMap, ops::Deref, rc::Rc, sync::RwLock};

use async_channel::{Receiver, Sender};
use ed25519_dalek::VerifyingKey;

use sremp_client::domain::{UiCommand, UiEvent};
use sremp_core::{
    chat::Chat,
    error::CoreResult,
    identity::{UserIdentity, format_key},
};

pub(crate) mod find;
pub(crate) mod listen;
pub(crate) mod tracked_widgets;
use tracked_widgets::TrackedWidgets;

use crate::{domain::listen::ListenerStatus, gui::identity::show_identity_created_success};

#[derive(Debug)]
pub(crate) struct UiDomain {
    pub(crate) command_channel: Sender<UiCommand>,
    pub(crate) event_channel: Receiver<UiEvent>,
    listen_status: ListenerStatus,
    // actual ui stuff
    pub(crate) tracked_widgets: TrackedWidgets,
    chats: HashMap<VerifyingKey, Chat>,
    user_identity: Option<UserIdentity>,
    selected_chat: Option<VerifyingKey>,
}

#[derive(Debug, Clone)]
pub(crate) struct UiDomainSync {
    inner: Rc<RwLock<UiDomain>>,
}

impl UiDomain {
    #[must_use]
    pub(crate) fn new(
        command_channel: Sender<UiCommand>,
        event_channel: Receiver<UiEvent>,
    ) -> Self {
        Self {
            command_channel,
            event_channel,
            chats: HashMap::with_capacity(32),
            tracked_widgets: Default::default(),
            user_identity: Default::default(),
            selected_chat: Default::default(),
            listen_status: Default::default(),
        }
    }
    #[must_use]
    #[inline]
    pub(crate) fn into_sync(self) -> UiDomainSync {
        UiDomainSync::new(self)
    }

    #[inline]
    pub(crate) fn send_cmd(&self, cmd: UiCommand) {
        log::info!("Sending ui command: {cmd}");
        self.command_channel
            .send_blocking(cmd)
            .expect("could not send Ui Command");
        log::trace!("ui command sent");
    }

    #[inline]
    pub(crate) fn user_identity(&self) -> Option<&UserIdentity> {
        self.user_identity.as_ref()
    }

    #[inline]
    pub(crate) fn set_user_identity(&mut self, iden: Option<UserIdentity>) {
        // we actually set the user identity working copy for the ui domain if the application
        // domain emits the event that tells us to do so
        self.send_cmd(UiCommand::SetIdentity(iden));
    }

    pub(crate) fn apply_user_identity(&mut self, iden: Option<UserIdentity>) {
        self.user_identity = iden.clone();

        if let Some(iden) = iden {
            log::info!(
                "Created new user identity for username '{}': {}",
                iden.identity.username(),
                format_key(&iden.identity.public_key)
            );

            show_identity_created_success(iden);
        }
    }

    pub(crate) fn chats(&self) -> &HashMap<VerifyingKey, Chat> {
        &self.chats
    }

    pub(crate) fn set_current_chat(&mut self, key: Option<VerifyingKey>) {
        if let Some(key) = key {
            if self.chats.contains_key(&key) {
                self.selected_chat = Some(key);
            }
        } else {
            log::warn!("Selected a chat that does not exist")
        }
    }

    pub(crate) fn current_chat(&self) -> Option<&Chat> {
        self.chats.get(self.selected_chat.as_ref()?)
    }
}

impl UiDomainSync {
    #[must_use]
    #[inline]
    pub(crate) fn new(state: UiDomain) -> Self {
        Self {
            inner: Rc::new(RwLock::new(state)),
        }
    }

    #[must_use]
    #[inline]
    pub(crate) fn borrow(&self) -> std::sync::RwLockReadGuard<'_, UiDomain> {
        self.read().expect("could not read UiDomain state")
    }

    #[must_use]
    #[inline]
    pub(crate) fn borrow_mut(&self) -> std::sync::RwLockWriteGuard<'_, UiDomain> {
        self.write().expect("could not read UiDomain state")
    }
}

impl Deref for UiDomainSync {
    type Target = Rc<RwLock<UiDomain>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
