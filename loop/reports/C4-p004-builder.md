# Cycle 4 — P-004 Builder Report

**Status:** COMPLETE — PR AND EXACT-MERGE-SHA D6 GREEN
**Date:** 2026-08-16
**Packet:** P-004 — admission bench harness
**Lane:** AUTO
**Branch:** `feat/p004-admission-bench`

## 1. FRAME

### Packet and done-condition

P-004 must build a reusable, non-consensus admission harness that evaluates a
candidate problem class against the evidence contract in Protocol Spec §5.1: a
published and explicitly scoped hardness assumption, a parameter floor that addresses
known attacks, measured solve/check asymmetry, and a checker-cost comparison against
the still-provisional P-3 validation-budget recommendation. It must then use the
harness on COINjecture 2.0's legacy problem classes as calibration, without modifying
2.0, importing its trust assumptions into CJ3, or admitting any class to the CJ3
registry.

The packet is done only when:

- the harness accepts a class-neutral candidate description and produces a
  deterministic, machine-readable record plus a human-readable admission report;
- the record distinguishes supplied provenance from measured evidence and represents
  missing evidence explicitly rather than converting absence into a passing default;
- measured runs identify the candidate, parameters, solver/checker boundary, command
  or adapter version, environment, sample count, statistic, failures, timeouts, and
  raw result artifact;
- all external candidate/solver output is treated as hostile data, bounded before
  allocation, parsed without panics, and evaluated outside consensus code;
- every locally available 2.0 legacy class is inventoried from an exact source
  revision and either run through a faithful calibration adapter or recorded as
  structurally/not-measurably ineligible with the exact missing requirement;
- calibration conclusions keep four questions separate: whether the instance
  distribution has a stated hardness basis, whether parameters clear known attacks,
  whether the solver/checker asymmetry is measured, and whether the class satisfies
  A1–A3's protocol-derived/pure-checker shape;
- no calibration result assigns a CJ3 `CLASS_ID`, changes the genesis registry,
  ratifies an ADR, or claims a legacy class is admitted;
- local verification and the exact-head origin D6 pipeline pass, an explicit
  adversary sweep finds no unresolved Critical, and a fresh D11/D17/private-repository
  gate permits merge.

A negative calibration is a valid result. P-004 measures and reports the legacy
classes; it does not repair a class by inventing a sampleable hard distribution,
consensus parameter floor, canonical encoding, or reward rule.

### Lane classification against the five D17 AUTO conditions

**Classification: AUTO.**

1. **Approved and unblocked:** P-004 is `NEXT — APPROVED`; P-003 and its durable
   closeout are merged, and exact-merge-SHA D6 run 31952554318 is green on
   `5ade307e0eb6f7fe8155129fd66e7a9c7b910a5e`. A fresh pickup read of the 2.0 loop
   still places GATE-1/GATE-2 under `Blocked / awaiting`, with no
   `remediation-priority` signal.
2. **Bounded authorized surface:** the approved packet expressly authorizes an
   offline admission harness, evidence artifacts, and read-only calibration against
   legacy 2.0 classes. It does not authorize registry admission, class IDs, changes
   to `ProblemClass`, block validation, beacon verification, fork choice, difficulty,
   rewards, or any other consensus-semantic behavior. A need to alter one of those
   surfaces downgrades the affected path to HUMAN before the edit.
3. **No formal-spec content:** P-004 will not edit `Spec/*.lean`, define protocol
   vectors, or reinterpret V1–V9/STF behavior. Any such need is HUMAN-lane work.
4. **No decision invention:** the harness may record evidence-backed outcomes under
   the already ratified admission criteria, including rejection or unknown. It may
   not turn a benchmark threshold, legacy constant, measurement timeout, or adapter
   convenience into a consensus parameter, hardness claim, or ADR ratification.
