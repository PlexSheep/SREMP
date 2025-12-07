use std::{collections::hash_map::Entry, net::SocketAddr, sync::Arc};

use tokio::net;

use crate::{
    current_function,
    domain::{ConnectionData, NetworkCommand, NetworkDomain, NetworkDomainSync, NetworkEvent},
    error::{CoreError, CoreResult},
    identity::UserIdentity,
    net::connection::{Connection, MAX_FRAME_SIZE},
};

impl NetworkDomain {
    pub(super) async fn process_network_command(
        state: NetworkDomainSync,
        command: NetworkCommand,
    ) -> CoreResult<()> {
        log::trace!("{}", current_function!());
        log::info!("Processing Network Command: {command}");
        match command {
            NetworkCommand::Connect(remote) => Self::connect_to(state.clone(), remote).await?,
            NetworkCommand::StartListener(listen_addr) => {
                state.write().await.listen(listen_addr).await?
            }
            NetworkCommand::StopListener => {
                if let Some(listener) = state.write().await.listener.take() {
                    log::info!("Stopping listener");
                    drop(listener);
                } else {
                    log::warn!("No listener currently exists!")
                }
                state
                    .read()
                    .await
                    .send_net_evt(NetworkEvent::ListenerStopped)
                    .await
            }
            NetworkCommand::SetIdentity(iden) => state.write().await.user_identity = iden,
            NetworkCommand::Disconnect(remote) => {
                if let Some(connection) = state.write().await.active_connections.remove(&remote) {
                    connection.conn.disconnect().await?;
                } else {
                    log::warn!("{remote} has no active connection")
                }
            }
            NetworkCommand::SendMessage(remote, cid, data) => {
                let mut state_b = state.write().await;
                let condat = state_b
                    .active_connections
                    .get_mut(&remote)
                    .expect("no active connection for this remote");

                if condat.iden.id() != cid {
                    panic!("connection identity does not match our entries somehow")
                }

                condat.conn.send_direct_message(&data).await?;
            }
            _ => todo!(),
        };
        Ok(())
    }

    async fn init_connection(
        state: NetworkDomainSync,
        remote: SocketAddr,
        connection: Connection,
    ) -> CoreResult<()> {
        log::trace!("{}", current_function!());
        let remote_identity = connection.peer_identity().await.clone();

        match state.write().await.active_connections.entry(remote) {
            // we already have a connection with this socket addr???
            Entry::Occupied(_en) => {
                log::warn!("Duplicated connection, closing second connection...");
                connection.disconnect().await?;
                state
                    .read()
                    .await
                    .send_net_evt(NetworkEvent::ConnectionFailed(
                        remote,
                        "already connected to this peer".to_string(),
                    ))
                    .await;
                return Ok(());
            }
            Entry::Vacant(en) => en.insert(ConnectionData {
                conn: connection,
                iden: remote_identity.clone(),
            }),
        };

        let state_c = state.clone();
        tokio::spawn(
            async move { Self::process_incoming_frames_for_connection(state_c, remote).await },
        );

        state
            .read()
            .await
            .send_net_evt(NetworkEvent::ConnectionEstablished(
                remote,
                remote_identity.into(),
            ))
            .await;
        Ok(())
    }

    fn identity(&self) -> CoreResult<Arc<UserIdentity>> {
        self.user_identity
            .as_ref()
            .ok_or(CoreError::NoUserIdentity)
            .inspect_err(|e| log::error!("Can't connect without identity: {e}"))
            .cloned()
    }

    async fn connect_to(state: NetworkDomainSync, remote: SocketAddr) -> CoreResult<()> {
        log::trace!("{}", current_function!());
        let connection = {
            let state_b = state.read().await;
            let user_identity = state_b.identity()?;
            Connection::connect_to(remote, &user_identity).await?
        };
        Self::init_connection(state, remote, connection).await
    }

    async fn connect_from(
        state: NetworkDomainSync,
        stream: net::TcpStream,
        remote: SocketAddr,
    ) -> CoreResult<()> {
        log::trace!("{}", current_function!());
        let connection = {
            let state_b = state.read().await;
            let user_identity = state_b.identity()?;
            Connection::connect_from(stream, remote, &user_identity).await?
        };
        Self::init_connection(state, remote, connection).await
    }

    async fn listen(&mut self, listen_addr: SocketAddr) -> CoreResult<()> {
        log::trace!("{}", current_function!());
        if self.listener.is_some() {
            let msg = "tried to start listening, but a listener already exists!";
            log::error!("{msg}");
            log::debug!("Listener: {:?}", self.listener);
            panic!("{msg}")
        }
        let listener = net::TcpListener::bind(listen_addr).await?;
        let listen_addr = listener.local_addr()?;

        self.listener = Some(listener);

        self.send_net_evt(NetworkEvent::ListenerStarted(listen_addr))
            .await;
        Ok(())
    }

    pub(super) async fn handle_incoming_connection(
        state: NetworkDomainSync,
        stream: net::TcpStream,
        remote: SocketAddr,
    ) -> CoreResult<()> {
        log::trace!("{}", current_function!());
        log::info!("Handling incoming connection from {remote}");

        Self::connect_from(state, stream, remote).await?;

        Ok(())
    }

    async fn process_incoming_frames_for_connection(
        state: NetworkDomainSync,
        remote: SocketAddr,
    ) -> CoreResult<()> {
        let mut receive_buff = Vec::with_capacity(MAX_FRAME_SIZE);
        log::debug!("Started listening for frames from {remote}");
        loop {
            let mut state_b = state.write().await;
            let conn = match state_b.active_connections.get_mut(&remote) {
                Some(conn) => conn,
                None => {
                    log::warn!(
                        "Active Connection to {remote} does not exist anymore, stopping listener job for this connection.",
                    );
                    return Ok(());
                }
            };

            if !conn.conn.has_receive_pending().await? {
                drop(state_b);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }
            log::debug!("There is a pending message for {remote}");

            conn.conn.receive_direct_message(&mut receive_buff).await?;
            let cid = conn.iden.id();
            log::debug!(
                "Received a direct message from {remote} {} ({})",
                cid,
                conn.iden.username()
            );

            // TODO: we dont yet actually make any checks if the message is signed, authentic and
            // so on. I think that should be done here?

            // WARN: i'm not sure how select works. It would be bad if we got a message and
            // the processing stopped because of some time limit.

            drop(state_b);
            let copy_buf = Arc::new(receive_buff.clone());
            state
                .read()
                .await
                .send_net_evt(NetworkEvent::IncomingMessage(remote, cid, copy_buf))
                .await;

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}
