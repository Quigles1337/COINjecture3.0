# Cycle 7 — P-006 Builder Report

**Status:** IMPLEMENTATION COMPLETE — LOCAL EVIDENCE SEALED; PARENT CI/MERGE PENDING
**Date:** 2026-08-16
**Packet:** P-006 — offline two-knob difficulty simulation
**Lane:** AUTO, limited to non-normative analysis
**Branch:** `feat/p006-difficulty-simulation`
**Delegated base:** `44ae3ae8807bc7ae594494870bffa96da6ea0ca1`

## 1. FRAME

### Packet and done-condition

P-006 builds a deterministic, offline model of the interaction between the hash-
eligibility retarget and the slower instance-size retarget. It exercises honest and
adversarial schedules for both hash power and solve power, measures normalized block-
interval error, overshoot, recovery, and controller saturation, and reports the
stability envelope that must inform the D2 Phase-2 gate.

The packet is done only when:

- a standard-library Python simulator lives entirely under `bench/`, uses fixed seeds,
  bounded runs, and regenerates byte-identical machine-readable evidence;
- the hash knob is driven only by simulated inter-block intervals and the size knob is
  driven only by simulated checker-derived quality margins, never by a miner-reported
  time, score, or quality field;
- stationary, step, alternating/periodic, and adversarial hash/solve-power schedules
  are represented, including strategic suppression of publishable quality margin;
- the report defines a quantitative stability envelope and distinguishes robust
  candidate regions from unstable or underdetermined regions;
- any candidate window, clamp, gain, target time, or signal model is labeled
  **NON-NORMATIVE / PROPOSED FOR HUMAN RATIFICATION** and no Protocol Spec P-1, P-2,
  P-11, exact retarget function, Gate-G0 choice, SI, or owned TBD is filled;
- the full local verification and A1–A11 adversary pass complete with durable evidence,
  and no Critical finding remains.

P-006 does not implement chain code. It cannot make the Phase-2 retarget normative;
Protocol Spec §10 expressly requires later ratification of this stability report.

### Lane classification against the five D17 AUTO conditions

**Classification: AUTO for bounded, non-normative offline analysis.**

1. **Approved and unblocked:** P-006 is approved/ready in `loop/PACKETS.md`. The
   delegated base records `CAPACITY_FLAG: cj2-blocked-on-external`, while a fresh
   targeted read of the operational 2.0 checkout found `CAPACITY_FLAG: none` and
   GATE-1/GATE-2 still listed as blocking open packets; no gate-clear or
   `remediation-priority` evidence was found.
2. **No consensus implementation:** all executable work stays under `bench/`; no
   node, consensus, class, kernel, beacon, networking, or storage source is touched.
3. **No HUMAN formal surface:** no `Spec/*.lean` file or vector definition is in
   scope.
4. **No ratification:** the simulator may compare explicitly hypothetical candidates
   because P-006 owns measurement, but it cannot select the exact protocol function or
   mark any candidate ratified. Any semantic choice needed to make a normative claim
   is a HUMAN/G0 stop.
5. **No owned-TBD fill:** P-1/P-2/P-11 remain visibly unfilled in
   `docs/PROTOCOL_SPEC.md`. The experiment uses a dimensionless target-time unit and
   candidate controller coordinates; it will not assign an absolute cadence, reward
   parameter, VDF delay, SIS floor, economics value, or Sarah/Al-owned input.

The autonomous prompt normally requires pickup from the canonical checkout on
`main`. Al's live continuous-batch direction expressly permits P-006/P-007/P-009 in
parallel, and the parent builder delegated this linked worktree and feature branch at
the exact main SHA above. `git worktree list --porcelain` ties this worktree to
`C:\Users\LEET\COINjecture3.0`; both origin URLs are exact, and GitHub reported the
repository `PRIVATE`. That live delegation overrides only the pickup path/branch
mechanics, not any tripwire, lane, evidence, or merge condition.

### Predicted diff surface

- This report: `loop/reports/C7-p006-builder.md`.
- A dedicated `bench/p006-difficulty/` tree containing the simulator, standard-library
  tests, reproduction documentation, immutable scenario/candidate definitions, and
  generated JSON/Markdown/environment evidence.