5. **No owned-TBD fill:** P-1/P-2/P-11 remain P-006 work; canonical encodings and
   domain bytes remain P-007/G0 work; reward shaping, `R_MAX`, the mu-balance hook,
   and every other Al-/Sarah-owned value remain unfilled. Provisional P-3/P-4 evidence
   may be cited only with its current status and uncertainty.

### Predicted diff surface

- Research, source-provenance, build, raw measurement, verification, adversary,
  merge, and calibration evidence in `loop/reports/C4-p004-builder.md`.
- A P-004-specific tree under `bench/p004-admission/` containing the generic runner,
  bounded input/output schema, candidate adapters or fixtures, environment/provenance
  manifests, reproducible commands, raw machine-readable results, and a generated or
  committed human-readable calibration report.
- Root/workspace manifests and `Cargo.lock` only if a safe-Rust bench crate or a
  narrowly reviewed dependency is needed to make the generic runner testable by D6.
- A narrowly scoped test or CI hook only if the existing workspace pipeline cannot
  exercise the admission harness and its hostile-input behavior.
- Packet-boundary bookkeeping in `loop/STATE.md`, `loop/PACKETS.md`, and
  `loop/reports/BATCH-LOG.md` only after implementation, origin verification, and
  merge evidence exist.

No edit to `loop/LEDGER.md`, `docs/PROTOCOL_SPEC.md`, `docs/ENGINEERING_PLAN.md`,
`spec/`, or any `crates/cj3-*` consensus/runtime source is predicted. The 2.0
checkout is a read-only evidence source and must remain byte-for-byte untouched. If
the harness cannot be built without changing a consensus-semantic surface, work
pauses for the SCOPE/HUMAN tripwire before any such edit.

### Authority-and-claims pre-check

- A fast checker does not establish a hard sampled distribution, and a slow solver
  on one workstation does not establish a security floor. The report will keep
  hardness citation, parameter analysis, measured cost, and structural A1–A3 fit as
  independent fields.
- A legacy class can be useful or NP-hard in the worst case while its actual 2.0
  instance distribution is unsuitable for consensus security. The harness will not
  substitute the problem name or worst-case complexity label for distributional
  evidence.
- Legacy wall-clock or self-reported scoring fields are calibration evidence about
  2.0, never inputs to a CJ3 admission decision. Any adapter will compute only what
  can be independently derived or checked from captured inputs and outputs.
- P-003's P-3/P-4 outputs are provisional recommendations feeding G0. P-004 may
  compare measured checker cost with the P-3 recommendation but may not call that
  budget ratified.
- Source readback and execution against 2.0 must be local/read-only. P-004 will not
  fork, comment on, watch, star, open an issue or PR against, or otherwise notify an
  external repository or person.

### Top risks

1. **Semantic-adapter laundering:** a convenience adapter could replace a 2.0
   miner-authored instance, trusted score, or timing field with a cleaner local value
   and accidentally make an ineligible class appear A1–A3 compatible. Calibration
   must preserve the legacy semantics and mark structural gaps instead of repairing
   them silently.
2. **Benchmark category error:** process startup, parser overhead, tiny fixtures,
   censored solves, host load, or incomparable parameters could dominate the observed
   ratio. Raw per-sample data, explicit phases, warmups, timeouts, environment data,
   and honest unknown/reject states are required before any conclusion.
3. **Hostile-input and reproducibility failure:** generic manifests, external solver
   output, large fixtures, paths, or commands can introduce injection, unbounded
   allocation, traversal, or irreproducible local-state dependencies. The runner must
   use a constrained schema, fixed adapter registry or argument-safe process API,
   bounded reads, typed errors, exact source revisions, and fail-closed validation.

**Falsifier:** this approach is wrong if a class-neutral harness cannot evaluate the
current `ProblemClass` evidence contract without changing consensus semantics, or if
the available 2.0 source and fixtures cannot support a faithful, reproducible
calibration without inventing a distribution, parameter floor, checker, or trust
boundary. In that event P-004 will preserve the negative evidence, identify the
unmet done-condition, checkpoint as blocked, and stop rather than manufacture an
admission result.

