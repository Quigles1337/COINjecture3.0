# CJ3 Audit Traceability Matrix — v0.2 (P-009 source-verified proposal)

**Purpose:** answer, finding by finding, whether the shortcomings documented for
COINjecture 2.0 have a binding CJ3 control, a future gate, an intentionally absent
surface, or an unresolved gap. A mapping is not an implementation claim.

**Status:** P-009 source-verified proposal. This document does not ratify a protocol
amendment, fill an owned TBD, or claim that Phase 1–4 code exists.

## §0 Evidence basis and method

### Source set

| ID | Source | Durable evidence | Integrity and inspected surface |
|---|---|---|---|
| S1 | COINjecture 2.0 Phase A Security Audit — Findings Report | `loop/evidence/COINjecture-2.0-Security-Audit.docx` | 31,811 bytes; SHA-256 `D6A9100E9E69A9677EC0A562C486FFF8876839CC8378CAE1FA157326E22B7A7F`; 8 pages; 33 findings: 7 Critical, 11 High, 10 Medium, 5 Low |
| S2 | DARQ-LV-001 COINjecture v2.6 Lean Audit v0.2.0 | `loop/evidence/DARQ-LV-001_COINjecture_v2.6_Lean_Audit.pdf` | 27,414 bytes; SHA-256 `4E20AA8B287C70F8D0871D9D53FF55BE251FBDDD8113CB2786CD733FFD9C9C30`; 6 pages; 25 claim-points and 8 recommendations |
| S3 | Codex security-scan remediation record | Source report remains outstanding; committed substitute described in §3 | Five Codex programs are cross-referenced to committed DARQ root causes; exact source-report finding IDs/text cannot be independently reconstructed without the source file |
| S4 | CJ3 authority and implementation evidence | `docs/ENGINEERING_PLAN.md`, `docs/PROTOCOL_SPEC.md`, `loop/LEDGER.md`, packet reports and CI evidence | Used to distinguish binding doctrine, HUMAN-ratified Lean content, implemented Phase 0 surfaces, and future gates |

S1 was exported read-only with Microsoft Word after the packaged LibreOffice renderer
reported that `soffice` was unavailable, then rasterized at 144 DPI with the bundled
Poppler runtime. Its internal page metadata and rendered page count both equal 8.
Every page was visually inspected and reconciled with `python-docx` paragraph/table
extraction. Page 4 has a source-layout collision between C7's location line and the
first sentence; extraction preserves the complete sentence. The DOCX contains no
comments or tracked insertions/deletions and no material footnote/endnote text.

S2 was rasterized at 144 DPI with the bundled Poppler runtime and all 6 pages were
visually inspected. Poppler warned about unavailable `Symbol` and `ArialUnicode`
display fonts, but the rendered claim matrix remained legible. `pdfplumber` extraction
was reconciled against the images; its `(cid:127)` bullet artifacts on page 5 were not
treated as source text.

S1 describes a manual static review of approximately 87,000 Rust lines across
network, consensus/PoUW, core, state/ledger, mempool, RPC/API, wallet, and node
surfaces. S2 is pinned to COINjecture 2.0 commit `005fcf1` (2026-05-30), scopes
approximately 1,117 lines across 18 Lean files, and explicitly says the Lean kernel
and full Mathlib build were not re-executed during that audit. Its Tier-I labels are
therefore source-inspection classifications contingent on the reported clean build,
not an independent replay result.

### Verdict vocabulary

- **MAPPED — binding now:** a governing CJ3 rule or HUMAN-ratified formal surface
  addresses the failure class. This still does not mean downstream Rust exists.
- **MAPPED — future gate:** the control is explicit, but its implementation and gate
  evidence are deferred to the named phase.
- **PARTIAL:** part of the failure is controlled, while a specific omission remains
  in §5's proposed gap register.
- **SURFACE EXCLUDED:** the vulnerable 2.0 feature is absent from CJ3's governed
  genesis architecture. The mapping must reopen if that feature is proposed later.

## §1 Third-party security audit — complete 33-finding matrix

### Critical findings

