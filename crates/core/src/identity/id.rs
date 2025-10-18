use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::ser_helper::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactId {
    #[serde(serialize_with = "ser_arc", deserialize_with = "deser_arc")]
    key: Arc<ed25519_dalek::VerifyingKey>,
}

impl From<ed25519_dalek::VerifyingKey> for ContactId {
    #[inline(always)]
    fn from(value: ed25519_dalek::VerifyingKey) -> Self {
        Self { key: value.into() }
    }
}

impl PartialOrd for ContactId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ContactId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.as_bytes().cmp(other.key.as_bytes())
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
