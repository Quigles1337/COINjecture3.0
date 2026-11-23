//! `H` and the preimages of Protocol Spec §1.2 (ruling G0-C2).
//!
//! `H` is SHA-256. A preimage is its domain tag, then fixed-width fields in the order
//! given, each encoded by §1.1. Under one tag every preimage has the same length and
//! the same field positions, and preimages under different tags differ in their first
//! 16 bytes, so two different inputs never give the same preimage.
//!
//! Every tagged preimage in the workspace is built here and nowhere else. In
//! particular [`addr`] is the single address derivation: no component may re-derive an
//! address locally (§2).

use sha2::{Digest as _, Sha256};

use crate::{
    Amount,
    class::ClassId,
    codec::{ArrayWriter, Writer},
    domain::{D_ADDR, D_BEACON, D_HDR, D_INST, D_SOL, D_TX, TAG_LEN},
    header::Header,
    keys::{Address, PublicKey, SigningDigest},
};

/// A 32-byte output of `H`.
pub type Digest = [u8; 32];

/// Length of the `addr(pk)` preimage.
pub const ADDR_PREIMAGE_LEN: usize = 48;
/// Length of the signing-digest preimage.
pub const SIGNING_PREIMAGE_LEN: usize = 108;
/// Length of the `instance_seed` preimage.
pub const INSTANCE_SEED_PREIMAGE_LEN: usize = 92;
/// Length of the `sol_commit` preimage.
pub const SOL_COMMIT_PREIMAGE_LEN: usize = 114;
/// Length of the beacon-input preimage.
pub const BEACON_INPUT_PREIMAGE_LEN: usize = 48;

/// `H`: SHA-256.
fn h(preimage: &[u8]) -> Digest {
    Sha256::digest(preimage).into()
}

/// The 48-byte preimage of `addr(pk)`: `D_ADDR ‖ pk[32]`.
#[must_use]
pub fn addr_preimage(public_key: &PublicKey) -> [u8; ADDR_PREIMAGE_LEN] {
    ArrayWriter::<ADDR_PREIMAGE_LEN>::new()
        .bytes(&D_ADDR)
        .bytes(public_key.as_bytes())
        .finish()
}

/// `addr(pk) = H(D_ADDR ‖ pk[32])`, the whole digest.
///
/// This is the single address derivation of Protocol Spec §2. It takes the public key
/// bytes as they are; whether they decode as a point is judged by the signature
/// verification, not here.
#[must_use]
pub fn addr(public_key: &PublicKey) -> Address {
    Address::from_bytes(h(&addr_preimage(public_key)))
}

/// The 108-byte preimage of the signing digest:
/// `D_TX ‖ network_id u32 ‖ from[32] ‖ to[32] ‖ amount u64 ‖ fee u64 ‖ nonce u64`.
///
/// `network_id` is the transaction's own field.
#[must_use]
pub fn signing_preimage(
    network_id: u32,
    from: &Address,
    to: &Address,
    amount: Amount,
    fee: Amount,
    nonce: u64,
) -> [u8; SIGNING_PREIMAGE_LEN] {
    ArrayWriter::<SIGNING_PREIMAGE_LEN>::new()
        .bytes(&D_TX)
        .u32(network_id)
        .bytes(from.as_bytes())
        .bytes(to.as_bytes())
        .u64(amount.get())
        .u64(fee.get())
        .u64(nonce)
        .finish()
}

/// The signing digest: `H` of [`signing_preimage`]. The Ed25519 message is this
/// 32-byte digest.
#[must_use]
pub fn signing_digest(
    network_id: u32,
    from: &Address,
    to: &Address,
    amount: Amount,
    fee: Amount,
    nonce: u64,
) -> SigningDigest {
    SigningDigest::from_bytes(h(&signing_preimage(
        network_id, from, to, amount, fee, nonce,
    )))
}

/// The preimage of the header hash: `D_HDR ‖ Header`, 322 bytes with the SIS anchor.
#[must_use]
pub fn header_hash_preimage(header: &Header) -> Vec<u8> {
    Writer::with_capacity(TAG_LEN + header.encoded_len())
        .bytes(&D_HDR)
        .bytes(&header.encode())
        .finish()
}

