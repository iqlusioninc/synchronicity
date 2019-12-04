//! Key Seed: base derivation key for all node cryptographic (private) keys

use hkd32::mnemonic;
use std::convert::TryInto;

/// Toplevel path component for personalizing all `synchro`-derived subkeys
pub const TOPLEVEL_DERIVATION_COMPONENT: &[u8] = b"synchro";

/// Key Seed
#[derive(Clone)]
pub struct KeySeed(mnemonic::Phrase);

impl KeySeed {
    /// Generate a random KeySeed
    pub fn generate() -> Self {
        KeySeed(mnemonic::Phrase::random(Default::default()))
    }

    /// Get the phrase for this `KeySeed` as a string
    pub fn phrase(&self) -> &str {
        self.0.phrase()
    }

    /// Derive a seed value given a domain and version
    pub fn derive_seed(&self, domain: &[u8], version: u32) -> [u8; 32] {
        let components = [
            TOPLEVEL_DERIVATION_COMPONENT,
            domain,
            &version.to_le_bytes(),
        ];

        let mut derivation_path = hkd32::PathBuf::new();

        for component in &components {
            derivation_path.push(hkd32::Component::new(component).unwrap());
        }

        self.0
            .clone()
            .derive_subkey(&derivation_path)
            .as_bytes()
            .try_into()
            .unwrap()
    }
}