No root workspace manifest, Cargo lockfile, CI workflow, Rust `src/`, formal spec,
protocol document, queue/state/batch ledger, or decision ledger edit is predicted.
The parent builder owns packet-boundary bookkeeping and hosted CI/merge evidence. A
need to touch any consensus-semantic or governance surface triggers re-FRAME or STOP
before that edit.

### Model boundary predicted before implementation

- Time is normalized so one target interval is `1.0`; this avoids inventing P-1.
- Hash difficulty is an abstract positive ratio, not a 256-bit target encoding.
- Instance size is an abstract positive work ratio, not a SIS tuple or P-4 mutation.
- Inter-block time is decomposed into a solution-acquisition component and a hash-race
  component so the two knobs can interact without claiming a production miner model.
- The size controller observes only a synthetic checker-derived quality margin. Its
  signal elasticity and adversarial suppression are sensitivity axes, not protocol
  facts.
- A stable candidate must meet predeclared error/recovery/saturation criteria across
  all required scenarios and multiple fixed seeds; passing this toy model is necessary
  evidence for a proposal, never sufficient evidence for ratification.

### Top risks

1. **Self-confirming toy model:** choosing one convenient relationship between solve
   power, instance size, and checked quality could make any controller look stable.
   The experiment must sweep signal elasticity/noise and expose sensitivity rather
   than treat one equation as measured reality.
2. **Consensus invention by labeling:** numeric candidate windows/clamps could be
   copied into §10 and mistaken for P-2/P-11. Every artifact and recommendation must
   preserve the non-normative/HUMAN-ratification boundary, and absolute P-1 must stay
   unresolved without production solver/VDF/propagation evidence.
3. **Adversarial observability gap:** checker-derived quality is authentic but miners
   can choose which valid solution to publish. A quality-driven size controller may
   therefore be gameable even though it reads no self-reported field; the simulation
   must model suppression and fail the candidate region if that attack drives it
   outside the stability envelope.

**Falsifier:** this approach is wrong if no controller candidate remains within the
predeclared stability envelope across the honest and adversarial sensitivity matrix,
or if stability depends on treating solve time/power as directly observable on-chain.
In that case P-006 will report the negative result and recommend the D2 fallback/gate
review; it will not tune away the failure or invent a trusted signal.

**Confidence:** MEDIUM. Deterministic offline experimentation is tractable, but the
quality-margin response to real SIS search effort is not measured at the provisional
P-4 candidate, so robust conclusions must remain conditional.

### Pickup verification recorded before BUILD

- Worktree was clean at exact delegated base
  `44ae3ae8807bc7ae594494870bffa96da6ea0ca1`; the base is also the linked canonical
  `main` HEAD at pickup.
- Origin fetch/push were exactly
  `https://github.com/Quigles1337/COINjecture3.0`; `gh repo view` returned
  `visibility: PRIVATE`.
- D17 is RATIFIED in `loop/LEDGER.md`; the P-005 exact-merge boundary makes P-006
  ready and P-101 remains blocked on G0.
- `docs/RESEARCH_SURVEY.md` is present. P-008 remains blocked on its exact URL and is
  unrelated to this packet.
- Fresh D11 evidence is the targeted 2.0 read described above; if that condition
  changes before handoff, P-006 pauses.

### Scope re-FRAME — shared Windows vector-checkout false red

After the simulator build and local Rust gates, the active Lean handler built all ten
targets but compared the freshly generated LF vector against a CRLF working-tree copy.
The index/blob vector hash was the expected
`30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`, while
Windows Git's inherited `core.autocrlf=true` checkout hashed
`B45D9472FB23174605DA4318FF0A6C35D1929EC0DD2CAA59E805AA87AEF92AD9`.
P-007 and P-009 independently reproduced the same fresh-worktree failure, falsifying
the one-off-environment hypothesis.

