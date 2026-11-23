//! Domain tags (Protocol Spec §1.2, ruling G0-C2b).
//!
//! A domain tag is 16 bytes: its ASCII text, padded on the right with `0x00`. Each tag
//! opens exactly one preimage, in [`crate::hash`]; no other crate builds a tagged
//! preimage. A change of layout is a protocol upgrade with new tag bytes (§1.1(c)).

/// Length of every domain tag.
pub const TAG_LEN: usize = 16;

/// `CJ3/v1/addr` — opens the `addr(pk)` preimage.
pub const D_ADDR: [u8; TAG_LEN] = *b"CJ3/v1/addr\0\0\0\0\0";

/// `CJ3/v1/tx` — opens the signing-digest preimage.
pub const D_TX: [u8; TAG_LEN] = *b"CJ3/v1/tx\0\0\0\0\0\0\0";

/// `CJ3/v1/header` — opens the header-hash preimage.
pub const D_HDR: [u8; TAG_LEN] = *b"CJ3/v1/header\0\0\0";

/// `CJ3/v1/instance` — opens the `instance_seed` preimage.
pub const D_INST: [u8; TAG_LEN] = *b"CJ3/v1/instance\0";

/// `CJ3/v1/solution` — opens the `sol_commit` preimage.
pub const D_SOL: [u8; TAG_LEN] = *b"CJ3/v1/solution\0";

/// `CJ3/v1/beacon` — opens the beacon-input preimage.
pub const D_BEACON: [u8; TAG_LEN] = *b"CJ3/v1/beacon\0\0\0";

/// `CJ3/v1/genesis` — has its bytes now and no preimage yet.
///
/// The genesis layout and its preimage are ruled with the Phase 1 genesis packet
/// (ruling G0-C2c, G0-F3), so nothing in this crate uses this tag.
pub const D_GENESIS: [u8; TAG_LEN] = *b"CJ3/v1/genesis\0\0";

#[cfg(test)]
mod tests {
    use super::{D_ADDR, D_BEACON, D_GENESIS, D_HDR, D_INST, D_SOL, D_TX, TAG_LEN};
    use crate::test_support::hex;

    /// The table of ruling clause G0-C2b, copied from `loop/LEDGER.md`.
    const RULED: [(&str, &str, [u8; TAG_LEN]); 7] = [
        ("CJ3/v1/addr", "434a332f76312f616464720000000000", D_ADDR),
        ("CJ3/v1/tx", "434a332f76312f747800000000000000", D_TX),
        ("CJ3/v1/header", "434a332f76312f686561646572000000", D_HDR),
        (
            "CJ3/v1/instance",
            "434a332f76312f696e7374616e636500",
            D_INST,
        ),
        ("CJ3/v1/solution", "434a332f76312f736f6c7574696f6e00", D_SOL),
        (
            "CJ3/v1/beacon",
            "434a332f76312f626561636f6e000000",
            D_BEACON,
        ),
        (
            "CJ3/v1/genesis",
            "434a332f76312f67656e657369730000",
            D_GENESIS,
        ),
    ];

    #[test]
    fn every_tag_equals_the_ruled_hex() {
        for (_, ruled_hex, tag) in RULED {
            assert_eq!(hex(&tag), ruled_hex);
        }
    }

    #[test]
    fn every_tag_is_its_ascii_text_padded_on_the_right_with_zero_bytes() {
        for (text, _, tag) in RULED {
            let text = text.as_bytes();
            assert!(text.is_ascii());
            assert!(text.len() <= TAG_LEN);
            assert_eq!(&tag[..text.len()], text);
            assert!(tag[text.len()..].iter().all(|byte| *byte == 0));
        }
    }

    #[test]
    fn tags_are_pairwise_distinct() {
        for (index, (_, _, left)) in RULED.iter().enumerate() {
            for (_, _, right) in &RULED[index + 1..] {
                assert_ne!(left, right);
            }
        }
    }
}
