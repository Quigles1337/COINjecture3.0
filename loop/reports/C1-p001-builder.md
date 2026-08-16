# Cycle 1 — P-001 Builder Report

**Status:** COMPLETE — MERGED AND MAINLINE GREEN
**Date:** 2026-08-15
**Packet:** P-001 — repository and CI scaffold
**Lane:** AUTO
**Branch:** `feat/p001-repo-scaffold`

## 1. FRAME

### Packet and done-condition

P-001 creates the non-semantic project skeleton described by Engineering Plan §2 and
the blocking CI control plane required by D6. It must add the Rust workspace and empty
crate boundaries, a committed lockfile and dependency policy, origin CI jobs for
formatting, linting, dependency policy/audit, zero-unsafe enforcement, tests, locked
builds, fuzz-gate scaffolding, and the explicitly phase-gated conservation/genesis
checks. It must also document the intended `main` branch protection policy.

The packet is done only when the complete P-001 pipeline exists on GitHub, every
blocking job is green on the origin branch, the crate skeleton contains no protocol
implementation, the adversary pass finds no Critical, and the merge is permitted by a
fresh D17/D11/private-repository re-check.

### Lane classification against the five D17 AUTO conditions

**Classification: AUTO.**

1. **Approved and unblocked:** P-001 is first in the ratified queue and D11 currently
   records COINjecture 2.0 as blocked on Sarah's GATE-1/GATE-2 answers.
2. **Bounded non-semantic surface:** the packet creates only workspace, crate-boundary,
   CI, policy, documentation, and loop-evidence files; it does not implement V-rules,
   STF, fork choice, difficulty, beacon verification, or any other consensus rule.
3. **No formal-spec content:** no `Spec/*.lean` content or vector definitions are in
   scope. The `spec/` boundary may be documented but not populated with Lean semantics.
4. **No decision invention:** the packet implements ratified D6/D8/D12 controls and
   records no new design ratification or consensus choice.
5. **No owned-TBD fill:** it will not assign P-1 through P-12 values, Sarah-owned
   reward/μ-balance inputs, the P-008 URL, or any Al-owned economics value. Phase-later
   tests will be named and fail-closed in CI metadata rather than faked with placeholder
   protocol values.

### Predicted diff surface

- Workspace control: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.gitignore`,
  `deny.toml`, and root `README.md`.
- Empty architecture crates under `crates/cj3-{types,kernel,classes,beacon,consensus,
  store,net,rpc,node,solver-sis}/`, limited to manifests, crate roots, and boundary
  documentation.
- Phase-boundary documentation in `spec/README.md` and `bench/README.md`; no Lean or
  benchmark implementation.
- CI and repository policy under `.github/workflows/`, `.github/`, and `scripts/ci/`.
- Packet evidence only in `loop/STATE.md`, `loop/PACKETS.md`,
  `loop/reports/C1-p001-builder.md`, and `loop/reports/BATCH-LOG.md`.

Any file outside that surface requires a recorded scope expansion before it is
touched. Any expansion into consensus semantics immediately downgrades this packet to
HUMAN and stops autonomous work.

### Top risks

1. **Toolchain/config drift:** current `cargo-audit`, `cargo-deny`, `cargo-geiger`, or
   GitHub Actions behavior may differ from remembered syntax and make the first origin
   pipeline red.
2. **Dishonest phase gating:** conservation, genesis-spend, Lean conformance, or codec
   fuzz work cannot be meaningfully executed before their owning packets; a green
   placeholder must not masquerade as a test.
3. **Semantic leakage:** crate skeletons or example code could accidentally freeze a
   protocol choice, introduce `unsafe`, a mainnet path, floating-point money, or an
   Al/Sarah-owned value.

**Falsifier:** this approach is wrong if the D6 control plane cannot be made blocking
and green on origin without either implementing protocol behavior in P-001 or falsely
claiming that a phase-later invariant/fuzz/genesis test executed.

**Confidence:** MEDIUM. The scope is conventional, but empty-workspace CI tooling and
honest phase gates have enough version-sensitive edges to justify calibration below
HIGH.

## 2. BUILD

Implemented the predicted P-001 surface without entering protocol semantics:

- Added a Rust 2024 workspace with all ten architecture crates from Engineering Plan
  §2. Every crate is private, inherits workspace lints, declares
  `#![forbid(unsafe_code)]`, and otherwise contains only boundary documentation plus
  the empty `main` functions required by the two binary scaffolds.
