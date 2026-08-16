# Cycle 2 — P-002 Builder Report

**Status:** ORIGIN CI GREEN — FINAL EVIDENCE HEAD PENDING
**Date:** 2026-08-15
**Packet:** P-002 — beacon spike
**Lane:** AUTO
**Branch:** `feat/p002-beacon-spike`

## 1. FRAME

### Packet and done-condition

P-002 surveys the currently available pure-Rust paths for a trustless verifiable delay
function, compares Wesolowski and Pietrzak proofs and class-group versus repeated-
squaring constructions, analyzes parent grinding and last-block withholding, and
executes D14's delegated technology selection. It then defines the bounded
`cj3-beacon` interface and an explicitly insecure iterated-hash devnet placeholder
behind that interface.

The packet is done only when:

- the recommendation is supported by primary-source evidence and explicitly separates
  an algorithm/construction choice from implementation-readiness claims;
- every evaluated implementation path is checked for A9 compatibility, including
  transitive native-code, FFI, `unsafe`, GMP, and toolchain requirements;
- the grinding analysis covers both pre-publication parent-field search and
  last-block withholding, without claiming that a VDF alone eliminates either;
- a minimal trait preserves Protocol Spec §4's parent-to-output and cheap-verification
  shape without inventing `Header`, domain-tag bytes, `TARGET_BLOCK_TIME`, a round
  count, production parameters, or final wire encoding owned by later packets;
- the devnet placeholder carries the exact banner
  `NOT-TESTNET-GRADE: sequentiality assumption only`, is opt-in, and fails compilation
  when combined with a testnet-tagged build;
- local and origin D6 verification pass, an adversary pass leaves no Critical finding,
  and a fresh D11/D17/private-repository check permits merge.

### Lane classification against the five D17 AUTO conditions

**Classification: AUTO.**

1. **Approved and unblocked:** P-002 is the approved NEXT packet after completed
   P-001, and `loop/STATE.md` still records COINjecture 2.0 as blocked on Sarah's
   GATE-1/GATE-2 answers.
2. **Bounded spike surface:** the packet is limited to research evidence, a beacon
   abstraction, and the already authorized devnet-only placeholder. It does not add a
   production VDF implementation, consensus integration, fork-choice behavior, block
   timing, or state-transition logic.
3. **No formal-spec content:** no `Spec/*.lean` content and no protocol vectors are in
   scope.
4. **No decision invention:** ratified D14 explicitly delegates the final VDF
   technology selection to P-002. Recording that evidence-backed selection executes
   the delegation; it does not ratify a new decision or change LEDGER status. If the
   evidence requires a trusted setup, a new trust assumption, a final parameter,
   domain-byte assignment, or consensus behavior outside D14, that work is HUMAN-lane
   and this packet stops rather than deciding it.
5. **No owned-TBD fill:** P-002 will not assign `TARGET_BLOCK_TIME`, VDF delay/round
   parameters, `D_BEACON` bytes, canonical `Header` encoding, economics, or any Al- or
   Sarah-owned value. The interface will consume already prepared parent material or
   use associated types so those later owners remain authoritative.

### Predicted diff surface

- Research, build, verification, adversary, merge, and calibration evidence in
  `loop/reports/C2-p002-builder.md`.
- Beacon-only implementation under `crates/cj3-beacon/`, expected to include its
  manifest, crate root, and a feature-gated devnet-placeholder module.
- `Cargo.lock` only if an accepted pure-Rust dependency is necessary for the bounded
  placeholder or tests. A surveyed production VDF library will not be added merely to
  demonstrate discovery.
- Packet-boundary bookkeeping in `loop/STATE.md`, `loop/PACKETS.md`, and
  `loop/reports/BATCH-LOG.md` after implementation evidence is complete.

No edit to `loop/LEDGER.md`, `docs/PROTOCOL_SPEC.md`, `Spec/`, `crates/cj3-types/`, or
another crate is predicted. Any need to touch those surfaces or define production
beacon parameters triggers a scope reclassification before the edit occurs.

### Authority-and-claims pre-check

- The selected technology will be described as D14's P-002 recommendation/selection,
  not as a production-ready or audited implementation unless evidence establishes
  that stronger claim.