**Authorized expansion:** add a text/EOL attribute for the real active artifact,
`spec/vectors/p005-draft.json text eol=lf`, and a rule confined to the already
predicted P-006 research tree, `bench/p006-difficulty/** text eol=lf`, while preserving
the existing DOCX/PDF binary rules. The second rule is necessary because exact-byte
regeneration embeds and verifies the simulator/config hashes; allowing Windows to
rewrite those new text files would reproduce the same false red inside P-006 itself.
The parent builder authorized this cross-packet infrastructure repair so fresh Windows
worktrees verify the same bytes as hosted Linux D6.

**Lane remains AUTO.** `.gitattributes` changes checkout normalization only. It does
not edit, regenerate, reinterpret, stage a content change to, or ratify the HUMAN
vector; the Git blob must remain the exact SHA-256 above. The P-006 rule changes only
checkout normalization for files already within the predicted tree. No `Spec/*.lean`,
vector definition, protocol semantic, or owned TBD is in scope. The expansion is
wrong if an attribute targets any broader tree, changes the vector blob, fails to
yield LF checkout-filter bytes under `core.autocrlf=true`, or a fresh Windows-normalized
snapshot cannot pass `python -B simulate.py check`; any of those outcomes stops the
repair rather than widening it.

## 2. BUILD

### Offline implementation

The packet added only standard-library Python analysis under
`bench/p006-difficulty/` plus the narrowly re-framed `.gitattributes` repair. No
Python module enters the Rust workspace or any node/runtime path.

- `config.json` is the complete experiment contract: 36 hypothetical controller
  coordinates, nine schedules, four fixed seeds, two solve-stage shares, three
  quality elasticities, 768 blocks per run, and a predeclared envelope. It carries
  the exact non-normative/HUMAN-ratification marker.
- `simulate.py` validates the schema, status marker, numeric domains, aggregate work,
  scenario identifiers, and equilibrium relationship before running. The
  configuration is capped at 1,000,000 bytes, individual dimensions are bounded, and
  aggregate execution is capped at 20,000,000 simulated blocks.
- `test_simulate.py` covers deterministic fixed-seed execution, the exact candidate
  grid, byte-identical regeneration, below-threshold/non-finite quality rejection,
  per-field and aggregate work bounds, the normative-status guard, and recovery-time
  accounting.
- `evidence/results.json` contains every coordinate/scenario result. No failing row is
  filtered out. `evidence/STABILITY-ENVELOPE.md` is generated from the same in-memory
  result; `evidence/ENVIRONMENT.md` seals the runtime, revision, and artifact hashes.
- `.gitignore` excludes only Python bytecode/cache artifacts inside this bench tree.

### Explicit non-normative model

Let `f` be the modeled share of the normalized target interval spent acquiring a
solution at equilibrium, `S` the abstract instance-size state, `D` the abstract hash-
difficulty state, and `P_s`/`P_h` the scheduled relative solve/hash power. The model
draws independent exponential stages with means:

```text
E[solution delay] = f · S / P_s
E[hash delay]     = (1-f) · D / P_h
E[block interval] = their sum
```

The hash loop sees only the resulting inter-block interval. It maintains an EMA with
candidate spans 16/32/64 blocks and applies a bounded logarithmic response under a
fixed research gain. No quality value enters this loop, preserving A4.

The synthetic checked-quality sensitivity is:

```text
Q = 1 + 0.5 · (P_s / S)^elasticity · mean-one lognormal noise
```

where elasticity is swept across 0.5/1/2. The size loop sees only the resulting
checker-derived `Q` values, averages them over candidate 64/128/256-block windows,
and applies a bounded response. `P_s` is an offline environmental coordinate; it is
never represented as an on-chain field or controller input. All controller states are
dimensionless. Numeric guard saturation is recorded as an envelope failure rather
than hidden by aborting or clipping a successful result.

This equation is an explicit sensitivity assumption, not observed SIS behavior. The
matrix evaluates 36 × 9 × 24 bounded runs, or 5,971,968 simulated blocks. The nine
schedules cover a stationary baseline; separate 4×/0.25× hash and solve shocks; an
opposed one-time power shock; a 64-block opposed adversarial power oscillation; and
35%/51% strategic quality-margin suppression. In the suppression cases the attacker
does not forge `Q`: it is assumed able to publish a different valid solution whose
checker-derived margin is lower.

### Predeclared stability envelope

A coordinate passes a scenario only if every seed/sensitivity run satisfies all
applicable bounds from `config.json`:

