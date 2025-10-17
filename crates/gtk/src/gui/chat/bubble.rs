use std::ops::Deref;

use gtk::prelude::*;
use sremp_client::domain::known_identities::SharedContact;
use sremp_core::chat::messages::SharedMessage;

use crate::GUI_SPACING_LARGE;
use crate::GUI_SPACING_MID;
use crate::GUI_SPACING_XLARGE;
use crate::GUI_SPACING_XXXLARGE;
use crate::gui::label;

#[derive(Debug, Clone)]
pub(super) struct MessageBubble {
    inner: SharedMessage,
}

impl MessageBubble {
    pub(super) fn widget(&self, author: &SharedContact) -> impl IsA<gtk::Widget> {
        let w_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let w_meta_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let w_lbl_author = label(author.username());
        let w_lbl_time = label(self.meta().time_received);
        w_lbl_time.set_halign(gtk::Align::Start);
        w_lbl_author.set_halign(gtk::Align::Start);
        w_lbl_author.set_margin_end(GUI_SPACING_XLARGE);

        w_meta_box.append(&w_lbl_author);
        w_meta_box.append(&w_lbl_time);

        w_meta_box.set_margin_top(GUI_SPACING_MID);
        w_meta_box.set_margin_bottom(GUI_SPACING_MID);
        w_meta_box.set_margin_start(GUI_SPACING_LARGE);
        w_meta_box.set_margin_end(GUI_SPACING_LARGE);

        let w_content = self.widget_content();
        w_content.set_margin_top(GUI_SPACING_XXXLARGE);
        w_content.set_halign(gtk::Align::Start);
        w_content.set_margin_top(GUI_SPACING_MID);
        w_content.set_margin_bottom(GUI_SPACING_MID);
        w_content.set_margin_start(GUI_SPACING_LARGE);
        w_content.set_margin_end(GUI_SPACING_LARGE);

        w_box.append(&w_meta_box);
        w_box.append(&w_content);

        gtk::Frame::builder()
            .child(&w_box)
            .margin_top(GUI_SPACING_MID)
            .margin_bottom(GUI_SPACING_MID)
            .margin_start(16)
            .margin_end(16)
            .build()
    }

    #[inline(always)]
    fn widget_content(&self) -> impl IsA<gtk::Widget> {
        label(&self.inner.text)
    }
}

impl Deref for MessageBubble {
    type Target = SharedMessage;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<SharedMessage> for MessageBubble {
    fn from(value: SharedMessage) -> Self {
        MessageBubble { inner: value }
    }
}

impl From<&SharedMessage> for MessageBubble {
    fn from(value: &SharedMessage) -> Self {
        MessageBubble {
            inner: value.clone(),
        }
    }
}
