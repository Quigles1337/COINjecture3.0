# COINjecture 3.0

COINjecture 3.0 is a ground-up, derive-don't-trust Proof-of-Useful-Work protocol
program. This repository is pre-testnet software: **no main network configuration
exists**, and no production-readiness or security claim is made.

The current code is the P-001 architecture and CI scaffold only. Crate roots are empty
trust-boundary markers; they do not implement transaction rules, state transition,
beacons, consensus, networking, RPC, storage, or solving behavior.

## Workspace boundaries

| Crate | Intended boundary | P-001 state |
|-------|-------------------|-------------|
| `cj3-types` | Canonical shared types and encodings | Empty scaffold |
| `cj3-kernel` | Transaction validity and atomic state transition | Empty scaffold |
| `cj3-classes` | Admitted problem-class registry and checkers | Empty scaffold |
| `cj3-beacon` | Trait-gated beacon implementations | Empty scaffold |
| `cj3-consensus` | Block validation, fork choice, and retargeting | Empty scaffold |
| `cj3-store` | Chain and authenticated-state storage | Empty scaffold |
| `cj3-net` | Bounded hostile-input networking | Empty scaffold |
| `cj3-rpc` | Authenticated RPC | Empty scaffold |
| `cj3-node` | Binary wiring and startup policy | Empty scaffold |
| `cj3-solver-sis` | Untrusted, out-of-process solver binary | Empty scaffold |

The governing design and its caveats live in `docs/ENGINEERING_PLAN.md`,
`docs/PROTOCOL_SPEC.md`, and `docs/RESEARCH_SURVEY.md`. Ratified decisions and current
execution state live under `loop/`.

## Local P-001 checks

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo deny --locked --all-features check
cargo audit --deny warnings
pwsh -File scripts/ci/verify-geiger.ps1
cargo test --workspace --all-targets --all-features --locked
cargo build --workspace --all-targets --all-features --locked
```

Phase-later CI gates report `NOT YET ADMITTED` with their owning packet. That message
is not a verification claim. Once an owning packet adds a gate handler under
`scripts/ci/active/`, the same blocking job executes that handler and fails closed on
any error.
