# Cycle 3 — P-003 Builder Report

**Status:** COMPLETE — MERGED AND MAINLINE GREEN
**Date:** 2026-08-16
**Packet:** P-003 — SIS genesis class
**Lane:** AUTO
**Branch:** `feat/p003-sis-genesis-class`

## 1. FRAME

### Packet and done-condition

P-003 turns ratified D1 and Protocol Spec §5 into a bounded, safe-Rust SIS problem-
class implementation. It must define the `ProblemClass` contract and genesis SIS
class, derive matrices deterministically from an already prepared 32-byte seed, check
decoded candidate vectors with checked integer arithmetic, keep the solver outside the
trusted process, and produce a reproducible parameter/asymmetry study. Candidate
`(n, m, q, beta)` values and a validation-budget recommendation must be discovered
from current primary sources, an identified estimator/toolchain, and measurements;
they must not be chosen merely because they compile or make a demo terminate.

The packet is done only when:

- the report contains a provenance matrix mapping every candidate, hardness claim,
  estimator input, solver boundary, and measurement to a primary source, exact tool
  version or commit, and falsification check;
- `cj3-classes` exposes the protocol-authorized static `ProblemClass` shape and an SIS
  implementation whose instance derivation and checker are deterministic and contain
  no clock, miner metadata, self-reported quantity, floating point, or unchecked
  arithmetic;
- matrix expansion uses SHAKE-256 with unbiased rejection sampling, with tests for
  determinism, rejection edges, dimensions, nonzero/length/coefficient constraints,
  the modular relation, the squared-norm bound, integer quality, and overflow/fail-
  closed behavior;
- wire decoding remains outside the class checker until P-007 defines the canonical
  codec; P-003 checks a decoded solution type and does not invent `D_INST` bytes or a
  serialization format;
- the reference solving experiment crosses an operating-system process boundary, is
  treated as untrusted, and has every emitted candidate rechecked by the same safe-
  Rust checker used for hostile submissions;
- raw, reproducible solve/check measurements identify the machine, build profile,
  sample count, statistic, candidate parameters, and all unsuccessful/censored runs;
  small demonstrator measurements are never presented as evidence for production-
  size hardness;
- any P-3/P-4 recommendation is traceable to that evidence, states its assumptions
  and uncertainty, and does not fill P-1, a reward curve, `R_MAX`, the mu-balance
  hook, or another Al-/Sarah-owned value;
- local and exact-head origin D6 verification pass, an explicit adversary sweep leaves
  no Critical finding, and a fresh D11/D17/private-repository gate permits merge.

`TARGET_BLOCK_TIME` remains TBD(P-006). P-003 can report measured solve envelopes and
parameter floors, but it cannot claim a final cadence match until P-006 supplies and
ratifies that target. If this prevents a defensible single P-4 selection, the packet
will report the bounded candidate set and the unresolved dependency instead of
inventing P-1.

### Lane classification against the five D17 AUTO conditions

**Classification: AUTO.**

1. **Approved and unblocked:** P-003 is `NEXT — APPROVED`, P-002 and its closeout are
   merged with exact-merge-SHA D6 green, and the live D11 state still says
   COINjecture 2.0 is blocked pending Sarah's GATE-1/GATE-2 answers. No
   `remediation-priority` signal was found at pickup.
2. **Bounded authorized surface:** ratified D1 and the approved P-003 definition
   expressly authorize the SIS trait/class, parameter search, external solver
   measurement, and recommendations. This packet does not wire the class into block
   validation, define a block/header codec, alter fork choice or difficulty, or add a
   trusted in-process solver. Any newly required V-rule, STF, beacon-verification,
   fork-choice, difficulty, or unrelated consensus behavior downgrades the packet to
   HUMAN before that edit.
3. **No formal-spec content:** `Spec/*.lean` content and protocol vector definitions
   are outside this packet and will not be changed.
