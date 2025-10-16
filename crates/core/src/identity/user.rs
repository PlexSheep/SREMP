use serde::{Deserialize, Serialize};

use crate::{
    error::CoreResult,
    identity::{
        Identity,
        crypto::{generate_good_key_ed25519, generate_good_key_x25519},
    },
};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserIdentity {
    pub identity: Identity,
    pub identity_key: ed25519_dalek::SigningKey,
    pub noise_key: x25519_dalek::StaticSecret,
}

impl UserIdentity {
    /// Creates a new [`UserIdentity`].
    pub fn create(username: &str) -> CoreResult<Self> {
        let mut identity_key = generate_good_key_ed25519();
        let noise_key = generate_good_key_x25519();
        let noise_pkey = x25519_dalek::PublicKey::from(&noise_key);

        let identity = Identity::create(username, &mut identity_key, noise_pkey)?;

        Ok(Self {
            identity,
            identity_key,
            noise_key,
        })
    }

    /// Returns a reference to the private key of this [`UserIdentity`].
    pub fn private_key(&self) -> &ed25519_dalek::SigningKey {
        &self.identity_key
    }
}

impl std::fmt::Debug for UserIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserIdentity")
            .field("identity", &self.identity)
            .field("identity_key", &self.identity_key)
            .field("noise_key", &"{redacted}")
            .finish()
    }
}
