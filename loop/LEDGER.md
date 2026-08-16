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

## Audit traceability amendment overlay — RATIFIED (Al, 2026-08-16)

The following non-preempting live ruling was received during P-004 and applied at its
next packet boundary, after P-004's exact-merge-SHA mainline D6 completed:

```text
GOVERNANCE INJECTION — non-preempting. Do NOT interrupt the in-flight packet;
process this at the next packet boundary.

docs/AUDIT_TRACEABILITY.md is already placed — verify presence (SHA-256
58C762568A63D8A4AFEACDEB1535AA9C930099F49118D2B45B9B483B8D560EFC) and commit it.

AMEND-1, AMEND-2, AMEND-3 (defined in that document, GAP-2/3/4) are RATIFIED —
apply them to PROTOCOL_SPEC §12 and the Gate G1 criteria in the same commit,
and record the ratification in the LEDGER overlay.

P-009 is approved, appended to the queue, and UNBLOCKED. Source documents:
  1. C:\Users\LEET\Downloads\COINjecture-2.0-Security-Audit.docx  (third-party audit)
  2. C:\Users\LEET\Downloads\DARQ-LV-001_COINjecture_v2.6_Lean_Audit.pdf  (optional
     third source — Lean audit; flag anything bearing on CJ3's A8 track)
First action of P-009: copy both into loop/evidence/ and commit — Downloads is
not durable evidence storage. The Codex security-scan report source is still
outstanding; if its findings are already fully represented in the committed
2.0 remediation record, say so with pointers rather than waiting on the file.

Continue continuous-batch afterward.
```

### Effective audit amendments and queue ruling

- **AMEND-1 — RATIFIED:** Protocol Spec §12 now requires explicit object-level
  authorization enumeration for every non-public endpoint and bounded remote-input
  log emission/rate discipline.
- **AMEND-2 — RATIFIED:** Gate G1 now requires a forced mid-block-apply process-death
  test proving that `cj3-store` restarts at the exact pre-block state.
- **AMEND-3 — RATIFIED:** every future value-moving structure must extend the
  normative Lean V-rules before any Rust implementation exists.
- **P-009 — APPROVED / UNBLOCKED:** perform read-only source-audit traceability
  verification. The two supplied audit files enter durable `loop/evidence/` storage
  before analysis; the missing Codex scan file does not block if its findings are
  already fully represented by exact committed 2.0 remediation pointers.

## P-010 documentation governance overlay — RATIFIED (Al, 2026-08-16)

The following non-preempting ruling was received after the P-005 HUMAN-lane stop and
applied at that packet boundary without merging or modifying P-005's formal-spec branch:

- **P-010 — APPROVED / AUTO:** replace the repository-root `README.md` with the
  institutional document specified by the live ruling. The packet is docs-only and
  has no consensus-semantic authority.
- **Queue placement:** P-010 is appended after P-009 in the durable queue. The live
  instruction to process it at the next packet boundary authorizes this documentation
  packet now; it does not mark P-005 complete or unblock P-006/P-007/P-009.
- **Claims discipline:** present capabilities require evidence pointers; future work,
  assumptions, and owned TBDs remain explicit. The repository remains private and
  pre-testnet, with no mainnet claim or configuration.
- **License:** `License: TBD — owner: Al/Ken`. P-010 does not create a `LICENSE` file
  or select a license.

### Standing README drift-control rule — RATIFIED (Al, 2026-08-16)

The README status block and roadmap current-position marker MUST be refreshed in the
same closeout that records every phase-gate decision (G0 through G4). A gate closeout
is incomplete if either surface is stale or omitted. This standing rule keeps the
repository entry document synchronized with the gate evidence in `loop/STATE.md`,
`loop/PACKETS.md`, and `loop/reports/`.

## P-005 HUMAN-lane authorization and review tail — RATIFIED (Al, 2026-08-16)

The following live ruling authorizes the previously stopped HUMAN-lane implementation
while preserving every ownership boundary and adding a non-automatic review tail:

