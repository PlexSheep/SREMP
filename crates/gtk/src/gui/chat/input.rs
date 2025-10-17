use crate::GUI_SPACING_MID;
use crate::domain::UiDomainSync;

use gtk::prelude::*;

pub(super) fn widget_input_area() -> impl IsA<gtk::Widget> {
    let w_frame = gtk::Frame::builder()
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(GUI_SPACING_MID)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_frame_input = gtk::Frame::builder()
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_input = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::Word)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .wrap_mode(gtk::WrapMode::Word)
        .build();
    w_input.buffer().set_enable_undo(true);

    let w_input_scroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(40)
        .min_content_width(200)
        .propagate_natural_height(true)
        .hexpand(true)
        .child(&w_input)
        .build();

    let w_emoji_chooser = gtk::EmojiChooser::new();

    w_frame_input.set_child(Some(&w_input_scroll));

    let w_box_buttons = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(GUI_SPACING_MID)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_btn_send = gtk::Button::builder()
        .label("Send")
        .valign(gtk::Align::End)
        .build();

    let w_btn_emoji = gtk::Button::builder()
        .label("Emoji")
        .valign(gtk::Align::End)
        .build();

    w_emoji_chooser.set_parent(&w_btn_emoji);

    w_box_buttons.append(&w_btn_emoji);
    w_box_buttons.append(&w_btn_send);

    let tb = w_input.buffer();
    w_btn_send.connect_clicked(move |_| {
        let text = tb.text(&tb.start_iter(), &tb.end_iter(), false);
        if !text.trim().is_empty() {
            // let msg = Message::new(text, Utc::now(), chat.contact().identity.public_key);
            log::warn!("Sending messages is not yet implemented");
            tb.set_text("");
        }
    });

    let tb = w_input.buffer();
    w_emoji_chooser.connect_emoji_picked(move |emoji_chooser, emoji| {
        tb.insert_at_cursor(emoji);
        emoji_chooser.popdown();
    });

    w_btn_emoji.connect_clicked(move |_| {
        w_emoji_chooser.popup();
    });

    w_box.append(&w_frame_input);
    w_box.append(&w_box_buttons);
    w_frame.set_child(Some(&w_box));

    w_frame
}
