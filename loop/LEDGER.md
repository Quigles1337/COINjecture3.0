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
  consensus skeleton, as proposed and subsequently amended by the Gate G0-A ruling
  below.
- **D14 — RATIFIED:** the beacon is a trait-gated VDF design, with final technology
  selected by P-002 and the marked devnet-only placeholder retained as proposed.
- **D15 — RATIFIED:** Ed25519 with domain-separated canonical encoding, as proposed.
- **D16 — RATIFIED:** reveal at G1, no later than the Phase 2 design freeze; the reveal
  event is organization migration, Sarah invite, and handoff of Sarah-owned TBDs.
- **D17 — RATIFIED:** autonomous continuous-batch mode is authorized under
  `loop/PROMPTS/AUTONOMOUS_BUILDER.md` and every guard named in the live ruling.
  - **The five D17 conditions (Al, P-007 resume ruling, clause 2.4).** They are the five
    that report C1 names: approved and unblocked; bounded non-semantic surface; no
    formal-spec content; no decision invention; no owned-TBD fill. This is the list
    that `loop/PROMPTS/AUTONOMOUS_BUILDER.md` means by "all five D17 conditions". Each
    packet's FRAME still argues them for itself.
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

## P-005 second-review approval and human ratification — RATIFIED (Al, 2026-08-16)

The following live ruling accepts every first-review remediation, authorizes the
reviewed HUMAN-lane PR to merge, and fixes the resulting P-101/toolchain boundary:

```text
P-005 SECOND REVIEW COMPLETE — APPROVED. Merge PR #9 into main.

F1, F2, and F3 are accepted as closed. The conservation proof, the
panic-free validation path plus its CI regression gate, and the binding
checker-derived reward provenance obligation all satisfy the review.

Record in the LEDGER: the Lean encoding of Protocol Spec §§7–8 (V1–V9 and
the STF, including conservation under LawfulStateOps) is HUMAN-RATIFIED as
of this merge. Update the Spec/*.lean normative-status headers from "draft
— pending human ratification" to "RATIFIED — human-reviewed; formal-
verification ownership reserved per LEDGER D16". Vectors remain
non-normative and symbolic pending SI-001/002/003.

P-101's binding obligations, carried forward: the three LawfulStateOps
laws; rewardInputs provenance per F3; concrete checked construction of the
R_MAX × SCALE divisor; storage read/write coherence.

After merge, verify the exact-merge-SHA D6 run green, then resume
continuous-batch with P-006, P-007, and P-009 per D17. P-101 remains
blocked on Gate G0 only. Also add a LEDGER note: CJ3 pins Lean 4.33.0 while
COINjecture 2.0's lean4 project is on 4.28.0 + Mathlib 4.28.0 — a toolchain
gap to resolve at the D16 reveal, not before.
```

### Effective second-review controls

- **F1/F2/F3 — CLOSED:** Al accepts the constructive conservation proof under
  `LawfulStateOps`, the panic-free validation path and regression gate, and the C2
  checker-derived reward provenance obligation.
- **Human ratification:** effective with PR #9's reviewed merge, the Lean encoding of
  Protocol Spec §§7–8 — V1–V9 and the STF, including conservation under
  `LawfulStateOps` — is HUMAN-RATIFIED. Every `Spec/*.lean` file carries
  `NORMATIVE STATUS: RATIFIED — human-reviewed; formal-verification ownership reserved
  per LEDGER D16`.
- **Vector boundary:** the generated JSON vectors remain non-normative, symbolic, and
  incapable of resolving SI-001, SI-002, or SI-003. No canonical byte choice or owner
  value is ratified by the formal-source status change.
- **P-101 binding obligations:** P-101 MUST (1) discharge read-after-write,
  read-other-address, and additive `totalBalances/setAccount` replacement for its
  concrete store; (2) source `Context.rewardInputs` only from
  `check(derive_instance(instance_seed,size_param),solution)` under F3; (3) construct
  `R_MAX·SCALE` with checked u128 multiplication, prove the divisor nonzero before
  floor division, and check the result back to u64; and (4) prove the kernel's actual
  storage reads and writes coherent with the three laws, with no bypass path.
- **Queue effect:** after PR #9 merges and its exact-merge-SHA D6 is green, P-101 is
  blocked on Gate G0 only. D17 resumes P-006, P-007, and P-009 subject to their
  tripwires and the fresh D11 capacity check.
- **D16 toolchain gap:** CJ3 pins Lean 4.33.0. Al records the COINjecture 2.0 Lean
  project at Lean 4.28.0 plus Mathlib 4.28.0. Compatibility is an explicit reveal-time
  gap to resolve at D16; no pre-reveal migration, Sarah contact, or silent version
  alignment is authorized.

## Gate G0 partial ruling overlay — G0-A RATIFIED / G0 HOLD (Al, 2026-08-16)

The following live ruling is preserved verbatim. It ratifies G0-A only, amends D2,
and leaves the overall gate and every other listed surface on HOLD:

```text
GATE G0 — PARTIAL RULING. G0 disposition: HOLD (not PASS).

G0-A RATIFIED — Option 2, static size_param.
  size_param is a protocol constant, changed only by explicit human-ratified
  protocol upgrade. No dynamic size retarget exists in the codebase. B3
  validates size_param against the constant. P-11 is struck as moot.
  P-1/P-2 apply to the hash-target knob ONLY: propose values derived from
  P-006's stable hash-only envelope (36/36) with the supporting evidence,
  for my review — do not fill them unilaterally.
  Record in LEDGER as an amendment to D2, with this rationale: the failure
  is in size calibration, not fork choice; D2's decoupling means an
  easy-drifting instance degrades usefulness, not chain security.
  Record the honest consequence in README limitations: CJ3's useful-work
  difficulty is governance-calibrated, not self-calibrating.
  Add a standing item: size adequacy is re-reviewed at every phase gate.

NEW STANDING LESSON (LEDGER) — SELECTION-BIAS CHANNEL: a consensus quantity
  may be honestly derived and still adversarially distributed, because
  miners choose which valid solutions to publish. Any future control loop
  that aggregates published miner behavior must state its resistance to
  strategic withholding or be rejected. Cite P-006's retention figures.

G0-B, G0-C, G0-D, G0-E: HOLD. I am drafting these with a reviewer and will
  supply normative text. Do NOT infer, propose defaults for, or partially
  implement them. P-007/PR #14 stays draft and unmerged.

Permitted work while G0 holds: nothing that touches held surfaces. If no
  packet qualifies, STOP and idle — do not manufacture work.
```

