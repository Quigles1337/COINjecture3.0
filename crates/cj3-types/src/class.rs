//! The class identifier table and the size-parameter layouts (ruling G0-C3b, G0-C3c).
//!
//! This module owns bytes only. Whether a tuple is a valid SIS tuple, how an instance
//! is derived from it and how a solution is checked are `cj3-classes` matters.
//!
//! Encoding v1 has one registered class. The statement that every preimage under one
//! tag has the same length (§1.2, ruling G0-C2d) holds for that one class. A second
//! class is admitted only by an ADR, and that ADR must show that header preimages stay
//! unambiguous (P-007 resume ruling, clause 2.1).

use crate::codec::{ArrayWriter, DecodeError, Reader};

/// A registered problem class.
///
/// SIS is 1. 0 is never assigned.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ClassId {
    /// Short Integer Solution, the genesis class.
    Sis,
}

impl ClassId {
    /// Looks up the class that a `class_id` field names.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::UnknownClassId`] if `class_id` names no registered class.
    pub const fn from_u16(class_id: u16) -> Result<Self, DecodeError> {
        match class_id {
            1 => Ok(Self::Sis),
            _ => Err(DecodeError::UnknownClassId { class_id }),
        }
    }

    /// Returns the value of the `class_id` field for this class.
    #[must_use]
    pub const fn to_u16(self) -> u16 {
        match self {
            Self::Sis => 1,
        }
    }
}

/// The SIS size parameter as its layout carries it: 20 bytes, `n u32`, `m u32`,
/// `q u32`, `beta_squared u64`.
///
/// This is a layout, not a validated tuple. Any four values encode and decode.
/// `cj3-classes` decides whether they form a valid SIS tuple.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SisSizeParam {
    /// Rows of the matrix.
    pub n: u32,
    /// Columns of the matrix, and coefficients in a solution.
    pub m: u32,
    /// The modulus.
    pub q: u32,
    /// The squared Euclidean bound.
    pub beta_squared: u64,
}

impl SisSizeParam {
    /// Length of the layout.
    pub const ENCODED_LEN: usize = 20;

    /// Encodes the four fields in layout order.
    #[must_use]
    pub fn encode(&self) -> [u8; Self::ENCODED_LEN] {
        ArrayWriter::<{ Self::ENCODED_LEN }>::new()
            .u32(self.n)
            .u32(self.m)
            .u32(self.q)
            .u64(self.beta_squared)
            .finish()
    }

    /// Decodes an input that is exactly the 20-byte layout.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::TooShort`] or [`DecodeError::TrailingBytes`].
    pub fn decode(input: &[u8]) -> Result<Self, DecodeError> {
        let mut reader = Reader::new(input);
        let value = Self::read(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }

    pub(crate) fn read(reader: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(Self {
            n: reader.u32()?,
            m: reader.u32()?,
            q: reader.u32()?,
            beta_squared: reader.u64()?,
        })
    }

    /// Length in bytes of a solution under this size parameter: `2 * m`.
    ///
    /// The block layout needs this length (§6.3). The coefficient coding itself lives
    /// with the class, in `cj3-classes`, which takes its length from here.
    #[must_use]
    pub fn solution_len(&self) -> u64 {
        2 * u64::from(self.m)
    }
}

/// A header's size parameter, in the layout of the class that `class_id` names.
///
/// The class identifier is not stored beside it: it is the variant. A header value
/// therefore cannot carry a class identifier that disagrees with its layout.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SizeParam {
    /// The 20-byte SIS layout.
    Sis(SisSizeParam),
}

impl SizeParam {
    /// The class whose layout this is.
    #[must_use]
    pub const fn class_id(&self) -> ClassId {
        match self {
            Self::Sis(_) => ClassId::Sis,
        }
    }

    /// Length in bytes of this size parameter in a header.
    #[must_use]
    pub const fn encoded_len(&self) -> usize {
        match self {
            Self::Sis(_) => SisSizeParam::ENCODED_LEN,
        }
    }

    /// Length in bytes of a solution under this size parameter.
    ///
    /// For the active tuple this is `SOL_MAX_BYTES` (ruling G0-C6b).
    #[must_use]
    pub fn solution_len(&self) -> u64 {
        match self {
            Self::Sis(size) => size.solution_len(),
        }
    }

    pub(crate) fn read(class_id: ClassId, reader: &mut Reader<'_>) -> Result<Self, DecodeError> {
        match class_id {
            ClassId::Sis => SisSizeParam::read(reader).map(Self::Sis),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClassId, SisSizeParam, SizeParam};
    use crate::codec::DecodeError;

    #[test]
    fn sis_is_one_and_zero_is_never_assigned() {
        assert_eq!(ClassId::Sis.to_u16(), 1);
        assert_eq!(ClassId::from_u16(1), Ok(ClassId::Sis));
        assert_eq!(
            ClassId::from_u16(0),
            Err(DecodeError::UnknownClassId { class_id: 0 })
        );
    }

    #[test]
    fn every_other_identifier_names_no_registered_class() {
        for class_id in (0..=u16::MAX).filter(|class_id| *class_id != 1) {
            assert_eq!(
                ClassId::from_u16(class_id),
                Err(DecodeError::UnknownClassId { class_id })
            );
        }
    }

    #[test]
    fn size_parameter_layout_is_twenty_bytes_in_field_order() {
        let size = SisSizeParam {
            n: 0x0403_0201,
            m: 0x0807_0605,
            q: 0x0c0b_0a09,
            beta_squared: 0x1413_1211_100f_0e0d,
        };
        let bytes = size.encode();
        let expected: Vec<u8> = (1..=20).collect();
        assert_eq!(bytes.as_slice(), expected.as_slice());
        assert_eq!(SisSizeParam::decode(&bytes), Ok(size));
    }

    #[test]
    fn size_parameter_decode_is_exact() {
        let bytes = [0_u8; 21];
        assert_eq!(
            SisSizeParam::decode(&bytes[..19]),
            Err(DecodeError::TooShort {
                needed: 8,
                remaining: 7
            })
        );
        assert!(SisSizeParam::decode(&bytes[..20]).is_ok());
        assert_eq!(
            SisSizeParam::decode(&bytes),
            Err(DecodeError::TrailingBytes { extra: 1 })
        );
    }

    #[test]
    fn solution_length_is_twice_m_without_overflow() {
        let size = SisSizeParam {
            n: 1,
            m: u32::MAX,
            q: 2,
            beta_squared: 1,
        };
        assert_eq!(size.solution_len(), 2 * u64::from(u32::MAX));
        let param = SizeParam::Sis(size);
        assert_eq!(param.solution_len(), 8_589_934_590);
        assert_eq!(param.encoded_len(), 20);
        assert_eq!(param.class_id(), ClassId::Sis);
    }
}
