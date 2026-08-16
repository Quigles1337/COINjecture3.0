# Cycle 5 — P-005 Builder Report

**Status:** BUILDER COMPLETE — REVIEW HANDOFF; DO NOT MERGE
**Date:** 2026-08-16
**Packet:** P-005 — Lean scaffold
**Lane:** HUMAN
**Branch:** `feat/p005-lean-scaffold`

## SI-004 MODEL 4 RULING AND SECOND RE-FRAME — 2026-08-16

Al ratified SI-004 Model 4 and explicitly resumed P-005 under all six standing
HUMAN-lane constraints. The governing formula is now
`reward = subsidy · min(Q,R_MAX·SCALE)/(R_MAX·SCALE)` using a u128 intermediate and
floor division, with `R_MAX ≥ 1`. Section §8 conserves realized reward rather than the
subsidy ceiling; unminted issuance disappears, and no funding account exists. P-7's
semantics change from inflation multiplier to quality span divisor, but its value and
all curve shaping remain owner-controlled and UNFILLED.

### Restated affected work and done-condition

Resume the existing draft formal model; amend the governance/specification trail;
replace the SI-004 placeholder with the ratified reward formula; prove in
`Spec/Stf.lean` that floor-divided reward never exceeds subsidy; update the symbolic
§14 vector surface for the threshold, cap, above-cap, `R_MAX = 1`, and realized-
issuance boundaries; generate deterministic noncanonical JSON; and admit the pinned
Lake build/exporter into the existing D6 Lean job.

The builder tail is complete only after the full Lake project and deterministic
artifact verify locally, the entire final diff passes the explicit ADVERSARY/A1–A11
sweep, the exact pushed head passes all eleven hosted D6 jobs, and PR #9 is marked
ready-for-review. PR #9 remains unmerged; Al's line-by-line review and merge are still
the packet done-condition and P-101 license.

### Lane and authority

**HUMAN — explicitly authorized.** The new ruling supplies the previously missing
reward/conservation semantics but does not fill an owned value. SI-001/2/3 remain
abstract, all Al/Sarah/G0 values remain symbolic, and no Sarah contact or attribution
is permitted before D16 reveal. `R_MAX ≥ 1` is a ratified domain constraint; no
concrete `R_MAX`, subsidy schedule, fee, codec, key, address, network, or μ-balance
choice may enter the implementation.

### Re-framed diff surface

- Governance/specification: `loop/LEDGER.md`, `docs/PROTOCOL_SPEC.md`, the conflicting
  R7 sentence in `docs/ENGINEERING_PLAN.md`, `loop/reports/SPEC-ISSUES.md`, and P-005
  state/packet/report bookkeeping. The Engineering Plan edit is consistency-only and
  carries no new semantic choice beyond Model 4.
- Formal model: `spec/Spec/Stf.lean`, the existing §7 model only where mechanical
  proof support requires it, `spec/Spec/Vectors.lean`, aggregate/executable roots,
  generated symbolic JSON, and `spec/README.md`.
- Verification: the existing active-handler convention and the minimum workflow
  setup necessary for the `lean-conformance` D6 job to compile the pinned project,
  regenerate the artifact, and fail on drift. No Rust conformance consumer enters
  before P-101.
- Evidence: this report, `loop/STATE.md`, `loop/PACKETS.md`, and
  `loop/reports/BATCH-LOG.md` at the final packet boundary.

### Risks, falsifier, and confidence

1. **Vacuous ceiling proof:** proving a generic division lemma while the STF still
   consumes an unrelated abstract reward would not make the issuance ceiling real.
   Control: the STF must derive its credited amount and conservation target from the
   same Model 4 function whose theorem is proved.
2. **Owned-value laundering:** using `R_MAX = 1`, a sample subsidy, or μ-normalization
   as executable fixture data could silently fill P-7/P-12. Control: all cases remain
   symbolic expressions and explicitly non-normative; `R_MAX = 1` appears only as a
   quantified semantic boundary.
3. **Arithmetic mismatch:** Nat proofs can hide u128/u64 conversion behavior. Control:
   model nonnegative mathematical arithmetic, prove the subsidy ceiling, prove the
   result fits u64 because subsidy is u64, and keep actual Rust/u128 conformance for
   P-101.

**Falsifier:** the approach is wrong if the credited STF reward is not definitionally
the proved floor-division function, if the upper-bound proof imports `reward ≤ subsidy`
as a premise, if any vector contains canonical bytes or an owned value, or if generated
JSON can drift from the Lean source without making CI red.

**Confidence:** MED. The ratified formula removes the economics ambiguity and its
upper bound is mathematically direct, but the Lean library lemma names, alias-sensitive
STF review, deterministic exporter, and hosted toolchain path remain to be exercised.

## RESUME RULING AND RE-FRAME — 2026-08-16

This section supersedes the earlier STOP disposition without erasing it. Al's live
`P-005 HUMAN-LANE AUTHORIZATION — RATIFIED, with review tail` ruling is preserved in
`loop/LEDGER.md` on `main` at
`8d9dbbfef8552e80c1e7e2e13c0d6815cded523e`. Its exact-merge-SHA D6 run
`31958836168` passed all eleven jobs. That higher-authority ruling explicitly
authorizes the builder to implement the HUMAN-lane Lean/vector content, subject to
its symbolic/TBD/SI boundary and non-automatic merge rule.

### Restated packet and done-condition

P-005 will add a pinned Lake project, a draft Lean model of Protocol Spec §7 V1–V9,
a draft §8 STF model and conservation theorem target, and a deterministic
`lake exe vectors` exporter covering the §14 case list. The formal model is
parameterized over unresolved byte-codec, cryptographic, address-derivation,
economics, reward, and block-prevalidation interfaces; it does not instantiate an
Al-owned, Sarah-owned, G0-controlled, or SI-controlled value.

The builder portion is done only when the complete diff has passed local mechanical
checks, an explicit adversary/A1–A11 pass, and all eleven hosted D6 jobs at the exact
PR #9 head. PR #9 is then marked ready-for-review and left unmerged. Al's line-by-line
review and merge—not a builder action—is the packet's ultimate done-condition and the
necessary P-101 license alongside Gate G0.

### Lane classification against D17

**Classification remains HUMAN, now explicitly authorized.** The ruling does not
convert formal semantics into AUTO work; it grants the builder authority for this
specific HUMAN surface and dictates its review tail. The five merge conditions are
therefore not used to auto-merge PR #9. They remain mandatory safety checks before the
PR is handed to Al.

- Queue authority and D11: PASS. P-005 is authorized, repository visibility remains
  PRIVATE, and the current CJ2 operational state still reports `CAPACITY_FLAG: none`
  with GATE-1/GATE-2 awaiting.
- Semantic surface: HUMAN by definition. Scope is bounded to Protocol Spec §§7–8 and
  §14 plus the minimum project/CI/bookkeeping machinery needed to build and verify it.
- Formal/vector content: HUMAN by definition and covered by the live authorization.
- Ratification/invention: no new ratification is permitted. Interfaces remain
  abstract wherever prose, a TBD, G0, or SI-001/2/3 withholds a concrete choice.
- Owned values: no Al-owned or Sarah-owned value is instantiated. Any executable
  fixture is labeled `NON-NORMATIVE TEST FIXTURE` in both Lean and emitted JSON.

### Known symbolic boundary (not new semantic rulings)

- V1 canonical decode and `input_bytes` production remain behind an abstract codec
  interface. Draft vectors use explicit symbolic input-byte expressions rather than
  selecting a wire format. This is the pre-existing P-005 unknown recorded in the
  original FRAME and is also guarded against the byte-level invention pattern in
  SI-002/SI-003; those SIS-specific issues themselves remain wholly unresolved.