### Effective G0-A amendment to D2

- **Gate disposition:** G0 is **HOLD**, not PASS. P-101 remains blocked. G0-B through
  G0-E remain HUMAN-held and receive no defaults, partial implementation, or inferred
  wording from this overlay.
- **Static useful-work size:** `size_param` is an active protocol constant. A block
  MUST carry exactly that constant, B3 MUST reject any other value, and the constant
  changes only through an explicit human-ratified protocol upgrade. There is no
  dynamic `size_param` retarget or published-quality controller on any protocol or
  runtime path. The sealed P-006 offline simulator remains historical rejection
  evidence only; it is not an admitted implementation.
- **P-11:** `SIZE_RETARGET_WINDOW` is **STRUCK — MOOT**. Its historical P-006 test
  range remains evidence, not an active parameter surface.
- **Hash-only scope:** P-1 and P-2 govern only the Nakamoto hash-target cadence and
  controller. They do not govern instance size.
- **D2 rationale:** P-006 falsified the tested size-calibration controller, not the
  longest-chain fork-choice skeleton. Because D2 decouples checked useful-work
  validity/quality from chain weight, an instance that becomes too easy degrades the
  usefulness/calibration objective; it does not grant that quality signal more chain
  weight or independently break fork choice.
- **Governance consequence:** useful-work difficulty is governance-calibrated rather
  than self-calibrating. Every G0–G4 phase-gate closeout MUST re-review whether the
  active static `size_param` remains adequate and either retain it explicitly or route
  a change through a human-ratified protocol upgrade.

### G0-A integration and P-2 provisional ratification overlay (Al, 2026-08-16)

The following live follow-up ruling is preserved verbatim:

```text
G0-A MERGED. P-2 RATIFIED AS PROVISIONAL: EMA window 32, gain 1/8,
multiplicative clamp [8/9, 9/8], on the stated rationale that this is a
conservative interior point of the 36/36 passing hash-only region, not a
tuned optimum. Marked provisional and subject to re-derivation once P-1 is
ratified. P-1 remains UNFILLED — correctly, since P-006's normalized
interval cannot justify an absolute cadence; do not propose one.

Complete the G0-A closeout: verify the exact-merge-SHA D6 run green, write
the BATCH-LOG integration row with real evidence, update STATE.md.

Then STOP at G0 HOLD. G0-B/C/D/E remain held; PR #14 stays draft. If no
packet qualifies, idle — do not manufacture work.
```

### Effective P-1/P-2 disposition

- **P-1 — UNFILLED / no proposal:** P-006 normalized its target interval to `1.0`
  and contains no propagation, reference-miner, production-VDF-delay, or solved-SIS
  distribution evidence from which seconds can be derived. A numeric P-1 would be
  invention. No builder proposal is authorized.
- **P-2 — RATIFIED AS PROVISIONAL:** EMA window `W = 32` blocks, controller gain
  `1/8`, and multiplicative per-update clamp `[8/9, 9/8]`. The human-ratified rationale
  is that this is a conservative interior point of P-006's 36/36 passing hash-only
  region, not a tuned or statistically preferred optimum. P-2 MUST be re-derived when
  P-1 is ratified.
- **Implementation status:** this ratifies the protocol parameter only. It does not
  admit a hash-retarget implementation while P-1 is unfilled or authorize any held
  G0-B/C/D/E choice.
- **Integration evidence:** G0-A merged through PR #18 at
  `9bbbd43fd9e4c07f8b389f182b34281183be3737`. Exact-head D6 run `31975177926` and
  exact-merge-SHA D6 run `31977067852` each passed all eleven jobs.

### Standing lesson — selection-bias channel

A consensus quantity can be honestly recomputed and still be adversarially
distributed because miners choose which valid solutions to publish. Any future
control loop that aggregates published miner behavior MUST state and evidence its
resistance to strategic withholding or be rejected. P-006's persistent-selection
model retained only `0.423–0.806` of honest equilibrium size at 35% strategic share
and `0.240–0.700` at 51%; changing window speed delayed the drift but did not remove
its direction.

## Gate G0 ruling overlay — G0-B to G0-E RULED / G0-F HELD (Al, 2026-11-22)

The following live ruling is preserved verbatim. It rules G0-B, G0-C, G0-D and G0-E,
holds G0-F, and leaves the gate disposition at HOLD. Where it differs from an earlier
entry in this ledger, it supersedes that entry without rewriting it.

