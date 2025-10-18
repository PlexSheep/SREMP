use gtk::prelude::*;
use sremp_client::domain::known_identities::KnownIdentities;
use sremp_core::{chat::Chat, trace_current_function};

use crate::{domain::UiDomainSync, gui::widget_detailbar};

mod bubble;
use bubble::*;
mod input;
use input::*;

#[derive(Debug)]
pub(crate) struct ChatView {
    pub(crate) widget: gtk::Box,
    list: gtk::ListBox,
    detailbar: gtk::Frame,
    scroller: gtk::ScrolledWindow,
    chat: Option<Chat>,
    contacts: KnownIdentities,
    state: UiDomainSync,
}

impl ChatView {
    pub(crate) fn new(contacts: KnownIdentities, chat: Option<Chat>, state: UiDomainSync) -> Self {
        let mut this = Self {
            widget: gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .build(),
            list: Default::default(),
            scroller: Default::default(),
            detailbar: Default::default(),
            chat,
            contacts,
            state,
        };
        this.regenerate();
        this
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.chat = Default::default();
        self.regenerate();
    }

    #[inline]
    pub(crate) fn set_contacts(&mut self, contacts: KnownIdentities) {
        self.contacts = contacts;
        self.regenerate();
    }

    #[inline]
    pub(crate) fn set_chat(&mut self, chat: Option<Chat>) {
        trace_current_function!();
        self.chat = chat;
        self.regenerate();
    }

    fn regenerate(&mut self) {
        trace_current_function!();
        log::trace!("chat of chat view is: {:#?}", self.chat);
        self.list = gtk::ListBox::builder()
            .vexpand(true)
            .selection_mode(gtk::SelectionMode::None)
            .show_separators(false)
            .build();

        match &self.chat {
            Some(c) => {
                log::trace!("Chat is some");
                self.detailbar = widget_detailbar("Chat");

                if c.messages().is_empty() {
                    self.detailbar = widget_detailbar("Chat is empty");
                } else {
                    for message in c.messages() {
                        let contact = &self.contacts[&message.meta().author_id];
                        let message_bubble = MessageBubble::from(message);
                        self.list.append(&message_bubble.widget(contact));
                    }
                }
            }
            None => {
                log::trace!("Chat is none");
                self.detailbar = widget_detailbar("No chat selected");
            }
        }

        self.scroller = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_height(400)
            .min_content_width(400)
            .child(&self.list)
            .build();

        while let Some(child) = self.widget.first_child() {
            self.widget.remove(&child);
        }

        // TODO: scroll to the bottom

        self.widget.append(&self.detailbar);
        self.widget.append(&self.scroller);
        self.widget.append(&widget_input_area(self.state.clone()));
    }
}
