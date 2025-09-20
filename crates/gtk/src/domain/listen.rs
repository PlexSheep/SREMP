use std::{fmt::Display, net::SocketAddr};

use sremp_client::{domain::UiCommand, error::ClientError};

use crate::domain::UiDomain;

#[derive(Debug, Default)]
pub(crate) enum ListenerStatus {
    #[default]
    Stopped,
    Starting,
    Active(SocketAddr),
    Error(ClientError),
}

impl UiDomain {
    #[cold]
    pub(crate) fn initiate_connection(&mut self, local_address: SocketAddr) {
        self.send_cmd(UiCommand::StartListener(local_address));
        self.listen_status = ListenerStatus::Starting;
    }
    pub(crate) fn fmt_listen_status(&self) -> String {
        self.listen_status.to_string()
    }
}

impl Display for ListenerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Stopped => "Listener is not active".to_string(),
                Self::Starting => "Listener is starting".to_string(),
                Self::Active(addr) => format!("Listener is listening on {addr}"),
                Self::Error(err) => format!("Listener failed: {err}"),
            }
        )
    }
}
