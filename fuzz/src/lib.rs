//! Fuzz properties for CJ3 Encoding v1 (Protocol Spec §1.1–§1.3, §5.2, §6.1, §6.3,
//! §7; evidence of ruling G0-C8d).
//!
//! Each public function is one property over arbitrary bytes. Each cargo-fuzz target
//! under `fuzz_targets/` is a wrapper around exactly one of them, so the D6
//! `codec-fuzz-smoke` job drives these functions with libFuzzer. They are plain
//! functions so that a stable toolchain can also run them, as ordinary tests, over the
//! committed seed inputs and a fixed pseudo-random stream (`cargo test` in `fuzz/`).
//!
//! The four properties of ruling G0-C8d, and where each is tested:
//!
//! 1. no input makes a decoder panic — every function decodes its raw input;
//! 2. decode then encode returns the input — asserted wherever a decode succeeds;
//! 3. encode then decode returns the value — asserted on a value built from the input;
//! 4. each length limit holds at the boundary, one below it and one above it —
//!    [`length_limits`].
//!
//! No tuple is active. The two tuples below are NON-NORMATIVE stand-ins for the active
//! tuple, both already measured in `bench/p003-sis`; the choice makes neither a
//! candidate for P-4 (ruling G0-C8e; P-007 resume ruling, clause 2.3).

#![forbid(unsafe_code)]

use cj3_classes::sis::{SisParameters, SisSolution, check_sis, derive_instance};
use cj3_types::{
    Address, Amount, BLOCK_MAX_BYTES, Block, DecodeError, EncodeError, Header, PublicKey,
    Signature, SisSizeParam, SizeParam, TX_MAX_BYTES, Transaction, ed25519,
};

/// NON-NORMATIVE stand-ins for the active tuple, both measured in `bench/p003-sis`.
pub const NON_NORMATIVE_STAND_INS: [SisSizeParam; 2] = [
    SisSizeParam {
        n: 8,
        m: 112,
        q: 67,
        beta_squared: 112,
    },
    SisSizeParam {
        n: 16,
        m: 288,
        q: 257,
        beta_squared: 288,
    },
];

/// Bytes of a block outside its solution and transactions: the 306-byte header and the
/// two `u32` counts.
const BLOCK_OVERHEAD: usize = 306 + 4 + 4;

/// `L`, the order of the Ed25519 base point, little-endian. Written here a second time,
/// on purpose: the harness judges clause (c) without using the code under test.
const L: [u8; 32] = [
    0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
];

/// Every 32-byte encoding that a lenient decoder maps to a point whose order divides
/// 8: the eight canonical encodings and the six alternative ones.
const SMALL_ORDER_ENCODINGS: [[u8; 32]; 14] = [
    with_ends(0x00, 0x00, 0x00),
    with_ends(0x00, 0x00, 0x80),
    with_ends(0x01, 0x00, 0x00),
    with_ends(0x01, 0x00, 0x80),
    with_ends(0xec, 0xff, 0x7f),
    with_ends(0xec, 0xff, 0xff),
    with_ends(0xed, 0xff, 0x7f),
    with_ends(0xed, 0xff, 0xff),
    with_ends(0xee, 0xff, 0x7f),
    with_ends(0xee, 0xff, 0xff),
    [
        0x26, 0xe8, 0x95, 0x8f, 0xc2, 0xb2, 0x27, 0xb0, 0x45, 0xc3, 0xf4, 0x89, 0xf2, 0xef, 0x98,
        0xf0, 0xd5, 0xdf, 0xac, 0x05, 0xd3, 0xc6, 0x33, 0x39, 0xb1, 0x38, 0x02, 0x88, 0x6d, 0x53,
        0xfc, 0x05,
    ],
    [
        0x26, 0xe8, 0x95, 0x8f, 0xc2, 0xb2, 0x27, 0xb0, 0x45, 0xc3, 0xf4, 0x89, 0xf2, 0xef, 0x98,
        0xf0, 0xd5, 0xdf, 0xac, 0x05, 0xd3, 0xc6, 0x33, 0x39, 0xb1, 0x38, 0x02, 0x88, 0x6d, 0x53,
        0xfc, 0x85,
    ],
    [
        0xc7, 0x17, 0x6a, 0x70, 0x3d, 0x4d, 0xd8, 0x4f, 0xba, 0x3c, 0x0b, 0x76, 0x0d, 0x10, 0x67,
        0x0f, 0x2a, 0x20, 0x53, 0xfa, 0x2c, 0x39, 0xcc, 0xc6, 0x4e, 0xc7, 0xfd, 0x77, 0x92, 0xac,
        0x03, 0x7a,
    ],
    [
        0xc7, 0x17, 0x6a, 0x70, 0x3d, 0x4d, 0xd8, 0x4f, 0xba, 0x3c, 0x0b, 0x76, 0x0d, 0x10, 0x67,
        0x0f, 0x2a, 0x20, 0x53, 0xfa, 0x2c, 0x39, 0xcc, 0xc6, 0x4e, 0xc7, 0xfd, 0x77, 0x92, 0xac,
        0x03, 0xfa,
    ],
];