**Confidence:** MEDIUM. A bounded offline harness and explicit evidence schema are
tractable, but faithful calibration depends on the legacy implementations' actual
instance/scoring boundaries and may yield structurally negative or non-comparable
results rather than timing ratios.

## Initial evidence ledger

### VERIFIED

- Canonical checkout: `C:\Users\LEET\COINjecture3.0`; the deleted decoy path tested
  false at pickup. Evidence: 2026-08-16 PowerShell preflight readback.
- `main`, `origin/main`, and local `HEAD` matched
  `5ade307e0eb6f7fe8155129fd66e7a9c7b910a5e`; worktree was clean. Evidence: pickup
  `git rev-parse` and `git status --porcelain=v1` readback.
- Origin fetch/push both exactly equal
  `https://github.com/Quigles1337/COINjecture3.0`; GitHub reported `PRIVATE` with
  default branch `main`. Evidence: pickup `git remote -v` and `gh repo view` readback.
- P-003 closeout merge SHA `5ade307e0eb6f7fe8155129fd66e7a9c7b910a5e`
  passed all eleven jobs in D6 run 31952554318. Evidence:
  https://github.com/Quigles1337/COINjecture3.0/actions/runs/31952554318
- The supplied survey and prompt hashes match exactly:
  `0A960F8DA6BF35315D60BBA1AD317DC99AD4F6910F53F684DB61FB639634D294`
  and `82AE71BB9CB1D296FC3FFD2BD7419FABA0ED3ABDB650B103823B5B83F7DE5971`.
  Evidence: pickup `Get-FileHash -Algorithm SHA256` readback.
- D17 is RATIFIED and P-004 is NEXT/APPROVED. Evidence: `loop/LEDGER.md` live
  overlay/effective ratifications and `loop/PACKETS.md` queue row.
- D11 remains clear for CJ3 at pickup: `C:\Users\LEET\COINjecture2.0-network\loop\STATE.md`
  lines 73–82 place GATE-1 and GATE-2 under `Blocked / awaiting`, and no current
  `CAPACITY_FLAG: remediation-priority` or gate-cleared signal was found in either
  local 2.0 checkout. Evidence: 2026-08-16 read-only `rg` scan and Al's live resume
  ruling.

### ASSUMED

- The local `COINjecture2.0-network` checkout is the operational 2.0 loop-state source
  because it is the only local 2.0 checkout containing `loop/STATE.md`; this is safe
  for pickup because Al's newer live resume ruling independently confirms the same
  capacity condition and directs a mechanical branch based on whether the answers
  arrived.

### UNKNOWN

- The exact inventory, languages, executable boundaries, and fixture availability of
  the 2.0 legacy problem classes. Resolution: read-only source inventory after this
  FRAME commit.
- Whether every legacy class can be executed reproducibly on this host without
  installing or trusting extra tooling. Resolution: provenance/toolchain audit and a
  bounded calibration dry run; unsupported classes remain explicit unknown/reject,
  never passing defaults.

## 2. BUILD

### Exact legacy inventory and read-only boundary

The read-only source of truth was the clean local 2.0 checkout at
`C:\Users\LEET\COINjecture2.0-network`, revision
`58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff`, tree
`4d59268a4b5e627e1b5d6c773f44a95d61ae57d8`. The calibration script refuses any
other revision or any mismatch in these defining files:

| source | SHA-256 | role |
|---|---|---|
| `core/src/problem.rs` | `EDB039CFBFBFE46AC39D4B0DEB0465CC779C04FE5B17C5C8C7DBB3509DCDBE27` | legacy instance, solution, verifier, and floating quality semantics |
| `consensus/src/miner.rs` | `3AA12C026FF8CCEEFB00CC42B9D1EC8BE44CEB7B8D5366AA09EC04CACB014BC5` | active generation and SubsetSum/SAT/TSP solver paths |
| `consensus/src/problem_registry.rs` | `02D001745A17E0F48A8660BC64C515490B7ACC22845E6784872A7F199F9E7A4B` | six default registry descriptors and their optional unimplemented methods |
| `Cargo.lock` | `9930A209663DD812D03DD654D5EA8F850152667DE455191B7C4645EB1CDB1BEA` | legacy dependency graph provenance |

