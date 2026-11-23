//! Rulings G0-C8a and G0-C8b: the SIS vectors, derived a second time by the Rust code.
//!
//! `bench/p007-vectors/shake-expansion.vectors.txt` and
//! `bench/p007-vectors/encoding-v1.vectors.txt` are written by a script that uses only
//! the Python standard library. This test requires `derive_instance`, the solution
//! codec and `check_sis` to agree with them.
//!
//! The expansion vectors are normative for the expansion rule and make no tuple a
//! candidate for P-4. The tuples named `stand-in` are NON-NORMATIVE stand-ins for the
//! active tuple, already measured in `bench/p003-sis` (ruling G0-C8e; P-007 resume
//! ruling, clause 2.3).

mod common;

use cj3_classes::{
    ProblemClass,
    sis::{Sis, SisParameters, SisSolution, check_sis, derive_instance},
};
use cj3_types::{Block, SisSizeParam, SizeParam, hash};
use common::Vectors;

const EXPANSION: &str = include_str!("../../../bench/p007-vectors/shake-expansion.vectors.txt");
const WORKED: &str = include_str!("../../../bench/p007-vectors/encoding-v1.vectors.txt");

fn u32_of(vectors: &Vectors, key: &str) -> u32 {
    u32::try_from(vectors.int(key)).expect("u32 vector value")
}

fn expansion_case(vectors: &Vectors, name: &str) -> ([u8; 32], SisParameters, Vec<u32>) {
    let parameters = SisParameters::new(
        u32_of(vectors, &format!("{name}.n")),
        u32_of(vectors, &format!("{name}.m")),
        u32_of(vectors, &format!("{name}.q")),
        vectors.int(&format!("{name}.beta_squared")),
    )
    .unwrap_or_else(|error| panic!("{name} is not a valid tuple: {error:?}"));
    let entries = vectors
        .ints(&format!("{name}.entries"))
        .into_iter()
        .map(|entry| u32::try_from(entry).expect("an entry below q"))
        .collect();
    (vectors.array(&format!("{name}.seed")), parameters, entries)
}

fn worked_parameters(worked: &Vectors) -> SisParameters {
    let layout =
        SisSizeParam::decode(&worked.bytes("out.size_param.bytes")).expect("a 20-byte layout");
    SisParameters::from_layout(layout).expect("the stand-in tuple is valid")
}

fn worked_solution(worked: &Vectors) -> SisSolution {
    SisSolution::new(
        worked
            .ints("in.solution.coefficients")
            .into_iter()
            .map(|coefficient| i16::try_from(coefficient).expect("a 16-bit coefficient"))
            .collect(),
    )
}

#[test]
fn every_expansion_vector_is_reproduced_by_derive_instance() {
    let vectors = Vectors::parse(EXPANSION);
    let cases = vectors.text("cases").split_whitespace().collect::<Vec<_>>();
    assert_eq!(
        cases,
        [
            "q17",
            "q2147483659-zero-seed",
            "q2147483659-seed-a5",
            "stand-in-n8",
            "stand-in-n16"
        ]
    );
    for name in cases {
        let (seed, parameters, expected) = expansion_case(&vectors, name);
        let instance = derive_instance(seed, parameters);
        assert_eq!(instance.as_column_major(), expected.as_slice(), "{name}");
        assert_eq!(
            instance,
            <Sis as ProblemClass>::derive_instance(seed, parameters),
            "{name} through the trait"
        );
    }
}

#[test]
fn the_q17_fixture_is_the_one_p003_recorded() {
    let vectors = Vectors::parse(EXPANSION);
    let (seed, parameters, expected) = expansion_case(&vectors, "q17");
    assert_eq!(seed, [0_u8; 32]);
    assert_eq!(parameters.q(), 17);
    assert_eq!(expected, [4, 12, 6, 7, 3, 15]);
    assert_eq!(vectors.int("q17.acceptance_limit"), 4_294_967_295);
}