```text
P-005 HUMAN-LANE AUTHORIZATION — RATIFIED, with review tail.

The builder is authorized to implement P-005's Spec/*.lean content and JSON
vector definitions strictly from ratified Protocol Spec §§7–8 and the §14
vector case list. Constraints:

1. All Al-owned, Sarah-owned, and G0-controlled TBDs remain symbolic or
   unfilled. Test-only fixture values must be explicitly labeled
   non-normative and must not populate any protocol TBD.
2. SI-001, SI-002, and SI-003 remain unresolved and must not be silently
   decided through Lean code or vectors. Where a V-rule depends on one
   (e.g., V1 on canonical-encoding details), model the dependency as an
   abstract, clearly marked interface citing its SI reference — never a
   concrete choice.
3. Any new semantic ambiguity triggers a fresh stop, not an inferred choice.
4. Each Spec/*.lean file carries a header: "NORMATIVE STATUS: draft —
   pending human ratification; formal-verification ownership reserved per
   LEDGER D16." No Sarah contact, review attribution, or notification
   before the D16 reveal.
5. MERGE RULE: the completed P-005 PR does NOT auto-merge. Finish the
   packet through the adversary pass and green CI, mark PR #9
   ready-for-review, and stop it there for my line-by-line review of the
   Lean encoding against Protocol Spec §§7–8. My merge is the packet's
   done-condition. That review is what licenses kernel autonomy at P-101;
   it is not skippable.
6. QUEUE: while PR #9 awaits my review, continue continuous-batch out of
   strict order with P-006, P-007, P-009 (interpretation of the ingested
   audit sources now permitted), and P-010 if present in the queue.
   P-101 remains blocked on Gate G0 AND the merged, human-reviewed P-005.

Resume.
```

### Effective P-005 controls

- **Implementation authority:** P-005's Lean files and JSON vector definitions may be
  implemented only from Protocol Spec §§7–8 and §14's vector case list.
- **Owned and gated values:** Al-owned, Sarah-owned, and G0-controlled TBDs remain
  symbolic or absent. Any test fixture is explicitly non-normative and cannot fill a
  protocol TBD.
- **SI integrity:** SI-001, SI-002, and SI-003 remain unresolved. A dependent rule is
  represented by an abstract interface that cites the applicable SI; code and vectors
  cannot select an encoding or other concrete resolution.
- **Ambiguity tripwire:** a new semantic ambiguity is a fresh HUMAN stop.
- **Normative status:** every `Spec/*.lean` file carries the exact required draft and
  ownership header. No Sarah contact, attribution, invite, or notification occurs
  before D16 reveal.
- **Review tail:** PR #9 completes builder work only after its adversary pass and exact-
  head green CI, when it is marked ready for Al's line-by-line review. It MUST NOT be
  auto-merged. Al's merge is the packet done-condition.
- **Queue exception:** while PR #9 awaits that review, P-006, P-007, P-009, and P-010
  (if present) may run out of strict order, subject to their own tripwires and D11.
- **P-101:** remains doubly blocked on Gate G0 and a merged, human-reviewed P-005.

## SI-004 bounded-above quality normalization — RATIFIED (Al, 2026-08-16)

The following live HUMAN ruling resolves SI-004 and resumes P-005 without weakening
any standing P-005 constraint:

```text
SI-004 RATIFIED — Model 4 (bounded-above quality normalization).

§11 amendment: reward(height, Q) = subsidy(height) · min(Q, R_MAX·SCALE)
  / (R_MAX · SCALE), computed in u128 with floor division, checked back to
  u64. Range: reward ∈ [floor(subsidy/R_MAX), subsidy]. R_MAX ≥ 1.
  R_MAX's semantics change from "maximum inflation multiple" to "quality
  span divisor": a threshold solution (Q = SCALE) earns subsidy/R_MAX; a
  solution at or beyond R_MAX·SCALE earns the full subsidy. R_MAX = 1
  degenerates to reward = subsidy for every valid block. Record the
  semantic change against P-7. R_MAX's VALUE and the curve shaping
  (including any μ-balance normalization) remain owner-controlled and
  UNFILLED — owner: Al (+ Sarah, reserved per D16).

§8 amendment: conservation becomes
  Σ balances(post) = Σ balances(pre) + reward(height, Q),
  subject to the standing invariant reward(height, Q) ≤ subsidy(height).
  subsidy(height) is a per-block issuance CEILING, not a target; realized
  issuance may fall below schedule and the unminted remainder is never
  minted. Fees transfer only, never mint — unchanged.

Funding source: none required. No treasury, reserve, or premium account is
introduced; no insufficient-funds branch exists by construction.

LEAN OBLIGATION: Spec/Stf.lean must carry reward ≤ subsidy as a proved
theorem under floor division, not an assumption — it is the invariant that
makes the ceiling real.

Resume P-005 under the standing HUMAN-LANE AUTHORIZATION (all six
constraints still bind, including the no-auto-merge review tail). Re-FRAME
the affected theorem and vector surface, complete the full lake build and
adversary pass, obtain exact-head D6, mark PR #9 ready-for-review, and STOP
for my review. Do not merge.
```

