# Security Review: COINjecture3.0

## Scope

Security diff review of P-005 PR #9 second-review remediation from 42da63c6cdec7c0cee84033365c509e04459fe07 through 97fa5110af60b832a9f3b26dd57cc7690a31cc75.

- Scan mode: branch_diff
- Target kind: git_diff
- Target ID: target_sha256_9a0ee0ac694f3cb1298f114324ff508f1d6006dbdecc0d4570c7a38b0d114b06
- Revision range: 42da63c6cdec7c0cee84033365c509e04459fe07...97fa5110af60b832a9f3b26dd57cc7690a31cc75
- Snapshot digest: codex-security-snapshot/v1:sha256:874d4f614dffd1325e3369f82b468099870928e8570eac19f2062ea7ba9b6166
- Inventory strategy: diff
- Included paths: .
- Excluded paths: none
- Runtime or test status: Pinned Lean 4.33.0 full Lake build and active Lean handler passed locally; vector count remained 27 with SHA-256 30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1. The complete local D6-equivalent Rust/CI suite also passed. Hosted exact-head D6 is intentionally a later immutable-head requirement.
- Artifacts reviewed: Exact 8-file Git range 42da63c6cdec7c0cee84033365c509e04459fe07..97fa5110af60b832a9f3b26dd57cc7690a31cc75, Formal transition surface: spec/Spec/Stf.lean and spec/Spec/Tx.lean, Active formal gate: scripts/ci/active/lean-conformance.ps1, Protocol and phase gates: docs/PROTOCOL_SPEC.md and docs/ENGINEERING_PLAN.md, Governance and packet controls: loop/LEDGER.md and loop/PACKETS.md, Builder FRAME/evidence surface: loop/reports/C5-p005-builder.md
- Scan context: The repository-scoped threat model was regenerated for the immutable remediation head. The packet is a HUMAN-authorized draft Lean specification and governance/CI admission surface in a private pre-testnet repository; PR #9 remains mandatory-owner-review gated and unmerged.

Limitations and exclusions:
- The workbench source classifier returned an empty compact inventory for the Lean/PowerShell/documentation-only range; the review therefore enumerated all eight changed files from the exact Git range and reviewed each manually.
- P-101 Rust vector consumption, concrete authenticated storage, and the production kernel do not exist and are not claimed as conformant.
- Canonical codec/signature/address implementations and owner/G0 parameter values remain deliberately abstract.
- The C2 reward-provenance wiring is a binding P-101/G2 obligation; P-005 records it but does not instantiate a checker or block decoder.

### Scan Summary

| Field | Value |
| --- | --- |
| Reportable findings | 0 |
| Severity mix | none |
| Confidence mix | none |
| Coverage | complete |
| Validation mode | Static exact-diff review plus complete Lean build, deterministic vector regeneration/hash comparison, source-policy checks, and local D6-equivalent execution. Discovery produced no plausible security candidate, so candidate validation and attack-path analysis were not applicable. |

Canonical artifacts: `scan-manifest.json`, `findings.json`, and `coverage.json`. This report is a deterministic projection of those files.

## Threat Model

CJ3 is a private pre-testnet PoUW L1 foundation whose primary assets are deterministic consensus, monetary conservation, signer/address and nonce integrity, atomic state application, checker-derived reward provenance, fork-choice independence from miner-controlled quality, solver isolation, and trustworthy formal/CI evidence.

### Assets

- Consensus and state agreement
- Balances, fees, subsidy ceiling, and reward conservation
- Transaction authorization, nonce, and address binding
- Checker-derived instance, solution quality, and reward provenance
- Canonical encodings, genesis, state roots, and persisted chain state
- Formal-spec/vector integrity and CI/release supply chain

### Trust Boundaries

- Hostile peers, miners, blocks, transactions, and solutions into deterministic validation
- Untrusted external solvers into safe-Rust checkers
- Validator-derived instance and checker output into reward calculation, excluding miner-supplied quality
- Consensus candidate state into a concrete authenticated store satisfying LawfulStateOps
- Human/developer formal specifications and vectors into future Rust kernel conformance
- Contributor-controlled source and dependencies into minimally privileged pinned CI