- Pinned Rust `1.97.1` in `rust-toolchain.toml`. The version was selected from the Rust
  project's official release index current on 2026-08-15 rather than from memory.
- Added `deny.toml` for advisories, bans, licenses, and sources; committed a generated
  dependency-free `Cargo.lock` covering the ten workspace packages.
- Added the serial D6 origin workflow: format → clippy → dependency policy → dependency
  audit → Geiger/source policy → tests → explicit phase gates → locked build. The
  checkout action is commit-pinned to `actions/checkout` v7.0.1.
- Added the nightly dependency-drift/audit workflow and the Phase-0 full-fuzz gate.
- Added a source-policy gate for banned consensus identifiers, production-network
  surfaces, crate entry points, and `forbid(unsafe_code)` declarations.
- Added a Geiger gate that enumerates workspace packages from `cargo metadata`, runs
  cargo-geiger against each absolute manifest, parses JSON metrics, and requires both
  `forbids_unsafe = true` and an aggregate unsafe count of zero per package.
- Added explicit phase-gate dispatch for conservation, Lean conformance, codec fuzz,
  and genesis spend testing. With no admitted handler, the jobs emit
  `NOT_YET_ADMITTED`, name the owning packet/phase, and state that no test execution is
  being claimed. Adding a handler makes the same blocking job execute it and fail
  closed on an error.
- Documented the desired `main` branch protection settings and added an
  evidence-oriented pull-request template.

### Build deviations and corrections

1. The first formatting check found one extra terminal blank line in each empty crate
   root. `cargo fmt --all` made only that mechanical correction.
2. The first Geiger design incorrectly assumed Cargo's virtual-workspace
   `--workspace` convention. cargo-geiger 0.13.0 rejects a virtual manifest and does
   not expose that flag. The gate was corrected to enumerate and scan every package by
   absolute manifest path.
3. The first PowerShell wrapper read a stale `$LASTEXITCODE` after a successful
   PowerShell script invocation. The redundant check was removed; PowerShell errors
   propagate under `ErrorActionPreference = Stop`, while `$LASTEXITCODE` remains used
   only for external cargo processes.
4. Pre-commit review found the same ambiguity in future active phase handlers. Those
   handlers now run in a fresh `pwsh` process, so the dispatcher's exit-code check is
   tied to that handler process rather than inherited shell state.
5. The first origin execution exposed that both `push` on `feat/**` and
   `pull_request` launched the same serial D6 pipeline for one commit. The feature-push
   trigger was removed; feature work now runs once through the PR event, while pushes
   to `main` remain covered.
6. The first origin PR pipeline completed successfully but took about 19 minutes
   because cargo-deny, cargo-audit, and cargo-geiger were compiled from source on every
   cold job. Exact-version binaries are now cached under tool-and-Rust-specific keys
   using official `actions/cache` v6.1.0 pinned to commit
   `55cc8345863c7cc4c66a329aec7e433d2d1c52a9`. Cache misses still install each exact
   version with Cargo's locked dependency graph; cache hits skip only that repeated
   installation step.
7. Adversarial review found an unratified legal assertion in workspace metadata:
   `LicenseRef-Proprietary`. No license choice appears in the governing sources, so the
   field and its ten crate inheritances were removed rather than silently making that
   decision for the project.
8. Adversarial review also found that the public root README named PoUW without
   explicitly acknowledging the governing survey's Ofelimos usefulness ceiling. The
   README now states the at-most-one-half ceiling, the absence of a problem class with
   provably high usefulness, and the requirement to label future hardness claims as
   assumptions with packet evidence.
9. The source-policy gate was strengthened to reject `f32`/`f64` anywhere in a
   `cj3-*` Rust crate and to catch joined or camel-case forms of `MainNetwork`, closing
   easy evasions of A5 and D12 before protocol types exist.

None of these corrections expanded the predicted file surface or entered a
consensus-semantic area.

## 3. VERIFY

The first complete origin PR pipeline passed all eleven serial/blocking jobs for
branch commit `ac7cf776705a6bb41fa9b30892bb75c0db3eb4ee`.

