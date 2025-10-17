use std::ops::Deref;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    error::CoreResult,
    identity::{Identity, Trust, UserIdentity},
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

    #[cold]
    pub fn from_user_identity(user: &UserIdentity) -> Self {
        Self {
            identity: user.identity.clone(),
            trust: Trust::Trusted,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
        }
    }

    /// Sets the last-seen timestamp of this [`ContactIdentity`] to now.
    pub fn seen(&mut self) {
        self.last_seen = Utc::now();
    }
}

impl Deref for ContactIdentity {
    type Target = Identity;

    fn deref(&self) -> &Self::Target {
        &self.identity
    }
}