4. **No decision invention:** P-003 owns evidence-backed discovery of Protocol
   parameters P-3 and P-4, not arbitrary assignment. Estimator output, primary-source
   bounds, checked-in raw measurements, and explicit assumptions must support any
   recommendation. A new hardness assumption, registry admission rule, canonical
   encoding, or ratification remains HUMAN/G0 work.
5. **No owned-TBD fill:** reward-margin shaping, `R_MAX`, the mu-balance hook, and all
   other Al-/Sarah-owned economics remain unfilled. P-1/P-2/P-11 remain P-006 work;
   domain bytes and canonical solution encoding remain P-007/G0 work. Missing values
   will be represented as typed inputs or documented dependencies, never placeholders.

### Predicted diff surface

- Research, build, measurement, verification, adversary, merge, and calibration
  evidence in `loop/reports/C3-p003-builder.md`.
- `crates/cj3-classes/Cargo.toml`, `crates/cj3-classes/src/lib.rs`, and expected
  class-local modules/tests under `crates/cj3-classes/src/` for the trait, SIS types,
  deterministic expansion, and safe checker.
- `crates/cj3-solver-sis/Cargo.toml` and `crates/cj3-solver-sis/src/` for the explicitly
  untrusted external solver/measurement boundary. The node and consensus crates will
  not depend on this binary crate.
- A P-003-specific directory below `bench/` containing a reproducible runner,
  environment manifest, candidate inputs, and raw/derived measurement artifacts.
  Python, if evidence makes it necessary, is confined to this non-consensus directory.
- Workspace manifests and `Cargo.lock` only for dependencies that clear source,
  license, advisory, native-code, unsafe, determinism, and maintenance review.
- Packet-boundary bookkeeping in `loop/STATE.md`, `loop/PACKETS.md`, and
  `loop/reports/BATCH-LOG.md` only after implementation and merge evidence exists.

No edit to `loop/LEDGER.md`, `docs/PROTOCOL_SPEC.md`, `docs/ENGINEERING_PLAN.md`,
`spec/`, `crates/cj3-consensus/`, `crates/cj3-kernel/`, `crates/cj3-beacon/`, or
`crates/cj3-types/` is predicted. If evidence requires one of those surfaces, work
pauses for re-framing and the applicable HUMAN/tripwire decision before any edit.

### Authority-and-claims pre-check

- Ajtai's reduction establishes a worst-case-to-average-case relationship only under
  stated parameter regimes and approximation factors; it does not prove that an
  arbitrary concrete matrix or chosen tuple takes a desired wall-clock time. The
  report will keep theorem, estimator model, empirical solver cost, and cadence
  projection separate.
- A lattice estimator estimates attacks on a specified model; it is not a benchmark
  and its output is not a proof that a reference solver will find a PoW witness.
- A successful toy solve validates plumbing and measurement only. It cannot validate
  a recommended security floor. Failed or timed-out runs are censored observations,
  not proof of hardness, and will remain in the raw record.
- Solver code and all of its transitive implementation are outside the trusted node
  boundary. Only the safe-Rust derivation/checker may justify acceptance.
- P-3 and P-4 outputs are P-003 recommendations feeding G0. The report will not call
  them ratified consensus constants before the gate rules on them.

### Top risks

1. **Estimator/model mismatch:** mapping CJ3's homogeneous SIS relation and norm to a
   current estimator may silently use an LWE/MLWE convention, norm, distribution, or
   attack model that does not apply. A plausible-looking bit-security number would
   then be fiction. Primary formulas and an independently checked mapping are
   required before any floor is recommended.
2. **Benchmark/cadence category error:** tractable demonstration instances may be far
   below a defensible security floor, while defensible instances may not yield a
   witness within this packet's measurement window. P-1 is also deliberately absent
   until P-006. The report must not extrapolate a toy solve into a block-cadence claim
   or hide censored samples.
3. **Checker and sampler edge failures:** biased modular sampling, signed remainder
   mistakes, dimension/allocation abuse, zero-vector acceptance, norm/quality
   division by zero, or intermediate overflow could violate A1–A3 despite ordinary
   happy-path tests. Bounds, checked operations, rejection tests, and hostile vectors
   must make these failures explicit and fail closed.