- Run: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31919794372>
- Event/result: `pull_request` / `success`
- GitHub-observed interval: 2026-08-16 01:32:13Z–01:51:17Z
- Phase-gate logs explicitly reported `STATUS=NOT_YET_ADMITTED` and the owning packet
  or phase; they also stated that no named test was claimed to have run.

The run calibrated two CI-process findings described in BUILD deviations 5–6. At that
point, final verification remained pending on the amended workflow; the earlier green
run was not reused as evidence for a changed branch head.

The cache-seed origin run then passed all eleven jobs for calibration commit
`d8b9b24e78bdc557e1c948253ef2a4d62a29f4e0`:

- Run: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31920639189>
- Event/result: `pull_request` / `success`
- GitHub-observed interval: 2026-08-16 01:54:40Z–02:13:44Z
- Durable PR-ref caches created: cargo-deny 0.20.2 (4,786,069 bytes), cargo-audit
  0.22.2 (7,780,154 bytes), and cargo-geiger 0.13.0 (10,364,091 bytes), each keyed to
  Linux and Rust 1.97.1.

That run proves the cache-miss/install/save path. It predates the adversary fixes and
therefore is not the final-head verification; the next origin run must prove cache
hits and the complete corrected diff.

The complete post-adversary branch head then passed all eleven jobs:

- Run: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31921481384>
- Exact head: `a454f15c1d31c928dbe68dbb986061d580f347df`
- Event/result: `pull_request` / `success`
- GitHub-observed interval: 2026-08-16 02:15:22Z–02:18:02Z
- All three restore steps were cache hits, all three corresponding install steps were
  `skipped`, and dependency policy/audit plus per-package Geiger executed successfully.

This is the authoritative P-001 branch verification. The post-merge mainline run also
passed all eleven jobs on exact merge SHA
`8367de08bb3d3766bf49b9970eb3109fd1af4389`:

- Run: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31921663254>
- Event/result: `push` / `success`
- GitHub-observed interval: 2026-08-16 02:20:15Z–02:35:57Z

The cold mainline run created all three default-branch tool caches. P-001 therefore
has both exact-head pre-merge evidence and exact-merge-SHA post-merge evidence.

## 4. ADVERSARY PASS

**Seat switch: ADVERSARY.** The complete 38-file diff was re-read from `main` to the
packet head, followed by mechanical surface, policy, file-mode, credential-shape, and
action-pinning sweeps.

### Findings, recorded verbatim

1. **Critical — A10 honest-claims omission (fixed):** “The public README calls the
   project Proof of Useful Work but does not itself acknowledge the governing
   Ofelimos `≤½` usefulness ceiling. A reader can encounter marketing language without
   its required limiting fact.” Fix: added the ceiling, the lack of a problem class
   with provably high usefulness, and the assumption/evidence rule to `README.md`.
2. **Critical — unratified project decision (fixed):** “`LicenseRef-Proprietary`
   silently selects legal metadata absent from Al's rulings and both governing design
   documents. P-001 has no authority to make that licensing decision.” Fix: removed
   the workspace license field and all crate inheritances; no replacement was guessed.
3. **Non-critical process finding (fixed):** “One feature commit launches duplicate
   serial pipelines through both `push` and `pull_request`, doubling cost without
   independent evidence.” Fix: feature branches now run through the PR trigger only;
   `main` retains its push trigger.
4. **Non-critical availability finding (fixed):** “Cold-compiling the three pinned
   audit tools consumes nearly the entire serial pipeline and magnifies the cost of
   every evidence-only commit.” Fix: added exact-version, exact-Rust cache keys via the
   commit-pinned official cache action; cache misses preserve locked installs.
5. **Accepted residual — honest phase deferrals:** “Four required status names are
   green today even though their future-phase tests do not yet exist.” This is not
   closed by pretending they ran: each log emits `NOT_YET_ADMITTED`, names its owner,
   and disclaims test execution; the owning packet must add the active handler. This
   residual remains visible until P-005/P-007/Phase 1 activation.
6. **Accepted residual — repository setting:** “GitHub's protection readback returns
   HTTP 404 / `Branch not protected` for `main`.” P-001's explicit done-condition is
   to document the protection configuration, which `.github/BRANCH_PROTECTION.md`
   does. Active protection is not claimed; the autonomous merge protocol remains the
   current enforcement layer.