| ID | 2.0 finding | CJ3 treatment and evidence | Verdict / real from |
|---|---|---|---|
| **C1** | Miner chooses the problem; validators never regenerate the assigned instance | A2; `ProblemClass::derive_instance`; B5–B6 re-derive and check the instance; miners submit only a solution | **MAPPED — future gate.** Doctrine/spec binding now; Phase 2 implementation and G2 adversarial wrong-instance corpus |
| **C2** | Self-reported `work_score` drives fork choice and rewards | A3/A4; checker-only `Quality`; §10 excludes Q from fork choice; §11/F3 binds reward inputs to `check(derive_instance(...), solution)`; banned-name CI gate | **MAPPED — future gate.** Source-policy gate live; consensus/reward wiring and spoof tests at Phase 2/G2 |
| **C3** | Components derive different addresses, stranding funds | Exactly one `addr()` in `cj3-types` (§2); all consumers import it; §13 genesis spend-test | **MAPPED — future gate.** Formal V3 is HUMAN-ratified; concrete codec/address implementation is P-007/P-101 and spend evidence is G1 |
| **C4** | Apply path does not enforce nonce equality | V4 requires exact equality; §8 re-evaluates V1–V9 against evolving state | **MAPPED — binding now/formal.** V4/STF are HUMAN-ratified; Rust conformance is P-101/G1 |
| **C5** | Raw balance arithmetic wraps/panics and can mint or burn | A5/D7; V6 and §8 require checked unsigned arithmetic; conservation and `reward ≤ subsidy` are proved in the ratified Lean STF | **MAPPED — binding now/formal.** Amount newtypes remain P-007; concrete kernel/store proof is P-101/G1 |
| **C6** | Atomic ledger apply trusts caller-supplied final state | §8 owns all checks, exposes no prevalidated bypass, rejects atomically; `LawfulStateOps` conservation is HUMAN-ratified; G1 requires forced process-death rollback | **MAPPED — future gate.** Formal boundary is ratified; concrete store laws and crash consistency are P-101/G1 |
| **C7** | Marketplace/faucet RPC lets callers debit or credit an arbitrary account without proving control | §12 requires auth on every mutating endpoint **and** enumerated object/principal/action authorization; value movement must enter the V-rules first | **MAPPED — future gate.** AMEND-1 corrected v0.1's transport-only framing; implementation and allow/deny matrix are Phase 3/G3 |

### High findings

| ID | 2.0 finding | CJ3 treatment and evidence | Verdict / real from |
|---|---|---|---|
| **H1** | Mesh gossip accepts unsigned messages from unknown senders and re-floods them | Strict codecs and ingress bounds do not establish authenticated peer envelopes or verified forwarding | **PARTIAL — GAP-7.** Phase 3 surface, but no explicit authenticated-gossip rule yet |
| **H2** | Gossip envelopes have no replay/freshness check | Transaction `network_id` and block timestamp rules do not define transport-envelope freshness or replay state | **PARTIAL — GAP-7.** A transport replay rule remains proposed, not ratified |
| **H3** | Unbounded peer exchange forces outbound dials and enables eclipse setup | §12 provides per-peer rate/size caps, but not total peer caps, subnet diversity, peer-exchange cardinality, or concurrent-dial bounds | **PARTIAL — GAP-8.** Phase 3 implementation absent |
| **H4** | Mempool admits transactions without nonce/balance validation | §7 requires mempool admission to call the same V1–V9 predicate as block validation; V4/V6 are ratified | **MAPPED — future gate.** Concrete mempool/kernel wiring is Phase 1+ |
| **H5** | No per-sender mempool limit lets one account occupy the pool | Per-peer network throttling and `FEE_MIN` do not bound one principal's pool occupancy or define eviction | **PARTIAL — GAP-9.** No per-sender mempool discipline is specified |
| **H6** | Escrow Release/Refund accepts unverified additional signatures | CJ3 genesis has one signed value-moving Tx and no escrow type; AMEND-3 forbids a future value-moving Rust type before normative Lean rules | **SURFACE EXCLUDED.** Reopen and define signer quorum if escrow is ever proposed |
| **H7** | Escrow signing bytes omit arbiter, conditions, and operation type | No escrow structure exists; D15 requires domain-separated canonical signing and AMEND-3 gates future value movement | **SURFACE EXCLUDED.** Reopen with a complete signed-field set before any escrow implementation |
| **H8** | `/admin/*` endpoints lack authentication | §12 requires mutating auth, fail-closed startup, and object-level allow/deny enumeration | **MAPPED — future gate.** Phase 3/G3 auth matrix |
| **H9** | PostgREST filter injection through raw query interpolation | CJ3's governed architecture contains no Supabase/PostgREST query layer | **SURFACE EXCLUDED.** Reopen with typed query construction/allowlists if such a layer is added |
| **H10** | Spoofed `X-Forwarded-For` bypasses admin allowlists and rate limits | Object authorization and rate limits are specified, but trusted-proxy semantics and authoritative client-address derivation are not | **PARTIAL — GAP-10.** Phase 3 omission |
| **H11** | Public POST/API routes have no rate limiting | §12 requires bounded ingress, rate-limited reads, and remote-input log discipline | **MAPPED — future gate.** Phase 3/G3; concrete resource bounds remain TBD(P-8) where applicable |

