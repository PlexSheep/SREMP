use gtk::prelude::*;

use crate::GUI_SPACING_MID;
use crate::domain::UiDomainSync;
use crate::gui::label;

mod bubble;
use bubble::*;
mod input;
use input::*;

pub(crate) fn widget_viewport_chat(
    app: &gtk::Application,
    state: UiDomainSync,
) -> impl IsA<gtk::Widget> {
    let vp_chat = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // Create a `ListBox` and add labels with integers from 0 to 100
    let w_list_box = gtk::ListBox::builder()
        .vexpand(true)
        .selection_mode(gtk::SelectionMode::None)
        .show_separators(false)
        .build();

    match state.borrow().current_chat() {
        Some(c) => {
            for message in c.messages() {
                let message_bubble = MessageBubble::from(message);
                w_list_box.append(&message_bubble.widget(app, state.clone()));
            }
        }
        None => {
            w_list_box.append(&label("No chat selected"));
        }
    }

    let w_chat_interface = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(400)
        .min_content_width(400)
        .child(&w_list_box)
        .build();

    // TODO: scroll to the bottom

    vp_chat.append(&w_chat_interface);
    vp_chat.append(&widget_input_area(app, state.clone()));

    vp_chat
}