### Axiom and hostile-input sweep

- **A1–A4:** no consensus structure, instance, score, quality, fork-choice, or miner
  metadata exists. The Rust sweep found none of `solve_time`, `work_score`,
  `reported_*`, or `self_reported`.
- **A5:** no amount or arithmetic implementation exists; the strengthened CI policy
  rejects `f32` and `f64` throughout every core crate.
- **A6–A7:** no apply path, state mutation, RPC endpoint, secret, or input codec exists.
- **A8:** no `.lean` file or vector definition exists; `spec/README.md` only marks the
  future HUMAN-lane boundary.
- **A9:** all ten implementation roots are Rust, no C/C++/Python/build-script/FFI file
  exists, every entry point forbids unsafe code, and Geiger reports zero unsafe items
  in every package.
- **A10:** the corrected README now carries the required caveats and makes no hardness,
  readiness, or security claim.
- **A11:** every green claim in this report points to a run URL or a committed artifact;
  future-phase jobs explicitly deny that their tests ran.
- **TBD integrity:** no governing plan/spec/decision value changed, no Al- or
  Sarah-owned value was filled, and P-008 remains blocked on the exact frontend URL.
- **Overflow/parser/injection:** there is no amount math or protocol codec. The only
  parser consumes Cargo-owned JSON with `ConvertFrom-Json`; resolved manifest paths
  are passed as one process argument. Gate input is a closed `ValidateSet`. No eval,
  command-string construction, or PR-event interpolation exists. All 17 workflow
  action references use full 40-hex commits and workflow permissions are read-only.

**Adversary result:** the two Critical findings were fixed and re-verified by the
complete origin pipeline. No known Critical remains.

## 5. MERGE

At merge time, the following were re-derived rather than carried forward from FRAME:

1. P-001 remained approved and unblocked; D11 still recorded COINjecture 2.0 blocked
   on Sarah's GATE-1/GATE-2 answers.
2. The exact diff remained bounded to the predicted scaffold/CI/evidence surface and
   every Rust root contained only boundary documentation, `forbid(unsafe_code)`, and
   the two empty binary entry points.
3. No `.lean` file or vector definition existed.
4. Governing documents and the LEDGER were unchanged by P-001, and the unratified
   license assertion had been removed; no new decision remained.
5. No Al- or Sarah-owned TBD was filled.

The same check re-read repository visibility as `PRIVATE`, confirmed canonical and
identical fetch/push remotes, matched local/origin/PR head at
`a454f15c1d31c928dbe68dbb986061d580f347df`, observed all eleven CI jobs green with
three cache hits, and read the PR as mergeable.

- PR: <https://github.com/Quigles1337/COINjecture3.0/pull/1>
- Merge time: 2026-08-16 02:20:13Z
- Merge SHA: `8367de08bb3d3766bf49b9970eb3109fd1af4389`
- Authoritative branch CI:
  <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31921481384>
- Post-merge `main` CI:
  <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31921663254> — `success`
  on exact merge SHA, all eleven jobs green.

Because a commit cannot contain its own future merge SHA, this merge record is being
ferried back through the evidence-only `feat/p001-closeout` branch and a separate PR;
there is no direct post-bootstrap push to `main`.

## 6. CALIBRATE

### Predictions versus outcomes

- **Diff surface:** the prediction held exactly at the category level: workspace
  control, ten empty crate boundaries, CI/policy, `spec/` and `bench/` boundary docs,
  repository policy, and loop evidence. The implementation merge contained 38 files,
  1,088 insertions, and 10 deletions relative to bootstrapped `main`; no file escaped
  the predicted surface and no protocol-semantic content landed.
- **Risk 1 — toolchain/config drift:** materialized. cargo-geiger 0.13.0 rejected the
  virtual-workspace invocation, and PowerShell exit-code semantics required two
  corrections. Exact local execution caught both before the first push.
- **Risk 2 — dishonest phase gating:** did not become a false claim. The jobs are green
  only as explicit deferrals, their logs name owners and say the tests did not run,
  and this remains an accepted residual until the owning packets activate them.
- **Risk 3 — semantic leakage:** no protocol behavior leaked in. The adversary instead
  found two non-code authority/claims leaks—the unratified license field and missing
  A10 usefulness disclosure—which were both treated as Critical and removed/corrected.
