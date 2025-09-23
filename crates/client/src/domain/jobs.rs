use std::{net::SocketAddr, sync::Arc};

use ed25519_dalek::VerifyingKey;
use sremp_core::{
    chat::messages::SharedMessage,
    current_function,
    domain::{NetworkCommand, NetworkEvent},
    error::{CoreError, CoreResult},
    identity::UserIdentity,
};

use crate::{
    domain::{ClientDomain, UiCommand, UiEvent},
    error::{ClientError, ClientResult},
};

impl ClientDomain {
    pub(super) async fn process_ui_command(&mut self, command: UiCommand) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        log::info!("Processing Ui Command: {command}");
        match command {
            UiCommand::StopListener => self.listener_stop().await,
            UiCommand::StartListener(local_addr) => self.listener_start(local_addr).await,
            UiCommand::Connect(remote) => self.connect(remote).await,
            UiCommand::Disconnect(remote) => self.disconnect(remote).await,
            UiCommand::SetIdentity(ident) => self.set_identity(ident).await,
            UiCommand::SendMessage(key, msg) => self.send_message(key, msg).await,
            UiCommand::LoadChat(key) => self.load_chat(key).await,
        }
    }

    pub(super) async fn process_net_event(&mut self, event: NetworkEvent) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        log::info!("Processing Net Event: {event}");
        match event {
            NetworkEvent::ListenerStopped => self.send_ui_evt(UiEvent::ListenerStopped).await,
            NetworkEvent::ListenerFailed(core_error) => todo!(),
            NetworkEvent::ListenerStarted(addr) => {
                self.send_ui_evt(UiEvent::ListenerStarted(addr)).await
            }
            NetworkEvent::ConnectionLost(remote, key) => {
                self.send_ui_evt(UiEvent::ConnectionLost(remote, key)).await
            }
            NetworkEvent::ConnectionFailed(remote, reason) => {
                self.send_ui_evt(UiEvent::ConnectionFailed(remote, reason))
                    .await
            }
            NetworkEvent::ConnectionEstablished(remote, reason) => {
                self.send_ui_evt(UiEvent::ConnectionEstablished(remote, reason))
                    .await
            }
            NetworkEvent::MessageSent(remote, key, data) => {
                todo!()
            }
            NetworkEvent::IncomingMessage(remote, key, data) => {
                self.incoming_message(remote, key, data).await?
            }
            NetworkEvent::ConnectionReset(remote) => {
                self.send_ui_evt(UiEvent::ConnectionReset(remote)).await
            }
        }
        Ok(())
    }

    pub(crate) async fn listener_stop(&self) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        self.net_command_channel()
            .send(NetworkCommand::StopListener)
            .await
            .map_err(CoreError::from)?;
        Ok(())
    }

    pub(crate) async fn listener_start(&self, addr: SocketAddr) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        self.net_command_channel()
            .send(NetworkCommand::StartListener(addr))
            .await
            .map_err(CoreError::from)?;
        Ok(())
    }

    pub(crate) async fn connect(&self, addr: SocketAddr) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        self.net_command_channel()
            .send(NetworkCommand::Connect(addr))
            .await
            .map_err(CoreError::from)?;
        Ok(())
    }

    pub(crate) async fn disconnect(&self, addr: SocketAddr) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        self.net_command_channel()
            .send(NetworkCommand::Disconnect(addr))
            .await
            .map_err(CoreError::from)?;
        Ok(())
    }

    pub(crate) async fn set_identity(&mut self, iden: Option<UserIdentity>) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        self.user_identity = iden.clone();
        self.net_command_channel()
            .send(NetworkCommand::SetIdentity(iden.clone()))
            .await
            .map_err(CoreError::from)?;
        self.ui_event_channel()
            .send(UiEvent::IdentitySet(self.user_identity.clone()))
            .await?;
        Ok(())
    }

    pub(crate) async fn send_message(
        &self,
        to: VerifyingKey,
        msg: SharedMessage,
    ) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        let data: Arc<Vec<u8>> = Arc::new(msg.to_wire());
        let remote = match self.open_connections.get(&to) {
            Some(r) => r,
            None => {
                return Err(ClientError::NoConnection(to));
            }
        };
        self.send_net_cmd(NetworkCommand::SendMessage(*remote, to, data))
            .await;
        self.send_ui_evt(UiEvent::MessageSent(*remote, to, msg))
            .await;
        Ok(())
    }

    pub(crate) async fn load_chat(&self, key: VerifyingKey) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        if self.chats.contains_key(&key) {
            self.send_ui_evt(UiEvent::ChatLoaded(self.chats.get(&key).unwrap().clone()))
                .await;
        } else {
            self.send_ui_evt(UiEvent::ChatNotFound(key)).await;
        }
        Ok(())
    }

    pub(crate) async fn incoming_message(
        &self,
        remote: SocketAddr,
        key: VerifyingKey,
        data: Arc<Vec<u8>>,
    ) -> CoreResult<()> {
        todo!()
    }
}
