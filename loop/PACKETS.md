# COINjecture 3.0 Packet Queue

**Queue status:** APPROVED WITH RATIFIED P-005 REVIEW TAIL — Al live rulings,
2026-08-15 through 2026-08-16. P-005 implementation is HUMAN-authorized under the
exact constraints in `loop/LEDGER.md`, but PR #9 MUST stop ready-for-review and MUST
NOT auto-merge. While that review tail is open, P-006, P-007, P-009, and P-010 (if
present) may run out of strict order under their own tripwires and D11 checks. P-008
remains blocked on the exact frontend URL. P-101 remains blocked on both Gate G0 and
the merged, human-reviewed P-005.

| Packet | Phase | Queue status | Blocking condition |
|--------|-------|--------------|--------------------|
| P-001 | Phase 0 | COMPLETE — `8367de08` | PR #1 and exact-merge-SHA D6 CI green |
| P-002 | Phase 0 | COMPLETE — `7ecba896` | PR #3 and exact-merge-SHA D6 CI green |
| P-003 | Phase 0 | COMPLETE — `e0056157` | PR #5 and exact-merge-SHA D6 CI green |
| P-004 | Phase 0 | COMPLETE — `4644374f` | PR #7 and exact-merge-SHA D6 CI green |
| P-005 | Phase 0 | IN PROGRESS — HUMAN AUTHORIZED / REVIEW TAIL | Implement on draft PR #9 within the ratified symbolic/SI boundary; adversary + exact-head green CI; mark ready; no builder merge |
| P-006 | Phase 0 | QUEUED — APPROVED / OUT-OF-ORDER PERMITTED | P-005 PR #9 ready-for-review and D11 re-check |
| P-007 | Phase 0 | QUEUED — APPROVED / OUT-OF-ORDER PERMITTED | P-005 PR #9 ready-for-review and D11 re-check; its own semantic tripwires remain armed |
| P-008 | Phase 0 / Phase 4 seam input | BLOCKED | Exact public frontend repository URL supplied by Al; URL MUST NOT be guessed |
| P-009 | Phase 0 / audit traceability | QUEUED — APPROVED / UNBLOCKED / INTERPRETATION AUTHORIZED | Source evidence copied durably; may execute while P-005 awaits review after its own D11 re-check |
| P-010 | Phase 0 / documentation | COMPLETE — `1e986cb2` | Feature PR #10 and closeout PR #11; both exact-merge-SHA D6 runs green |
| P-101 | Phase 1 head | BLOCKED | Gate G0 **and** merged, human-reviewed P-005 |

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
`loop/LEDGER.md`; PR #9 stops ready-for-review and Al's merge is its done-condition.

### P-006 — Difficulty simulation

Offline, no chain code: model the two-knob retarget under honest/adversarial hash- and
solve-power schedules; report stability envelope; feeds the D2 Phase-2 gate.

### P-007 — `cj3-types`

Canonical encodings, domain tags, the single `addr()`, amount newtypes, codec fuzz
targets wired into CI.

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

Phase 1 head, blocked on G0 **and** a merged, human-reviewed P-005: V1–V9 + STF versus
Lean vectors.

## Pickup guard

Every pickup MUST re-read `loop/STATE.md`, check this repository's `CAPACITY_FLAG`, and
re-check the live D11 condition. Current authorization exists only while COINjecture
2.0 remains blocked pending Sarah's GATE-1/GATE-2 answers. `remediation-priority` in
either project, or evidence that those gates cleared, pauses CJ3 immediately.
