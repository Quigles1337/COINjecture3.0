//! Trait-gated beacon boundary.
//!
//! P-002 selects Wesolowski proofs of repeated squaring in a transparently
//! derived imaginary-quadratic class group for the production construction.
//! No production implementation or parameter set ships in this packet.
//!
//! The associated types deliberately leave the canonical parent and the
//! proof-carrying output envelope to `cj3-types` and Gate G0. In particular,
//! the protocol's 32-byte random output must not be mistaken for the complete
//! production VDF witness.
//!
//! Testnet build profiles must set the registered `cj3_testnet` configuration
//! tag. Combining that tag with `devnet-placeholder` is a compile-time error.

#![forbid(unsafe_code)]

#[cfg(all(feature = "devnet-placeholder", cj3_testnet))]
compile_error!(
    "NOT-TESTNET-GRADE: sequentiality assumption only; the devnet placeholder is forbidden in a cj3_testnet build"
);

#[cfg(feature = "devnet-placeholder")]
pub mod devnet;

/// Deterministic, verifiable randomness derived from a parent.
///
/// Production implementations must make [`Self::verify`] substantially
/// cheaper than [`Self::output`]. The devnet placeholder intentionally cannot
/// provide that asymmetry and is excluded from testnet-tagged builds.
pub trait Beacon {
    /// Canonical parent representation supplied by the owning types crate.
    type Parent: ?Sized;

    /// Beacon result plus any proof material required for cheap verification.
    type Output;

    /// Computes the deterministic beacon result for `parent`.
    #[must_use]
    fn output(&self, parent: &Self::Parent) -> Self::Output;

    /// Verifies that `output` is the unique result for `parent`.
    #[must_use]
    fn verify(&self, parent: &Self::Parent, output: &Self::Output) -> bool;
}
