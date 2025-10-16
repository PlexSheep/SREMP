use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};
use ed25519_dalek::ed25519::signature::SignerMut;
use serde::{Deserialize, Serialize};

use crate::error::CoreResult;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserIdentity {
    pub identity: Identity,
    pub private_key: ed25519_dalek::SigningKey,
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
    pub fn create(username: &str, private_key: &mut ed25519_dalek::SigningKey) -> CoreResult<Self> {
        Self::validate_username(username)?;

        let vd = IdentityVerifiedData {
            username: username.to_string(),
            identity_key: private_key.verifying_key(),
            noise_key: generate_good_key_x25519(),
            flags: Default::default(),
            extensions: Default::default(),
            version: 0,
            created: Utc::now(),
        };

        let sig = vd.sign(private_key)?;
        Ok(Self {
            verified: vd,
            signature: sig,
        })
    }

    pub fn verify(&self) -> CoreResult<()> {
        self.verified
            .identity_key
            .verify_strict(self.verified.bytes()?.as_slice(), &self.signature)?;
        Self::validate_username(&self.verified.username)?;
        Ok(())
    }

    /// Returns a reference to the username of this [`Identity`].
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
}

impl UserIdentity {
    /// Creates a new [`UserIdentity`].
    pub fn build(username: &str) -> CoreResult<Self> {
        let key = generate_good_key_ed25519();
        Self::load(username, key, Utc::now())
    }

    /// Create a [`UserIdentity`] from the necessary values.
    pub fn load(
        username: &str,
        key: ed25519_dalek::SigningKey,
        created: DateTime<Utc>,
    ) -> CoreResult<Self> {
        todo!()
    }

    /// Returns a reference to the private key of this [`UserIdentity`].
    pub fn private_key(&self) -> &ed25519_dalek::SigningKey {
        &self.private_key
    }
}

impl ContactIdentity {
    /// Creates a new [`ContactIdentity`].
    pub fn build(
        username: &str,
        public_key: ed25519_dalek::VerifyingKey,
        trust: Trust,
        first_seen: DateTime<Utc>,
        last_seen: DateTime<Utc>,
    ) -> CoreResult<Self> {
        let identity = Identity::create(username, public_key)?;
        Ok(Self {
            identity,
            trust,
            first_seen,
            last_seen,
        })
    }

    /// Sets the last-seen timestamp of this [`ContactIdentity`].
    pub fn set_last_seen(&mut self, last_seen: DateTime<Utc>) {
        self.last_seen = last_seen;
    }

    /// Get a dummy [`ContactIdentity`], only available in debug mode.
    #[cfg(debug_assertions)]
    pub fn debug_contact() -> Self {
        let key = generate_good_key_ed25519();
        ContactIdentity::build(
            "DEBUG_CONTACT",
            key.verifying_key(),
            Trust::Unknown,
            Utc::now(),
            Utc::now(),
        )
        .unwrap()
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

fn generate_good_key_x25519() -> x25519_dalek::StaticSecret {
    let mut csprng: rand::rngs::OsRng = rand::rngs::OsRng;
    let mut k;
    let mut guard = 0;
    loop {
        k = x25519_dalek::StaticSecret::random_from_rng(&mut csprng);
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

fn generate_good_key_ed25519() -> ed25519_dalek::SigningKey {
    let mut csprng: rand::rngs::OsRng = rand::rngs::OsRng;
    let mut k;
    let mut guard = 0;
    loop {
        k = ed25519_dalek::SigningKey::random_from_rng(&mut csprng);
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