The inventory resolved the FRAME unknown without flattening unlike surfaces:

- SubsetSum, SAT, and TSP are the only executable mining variants in `ProblemType`;
  the driver directly invoked their exact public `solve_problem_blocking` and
  `Solution::quality` implementation from the pinned 2.0 source.
- GraphColoring, Factorization, and SVP are default registry descriptors, but they
  have no `ProblemType` variants and inherit the trait's unimplemented generation and
  verification methods. They are durable `not_implemented` inventory rows, not fake
  timing adapters.
- Custom is an opaque user payload whose legacy solver returns no solution and whose
  verifier rejects the variant. It is inventoried separately and rejected under A2;
  it is not called a concrete problem class.

The isolated driver compiled from a temporary manifest with path dependencies on the
hash-verified legacy checkout. Its transitive graph is pinned at
`bench/p004-admission/legacy-driver-Cargo.lock` (SHA-256
`C1A48885D64A8D85D22A3EE24807853BE3112C9569741B7D2C563D7628EC321D`). The temporary
build directory was mechanically constrained to the system Temp root and removed
after each run; all durable outputs were written below the P-004 evidence directory.
Post-run `git -C C:\Users\LEET\COINjecture2.0-network status --short` remained empty,
so the legacy source was not edited.

### Generic controller

`bench/p004-admission` is a workspace-tested, safe-Rust analysis crate with no new
third-party dependency. It provides:

- a class-neutral `BenchCandidate` contract with separate prepare, solver, and
  independently checked output phases;
- `ProblemClassCandidate<C, S>`, generic over any already implemented CJ3
  `ProblemClass`, without assigning or interpreting `CLASS_ID`;
- bounded controls: at most 1,000 warmups, 10,000 retained samples, and 1,000,000
  checker repetitions per timed batch;
- nanosecond min/median/nearest-rank-p95/max summaries, integer fixed-point asymmetry,
  and a maximum-observation comparison with P-003's provisional 15 ms P-3;
- a stable-schema JSONL writer with complete control-character escaping and a
  Markdown generator that sanitizes inline candidate text;
- dispositions limited to `reject`, `insufficient_evidence`, or
  `eligible_for_adr_review`. The final state deliberately cannot say `admitted`.

The driver fixtures deterministically select the exact three legacy solver/checker
paths. They are not samples from 2.0's active generator: the SubsetSum fixture selects
the one-dimensional DP path, the SAT unit-clause fixture forces the 18-variable brute-
force path to its last assignment, and the symmetric TSP matrix selects nearest-
neighbor plus two-opt. This distinction is in every measured record; fixture timing
cannot support a distributional-hardness claim.

### Tripwire and failure log

- **SCOPE:** the implementation stayed inside the predicted bench tree plus root
  workspace manifests/lockfile and this report. No `cj3-*` runtime/consensus source,
  formal spec, protocol document, CI workflow, registry, class ID, or 2.0 source was
  changed. No scope expansion occurred.
- **REPETITION:** the controller first hit a Rust double-borrow compile error, then a
  distinct missing-errors-doc lint; the isolated driver then hit a separate temporary-
  slice lifetime error. Each was corrected once. No same failure recurred three
  consecutive times, so the repetition ceiling did not trip.
- **BENCHMARK FALSIFICATION:** the first single-call capture produced sub-clock-
  resolution zero-nanosecond checker observations and nonsensical ratios. That
  evidence was rejected and never committed as authoritative. The controller now
  times 10,000 bounded checker calls per sample and reports an integer per-operation
  average. The adversary pass then made this fail closed: any future batch whose
  integer per-operation duration is still zero returns `TimerResolution` instead of
  emitting an observation. The final 21-sample capture has zero zero-duration
  observations.
