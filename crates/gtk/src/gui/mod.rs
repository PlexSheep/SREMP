use std::fmt::Display;

use gtk::prelude::*;
use sremp_client::domain::chats::Chats;

use crate::domain::UiDomainSync;
use crate::{GUI_SPACING_LARGE, GUI_SPACING_MID, GUI_SPACING_XXLARGE};

pub(crate) mod chat;
pub(crate) mod chats;
pub(crate) mod connect;
pub(crate) mod identity;
pub(crate) mod tofu;
pub(crate) mod topbar;

use chat::*;
use chats::*;
use topbar::*;

pub(crate) fn start_application(app: &gtk::Application, state: UiDomainSync) {
    let w_window_content = gtk::Box::builder()
        .overflow(gtk::Overflow::Hidden)
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let w_chat_list = ChatList::new(app, Default::default(), Chats::default(), state.clone());
    w_window_content.append(&w_chat_list.widget);
    state
        .borrow_mut()
        .tracked_widgets
        .set_chat_list(Some(w_chat_list));

    let w_chat_view = ChatView::new(Default::default(), None, state.clone());
    w_window_content.append(&w_chat_view.widget);
    state
        .borrow_mut()
        .tracked_widgets
        .set_chat_view(Some(w_chat_view));

    let w_global_frame = gtk::Frame::builder()
        .child(&w_window_content)
        .margin_top(GUI_SPACING_XXLARGE)
        .margin_bottom(GUI_SPACING_XXLARGE)
        .margin_start(GUI_SPACING_XXLARGE)
        .margin_end(GUI_SPACING_XXLARGE)
        .build();

    // Create a window and set the title
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title(env!("CARGO_BIN_NAME").to_uppercase().replace("-", " "))
        .default_width(600)
        .default_height(900)
        .child(&w_global_frame)
        .build();

    window.set_titlebar(Some(&widget_topbar(app, state.clone())));

    window.present();
}

#[inline]
pub(crate) fn label(content: impl Display) -> gtk::Label {
    gtk::Label::new(Some(&content.to_string()))
}

fn widget_detailbar(content: impl Display) -> gtk::Frame {
    let lbl = label(content);
    lbl.set_margin_top(GUI_SPACING_LARGE);
    lbl.set_margin_bottom(GUI_SPACING_LARGE);
    lbl.set_margin_start(GUI_SPACING_MID);
    lbl.set_margin_end(GUI_SPACING_MID);

    gtk::Frame::builder()
        .margin_top(GUI_SPACING_XXLARGE)
        .margin_bottom(GUI_SPACING_XXLARGE)
        .margin_start(GUI_SPACING_LARGE)
        .margin_end(GUI_SPACING_LARGE)
        .child(&lbl)
        .build()
}