```text
LIVE CJ3 RULING - 2026-11-22
GATE G0: G0-B, G0-C, G0-D AND G0-E RULED. G0-F HELD. GATE DISPOSITION: HOLD (NOT PASS).

Read loop/PROMPTS/AUTONOMOUS_BUILDER.md, loop/LEDGER.md and loop/STATE.md first. Then act as
the BUILDER seat under that prompt, as Part 0 amends it. This session's whole scope is Part 6.
Write no protocol code in this session. If any clause below is ambiguous, or conflicts with an
axiom, the Lean files or another clause, STOP and name the clause. Do not choose a reading.

PART 0 - THE STATE OF THE RECORD

0.1 Visibility. The repository is PUBLIC, by my decision. D9's "private through the build" is
    superseded. Preflight step (d) now requires the visibility query to return PUBLIC, and any
    other value is a STOP. Change that step in loop/PROMPTS/AUTONOMOUS_BUILDER.md and nothing
    else in that file. "No visibility changes" stays a standing prohibition.
0.2 D10 and D16 are not ruled here. Every value and seat they reserve stays reserved and
    UNFILLED. Every prohibition on contacting or notifying anyone stays in force.
0.3 Capacity (D11). No COINjecture 2.0 remediation work is queued or running. CJ3 may proceed.
    This is the capacity condition for preflight step (f) and for the pickup guard until I
    state another.
0.4 PR #14 was merged on 2026-10-12 as 358114048c3a38f4821b88fce8dffd14abbd42a1, and D6 CI run
    38323998550 on that commit concluded success. The merge stands. It carried only the
    checked Amount boundary and the P-007 stop report. P-007 remains INCOMPLETE. Correct every
    line in STATE, PACKETS and README that calls PR #14 draft or unmerged, and add a line to
    BATCH-LOG that records the merge. Earlier BATCH-LOG lines and reports stay as written.

PART 1 - G0-B: SIS CLAIM LANGUAGE, P-3, P-4

B1 (SI-001) RESOLVED. In Protocol Spec 5.2 replace the whole "Hardness basis" bullet with:

      Hardness basis (an assumption, labeled as A10 requires). For uniformly sampled A, SIS
      has a conditional worst-case-to-average-case reduction lineage (Ajtai 1996;
      Micciancio-Regev 2007; surveyed in Peikert 2016). In parameter regimes that meet the
      reductions' asymptotic conditions, an efficient solver for random instances would give
      an efficient approximation algorithm for certain worst-case lattice problems. The
      reductions do not prove those worst-case problems hard, do not by themselves cover any
      concrete finite tuple, and say nothing about how long a solver takes on an instance. A
      concrete tuple therefore rests on three separately labeled things: the assumption that
      the worst-case problems are hard, estimator models at a pinned estimator commit, and
      measured solver and checker cost. None of the three is a proof. Parameters are chosen
      from a measured solver envelope (ruling G0-B4), with estimator output cited as a
      consistency check.

    Engineering Plan D1 keeps its text as the record of the decision; add one line under it
    that points to this resolution. Mark SI-001 RESOLVED.

B2 (P-3) ACCEPTED AS PROVISIONAL, with its scope corrected. VALIDATION_BUDGET = 15 ms.
    a. It covers the whole per-block cost of validating one solution: derive_instance plus
       check, single-threaded, release profile.
    b. It is an admission ceiling measured on reference hardware (D4). It is never a run-time
       check: no clock enters consensus (A3). At run time the tuple bounds the cost.
    c. The evidence so far times check alone on a pre-built matrix
       (crates/cj3-classes/examples/p003_check_bench.rs,
       bench/p003-sis/verifier-results.jsonl). Derivation is not in it. P-003's margins of
       2.68x and 1.92x describe check alone and are not to be quoted for the budget.
    d. Reference host v0 is the machine in bench/p003-sis/ENVIRONMENT.md. The measurement is
       repeated with derivation included, on that host and on one other class of machine,
       inside the packet B4 commissions. I re-rule the budget and the hardware definition
       from that.

B3 (P-4) REJECTED: the tuple (n, m, q, beta_squared) = (128, 3840, 16411, 3840), and the
    128-bit threshold as a way of choosing a tuple.
    a. Every block must carry a valid solution (B6), so the active tuple has to be one that
       miners can solve inside the block interval. bench/p003-sis/estimator-results.jsonl
       puts the cost of finding a solution at this tuple at 2^143.956 (rough model) to
       2^165.520 (default model) operations, at reduction block size 493.
       bench/p003-sis/solver-results.jsonl shows the bundled demonstrator solving n = 16 and
       returning no solution at n = 24 in its one recorded run. A chain with this tuple would
       never produce a block. P-003's report says the tuple is "not a final
       cadence-compatible P-4".
    b. A 128-bit floor is the right test for a scheme that needs nobody to solve the problem.
       A mined puzzle needs miners to solve it. Under D2 as G0-A amended it, chain weight
       comes from the hash race alone: the tuple sets how much useful work a block carries.
       It also sets the head start a faster solver gets in the hash race, which is why it has
       to be chosen from measured solve times.
    c. What stands from P-003: the checker, the derivation, the measurement harness, and the
       family q(n) = first prime above n^2, m(n) = 2 n ceil(log2 q(n)), beta_squared(n) = m(n)
       as the candidate family. The family is not ratified.
    d. P-4 and ACTIVE_SIZE_PARAM stay UNFILLED. Record in the README limitations that no SIS
       tuple is ratified and that the one proposed could not be mined.

B4 (the cadence link) COMMISSIONED: one new Phase 0 packet, the SIS solver envelope, under
    the next free packet identifier. Queue it after P-007. Lane AUTO if the five D17
    conditions hold at pickup. It measures and reports. It proposes no value for P-1, P-4,
    P-10 or ACTIVE_SIZE_PARAM.
    a. For family points from n = 16 upward, in steps the report justifies, until two
       consecutive points fail: instances come from derive_instance on distinct recorded
       seeds; a reference solver using lattice reduction stronger than LLL runs out of
       process and untrusted; every output is re-checked by check_sis; a run that reaches the
       limit is recorded as censored and never dropped. The limit is 30 minutes per run. A
       point fails when fewer than half of its runs succeed inside the limit.
    b. At least 100 seeds per point while the median solve is under 60 seconds, and at least
       20 beyond that. Report only the quantiles a sample supports.
    c. Reported per point: success rate; solve-time quantiles; the quality Q of each
       solution; peak memory; the estimator's block size and cost beside the measured cost,
       with any point where measurement beats the model by more than a factor of ten called
       out; and derive_instance plus check time on the same host (B2d).
    d. D8 keeps other languages out unless a decision admits them. This clause is that
       decision, for this use only: an external lattice-reduction tool may be run in a
       container pinned by digest for these measurements, as the estimator was. Nothing of
       the tool is linked into, or vendored beside, any cj3-* crate.
    e. I rule P-1, P-4, P-10, ACTIVE_SIZE_PARAM and the re-derived P-2 together, from that
       report. The packet blocks Phase 2. It does not block P-007 or P-101.

PART 2 - G0-C: CJ3 ENCODING v1 (resolves SI-002 and SI-003; ratifies GAP-12)

C1 Grammar.
    a. Integers are fixed width and little-endian: u16 in 2 bytes, u32 in 4, u64 in 8. This
       replaces "minimal-length integers" in Protocol Spec section 1. Zero is the all-zero
       bytes of the field's width, and no value has a second encoding.
    b. A fixed-length byte array [N] is its N bytes, with no length in front.
    c. A structure is its fields concatenated in the order C3 gives. There are no tags,
       names, padding, optional fields, version bytes or checksums. A change of layout is a
       protocol upgrade with new domain-tag bytes. Nothing in band announces a version.
    d. Two parts vary in length, both in the block: the solution bytes and the transaction
       list. Each has a u32 count in front. A decoder checks a count against its limit and
       against the bytes that remain before it allocates.
    e. An input is valid only if it is exactly the encoding of a value. It is rejected, with
       a distinct typed error per case and never a panic, when it is shorter than required,
       when bytes remain after the last field, when a count or the whole exceeds its limit,
       when the solution length is not the exact length C3 gives, or when class_id names no
       registered class. Non-minimal, duplicated, missing and unknown fields cannot be
       written in this grammar: every attempt at one is one of those rejections.
    f. There is no envelope. Framing messages on the wire belongs to Phase 3.

C2 Hash, tags and preimages.
    a. H is SHA-256. Confirmed.
    b. A domain tag is 16 bytes: the ASCII text below, padded on the right with 0x00.
           D_ADDR     CJ3/v1/addr      434a332f76312f616464720000000000
           D_TX       CJ3/v1/tx        434a332f76312f747800000000000000
           D_HDR      CJ3/v1/header    434a332f76312f686561646572000000
           D_INST     CJ3/v1/instance  434a332f76312f696e7374616e636500
           D_SOL      CJ3/v1/solution  434a332f76312f736f6c7574696f6e00
           D_BEACON   CJ3/v1/beacon    434a332f76312f626561636f6e000000
           D_GENESIS  CJ3/v1/genesis   434a332f76312f67656e657369730000
    c. A preimage is its tag, then fixed-width fields in this order, each encoded by C1.
           addr(pk)       = H(D_ADDR || pk[32])                               48 bytes
           signing digest = H(D_TX || network_id u32 || from[32] || to[32] ||
                              amount u64 || fee u64 || nonce u64)            108 bytes
           header hash    = H(D_HDR || Header)                               322 bytes
           instance_seed  = H(D_INST || network_id u32 || height u64 ||
                              parent_hash[32] || beacon_out[32])              92 bytes
           sol_commit     = H(D_SOL || class_id u16 || instance_seed[32] ||
                              SHA-256(solution bytes)[32] || miner_addr[32]) 114 bytes
           beacon input   = H(D_BEACON || parent_hash[32])                    48 bytes
       addr(pk) is the whole digest. In the signing digest, network_id is the transaction's
       own field, and the Ed25519 message is the 32-byte digest. parent_hash is the header
       hash of the parent. In sol_commit the inner hash is over the C3e solution bytes alone,
       without the block's count in front; it is the only use of H without a tag in these
       layouts.
       D_GENESIS has its bytes now; the genesis layout and preimage are ruled with the
       Phase 1 genesis packet.
    d. Under one tag every preimage has the same length and the same field positions, and
       preimages under different tags differ in their first 16 bytes. So two different
       inputs never give the same preimage.

C3 Layouts.
    a. Transaction, 188 bytes: network_id u32, from [32], to [32], amount u64, fee u64,
       nonce u64, pubkey [32], sig [64]. pubkey is the RFC 8032 public-key encoding, and sig
       is R then S.
    b. Class identifiers: SIS is 1. 0 is never assigned. This fills the identifier P-003
       left unbound.
    c. SIS size parameter, 20 bytes: n u32, m u32, q u32, beta_squared u64.
    d. Header, 306 bytes with the SIS anchor: network_id u32, height u64, parent_hash [32],
       beacon_out [32], instance_seed [32], class_id u16, size_param in the layout of the
       class that class_id names, hash_target [32], tx_root [32], state_root [32],
       sol_commit [32], timestamp u64, miner_addr [32], nonce u64.
    e. SIS solution, 2 * m bytes: the m coefficients in index order, each a 16-bit
       two's-complement integer, little-endian.
    f. Block: the header, a u32 solution length, the solution bytes, a u32 transaction
       count, the transactions. The solution length must equal 2 * m for the header's
       size_param. The whole must not exceed BLOCK_MAX_BYTES.

C4 (SI-002) RESOLVED. A coefficient is encoded as C3e says. The check rejects a coefficient
    when its square exceeds beta_squared, before it adds to the norm. That guard replaces
    "|s_i| <= s_max(P-4)" in Protocol Spec 5.2. It follows from the norm bound and changes
    no set of valid solutions. A tuple with beta_squared above 32767^2 = 1,073,676,289 is
    not admissible under encoding v1.

C5 (SI-003) RESOLVED: the convention of crates/cj3-classes/src/sis.rs is ratified as it
    stands. SHAKE-256 absorbs the 32 seed bytes and nothing else. Its output is read four
    bytes at a time, each group a little-endian u32. A group is accepted when it is below
    floor(2^32 / q) * q, and it yields its value modulo q. A rejected group is used up
    whole: the next candidate starts at the next four-byte boundary. Accepted values fill
    the matrix column by column, rows 0 to n-1 within a column, until n * m are placed.

C6 (P-8) Size limits.
    a. TX_MAX_BYTES = 188: the exact size of a transaction, so V1's bound and exact decode
       agree.
    b. SOL_MAX_BYTES = 2 * m for the active tuple. It is derived, and unfilled while P-4 is.
    c. BLOCK_MAX_BYTES = 1,048,576, PROVISIONAL. No evidence stands behind this number. It
       is a conservative round figure so that decoders have a bound, and I re-rule it with
       P-1.

C7 (GAP-12) Signatures. Ed25519 as RFC 8032 section 5.1, pure (no context, no prehash), with
    exactly this verification. Inputs: public key A, 32 bytes; signature R || S, 32 and 32
    bytes; message M, the 32-byte signing digest of C2c.
    a. Decode A and R as points by RFC 8032 section 5.1.3 exactly. A y-coordinate not below
       p = 2^255 - 19 fails. A y with no matching x fails. An x of zero with the sign bit
       set fails.
    b. Reject if A or R is one of the eight points whose order divides 8, the identity
       included.
    c. Read S as a little-endian integer. Reject unless S < L, the order of the base point.
    d. k = SHA-512(R || A || M) over the bytes as received, read little-endian and reduced
       modulo L.
    e. Accept if and only if [S]B = R + [k]A. The form multiplied through by 8 is not a
       substitute and must not be used. Batch verification is not used on a consensus path.
    f. An all-zero signature and an all-zero key are rejected by (b).

C8 Evidence that P-007 owes for Part 2.
    a. For every layout in C3 and every preimage in C2c, at least one worked vector with all
       of its bytes shown, derived twice independently: by the Rust code, and by a script
       that uses only a standard library, as bench/p003-sis/SHAKE-VECTOR.md did.
    b. For C5: the existing q = 17 fixture, now normative; one vector with a prime modulus
       above 2^31, where candidates are rejected; and one at each tuple used as a fixture.
    c. For C7: the RFC 8032 section 7.1 test vectors, and the C2SP CCTV ed25519 edge-case
       vectors pinned by commit, with the expected result for each derived from C7 clause by
       clause and recorded. Do not take expected results from the library under test.
    d. Fuzz targets, live in D6: no input makes a decoder panic; decode then encode returns
       the input; encode then decode returns the value; each length limit holds at the
       boundary, one below it and one above it.
    e. Until P-4 is ruled, no tuple is compiled in as active. Codecs take the tuple as an
       argument. Fixtures use tuples already measured in bench/p003-sis and are labeled
       non-normative.

PART 3 - G0-D: THE REMAINING G0 VALUES

D1 P-10 (TIMESTAMP_DRIFT) stays UNFILLED. No value can be evidenced while P-1 is unfilled. I
   rule P-10, and the unit of the timestamp field, together with P-1 (B4e).
D2 P-8 is ruled in C6.
D3 P-7, P-9 and P-12 are unchanged: owner values, UNFILLED (0.2).

PART 4 - G0-E: GAP-7 TO GAP-13

E1 GAP-12 is RATIFIED by C7.
E2 GAP-7, GAP-8, GAP-10, GAP-11 and GAP-13 are RATIFIED as Phase 3 requirements, as worded in
   docs/AUDIT_TRACEABILITY.md section 5. Add them to Protocol Spec section 12. Gate G3 tests
   each.
E3 GAP-9 is RATIFIED as a requirement on the packet that introduces the mempool. Gate G3
   tests it.
E4 The numbers these need (peer and dial limits, timeouts, limiter sizes, per-sender mempool
   limits) are operating configuration, not consensus parameters. The builder proposes each
   default with its reasoning in the packet report, and I review them at G3.
E5 The inventory of the original Codex report stays UNKNOWN and is recorded as unknown. It
   does not block G0.

PART 5 - G0-F HELD, AND WHAT IS CARRIED

F1 G0 stays HOLD. I rule G0-F after P-007 is merged with exact-merge-SHA D6 green.
F2 Order of work: this ruling's record (Part 6); then P-007, resumed on a new branch in the
   HUMAN lane and not merged by the builder; then my G0-F ruling; then P-101 and the solver
   envelope, in the order I approve.
F3 Carried, each to the boundary where I rule it:
   - P-1, P-4, P-10, ACTIVE_SIZE_PARAM, the re-derived P-2, the re-ruled BLOCK_MAX_BYTES and
     the timestamp unit: from the solver-envelope report. They block Phase 2.
   - How tx_root and the state commitment are built: at the P-101 FRAME. P-007 carries both
     as 32 opaque bytes.
   - The byte order in which a 32-byte value is read as a number (B8, and the weight formula
     and tie-break of section 10): at the first Phase 2 FRAME. C1a does not decide it.
   - The genesis layout and its preimage: with the Phase 1 genesis packet.
   - P-7, P-9 and P-12: with their owners (0.2).
   - P-008: still blocked on the URL.

PART 6 - THIS SESSION

1. Run the preflight as 0.1 and 0.3 amend it.
2. Record this ruling verbatim in loop/LEDGER.md as a new overlay, before any other edit.
3. On a new branch, one documentation-only change that applies it:
   - docs/PROTOCOL_SPEC.md to v0.2-DRAFT, with every amendment in Parts 1 to 4 and its
     open-TBD index brought up to date;
   - loop/reports/SPEC-ISSUES.md: SI-001, SI-002 and SI-003 RESOLVED, each pointing here;
   - docs/AUDIT_TRACEABILITY.md section 5: the status of GAP-7 to GAP-13;
   - docs/ENGINEERING_PLAN.md: the line under D1, and the Gate G0 line in section 4;
   - README.md: the status block (scope PUBLIC, PR #14 merged, this ruling) and the
     limitation in B3d;
   - loop/PACKETS.md and loop/STATE.md: brought to the truth, with P-007 ready to resume,
     the solver envelope queued, and the pickup guard on 0.3;
   - loop/reports/BATCH-LOG.md: new lines only, for the PR #14 merge and for this ruling;
   - loop/PROMPTS/AUTONOMOUS_BUILDER.md: step (d) only.
4. Change nothing under crates/, spec/, bench/, scripts/ or .github/.
5. Adversary pass: read the diff against this ruling clause by clause, and against A1 to A11
   and the Lean files. Report every place where an amended sentence says more or less than
   its clause.
6. Open the pull request, get exact-head D6 green, mark it ready, and STOP. Do not merge. I
   review and merge.
```

