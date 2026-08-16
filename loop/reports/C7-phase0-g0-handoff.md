# C7 Phase 0 — Gate G0 Handoff

## FRAME

### Task and done-condition

Close the authorized P-006/P-007/P-009 continuous batch without crossing Gate G0.
The done-condition is a reconstruction-complete record that:

- seals P-006 and P-009 with exact-head and exact-merge-SHA D6 evidence;
- preserves P-007 as an unmerged HUMAN/semantic-ambiguity stop;
- makes `loop/STATE.md`, `loop/PACKETS.md`, `loop/reports/BATCH-LOG.md`, and the
  README current-status block agree;
- enumerates every ruling needed at G0 without supplying any ruling; and
- leaves P-008 blocked, P-101 blocked only on G0, and all Al/Sarah/G0-controlled
  values unfilled.

### Lane classification

**Bookkeeping closeout: AUTO. Gate decision: HUMAN.** The closeout is deterministic
documentation of already observable commits, runs, reports, and open gaps. It does
not change a V-rule, STF, fork choice, difficulty function, beacon verifier, formal
source, protocol vector, parameter value, ratification, or owner assignment.

The D17 AUTO conditions hold for the documentation change:

1. no new design decision is required to state the existing evidence and STOP;
2. every recorded SHA, run, count, and status has a mechanical source;
3. no Al-owned, Sarah-owned, or G0-controlled TBD is filled;
4. the normal full D6 pipeline can validate the exact branch and merge heads; and
5. no external third party is contacted or notified.

The G0 decision itself cannot satisfy those conditions: it must ratify semantics and
resolve HUMAN issues. This report therefore stops at the boundary.

### Predicted diff surface

- `loop/STATE.md` — exact current stop and next action.
- `loop/PACKETS.md` — P-006/P-009 complete; P-007 HUMAN stop; P-008/P-101 blocks.
- `loop/reports/BATCH-LOG.md` — one line per completed/stopped packet and the final G0
  stop line.
- `loop/reports/C7-p006-builder.md` and `C7-p009-builder.md` — parent-owned hosted
  merge evidence addenda.
- `README.md` — replace the stale P-005 checkpoint status with the actual G0 state.
- this handoff report.

No `loop/LEDGER.md`, Protocol Spec, Engineering Plan, Rust source, Lean source,
vector, workflow, dependency, evidence binary, or external repository is in scope.

### Top risks, falsifier, and confidence

1. **Accidental ratification:** presenting a P-006 coordinate, P-007 encoding, or
   P-009 gap as selected would cross the HUMAN boundary.
2. **False completion:** calling P-007 or codec fuzzing complete because its D6 run is
   green would conceal the handler's `NOT_YET_ADMITTED` marker.
3. **Recursive evidence drift:** committing a hosted run ID after that run would move
   the head and make the cited run stale.

**Falsifier:** this closeout is wrong if any claimed SHA/run does not match GitHub, if
the diff changes a normative or executable surface, if P-007 is merged, or if a TBD
value/semantic appears in the handoff as a decision rather than a question.

**Confidence: HIGH** for evidence and queue state; **no confidence claim** is made for
the unmade G0 decisions.

## Preflight and D11 capacity

VERIFIED at the final packet boundary on 2026-08-16:

- canonical path: `C:\Users\LEET\COINjecture3.0`; no `OneDrive` segment;
- base branch before closeout: clean `main` at
  `ce0b9752c9101e89f93a90379cd9e5ac1a08842d`;
- origin fetch and push:
  `https://github.com/Quigles1337/COINjecture3.0`;
- GitHub visibility: `PRIVATE` / `isPrivate=true`;
- D17: RATIFIED in `loop/LEDGER.md`;
- research survey SHA-256:
  `0A960F8DA6BF35315D60BBA1AD317DC99AD4F6910F53F684DB61FB639634D294`;
- P-008 target remains the literal `TBD(Al-supplied frontend URL)`; and
- a fresh targeted search of the available
  `C:\Users\LEET\COINjecture2.0` checkout found no GATE-1/GATE-2 answer,
  `remediation-priority`, or contrary capacity marker. Its unrelated modified
  `Dockerfile` was observed and left untouched. The live D11 condition therefore
  remains `cj2-blocked-on-external` for this closeout. Any later gate answer still
  causes CJ3 to pause immediately.

