use std::{net::SocketAddr, sync::Arc};

use async_channel::{Receiver, Sender};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
    task::JoinHandle,
};

mod active_connections;
mod commands;
mod events;
mod jobs;

pub(crate) use active_connections::*;
pub use commands::NetworkCommand;
pub use events::NetworkEvent;

use crate::{current_function, error::CoreResult, identity::UserIdentity};

pub type NetworkDomainSync = Arc<tokio::sync::RwLock<NetworkDomain>>;

const JOB_ITERATION_INTERVAL_MS: u64 = 30;

#[derive(Debug, Clone)]
pub(crate) struct Channels {
    pub(crate) net_command_channel: Receiver<NetworkCommand>,
    pub(crate) net_event_channel: Sender<NetworkEvent>,
}

#[derive(Debug, Default)]
pub struct NetworkDomain {
    pub(crate) active_connections: ActiveConnections,
    pub(crate) user_identity: Option<UserIdentity>,
    pub(crate) listener: Option<TcpListener>,
    channels: Option<Channels>,
}

impl NetworkDomain {
    pub fn new() -> Self {
        Self::default()
    }

    fn into_sync(self) -> NetworkDomainSync {
        Arc::new(RwLock::new(self))
    }

    async fn listener_accept_or_wait(&self) -> CoreResult<(TcpStream, SocketAddr)> {
        let incoming = match &self.listener {
            Some(l) => l.accept().await?,
            None => std::future::pending().await,
        };
        Ok(incoming)
    }

    #[inline]
    pub(crate) fn channels_ref(&self) -> &Channels {
        self.channels
            .as_ref()
            .expect("channels were not initialized")
    }

    #[inline]
    pub(crate) fn net_command_channel(&self) -> &Receiver<NetworkCommand> {
        &self.channels_ref().net_command_channel
    }

    #[inline]
    pub(crate) fn net_event_channel(&self) -> &Sender<NetworkEvent> {
        &self.channels_ref().net_event_channel
    }

    #[inline]
    pub(crate) async fn send_net_evt(&self, evt: NetworkEvent) {
        log::info!("Emitting net event: {evt}");
        self.net_event_channel()
            .send(evt)
            .await
            .expect("could not send net event");
    }

    async fn run(self) -> CoreResult<()> {
        log::trace!("{}", current_function!());
        let ssy = self.into_sync();
        loop {
            let this = ssy.read().await;
            tokio::select! {
                cmd = this.net_command_channel().recv() => {
                    drop(this);
                    ssy.write().await.process_network_command(cmd?).await?;
                },
                incoming = this.listener_accept_or_wait() => {
                    drop(this);
                    let (stream, remote) = incoming?;
                    let ssyc = ssy.clone();
                    tokio::spawn(async move {
                        Self::handle_incoming_connection(ssyc, stream, remote).await
                    });
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(JOB_ITERATION_INTERVAL_MS)) => {
                    // nothing
                }
            };
        }
    }

    pub fn start(
        mut self,
        net_command_channel: Receiver<NetworkCommand>,
        net_event_channel: Sender<NetworkEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> CoreResult<JoinHandle<CoreResult<()>>> {
        self.channels = Some(Channels {
            net_command_channel,
            net_event_channel,
        });

        let handle = rt.spawn(async { self.run().await });
        log::info!("Network domain has been started");
        Ok(handle)
    }
}
