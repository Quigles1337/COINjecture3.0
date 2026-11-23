//! CJ3 Encoding v1 grammar (Protocol Spec §1.1, ruling G0-C1).
//!
//! Integers are fixed width and little-endian. A fixed-length byte array is its bytes
//! with no length in front. A structure is its fields concatenated in layout order.
//! There are no tags, names, padding, optional fields, version bytes or checksums, so
//! no value has a second encoding and an input is valid only if it is exactly the
//! encoding of a value.

/// Why an input is not exactly the encoding of a value (Protocol Spec §1.1(e)).
///
/// Every decoder in the workspace reports one of these and never panics. When an input
/// is wrong in more than one way, the variant reported follows the order of checks
/// fixed in `loop/reports/C10-p007-builder.md` §1.5. That order is not a consensus
/// matter: every variant rejects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError {
    /// The input is shorter than required: it ends before a field is complete.
    TooShort {
        /// Bytes the next field or list needs.
        needed: u64,
        /// Bytes that remain.
        remaining: u64,
    },
    /// Bytes remain after the last field.
    TrailingBytes {
        /// Bytes left over.
        extra: u64,
    },
    /// A count exceeds its limit.
    CountExceedsLimit {
        /// The count read from the input.
        count: u64,
        /// The limit it must not exceed.
        limit: u64,
    },
    /// The whole input exceeds its limit.
    WholeExceedsLimit {
        /// Length of the whole input.
        length: u64,
        /// The limit it must not exceed.
        limit: u64,
    },
    /// The solution length is not the exact length its layout gives.
    SolutionLengthMismatch {
        /// The exact length the layout gives.
        expected: u64,
        /// The length found.
        actual: u64,
    },
    /// `class_id` names no registered class.
    UnknownClassId {
        /// The identifier read from the input.
        class_id: u16,
    },
}

/// Converts a length to the width used in [`DecodeError`] without a lossy cast.
#[must_use]
pub fn len_u64(len: usize) -> u64 {
    u64::try_from(len).unwrap_or(u64::MAX)
}

/// Reads fixed-width fields from the front of an input, in order.
///
/// Every read is checked: a field that does not fit is [`DecodeError::TooShort`], and
/// nothing is indexed or allocated.
#[derive(Clone, Copy, Debug)]
pub struct Reader<'a> {
    remaining: &'a [u8],
}

impl<'a> Reader<'a> {
    /// Starts reading at the first byte of `input`.
    #[must_use]
    pub const fn new(input: &'a [u8]) -> Self {
        Self { remaining: input }
    }

    /// Returns how many bytes have not been read.
    #[must_use]
    pub const fn remaining(&self) -> usize {
        self.remaining.len()
    }

    /// Reads the next `len` bytes.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::TooShort`] if fewer than `len` bytes remain.
    pub fn bytes(&mut self, len: usize) -> Result<&'a [u8], DecodeError> {
        match self.remaining.split_at_checked(len) {
            Some((field, rest)) => {
                self.remaining = rest;
                Ok(field)
            }
            None => Err(DecodeError::TooShort {
                needed: len_u64(len),
                remaining: len_u64(self.remaining.len()),
            }),
        }
    }

    /// Reads a fixed-length byte array `[N]`.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::TooShort`] if fewer than `N` bytes remain.
    pub fn array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        let field = self.bytes(N)?;
        <[u8; N]>::try_from(field).map_err(|_| DecodeError::TooShort {
            needed: len_u64(N),
            remaining: len_u64(field.len()),
        })
    }

    /// Reads a `u16`: two bytes, little-endian.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::TooShort`] if fewer than two bytes remain.
    pub fn u16(&mut self) -> Result<u16, DecodeError> {
        self.array().map(u16::from_le_bytes)
    }

    /// Reads a `u32`: four bytes, little-endian.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::TooShort`] if fewer than four bytes remain.
    pub fn u32(&mut self) -> Result<u32, DecodeError> {
        self.array().map(u32::from_le_bytes)
    }

    /// Reads a `u64`: eight bytes, little-endian.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::TooShort`] if fewer than eight bytes remain.
    pub fn u64(&mut self) -> Result<u64, DecodeError> {
        self.array().map(u64::from_le_bytes)
    }

    /// Ends the read after the last field.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::TrailingBytes`] if any byte has not been read.
    pub fn finish(self) -> Result<(), DecodeError> {
        if self.remaining.is_empty() {
            Ok(())
        } else {
            Err(DecodeError::TrailingBytes {
                extra: len_u64(self.remaining.len()),
            })
        }
    }
}

