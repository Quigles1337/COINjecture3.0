# COINjecture 3.0 Packet Queue

**Queue status:** STOPPED / IDLE AT GATE G0 HOLD — G0-A is RATIFIED: `size_param` is a
static human-upgraded protocol constant, P-11 is struck as moot, P-1 remains unfilled
with no proposal, and P-2 is provisionally ratified as `W=32`, gain `1/8`, clamp
`[8/9,9/8]`, subject to re-derivation when P-1 is ratified. G0-B through G0-E remain
held. P-007 is HUMAN / INCOMPLETE at the canonical-codec and open-SI tripwire; its
checked-amount checkpoint remains draft PR #14 and MUST NOT merge. P-008 remains
blocked on the exact frontend URL. P-101 is blocked on Gate G0 only. No held or owner-controlled
value was inferred.

| Packet | Phase | Queue status | Blocking condition |
|--------|-------|--------------|--------------------|
| P-001 | Phase 0 | COMPLETE — `8367de08` | PR #1 and exact-merge-SHA D6 CI green |
| P-002 | Phase 0 | COMPLETE — `7ecba896` | PR #3 and exact-merge-SHA D6 CI green |
| P-003 | Phase 0 | COMPLETE — `e0056157` | PR #5 and exact-merge-SHA D6 CI green |
| P-004 | Phase 0 | COMPLETE — `4644374f` | PR #7 and exact-merge-SHA D6 CI green |
| P-005 | Phase 0 | COMPLETE — HUMAN-RATIFIED — `916e5027` | PR #9 and exact-merge-SHA D6 green |
| P-006 | Phase 0 | COMPLETE — `7244f094`; G0-A applied | PR #16 exact-merge D6 green; static `size_param` ratified, P-11 moot, P-1 unfilled, P-2 provisional |
| P-007 | Phase 0 | HUMAN STOP / INCOMPLETE — draft PR #14, `9acea83c` | G0 must rule canonical codec/hash/domain/P-8/SI-002/SI-003/strict Ed25519; SI-001 also remains open |
| P-008 | Phase 0 / Phase 4 seam input | BLOCKED | Exact public frontend repository URL supplied by Al; URL MUST NOT be guessed |
| P-009 | Phase 0 / audit traceability | COMPLETE — `ce0b9752` | PR #15 and exact-merge-SHA D6 green; GAP-7–13 remain non-ratified |
| P-010 | Phase 0 / documentation | COMPLETE — `1e986cb2` | Feature PR #10 and closeout PR #11; both exact-merge-SHA D6 runs green |
| P-101 | Phase 1 head | BLOCKED | Gate G0 only |

## Seed packet definitions

The P-001 through P-007 and P-101 definitions below are seeded from
`docs/ENGINEERING_PLAN.md` §9.

### P-001 — Repo scaffold

Workspace, crate skeletons per Plan §2, full D6 CI green on the empty workspace,
branch protection config documented. **No protocol code.**

### P-002 — Beacon spike

Read/report: survey pure-Rust VDF options (Wesolowski, Pietrzak, class-group vs RSW),
grinding analysis including withholding; recommend; define `Beacon` trait; devnet
placeholder behind trait with `NOT-TESTNET-GRADE` banner.

### P-003 — SIS genesis class

Implement `ProblemClass` for SIS; parameter search (n, m, q, β candidates —
**discovered, not invented**, per A11) targeting the block cadence; measure
solve/verify asymmetry with the out-of-process solver; report measured numbers and
recommended floors. Reward-margin analysis is flagged for Al + Sarah.

### P-004 — Admission bench harness

Generic runner that takes any `ProblemClass` candidate and produces the
hardness/asymmetry report; run 2.0's legacy classes through it first as calibration
(read-only with respect to consensus).

### P-005 — Lean scaffold

Lake project; `Spec/Tx.lean` encoding V1–V9 from Protocol Spec §7;
`Spec/Stf.lean` skeleton; JSON vector exporter (`lake exe vectors`). **Designed Sarah
entry seam.** Conformance test lands with P-101, not here. The 2026-08-16 HUMAN-lane
authorization permits builder implementation only within the symbolic/SI boundary in
`loop/LEDGER.md`; Al's completed second review and PR #9 merge are its done-condition.

Pickup on 2026-08-16 classified the whole packet HUMAN because the done-condition
directly requires both `Spec/*.lean` content and protocol-vector definitions, each
named by D17 as HUMAN lane. Al reviewed the complete encoding twice, returned F1/F2/F3
after the first review, and accepted all three as closed after remediation. PR #9
merged at `916e5027f0918973bf45d6a2bf90abd2ae253197`; exact-head run `31967696333`
and exact-merge-SHA run `31968030240` both passed all eleven jobs. The V1–V9/STF
encoding is HUMAN-RATIFIED. The vector artifact remains explicitly non-normative and
symbolic pending SI-001/SI-002/SI-003.

### P-006 — Difficulty simulation

Offline, no chain code: model the two-knob retarget under honest/adversarial hash- and
solve-power schedules; report stability envelope; feeds the D2 Phase-2 gate.

