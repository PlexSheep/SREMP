use gtk::prelude::{BoxExt, DialogExt, GtkWindowExt, WidgetExt};
use sremp_client::domain::{UiCommand, known_identities::SharedContact};
use sremp_core::identity::Trust;

use crate::{domain::UiDomainSync, gui::label};

pub(crate) fn show_tofu_dialog(
    state: UiDomainSync,
    contact: SharedContact,
    socket: std::net::SocketAddr,
) {
    let dialog = gtk::Dialog::builder()
        .title("Trust this identity?")
        .modal(true)
        .build();

    let content_area = dialog.content_area();
    content_area.set_spacing(12);
    content_area.set_margin_top(12);
    content_area.set_margin_bottom(12);
    content_area.set_margin_start(12);
    content_area.set_margin_end(12);

    let info_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(8)
        .build();

    info_box.append(&gtk::Label::new(Some("A peer has connected to you:")));
    info_box.append(&label(format!("Identity: {}", contact.id())));
    info_box.append(&label(format!("Username: {}", contact.username())));
    info_box.append(&label(format!("Created: {}", contact.created())));
    info_box.append(&label(format!("Network Address: {socket}")));

    let question = gtk::Label::new(Some("\nDo you trust this identity?"));
    question.add_css_class("bold");
    info_box.append(&question);

    content_area.append(&info_box);

    dialog.add_button("Reject", gtk::ResponseType::Reject);
    dialog.add_button("Trust", gtk::ResponseType::Accept);

    let contact_id = contact.id();

    dialog.connect_response(move |dialog, response| {
        match response {
            gtk::ResponseType::Accept => {
                state
                    .borrow()
                    .send_cmd(UiCommand::TrustContact(contact_id.clone(), Trust::Trusted));
                state
                    .borrow()
                    .send_cmd(UiCommand::StartChat(contact_id.clone()));

                // Open the new chat in the ui and hope no race condition happens
                state
                    .borrow_mut()
                    .set_selected_chat(Some(contact_id.clone()));
            }
            gtk::ResponseType::Reject => {
                // User chose to reject - send command to block and disconnect
                state
                    .borrow()
                    .send_cmd(UiCommand::TrustContact(contact_id.clone(), Trust::Rejected));
                state.borrow().send_cmd(UiCommand::Disconnect(socket));
            }
            gtk::ResponseType::DeleteEvent => {
                log::debug!("If you only close this dialog and not choose trust or reject, tiny kitties might die.")
            }
            other => log::warn!("Undefined dialog action: {other:?}"),
        }
        dialog.close();
    });

    // NOTE: Use present(), not run()
    // present() returns immediately and doesn't block
    dialog.present();
}
