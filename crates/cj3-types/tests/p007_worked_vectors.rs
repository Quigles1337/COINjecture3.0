//! Ruling G0-C8a: the worked vectors, derived a second time by the Rust code.
//!
//! `bench/p007-vectors/encoding-v1.vectors.txt` is written by a script that uses only
//! the Python standard library. This test reads the script's `in.` values, builds each
//! value with `cj3-types`, and requires every `out.` value byte for byte. It then
//! decodes the script's bytes back to the same values.
//!
//! Every `in.` value is a NON-NORMATIVE fixture. The tuple stands in for the active
//! tuple and was already measured in `bench/p003-sis`; the choice makes it no candidate
//! for P-4 (ruling G0-C8e; P-007 resume ruling, clause 2.3).

mod common;

use cj3_types::{
    Address, Amount, Block, ClassId, DecodeError, Header, PublicKey, Signature, SignatureError,
    SisSizeParam, SizeParam, Transaction, addr, domain, hash,
};
use common::Vectors;
use curve25519_dalek::{edwards::EdwardsPoint, scalar::Scalar};
use sha2::{Digest as _, Sha512};

const VECTORS: &str = include_str!("../../../bench/p007-vectors/encoding-v1.vectors.txt");

fn vectors() -> Vectors {
    Vectors::parse(VECTORS)
}

/// Test-only Ed25519 signing, RFC 8032 section 5.1.6.
///
/// No `cj3-*` library signs. This exists so that the fixture signatures are derived
/// twice: once by the standard-library script and once here.
fn sign(secret_key: &[u8; 32], message: &[u8]) -> ([u8; 32], [u8; 64]) {
    let expanded: [u8; 64] = Sha512::digest(secret_key).into();
    let mut scalar_bytes = [0_u8; 32];
    scalar_bytes.copy_from_slice(&expanded[..32]);
    scalar_bytes[0] &= 0xf8;
    scalar_bytes[31] &= 0x7f;
    scalar_bytes[31] |= 0x40;
    // The clamped integer is below 2^255; its residue modulo L multiplies the base
    // point to the same public key, because the base point has order L.
    let secret_scalar = Scalar::from_bytes_mod_order(scalar_bytes);
    let public_key = EdwardsPoint::mul_base(&secret_scalar).compress().to_bytes();

    let mut hasher = Sha512::new();
    hasher.update(&expanded[32..]);
    hasher.update(message);
    let nonce_wide: [u8; 64] = hasher.finalize().into();
    let nonce = Scalar::from_bytes_mod_order_wide(&nonce_wide);
    let r_bytes = EdwardsPoint::mul_base(&nonce).compress().to_bytes();

    let mut hasher = Sha512::new();
    hasher.update(r_bytes);
    hasher.update(public_key);
    hasher.update(message);
    let k_wide: [u8; 64] = hasher.finalize().into();
    let k = Scalar::from_bytes_mod_order_wide(&k_wide);
    let s = nonce + k * secret_scalar;

    let mut signature = [0_u8; 64];
    signature[..32].copy_from_slice(&r_bytes);
    signature[32..].copy_from_slice(s.as_bytes());
    (public_key, signature)
}

fn key(vectors: &Vectors, name: &str) -> ([u8; 32], PublicKey) {
    let secret = vectors.array::<32>(&format!("in.{name}.secret_key"));
    let public = PublicKey::from_bytes(vectors.array(&format!("in.{name}.public_key")));
    (secret, public)
}

fn network_id(vectors: &Vectors) -> u32 {
    u32::try_from(vectors.int("in.network_id")).expect("u32 fixture")
}

fn transaction(vectors: &Vectors, name: &str) -> Transaction {
    let (secret, pubkey) = key(vectors, vectors.text(&format!("in.{name}.signer")));
    let (_, recipient_key) = key(vectors, vectors.text(&format!("in.{name}.to")));
    let mut value = Transaction {
        network_id: network_id(vectors),
        from: addr(&pubkey),
        to: addr(&recipient_key),
        amount: Amount::new(vectors.int(&format!("in.{name}.amount"))),
        fee: Amount::new(vectors.int(&format!("in.{name}.fee"))),
        nonce: vectors.int(&format!("in.{name}.nonce")),
        pubkey,
        sig: Signature::from_bytes([0_u8; 64]),
    };
    let (derived_public_key, signature) = sign(&secret, value.signing_digest().as_bytes());
    assert_eq!(&derived_public_key, pubkey.as_bytes(), "{name} signer key");
    value.sig = Signature::from_bytes(signature);
    value
}