### Medium findings

| ID | 2.0 finding | CJ3 treatment and evidence | Verdict / real from |
|---|---|---|---|
| **M1** | SubsetSum checker accepts out-of-range and duplicate indices | SIS is the only genesis class; P-004 rejected the legacy SubsetSum class on hardness and checker-contract evidence; every future class needs an admission report/ADR | **MAPPED — exclusion/admission.** `bench/p004-admission/evidence/LEGACY-CALIBRATION.md` |
| **M2** | TSP checker can index malformed dimensions and panic | P-004 rejected the legacy TSP class; canonical decode and class checkers must return typed `Invalid`, never panic | **MAPPED — exclusion/admission.** No TSP consensus surface exists |
| **M3** | Direct block-apply callers bypass signature revalidation | §7 defines one validity predicate; §8 re-evaluates V1–V9 at mutation time and grants mempool prevalidation no authority | **MAPPED — binding now/formal.** This is distinct from DARQ-021; v0.1 conflated them |
| **M4** | Marketplace escrows a bounty without debiting the submitter | No marketplace value type exists; conservation covers the genesis STF; AMEND-3 gates any future debit/credit structure | **SURFACE EXCLUDED.** Reopen before marketplace economics enter CJ3 |
| **M5** | Multi-table writes commit separately and corrupt state on crash | §8 requires block-level atomicity; AMEND-2 adds a kill-mid-apply recovery test to G1 | **MAPPED — future gate.** Concrete store evidence is P-101/G1 |
| **M6** | `dimensional_scale == 0` underflows; float-to-int casts are unchecked | A5 bans floating point on consensus paths and requires checked unsigned arithmetic; CJ3 has no dimensional-pool state path | **MAPPED — structural/excluded surface.** Reopen only if a future class adds such state |
| **M7** | Dual bincode/JSON hashing creates two commitment/PoW acceptance surfaces | §1 requires one strict canonical codec and one hash preimage per domain | **MAPPED — future gate.** Concrete bytes remain P-007/G0 and SI-001/002/003 stay unresolved |
| **M8** | Peer authentication disappears when encryption is disabled | No rule yet requires one authenticated transport or forbids an unauthenticated mode | **PARTIAL — GAP-7.** Strict decode is not peer authentication |
| **M9** | EclipseGuard treats all RFC1918 space as unlimited loopback | §12 does not define subnet diversity or trusted/private-address classification | **PARTIAL — GAP-8.** Phase 3 omission |
| **M10** | Order-cancel IDOR and internal-error disclosure | AMEND-1 object authorization covers the IDOR half; typed errors do not expressly forbid internal diagnostic leakage | **PARTIAL — GAP-11.** Principal/object checks are future G3; public-error hygiene remains proposed |

### Low findings

| ID | 2.0 finding | CJ3 treatment and evidence | Verdict / real from |
|---|---|---|---|
| **L1** | Keystore silently accepts an empty password | Wallet/keystore functionality is outside the CJ3 program; §12 separately refuses an empty RPC secret at startup | **SURFACE EXCLUDED.** Do not treat RPC-secret fail-closed behavior as a future wallet/key-custody design |
| **L2** | Ed25519 verification is non-strict and does not explicitly reject zero signatures | D15 selects Ed25519 and V2 requires verification, but neither authority states strict verification/canonical signature rules | **PARTIAL — GAP-12.** P-007/G0 cannot silently decide this semantic |
| **L3** | Per-transaction hot-path logging enables log amplification | §12 forbids unbounded remote-controlled log lines and rates at the emission site | **MAPPED — future gate.** Phase 3 implementation |
| **L4** | Marketplace expiry arithmetic can overflow | A5 requires checked consensus arithmetic; the marketplace/expiry surface is absent | **SURFACE EXCLUDED / structurally guarded.** Reopen if time-based value movement is proposed |
| **L5** | Rate-limit map is unbounded; TLS proxy lacks timeouts/connection caps | §12 requires rate/size caps, but not bounded limiter-key cardinality, idle/handshake timeouts, or connection ceilings | **PARTIAL — GAP-13.** Phase 3 omission |

