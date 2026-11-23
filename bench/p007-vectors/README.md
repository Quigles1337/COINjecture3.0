# P-007 vectors for CJ3 Encoding v1

This directory holds the evidence that ruling clause G0-C8 says P-007 owes for Protocol
Spec §1.1–§1.3, §5.2, §6.1, §6.3 and §7 (v0.2-DRAFT when P-007 was built; the same text
is v0.2, normative, since 2026-11-23). Everything here is derived twice,
independently: by a script that uses only the Python standard library, and by the Rust
code. D8 permits Python only under `bench/`; nothing here is part of the node.

The two derivations share no code and no library. They do share an author, so they are
a check on implementation, not on the reading of the specification.

## Status of these vectors

Al's Gate G0-F ruling, clause 2.4, makes three files NORMATIVE conformance vectors, from
the merge of PR #21:

- `encoding-v1.vectors.txt`, for the layouts and preimages of CJ3 Encoding v1;
- `shake-expansion.vectors.txt`, for the expansion rule of §5.2. These were already
  normative for that rule (ruling G0-C8b; resume clause 2.3). They test the rule itself
  and make no tuple a candidate for P-4;
- `ed25519/c7-expected.vectors.txt`, for Protocol Spec §1.3, the thirteen
  builder-authored vectors included.

What is normative is the relation: these inputs give these bytes and these verdicts, and
every implementation reproduces them. If the text of the specification and a vector ever
disagree, that is a STOP and a ruling. Neither wins by default.

The ruling names those three files. `WORKED-VECTORS.md` and `ed25519/C7-SUMMARY.md`
render them for reading and are not named. The P-005 vectors under `spec/vectors/` stay
symbolic and non-normative and are a separate artifact.

## Fixture values are not parameters

The input values assign nothing: no network identifier, fee, target, root, timestamp
unit or tuple (clause 2.4). `hash_target`, `tx_root` and `state_root` are opaque byte
patterns.

No SIS tuple is active (P-4 and `ACTIVE_SIZE_PARAM` are unfilled). Two tuples stand in
for the active tuple, as ruling G0-C8e and resume clause 2.3 require: both were already
measured in `bench/p003-sis`, the choice is non-normative, and it makes neither a
candidate for P-4.

| Stand-in tuple `(n, m, q, beta_squared)` | Measured in `bench/p003-sis` | Used for |
|---|---|---|
| `(8, 112, 67, 112)` | `solver-results.jsonl`, `verifier-results.jsonl` | the worked vectors; property tests; fuzzing |
| `(16, 288, 257, 288)` | `estimator-results.jsonl`, `solver-results.jsonl`, `verifier-results.jsonl` | a second point for property tests and fuzzing |

The three keys are the public test keys of RFC 8032 section 7.1, tests 1 to 3.

## Files

| File | What it is | Written by |
|---|---|---|
| `cj3_encoding_v1.py` | The standard-library derivation: grammar, tags, preimages, layouts, the expansion rule, the SIS check, and Ed25519 from RFC 8032 section 5.1 | hand |
| `derive.py` | Builds every generated file below; `--check` fails if a committed file differs | hand |
| `encoding-v1.vectors.txt` | G0-C8a: every layout of G0-C3 and every preimage of G0-C2c, all bytes, as `key = value` | `derive.py` |
| `WORKED-VECTORS.md` | The same vectors field by field, for reading | `derive.py` |
| `shake-expansion.vectors.txt` | G0-C8b: the q = 17 fixture, a prime modulus above 2^31 with rejected candidates, and the expansion at each stand-in tuple | `derive.py` |
| `solver-output-n8.txt` | The exact standard output of the out-of-process solver for the worked instance | `cj3-solver-sis` |
| `ed25519/rfc8032-section-7.1.txt` | The five Ed25519 test vectors of RFC 8032 section 7.1 | `ed25519/extract_rfc8032.py` |
| `ed25519/cctv/ed25519vectors.json` | The C2SP CCTV ed25519 vectors, verbatim | upstream |
| `ed25519/cctv/LICENSE` | Their licence, verbatim | upstream |
| `ed25519/c7-expected.vectors.txt` | G0-C8c: each vector with every §1.3 clause evaluated and the verdict | `derive.py` |
| `ed25519/C7-SUMMARY.md` | Counts of those verdicts | `derive.py` |
| `../../fuzz/seeds/` | Seed inputs for the fuzz targets: the worked bytes | `derive.py` |