- V2 verification, its signing preimage construction, V3 `addr(pubkey)`, and any
  cryptographic byte operation are abstract interfaces. No hash/codec layout is
  chosen through the formal model.
- `TX_MAX_BYTES`, `FEE_MIN`, the block-prevalidation result, miner address, subsidy,
  and reward application are inputs or abstract operations. No P-8/P-9/P-12 or other
  G0/owned value is supplied.
- SI-001's hardness-claims conflict and SI-002/SI-003's SIS encoding issues do not
  enter the transaction/STF model and remain OPEN. P-005 makes no SIS claim or vector.

If implementation reveals a semantic ambiguity beyond these already identified and
explicitly abstracted boundaries, the build stops immediately and records a new
issue rather than choosing a behavior.

### Predicted diff surface

- `spec/lean-toolchain`, `spec/lakefile.toml`, and the minimal Lake module/executable
  roots.
- `spec/Spec/Tx.lean`, `spec/Spec/Stf.lean`, and at most one `Spec/*.lean` vector-case
  module. Every `Spec/*.lean` file will carry the exact required draft/ownership
  header.
- A committed deterministic draft JSON vector artifact beneath `spec/vectors/`, with
  each `input_bytes` value explicitly symbolic and every executable fixture labeled
  non-normative.
- `scripts/ci/active/lean-conformance.ps1` plus only the minimal pinned workflow
  setup needed to make the existing `lean-conformance` D6 job run the Lake build and
  exporter rather than report `NOT_YET_ADMITTED`.
- This report and the packet boundary files in `loop/`.

No Rust crate, consensus implementation, canonical codec, dependency lockfile,
protocol parameter, Protocol Spec text, Engineering Plan text, or P-101 conformance
consumer is predicted or authorized.

### Top risks, falsifier, and confidence

1. **Abstraction leak:** a convenience mock could become a de facto canonical codec,
   signature, address, fee, or reward choice. Control: interfaces are named as
   unresolved and exported bytes stay symbolic; fixtures carry non-normative labels.
2. **STF aliasing/atomicity error:** self-send or miner/source/destination aliasing
   could accidentally mint/burn value or expose partial state. Control: apply the §8
   operations in the stated order to a candidate state and return the original state
   on any failure; adversary vectors cover aliasing and late-block rollback.
3. **False vector-conformance claim:** green Lean compilation could be mistaken for
   ratified wire vectors or Rust conformance. Control: D6 names this a P-005 draft
   model/export check; P-101 consumption remains absent and blocked.

**Falsifier:** this approach is wrong if any emitted byte string could be consumed as
the canonical transaction encoding, any fixture fixes an owned/G0/SI value, any V-rule
or §8 operation cannot be traced directly to the ratified prose, or atomic failure can
return a mutated state. Any such observation triggers a fresh HUMAN stop.

**Confidence:** MED. The abstract-interface design directly fits the live ruling, but
the self-send/aliasing and atomic rollback proof surface deserves adversarial testing,
and hosted Lean toolchain admission has not yet been exercised in this repository.

## RESUMED BUILD STOP — SI-004

The resumed build reached a new semantic ambiguity and stopped before a vector
artifact, CI admission, adversary completion, or ready-for-review transition.

### Tripwire

Protocol Spec §8 directs the STF to credit §11's quality-scaled `reward(height, Q)`
but defines the conservation target as total post-state balances equaling total
pre-state balances plus exactly `subsidy(height)`. Section §11 allows the credited
reward to exceed the subsidy when `Q > SCALE` and `R_MAX > 1`; fees are transfers and
no quality-premium funding debit exists in the state transition. The two requirements
therefore cannot both hold generally without a new economics decision.

P-005 did not choose among equating issuance with reward, constraining an owner value,
or adding a funding source. The conflict is now `SI-004` in
`loop/reports/SPEC-ISSUES.md`. It requires an Al-owned HUMAN ruling and corresponding
§8/§11 amendment before the conservation target can be encoded coherently.

### Concurrency incident

During the first full Lake compile, a second already-running Codex task wrote
`spec/Spec/Tx.lean`, `spec/Spec/Stf.lean`, and `spec/README.md` into the same canonical
checkout. The namespace/content changed between two reads. This session treated that
as an environment/scope tripwire, identified the peer through the local task list,
and instructed it to stop all writes. The peer stopped without committing, pushing,
changing PR #9, starting CI, or advancing the queue. Its candidate files are retained
in this checkpoint for review; compilation alone does not confer semantic authority.

### Partial checkpoint surface

- `spec/lean-toolchain` pins Lean 4.33.0; the local environment reports Lean 4.33.0
  and Lake 5.0.0.
- Candidate `Spec/Tx.lean` and `Spec/Stf.lean` files carry the exact required header
  and keep codec, signature, address, parameter, block, reward, and storage choices
  behind interfaces. `lake build Spec.Tx Spec.Stf` passed locally on the pinned
  toolchain after the concurrency stop. They remain partial HUMAN work, not ratified
  semantics or a full-project build.
- A draft symbolic `Spec/Vectors.lean` manifest and executable entrypoint were started
  but the full project does not build. No committed JSON vector artifact exists.
- The existing D6 `lean-conformance` job remains the explicit
  `NOT_YET_ADMITTED` path. No workflow or active-handler change was made, so a green
  branch D6 must not be described as compilation or conformance of these files.
- No Rust source, protocol value, canonical byte encoding, signature implementation,
  address derivation, Protocol Spec text, or P-101 consumer changed.

PR #9 therefore remains draft and MUST NOT be marked ready or merged. The out-of-order
review-tail queue exception has not begun because P-005 did not reach its completed
ready-for-review tail.

## 1. FRAME

### Packet and done-condition

P-005 must create the Lean 4/Lake formal-specification project, encode Protocol Spec
§7's V1–V9 transaction-validity rules in `Spec/Tx.lean`, establish the
`Spec/Stf.lean` state-transition skeleton, and define a JSON vector exporter exposed
as `lake exe vectors`. The vectors must cover the normative rule boundaries that the
Phase 1 Rust kernel will consume in P-101. The Rust conformance consumer is expressly
outside this packet.

The packet is done only when:

- a reproducible, pinned Lake project builds without reaching outside its declared
  dependency/toolchain boundary;
- `Spec/Tx.lean` gives a human-owned normative encoding of V1–V9 rather than a
  builder-invented approximation;
- `Spec/Stf.lean` states the §8 transition skeleton and conservation target without
  silently resolving an Al-/Sarah-owned value or an open G0 interpretation issue;
- `lake exe vectors` emits deterministic, canonical JSON cases whose inputs and
  expected outcomes are explicitly human-owned protocol definitions;
- no Rust state-machine implementation lands before that formal specification, per
  A8/D5;
- full verification and adversary review pass after the HUMAN-lane content has been
  supplied or explicitly authorized under a new live ruling.

P-005 cannot satisfy that done-condition through a structural Lake shell alone. The
packet definition expressly requires both `Spec/*.lean` content and vector
definitions, and D17 names both surfaces as HUMAN lane.

### Lane classification against the five D17 AUTO conditions

**Classification: HUMAN — mandatory STOP before implementation.**

1. **Approved and unblocked:** PASS as a queue condition. P-005 is `NEXT — APPROVED`;
   P-004 and the governance/source-ingest boundary are merged at
   `1f379308ddba03d41774cf9d35ea6b27c5795fb0`, with all eleven jobs successful in
   exact-merge-SHA D6 run 31955437332. A fresh 2.0 state read still reports
   `CAPACITY_FLAG: none`, while GATE-1/GATE-2 retain awaiting owners.