- **EVIDENCE GENERATION:** the first environment writer exposed incorrect PowerShell
  here-string escaping in its own Markdown. It was repaired before the sealed run;
  final source and output hashes are embedded in `evidence/ENVIRONMENT.md`.
- **INVENTION/HUMAN:** no missing descriptor implementation, parameter floor,
  distribution, class ID, canonical codec, reward rule, or owned TBD was supplied.
  All four non-runnable surfaces remain explicit unknown/not-implemented/reject rows.

No failure repeated to the tripwire ceiling, and no consensus-semantic surface became
necessary. The packet therefore remained AUTO.

### Sealed calibration run

The final run used release mode, 3 warmups, 21 retained samples, and 10,000 checker
calls per timed batch on the Ryzen/Windows host named in
`bench/p004-admission/evidence/ENVIRONMENT.md`. Timings below are per operation; they
are calibration observations, not security estimates:

| candidate | solver median | checker median / p95 / max | median asymmetry | P-3 comparison | disposition |
|---|---:|---:|---:|---|---|
| SubsetSum DP fixture | 2,044,000 ns | 9 / 9 / 9 ns | 227,111.111× | supported | reject |
| SAT brute-force fixture | 14,198,500 ns | 18 / 18 / 19 ns | 788,805.555× | supported | reject |
| TSP NN + two-opt fixture | 636,300 ns | 4,090 / 4,169 / 4,180 ns | 155.574× | supported | reject |
| GraphColoring descriptor | n/a | n/a | n/a | unknown | insufficient evidence |
| Factorization descriptor | n/a | n/a | n/a | unknown | insufficient evidence |
| SVP descriptor | n/a | n/a | n/a | unknown | insufficient evidence |
| Custom payload | n/a | n/a | n/a | unknown | reject |

All three happy-path checkers were far below provisional P-3 on these bounded
fixtures. That fact does not rescue any class:

- SubsetSum's active bounded-positive planted distribution lacks a distributional
  reduction and remains exposed to pseudopolynomial algorithms; its checker also maps
  out-of-range indices to zero, uses unchecked `i64` accumulation, and returns
  floating quality.
- SAT's active generator uses three clauses per variable rather than the survey's
  approximately 4.267 random-3-SAT transition; its planting loop does not actually
  force every clause to match the planted assignment. Its checker has no strict typed
  failure for literal zero and uses floating quality.
- Random complete-matrix TSP is typically easy; the legacy validity predicate accepts
  any permutation without a cost threshold, while quality uses unchecked matrix
  indexing/addition and floating point.

Thus P-004 does not recommend any 2.0 legacy class for ADR review. The three executable
classes are rejected on sampled-hardness and/or checker-contract evidence; the three
descriptor stubs remain insufficient evidence; Custom is not an admissible class.

### Durable evidence hashes

- Raw stable-schema records:
  `bench/p004-admission/evidence/legacy-results.jsonl`, SHA-256
  `8A4B1590266F6CB0BD34D73FE4538B929215201F8957ADC7258DB8099CBDABCF`.
- Generated calibration report:
  `bench/p004-admission/evidence/LEGACY-CALIBRATION.md`, SHA-256
  `07D0BA0ED8A4514CB68D073A48288C740023D019E9C87ABF21D6EC1E2CE79299`.
- Environment/source seal:
  `bench/p004-admission/evidence/ENVIRONMENT.md`, SHA-256
  `7CFA0A8F0330C6BE63501711D6ACDC3EB4CCB5C8717D3566468FA360CAC59E70`.

The environment seal also records the exact controller, driver, and orchestration-
script hashes used to generate the first two artifacts. JSONL parsing produced seven
records, exactly three with timing summaries; every record carries the same
revision/config, and the only dispositions are `reject` and
`insufficient_evidence`.