/// A 32-byte value whose first byte, 30 middle bytes and last byte are given.
const fn with_ends(first: u8, middle: u8, last: u8) -> [u8; 32] {
    let mut bytes = [middle; 32];
    bytes[0] = first;
    bytes[31] = last;
    bytes
}

/// Reads structured values out of arbitrary bytes. When the input runs out, the rest of
/// every value is zero, so every input yields a value.
struct Source<'a> {
    data: &'a [u8],
    at: usize,
}

impl<'a> Source<'a> {
    const fn new(data: &'a [u8]) -> Self {
        Self { data, at: 0 }
    }

    fn byte(&mut self) -> u8 {
        let byte = self.data.get(self.at).copied().unwrap_or(0);
        self.at = self.at.saturating_add(1);
        byte
    }

    fn array<const N: usize>(&mut self) -> [u8; N] {
        let mut out = [0_u8; N];
        for byte in &mut out {
            *byte = self.byte();
        }
        out
    }

    fn u32(&mut self) -> u32 {
        u32::from_le_bytes(self.array())
    }

    fn u64(&mut self) -> u64 {
        u64::from_le_bytes(self.array())
    }

    fn bytes(&mut self, len: usize) -> Vec<u8> {
        (0..len).map(|_| self.byte()).collect()
    }

    fn transaction(&mut self) -> Transaction {
        Transaction {
            network_id: self.u32(),
            from: Address::from_bytes(self.array()),
            to: Address::from_bytes(self.array()),
            amount: Amount::new(self.u64()),
            fee: Amount::new(self.u64()),
            nonce: self.u64(),
            pubkey: PublicKey::from_bytes(self.array()),
            sig: Signature::from_bytes(self.array()),
        }
    }

    fn header(&mut self, size_param: SizeParam) -> Header {
        Header {
            network_id: self.u32(),
            height: self.u64(),
            parent_hash: self.array(),
            beacon_out: self.array(),
            instance_seed: self.array(),
            size_param,
            hash_target: self.array(),
            tx_root: self.array(),
            state_root: self.array(),
            sol_commit: self.array(),
            timestamp: self.u64(),
            miner_addr: Address::from_bytes(self.array()),
            nonce: self.u64(),
        }
    }

    fn stand_in(&mut self) -> SisSizeParam {
        NON_NORMATIVE_STAND_INS[usize::from(self.byte() & 1)]
    }
}

fn len_u64(len: usize) -> u64 {
    u64::try_from(len).expect("a length fits 64 bits")
}

fn usize_of(value: u64) -> usize {
    usize::try_from(value).expect("a stand-in length fits usize")
}

/// A buffer of exactly `len` bytes: `data` repeated, or zeros when `data` is empty.
fn filled(data: &[u8], len: usize) -> Vec<u8> {
    if data.is_empty() {
        vec![0_u8; len]
    } else {
        data.iter().copied().cycle().take(len).collect()
    }
}