- worst tail normalized-interval error ≤ 20%;
- worst tail hash-state and size-state relative error ≤ 35% when an equilibrium state
  exists;
- a step recovers to a ±20% rolling interval band, held for 24 observations, within
  384 blocks;
- controller excursion stays within 5 log2 units and never hits the numeric guard;
- an adversarial quality-selection case retains at least 80% of honest equilibrium
  instance size.

These are falsification thresholds for this experiment, not proposed protocol
constants.

### Candidate findings

| surface | result | permissible conclusion |
|---|---:|---|
| stationary baseline | 36 / 36 pass | the toy controller has an equilibrium |
| isolated 4×/0.25× hash shocks | 36 / 36 full-grid coordinates pass, representing 6 / 6 unique hash-loop settings | broad hash-only research region: EMA 16–64 blocks, 1.125×–1.25× update caps under the fixed gain |
| isolated 4×/0.25× solve shocks | 0 / 36 pass | no tested size-window coordinate is robust across the elasticity/stage-share sweep |
| opposed one-time and 64-block periodic power schedules | 0 / 36 pass | the two-knob system does not clear the declared interaction envelope |
| full honest/unmanipulated-quality envelope | 0 / 36 pass | no full retarget candidate may be promoted |
| full adversarial envelope | 0 / 36 pass | no full retarget candidate may be promoted |

The 36/36 restricted full-grid result comprises 6/6 unique hash-loop settings (three
EMA spans × two caps), each repeated across six size-window/cap coordinates. It is
deliberately not narrowed to one P-2 value: the experiment cannot distinguish among
those six settings. The tested P-11 region of 64–256 blocks has no robust candidate.
Therefore P-2/P-11 and the exact clamps remain unratified, and a size retarget driven
solely by winning-block quality margin is not supported by this evidence.

The strategic-selection direction is structural inside the stated model. With an
attacker controlling fraction `a` of published blocks and suppressing its excess
margin to threshold, the asymptotic size ratio is `(1-a)^(1/elasticity)`. It ranges
from 0.423–0.806 at `a=0.35` and 0.240–0.700 at `a=0.51` over the configured
elasticities. Some slow candidates meet the finite-run retention threshold only
because the same slowness makes them fail honest solve-power adaptation; delay is not
resistance.

### P-1 and solve-once boundary

No absolute block cadence is proposed. P-003 did not solve its provisional P-4
candidate, P-002 supplied no production VDF delay, and there is no propagation or
reference-miner distribution. Assigning seconds would be invention, so P-1 remains
unfilled for G0.

The model does quantify the solve-once boundary without inventing propagation. At
solve-stage shares 0.2/0.5, an equal-power miner's initial cycle has normalized mean
1.0 and variance 0.68/0.50. After a valid solution is public, a copier still reruns
the full miner-bound hash race but skips the solve stage, giving hash-only mean
0.8/0.5 and variance 0.64/0.25. Fork probability remains unknown because it requires
propagation, miner count/power, and withholding data not present here.

### Tripwire and failure log

1. **Fixed — unstable coordinate aborted the matrix.** The first generation stopped
   when a solve-power-down scenario drove hash state below the numeric domain. That
   was evidence about controller failure, not an infrastructure error. The simulator
   now records guard hits as saturation failures and continues the bounded matrix, so
   no unstable coordinate disappears through an exception.
2. **Fixed — recovery was credited too early.** Initial recovery accounting returned
   the start offset of a qualifying rolling window. The adversary review corrected it
   to the end of the 32-block observation window plus the 24-observation hold, with a
   direct regression test.
3. **Fixed — aggregate-work and Markdown-input gaps.** Individually bounded axes
   could multiply into excessive work, and an unrestricted local scenario ID could
   alter generated Markdown structure. The global block cap and lowercase ASCII ID
   grammar now fail closed before execution/output.
4. **Re-framed/fixed — shared CRLF evidence false red.** Fresh Windows worktrees
   transformed the committed vector's LF into CRLF because `.gitattributes` covered
   only audit binaries; the same conversion would also invalidate P-006's embedded
   source/config hashes. The exact active vector and the dedicated P-006 research tree
   now have `text eol=lf`. The vector blob remains unchanged; details are in VERIFY.