/// The header hash: `H` of [`header_hash_preimage`].
///
/// A child's `parent_hash` is the header hash of its parent.
#[must_use]
pub fn header_hash(header: &Header) -> Digest {
    h(&header_hash_preimage(header))
}

/// The 92-byte preimage of `instance_seed`:
/// `D_INST ‖ network_id u32 ‖ height u64 ‖ parent_hash[32] ‖ beacon_out[32]`.
///
/// `timestamp` is not an input: it is the one cheap miner-adjustable header field.
#[must_use]
pub fn instance_seed_preimage(
    network_id: u32,
    height: u64,
    parent_hash: &Digest,
    beacon_out: &Digest,
) -> [u8; INSTANCE_SEED_PREIMAGE_LEN] {
    ArrayWriter::<INSTANCE_SEED_PREIMAGE_LEN>::new()
        .bytes(&D_INST)
        .u32(network_id)
        .u64(height)
        .bytes(parent_hash)
        .bytes(beacon_out)
        .finish()
}

/// `instance_seed`: `H` of [`instance_seed_preimage`].
#[must_use]
pub fn instance_seed(
    network_id: u32,
    height: u64,
    parent_hash: &Digest,
    beacon_out: &Digest,
) -> Digest {
    h(&instance_seed_preimage(
        network_id,
        height,
        parent_hash,
        beacon_out,
    ))
}

/// The 114-byte preimage of `sol_commit`:
/// `D_SOL ‖ class_id u16 ‖ instance_seed[32] ‖ SHA-256(solution bytes)[32] ‖ miner_addr[32]`.
///
/// The inner hash is over the solution bytes alone, without the block's count in
/// front. It is the only use of `H` without a tag in these layouts.
#[must_use]
pub fn sol_commit_preimage(
    class_id: ClassId,
    instance_seed: &Digest,
    solution_bytes: &[u8],
    miner_addr: &Address,
) -> [u8; SOL_COMMIT_PREIMAGE_LEN] {
    ArrayWriter::<SOL_COMMIT_PREIMAGE_LEN>::new()
        .bytes(&D_SOL)
        .u16(class_id.to_u16())
        .bytes(instance_seed)
        .bytes(&h(solution_bytes))
        .bytes(miner_addr.as_bytes())
        .finish()
}

/// `sol_commit`: `H` of [`sol_commit_preimage`].
#[must_use]
pub fn sol_commit(
    class_id: ClassId,
    instance_seed: &Digest,
    solution_bytes: &[u8],
    miner_addr: &Address,
) -> Digest {
    h(&sol_commit_preimage(
        class_id,
        instance_seed,
        solution_bytes,
        miner_addr,
    ))
}

/// The 48-byte preimage of the beacon input: `D_BEACON ‖ parent_hash[32]`.
#[must_use]
pub fn beacon_input_preimage(parent_hash: &Digest) -> [u8; BEACON_INPUT_PREIMAGE_LEN] {
    ArrayWriter::<BEACON_INPUT_PREIMAGE_LEN>::new()
        .bytes(&D_BEACON)
        .bytes(parent_hash)
        .finish()
}

/// The beacon input: `H` of [`beacon_input_preimage`].
#[must_use]
pub fn beacon_input(parent_hash: &Digest) -> Digest {
    h(&beacon_input_preimage(parent_hash))
}

#[cfg(test)]
mod tests {
    use super::{
        ADDR_PREIMAGE_LEN, BEACON_INPUT_PREIMAGE_LEN, INSTANCE_SEED_PREIMAGE_LEN,
        SIGNING_PREIMAGE_LEN, SOL_COMMIT_PREIMAGE_LEN, addr, addr_preimage, beacon_input,
        beacon_input_preimage, h, instance_seed_preimage, signing_preimage, sol_commit_preimage,
    };
    use crate::{
        Amount,
        class::ClassId,
        domain::{D_ADDR, D_BEACON, D_INST, D_SOL, D_TX},
        keys::{Address, PublicKey},
        test_support::hex,
    };