fn stand_in(vectors: &Vectors) -> SisSizeParam {
    SisSizeParam {
        n: u32::try_from(vectors.int("in.size_param.n")).expect("u32 fixture"),
        m: u32::try_from(vectors.int("in.size_param.m")).expect("u32 fixture"),
        q: u32::try_from(vectors.int("in.size_param.q")).expect("u32 fixture"),
        beta_squared: vectors.int("in.size_param.beta_squared"),
    }
}

fn header(vectors: &Vectors) -> Header {
    let (_, miner_key) = key(vectors, vectors.text("in.header.miner"));
    let miner_addr = addr(&miner_key);
    let network_id = network_id(vectors);
    let height = vectors.int("in.header.height");
    let parent_hash = vectors.array("in.header.parent_hash");
    let beacon_out = vectors.array("in.header.beacon_out");
    let instance_seed = hash::instance_seed(network_id, height, &parent_hash, &beacon_out);
    let size_param = SizeParam::Sis(stand_in(vectors));
    Header {
        network_id,
        height,
        parent_hash,
        beacon_out,
        instance_seed,
        size_param,
        hash_target: vectors.array("in.header.hash_target"),
        tx_root: vectors.array("in.header.tx_root"),
        state_root: vectors.array("in.header.state_root"),
        sol_commit: hash::sol_commit(
            size_param.class_id(),
            &instance_seed,
            &vectors.bytes("out.solution.bytes"),
            &miner_addr,
        ),
        timestamp: vectors.int("in.header.timestamp"),
        miner_addr,
        nonce: vectors.int("in.header.nonce"),
    }
}

#[test]
fn domain_tags_match() {
    let vectors = vectors();
    for (name, tag) in [
        ("D_ADDR", domain::D_ADDR),
        ("D_TX", domain::D_TX),
        ("D_HDR", domain::D_HDR),
        ("D_INST", domain::D_INST),
        ("D_SOL", domain::D_SOL),
        ("D_BEACON", domain::D_BEACON),
        ("D_GENESIS", domain::D_GENESIS),
    ] {
        assert_eq!(
            vectors.array::<16>(&format!("out.tag.{name}")),
            tag,
            "{name}"
        );
    }
}

#[test]
fn addr_preimages_and_digests_match() {
    let vectors = vectors();
    for name in ["key1", "key2", "key3"] {
        let (secret, public) = key(&vectors, name);
        assert_eq!(
            &sign(&secret, b"").0,
            public.as_bytes(),
            "{name} public key"
        );
        assert_eq!(
            hash::addr_preimage(&public).as_slice(),
            vectors.bytes(&format!("out.addr.{name}.preimage")),
            "{name} preimage"
        );
        assert_eq!(
            addr(&public),
            Address::from_bytes(vectors.array(&format!("out.addr.{name}"))),
            "{name} addr"
        );
    }
}

#[test]
fn transaction_preimages_signatures_and_layouts_match() {
    let vectors = vectors();
    for name in ["tx1", "tx2"] {
        let value = transaction(&vectors, name);
        assert_eq!(
            value.signing_preimage().as_slice(),
            vectors.bytes(&format!("out.{name}.signing_preimage")),
            "{name} signing preimage"
        );
        assert_eq!(
            value.signing_digest().as_bytes(),
            &vectors.array::<32>(&format!("out.{name}.signing_digest")),
            "{name} signing digest"
        );
        assert_eq!(
            value.sig.as_bytes(),
            &vectors.array::<64>(&format!("out.{name}.sig")),
            "{name} signature"
        );
        let expected = vectors.bytes(&format!("out.{name}.bytes"));
        assert_eq!(value.encode().as_slice(), expected, "{name} layout");
        assert_eq!(Transaction::decode(&expected), Ok(value), "{name} decode");
        assert_eq!(value.verify_signature(), Ok(()), "{name} verifies");

        // The signature covers the signed fields: a different nonce breaks it.
        let mut altered = value;
        altered.nonce ^= 1;
        assert_eq!(
            altered.verify_signature(),
            Err(SignatureError::EquationFailed),
            "{name} altered"
        );
    }
}

