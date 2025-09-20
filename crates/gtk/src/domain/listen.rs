use std::net::SocketAddr;

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
    pub(crate) fn initiate_connection(&self, local_address: SocketAddr) {
        self.send_cmd(UiCommand::StartListener(local_address))
    }
    pub(crate) fn fmt_listen_status(&self) -> String {
        todo!()
    }
}