2. **Bounded non-semantic surface:** FAIL for AUTO. P-005's required output is the
   normative transaction-validity and STF surface. Even a faithful-looking encoding
   can choose semantics for canonical decoding, signatures, address binding,
   checked arithmetic, nonce/state lookup, and atomic application. The SCOPE rule
   says a packet that reaches V-rules or STF is HUMAN immediately.
3. **No formal-spec or vector content:** FAIL explicitly. The packet requires
   `Spec/Tx.lean`, `Spec/Stf.lean`, and JSON protocol-vector definitions. D17 lists
   `Spec/*.lean` content and vector definitions as HUMAN-lane work by name.
4. **No decision invention or ratification:** NOT ESTABLISHED for autonomous build.
   Transcribing prose into executable predicates and expected vector outcomes can
   resolve ambiguities or make a de facto normative choice. The builder has no
   authority to ratify those choices. AMEND-3 strengthens this ordering; it does not
   convert formal semantics into AUTO work.
5. **No Al-/Sarah-owned TBD fill:** PASS only under a symbolic, fail-closed design,
   and therefore insufficient to restore AUTO status. `FEE_MIN`, economics values,
   ingress defaults, cryptographic details awaiting G0 confirmation, and every
   Sarah-owned item must remain parameters or unresolved. Any concrete vector that
   depends on one of those values needs a human-supplied value or boundary case.

Because conditions 2 and 3 fail by the packet's own definition, no decomposition
into an AUTO “scaffold first” subpacket is inferred. Doing so would silently rewrite
the approved packet and could make an empty shell look like progress on A8.

### Predicted diff surface if HUMAN authority resumes P-005

- A pinned Lean/Lake project beneath `spec/`, retaining `spec/README.md` as the
  boundary statement or updating it only to describe actually supplied formal work.
- Human-owned files beneath `spec/Spec/`, including `Spec/Tx.lean` and
  `Spec/Stf.lean`, plus only the minimal module/entrypoint files required by Lake.
- A deterministic JSON vector-export executable and human-owned vector definitions
  beneath the same `spec/` project.
- The existing Lean phase-gate script and D6 workflow only if necessary to replace
  the current explicit `NOT_YET_ADMITTED` deferral with execution of the pinned
  project; no Rust conformance consumer before P-101.
- This report plus packet-boundary bookkeeping in `loop/STATE.md`,
  `loop/PACKETS.md`, and `loop/reports/BATCH-LOG.md`.

Before new HUMAN authority, the predicted and authorized diff surface is only this
FRAME/STOP report and the required loop checkpoint. No file beneath `spec/Spec/`, no
vector, no CI admission change, and no consensus/runtime source is authorized.

### Authority-and-claims pre-check

- Protocol Spec §14 says Lean becomes normative once ratified; an executable file is
  not self-ratifying merely because it mirrors prose or compiles.
- D16 keeps Sarah uninvolved until reveal at G1. P-005 is designed as her entry seam,
  but this stop does not authorize an invitation, message, GitHub notification, or
  Sarah-owned choice before reveal.
- The current `spec/README.md` explicitly assigns the Lean project and all
  `Spec/*.lean` content to P-005/HUMAN and says no formal rule or vector is defined
  yet. The builder will not contradict that boundary with an autonomous shell.
- Open G0/HUMAN issues SI-001 through SI-003 remain unresolved. P-005 must not use a
  formalization or vector fixture to smuggle in a concrete hardness claim, SIS
  coefficient encoding, or SHAKE candidate-byte convention.
- P-009 was appended after the earlier approved packets. Its evidence sources are
  durably committed but remain unopened/uninterpreted; the P-005 HUMAN stop occurs
  before P-009's analysis turn in the ratified top-down queue.

### Top risks

1. **Executable-spec invention:** a mechanically plausible Lean predicate can choose
   behavior the prose leaves ambiguous and thereby become an unauthorized normative
   ruling once downstream Rust consumes its vectors.
2. **TBD laundering through fixtures:** concrete JSON examples can freeze
   `FEE_MIN`, encoding widths, cryptographic conventions, or other G0/owned values
   without an obvious parameter edit. Expected outputs are protocol decisions, not
   harmless test data.
3. **Premature Sarah disclosure or false review claim:** treating “Sarah entry seam”
   as permission to invite or attribute review would violate D10/D16. Until reveal,
   the report must say HUMAN review is absent, not imply it occurred.

**Falsifier:** this lane classification would be wrong only if P-005 could meet its
approved done-condition without creating `Spec/*.lean` content, defining protocol
vectors, selecting consensus semantics, or relying on human-owned decisions. The
packet definition and D17 state the opposite explicitly. Therefore the falsifier is
already mechanically false, and implementation must not start without a higher-
authority live instruction that supplies or authorizes the HUMAN content.

**Confidence:** HIGH. The packet text, A8/D5 ordering, `spec/README.md`, and D17 all
independently place the required deliverables in the HUMAN lane.

## Initial evidence ledger

### VERIFIED

- Canonical checkout was exactly `C:\Users\LEET\COINjecture3.0`; the deleted decoy
  path tested `False`, and neither path contains `OneDrive`. Evidence: 2026-08-16
  pickup PowerShell readback.
- Local `main`, `origin/main`, and `HEAD` matched
  `1f379308ddba03d41774cf9d35ea6b27c5795fb0`; the worktree was clean before this
  branch was created. Evidence: pickup `git rev-parse` and porcelain-status readback.
- Origin fetch and push URLs both exactly matched
  `https://github.com/Quigles1337/COINjecture3.0`; GitHub reported `PRIVATE`, with
  `main` as default. Evidence: pickup `git remote -v` and `gh repo view` readback.
- The governance/source-ingest boundary merge SHA above passed all eleven hosted D6
  jobs. Evidence:
  https://github.com/Quigles1337/COINjecture3.0/actions/runs/31955437332
- D17 is RATIFIED; P-005 is the next approved packet; P-008 remains blocked on the
  exact Al-supplied frontend URL; P-009 is appended after the earlier approved
  packets. Evidence: `loop/LEDGER.md` and `loop/PACKETS.md` readback.
- D11 remains clear for CJ3 at pickup: the operational 2.0 loop file reports
  `CAPACITY_FLAG: none`, and GATE-1/GATE-2 still list awaiting owners. Evidence:
  read-only `C:\Users\LEET\COINjecture2.0-network\loop\STATE.md` readback.
- `docs/RESEARCH_SURVEY.md` is present, and the P-009 source manifest/evidence files
  are already durable on `main`. Evidence: filesystem and Git-tree readback.
- P-005 requires `Spec/Tx.lean`, `Spec/Stf.lean`, and `lake exe vectors`; D17 makes
  `Spec/*.lean` content and vector definitions HUMAN lane. Evidence:
  `loop/PACKETS.md`, `docs/ENGINEERING_PLAN.md` §9, and
  `loop/PROMPTS/AUTONOMOUS_BUILDER.md` lines 54–58.

### ASSUMED

- `C:\Users\LEET\COINjecture2.0-network\loop\STATE.md` remains the operational local
  capacity-state source. This is safe for pickup because Al's live 2026-08-16 D11
  ruling independently specifies the same branch condition and the file still shows
  neither gate answered nor `remediation-priority` asserted.
- The HUMAN lane can be resumed by a new live instruction from Al that explicitly
  authorizes the Lean/vector work or supplies human-ratified content. This follows
  the recorded authority chain; the exact form of the ruling is not inferred here.

### UNKNOWN

