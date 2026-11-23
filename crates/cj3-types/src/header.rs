//! The header layout (Protocol Spec §6.1, ruling G0-C3d).
//!
//! The header is its fields concatenated in order: 306 bytes with the SIS anchor.
//! `size_param` is in the layout of the class that `class_id` names. `tx_root` and
//! `state_root` are carried as 32 opaque bytes; how they are built is ruled at the
//! P-101 FRAME. `hash_target` is 32 opaque bytes here; the byte order in which it is
//! read as a number is ruled at the first Phase 2 FRAME. The unit of `timestamp` is
//! ruled together with P-1 and P-10.
//!
//! Decoding a header says nothing about whether it is valid: block rules B2–B12 belong
//! to consensus.

use crate::{
    class::{ClassId, SizeParam},
    codec::{DecodeError, Reader, Writer},
    hash::{self, Digest},
    keys::Address,
};

/// Bytes of a header outside its size parameter: 110 before it and 176 after it.
const FIXED_LEN: usize = 286;

/// A block header as its layout carries it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    /// The network this header is for.
    pub network_id: u32,
    /// Height of the block.
    pub height: u64,
    /// Header hash of the parent.
    pub parent_hash: Digest,
    /// Beacon output for this block.
    pub beacon_out: Digest,
    /// The derived instance seed.
    pub instance_seed: Digest,
    /// The size parameter. Its variant is the header's `class_id`.
    pub size_param: SizeParam,
    /// The hash target, as 32 opaque bytes.
    pub hash_target: [u8; 32],
    /// The transaction root, as 32 opaque bytes.
    pub tx_root: [u8; 32],
    /// The state root, as 32 opaque bytes.
    pub state_root: [u8; 32],
    /// The solution commitment.
    pub sol_commit: Digest,
    /// The timestamp. Its unit is not yet ruled.
    pub timestamp: u64,
    /// The miner's address.
    pub miner_addr: Address,
    /// The hash-race nonce.
    pub nonce: u64,
}

impl Header {
    /// The header's `class_id`: the class whose layout `size_param` is in.
    #[must_use]
    pub const fn class_id(&self) -> ClassId {
        self.size_param.class_id()
    }

    /// Length of this header's encoding: 306 bytes with the SIS anchor.
    #[must_use]
    pub const fn encoded_len(&self) -> usize {
        FIXED_LEN + self.size_param.encoded_len()
    }