/// The transaction codec (Protocol Spec §7).
///
/// # Panics
///
/// Panics when the codec breaks one of the properties of ruling G0-C8d.
pub fn transaction_codec(data: &[u8]) {
    // An input decodes exactly when it is 188 bytes, and then it encodes back.
    match Transaction::decode(data) {
        Ok(value) => {
            assert_eq!(data.len(), TX_MAX_BYTES);
            assert_eq!(value.encode().as_slice(), data);
        }
        Err(error) => {
            assert_ne!(data.len(), TX_MAX_BYTES);
            if data.len() > TX_MAX_BYTES {
                assert_eq!(
                    error,
                    DecodeError::WholeExceedsLimit {
                        length: len_u64(data.len()),
                        limit: len_u64(TX_MAX_BYTES),
                    }
                );
            } else {
                assert!(matches!(error, DecodeError::TooShort { .. }));
            }
        }
    }

    // Every value encodes to 188 bytes that decode to the same value.
    let value = Source::new(data).transaction();
    let bytes = value.encode();
    assert_eq!(Transaction::decode(&bytes), Ok(value));

    // The signing preimage is the tag and then the signed fields at fixed positions.
    let preimage = value.signing_preimage();
    assert_eq!(preimage[16..20], bytes[0..4]);
    assert_eq!(preimage[20..108], bytes[4..92]);

    // Judging the signature of arbitrary bytes rejects or accepts and never panics.
    let _ = value.verify_signature();
}

/// The header codec and the size-parameter layout (Protocol Spec §5.2, §6.1).
///
/// # Panics
///
/// Panics when the codec breaks one of the properties of ruling G0-C8d.
pub fn header_codec(data: &[u8]) {
    if let Ok(value) = Header::decode(data) {
        assert_eq!(data.len(), value.encoded_len());
        assert_eq!(value.encode(), data);
    }
    match SisSizeParam::decode(data) {
        Ok(value) => assert_eq!(value.encode().as_slice(), data),
        Err(_) => assert_ne!(data.len(), SisSizeParam::ENCODED_LEN),
    }

    // The size-parameter layout carries any four values; here they are arbitrary
    // layout bytes and stand in for no tuple.
    let mut source = Source::new(data);
    let layout = SisSizeParam {
        n: source.u32(),
        m: source.u32(),
        q: source.u32(),
        beta_squared: source.u64(),
    };
    assert_eq!(SisSizeParam::decode(&layout.encode()), Ok(layout));
    // Judging the four values as a tuple rejects or accepts and never panics.
    let _ = SisParameters::from_layout(layout);

    let value = source.header(SizeParam::Sis(layout));
    let bytes = value.encode();
    assert_eq!(bytes.len(), 306);
    assert_eq!(Header::decode(&bytes), Ok(value));

    let preimage = value.hash_preimage();
    assert_eq!(preimage.len(), 322);
    assert_eq!(preimage[16..], bytes);
    let _ = value.hash();
}

/// The block codec (Protocol Spec §6.3), with each stand-in as the active tuple.
///
/// # Panics
///
/// Panics when the codec breaks one of the properties of ruling G0-C8d.
pub fn block_codec(data: &[u8]) {
    for active in NON_NORMATIVE_STAND_INS.map(SizeParam::Sis) {
        if let Ok(value) = Block::decode(data, &active) {
            assert_eq!(value.encode(&active), Ok(data.to_vec()));
        }
    }

    // A block value built from the input. Its header carries one stand-in and the
    // codec is given one stand-in, which may differ. Its solution is the exact length
    // for the header's tuple, or one byte off when the input says so.
    let mut source = Source::new(data);
    let active = SizeParam::Sis(source.stand_in());
    let in_header = source.stand_in();
    let exact = usize_of(in_header.solution_len());
    let solution_len = match source.byte() % 8 {
        0 => exact - 1,
        1 => exact + 1,
        _ => exact,
    };
    let transaction_count = usize::from(source.byte() % 4);
    let value = Block {
        header: source.header(SizeParam::Sis(in_header)),
        solution: source.bytes(solution_len),
        transactions: (0..transaction_count)
            .map(|_| source.transaction())
            .collect(),
    };

    let whole = BLOCK_OVERHEAD + solution_len + transaction_count * TX_MAX_BYTES;
    let has_encoding = whole <= BLOCK_MAX_BYTES
        && len_u64(solution_len) <= active.solution_len()
        && solution_len == exact;
    match value.encode(&active) {
        Ok(bytes) => {
            assert!(has_encoding);
            assert_eq!(bytes.len(), whole);
            assert_eq!(Block::decode(&bytes, &active), Ok(value));
        }
        Err(error) => {
            assert!(!has_encoding);
            assert!(matches!(
                error,
                EncodeError::SolutionExceedsLimit { .. }
                    | EncodeError::SolutionLengthMismatch { .. }
            ));
        }
    }
}

