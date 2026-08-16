# COINjecture 3.0 Decision Ledger

This ledger has three layers:

1. the required Cycle 0 seed of D1–D15 from `docs/ENGINEERING_PLAN.md` §3, with the
   source statuses preserved verbatim; and
2. later Al rulings/additions received during the blocked Cycle 0 session. Later
   rulings supersede the corresponding seed status without rewriting the historical
   seed; and
3. the 2026-08-15 live autonomy ruling, preserved verbatim below, which ratifies the
   remaining Cycle 0 decisions and authorizes the Phase 0 packet queue.

## D1–D15 seed from Engineering Plan §3

Statuses below are preserved from the supplied Engineering Plan:
**ACCEPTED** (binding now), **PROPOSED** (awaiting Al), and **OPEN — Al** (Al owns).

| ID | Seed status | Decision |
|----|-------------|----------|
| D1 | PROPOSED | Genesis problem class = **lattice SIS** behind a `ProblemClass` trait registry; legacy 2.0 classes enter only via the P-004 admission bench. *(Full ADR below.)* |
| D2 | PROPOSED | Consensus skeleton = **Nakamoto longest-chain with eligibility/quality decoupling** (deviation from RS §5's Stage-3 recommendation; full ADR below). |
| D3 | ACCEPTED | Axiom set A1–A11 binds all packets. Violations are Criticals regardless of test status. |
| D4 | ACCEPTED | Verification is **Tier 0 (full deterministic recomputation) only** at genesis. Tier 1 (optimistic fraud proofs) interface is reserved but unbuilt; trigger: admission of a class whose checker exceeds the per-block validation budget (§SPEC P-3). Tier 2 (SNARK/folding) trigger: proving a class checker at <10× native solve cost with <100 ms verify at target sizes (RS §4 threshold). |
| D5 | ACCEPTED | Lean 4 specifies Tx validity + STF **first** (RS §6: exactly the layer that failed 2.0's audit; deterministic, no distributed reasoning needed). Conformance = generated vectors in CI, not extraction (Lean→Rust extraction is not production-ready; Aeneas runs the other direction). Fork-choice spec is Phase 2+. |
| D6 | ACCEPTED | CI gates from the **first commit**: `cargo audit --deny warnings`, `cargo deny check` with committed `deny.toml`, `clippy -D warnings`, `cargo geiger` (zero unsafe in `cj3-*` consensus crates), `fmt --check`, tests, committed `Cargo.lock`, codec fuzz smoke jobs, conservation-invariant test, genesis spend-test (§6). (Ports BEANlet D10; closes 2.0's dependency-audit blind spot.) |
| D7 | ACCEPTED | Amounts are `u64` newtypes with `checked_*` only; overflow/underflow is `Invalid`, never saturate/wrap. Conservation invariant is a CI property test. |
| D8 | ACCEPTED | Single-language Rust core. No C/C++ FFI on apply/fork-choice paths; solvers out-of-process untrusted (A9). Python permitted **only** in `bench/` analysis tooling, never in the node. Haskell/Go/C++ enter only via a future ADR proving a boundary that demands them. |
| D9 | **OPEN — Al** | Repo home: `COINjecture-Network` org vs `Quigles1337` personal. Recommendation: migrate to the org **before Phase 1 merge history accumulates** — the org exists so the flagship isn't in one person's namespace, and CJ3 doubly so. Builder STEP 0 pins to whichever remote is ratified here. |
| D10 | **OPEN — Al** | Sarah disclosure. Recommendation (standing from prior discussion): the two-line heads-up before Phase 0 code lands. The plan is structured either way so her seat is modular — the Lean track (P-005, A8) is the natural entry seam and touches no Rust build loop. Blocked packet on this decision: none mechanically; P-005 review quality degrades without her. |
| D11 | ACCEPTED | **Capacity: COINjecture 2.0 remediation outranks all CJ3 packets.** The CJ3 loop runs only while the 2.0 queue is blocked-on-external (e.g., awaiting GATE-1/GATE-2) or drained. The DARQ-021 + C1 + C2 coordinated fork on 2.0 is the standing critical path. `CAPACITY_FLAG: remediation-priority` in either project's loop pauses CJ3. |
| D12 | ACCEPTED | Testnet-only network allowlist hardwired in policy paths; no mainnet configuration exists in the codebase during this program. |
| D13 | ACCEPTED | Honest-claims rule (A10) applies to README, docs, and any public material generated from this repo. |
| D14 | PROPOSED | Beacon = VDF over parent-chain data behind a trait. Technology selected by P-002 spike (pure-Rust Wesolowski/Pietrzak availability is the open question — chiavdf is C++, which A9 forbids on the trusted path). Devnet placeholder: iterated-hash delay behind the same trait, compile-time-gated, with a `NOT-TESTNET-GRADE` banner. |
| D15 | PROPOSED | Signature scheme Ed25519 with domain-separated canonical encoding. Confirm against 2.0 key continuity desires before freeze (owner: Al; if 2.0 wallets should carry over, match its curve). |

## Effective Cycle 0 rulings and additions

### D9 — RULED (Al, 2026-08-15)

CJ3 stays in Quigles1337/COINjecture3.0 (private) through the build. Org migration
occurs at the D16 reveal event. This ruling supersedes D9's seed status of OPEN.

### D10 — RULED (Al, 2026-08-15)

No pre-disclosure. Sarah joins at the D16 reveal. Standing condition: 2.0 remediation
stays fully live per D11 — nothing Sarah does on 2.0 during the window may be secretly
mooted. This ruling supersedes D10's seed status of OPEN.

### D16 — PROPOSED

Reveal gate = G1 (kernel green against the Lean vectors); hard ceiling = before Phase 2
design freeze. Reveal event = org migration + Sarah invite + handoff of
Sarah-owned TBDs. Her TBDs (P-7, reward curve, μ-balance hook, Lean review seat) stay
UNFILLED until reveal.

## Live autonomy ruling overlay — RATIFIED (Al, 2026-08-15)

The following live ruling is preserved verbatim. It supersedes the pending Cycle 0
items above without rewriting their historical status:

```text
LIVE CJ3 RULING — 2026-08-15

Use C:\Users\LEET\COINjecture3.0 as the canonical checkout. Do not use the empty
C:\Users\LEET\projects\COINJECTURE 3.0 repository; I am deleting it.

D1 RATIFIED as proposed.
D2 RATIFIED as proposed.
D14 RATIFIED as proposed, with final VDF technology selected by P-002.
D15 RATIFIED as proposed.

D16 RATIFIED: reveal at G1, with a hard ceiling before the Phase 2 design freeze.
At reveal, migrate to the organization, invite Sarah, and hand off Sarah-owned TBDs.
Until reveal, all Sarah-owned TBDs remain unfilled.

D17 RATIFIED: autonomous continuous-batch execution is authorized under the supplied
AUTONOMOUS_BUILDER prompt, subject to every tripwire, HUMAN-lane restriction, phase
gate, capacity check, and stop condition in that prompt.

The existing D9 and D10 rulings in loop/LEDGER.md are confirmed.

The P-001 through P-007 queue is approved in order. P-008 remains blocked until I
supply the exact frontend URL. P-101 remains blocked on Gate G0.

Current D11 capacity condition:
COINjecture 2.0's agent queue is blocked pending Sarah's GATE-1/GATE-2 answers and
cannot advance until they arrive; CJ3 may proceed under D11 until those gates clear,
at which point 2.0 immediately reclaims capacity and CJ3 pauses.

The required research survey is already placed at
C:\Users\LEET\COINjecture3.0\docs\RESEARCH_SURVEY.md — verify presence and commit it.

First governance commit: record this ruling as the LEDGER overlay (D1/D2/D14/D15/D16/
D17 RATIFIED, D9/D10 confirmed), correct loop/STATE.md BRANCH from master to main,
commit the autonomous prompt as loop/PROMPTS/AUTONOMOUS_BUILDER.md, and commit
docs/RESEARCH_SURVEY.md. Then run the full autonomy preflight, bootstrap-push main
after verifying the repo is PRIVATE, and start P-001 on feat/p001-repo-scaffold in
continuous-batch mode. STOP at Gate G0.
```

### Effective ratifications

- **D1 — RATIFIED:** lattice SIS is the genesis problem class behind the
  `ProblemClass` registry, as proposed.
- **D2 — RATIFIED:** Nakamoto longest-chain with eligibility/quality decoupling is the
  consensus skeleton, as proposed.
- **D14 — RATIFIED:** the beacon is a trait-gated VDF design, with final technology
  selected by P-002 and the marked devnet-only placeholder retained as proposed.
- **D15 — RATIFIED:** Ed25519 with domain-separated canonical encoding, as proposed.
- **D16 — RATIFIED:** reveal at G1, no later than the Phase 2 design freeze; the reveal
  event is organization migration, Sarah invite, and handoff of Sarah-owned TBDs.
- **D17 — RATIFIED:** autonomous continuous-batch mode is authorized under
  `loop/PROMPTS/AUTONOMOUS_BUILDER.md` and every guard named in the live ruling.
- **D9/D10 — CONFIRMED:** the prior effective rulings remain unchanged.

### Effective queue and capacity ruling

- P-001 through P-007 are approved in order.
- P-008 remains blocked on the exact Al-supplied frontend URL.
- P-101 remains blocked on Gate G0.
- D11 capacity is clear only while the COINjecture 2.0 agent queue remains blocked on
  Sarah's GATE-1/GATE-2 answers. If those gates clear, CJ3 pauses immediately.