    /// Encodes the fields in layout order.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let writer = Writer::with_capacity(self.encoded_len())
            .u32(self.network_id)
            .u64(self.height)
            .bytes(&self.parent_hash)
            .bytes(&self.beacon_out)
            .bytes(&self.instance_seed)
            .u16(self.class_id().to_u16());
        let writer = match &self.size_param {
            SizeParam::Sis(size) => writer.bytes(&size.encode()),
        };
        writer
            .bytes(&self.hash_target)
            .bytes(&self.tx_root)
            .bytes(&self.state_root)
            .bytes(&self.sol_commit)
            .u64(self.timestamp)
            .bytes(self.miner_addr.as_bytes())
            .u64(self.nonce)
            .finish()
    }

    /// Decodes an input that is exactly one header.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::TooShort`] for the first field that cannot be read,
    /// [`DecodeError::UnknownClassId`] when `class_id` names no registered class, and
    /// [`DecodeError::TrailingBytes`] when bytes remain after `nonce`.
    pub fn decode(input: &[u8]) -> Result<Self, DecodeError> {
        let mut reader = Reader::new(input);
        let value = Self::read(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }

    pub(crate) fn read(reader: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let network_id = reader.u32()?;
        let height = reader.u64()?;
        let parent_hash = reader.array()?;
        let beacon_out = reader.array()?;
        let instance_seed = reader.array()?;
        // `class_id` is judged as soon as it is read, before the size parameter.
        let class_id = ClassId::from_u16(reader.u16()?)?;
        let size_param = SizeParam::read(class_id, reader)?;
        Ok(Self {
            network_id,
            height,
            parent_hash,
            beacon_out,
            instance_seed,
            size_param,
            hash_target: reader.array()?,
            tx_root: reader.array()?,
            state_root: reader.array()?,
            sol_commit: reader.array()?,
            timestamp: reader.u64()?,
            miner_addr: Address::from_bytes(reader.array()?),
            nonce: reader.u64()?,
        })
    }

    /// The preimage of this header's hash: `D_HDR ‖ Header` (§1.2).
    #[must_use]
    pub fn hash_preimage(&self) -> Vec<u8> {
        hash::header_hash_preimage(self)
    }

    /// This header's hash (§1.2).
    #[must_use]
    pub fn hash(&self) -> Digest {
        hash::header_hash(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Header;
    use crate::{
        class::{ClassId, SisSizeParam, SizeParam},
        codec::DecodeError,
        domain::D_HDR,
        keys::Address,
    };

    /// A header whose every byte is its own offset modulo 251, except `class_id`.
    ///
    /// The size parameter it carries is whatever those bytes spell. It is not a tuple
    /// and stands in for none.
    fn patterned_bytes() -> Vec<u8> {
        let mut bytes = (0..306_u32)
            .map(|offset| u8::try_from(offset % 251).expect("below 251"))
            .collect::<Vec<_>>();
        bytes[108] = 1;
        bytes[109] = 0;
        bytes
    }

    #[test]
    fn layout_is_306_bytes_with_the_sis_anchor() {
        let bytes = patterned_bytes();
        let header = Header::decode(&bytes).expect("patterned header decodes");
        assert_eq!(header.encoded_len(), 306);
        assert_eq!(header.encode(), bytes);
        assert_eq!(header.class_id(), ClassId::Sis);
    }

    #[test]
    fn every_field_sits_at_its_position() {
        let bytes = patterned_bytes();
        let header = Header::decode(&bytes).expect("patterned header decodes");
        assert_eq!(header.network_id.to_le_bytes(), bytes[0..4]);
        assert_eq!(header.height.to_le_bytes(), bytes[4..12]);
        assert_eq!(header.parent_hash, bytes[12..44]);
        assert_eq!(header.beacon_out, bytes[44..76]);
        assert_eq!(header.instance_seed, bytes[76..108]);
        assert_eq!(header.class_id().to_u16().to_le_bytes(), bytes[108..110]);
        let SizeParam::Sis(size) = header.size_param;
        assert_eq!(size.encode(), bytes[110..130]);
        assert_eq!(header.hash_target, bytes[130..162]);
        assert_eq!(header.tx_root, bytes[162..194]);
        assert_eq!(header.state_root, bytes[194..226]);
        assert_eq!(header.sol_commit, bytes[226..258]);
        assert_eq!(header.timestamp.to_le_bytes(), bytes[258..266]);
        assert_eq!(header.miner_addr.as_bytes(), &bytes[266..298]);
        assert_eq!(header.nonce.to_le_bytes(), bytes[298..306]);
    }

    #[test]
    fn encode_then_decode_returns_the_value() {
        let header = Header {
            network_id: 7,
            height: 9,
            parent_hash: [1; 32],
            beacon_out: [2; 32],
            instance_seed: [3; 32],
            size_param: SizeParam::Sis(SisSizeParam {
                n: 0,
                m: 0,
                q: 0,
                beta_squared: 0,
            }),
            hash_target: [4; 32],
            tx_root: [5; 32],
            state_root: [6; 32],
            sol_commit: [7; 32],
            timestamp: u64::MAX,
            miner_addr: Address::from_bytes([8; 32]),
            nonce: 0,
        };
        assert_eq!(Header::decode(&header.encode()), Ok(header));
    }

    #[test]
    fn header_length_holds_at_the_boundary_one_below_and_one_above() {
        let mut bytes = patterned_bytes();
        assert!(Header::decode(&bytes).is_ok());
        assert_eq!(
            Header::decode(&bytes[..305]),
            Err(DecodeError::TooShort {
                needed: 8,
                remaining: 7
            })
        );
        bytes.push(0);
        assert_eq!(
            Header::decode(&bytes),
            Err(DecodeError::TrailingBytes { extra: 1 })
        );
    }

    #[test]
    fn an_unregistered_class_id_is_rejected_before_the_size_parameter_is_read() {
        let mut bytes = patterned_bytes();
        for class_id in [0_u16, 2, 0x0100, u16::MAX] {
            bytes[108..110].copy_from_slice(&class_id.to_le_bytes());
            assert_eq!(
                Header::decode(&bytes),
                Err(DecodeError::UnknownClassId { class_id })
            );
            // Truncated just after `class_id`: the class is still the error reported.
            assert_eq!(
                Header::decode(&bytes[..110]),
                Err(DecodeError::UnknownClassId { class_id })
            );
        }
    }

    #[test]
    fn every_shorter_input_is_too_short() {
        let bytes = patterned_bytes();
        for len in 0..306 {
            assert!(matches!(
                Header::decode(&bytes[..len]),
                Err(DecodeError::TooShort { .. })
            ));
        }
    }

    #[test]
    fn hash_preimage_is_the_tag_then_the_306_header_bytes() {
        let bytes = patterned_bytes();
        let header = Header::decode(&bytes).expect("patterned header decodes");
        let preimage = header.hash_preimage();
        assert_eq!(preimage.len(), 322);
        assert_eq!(&preimage[..16], &D_HDR);
        assert_eq!(&preimage[16..], bytes.as_slice());
    }

    #[test]
    fn every_header_byte_changes_the_header_hash() {
        let bytes = patterned_bytes();
        let base = Header::decode(&bytes)
            .expect("patterned header decodes")
            .hash();
        for offset in (0..306).filter(|offset| !(108..110).contains(offset)) {
            let mut changed = bytes.clone();
            changed[offset] ^= 0x80;
            let header = Header::decode(&changed).expect("still a header");
            assert_ne!(header.hash(), base, "offset {offset}");
        }
    }
}
