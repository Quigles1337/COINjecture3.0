//! Devnet-only iterated-hash beacon placeholder.
//!
//! `NOT-TESTNET-GRADE: sequentiality assumption only`
//!
//! This module has no succinct proof: verification repeats the entire hash
//! chain. It is an integration aid, not a production VDF, and compilation is
//! rejected when the `cj3_testnet` build tag is present.

use core::{marker::PhantomData, num::NonZeroU64};

use crate::Beacon;

/// Exact warning text that consumer-facing integrations must surface.
pub const SECURITY_BANNER: &str = "NOT-TESTNET-GRADE: sequentiality assumption only";

/// A 32-byte seed already derived from canonical parent data.
///
/// This module does not derive `H(D_BEACON || parent_hash)`. P-007 remains the
/// sole owner of that domain separation and canonical encoding.
pub type PreparedParent = [u8; 32];

/// The devnet placeholder's fixed-width output.
pub type DevnetBeaconOutput = [u8; 32];

/// Supplies the 32-byte digest implementation owned by cryptographic-types code.
///
/// Keeping the digest generic prevents this devnet placeholder from freezing
/// the protocol hash before Gate G0.
pub trait Digest32 {
    /// Hashes one fixed-width chain element into the next.
    #[must_use]
    fn digest(input: &[u8; 32]) -> [u8; 32];
}

/// Fixed-length iterated-hash placeholder for devnet integration.
///
/// The nonzero iteration count is explicit and immutable for an instance. CJ3
/// does not provide a default because P-006/Gate G0 own timing calibration.
pub struct IteratedHash<D> {
    iterations: NonZeroU64,
    digest: PhantomData<fn() -> D>,
}

impl<D> IteratedHash<D> {
    /// Creates a placeholder with an explicit nonzero chain length.
    ///
    /// `iterations` is trusted, network-wide local configuration. It must not
    /// be decoded from an untrusted block or request, because output and
    /// verification both perform exactly that many digest calls.
    #[must_use]
    pub const fn new(iterations: NonZeroU64) -> Self {
        Self {
            iterations,
            digest: PhantomData,
        }
    }

    /// Returns the configured fixed chain length.
    #[must_use]
    pub const fn iterations(&self) -> NonZeroU64 {
        self.iterations
    }
}

impl<D: Digest32> Beacon for IteratedHash<D> {
    type Output = DevnetBeaconOutput;
    type Parent = PreparedParent;

    fn output(&self, parent: &Self::Parent) -> Self::Output {
        let mut output = *parent;
        for _ in 0..self.iterations.get() {
            output = D::digest(&output);
        }
        output
    }

    fn verify(&self, parent: &Self::Parent, output: &Self::Output) -> bool {
        self.output(parent) == *output
    }
}

#[cfg(test)]
mod tests {
    use super::{Digest32, IteratedHash, SECURITY_BANNER};
    use crate::Beacon;
    use core::num::NonZeroU64;

    struct RotateDigest;

    impl Digest32 for RotateDigest {
        fn digest(input: &[u8; 32]) -> [u8; 32] {
            let mut output = *input;
            output.rotate_left(1);
            output
        }
    }

    #[test]
    fn output_is_deterministic_and_verifiable() {
        let beacon = IteratedHash::<RotateDigest>::new(NonZeroU64::MIN);
        let parent = core::array::from_fn(|index| u8::try_from(index).expect("index fits in u8"));
        let mut expected = parent;
        expected.rotate_left(1);

        let output = beacon.output(&parent);

        assert_eq!(beacon.iterations(), NonZeroU64::MIN);
        assert_eq!(output, expected);
        assert_eq!(output, beacon.output(&parent));
        assert!(beacon.verify(&parent, &output));
    }

    #[test]
    fn verification_rejects_a_different_output() {
        let beacon = IteratedHash::<RotateDigest>::new(NonZeroU64::MIN);
        let parent = [0_u8; 32];
        let mut wrong_output = beacon.output(&parent);
        wrong_output[0] ^= 1;

        assert!(!beacon.verify(&parent, &wrong_output));
    }

    #[test]
    fn warning_is_exact() {
        assert_eq!(
            SECURITY_BANNER,
            "NOT-TESTNET-GRADE: sequentiality assumption only"
        );
    }
}
