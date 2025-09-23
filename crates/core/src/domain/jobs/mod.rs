use std::{collections::hash_map::Entry, net::SocketAddr};

use async_channel::{Receiver, Sender};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net,
};

use crate::{
    current_function,
    domain::{ConnectionData, NetworkCommand, NetworkDomain, NetworkDomainSync, NetworkEvent},
    error::{CoreError, CoreResult},
    identity::UserIdentity,
    net::connection::Connection,
};

impl NetworkDomain {
    pub(super) async fn process_network_command(
        &mut self,
        command: NetworkCommand,
    ) -> CoreResult<NetworkEvent> {
        log::trace!("{}", current_function!());
        log::info!("Processing Network Command: {command}");
        let event = match command {
            NetworkCommand::Connect(remote) => self.connect_to(remote).await?,
            NetworkCommand::StartListener(listen_addr) => self.listen(listen_addr).await?,
            NetworkCommand::StopListener => {
                if let Some(listener) = self.listener.take() {
                    log::info!("Stopping listener");
                    drop(listener);
                } else {
                    log::warn!("No listener currently exists!")
                }
                NetworkEvent::ListenerStopped
            }
            _ => todo!(),
        };
        log::info!("Event emerged after processing the Network Command: {event}");
        Ok(event)
    }

    async fn init_connection(
        &mut self,
        remote: SocketAddr,
        connection: Connection,
    ) -> CoreResult<NetworkEvent> {
        log::trace!("{}", current_function!());
        log::debug!("Initializing TLS connection for {remote}");
        let remote_identity = connection.peer_identity().await.clone();

        match self.active_connections.entry(remote) {
            // we already have a connection with this socket addr???
            Entry::Occupied(_en) => {
                log::warn!("Duplicated connection, closing second connection...");
                connection.disconnect().await?;
                return Ok(NetworkEvent::ConnectionFailed(
                    remote,
                    "already connected to this peer".to_string(),
                ));
            }
            Entry::Vacant(en) => en.insert(ConnectionData {
                conn: connection,
                iden: remote_identity.clone(),
            }),
        };

        Ok(NetworkEvent::ConnectionEstablished(
            remote,
            remote_identity.public_key,
        ))
    }

    fn identity(&self) -> CoreResult<&UserIdentity> {
        self.user_identity
            .as_ref()
            .ok_or(CoreError::NoUserIdentity)
            .inspect_err(|e| log::error!("Can't connect without identity: {e}"))
    }

    async fn connect_to(&mut self, remote: SocketAddr) -> CoreResult<NetworkEvent> {
        log::trace!("{}", current_function!());
        let user_identity = self.identity()?;
        let connection = Connection::connect_to(remote, user_identity).await?;
        self.init_connection(remote, connection).await
    }

    async fn connect_from(
        &mut self,
        stream: net::TcpStream,
        remote: SocketAddr,
    ) -> CoreResult<NetworkEvent> {
        log::trace!("{}", current_function!());
        let user_identity = self.identity()?;
        let connection = Connection::connect_from(stream, remote, user_identity).await?;
        self.init_connection(remote, connection).await
    }

    async fn listen(&mut self, listen_addr: SocketAddr) -> CoreResult<NetworkEvent> {
        log::trace!("{}", current_function!());
        if self.listener.is_some() {
            log::error!("tried to start listening, but a listener already exists!");
            panic!()
        }
        let listener = net::TcpListener::bind(listen_addr).await?;
        let listen_addr = listener.local_addr()?;

        self.listener = Some(listener);

        Ok(NetworkEvent::ListenerStarted(listen_addr))
    }

    pub(super) async fn handle_incoming_connection(
        state: NetworkDomainSync,
        stream: net::TcpStream,
        remote: SocketAddr,
        event_channel: Sender<NetworkEvent>,
    ) -> CoreResult<()> {
        log::trace!("{}", current_function!());
        log::info!("Handling incoming connection from {remote}");
        let event = state.write().await.connect_from(stream, remote).await?;
        event_channel.send(event).await?;

        Ok(())
    }
}
