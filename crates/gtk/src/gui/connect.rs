use crate::{GUI_SPACING_MID, domain::UiDomainSync, gui::label};

use gtk::prelude::*;

pub(crate) fn dialog_connect(app: &gtk::Application, state: UiDomainSync) {
    let win_dialog = gtk::Window::builder()
        .modal(true)
        .default_width(300)
        .default_height(150)
        .resizable(false)
        .title("Establish a new Connection")
        .build();

    if let Some(window) = app.active_window() {
        win_dialog.set_transient_for(Some(&window));
    }

    let w_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(GUI_SPACING_MID)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(12)
        .build();

    let w_host_entry = gtk::Entry::builder()
        .placeholder_text("192.168.1.19")
        .hexpand(true)
        .build();

    let w_port_entry = gtk::Entry::builder()
        .placeholder_text("51673")
        .text("51673")
        .build();

    let w_box_btn = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .halign(gtk::Align::End)
        .build();

    let w_btn_cancel = gtk::Button::builder().label("Cancel").build();
    let w_btn_accept = gtk::Button::builder().label("Connect").build();
    w_btn_accept.add_css_class("suggested-action");

    w_box_btn.append(&w_btn_cancel);
    w_box_btn.append(&w_btn_accept);

    w_grid.attach(&label("Host"), 0, 0, 1, 1);
    w_grid.attach(&w_host_entry, 1, 0, 1, 1);
    w_grid.attach(&label("Port"), 0, 1, 1, 1);
    w_grid.attach(&w_port_entry, 1, 1, 1, 1);

    let w_error = label("undefined error");
    w_error.set_visible(false);
    w_grid.attach(&w_error, 0, 2, 2, 1);

    w_box.append(&w_grid);
    w_box.append(&w_box_btn);

    win_dialog.set_child(Some(&w_box));

    let win_dialog_clone = win_dialog.clone();
    w_btn_cancel.connect_clicked(move |_| {
        win_dialog_clone.close();
    });

    let win_dialog_clone = win_dialog.clone();
    let w_error_clone = w_error.clone();

    w_btn_accept.connect_clicked(move |_| {
        let raw_host = w_host_entry.text().to_string();
        let raw_port = w_port_entry.text().to_string();

        let handle_error = |reason: String| {
            w_error_clone.set_text(&reason);
            w_error_clone.set_visible(true);
        };

        match format!("{raw_host}:{raw_port}").parse::<std::net::SocketAddr>() {
            Ok(remote) => {
                state.borrow().initiate_connection(remote);
                win_dialog_clone.close();
            }
            Err(e) => handle_error(format!("Could not parse remote address: {e}")),
        }
    });

    win_dialog.present();
}