**Falsifier:** this approach is wrong if no parameter regime can be supported by an
applicable current estimator and primary-source SIS bound while also admitting a
reproducible out-of-process witness experiment and a verifier inside a measured,
bounded budget; or if implementing the class requires inventing P-1, canonical wire
bytes, reward semantics, or an unstated trust assumption. In that case P-003 will
preserve the negative evidence, omit unsupported P-3/P-4 constants, checkpoint the
packet as blocked, and stop rather than convert missing evidence into a parameter.

**Confidence:** MEDIUM-LOW. The deterministic checker and process isolation are
bounded, but concrete SIS parameter mapping, witness generation at meaningful sizes,
and honest interpretation of estimator-versus-benchmark evidence are specialized and
may expose a genuine feasibility boundary.

## 2. BUILD

### Tripwire log — interpretation and scope re-frame 1

Before implementation, primary-source review found a nonblocking conflict between
Engineering Plan A10 and Protocol Spec §5.2:

- A10 requires hardness assumptions to be labeled as assumptions everywhere.
- §5.2 says the sampled SIS distribution's hardness is “provable, not assumed.”
- Ajtai and the later Micciancio–Regev/GPV results instead give *conditional*,
  asymptotic worst-case-to-average-case reductions in stated parameter regimes. They
  neither prove the underlying worst-case lattice problems hard nor turn a concrete
  estimator output or wall-clock duration into a theorem.

Per the INTERPRETATION tripwire, P-003 proceeds on the stricter A10 reading and logs
the wording defect as `SI-001` in `loop/reports/SPEC-ISSUES.md`; correcting normative
spec wording remains G0/HUMAN work. This expands the predicted diff by that one
required issue-log file. It does not expand implementation scope or touch a
consensus-semantic file, so the AUTO classification remains valid.

The initial containerized estimator smoke sequence also exposed three distinct
environment-contract failures rather than a repeated estimator result: the official
Sage development image lacks `git`; its shell entrypoint re-splits a one-line `-c`
argument; and Sage 9.5 omits the working directory from `sys.path`. No estimator code
ran in those failed probes. Source SHA-256 comparison, a direct Sage entrypoint, and
an explicit `PYTHONPATH=/lattice-estimator` resolved the causes; the next run
reproduced the official documented `n=113, q=2048, m=276, length_bound=512` result of
approximately `2^47` operations. The failures were not retried unchanged and did not
reach the REPETITION stop condition.

Research and implementation continue below.

### Tripwire log — interpretation and scope re-frame 2

Protocol Spec §5.2 refers to `s_max(P-4)`, but the P-4 tuple does not contain that
quantity and no definition exists elsewhere. Under the same INTERPRETATION rule,
P-003 logged `SI-002` and uses only the stricter consequence already implied by the
global norm: reject a coefficient when `s_i^2 > beta_squared`, before accumulation.
This changes no valid solution set and adds no wire representation. P-007/G0 still
owns the coefficient width, strict decoder, and wording repair.

### Research and parameter result

The selected candidate *family*, rather than an isolated hand-tuned tuple, is:

```text
q(n) = first prime strictly greater than n^2
m(n) = 2 n ceil(log2(q(n)))
beta_squared(n) = m(n)
```

This makes `2^m > q^n`; by pigeonhole, two binary vectors share a syndrome and their
nonzero difference is a `{−1,0,1}^m` kernel vector with squared norm at most
`m = beta_squared`. It also has
`q / (beta sqrt(n ln n)) = Theta(n / ln n)`, so the family—not an arbitrary finite
tuple—follows the growth direction required by the cited Micciancio-Regev/GPV SIS
reductions. The concrete points remain conditional security candidates, not proofs.

The pinned estimator sweep is in `bench/p003-sis/estimator-results.jsonl`. The default
and rough/ADPS16 models disagree materially. At the largest sampled point,
`(128, 3840, 16411, 3840)`, they report log2 operation estimates 165.520 and 143.956,
respectively. That point is the first *sampled* candidate above an explicitly assumed
128-bit threshold under both models. Neither the threshold nor the estimate is called
a ratified CJ3 security level.

