# COINjecture 3.0 (CJ3) — Engineering Plan

**Status:** DRAFT v0.1 — pending Al's ratification of PROPOSED/OPEN decisions at Cycle 0
**Repo home:** unratified (see D9) · initial remote `Quigles1337/COINjecture3.0`
**Companion documents:** `docs/PROTOCOL_SPEC.md` (normative once ratified), `CYCLE0_BUILDER_PROMPT.md`
**Research foundation:** "Grounding COINjecture 3.0: A Derive-Don't-Trust PoUW Survey" (Aug 2026) — cited below as RS §n

---

## §0 Mission and framing

CJ3 is a ground-up redesign of COINjecture: a Proof-of-Useful-Work L1 where the entire
vulnerability class found in the 2.0 third-party audit is **structurally unrepresentable**
rather than patched. The 2.0 audit's root disease was *trust the miner*: self-supplied
problem instances (C1), self-reported scores and solve times driving fork choice and
rewards (C2), and an apply path that believed its callers (C4/C5/C6, DARQ-021). CJ3's
design axiom — "derive it, don't read it" — is enforced at the type-system, spec, and CI
layers so that writing the 2.0 bugs again is a compile error, a spec-conformance failure,
or a red pipeline, not a code-review catch.

**What CJ3 is not (non-goals for this program):**
- No mainnet. Network allowlist is hardwired to test networks in all policy paths.
- No token economics finalization (subsidy schedule is a placeholder; owner: Al + Ken).
- No wallet (BEANlet is a separate program), no bridges, no exchange integrations.
- No SNARK/zk infrastructure at genesis (Tier 2 is trigger-gated; see D4).
- No ML-training, protein-folding, or raw random TSP/knapsack classes on the
  consensus-critical path (RS §3: trusted scorers, non-determinism, or typical-case
  easiness disqualify them). They may enter the registry later via the admission bench.
- No polyglot consensus core (see D8).
- Frontend integration is a defined Phase 4 seam, not an ongoing dependency.

---

## §1 Axioms (binding on every packet; violations are Criticals)

- **A1 — Derive, don't read.** Every consensus-relevant quantity is deterministically
  derivable by any validator from committed on-chain data, or it does not exist. No
  consensus structure contains a self-reported field. (Generalization of the GATE-3
  ruling on 2.0's C2.)
- **A2 — Protocol-generated instances.** Problem instances are derived from beacon
  randomness and chain state. Miners never author, choose, or parameterize instances.
  (Kills C1 structurally.)
- **A3 — Pure scoring.** `score/check` is a deterministic pure function of
  `(instance, solution)`. No timing inputs, no miner metadata, no `solve_time` anywhere
  in the codebase. (Kills C2 structurally.)
- **A4 — Decoupled fork choice.** Chain weight derives only from low-variance,
  hash-derived quantities. Solution quality never weights fork choice; it gates block
  *validity* and shapes *rewards* only. (RS §5: Ofelimos decoupling lesson;
  high-variance scores widen selfish-mining surface.)
- **A5 — Integer money.** All amounts are unsigned integers with checked arithmetic.
  Any `f64`/`f32` within reach of an amount is a Critical the author created. A global
  conservation invariant (Σ balances + fees burned/paid = issuance schedule) is
  CI-tested. (Ports BEANlet D11; kills C5.)
- **A6 — Apply path trusts nothing.** Every precondition is re-validated at the point
  of state mutation: strict nonce equality, explicit `from == addr(pubkey)` binding at
  the validation site (2.0 P-021 ruling carried forward), balances via checked ops,
  block-level atomic commit that re-checks its own preconditions. (Kills C4/C6/DARQ-021.)
- **A7 — Fail closed.** All state-mutating RPC is authenticated. A missing or empty
  secret at startup refuses to start the process — never falls open. Malformed input
  returns errors, never panics. (Kills C7, H8/H10 class, and the SEC-PR-001 class.)
- **A8 — Spec before code for the state machine.** The Lean 4 specification of
  transaction validity and the state-transition function is the source of truth; the
  Rust kernel conforms via generated test vectors in CI. One derivation, one place.
  (Kills the C3 divergence class; RS §6: model-based conformance, the CometBFT pattern.)
- **A9 — Minimal trusted computing base.** Single-language Rust consensus core. No
  C/C++ FFI on the apply or fork-choice paths. Heavy solvers run as untrusted external
  processes whose *output* is validated by the safe-Rust checker. `unsafe` is zero in
  consensus crates (geiger-enforced). (RS §7.)
- **A10 — Honest claims.** Hardness assumptions are labeled as assumptions in all code
  comments, docs, and public materials. The usefulness ceiling of PoUW (RS §1, Ofelimos
  ≤½) is acknowledged, never marketed around.
- **A11 — Evidence or it didn't happen.** No verification claim (tests pass, CI green,
  bench result) without an evidence pointer: CI run URL, artifact path, or committed
  report. (2.0 LEDGER D18, carried as a standing house rule.)

