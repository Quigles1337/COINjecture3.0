# CJ3 Audit Traceability Matrix — v0.1

**Purpose:** answer, with evidence rather than assurance, whether every shortcoming from
the two COINjecture 2.0 audits is addressed in the CJ3 governing documents.
**Sources:** (1) the third-party COINjecture 2.0 security audit (7 Criticals +
High/Medium/Low tables); (2) Al's Codex security scan (root-cause collapse);
(3) the 2.0 remediation-program rulings and integrity incidents (loop LEDGER).
**Epistemic status:** compiled from the remediation-program record, not re-read from the
audit source documents. GAP-6 / packet P-009 closes that loop. Until P-009 reports,
this matrix is evidence of *intent coverage*, not proof of *complete coverage*.

---

## §1 Finding-class matrix

| Finding (source) | 2.0 failure | CJ3 control | Enforced at | Real from |
|---|---|---|---|---|
| **C1** (3P audit; Codex "deterministic transition validation" family) | Miner-supplied problem instance never regenerated or verified | A2: instances derived from beacon + parent hash; miner submissions have no instance field to lie in; validators re-derive (SPEC B5–B6) | Type system + spec | Phase 2 code; doctrine binding now |
| **C2** (3P; same Codex family) | Self-reported `work_score` / `solve_time` drive fork choice and rewards | A3: `check(instance, solution) → Quality` pure function, no timing inputs exist; A4: quality never weights fork choice; banned-identifier CI grep-gate | Trait signature + SPEC §10–11 + CI | Grep-gate live since P-001; consensus Phase 2 |
| **C3** (3P) | Address derivation diverged across components; genesis allocations unspendable | Exactly one `addr()` in `cj3-types`, Lean-specified; **genesis spend-test runs in CI on every build** | SPEC §2, §13 + CI | Phase 1 (P-007/P-101) |
| **C4** (3P; Codex family) | Nonce unchecked → replay | V4: strict `nonce == account.nonce`, re-checked at apply | Lean `Spec/Tx.lean` + kernel + conformance vectors | Phase 1 |
| **C5** (3P; Codex family) | Unchecked balance arithmetic | A5/D7: `u64` newtypes, `checked_*` only, conservation property test in CI | Type system + CI | Phase 1 |
| **C6** (3P; Codex family) | Atomic commit trusts its caller | SPEC §8: commit path re-validates all preconditions internally; no bypass constructor or "pre-validated" API exists | Kernel API design + spec | Phase 1 |
| **C7** (3P; Codex "unauthed RPC state mutation / arbitrary debits") | Unauthenticated state-mutating RPC | A7: every mutating endpoint authenticated; startup refuses empty/missing secret; G3 auth-matrix test enumerates every endpoint × {no token, bad token, empty secret} | `cj3-node` + G3 gate | **Phase 3 — see GAP-1** |
| **DARQ-021** (2.0 loop) | `from ≠ addr(pubkey)` — theft class | V3: explicit binding check `from == addr(pubkey)` **at the validation site** | Lean + kernel + adversarial corpus | Phase 1 |
| **M3-class** (3P Medium) | Relay/mempool improvements routed traffic through a validator that skipped the binding check | One validity predicate; mempool admission calls the identical function block validation calls | SPEC §7 | Phase 1 |
| **Bounded ingress** (Codex ~25; panics on malformed input, IDOR, log-flood) | Hostile input → panics, unauthorized object access, log exhaustion | Typed decode errors everywhere; size/rate caps; fuzz targets on every codec from the moment it exists; per-object authz + log-rate discipline (**GAP-2**) | `cj3-net`/`cj3-rpc` + fuzz CI + G3 | Fuzz from first codec (Phase 0–1); full surface Phase 3 |
| **Fail-open auth** (Codex; SEC-PR-001 empty-secret) | Missing config fell open | A7 fail-closed doctrine: refuse to start | `cj3-node` startup + G3 matrix | Phase 3 |
| **Dependency blind spot** (flagged twice on 2.0) | No `cargo audit`/`deny` gating | D6: full gate set from first commit + nightly advisory-drift report | CI | **Live since P-001** |

## §2 Program rulings and integrity lessons → CJ3 doctrine

