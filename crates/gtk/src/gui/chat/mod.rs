use gtk::prelude::*;
use sremp_client::domain::known_identities::{KnownIdentities, SharedContact};
use sremp_core::chat::Chat;

use crate::{domain::UiDomainSync, gui::label};

mod bubble;
use bubble::*;
mod input;
use input::*;

#[derive(Debug)]
pub(crate) struct ChatView {
    pub(crate) widget: gtk::Box,
    list: gtk::ListBox,
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
        self.chat = chat;
        self.regenerate();
    }

    fn regenerate(&mut self) {
        self.list = gtk::ListBox::builder()
            .vexpand(true)
            .selection_mode(gtk::SelectionMode::None)
            .show_separators(false)
            .build();

        match &self.chat {
            Some(c) => {
                for message in c.messages() {
                    let contact = &self.contacts[&message.meta().author_id];
                    let message_bubble = MessageBubble::from(message);
                    self.list.append(&message_bubble.widget(contact));
                }
            }
            None => {
                self.list.append(&label("\n\n\nNo chat selected\n\n\n"));
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

        self.widget.append(&self.scroller);
        self.widget.append(&widget_input_area());
    }
}
