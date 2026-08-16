# C8 — Gate G0-A Partial-Ruling Builder Report

## 1. FRAME

### Task and done-condition

Al's live 2026-08-16 Gate G0 partial ruling holds G0 overall while ratifying only
G0-A. This governance closeout is not a newly manufactured engineering packet. Its
done-condition is to:

1. preserve `G0: HOLD` and leave G0-B, G0-C, G0-D, and G0-E unchanged and unfilled;
2. record the D2 amendment: `size_param` is a protocol constant changed only by an
   explicit human-ratified protocol upgrade, no dynamic size retarget exists, and B3
   checks the header value against that constant;
3. strike P-11 as moot and confine P-1/P-2 to the hash-target controller;
4. present only evidence-bounded P-1/P-2 proposals for Al's review, never normative
   values or defaults;
5. record the selection-bias standing lesson with P-006's retention figures;
6. record that size adequacy is re-reviewed at every phase gate;
7. state honestly in the README that useful-work difficulty is governance-calibrated,
   not self-calibrating;
8. update loop state, queue, and batch evidence; publish through exact-head and
   exact-merge-SHA D6; then stop idle at G0 HOLD.

### Authority and lane classification

**Lane: AUTO documentation of a supplied HUMAN ruling.** Decision ratification itself
is HUMAN under D17; Al already performed that act in the live instruction. This work
has no authority to extend it. Against the five D17 AUTO conditions:

1. the governing text is explicit and higher than the repository documents;
2. every normative edit is a mechanical application of G0-A, not an inferred choice;
3. P-1/P-2 remain proposals for review and no Al-, Sarah-, Ken-, or held-G0 value is
   filled;
4. no dependency, public API, executable behavior, formal source, vector, or workflow
   changes;
5. the existing D6 documentation path can verify the exact branch and merge heads.

G0-B through G0-E are exclusion zones. Draft PR #14 remains unmerged and is not a
write surface for this closeout.

### Predicted diff surface

- `loop/LEDGER.md` — verbatim ruling overlay, effective D2 amendment, and standing
  selection-bias/phase-gate lessons.
- `docs/PROTOCOL_SPEC.md` — only the ratified G0-A consequences in P-1/P-2/P-11, B3,
  difficulty, and parameter traceability.
- `docs/ENGINEERING_PLAN.md` — mark the D2/G0-A consequence and the every-gate size
  adequacy review without resolving any held G0 item.
- `README.md` — G0 HOLD status, governance-calibrated limitation, and corrected
  P-1/P-2/P-11 status.
- `loop/STATE.md` and `loop/PACKETS.md` — exact partial-ruling and stop state.
- `loop/reports/BATCH-LOG.md` — one partial-gate entry, completed after integration.
- this report — FRAME, verification, adversary pass, evidence classification, and
  calibration.

The historical P-006 simulator, generated evidence, Lean/vector surfaces, Rust crates,
P-007 branch/PR, audit matrix, and G0-B through G0-E text are not predicted writes.

### Evidence-bounded P-1/P-2 proposal posture

P-006 used a normalized target interval of `1.0`, not seconds. It provides no
propagation, reference-miner, production-VDF-delay, or solved-SIS distribution from
which an absolute P-1 can be derived. Therefore an absolute P-1 proposal would be
invented; this closeout will explicitly propose **no numeric P-1 value**.

For P-2, P-006 found all six unique hash-loop coordinates green: EMA windows
`{16, 32, 64}` blocks crossed with multiplicative update caps `{9/8, 5/4}`, under the
fixed model gain `1/8`. The review proposal will identify `W = 32`, gain `1/8`, and
cap `[8/9, 9/8]` as a conservative interior candidate, while stating that P-006 did
not distinguish it statistically from the other five passing coordinates. It remains
`PROPOSED FOR AL REVIEW`, never a protocol fill.

### Risks, falsifier, and confidence

1. **Ratification laundering:** a proposed hash coordinate could be presented as
   selected. Control: every occurrence carries `PROPOSED / NOT RATIFIED`, and P-1/P-2
   remain TBD in the normative table.