/// Writes fixed-width fields, in order, into an array of exactly `N` bytes.
///
/// The layouts that use it have a fixed length, so `N` and the field widths are all
/// compile-time constants. `finish` asserts in debug builds that exactly `N` bytes were
/// written; every layout is encoded by a unit test, so a layout that does not add up
/// fails the test suite rather than a release build.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArrayWriter<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> ArrayWriter<N> {
    pub(crate) const fn new() -> Self {
        Self {
            bytes: [0_u8; N],
            len: 0,
        }
    }

    pub(crate) fn bytes(mut self, field: &[u8]) -> Self {
        let end = self.len.saturating_add(field.len());
        if let Some(target) = self.bytes.get_mut(self.len..end) {
            target.copy_from_slice(field);
        }
        self.len = end;
        self
    }

    pub(crate) fn u16(self, value: u16) -> Self {
        self.bytes(&value.to_le_bytes())
    }

    pub(crate) fn u32(self, value: u32) -> Self {
        self.bytes(&value.to_le_bytes())
    }

    pub(crate) fn u64(self, value: u64) -> Self {
        self.bytes(&value.to_le_bytes())
    }

    pub(crate) fn finish(self) -> [u8; N] {
        debug_assert!(self.len == N, "a fixed layout must fill its array exactly");
        self.bytes
    }
}

/// Writes fixed-width fields, in order, into a growing buffer.
///
/// Used by the two layouts whose length is not a compile-time constant: the header,
/// whose size parameter is in the layout of its class, and the block.
#[derive(Clone, Debug)]
pub(crate) struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn bytes(mut self, field: &[u8]) -> Self {
        self.bytes.extend_from_slice(field);
        self
    }

    pub(crate) fn u16(self, value: u16) -> Self {
        self.bytes(&value.to_le_bytes())
    }

    pub(crate) fn u32(self, value: u32) -> Self {
        self.bytes(&value.to_le_bytes())
    }

    pub(crate) fn u64(self, value: u64) -> Self {
        self.bytes(&value.to_le_bytes())
    }

    pub(crate) fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{ArrayWriter, DecodeError, Reader};

    #[test]
    fn integers_are_fixed_width_little_endian() {
        let bytes = ArrayWriter::<14>::new()
            .u16(0x0102)
            .u32(0x0304_0506)
            .u64(0x0708_090a_0b0c_0d0e)
            .finish();
        assert_eq!(
            bytes,
            [
                0x02, 0x01, 0x06, 0x05, 0x04, 0x03, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07
            ]
        );

        let mut reader = Reader::new(&bytes);
        assert_eq!(reader.u16(), Ok(0x0102));
        assert_eq!(reader.u32(), Ok(0x0304_0506));
        assert_eq!(reader.u64(), Ok(0x0708_090a_0b0c_0d0e));
        assert_eq!(reader.finish(), Ok(()));
    }

    #[test]
    fn zero_is_the_all_zero_bytes_of_the_field_width() {
        let bytes = ArrayWriter::<14>::new().u16(0).u32(0).u64(0).finish();
        assert_eq!(bytes, [0_u8; 14]);
    }

    #[test]
    fn a_field_that_does_not_fit_is_too_short_and_consumes_nothing() {
        let mut reader = Reader::new(&[1, 2, 3]);
        assert_eq!(
            reader.u32(),
            Err(DecodeError::TooShort {
                needed: 4,
                remaining: 3
            })
        );
        assert_eq!(reader.remaining(), 3);
        assert_eq!(reader.u16(), Ok(0x0201));
        assert_eq!(
            reader.array::<2>(),
            Err(DecodeError::TooShort {
                needed: 2,
                remaining: 1
            })
        );
    }

    #[test]
    fn bytes_after_the_last_field_are_trailing() {
        let mut reader = Reader::new(&[0, 0, 9]);
        assert_eq!(reader.u16(), Ok(0));
        assert_eq!(
            reader.finish(),
            Err(DecodeError::TrailingBytes { extra: 1 })
        );
    }

    #[test]
    fn a_fixed_array_has_no_length_in_front() {
        let mut reader = Reader::new(&[7, 8, 9]);
        assert_eq!(reader.array::<3>(), Ok([7, 8, 9]));
        assert_eq!(reader.finish(), Ok(()));
    }
}