/// The SIS solution codec and the checker behind it (Protocol Spec §5.2).
///
/// # Panics
///
/// Panics when the codec breaks one of the properties of ruling G0-C8d.
pub fn sis_solution_codec(data: &[u8]) {
    for layout in NON_NORMATIVE_STAND_INS {
        let parameters = SisParameters::from_layout(layout).expect("a measured tuple is valid");
        let exact = usize_of(layout.solution_len());

        match SisSolution::decode(data, parameters) {
            Ok(solution) => {
                assert_eq!(data.len(), exact);
                assert_eq!(solution.coefficients().len() * 2, exact);
                assert_eq!(solution.encode(), data);
            }
            Err(error) => {
                assert_ne!(data.len(), exact);
                assert_eq!(
                    error,
                    DecodeError::SolutionLengthMismatch {
                        expected: len_u64(exact),
                        actual: len_u64(data.len()),
                    }
                );
            }
        }

        // Every 2 * m bytes are a value; the value encodes back to them; and the
        // checker judges the value against a derived instance without panicking.
        let mut source = Source::new(data);
        let seed = source.array();
        let bytes = source.bytes(exact);
        let solution = SisSolution::decode(&bytes, parameters).expect("the exact length decodes");
        assert_eq!(solution.encode(), bytes);
        let instance = derive_instance(seed, parameters);
        if let Ok(quality) = check_sis(&instance, &solution) {
            // A valid solution has quality at least SCALE (Protocol Spec §5.2, P-5).
            assert!(quality.get() >= cj3_classes::QUALITY_SCALE);
        }
    }
}

/// Each length limit at the boundary, one below it and one above it (ruling G0-C8d).
///
/// `TX_MAX_BYTES`, `SOL_MAX_BYTES = 2 * m` for each stand-in tuple, and
/// `BLOCK_MAX_BYTES`. The contents of every buffer come from the input.
///
/// # Panics
///
/// Panics when a limit does not hold at one of those three lengths.
pub fn length_limits(data: &[u8]) {
    // TX_MAX_BYTES = 188.
    assert!(Transaction::decode(&filled(data, TX_MAX_BYTES)).is_ok());
    assert!(matches!(
        Transaction::decode(&filled(data, TX_MAX_BYTES - 1)),
        Err(DecodeError::TooShort { .. })
    ));
    assert_eq!(
        Transaction::decode(&filled(data, TX_MAX_BYTES + 1)),
        Err(DecodeError::WholeExceedsLimit {
            length: len_u64(TX_MAX_BYTES + 1),
            limit: len_u64(TX_MAX_BYTES),
        })
    );

    for layout in NON_NORMATIVE_STAND_INS {
        let active = SizeParam::Sis(layout);
        let parameters = SisParameters::from_layout(layout).expect("a measured tuple is valid");
        let exact = usize_of(layout.solution_len());

        // SOL_MAX_BYTES = 2 * m, at the class's own codec.
        assert!(SisSolution::decode(&filled(data, exact), parameters).is_ok());
        for wrong in [exact - 1, exact + 1] {
            assert_eq!(
                SisSolution::decode(&filled(data, wrong), parameters),
                Err(DecodeError::SolutionLengthMismatch {
                    expected: len_u64(exact),
                    actual: len_u64(wrong),
                })
            );
        }

        // SOL_MAX_BYTES = 2 * m, at the block's solution-length field.
        let mut source = Source::new(data);
        let value = Block {
            header: source.header(active),
            solution: source.bytes(exact),
            transactions: vec![source.transaction()],
        };
        let bytes = value.encode(&active).expect("a well-formed block encodes");
        assert_eq!(Block::decode(&bytes, &active), Ok(value.clone()));
        let with_length = |length: usize| {
            let mut changed = bytes.clone();
            changed[306..310].copy_from_slice(&u32::try_from(length).expect("small").to_le_bytes());
            Block::decode(&changed, &active)
        };
        assert_eq!(
            with_length(exact + 1),
            Err(DecodeError::CountExceedsLimit {
                count: len_u64(exact + 1),
                limit: len_u64(exact),
            })
        );
        assert_eq!(
            with_length(exact - 1),
            Err(DecodeError::SolutionLengthMismatch {
                expected: len_u64(exact),
                actual: len_u64(exact - 1),
            })
        );

        // BLOCK_MAX_BYTES, tested before any field is read. One byte above the limit is
        // rejected for its length whatever it holds. At the limit and one below it, the
        // input is judged on its fields and never for its length.
        let above = filled(data, BLOCK_MAX_BYTES + 1);
        assert_eq!(
            Block::decode(&above, &active),
            Err(DecodeError::WholeExceedsLimit {
                length: len_u64(BLOCK_MAX_BYTES + 1),
                limit: len_u64(BLOCK_MAX_BYTES),
            })
        );
        for within in [BLOCK_MAX_BYTES, BLOCK_MAX_BYTES - 1] {
            assert!(!matches!(
                Block::decode(&above[..within], &active),
                Err(DecodeError::WholeExceedsLimit { .. })
            ));
        }

        // The largest well-formed block under the limit round-trips, and one more
        // transaction has no encoding. (No well-formed block at a stand-in tuple is
        // exactly BLOCK_MAX_BYTES long: see the P-007 report.)
        let most = (BLOCK_MAX_BYTES - BLOCK_OVERHEAD - exact) / TX_MAX_BYTES;
        let transaction = value.transactions[0];
        let mut largest = value;
        largest.transactions = vec![transaction; most];
        let encoded = largest.encode(&active).expect("the largest block encodes");
        assert!(encoded.len() <= BLOCK_MAX_BYTES);
        assert!(encoded.len() + TX_MAX_BYTES > BLOCK_MAX_BYTES);
        assert_eq!(Block::decode(&encoded, &active).as_ref(), Ok(&largest));
        largest.transactions.push(transaction);
        assert_eq!(
            largest.encode(&active),
            Err(EncodeError::WholeExceedsLimit {
                length: len_u64(encoded.len() + TX_MAX_BYTES),
                limit: len_u64(BLOCK_MAX_BYTES),
            })
        );
    }
}