2. **Held-surface bleed:** nearby G0-B/C/D/E text could be altered while editing shared
   files. Control: targeted hunks plus a final semantic diff/search against those
   surfaces.
3. **Historical-evidence rewrite:** P-006's negative result could be softened after the
   ruling. Control: its simulator and generated evidence stay byte-untouched; the new
   overlay cites 0/36 full and 6/6 restricted results plus both retention ranges.

**Falsifier:** this approach is wrong if any changed text makes `size_param` dynamic,
fills P-1/P-2, resolves a held G0 item, changes PR #14, or claims the isolated hash
experiment selected a unique coordinate.

**Confidence: HIGH** for the mechanical G0-A application; **MEDIUM** for the P-2
review candidate because the passing envelope is broad and non-discriminating.

## 2. BUILD

### Governance and normative application

- Added the complete live ruling verbatim to `loop/LEDGER.md`, then separated its
  binding G0-A consequences from the non-ratified P-1/P-2 review proposal.
- Amended D2 without reopening fork choice: the historical finality trigger based on
  two-knob instability is superseded; fast finality remains an independent future ADR
  reason.
- Updated Protocol Spec §3, header documentation, B3, §10, and §15 so the only active
  retarget is the hash target. `size_param` equals a symbolic active protocol constant
  whose numeric value remains unfilled and whose change requires human ratification.
- Struck P-11 as moot without deleting or rewriting its historical P-006 evidence.
- Added the G0–G4 standing size-adequacy review to the Engineering Plan and Protocol
  Spec. The review cannot itself change the constant.
- Added the README limitation that useful-work difficulty is governance-calibrated,
  not self-calibrating, including the D2 security/usefulness distinction and P-006
  selection-retention ranges.
- Updated `loop/STATE.md` and `loop/PACKETS.md` to `G0_HOLD_IDLE`; P-007, P-008, and
  P-101 remain blocked exactly as before.

### Hash-target proposal and evidence boundary

The six P-006 hash-only coordinates were:

| EMA window | Upper cap | Reciprocal lower cap | Fixed gain | P-006 result |
|---:|---:|---:|---:|---|
| 16 | 9/8 | 8/9 | 1/8 | pass |
| 16 | 5/4 | 4/5 | 1/8 | pass |
| 32 | 9/8 | 8/9 | 1/8 | pass |
| 32 | 5/4 | 4/5 | 1/8 | pass |
| 64 | 9/8 | 8/9 | 1/8 | pass |
| 64 | 5/4 | 4/5 | 1/8 | pass |

The builder candidate `W=32`, gain `1/8`, clamp `[8/9,9/8]` is the interior window
and smaller passing cap. This is a conservative review heuristic, not an empirical
winner: P-006 reported 6/6 and did not rank them. It therefore remains visibly
`PROPOSED FOR AL REVIEW / NOT RATIFIED` everywhere.

P-1 stayed numerically blank. The P-006 model's `normalized_target_interval = 1.0`
does not map to seconds, and using the held P-3/P-4 cadence link to manufacture one
would violate both the invention tripwire and the G0-B HOLD.

### Tripwire log

- **INVENTION — contained:** the request for P-1/P-2 proposals reached an evidence
  boundary for absolute P-1. The path stopped at `TBD`; no seconds were guessed.
- **SCOPE — no expansion:** all writes stayed inside the predicted governance,
  protocol-documentation, current-status, and report surfaces.
- **HELD SURFACES — preserved:** no G0-B/C/D/E value or wording was supplied; no Rust,
  Lean, vector, workflow, dependency, P-007 branch, or historical simulator artifact
  changed.
- **ENVIRONMENT/REPETITION/DEGRADATION:** no trip.

## 3. VERIFY

### Local mechanical checks

- `cargo fmt --all -- --check` — PASS.
- workspace Clippy with all targets/features, locked dependencies, and warnings denied
  — PASS.
