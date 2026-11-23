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