### Attacker Capabilities

- Choose malformed and boundary-value protocol bytes, replay data, alias accounts, maximize quality, withhold blocks, and coordinate peers or miners
- Attempt to supply a forged quality or work score independently of the solution checked over the derived instance
- Return arbitrary solver outputs and consume local solver resources
- Submit malicious source, formal proof, documentation, or workflow changes subject to repository permissions and mandatory human merge controls

### Security Objectives

- Derive every consensus quantity from committed data and checker output; never trust miner-reported substitutes
- Use checked unsigned money arithmetic and reject atomically without partial state
- Prove transaction conservation and exact reward issuance from explicit concrete-store laws
- Keep quality out of fork weight and reward at or below the subsidy ceiling
- Keep proof placeholders, panic paths, unsafe Rust, and linked solver code out of the consensus TCB
- Make every security and verification claim traceable to an executed gate or durable artifact

### Assumptions

- The repository is pre-testnet and has no exposed production node or mainnet configuration.
- Cryptographic primitives are secure only under exact domain and canonical-byte use; open SI/TBD values are not assumed.
- LawfulStateOps is an abstract P-005 contract that a concrete P-101 authenticated store must prove without custom axioms or admissions.
- Context.rewardInputs remains abstract in P-005 and must be instantiated only from check(derive_instance(instance_seed, size_param), solution) in P-101.
- Human-owned formal semantics remain draft until the mandated second line-by-line review and owner merge.

## Findings

### No findings

No reportable findings survived the canonical discovery, validation, and reportability gates.

## Reviewed Surfaces

| Surface | Risk Area | Outcome | Notes |
| --- | --- | --- | --- |
| Panic-free V1-V9 validation and checked arithmetic | Validation bypass, malformed-input panic, self-send divergence, or unchecked u64 arithmetic | No issue found | Reviewed spec/Spec/Tx.lean. Tx.validate now returns directly after V8 with no unreachable! or other panic path; v9_allows_self_send remains. Successful checked-add/subtraction lemmas state exact Nat arithmetic and the active gate rejects proof placeholders and panic regression. |
| LawfulStateOps and transaction conservation | Vacuous conservation premise, incorrect total-balance replacement law, or sender/recipient/miner aliasing gap | No issue found | Reviewed the full sequential transition and proof. LawfulStateOps names the additive totalBalances/setAccount law plus read-after-write and read-other-address. applyTransaction_conserves connects the actual validator and state transition, while classifyAddressAliasing exhausts all-distinct, sender=recipient, sender=miner, recipient=miner, and all-three-equal cases. |
| Successful block-candidate conservation and reward ceiling | Fee mint/burn, partial-state accounting, reward overissuance, or proof disconnected from the executable result | No issue found | applyTransactions_conserves inducts over evolving state, applyReward_conserves credits exactly rewardNat, and applyBlockCandidate_conserves traces actual Except success through rule, transaction, reward, and state-root branches. conservationTarget_holds discharges the public property under LawfulStateOps; reward_le_subsidy remains a theorem, not an assumption. |
| C2 checker-derived reward provenance | Miner-controlled quality, work score, timing, or wrong-instance solution influencing reward | No issue found | Protocol Spec §11, the LEDGER, P-101 queue definition, and Gate G2 now bind Context.rewardInputs to check(derive_instance(instance_seed, size_param), solution) for the validated block and forbid block-supplied quality. The abstract P-005 Context was intentionally not concretized. |
| Active Lean CI admission evidence | Green-without-proof, string-only false assurance, ambient toolchain acceptance, or vector drift | No issue found | The handler still runs the pinned full Lake build and deterministic vector regeneration/hash comparison. The new declaration checks and Tx panic regression are supplemental; successful compilation remains the enforcement that the named theorems exist. No axiom, sorry, or admit appears in Spec/\*.lean. |
| Governance, packet state, and review-tail controls | Premature conformance or completion claim, hidden owner/SI decision, or automatic merge | No issue found | The exact live ruling is preserved, the P-101 and G2 obligations are explicit, owner/G0 values and SI-001/2/3 remain unfilled, and PR #9 remains open, ready-for-review, and unmerged pending Al's second review. |