### Effective record of the 2026-11-22 ruling

The verbatim text above governs. This index only names what the ruling supersedes,
what it leaves unruled, and where it is applied. On any difference, the verbatim text
wins.

- **Clause names.** Other documents cite a clause of this ruling as `G0-` plus its
  label (`G0-B3`, `G0-C7e`, `G0-D1`), and Parts 0, 5 and 6 as "Part 0.1", "F2" or
  "Part 6 step 3". This keeps the ruling's clauses B1–B4 and D1–D3 apart from block
  rules B1–B12 and from decisions D1–D17 in this ledger.
- **Superseded by Part 0.1:** the words "(private) through the build" in the D9 ruling
  of 2026-08-15, and "The repository remains private" in the P-010 overlay of
  2026-08-16. The rest of D9 is not ruled here.
- **Superseded by Part 0.3:** the capacity conditions of 2026-08-15 and after, which
  tied CJ3 to Sarah's GATE-1/GATE-2 answers. D11 itself is unchanged.
- **Superseded by Part 0.4 and Parts 1 to 4:** in the G0-A overlays of 2026-08-16,
  "G0-B, G0-C, G0-D, G0-E: HOLD", "P-007/PR #14 stays draft and unmerged" and "PR #14
  stays draft".
- **Resolved:** SI-001 (clause B1), SI-002 (clause C4) and SI-003 (clause C5). Earlier
  entries that call them unresolved describe the record before this ruling. The ruling
  changes no Lean file and does not change the status of the P-005 vectors.