### Implementation result

- `cj3-classes` now exposes a static `ProblemClass` contract, fixed-point `Quality`,
  structured invalid reasons, a validated SIS tuple, deterministic column-major
  SHAKE-256 expansion with rejection sampling, a decoded solution type with no codec,
  and a safe integer checker. The checker validates exact length, nonzero, the derived
  per-coefficient bound, global squared norm, every modular row, and quality with
  checked/u128 intermediates.
- `Sis<const ID: u16>` requires the production registry identifier to be bound
  explicitly. No numeric genesis ID appears because no authority source assigns one.
- The independent Python `hashlib` vector is preserved at
  `bench/p003-sis/SHAKE-VECTOR.md` and locked by a Rust unit test.
- `cj3-solver-sis` is a standalone binary. It constructs a full-rank integer basis for
  the modular kernel, applies exact integral LLL, converts only coefficients that fit
  the decoded type, and rechecks a candidate before standard output. Its space-
  separated text is labeled benchmark-only/noncanonical. No trusted crate depends on
  the solver.
- `sha3 = 0.10.9` is exact-pinned with default features disabled. The numerically newer
  `sha3` 0.12.0 package no longer exported `Shake256` in the inspected API, so the
  signed RustCrypto 0.10.9 SHAKE-capable release was selected rather than pretending a
  fixed-output SHA-3 hash was the required XOF.
- `puremp = 0.2.4` is confined to the solver with default features disabled and only
  `std` plus `lattice` enabled. That selects its pure-Rust exact integer/rational LLL;
  float, FFI, CLI, and unrelated algorithms are not enabled. A pre-verification
  `cargo deny` run correctly rejected the initial versionless path dependency on
  `cj3-classes`; adding its exact workspace version corrected that single cause.

### Reproducible measurements

The environment is recorded at `bench/p003-sis/ENVIRONMENT.md`; runners and raw JSON
Lines are in the same directory.

- Exact-LLL solver, release profile: `n=8` solved/rechecked 5/5 with 0.244 s median;
  `n=12` solved/rechecked 5/5 with 1.184 s median; `n=16` solved/rechecked 3/3 with
  3.897 s median. At `n=24`, LLL completed in 19.752 s but returned no row within the
  admitted bound. The negative result is preserved and is not interpreted as no
  solution existing.
- Full-path checker fixture, release profile, 200 samples per tuple: the provisional
  `(128, 3840, 16411, 3840)` point measured 4.366 ms median, 5.589 ms p95, and
  7.818 ms maximum on the named Ryzen host. The valid zero-matrix/unit-vector fixture
  forces traversal of all matrix terms but is not a hardness-distributed instance.

### P-3 and P-4 recommendations

- **P-3:** provisional `VALIDATION_BUDGET = 15 ms` on the reference host recorded in
  `bench/p003-sis/ENVIRONMENT.md`. This is 2.68× the measured p95 and 1.92× the
  observed maximum for the largest candidate. G0 must ratify the hardware definition
  and budget; this single-host sample is not a platform guarantee.
- **P-4:** provisional minimum candidate
  `(n, m, q, beta_squared) = (128, 3840, 16411, 3840)`, conditional on G0 accepting
  the explicitly labeled 128-bit model threshold and estimator mapping. It guarantees
  existence of a binary witness, excludes `q e_i`, clears both pinned estimator models
  at that assumed threshold, and verifies within the proposed P-3 budget.
- **Unresolved:** this is not a final cadence-compatible P-4. P-1 belongs to P-006,
  and the bundled LLL demonstrator does not solve the candidate. P-006/G0 must connect
  a reproducible solver envelope to the chosen cadence, revise the family without
  losing its reduction/security properties, or reject the recommendation.

### Provenance matrix

