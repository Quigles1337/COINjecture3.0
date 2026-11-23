//! Fixed-width byte values that the layouts carry.
//!
//! Each is a distinct type so that an address cannot be passed where a public key is
//! expected. None of them validates its bytes: a transaction's `pubkey` and `sig` are
//! opaque to the decoder and are judged only by [`crate::ed25519::verify`].

/// A 32-byte account address: the whole digest `addr(pk)` (Protocol Spec §2).
///
/// A value read from the wire is any 32 bytes; only [`crate::addr`] derives one.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address([u8; 32]);

impl Address {
    /// Length of an address in every layout.
    pub const LEN: usize = 32;

    /// Wraps 32 bytes read from a layout or produced by [`crate::addr`].
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the 32 bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// A 32-byte Ed25519 public key in the RFC 8032 encoding, not yet judged.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PublicKey([u8; 32]);

impl PublicKey {
    /// Length of a public key in every layout.
    pub const LEN: usize = 32;

    /// Wraps 32 bytes as received.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the 32 bytes as received.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// A 64-byte Ed25519 signature, `R` then `S`, not yet judged.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Signature([u8; 64]);

impl Signature {
    /// Length of a signature in every layout.
    pub const LEN: usize = 64;

    /// Wraps 64 bytes as received.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Returns the 64 bytes as received.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

/// The 32-byte signing digest of a transaction: the Ed25519 message (§1.2).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SigningDigest([u8; 32]);

impl SigningDigest {
    pub(crate) const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the 32 bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