**Outcome:** COMPLETE through PR #16 at
`7244f094aa22d1890039a535ad997c9775ba9ed3`; exact-head run `31971074815` and
exact-merge-SHA run `31971438778` passed all eleven jobs. The sealed 5,971,968-block
matrix found 0/36 full honest/unmanipulated-quality passes and 0/36 full adversarial
passes. All 6/6 unique isolated hash-loop settings passed, but no tested size-window
setting survived the solve-power/interaction sensitivities. G0-A subsequently
ratified the static human-upgraded `size_param` option and struck P-11 as moot. P-1
and P-2 now apply only to the hash-target controller. P-1 remains unfilled and no
absolute cadence is proposed. P-2 is provisionally ratified as `W=32`, gain `1/8`,
clamp `[8/9,9/8]`: a conservative interior point, not a tuned optimum, which MUST be
re-derived when P-1 is ratified. It is not yet implemented.

### P-007 — `cj3-types`

Canonical encodings, domain tags, the single `addr()`, amount newtypes, codec fuzz
targets wired into CI.

**Outcome:** HUMAN STOP / INCOMPLETE. Draft PR #14 exact head
`9acea83ca17d67a19e0d41aeb5e7275666a54013` passed all eleven jobs in run
`31969740766` and preserves only the independently specified checked `Amount(u64)`
boundary. It remains unmerged. The complete packet requires G0/HUMAN choices for the
canonical codec grammar; SHA-256 confirmation; exact bytes/framing for all seven
domain tags; P-8 byte limits; SI-002 signed-coefficient representation and bound
wording; SI-003 SHAKE consumption; and GAP-12 strict Ed25519 behavior. The active
codec-fuzz job still reports `NOT_YET_ADMITTED`; green is not a fuzz-success claim.

### P-008 — Frontend contract extraction

Read-only, external: survey Sarah's public frontend repository at
`TBD(Al-supplied frontend URL)`; extract routes, payload shapes, event/stream
expectations, and auth assumptions into `docs/FRONTEND_SEAM.md`. No forks, no PRs, no
issues, no watches, and no action that generates a notification on Sarah's side. This
packet informs the Phase 4 seam; Protocol Spec §12's read-API surface MUST NOT
contradict it.

The legacy repository
`https://github.com/COINjecture-Network/COINjecture2.0` is explicitly **not** treated as
the missing frontend URL. Agents MUST NOT guess the frontend URL (A11).

### P-009 — Audit traceability verification

Read-only verification against the source audit documents: enumerate every finding
ID/class, diff it against `docs/AUDIT_TRACEABILITY.md`, report every unmapped or
mis-mapped item, and propose matrix v0.2. The third-party security audit and optional
Lean audit are first copied byte-for-byte from the supplied Downloads paths into
`loop/evidence/`; the Lean audit is checked specifically for anything bearing on
CJ3's A8 track. The Codex security-scan source remains outstanding, but P-009 does
not wait if exact committed 2.0 remediation records fully represent its findings and
are cited as substitutes. No `src/` change is authorized.

**Outcome:** COMPLETE through PR #15 at
`ce0b9752c9101e89f93a90379cd9e5ac1a08842d`; refreshed exact-head run `31971741833`
and exact-merge-SHA run `31972017916` passed all eleven jobs. Matrix v0.2 accounts for
33/33 third-party findings (17 mapped, 10 partial, six surface-excluded), all 25 Lean
claim-points, and R1–R8. GAP-7 through GAP-13 are proposals only. The absent original
Codex report's exact IDs/titles/severities/count remain UNKNOWN; exact committed CJ2
remediation pointers preserve the known five program families and two finding IDs
without pretending to replace the missing source inventory.

### P-010 — Institutional README

Replace the repository-root `README.md` with a GitHub-flavored, diligence-oriented
entry document below roughly 400 lines. It leads with the 2.0 audit lineage, derives
the structural response without claiming unimplemented consensus behavior, keeps one
current status block, condenses A1–A11, maps the planned crate architecture, explains
governance and evidence inspection, provides only currently working commands, states
limitations and owned TBDs, and marks the current G0–G4 position. A D6 badge is the
only permitted badge class; no drifting version badge is permitted.

The packet is docs-only and AUTO eligible. It may edit the README and its own durable
governance/report/bookkeeping surfaces, but no source, workflow, formal specification,
protocol vector, consensus semantic, parameter, or license decision. Its adversary
pass removes or points every unsupported capability sentence. The README status block
and roadmap marker become mandatory refresh surfaces at every gate closeout under the
standing rule in `loop/LEDGER.md`.

### P-101 — Kernel

Phase 1 head, blocked on G0 only: V1–V9 + STF versus Lean vectors. Its concrete
authenticated store MUST discharge all three
`LawfulStateOps` laws from Protocol Spec §8 and prove that the kernel's actual storage
reads and writes are coherent with them. Its `Context.rewardInputs` wiring MUST
consume only the Q returned by
`check(derive_instance(instance_seed, size_param), solution)` for the validated block
and MUST expose no block-supplied quality path; this is the binding C2 structural
boundary and is re-checked at Gate G2. The concrete `R_MAX·SCALE` divisor MUST be
constructed with checked u128 multiplication, proved nonzero before floor division,
and the result checked back to u64.

## Pickup guard

Every pickup MUST re-read `loop/STATE.md`, check this repository's `CAPACITY_FLAG`, and
re-check the live D11 condition. Current authorization exists only while COINjecture
2.0 remains blocked pending Sarah's GATE-1/GATE-2 answers. `remediation-priority` in
either project, or evidence that those gates cleared, pauses CJ3 immediately.
