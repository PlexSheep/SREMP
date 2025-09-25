// this code is really delicate, much care needs to be
// taken to avoid deadlocks in functions called from this one
#![deny(clippy::await_holding_refcell_ref)]
#![deny(clippy::await_holding_lock)]

use log::trace;
use sremp_client::domain::UiEvent;
use sremp_core::current_function;

use crate::{
    domain::{UiDomain, UiDomainSync},
    gui::identity::show_identity_created_success,
};

use gtk::glib;

pub(super) fn start_jobs(state: UiDomainSync) {
    glib::spawn_future_local(event_processor(state));
}

async fn event_processor(state: UiDomainSync) {
    loop {
        {
            if let Ok(event) = state.borrow().event_channel.try_recv() {
                log::info!("Processing network event: {event}");

                match event {
                    UiEvent::ListenerStarted(_addr) => {
                        update_listener_label(&state.borrow());
                    }
                    UiEvent::ListenerStopped => {
                        update_listener_label(&state.borrow());
                    }
                    UiEvent::IdentitySet(iden) => {
                        log::trace!("borrowing mutable ui domain state");
                        // BUG: Deadlock here?
                        state.borrow_mut().apply_user_identity(iden.clone());
                    }
                    other => {
                        log::warn!("Received unimplemented Ui event: {other}")
                    }
                }
            }
        }

        glib::timeout_future(std::time::Duration::from_millis(50)).await;
    }
}

pub(crate) fn update_listener_label(state: &std::sync::RwLockReadGuard<'_, UiDomain>) {
    log::trace!("{}", current_function!());
    let new_text = state.fmt_listen_status();
    state
        .tracked_widgets
        .lbl_listener_status()
        .expect("menu listen status label does not exist")
        .set_text(&new_text);
}