- **D8, one use only:** clause B4d admits an external lattice-reduction tool, run in a
  container pinned by digest, for the solver-envelope measurements. D8 is otherwise
  unchanged.
- **Not ruled:** D10 and D16 (Part 0.2), G0-F (clause F1), and every item clause F3
  carries.
- **Packet identifier:** the SIS solver envelope that clause B4 commissions is packet
  P-011, the next free identifier after P-010. It is not parameter P-11
  (`SIZE_RETARGET_WINDOW`), which G0-A struck.
- **Applied in:** `docs/PROTOCOL_SPEC.md` v0.2-DRAFT, `loop/reports/SPEC-ISSUES.md`,
  `docs/AUDIT_TRACEABILITY.md` §5, `docs/ENGINEERING_PLAN.md` (the D1 pointer and the
  Gate G0 line), `README.md`, `loop/PACKETS.md`, `loop/STATE.md`,
  `loop/reports/BATCH-LOG.md` and step (d) of `loop/PROMPTS/AUTONOMOUS_BUILDER.md`. The
  builder's report is `loop/reports/C9-g0-bcde-ruling-builder.md`.

## P-007 resume ruling overlay — PR #20 confirmed / report C9 points ruled / P-007 resumed (Al, 2026-11-22)

The following live ruling is preserved verbatim. It was received on 2026-11-22 (local
time), after Al merged PR #20. It confirms that merge, rules the eight points of report
C9 section 4.2, orders one text-only commit for stale statements, and resumes P-007 in
the HUMAN lane. Where it differs from an earlier entry in this ledger, it supersedes
that entry without rewriting it.