#[test]
fn the_vector_above_two_to_the_31_has_a_prime_modulus_and_rejected_candidates() {
    let vectors = Vectors::parse(EXPANSION);
    for name in ["q2147483659-zero-seed", "q2147483659-seed-a5"] {
        let (_, parameters, expected) = expansion_case(&vectors, name);
        // `SisParameters::new` accepted the modulus, so it is prime.
        assert!(u64::from(parameters.q()) > 1 << 31, "{name}");
        let rejected = vectors.ints(&format!("{name}.rejected_groups"));
        assert!(!rejected.is_empty(), "{name} shows a rejection");
        let consumed =
            usize::try_from(vectors.int(&format!("{name}.groups_consumed"))).expect("small");
        assert_eq!(consumed, expected.len() + rejected.len(), "{name}");
        // With floor(2^32 / q) = 1 an accepted group is its own value.
        assert_eq!(
            vectors.int(&format!("{name}.acceptance_limit")),
            u64::from(parameters.q()),
            "{name}"
        );
    }
    // In the zero-seed vector the very first group is rejected and used up whole: the
    // first entry is the second group.
    let first_groups = vectors.ints("q2147483659-zero-seed.first_groups");
    let (_, _, entries) = expansion_case(&vectors, "q2147483659-zero-seed");
    assert_eq!(vectors.ints("q2147483659-zero-seed.rejected_groups"), [0]);
    assert_eq!(i64::from(entries[0]), first_groups[1]);
}

#[test]
fn the_worked_solution_encodes_decodes_and_checks() {
    let worked = Vectors::parse(WORKED);
    let parameters = worked_parameters(&worked);
    let solution = worked_solution(&worked);

    let bytes = worked.bytes("out.solution.bytes");
    assert_eq!(solution.encode(), bytes);
    assert_eq!(
        SisSolution::decode(&bytes, parameters),
        Ok(solution.clone())
    );

    let seed = worked.array("out.instance_seed");
    let instance = derive_instance(seed, parameters);
    let quality = check_sis(&instance, &solution).expect("the worked solution is valid");
    assert_eq!(quality.get(), worked.int("out.solution.quality"));

    let norm_squared = solution
        .coefficients()
        .iter()
        .map(|coefficient| u64::from(coefficient.unsigned_abs()).pow(2))
        .sum::<u64>();
    assert_eq!(norm_squared, worked.int("out.solution.norm_squared"));
}

#[test]
fn the_worked_instance_is_the_stand_in_expansion_vector() {
    let worked = Vectors::parse(WORKED);
    let expansion = Vectors::parse(EXPANSION);
    let (seed, parameters, expected) = expansion_case(&expansion, "stand-in-n8");
    assert_eq!(seed, worked.array::<32>("out.instance_seed"));
    assert_eq!(parameters, worked_parameters(&worked));
    assert_eq!(
        derive_instance(seed, parameters).as_column_major(),
        expected.as_slice()
    );
}

#[test]
fn the_worked_block_decodes_to_a_solution_the_checker_accepts() {
    // Bytes to meaning, end to end: the block layout of `cj3-types`, then the class's
    // solution codec, then derivation and the checker.
    let worked = Vectors::parse(WORKED);
    let parameters = worked_parameters(&worked);
    let active = SizeParam::Sis(parameters.layout());
    let block = Block::decode(&worked.bytes("out.block.bytes"), &active).expect("block decodes");
    let header = block.header;

    let SizeParam::Sis(layout) = header.size_param;
    assert_eq!(SisParameters::from_layout(layout), Ok(parameters));
    assert_eq!(header.class_id().to_u16(), <Sis as ProblemClass>::CLASS_ID);

    // The header's own fields give its instance seed and its solution commitment.
    assert_eq!(
        header.instance_seed,
        hash::instance_seed(
            header.network_id,
            header.height,
            &header.parent_hash,
            &header.beacon_out
        )
    );
    assert_eq!(
        header.sol_commit,
        hash::sol_commit(
            header.class_id(),
            &header.instance_seed,
            &block.solution,
            &header.miner_addr
        )
    );

    let solution = SisSolution::decode(&block.solution, parameters).expect("exact length");
    let instance = <Sis as ProblemClass>::derive_instance(header.instance_seed, parameters);
    let quality = <Sis as ProblemClass>::check(&instance, &solution).expect("valid solution");
    assert_eq!(quality.get(), worked.int("out.solution.quality"));

    for transaction in &block.transactions {
        assert_eq!(transaction.verify_signature(), Ok(()));
    }
}
