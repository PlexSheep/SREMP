use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::CoreResult;

mod id;
pub use id::*;

mod trust;
pub use trust::*;

mod user;
pub use user::*;

mod contact;
pub use contact::*;

mod crypto;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Flags {
    pub uses_relay: bool,
    pub is_machine_account: bool,
    pub is_relay_server: bool,
    pub prefers_async: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityVerifiedData {
    username: String, // TODO: 1 to 40 characters according to spec
    identity_key: ed25519_dalek::VerifyingKey,
    noise_key: x25519_dalek::PublicKey,
    flags: Flags,
    extensions: Option<Extensions>,
    version: u64,
    created: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identity {
    verified: IdentityVerifiedData,
    signature: ed25519_dalek::Signature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Extensions {
    profile_picture: Option<Vec<u8>>,
    additional_metadata: HashMap<String, Vec<u8>>,
}

impl Identity {
    /// Creates a new [`Identity`].
    pub fn create(
        username: &str,
        identity_private_key: &mut ed25519_dalek::SigningKey,
        noise_public_key: x25519_dalek::PublicKey,
    ) -> CoreResult<Self> {
        Self::validate_username(username)?;

        let vd = IdentityVerifiedData {
            username: username.to_string(),
            identity_key: identity_private_key.verifying_key(),
            noise_key: noise_public_key,
            flags: Default::default(),
            extensions: Default::default(),
            version: 0,
            created: Utc::now(),
        };

        let sig = vd.sign(identity_private_key)?;
        Ok(Self {
            verified: vd,
            signature: sig,
        })
    }

    /// Returns a reference to the username of this [`Identity`].
    #[inline]
    pub fn username(&self) -> &str {
        &self.verified.username
    }

    pub fn validate_username(username: &str) -> CoreResult<()> {
        let chars_len = username.chars().count();
        if !(1..=40).contains(&chars_len) {
            Err(crate::error::CoreError::InvalidUsername)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn set_username(
        &mut self,
        username: &str,
        private_key: &mut ed25519_dalek::SigningKey,
    ) -> CoreResult<()> {
        Self::validate_username(username)?;
        self.verified.username = username.to_string();
        self.post_update(private_key)
    }

    #[inline(always)]
    pub fn id(&self) -> ContactId {
        self.identity_key().into()
    }

    #[inline(always)]
    pub fn identity_key(&self) -> ed25519_dalek::VerifyingKey {
        self.verified.identity_key
    }

    #[inline(always)]
    pub fn noise_key(&self) -> x25519_dalek::PublicKey {
        self.verified.noise_key
    }

    #[inline]
    pub fn set_noise_key(
        &mut self,
        noise_key: x25519_dalek::PublicKey,
        private_key: &mut ed25519_dalek::SigningKey,
    ) -> CoreResult<()> {
        self.verified.noise_key = noise_key;
        self.post_update(private_key)
    }

    #[inline(always)]
    pub fn flags(&self) -> Flags {
        self.verified.flags
    }

    #[inline]
    pub fn set_flags(
        &mut self,
        flags: Flags,
        private_key: &mut ed25519_dalek::SigningKey,
    ) -> CoreResult<()> {
        self.verified.flags = flags;
        self.post_update(private_key)
    }

    #[inline(always)]
    pub fn extensions(&self) -> Option<&Extensions> {
        self.verified.extensions.as_ref()
    }

    #[inline]
    pub fn set_extensions(
        &mut self,
        extensions: Option<Extensions>,
        private_key: &mut ed25519_dalek::SigningKey,
    ) -> CoreResult<()> {
        self.verified.extensions = extensions;
        self.post_update(private_key)
    }

    #[inline(always)]
    pub fn version(&self) -> u64 {
        self.verified.version
    }

    #[inline(always)]
    pub fn created(&self) -> DateTime<Utc> {
        self.verified.created
    }
}