5. **HUMAN/G0 stop preserved — size observable.** No manipulation-resistant size
   signal or static-size policy was inferred. No P-1/P-2/P-11 value, Protocol Spec
   edit, D2 revision, SI decision, or owned value was made.

No identical failure recurred three times. Except for the explicitly re-framed EOL
repair, the implementation stayed inside the predicted bench/report surface.

## 3. VERIFY

### Simulator evidence

- `python -B simulate.py generate` — PASS; produced both committed artifacts.
- `python -B simulate.py check` — PASS; regenerated both artifacts under a temporary
  directory and compared exact bytes.
- `python -B -m unittest -v test_simulate.py` — PASS, 8/8 tests.
- PowerShell `ConvertFrom-Json` parsed both `config.json` and `results.json` — PASS.
- `git diff --check` — PASS at the verification boundary.
- Final SHA-256 seals:
  - config: `FAADF793F6B196C66F48AA2C5A5E2CCDF12A8BAEC2517FFB6C4A0CF23F13F51D`;
  - simulator: `C90157F08DA2A93080177862B7A5FA1B5D9D0DA2F36159CED47801B8A5DB12A5`;
  - tests: `1067F5CCC7FDDA505AE7BD7EA0F97C0D3DE07C24F394EC0EEF7C39F7A5B95630`;
  - JSON result: `32ABD54939FE6CB4F3D1912A60CF1A2D7B5176E27C913BBC47255FF0F477698B`;
  - generated Markdown:
    `BF4C71C4A2BAE7BB7EC27FEFDA4CF84BDDC0CA46086D018E3ABF98AB9DE59BC3`.

### Local D6-equivalent verification

- `cargo fmt --all -- --check` — PASS.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` —
  PASS.
- `cargo deny --locked --all-features check` — PASS: advisories, bans, licenses, and
  sources all `ok`.
- `cargo audit --deny warnings` — PASS; 22 lockfile dependencies scanned against the
  current local RustSec database.
- `./scripts/ci/verify-geiger.ps1` — PASS: source policy passed and all 11 `cj3-*`
  workspace packages reported zero unsafe with `forbid(unsafe)` enforced.
- `cargo test --workspace --all-targets --all-features --locked` — PASS: all admitted
  tests green (18 non-empty Rust tests across the existing suites).
- `conservation-invariant`, `codec-fuzz-smoke`, and `genesis-spend-test` returned
  their exact `NOT_YET_ADMITTED` markers; this report does not claim those deferred
  tests executed.
- The pinned Lean 4.33.0 project built all 10 targets successfully and emitted the
  expected generated artifact hash
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`.
  Before the EOL repair, the active wrapper then failed on the CRLF worktree hash
  `B45D9472...`; this was the shared checkout defect described above, not Lean or
  semantic vector drift.
- `cargo build --workspace --all-targets --all-features --locked` — PASS.

### EOL repair verification

- `git check-attr text eol` returned `text: set` and `eol: lf` for the active vector
  and the P-006 simulator/config/evidence text paths.
