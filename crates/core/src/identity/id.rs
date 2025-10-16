use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactId {
    #[serde(flatten)]
    key: ed25519_dalek::VerifyingKey,
}

impl From<ed25519_dalek::VerifyingKey> for ContactId {
    #[inline(always)]
    fn from(value: ed25519_dalek::VerifyingKey) -> Self {
        Self { key: value }
    }
}

impl PartialOrd for ContactId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ContactId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.as_bytes().cmp(&other.key.as_bytes())
    }
}

impl Display for ContactId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_key(&self.key))
    }
}

#[inline]
pub fn format_key(key: &ed25519_dalek::VerifyingKey) -> String {
    let mut buf = String::new();
    for b in key.as_bytes() {
        buf.push_str(&format!("{b:02X}"));
    }
    buf
}
