//! Deterministic problem-class and checker boundary.
//!
//! `cj3-types` owns bytes and this crate owns meaning: derivation, checking and the
//! validity rules of a tuple. It imports the layouts and the class identifier table
//! from `cj3-types` and defines no layout of its own except the coefficient coding of
//! a SIS solution, which lives with the class.
//!
//! The sampled-SIS class is bound to class identifier 1 (Protocol Spec §5.1, ruling
//! G0-C3b). [`sis::Sis`] reads the identifier from `cj3-types`; the number is written
//! nowhere in this crate.

#![forbid(unsafe_code)]

pub mod sis;

/// Fixed-point denominator used by every problem-class quality value.
pub const QUALITY_SCALE: u64 = 1_000_000;

/// Integer-only quality in [`QUALITY_SCALE`] units.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Quality(u64);

impl Quality {
    /// Returns the fixed-point integer value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    const fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

/// Reasons a decoded solution fails deterministic class validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Invalid {
    /// The decoded vector length differs from the instance dimension.
    WrongLength {
        /// Required vector length.
        expected: usize,
        /// Supplied vector length.
        actual: usize,
    },
    /// The all-zero vector is never a solution.
    ZeroVector,
    /// A coefficient alone exceeds the global squared-norm bound.
    CoefficientOutOfRange {
        /// Index of the first offending coefficient.
        index: usize,
    },
    /// Squared-norm accumulation could not be represented exactly.
    NormOverflow,
    /// The vector's squared norm exceeds the admitted bound.
    NormTooLarge,
    /// A matrix-vector arithmetic step could not be represented exactly.
    RelationOverflow,
    /// A row of the modular relation is nonzero.
    ModularRelation {
        /// Index of the first nonzero row.
        row: usize,
    },
    /// Fixed-point quality could not be represented as a `u64`.
    QualityOverflow,
}

/// Static contract implemented by an admitted useful-work problem class.
///
/// The checker deliberately has no receiver, clock, miner field, or metadata input.
/// `SizeParam` is associated so a class exposes a validated parameter object; its
/// bytes in a header are the layout that `cj3-types` gives the class.
pub trait ProblemClass {
    /// Registry identifier, as the class identifier table of `cj3-types` assigns it.
    const CLASS_ID: u16;

    /// Validated parameters used to derive an instance.
    type SizeParam: Copy;
    /// Deterministically derived instance.
    type Instance;
    /// Strictly decoded solution, produced by the class's own solution codec.
    type Solution;

    /// Derives an instance solely from committed seed bytes and validated parameters.
    fn derive_instance(seed: [u8; 32], size: Self::SizeParam) -> Self::Instance;

    /// Checks one decoded solution using bounded, integer-only deterministic work.
    ///
    /// # Errors
    ///
    /// Returns [`Invalid`] when the decoded solution violates any class rule or an
    /// arithmetic step cannot be represented exactly.
    fn check(inst: &Self::Instance, sol: &Self::Solution) -> Result<Quality, Invalid>;
}
