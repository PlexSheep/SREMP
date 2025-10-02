use serde::{Deserialize, Serialize};

use crate::PROTOCOL_DIRECT_NAME;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionHeader {
    name: [u8; 12],
    major: u8,
    minor: u8,
}

impl VersionHeader {
    // WARN: Changing the length in bytes is not backwards compatible
    pub const BYTE_LENGTH: usize = PROTOCOL_DIRECT_NAME.len() + 2;
}

impl Default for VersionHeader {
    fn default() -> Self {
        PROTOCOL_DIRECT_VERSION_HEADER
    }
}

pub const PROTOCOL_DIRECT_VERSION_HEADER: VersionHeader = VersionHeader {
    name: *PROTOCOL_DIRECT_NAME,
    major: 0,
    minor: 1,
};
