# Cycle 4 — P-004 Builder Report

**Status:** FRAME COMMITTED — BUILD NOT STARTED
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