```text
LIVE CJ3 RULING - P-007 RESUME

Read loop/PROMPTS/AUTONOMOUS_BUILDER.md, loop/LEDGER.md, loop/STATE.md, loop/PACKETS.md and
loop/reports/C9-g0-bcde-ruling-builder.md first. Then act as the BUILDER seat. This session
resumes P-007 in the HUMAN lane. Record this ruling verbatim in loop/LEDGER.md as the first
commit of the P-007 branch. If any clause below is ambiguous, or conflicts with an axiom, the
Lean files, the 2026-11-22 Gate G0 ruling or another clause, STOP and name the clause. Do not
choose a reading.

PART 1 - PR #20

1.1 I reviewed and merged PR #20. In the preflight, verify the merge and its exact-merge-SHA
    D6 run on GitHub. A missing or red run is a STOP.
1.2 The four edits no clause named stand: the admission condition in Protocol Spec 5.1, GAP-9
    in section 12, the README rows and roadmap marker, and the name "packet P-011".

PART 2 - THE POINTS OF REPORT C9 SECTION 4.2

2.1 (C2d with C3d) C2d stands as a statement about encoding v1 with its one registered class.
    A second class is admitted only by an ADR, and that ADR must show that header preimages
    stay unambiguous.
2.2 (the transaction count) It has no limit of its own. The whole length is tested against
    BLOCK_MAX_BYTES before any field is read. After the count is read, the bytes that remain
    must equal 188 times the count. If the count needs more bytes than remain, that is the
    "shorter than required" rejection. If it needs fewer, that is the "bytes remain"
    rejection. Which typed error a doubly wrong input gets is not a consensus matter: fix
    the order of checks in the FRAME and record it.
2.3 (C8b with C8e) C8e governs the choice of a tuple that stands in for the active tuple in
    codec, solution and block tests: such a tuple is one already measured in bench/p003-sis,
    and the choice is labeled non-normative. The C8b vectors test the expansion rule itself.
    They may use any valid modulus, they are normative for C5, and they make no tuple a
    candidate for P-4. That includes the vector at each stand-in tuple: the choice of the
    tuple is non-normative, and the expansion it yields is a normative vector of C5.
2.4 (the five D17 conditions) They are the five that report C1 names: approved and unblocked;
    bounded non-semantic surface; no formal-spec content; no decision invention; no
    owned-TBD fill. Record the five once in the LEDGER, beside D17, so the prompt's
    reference resolves. Each packet's FRAME still argues them for itself.
2.5 (packet P-011) The name stands, always written "packet P-011".
2.6 (V7) V7 stays in the rule set. Under encoding v1 no wire input can fail it. P-101's
    conformance test supplies that case at the candidate level.
2.7 (the P-005 vectors) They stay symbolic and non-normative. The worked vectors P-007 owes
    are a separate artifact, and their status is part of my review of P-007.
2.8 (the overlay) Putting the ruling on the branch as its first commit was right.

PART 3 - STALE TEXT

In one commit of its own, text only:

3.1 spec/Spec/Tx.lean, spec/Spec.lean, spec/Main.lean and spec/README.md: the sentences that
    say SI-001, SI-002 or SI-003 are open, unresolved or pending, and no others. Comments and
    README prose only. No definition, theorem, vector or exported string changes. The
    NORMATIVE STATUS lines stay as they are. spec/vectors/p005-draft.json must be
    byte-identical before and after, and the report gives its digest both times.
3.2 bench/p003-sis/SHAKE-VECTOR.md: the q = 17 fixture is normative for the expansion rule.
    Keep the file's account of how the vector was derived.
3.3 docs/ENGINEERING_PLAN.md: one pointer line under the D2 paragraph that says G0-B through
    G0-E await normative text, as was done under D1. The header line on the repository home
    and the D9 row stay as seed text; the ledger governs.
3.4 README.md: the three older drifts the report lists (the repository-map row for spec/, the
    sentence that calls Lean conformance deferred, and the two links to an anchor that does
    not exist).
3.5 Left as written: the BOOTSTRAP paragraph of the builder prompt, which describes the first
    session, when the repository was private; and the audit matrix's section 1 rows and A8
    conclusion, which are P-009's verified record and are governed by the note in section 5.

The comment in crates/cj3-classes/src/lib.rs that calls the identifier unratified is corrected
with the code that binds it, not in this commit.

PART 4 - P-007

4.1 Build what loop/PACKETS.md says P-007 owes, to Protocol Spec v0.2-DRAFT sections 1.1 to
    1.3, 5.2, 6.1, 6.3 and 7, with the evidence of ruling clause C8.
4.2 cj3-types owns bytes, and cj3-classes owns meaning. The grammar, the domain tags, H,
    every preimage of section 1.2, addr(), the transaction, header and block layouts, the
    class identifier table and the 20-byte SIS size-parameter layout live in cj3-types.
    Derivation, checking and the validity rules of a tuple stay in cj3-classes, which
    imports the layouts. The SIS solution encoding lives with the class. If this split
    cannot be built without a dependency cycle or a second implementation of a layout, STOP
    and say why.
4.3 The verification of section 1.3 is part of P-007 and lives in cj3-types, beside addr()
    and the signing digest: one implementation, imported everywhere.
4.4 Dependencies. Propose each crate in the FRAME with its A9 case, its licence and its exact
    pinned version. I accept or refuse them at review. deny.toml is not loosened. No
    library's own notion of strict verification is taken as section 1.3: conformance is
    shown by the C8c vectors.
4.5 Fuzzing. The codec-fuzz-smoke job becomes live with cargo-fuzz targets, as section 5 of
    the plan names. That job alone may use a nightly toolchain pinned by date. The workspace
    and every other job stay on the pinned stable toolchain.
4.6 Vectors. The standard-library scripts live under bench/, as D8 allows. For the C2SP CCTV
    vectors, record the source commit, the file's digest and its licence. If the licence
    does not allow a copy in this repository, STOP and say so.
4.7 Nothing in this session fills P-1, P-4, P-10, ACTIVE_SIZE_PARAM or any owner value, and
    no tuple is compiled in as active (C8e).
4.8 Finish as the HUMAN lane requires: adversary pass, exact-head D6 green, pull request
    marked ready, and STOP. Do not merge. I review and merge, and then I rule G0-F.
```