---

## §2 Architecture overview

```
                        ┌────────────────────────────────────────────┐
                        │                 cj3-node                   │
                        │  (binary: wiring, config, startup checks)  │
                        └───────┬──────────────┬───────────┬─────────┘
                                │              │           │
        ┌──────────────┐  ┌─────▼─────┐  ┌─────▼────┐  ┌───▼────┐
        │  cj3-solver-* │  │ cj3-rpc   │  │ cj3-net  │  │cj3-store│
        │ (out-of-proc, │  │ (authz,   │  │ (gossip, │  │(chain + │
        │  UNTRUSTED)   │  │ fail-closed│ │ bounded  │  │ state DB)│
        └───────┬──────┘  │ rate-limit)│  │ ingress) │  └───┬────┘
                │ solution └─────┬─────┘  └─────┬────┘      │
                ▼ (data only)    │              │           │
        ┌───────────────────────▼──────────────▼───────────▼──────┐
        │                      cj3-consensus                       │
        │   block validation · fork choice · difficulty retarget   │
        └───────┬───────────────────┬───────────────────┬──────────┘
                │                   │                   │
        ┌───────▼───────┐   ┌───────▼────────┐   ┌──────▼───────┐
        │  cj3-classes  │   │   cj3-beacon   │   │  cj3-kernel  │
        │ ProblemClass  │   │ VDF-hardened   │   │ tx validity +│
        │ registry; SIS │   │ randomness     │   │ STF (Lean-   │
        │ genesis class │   │ (trait-gated)  │   │ conformant)  │
        └───────┬───────┘   └────────────────┘   └──────┬───────┘
                │                                        │
        ┌───────▼────────────────────────────────────────▼───────┐
        │                       cj3-types                         │
        │  canonical encodings · addr() (THE single derivation) · │
        │            checked-amount newtypes · domains            │
        └─────────────────────────────────────────────────────────┘

        spec/   — Lean 4 (lake project): Spec/Tx.lean, Spec/Stf.lean,
                  vector exporter → JSON consumed by cj3-kernel CI tests
        bench/  — hardness admission bench + checker asymmetry bench
                  (non-consensus, may use Python for analysis/plots only)
```

**Trust boundaries.** Solvers are adversarial by construction — they communicate with
the node only by handing over candidate `Solution` bytes, which the consensus checker
validates exactly as it would a remote miner's block. The node never links solver code.
RPC and net are hostile-input surfaces: fuzz targets required on every codec (H-class).

**Data flow (happy path).** parent block → `cj3-beacon` output → instance seed →
`cj3-classes::derive_instance` → solver (out-of-process) → solution → miner assembles
block (txs from mempool, solution commitment in header) → hash-eligibility race →
gossip → every validator independently: re-derive instance, `check()` solution,
re-validate every tx at apply, recompute state root → fork choice on hash weight only.

---

## §3 Decision log

Statuses: **ACCEPTED** (binding now) · **PROPOSED** (my recommendation; Al ratifies at
Cycle 0 STOP) · **OPEN** (Al owns; loop pauses on dependent packets until ruled).
Deciders: Al (all); Sarah where noted, once/if engaged per D10.

