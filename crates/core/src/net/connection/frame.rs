use std::ops::Deref;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net,
};

use crate::error::{CoreError, CoreResult};

mod version_header;
pub use version_header::*;

pub const MAX_FRAME_SIZE: usize = 65535;
#[allow(clippy::cast_possible_truncation)]
pub const FRAME_OVERHEAD: u16 = (u16::BITS as u16 / 8u16) // 2 bytes for the length of the payload
        + VersionHeader::BYTE_LENGTH as u16 // 14 bytes for the version of the protocol used
;
pub const MAX_FRAME_PAYLOAD_SIZE: usize = MAX_FRAME_SIZE - FRAME_OVERHEAD as usize;

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Frame {
    version: VersionHeader,
    data: Vec<u8>,
}

impl Frame {
    #[inline]
    fn from_raw(full_frame_raw: &[u8]) -> CoreResult<Self> {
        if full_frame_raw.len() > MAX_FRAME_SIZE {
            return Err(CoreError::FrameTooLarge(full_frame_raw.len()));
        }
        Ok(Self {
            version: check_version(
                full_frame_raw[0..VersionHeader::BYTE_LENGTH]
                    .try_into()
                    .unwrap(),
            )?,
            data: full_frame_raw[VersionHeader::BYTE_LENGTH + 1..].to_vec(),
        })
    }

    #[inline]
    pub fn from_payload(payload: &[u8]) -> CoreResult<Self> {
        check_payload_length(payload.len())?;
        Ok(Self {
            version: PROTOCOL_DIRECT_VERSION_HEADER,
            data: payload.to_vec(),
        })
    }

    pub async fn send(self, stream: &mut net::TcpStream) -> CoreResult<()> {
        log::debug!("Sending Frame");
        log::trace!("Sending Payload-Length: {}", self.payload_len());
        stream.write_u16(self.payload_len()).await?;

        log::trace!("Sending version: {}", self.version());
        stream.write_all(self.version().as_bytes()).await?;
        stream.flush().await?;
        log::trace!("Sending Data: {:x?}", self.data);
        stream.write_all(&self.data).await?;
        stream.flush().await?;

        log::trace!("Sending Finished");
        Ok(())
    }

    pub async fn recv(stream: &mut net::TcpStream) -> CoreResult<Self> {
        log::debug!("Receiving Frame");
        log::trace!("Reading Payload-Length");
        let plen = stream.read_u16().await? as usize;
        check_payload_length(plen)?;
        if plen > MAX_FRAME_SIZE {
            return Err(CoreError::FrameTooLarge(plen));
        }
        log::trace!("Payload-Length: {plen}");

        log::trace!("Reading version");
        let mut buf = [0; VersionHeader::BYTE_LENGTH];
        stream.read_exact(&mut buf).await?;
        let version = check_version(&buf)?;

        log::trace!("Reading Data");
        let mut buf = vec![0; plen];
        buf.reserve_exact(plen);
        stream.read_exact(&mut buf).await?;
        log::trace!("Data: {buf:x?}");

        check_payload_length(buf.len())?;

        Ok(Self { version, data: buf })
    }

    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn payload_len(&self) -> u16 {
        self.data.len() as u16 // cannot construct a frame that is too big
    }

    #[inline(always)]
    pub(super) fn data(&self) -> &[u8] {
        &self.data
    }

    #[inline(always)]
    pub fn version(&self) -> &VersionHeader {
        &self.version
    }
}

impl Deref for Frame {
    type Target = Vec<u8>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[inline]
fn check_version(raw_data: &[u8; VersionHeader::BYTE_LENGTH]) -> CoreResult<VersionHeader> {
    let version: VersionHeader = VersionHeader::from_raw(raw_data)
        .inspect_err(|e| log::error!("Version of frame could not be read: {e}"))?;
    log::trace!("Version: {version}");
    Ok(version)
}

#[inline]
fn check_payload_length(plen: usize) -> CoreResult<()> {
    if plen > MAX_FRAME_PAYLOAD_SIZE {
        return Err(CoreError::FrameTooLarge(plen));
    }
    Ok(())
}