### Effective SI-004 controls

- **Reward formula:** derive the credited amount with u128 intermediate arithmetic
  and floor division by `R_MAX·SCALE`; validators derive it from Q and never read it.
- **Issuance ceiling:** §8 conservation adds realized reward, and formalization proves
  `reward ≤ subsidy` from the formula plus `R_MAX ≥ 1`. That inequality is not an
  interface assumption.
- **P-7 semantics and ownership:** `R_MAX` is a quality span divisor. Its value and
  curve shaping, including μ-balance normalization, remain UNFILLED and owned by Al
  (+ Sarah, reserved per D16).
- **No funding source:** there is no treasury, reserve, premium account, carry-forward,
  or insufficient-funds branch. Unminted subsidy never enters state.
- **P-005 tail:** all six standing HUMAN constraints remain binding. PR #9 is marked
  ready only after full local verification, adversary review, and exact-head D6; the
  builder never merges it.

## P-005 first-review findings — BINDING (Al, 2026-08-16)

Al completed the first mandatory line-by-line review of PR #9 without merging it and
returned the following live ruling. It resumes the same HUMAN lane; every earlier
ownership, SI, capacity, phase-gate, no-Sarah-contact, and no-auto-merge control remains
in force.

```text
P-005 REVIEW COMPLETE — NOT MERGED. Three findings; resume under the
standing HUMAN-LANE AUTHORIZATION (all six constraints still bind).

F1 (blocking merge) — ConservationTarget is stated but undischarged and
unprovable against abstract StateOps. Add a LawfulStateOps structure with
the state-ops laws (totalBalances/setAccount additive law, read-after-write,
read-other-address), then PROVE conservation for applyTransaction and
applyBlockCandidate under it, with explicit case analysis over the aliasing
cases: sender=recipient, sender=miner, recipient=miner, and all three equal.
Record the laws in the spec as P-101 conformance obligations. Do not
introduce any axiom or sorry — if a law cannot be stated without one, STOP
and report.

F2 (blocking merge) — remove the `unreachable!` from Tx.validate. A panic
on the validation path violates SPEC §12 even when provably dead. Keep
v9_allows_self_send as the documenting theorem.

F3 (obligation, not a code change) — record in PROTOCOL_SPEC §11 and the
LEDGER: Context.rewardInputs MUST be instantiated as the checker output
over the derived instance — check(derive_instance(instance_seed,
size_param), solution) — and MUST NOT read any block-supplied quality
field. Cite it as the C2 structural boundary. This becomes a P-101 binding
obligation and a G2 gate check.

Accepted as correct, no change needed: V1–V9 independence and ordering, V3
binding at the validation site, V4 strict equality, per-transaction
revalidation against evolving state, sequential-read aliasing handling,
atomic rejection, and the Model 4 reward theorem surface.

Work on feat/p005-lean-scaffold. Keep PR #9 open, ready-for-review, UNMERGED
— return to me for a second review after F1/F2/F3. Batch P-006, P-007, and
P-009 may run in parallel with this work.
```

### Effective first-review controls

- **Lawful store boundary:** P-005 defines and reasons from exactly three classes of
  concrete-store law: read-after-write, read-other-address, and the additive
  `totalBalances/setAccount` replacement equation. P-101 MUST prove those laws for its
  authenticated store; an axiom, `sorry`, or assumed conservation result is forbidden.
- **Conservation discharge:** successful `applyTransaction` preserves total balances
  in every realizable sender/recipient/miner alias partition. Successful
  `applyBlockCandidate` increases total balances by exactly the derived Model 4 reward.
- **Panic-free validation:** `Tx.validate` has no panic path. V9 remains the theorem-
  documented always-allow rule.
- **C2 reward provenance:** P-101 MUST instantiate `Context.rewardInputs` only from
  `check(derive_instance(instance_seed, size_param), solution)` for the validated
  derived instance. It MUST NOT read block-supplied quality, work score, timing, or an
  equivalent miner-controlled field. Gate G2 checks this structural boundary with
  wrong-instance and quality-spoof adversarial cases.
- **Second review tail:** PR #9 stays open, ready-for-review, and unmerged throughout
  remediation. After full adversary review and exact-head D6, it returns to Al; only
  Al's merge can complete P-005 or help unblock P-101.