| Claim or artifact | Primary/exact source | Mapping and falsification |
|---|---|---|
| Random modular matrices have a conditional worst-case-to-average-case lattice reduction lineage | Ajtai, ECCC TR96-007: <https://eccc.weizmann.ac.il/report/1996/007/>; Micciancio-Regev author copy and publication record: <https://cseweb.ucsd.edu/~daniele/papers/Gaussian.html> | Supports the reduction lineage only. `SI-001` falsifies any reading that the source worst-case problem or a concrete tuple is unconditionally hard. |
| Modern polynomial-parameter SIS/ISIS reduction conditions and Gaussian sampling | Gentry-Peikert-Vaikuntanathan, ECCC TR07-133: <https://eccc.weizmann.ac.il/report/2007/133/> | The family is checked against the asymptotic growth conditions; finite ratios and estimator outputs are reported separately. A bounded ratio or trivial `beta >= q` would falsify the mapping. |
| SHAKE-256 XOF semantics | NIST FIPS 202: <https://csrc.nist.gov/pubs/fips/202/final> | Protocol seed bytes feed SHAKE-256; 32-bit little-endian candidates use a rejection ceiling. `bench/p003-sis/SHAKE-VECTOR.md` cross-checks RustCrypto with Python `hashlib`; `u32::MAX` at q=17 falsifies modulo-only sampling. |
| Rust SHAKE implementation | RustCrypto signed `sha3-v0.10.9` release: <https://github.com/RustCrypto/hashes/releases/tag/sha3-v0.10.9>; exact version in `Cargo.lock` | Default features are off. Determinism/rejection tests and full D6 supply-chain checks must pass; any vector mismatch or advisory fails the choice. |
| Concrete attack estimates | `malb/lattice-estimator` commit `3e48ef421ec256afddb3e7d2249a77eab6e9ba12`: <https://github.com/malb/lattice-estimator/commit/3e48ef421ec256afddb3e7d2249a77eab6e9ba12>; source SHA-256 pinned by `run-estimator.ps1` | Inputs use Euclidean norm and `sqrt(beta_squared)`. Both default and rough models are retained. Model divergence, a newer attack, or a mapping correction can falsify the proposed floor. |
| Exact external LLL implementation | `puremp` release 0.2.4 commit `1bc6985a977675339f84c9a88fdbddcc9966c7fe`: <https://github.com/KarpelesLab/puremp/commit/1bc6985a977675339f84c9a88fdbddcc9966c7fe>; exact lockfile entry | Only `std,lattice` features are enabled. Every output is rechecked. A checker rejection, dependency-policy failure, or trusted-crate dependency would falsify the boundary. |
| Candidate and validation measurements | `bench/p003-sis/*-results.jsonl`, `ENVIRONMENT.md`, and the committed runners | Raw successes, failure, limits, hashes, build profile, sample count, statistics, and host are present. Reruns outside the margin or a target host over 15 ms falsify P-3. |

## 3. VERIFY

### Original exact-head origin verification

Implementation commit `4516d6bc71612c1b64ec6fb662d61082b0fffb77` was pushed to
`origin/feat/p003-sis-genesis-class` and opened as draft PR #5. D6 CI run
`31925679014` completed all eleven jobs successfully on that exact SHA: format,
clippy, dependency policy, dependency audit, zero-unsafe audit, tests, four explicit
phase gates, and locked build.

### Post-adversary evidence replay

The hardened estimator runner executed the exact Sage image manifest
`sha256:ec32d9752b3a11c628103ca6802db890b63cbe9bb480cfea02de09656ecc84a2`
directly against a clean checkout at estimator commit
`3e48ef421ec256afddb3e7d2249a77eab6e9ba12`. The checkout and script were read-only,
the container root was read-only, temporary storage was a bounded 64 MiB `tmpfs`, and
networking was disabled. All nine normalized JSON records matched
`estimator-results.jsonl` exactly (`ESTIMATOR_RESULTS_MATCH=9`). An intentional
untracked-file probe then exited 1 at the cleanliness check before Docker execution;
removing the probe restored a clean checkout.

