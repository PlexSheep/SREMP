use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use async_channel::{Receiver, Sender};
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, task::JoinHandle};

mod commands;
mod events;
mod jobs;

pub use commands::UiCommand;
pub use events::UiEvent;
use sremp_core::{
    chat::Chat,
    current_function,
    domain::{NetworkCommand, NetworkEvent},
    error::CoreError,
    identity::UserIdentity,
};

use crate::domain::known_identities::KnownIdentities;
use crate::error::ClientResult;

pub mod known_identities;

pub type ClientDomainSync = Arc<RwLock<ClientDomain>>;

const JOB_ITERATION_INTERVAL_MS: u64 = 200;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClientDomain {
    pub(crate) known_identities: KnownIdentities,
    pub(crate) chats: HashMap<VerifyingKey, Chat>,
    pub(crate) user_identity: Option<UserIdentity>,
    pub(crate) open_connections: HashMap<VerifyingKey, SocketAddr>,
    #[serde(skip)]
    channels: Option<Channels>,
}

#[derive(Debug, Clone)]
pub(crate) struct Channels {
    pub(crate) net_command_channel: Sender<NetworkCommand>,
    pub(crate) net_event_channel: Receiver<NetworkEvent>,
    pub(crate) ui_command_channel: Receiver<UiCommand>,
    pub(crate) ui_event_channel: Sender<UiEvent>,
}

impl ClientDomain {
    pub fn new() -> Self {
        Self::default()
    }

    fn into_sync(self) -> ClientDomainSync {
        Arc::new(RwLock::new(self))
    }

    async fn run(self) -> ClientResult<()> {
        let ssy = self.into_sync();
        loop {
            let this = ssy.read().await;
            tokio::select! {
                cmd = this.ui_command_channel().recv() => {
                    drop(this);
                    let cmd = cmd.map_err(CoreError::from)?;
                    ssy.write().await.process_ui_command(
                        cmd,
                    ).await?;
                },
                evt = this.net_event_channel().recv() => {
                    drop(this);
                    let evt = evt.map_err(CoreError::from)?;
                    ssy.write().await.process_net_event(evt).await?;
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(JOB_ITERATION_INTERVAL_MS)) => {
                    // WARN: not sure, but this might kill the execution of other branches?
                    continue;
                }
            };
        }
    }

    pub fn start(
        mut self,
        net_command_channel: Sender<NetworkCommand>,
        net_event_channel: Receiver<NetworkEvent>,
        ui_command_channel: Receiver<UiCommand>,
        ui_event_channel: Sender<UiEvent>,
        rt: &mut tokio::runtime::Runtime,
    ) -> ClientResult<JoinHandle<ClientResult<()>>> {
        self.channels = Some(Channels {
            net_command_channel,
            net_event_channel,
            ui_command_channel,
            ui_event_channel,
        });
        let handle = rt.spawn(async { self.run().await });
        log::info!("Application domain has started");
        Ok(handle)
    }

    #[inline]
    pub(crate) fn channels_ref(&self) -> &Channels {
        self.channels
            .as_ref()
            .expect("channels were not initialized")
    }

    #[inline]
    pub(crate) fn channels(&self) -> Channels {
        self.channels_ref().clone()
    }

    #[inline]
    pub(crate) fn net_command_channel(&self) -> &Sender<NetworkCommand> {
        &self.channels_ref().net_command_channel
    }

    #[inline]
    pub(crate) fn net_event_channel(&self) -> &Receiver<NetworkEvent> {
        &self.channels_ref().net_event_channel
    }

    #[inline]
    pub(crate) fn ui_command_channel(&self) -> &Receiver<UiCommand> {
        &self.channels_ref().ui_command_channel
    }

    #[inline]
    pub(crate) fn ui_event_channel(&self) -> &Sender<UiEvent> {
        &self.channels_ref().ui_event_channel
    }

    #[inline]
    pub(crate) async fn send_net_cmd(&self, cmd: NetworkCommand) {
        log::info!("Sending net command: {cmd}");
        self.net_command_channel()
            .send(cmd)
            .await
            .expect("could not send net command");
    }

    #[inline]
    pub(crate) async fn send_ui_evt(&self, evt: UiEvent) {
        log::info!("Emitting ui event: {evt}");
        self.ui_event_channel()
            .send(evt)
            .await
            .expect("could not send ui event");
    }
}
