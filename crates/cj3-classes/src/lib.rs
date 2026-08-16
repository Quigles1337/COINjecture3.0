//! Deterministic problem-class and checker boundary.
//!
//! P-003 implements the sampled-SIS class without assigning its still-unratified
//! numeric registry identifier. Callers bind an identifier through [`sis::Sis`]'s
//! const generic only after the registry value is ratified.

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
/// `SizeParam` is associated so a class can expose a validated parameter object
/// without introducing a cross-class wire representation before P-007.
pub trait ProblemClass {
    /// Registry identifier, bound only after the owning gate assigns it.
    const CLASS_ID: u16;

    /// Validated parameters used to derive an instance.
    type SizeParam: Copy;
    /// Deterministically derived instance.
    type Instance;
    /// Strictly decoded solution supplied by the P-007 codec boundary.
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
