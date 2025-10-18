use std::{net::SocketAddr, sync::Arc};

use sremp_core::{
    chat::messages::{Message, SharedMessage},
    current_function,
    domain::{NetworkCommand, NetworkEvent},
    error::{CoreError, CoreResult},
    identity::{ContactId, ContactIdentity, UserIdentity},
    trace_current_function,
};

use crate::{
    domain::{ClientDomain, UiCommand, UiEvent, known_identities::SharedContact},
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
            UiCommand::StartChat(cid) => {
                self.chats.entry(cid).or_default();
                self.send_ui_evt(UiEvent::LoadedChats(self.chats.clone()))
                    .await;
                Ok(())
            }
            UiCommand::SelectChat(cid) => {
                self.send_ui_evt(UiEvent::OpenChat(cid)).await;
                Ok(())
            }
            UiCommand::TrustContact(cid, trust) => {
                if let Some(contact) = self.known_identities.get(&cid) {
                    // replace the contact with the changed one
                    let mut nc: ContactIdentity = (**contact).clone();
                    nc.trust = trust;
                    self.known_identities.insert(cid, Arc::new(nc));
                } else {
                    log::warn!("Could not set trust for {cid}, because this is not a known contact")
                }
                Ok(())
            }
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
            NetworkEvent::ConnectionEstablished(remote, iden) => {
                self.known_identities.create_or_update(&iden)?;
                self.send_ui_evt(UiEvent::SetKnownIdentities(self.known_identities.clone()))
                    .await;
                self.send_ui_evt(UiEvent::ConnectionEstablished(remote, iden.id()))
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

    pub(crate) async fn set_identity(
        &mut self,
        iden: Option<Arc<UserIdentity>>,
    ) -> ClientResult<()> {
        log::trace!("{}", current_function!());
        self.user_identity = iden.clone();

        // we must have the local user identity in the known identities.
        // - We display them for the messages the user wrote
        // - We need them if the user wants to send messages to themselves as a self-contact
        if let Some(iden) = iden.clone() {
            let self_contact: SharedContact = ContactIdentity::from_user_identity(&iden).into();
            self.known_identities.create_or_update(&self_contact)?;
            self.send_ui_evt(UiEvent::SetKnownIdentities(self.known_identities.clone()))
                .await;
        }

        self.net_command_channel()
            .send(NetworkCommand::SetIdentity(iden))
            .await
            .map_err(CoreError::from)?;
        self.ui_event_channel()
            .send(UiEvent::IdentitySet(self.user_identity.clone()))
            .await?;
        Ok(())
    }

    pub(crate) async fn send_message(&self, to: ContactId, msg: SharedMessage) -> ClientResult<()> {
        trace_current_function!();
        log::trace!("formatting message for wire");
        let data: Arc<Vec<u8>> = Arc::new(msg.to_wire());
        log::trace!("getting open connection");
        // BUG: open_connections is never filled, so this always errors
        let remote = match self.open_connections.get(&to) {
            Some(r) => {
                log::trace!("open connection exists! {r}");
                r
            }
            None => {
                log::trace!("no open connection exists!");
                return Err(ClientError::NoConnection(to.into()));
            }
        };
        log::trace!("sending commands and events");
        self.send_net_cmd(NetworkCommand::SendMessage(*remote, to.clone(), data))
            .await;
        self.send_ui_evt(UiEvent::MessageSent(*remote, to, msg))
            .await;
        Ok(())
    }

    pub(crate) async fn incoming_message(
        &mut self,
        remote: SocketAddr,
        id: ContactId,
        data: Arc<Vec<u8>>,
    ) -> CoreResult<()> {
        let msg: Message = rmp_serde::from_slice(&data)?;

        self.chats.add_message(id.clone(), msg.into());

        self.send_ui_evt(UiEvent::LoadedChats(self.chats.clone()))
            .await;
        Ok(())
    }
}