## Phase 0 evidence sealed in this batch

| Surface | Result | Immutable evidence |
|---|---|---|
| P-005 HUMAN-ratified Lean V1–V9/STF | COMPLETE; vectors stay symbolic/non-normative | PR #9 reviewed head `8d8ec440...`, run `31967696333`; merge `916e5027...`, run `31968030240`; closeout `44ae3ae...`, run `31968806291` |
| P-006 difficulty simulation | COMPLETE; full honest and adversarial envelopes 0/36; hash-only 6/6 unique settings | PR #16 head `eb6b96d...`, run `31971074815`; merge `7244f094...`, run `31971438778` |
| P-007 `cj3-types` | HUMAN STOP / INCOMPLETE; checked `Amount` only; draft and unmerged | draft PR #14 head `9acea83c...`; exact-head run `31969740766` |
| P-009 audit traceability | COMPLETE; 33 security rows, 25 Lean claims, R1–R8; GAP-7–13 proposed | PR #15 head `34ce596d...`, run `31971741833`; merge `ce0b9752...`, run `31972017916` |
| P-008 frontend seam | BLOCKED | exact Al-supplied URL absent; no URL inferred and no Sarah contact |
| P-101 kernel | BLOCKED | Gate G0 only; P-005 human-review prerequisite is closed |

The full earlier P-001 through P-004 and P-010 lineage remains in
`loop/reports/BATCH-LOG.md` and `loop/STATE.md`.

## What P-006 actually establishes

VERIFIED:

- 36 full-grid coordinates, nine scenarios, 24 fixed sensitivities, and 5,971,968
  simulated blocks were checked.
- The full honest/unmanipulated-quality envelope is 0/36.
- The full adversarial envelope is 0/36.
- The isolated hash loop passes 36/36 repeated full-grid coordinates, representing
  all 6/6 unique hash settings (three EMA spans by two caps).
- No tested 64–256-block size window survives solve-power and interaction
  sensitivities.
- Under the stated model, an authentic winning-block-quality signal remains
  strategically selectable and biases a quality-only size loop.

NOT ESTABLISHED:

- no P-1 target in seconds;
- no selected P-2 EMA/window/clamp;
- no P-11 size window or exact retarget function;
- no real SIS effort-to-quality response, propagation model, or fork probability;
  and
- no conclusion that D2 is automatically repealed or that a finality gadget is
  automatically selected.

The negative result triggers HUMAN review under D2's own promotion/fallback language;
it does not authorize the builder to choose the response.

## Exact Gate G0 ruling package

### G0-A — P-006 response under ratified D2

Rule how the Phase 2 design proceeds after the full two-knob model produced 0/36:

1. retain D2 and require a manipulation-resistant size observable before any dynamic
   size retarget freezes; or
2. retain D2 with `size_param` static between explicit human protocol upgrades; or
3. activate D2's ADR trigger and evaluate the named finality/fallback path.

This list frames the already documented alternatives; it is not a recommendation or
ratification. In every case, P-1/P-2/P-11 remain UNFILLED until evidence supports
exact values and functions.

### G0-B — SIS claim language and provisional P-3/P-4

1. Resolve SI-001 by replacing Protocol Spec §5.2's unconditional-hardness wording
   with the A10-compliant conditional reduction/estimator/measurement wording, or
   supply a different explicit ruling.
2. Accept, revise, or reject P-003's provisional `VALIDATION_BUDGET = 15 ms` together
   with its named reference-hardware definition.
3. Accept, revise, or reject provisional SIS tuple
   `(n,m,q,beta_squared) = (128,3840,16411,3840)` and its explicitly conditional
   128-bit estimator-model threshold.
4. Address the unresolved cadence link: P-003 did not demonstrate the tuple's solve
   distribution, and P-006 could not supply P-1 or a robust quality-driven size loop.

No item above is silently accepted by inclusion in this report.

### G0-C — P-007 canonical byte and signature semantics

P-007 cannot complete until G0/HUMAN authority supplies one exact answer for each:

1. **Canonical codec grammar:** envelope/framing, byte order, canonical zero and
   minimal unsigned integers, byte/list lengths, schema/version treatment, fixed
   field order, and rejection of nonminimal, duplicate, trailing, missing, and
   unknown data.