- `cargo deny --locked --all-features check` — PASS for advisories, bans, licenses,
  and sources.
- `cargo audit --deny warnings` — PASS; 22 lockfile dependencies scanned against
  1,216 loaded RustSec advisories.
- source policy — PASS, 14 files across 10 governed crates.
- Geiger policy — PASS, 11 governed packages and zero unsafe.
- workspace tests — PASS, 18 admitted Rust tests.
- conservation, codec-fuzz, and genesis handlers — returned their exact explicit
  `NOT_YET_ADMITTED` markers.
- locked workspace/all-targets/all-features build — PASS.
- `python -B simulate.py check` — `P006_REPRODUCIBILITY=PASS`.
- P-006 Python regression suite — PASS, 8/8.
- changed-document local-link existence check — PASS, 7/7 files.
- `git diff --check` — PASS.
- executable/evidence-surface diff (`.github`, `crates`, `spec`, `bench`, Cargo and
  toolchain policy files) — empty.
- runtime/formal-source search for a dynamic size-retarget or P-11 implementation —
  zero matches. The only dynamic-size logic remains the explicitly non-normative,
  sealed P-006 offline simulator.
- banned identifier scan in Rust (`solve_time`, `work_score`, `reported_`,
  `self_reported`) — zero matches.

The first two ad hoc local-link checker attempts failed before inspecting content
because their PowerShell `Join-Path` helper mishandled a root-level Markdown file.
The failure was diagnosed rather than retried unchanged; the replacement uses absolute
`.NET` path resolution and passed all seven changed files. No repository file was
written by any checker attempt.

### P-006 evidence readback

- committed `config.json` hash:
  `FAADF793F6B196C66F48AA2C5A5E2CCDF12A8BAEC2517FFB6C4A0CF23F13F51D`;
- committed `results.json` hash:
  `32ABD54939FE6CB4F3D1912A60CF1A2D7B5176E27C913BBC47255FF0F477698B`;
- committed generated Markdown hash:
  `BF4C71C4A2BAE7BB7EC27FEFDA4CF84BDDC0CA46086D018E3ABF98AB9DE59BC3`;
- machine config readback: windows `16,32,64`, caps `1.125,1.25`, gain `0.125`,
  normalized interval `1.0`;
- report readback: 36/36 restricted full-grid, 6/6 unique hash settings, retention
  `0.423–0.806` at 35% and `0.240–0.700` at 51%.

### Lean gate caveat

The active Lean gate built all 10 targets successfully in the long-lived canonical
Windows checkout, then returned RED at the known raw-vector-byte comparison:

- checked-out CRLF hash:
  `B45D9472FB23174605DA4318FF0A6C35D1929EC0DD2CAA59E805AA87AEF92AD9`;
- generated/Git-blob LF hash:
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`.

This packet does not claim that command passed and does not modify the vector. A fresh
staged checkout under the committed LF policy and hosted Linux D6 remain the exact-
head authorities.

The isolated staged-tree checkout then provided the positive control:

- vector SHA-256:
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`;
- CRLF count: `0`;
- pinned Lean: `4.33.0`;
- Lake build: 10/10 targets;
- symbolic vectors: 27/27;
- active gate: `P005_RATIFIED_SPEC_BUILD=PASS`;
- P-101 Rust conformance remained correctly `NOT_YET_ADMITTED`.

The snapshot path was validated beneath the OS Temp directory before creation and
again before recursive removal; it was deleted after the successful gate.

## 4. ADVERSARY PASS

### Findings and corrections

1. **Unsupported P-1 number:** P-006's `1.0` is dimensionless. Converting it to
   seconds would have crossed the invention tripwire and the held cadence link. The
   proposal was corrected to no numeric P-1.
2. **False uniqueness risk:** choosing the middle window could be misread as a measured
   optimum. Every proposal occurrence now says all 6/6 coordinates passed, P-006 did
   not distinguish them, and `W=32`/`9/8` is a conservative review heuristic only.
