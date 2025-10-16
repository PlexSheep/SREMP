use std::sync::Arc;

use gtk::prelude::*;
use sremp_core::identity::{ContactIdentity, UserIdentity, format_key};

use crate::{GUI_SPACING_MID, domain::UiDomainSync, gui::label};

/// Creates and shows a dialog for creating a new user identity
pub(crate) fn dialog_create_identity(app: &gtk::Application, state: UiDomainSync) {
    if state.borrow().user_identity().is_some() {
        // TODO: don't allow creating a new identity when one already exists #8
        log::warn!("Creating a new identity even if you already have one");
    }

    let win_dialog = gtk::Window::builder()
        .modal(true)
        .default_width(400)
        .default_height(200)
        .resizable(false)
        .title("Create Your Identity")
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

    // Description text
    let w_description = label(
        "Choose a username for your SREMP identity.\n\
        A cryptographic keypair will be generated for you.\n\
        Your username should have between 1 and 40 characters.",
    );
    w_description.set_halign(gtk::Align::Start);
    w_description.set_margin_bottom(GUI_SPACING_MID);

    let w_grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(12)
        .build();

    let w_username_entry = gtk::Entry::builder()
        .placeholder_text("Enter username (1-40 characters)")
        .hexpand(true)
        .max_length(40)
        .build();

    let w_box_btn = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .halign(gtk::Align::End)
        .build();

    let w_btn_cancel = gtk::Button::builder().label("Cancel").build();
    let w_btn_create = gtk::Button::builder().label("Create Identity").build();
    w_btn_create.add_css_class("suggested-action");

    w_box_btn.append(&w_btn_cancel);
    w_box_btn.append(&w_btn_create);

    w_grid.attach(&label("Username"), 0, 0, 1, 1);
    w_grid.attach(&w_username_entry, 1, 0, 1, 1);

    // Error label (initially hidden)
    let w_error =
        label("Username must be 1-40 characters and contain only letters, numbers, - and _");
    w_error.set_visible(false);
    w_error.add_css_class("error");
    w_grid.attach(&w_error, 0, 1, 2, 1);

    w_box.append(&w_description);
    w_box.append(&w_grid);
    w_box.append(&w_box_btn);

    win_dialog.set_child(Some(&w_box));

    // Event handlers
    let win_dialog_clone = win_dialog.clone();
    w_btn_cancel.connect_clicked(move |_| {
        win_dialog_clone.close();
    });

    let win_dialog_clone = win_dialog.clone();
    let w_error_clone = w_error.clone();
    let state_clone = state.clone();

    let w_username_entry_c = w_username_entry.clone();
    w_btn_create.connect_clicked(move |_| {
        let username = w_username_entry_c.text().to_string().trim().to_string();

        let handle_error = |reason: String| {
            w_error_clone.set_text(&reason);
            w_error_clone.set_visible(true);
        };

        // Validate username
        if username.is_empty() {
            handle_error("Username cannot be empty".to_string());
            return;
        }

        if username.len() > 40 {
            handle_error("Username must be 40 characters or less".to_string());
            return;
        }

        // Validate username characters (basic validation)
        if !username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            handle_error(
                "Username can only contain letters, numbers, hyphens, and underscores".to_string(),
            );
            return;
        }

        // Try to create the identity
        match UserIdentity::create(&username) {
            Ok(user_identity) => {
                {
                    let user_identity = Arc::new(user_identity);
                    state_clone
                        .borrow_mut()
                        .set_user_identity(Some(user_identity.clone()));
                }

                win_dialog_clone.close();
            }
            Err(e) => {
                handle_error(format!("Failed to create identity: {e}"));
            }
        }
    });

    // Allow Enter key to create identity
    w_username_entry.connect_activate(move |_| {
        w_btn_create.emit_clicked();
    });

    win_dialog.present();
}

/// Shows a success dialog after identity creation
pub(crate) fn show_identity_created_success(user_identity: Arc<UserIdentity>) {
    let w_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(GUI_SPACING_MID)
        .margin_top(GUI_SPACING_MID)
        .margin_bottom(GUI_SPACING_MID)
        .margin_start(GUI_SPACING_MID)
        .margin_end(GUI_SPACING_MID)
        .build();

    let w_lbl = label(format!(
        "Your SREMP identity has been created.\n\nUsername: {}\nPublic Key: {}\n\nYou can now start messaging!",
        user_identity.identity.username(),
        sremp_core::identity::format_key(&user_identity.identity.identity_key())
    ));

    let w_btn_ok = gtk::Button::builder().label("OK").build();

    w_box.append(&w_lbl);
    w_box.append(&w_btn_ok);

    let win_dialog = gtk::Window::builder()
        .modal(true)
        .default_width(400)
        .default_height(200)
        .resizable(false)
        .title("Identity Created Successfully!")
        .child(&w_box)
        .build();

    let win_dialog_c = win_dialog.clone();
    w_btn_ok.connect_clicked(move |_| {
        win_dialog_c.close();
    });

    win_dialog.show();
    log::debug!("Showing identity created success window");
}

pub(crate) fn show_user_identity(app: &gtk::Application, user: UserIdentity) {
    let win_dialog = gtk::Window::builder()
        .modal(true)
        .default_width(500)
        .default_height(650)
        .resizable(true)
        .title("User Identity")
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

    w_box.append(&label(format!("Username: {}", user.identity.username())));
    w_box.append(&label(format!(
        "Public Key: {}",
        format_key(&user.identity.identity_key())
    )));
    w_box.append(&label(format!("Created: {}", user.identity.created())));

    win_dialog.set_child(Some(&w_box));

    win_dialog.present();
}

pub(crate) fn show_contact_identity(app: &gtk::Application, contact: &ContactIdentity) {
    let win_dialog = gtk::Window::builder()
        .modal(true)
        .default_width(500)
        .default_height(650)
        .resizable(true)
        .title("Contact Identity")
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

    w_box.append(&label(format!("Username: {}", contact.identity.username())));
    w_box.append(&label(format!(
        "Public Key: {}",
        format_key(&contact.identity.identity_key())
    )));
    w_box.append(&label(format!("Trust: {}", contact.trust)));
    w_box.append(&label(format!("First Seen: {}", contact.first_seen)));
    w_box.append(&label(format!("Last Seen: {}", contact.last_seen)));

    win_dialog.set_child(Some(&w_box));

    win_dialog.present();
}