    #[test]
    fn h_is_sha_256() {
        // FIPS 180-4 examples: the empty message and "abc".
        assert_eq!(
            hex(&h(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex(&h(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn preimage_lengths_are_the_ruled_lengths() {
        assert_eq!(ADDR_PREIMAGE_LEN, 48);
        assert_eq!(SIGNING_PREIMAGE_LEN, 108);
        assert_eq!(INSTANCE_SEED_PREIMAGE_LEN, 92);
        assert_eq!(SOL_COMMIT_PREIMAGE_LEN, 114);
        assert_eq!(BEACON_INPUT_PREIMAGE_LEN, 48);
    }

    #[test]
    fn addr_preimage_is_tag_then_key_and_addr_is_the_whole_digest() {
        let key = PublicKey::from_bytes([0xab; 32]);
        let preimage = addr_preimage(&key);
        assert_eq!(&preimage[..16], &D_ADDR);
        assert_eq!(&preimage[16..], &[0xab; 32]);
        assert_eq!(addr(&key).as_bytes(), &h(&preimage));
    }

    #[test]
    fn signing_preimage_has_every_field_at_its_position() {
        let from = Address::from_bytes([0x11; 32]);
        let to = Address::from_bytes([0x22; 32]);
        let preimage = signing_preimage(
            0x0403_0201,
            &from,
            &to,
            Amount::new(0x0807_0605_0403_0201),
            Amount::new(0x100f_0e0d_0c0b_0a09),
            0x1817_1615_1413_1211,
        );
        assert_eq!(&preimage[..16], &D_TX);
        assert_eq!(&preimage[16..20], &[1, 2, 3, 4]);
        assert_eq!(&preimage[20..52], &[0x11; 32]);
        assert_eq!(&preimage[52..84], &[0x22; 32]);
        assert_eq!(&preimage[84..92], &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(&preimage[92..100], &[9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(&preimage[100..108], &[17, 18, 19, 20, 21, 22, 23, 24]);
    }

    #[test]
    fn instance_seed_preimage_has_every_field_at_its_position() {
        let preimage =
            instance_seed_preimage(0x0403_0201, 0x0c0b_0a09_0807_0605, &[0x33; 32], &[0x44; 32]);
        assert_eq!(&preimage[..16], &D_INST);
        assert_eq!(&preimage[16..20], &[1, 2, 3, 4]);
        assert_eq!(&preimage[20..28], &[5, 6, 7, 8, 9, 10, 11, 12]);
        assert_eq!(&preimage[28..60], &[0x33; 32]);
        assert_eq!(&preimage[60..92], &[0x44; 32]);
    }

    #[test]
    fn sol_commit_preimage_hashes_the_solution_bytes_alone() {
        let miner = Address::from_bytes([0x66; 32]);
        let solution = [1_u8, 0, 0xff, 0xff];
        let preimage = sol_commit_preimage(ClassId::Sis, &[0x55; 32], &solution, &miner);
        assert_eq!(&preimage[..16], &D_SOL);
        assert_eq!(&preimage[16..18], &[1, 0]);
        assert_eq!(&preimage[18..50], &[0x55; 32]);
        assert_eq!(&preimage[50..82], &h(&solution));
        assert_eq!(&preimage[82..114], &[0x66; 32]);
    }

    #[test]
    fn beacon_input_is_tag_then_parent_hash() {
        let preimage = beacon_input_preimage(&[0x77; 32]);
        assert_eq!(&preimage[..16], &D_BEACON);
        assert_eq!(&preimage[16..], &[0x77; 32]);
        assert_eq!(beacon_input(&[0x77; 32]), h(&preimage));
    }

    #[test]
    fn preimages_under_different_tags_differ_in_their_first_sixteen_bytes() {
        // The two 48-byte preimages carry the same 32 payload bytes here.
        let same = [0x5a; 32];
        let address = addr_preimage(&PublicKey::from_bytes(same));
        let beacon = beacon_input_preimage(&same);
        assert_ne!(&address[..16], &beacon[..16]);
        assert_eq!(&address[16..], &beacon[16..]);
        assert_ne!(h(&address), h(&beacon));
    }
}
