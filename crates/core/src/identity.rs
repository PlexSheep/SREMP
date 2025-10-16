use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use chrono::{DateTime, Utc};
use ed25519_dalek::ed25519::signature::SignerMut;
use serde::{Deserialize, Serialize};

use crate::error::CoreResult;

pub type ContactId = ed25519_dalek::VerifyingKey;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Trust {
    Unknown,
    Trusted,
    Rejected,
}

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

#[derive(Clone, Serialize, Deserialize)]
pub struct UserIdentity {
    pub identity: Identity,
    pub identity_key: ed25519_dalek::SigningKey,
    pub noise_key: x25519_dalek::StaticSecret,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactIdentity {
    pub identity: Identity,
    pub trust: Trust,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

impl IdentityVerifiedData {
    /// NOTE: this is not the in memory representation, but should still be used for signing and
    /// verification
    pub fn bytes(&self) -> CoreResult<Vec<u8>> {
        Ok(rmp_serde::to_vec(self)?)
    }

    pub fn sign(
        &self,
        private_key: &mut ed25519_dalek::SigningKey,
    ) -> CoreResult<ed25519_dalek::Signature> {
        Ok(private_key.try_sign(&self.bytes()?)?)
    }
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

    #[inline]
    pub fn verify(&self) -> CoreResult<()> {
        self.verified
            .identity_key
            .verify_strict(self.verified.bytes()?.as_slice(), &self.signature)?;
        Self::validate_username(&self.verified.username)?;
        Ok(())
    }

    fn post_update(&mut self, private_key: &mut ed25519_dalek::SigningKey) -> CoreResult<()> {
        if let Some(nv) = self.verified.version.checked_add(1) {
            self.verified.version = nv;
        } else {
            panic!(
                "Updating the identity has failed: Version overflow (was {})",
                self.verified.version
            )
        }

        self.signature = self.verified.sign(private_key)?;
        Ok(())
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
        self.identity_key()
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

impl Display for Trust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "Unknown",
                Self::Trusted => "Trusted",
                Self::Rejected => "Rejected",
            }
        )
    }
}

impl Debug for UserIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserIdentity")
            .field("identity", &self.identity)
            .field("identity_key", &self.identity_key)
            .field("noise_key", &"{redacted}")
            .finish()
    }
}

fn generate_good_key_x25519() -> x25519_dalek::StaticSecret {
    let mut csprng: rand::rngs::OsRng = rand::rngs::OsRng;
    x25519_dalek::StaticSecret::random_from_rng(&mut csprng)
}

fn generate_good_key_ed25519() -> ed25519_dalek::SigningKey {
    let mut csprng: rand::rngs::OsRng = rand::rngs::OsRng;
    let mut k;
    let mut guard = 0;
    loop {
        k = ed25519_dalek::SigningKey::generate(&mut csprng);
        if !k.verifying_key().is_weak() {
            return k;
        }
        guard += 1;
        if guard > 10 {
            panic!(
                "10 fails in a row to creating a good key. This is almost impossible! Something is wrong with your system!"
            )
        }
    }
}

pub fn format_key(key: &ed25519_dalek::VerifyingKey) -> String {
    let mut buf = String::new();
    for b in key.as_bytes() {
        buf.push_str(&format!("{b:02X}"));
    }
    buf
}