- The iterated-hash module will be called a devnet placeholder in both code and
  documentation. It will make no cryptographic sequentiality, testnet-readiness, or
  adversarial-security claim beyond the exact mandatory warning.
- A pure-Rust source tree is not sufficient evidence by itself: dependency metadata,
  build scripts, native libraries, FFI declarations, and `unsafe` usage must also be
  examined before A9 compatibility is claimed.

### Top risks

1. **Hidden native or unsafe implementation boundary:** an apparently Rust-facing VDF
   crate may transitively require C/C++, GMP, a build script, FFI, or substantial
   `unsafe`, violating A9 even if its public API is Rust.
2. **Premature semantic freeze:** a concrete `Header`, domain separator, output
   encoding, delay count, or verification error model could take authority away from
   P-007, P-006, or Gate G0 and quietly turn a spike into consensus design.
3. **Placeholder escape or overclaim:** the devnet iterated-hash implementation could
   compile in a testnet-tagged build, be selected by default, or be mistaken for a
   production VDF despite lacking a succinct proof and robust sequentiality analysis.

**Falsifier:** this plan is wrong if no surveyed construction has a credible route to
no trusted party, inherently sequential evaluation, cheap verification, and the A9
pure-Rust boundary, or if the required trait cannot be expressed without assigning an
unresolved protocol type or parameter. In that case P-002 will preserve the negative
research result, omit any unsupported production selection or implementation, and
stop at the governing decision boundary.

**Confidence:** MEDIUM. The interface and devnet feature boundary are small, but the
current implementation ecosystem, transitive dependency audit, and construction-
level withholding analysis are specialized and evidence-sensitive.

## 2. BUILD

### Primary-source construction comparison

The survey used the original construction papers, authors' publication records, and
the source/manifests of candidate implementations rather than package descriptions
alone.

| Choice | Trust/setup | Proof and verifier | P-002 assessment |
|---|---|---|---|
| RSW repeated squaring in an RSA group | Whoever knows the modulus factorization can reduce the exponent and skip the delay. Avoiding that trapdoor requires a trusted generator or a separately governed MPC ceremony. | Both Wesolowski and Pietrzak make the RSW relation publicly verifiable. | Rejected for CJ3: a ceremony or trusted modulus conflicts with §4's “no trusted party” requirement and would add an unratified setup protocol. |
| Repeated squaring in an imaginary-quadratic class group | A discriminant can be derived transparently; no party need know the group order. | Supports the same proof families. Arithmetic is more complex and slower, but Chia provides deployed engineering evidence for this route. | Selected group family: it satisfies the no-trusted-party boundary without adding a ceremony. Final discriminant size and derivation encoding remain Gate-G0/P-007 parameters, not P-002 inventions. |
| Wesolowski proof | Adds a computational adaptive-root-style assumption to repeated squaring. | Result and proof are each one group element; verification is the smaller/faster of the two surveyed proof systems. | Selected proof family: best fit for a per-block, cheap verifier and the strongest current implementation/interoperability path. |
| Pietrzak proof | The interactive protocol has statistical soundness; the non-interactive VDF uses Fiat–Shamir. Applying it outside the paper's RSA setting requires careful assumption review. | Roughly `O(log T)` group elements/rounds. The paper reports about 10 KiB and roughly three RSA exponentiations for its representative parameters; proof-generation overhead is lower than Wesolowski's asymptotically. | Retained as the fallback if later cryptanalysis or implementation evidence defeats Wesolowski, but not selected because proof size, verifier work, and current pure-Rust interoperability are worse for CJ3. |

The original Pietrzak paper describes `y = x^(2^T)` as requiring `T` sequential
squarings when the group order is unknown and directly compares its proof with
Wesolowski's single-group-element proof:
<https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITCS.2019.60>.
Wesolowski's publication record states that imaginary-quadratic class groups permit
setup with an unknown trapdoor and that the result and proof are each one group
element: <https://ir.cwi.nl/pub/28683>. Chia's implementation guide independently
documents repeated squaring in ideal class groups and the reason an unknown group
order matters: <https://chia-network.github.io/2018/11/07/chia-vdf-competition-guide.en.html>.

### D14 technology selection