- **Confidence calibration:** MEDIUM was appropriate. The architecture and bounded
  surface held, but version-sensitive CI edges and two governance/claims findings
  justified the caution; all were repairable without scope expansion.
- **Surprise:** compiling three pinned audit tools dominated a cold serial run, while
  cache hits reduced the final corrected PR pipeline from roughly 19 minutes to under
  3 minutes. Duplicate push/PR triggers initially doubled that cost.

**One process improvement for P-002:** before its first research or implementation
commit, perform a dedicated authority-and-claims sweep of public prose and package
metadata against D17 and A10. That single pre-commit pass would have caught both P-001
Criticals before origin verification.

## VERIFIED

- The packet branch was created from bootstrapped `main` at
  `28beb55623835981cb60ec3575569010c8fb6263`.
  Evidence: Git history and the bootstrap command output retained in the session.
- Repository visibility was re-read as `PRIVATE` immediately before branch creation.
  Evidence: `gh repo view Quigles1337/COINjecture3.0 --json visibility`.
- D11 clearance and P-001 approval are committed.
  Evidence: `loop/STATE.md`, `loop/PACKETS.md`, and `loop/LEDGER.md`.
- Rust `1.97.1` was the latest official release listed at selection time.
  Evidence: <https://blog.rust-lang.org/releases/> (official Rust release index,
  observed 2026-08-15).
- Tool versions selected for CI matched current primary package/release metadata:
  cargo-deny 0.20.2, cargo-audit 0.22.2, cargo-geiger 0.13.0, and
  actions/checkout v7.0.1 at commit
  `3d3c42e5aac5ba805825da76410c181273ba90b1`.
  Evidence: crates.io metadata and the corresponding official GitHub repositories,
  queried during BUILD.
- Local formatting, clippy, cargo-deny, cargo-audit, source policy, per-package Geiger,
  workspace tests, explicit phase gates, and locked build completed successfully.
  Evidence: this session's command output.
- Geiger reported `UNSAFE_COUNT=0` and `FORBIDS_UNSAFE=true` for each of the ten
  workspace packages.
  Evidence: `scripts/ci/verify-geiger.ps1` output retained in this session.
- The complete initial origin pipeline succeeded for the first P-001 build commit.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31919794372>.
- The cache-seed pipeline succeeded and GitHub lists all three exact cache entries on
  the PR merge ref.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31920639189>
  and `gh cache list --ref refs/pull/1/merge` output recorded above.
- `actions/cache` v6.1.0 resolves to commit
  `55cc8345863c7cc4c66a329aec7e433d2d1c52a9`.
  Evidence: GitHub's `actions/cache` release and Git-ref APIs queried on 2026-08-15.
- The post-adversary local D6 sequence passed after removing unratified license
  metadata, correcting A10 disclosure, and strengthening source policy.
  Evidence: this session's command output and the changed files themselves.
- The mechanical adversary sweep found exactly 38 changed files, no deletions, no
  out-of-surface path, ten Rust entry points, no formal/polyglot/native file, no
  injection-prone script construct, no credential-shaped text, no special Git mode,
  and ten workspace members in `Cargo.lock`.
  Evidence: this session's recorded adversary command output.
- The exact post-adversary branch head passed all eleven origin jobs with all three
  install steps skipped on verified cache hits.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31921481384>.
- PR #1 merged that exact head into `main` as
  `8367de08bb3d3766bf49b9970eb3109fd1af4389`.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/pull/1> and matching local,
  origin, and PR API readback.
- The post-merge `main` pipeline passed all eleven jobs on the exact merge SHA and
  created all three default-branch tool caches.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31921663254>
  and `gh cache list --ref refs/heads/main`.

## ASSUMED

- Phase-later D6 gates may report an explicit `NOT YET ADMITTED` state only when the
  owning packet is named and the gate becomes fail-closed once its activation marker
  is committed. This preserves honest claims while P-001 remains non-semantic.

## UNKNOWN

- Whether repository-plan permissions allow applying branch protection through the
  API. Current evidence is HTTP 404 / `Branch not protected`; P-001 requires a
  documented configuration, and active protection will not be claimed unless a future
  setting change is read back from GitHub.