3. **“No codebase retarget” ambiguity:** the repository intentionally retains the
   historical P-006 offline simulator. Current documents distinguish that sealed
   rejection artifact from protocol/runtime code; no production implementation exists.
4. **Stale D2 fallback trigger:** the Engineering Plan still said two-knob instability
   could open a finality ADR. G0-A's rationale supersedes that trigger, so the text now
   makes fast-finality need the only surviving finality-ADR reason.
5. **Held-surface adjacency:** G0-B/C/D/E values appear near edited tables. The final
   hunk review confirms their numeric/semantic contents did not change; only explicit
   HOLD labels and existing blockers were added.

No Critical finding remains.

### A1–A11 sweep

- **A1 — PASS:** validators recompute the hash target and compare `size_param` to the
  active ratified constant; neither header field is trusted as self-report.
- **A2 — PASS:** miners cannot author or tune instance size through headers or
  published behavior; B3 rejects deviation from the active constant.
- **A3 — PASS:** scoring/checker definitions and inputs are unchanged; banned
  identifier scan is empty.
- **A4 — PASS:** fork choice remains hash-weight-only. The amendment explicitly
  rejects treating size-calibration failure as fork-choice failure.
- **A5 — PASS:** no amount, reward arithmetic, or executable numeric path changed.
- **A6 — PASS:** B3 now states the static-value check at validation, not as an assumed
  configuration property.
- **A7 — PASS:** no parser, RPC, startup, or error path changed.
- **A8 — PASS:** this applies Al's ratified protocol text only; Lean/vector content is
  untouched and no new formal claim is made.
- **A9 — PASS:** no dependency, language, FFI, unsafe, or solver boundary changed;
  Geiger remains zero-unsafe.
- **A10 — PASS:** the README states the governance-calibration limitation, the
  selection-bias channel, and the non-discriminating nature of the P-2 evidence.
- **A11 — PASS:** PR #18 exact head `a3b54337b3a930a421f59faac4d7368ec77627f1`
  passed D6 run `31975177926`; exact merge
  `9bbbd43fd9e4c07f8b389f182b34281183be3737` passed all eleven jobs in D6 run
  `31977067852`. The immutable URLs are recorded in the batch log.

## 5. VERIFIED / ASSUMED / UNKNOWN

### VERIFIED

- Al's supplied text ratifies G0-A and holds G0 plus G0-B through G0-E.
- P-006's machine-readable grid and evidence hashes match the committed artifacts.
- All six unique hash coordinates passed the restricted envelope; none was selected by
  the experiment.
- No runtime/formal implementation of dynamic size retarget exists.
- Draft PR #14 was open, draft, and unmerged at pickup.
- D11 was clear at pickup: CJ2 operational state reported `CAPACITY_FLAG: none` with
  the surviving GATE-1/GATE-2 items blocked/awaiting.

### ASSUMED

- During the original proposal pass, `W=32` plus the smaller passing cap was treated
  only as an interior/conservative review heuristic. No builder assumption supplied
  ratification; Al's later explicit ruling is the sole authority for provisional P-2.
- The active static size value will be supplied through held normative text or a later
  explicit upgrade. Its absence is safe here because B3's comparison shape can be
  stated symbolically without choosing the value.

### UNKNOWN

- P-1 in seconds; resolution requires propagation, production beacon delay, reference-
  miner, and solved-class distribution evidence or Al's explicit ruling.
- P-1's future ratification and the resulting mandatory re-derivation of provisional
  P-2.
- The value of the active static `size_param` and every G0-B/C/D/E item.
- Future adequacy of any static size under real solver progress; every phase gate now
  re-opens that evidence question without creating an automatic controller.

## 6. CALIBRATE — integrated

- **Predicted versus actual surface:** the original seven predicted governance/current-
  status files plus this report changed in PR #18. The closeout adds the previously
  deferred batch-log row and refreshes only current documentation/evidence surfaces.