**Selected by P-002 under ratified D14: Wesolowski proof of repeated squaring in a
transparently derived imaginary-quadratic class group.**

This selects the production construction, not a production-ready crate, parameter
set, or wire format. In particular, P-002 does **not** select a discriminant bit size,
Fiat–Shamir challenge size, iteration count, reference hardware, or the bytes used to
derive the class-group discriminant. Those values affect security or consensus and
must be made explicit at their owning packet/Gate G0 rather than copied from Chia.

The selection also exposes an ambiguity in Protocol Spec §4: cheap Wesolowski
verification needs both the VDF result and proof, while the pseudocode comments
`BeaconOut` as 32 bytes. A 32-byte random digest can be derived from the result, but
it is not by itself the proof carrier. The P-002 trait therefore leaves its associated
output type opaque. P-007/Gate G0 must define the canonical envelope for the 32-byte
randomness and the VDF witness; P-002 does not silently enlarge or reinterpret the
wire type.

### Current Rust implementation survey (observed 2026-08-15)

- [`Chia-Network/chiavdf`](https://github.com/Chia-Network/chiavdf) is the strongest
  deployed implementation evidence for the selected construction, but its Rust
  package is a `bindgen`/CMake wrapper over C++ and links GMP. It is A9-forbidden on
  CJ3's trusted path.
- [`poanetwork/vdf`](https://github.com/poanetwork/vdf) and its 2026 maintained fork
  [`vdf-rs`](https://github.com/jose-compu/vdf-rs) implement class-group Wesolowski
  and Pietrzak behind Rust APIs, but both explicitly require GMP. The original last
  pushed code in 2021; the fork does not remove the native boundary.
- [`MystenLabs/fastcrypto/fastcrypto-vdf`](https://github.com/MystenLabs/fastcrypto/tree/main/fastcrypto-vdf)
  contains evaluator and verifier code for class-group Wesolowski using `num-bigint`,
  but labels the crate experimental. As packaged, it unconditionally depends on the
  broad `fastcrypto` crate, whose manifest includes native-backed `secp256k1` and
  `blst`; it is also consumed from Git rather than a published crate. Importing it
  now would violate the strict A9/source-policy boundary and the committed source
  policy.
- [`richardkiss/chia-vdf-verify`](https://github.com/richardkiss/chia-vdf-verify)
  is a safe, pure-Rust verifier port with Chia vectors and no C/GMP dependency, but it
  is verifier-only, single-maintainer, not published on crates.io, and has no recorded
  independent audit.
- [`kyn-vdf` 0.1.1](https://crates.io/crates/kyn-vdf/0.1.1) is another pure-Rust,
  Chia-compatible verifier, but it is verifier-only, was first published twelve days
  before this survey, had 41 aggregate downloads at observation time, and includes an
  unrelated unconditional `wasm = "0.0.0"` dependency. It is useful evidence that a
  safe verifier is feasible, not sufficient production evidence.
- [`pso-vdf` 0.2.3](https://github.com/psonet/pso-vdf) is pure Rust but is not a
  valid substitute. Its one-state known-order-field map explicitly observes that
  `T` iterations equal `x^(e^T mod (p-1))`; computing that exponent and performing one
  exponentiation skips the advertised serial loop. More generally, the Ethereum
  Foundation's public MinRoot assessment reports parallel speedups against the actual
  two-state MinRoot design and does not recommend VDF use there:
  <https://ethresear.ch/t/statement-regarding-the-public-report-on-the-analysis-of-minroot/16670>.
- `rsa-vdf`, `torpor`, and RSW server puzzles either depend on a factorization/public
  setup or let the setup party shortcut evaluation. Arweave's Rust hash-chain
  validator uses vendored OpenSSL and verifies `O(T)` checkpoints rather than a cheap
  succinct proof. They do not meet the combined §4/A9 requirements.

**Implementation conclusion:** no surveyed dependency is admitted to CJ3 in P-002.
The credible path is to isolate, audit, and independently test a safe-Rust evaluator
and verifier for the selected construction in a later explicitly scoped packet,
using Chia-compatible differential vectors without linking Chia's native code into
the node. D14's planned devnet placeholder is therefore necessary, not evidence that
the production VDF already exists.

### Grinding and last-block-withholding analysis

Let `T` be the eventual iteration count, `delta(T)` the measured honest evaluation
time on the eventual reference hardware, `a` an attacker's serial speed advantage,
`q` its number of independent VDF engines, `k` the number of independently mutable
parent candidates, and `p` the baseline probability of a chosen “favourable” output
event. These are variables only; P-002 assigns none of their unresolved values.

1. **Published-parent fixity.** Once canonical parent bytes and production parameters
   are fixed, repeated squaring has one result. Wesolowski proof non-uniqueness, if
   any, does not create a second random output because randomness must be derived from
   the result, not the witness. Every verifier must recompute the same input mapping.
2. **Parent-field grinding.** One candidate costs `T` dependent squarings and at least
   about `delta(T)/a` wall time on one attacker engine. Testing `k` candidates costs
   aggregate work `k*T`; with `q` independent engines the idealized wall-clock lower
   bound is `ceil(k/q) * delta(T)/a`. Sequentiality prevents parallelizing one chain,
   but it does **not** prevent embarrassingly parallel evaluation of different
   parents. If all `k` outputs can be tested before a deadline, the probability of at
   least one favourable output rises from `p` to `1 - (1-p)^k`.
3. **Cheap fields.** Protocol Spec §9 correctly excludes timestamp from instance-seed
   inputs. Every other miner-adjustable parent byte that affects `parent_hash` still
   creates a candidate, but now requires both a valid parent and a full VDF evaluation.
   Canonical encoding and rejection of unknown fields in P-007 are therefore part of
   the grinding boundary, not mere codec hygiene.
4. **Last-block withholding.** A miner that finds a parent can keep it private, run
   the VDF, and publish only after seeing a favourable child seed. The VDF does not
   remove that selective-abort option. If competing honest blocks arrive as a Poisson
   process with rate `lambda_h`, an idealized private candidate survives an evaluation
   lasting `delta(T)/a` with probability `exp(-lambda_h * delta(T)/a)`. Its probability
   of both surviving and satisfying the attacker's predicate is therefore
   `p * exp(-lambda_h * delta(T)/a)`. Ignoring race losses, repeated independent trials
   need `1/p` candidates in expectation, discard `(1-p)/p` parents, and consume
   aggregate VDF work `T/p`; real fork choice and block races only add costs.
5. **Residual risk and owner.** The miner may sacrifice its parent reward and lose the
   chain race, but a sufficiently valuable downstream lottery can still justify that
   sacrifice. Exact bias depends on mining share, propagation, fork choice, reward,
   `TARGET_BLOCK_TIME`, calibrated hardware advantage, and VDF delay. P-006 can model
   those variables once their owners supply them. A seed from an older/finalized
   ancestor, external entropy contributions, or a slashable commitment could reduce
   withholding leverage, but each changes protocol latency/trust/economics and is a
   Gate-G0/HUMAN decision rather than a P-002 code change.

The implementation will now add only the associated-type `Beacon` boundary and the
feature-gated, generic iterated-hash devnet placeholder. The placeholder will consume
an already prepared 32-byte parent seed, so P-007 remains the sole owner of
`H(D_BEACON || parent_hash)`, and it will take a nonzero iteration count supplied by
the caller rather than inventing a consensus delay.

### Implemented spike boundary

- Added an instance-based `Beacon` trait with associated `Parent` and `Output` types.
  The methods retain the spec's `output(parent)` / `verify(parent, output) -> bool`
  shape. `&self` permits immutable configuration without a global or invented
  constant.
- Kept `Output` opaque so a production implementation can carry both the 32-byte
  derived randomness and its VDF witness after P-007/Gate G0 define the envelope.
- Added the opt-in `devnet-placeholder` feature. It is not a default feature and adds
  no dependency.
- Added `devnet::IteratedHash<D>`, parameterized by a `Digest32` provider and a
  `NonZeroU64` iteration count. It accepts an already prepared 32-byte parent seed,
  performs exactly the configured number of digest calls, and verifies by
  recomputation. The absence of a succinct proof is documented as a deliberate
  testnet-disqualifying limitation.
- Exported the exact warning text as `devnet::SECURITY_BANNER` and repeated it in the
  module documentation and testnet compile error.
- Registered the custom compile-time tag `cj3_testnet` through a safe, dependency-free
  build script. A build with both that tag and `devnet-placeholder` fails at compile
  time; a tagged build without the placeholder remains valid.
- Added three unit tests covering deterministic output, successful verification,
  rejection of a different output, the immutable nonzero iteration count, and exact
  warning text.

No dependency, `Cargo.lock` change, production VDF code, hash implementation, domain
derivation, concrete header, wire format, or timing value was added.

## 3. VERIFY

### Focused verification

- `cargo clippy -p cj3-beacon --all-targets --all-features --locked -- -D warnings`
  passed.
- `cargo test -p cj3-beacon --all-targets --all-features --locked` passed all three
  tests.
- `cargo check -p cj3-beacon --no-default-features --locked` passed.
- With `RUSTFLAGS="--cfg cj3_testnet"`, the boundary compiled without the placeholder,
  while the same build with `--features devnet-placeholder` failed with the exact
  mandatory warning. The harness treated that failure as the expected pass condition
  and emitted `TESTNET_PLACEHOLDER_GATE=PASS expected_compile_failure=true`.

### Full local D6 sequence

The complete local sequence passed after implementation and before the adversary
seat:

1. format check;
2. workspace/all-target/all-feature Clippy with warnings denied;
3. locked cargo-deny policy;
4. cargo-audit with warnings denied;
5. source policy and per-package cargo-geiger;
6. workspace/all-target/all-feature tests;
7. all four explicit phase gates;
8. locked workspace/all-target/all-feature build.

Source policy examined 12 Rust files. Geiger reported `UNSAFE_COUNT=0` and
`FORBIDS_UNSAFE=true` for every one of the ten workspace packages. The beacon's three
tests passed; the remaining scaffold crates still contain no tests. Conservation,
Lean conformance, codec fuzz, and genesis spend each reported their existing
`NOT_YET_ADMITTED` state and owner rather than claiming execution.

`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
also passed during the mechanical adversary sweep.

After the adversary wording/configuration hardening, the complete D6 sequence and
the positive/negative testnet-tag matrix were run again. They ended in
`POST_ADVERSARY_LOCAL_D6=PASS` and `POST_ADVERSARY_TESTNET_GATE=PASS`.

### First exact-head origin run

The complete serial D6 workflow passed all eleven jobs for the intentional feature
commit:

- Run: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31923287404>
- Exact head: `8b1c8d601629a499b47ae4ab9e904dd0949fcd65`
- Event/result: `pull_request` / `success`
- GitHub-observed interval: 2026-08-16 03:02:29Z–03:05:11Z
- The cargo-deny, cargo-audit, and cargo-geiger restore steps all succeeded; all three
  corresponding install steps were skipped on cache hits.

The jobs were format, Clippy, dependency policy, dependency audit, unsafe audit,
unit/property tests, the four explicit phase gates, and locked build. This run proves
the implementation commit on origin. Because adding this evidence to the report
changes the PR head, a second complete exact-head run is required before merge.

## 4. ADVERSARY PASS

**Seat switch: ADVERSARY.** The research recommendation, five-file working-tree
surface, public API, feature combinations, compile-time exclusion, documentation, and
D17 authority boundary were re-read independently of the BUILD rationale.

### Findings

1. **Critical ecosystem candidate — known-order shortcut (rejected):** `pso-vdf`
   advertises its one-state fifth-root loop as a VDF while its own source derives the
   closed form `x^(e^T mod (p-1))`. Modular exponentiation computes `e^T mod (p-1)` in
   logarithmic rather than `T` sequential work, after which one field exponentiation
   computes the output. It cannot satisfy §4 and was explicitly rejected rather than
   imported based on its “pure Rust” label.
2. **Critical interface ambiguity — 32-byte output versus proof (contained):** a
   literal `[u8; 32]` production `BeaconOut` cannot also carry the Wesolowski witness
   needed for cheap verification. Freezing that type in P-002 would either make
   verification `O(T)` or invent a second wire field. The associated `Output` remains
   opaque and the ambiguity is promoted to P-007/Gate G0. The devnet-only output can
   remain 32 bytes because it openly recomputes and is testnet-forbidden.
3. **Non-critical build-contract finding (documented and tested):** a custom cfg gate
   cannot force a future build system to label itself correctly. The crate now states
   that every testnet build profile must set `cj3_testnet`; once set, the compiler
   mechanically rejects the placeholder. P-007/D12 integration must make the tag part
   of the testnet build contract.
4. **Non-critical A10 wording finding (fixed):** the first constant comment said the
   warning was “carried by every” consumer integration, which code cannot enforce.
   It now supplies the exact text consumers *must surface* without claiming that a
   library constant controls their UI or logs.
5. **Accepted residual — implementation readiness:** two new safe-Rust Chia-compatible
   verifiers are useful feasibility evidence, but both are verifier-only,
   single-maintainer, and unaudited; the full evaluator/prover choices remain native-
   backed or experimental. No production-readiness claim or dependency was added.
6. **Accepted residual — withholding and hardware parallelism:** the chosen VDF makes
   each candidate sequential but does not prevent separate engines from testing
   separate parents, nor does it remove a winning miner's publish/abort option. The
   symbolic bounds remain in this report; numeric calibration belongs to P-006 and
   the protocol choice remains visible at Gate G0.

### Axiom, hostile-input, and authority sweep

- **A1–A4:** no problem class, quality, fork choice, reward, solve-time, or work-score
  behavior exists in the diff.
- **A5:** no floating-point type or amount arithmetic exists. The only numeric
  configuration is a nonzero hash-chain iteration count; it is local trusted config,
  not decoded wire input, and no default value is supplied.
- **A6–A7:** no state mutation, RPC, parser, secret, or network input path was added.
- **A8:** no Lean or vector file changed.
- **A9:** all implementation files are Rust, both crate and build-script roots forbid
  unsafe code, no FFI/build dependency exists, and Geiger reports zero unsafe items.
- **A10:** the devnet limitation, absence of succinct verification, production-library
  gap, cryptographic assumptions, and withholding residual are explicit. The report
  does not turn source availability into an audit/readiness claim.
- **A11/TBD integrity:** no delay, target time, discriminant size, challenge size,
  hash choice, domain bytes, header encoding, or economics value was assigned.
- **Feature/DoS boundary:** the feature defaults off; `NonZeroU64` excludes a zero
  chain; docs forbid deriving the iteration count from untrusted data; the exact
  tagged combination fails compilation. An untagged devnet caller can still choose a
  very large local count, which is configuration error rather than remote input.
- **Diff authority:** only `crates/cj3-beacon/{Cargo.toml,build.rs,src/lib.rs,
  src/devnet.rs}` and this report changed. `loop/LEDGER.md`, the protocol/engineering
  documents, `Spec/`, `cj3-types`, all other crates, and `Cargo.lock` are untouched.

**Adversary result:** both Critical hazards are rejected/contained, the A10 wording
finding is fixed, and no known Critical remains. The post-fix local checks and origin
CI must still verify the final exact head.

## 5. MERGE

Pending.

## 6. CALIBRATE

Pending.

## VERIFIED

- P-002 was `NEXT — APPROVED` when picked up, and P-001's closeout was already on
  `main`.
  Evidence: `loop/PACKETS.md` and `loop/STATE.md` at
  `ad480d620e881ea2fc16bd4415c7120b0411afef`.
- The packet branch was created from a clean local `main` exactly matching
  `origin/main` at `ad480d620e881ea2fc16bd4415c7120b0411afef`.
  Evidence: pickup command output retained in this session.
- Wesolowski's result/proof are each one group element and the construction supports
  unknown-order class groups without a known setup trapdoor.
  Evidence: <https://ir.cwi.nl/pub/28683> and
  <https://eprint.iacr.org/2018/623.pdf>.
- Pietrzak's RSW proof uses `T` sequential squarings, reports an approximately 10 KiB
  proof and roughly three RSA exponentiations for representative parameters, and
  directly identifies Wesolowski's proof-size/verification advantage.
  Evidence: <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITCS.2019.60>.
- Chia's primary implementation guide selects repeated squaring in ideal class groups
  because their order is unknown and documents per-iteration composition.
  Evidence:
  <https://chia-network.github.io/2018/11/07/chia-vdf-competition-guide.en.html>.
- Chia's current Rust binding uses CMake/bindgen and links native code/GMP; the
  POA/vdf-rs implementation families explicitly require GMP.
  Evidence: candidate manifests, build scripts, READMEs, and source trees inspected
  from their official repositories and crates.io packages on 2026-08-15.
- `fastcrypto-vdf` is feature-labeled experimental and its package manifest depends
  unconditionally on `fastcrypto`; the latter includes `secp256k1` and `blst`.
  Evidence:
  <https://github.com/MystenLabs/fastcrypto/tree/main/fastcrypto-vdf> and the exact
  official manifests read through the GitHub API.
- `chia-vdf-verify` and `kyn-vdf` are verifier-only pure-Rust efforts; the former was
  not published on crates.io, while crates.io reported `kyn-vdf` 0.1.1 at 41 total
  downloads, first published 2026-08-03.
  Evidence: official repository trees/manifests and crates.io API readback on
  2026-08-15.
- `pso-vdf` source explicitly reduces `T` applications of its one-state map to
  `x^(e^T mod (p-1))`, exposing the non-sequential shortcut used to reject it.
  Evidence: <https://github.com/psonet/pso-vdf/blob/main/src/minroot.rs>.
- The focused beacon Clippy and tests passed, including all three devnet tests.
  Evidence: local command output retained in this session.
- The custom `cj3_testnet` gate passed its positive/negative matrix: ordinary and
  tagged-without-placeholder builds succeeded; tagged-with-placeholder compilation
  failed with the exact mandatory warning.
  Evidence: local command output ending in
  `TESTNET_PLACEHOLDER_GATE=PASS expected_compile_failure=true`.
- The complete local D6 sequence passed. Source policy inspected 12 Rust files and
  Geiger reported zero unsafe items plus `forbids_unsafe=true` for all ten packages.
  Evidence: local command output ending in `LOCAL_D6=PASS`.
- The same complete sequence and negative testnet-tag gate passed after adversary
  hardening.
  Evidence: local output ending in `POST_ADVERSARY_LOCAL_D6=PASS` and
  `POST_ADVERSARY_TESTNET_GATE=PASS`.
- Warning-denied workspace rustdoc passed, and the mechanical diff sweep found no
  change to the LEDGER, governing docs, formal-spec area, `cj3-types`, another crate,
  or `Cargo.lock`.
  Evidence: local command output ending in `ADVERSARY_MECHANICAL_SWEEP=PASS` and Git
  status/diff inspection.
- The intentional feature commit passed all eleven origin D6 jobs with all three
  audit-tool install steps skipped on verified cache hits.
  Evidence: <https://github.com/Quigles1337/COINjecture3.0/actions/runs/31923287404>
  on exact head `8b1c8d601629a499b47ae4ab9e904dd0949fcd65`.

## ASSUMED

- Repeated squaring in a sufficiently sized imaginary-quadratic class group and the
  Wesolowski proof meet their published sequentiality and adaptive-root-style
  assumptions for parameters later ratified at Gate G0. P-002 has not independently
  proven those cryptographic assumptions.
- Future testnet build profiles will set the registered `cj3_testnet` configuration
  tag as part of the D12 build contract. The compiler enforces exclusion after the
  tag is set; it cannot identify an intentionally or accidentally untagged artifact.
- The Poisson arrival expression in the withholding analysis is an analytical model,
  not a ratified block-arrival rule or a measured CJ3 parameter.

## UNKNOWN

- Which safe-Rust production evaluator/verifier implementation will be admitted and
  independently validated. No complete surveyed crate currently clears the combined
  A9, maturity, audit, evaluator, and packaging bar.
- The production discriminant/security size, Fiat–Shamir challenge size, iteration
  count, reference hardware, and relationship to unresolved `TARGET_BLOCK_TIME`.
- Whether the class-group discriminant is derived per parent or a fixed group is used
  with a hash-to-group input; both require canonical derivation and parameter review.
- The canonical carrier for 32-byte beacon randomness plus its Wesolowski witness,
  and the final `D_BEACON` bytes. These are P-007/Gate-G0 work, not P-002 values.
- The numeric profitability and chain-wide bias of last-block withholding; it depends
  on later mining-share, propagation, fork-choice, reward, timing, and hardware data.
