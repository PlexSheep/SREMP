use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use sremp_client::domain::UiCommand;

use super::ids::*;
use super::macros::simple_action;
use crate::{domain::UiDomainSync, gui::connect::dialog_connect};

use gtk::{Application, prelude::*};

pub(super) fn register_actions(app: &Application, state: UiDomainSync) {
    simple_action!(app, state, _app_c, state_c, A_ID_CONNECTION_LISTEN!(), {
        let addr = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), 0);
        state_c.borrow().send_cmd(UiCommand::StartListener(addr));
    });
    simple_action!(app, state, app_c, state_c, A_ID_CONNECTION_CONNECT!(), {
        dialog_connect(&app_c.clone(), state_c.clone());
    });
    simple_action!(
        app,
        state,
        _app_c,
        state_c,
        A_ID_CONNECTION_DISCONNECT!(),
        {
            state_c.borrow().send_cmd(UiCommand::StopListener);
        }
    );
}