## 3. VERIFY — local pre-push evidence

The complete local equivalent of the current eleven-job D6 workflow passed against
the hardened source and regenerated artifacts:

- `cargo fmt --all -- --check` — PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` —
  PASS, including `cj3-admission-bench`.
- `cargo deny --locked --all-features check` — PASS: advisories, bans, licenses, and
  sources all `ok`.
- `cargo audit --deny warnings` — PASS on the committed CJ3 `Cargo.lock`, 22 crate
  dependencies scanned.
- `./scripts/ci/verify-geiger.ps1` — PASS: source policy passed and all eleven CJ3
  workspace packages reported zero unsafe use; the new crate contains
  `#![forbid(unsafe_code)]`.
- `cargo test --workspace --all-targets --all-features --locked` — PASS. The new
  controller's four tests cover bounded configuration rejection, generic
  `ProblemClass` adaptation, stable JSON escaping, and fail-closed timer resolution.
- The `conservation-invariant`, `lean-conformance`, `codec-fuzz-smoke`, and
  `genesis-spend-test` phase-gate scripts each returned their expected explicit
  `NOT_YET_ADMITTED` deferral. This is not represented as execution of the deferred
  Phase 1/P-007 tests.
- `cargo build --workspace --all-targets --all-features --locked` — PASS.
- `git diff --check` and `git diff --cached --check` — PASS.

The sealed legacy command also passed after the timer-resolution hardening:

```powershell
./bench/p004-admission/run-legacy-calibration.ps1 `
  -LegacyRoot C:\Users\LEET\COINjecture2.0-network `
  -Warmups 3 -Samples 21 -CheckerRepetitions 10000
```

It emitted exactly seven parseable JSON records, three measured records, no zero
timing observations, and only the `reject`/`insufficient_evidence` dispositions.
The legacy checkout was clean before and after execution, and all source hashes were
rechecked after the run.

Origin D6 is deliberately not claimed in this pre-push checkpoint. Under A11, the
exact-head PR run URL and job readback will be added only after the CI system reports
them.

## 4. ADVERSARY PASS — pre-push seat switch

**Seat switch: ADVERSARY.** I re-read the complete implementation, driver,
orchestration script, manifests, generated evidence, report, and mechanically
inspected the 1,071-line legacy driver lockfile rather than trusting the BUILD
narrative.

### Findings and dispositions

1. **Fixed — timer-quantization evidence laundering.** A fast checker could still
   produce a batch duration smaller than its repetition count and therefore an
   integer per-call value of zero. The controller now returns `TimerResolution`
   instead of emitting such a result, with a direct regression test. The sealed run
   was regenerated from the hardened source; every observation remained nonzero.
2. **Fixed — lock bootstrap did not express its name safely.** The first script
   revision could refresh the driver lock under the normal path. The final script
   copies and enforces the committed lock with `--locked` by default; only the
   explicit `-BootstrapLock` switch can replace it. The final calibration used the
   normal locked path.
3. **Fixed — wildcard local dependency version.** Local `cargo deny` rejected the
   harness's initial wildcard path dependency. It is now pinned to
   `cj3-classes = "=0.0.0"`; the full policy check passes.
4. **Open but quarantined — inherited unmaintained dependency.** A separate
   `cargo audit --deny warnings --file
   bench/p004-admission/legacy-driver-Cargo.lock` scans 118 packages and exits 1 on
   the `bincode 1.3.3` unmaintained warning, `RUSTSEC-2025-0141`. The advisory reports
   maintenance status, not a known vulnerability. This graph is inherited from the
   exact 2.0 source and is intentionally outside CJ3's workspace lock and every
   runtime/node path. Upgrading it inside P-004 would stop measuring the pinned
   legacy graph. The driver is therefore retained only as an offline, read-only,
   temporary calibration process; the warning is not waived or described as green.