2. **Hash and preimage boundary:** confirm or revise SHA-256 for `H`; assign exact
   byte strings for `D_ADDR`, `D_TX`, `D_HDR`, `D_INST`, `D_SOL`, `D_BEACON`, and
   `D_GENESIS`; define collision-free concatenation/framing.
3. **SI-002:** choose canonical signed-coefficient width/minimality, endian and sign
   convention, reject alternate encodings, and either define `s_max` or ratify the
   equivalent derived per-coefficient squared guard.
4. **SI-003:** choose SHAKE-256 candidate width, endian, acceptance ceiling, rejected-
   candidate consumption, and canonical derivation vectors.
5. **P-8:** ratify `TX_MAX_BYTES`, `BLOCK_MAX_BYTES`, and `SOL_MAX_BYTES` so decoder
   and fuzz boundary behavior is meaningful.
6. **GAP-12 / Ed25519:** require or revise strict verification, canonical key and
   signature encodings, and explicit invalid/zero-signature rejection.

Draft PR #14 MUST remain unmerged until these rulings are applied and the complete
packet passes its adversary pass and a new exact-head D6. Its current green codec job
is only the committed `NOT_YET_ADMITTED` marker.

### G0-D — remaining G0-owned protocol values

Protocol Spec v0.1 still marks P-10 `TIMESTAMP_DRIFT_Δ` for G0 ratification and
P-8 as above. Gate G0 must either provide evidence-backed values or explicitly keep
the specification draft and assign a later ruling boundary. This closeout supplies
neither value.

P-7, P-9, and P-12 retain their existing Al/Sarah/Ken ownership. In particular,
P-7 and reward-curve shaping remain unfilled until the D16 reveal; G0 must not treat
their absence as agent permission to choose them.

### G0-E — P-009 proposed security amendments

For GAP-7 through GAP-13, rule one of: ratify as future phase requirements, revise,
or explicitly defer with an owner/gate. The proposals cover:

- authenticated gossip and freshness;
- peer-set, dial, and eclipse bounds;
- per-principal mempool occupancy and eviction;
- authoritative client address / trusted-proxy handling;
- public error hygiene;
- strict Ed25519 semantics (also blocking P-007); and
- bounded limiter/TLS/connection state.

P-009 did not implement or ratify any of them. The absent original Codex report also
means its exact per-finding inventory cannot be declared complete.

### G0-F — gate disposition

After the rulings above, Al can accept/revise the Phase 0 spike record, decide whether
Protocol Spec v0.1 becomes normative, and explicitly PASS or hold G0. P-101 remains
blocked until that gate disposition. D1, D2, D14, D15, D9, D10, D16, D17, the P-005
human ratification, and SI-004 are already ruled and are not reopened by this handoff.

P-008's missing frontend URL is an independent blocked input; no URL is inferred here.

## Explicit non-decisions

This closeout does **not**:

- select a P-006 coordinate or a difficulty/size-retarget function;
- fill P-1, P-2, P-3, P-4, P-7, P-8, P-9, P-10, P-11, or P-12;
- choose codec bytes, domain bytes, an `s_max`, or SHAKE consumption;
- ratify GAP-7 through GAP-13;
- merge or present draft PR #14 as complete;
- claim codec fuzzing exists;
- contact, invite, attribute review to, or notify Sarah;
- resolve the Lean 4.33.0 versus CJ2 Lean/Mathlib 4.28.0 gap before D16; or
- infer the P-008 frontend URL.

## Local mechanical verification

The closeout was tested in the canonical Windows checkout before commit:

- `cargo fmt --all -- --check`: PASS.
- workspace all-target/all-feature clippy with `-D warnings`: PASS.
- `cargo deny --locked --all-features check`: PASS for advisories, bans, licenses,
  and sources.
- `cargo audit --deny warnings`: PASS across 22 locked dependencies.
- `scripts/ci/verify-geiger.ps1`: PASS for 11 packages, zero unsafe.
- workspace all-target/all-feature tests: PASS (18 Rust tests).
- conservation, codec-fuzz, and genesis handlers returned their exact explicit
  deferral markers; no deferred feature success is claimed.
- locked all-target/all-feature workspace build: PASS.
- `python -B bench/p006-difficulty/simulate.py check`: PASS.
- P-006 Python unit suite: PASS, 8/8.
- `git diff --check`: PASS.