### Positive observations from S1 (not findings and not inherited evidence)

S1 also found sound 2.0 implementations for CSPRNG-backed key generation and encrypted
keystores, JWT verification, SIWB nonce/signature handling, the CPP wire codec, CPP
identity binding when encryption is enabled, and several isolated hardening choices.
CJ3 receives no implementation credit from those observations: its wallet is out of
scope, its network/RPC crates are future work, and every relevant behavior must pass
its own CJ3 spec and gate.

The source's recommended order is preserved: consensus integrity (C1/C2/M3/M7), money
safety (C3–C7/M4), escrow (H6/H7), liveness/DoS (H1–H5/M2/M8/M9), API hardening
(H8–H11/M10), attacker-reachable panic removal, then Low findings. D11 independently
keeps the COINjecture 2.0 remediation queue ahead of CJ3 capacity.

## §2 Lean audit — all 25 claim-points and CJ3 A8 disposition

The S2 tier is the strongest artifact backing the **2.0 whitepaper claim**; it is not a
CJ3 proof rating. Tier counts reconcile exactly: I = 8, II = 3, III = 8, IV = 6.

| # | Tier | Audited 2.0 claim | Bearing on CJ3 A8 |
|---:|---|---|---|
| 1 | II — numeric | Unit-normalization gives `\|x\|=\|y\|=1/sqrt(2)` | Float `native_decide` samples are not analytic proofs. CJ3's consensus spec contains no Float/native-decide surface. |
| 2 | I — proof | `\|mu\|=1` | Genuine 2.0 proof, but unrelated to CJ3 V1–V9/STF; no proof credit carries across projects. |
| 3 | I — proof | Unique `mu=(-1+i)/sqrt(2)` under three constraints | Same: earned in its source model, not CJ3 evidence. |
| 4 | I — proof | Anchor/coherence factorization and light-cone result | Same: no executable CJ3 consensus link is inferred. |
| 5 | II — numeric | Eight-fold closure / primitive 8-cycle | Numeric Float checks do not prove group-theoretic closure. CJ3 leaves any mu/curve hook owner-controlled and unfilled. |
| 6 | II — numeric | Coherence identities and decay | Sample points are not universal real theorems; they cannot shape CJ3 consensus or rewards by implication. |
| 7 | IV — unbacked | `mu`/coherence governs consensus dynamics | Direct A8/A10 warning. CJ3 explicitly leaves the mu-balance hook unfilled and outside the executable path. |
| 8 | III — axiom | Real-valued work-score formula | CJ3 bans self-reported `work_score`/timing and uses checker-derived integer quality. The axiom is not imported. |
| 9 | III — axiom | Doubling solve time adds one work bit | Same; solve time is not a CJ3 consensus input. |
| 10 | I — proof | Deterministic fixed-point on-chain work score | Genuine 2.0 Nat result, but CJ3 deliberately uses a different fork-choice/reward architecture. |
| 11 | I — proof | Reward monotonicity at fixed parent work | S2's formula is not CJ3's Model 4 formula; CJ3 relies only on its separately reviewed STF proof surface. |
| 12 | I — proof | First block mints about 50 coins | 2.0-specific and not imported. CJ3 P-12 subsidy remains an owned placeholder. |
| 13 | IV — unbacked | Nonzero asymptotic growth/no hard cap | CJ3 makes no such theorem claim and keeps issuance schedule ownership explicit. |
| 14 | III — axiom | Chain security equals sum of ideal bit scores | Not imported; CJ3 §10 defines hash-target weight and leaves Phase 2 formal treatment future. |
| 15 | I — proof | Integer inequality underlying catch-up | Proves only `q^z < p^z` under `q<p`, not a full attack-probability law. P-006 simulation must remain empirical evidence. |
| 16 | IV — unbacked | `(q/p)^z` Poisson magnitude bound | CJ3 must not claim this as a proof; any future formal fork-choice claim needs its own model. |
| 17 | I — proof | Positive work increases cumulative work | Genuine in 2.0's model, but CJ3's weight definition differs and gets no inherited theorem. |
| 18 | III — axiom | Adversary cannot forge a passing witness | A typed assumption is not a theorem. CJ3 uses executable checkers/adversarial gates and labels hardness assumptions. |
| 19 | III — axiom | SubsetSum NP-complete | Classically correct but unformalized there; P-004 rejected the sampled legacy class for CJ3 admission. |
| 20 | III — axiom | 3-SAT NP-complete | Same distinction between worst-case class and admitted sampled distribution. |
| 21 | III — axiom | Decision TSP NP-hard | Same; P-004 rejected the legacy TSP class. |
| 22 | IV — unbacked | Factorization is NP-hard with O(1) verify | Incorrect/unsupported in S2; P-004 left Factorization at insufficient evidence and did not admit it. |
| 23 | IV — unbacked | Private submissions use ZK well-formedness proofs | CJ3 has no ZK/private-submission claim at genesis; Tier 2 is trigger-gated and absent. |
| 24 | III — axiom | Three falsifiable predictions | S2 calls the empirical harness correctly scoped; CJ3 likewise must not relabel simulation as proof. |
| 25 | IV — unbacked | Builds on Lean/Mathlib 4.14.0 | Repository actually used 4.28.0. CJ3 pins Lean 4.33.0 and records the 4.28.0 + Mathlib 4.28.0 reveal-time compatibility gap under D16. |