The legacy lock contains 118 packages: 112 checksummed crates.io registry packages
and six local path packages (`cj3-admission-bench`, `cj3-classes`, the temporary
driver, and the three pinned 2.0 crates). No git, file-URL, or absolute-path source is
embedded in the committed lock.

### Axiom sweep

| axiom | adversary result |
|---|---|
| A1 — derive, don't read | PASS. The generic CJ3 adapter calls `derive_instance` from a committed seed and validated size object. Legacy miner-authored/self-reported fields are not imported; structural gaps are rejected or unknown. |
| A2 — protocol-generated instances | PASS. No consensus instance type or registry changed. The three deterministic legacy fixtures are labeled calibration-only and cannot be mistaken for active generator samples. |
| A3 — pure scoring | PASS. CJ3 checks receive only instance and solution and return integer quality. The driver discards legacy-reported duration/memory and independently calls legacy verification/quality. No banned identifier was introduced in Rust source. |
| A4 — decoupled fork choice | PASS by non-reachability. Bench outputs cannot affect fork choice, validity, rewards, or a registry. |
| A5 — integer money | PASS. No amount surface was touched. The sole floating-point conversion is an explicit isolated adapter for legacy 2.0 quality, bounded to finite `[0,1]`; it is not linked into CJ3 or within reach of money. |
| A6 — apply path trusts nothing | PASS by non-reachability. No state/apply path or mutation API changed. |
| A7 — fail closed | PASS at the touched boundary. Arguments and resource counts are bounded, candidate/check failures abort evidence generation, timer under-resolution aborts, source/hash/tree drift aborts, and temporary deletion requires a verified root and name prefix. |
| A8 — spec before state-machine code | PASS by scope. No state-machine implementation or formal spec changed. |
| A9 — minimal trusted computing base | PASS. The workspace harness is safe Rust and offline-only; all CJ3 packages remain zero-unsafe. The pinned legacy solver executes in an isolated calibration process and never links into the node. |
| A10 — honest claims | PASS. Every timing is host/fixture scoped; worst-case labels are not treated as sampled-distribution hardness; P-3 remains provisional; no candidate is called admitted. |
| A11 — evidence or it didn't happen | PASS locally. Raw JSONL, generated Markdown, environment/source hashes, driver lock, and this report are durable repository files. Temp contains only the disposable driver project, never the sole report. Origin-green remains pending rather than inferred. |

### Parser, injection, overflow, and scope review

- The JSON writer escapes quotes, slashes, control characters, and newlines; Markdown
  cells/inline fields sanitize delimiters and formatting characters. CLI arguments are
  parsed as typed `u32` values and validated against hard ceilings before candidate
  code runs.
- Duration conversion and fixed-point asymmetry use checked conversions/arithmetic.
  Checker batches use nonzero validated repetition counts. The bounded fixtures do
  not perform unbounded reads or accept external solver output.
- PowerShell passes arguments as an array, never through `Invoke-Expression` or a
  constructed shell command. The resolved legacy tree must match an exact revision,
  four defining hashes, and a clean status both before and after execution.
- The staged surface contains only the P-004 bench tree, root workspace manifests and
  lockfile, and this packet report. No protocol/spec/CI workflow, registry, class ID,
  Sarah-owned TBD, 2.0 file, external repository, or human notification was changed.

No Critical remains. The open legacy dependency warning is quarantined and disclosed;
the branch is eligible for origin verification, not yet for merge.

## 5. MERGE

