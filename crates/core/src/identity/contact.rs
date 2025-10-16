use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    error::CoreResult,
    identity::{Identity, Trust},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactIdentity {
    pub identity: Identity,
    pub trust: Trust,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

impl ContactIdentity {
    /// Creates a new [`ContactIdentity`].
    pub fn from_peer_identity(identity: Identity, trust: Trust) -> CoreResult<Self> {
        Ok(Self {
            identity,
            trust,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
        })
    }

    /// Sets the last-seen timestamp of this [`ContactIdentity`].
    pub fn set_last_seen(&mut self, last_seen: DateTime<Utc>) {
        self.last_seen = last_seen;
    }
}