| ID | Status | Decision |
|----|--------|----------|
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

### D1 (ADR) — Genesis problem class

**Context.** COINjecture's identity is "NP-hard solutions replace hash mining," and the
2.0 registry spans multiple `problem_type`s. But RS §2's central finding is that
*random ≠ hard*: random instances of most NP-hard problems are easy outside narrow
phase-transition bands, and the only family with **provable** average-case hardness from
a worst-case assumption is lattices (Ajtai's SIS reduction). Under A2, the protocol —
not the miner — samples instances, so the sampled distribution's hardness is the
network's entire security.

**Options.**
- **(a) Lattice SIS as genesis anchor** — provable worst-case→average-case hardness;
  checker is a mod-q matrix-vector product + norm bound, near-linear, metadata-free
  (perfect A3 fit). Cons: narrow external "usefulness" of short vectors; departs from
  2.0's visible problem set; reward shaping over norm margins needs care (P-003).
- **(b) Planted SAT / planted clique, protocol-planted via beacon** — familiar identity,
  trivial verification. Cons: hardness is empirical, not proven; parameter floors must
  clear known quasi-polynomial attacks (RS §2 planted-clique caveat); phase-transition
  sampling is an assumption dressed as a dial.
- **(c) Carry 2.0's classes forward as-is** — maximum continuity. Cons: exactly the
  distributions RS §2 flags as typically easy; would rebuild 2.0's C1 exposure with
  better plumbing.

**Decision (proposed).** (a) as the genesis consensus anchor, with the trait registry
preserving the multi-class identity: legacy and planted classes onboard **only** after
passing the P-004 admission bench (published hardness assumption + parameter floor +
measured solve/verify asymmetry + checker cost bound). The registry — a growing,
audited catalog of useful problems — *is* COINjecture; the anchor is just the class the
chain's security leans on. Reward-curve shaping over quality margins is the natural
slot for existing COINjecture mathematics (μ-balance normalization) — owner Al + Sarah,
explicitly not agent-inventable (A11 discipline: no invented parameters).

**Consequences.** Easier: security argument, Lean-friendly checker, tiny audit surface.
Harder: community story ("why lattices first") — mitigated by the registry framing and
D13 honesty. Revisit: if P-004 shows a planted class clearing the bench with margin,
promote it to co-anchor via ADR.

### D2 (ADR) — Consensus skeleton

**Context.** RS §5 compares (a) longest-chain by deterministic work, (b) useful work as
admission lottery into BFT finality, (c) conventional chain + compute marketplace — and
its Stage-3 recommendation is (b), as minimizing *theoretical* audited surface.

**Deviation and why.** For DARQ, audited surface means **code we write and must review
with a two-seat team**. A from-scratch BFT layer (view changes, locking, evidence,
committee PKI) is a large new implementation and its own audit program; 2.0's mental
model, tooling, and Sarah's existing proofs are Nakamoto-shaped. The plan therefore
proposes **(a′): longest-chain where block *eligibility* is a hash race (low-variance,
fork-choice-weighted) and block *validity* requires a checked solution to the
beacon-derived instance meeting threshold θ, with rewards shaped by checker-derived
quality**. This imports Ofelimos's actual security lesson — decouple block success from
solution quality (A4) — without importing its SNARG (unneeded: Tier-0 checkers are
cheap, D4) or a BFT stack.

**Options considered.**

| Dimension | (a′) Decoupled longest-chain | (b) Lottery + BFT | (c) Marketplace |
|---|---|---|---|
| New code DARQ must audit | Small (retarget + weight fn) | Large (full BFT) | Medium (settlement games) |
| Continuity with 2.0 & Lean work | High | Low | Low |
| Finality | Probabilistic | Fast/absolute | Chain-dependent |
| "PoUW purity" | Work gates validity | Work gates entry | Work is a paid service |
| Known formal grounding | Ofelimos (CRYPTO'22) pattern | Hybrid-consensus literature | Filecoin/rollup patterns |

**Decision (proposed).** (a′). **Promotion criteria to (b):** if the network later needs
fast finality, or P-006 simulation shows the two-knob retarget (hash target × instance
size) cannot be stabilized, open an ADR to bolt a finality gadget onto (a′) rather than
rebuild. (c) remains the honest fallback if the usefulness/security tension (RS caveats)
proves unresolvable for any admitted class.

**Consequences.** Easier: smallest consensus codebase, direct reuse of 2.0 remediation
learnings (the fork work on 2.0 is literally this validation path, done right). Harder:
two interacting difficulty knobs (named risk R2, spiked in P-006); probabilistic
finality. Revisit at Phase 2 gate with P-006 data in hand.

---

## §4 Phase plan

Each phase ends at a **gate**: builder ships a phase report with evidence pointers
(A11); Al reviews and rules before the next phase's packets unblock. Estimates are
agent-loop cycles, not calendar promises.

**Phase 0 — Foundations & spikes (packets P-001…P-007).**
Repo + CI scaffold green on an empty workspace; beacon tech selection; SIS
parameterization and asymmetry bench with *measured* numbers; admission bench harness;
Lean spec of Tx validity with vector exporter; difficulty two-knob simulation;
`cj3-types` with the single `addr()` derivation and codec fuzz targets.
**Gate G0:** all spike reports in `loop/reports/`, PROPOSED decisions D1/D2/D14/D15
ratified or revised with spike evidence, OPEN D9/D10 ruled.

**Phase 1 — Kernel (P-101…P-104).**
`cj3-kernel`: tx validity V1–V9 and STF implemented against Lean vectors (conformance
test in CI, red = blocked merge); `cj3-store` chain/state DB; genesis file format +
the genesis spend-test in CI (§6). **Gate G1:** kernel conformance green; property
tests for conservation invariant; mutation-testing spot check on validity predicate;
and a crash-consistency test that kills the process mid-block-apply and requires
`cj3-store` to recover to the exact pre-block state on restart.

**Phase 2 — Consensus (P-201…P-205).**
`cj3-beacon` (ratified tech), instance derivation, `cj3-classes` SIS wiring, block
validation, fork choice, difficulty retarget per P-006's ratified model.
**Gate G2:** two-node deterministic replay test (same blocks → same state root);
adversarial block corpus (malformed solutions, wrong-instance solutions, replayed
solutions, overflow txs, future nonces) all rejected with typed errors, zero panics;
and a C2 provenance check proves the reward input is exactly the output of
`check(derive_instance(instance_seed, size_param), solution)` for the validated block.
The G2 corpus MUST attempt to spoof a block-supplied quality value and verify that no
such field can influence `Context.rewardInputs` or the credited reward.

**Phase 3 — Network & RPC (P-301…P-304).**
`cj3-net` bounded ingress (size/rate caps, typed decode errors), `cj3-rpc` with
fail-closed auth from the first endpoint, fuzz targets on all wire codecs.
**Gate G3:** devnet of ≥3 local nodes survives the adversarial corpus replayed over
the wire; auth matrix test (every mutating endpoint × no-token/bad-token/empty-secret).

**Phase 4 — Testnet & integration seam (P-401…).**
Public testnet genesis ceremony (allocations file → hash into genesis header →
CI spend-test against the real file); frontend integration surface (read APIs +
event streams) specified and stabilized — **this is the designed Sarah/frontend
entry point**; observability. **Gate G4:** testnet up N days with the solver fleet
external; incident-free adversarial replay; THEN and only then any 2.0→3.0
migration conversation begins (explicitly out of scope until here).

---

## §5 CI pipeline (D6, from the first commit)

Jobs (all blocking): `fmt` → `clippy -D warnings` → `cargo deny check` (committed
`deny.toml`: licenses, bans, advisories) → `cargo audit --deny warnings` →
`cargo geiger` zero-unsafe assertion on consensus crates → unit + property tests
(conservation invariant lives here) → Lean vector conformance (Phase 1+) → codec fuzz
smoke (bounded-iteration libFuzzer/cargo-fuzz targets; long fuzz runs nightly) →
genesis spend-test (Phase 1+) → build with `Cargo.lock` frozen (`--locked`).
Branch protection: no merge on red; no direct pushes to `main`; PRs carry evidence
pointers in the description (A11). Nightly: full fuzz corpus, `cargo update --dry-run`
advisory diff report (dependency drift visibility without auto-bumping).

---

## §6 Threat model — 2.0 audit class → CJ3 structural kill

| 2.0 finding class | CJ3 defense | Enforced at |
|---|---|---|
| C1 miner-supplied instance never regenerated | Instances derived from beacon + parent hash; miners cannot author instances at all (A2) | Type system (no instance field in submissions) + spec |
| C2 self-reported `work_score` / `solve_time` in fork choice & rewards | `check(instance, solution) → Quality` pure function; no timing fields exist (A3); quality never weights fork choice (A4) | Trait signature + spec + grep-gate in CI for banned field names |
| C3 address derivation mismatch across components | Exactly one `addr()` in `cj3-types`, Lean-specified; **genesis spend-test in CI** — every build literally spends from a genesis allocation in a test | Spec + CI |
| C4 nonce unchecked / replay | Strict `tx.nonce == account.nonce` equality in V-rules, re-checked at apply (A6) | Lean spec V-rules + kernel + conformance vectors |
| C5 unchecked balance arithmetic | `u64` newtypes, `checked_*` only, conservation property test (A5/D7) | Type system + CI property test |
| C6 atomic commit trusts caller | Commit path re-validates all preconditions internally; no "pre-validated" fast path exists (A6) | Kernel API design (no bypass constructor) + spec |
| C7 unauthenticated state-mutating RPC | Fail-closed auth; empty secret refuses startup (A7) | `cj3-node` startup checks + G3 auth matrix test |
| DARQ-021 `from` ≠ `addr(pubkey)` | Explicit binding check `from == addr(pubkey)` at the validation site, as its own V-rule (2.0 ruling carried) | Lean spec + kernel + adversarial corpus |
| H3/H4/H5/H11 panics on malformed input, IDOR, log flood | Typed decode errors everywhere, bounded ingress, rate limits, fuzz targets on all codecs | `cj3-net`/`cj3-rpc` + fuzz CI |
| H8/H10 fail-open auth paths | A7 fail-closed doctrine; auth matrix test enumerates every mutating endpoint | G3 gate |
| (2.0 process finding) dependency-audit blind spot | D6 gates from first commit; nightly advisory drift report | CI |
| (2.0 process finding) agent false "CI green" claim | A11 evidence-pointer rule; PR template requires run URLs | Loop protocol |

## §7 Risk register

- **R1 — Instance-hardness assumption (any non-lattice class).** Outside SIS there is
  no proven hard sampleable distribution (RS §2). Mitigation: admission bench (P-004)
  with published parameter floors; D13 honesty; anchor stays SIS until an ADR says
  otherwise.
- **R2 — Two-knob difficulty interaction.** Hash target (rate/variance) × instance
  size (work calibration) may oscillate. Mitigation: P-006 offline simulation before
  any Phase 2 freeze; promotion criteria in D2 if unstabilizable.
- **R3 — Pure-Rust VDF availability.** chiavdf is C++ (A9-forbidden on trusted path).
  Mitigation: P-002 spike; trait-gated placeholder for devnet only; worst case,
  beacon ships later than consensus scaffold without blocking Phase 1.
- **R4 — Solo-review bottleneck.** One human reviewer for consensus code. Mitigation:
  Lean vectors act as a mechanical second reviewer for the kernel; adversarial corpus
  as regression reviewer; D10's Sarah seam for the spec layer.
- **R5 — Capacity bleed from 2.0.** The 2.0 coordinated fork (DARQ-021+C1+C2) is the
  company's actual critical path. Mitigation: D11 hard rule; CJ3 loop yields whenever
  2.0 unblocks.
- **R6 — Beacon grinding edges.** Seed derivation must exclude miner-influencable
  low-cost fields (timestamp excluded by spec); analysis section in SPEC §9 tracks
  residual grinding vectors; P-002 report must address last-block-withholding.
- **R7 — Reward-shaping exploitability.** Quality normalization can still create
  variance incentives even though it cannot increase issuance above the subsidy.
  Mitigation: the Model 4 curve is deterministic and bounded in
  `[floor(subsidy/R_MAX), subsidy]`, with `R_MAX ≥ 1` acting as a quality span divisor
  rather than an inflation multiple (SPEC §11). The `R_MAX` value and curve shaping,
  including any μ-balance normalization, remain owned by Al (+ Sarah per D16) and are
  never agent-invented.

## §8 Loop protocol binding

The DARQ AGI Mode loop governs execution: `loop/STATE.md` (CYCLE/PHASE/PACKET/BRANCH/
CAPACITY_FLAG), `loop/PACKETS.md` (queue), `loop/LEDGER.md` (standing decisions incl.
D1–D15 with statuses), `loop/reports/` (evidence). Builder seat has empirical repo
access; reports are other seats' ground truth. **Al approves the packet queue before
any code (Cycle 0 STOP).** Branch naming `feat/pNNN-slug`; draft PRs; ferry reports
back; no merge without Al. D11 capacity rule is checked at every packet pickup.

## §9 Packet seed queue (Cycle 0 seeds these into PACKETS.md)

- **P-001** Repo scaffold: workspace, crate skeletons per §2, full D6 CI green on the
  empty workspace, branch protection config documented. *No protocol code.*
- **P-002** Beacon spike (read/report): survey pure-Rust VDF options (Wesolowski,
  Pietrzak, class-group vs RSW), grinding analysis incl. withholding; recommend; define
  `Beacon` trait; devnet placeholder behind trait with `NOT-TESTNET-GRADE` banner.
- **P-003** SIS genesis class: implement `ProblemClass` for SIS; parameter search
  (n, m, q, β candidates — **discovered, not invented**, per A11) targeting the block
  cadence; measure solve/verify asymmetry with the out-of-process solver; report
  measured numbers + recommended floors. Reward-margin analysis flagged for Al+Sarah.
- **P-004** Admission bench harness: generic runner that takes any `ProblemClass`
  candidate and produces the hardness/asymmetry report; run 2.0's legacy classes
  through it first as calibration (read-only with respect to consensus).
- **P-005** Lean scaffold: lake project; `Spec/Tx.lean` encoding V1–V9 from SPEC §7;
  `Spec/Stf.lean` skeleton; JSON vector exporter (`lake exe vectors`). *Designed
  Sarah entry seam.* Conformance test lands with P-101, not here.
- **P-006** Difficulty simulation (offline, no chain code): model the two-knob
  retarget under honest/adversarial hash- and solve-power schedules; report stability
  envelope; feeds the D2 Phase-2 gate.
- **P-007** `cj3-types`: canonical encodings, domain tags, the single `addr()`,
  amount newtypes, codec fuzz targets wired into CI.
- **P-101** (Phase 1 head, blocked on G0) Kernel: V1–V9 + STF vs Lean vectors.

## §10 References

RS = research survey artifact (this repo `docs/RESEARCH_SURVEY.md` once committed).
Primary anchors: Ofelimos (Fitzi–Kiayias–Panagiotakos–Russell, CRYPTO 2022; ePrint
2021/1379) and FRLS follow-up (ePrint 2025/2091); Merlina–Garrett–Vitenberg PoUW SoK
(arXiv:2404.15735); Ball–Rosen–Sabin–Vasudevan (STOC 2017) fine-grained PoW; Ajtai
worst-case→average-case SIS; Wesolowski (EUROCRYPT 2019) / Pietrzak (ITCS 2019) VDFs;
CometBFT TLA+ model-based conformance pattern; IronFleet; "Rewrite it in Rust
Considered Harmful?" (UCSD HotOS) on C-Rust FFI; Arbitrum interactive fraud proofs.

---
*CJ3 Engineering Plan v0.1 — DARQ Labs LLC — prepared for Cycle 0 handoff.*
