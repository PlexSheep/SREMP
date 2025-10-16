use serde::{Deserialize, Serialize};

use crate::{
    error::CoreResult,
    identity::{
        ContactId, Identity,
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
    #[cold]
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
    #[inline(always)]
    pub fn identity_private_key(&self) -> &ed25519_dalek::SigningKey {
        &self.identity_key
    }

    #[inline(always)]
    pub fn noise_private_key(&self) -> &x25519_dalek::StaticSecret {
        &self.noise_key
    }

    #[cold]
    pub fn rotate_noise_private_key(&mut self) -> CoreResult<()> {
        let noise_priv = generate_good_key_x25519();
        let noise_pub = x25519_dalek::PublicKey::from(&noise_priv);

        let mut id_priv = self.identity_private_key().clone();
        self.identity.set_noise_key(noise_pub, &mut id_priv)
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
