use async_channel::SendError;
use sremp_core::{error::CoreError, identity::ContactId};
use thiserror::Error;

use crate::domain::{UiCommand, UiEvent};

pub type ClientResult<T> = std::result::Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Could send a ui event to the over the local async channel")]
    ChannelSendUiEvent(Box<async_channel::SendError<UiEvent>>),
    #[error("Could send a ui command to the over the local async channel")]
    ChannelSendUiCmd(Box<async_channel::SendError<UiCommand>>),
    #[error(transparent)]
    CoreError(CoreError),
    #[error("No connection exists to {}. Can't send message to them!", .0)]
    NoConnection(ContactId),
}

impl From<CoreError> for ClientError {
    fn from(value: CoreError) -> Self {
        Self::CoreError(value)
    }
}

impl From<SendError<UiCommand>> for ClientError {
    fn from(value: SendError<UiCommand>) -> Self {
        ClientError::ChannelSendUiCmd(Box::new(value))
    }
}

impl From<SendError<UiEvent>> for ClientError {
    fn from(value: SendError<UiEvent>) -> Self {
        ClientError::ChannelSendUiEvent(Box::new(value))
    }
}