- **Materialized risks:** unsupported absolute P-1, false unique-coordinate signaling,
  the stale D2 fallback trigger, and the offline-simulator wording ambiguity all
  materialized and were contained before commit.
- **Confidence:** HIGH held for applying G0-A, integrating immutable evidence, and
  preserving held surfaces. P-2 is explicitly provisional by human ruling; the
  experiment still does not support a tuned-optimum claim.
- **Surprise:** the strongest outcome of the proposal exercise is negative: P-006 can
  justify a broad hash-controller envelope, but not an absolute cadence or a uniquely
  preferred coordinate.
- **Next-process improvement:** parameter-proposal packets should carry a machine-
  readable `identified`, `bounded`, or `underdetermined` classification so a broad
  passing envelope cannot silently become a selected default.

## 7. G0-A integration closeout — re-FRAME recorded before closeout edits

Al's 2026-08-16 follow-up confirms that PR #18 is merged and ratifies P-2
**provisionally**: EMA window `W=32`, gain `1/8`, and multiplicative clamp
`[8/9,9/8]`. The stated rationale is the conservative interior position within the
36/36 passing hash-only grid, not a tuned optimum. P-2 MUST be re-derived when P-1 is
ratified. P-1 remains unfilled, and no numeric cadence may be proposed from P-006's
normalized interval.

This closeout is **HUMAN ruling / AUTO evidence integration**. Its done-condition is
limited to:

1. verify PR #18's actual merge SHA and the D6 run at that exact SHA;
2. record the P-2 provisional ratification without implying implementation or tuning;
3. keep P-1 visibly unfilled and proposal-free;
4. add the real G0-A integration row to `loop/reports/BATCH-LOG.md`;
5. update current governance, specification, plan, queue, README, and state text that
   still labels P-2 as proposed; and
6. stop at G0 HOLD with G0-B/C/D/E and draft PR #14 untouched.

Predicted closeout surface: `loop/LEDGER.md`, `docs/PROTOCOL_SPEC.md`,
`docs/ENGINEERING_PLAN.md`, `README.md`, `loop/PACKETS.md`, `loop/STATE.md`,
`loop/reports/BATCH-LOG.md`, and this report. No Rust, Lean, vector, workflow,
dependency, historical P-006 evidence, or PR #14 change is authorized.

Top risks are (a) accidentally laundering a P-1 value through P-2 notation, (b)
describing the provisional coordinate as experimentally preferred, (c) treating the
partial gate ruling as G0 PASS, and (d) citing a synthetic merge SHA or an incomplete
run. The falsifier is any diff outside the predicted documentation/evidence surface,
any changed G0-B/C/D/E content, any P-1 number/proposal, or any statement that P-2 is
final, tuned, or implemented. Confidence is HIGH for the mechanical evidence
closeout; P-2's own protocol status is PROVISIONAL by human ruling.

## 8. Closeout BUILD / VERIFY / ADVERSARY / CALIBRATE

### BUILD

- Preserved Al's follow-up ruling verbatim in the LEDGER and separated the binding
  disposition from the historical proposal record.
- Set P-1 to `UNFILLED; no builder proposal` everywhere current state is summarized.
- Set P-2 to `RATIFIED AS PROVISIONAL`: `W=32`, gain `1/8`, clamp `[8/9,9/8]`, not a
  tuned optimum, not implemented, and subject to mandatory re-derivation when P-1 is
  ratified.
- Added the G0-A batch row using PR #18's real merge SHA and immutable exact-head and
  exact-merge D6 runs; refreshed STATE while retaining `G0_HOLD_IDLE`.
- Changed only the eight predicted Markdown governance/evidence files. No executable,
  formal, vector, dependency, workflow, or P-006 evidence file changed.

### VERIFY

- PR #18: exact head `a3b54337b3a930a421f59faac4d7368ec77627f1`, D6 run
  `31975177926`, 11/11 jobs green.
- PR #18 merge: `9bbbd43fd9e4c07f8b389f182b34281183be3737`, exact-merge-SHA
  D6 run `31977067852`, `push` event, 11/11 jobs green including Lean conformance and
  locked build.