The active Lean handler built all ten Lean targets, then correctly returned **RED** in
this pre-existing canonical checkout because its working-tree JSON still has CRLF:

- checked-out SHA-256:
  `B45D9472FB23174605DA4318FF0A6C35D1929EC0DD2CAA59E805AA87AEF92AD9`;
- canonical/generated LF SHA-256:
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`.

The strings are byte-identical after CRLF-to-LF normalization, and P-006 already
committed the narrow `text eol=lf` rule. This closeout does not edit or regenerate the
HUMAN vector.

A fresh checkout-index snapshot of the complete staged closeout then verified the
portable path mechanically: CRLF count zero, vector SHA-256 `30CABF85...`, all ten
Lean targets built, the active handler reported 27 vectors and PASS, and the P-006
reproducibility check passed. The isolated snapshot was removed after verification.
Hosted exact-head and exact-merge-SHA D6 remain authoritative.

## ADVERSARY PASS

**Seat switch: ADVERSARY.** The complete documentation diff is re-read against A1–A11
before publication.

- **A1–A4:** no runtime input, instance derivation, scoring, fork-weight, or
  difficulty code changes. The negative P-006 result is not converted into a passing
  coordinate.
- **A5–A7:** no amount arithmetic, parser, decoder, apply path, or error path changes.
- **A8:** no Protocol Spec, `Spec/*.lean`, or vector change. HUMAN semantics are
  enumerated only as questions.
- **A9:** no dependency, FFI, solver, or trusted-code change.
- **A10:** P-007 remains incomplete; deferred D6 markers are not described as feature
  success; P-009's missing Codex inventory stays UNKNOWN; proposed gaps stay proposed.
- **A11:** every completion claim carries a SHA/run/report pointer. Final hosted D6
  for this closeout is recorded in PR metadata so the evidence does not recursively
  invalidate the commit it attests.

Critical falsifiers reviewed: no owner value, no semantic choice, no P-007 merge, no
Sarah action, no public-visibility change, and no hidden red packet result.

## VERIFIED / ASSUMED / UNKNOWN

### VERIFIED

- All recent packet SHAs and hosted run IDs in the Phase 0 table match the CI system.
- P-006 and P-009 are merged and their exact merge SHAs passed all eleven D6 jobs.
- P-007 draft PR #14 is open, draft, unmerged, and exact-head green.
- P-008's exact URL remains absent; P-101's only remaining recorded block is G0.
- P-006's negative matrix counts and P-009's source-inventory counts match their
  committed machine/report evidence.

### ASSUMED

- The absence of a new CJ2 gate-answer/capacity marker, combined with Al's live D11
  ruling, is sufficient for this bounded closeout. This assumption authorizes no
  further packet after the G0 STOP.
- PR metadata is the durable place for the closeout head/merge run IDs because adding
  those IDs to the attested commit would move the head.

### UNKNOWN

- Every HUMAN decision listed in G0-A through G0-F.
- The exact contents of the absent original Codex scan.
- The P-008 frontend URL.
- Whether CJ2's external gate answers will arrive before a future CJ3 resume.

## CALIBRATE

- **Predicted versus actual surface:** exact. The staged tree contains only the six
  predicted existing documentation files plus this report; no executable, normative,
  vector, evidence-binary, or LEDGER path changed.
- **Risk materialization:** the main risk was stale public documentation: the README
  still said P-005 had stopped before Lean content, despite its reviewed merge. The
  closeout corrects that claim without changing authority.
- **Confidence:** HIGH remains appropriate for evidence reconciliation. The stale-
  checkout red and fresh-checkout green were both preserved; the report explicitly
  makes no confidence claim for G0's design choices.
- **Surprise:** the P-006 hash loop was broadly stable while every full coordinate
  failed through the size signal, making the observable—not fine-tuning the EMA—the
  decision surface.
- **One process improvement:** future phase batches should create a gate-ruling matrix
  at packet FRAME time and update it after each packet, so the final gate handoff is a
  reconciliation rather than a late reconstruction.

## STOP

**Gate G0 — HUMAN RULING REQUIRED.** No Phase 1 packet may start. Resume only after
Al rules the package above and D11 is re-checked.