- The exact human-approved Lean representation of cryptographic verification,
  canonical byte decoding, account-state lookup, checked arithmetic, and atomic STF
  application. Resolution: Al supplies or explicitly ratifies the P-005 formal
  semantics before implementation.
- Which concrete vector cases may instantiate G0- or owner-controlled parameters
  while keeping all TBDs unfilled. Resolution: human-owned vector definitions or an
  explicit ruling that identifies the permitted symbolic/concrete boundary.
- Whether Sarah will review P-005 at G1 or later. Resolution: D16 reveal; no outreach
  or pre-reveal attribution is authorized.

## 2. BUILD — NOT STARTED

The HUMAN-lane tripwire fired during FRAME, before any Lean project, formal predicate,
vector definition, dependency, CI gate, or runtime source was created or edited.

## 3. VERIFY — STOP CHECKPOINT

- FRAME-first commit: `89bd450afa704aad69b70dd8ea6f3bea192faac0`.
- Draft pull request: [#9 — P-005 HUMAN-lane stop](https://github.com/Quigles1337/COINjecture3.0/pull/9).
- Stop-checkpoint content commit:
  `1268f3acd74af0208b3dc033be27c3071150c03c`.
- Exact checkpoint-content D6:
  [run 31955875423](https://github.com/Quigles1337/COINjecture3.0/actions/runs/31955875423),
  success on `1268f3acd74af0208b3dc033be27c3071150c03c`; all eleven jobs succeeded.
- This evidence-only update records that immutable run and changes no lane,
  implementation, or protocol claim. Its own exact-head check rollup will be attached
  to draft PR #9 rather than recursively creating another commit to embed its run ID.
- No Lean toolchain is invoked and the existing phase gate remains an explicit
  `NOT_YET_ADMITTED` deferral. This is intentionally not evidence that P-005 formal
  content exists or passes.

## 4. ADVERSARY PASS — STOP-SURFACE REVIEW

**Seat switch: ADVERSARY.** The authorized stop surface was checked for any
implicit normative choice, owned-TBD value, false completion claim, pre-reveal Sarah
notification, or out-of-order P-009 analysis. No implementation diff exists to
attack.

### Findings and dispositions

1. **No formal-content smuggling:** no file beneath `spec/` changed, and no example
   predicate, pseudocode, fixture, expected result, or dependency pin was added.
2. **No TBD laundering:** the checkpoint names unresolved values only as unresolved
   parameters; it supplies no amount, fee, encoding width, crypto convention, reward
   value, or Sarah-owned choice.
3. **No pre-reveal disclosure:** no reviewer was requested, and no external fork,
   issue, comment, star, watch, or message was created. Draft PR #9 is in the already
   verified private personal repository and attributes no Sarah review.
4. **No queue inversion:** neither preserved audit source was opened or interpreted.
   P-009 remains after P-006/P-007, exactly as the appended queue says.

### Axiom sweep

| axiom | stop-surface result |
|---|---|
| A1–A4 | PASS by non-reachability: no instance, checker, score, eligibility, or fork-choice behavior changed. |
| A5–A7 | PASS by non-reachability: no amount, arithmetic, apply path, endpoint, or configuration changed. |
| A8 | PASS: formal/state-machine ordering is preserved by stopping before both formal content and Rust implementation. |
| A9 | PASS: no dependency, executable, FFI, or trusted-computing-base surface changed. |
| A10 | PASS: the packet is reported as unimplemented/HUMAN, not scaffolded, reviewed, or green. |
| A11 | PASS at checkpoint scope: authority, base SHA/run, preflight readbacks, FRAME commit, branch, draft PR, and exact checkpoint-content D6 are explicit evidence pointers. The final evidence-only head is verified through the PR check rollup rather than inferred. |

No Critical exists on the docs-only stop surface. P-005 remains ineligible for merge
because HUMAN-lane authorization/content—not a defect fix—is absent.

## 5. MERGE — PROHIBITED

D17 permits merge only for AUTO-lane packets. This HUMAN-lane branch must remain
unmerged until a higher-authority ruling resumes P-005 and the resulting formal
content completes the full six-move protocol.

## 6. CALIBRATE

### Predictions versus outcomes

- **Diff surface:** the FRAME prediction held. Only the report and loop checkpoint
  changed; no `spec/`, CI, dependency, Rust, protocol, or engineering-plan surface
  was touched.
- **Risk materialization:** none of the three implementation risks was allowed to
  materialize. The lane gate prevented executable-spec invention, TBD laundering,
  and premature Sarah attribution before a formal artifact existed.
- **Confidence:** HIGH was calibrated correctly. Four independent authorities—the
  packet definition, A8/D5, `spec/README.md`, and D17—agree on the stop.
- **Surprise:** loading the DOCX/PDF review skills for the later P-009 packet before
  re-reading the queue exposed a sequencing hazard. The sources themselves were not
  opened; the authoritative `execute after earlier approved packets` row corrected
  course before any analysis.

### Final VERIFIED / ASSUMED / UNKNOWN ledger

**VERIFIED**

- P-005 is approved but HUMAN, and draft PR #9 contains no Lean/vector content.
- The exact base-main boundary passed all eleven D6 jobs in run 31955437332.
- The stop-checkpoint content commit passed all eleven hosted D6 jobs in run
  31955875423. The follow-up commit changes evidence pointers only; its exact-head
  rollup is recorded on draft PR #9.

**ASSUMED**

- Al can resume the packet through a higher-authority live instruction that supplies
  or explicitly authorizes the HUMAN semantic surface. This follows the recorded
  authority chain but does not presume the content of that instruction.

**UNKNOWN**

- The human-approved Lean representation and vector set remain unknown by design.
- The symbolic/concrete boundary for vectors involving owner-controlled or G0 values
  remains unknown until Al rules; no value has been inferred.

**One process improvement for the next pickup:** complete top-down queue and D17 lane
classification before loading packet-specific artifact sources or tooling. This makes
an adjacent, already-staged packet less likely to pull work across an ordering gate.

## 7. RESUME CALIBRATION

### Predictions versus outcomes

- **Diff surface:** the predicted `spec/` surface began to materialize. Workflow and
  active-gate edits were correctly withheld because the full project never reached a
  verified build. `SPEC-ISSUES.md` was an unpredicted but tripwire-required expansion.
- **Risk materialization:** the predicted conservation/atomicity risk exposed SI-004
  before a theorem target could encode one economics interpretation. A separate
  concurrency hazard also materialized when a peer task wrote the same files.
- **Confidence:** MED was appropriately cautious. Abstract interfaces contained the
  known codec/TBD surface, but they could not remove the newly discovered internal
  §8/§11 issuance conflict.
- **Surprise:** the protocol's state transition and its own conservation equation use
  different issuance quantities. Compilation pressure did not reveal this; a direct
  equation-level reread did.

### Final VERIFIED / ASSUMED / UNKNOWN ledger after resumed stop

**VERIFIED**

- Al's P-005 HUMAN authorization is merged on `main` at
  `8d9dbbfef8552e80c1e7e2e13c0d6815cded523e`; all eleven jobs passed in exact-main
  run `31958836168`.
- Protocol Spec §8 names `subsidy(height)` in the conservation equation, while §8
  applies §11 reward and §11 defines a potentially quality-scaled amount. Evidence:
  `docs/PROTOCOL_SPEC.md` §§8 and 11 and `loop/reports/SPEC-ISSUES.md` SI-004.
- The peer task stopped without a commit, push, PR mutation, hosted run, or later-
  packet pickup. Evidence: local task completion readback recorded during this turn.
- No active Lean gate or workflow edit exists in the checkpoint diff; P-101 remains
  blocked on both G0 and merged, human-reviewed P-005.
- The exact required normative-status sentence appears in all three current
  `Spec/*.lean` files, and a source scan found zero `sorry`, `admit`, or `axiom`
  placeholders. Evidence: local checkpoint header/placeholder scan and the committed
  candidate files.

**ASSUMED**

- None for the stop decision. SI-004 follows algebraically from the two governing
  equations unless an unstated debit or equality constraint exists; inventing either
  is exactly what the tripwire forbids.

**UNKNOWN**

- Whether quality premium is intended to mint additional issuance, debit an explicit
  pool, or be constrained away. Resolution: Al's HUMAN ruling and an amended §8/§11.
- The final approved conservation theorem statement and corresponding vector outcomes.
- Whether the retained partial Lean candidate will survive line-by-line human review
  after SI-004 is resolved. No conformance or normative claim is made now.

**One process improvement for the next resume:** obtain an exclusive-write readback
for the canonical branch before the first formal edit, then translate every prose
state update and invariant into side-by-side equations during FRAME. That would expose
both shared-worktree collision and issuance-identity conflicts earlier.

## 8. MODEL 4 BUILD, VERIFY, AND ADVERSARY CLOSEOUT

This section supersedes the resumed-stop execution status without erasing the earlier
tripwire history. The SI-004 governance checkpoint is commit
`316584a42501f7ed64135490e1a1a070a7a863eb`; the implementation checkpoint is commit
`388ede835e8c9e668f5f0126918193ff59f589cd`. The complete implementation/report head
`f494a605825a2d2dcd15d8babb71193abccae18f` passed hosted D6 run `31963016862` before
this evidence-only closeout.

### 2. BUILD — COMPLETE

- `Spec/Tx.lean` defines the single draft V1–V9 path. Strict canonical decoding,
  signature/preimage construction, address derivation, `TX_MAX_BYTES`, `FEE_MIN`, and
  the chain network identifier remain abstract interfaces. Checked addition and
  subtraction return `Option UInt64`; successful conversion uses `UInt64.ofNatLT`
  with a proof rather than an implicit modular conversion.
- `Spec/Stf.lean` revalidates each transaction against the evolving candidate state,
  performs every debit/credit/nonce/reward operation with checked arithmetic,
  validates the post-state root, and exposes only an atomic wrapper that returns the
  exact pre-state on any error. Concrete account read/write coherence remains an
  explicit P-101 instantiation obligation rather than a property invented for the
  abstract store interface.
- The same STF file defines SI-004 Model 4 as
  `subsidy * min(Q, R_MAX*SCALE) / (R_MAX*SCALE)` over mathematical naturals, with
  symbolic `R_MAX ≥ 1`, u64 subsidy/Q inputs, and no selected P-7/P-12 value. It proves:
  denominator positivity; the numerator fits u128; `reward ≤ subsidy` from the
  `min`/floor-division formula; the valid-Q lower bound; exact threshold, cap, and
  `R_MAX = 1` behavior; the complete range; and the checked result's u64 fit. The STF
  credit and conservation target consume this exact derived function, not an unrelated
  interface reward.
- `Spec/Vectors.lean` exports 27 unique §14/Model 4 cases. Every `input_bytes` value is
  an object containing a symbolic expression and interface reference, carries
  `normative: false`, and has the exact status `NON-NORMATIVE TEST FIXTURE — ABSTRACT
  INTERFACE, NOT CANONICAL BYTES`. The `R_MAX = 1` case is a quantified ratified
  boundary, not a chosen parameter value.
- `lake exe vectors` deterministically generated
  `spec/vectors/p005-draft.json`: 10,699 bytes, SHA-256
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`.
- The active `lean-conformance` handler builds the complete pinned Lean 4.33.0 project,
  regenerates and hash-compares the artifact, checks required cases/schema/status,
  enforces the exact D16 header on every `Spec/*.lean`, rejects
  `sorry`/`admit`/`axiom`, and prints
  `P101_RUST_CONFORMANCE=NOT_YET_ADMITTED`. The workflow installs Lean through the
  commit-pinned official action with automatic build/test/lint/cache behavior disabled.
- No Rust source, canonical codec, cryptographic implementation, state-store
  implementation, production dependency, owner-controlled value, Sarah attribution,
  reviewer request, or merge behavior changed.

### 3. VERIFY — LOCAL AND HOSTED COMPLETE

Local evidence from the canonical checkout at implementation commit `388ede8` plus the
durable report-only closeout working tree:

- `lake build`: PASS, complete project, 10 jobs.
- Active Lean gate: PASS, Lean 4.33.0, 27 vectors, regenerated SHA-256 exactly
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  PASS.
- `cargo deny --locked --all-features check`: PASS for advisories, bans, licenses,
  and sources.
- `cargo audit --deny warnings`: PASS against 1,216 loaded advisories and the 22-crate
  lockfile dependency set.
- `scripts/ci/verify-geiger.ps1`: PASS across 11 packages, zero unsafe, with
  `forbid(unsafe_code)` present.
- `cargo test --workspace --all-targets --all-features --locked`: PASS, 18 tests,
  zero failures.
- Phase gates: `lean-conformance` ACTIVE/PASS; `conservation-invariant`,
  `codec-fuzz-smoke`, and `genesis-spend-test` explicitly `NOT_YET_ADMITTED` under
  their recorded later owners. This is not a claim that the three deferred tests ran.
- `cargo build --workspace --all-targets --all-features --locked`: PASS.
- Static policy sweep: 14 Rust source files; zero banned identifiers, zero `unsafe`,
  zero `f32`/`f64`; all three `Spec/*.lean` files carry the required header and contain
  zero proof placeholders; all 27 vector names are unique and non-normative.

Hosted D6 run
[31963016862](https://github.com/Quigles1337/COINjecture3.0/actions/runs/31963016862)
completed successfully on exact SHA
`f494a605825a2d2dcd15d8babb71193abccae18f`; all eleven jobs passed. In particular,
`lean-conformance` installed the pinned Lean toolchain and executed the active handler,
while the three later phase gates remained explicit deferrals. This evidence-only
closeout changes loop state, the batch line, and run pointers only. Its own exact-head
rollup is attached to PR #9 before the ready-for-review transition rather than creating
an infinite series of commits that each tries to embed its own future run ID.

### 4. ADVERSARY PASS — COMPLETE

**Seat switch: ADVERSARY.** The exact `origin/main..388ede8` range and its supporting
interfaces were re-read for consensus bypass, arithmetic mismatch, hidden assumptions,
TBD laundering, canonical-byte leakage, CI greenwashing, toolchain drift, command/path
injection, external notification, and merge automation.

#### Findings and dispositions

1. **FIXED — modular-conversion ambiguity.** The partial Tx draft originally used
   `UInt64.ofNat` after a bound check. Even though the check made the value small, the
   constructor's modular semantics weakened the audit trail. Both checked operations
   now use `UInt64.ofNatLT` with explicit proofs.
2. **FIXED — full-project build failures.** The partial checkpoint placed imports after
   a module documentation block and used inconsistent namespaces. The aggregate
   project—not merely isolated files—now builds cleanly.
3. **FIXED — ambient Lean toolchain acceptance.** The first active-handler draft ran
   `lean --version` from repository root, where an ambient default could mask the
   package pin. The check now runs while `spec/lean-toolchain` is in scope before any
   Lake command.
4. **CONTAINED — abstract storage laws.** `StateOps.setAccount`, account lookup, total
   balances, and root computation are deliberately abstract and therefore do not prove
   concrete read/write coherence or the conservation target. The model sequences every
   read over the candidate state and proves atomic rejection; P-101 must instantiate
   and prove the store laws. The source and security report state this limit explicitly.
5. **CONTAINED — concrete `R_MAX` representation.** P-005 models the symbolic divisor
   as a natural, proves the capped numerator fits u128, and proves the result fits u64.
   It does not select a wire/storage type or owned value for `R_MAX`. Checked concrete
   computation of `R_MAX*SCALE` remains a P-007/P-101 instantiation obligation and must
   conform to the ratified u128 rule; no success claim is made for that absent code.
6. **PASS — executable-path review.** The vector exporter's optional path is a local
   developer CLI argument, not remote/node input; CI supplies a GUID temp filename and
   removes only that exact literal path. Symbolic JSON strings are parsed as data and
   never evaluated. The Lake manifest has zero packages. No reachable injection or
   credential-bearing workflow permission was found.
7. **PASS — independent security-diff workflow.** The sealed exact-range scan found
   zero reportable security findings. Its deterministic report was copied byte-for-byte
   from Temp to `loop/reports/P-005-security-diff-scan.md` at SHA-256
   `4E03D75535B68B0B0447CA2F452E1EAF692A90AA7F2F973300614A3619696C0F` so transient
   storage is not the only A11 evidence. The scan's generated inventory directly named
   the two JSON artifacts; the packet adversary review additionally covered all 21
   changed files and the supporting gate/policy sources.

#### Axiom sweep

| axiom | result |
|---|---|
| A1 | PASS: the block does not supply reward; the STF derives it from symbolic subsidy, checker Q, and symbolic `R_MAX`. No self-reported consensus field was added. |
| A2 | PASS by non-reachability: instance derivation and miner instance choice are untouched. |
| A3 | PASS: no timing/miner metadata enters the model; the source-policy scan found zero banned identifiers. |
| A4 | PASS by non-reachability: fork-choice code and chain weight are untouched; quality appears only in the reward model. |
| A5 | PASS at P-005 scope: amounts/Q are unsigned, arithmetic and u64 conversion are checked/proved, no float exists, and conservation names realized reward. Concrete store conservation remains P-101 work and is not claimed. |
| A6 | PASS at draft-spec scope: the single V1–V9 predicate runs against current candidate state before mutation, every state update is checked, and the public atomic result restores pre-state on rejection. Concrete storage laws remain explicit. |
| A7 | PASS by non-reachability: no runtime RPC/network/config surface changed. Abstract strict decode returns typed failure; the developer-only exporter adds no remote parser. |
| A8 | PASS: the Lean draft and symbolic vectors precede any Rust kernel consumer; CI explicitly refuses to call this P-101 conformance. |
| A9 | PASS: no Rust dependency/FFI/unsafe surface changed; the Lake manifest has no package dependencies and the workflow action is commit-pinned. |
| A10 | PASS: draft/non-normative/P-101-not-admitted labels are mechanical, and no hardness, normative-ratification, human-review, or conformance claim is inflated. |
| A11 | PASS: build, vector hash, scans, durable security report, exact content/report SHA, and hosted run 31963016862 are named above. The final evidence-only head rollup is attached to PR #9 before readiness. |

No Critical remains. The two contained abstraction boundaries are intentional packet
seams, are visible to Al's mandatory line-by-line review, and do not authorize P-101.

### 6. CALIBRATE — MODEL 4 RESUME

#### Predictions versus outcomes

- **Diff surface:** the prediction held. Governance/spec text, the Lean model/exporter,
  one active phase handler, minimum workflow setup, generated JSON, and loop evidence
  changed. No Rust/runtime implementation or owner value entered.
- **Risk materialization:** arithmetic auditability and hosted toolchain selection both
  required fixes. The feared vacuous ceiling proof did not materialize: the STF credit
  and conservation target use the exact function proved by `reward_le_subsidy`.
- **Confidence:** MED moved to HIGH for the bounded P-005 builder artifact after the
  complete Lake build, deterministic regeneration, local D6-equivalent pass, axiom
  sweep, and zero-finding security diff. Confidence remains intentionally unassigned
  to the future concrete codec/store/kernel instantiations.
- **Surprise:** the most consequential fixes were not in the Model 4 inequality; they
  were audit-seam details—a modular-looking constructor and a toolchain check performed
  one directory too high.

#### Final VERIFIED / ASSUMED / UNKNOWN ledger

**VERIFIED**

- SI-004 governance is committed at `316584a42501f7ed64135490e1a1a070a7a863eb`.
- The formal implementation is committed at
  `388ede835e8c9e668f5f0126918193ff59f589cd` and the local evidence above is green.
- `reward_le_subsidy` is a theorem derived without a reward-bound field, axiom,
  `sorry`, or admitted premise; the exact derived reward is credited by the STF.
- All owned/G0 values and SI-001/2/3 remain symbolic/unresolved; all 27 emitted cases
  are explicitly non-normative abstract interfaces.
- The durable exact-range security report records zero reportable findings.
- PR #9 is still draft, has no requested reviewer or auto-merge request, and is
  unmerged at the evidence-commit boundary. It is changed to ready-for-review only
  after the evidence-only head's exact hosted rollup succeeds.
- Hosted D6 run `31963016862` passed all eleven jobs on exact SHA
  `f494a605825a2d2dcd15d8babb71193abccae18f`.

**ASSUMED**

- `Context.blockRulesHold` faithfully instantiates B1–B8/B11–B12 when P-101 supplies
  concrete block validation. This is safe for a skeleton interface only because P-101
  remains blocked and no kernel conformance claim is made.

**UNKNOWN**

- The concrete canonical byte encoding, signing-preimage construction, address
  derivation, authenticated-store laws, and `R_MAX` storage/checked-u128 representation.
  Resolution: P-007/P-101 under G0 and the merged human-reviewed P-005.
- Whether Al will ratify the draft Lean representation line by line. Resolution: the
  mandatory PR #9 review and Al's merge.

**One process improvement for the next packet:** make every active toolchain gate run
its version check from the directory containing that packet's committed toolchain file,
and fail before the build if the exact version is not observed. This turns ambient
developer state into a mechanically excluded input.

## SECOND HUMAN REVIEW RESUME — F1/F2/F3 — 2026-08-16

Al completed the first line-by-line review without merging PR #9 and returned three
binding findings. This section resumes the same HUMAN packet under all six standing
constraints; it does not erase the earlier build, adversary record, or review tail.

### 1. FRAME — written before remediation

#### Restated work and done-condition

- **F1:** replace the undischarged `ConservationTarget` boundary with a
  `LawfulStateOps` contract containing the exact balance-replacement and account-read
  laws, then prove transaction conservation and successful block-candidate conservation.
  The transaction theorem must visibly partition sender/recipient/miner aliasing into
  all-distinct, sender=recipient, sender=miner, recipient=miner, and all-three-equal
  cases. Every law becomes a named P-101 concrete-store conformance obligation.
- **F2:** remove the `unreachable!` from `Tx.validate`; V9 remains documented by
  `v9_allows_self_send`, but no validation-path branch may panic.
- **F3:** bind `Context.rewardInputs` at P-101 to the checker output from
  `check(derive_instance(instance_seed, size_param), solution)`. A block-supplied
  quality field is forbidden. Record this as the C2 structural boundary and an explicit
  G2 check, without changing or concretizing the abstract P-005 checker interface.

The resumed builder tail is complete only when those changes compile without `axiom`,
`sorry`, or another proof placeholder; the full local Lake/D6 suite and a new exact-
delta adversary/security pass are green; the exact pushed PR head passes hosted D6; and
PR #9 remains open, ready-for-review, and unmerged for Al's second review.

#### Lane and authority

**HUMAN — explicitly resumed by Al.** F1 changes the formal STF theorem surface, F2
changes the normative validation function, and F3 adds a binding consensus provenance
obligation. The live ruling supplies the semantics and exact required cases; it does
not authorize any owner value, canonical encoding, SI-001/2/3 choice, Sarah contact,
kernel implementation, reviewer request, or merge.

#### Predicted diff surface

- Formal model: `spec/Spec/Stf.lean` and `spec/Spec/Tx.lean`.
- Binding prose/governance: `docs/PROTOCOL_SPEC.md`, `docs/ENGINEERING_PLAN.md`,
  `loop/LEDGER.md`, and `loop/PACKETS.md`.
- Verification/evidence only if mechanically required: the active Lean gate plus this
  report, `loop/STATE.md`, `loop/reports/BATCH-LOG.md`, and a durable second-review
  security-diff report. The symbolic vector definitions are predicted unchanged.

#### Top risks, falsifier, and confidence

1. **Vacuous store law:** a balance law that assumes the desired global conservation
   result, or omits the replaced account's old balance, would merely rename F1.
2. **Aliasing sleight of hand:** a proof for pairwise-distinct addresses would not cover
   the sequential-read semantics already accepted by Al; the public theorem must case-
   split on all five realizable equality partitions.
3. **Success-only mismatch:** proving helper updates while failing to connect them to
   the actual `Except` results of `applyTransaction` and `applyBlockCandidate` would
   leave the executable model undischarged.

**Falsifier:** this approach is wrong if any theorem imports conservation as a premise,
if any alias partition is absent, if the validation path retains a panic, if reward
quality can be sourced independently of the derived-instance checker, or if the proof
needs an axiom, `sorry`, or owner-controlled instantiation.

**Confidence: MED.** The additive accounting argument is straightforward, but exact
Lean reduction through checked arithmetic and `Except` sequencing is proof-engineering
sensitive. Any law that cannot be stated constructively inside the authorized interface
will trigger the required stop rather than an inferred semantic choice.

Al also accepted V1–V9 independence/order, V3 binding, V4 strict equality, evolving-
state revalidation, sequential alias handling, atomic rejection, and the Model 4 reward
theorems. Those surfaces are held fixed except where F1/F2 mechanically require proof
support.

### 2. BUILD — SECOND-REVIEW REMEDIATION

The governance FRAME was committed first at
`067d14efb3ac3b6cab433665232efc4f82723e28`. The formal remediation was then committed
at `97fa5110af60b832a9f3b26dd57cc7690a31cc75`.

#### F1 — lawful state operations and discharged conservation

- `LawfulStateOps context ops` is a `Prop` structure with the three required
  concrete-store obligations:
  - `totalBalances_setAccount` relates a replacement write to the old account balance
    with the additive equation
    `total(post) + old.balance = total(pre) + replacement.balance`;
  - `account_set_same` is read-after-write at the written address; and
  - `account_set_other` preserves every read whose address differs from the written
    address.
- `totalBalances_after_checkedSub`, `totalBalances_after_checkedAdd`, and
  `totalBalances_after_nonce` derive exact balance deltas from that local contract and
  the proved checked-arithmetic equalities. None imports a conservation premise.
- `applyValidatedTransaction_conserves_core` follows the executable match tree through
  debit, recipient credit, miner fee credit, and nonce increment. Each successful
  checked operation contributes its exact mathematical delta; `omega` closes only the
  resulting natural-number arithmetic.
- `AddressAliasing` and `classifyAddressAliasing` exhaust the five realizable
  partitions: all distinct, sender=recipient, sender=miner, recipient=miner, and all
  three equal. `applyTransaction_conserves` connects the public
  `Tx.validate`/`applyTransaction` success result to the core proof and explicitly
  eliminates every partition. The core accounting is intentionally alias-invariant
  because each credit reads the candidate state produced by the immediately preceding
  write; no pairwise-distinct premise exists.
- `applyTransactions_conserves` proves the evolving-state ordered fold by induction.
  `applyReward_conserves` proves exact addition of the same Model 4
  `rewardNat` credited by the executable path. `applyBlockCandidate_conserves`
  follows the actual block-rule, transaction, reward, and state-root success branches
  and proves
  `total(post) = total(pre) + rewardNat(context.rewardInputs block)`.
  `conservationTarget_holds` discharges the public `ConservationTarget` for every
  candidate outcome.

Protocol Spec §8 and the P-101 queue entry now bind a concrete authenticated store to
all three `LawfulStateOps` laws. P-005 does not pretend that an abstract law is already
a concrete implementation proof.

#### F2 — panic-free validation

`Tx.validate` no longer executes or mentions `unreachable!`. After V1–V8 succeed it
returns the candidate directly. The definitional V9 rule and
`v9_allows_self_send` theorem remain unchanged and continue to document that
self-sends are legal.

The active Lean handler now rejects any `unreachable!` regression in
`Spec/Tx.lean`. This textual tripwire supplements, rather than substitutes for, the
full Lake build.

#### F3 — C2 provenance obligation

Protocol Spec §11, the LEDGER overlay, the P-101 packet definition, and Gate G2 now bind
`Context.rewardInputs` to the Q returned by
`check(derive_instance(instance_seed, size_param), solution)` for that same validated
block. A block-supplied quality, work score, timing value, or equivalent field is
forbidden. Gate G2 must attempt both a wrong-instance solution and a quality-spoof
input and prove neither can influence reward.

As directed, this is an obligation rather than a premature P-005 implementation:
`Context.rewardInputs` remains abstract, no checker/decoder wiring was invented, and
the exact P-101/G2 boundary is now auditable before any Rust kernel is admitted.

No vector definition changed. All owner/G0-controlled values remain symbolic or
unfilled, and SI-001/SI-002/SI-003 remain unresolved.

### 3. VERIFY — LOCAL AND SEALED SECURITY EVIDENCE

Local verification at exact formal-remediation head
`97fa5110af60b832a9f3b26dd57cc7690a31cc75`:

- `lake build`: PASS, complete pinned Lean 4.33.0 project, 10 jobs.
- Active `lean-conformance.ps1`: PASS with
  `P005_DRAFT_SPEC_BUILD=PASS LEAN=4.33.0 VECTORS=27
  SHA256=30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`
  and `P101_RUST_CONFORMANCE=NOT_YET_ADMITTED`.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  PASS.
- `cargo deny --locked --all-features check`: PASS.
- `cargo audit --deny warnings`: PASS after loading 1,216 advisories.
- `scripts/ci/verify-geiger.ps1`: PASS across all 11 packages with zero unsafe.
- `cargo test --workspace --all-targets --all-features --locked`: PASS, 18 tests,
  zero failures.
- Conservation runtime gate: explicitly `NOT_YET_ADMITTED` for P-101.
- Codec fuzz and genesis gates: explicitly deferred to P-007 and Phase 1 respectively.
- `cargo build --workspace --all-targets --all-features --locked`: PASS.
- `git diff --check`: PASS.

A temporary proof-audit module used `#print axioms` against the two checked-arithmetic
lemmas and the public conservation theorems, then was deleted before commit. Lean
reported only its standard `propext` and `Quot.sound` foundations (the arithmetic
lemmas used `propext`; the conservation theorems used `propext, Quot.sound`). The
packet introduces no custom axiom declaration, `sorry`, `admit`, or proof
placeholder; the active gate independently rejects those tokens in every
`Spec/*.lean` file.

The Codex Security exact-diff scan
`1b01d903-6760-4402-8925-311735487350` covered
`42da63c6cdec7c0cee84033365c509e04459fe07..97fa5110af60b832a9f3b26dd57cc7690a31cc75`.
It sealed complete coverage with zero candidates, zero reportable findings, and no
deferred work. Its source classifier returned an empty compact inventory for this
Lean/PowerShell/documentation-only range, so the scan explicitly compensated by
enumerating and reviewing all eight changed files from Git. The deterministic report
was copied byte-for-byte from Temp to
`loop/reports/P-005-second-review-security-diff-scan.md` at SHA-256
`E1C78E9602266105A6739C7104637A384F4C280618C1C056805827F726C07B68`.
Measured scan usage was 173,445 total tokens, including 4,149,423 input tokens,
3,991,040 cached input tokens, and 15,062 output tokens; usage coverage was complete.

Hosted D6 run `31965617214` passed all eleven jobs on exact content/evidence SHA
`737d462e9ebf01bb9b162ee55bbc39fe11b3db06`. This final evidence-only rollup records
that immutable result and must itself pass exact-head D6 before handoff. The rollup
SHA/run URL is attached directly to PR #9 after success, with no post-run repository
commit that could invalidate the exact-head claim.

### 4. ADVERSARY — SECOND-REVIEW DELTA

#### Falsifier and abuse-case pass

| challenge | result |
|---|---|
| Can the additive law smuggle in the desired block conservation theorem? | NO. It relates exactly one `setAccount` to the account it replaces; transaction and block conservation are derived over the executable write sequence. |
| Is any equality partition silently assumed away? | NO. `classifyAddressAliasing` is constructive and exhaustive; the public transaction theorem eliminates all-distinct plus the four required alias classes. |
| Does the proof cover only a helper rather than the public `Except` result? | NO. The public transaction and block theorems destruct the actual validator/transition results and impossible error branches. |
| Can failure leak a partial candidate state? | NO. Error paths return no state, and `applyBlockAtomic` returns the exact pre-state on every candidate error. |
| Can validation still panic on V9? | NO. `Tx.validate` contains no panic path and returns directly after V8; V9 remains theorem-documented. |
| Can P-005 source Q from a block field? | NO concrete source exists in P-005. The future instantiation is now bound in §11/LEDGER/P-101/G2 to the checker over the derived instance. |
| Did remediation choose a codec, parameter, SI answer, or owner value? | NO. The symbolic Context and unchanged 27-vector artifact preserve every ownership and SI boundary. |
| Can the CI declaration grep alone counterfeit a proof? | NO. The grep is supplemental; `lake build` elaborates the complete theorem bodies and the vector artifact is regenerated and hash-compared. |

#### Axiom sweep

| axiom | result |
|---|---|
| A1 | PASS at P-005 scope: the Model 4 reward remains derived, and the new binding obligation permits Q only from the checker over the validator-derived instance. P-101/G2, not this abstract scaffold, must prove the concrete wiring. |
| A2 | PASS: no miner-authored instance surface was introduced; F3 explicitly couples checker input to `derive_instance(instance_seed,size_param)`. |
| A3 | PASS: no timing or miner metadata enters reward; the quality-spoof boundary is explicit. |
| A4 | PASS by non-reachability: fork-choice code is unchanged and Q appears only in validity/reward surfaces. |
| A5 | PASS at draft-spec scope: exact checked u64 arithmetic feeds a local additive store law; successful transactions conserve exactly and successful blocks add only the proved bounded reward. |
| A6 | PASS: the proof connects the single V1–V9 validator, sequential evolving-state writes, ordered transaction fold, reward, root check, and atomic rejection path. |
| A7 | PASS by non-reachability: no network, RPC, parser implementation, secret, or authorization surface changed. |
| A8 | PASS: the formal proof and P-101 obligations precede any Rust kernel; the handler continues to print `P101_RUST_CONFORMANCE=NOT_YET_ADMITTED`, and human review remains mandatory. |
| A9 | PASS: only Lean core `Init.Omega` was imported; no package dependency, FFI, Rust unsafe, or linked solver surface was added. |
| A10 | PASS: the report distinguishes proved abstract theorems from unproved concrete-store/checker instantiations and discloses Lean's standard proof foundations. |
| A11 | PASS: exact commits, commands, vector hash, scan ID/range/hash, coverage limitation, and durable report are recorded. Exact hosted evidence will be attached to the immutable PR head. |

No Critical, High, Medium, or Low security finding survived. The adversary pass found
no new semantic ambiguity and required no code change after the sealed scan.

### 5. CALIBRATE — SECOND-REVIEW REMEDIATION

- **Prediction versus outcome:** the predicted formal, governance, gate, and evidence
  surfaces held. The symbolic vector artifact remained byte-identical and no Rust
  runtime surface changed.
- **Risk materialization:** the primary proof-engineering risk was controlled by making
  the accounting core independent of address distinctness while still exposing an
  explicit exhaustive alias partition at the public theorem. No conservation premise,
  hidden equality premise, panic branch, or checker-provenance implementation choice
  was needed.
- **Confidence:** MED moved to HIGH for the bounded P-005 remediation artifact after
  complete elaboration, full local D6-equivalent execution, explicit proof-foundation
  audit, exhaustive alias classification, and a zero-finding sealed exact-diff scan.
  Confidence remains intentionally unassigned to P-101's future concrete store and
  checker wiring.
- **Surprise:** the requested read-after-write and read-other-address laws are stronger
  concrete-store obligations than the balance-total proof strictly consumes. The
  conservation algebra needs the additive replacement law; the two read laws remain
  binding because they establish the operational coherence P-101 must provide, rather
  than being discarded merely because this particular theorem can close without
  projecting them.

#### Final VERIFIED / ASSUMED / UNKNOWN ledger

**VERIFIED**

- F1's three-law `LawfulStateOps` contract is stated constructively.
- Successful `applyTransaction` and `applyBlockCandidate` conservation is proved
  against the actual executable results, with the required explicit alias partition.
- F2's validation panic is removed and `v9_allows_self_send` remains.
- F3 is recorded in Protocol Spec §11, the LEDGER, P-101, and Gate G2 as the C2
  structural boundary.
- Full local Lean/D6 verification is green; the symbolic vector remains exactly
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`.
- The sealed exact-range security scan has complete coverage, zero reportable findings,
  and durable report hash
  `E1C78E9602266105A6739C7104637A384F4C280618C1C056805827F726C07B68`.

**ASSUMED**

- `Context.blockRulesHold` remains the authorized abstract projection of B1–B8 and
  B11–B12. P-005 neither implements nor claims production block validation.

**UNKNOWN / RESERVED**

- Whether the future P-101 authenticated store satisfies all three
  `LawfulStateOps` laws.
- Whether P-101's concrete decoder/checker pipeline satisfies the exact F3 reward-input
  provenance obligation.
- Canonical bytes, signing preimage, address derivation, owner-controlled `R_MAX`
  value/curve, and SI-001/SI-002/SI-003.
- Al's second line-by-line judgment and merge decision. PR #9 remains unmerged, and
  only Al's merge can complete P-005 or help unblock P-101.

**Process improvement:** keep a theorem-surface audit that distinguishes required
concrete conformance laws from the subset each abstract proof projects. This makes an
unused law visible as a deliberate future obligation rather than allowing it to drift
out of the spec.

### 6. HOSTED CONTENT/EVIDENCE VERIFICATION

- Exact SHA: `737d462e9ebf01bb9b162ee55bbc39fe11b3db06`.
- D6 run: `31965617214`.
- Result: PASS, all eleven jobs.
- Lean job: PASS after a cold full build in 2m36s.
- PR state at the boundary: open, ready-for-review, no auto-merge, no requested
  reviewer, no assignee, and unmerged.
- Final rollup rule: the commit containing this paragraph receives its own exact-head
  D6. That run is recorded in the PR description rather than another commit.