## Running

```text
python -I -B bench/p007-vectors/derive.py --check
cargo test -p cj3-types -p cj3-classes --locked
```

The first command is the standard-library derivation; D6 runs it in the
`unit-and-property-tests` job before the Rust tests. The Rust tests are
`crates/cj3-types/tests/p007_worked_vectors.rs`,
`crates/cj3-types/tests/p007_ed25519_vectors.rs` and
`crates/cj3-classes/tests/p007_vectors.rs`. They read the `in.` values, build each value
with the Rust code and require every `out.` value byte for byte; they also decode the
script's bytes back.

## Sources and licences

**C2SP CCTV ed25519 vectors.** Repository <https://github.com/C2SP/CCTV>, commit
`b88c4b206254ac543df240097e0f1fcc6a664821` (2026-11-01). The directory `ed25519/` was
last changed there by commit `3ec4d716e80597545ed285cf62af3dded3a14f65` (2024-03-06).

| Upstream file | Git blob | Bytes | SHA-256 |
|---|---|---:|---|
| `ed25519/ed25519vectors.json` | `ebb2192a14d88c26979c40be0aff4cfba6892ed3` | 351,077 | `b38e84caf3e7e89170ff520292dbeae421b0a794c27408ce5ce973018fe3d7f9` |
| `ed25519/LICENSE` | `1888ba4ebd623386ca4b43ff8dc2fe3a9aeb22ed` | 1,480 | `9fe727c21f00a770a683a4c8491bf24203ec839628bad09b026258c5afc17076` |

The licence is the three-clause BSD text, "Copyright 2019 Google LLC" and "Copyright
2022 Filippo Valsorda". It permits redistribution in source form provided the copyright
notice, the list of conditions and the disclaimer are retained; they are retained in
`ed25519/cctv/LICENSE`. Both files are stored unconverted (`.gitattributes`), and
`derive.py` refuses a vector file with any other SHA-256. The licence covers those two
files only. It is not a licence for this repository, whose licence is not yet chosen.

The CCTV flags decide nothing here. Each expected result is computed from §1.3 by
`verify_clauses` in `cj3_encoding_v1.py`; the flags are printed beside it for
cross-reference, and `ed25519/C7-SUMMARY.md` tabulates the two against each other.

**RFC 8032.** <https://www.rfc-editor.org/rfc/rfc8032.txt>, SHA-256
`ed63657ff389301282b169b0abde9b5dd2c7e4d524fdfa5da6ff3094fc93c4c3`. The RFC text is not
committed. `ed25519/extract_rfc8032.py` transcribes section 7.1 from a file with that
digest and refuses any other. `derive.py` checks the transcription a second way: it
recomputes every public key and signature from the secret key with its own signing code.

**Builder-authored vectors.** The lines marked `extra` in
`ed25519/c7-expected.vectors.txt` are not from either source. They cover what the two
sources do not: `S` not below `L` (clause c), a y-coordinate with no matching x, the
all-zero signature and key (clause f), and a non-canonical encoding `y + p` of a point of
large order, which a blocklist of small-order points does not catch.

**The solver output.** `solver-output-n8.txt` is the standard output of

```text
cargo run --release --locked -p cj3-solver-sis -- 8 112 67 112 747d910ac8e08a047fbf0211b4e941663a57d4c73892cd45743f52a8802b3931
```

where the last argument is the worked vectors' `instance_seed`. Its SHA-256 is
`89b0713d8725ef2b1830b4209d4e55b7cc55fe0815c0abcab33f195b819e8873`, and `derive.py`
refuses any other. The solver is untrusted: its output is used only after the
standard-library checker and `check_sis` have each accepted it.
