use gtk::prelude::*;
use sremp_client::domain::known_identities::{KnownIdentities, SharedContact};
use sremp_core::{chat::Chat, current_function, trace_current_function};

use crate::gui::{label, widget_detailbar};

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
}

impl ChatView {
    pub(crate) fn new(contacts: KnownIdentities, chat: Option<Chat>) -> Self {
        let mut this = Self {
            widget: Default::default(),
            list: Default::default(),
            scroller: Default::default(),
            detailbar: Default::default(),
            chat,
            contacts,
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

                for message in c.messages() {
                    let contact = &self.contacts[&message.meta().author_id];
                    let message_bubble = MessageBubble::from(message);
                    self.list.append(&message_bubble.widget(contact));
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

        self.widget = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        // TODO: scroll to the bottom

        self.widget.append(&self.detailbar);
        self.widget.append(&self.scroller);
        self.widget.append(&widget_input_area());
    }
}