- Local Rust/security suite: formatting, Clippy with warnings denied, dependency
  policy, RustSec audit over 22 locked dependencies, source policy, 11-package
  zero-unsafe enforcement, 18 workspace tests, and locked build all passed.
- Phase handlers: conservation, codec fuzz, and genesis returned their exact explicit
  `NOT_YET_ADMITTED` markers; no future-test success is claimed.
- P-006: exact-byte reproducibility passed and all 8 simulator unit tests passed.
- Documentation: `git diff --check` passed; 130 local Markdown link targets resolved;
  the stale-current-surface scan found no remaining current claim that P-2 is TBD,
  unratified, or awaiting review; executable diff count was zero.
- Lean/EOL evidence: the long-lived Windows checkout built 10/10 Lean jobs but its
  wrapper correctly rejected the stale CRLF working copy (`B45D9472...`). A staged
  checkout with exact Git bytes had zero CRLF, vector SHA-256
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`, and passed the
  active Lean gate with Lean 4.33.0, 10/10 jobs, and 27 vectors. No vector changed.
- D11 remained clear at closeout pickup: CJ2 reported `CAPACITY_FLAG: none` and the
  surviving GATE-1/GATE-2 items remained blocked/awaiting. PR #14 remained open,
  draft, unmerged, and unchanged at `9acea83ca17d67a19e0d41aeb5e7275666a54013`.

### ADVERSARY — A1–A11

- **A1/A2/A3 — PASS:** no instance, checker-input, scoring, or self-report path changed;
  no executable source changed.
- **A4 — PASS:** D2 remains hash-weight-only; provisional P-2 governs the hash-target
  controller and does not couple quality or size to fork choice.
- **A5 — PASS:** no amount, reward, issuance, or arithmetic surface changed.
- **A6 — PASS:** B3's static `size_param` obligation is unchanged; no prevalidation or
  state-application path was introduced.
- **A7 — PASS:** no parser, network, RPC, startup, panic, or remote-input path changed.
- **A8 — PASS:** no Lean/vector semantic was inferred. The staged-tree check verified
  the existing human-ratified formal artifact byte-for-byte.
- **A9 — PASS:** no dependency, unsafe, native, FFI, build, or workflow surface changed.
- **A10 — PASS:** every current surface says P-2 is provisional, non-tuned,
  unimplemented, and re-derived after P-1; P-1 is unfilled and proposal-free; G0 is
  HOLD; G0-B/C/D/E remain held.
- **A11 — PASS:** the BATCH-LOG row cites the actual PR #18 merge SHA and both immutable
  D6 runs. Local red/green EOL evidence is distinguished explicitly.

No Critical finding exists. The closeout does not qualify another packet.

### Final evidence classification and calibration

- **VERIFIED:** PR #18 merge identity and 11/11 exact-merge D6 result; P-006's 36/36
  hash-only envelope; the eight-file docs-only surface; P-1's blank state; provisional
  P-2 wording; D11 clearance; PR #14's draft/unmerged state; all checks listed above.
- **ASSUMED:** none of the provisional P-2 authority. The interior/conservative
  rationale is human-ratified but remains a design rationale, not experimental proof
  of optimality.
- **UNKNOWN:** P-1's absolute cadence; the later P-2 re-derivation outcome; the active
  static `size_param` value; every held G0-B/C/D/E value; future size adequacy under
  solver progress.
- **Prediction versus result:** the eight predicted documentation/evidence files are
  the exact diff. The key materialized risk was stale proposal language, which the
  current-surface scan eliminated without rewriting the historical FRAME record.
- **Process delta:** parameter evidence and parameter authority are now recorded
  separately: P-006 bounds a passing region, Al provisionally chooses an interior
  point, and the mandatory P-1-triggered re-derivation prevents provisionality from
  silently becoming permanence.
- **Terminal state:** STOP / IDLE at G0 HOLD. No additional packet qualifies without
  touching a held surface.