- Feature commit: `06decc277bedf42ae753024ec0dacf7b08afdec8`.
- Pull request: [#7 — P-004: add legacy-class admission bench](https://github.com/Quigles1337/COINjecture3.0/pull/7).
- Exact-head D6: [run 31954357570](https://github.com/Quigles1337/COINjecture3.0/actions/runs/31954357570), success on
  `06decc277bedf42ae753024ec0dacf7b08afdec8`; all eleven jobs succeeded.
- Guarded merge commit: `4644374f5073a929bf5ecc88c0e191f0d9bab1be`.
- Exact-merge-SHA mainline D6:
  [run 31954588282](https://github.com/Quigles1337/COINjecture3.0/actions/runs/31954588282), success on
  `4644374f5073a929bf5ecc88c0e191f0d9bab1be`; all eleven jobs succeeded.

Immediately before merge, GitHub reported PR #7 `MERGEABLE`/`CLEAN`, the repository
`PRIVATE`, the head SHA unchanged, and every required check successful. A fresh D11
readback still showed 2.0 `CAPACITY_FLAG: none` with GATE-1/GATE-2 under awaiting
ownership. The five D17 AUTO conditions remained true: the packet was approved and
unblocked; the diff stayed within the bounded bench/report surface; no Lean/vector
content changed; no decision or consensus parameter was invented; and no Al- or
Sarah-owned TBD was filled.

## 6. CALIBRATE

### Predictions versus outcomes

- **Diff surface:** the prediction held. The implementation touched the dedicated
  `bench/p004-admission` tree, the root workspace manifest/lock, and this report. It
  did not need a CI-workflow hook, consensus/runtime source, protocol/spec edit,
  registry change, class ID, or 2.0 mutation.
- **Risk materialization:** all three top risks appeared in useful form. Semantic
  laundering was prevented by explicit rejection/unknown states; benchmark-category
  error appeared as sub-clock-resolution zero observations and was fixed
  fail-closed; reproducibility/hostile-state risk appeared in lock bootstrap,
  PowerShell environment rendering, exact source pins, and the inherited legacy
  dependency warning.
- **Confidence:** MEDIUM was calibrated correctly. A reusable bounded controller and
  faithful three-class driver were tractable, while four legacy surfaces proved
  structurally unmeasurable and no legacy class cleared the evidence contract.
- **Surprise:** every executable happy-path checker was far below provisional P-3 on
  its fixture, yet every executable class still failed admission evidence. That is a
  strong empirical demonstration that checker speed and solve/check asymmetry cannot
  substitute for a hard sampled distribution or a strict checker contract.

### Final VERIFIED / ASSUMED / UNKNOWN ledger

**VERIFIED**

- The generic controller, exact legacy inventory, raw results, generated report,
  environment seal, and pinned driver lock are committed at the merge SHA above.
- Three executable legacy paths were measured; all three are `reject`. Three
  descriptor-only paths are `insufficient_evidence`; Custom is `reject` and is not a
  concrete class. Evidence: committed JSONL and generated Markdown under
  `bench/p004-admission/evidence/`.
- Both exact-head and exact-merge-SHA hosted D6 runs passed all eleven jobs. Evidence:
  the two CI URLs above.

**ASSUMED**

- `C:\Users\LEET\COINjecture2.0-network` remains the operational local 2.0
  source/state checkout. This was safe for P-004 because the defining code and lock
  were independently pinned by revision and SHA-256, the checkout remained clean,
  and Al's live D11 ruling independently supplied the capacity branch.

**UNKNOWN**

- Fixture timings do not establish active-generator timing distributions or
  security floors. Resolving that would require a separately approved generator-
  sampled study plus published attack analysis; P-004 does not infer either.
- Whether a maintained replacement for legacy `bincode 1.3.3` preserves exact 2.0
  semantics is unknown. Resolving it requires a 2.0-owned dependency migration and
  equivalence evidence; CJ3 does not waive or silently upgrade the calibration graph.
- Descriptor-only GraphColoring, Factorization, and SVP remain unmeasurable until
  real generator, solver, checker, and parameter-floor implementations exist. Their
  names and empirical registry exponents are not substitutes.

**One process improvement for the next measurement packet:** run a timer-resolution
probe and audit every auxiliary lockfile before the first sealed capture. This turns
quantization and inherited-dependency surprises into FRAME-time constraints rather
than late adversary findings.
