//! Ruling G0-C8c: every Ed25519 vector, with the verdict Protocol Spec §1.3 gives it.
//!
//! `bench/p007-vectors/ed25519/c7-expected.vectors.txt` holds the five RFC 8032
//! section 7.1 vectors, the 914 C2SP CCTV ed25519 vectors pinned by commit, and a few
//! builder-authored ones. The verdict on each line was derived clause by clause by a
//! standard-library implementation written from the RFC. No expected result comes from
//! the library this crate uses, and none comes from the CCTV flags.
//!
//! This test requires `cj3_types::ed25519::verify` to return, for every line, the same
//! verdict: `ACCEPT`, or the same first rejecting clause.

mod common;

use cj3_types::{
    PublicKey, Signature,
    ed25519::{PointRole, SignatureError, verify},
};
use common::unhex;

const EXPECTED: &str = include_str!("../../../bench/p007-vectors/ed25519/c7-expected.vectors.txt");

/// The verdict text that the vector file uses for a Rust result.
fn verdict(result: Result<(), SignatureError>) -> &'static str {
    match result {
        Ok(()) => "ACCEPT",
        Err(SignatureError::YNotBelowP(PointRole::A)) => "a:A:y-not-below-p",
        Err(SignatureError::NoMatchingX(PointRole::A)) => "a:A:no-matching-x",
        Err(SignatureError::ZeroXWithSignBit(PointRole::A)) => "a:A:zero-x-with-sign-bit",
        Err(SignatureError::YNotBelowP(PointRole::R)) => "a:R:y-not-below-p",
        Err(SignatureError::NoMatchingX(PointRole::R)) => "a:R:no-matching-x",
        Err(SignatureError::ZeroXWithSignBit(PointRole::R)) => "a:R:zero-x-with-sign-bit",
        Err(SignatureError::SmallOrder(PointRole::A)) => "b:A",
        Err(SignatureError::SmallOrder(PointRole::R)) => "b:R",
        Err(SignatureError::SNotBelowL) => "c",
        Err(SignatureError::EquationFailed) => "e",
    }
}

struct Line<'a> {
    source: &'a str,
    id: &'a str,
    key: PublicKey,
    signature: Signature,
    message: Vec<u8>,
    expected: &'a str,
}

fn lines() -> Vec<Line<'static>> {
    EXPECTED
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let columns = line.split('|').collect::<Vec<_>>();
            assert_eq!(columns.len(), 13, "thirteen columns: {line}");
            Line {
                source: columns[0],
                id: columns[1],
                key: PublicKey::from_bytes(unhex(columns[2]).try_into().expect("a 32-byte key")),
                signature: Signature::from_bytes(
                    unhex(columns[3]).try_into().expect("a 64-byte signature"),
                ),
                message: unhex(columns[4]),
                expected: columns[11],
            }
        })
        .collect()
}

#[test]
fn every_vector_gets_the_verdict_derived_from_section_1_3() {
    let lines = lines();
    let mut disagreements = Vec::new();
    for line in &lines {
        let actual = verdict(verify(&line.key, &line.signature, &line.message));
        if actual != line.expected {
            disagreements.push(format!(
                "{} {}: expected {}, got {actual}",
                line.source, line.id, line.expected
            ));
        }
    }
    assert!(disagreements.is_empty(), "{disagreements:#?}");
}

#[test]
fn the_file_holds_every_vector_of_each_source() {
    let lines = lines();
    let count = |source: &str| lines.iter().filter(|line| line.source == source).count();
    assert_eq!(count("rfc8032"), 5);
    assert_eq!(count("cctv"), 914);
    assert_eq!(count("extra"), 13);
    assert_eq!(lines.len(), 932);

    // The CCTV vectors are numbered 0 to 913, each once, in order.
    let numbers = lines
        .iter()
        .filter(|line| line.source == "cctv")
        .map(|line| line.id.parse::<usize>().expect("a CCTV number"))
        .collect::<Vec<_>>();
    assert_eq!(numbers, (0..914).collect::<Vec<_>>());
}

#[test]
fn every_rfc_8032_vector_is_accepted_and_every_clause_has_a_rejecting_vector() {
    let lines = lines();
    assert!(
        lines
            .iter()
            .filter(|line| line.source == "rfc8032")
            .all(|line| line.expected == "ACCEPT")
    );
    for clause in [
        "ACCEPT",
        "a:A:y-not-below-p",
        "a:A:no-matching-x",
        "a:A:zero-x-with-sign-bit",
        "a:R:y-not-below-p",
        "a:R:no-matching-x",
        "a:R:zero-x-with-sign-bit",
        "b:A",
        "b:R",
        "c",
        "e",
    ] {
        assert!(
            lines.iter().any(|line| line.expected == clause),
            "no vector for {clause}"
        );
    }
}

#[test]
fn a_non_canonical_encoding_of_a_large_order_point_is_rejected_by_clause_a() {
    // The arithmetic library reduces y modulo p and would decode this key to a point
    // of large order, which a small-order test does not catch. Only "y not below p"
    // rejects it.
    let lines = lines();
    let line = lines
        .iter()
        .find(|line| line.source == "extra" && line.id == "a-large-order-y-plus-p")
        .expect("the vector is present");
    assert_eq!(
        verify(&line.key, &line.signature, &line.message),
        Err(SignatureError::YNotBelowP(PointRole::A))
    );
    let lenient = curve25519_dalek::edwards::CompressedEdwardsY(*line.key.as_bytes())
        .decompress()
        .expect("the library alone decodes it");
    assert!(!lenient.is_small_order());
}
