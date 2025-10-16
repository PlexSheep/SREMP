use ed25519_dalek::ed25519::signature::SignerMut;

use crate::{
    error::CoreResult,
    identity::{Identity, IdentityVerifiedData},
};

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
    pub(super) fn post_update(
        &mut self,
        private_key: &mut ed25519_dalek::SigningKey,
    ) -> CoreResult<()> {
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

    #[inline]
    pub fn verify(&self) -> CoreResult<()> {
        log::debug!("Verifying Identity {}.", self.id());
        self.verified
            .identity_key
            .verify_strict(self.verified.bytes()?.as_slice(), &self.signature)?;
        log::debug!("Signature is valid");
        Self::validate_username(&self.verified.username)?;
        log::debug!("Username is valid");
        Ok(())
    }
}

pub(super) fn generate_good_key_x25519() -> x25519_dalek::StaticSecret {
    let mut csprng: rand::rngs::OsRng = rand::rngs::OsRng;
    x25519_dalek::StaticSecret::random_from_rng(&mut csprng)
}

pub(super) fn generate_good_key_ed25519() -> ed25519_dalek::SigningKey {
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
