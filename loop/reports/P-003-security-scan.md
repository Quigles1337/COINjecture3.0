# Security Review: COINjecture3.0

## Scope

Security diff review of the complete P-003 sampled-SIS genesis-class range 4b0102b8e03a1701d2196ed758aeff768b984ef3..4516d6bc71612c1b64ec6fb662d61082b0fffb77. The exact 19-file diff was reviewed, with the five executable Rust/manifests treated as the authoritative risk worklist and the benchmark, evidence, lockfile, and governance artifacts reviewed directly by the parent.

- Scan mode: branch_diff
- Target kind: git_diff
- Target ID: target_sha256_9a0ee0ac694f3cb1298f114324ff508f1d6006dbdecc0d4570c7a38b0d114b06
- Revision range: 4b0102b8e03a1701d2196ed758aeff768b984ef3...4516d6bc71612c1b64ec6fb662d61082b0fffb77
- Snapshot digest: codex-security-snapshot/v1:sha256:81b49bc87e6cb10b953b8f13228779f1eac58a997836cae138fee85f39e6cf8d
- Inventory strategy: diff
- Included paths: .
- Excluded paths: none
- Runtime or test status: Local D6 checks passed (format, clippy, dependency policy/advisory checks, zero-unsafe scan, tests, and build); GitHub Actions run 31925679014 passed all 11 jobs on the exact scanned head.
- Artifacts reviewed: Exact Git range 4b0102b8e03a1701d2196ed758aeff768b984ef3..4516d6bc71612c1b64ec6fb662d61082b0fffb77, crates/cj3-classes/Cargo.toml, src/lib.rs, src/sis.rs, and examples/p003_check_bench.rs, crates/cj3-solver-sis/Cargo.toml and src/main.rs, Cargo.lock and cargo tree reverse-dependency/feature output, bench/p003-sis/\*\* evidence scripts, inputs, environment records, and result logs, loop/reports/C3-p003-builder.md and loop/reports/SPEC-ISSUES.md, docs/PROTOCOL_SPEC.md, docs/ENGINEERING_PLAN.md, and repository governance state used to establish boundaries
- Scan context: The threat model was generated during this scan from repository protocol, engineering, governance, dependency, and source evidence. P-003 is pre-G0 and not wired into a node, network, RPC, or consensus runtime.

Limitations and exclusions:
- No node, network, RPC, wire decoder, or consensus consumer exists for cj3-classes in the scanned revision, so future hostile-runtime reachability cannot be dynamically exercised.
- A deliberately enormous allocation was not executed because it would only exhaust the reviewer host and would not prove the missing cross-principal boundary.
- The estimator and solver measurements are research evidence rather than proof of a production security level; parameter ratification remains outside this security scan.

### Scan Summary

| Field | Value |
| --- | --- |
| Reportable findings | 0 |
| Severity mix | none |
| Confidence mix | none |
| Coverage | complete |
| Validation mode | Static source/control/sink tracing, full call-site search, Cargo reverse-dependency analysis, dependency feature inspection, existing tests, and exact-head CI evidence. |

Canonical artifacts: `scan-manifest.json`, `findings.json`, and `coverage.json`. This report is a deterministic projection of those files.

## Threat Model

COINjecture 3.0 is a pre-testnet proof-of-useful-work blockchain. Future peers, RPC clients, blocks, transactions, solution bytes, class identifiers, and size parameters are hostile; useful-work solvers are explicitly outside the trusted computing base. P-003 currently adds a safe-Rust SIS class/checker and a standalone developer solver, but no node or consensus crate consumes them yet. Security-critical instances must eventually be derived from committed chain state under ratified, bounded parameters, and all benchmark claims must remain evidence rather than silently becoming consensus constants.

### Assets

- Consensus agreement and deterministic validation
- Integrity of protocol-derived useful-work instances
- Integrity of admitted class identifiers and parameter floors
- Node availability and bounded checker resource use
- Build and dependency supply-chain integrity
- Auditability of specification-to-implementation claims

### Trust Boundaries

- Future hostile network/RPC/block/solution input to trusted safe-Rust codecs and checkers
- Untrusted out-of-process SIS solver to trusted checker
- Committed chain state and beacon seed to deterministic instance derivation
- Developer benchmark/evidence tooling to governance parameter ratification
- Pinned Rust dependencies and CI controls to trusted validation artifacts

