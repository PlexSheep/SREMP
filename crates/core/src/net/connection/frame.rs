use std::ops::Deref;

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net,
};

use crate::error::{CoreError, CoreResult};

mod version_header;
pub use version_header::*;

pub(super) const MAX_FRAME_SIZE: usize = 65535;

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub(super) struct Frame {
    version: VersionHeader,
    data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct FrameBody {}

impl Frame {
    #[inline]
    fn from_raw(data: &[u8]) -> CoreResult<Self> {
        check_length(data.len())?;
        Ok(Self {
            version: check_version(data)?,
            data: data[VersionHeader::BYTE_LENGTH..].to_vec(),
        })
    }

    #[inline]
    pub fn from_payload(data: &[u8]) -> CoreResult<Self> {
        check_length(data.len())?;
        Ok(Self {
            version: PROTOCOL_DIRECT_VERSION_HEADER,
            data: data.to_vec(),
        })
    }

    pub async fn send(self, stream: &mut net::TcpStream) -> CoreResult<()> {
        log::debug!("Sending Frame");
        log::trace!("Sending Length");
        stream.write_u16(self.len()).await?;
        log::trace!("Sending version");
        stream.write_all(&rmp_serde::to_vec(&self.version)?).await?;
        stream.flush().await?;
        log::trace!("Sending Data");
        stream.write_all(&self.data).await?;
        stream.flush().await?;
        log::trace!("Sending Finished");
        Ok(())
    }

    pub async fn recv(stream: &mut net::TcpStream) -> CoreResult<Self> {
        log::debug!("Receiving Frame");
        log::trace!("Reading Length");
        let len = stream.read_u16().await? as usize;
        check_length(len)?;
        log::trace!("Length: {len}");

        log::trace!("Reading Data");
        let mut buf = vec![0; len];
        buf.reserve_exact(len);
        stream.read_exact(&mut buf).await?;
        log::trace!("Data: {buf:x?}");

        Self::from_raw(&buf[..])
    }

    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn len(&self) -> u16 {
        self.data.len() as u16 // cannot construct a frame that is too big
    }

    #[inline(always)]
    pub(super) fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Deref for Frame {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[inline]
fn check_length(length: usize) -> CoreResult<u16> {
    if length > MAX_FRAME_SIZE {
        return Err(CoreError::FrameTooLarge(length));
    }
    Ok(length.try_into().unwrap())
}

#[inline]
fn check_version(raw_data: &[u8]) -> CoreResult<VersionHeader> {
    let version: VersionHeader =
        rmp_serde::from_slice(&raw_data[0..=VersionHeader::BYTE_LENGTH])
            .inspect_err(|e| log::error!("Version of frame could not be read: {e}"))?;
    log::trace!("Version: {version:#?}");
    Ok(version)
}
