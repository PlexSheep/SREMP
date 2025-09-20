use gtk::{gio, prelude::*};

use crate::{actions, domain::UiDomainSync, gui::label};

pub(crate) fn widget_topbar(_app: &gtk::Application, state: UiDomainSync) -> impl IsA<gtk::Widget> {
    let menu: gio::Menu = gio::Menu::new();
    let menu_connection: gio::Menu = gio::Menu::new();
    let menu_settings: gio::Menu = gio::Menu::new();
    let menu_info: gio::Menu = gio::Menu::new();
    let menu_identity: gio::Menu = gio::Menu::new();
    let menu_info_versions: gio::Menu = gio::Menu::new();

    menu_connection.append(
        Some("Connect"),
        Some(actions::ids::A_ID_CONNECTION_CONNECT!(app)),
    );
    menu_connection.append(
        Some("Listen"),
        Some(actions::ids::A_ID_CONNECTION_LISTEN!(app)),
    );
    menu_connection.append(
        Some("Disconnect"),
        Some(actions::ids::A_ID_CONNECTION_DISCONNECT!(app)),
    );

    menu_settings.append(
        Some("Delete everything"),
        Some(actions::ids::A_ID_SETTINGS_DELETE_EVERYTHING!(app)),
    );
    menu_settings.append(
        Some("Delete my Identity"),
        Some(actions::ids::A_ID_SETTINGS_DELETE_IDENTITY!(app)),
    );
    menu_settings.append(
        Some("Delete chats"),
        Some(actions::ids::A_ID_SETTINGS_DELETE_CHATS!(app)),
    );

    menu_info_versions.append(Some(&crate::version()), Some("void"));
    menu_info_versions.append(Some(&sremp_client::version()), Some("void"));
    menu_info_versions.append(Some(&sremp_core::version()), Some("void"));
    menu_info_versions.append(
        Some(&format!(
            "This is free software\nlicensed under {}",
            env!("CARGO_PKG_LICENSE")
        )),
        Some("void"),
    );

    menu_info.append(Some("Show About"), Some(actions::ids::A_ID_INFO!(app)));
    menu_info.append_section(Some("versions"), &menu_info_versions);

    menu_identity.append(
        Some("Create new Identity"),
        Some(actions::ids::A_ID_IDENTITY_CREATE!(app)),
    );
    menu_identity.append(
        Some("Show my Identity"),
        Some(actions::ids::A_ID_IDENTITY_SHOW_USER!(app)), // TODO: open a show window on clicked
    );

    menu.append_submenu(Some("Connection"), &menu_connection);
    menu.append_submenu(Some("Identity"), &menu_identity);
    menu.append_submenu(Some("Settings"), &menu_settings);
    menu.append_submenu(Some("Info"), &menu_info);

    let head_bar = gtk::HeaderBar::new();
    let custom_menu_bar = gtk::PopoverMenuBar::from_model(Some(&menu));

    head_bar.pack_start(&custom_menu_bar);

    let w_lbl_listener_status = label(state.borrow().fmt_listen_status());
    w_lbl_listener_status.add_css_class("dim-label");

    head_bar.pack_start(&w_lbl_listener_status);
    state
        .borrow_mut()
        .tracked_widgets
        .set_lbl_listener_status(Some(w_lbl_listener_status.clone()));

    head_bar
}