### Post-adversary local D6

The complete local D6 sequence passed after the hardening edits:

- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`;
- `cargo deny --locked --all-features check`;
- `cargo audit --deny warnings` against 1,216 loaded RustSec advisories and 21 locked
  dependency packages;
- `scripts/ci/verify-geiger.ps1`, reporting `SOURCE_POLICY=PASS files=14 crates=10`
  and zero unsafe use in all ten `cj3-*` packages;
- `cargo test --workspace --all-targets --all-features --locked`, including all seven
  SIS checker/sampler tests and all four external-solver tests;
- all four D6 phase-gate scripts, each returning its explicit `NOT_YET_ADMITTED`
  owner rather than claiming an unimplemented test ran; and
- `cargo build --workspace --all-targets --all-features --locked`.

Post-hardening commit `70278e4826b5f021f42e50dbc33e5268b8f88ef5` passed every
origin D6 job in run `31926469157` before merge.

## 4. ADVERSARY

### Formal security diff scan

Codex Security scan `aabd95d5-7900-4200-96f5-86f9d8810b31` reviewed the exact
P-003 range
`4b0102b8e03a1701d2196ed758aeff768b984ef3..4516d6bc71612c1b64ec6fb662d61082b0fffb77`
under a repository-specific threat model. It sealed with complete coverage and zero
reportable findings. Its readable report is preserved byte-for-byte at
[`P-003-security-scan.md`](./P-003-security-scan.md), SHA-256
`9C2E0C5FC428139D266174BB573EA6183BEE600C6A044874199BC557239F00BC`.

Two plausible source-level candidates were validated and rejected as current
security findings, while their boundary-change conditions remain explicit:

1. `SisInstance::from_column_major` can construct a canonical explicit matrix with no
   committed-seed provenance. Repository-wide callers and `cargo tree --invert`
   showed only tests and measurement tooling; no trusted runtime consumes the crate.
   P-006/P-007/P-101 must re-evaluate or narrow this API before a hostile admission
   path can accept an instance.
2. `SisParameters::new` proves arithmetic representability but has no operational
   matrix-entry ceiling, and `derive_instance` allocates `n * m` entries. The only
   current attacker-controlled source is local argv supplied by the same developer
   invoking the short-lived, explicitly untrusted solver. Before any node, RPC, wire,
   or consensus caller exists, admission must restrict values to the ratified tuple
   or a hard resource ceiling and allocation must fail closed.

### Manual hostile-evidence sweep and remediations

The formal scan and a separate line-by-line adversary pass found two non-reportable
but merge-relevant governance/reproducibility defects:

- The estimator runner checked the estimator HEAD and one source hash but built the
  checkout's `docker/Dockerfile.dev`. A dirty Dockerfile or requirements file could
  therefore execute even while HEAD remained pinned; the Dockerfile also named the
  mutable `sagemath/sagemath:9.5` tag and unpinned developer dependencies. The runner
  now refuses dirty/untracked worktrees and performs no build. It runs the exact base
  manifest by digest with read-only mounts/root, bounded temporary storage, and no
  network. The resulting nine records match the committed evidence exactly.
- The Protocol Spec names SHAKE-256 and rejection sampling but leaves candidate word
  width, byte order, rejection ceiling, and consumption rules undefined. Independent
  implementations could therefore derive different matrices without either being
  biased. `SI-003` records this as a G0/P-007 HUMAN decision, and the P-003 fixture is
  now explicitly nonnormative. No consensus or wire rule was silently created.

The pass also rechecked A1–A11 boundaries: the solver remains out of process and
outside the trusted dependency graph; checker output is pure and exact; class ID,
codec bytes, cadence, economics, and Sarah-owned values remain unfilled; provisional
P-3/P-4 claims stay qualified; and no Critical finding remains.

## 5. MERGE

At merge time, all five D17 AUTO conditions were re-derived from current evidence:

1. P-003 remained approved and unblocked. P-002 and its closeout were on `main`; the
   CJ2 capacity scan found neither answered-gate evidence nor a
   `remediation-priority` signal.
2. The diff stayed within the authorized class/checker, external solver, research
   evidence, dependency manifests/lockfile, and packet-report surfaces. No block,
   fork-choice, retarget, state-transition, beacon-verification, or reward behavior
   was added.
3. No `Spec/*.lean` content or vector definition changed.
4. The provisional P-3/P-4 recommendations remained evidence for G0, not ratified
   constants. `Sis<const ID: u16>` left the numeric registry value unassigned, and
   `SI-001` through `SI-003` preserved specification decisions for HUMAN owners.
5. No Al- or Sarah-owned cadence, economics, reward curve, margin hook, class ID,
   domain bytes, or canonical codec value was filled.

The merge gate also re-read the repository as `PRIVATE`, confirmed exact canonical
fetch/push remotes, found a clean worktree, matched local/origin/PR head at
`70278e4826b5f021f42e50dbc33e5268b8f88ef5`, matched PR base and merge base to
`4b0102b8e03a1701d2196ed758aeff768b984ef3`, observed all eleven checks successful,
and read PR #5 as mergeable.

- PR: <https://github.com/Quigles1337/COINjecture3.0/pull/5>
- Final exact-head PR CI:
  <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31926469157>
- Exact PR head: `70278e4826b5f021f42e50dbc33e5268b8f88ef5`
- PR CI interval: 2026-08-16 04:22:10Z–04:25:05Z
- Merge time: 2026-08-16 04:26:40Z
- Merge SHA: `e00561572c48137d535a44bcce55c04ad7db732b`
- Post-merge `main` CI:
  <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31926645964>
- Main CI interval: 2026-08-16 04:26:42Z–04:29:26Z

Both the final feature head and exact merge SHA passed all eleven D6 jobs. The first
session was interrupted while watching the mainline run, so the resumed session read
the completed run directly from GitHub and verified its `headSha`, conclusion, and
every job before recording success. The merge SHA and mainline result necessarily
postdate the implementation commit; this record and the durable security report are
being ferried through evidence-only branch `feat/p003-closeout`.

## 6. CALIBRATE

### Predictions versus outcomes

- **Diff surface:** the prediction substantially held. P-003 changed the two expected
  crates, added the benchmark/evidence directory, updated `Cargo.lock`, and wrote the
  packet report. The only re-framed expansion was `SPEC-ISSUES.md`; governing spec,
  LEDGER, formal-spec, consensus, kernel, beacon, and types files stayed untouched.
- **Risk 1 — estimator/model mismatch:** materialized. Default and rough estimator
  models diverged by tens of log2 operations, and the conditional reduction lineage
  did not prove concrete hardness. Pinning the exact estimator source, retaining both
  outputs, and labeling the 128-bit target as an assumption contained the claim.
- **Risk 2 — benchmark/cadence category error:** materialized. The exact LLL
  demonstrator solved only toy tuples and did not solve the provisional P-4 point;
  P-1 remains P-006-owned. The packet therefore reports a provisional minimum and an
  unresolved cadence dependency instead of claiming a deployable floor.
- **Risk 3 — checker/sampler edge failure:** the implementation itself survived the
  deterministic, rejection, norm, modular, overflow, and zero-vector tests. The
  adversary pass instead exposed the adjacent consensus ambiguity: word width and
  byte order were not normative. `SI-003` and a nonnormative fixture preserved G0 and
  P-007 ownership.
- **Falsifier outcome:** the cadence portion partially fired because the target tuple
  lacks a demonstrated witness-generation envelope. That did not defeat the safe
  checker or evidence packet; it prevented final P-4 ratification and remains an
  explicit P-006/G0 dependency.
- **Confidence calibration:** MEDIUM-LOW was appropriate. The bounded safe-Rust
  implementation and verifier budget were reproducible, while concrete hardness,
  solver cadence, and specification conventions retained material uncertainty.
- **Surprise:** the estimator commit pin and core-source hash were insufficient for
  reproducibility while the runner still executed a dirty-capable Docker build from
  a mutable base tag. The digest-pinned, networkless, read-only replay produced the
  same nine records and showed that environment provenance belongs inside the
  evidence claim, not beside it.

**One process improvement for P-004:** establish the generic harness's evidence
envelope before its first calibration run: exact source commit, complete clean-tree
check, immutable runtime digest, disabled network, bounded temporary resources, and a
machine-readable input/output receipt must be mandatory runner behavior rather than
post-benchmark documentation.

## VERIFIED

- The P-003 pickup preflight ran from
  `C:\Users\LEET\COINjecture3.0` on clean `main` at
  `4b0102b8e03a1701d2196ed758aeff768b984ef3`, equal to `origin/main`.
  Evidence: local preflight output ending in `P003_PREFLIGHT=PASS`.
- GitHub reported `Quigles1337/COINjecture3.0` as `PRIVATE`; origin fetch and push
  both matched the canonical HTTPS URL; D17 was RATIFIED; P-003 was
  `NEXT — APPROVED`; D11 remained `cj2-blocked-on-external`; and no CJ2
  `remediation-priority` signal was found.
  Evidence: the same local preflight output.
- The supplied research survey and autonomous prompt remained byte-identical to Al's
  hashes: SHA-256 `0A960F8DA6BF35315D60BBA1AD317DC99AD4F6910F53F684DB61FB639634D294`
  and `82AE71BB9CB1D296FC3FFD2BD7419FABA0ED3ABDB650B103823B5B83F7DE5971`.
  Evidence: `Get-FileHash` results in the pickup preflight.
- P-002's evidence-only closeout passed all eleven mainline D6 jobs on the exact
  starting SHA.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31923976170>.
- The final P-003 feature head passed every origin D6 job.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31926469157>
  on `70278e4826b5f021f42e50dbc33e5268b8f88ef5`.
- PR #5 merged that exact head as
  `e00561572c48137d535a44bcce55c04ad7db732b`.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/pull/5> and matching local
  `origin/main` readback.
- The exact-merge-SHA mainline pipeline passed all eleven jobs.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31926645964>.
- The sealed zero-finding security report is durable and byte-identical to the
  generated artifact.
  Evidence: [`P-003-security-scan.md`](./P-003-security-scan.md) and matching source/
  destination SHA-256
  `9C2E0C5FC428139D266174BB573EA6183BEE600C6A044874199BC557239F00BC`.
- The 2026-08-16 D11 resume check found GATE-1 and GATE-2 still listed under
  `Blocked / awaiting` in the CJ2 network loop state and found no answer, capacity-
  release, or `remediation-priority` signal.
  Evidence: `C:\Users\LEET\COINjecture2.0-network\loop\STATE.md:73-83` plus the
  bounded two-checkout search performed at closeout.

## ASSUMED

- The current CJ2 loop files are an accurate reflection of whether Sarah's answers
  have arrived. This is safe only as a packet-boundary capacity observation and is
  re-checked before P-004; any contrary live instruction or gate-clear evidence wins
  immediately under D11.
- A prepared `[u8; 32]` input to `ProblemClass::derive_instance` is the instance seed,
  not the place to assign `D_INST` bytes. This follows Protocol Spec §5.1's trait
  signature and preserves P-007's canonical-domain ownership.

## UNKNOWN

- Whether G0 accepts the current homogeneous-SIS estimator mapping, the assumed
  128-bit operation threshold, provisional P-3, or provisional P-4 remains unknown;
  the packet supplies evidence and recommendations, not ratification.
- The normative SHAKE candidate width, byte order, rejection-consumption rule, class
  identifier, and canonical solution codec remain unknown until P-007/G0 resolves
  `SI-002` and `SI-003`.
- Final block cadence is unknown because P-1 belongs to P-006. P-003 can measure and
  recommend parameter floors, but P-006/G0 must resolve cadence integration.
- Reward-margin shaping remains intentionally unknown and unfilled for Al + Sarah at
  the D16 reveal.
