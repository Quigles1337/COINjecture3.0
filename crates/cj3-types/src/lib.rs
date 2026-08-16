//! Canonical shared-type boundary.
//!
//! P-007 currently admits only the representation-independent checked amount
//! boundary. Canonical codecs, domain-separation bytes, and `addr()` remain absent
//! until their G0/HUMAN semantics are ratified; this crate must not make those
//! choices by convention.

#![forbid(unsafe_code)]

/// An integer monetary amount.
///
/// The inner `u64` is deliberately opaque so value-changing code goes through
/// [`Self::checked_add`] or [`Self::checked_sub`]. This type does not define a wire
/// encoding; converting a value for a future canonical codec is a separate P-007/G0
/// concern.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Amount(u64);

impl Amount {
    /// The additive identity.
    pub const ZERO: Self = Self(0);

    /// Wraps an already validated unsigned amount.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying integer without performing arithmetic.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Adds two amounts without wrapping or saturating.
    ///
    /// # Errors
    ///
    /// Returns [`AmountError::Overflow`] if the exact sum cannot be represented as a
    /// `u64`.
    pub const fn checked_add(self, rhs: Self) -> Result<Self, AmountError> {
        match self.0.checked_add(rhs.0) {
            Some(value) => Ok(Self(value)),
            None => Err(AmountError::Overflow),
        }
    }

    /// Subtracts one amount from another without wrapping or saturating.
    ///
    /// # Errors
    ///
    /// Returns [`AmountError::Underflow`] if `rhs` is greater than `self`.
    pub const fn checked_sub(self, rhs: Self) -> Result<Self, AmountError> {
        match self.0.checked_sub(rhs.0) {
            Some(value) => Ok(Self(value)),
            None => Err(AmountError::Underflow),
        }
    }
}

/// A checked amount operation could not be represented exactly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AmountError {
    /// Addition exceeded `u64::MAX`.
    Overflow,
    /// Subtraction would have produced a negative value.
    Underflow,
}

#[cfg(test)]
mod tests {
    use super::{Amount, AmountError};

    #[test]
    fn preserves_the_complete_u64_value_domain() {
        assert_eq!(Amount::ZERO.get(), 0);
        assert_eq!(Amount::new(u64::MAX).get(), u64::MAX);
    }

    #[test]
    fn checked_add_returns_exact_results_or_overflow() {
        assert_eq!(
            Amount::new(19).checked_add(Amount::new(23)),
            Ok(Amount::new(42))
        );
        assert_eq!(
            Amount::new(u64::MAX).checked_add(Amount::new(1)),
            Err(AmountError::Overflow)
        );
    }

    #[test]
    fn checked_sub_returns_exact_results_or_underflow() {
        assert_eq!(
            Amount::new(42).checked_sub(Amount::new(23)),
            Ok(Amount::new(19))
        );
        assert_eq!(
            Amount::ZERO.checked_sub(Amount::new(1)),
            Err(AmountError::Underflow)
        );
    }

    #[test]
    fn representative_successful_operations_match_u64_arithmetic() {
        let samples = [0, 1, 2, u64::from(u32::MAX), u64::MAX - 1, u64::MAX];

        for lhs in samples {
            for rhs in samples {
                assert_eq!(
                    Amount::new(lhs)
                        .checked_add(Amount::new(rhs))
                        .map(Amount::get),
                    lhs.checked_add(rhs).ok_or(AmountError::Overflow)
                );
                assert_eq!(
                    Amount::new(lhs)
                        .checked_sub(Amount::new(rhs))
                        .map(Amount::get),
                    lhs.checked_sub(rhs).ok_or(AmountError::Underflow)
                );
            }
        }
    }
}
