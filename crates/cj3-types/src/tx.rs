//! The transaction layout (Protocol Spec §7, ruling G0-C3a).
//!
//! A transaction is 188 bytes: `network_id u32`, `from [32]`, `to [32]`, `amount u64`,
//! `fee u64`, `nonce u64`, `pubkey [32]`, `sig [64]`. `pubkey` is the RFC 8032
//! public-key encoding, and `sig` is `R` then `S`.
//!
//! This module owns the bytes of a transaction and its signature check. The validity
//! rules V1–V9 as a whole belong to the kernel, which conforms to the Lean
//! specification; nothing here reads state.

use crate::{
    Amount,
    codec::{ArrayWriter, DecodeError, Reader, len_u64},
    ed25519::{self, SignatureError},
    hash::{self, SIGNING_PREIMAGE_LEN},
    keys::{Address, PublicKey, Signature, SigningDigest},
};

/// `TX_MAX_BYTES` (P-8, ruling G0-C6a): the exact size of a transaction, so V1's
/// bound and exact decode agree.
pub const TX_MAX_BYTES: usize = 188;

/// A transaction as its 188 bytes carry it.
///
/// Every 188-byte input decodes to exactly one value and every value encodes to
/// exactly one 188-byte string. `pubkey` and `sig` are opaque here; a key that is not
/// a point fails the signature check (V2), not the decoder (V1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Transaction {
    /// The network this transaction is for.
    pub network_id: u32,
    /// The paying account.
    pub from: Address,
    /// The receiving account.
    pub to: Address,
    /// The amount moved.
    pub amount: Amount,
    /// The fee paid.
    pub fee: Amount,
    /// The sender's account nonce.
    pub nonce: u64,
    /// The signer's public key.
    pub pubkey: PublicKey,
    /// The signature, `R` then `S`.
    pub sig: Signature,
}

impl Transaction {
    /// Length of the layout. It equals [`TX_MAX_BYTES`].
    pub const ENCODED_LEN: usize = TX_MAX_BYTES;

    /// Encodes the eight fields in layout order.
    #[must_use]
    pub fn encode(&self) -> [u8; Self::ENCODED_LEN] {
        ArrayWriter::<{ Self::ENCODED_LEN }>::new()
            .u32(self.network_id)
            .bytes(self.from.as_bytes())
            .bytes(self.to.as_bytes())
            .u64(self.amount.get())
            .u64(self.fee.get())
            .u64(self.nonce)
            .bytes(self.pubkey.as_bytes())
            .bytes(self.sig.as_bytes())
            .finish()
    }

