# COINjecture 3.0


[![D6 CI](https://github.com/Quigles1337/COINjecture3.0/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Quigles1337/COINjecture3.0/actions/workflows/ci.yml)

COINjecture 3.0 specifies a derive-don't-trust redesign of proof-of-useful-work
consensus: validators derive the consensus quantities they use instead of accepting
miner-reported substitutes ([Engineering Plan A1–A4](docs/ENGINEERING_PLAN.md#1-axioms-binding-on-every-packet-violations-are-criticals),
[Protocol Spec §§5–11](docs/PROTOCOL_SPEC.md#5-problem-classes)).

The COINjecture 2.0 C1/C2 failure class is intended to be structurally
unrepresentable at the governing interface: the protocol derives the instance from
committed chain data, `check(instance, solution)` derives integer `Quality` without a
clock or miner metadata, and fork choice excludes quality. The first two boundaries
exist in the P-003 Rust trait and SIS prototype; Phase 2 block validation and fork
choice remain planned, so this is not a claim that a consensus implementation already
enforces the complete design ([audit mapping](docs/AUDIT_TRACEABILITY.md#1-third-party-security-audit--complete-33-finding-matrix),
[`ProblemClass`](crates/cj3-classes/src/lib.rs),
[phase plan](docs/ENGINEERING_PLAN.md#4-phase-plan)).

## Status

> Refreshed 2026-11-23 for Gate G0 PASS and the opening of Phase 1, and brought up to date the same day for the ruling that takes P-101 to its FRAME. This is the repository's single current-status
> block; its refresh is required at every gate closeout
> ([standing rule](loop/LEDGER.md#standing-readme-drift-control-rule--ratified-al-2026-08-16)).
>
> | Field | Current record | Evidence |
> |---|---|---|
> | Scope | Public repository; pre-testnet; no running testnet, production node, or mainnet configuration | [2026-11-22 ruling, Part 0.1](loop/LEDGER.md#gate-g0-ruling-overlay--g0-b-to-g0-e-ruled--g0-f-held-al-2026-11-22), [D12](loop/LEDGER.md#d1d15-seed-from-engineering-plan-3), [source-policy gate](scripts/ci/check-source-policy.ps1) |
> | Phase | **Phase 1 — open.** Gate G0 is PASS, with items carried. P-101, the kernel packet, runs first and is at its FRAME, stopped for Al's ruling. No kernel code exists | [Gate G0 ruling of 2026-11-23](loop/LEDGER.md#gate-g0-f-ruling-overlay--gate-g0-pass--pr-21-confirmed--report-c10-points-ruled--phase-1-open-al-2026-11-23), [ruling "P-101 TO ITS FRAME"](loop/LEDGER.md#p-101-frame-ruling-overlay--pr-22-confirmed--report-c11-points-ruled--p-101-to-its-frame-al-2026-11-23), [P-101's FRAME](loop/reports/C12-p101-builder.md), [phase plan](docs/ENGINEERING_PLAN.md#4-phase-plan), [packet queue](loop/PACKETS.md) |
> | Last phase gate | **G0 — PASS on 2026-11-23**, with items carried to the boundaries where Al rules them. G1–G4 have not passed | [Gate G0 ruling of 2026-11-23](loop/LEDGER.md#gate-g0-f-ruling-overlay--gate-g0-pass--pr-21-confirmed--report-c10-points-ruled--phase-1-open-al-2026-11-23), [gate report](loop/reports/C11-g0f-gate-builder.md), [queue state](loop/PACKETS.md) |
> | Completed packet evidence | P-001 through P-007, P-009 and P-010 are merged with exact-mainline CI records. P-005's Lean V1–V9/STF is HUMAN-RATIFIED. P-007 is COMPLETE: Al merged PR #21 on 2026-11-23 as `333c6708`, and D6 run 45118879871 on that commit passed all eleven jobs. PR #14, merged on 2026-10-12 as `35811404`, had carried only the checked `Amount` boundary and the stop report | [batch log](loop/reports/BATCH-LOG.md), [gate report, evidence for clause 3.2](loop/reports/C11-g0f-gate-builder.md#32-evidence-for-pass-32), [PR #21](https://github.com/Quigles1337/COINjecture3.0/pull/21), [D6 run 45118879871](https://github.com/Quigles1337/COINjecture3.0/actions/runs/45118879871), [PR #14](https://github.com/Quigles1337/COINjecture3.0/pull/14) |
> | Current stop | Al merged PR #22, the record of the Gate G0 ruling, on 2026-11-23 as `d2c5f759`, and D6 run 45177244858 on that commit passed all eleven jobs. His ruling "P-101 TO ITS FRAME" says that P-101 runs first, takes it to its FRAME and no further, and orders two build items: the word size of the curve arithmetic pinned to 64, and `sha2`'s portable code on every processor. The FRAME puts twelve decisions to Al, among them how `tx_root` and the state commitment are built and six proposed domain tags. None is ruled, and no tag is assigned. The record is PR #23, which stops for Al's review; the builder does not merge it and does not begin P-101's build. Packet P-011 is unblocked and not started | [ruling "P-101 TO ITS FRAME"](loop/LEDGER.md#p-101-frame-ruling-overlay--pr-22-confirmed--report-c11-points-ruled--p-101-to-its-frame-al-2026-11-23), [P-101's FRAME](loop/reports/C12-p101-builder.md), [session report](loop/reports/C12-p101-frame-ruling-builder.md), [PR #23](https://github.com/Quigles1337/COINjecture3.0/pull/23), [D6 run 45177244858](https://github.com/Quigles1337/COINjecture3.0/actions/runs/45177244858), [loop state](loop/STATE.md) |
> | Implemented surfaces | D6 CI and source policy, including the rule that no Rust source outside `cj3-types` names a domain tag; dependency policy over the workspace and, under a policy of its own, over the fuzz workspace; trait-gated beacon boundary; sampled-SIS derivation/checking and external demonstrator; CJ3 Encoding v1 in `cj3-types` (grammar, domain tags, preimages, the single `addr()`, transaction, header and block layouts, the §1.3 Ed25519 verification on one arithmetic back end) with the SIS solution encoding in `cj3-classes`; three normative conformance-vector files, each derived twice; six fuzz targets, run as a D6 smoke job and as a nightly long job; checked `Amount(u64)`; admission and difficulty benches; HUMAN-ratified Lean V1–V9/STF with symbolic non-normative vectors; source-verified audit matrix v0.2 | [workflow](.github/workflows/ci.yml), [nightly workflow](.github/workflows/nightly.yml), [`cj3-types`](crates/cj3-types/src/lib.rs), [vectors](bench/p007-vectors/README.md), [fuzz properties](fuzz/src/lib.rs), [formal project](spec/README.md), [P-006 bench](bench/p006-difficulty/README.md), [audit matrix](docs/AUDIT_TRACEABILITY.md) |
> | Planned or absent | A ratified SIS tuple, block cadence, timestamp unit and drift bound; how `tx_root` and the state commitment are built; the genesis layout; kernel, storage, consensus, networking, RPC, node behavior; production VDF; testnet | [open-TBD index](docs/PROTOCOL_SPEC.md#15-open-tbd-index), [explicit phase deferrals](scripts/ci/check-phase-gate.ps1), [roadmap](docs/ENGINEERING_PLAN.md#4-phase-plan) |

## Contents

- [Provenance: the audit lineage](#provenance-the-audit-lineage)
- [Design axioms](#design-axioms)
- [Architecture](#architecture)
- [Governance](#governance)
- [Verification and evidence](#verification-and-evidence)
- [Repository map](#repository-map)
- [Quickstart](#quickstart)
- [Limitations and open assumptions](#limitations-and-open-assumptions)
- [Roadmap](#roadmap)
- [License](#license)

## Provenance: the audit lineage

COINjecture 2.0's README presented a work-score formula containing self-reported
`solve_time` as a security mechanism; the third-party audit's C2 finding showed that
the same miner-controlled values affected fork choice and rewards. The CJ3 traceability
matrix retains that failure beside C1's miner-authored instance and the apply-path
findings rather than replacing them with a clean-slate narrative
([C1–C7 matrix](docs/AUDIT_TRACEABILITY.md#1-third-party-security-audit--complete-33-finding-matrix),
[preserved source manifest](loop/evidence/P-009-SOURCE-MANIFEST.md)).

The response is structural, not testimonial. The specified submission/checking path
has no timing or reported-score input, the instance is re-derived, and the specified
fork weight uses only a validated hash target; the source-policy gate also rejects the
known identifier family in every `cj3-*` Rust source file
([Protocol Spec §5.1](docs/PROTOCOL_SPEC.md#51-the-trait-contract-the-heart-of-a1a3),
[Protocol Spec §10](docs/PROTOCOL_SPEC.md#10-fork-choice-and-difficulty),
[source-policy script](scripts/ci/check-source-policy.ps1)).

That record has a deliberate limit: P-009 reconciled all 33 third-party findings and
all 25 Lean claim-points against the preserved sources, but the original Codex scan
file remains absent. The exact missing report inventory is therefore UNKNOWN even
though committed 2.0 remediation pointers preserve the known five program families
and two finding IDs
([matrix epistemic status](docs/AUDIT_TRACEABILITY.md),
[P-009 report](loop/reports/C7-p009-builder.md)).

## Design axioms

The following are ratified requirements, not assertions that all implementation phases
exist ([D3 and effective rulings](loop/LEDGER.md#effective-ratifications)).

| Axiom | Condensed requirement | Primary evidence or specification |
|---|---|---|
| A1 — Derive, don't read | A consensus value must be reproducible from committed data or be excluded. | [Plan A1](docs/ENGINEERING_PLAN.md#1-axioms-binding-on-every-packet-violations-are-criticals), [block rules B3–B12](docs/PROTOCOL_SPEC.md#62-block-validity-rules-b-rules-all-must-hold-typed-errors-never-panics) |
| A2 — Protocol-generated instances | Miners do not author or parameterize the instance used for validation. | [Plan A2](docs/ENGINEERING_PLAN.md#1-axioms-binding-on-every-packet-violations-are-criticals), [B5–B6](docs/PROTOCOL_SPEC.md#62-block-validity-rules-b-rules-all-must-hold-typed-errors-never-panics) |
| A3 — Pure scoring | Quality is an integer result of `(instance, solution)` with no timing or miner metadata. | [`ProblemClass` source](crates/cj3-classes/src/lib.rs), [Spec §5.1](docs/PROTOCOL_SPEC.md#51-the-trait-contract-the-heart-of-a1a3) |
| A4 — Decoupled fork choice | Quality gates validity and may shape rewards; it does not weight chain selection. | [Spec §10](docs/PROTOCOL_SPEC.md#10-fork-choice-and-difficulty) |
| A5 — Integer money | Require unsigned checked arithmetic and a conservation invariant on amount paths. | [Plan A5/D7](docs/ENGINEERING_PLAN.md#1-axioms-binding-on-every-packet-violations-are-criticals), [planned STF](docs/PROTOCOL_SPEC.md#8-state-and-the-state-transition-function-stf) |
| A6 — Apply trusts nothing | Require mutation to revalidate nonce, signer/address binding, balances, and atomicity. | [V-rules](docs/PROTOCOL_SPEC.md#7-transactions), [planned STF](docs/PROTOCOL_SPEC.md#8-state-and-the-state-transition-function-stf) |
| A7 — Fail closed | Require authenticated mutation, startup refusal on empty secrets, and typed malformed-input errors. | [Plan A7](docs/ENGINEERING_PLAN.md#1-axioms-binding-on-every-packet-violations-are-criticals), [planned Phase 3 rules](docs/PROTOCOL_SPEC.md#12-networking-and-rpc--baseline-requirements-normative-for-phase-3) |
| A8 — Spec before state-machine code | Require Lean transaction/STF semantics before Rust kernel conformance. | [Plan D5](docs/ENGINEERING_PLAN.md#3-decision-log), [current formal boundary](spec/README.md) |
| A9 — Minimal trusted base | Keep consensus crates in safe Rust and heavy solvers external and untrusted. | [workspace lint](Cargo.toml), [zero-unsafe gate](scripts/ci/verify-geiger.ps1), [solver boundary](crates/cj3-solver-sis/src/main.rs) |
| A10 — Honest claims | Label hardness as an assumption and retain PoUW's usefulness ceiling. | [hardness wording issue SI-001, resolved](loop/reports/SPEC-ISSUES.md#si-001--sis-reduction-described-as-unconditional-proof), [survey caveats](docs/RESEARCH_SURVEY.md#caveats-and-honest-disagreements-in-the-literature) |
| A11 — Evidence or it did not happen | Require a run, artifact, or committed report for verification statements. | [batch log](loop/reports/BATCH-LOG.md), [autonomy protocol](loop/PROMPTS/AUTONOMOUS_BUILDER.md) |

## Architecture

This is the governed crate map, adapted from
[Engineering Plan §2](docs/ENGINEERING_PLAN.md#2-architecture-overview); it describes
the intended trust boundaries, not a claim that the full runtime is implemented.

```mermaid
flowchart TB
    Solver["cj3-solver-*<br/>untrusted external process"]
    Node["cj3-node<br/>wiring and startup policy"]
    RPC["cj3-rpc<br/>authenticated RPC"]
    Net["cj3-net<br/>bounded gossip ingress"]
    Store["cj3-store<br/>chain and state"]
    Consensus["cj3-consensus<br/>block validation · fork choice · retarget"]
    Classes["cj3-classes<br/>derived instances · pure checkers"]
    Beacon["cj3-beacon<br/>trait-gated randomness"]
    Kernel["cj3-kernel<br/>transaction validity · STF"]
    Types["cj3-types<br/>canonical encodings · addr · amounts"]
    Lean["spec/<br/>ratified Lean rules · symbolic vectors"]
    Bench["bench/<br/>non-consensus evidence"]

    Node --> RPC
    Node --> Net
    Node --> Store
    RPC --> Consensus
    Net --> Consensus
    Store --> Consensus
    Consensus --> Classes
    Consensus --> Beacon
    Consensus --> Kernel
    Classes --> Types
    Beacon --> Types
    Kernel --> Types
    Solver -. "candidate solution bytes" .-> Classes
    Lean -. "conformance vectors" .-> Kernel
    Bench -. "admission evidence" .-> Classes
```

Solvers cross the boundary only through candidate output that a safe-Rust checker
validates; the node does not link the solver library under the governing architecture
([Plan trust boundary](docs/ENGINEERING_PLAN.md#2-architecture-overview),
[current demonstrator manifest](crates/cj3-solver-sis/Cargo.toml),
[current node manifest](crates/cj3-node/Cargo.toml)).

## Governance

Authority overlays and ratified decisions live in the [decision ledger](loop/LEDGER.md), while executable order and current custody live in the [packet queue](loop/PACKETS.md) and [loop state](loop/STATE.md).
Each approved packet follows FRAME, BUILD, VERIFY, ADVERSARY, MERGE, and CALIBRATE, then leaves a report and a one-line [batch record](loop/reports/BATCH-LOG.md) under the [autonomy doctrine](loop/PROMPTS/AUTONOMOUS_BUILDER.md).
D17 permits autonomous merge only while all AUTO conditions and the D11 capacity check hold; formal semantics, vector definitions, decision ratification, owned TBDs, tripwires, and phase gates stop for human authority ([D17 ruling](loop/LEDGER.md#live-autonomy-ruling-overlay--ratified-al-2026-08-15), [G0–G4 gates](docs/ENGINEERING_PLAN.md#4-phase-plan)).

## Verification and evidence

A reader can audit a claim through five layers:

1. **Hosted execution.** The [D6 workflow](.github/workflows/ci.yml) defines
   formatting, Clippy, dependency policy/audit, zero-unsafe checks, tests, four named
   phase jobs, and a locked build; immutable packet and mainline URLs are recorded in
   the [batch log](loop/reports/BATCH-LOG.md).
2. **Deferral inspection.** A green phase job is not automatically proof that its
   future test ran: absent handlers emit `NOT_YET_ADMITTED` with an owner through
   [`check-phase-gate.ps1`](scripts/ci/check-phase-gate.ps1). Conservation and the
   genesis spend test are currently in that explicit deferral state. Three gates are
   live. Lean conformance: its
   [active handler](scripts/ci/active/lean-conformance.ps1) builds the ratified Lean
   project and regenerates the symbolic vectors, and it still reports the Rust kernel
   conformance test as `NOT_YET_ADMITTED`
   ([P-005 merge](https://github.com/Quigles1337/COINjecture3.0/pull/9)). Codec fuzz
   smoke: its [active handler](scripts/ci/active/codec-fuzz-smoke.ps1) runs the six
   cargo-fuzz targets under [`fuzz/`](fuzz/src/lib.rs) for a fixed number of runs from
   a fixed seed. That is a bounded smoke run, not evidence that no defect exists. The
   nightly long run: its [active handler](scripts/ci/active/full-fuzz-corpus.ps1) runs
   the same six targets, on the same pinned nightly toolchain, for a fixed time each in
   the D6 Nightly workflow. A long run that finds nothing is not that evidence either.
3. **Packet evidence.** P-003 preserves the pinned estimator model, environment,
   solver observations, and checker observations under
   [`bench/p003-sis/`](bench/p003-sis/README.md); the packet report labels P-3/P-4 as
   provisional rather than ratified ([P-003 report](loop/reports/C3-p003-builder.md)).
4. **Negative evidence.** P-004 preserves raw and rendered legacy-class results;
   SubsetSum, SAT, and TSP were rejected, three descriptor-only classes remained
   insufficient evidence, and no class was admitted
   ([calibration report](bench/p004-admission/evidence/LEGACY-CALIBRATION.md),
   [raw JSONL](bench/p004-admission/evidence/legacy-results.jsonl)).
5. **Double derivation.** Every layout, preimage, expansion vector and Ed25519 verdict
   of CJ3 Encoding v1 is derived by a script that uses only the Python standard
   library and again by the Rust code, and D6 runs both. The two share no code and no
   library; they do share an author. Three of the vector files are NORMATIVE conformance
   vectors by Al's ruling of 2026-11-23. What is normative is the relation: these inputs
   give these bytes and these verdicts, and every implementation reproduces them. The
   input values assign nothing ([vectors guide](bench/p007-vectors/README.md),
   [P-007 report](loop/reports/C10-p007-builder.md)).

When checking any result, use the exact SHA and CI URL from the packet report rather
than assuming the current branch reproduces an earlier observation
([A11 procedure](loop/PROMPTS/AUTONOMOUS_BUILDER.md)).

## Repository map

| Path | Current role | Depth pointer |
|---|---|---|
| `crates/` | Rust workspace: `cj3-types` (CJ3 Encoding v1), `cj3-classes` (SIS derivation, checker and solution encoding), the beacon trait and the external solver; the other crates are empty boundaries | [crate sources](crates/), [workspace manifest](Cargo.toml) |
| `bench/p003-sis/` | Reproducible estimator, solver, and verifier records for provisional SIS candidates | [bench guide](bench/p003-sis/README.md) |
| `bench/p004-admission/` | Bounded admission controller and read-only legacy calibration evidence | [bench guide](bench/p004-admission/README.md) |
| `bench/p101-roots/` | Worked examples of the `tx_root` and state commitment that P-101's FRAME proposes, derived by a standard-library script. **Proposed, not ruled:** not conformance vectors, and their domain tags are not assigned | [worked examples](bench/p101-roots/WORKED-EXAMPLES.md), [P-101's FRAME](loop/reports/C12-p101-builder.md) |
| `bench/p007-vectors/` | Standard-library derivation of the Encoding v1 vectors, three of whose files are normative conformance vectors; the pinned C2SP CCTV ed25519 vectors with their licence | [vectors guide](bench/p007-vectors/README.md) |
| `fuzz/` | cargo-fuzz harness for the Encoding v1 codecs: its own workspace, linked into no `cj3-*` crate, and checked by `cargo deny` under a policy of its own, which is the root policy plus one licence term for `libfuzzer-sys` alone | [properties](fuzz/src/lib.rs), [shared gate script](scripts/ci/codec-fuzz-common.ps1), [fuzz policy](fuzz/deny.toml) |
| `spec/` | HUMAN-lane Lean 4 project: the HUMAN-RATIFIED V1–V9/STF encoding, with conservation proved under `LawfulStateOps`, and 27 symbolic, non-normative vectors | [formal project](spec/README.md), [vector artifact](spec/vectors/p005-draft.json) |
| `docs/` | Governing plan, Protocol Specification v0.2 (normative, except for its unfilled, provisional and carried items), audit traceability, and research caveats | [Engineering Plan](docs/ENGINEERING_PLAN.md), [Protocol Spec](docs/PROTOCOL_SPEC.md), [audit matrix](docs/AUDIT_TRACEABILITY.md), [research survey](docs/RESEARCH_SURVEY.md) |
| `loop/` | Decisions, queue, state, prompts, reports, and durable audit-source custody | [ledger](loop/LEDGER.md), [queue](loop/PACKETS.md), [reports](loop/reports/) |
| `scripts/ci/` | Source policy, zero-unsafe enforcement, the arithmetic back-end and word-size check, the portable-hash check, the fuzz policy check, the fuzz gates, and fail-explicit phase dispatch | [source policy](scripts/ci/check-source-policy.ps1), [arithmetic check](scripts/ci/check-arithmetic-backend.ps1), [hash check](scripts/ci/check-sha2-portable.ps1), [phase dispatcher](scripts/ci/check-phase-gate.ps1) |
| `.github/` | D6/nightly workflows, PR evidence template, and the documented branch-protection contract | [D6 workflow](.github/workflows/ci.yml), [branch-protection contract](.github/BRANCH_PROTECTION.md) |

## Quickstart

From the repository root, the following commands check and build the workspace that
exists today; they do not start a node or a network
([P-010 command evidence](loop/reports/C6-p010-builder.md#3-verify),
[pinned toolchain](rust-toolchain.toml), [workspace members](Cargo.toml)).

```powershell
rustup show active-toolchain
cargo fmt --all -- --check
pwsh -NoProfile -File scripts/ci/check-source-policy.ps1
cargo test --workspace --all-targets --all-features --locked
cargo build --workspace --all-targets --all-features --locked
```

The P-007 vectors are re-derived, outside cargo, by
`python -I -B bench/p007-vectors/derive.py --check`
([vectors guide](bench/p007-vectors/README.md)).

The full dependency and unsafe-code toolchain is defined by the hosted workflow; use
its immutable run rather than treating an unpinned local installation as equivalent
([D6 workflow](.github/workflows/ci.yml), [recorded runs](loop/reports/BATCH-LOG.md)).

## Limitations and open assumptions

- The ≤1/2 usefulness ceiling is an Ofelimos prior-art result, not a measured CJ3
  efficiency figure, and the survey finds no problem class with provably high
  usefulness ([research survey Area 5](docs/RESEARCH_SURVEY.md#area-5--consensus-skeleton-options),
  [survey caveats](docs/RESEARCH_SURVEY.md#caveats-and-honest-disagreements-in-the-literature)).
- SIS has a conditional worst-case-to-average-case reduction under stated asymptotic
  conditions; concrete parameter hardness and runtime remain assumptions supported by
  models and measurements, not unconditional or per-instance proofs
  ([SI-001](loop/reports/SPEC-ISSUES.md#si-001--sis-reduction-described-as-unconditional-proof),
  [P-003 evidence](loop/reports/C3-p003-builder.md)).
- **No SIS tuple is ratified, and the one proposed could not be mined.** P-003 proposed
  `(n, m, q, beta_squared) = (128, 3840, 16411, 3840)`. Al rejected it on 2026-11-22,
  together with the 128-bit threshold as a way of choosing a tuple: the pinned estimator
  puts the cost of finding a solution there at 2^143.956 to 2^165.520 operations, and
  the bundled demonstrator solved n = 16 and returned no solution at n = 24 in its one
  recorded run, so a chain with that tuple would never produce a block. P-4 and
  `ACTIVE_SIZE_PARAM` stay UNFILLED until Al rules them from a measured solver envelope
  ([ruling clauses B3 and B4](loop/LEDGER.md#gate-g0-ruling-overlay--g0-b-to-g0-e-ruled--g0-f-held-al-2026-11-22),
  [estimator results](bench/p003-sis/estimator-results.jsonl),
  [solver results](bench/p003-sis/solver-results.jsonl)).
- **D1 stands on a condition.** SIS remains the genesis anchor class. No tuple is
  ratified, and it is not yet measured whether the family has a size that miners can
  solve at a cost worth calling useful work. Packet P-011 measures that. If it finds no
  such size, D1 returns to Al
  ([Gate G0 ruling of 2026-11-23, clause 3.3](loop/LEDGER.md#gate-g0-f-ruling-overlay--gate-g0-pass--pr-21-confirmed--report-c10-points-ruled--phase-1-open-al-2026-11-23),
  [packet P-011](loop/PACKETS.md#p-011--sis-solver-envelope)).
- The selected production beacon construction has no admitted implementation or
  security/timing parameter set; the present iterated-hash implementation is explicitly
  devnet-only and compile-rejected for testnet-tagged builds
  ([P-002 report](loop/reports/C2-p002-builder.md),
  [beacon source](crates/cj3-beacon/src/lib.rs)).
- The Lean V1–V9/STF encoding is HUMAN-RATIFIED and proves conservation under
  `LawfulStateOps`; its JSON vectors remain symbolic and non-normative (the 2026-11-22
  ruling resolved SI-001/SI-002/SI-003 without changing any Lean file or vector), and
  no Rust kernel conformance claim exists yet
  ([formal boundary](spec/README.md), [P-005 merge](https://github.com/Quigles1337/COINjecture3.0/pull/9)).
- CJ3 Encoding v1 is implemented and merged; it is not yet used by a kernel or a node.
  `cj3-types` hashes with `sha2 =0.10.9` and verifies Ed25519 with arithmetic from
  `curve25519-dalek =4.1.3`. Al accepted both as pinned on 2026-11-23. No SIS tuple,
  network identifier, fee floor or timestamp unit is compiled in; the two tuples in the
  tests and vectors are non-normative stand-ins. The D6 fuzz job is a bounded smoke run
  and the nightly job fuzzes each target for a fixed time, both on a nightly toolchain
  pinned by date; neither is evidence that no defect exists
  ([P-007 report](loop/reports/C10-p007-builder.md),
  [vectors guide](bench/p007-vectors/README.md)).
- **The same curve arithmetic and the same hash code on every processor, and what that
  does not cover.** The reason for each pin below is that every node should run the
  same code whatever its processor. None of it is a claim that no `unsafe` code remains
  on the path.
  - *Curve arithmetic.* Every build of `cj3-types` in this repository uses
    `curve25519-dalek`'s serial back end with its word size set to exactly 64, and a
    build that sets either value otherwise, or not at all, fails to compile; D6 proves
    each refusal on every run. The serial back end is two bodies of code, one written
    for 64-bit words and one for 32-bit words. Left alone, the library chooses between
    them by the target's pointer width. With the word size pinned, the 64-bit code is
    the one compiled for every target. D6 builds and tests on x86_64 only: no job
    builds a 32-bit target, so that this code compiles and reproduces the vectors
    there is not shown.
  - *SHA-2.* `cj3-types` enables `sha2`'s `force-soft` feature, so SHA-256 and the
    SHA-512 inside Ed25519 verification run the library's portable code on every
    processor. Without it, on x86 processors, `sha2` chooses between that code and
    processor extensions when it runs. D6 fails if what cargo resolved, or any `sha2`
    that the build of the tests compiles, lacks the feature. This binds the code every
    node checks with. Code that mines may hash as it likes when it comes, because every
    node checks the result with this code.
  - *SHA-3.* `cj3-classes` expands an instance seed with SHAKE-256 from `sha3 =0.10.9`,
    over `keccak 0.1.6`. That library has one portable implementation and picks another
    only when its `asm` feature is enabled, on 64-bit ARM, by processor at run time.
    The feature is not enabled here, so SHA-3 also runs one body of code on every
    processor. Unlike the two pins above, no D6 check fails if a later change enables
    it.
  - *What stays true of `unsafe`.* The zero-unsafe gate covers the `cj3-*` crates, not
    their dependencies. As built here, `curve25519-dalek` still compiles one `unsafe`
    volatile read in its scalar arithmetic, its dependency `subtle` has two `unsafe`
    blocks, and `sha2` has two `unsafe` slice casts; `sha2`'s `unsafe` processor
    intrinsics are no longer compiled.

  ([cargo configuration](.cargo/config.toml),
  [the refusals](crates/cj3-types/build.rs),
  [arithmetic check](scripts/ci/check-arithmetic-backend.ps1),
  [`cj3-types` manifest](crates/cj3-types/Cargo.toml),
  [hash check](scripts/ci/check-sha2-portable.ps1)).
- P-004 measured fixture behavior but admitted no legacy class; checker speed does not
  establish a hard sampled distribution
  ([P-004 report](loop/reports/C4-p004-builder.md),
  [calibration evidence](bench/p004-admission/evidence/LEGACY-CALIBRATION.md)).
- CJ3's useful-work difficulty is **governance-calibrated, not self-calibrating**.
  G0-A makes `size_param` a static protocol constant changed only by explicit human-
  ratified upgrade; every phase gate re-reviews its adequacy. An instance that becomes
  too easy loses usefulness, even though D2's eligibility/quality decoupling prevents
  that quality drift from directly weighting fork choice. P-006 also showed that
  checker-honest published quality can remain selection-biased: modeled equilibrium
  size retention was `0.423–0.806` at 35% strategic share and `0.240–0.700` at 51%
  ([G0-A amendment](loop/LEDGER.md#effective-g0-a-amendment-to-d2),
  [P-006 result](loop/reports/C7-p006-builder.md#candidate-findings)).

Material unfilled values remain visible:

| Item | State and owner | Evidence |
|---|---|---|
| P-1 — hash-target cadence only | **UNFILLED; no builder proposal** — P-006's normalized interval cannot justify an absolute cadence | [G0-A follow-up](loop/LEDGER.md#g0-a-integration-and-p-2-provisional-ratification-overlay-al-2026-08-16), [Spec §3](docs/PROTOCOL_SPEC.md#3-protocol-parameters) |
| P-2 — hash-target controller | **RATIFIED AS PROVISIONAL** — `W=32`, gain `1/8`, clamp `[8/9,9/8]`; conservative interior point, not a tuned optimum; re-derive when P-1 is ratified | [effective disposition](loop/LEDGER.md#effective-p-1p-2-disposition), [Spec §3](docs/PROTOCOL_SPEC.md#3-protocol-parameters) |
| P-11 — size-retarget window | **STRUCK — MOOT** — `size_param` is static and changed only by human-ratified protocol upgrade | [G0-A amendment](loop/LEDGER.md#effective-g0-a-amendment-to-d2), [Spec §10](docs/PROTOCOL_SPEC.md#10-fork-choice-and-difficulty) |
| P-3 — validation budget | **PROVISIONAL: 15 ms** for `derive_instance` plus `check` on reference host v0; the evidence so far times `check` alone; Al re-rules it from the solver-envelope packet's measurements | [Spec §3](docs/PROTOCOL_SPEC.md#3-protocol-parameters), [verifier results](bench/p003-sis/verifier-results.jsonl) |
| P-4 — SIS tuple, and `ACTIVE_SIZE_PARAM` | **UNFILLED** — the proposed tuple is rejected and the candidate family is not ratified; Al rules both from the solver-envelope report | [Spec §3](docs/PROTOCOL_SPEC.md#3-protocol-parameters), [P-003 report](loop/reports/C3-p003-builder.md) |
| P-7 — reward cap and reward curve | **TBD** — owner: Al + Sarah after the D16 reveal | [Spec §3](docs/PROTOCOL_SPEC.md#3-protocol-parameters), [D16 ratification](loop/LEDGER.md#effective-ratifications) |
| P-8 — size limits | `TX_MAX_BYTES = 188`; `SOL_MAX_BYTES = 2 * m`, unfilled while P-4 is; `BLOCK_MAX_BYTES = 1,048,576` **PROVISIONAL**, with no evidence behind the number | [Spec §3](docs/PROTOCOL_SPEC.md#3-protocol-parameters) |
| P-10 — timestamp drift, and the timestamp unit | **UNFILLED** — Al rules both together with P-1 | [Spec §3](docs/PROTOCOL_SPEC.md#3-protocol-parameters) |
| P-9 — minimum fee | **TBD** — owner: Al | [Spec §3](docs/PROTOCOL_SPEC.md#3-protocol-parameters) |
| P-12 — subsidy schedule | Placeholder only; owner: Al + Ken | [Spec §3](docs/PROTOCOL_SPEC.md#3-protocol-parameters) |
| Codec/domain bytes, strict Ed25519, SIS coefficient encoding, and SHAKE candidate convention | **RULED 2026-11-22; implemented and merged in PR #21 (`333c6708`)** — the Rust codecs, the vectors and the fuzz targets that P-007 owed. Three vector files are NORMATIVE conformance vectors by the ruling of 2026-11-23. PR #14, merged 2026-10-12, carried only the checked `Amount` boundary | [Spec §1](docs/PROTOCOL_SPEC.md#1-notation-and-cryptographic-primitives), [P-007 report](loop/reports/C10-p007-builder.md), [vectors guide](bench/p007-vectors/README.md), [SI-002/SI-003](loop/reports/SPEC-ISSUES.md) |
| Audit matrix v0.2 reconciliation | **COMPLETE** — 33 security findings, 25 Lean claims, R1–R8; GAP-7–13 were ratified on 2026-11-22 as requirements for later packets and none is implemented | [audit matrix](docs/AUDIT_TRACEABILITY.md), [P-009 report](loop/reports/C7-p009-builder.md) |

## Roadmap

The phase gates are evidence thresholds, not calendar or deployment promises
([full criteria](docs/ENGINEERING_PLAN.md#4-phase-plan)).

| Gate | Planned phase outcome | Current position |
|---|---|---|
| **G0 — Foundations and spikes** | P-001–P-007 reports complete; D1/D2/D14/D15 and open interpretation issues resolved with spike evidence | **PASSED on 2026-11-23, with items carried.** D1 stands on a condition, and packet P-011 measures what that condition asks ([Gate G0 ruling of 2026-11-23](loop/LEDGER.md#gate-g0-f-ruling-overlay--gate-g0-pass--pr-21-confirmed--report-c10-points-ruled--phase-1-open-al-2026-11-23), [gate report](loop/reports/C11-g0f-gate-builder.md)) |
| **G1 — Kernel** | Lean-vector conformance, conservation properties, mutation spot-check, genesis spend, and crash consistency | **▶ CURRENT — Phase 1 is open; P-101 runs first and is at its FRAME, stopped for Al's ruling; no kernel code exists** ([Plan G1](docs/ENGINEERING_PLAN.md#4-phase-plan), [P-101's FRAME](loop/reports/C12-p101-builder.md), [packet queue](loop/PACKETS.md)) |
| **G2 — Consensus** | Deterministic two-node replay and typed rejection of the adversarial block corpus | Planned after G1 ([Plan G2](docs/ENGINEERING_PLAN.md#4-phase-plan)) |
| **G3 — Network and RPC** | Three-node devnet adversarial replay and complete fail-closed auth matrix | Planned after G2 ([Plan G3](docs/ENGINEERING_PLAN.md#4-phase-plan)) |
| **G4 — Testnet and integration seam** | Testnet operations, external solver fleet, adversarial replay, and stable frontend read/event surface | Planned after G3 ([Plan G4](docs/ENGINEERING_PLAN.md#4-phase-plan)) |

## License

License: TBD 
