# Cycle 3 — P-003 Builder Report

**Status:** FRAME COMPLETE — BUILD NOT STARTED
**Date:** 2026-08-15
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

## 3. VERIFY

Pending.

## 4. ADVERSARY

Pending.

## 5. MERGE

Pending.

## 6. CALIBRATE

Pending.

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

## ASSUMED

- The 2026-08-15 live D11 ruling remains current because `loop/STATE.md` repeats it,
  no later live instruction supersedes it, the local COINjecture 2.0 checkout exposes
  no `loop/STATE.md`, and a read-only scan found no `remediation-priority` capacity
  flag. This is re-checked at every packet boundary and immediately before merge.
- A prepared `[u8; 32]` input to `ProblemClass::derive_instance` is the instance seed,
  not the place to assign `D_INST` bytes. This follows Protocol Spec §5.1's trait
  signature and preserves P-007's canonical-domain ownership.

## UNKNOWN

- The exact current estimator/tool mapping for homogeneous SIS, applicable security
  target, and defensible candidate floor are unknown until the primary-source and
  estimator survey is complete.
- The attainable witness-generation envelope on this machine and the resulting
  solve/check asymmetry are unknown until the external-process benchmark runs.
- Final block cadence is unknown because P-1 belongs to P-006. P-003 can measure and
  recommend parameter floors, but P-006/G0 must resolve cadence integration.
- Reward-margin shaping remains intentionally unknown and unfilled for Al + Sarah at
  the D16 reveal.
