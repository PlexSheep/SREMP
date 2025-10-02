use std::net::SocketAddr;

use sremp_client::domain::UiCommand;

use crate::domain::UiDomain;

impl UiDomain {
    pub(crate) fn initiate_connection(&mut self, remote_address: SocketAddr) {
        self.send_cmd(UiCommand::Connect(remote_address));
    }
}