### S2 recommendations R1–R8

| ID | Recommendation | CJ3 consequence |
|---|---|---|
| R1 | Propagate proof/numeric/axiom/unbacked labels | Adopted as the epistemic rule for this matrix: compilation, numeric evaluation, assumptions, and theorems are never conflated. |
| R2 | Correct the 2.0 toolchain line to 4.28.0 | Recorded as the D16 toolchain gap; no pre-reveal alignment is authorized. |
| R3 | Disclose that private-submission ZK is a placeholder | CJ3 makes no ZK implementation claim. |
| R4 | Repair Factorization/SVP registry claims | P-004 admits neither; Factorization/SVP remain insufficient evidence. |
| R5 | Keep the earned `mu_unique` result but soften other mu prose | CJ3 imports no mu proof and leaves the mu-balance hook unfilled. |
| R6 | Disclose the trusted-base cost of `native_decide` Float results | CJ3's ratified V1–V9/STF surface contains no `Float`, `native_decide`, `axiom`, `sorry`, or `admit` token. |
| R7 | Down-scope or formally strengthen the catch-up claim | P-006/Phase 2 must distinguish simulation envelopes from a formal security proof. |
| R8 | Optionally replace numeric/placeholder results with Real/Complex theorems | No CJ3 action before a governed formal packet; this recommendation supplies no authority by itself. |

**A8-specific conclusion:** S2 materially bears on CJ3 even though it audits older
artifacts. Its lesson is not “use more Lean”; it is to state exactly what the kernel
checked. P-005's HUMAN-ratified V1–V9/STF proofs are evidence only for their declared
abstract interfaces and theorems. They do not ratify symbolic vectors, canonical
bytes, SI-001/002/003, a fork-choice security theorem, an economics schedule, a mu
hook, ZK, or Phase 1 Rust conformance.

## §3 Outstanding Codex source — committed substitute and limit

The requested Codex scan report file is still absent. P-009 did not wait for it. The
committed COINjecture 2.0 remediation record identifies
`C:\Users\LEET\COINjecture2.0-network` as the checkout matching the Codex baseline
revision `28c50a122f2caab70582e8215b670b0ddc4d236d` and Cargo.lock hash; it explicitly
rejects the separate dirty personal clone as traceability ground truth.

Exact committed pointers:

- `loop/reports/C0-builder.md` at commit
  `27844613ddf25f17a9fd059836e31ab54dbb3034`, §1 “Zero drift from the Codex
  baseline,” §2 SEC-PR inventory/overlap, §3 C1/C2/C3 verification, and §4 program
  rollup.
- `loop/REGISTRY.md` at commit
  `7eac79154ccc1d0dd5f811d885a0449a5db7f110`, “Codex program cross-reference”:
  P1 → DARQ-001/002/003/004/005/006/008/013/015; P2 → DARQ-005/014;
  P3 → DARQ-007/010/011/012; P4 → DARQ-017; P5 → DARQ-009.
- The same registry records SEC-PR-001 at `ff6e65c4` as built but unmerged and
  SEC-PR-002…005 as unimplemented specifications. C0 §2 preserves two exact Codex
  finding IDs for SEC-PR-002's fail-open RPC transport family:
  `csf_c35bded78cd790abb52fe9b1` and `csf_8dc6eb9ab34e150bf775be40`.
