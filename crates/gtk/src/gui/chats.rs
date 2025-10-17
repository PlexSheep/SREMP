use gtk::prelude::*;
use sremp_client::domain::chats::Chats;
use sremp_core::chat::Chat;
use sremp_core::identity::ContactId;

use crate::domain::UiDomainSync;
use crate::gui::label;
use crate::{GUI_SPACING_LARGE, GUI_SPACING_MID};

#[derive(Debug)]
pub(crate) struct ChatList {
    pub(crate) widget: gtk::Frame,
    chats: Chats,
    list: gtk::ListBox,
    selected: Option<ContactId>,
    state: UiDomainSync,
    app: gtk::Application,
}

impl ChatList {
    pub(crate) fn new(app: &gtk::Application, state: UiDomainSync, chats: Chats) -> Self {
        let widget = gtk::Frame::builder()
            .margin_top(GUI_SPACING_LARGE)
            .margin_bottom(GUI_SPACING_LARGE)
            .margin_start(GUI_SPACING_LARGE)
            .margin_end(GUI_SPACING_LARGE)
            .build();

        let mut this = Self {
            widget,
            list: Default::default(),
            selected: None,
            state,
            app: app.clone(),
            chats,
        };

        this.regenerate();

        this
    }

    // PERF: we probably should call this too often
    fn regenerate(&mut self) {
        self.list = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .build();

        self.widget.set_child(Some(&self.list));

        if self.chats.is_empty() {
            let w_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .margin_top(GUI_SPACING_LARGE)
                .margin_bottom(GUI_SPACING_LARGE)
                .margin_start(GUI_SPACING_LARGE)
                .margin_end(GUI_SPACING_LARGE)
                .build();

            w_box.append(&label("No chats yet"));

            self.list.append(
                &gtk::Frame::builder()
                    .margin_top(GUI_SPACING_MID)
                    .margin_bottom(GUI_SPACING_MID)
                    .margin_start(GUI_SPACING_MID)
                    .margin_end(GUI_SPACING_MID)
                    .child(&w_box)
                    .build(),
            );
        } else {
            for (cid, chat) in self.chats.iter() {
                let w_chat_card = widget_chat_card(&self.app, &self.state, cid, chat);
                self.list.append(&w_chat_card);
            }
        }
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
    pub(crate) fn selected_chat(&self) -> Option<ContactId> {
        self.selected.clone()
    }

    #[inline(always)]
    pub(crate) fn set_selected_chat(&mut self, chat: Option<ContactId>) {
        self.selected = chat;
        self.regenerate();
    }
}

fn widget_chat_card(
    _app: &gtk::Application,
    state: &UiDomainSync,
    cid: &ContactId,
    chat: &Chat,
) -> impl IsA<gtk::Widget> {
    let w_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(GUI_SPACING_LARGE)
        .margin_bottom(GUI_SPACING_LARGE)
        .margin_start(GUI_SPACING_LARGE)
        .margin_end(GUI_SPACING_LARGE)
        .build();

    // BUG: RACE CONDITION
    //let contact = state.borrow().contacts[cid].clone();
    //w_box.append(&label(contact.username()));
    w_box.append(&label("TODO GET USERNAME"));

    gtk::Frame::builder()
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .child(&w_box)
        .build()
}
