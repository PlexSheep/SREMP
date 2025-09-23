use std::sync::Arc;

use async_channel::{Receiver, Sender};
use tokio::{net::TcpListener, sync::RwLock, task::JoinHandle};

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

#[derive(Debug, Default)]
pub struct NetworkDomain {
    pub(crate) active_connections: ActiveConnections,
    pub(crate) user_identity: Option<UserIdentity>,
    pub(crate) listener: Option<TcpListener>,
}

impl NetworkDomain {
    pub fn new() -> Self {
        Self::default()
    }

    fn into_sync(self) -> NetworkDomainSync {
        Arc::new(RwLock::new(self))
    }

    async fn get_listener_or_wait(&self) -> &TcpListener {
        match &self.listener {
            Some(l) => l,
            None => std::future::pending().await,
        }
    }

    async fn run(
        self,
        command_channel: Receiver<NetworkCommand>,
        event_channel: Sender<NetworkEvent>,
    ) -> CoreResult<()> {
        log::trace!("{}", current_function!());
        let ssy = self.into_sync();
        loop {
            log::trace!("net Workload");
            let this = ssy.read().await;
            log::trace!("net Workload got this");
            // BUG: This select never finishes, not even with sleep timeout
            tokio::select! {
                cmd = command_channel.recv() => {
                    log::trace!("net workload command");
                    drop(this);
                    let event = ssy.write().await.process_network_command(cmd?).await?;
                    event_channel.send(event).await?;
                },
                incoming = this.get_listener_or_wait().await.accept() => {
                    log::trace!("net workload incoming");
                    drop(this);
                    let (stream, remote) = incoming?;
                    let ssyc = ssy.clone();
                    let evtc = event_channel.clone();
                    tokio::spawn(async move {
                        Self::handle_incoming_connection(ssyc,stream,remote, evtc).await
                    });
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(JOB_ITERATION_INTERVAL_MS)) => {
                    log::trace!("net Workload timed out");
                    continue;
                }
            }
        }
    }

    pub fn start(
        self,
        command_channel: Receiver<NetworkCommand>,
        event_channel: Sender<NetworkEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> CoreResult<JoinHandle<CoreResult<()>>> {
        let handle = rt.spawn(async { self.run(command_channel, event_channel).await });
        log::info!("Network domain has been started");
        Ok(handle)
    }
}