- The unchanged Git blob SHA-256 remains
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`.
- A new full-index `git checkout-index` snapshot produced under an explicit
  `core.autocrlf=true` in an isolated temporary directory retained the vector's same
  SHA-256, zero CRLF sequences, and 247 LF bytes. Inside that fresh snapshot,
  `python -B simulate.py check` returned `P006_REPRODUCIBILITY=PASS`, proving the new
  source/config hashes and generated evidence survive a Windows-normalized checkout.
  `git status`/`git diff` showed no vector content change; `.gitattributes` is the only
  cross-packet repair surface.

Hosted exact-head D6 is intentionally not claimed by this delegated worker. The
parent builder owns push/PR, CI-system readback, merge, and exact-merge-SHA D6.

## 4. ADVERSARY PASS

**Seat switch: ADVERSARY.** I re-read the complete model, configuration, tests,
generated evidence, environment seal, report, and `.gitattributes` expansion after
verification. The three fixed implementation findings are preserved verbatim in the
tripwire log instead of being erased from the narrative.

### Axiom sweep

| axiom | adversary result |
|---|---|
| A1 — derive, don't read | PASS by scope. The experiment reads explicit offline schedules only; it creates no consensus field or retarget implementation. Any future protocol target/size remains validator-derived under §10. |
| A2 — protocol-generated instances | PASS. No instance, seed, SIS tuple, class registry, or miner-controlled protocol parameter exists in this diff. |
| A3 — pure scoring | PASS. Synthetic `Q` is computed from model state and fixed-seed noise. Strategic selection chooses a modeled valid solution; it never injects reported quality. No banned identifier occurs in Rust `src/`. |
| A4 — decoupled fork choice | PASS. The hash loop consumes only interval observations. `Q` affects only the isolated size experiment and never chain weight. |
| A5 — integer money | PASS. Python floating point is confined to D8-authorized bench analysis and cannot reach an amount, reward, or runtime crate. No money surface changed. |
| A6 — apply trusts nothing | PASS by non-reachability. No state, transaction, block, or apply API changed. |
| A7 — fail closed | PASS for the touched tool. Invalid/non-finite inputs, wrong schema/status, oversized configuration/work, numeric divergence, malformed scenario identifiers, missing evidence, and regeneration drift fail explicitly. |
| A8 — spec before code | PASS. No state-machine code, `Spec/*.lean`, or vector content changed. The EOL attribute preserves the already ratified vector blob exactly. |
| A9 — minimal TCB | PASS. The tool is Python-standard-library and bench-only as D8 permits. It launches no solver, network process, FFI, or native extension. All 11 CJ3 packages remain zero-unsafe. |
| A10 — honest claims | PASS. Full-system stability is 0/36, not tuned into a pass. Hash-only evidence is separated from the failed size loop; model equations, adversary capability, absolute-cadence gap, and unknown fork probability are explicit. |
| A11 — evidence or it didn't happen | PASS locally. Complete JSON, generated Markdown, environment/source hashes, deterministic regeneration, tests, and this report are durable. Origin green remains pending rather than inferred. |

### Injection, arithmetic, determinism, and scope review

- JSON decoding/encoding uses the standard library; scenario IDs are restricted to a
  non-Markdown grammar. The script contains no `eval`, `exec`, shell invocation,
  subprocess, pickle/marshal, network request, or dynamic import.
- All configured numbers and controller states are finite and bounded. Exponential
  rates remain positive; a state hitting the computational guard is counted as a
  failed run. Candidate/scenario/seed/sensitivity multiplication is checked before
  the first simulated block.
- Fixed seeds and common random streams make candidate comparisons deterministic on
  the sealed CPython 3.11.9 runtime. Cross-implementation floating reproducibility is
  not claimed.
- The staged surface contains only `.gitattributes`, the dedicated P-006 bench tree,
  and this packet report. No Rust source, workflow, Cargo manifest/lock, protocol
  spec, Lean source, vector blob, shared loop bookkeeping, Sarah-owned TBD, external
  repository, or notification was changed.

### Adversary conclusion

No Critical exists in the implemented offline tool or EOL repair. The quality-
selection result is a **protocol-design blocker if a Q-only size retarget were to be
frozen**, so the implementation path is deliberately stopped: no P-11/exact function
is proposed as conforming. That negative finding is the packet's required evidence
for the D2/G0 HUMAN review, not a reason to conceal or retry the experiment.

## 5. MERGE

Not performed by this delegated packet worker. The parent builder owns push, hosted
D6 verification, merge, exact-merge-SHA verification, and shared bookkeeping.

## 6. CALIBRATE

### Predictions versus outcomes

- **Diff surface:** the predicted report and dedicated bench tree were correct. One
  shared infrastructure file, `.gitattributes`, was added only after a written scope
  re-FRAME and parent authorization. No consensus, protocol, formal, workflow, or
  shared-bookkeeping surface changed.
- **Risk materialization:** all three predicted risks materialized. Model dependence
  required an elasticity/stage-share sweep; candidate coordinates needed repeated
  non-normative labels; and authentic but strategically selected `Q` proved capable
  of biasing the size loop. The initial numeric abort and recovery-accounting bug also
  validated the need to treat evidence tooling as adversarial code.
- **Confidence:** MEDIUM was calibrated correctly. The offline matrix and restricted
  hash-only envelope are reproducible, while the missing real SIS quality response,
  absolute cadence inputs, and propagation model prevent a full controller choice.
- **Surprise:** all 6/6 unique hash-loop settings (36/36 repeated full-grid
  coordinates) cleared isolated hash shocks, yet
  no coordinate cleared even the honest solve-power shocks across all sensitivities.
  The limiting problem is not picking a slightly different EMA; it is the observability
  and interaction of the size knob.

### Final VERIFIED / ASSUMED / UNKNOWN ledger

**VERIFIED**

- The sealed matrix contains exactly 36 coordinates, nine scenarios, 24 fixed
  sensitivity runs per coordinate/scenario, and 5,971,968 simulated blocks. Evidence:
  committed `results.json` and its SHA-256 above.
- The full-system honest/unmanipulated-quality and adversarial envelopes each have
  0/36 passes; the restricted hash-only envelope has 36/36 full-grid passes,
  representing 6/6 unique hash-loop settings. Evidence: generated
  `STABILITY-ENVELOPE.md` and complete machine rows.
- Exact-byte regeneration and all eight Python tests passed; the Rust/policy suite and
  Lean build results are enumerated above.
- The EOL repair leaves the ratified vector Git blob at SHA-256 `30CABF85...` and
  produces an LF fresh checkout under Windows autocrlf.

**ASSUMED**

- Independent exponential solution/hash stages and the stated checker-quality
  response are sensitivity models. This is safe only because no normative inference
  is made from a passing row and failure is preserved.
- The quality-selection attacker is conservatively assumed able to choose a valid
  threshold-margin solution when it wins. Whether a concrete SIS solver always
  supplies that choice is unknown; testing the capability reveals the observable's
  incentive sensitivity rather than asserting deployed exploitability.

**UNKNOWN**

- P-1 in seconds remains unresolved until a ratified P-4 solver distribution,
  production VDF delay, propagation budget, and reference-miner envelope exist.
- Real SIS effort-to-quality elasticity and strategic solution portfolios are
  unmeasured at the provisional target class.
- P-2/P-11 and exact clamps/functions remain HUMAN/G0 choices; this packet supplies no
  robust P-11 candidate.
- Same-height solution-reuse fork probability needs propagation, miner count/power,
  and withholding data.
- Hosted exact-head and exact-merge-SHA D6 remain parent-owned pending evidence.

**One process improvement for the next simulation packet:** define the total work
budget, failure-through saturation semantics, and adversarial observability test at
FRAME time, before the first matrix run. That prevents both accidental evidence
denial and late discovery that a controller's input is authentic but strategically
selectable.

## Parent integration addendum — hosted verification and merge

- PR #16 exact head `eb6b96d612bf8de4f43d6f42639da874b811bd0b` passed all
  eleven hosted D6 jobs in run
  [31971074815](https://github.com/Quigles1337/COINjecture3.0/actions/runs/31971074815).
- The expected-head guarded merge produced
  `7244f094aa22d1890039a535ad997c9775ba9ed3`; exact-merge-SHA run
  [31971438778](https://github.com/Quigles1337/COINjecture3.0/actions/runs/31971438778)
  passed all eleven jobs.
- The merge does not ratify a coordinate. The honest/full and adversarial/full 0/36
  results, 6/6 isolated hash-loop region, unfilled P-1/P-2/P-11 values, and G0/HUMAN
  size-observable decision all remain exactly as reported above.
- The narrow LF policy repair was exercised by hosted Linux CI and a fresh Windows
  autocrlf checkout; the HUMAN vector content stayed at SHA-256
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`.

## Parent closeout calibration

- **Prediction versus outcome:** the hosted and merge boundaries preserved the exact
  negative matrix, narrow EOL repair, and no-normative-choice scope.
- **Risk and confidence:** HIGH merge-readiness confidence was justified by two exact
  green heads; the result does not increase confidence in a quality-only size loop.
- **Process improvement retained:** future simulation packets must define work bounds,
  saturation semantics, and adversarial observability in FRAME before running the
  matrix.