/// The §1.3 verification on arbitrary bytes: a key, a signature and a message.
///
/// # Panics
///
/// Panics when the verification panics, is not a function of its inputs, or accepts a
/// signature that §1.3 clause (a), (b) or (c) rejects on its face.
pub fn ed25519_verify(data: &[u8]) {
    let mut source = Source::new(data);
    let key: [u8; 32] = source.array();
    let signature: [u8; 64] = source.array();
    let message = data.get(96..).unwrap_or(&[]);

    let public_key = PublicKey::from_bytes(key);
    let result = ed25519::verify(&public_key, &Signature::from_bytes(signature), message);
    assert_eq!(
        result,
        ed25519::verify(&public_key, &Signature::from_bytes(signature), message)
    );

    if result.is_ok() {
        let r: [u8; 32] = signature[..32].try_into().expect("32 bytes");
        let s: [u8; 32] = signature[32..].try_into().expect("32 bytes");
        // Clause (c): S is below L.
        assert!(s.iter().rev().cmp(L.iter().rev()).is_lt());
        // Clauses (a) and (b): neither point is any encoding of a small-order point.
        assert!(!SMALL_ORDER_ENCODINGS.contains(&key));
        assert!(!SMALL_ORDER_ENCODINGS.contains(&r));
        // An accepted signature is rejected once its message changes.
        let mut other = message.to_vec();
        other.push(0);
        assert!(ed25519::verify(&public_key, &Signature::from_bytes(signature), &other).is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::{
        block_codec, ed25519_verify, header_codec, length_limits, sis_solution_codec,
        transaction_codec,
    };

    const TX1: &[u8] = include_bytes!("../seeds/transaction_codec/worked-tx1");
    const TX2: &[u8] = include_bytes!("../seeds/transaction_codec/worked-tx2");
    const HEADER: &[u8] = include_bytes!("../seeds/header_codec/worked-header");
    const SIZE_PARAM: &[u8] = include_bytes!("../seeds/header_codec/worked-size-param");
    const BLOCK: &[u8] = include_bytes!("../seeds/block_codec/worked-block");
    const SOLUTION: &[u8] = include_bytes!("../seeds/sis_solution_codec/worked-solution");
    const SIGNED_TX1: &[u8] = include_bytes!("../seeds/ed25519_verify/worked-tx1");
    const RFC_1: &[u8] = include_bytes!("../seeds/ed25519_verify/rfc8032-1");
    const RFC_2: &[u8] = include_bytes!("../seeds/ed25519_verify/rfc8032-2");
    const SMALL_ORDER_R: &[u8] = include_bytes!("../seeds/ed25519_verify/zero-signature");

    const SEEDS: [&[u8]; 10] = [
        TX1,
        TX2,
        HEADER,
        SIZE_PARAM,
        BLOCK,
        SOLUTION,
        SIGNED_TX1,
        RFC_1,
        RFC_2,
        SMALL_ORDER_R,
    ];

    /// A fixed pseudo-random stream (xorshift64*), so the stable run is repeatable.
    struct Stream(u64);

    impl Stream {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 >> 12;
            self.0 ^= self.0 << 25;
            self.0 ^= self.0 >> 27;
            self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
        }

        fn below(&mut self, bound: usize) -> usize {
            usize::try_from(self.next() % u64::try_from(bound).expect("small")).expect("small")
        }

        fn bytes(&mut self, len: usize) -> Vec<u8> {
            (0..len)
                .map(|_| u8::try_from(self.next() >> 56).expect("a byte"))
                .collect()
        }
    }

    /// The seeds, every one-byte-shorter and one-byte-longer neighbour of each, random
    /// inputs of many lengths, and seeds with a few bytes changed.
    fn inputs(count: usize) -> Vec<Vec<u8>> {
        let mut stream = Stream(0x00c0_ffee_2026_1122);
        let mut inputs = vec![Vec::new()];
        for seed in SEEDS {
            inputs.push(seed.to_vec());
            inputs.push(seed[..seed.len() - 1].to_vec());
            let mut longer = seed.to_vec();
            longer.push(0xa5);
            inputs.push(longer);
        }
        for _ in 0..count {
            let len = match stream.below(8) {
                0 => stream.below(8),
                1 => 20,
                2 => 188,
                3 => 306,
                4 => 96 + stream.below(64),
                _ => stream.below(1400),
            };
            inputs.push(stream.bytes(len));
            let mut changed = SEEDS[stream.below(SEEDS.len())].to_vec();
            for _ in 0..=stream.below(4) {
                let at = stream.below(changed.len());
                changed[at] = u8::try_from(stream.next() >> 56).expect("a byte");
            }
            inputs.push(changed);
        }
        inputs
    }

    #[test]
    fn transaction_codec_holds() {
        inputs(2000)
            .iter()
            .for_each(|input| transaction_codec(input));
    }

    #[test]
    fn header_codec_holds() {
        inputs(2000).iter().for_each(|input| header_codec(input));
    }

    #[test]
    fn block_codec_holds() {
        inputs(2000).iter().for_each(|input| block_codec(input));
    }

    #[test]
    fn sis_solution_codec_holds() {
        inputs(500)
            .iter()
            .for_each(|input| sis_solution_codec(input));
    }

    #[test]
    fn length_limits_hold() {
        inputs(8).iter().for_each(|input| length_limits(input));
    }

    #[test]
    fn ed25519_verify_holds() {
        inputs(300).iter().for_each(|input| ed25519_verify(input));
    }

    #[test]
    fn the_seed_signatures_are_accepted_and_the_small_order_one_is_not() {
        use cj3_types::{PublicKey, Signature, ed25519};
        for (seed, accepted) in [
            (SIGNED_TX1, true),
            (RFC_1, true),
            (RFC_2, true),
            (SMALL_ORDER_R, false),
        ] {
            let key = PublicKey::from_bytes(seed[..32].try_into().expect("32 bytes"));
            let signature = Signature::from_bytes(seed[32..96].try_into().expect("64 bytes"));
            assert_eq!(
                ed25519::verify(&key, &signature, &seed[96..]).is_ok(),
                accepted
            );
        }
    }
}
