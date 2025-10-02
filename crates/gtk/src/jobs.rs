// this code is really delicate, much care needs to be
// taken to avoid deadlocks in functions called from this one
#![deny(clippy::await_holding_refcell_ref)]
#![deny(clippy::await_holding_lock)]

use log::trace;
use sremp_client::domain::UiEvent;
use sremp_core::current_function;

use crate::{
    domain::{UiDomain, UiDomainSync, listen::ListenerStatus},
    gui::identity::show_identity_created_success,
};

use gtk::glib;

pub(super) fn start_jobs(state: UiDomainSync) {
    glib::spawn_future_local(event_processor(state));
}

async fn event_processor(state: UiDomainSync) {
    loop {
        {
            // WARN: explcit binding and dropping the binding is required here, otherwise the
            // binding is held during the processing of the received event, even though the event
            // is owned and not bound to the held lock on the ui domain.
            // Holding the lock while processing the event may lead to deadlocks.
            let state_b = state.borrow();
            if let Ok(event) = state_b.event_channel.try_recv() {
                drop(state_b);
                log::info!("Processing network event: {event}");

                match event {
                    UiEvent::ListenerStarted(addr) => {
                        state.borrow_mut().listen_status = ListenerStatus::Active(addr);
                        log::trace!(
                            "Listener was started, text should show that is is running on {addr}"
                        );
                        update_listener_label(&state.borrow());
                    }
                    UiEvent::ListenerStopped => {
                        update_listener_label(&state.borrow());
                    }
                    UiEvent::IdentitySet(iden) => {
                        log::trace!("borrowing mutable ui domain state");
                        // NOTE: Deadlock if the lock is still held above
                        state.borrow_mut().apply_user_identity(iden.clone());
                    }
                    other => {
                        log::warn!("Received unimplemented Ui event: {other}")
                    }
                }
            }
        }

        glib::timeout_future(std::time::Duration::from_millis(20)).await;
    }
}

pub(crate) fn update_listener_label(state: &tokio::sync::RwLockReadGuard<'_, UiDomain>) {
    log::trace!("{}", current_function!());
    let new_text = state.fmt_listen_status();
    state
        .tracked_widgets
        .lbl_listener_status()
        .expect("menu listen status label does not exist")
        .set_text(&new_text);
}
