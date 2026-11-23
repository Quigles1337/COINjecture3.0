# C10 / P-007 Builder Report — `cj3-types` and CJ3 Encoding v1 (resumed)

Al's live ruling "LIVE CJ3 RULING - P-007 RESUME" is recorded verbatim in
[`loop/LEDGER.md`](../LEDGER.md#p-007-resume-ruling-overlay--pr-20-confirmed--report-c9-points-ruled--p-007-resumed-al-2026-11-22).
This report cites its clauses as "resume 2.2", "resume 4.5" and so on. It cites a clause
of the 2026-11-22 Gate G0 ruling as `G0-` plus its label (`G0-C3f`, `G0-C8b`). A bare
B1–B12 is a block rule of Protocol Spec §6.2, a bare D1–D17 is a ledger decision, and
"packet P-011" is the SIS solver-envelope packet, never parameter P-11.

The earlier P-007 report, [`C7-p007-builder.md`](C7-p007-builder.md), records the stop of
2026-08-16 and stays as written.

## 1. FRAME — written before any build edit

Three commits precede this FRAME on the branch, each ordered by the ruling: the verbatim
overlay (`d561a92`, resume preamble), the text-only stale-statement commit (`6836de1`,
resume Part 3) and the five D17 conditions beside D17 (`a132fcb`, resume 2.4). No file
under `crates/`, `fuzz/`, `scripts/` or `.github/` had changed when this was written.

### 1.1 Packet and done-condition

P-007 is the Phase 0 shared-types packet: canonical encodings, domain tags, the single
`addr()`, amount newtypes, and codec fuzz targets wired into CI (`loop/PACKETS.md`). PR
#14 delivered only the checked `Amount`. This session builds the rest, to Protocol Spec
v0.2-DRAFT §1.1–§1.3, §5.2, §6.1, §6.3 and §7 (resume 4.1). It is done when:

1. `cj3-types` holds the grammar, the seven domain tags, `H`, every preimage of §1.2,
   `addr()`, the transaction, header and block layouts, the class identifier table, the
   20-byte SIS size-parameter layout and the §1.3 verification (resume 4.2, 4.3);
2. `cj3-classes` imports those layouts, binds SIS to class identifier 1, holds the SIS
   solution encoding and the validity rules of a tuple, and still owns derivation and
   checking (resume 4.2);
3. the evidence of G0-C8 exists: worked vectors derived twice (a), the expansion
   vectors (b), the Ed25519 vectors with expected results derived clause by clause (c),
   live fuzz targets in D6 (d), and no tuple compiled in as active (e);
4. the adversary pass is written down, the exact head is green in D6 on origin, the
   pull request is marked ready, and it is left unmerged for Al (resume 4.8).

### 1.2 Clause check before acting

The ruling orders a STOP if a clause is ambiguous or conflicts with an axiom, the Lean
files, the 2026-11-22 ruling or another clause. I read every clause against A1–A11, the
four Lean sources, Protocol Spec v0.2-DRAFT and the 2026-11-22 ruling before the first
edit. I found no clause that needs a STOP. Four places needed care, and each is settled
by the text rather than by a choice of mine:

- **Resume 4.2, the split.** The block layout (G0-C3f) needs the solution's length in
  bytes for a size parameter, and the solution encoding lives with the class. This is
  buildable without a cycle and without a second implementation: `cj3-types` owns the
  size-parameter layout, so it also holds the one function that gives the solution
  length for it (`2 * m`); `cj3-classes` imports that function and holds the
  coefficient coding. `cj3-classes` depends on `cj3-types`, never the reverse.
- **Resume 4.4 with 4.5.** cargo-fuzz targets need `libfuzzer-sys`, whose licence is
  `(MIT OR Apache-2.0) AND NCSA`. NCSA is not on `deny.toml`'s allow-list, and
  `deny.toml` is not loosened. The two clauses hold together because the fuzz crate is a
  separate cargo workspace under `fuzz/` (the cargo-fuzz layout), so it is outside the
  graph `cargo deny` checks and is linked into no `cj3-*` crate. I propose the crate
  below, with that fact stated, for Al to accept or refuse.
- **Resume 4.5, "that job alone".** The D6 Nightly workflow has a `full-fuzz-corpus` job
  that calls the same gate. Once the gate has a handler, that job would run cargo-fuzz
  and need a nightly toolchain, which 4.5 allows to the `codec-fuzz-smoke` job alone. So
  the nightly job is pointed at its own gate name and stays an explicit deferral. I
  record it as a point for Al (section 4.2).
- **G0-C7 with G0-C8c.** G0-C7 fixes the consensus message as the 32-byte signing
  digest. The RFC 8032 and CCTV vectors use messages of other lengths. The verification
  is therefore one function over a message of any length, and the transaction path
  passes it the 32-byte digest and nothing else.

### 1.3 Lane

**HUMAN**, by resume 4.8 and on the five D17 conditions argued for this packet:

| Condition | Holds? | Why |
|---|---|---|
| Approved and unblocked | Yes | The resume ruling approves it and G0-C rules everything the 2026-08-16 stop needed. |
| Bounded non-semantic surface | **No** | Canonical bytes, preimages and signature verification are consensus-semantic. |
| No formal-spec content | **No** | Part 3 edits comments in `spec/*.lean`, and the worked vectors are vector definitions, which D17 names as HUMAN. |
| No decision invention | Yes, as a control | Every value written is supplied by a ruling; the fixture values are labeled as fixtures. |
| No owned-TBD fill | Yes, as a control | P-1, P-4, P-7, P-9, P-10, P-12 and `ACTIVE_SIZE_PARAM` stay unfilled (resume 4.7). |

The builder does not merge.

### 1.4 How the split of resume 4.2 is built

| Lives in `cj3-types` (bytes) | Lives in `cj3-classes` (meaning) |
|---|---|
| Grammar: reader, writers, the typed `DecodeError` (§1.1) | `SisParameters`: the validity rules of a tuple, built from the imported layout |
| The seven domain tags; `H`; the six preimages of §1.2 | `derive_instance` (unchanged) and `check_sis` (unchanged) |
| `addr()`, the signing digest and the §1.3 verification | The SIS solution encoding: `2 * m` bytes of 16-bit two's-complement little-endian coefficients |
| Transaction, header and block layouts | The binding of SIS to class identifier 1 |
| The class identifier table (SIS = 1; 0 never assigned) | The tuple rule of G0-C4: `beta_squared` above `32767^2` is not admissible |
| The 20-byte SIS size-parameter layout, and the solution length it implies | |

In `cj3-types` the solution is opaque bytes. In the header, `class_id` is not stored
beside `size_param`: it is derived from the size parameter's variant, so a header value
cannot hold a class identifier that disagrees with its layout.

### 1.5 The order of checks (resume 2.2) — fixed here

Which typed error a doubly wrong input gets is not a consensus matter. The order is:

**Block** — `Block::decode(bytes, active)`, where `active` is the size parameter that
stands in for the active tuple (G0-C8e):

1. The whole length against `BLOCK_MAX_BYTES`, before any field is read: `WholeExceedsLimit`.
2. The header's fields in layout order. The first field that cannot be read:
   `TooShort`. `class_id` is tested when it is read, before `size_param`:
   `UnknownClassId`.
3. The `u32` solution length: (a) above `SOL_MAX_BYTES = 2 * m` of `active`:
   `CountExceedsLimit`; (b) not `2 * m` of the header's own `size_param`:
   `SolutionLengthMismatch`; (c) more than the bytes that remain: `TooShort`. Nothing is
   allocated before (c) passes.
4. The `u32` transaction count. It has no limit of its own. `188 * count` is computed
   in 64 bits. More than the bytes that remain: `TooShort`. Fewer: `TrailingBytes`.
   Nothing is allocated before the two are equal.
5. The transactions, in order.

The decoder does not compare the header's `size_param` with `active`. That comparison is
block rule B3, which is not a decoding rule. `active` supplies only the P-8 limit of
step 3(a), so that B1 ("sizes within P-8") is complete at the decoder.

**Transaction** — the whole length against `TX_MAX_BYTES` first (`WholeExceedsLimit`),
as the Lean `v1` tests the size before it decodes; then the fields in order
(`TooShort`). A transaction's public key and signature are opaque bytes to the decoder:
G0-C1e lists every decoding rejection and none of them is about a point, so a key that
is not a point fails V2 (§1.3), not V1.

**Header, size parameter** — the fields in order (`TooShort`, `UnknownClassId`), then
`TrailingBytes`.

**SIS solution** — one test: the length is exactly `2 * m` (`SolutionLengthMismatch`).

**Signature (§1.3)** — decode `A`, then decode `R` (clause a: y not below p, then no
matching x, then x of zero with the sign bit); `A` of small order, then `R` (clause b);
`S` not below `L` (clause c); the equation (clauses d and e).

### 1.6 Dependencies proposed (resume 4.4)

Al accepts or refuses each at review. `deny.toml` is unchanged. In a scratch crate with
this repository's `deny.toml`, `sha3 =0.10.9`, `sha2 =0.10.9` and
`curve25519-dalek =4.1.3` together passed `cargo deny --locked --all-features check`
(exit 0) and `cargo audit --deny warnings` (exit 0) on 2026-11-22.

| Crate, exact pin | Licence | Used for | A9 case |
|---|---|---|---|
| `sha2 =0.10.9`, no default features | MIT OR Apache-2.0 | SHA-256 (`H`) and SHA-512 (§1.3 clause d) | Pure Rust, RustCrypto, the same `digest 0.10.7` line that `sha3 =0.10.9` already uses, so no second version of any crate. No C. It contains `unsafe` for CPU intrinsics (8 source files name the word). |
| `curve25519-dalek =4.1.3`, no default features | BSD-3-Clause | Point decompression, group operations and scalars for §1.3 | Pure Rust. No C. It contains `unsafe` in its vector back ends (6 source files name the word); the back end is chosen at build and run time by CPU. Only its arithmetic is used: every acceptance rule of §1.3 is written in `cj3-types`, and no `verify` or `verify_strict` of any library is called. |

`cj3-types` gains no other direct dependency. The lockfile gains, through those two:
`cfg-if 1.0.5`, `curve25519-dalek-derive 0.1.1` (proc-macro, MIT/Apache-2.0),
`fiat-crypto 0.2.9` (MIT OR Apache-2.0 OR BSD-1-Clause; a back end selected only by an
explicit cfg), `subtle 2.6.1` (BSD-3-Clause), `rustc_version 0.4.1` and `semver 1.0.28`
(build-time), and `proc-macro2 1.0.107`, `quote 1.0.47`, `syn 2.0.119`,
`unicode-ident 1.0.27` (build-time, through the proc-macro).

Why not newer: `curve25519-dalek 5.0.0` needs `cpufeatures 0.3`, and with
`sha3 =0.10.9` in the graph `cargo deny` then fails on two versions of `cpufeatures`
(tested, exit 1). `sha2 0.11.1` needs `digest 0.11`, which duplicates `digest` beside
`sha3 =0.10.9`. Moving either would mean moving `sha3`, the dependency behind the
expansion that G0-C5 ratified "as it stands", which is not this packet's to move.
`4.1.3` is the newest unyanked 4.x (`4.2.0` is yanked) and carries the fix for
RUSTSEC-2024-0344.

No Ed25519 signing code enters any `cj3-*` library. Tests sign with a test-only routine.

**Fuzz tooling, outside the workspace** (not linked into any `cj3-*` crate, not in the
root `Cargo.lock`, not seen by `cargo deny`):

| Tool or crate, exact pin | Licence | Note |
|---|---|---|
| `cargo-fuzz 0.13.2` (installed in the CI job) | MIT OR Apache-2.0 | The driver. |
| `libfuzzer-sys =0.4.13` | **(MIT OR Apache-2.0) AND NCSA** | Bundles LLVM's libFuzzer, which is C++. NCSA is not on `deny.toml`'s allow-list. It is compiled only into fuzz binaries. |
| Rust `nightly-2026-11-15` | — | Used by the `codec-fuzz-smoke` job alone (resume 4.5). |

`fuzz/Cargo.lock` is committed, and the `dependency-audit` job audits it as well as the
root lockfile.

### 1.7 Fuzzing (resume 4.5, G0-C8d)

One fuzz crate under `fuzz/`, its own workspace. The properties are plain functions in
its library, so a stable toolchain can run them over the seed inputs as ordinary tests;
each fuzz target is a wrapper around one of them. The targets cover: no input makes a
decoder panic; decode then encode returns the input; encode then decode returns the
value; each length limit at the boundary, one below and one above; and the §1.3
verification on arbitrary bytes. `scripts/ci/active/codec-fuzz-smoke.ps1` makes the gate
live: it runs every target for a fixed number of runs from a fixed seed, on the pinned
nightly, and fails if any target is missing from its list.

### 1.8 Vectors (resume 2.3, 2.7, 4.6; G0-C8a to c)

Under `bench/p007-vectors/`, Python standard library only:

- **G0-C8a.** One script derives every layout and preimage from stated field values and
  writes all bytes. Rust tests read the same field values, build the values with the
  Rust code and require the same bytes; they also decode the script's bytes back. The
  status of these worked vectors is Al's to give at review (resume 2.7); the files say
  so and claim none.
- **G0-C8b.** The q = 17 fixture; a prime modulus above 2^31 with rejected candidates;
  and the expansion at each stand-in tuple. These are normative for the expansion rule
  (resume 2.3) and make no tuple a candidate for P-4.
- **G0-C8c.** The five RFC 8032 §7.1 vectors and the C2SP CCTV ed25519 vectors, pinned
  by commit. A standard-library implementation of §1.3 evaluates each clause for each
  vector and records the result; the Rust verification must agree vector by vector. No
  expected result comes from the library under test or from the CCTV flags.

**CCTV licence (resume 4.6).** Read before anything was copied: repository
`C2SP/CCTV` at commit `b88c4b206254ac543df240097e0f1fcc6a664821` (2026-11-01); the
directory `ed25519/` was last changed by `3ec4d716e80597545ed285cf62af3dded3a14f65`.
`ed25519/ed25519vectors.json` is git blob `ebb2192a14d88c26979c40be0aff4cfba6892ed3`,
351,077 bytes, SHA-256
`b38e84caf3e7e89170ff520292dbeae421b0a794c27408ce5ce973018fe3d7f9`. `ed25519/LICENSE`
(SHA-256 `9fe727c21f00a770a683a4c8491bf24203ec839628bad09b026258c5afc17076`) is the
three-clause BSD text, "Copyright 2019 Google LLC" and "Copyright 2022 Filippo
Valsorda". It permits redistribution in source form when the notice, the conditions and
the disclaimer are kept. The copy is therefore allowed, and the licence file is copied
beside the vectors. No STOP.

**Stand-in tuples (G0-C8e, resume 2.3).** Two, both measured in `bench/p003-sis`:
`(8, 112, 67, 112)` (solver and verifier results) for the worked vectors, and
`(16, 288, 257, 288)` (estimator, solver and verifier results) as a second point for
property tests and fuzzing. Both are labeled non-normative wherever they appear. They
appear only in test code, in the fuzz crate and in `bench/`; no `cj3-*` library names a
tuple.

### 1.9 Predicted diff surface

- `loop/LEDGER.md` — done above; one short index of this ruling at closeout.
- The seven files of the Part 3 commit — done above.
- `crates/cj3-types/` — `Cargo.toml`, `src/lib.rs` and new modules, `tests/`.
- `crates/cj3-classes/` — `Cargo.toml`, `src/lib.rs`, `src/sis.rs`, `tests/`, and one
  literal in `examples/p003_check_bench.rs`.
- `crates/cj3-solver-sis/src/main.rs` and the test module of
  `bench/p004-admission/src/lib.rs` — mechanical follow-through, because a decoded
  solution becomes 16-bit coefficients and `Sis` loses its unbound identifier.
- `Cargo.lock`.
- `bench/p007-vectors/` — scripts, generated vector files, the CCTV copy and its licence.
- `fuzz/` — manifest, lockfile, toolchain pin, library, targets, seed inputs.
- `scripts/ci/active/codec-fuzz-smoke.ps1`, `scripts/ci/check-phase-gate.ps1`.
- `.github/workflows/ci.yml` (the `codec-fuzz-smoke` job; one step that re-derives the
  vectors; the audit of `fuzz/Cargo.lock`) and `.github/workflows/nightly.yml` (the
  deferral).
- `.gitattributes`, `.gitignore`.
- `README.md`, `loop/STATE.md`, `loop/PACKETS.md`, `loop/reports/BATCH-LOG.md`, this
  report.
- `docs/PROTOCOL_SPEC.md` — one half-sentence in §5.2 that says the bench file still
  calls the q = 17 fixture nonnormative. Resume 3.2 makes it false. Nothing else there.

Not predicted: `deny.toml`; `rust-toolchain.toml`; any Lean definition, theorem or
vector; `cj3-beacon`, `cj3-kernel`, `cj3-consensus`, `cj3-net`, `cj3-rpc`, `cj3-store`,
`cj3-node`; `docs/ENGINEERING_PLAN.md` beyond the Part 3 line; the audit matrix; the
builder prompt; any earlier report or BATCH-LOG row.

### 1.10 Top risks, falsifier, confidence

1. **A verification that is almost §1.3.** The arithmetic library decodes more points
   than RFC 8032 allows. Control: the canonical tests are written in `cj3-types` from
   the bytes, and the CCTV vectors, judged by an independent implementation, decide.
2. **A decoder that accepts a second encoding or panics.** Control: one reader with
   checked reads and no indexing; exact-length rules; fuzz targets for all four
   properties.
3. **A fixture that becomes a parameter.** Control: no tuple, network identifier, fee or
   timestamp unit appears in any library; every fixture value is labeled.

**Falsifier:** this approach is wrong if the Rust code and the standard-library script
disagree on any byte; if any CCTV or RFC vector gets a different verdict from the two
implementations; if a `cj3-*` library names a tuple; or if the split needs
`cj3-types` to depend on `cj3-classes`.

**Confidence: MED-HIGH** on the codecs and preimages, **MED** on the first hosted run of
the fuzz job, which cannot be linked on this Windows host.

## Preflight evidence (prompt STEP 0; resume 1.1)

| Step | Result | Evidence |
|---|---|---|
| a. Path | PASS | `pwd -W` returned `C:/Users/LEET/COINjecture3.0`; no `OneDrive` segment |
| b. Remote | PASS | `git remote -v`: fetch and push are both `https://github.com/Quigles1337/COINjecture3.0` |
| c. Branch and tree | PASS | on `main`, `git status --porcelain` empty, `HEAD` = `origin/main` = `git ls-remote origin main` = `f601224db855e6cc69e840fb169d85df22c547b8` after `git fetch` |
| d. Visibility | PASS | `gh repo view Quigles1337/COINjecture3.0 --json visibility` returned `PUBLIC` |
| e. Authorization | PASS | `loop/LEDGER.md` holds D17 RATIFIED (2026-08-15 overlay) |
| f. Capacity | PASS | `loop/STATE.md` carries the condition of Part 0.3 of the 2026-11-22 ruling; `C:\Users\LEET\COINjecture2.0-network\loop\STATE.md` reads `CAPACITY_FLAG: none`, not `remediation-priority` |
| g. Survey and P-008 | PASS | `docs/RESEARCH_SURVEY.md` present; the P-008 URL is still `TBD(Al-supplied frontend URL)`, so P-008 stays blocked |
| Resume 1.1 | PASS | `gh pr view 20`: state `MERGED`, merged at `2026-11-23T03:51:34Z`, merge commit `f601224db855e6cc69e840fb169d85df22c547b8`, head `debf20192f63816a58a0458a490a40c2600833bb`. `gh run view 45108419759`: workflow `D6 CI`, event `push`, that same commit, conclusion `success`, eleven jobs each `success` with five to nine steps (<https://github.com/Quigles1337/COINjecture3.0/actions/runs/45108419759>). The run was still in progress when first read; nothing was written to the repository until it concluded. |

## 2. BUILD

### 2.1 What was built

- **`crates/cj3-types`** — `codec` (reader, writers, `DecodeError`), `domain` (the seven
  tags), `hash` (`H`, six preimages, `addr`), `keys`, `class` (`ClassId`,
  `SisSizeParam`, `SizeParam`), `tx`, `header`, `block`, `ed25519`. `Amount` is
  unchanged. Sixty unit tests.
- **`crates/cj3-classes`** — `Sis` reads its identifier from `ClassId::Sis`;
  `SisParameters::from_layout` and `layout`; `ParameterError::NormBoundExceedsEncoding`
  for `beta_squared` above `32767^2`; `SisSolution` holds 16-bit coefficients with
  `encode` and `decode`. `derive_instance` and the body of `check_sis` are byte for byte
  as P-003 left them. Seven new unit tests.
- **`bench/p007-vectors`** — the standard-library derivation, the five generated vector
  files, the RFC 8032 transcription and its extractor, the CCTV copy and licence, the
  solver output, a README.
- **`fuzz`** — its own workspace: six properties in a library, six targets, eleven seed
  inputs, a lockfile and the toolchain pin.
- **CI** — `scripts/ci/active/codec-fuzz-smoke.ps1`; the `codec-fuzz-smoke` job
  installs cargo-fuzz 0.13.2 and `nightly-2026-11-15`; `unit-and-property-tests` runs
  `derive.py --check`; `dependency-audit` audits `fuzz/Cargo.lock`; the D6 Nightly
  `full-fuzz-corpus` job calls a gate of its own that has no handler.
- **Follow-through** — `cj3-solver-sis` and `examples/p003_check_bench.rs` build 16-bit
  coefficient vectors; the admission bench's test names `Sis` without a parameter.

### 2.2 Tripwires

- **SCOPE: held.** The 76 files of `git diff --name-only main` are all inside the
  predicted surface of §1.9. No file of `cj3-beacon`, `cj3-kernel`, `cj3-consensus`,
  `cj3-net`, `cj3-rpc`, `cj3-store` or `cj3-node` changed; `deny.toml` and
  `rust-toolchain.toml` did not change.
- **INVENTION: held.** Values this change writes, and where each comes from:
  `TX_MAX_BYTES = 188` and `BLOCK_MAX_BYTES = 1,048,576` (G0-C6, the second labeled
  provisional); class identifier 1 (G0-C3b); `32767^2` (G0-C4); the tag bytes (G0-C2b).
  Values I chose, none of them a protocol value: the fixture field values, the modulus
  2147483659 (the first prime above 2^31), the fuzz run counts and the libFuzzer seed,
  the nightly date, and the tool versions.
- **REPETITION: not tripped.** No failure repeated three times.
- **INTERPRETATION: not tripped.** No conflict between the plan and the spec blocked
  the packet. The places where I had to settle how clauses fit together are in §1.2 and
  §4.2, and none changes a set of consensus-valid inputs.
- **ENVIRONMENT: held.** Visibility read `PUBLIC` again before the first push, and the
  COINjecture 2.0 loop still read `CAPACITY_FLAG: none`.
- **DEGRADATION: one slip, caught.** While diagnosing the unsafe scan (§3.3) I moved
  `fuzz/` out of the repository to rule it out, and the first move back failed with a
  transient "Permission denied". The second succeeded. Nothing was committed in
  between; afterwards `derive.py --check` passed on all sixteen generated files and the
  fuzz properties passed, which is how I know the directory came back whole. I should
  have tested on a copy.

### 2.3 What changed on this machine outside the repository

Recorded because Al should not find these by surprise:

- WSL Ubuntu: `rustup toolchain install nightly-2026-11-15 --profile minimal` and
  `cargo install cargo-fuzz --version 0.13.2 --locked`, so the fuzz targets could run
  on Linux before the first push. Build output is in `~/cj3-fuzz-target` there.
- Windows: rustup installed `nightly-2026-11-15` on demand when the first draft of the
  gate handler asked that toolchain for its version. I had not meant to install it. The
  handler now reads `rustup toolchain list` instead.
- Read-only fetches: a clone of `C2SP/CCTV` and a download of `rfc8032.txt`, both into
  the session's scratch directory. Nothing was sent to either.

## 3. VERIFY

### 3.1 The two derivations agree

- `python -I -B bench/p007-vectors/derive.py --check` → `P007_VECTORS=PASS files=16`,
  on Windows with Python 3.11.9 and again on Linux (WSL) with Python 3.12.3 against the
  same files.
- `crates/cj3-types/tests/p007_worked_vectors.rs` (9 tests) builds every value from the
  `in.` lines with the Rust code and requires every `out.` line: the seven tags; three
  `addr` preimages and digests; two signing preimages, digests, signatures and
  188-byte transactions; the 20-byte size parameter; `instance_seed`; the beacon input;
  `sol_commit`; the 306-byte header and its hash; the 914-byte block. The signatures
  are derived twice too: by the script's RFC 8032 code and by a test-only signer.
- `crates/cj3-classes/tests/p007_vectors.rs` (6 tests): `derive_instance` reproduces
  all five expansion vectors; the solver's solution encodes to the script's bytes,
  decodes back, and `check_sis` gives the quality the script computed (7,000,000, from
  squared norm 16 under `beta_squared = 112`).
- `crates/cj3-types/tests/p007_ed25519_vectors.rs` (4 tests): for all 932 lines the
  Rust verification returns the verdict the script derived. By source:

  | Source | Vectors | Accepted | Rejected by (a) | by (b) | by (c) | by (e) |
  |---|---:|---:|---:|---:|---:|---:|
  | RFC 8032 §7.1 | 5 | 5 | 0 | 0 | 0 | 0 |
  | C2SP CCTV | 914 | 43 | 490 | 318 | 0 | 63 |
  | builder-authored | 13 | 0 | 4 | 3 | 3 | 3 |

- The agreement was reached at the first comparison. No vector, test or line of
  verification code was changed to obtain it.
- The CCTV copy is verbatim: its git blob ids in this repository,
  `ebb2192a14d88c26979c40be0aff4cfba6892ed3` and
  `1888ba4ebd623386ca4b43ff8dc2fe3a9aeb22ed`, equal the upstream ones.

### 3.2 `spec/vectors/p005-draft.json` (resume 3.1)

Git blob `cc0c3f71c3f7af7db50287f4f56e526bbc56d831` at `main` (`f601224`) and at the
head of this branch. SHA-256 of that blob, before and after:
`30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`. After the Part 3
edits the Lean gate rebuilt `Spec.Tx`, `Spec.Stf`, `Spec.Vectors` and `Main` and
regenerated the same 27 vectors:
`P005_RATIFIED_SPEC_BUILD=PASS LEAN=4.33.0 VECTORS=27 SHA256=30CABF85…C324E1`. The
working-tree copy in this checkout had CRLF line endings from before the `eol=lf`
attribute existed (SHA-256 `b45d9472…`); I re-checked it out so the gate could run
here. That changed no tracked content.

### 3.3 Local checks at the code head

| Check | Result |
|---|---|
| `cargo fmt --all -- --check`; the same for `fuzz/` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --all-targets --all-features --locked` | PASS, 104 tests (22 before this packet) |
| `cargo build --workspace --all-targets --all-features --locked` | PASS |
| `cargo deny --locked --all-features check`, `deny.toml` unchanged | `advisories ok, bans ok, licenses ok, sources ok` |
| `cargo audit --deny warnings`; the same with `--file fuzz/Cargo.lock` | PASS, 34 and 33 crates |
| `scripts/ci/check-source-policy.ps1` | `SOURCE_POLICY=PASS files=28 crates=10` |
| `scripts/ci/verify-geiger.ps1`, in a fresh target directory | `GEIGER_POLICY=PASS packages=11 unsafe_count=0` |
| Lean gate (`check-phase-gate.ps1 -Gate lean-conformance`) | PASS, 27 vectors, digest above |
| `cargo test --lib --locked` in `fuzz/` (the properties, on stable) | PASS, 7 tests |
| cargo-fuzz on Linux (WSL), ASan, `nightly-2026-11-15`, cargo-fuzz 0.13.2 | every target exit 0: 500,000 runs each of `transaction_codec`, `header_codec`, `block_codec`; 20,000 each of `sis_solution_codec`, `ed25519_verify`; 300 of `length_limits` |
| `cj3-solver-sis` against four output hashes of `bench/p003-sis/solver-results.jsonl` | all four reproduced, so the 16-bit conversion did not change what the solver prints |

The unsafe scan first failed in the shared `target/` of this checkout:
`cargo-geiger` read a dependency record dated 2026-08-15 for a crate that is in no
lockfile here and whose source the registry no longer holds. It is a local leftover,
not a property of this change: the same scan passes in a fresh target directory and on
an untouched checkout of `main`, and a hosted runner starts clean.

On this Windows host the fuzz binaries build but do not run (the ASan runtime is
missing), and the gate handler stops there on purpose, because cargo-fuzz here is
0.13.1 and it requires 0.13.2. The handler's own run is therefore evidenced by the
hosted job, not locally.

### 3.4 Origin D6

The exact-head run for the final head is recorded on
[PR #21](https://github.com/Quigles1337/COINjecture3.0/pull/21). This file cannot hold
the run of the commit that contains it, so it makes no green claim for that head.

An earlier head can be cited. Commit `02ec4ffbd2f8918bd302bb1743ea377ba611b509` is the
last commit that changes code, vectors, fuzz targets or CI; the commits after it change
documentation only. D6 run `45114338427` on that commit
(<https://github.com/Quigles1337/COINjecture3.0/actions/runs/45114338427>, event
`pull_request`) concluded success in all eleven jobs, each with five to ten steps. Read
from its log with `gh run view --log`:

- `dependency-policy`: `advisories ok, bans ok, licenses ok, sources ok`, with
  `deny.toml` unchanged;
- `unsafe-audit`: `GEIGER_POLICY=PASS packages=11 unsafe_count=0`;
- `unit-and-property-tests`: `P007_VECTORS=PASS files=16`, then the workspace tests;
- `lean-conformance`: `P005_RATIFIED_SPEC_BUILD=PASS LEAN=4.33.0 VECTORS=27
  SHA256=30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`;
- `codec-fuzz-smoke`: `STATUS=ACTIVE`, `FUZZ_TOOLCHAIN=rustc 1.102.0-nightly
  (5f4effd57 2026-11-13)`, `FUZZ_DRIVER=cargo-fuzz 0.13.2`, each of the six targets
  `RESULT=PASS`, and `CODEC_FUZZ_SMOKE=PASS TOOLCHAIN=nightly-2026-11-15 TARGETS=6
  TOTAL_RUNS=740300 SEED=20261122`;
- `conservation-invariant` and `genesis-spend-test`: still `NOT_YET_ADMITTED`. Their
  green is a deferral marker, not a test result.

## 4. ADVERSARY PASS

Seat switched. I re-read the whole diff as someone trying to make two honest nodes
disagree, make a node panic, or make this record claim more than it shows.

### 4.1 Findings

1. **A hole in the evidence, closed.** RFC 8032 and CCTV give no vector for a
   non-canonical encoding of a point of *large* order. `y = 3` is on the curve and not
   of small order, so the bytes of `3 + p` are such an encoding: the arithmetic library
   alone decodes them, and a small-order test does not reject them. Only the test
   "y not below p" does. I added that vector for `A` and for `R`; the Rust code rejects
   both by clause (a), and a test shows the library alone would not. Neither source
   exercises clause (c) either, so there are builder-authored vectors for `S = L`,
   `S + L`, `S = 2^256 - 1` and `S = L - 1`.
2. **The trusted base grew, and the unsafe gate does not see it.** `sha2` and
   `curve25519-dalek` contain `unsafe` code and pick a back end by CPU.
   `cargo geiger` counts `unsafe` in the `cj3-*` crates only. A9's "zero in consensus
   crates" still holds as written; "no unsafe on the verification path" does not, and
   the README says so.
3. **`ed25519::verify` takes a message of any length.** It has to, for the G0-C8c
   vectors. A caller in another crate could pass it something other than a signing
   digest. Controls: `Transaction::verify_signature` is the only path that builds the
   message, and only `cj3-types` can construct a `SigningDigest`.
4. **The domain tags are public constants.** Another crate could build a tagged
   preimage of its own, which §2 forbids for addresses. Nothing but review prevents it.
   They are public because the tests outside the crate compare them with the ruled
   bytes and because `D_GENESIS` has no user yet.
5. **The block decoder does not enforce B3.** A block whose header carries a tuple
   other than `active` decodes when its lengths are consistent and within the limit.
   That is by design and is tested; the consensus packet must apply B3 and must not read
   a successful decode as "the size parameter is the active one".
6. **A transaction with a key that is not a point decodes.** It fails V2. The Lean
   `Error` type distinguishes V1 from V2, so P-101's conformance test has to expect
   `v2Signature` for such inputs.
7. **No well-formed block is exactly `BLOCK_MAX_BYTES` long at a stand-in tuple.** A
   block is `314 + 2m + 188k` bytes, always even, and neither `m = 112` nor `m = 288`
   reaches 1,048,576. So "holds at the boundary" is shown three ways instead: a block
   accepted against its own length as the limit and rejected against one less; the
   largest block that fits the real limit round-trips and one more transaction has no
   encoding; and any input of 1,048,577 bytes is rejected for its length before a field
   is read while inputs of 1,048,576 and 1,048,575 bytes never are.
8. **The fuzz job is a smoke run.** 740,300 runs from a fixed seed. Valid signatures
   and valid blocks reach the fuzzer only through the seed inputs and through values
   built from the input; libFuzzer will not find a valid signature by mutation.
9. **The two derivations share an author.** They share no code and no library, so they
   check implementation. They do not check my reading of the specification; Al's review
   does.
10. **A behaviour change for existing tools.** `SisParameters::new` now rejects a
    `beta_squared` above `32767^2`. Every tuple in `bench/p003-sis` and
    `bench/p004-admission` is far below it.
11. **Overflow and allocation.** `188 * count` and `2 * m` are computed in 64 bits and
    cannot wrap; every `usize` conversion is checked; the solution and the transaction
    list are allocated only after their lengths are known to fit the input, which is at
    most `BLOCK_MAX_BYTES`. A count of `u32::MAX` is tested.
12. **Panic paths.** Outside test modules, `cj3-types` has no `unwrap`, `expect` or
    `panic!`; one `debug_assert!` in the fixed-array writer, which every layout's unit
    test exercises; and two constant-index reads of a `[u8; 32]`. The new code in
    `cj3-classes` has none.

No Critical. Nothing found needed a STOP.

### 4.2 Points for Al — noticed, recorded, not decided

1. **The two dependencies** (resume 4.4): `sha2 =0.10.9` and `curve25519-dalek =4.1.3`.
   The alternative to the second is field and group arithmetic written in `cj3-types`:
   no dependency and no `unsafe`, and several hundred lines of new arithmetic that only
   the vectors would vouch for.
2. **`libfuzzer-sys`**: `(MIT OR Apache-2.0) AND NCSA`, C++ inside, outside the
   workspace and outside `cargo deny`'s graph. With it the fuzz lockfile also carries
   `arbitrary`, `cc`, `find-msvc-tools`, `getrandom`, `jobserver`, `shlex`
   (MIT OR Apache-2.0) and `r-efi` (MIT OR Apache-2.0 OR LGPL-2.1-or-later).
3. **The nightly long fuzz run stays a deferral.** Resume 4.5 allows a nightly
   toolchain to the D6 job alone. One line re-points the D6 Nightly job at the live gate
   if Al wants it.
4. **The status of the worked vectors and of the Ed25519 verdict file** (resume 2.7).
5. **`active` in the block codec.** I read G0-C1d, G0-C6b and G0-C8e together as: the
   block codec takes the tuple that stands for the active one and uses it for
   `SOL_MAX_BYTES`, and takes the exact length from the header's own size parameter
   (§1.5). The other reading gives the codec no argument and leaves `SOL_MAX_BYTES` to
   B3. Both accept the same consensus-valid blocks.
6. **Typed errors I fixed** (resume 2.2): the full order is §1.5. Two that may
   surprise: a 189-byte transaction is `WholeExceedsLimit`, never `TrailingBytes`; and a
   solution length above the limit is `CountExceedsLimit` even when it is also not the
   exact length.
7. **Layout tests with arbitrary bytes.** Where a tuple plays the active tuple (the
   block codec's argument, the solution codec, every block), the tests use the two
   measured stand-ins. The tests of the 20-byte layout itself, and the header fuzz
   target, put arbitrary values in those 20 bytes, because a layout has to be tested
   over all its values. They are labeled as layout bytes that stand in for no tuple. If
   resume 2.3 reaches them, they change to the stand-ins and lose that coverage.
8. **Edits no clause named.** One half-sentence left Protocol Spec §5.2 (it said the
   bench file still calls the q = 17 fixture nonnormative, which resume 3.2 made
   false). `unit-and-property-tests` gained the step that re-derives the vectors, and
   `dependency-audit` the audit of the fuzz lockfile. The README gained a limitation
   about the dependencies and rows for `bench/p007-vectors` and `fuzz`.
9. **Stale text I left, because resume 3.1 says "and no others".**
   `spec/Spec/Tx.lean` line 119 still calls the strict canonical decoder "unresolved"
   without naming an SI. The README's architecture diagram still labels `spec/` as
   "planned Lean rules and vectors".
10. **`cj3-classes` API changes**: `Sis` lost its const generic, and a `SisSolution` is
    16-bit coefficients. Both follow from the ruling; both touch the solver and one
    bench test.

### 4.3 A1–A11, banned identifiers, unsafe, TBD integrity

| Axiom | What I looked for | Result |
|---|---|---|
| A1 | A self-reported field in a layout | None. The header and transaction carry exactly the fields of §6.1 and §7. |
| A2 | A miner-chosen input to the instance | None. The `instance_seed` preimage is `network_id`, `height`, `parent_hash`, `beacon_out`; a test changes nothing else. |
| A3 | A clock or metadata reaching the checker | None. `check_sis` is unchanged; the solution decoder takes bytes and a tuple. |
| A4 | Quality entering fork choice | Nothing here touches fork choice. |
| A5 | A float, or a signed or wrapping amount | None. Amounts are `u64` in `Amount`; the only signed integers are SIS coefficients. The source-policy gate found no `f32` or `f64`. |
| A6 | A check smuggled into another | `verify_signature` does not test `from == addr(pubkey)`; V3 stays a separate check, and its doc says so. |
| A7 | A panic or a fail-open path on input | None found (finding 12). The one arm that cannot run in `verify` rejects. |
| A8 | Rust deciding what Lean owns | No Lean definition, theorem, vector or exported string changed. The decoder's size-test-first order mirrors the Lean `v1`. |
| A9 | Foreign code on a trusted path | No C or C++ in any `cj3-*` crate. libFuzzer's C++ is in fuzz binaries only. Finding 2 states the limit. |
| A10 | A claim beyond the evidence | The README calls the fuzz job a bounded smoke run and the dependencies' `unsafe` by name; the vector files state their status, or that they claim none. |
| A11 | A verification claim without a pointer | Each claim in §3 names its command; the hosted run is on the PR. |

The source-policy gate passed over 28 Rust files, so none of `solve_time`,
`work_score`, `reported_*` or `self_reported` is present. `unsafe` in `cj3-*` crates is
zero in 11 packages. P-1, P-4, P-7, P-9, P-10, P-12 and `ACTIVE_SIZE_PARAM` are
unfilled: no library names a tuple, a network identifier, a fee or a timestamp unit,
and the two stand-in tuples appear only in test modules, the fuzz crate and `bench/`.

## 5. HANDOFF — no MERGE

[PR #21](https://github.com/Quigles1337/COINjecture3.0/pull/21) carries this change.
Its description holds the exact-head D6 run. It is marked ready and left unmerged.
Resume 4.8 keeps the review and the merge with Al, and then the G0-F ruling.

## 6. CALIBRATE

- **Diff surface:** as predicted. 76 files, all inside §1.9.
- **Risk 1, a verification that is almost §1.3:** did not materialize in the code. It
  did show up as a hole in the vectors (finding 1), which is the same risk one step
  earlier, and the adversary pass closed it.
- **Risk 2, a second encoding or a panic:** did not materialize. The fuzz targets and
  the truncation tests found nothing.
- **Risk 3, a fixture becoming a parameter:** held. The one judgment call is §4.2
  point 7.
- **Confidence against result:** MED-HIGH on the codecs was a little low: both
  derivations agreed on every byte and every verdict at the first comparison. MED on
  the fuzz job was about right, for a different reason than I gave: the targets ran on
  Linux at the first attempt, and the time went to tooling around them.
- **Surprises:** the stale record that broke the unsafe scan; rustup installing a
  toolchain because a script asked for its version; and that no block at a stand-in
  tuple can be exactly `BLOCK_MAX_BYTES` long.
- **One process improvement:** before the first local run of any tool that reads the
  build directory, point it at a fresh target directory. A shared `target/` carries
  three months of history that a hosted runner never sees.

## VERIFIED

- Preflight steps a–g and resume 1.1: the table above.
- The two derivations agree on every worked vector, every expansion vector and all 932
  Ed25519 verdicts: §3.1.
- `spec/vectors/p005-draft.json` is byte-identical before and after, and the Lean gate
  regenerates it: §3.2.
- Format, lints, tests, locked build, dependency policy and audit, source policy,
  unsafe count, fuzz properties on stable and fuzz targets on Linux: §3.3.
- The CCTV licence permits the copy, and the copy is verbatim: §1.8 and §3.1.
- No file outside the predicted surface changed: `git diff --name-only main`.

## ASSUMED

- The fuzz crate may be its own workspace, outside `cargo deny`'s graph. Safe: it is the
  cargo-fuzz layout, resume 4.5 orders cargo-fuzz targets, `deny.toml` is untouched, and
  the fact is stated wherever the crate is described so that Al can refuse it.
- A message of any length may reach `ed25519::verify`. Safe: G0-C8c cannot be met
  otherwise, and the consensus path passes the 32-byte digest only.
- `active` supplies only `SOL_MAX_BYTES` to the block codec (§4.2 point 5). Safe: the
  two readings accept the same consensus-valid blocks.
- Installing a toolchain and cargo-fuzz in WSL was within the task. Safe to undo:
  `rustup toolchain uninstall nightly-2026-11-15` and `cargo uninstall cargo-fuzz`.
- The report number is C10, after C9.

## UNKNOWN

- Whether Al accepts each proposed dependency. His review resolves it.
- The status of the worked vectors and of the Ed25519 verdict file. His review resolves
  it (resume 2.7).
- Whether the nightly long fuzz run may use a nightly toolchain (§4.2 point 3).
- Whether resume 2.3 reaches the arbitrary-byte layout tests (§4.2 point 7).
- The D6 result for the final head: recorded on PR #21 once the run completes.