### Attacker Capabilities

- Submit malformed, oversized, adversarial, or boundary-value future protocol inputs
- Run or compromise an external solver and emit arbitrary candidates
- Attempt to choose weak instances or parameters if a trusted path reads rather than derives them
- Exploit arithmetic, allocation, parser, determinism, or dependency-feature mistakes once wired
- Influence developer evidence through compromised or mutable research tooling if provenance controls are weak

### Security Objectives

- Derive every consensus instance from authenticated committed inputs with byte-identical rules
- Accept solutions only through bounded, exact, panic-free safe-Rust validation
- Keep external solvers and their dependencies outside the trusted computing base
- Admit only ratified class identifiers and evidence-backed bounded parameter tuples
- Prevent benchmark conventions or provisional measurements from becoming unreviewed consensus rules

### Assumptions

- The repository is pre-testnet and the protocol remains a draft until Gate G0.
- Local developers/operators who intentionally invoke benchmark binaries are trusted with their own short-lived processes.
- No production network, deployed RPC surface, or downstream cj3-classes consumer exists at the scanned revision.
- Cryptographic hardness estimators are heuristic evidence, not proofs of a deployed security level.
- Future P-006/P-007/P-101 integration must re-evaluate parameter bounds and explicit-instance construction at the first hostile runtime boundary.

## Findings

### No findings

No reportable findings survived the canonical discovery, validation, and reportability gates.

## Reviewed Surfaces

| Surface | Risk Area | Outcome | Notes |
| --- | --- | --- | --- |
| Trusted SIS class contract, derivation, and checker | Consensus determinism, committed-instance provenance, arithmetic safety, and verifier resource bounds | Rejected | Reviewed crates/cj3-classes/src/lib.rs and src/sis.rs completely. SHAKE rejection sampling, exact norm/modular checks, private instance fields, and checked arithmetic survived review. The explicit-matrix constructor candidate was rejected as a current vulnerability because every caller is a test or measurement tool and no trusted runtime depends on the crate; this must be re-evaluated when node/consensus wiring begins. |
| SIS parameter validation and matrix allocation | Denial of service from attacker-controlled dimensions | Rejected | SisParameters proves representability but does not impose an operational entry ceiling, and derive_instance allocates n\*m entries. The only realistic current source is local argv to the deliberately untrusted developer solver, so impact is self-only and not security-reportable in this revision. Ratified tuple admission or a hard resource ceiling is required before any hostile runtime caller is added. |
| External SIS solver and dependency isolation | Untrusted solver output, parser robustness, unsafe/native dependency leakage, and TCB separation | No issue found | Reviewed both solver manifest and implementation, all argument parsing, kernel construction, LLL conversion, and final checker revalidation. Cargo reverse-dependency evidence shows no node or consensus crate depends on the solver or cj3-classes, and the solver's text protocol is explicitly noncanonical. |
| Rust dependency manifests and lockfile | Dependency confusion, default-feature activation, unsafe-code ingress, and version drift | No issue found | Reviewed Cargo.lock and the changed manifests. Direct dependency versions are exact, default features are disabled where intended, source policy and advisory checks pass, and the workspace zero-unsafe scan is green. |
| P-003 benchmark and estimator evidence pipeline | Research reproducibility and developer-side supply-chain influence | Rejected | Reviewed bench/p003-sis/\*\* and the checker example. The existing runner verifies the estimator Git revision and core estimator hash but can still consume dirty checkout build files and a mutable upstream base tag; this is a reproducibility/governance defect rather than an exploitable product vulnerability because the path is developer-only and results are not admitted automatically. It is queued for pre-merge hardening to a clean checkout and digest-pinned image. |
| Consensus convention and governance integrity | Unauthorized CLASS_ID assignment, wire/derivation invention, and misleading hardness claims | Rejected | The numeric SIS class identifier remains an explicit generic binding, target-tuple security claims are qualified, and no P-006/P-007 values are silently ratified. The protocol does not yet define SHAKE word chunking/endian behavior; the local fixture therefore remains nonnormative and the gap is recorded for G0 rather than treated as deployed consensus behavior. |