### Effective record of the P-007 resume ruling

The verbatim text above governs. This index only names what the ruling confirms, what
it supersedes, and where it is applied. On any difference, the verbatim text wins.

- **Clause names.** Other documents cite a clause of this ruling as "resume" plus its
  number ("resume 2.2", "resume 4.5"). A clause of the 2026-11-22 Gate G0 ruling keeps
  its `G0-` prefix.
- **Confirmed by clause 1.1:** PR #20 merged at `2026-11-23T03:51:34Z` as
  `f601224db855e6cc69e840fb169d85df22c547b8`, and exact-merge-SHA D6 run `45108419759`
  on that commit concluded success in all eleven jobs. Both were read on GitHub before
  the branch was cut.
- **Confirmed by clause 1.2:** the admission condition in Protocol Spec §5.1, GAP-9 in
  §12, the README rows and roadmap marker, and the name "packet P-011".
- **Ruled by Part 2:** the eight points of report C9 §4.2. Clause 2.1 limits the
  statement of G0-C2d to encoding v1 with its one registered class and sets the bar for
  a second class. Clause 2.2 gives the transaction count no limit of its own and leaves
  the order of checks to the P-007 FRAME, where it is fixed (report C10 §1.5). Clause
  2.3 separates the stand-in tuples, whose choice is non-normative, from the expansion
  vectors, which are normative for the expansion rule. Clause 2.4 fixes the five D17
  conditions, recorded beside D17 in this ledger. Clause 2.6 keeps V7 in the rule set.
  Clause 2.7 keeps the P-005 vectors symbolic and non-normative and reserves the status
  of P-007's worked vectors to Al's review.
- **Superseded by Part 3:** the statements that SI-001, SI-002 or SI-003 are open in
  `spec/Spec/Tx.lean`, `spec/Spec.lean`, `spec/Main.lean` and `spec/README.md`; the
  nonnormative label in `bench/p003-sis/SHAKE-VECTOR.md`; and three older README drifts.
  Clause 3.5 leaves the BOOTSTRAP paragraph of the builder prompt and the audit matrix's
  §1 rows and A8 conclusion as written.
- **Not ruled here:** D10 and D16; G0-F, which Al rules after P-007 is merged; P-1, P-4,
  P-10, `ACTIVE_SIZE_PARAM` and every owner value (clause 4.7); the dependencies P-007
  proposes, which Al accepts or refuses at review (clause 4.4); and the status of
  P-007's worked vectors (clause 2.7).
- **Applied in:** the text-only commit of Part 3; `crates/cj3-types`,
  `crates/cj3-classes`, `bench/p007-vectors`, `fuzz`, `scripts/ci` and
  `.github/workflows` for Part 4; and `README.md`, `loop/PACKETS.md`, `loop/STATE.md`
  and `loop/reports/BATCH-LOG.md`. The builder's report is
  `loop/reports/C10-p007-builder.md`. The pull request is PR #21, which the builder does
  not merge.

## Gate G0-F ruling overlay — GATE G0 PASS / PR #21 confirmed / report C10 points ruled / Phase 1 open (Al, 2026-11-23)

The following live ruling is preserved verbatim. It was received on 2026-11-23 (local
time), after Al merged PR #21. It confirms that merge and completes P-007, rules the
points of report C10 section 4.2, passes Gate G0 with carried items, opens Phase 1, and
orders this session's gate record, text corrections and four build items. Where it
differs from an earlier entry in this ledger, it supersedes that entry without rewriting
it.

