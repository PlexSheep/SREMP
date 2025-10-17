use std::{ops::Deref, rc::Rc, sync::Arc};

use async_channel::{Receiver, Sender};
use tokio::sync::RwLock;

use sremp_client::domain::{UiCommand, UiEvent, chats::Chats, known_identities::KnownIdentities};
use sremp_core::{
    chat::Chat,
    identity::{ContactId, UserIdentity, format_key},
};

pub(crate) mod connect;
pub(crate) mod listen;
pub(crate) mod tracked_widgets;
use tracked_widgets::TrackedWidgets;

use crate::{
    RUNTIME,
    domain::listen::ListenerStatus,
    gui::{chat::ChatView, chats::ChatList, identity::show_identity_created_success},
};

#[derive(Debug)]
pub(crate) struct UiDomain {
    pub(crate) command_channel: Sender<UiCommand>,
    pub(crate) event_channel: Receiver<UiEvent>,
    pub(crate) listen_status: ListenerStatus,
    // actual UI stuff
    pub(crate) tracked_widgets: TrackedWidgets,
    user_identity: Option<Arc<UserIdentity>>,
}

#[derive(Debug, Clone)]
pub(crate) struct UiDomainSync {
    inner: Rc<RwLock<UiDomain>>,
}

impl UiDomain {
    #[must_use]
    #[cold]
    pub(crate) fn new(
        command_channel: Sender<UiCommand>,
        event_channel: Receiver<UiEvent>,
    ) -> Self {
        Self {
            command_channel,
            event_channel,
            tracked_widgets: Default::default(),
            user_identity: Default::default(),
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
    pub(crate) fn user_identity(&self) -> Option<Arc<UserIdentity>> {
        self.user_identity.clone()
    }

    #[inline]
    pub(crate) fn set_user_identity(&mut self, iden: Option<Arc<UserIdentity>>) {
        // we actually set the user identity working copy for the UI domain if the application
        // domain emits the event that tells us to do so
        self.send_cmd(UiCommand::SetIdentity(iden));
    }

    #[inline]
    pub(crate) fn apply_user_identity(&mut self, iden: Option<Arc<UserIdentity>>) {
        self.user_identity = iden.clone();

        if let Some(iden) = iden {
            log::info!(
                "Created new user identity for username '{}': {}",
                iden.identity.username(),
                format_key(&iden.identity.identity_key())
            );
            show_identity_created_success(iden);
        }
    }

    #[inline]
    pub(crate) fn chats(&self) -> Option<&Chats> {
        Some(self.tracked_widgets.chat_list()?.chats())
    }

    #[inline]
    pub(crate) fn chats_mut(&mut self) -> Option<&mut Chats> {
        Some(self.tracked_widgets.chat_list_mut()?.chats_mut())
    }

    #[inline]
    fn chat_view(&self) -> &ChatView {
        self.tracked_widgets
            .chat_view()
            .as_ref()
            .expect("could not get chat view")
    }

    #[inline]
    fn chat_view_mut(&mut self) -> &mut ChatView {
        self.tracked_widgets
            .chat_view_mut()
            .as_mut()
            .expect("could not get chat view")
    }

    #[inline]
    fn chat_list(&self) -> &ChatList {
        self.tracked_widgets
            .chat_list()
            .expect("could not get chat list")
    }

    #[inline]
    fn chat_list_mut(&mut self) -> &mut ChatList {
        self.tracked_widgets
            .chat_list_mut()
            .expect("could not get chat list")
    }

    pub(crate) fn set_selected_chat(&mut self, chat_id: Option<ContactId>) {
        self.chat_list_mut().set_selected_chat(chat_id.clone());
        if let Some(cid) = chat_id {
            let chat = self
                .chats_mut()
                .as_mut()
                .expect("could not get chats")
                .entry(cid)
                .or_default()
                .clone();
            self.chat_view_mut().set_chat(Some(chat));
        } else {
            self.chat_view_mut().set_chat(None);
        }
    }

    #[inline]
    pub(crate) fn contacts(&self) -> &KnownIdentities {
        self.chat_list().contacts()
    }

    #[inline]
    pub(crate) fn set_contacts(&mut self, contacts: KnownIdentities) {
        self.chat_list_mut().set_contacts(contacts.clone());
        self.chat_view_mut().set_contacts(contacts);
    }

    #[inline]
    pub(crate) fn selected_chat(&self) -> Option<ContactId> {
        self.tracked_widgets.chat_list()?.selected_chat()
    }

    #[inline]
    pub(crate) fn current_chat(&self) -> Option<&Chat> {
        self.chats()?.get(self.selected_chat().as_ref()?)
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

    #[inline]
    pub(crate) fn borrow(&self) -> tokio::sync::RwLockReadGuard<'_, UiDomain> {
        RUNTIME
            .get()
            .expect("could not use the tokio runtime in ui domain")
            .block_on(async { self.read().await })
    }

    #[inline] // PERF: this has gotten pretty big with the deadlock detection
    pub(crate) fn borrow_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, UiDomain> {
        let rt = RUNTIME
            .get()
            .expect("could not use the tokio runtime in ui domain");
        rt.block_on(async {
            let mut guard = 0;
            const MAX_TRIES: u32 = 24;
            loop {
                match tokio::time::timeout(tokio::time::Duration::from_millis(100), async {
                    self.write().await
                })
                .await
                {
                    Ok(lock) => return lock,
                    Err(timeout) => {
                        log::warn!("Could not acquire mutable lock of ui domain ({timeout}), trying again... ({guard} / {MAX_TRIES})");
                        guard +=1;
                        if guard > MAX_TRIES {
                            panic!("Waited way too long for the mutable lock on the ui domain.")
                        }
                        continue;
                    }
                }
            }
        })
    }
}

impl Deref for UiDomainSync {
    type Target = Rc<RwLock<UiDomain>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
