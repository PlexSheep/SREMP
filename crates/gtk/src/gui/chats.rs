use std::fmt::Display;

use gtk::prelude::*;
use sremp_client::domain::UiCommand;
use sremp_client::domain::chats::Chats;
use sremp_client::domain::known_identities::{KnownIdentities, SharedContact};
use sremp_core::chat::Chat;
use sremp_core::identity::ContactId;

use crate::domain::UiDomainSync;
use crate::gui::{label, widget_detailbar};
use crate::{GUI_SPACING_LARGE, GUI_SPACING_MID};

#[derive(Debug)]
pub(crate) struct ChatList {
    pub(crate) widget: gtk::Frame,
    chats: Chats,
    list: gtk::ListBox,
    inner_box: gtk::Box,
    detailbar: gtk::Frame,
    selected: Option<ContactId>,
    app: gtk::Application,
    contacts: KnownIdentities,
    state: UiDomainSync,
}

impl ChatList {
    pub(crate) fn new(
        app: &gtk::Application,
        contacts: KnownIdentities,
        chats: Chats,
        state: UiDomainSync,
    ) -> Self {
        let widget = gtk::Frame::builder()
            .margin_top(GUI_SPACING_LARGE)
            .margin_bottom(GUI_SPACING_LARGE)
            .margin_start(GUI_SPACING_LARGE)
            .margin_end(GUI_SPACING_LARGE)
            .build();

        let mut this = Self {
            widget,
            list: Default::default(),
            detailbar: Default::default(),
            inner_box: Default::default(),
            selected: None,
            app: app.clone(),
            contacts,
            chats,
            state,
        };

        this.regenerate();

        this
    }

    // PERF: we probably should call this too often
    fn regenerate(&mut self) {
        self.list = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .build();

        if let Some(chat_cid) = &self.selected {
            self.detailbar = widget_detailbar(format!("{} Chats", self.chats().len()));

            for (cid, chat) in self.chats.iter() {
                let contact = self.contacts[cid].clone();
                let w_chat_card = widget_chat_card(&self.app, self.state.clone(), contact, chat);
                self.list.append(&w_chat_card);
            }
        } else {
            let w_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .margin_top(GUI_SPACING_LARGE)
                .margin_bottom(GUI_SPACING_LARGE)
                .margin_start(GUI_SPACING_LARGE)
                .margin_end(GUI_SPACING_LARGE)
                .build();

            self.detailbar = widget_detailbar("No chats yet");

            self.list.append(
                &gtk::Frame::builder()
                    .margin_top(GUI_SPACING_MID)
                    .margin_bottom(GUI_SPACING_MID)
                    .margin_start(GUI_SPACING_MID)
                    .margin_end(GUI_SPACING_MID)
                    .child(&w_box)
                    .build(),
            );
        }
        self.inner_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        self.inner_box.append(&self.detailbar);
        self.inner_box.append(&self.list);

        self.widget.set_child(Some(&self.inner_box));
    }

    #[inline]
    pub(crate) fn update_chat(&mut self, cid: ContactId, chat: &Chat) {
        self.chats.insert(cid, chat.clone());
        self.regenerate(); // PERF: might become high load if this is called in a chain
    }

    #[inline]
    pub(crate) fn replace_chats(&mut self, chats: Chats) {
        self.chats = chats;
        self.regenerate();
    }

    #[inline(always)]
    pub(crate) fn chats(&self) -> &Chats {
        &self.chats
    }

    #[inline(always)]
    pub(crate) fn chats_mut(&mut self) -> &mut Chats {
        &mut self.chats
    }

    #[inline(always)]
    pub(crate) fn selected_chat(&self) -> Option<ContactId> {
        self.selected.clone()
    }

    #[inline(always)]
    pub(crate) fn set_selected_chat(&mut self, chat: Option<ContactId>) {
        self.selected = chat;
        self.regenerate();
    }

    #[inline(always)]
    pub(crate) fn contacts(&self) -> &KnownIdentities {
        &self.contacts
    }

    #[inline(always)]
    pub(crate) fn set_contacts(&mut self, contacts: KnownIdentities) {
        self.contacts = contacts;
    }

    #[inline(always)]
    pub(crate) fn contacts_mut(&mut self) -> &mut KnownIdentities {
        &mut self.contacts
    }
}

fn widget_chat_card(
    _app: &gtk::Application,
    state: UiDomainSync,
    contact: SharedContact,
    chat: &Chat,
) -> impl IsA<gtk::Widget> {
    let w_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(GUI_SPACING_LARGE)
        .margin_bottom(GUI_SPACING_LARGE)
        .margin_start(GUI_SPACING_LARGE)
        .margin_end(GUI_SPACING_LARGE)
        .build();

    w_box.append(&label(contact.username()));

    let on_click = gtk::GestureClick::new();
    on_click.connect_released(move |_gesture, _press, _x, _y| {
        log::debug!("Chat card of {} clicked!", contact.id());
        state.borrow().send_cmd(UiCommand::SelectChat(contact.id()));
    });

    let frame = gtk::Frame::builder()
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .child(&w_box)
        .build();

    frame.add_controller(on_click);

    frame
}