```text
LIVE CJ3 RULING - GATE G0-F

Read loop/PROMPTS/AUTONOMOUS_BUILDER.md, loop/LEDGER.md, loop/STATE.md, loop/PACKETS.md and
loop/reports/C10-p007-builder.md first. Then act as the BUILDER seat. Record this ruling
verbatim in loop/LEDGER.md as the first commit of a new branch. This session's whole scope is
Part 4. If any clause below is ambiguous, or conflicts with an axiom, the Lean files, an
earlier ruling or another clause, STOP and name the clause. Do not choose a reading.

PART 1 - PR #21

1.1 I reviewed and merged PR #21. In the preflight, verify the merge and its exact-merge-SHA
    D6 run on GitHub. A missing or red run is a STOP. P-007 is COMPLETE.

PART 2 - THE POINTS OF REPORT C10 SECTION 4.2

2.1 (the two dependencies) sha2 =0.10.9 and curve25519-dalek =4.1.3 are accepted as pinned,
    the second with the condition of 4.5c. Arithmetic written in cj3-types is refused:
    several hundred lines of new field and group code that only the vectors vouch for is
    the larger risk.
2.2 (libfuzzer-sys) Accepted for the fuzz crate alone, as test tooling outside the graph
    the node is built from, with the condition of 4.5b.
2.3 (the nightly long fuzz run) The permission of resume clause 4.5 extends to the D6
    Nightly long fuzz job: the same pinned nightly toolchain and the same targets. Make it
    live (4.5a).
2.4 (the status of the vectors) From the merge of PR #21, three files are NORMATIVE
    conformance vectors: bench/p007-vectors/encoding-v1.vectors.txt for the layouts and
    preimages of Encoding v1, bench/p007-vectors/shake-expansion.vectors.txt for the
    expansion rule, and bench/p007-vectors/ed25519/c7-expected.vectors.txt for Protocol
    Spec 1.3, the thirteen builder-authored vectors included. What is normative is the
    relation: these inputs give these bytes and these verdicts, and every implementation
    reproduces them. The input values assign nothing: no network identifier, fee, target,
    root, timestamp unit or tuple. If the text of the specification and a vector ever
    disagree, that is a STOP and a ruling. Neither wins by default. The P-005 vectors stay
    symbolic and non-normative.
2.5 (active in the block codec) The builder's reading stands. The codec takes the size
    parameter that stands for the active tuple and uses it for SOL_MAX_BYTES alone. The
    exact solution length comes from the header's own size parameter. B3 is a block rule:
    consensus applies it, and a successful decode never implies it.
2.6 (typed errors) The order of checks in report C10 section 1.5 stands. It is not a
    consensus matter.
2.7 (layout tests with arbitrary bytes) C8e and resume clause 2.3 do not reach them. A test
    of a layout over arbitrary field values names no tuple. They stay.
2.8 (edits no clause named) They stand.
2.9 (stale text left) Correct both, text only: the comment in spec/Spec/Tx.lean that calls
    the strict canonical decoder unresolved, and the README diagram label that calls the
    Lean rules and vectors planned. The conditions of resume clause 3.1 apply to the Lean
    file.
2.10 (the cj3-classes API) Accepted.
2.11 The three ways of showing the BLOCK_MAX_BYTES boundary (finding 7) are accepted.

PART 3 - G0-F

3.1 GATE G0: PASS, with the items of 3.5 carried. Phase 1 is open.
3.2 The conditions of Engineering Plan section 4 are met as follows. The reports of P-001 to
    P-007, P-009 and P-010 are in loop/reports and their work is merged. D1, D2, D14 and
    D15 were ratified on 2026-08-15 and are confirmed here on the spike evidence since, D2
    as G0-A amended it and D1 as 3.3 conditions it. D9 and D10 were ruled then, and D9 was
    superseded on 2026-11-22. P-008 stays blocked and is carried.
3.3 D1 stands on a condition. SIS remains the genesis anchor class. No tuple is ratified,
    and it is not yet measured whether the family has a size that miners can solve at a
    cost worth calling useful work. Packet P-011 measures that. If it finds no such size,
    D1 returns to me. Say so in the README limitations.
3.4 The Protocol Specification becomes v0.2 and is NORMATIVE from this ruling: its rules
    and layouts bind every implementation and change only by my ruling. Excepted are every
    value marked UNFILLED or PROVISIONAL and every carried item of 3.5. No implementation
    may assume a value for one.
3.5 Carried, each to the boundary where I rule it:
    - P-1, P-4, P-10, ACTIVE_SIZE_PARAM, the re-derived P-2, the re-measured P-3, the
      re-ruled BLOCK_MAX_BYTES and the timestamp unit: from packet P-011's report. They
      block Phase 2.
    - How tx_root and the state commitment are built: P-101 proposes both in its FRAME and
      stops there for my ruling.
    - The byte order in which a 32-byte value is read as a number: at the first Phase 2
      FRAME.
    - The genesis layout and its preimage: with the Phase 1 genesis packet.
    - P-7, P-9 and P-12: with their owners. D10 and D16 are still not ruled.
    - P-008: blocked on the URL.
3.6 P-101 and packet P-011 are both unblocked. I say which runs first in the message that
    launches it.

PART 4 - THIS SESSION

4.1 Run the preflight, with 1.1.
4.2 Record this ruling verbatim, as the first commit of a new branch.
4.3 The gate record, documentation only:
    - docs/PROTOCOL_SPEC.md: its version line, its gate-status paragraph and any other
      sentence that 3.1 or 3.4 makes false. No rule, layout or value changes.
    - docs/ENGINEERING_PLAN.md: the Gate G0 line in section 4.
    - README.md: the status block, and the limitation of 3.3.
    - loop/PACKETS.md and loop/STATE.md: P-007 COMPLETE with its merge commit and runs,
      Phase 1 open, P-101 and packet P-011 unblocked.
    - loop/reports/BATCH-LOG.md: new lines only.
    - This session's report under loop/reports carries the evidence for 3.2.
4.4 Text, in a commit of its own: the stale text of 2.9; and the status lines of the three
    vector files as 2.4 rules them, changed in derive.py and regenerated. Every vector value
    stays byte-identical, and the report shows it.
4.5 Four build items, each in a commit of its own. If one cannot be made green without
    loosening a policy, leave it out of the pull request and report it. The gate record
    does not wait for it.
    a. The D6 Nightly long fuzz job is live on the pinned nightly toolchain (2.3).
    b. cargo deny runs over the fuzz workspace with a configuration of its own: the root
       policy, plus the one licence term libfuzzer-sys needs, for that crate alone. If it
       needs any other exception, report which and why, and do not add it.
    c. One arithmetic back end. Every build of cj3-types in this repository uses
       curve25519-dalek's serial back end, and a build that does not fails to compile.
       The tests, the vectors and the fuzz targets run under it in D6. The reason is that
       every node should run the same arithmetic whatever its processor. It is not a
       claim that no unsafe code remains on the path. Correct the README limitation to
       say what is then true.
    d. Source policy. No Rust source outside crates/cj3-types names a domain tag constant,
       and the source-policy gate fails on one. The tests of cj3-types and the vector
       scripts are exempt.
4.6 Adversary pass over the whole diff. Exact-head D6 green. Pull request marked ready.
    STOP. Do not merge. I review and merge.
```