#[test]
fn a_transaction_signed_for_one_network_does_not_verify_on_another() {
    let vectors = vectors();
    let mut value = transaction(&vectors, "tx1");
    value.network_id ^= 1;
    assert_eq!(
        value.verify_signature(),
        Err(SignatureError::EquationFailed)
    );
}

#[test]
fn size_parameter_layout_matches() {
    let vectors = vectors();
    let size = stand_in(&vectors);
    let expected = vectors.bytes("out.size_param.bytes");
    assert_eq!(size.encode().as_slice(), expected);
    assert_eq!(SisSizeParam::decode(&expected), Ok(size));
    assert_eq!(SizeParam::Sis(size).class_id(), ClassId::Sis);
}

#[test]
fn instance_seed_and_beacon_input_match() {
    let vectors = vectors();
    let network_id = network_id(&vectors);
    let height = vectors.int("in.header.height");
    let parent_hash = vectors.array("in.header.parent_hash");
    let beacon_out = vectors.array("in.header.beacon_out");
    assert_eq!(
        hash::instance_seed_preimage(network_id, height, &parent_hash, &beacon_out).as_slice(),
        vectors.bytes("out.instance_seed.preimage")
    );
    assert_eq!(
        hash::instance_seed(network_id, height, &parent_hash, &beacon_out),
        vectors.array::<32>("out.instance_seed")
    );
    assert_eq!(
        hash::beacon_input_preimage(&parent_hash).as_slice(),
        vectors.bytes("out.beacon_input.preimage")
    );
    assert_eq!(
        hash::beacon_input(&parent_hash),
        vectors.array::<32>("out.beacon_input")
    );
}

#[test]
fn sol_commit_preimage_and_digest_match() {
    let vectors = vectors();
    let header = header(&vectors);
    let solution = vectors.bytes("out.solution.bytes");
    let preimage = hash::sol_commit_preimage(
        ClassId::Sis,
        &header.instance_seed,
        &solution,
        &header.miner_addr,
    );
    assert_eq!(
        preimage.as_slice(),
        vectors.bytes("out.sol_commit.preimage")
    );
    // The inner hash is over the solution bytes alone.
    assert_eq!(&preimage[50..82], vectors.bytes("out.solution.sha256"));
    assert_eq!(header.sol_commit, vectors.array::<32>("out.sol_commit"));
}

#[test]
fn header_layout_and_hash_match() {
    let vectors = vectors();
    let value = header(&vectors);
    let expected = vectors.bytes("out.header.bytes");
    assert_eq!(expected.len(), 306);
    assert_eq!(value.encode(), expected);
    assert_eq!(Header::decode(&expected), Ok(value));
    assert_eq!(
        value.hash_preimage(),
        vectors.bytes("out.header_hash.preimage")
    );
    assert_eq!(value.hash(), vectors.array::<32>("out.header_hash"));
}

#[test]
fn block_layout_matches_and_decodes_back() {
    let vectors = vectors();
    let active = SizeParam::Sis(stand_in(&vectors));
    let value = Block {
        header: header(&vectors),
        solution: vectors.bytes("out.solution.bytes"),
        transactions: vec![transaction(&vectors, "tx1"), transaction(&vectors, "tx2")],
    };
    let expected = vectors.bytes("out.block.bytes");
    assert_eq!(
        u64::try_from(expected.len()).expect("small"),
        vectors.int("out.block.length")
    );
    assert_eq!(value.encode(&active), Ok(expected.clone()));
    assert_eq!(Block::decode(&expected, &active), Ok(value));

    // One byte more or one byte fewer is not this block.
    let mut longer = expected.clone();
    longer.push(0);
    assert_eq!(
        Block::decode(&longer, &active),
        Err(DecodeError::TrailingBytes { extra: 1 })
    );
    assert!(matches!(
        Block::decode(&expected[..expected.len() - 1], &active),
        Err(DecodeError::TooShort { .. })
    ));
}
