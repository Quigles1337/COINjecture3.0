# Cycle 6 — P-010 Builder Report

**Status:** IN PROGRESS — FRAME COMPLETE
**Date:** 2026-08-16
**Packet:** P-010 — DOCUMENTARIAN / institutional README
**Lane:** AUTO
**Branch:** `feat/p010-institutional-readme`

## 1. FRAME

### Packet and done-condition

P-010 replaces the stale P-001 root README with a diligence-oriented index to the
current repository record. It must state the audit lineage before design claims,
separate implemented evidence from governing design and planned work, condense A1–A11,
map the intended crate architecture, explain the loop and verification record, expose
open assumptions and every material TBD with its owner, provide only commands that run
now, and mark the current position before Gate G0. The README status block and roadmap
marker also become mandatory gate-closeout surfaces through a standing LEDGER rule.

The packet is done only when:

- `README.md` is GitHub-flavored Markdown, remains below roughly 400 lines, includes a
  table of contents, and uses Mermaid only for the architecture relationship;
- every present-tense capability claim has a committed path, report, CI run, or
  specification pointer, while planned work, assumptions, and TBD ownership remain
  explicit;
- one status block states private/pre-testnet/no-mainnet scope, the current phase, the
  last phase gate, what is implemented, and what is planned;
- provenance starts with the COINjecture 2.0 audit failure and points to
  `docs/AUDIT_TRACEABILITY.md` without claiming P-009 source reconciliation is done;
- quickstart contains only commands executed successfully against this packet head;
- the adversary pass removes sales language and any claim the current code cannot
  support;
- the live P-010 queue ruling and standing README refresh rule are durable in
  `loop/LEDGER.md` and `loop/PACKETS.md`;
- local checks and exact-head D6 pass before any AUTO merge, followed by exact-merge-
  SHA mainline D6 evidence.

### Lane classification against the five D17 AUTO conditions

**Classification: AUTO.**

1. **Approved and unblocked:** Al's live 2026-08-16 governance injection expressly
   approves P-010, identifies it as AUTO eligible, and directs processing at this
   packet boundary. P-005 remains stopped and unmerged on its HUMAN branch; P-010 is
   based independently on `main` and does not import formal content.
2. **Bounded non-semantic surface:** the deliverable and supporting loop edits are
   Markdown only. They describe existing code, governing design, and planned seams;
   they do not change a V-rule, STF, checker, beacon, fork-choice, difficulty, reward,
   network, RPC, or storage behavior.
3. **No formal-spec or vector content:** no `spec/` file, Lean term, or protocol-vector
   definition is in scope. The README must say that those artifacts do not yet exist.
4. **No decision invention or ratification:** the only new governance is the live
   standing rule supplied by Al. The packet records it without assigning a new
   decision number or inferring any additional authority.
5. **No Al-/Sarah-owned TBD fill:** the exact license line remains
   `License: TBD — owner: Al/Ken`; economics, parameter, reward, encoding, and formal-
   semantics TBDs remain unfilled with their recorded owners.

### Predicted diff surface

- `README.md` — complete institutional rewrite and current evidence index.
- `loop/LEDGER.md` — P-010 approval overlay and the exact standing README refresh rule.
- `loop/PACKETS.md` — appended P-010 row/definition and factual P-005 stop pointer.
- `loop/STATE.md` — packet-boundary state after guarded completion.
- `loop/reports/C6-p010-builder.md` — FRAME through CALIBRATE evidence.
- `loop/reports/BATCH-LOG.md` — one P-010 heartbeat row after merge evidence exists.

No source, manifest, lockfile, workflow, formal-specification file, audit source, or
other document is predicted to change. P-005's branch and draft PR remain untouched.

### Authority-and-claims pre-check

- The thesis is a governing design consequence, not a claim that Phase 2 consensus
  code exists: miners have no instance-authoring or timing input in the specified
  trait, validator quality is a pure checker output, and quality is absent from the
  specified fork-choice function. The README will label implementation as planned.
- A successful D6 rollup does not mean deferred phase jobs executed. The README will
  tell readers to inspect each job for `NOT_YET_ADMITTED` and identify the owner.
- P-003 produced a deterministic sampled-SIS prototype and provisional parameter
  evidence; it did not prove concrete finite-parameter hardness or ratify P-3/P-4.
- P-004's fast fixture checkers did not establish sampled hardness or admit a legacy
  class. Its negative result is part of the evidence, not a defect to hide.
- `docs/AUDIT_TRACEABILITY.md` remains an intent-coverage matrix compiled from the
  remediation record. The source audits are preserved, but P-009 has not interpreted
  or reconciled them, so the README cannot call the matrix complete.

### Top risks

1. **Design/implementation tense leakage:** prose could describe a specified future
   crate or control as shipped. The status block, architecture legend, and per-claim
   pointers must keep those categories separate.
2. **Green-job overclaim:** D6 includes phase-gate jobs that currently pass by explicit
   deferral. The verification section must distinguish active tests from deferred
   handlers and point to the gate script.
3. **Audit-resistant marketing:** terms such as “secure,” “proven,” “production,” or
   broad usefulness language could outrun the repo. The adversary pass will require a
   proof pointer or remove/rewrite each such sentence.

**Falsifier:** this approach is wrong if a status or capability sentence cannot be
traced to a committed artifact or immutable run, if any quickstart command fails on
the packet head, or if the document makes planned design read as running protocol
behavior. Any such case requires correction before origin verification.

**Confidence:** MEDIUM. The repository has unusually detailed evidence, but several
governing documents intentionally describe future phases and therefore create a high
risk of grammatically subtle overclaiming.

## Initial evidence ledger

### VERIFIED

- Canonical checkout, exact origin, private visibility, authenticated GitHub CLI,
  clean branch point, and `main == origin/main ==
  1f379308ddba03d41774cf9d35ea6b27c5795fb0` were read back during P-010 preflight.
- P-001 through P-004 are merged with exact-mainline D6 URLs in
  `loop/reports/BATCH-LOG.md`; P-005 is an unmerged HUMAN stop in draft PR #9, whose
  checkpoint commit passed D6 run 31955875423.
- The required Engineering Plan, Protocol Spec, audit matrix, research survey,
  decision ledger, batch log, and latest C4/C5 packet reports were read before the
  README draft.
- Current code inspection found an implemented beacon trait/devnet-only placeholder,
  sampled-SIS derive/check boundary, external demonstrator, and admission bench; the
  kernel, consensus, store, net, RPC, node, and types crates remain empty boundaries.
- The 2.0 capacity file still reports `CAPACITY_FLAG: none`; GATE-1/GATE-2 remain in
  its awaiting table, satisfying the specific live D11 continuation condition for
  this packet.

### ASSUMED

- Authenticated private-repository viewers can resolve the D6 workflow badge, as Al's
  packet specification explicitly permits that badge class. No public-view behavior
  is assumed.

### UNKNOWN

- The final P-010 head SHA, pull-request number, exact-head D6 URL, merge SHA, and
  exact-merge-SHA D6 URL do not exist yet. They will be recorded only after GitHub
  reports each value.
- P-009's finding-by-finding source reconciliation remains undone; the README must not
  infer its outcome from source preservation alone.

## 2. BUILD — PENDING

## 3. VERIFY — PENDING

## 4. ADVERSARY PASS — PENDING

## 5. MERGE — PENDING

## 6. CALIBRATE — PENDING
