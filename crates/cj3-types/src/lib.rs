//! Canonical shared-type boundary: the bytes of CJ3 Encoding v1.
//!
//! This crate owns bytes, and `cj3-classes` owns meaning. Here live the grammar
//! ([`codec`]), the seven domain tags ([`domain`]), `H` and every preimage of Protocol
//! Spec §1.2 ([`hash`]), the single [`addr`], the transaction, header and block layouts
//! ([`tx`], [`header`], [`block`]), the class identifier table and the 20-byte SIS
//! size-parameter layout ([`class`]), the Ed25519 verification of §1.3 ([`ed25519`]),
//! and the checked [`Amount`].
//!
//! Nothing here is a protocol parameter that a ruling has left unfilled. No SIS tuple
//! is compiled in as active: every codec that needs one takes it as an argument. No
//! network identifier, fee floor or timestamp unit is named. `BLOCK_MAX_BYTES` is the
//! provisional figure of ruling G0-C6c and is labeled so.

#![forbid(unsafe_code)]

pub mod block;
pub mod class;
pub mod codec;
pub mod domain;
pub mod ed25519;
pub mod hash;
pub mod header;
pub mod keys;
pub mod tx;

pub use block::{BLOCK_MAX_BYTES, Block, EncodeError};
pub use class::{ClassId, SisSizeParam, SizeParam};
pub use codec::DecodeError;
pub use ed25519::SignatureError;
pub use hash::{Digest, addr};
pub use header::Header;
pub use keys::{Address, PublicKey, Signature, SigningDigest};
pub use tx::{TX_MAX_BYTES, Transaction};

#[cfg(test)]
pub(crate) mod test_support {
    use core::fmt::Write as _;

    /// Lowercase hex of `bytes`.
    pub(crate) fn hex(bytes: &[u8]) -> String {
        bytes.iter().fold(String::new(), |mut text, byte| {
            write!(text, "{byte:02x}").expect("writing to a String cannot fail");
            text
        })
    }

    /// The `N` bytes that `text` spells in hex.
    pub(crate) fn unhex<const N: usize>(text: &str) -> [u8; N] {
        assert_eq!(text.len(), 2 * N, "hex length");
        let mut out = [0_u8; N];
        for (index, byte) in out.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&text[2 * index..2 * index + 2], 16).expect("hex digit");
        }
        out
    }
}

/// An integer monetary amount.
///
/// The inner `u64` is deliberately opaque so value-changing code goes through
/// [`Self::checked_add`] or [`Self::checked_sub`]. In CJ3 Encoding v1 an amount is a
/// `u64` field: eight bytes, little-endian (Protocol Spec §1.1(a), §7).
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
