use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{PROTOCOL_DIRECT_NAME, error::CoreResult};

// WARN: Changing the length in bytes is not backwards compatible
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionHeader {
    name: [u8; 12],
    major: u8,
    minor: u8,
}

impl VersionHeader {
    pub const BYTE_LENGTH: usize = PROTOCOL_DIRECT_NAME.len() + 2;

    pub fn as_bytes(&self) -> &[u8; Self::BYTE_LENGTH] {
        unsafe { std::mem::transmute(self) }
    }

    pub fn from_raw(data: &[u8; Self::BYTE_LENGTH]) -> CoreResult<Self> {
        let name: [u8; PROTOCOL_DIRECT_NAME.len()] =
            data[0..PROTOCOL_DIRECT_NAME.len()].try_into().unwrap();
        let major = data[Self::BYTE_LENGTH - 2];
        let minor = data[Self::BYTE_LENGTH - 1];
        if name != *PROTOCOL_DIRECT_NAME {
            return Err(crate::error::CoreError::BadProtocolName(name));
        }
        Ok(Self {
            name: *PROTOCOL_DIRECT_NAME,
            major,
            minor,
        })
    }
}

impl Default for VersionHeader {
    fn default() -> Self {
        PROTOCOL_DIRECT_VERSION_HEADER
    }
}

impl Display for VersionHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} v{}.{}",
            String::from_utf8_lossy(&self.name),
            self.major,
            self.minor
        )
    }
}

pub const PROTOCOL_DIRECT_VERSION_HEADER: VersionHeader = VersionHeader {
    name: *PROTOCOL_DIRECT_NAME,
    major: 0,
    minor: 1,
};
