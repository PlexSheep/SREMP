use async_channel::{Receiver, Sender};
use gtk::prelude::*;
use gtk::{Application, glib};

use sremp_client::domain::{UiCommand, UiEvent};

use crate::actions::register_actions;
use crate::domain::UiDomain;
use crate::gui::start_application;

pub(crate) const GUI_SPACING_MID: i32 = 8;
pub(crate) const GUI_SPACING_LARGE: i32 = 12;
pub(crate) const GUI_SPACING_XLARGE: i32 = 16;
pub(crate) const GUI_SPACING_XXLARGE: i32 = 24;
pub(crate) const GUI_SPACING_XXXLARGE: i32 = 32;
pub const APP_ID: &str = "de.cscherr.sremp";

mod actions;
mod domain;
mod gui;
mod jobs;

fn main() -> glib::ExitCode {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .parse_default_env()
        .init();
    let mut rt = tokio::runtime::Runtime::new().expect("could not create tokio runtime");

    let (net_command_tx, net_command_rx) = async_channel::unbounded();
    let (net_event_tx, net_event_rx) = async_channel::unbounded();

    let (ui_command_tx, ui_command_rx) = async_channel::unbounded();
    let (ui_event_tx, ui_event_rx) = async_channel::unbounded();

    let net_domain = sremp_core::domain::NetworkDomain::new();
    net_domain
        .start(net_command_rx, net_event_tx, &mut rt)
        .expect("could not start network domain");

    let app_domain = sremp_client::domain::ClientDomain::new();
    app_domain
        .start(
            net_command_tx,
            net_event_rx,
            ui_command_rx,
            ui_event_tx,
            &mut rt,
        )
        .expect("could not start application domain");

    let ret = start_gui(ui_command_tx, ui_event_rx);
    return ret;
}

fn start_gui(command_tx: Sender<UiCommand>, event_rx: Receiver<UiEvent>) -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let domain = UiDomain::new(command_tx.clone(), event_rx.clone()).into_sync();

        register_actions(app, domain.clone());
        start_application(app, domain.clone());

        jobs::start_jobs(domain);
    });

    app.run()
}

pub fn version() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .trim()
        .to_string()
}