- `loop/reports/C4-builder.md` at commit
  `58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff`, §§10–12, proves DARQ-021 is distinct
  from M3 and preserves the four second-opinion questions.

This is enough to retain all five Codex **program/root-cause families** in CJ3 without
waiting. It is not enough to reconstruct the scan's exact per-finding ID/title/count
inventory. Therefore v0.2 does not claim source-level Codex completeness; that narrow
unknown resolves only when the report itself is supplied. No Codex family known to the
committed cross-reference is dropped from the CJ3 controls above.

## §4 v0.1 → v0.2 reconciliation

- **Complete inventory added:** v0.1 named C1–C7 and M3, then collapsed the remaining
  third-party findings into broad Codex families. V0.2 enumerates all 33 S1 IDs and all
  25 S2 claim-points.
- **C7 corrected:** transport bearer authentication does not prove that a caller owns
  the account it names. AMEND-1's object/principal/action matrix is the relevant CJ3
  control; Phase 3 still has to implement it.
- **M3 corrected:** M3 is a direct-apply validation bypass. DARQ-021 is the distinct
  case where validation runs but checks only the signature and not `from == addr(pk)`.
  V3 and §8 intentionally cover both without merging their root causes.
- **Aggregate ingress claim split:** H1/H2/M8, H3/M9, H5, H10, M10, L2, and L5 expose
  requirements not supplied by generic size/rate/error language. They are PARTIAL,
  not silently covered.
- **GAP-2/3/4 closed as proposals:** AMEND-1/2/3 are ratified and present in §12, G1,
  and the spec-before-code value-movement rule respectively.
- **GAP-5 source uncertainty closed for CJ3 traceability:** the four C4 §11 questions
  are now read directly. Items 1–2 are addressed in CJ3 by standalone V3 plus
  AMEND-3; item 3 is a 2.0 historical-chain investigation and CJ3 has no inherited
  chain; item 4 is 2.0 remediation ordering, with D11 retaining priority. This does
  not claim those legacy questions are remediated in 2.0.
- **GAP-6 execution complete in this proposal:** both supplied sources were read and
  reconciled. Packet CI/merge evidence belongs in the P-009 builder report and loop
  closeout, not in the source matrix.

## §5 Known gaps and proposed follow-ups — not ratified

- **GAP-1 remains — doctrine is not shipped code.** Phase 1–3 controls become real
  only at their named implementations and gates. A green Phase 0 D6 run cannot close
  a deferred handler.
- **GAP-7 — authenticated gossip and freshness.** Require one authenticated peer
  transport/envelope model; never forward an unverified sender; define replay and
  freshness checks; forbid a configuration that silently disables authentication.
  Covers H1, H2, M8.
- **GAP-8 — peer-set resource and eclipse discipline.** Bound peer-exchange entries,
  total peers, concurrent dials, and address-class/subnet concentration. Covers H3,
  M9.
- **GAP-9 — per-principal mempool occupancy.** Define per-sender limits and an
  adversarially safe eviction rule in addition to global/per-peer bounds. Covers H5.
- **GAP-10 — authoritative client address.** Use the socket peer by default; honor
  forwarding headers only through an explicitly configured trusted-proxy boundary.
  Covers H10.
- **GAP-11 — public error hygiene.** Map internal errors to bounded typed public
  responses without filesystem, query, stack, or internal diagnostic disclosure.
  Covers M10's second half.
- **GAP-12 — strict Ed25519 semantics.** Before P-007/G0 fixes canonical signature
  bytes, specify strict verification, canonical encodings, and explicit rejection of
  invalid/zero signatures. Covers L2; this packet supplies no encoding choice.
- **GAP-13 — bounded limiter/TLS state.** Bound rate-limiter key cardinality and
  eviction; set handshake/idle/read timeouts and total/per-peer connection ceilings.
  Covers L5.
- **Codex source-level inventory remains unknown.** The committed P1–P5 root-cause
  cross-reference is sufficient to proceed, but only the original report can restore
  its exact per-finding IDs, titles, severities, and counts.

---
*docs/AUDIT_TRACEABILITY.md — CJ3 program, DARQ Labs LLC. V0.2 is a P-009
source-verified proposal, not a protocol ratification or shipped-control claim.*
