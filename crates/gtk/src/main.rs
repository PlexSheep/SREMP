use std::sync::OnceLock;

use async_channel::{Receiver, Sender};
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{Application, glib};

use sremp_client::domain::{UiCommand, UiEvent};

use crate::actions::register_actions;
use crate::domain::UiDomain;
use crate::gui::start_application;

mod actions;
mod domain;
mod gui;
mod jobs;

pub(crate) const GUI_SPACING_MID: i32 = 8;
pub(crate) const GUI_SPACING_LARGE: i32 = 12;
pub(crate) const GUI_SPACING_XLARGE: i32 = 16;
pub(crate) const GUI_SPACING_XXLARGE: i32 = 24;
pub(crate) const GUI_SPACING_XXXLARGE: i32 = 32;
pub const APP_ID: &str = "de.cscherr.sremp.gtk";

pub(crate) static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

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

    start_gui(ui_command_tx, ui_event_rx, rt)
}

fn start_gui(
    command_tx: Sender<UiCommand>,
    event_rx: Receiver<UiEvent>,
    rt: tokio::runtime::Runtime,
) -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    // NOTE: We want to be able to run the application in two separate processes, so that we can
    // test if they can really talk to eachother with networking. #19
    app.set_flags(ApplicationFlags::NON_UNIQUE);

    let _ = RUNTIME.get_or_init(|| rt); // store the runtime just in case it might be needed later

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