    /// Decodes an input that is exactly the 188-byte layout.
    ///
    /// The whole length is tested against [`TX_MAX_BYTES`] first, as the Lean `v1`
    /// tests the size before it decodes.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::WholeExceedsLimit`] for an input longer than
    /// [`TX_MAX_BYTES`] and [`DecodeError::TooShort`] for a shorter one.
    pub fn decode(input: &[u8]) -> Result<Self, DecodeError> {
        if input.len() > TX_MAX_BYTES {
            return Err(DecodeError::WholeExceedsLimit {
                length: len_u64(input.len()),
                limit: len_u64(TX_MAX_BYTES),
            });
        }
        let mut reader = Reader::new(input);
        let value = Self::read(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }

    pub(crate) fn read(reader: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(Self {
            network_id: reader.u32()?,
            from: Address::from_bytes(reader.array()?),
            to: Address::from_bytes(reader.array()?),
            amount: Amount::new(reader.u64()?),
            fee: Amount::new(reader.u64()?),
            nonce: reader.u64()?,
            pubkey: PublicKey::from_bytes(reader.array()?),
            sig: Signature::from_bytes(reader.array()?),
        })
    }

    /// The 108-byte preimage of this transaction's signing digest (§1.2).
    #[must_use]
    pub fn signing_preimage(&self) -> [u8; SIGNING_PREIMAGE_LEN] {
        hash::signing_preimage(
            self.network_id,
            &self.from,
            &self.to,
            self.amount,
            self.fee,
            self.nonce,
        )
    }

    /// This transaction's signing digest: the 32-byte Ed25519 message (§1.2).
    #[must_use]
    pub fn signing_digest(&self) -> SigningDigest {
        hash::signing_digest(
            self.network_id,
            &self.from,
            &self.to,
            self.amount,
            self.fee,
            self.nonce,
        )
    }

    /// Verifies `sig` under `pubkey` over the signing digest, exactly as §1.3 states.
    ///
    /// This is the signature half of V2. It does not test `from == addr(pubkey)`; that
    /// is V3, a separate check at the validation site.
    ///
    /// # Errors
    ///
    /// Returns the [`SignatureError`] of the first §1.3 clause that rejects.
    pub fn verify_signature(&self) -> Result<(), SignatureError> {
        ed25519::verify(&self.pubkey, &self.sig, self.signing_digest().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::{TX_MAX_BYTES, Transaction};
    use crate::{
        Amount,
        codec::DecodeError,
        keys::{Address, PublicKey, Signature},
    };

    fn sample() -> Transaction {
        Transaction {
            network_id: 0x0403_0201,
            from: Address::from_bytes([0x11; 32]),
            to: Address::from_bytes([0x22; 32]),
            amount: Amount::new(0x0c0b_0a09_0807_0605),
            fee: Amount::new(0x1413_1211_100f_0e0d),
            nonce: 0x1c1b_1a19_1817_1615,
            pubkey: PublicKey::from_bytes([0x33; 32]),
            sig: Signature::from_bytes([0x44; 64]),
        }
    }

    #[test]
    fn layout_is_188_bytes_with_every_field_at_its_position() {
        let bytes = sample().encode();
        assert_eq!(bytes.len(), 188);
        assert_eq!(TX_MAX_BYTES, 188);
        assert_eq!(&bytes[0..4], &[1, 2, 3, 4]);
        assert_eq!(&bytes[4..36], &[0x11; 32]);
        assert_eq!(&bytes[36..68], &[0x22; 32]);
        assert_eq!(&bytes[68..76], &[5, 6, 7, 8, 9, 10, 11, 12]);
        assert_eq!(&bytes[76..84], &[13, 14, 15, 16, 17, 18, 19, 20]);
        assert_eq!(&bytes[84..92], &[21, 22, 23, 24, 25, 26, 27, 28]);
        assert_eq!(&bytes[92..124], &[0x33; 32]);
        assert_eq!(&bytes[124..188], &[0x44; 64]);
    }

    #[test]
    fn encode_then_decode_returns_the_value() {
        let value = sample();
        assert_eq!(Transaction::decode(&value.encode()), Ok(value));
    }

    #[test]
    fn decode_then_encode_returns_the_input_for_every_byte_pattern() {
        for fill in [0x00_u8, 0x01, 0x7f, 0x80, 0xff] {
            let input = [fill; 188];
            let value = Transaction::decode(&input).expect("any 188 bytes decode");
            assert_eq!(value.encode(), input);
        }
    }

    #[test]
    fn the_size_limit_holds_at_the_boundary_one_below_and_one_above() {
        let bytes = [0x5a_u8; 189];
        assert!(Transaction::decode(&bytes[..188]).is_ok());
        assert_eq!(
            Transaction::decode(&bytes[..187]),
            Err(DecodeError::TooShort {
                needed: 64,
                remaining: 63
            })
        );
        assert_eq!(
            Transaction::decode(&bytes),
            Err(DecodeError::WholeExceedsLimit {
                length: 189,
                limit: 188
            })
        );
    }

    #[test]
    fn every_shorter_input_is_too_short() {
        let bytes = [0_u8; 188];
        for len in 0..188 {
            assert!(matches!(
                Transaction::decode(&bytes[..len]),
                Err(DecodeError::TooShort { .. })
            ));
        }
    }

    #[test]
    fn signing_preimage_uses_the_transactions_own_network_id() {
        let mut value = sample();
        let first = value.signing_preimage();
        value.network_id ^= 1;
        let second = value.signing_preimage();
        assert_ne!(first[16..20], second[16..20]);
        assert_eq!(first[20..], second[20..]);
        assert_ne!(
            sample().signing_digest().as_bytes(),
            value.signing_digest().as_bytes()
        );
    }

    #[test]
    fn public_key_and_signature_are_not_inputs_to_the_signing_digest() {
        let mut value = sample();
        let digest = value.signing_digest();
        value.pubkey = PublicKey::from_bytes([0x99; 32]);
        value.sig = Signature::from_bytes([0x98; 64]);
        assert_eq!(value.signing_digest(), digest);
    }
}