| 2.0 lesson | Where it lives in CJ3 |
|---|---|
| False "CI green" claim (integrity incident → D18) | A11 evidence-pointer rule; autonomous prompt VERIFY step: CI status read from the CI system only, run URL mandatory; "your memory of green is not green" |
| Ruling 1 — fix is an explicit binding check, not a `verify_signature()` swap; no smuggled bounds checks | SPEC V3 carries the ruling language verbatim: explicit check, at the validation site, as its own rule |
| Ruling 2 — every value-moving type debits `.from`; Transfer-only fix under-fixes | Single validity predicate for all value movement (SPEC §7); AMEND-3 extends the rule to any future tx type |
| Ruling 3 — fixing downstream before upstream creates false closure | Structural: A2 (canonical instance) and A3 (derived score) are inseparable by construction — C1 is literally an input to C2's fix |
| Ruling 4 / standing constraint — no configuration mitigates a validation-path hole | A6/A7 design: validity is never configuration-dependent |
| Quality-provenance question ("a supplied quality field is the same bug relocated") | `Quality` exists only as checker output; B12 derives the reward from it and every node validates the derivation |
| Audit epistemics — clean checkmarks are weaker evidence than findings | Adversary-pass rule: a clean pass must state what was looked for; findings recorded even when embarrassing |
| Line-drift — audits reference stale commits | All verification pinned to exact SHAs with CI URLs (mechanical A11) |
| DARQ-TA-001 — torn-write spiral, sandbox lifecycle defects | Three-strikes repetition tripwire; degradation checkpoint; OneDrive path ban in preflight; single-writer discipline (decoy repo deleted) |
| History-forensics lesson — a validity change that can't resync from genesis is worse than the bug | G2 deterministic-replay gate; greenfield chain, so genesis-forward replay is total; fork/upgrade governance deferred with eyes open (pre-testnet scope) |
| 2.0 "done well" list (JWT verify logic; keystore argon2id/AES-256-GCM; codec identity binding; SIWB) — do not regress | Wallet/keystore out of CJ3 scope (BEANlet program); RPC auth inherits A7; identity binding superseded by V2/V3 + domain-separated signing |

## §3 Known gaps — the honest list

- **GAP-1 (doctrine ≠ shipped).** C7 and the H-class controls are spec-level until
  Phase 3; G3 is where they become real. No closure claim before G3 evidence exists.
  Secret *provisioning/rotation* procedure (SEC-PR-001's operational half) is
  deliberately deferred to testnet ops — deferred, not forgotten.
- **GAP-2 → AMEND-1 (proposed).** SPEC §12 lacks two explicit lines: (a) object-level
  authorization enumeration for any non-public endpoint (the IDOR class by name);
  (b) log-emission rate discipline — no unbounded log line reachable from remote
  input (the log-flood class by name). One-sentence spec amendments each.
- **GAP-3 → AMEND-2 (proposed).** Gate G1 lacks a crash-consistency criterion for
  `cj3-store`: a `kill -9` mid-block-apply must recover to the exact pre-block state
  on restart. §8's atomicity currently covers logic, not process death. This is the
  torn-write lesson applied to the chain's own database.
- **GAP-4 → AMEND-3 (proposed).** Explicit rule: any future value-moving structure
  (new tx type, fee mechanics, anything that debits or credits) extends the Lean
  V-rules **before** any Rust exists. A8 implies it; additions deserve it stated.
- **GAP-5 (unresolved, 2.0-side).** The 2.0 C4 synthesis report §11 flagged four items
  wanting a second opinion; two were ruled, **two never reached synthesis and are not
  in this record.** Action: when the 2.0 loop resumes, extract both, test them against
  CJ3's axioms, extend this matrix. Until then their CJ3 coverage is unknown.
- **GAP-6 → P-009 (proposed packet).** This matrix is compiled from the remediation
  record, not the source documents. **P-009 — Audit traceability verification
  (read-only):** ingest `COINjecture-2.0-Security-Audit.docx` and the Codex scan
  report (paths supplied by Al); enumerate every finding ID/class; diff against this
  matrix; report anything unmapped or mis-mapped; propose matrix v0.2. No `src/`
  changes. Blocked until Al supplies the two source-document paths.

---
*docs/AUDIT_TRACEABILITY.md — CJ3 program, DARQ Labs LLC. Matrix v0.2 requires P-009.*
