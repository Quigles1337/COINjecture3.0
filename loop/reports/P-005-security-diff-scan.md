# Security Review: COINjecture3.0

## Scope

Security diff review of P-005 PR #9 from main 8d9dbbfef8552e80c1e7e2e13c0d6815cded523e through implementation commit 388ede835e8c9e668f5f0126918193ff59f589cd.

- Scan mode: branch_diff
- Target kind: git_diff
- Target ID: target_sha256_9a0ee0ac694f3cb1298f114324ff508f1d6006dbdecc0d4570c7a38b0d114b06
- Revision range: 8d9dbbfef8552e80c1e7e2e13c0d6815cded523e...388ede835e8c9e668f5f0126918193ff59f589cd
- Snapshot digest: codex-security-snapshot/v1:sha256:6706b02e0895c0b943a8c0767c53d0f96267a53b2df920f42ba402b81b274cb0
- Inventory strategy: diff
- Included paths: .
- Excluded paths: none
- Runtime or test status: Pinned Lean 4.33.0 Lake build and deterministic 27-case export passed locally; the Rust workspace D6-equivalent checks passed locally. Hosted exact-head D6 is intentionally outside this immutable scan and remains required before PR readiness.
- Artifacts reviewed: Exact 21-file Git range 8d9dbbfef8552e80c1e7e2e13c0d6815cded523e..388ede835e8c9e668f5f0126918193ff59f589cd, Workbench inventory: spec/lake-manifest.json and spec/vectors/p005-draft.json, Supporting formal sources: spec/Spec/Tx.lean, spec/Spec/Stf.lean, spec/Spec/Vectors.lean, spec/Main.lean, CI trust boundary: .github/workflows/ci.yml, scripts/ci/active/lean-conformance.ps1, scripts/ci/check-phase-gate.ps1, Governing controls: docs/PROTOCOL_SPEC.md, docs/ENGINEERING_PLAN.md, loop/LEDGER.md, loop/PACKETS.md, loop/STATE.md, loop reports
- Scan context: The repository-scoped threat model was generated during Phase 1. The packet is a HUMAN-authorized, draft Lean specification and CI admission surface in a private pre-testnet repository; it is not deployed consensus code and remains subject to mandatory owner review before merge.

Limitations and exclusions:
- P-101 Rust vector consumption and the concrete kernel do not exist and were not claimed as conformance.
- Canonical codec/signature/address implementations and owner/G0 parameter values remain deliberately abstract.
- Concrete authenticated-store read/write coherence and concrete R_MAX representation are later instantiation obligations, not properties claimed by this draft.

### Scan Summary

| Field | Value |
| --- | --- |
| Reportable findings | 0 |
| Severity mix | none |
| Confidence mix | none |
| Coverage | complete |
| Validation mode | Static diff review plus deterministic build/export and local CI-equivalent execution; no dynamic exploit validation was needed because discovery produced no plausible security candidate. |

Canonical artifacts: `scan-manifest.json`, `findings.json`, and `coverage.json`. This report is a deterministic projection of those files.

## Threat Model

CJ3 is a private pre-testnet PoUW L1 foundation whose primary assets are deterministic consensus, monetary conservation, signer/address and nonce integrity, atomic state application, fork-choice independence from miner-controlled quality, strict ingress, solver isolation, and trustworthy formal/CI evidence.

### Assets

- Consensus and state agreement
- Balances, fees, subsidy ceiling, and reward conservation
- Transaction authorization, nonce, and address binding
- Canonical encodings, genesis, state roots, and persisted chain state
- RPC secrets and future object authorization
- Formal-spec/vector integrity and CI/release supply chain

### Trust Boundaries

- Hostile peers, miners, blocks, transactions, and solutions into deterministic validation
- Untrusted external solvers into safe-Rust checkers
- Future RPC clients into authenticated and object-authorized mutation
- Consensus candidate state into coherent authenticated storage
- Human/developer formal specifications and vectors into future Rust kernel conformance
- Contributor-controlled source and dependencies into minimally privileged pinned CI

### Attacker Capabilities

- Choose malformed and boundary-value protocol bytes, replay data, alias accounts, maximize Q, withhold blocks, and coordinate peers/miners
- Return arbitrary solver outputs and consume local solver resources
- Probe future RPC authentication, object authorization, rate, size, and log boundaries
- Submit malicious pull-request source or workflow changes subject to repository permissions and human merge controls

### Security Objectives

- Derive consensus quantities from committed data; never trust miner-reported substitutes
- Use checked unsigned money arithmetic and reject atomically without partial state
- Keep quality out of fork weight and reward at or below the subsidy ceiling
- Keep canonical decoding, signature bytes, and address derivation singular and explicit
- Keep unsafe Rust and linked solver code out of the consensus TCB
- Make every security/verification claim traceable to an executed gate or durable artifact

### Assumptions

- The current repository is pre-testnet and has no exposed production node or mainnet configuration.
- Cryptographic primitives are secure only under exact domain/canonical-byte use; open SI/TBD values are not assumed.
- Human-owned formal semantics remain draft until the mandated line-by-line review and merge.
- Planned/stub components do not enforce behavior merely because protocol prose specifies it.

## Findings

### No findings

No reportable findings survived the canonical discovery, validation, and reportability gates.

## Reviewed Surfaces

| Surface | Risk Area | Outcome | Notes |
| --- | --- | --- | --- |
| Lean transaction validity and checked arithmetic | V1-V9 bypass, signer/address mismatch, nonce drift, unchecked u64 wraparound | No issue found | Reviewed Tx.lean and its protocol mapping. Canonical decoding, signature construction, address derivation, limits, fees, and network id remain abstract; checked conversions use proof-bounded UInt64.ofNatLT and no alternate validity path is introduced. |
| Lean STF and SI-004 reward model | Partial application, aliasing, reward inflation, floor-division or u128/u64 boundary errors | No issue found | Reviewed the full candidate-state flow and Model 4 theorems. reward_le_subsidy is derived from the formula/min bound, not supplied as a field or assumption; conversion back to u64 is proof-bounded. Atomic rejection returns pre-state. Concrete store coherence and R_MAX representation are explicitly later obligations. |
| Symbolic vector generator and committed JSON | TBD laundering, canonical-byte smuggling, schema drift, command/path injection | No issue found | All 27 unique cases carry object-valued input_bytes with normative=false and the exact noncanonical status. No owner/G0 value is selected. The JSON contains data-only symbolic expressions; the developer CLI output path is not a remote/runtime input. |
| Lean package metadata and dependencies | Unreviewed dependency or toolchain supply-chain expansion | No issue found | lake-manifest.json contains zero packages and the project pins Lean 4.33.0. No runtime or foreign dependency is introduced. |
| Active Lean CI gate and workflow | Green-without-execution, ambient toolchain acceptance, artifact drift, repository-script injection | No issue found | The action is commit-pinned with read-only workflow permissions and caches/config/build disabled. The handler resolves Lean while spec/lean-toolchain is in scope, runs the full Lake build/export, compares SHA-256 artifacts, enforces required cases/status/header/no-placeholder checks, and explicitly says P-101 Rust conformance is not admitted. |
| Governance, packet state, and claims | Premature normative/conformance claim, hidden owner decision, merge/reviewer notification bypass | No issue found | The exact SI-004 ruling is preserved; P-7 value/curve and SI-001/2/3 stay unfilled; P-101 remains blocked; the formal sources are marked draft; PR #9 remains HUMAN review-gated and unmerged. |
