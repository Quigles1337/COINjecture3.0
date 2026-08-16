# COINjecture 3.0 — Protocol Specification

**Version:** v0.1-DRAFT · becomes normative at Gate G0 ratification
**Authority:** subordinate to `docs/ENGINEERING_PLAN.md` axioms A1–A11 and decision log
**Convention:** `TBD(P-xxx)` marks values discovered by the named packet — agents MUST
NOT invent these (A11). "MUST/MUST NOT/SHOULD" per RFC 2119.

**Gate status:** G0 is HOLD. G0-A is ratified and incorporated below; G0-B through
G0-E remain held. This document remains draft and no held value is implied by the
G0-A amendment.

---

## §1 Notation and cryptographic primitives

- `H(·)` — SHA-256. *(PROPOSED per plan D15 discussion: boring, ubiquitously audited,
  Lean-friendly. Confirm at G0; if changed, changes everywhere via `cj3-types` only.)*
- `XOF(·)` — SHAKE-256, used only for expanding instance seeds into matrices.
- Signatures — Ed25519 (plan D15, PROPOSED). Signing bytes are always
  `H(domain_tag ‖ network_id ‖ canonical_payload)`.
- **Domain tags** (byte-string constants in `cj3-types`, each used exactly once):
  `D_ADDR`, `D_TX`, `D_HDR`, `D_INST`, `D_SOL`, `D_BEACON`, `D_GENESIS`.
- **Canonical encoding:** single deterministic codec for all consensus structures
  (fixed field order, minimal-length integers, no floats anywhere, unknown fields
  REJECT — not ignore). One implementation in `cj3-types`; all crates import it.
- **Integers only.** All consensus arithmetic is unsigned integer with `checked_*`
  operations; comparisons of magnitudes use squared norms / scaled integers so that no
  floating point exists on any consensus path (A5). Overflow in any check ⇒ the object
  is `Invalid` (never wrap, never saturate).

## §2 Identifiers and addresses

```
addr(pk) = H(D_ADDR ‖ pk)[0..32]
```
This function exists **once**, in `cj3-types::addr`. Every component — kernel,
consensus, RPC, genesis tooling, wallets — MUST call it. No component may re-derive
addresses locally. *(This is the C3 kill. The genesis spend-test in CI (§13) exists to
make any future divergence a red build, not a stuck testnet.)*

`network_id` — u32, distinct per network (devnet/testnet). Included in all signing
bytes and in instance seeds ⇒ no cross-network replay of transactions or solutions.

## §3 Protocol parameters

| ID | Parameter | Value | Source |
|----|-----------|-------|--------|
| P-1 | `TARGET_BLOCK_TIME` — hash-target cadence only | TBD — P-006 normalized to `1.0` and cannot derive seconds | G0-A; proposal review required |
| P-2 | `RETARGET_WINDOW_W` — hash-target controller only | TBD — review candidate below is NOT RATIFIED | G0-A; P-006 hash-only envelope |
| P-3 | `VALIDATION_BUDGET` — max checker cost per block (ms, reference hardware) | TBD(P-003) | asymmetry bench; gates Tier-1 trigger (plan D4) |
| P-4 | SIS `(n, m, q, β²)` | TBD(P-003) | parameter search |
| P-5 | `θ` threshold quality (see §5.3) | `SCALE` (i.e. ‖s‖² ≤ β²) | definitional |
| P-6 | `SCALE` quality fixed-point | 1_000_000 | definitional |
| P-7 | `R_MAX` quality span divisor (`R_MAX ≥ 1`) | TBD — owner Al (+ Sarah), not agent-inventable | economics; Model 4 semantics ratified, value/curve unfilled |
| P-8 | `TX_MAX_BYTES`, `BLOCK_MAX_BYTES`, `SOL_MAX_BYTES` | TBD(P-001 defaults, ratify G0) | ingress bounds |
| P-9 | `FEE_MIN` | TBD — owner Al | economics |
| P-10 | `TIMESTAMP_DRIFT_Δ` | TBD(P-001 default, ratify G0) | §6 B11 |
| P-11 | `SIZE_RETARGET_WINDOW` | STRUCK — MOOT; no dynamic size retarget | G0-A ratified |
| P-12 | Subsidy schedule | placeholder constant; owner Al + Ken | explicitly out of program scope |

`size_param` is the symbolic active protocol constant `ACTIVE_SIZE_PARAM`. G0-A
ratifies its static status, not its numeric value: the value remains unfilled while
the held class/parameter surface is unresolved. It changes only through an explicit
human-ratified protocol upgrade, never through block history or miner observations.

**P-1/P-2 proposal for Al review — non-normative:** P-006 supports the broad hash-only
region `W ∈ {16, 32, 64}` blocks and multiplicative upper cap
`c ∈ {9/8, 5/4}` under fixed gain `1/8`; all six unique coordinates passed. The
builder review candidate is `W = 32`, gain `1/8`, clamp `[8/9, 9/8]`, chosen as the
interior window and conservative passing cap. P-006 did not distinguish it from the
other five coordinates, so P-2 remains TBD pending Al's review. No absolute P-1 is
proposed because P-006 contains only normalized time and cannot justify seconds.

## §4 Beacon

```rust
trait Beacon {
    /// Deterministic, verifiable randomness for the child of `parent`.
    fn output(parent: &Header) -> BeaconOut;            // 32 bytes
    fn verify(parent: &Header, out: &BeaconOut) -> bool; // cheap
}
```
**Security requirements (P-002 report MUST address each):** output is fixed once the
parent block is published; computing it requires sequential delay ≥ a substantial
fraction of `TARGET_BLOCK_TIME` (grinding a parent-field costs a full delay per
attempt); verification is cheap; no trusted party. Candidate: Wesolowski/Pietrzak VDF
over `H(D_BEACON ‖ parent_hash)` — **pure-Rust implementation availability is the P-002
question** (chiavdf is C++, forbidden on the trusted path by A9).

**Devnet placeholder:** iterated-hash chain of fixed length behind the same trait,
compile-time feature-gated, banner `NOT-TESTNET-GRADE: sequentiality assumption only`.
MUST NOT be compiled into any testnet-tagged build.

## §5 Problem classes

### §5.1 The trait contract (the heart of A1–A3)

```rust
trait ProblemClass {
    const CLASS_ID: u16;
    type Instance;
    type Solution;   // decoded from untrusted bytes via the canonical codec

    /// Pure. Deterministic. Everything from committed data.
    fn derive_instance(seed: [u8; 32], size: SizeParam) -> Self::Instance;

    /// Pure. Bounded by VALIDATION_BUDGET (P-3). Returns integer quality.
    /// NOTE THE SIGNATURE: no &self, no clock, no miner fields, no metadata.
    /// The 2.0 C1/C2 bug class is unrepresentable here — there is nothing
    /// to lie about because there is nothing to report.
    fn check(inst: &Self::Instance, sol: &Self::Solution) -> Result<Quality, Invalid>;
}
```
`Quality` is a `u64` in fixed-point `SCALE` units. A CI grep-gate rejects any
occurrence of `solve_time`, `work_score`, `reported_*`, or `self_reported` in `src/`.

**Registry.** `cj3-classes` holds admitted classes keyed by `CLASS_ID`. Genesis
registry = { SIS }. Admission of any further class requires: a P-004 bench report
(published hardness assumption, parameter floor clearing known attacks, measured
solve/verify asymmetry, checker cost ≤ P-3) + an ADR ratified by Al. *(The registry is
COINjecture's identity — a growing, audited catalog of useful problems. The anchor is
merely the class the chain's security leans on. Plan D1.)*

### §5.2 Genesis class: SIS (Short Integer Solution)

- **Instance.** `A ∈ Z_q^{n×m}` expanded column-wise from `XOF(seed)` with rejection
  sampling to uniform mod q. `(n, m, q, β²)` = P-4.
- **Solution.** `s ∈ Z^m`, encoded as `m` bounded signed integers, `s ≠ 0`.
- **check(A, s):**
  1. decode strictly; `len(s) == m`; each `|sᵢ| ≤ s_max(P-4)`; `s ≠ 0`;
  2. `A·s ≡ 0 (mod q)` — integer matrix-vector product, checked arithmetic;
  3. `‖s‖² ≤ β²` — squared norm, integer-only (no square roots on consensus paths);
  4. `Quality Q = (β² · SCALE) / ‖s‖²` (integer division, u128 intermediate).
     `Q ≥ SCALE` ⟺ valid (θ, P-5); larger Q = shorter vector = better solution.
- **Hardness basis:** Ajtai worst-case→average-case reduction — the sampled
  distribution's hardness is provable, not assumed (RS §2). Parameter floors from
  P-003 MUST cite current lattice-estimator results, not folklore.
- **Solver:** out-of-process, untrusted (A9). Reference solver ships in
  `cj3-solver-sis` (lattice reduction; free choice of algorithm — the protocol only
  ever sees `Solution` bytes).
- **Reward-margin caution (risk R7):** the bounded-above Q-normalization curve
  interacts with diminishing returns of reduction algorithms. `R_MAX` is the quality
  span divisor from §11, not a maximum inflation multiple. Its value and all curve
  shaping — including any μ-balance normalization — are owned by Al + Sarah and are
  NOT agent-inventable.

## §6 Blocks

### §6.1 Header (all fields consensus-validated; nothing "informational")

```
Header {
  network_id      u32
  height          u64
  parent_hash     [32]
  beacon_out      [32]        // §4, verified B4
  instance_seed   [32]        // derived, verified B5
  class_id        u16         // must equal active anchor class
  size_param      SizeParam   // equals ACTIVE_SIZE_PARAM, verified B3
  hash_target     [32]        // derived by retarget, verified B3
  tx_root         [32]        // Merkle root of canonical tx encodings
  state_root      [32]        // post-state commitment, verified B10
  sol_commit      [32]        // §6.3, verified B7
  timestamp       u64         // bounded B11; NEVER an input to any derivation
  miner_addr      [32]
  nonce           u64         // hash-race nonce
}
```

### §6.2 Block validity rules (B-rules; all MUST hold; typed errors, never panics)

- **B1** Header and body decode canonically; sizes within P-8.
- **B2** `parent_hash` known; `height = parent.height + 1`; `network_id` matches.
- **B3** `hash_target` EQUALS the hash-target retarget output computed from ancestor
  headers (§10), and `size_param == ACTIVE_SIZE_PARAM`. The header fields are checked,
  never trusted. `ACTIVE_SIZE_PARAM` changes only by explicit human-ratified protocol
  upgrade; no dynamic size retarget exists.
- **B4** `Beacon::verify(parent, beacon_out)`.
- **B5** `instance_seed == H(D_INST ‖ network_id ‖ height ‖ parent_hash ‖ beacon_out)`.
- **B6** Body contains exactly one `Solution`; `class_id` = active anchor;
  `check(derive_instance(instance_seed, size_param), solution) = Ok(Q)` with `Q ≥ θ`.
- **B7** `sol_commit == H(D_SOL ‖ class_id ‖ instance_seed ‖ H(solution_bytes) ‖ miner_addr)`.
- **B8** Eligibility: `H(D_HDR ‖ canonical(Header)) ≤ hash_target`.
- **B9** `tx_root` matches body; every transaction satisfies V1–V9 (§7) in order
  against the evolving state.
- **B10** `state_root` equals the root recomputed by applying §8 locally.
- **B11** `median_time(last 11 ancestors) < timestamp ≤ local_now + Δ(P-10)`.
- **B12** Coinbase/reward entry matches §11 exactly (derived, not read).

## §7 Transactions

```
Tx { network_id u32, from [32], to [32], amount u64, fee u64,
     nonce u64, pubkey [32], sig [64] }
```

**Validity rules (V-rules) — the Lean `Spec/Tx.lean` normative set:**

- **V1** Strict canonical decode; size ≤ `TX_MAX_BYTES`; unknown fields reject.
- **V2** `sig` verifies under `pubkey` over `H(D_TX ‖ network_id ‖ canonical(from,to,amount,fee,nonce))`.
- **V3** `from == addr(pubkey)` — **explicit binding check at the validation site.**
  *(DARQ-021 kill, carried per the 2.0 P-021 ruling: exactly this behavioural check,
  not smuggled into signature verification.)*
- **V4** `nonce == state.account(from).nonce` — strict equality. *(C4 kill.)*
- **V5** `fee ≥ FEE_MIN` (P-9).
- **V6** `state.balance(from) ≥ amount ⊕ fee` where `⊕` is `checked_add`; any
  overflow anywhere ⇒ Invalid. *(C5 kill.)*
- **V7** `to` is well-formed (32 bytes; no other semantics at genesis).
- **V8** `network_id` matches the chain (replay isolation across networks).
- **V9** `from ≠ to` is **not** required (self-sends legal, still pay fees) — stated
  so the Lean spec and kernel agree on the boundary case explicitly.

Every value-moving path in the system routes through V1–V9. There is exactly one
validity predicate; mempool admission calls the same function as block validation.
*(2.0 ruling carried: a Transfer-only fix under-fixes; here there is one predicate.)*

Any future value-moving structure — including a new transaction type or fee
mechanic, and anything else that debits or credits value — MUST extend the normative
Lean V-rules before any Rust implementation exists. There is no Rust-first exception.

## §8 State and the state-transition function (STF)

- **State:** `account(addr) -> { balance u64, nonce u64 }` in an authenticated
  structure (Merkle-ized; structure choice in P-101 within spec constraints).
- **apply_block(state, block):**
  1. Validate B1–B8, B11–B12.
  2. For each tx in order: re-evaluate V1–V9 **against current state** (mempool
     pre-validation confers nothing — C6 kill), then
     `balance(from) −= amount+fee`, `balance(to) += amount`,
     `balance(miner) += fee`, `nonce(from) += 1` — all checked ops.
  3. Apply §11 reward to `miner_addr` (checked).
  4. Recompute `state_root`; require equality with header (B10).
  5. Commit **atomically at block granularity**: any failure anywhere ⇒ the entire
     block is rejected and state is untouched. There is no partial-apply API; the
     commit path owns its own precondition checks and exposes no bypass constructor.
- **Conservation invariant (CI property test):** for every applied block,
  `Σ balances(post) = Σ balances(pre) + reward(height, Q)`, subject to the standing
  invariant `reward(height, Q) ≤ subsidy(height)`. `subsidy(height)` is a per-block
  issuance ceiling, not a target: realized issuance may be lower, and the unminted
  remainder is never minted. Fees transfer, never mint. No treasury, reserve,
  premium account, or insufficient-funds branch exists for reward issuance.
- **P-101 binding state-store laws:** the concrete authenticated store MUST instantiate
  the Lean `LawfulStateOps` contract and prove, without axioms or admitted premises:
  1. read-after-write at the written address;
  2. reads at every other address are unchanged; and
  3. replacing account `a` with `new` obeys the additive balance equation
     `totalBalances(setAccount(state,a,new)) + balance(account(state,a)) =
     totalBalances(state) + balance(new)`.
  These laws bind nonce-only updates to preserve total balances and must hold for every
  address-aliasing case among sender, recipient, and miner. P-005 proves transaction
  and successful-block conservation from this contract; P-101 must discharge the
  contract for its concrete store before claiming Lean conformance. The exported
  storage read/write operations used by the kernel MUST be coherent with those same
  laws; no alternate write path, cache, or adapter may bypass read-after-write,
  read-other-address, or the additive replacement equation.

## §9 Instance derivation — grinding and theft analysis

- **Seed inputs are all committed or delay-locked:** `height`, `parent_hash`,
  `beacon_out`. `timestamp` is deliberately excluded — it is the one cheap
  miner-adjustable header field.
- **Parent-choice grinding:** re-rolling the instance requires mining on a different
  parent (forgoing the best chain — economically self-punishing) or withholding a
  block to delay the beacon (P-002 MUST quantify; VDF delay makes per-attempt cost ≥
  the sequential delay).
- **Solution theft:** solutions are public once a block gossips. At the same height,
  a rival MAY lift a seen solution — but B7 binds `miner_addr` into `sol_commit`,
  which B8's hash race covers, so the thief re-runs the full eligibility race at the
  same expected cost as honest mining. Residual dynamic (first-solver head start,
  solve-once-then-hash variance) is a P-006 simulation deliverable, reported honestly
  per A10.
- **Cross-height reuse:** impossible — `instance_seed` changes every block (B5), and
  `check` runs against the derived instance, not a claimed one.

## §10 Fork choice and difficulty

- **Weight:** `weight(block) = ⌊2²⁵⁶ / (hash_target + 1)⌋` — deterministic from a
  header field that B3 itself validates. Chain weight = Σ weights. Ties: lower
  `H(header)` wins. **Quality Q appears nowhere in this section (A4).**
- **Retarget — hash target only:**
  - `hash_target`: clamped EMA on inter-block times over `W` (P-2), targeting P-1.
    P-1/P-2 remain unfilled pending Al's review of the non-normative proposal in §3.
  - `size_param`: exactly `ACTIVE_SIZE_PARAM`, a human-governed protocol constant.
    There is no on-chain size controller, no dynamic size-retarget function, and no
    aggregation of published quality margins for instance-size adjustment. P-11 is
    struck as moot. P-006's sealed offline simulator is retained as historical
    rejection evidence and is not protocol/runtime code.
- **Selection-bias boundary:** any future control loop proposed over published miner
  behavior MUST state and evidence resistance to strategic withholding or be rejected.
  An honestly recomputed observation is not necessarily honestly distributed.
- **Adequacy review:** every G0–G4 phase gate MUST re-review whether
  `ACTIVE_SIZE_PARAM` remains adequate. A change requires an explicit human-ratified
  protocol upgrade; gate review is not an automatic retarget.

## §11 Rewards

`reward(height, Q) = subsidy(height) · min(Q, R_MAX·SCALE) /
(R_MAX·SCALE)`, computed in u128 with floor division, checked back to u64, credited in
§8 step 3, and **validated by every node (B12)** — the reward is derived from the
checker's Q, never read from the block. `R_MAX ≥ 1`, and
`reward ∈ [floor(subsidy/R_MAX), subsidy]` for every valid solution (`Q ≥ SCALE`). A
threshold solution (`Q = SCALE`) earns `floor(subsidy/R_MAX)`; a solution at or beyond
`R_MAX·SCALE` earns the full subsidy. `R_MAX = 1` degenerates to `reward = subsidy`
for every valid block.

`Context.rewardInputs` MUST be instantiated from the checker output over the derived
instance for that same validated block:
`check(derive_instance(instance_seed, size_param), solution) = Ok(Q)`. It MUST NOT
read `Q`, quality, work score, timing, or any equivalent reward input from a block-
supplied field. This is the C2 structural boundary: miners supply a solution, while
validators derive the instance and recompute the only quality value that may enter
reward calculation. Concrete P-101 wiring is bound to this provenance rule, and Gate
G2 MUST exercise it explicitly, including a block-supplied-quality spoof attempt and a
wrong-instance solution.

`R_MAX` is a quality span divisor, not a maximum inflation multiple. Its value and the
curve shaping, including any μ-balance normalization, remain P-7 owner-controlled and
UNFILLED — owner: Al (+ Sarah, reserved per D16). `subsidy(·)` remains the P-12
placeholder pending economics ownership. The normalization is bounded above by the
subsidy, so no reward funding source or premium account exists and the unminted
remainder is never carried forward.

P-101 MUST construct the concrete `R_MAX·SCALE` divisor with checked u128
multiplication, prove it nonzero from the ratified domains before division, and check
the final reward back to u64. A wrapped, saturated, unchecked, block-supplied, or
otherwise pretrusted divisor is non-conforming.

## §12 Networking and RPC — baseline requirements (normative for Phase 3)

- All wire decodes are strict-canonical with typed errors; fuzz targets on every
  codec are CI jobs from the moment the codec exists. Panic on remote input = Critical.
- Ingress bounds: per-peer message rate and size caps (P-8); oversize/overrate ⇒
  scored disconnect, never unbounded buffering (H3/H4/H5/H11 class).
- RPC: every state-mutating endpoint requires the auth token; the node REFUSES TO
  START if the secret is unset or empty (fail-closed; A7). Read endpoints are
  rate-limited. The G3 auth matrix test enumerates every mutating endpoint ×
  {no token, bad token, empty-secret-config}.
- Object-level authorization: every non-public endpoint is enumerated with its
  protected object and allowed principal/action combinations, and the G3 matrix tests
  each allowed and denied case. Authentication alone never authorizes arbitrary
  object access.
- Log-emission discipline: no unbounded log line or unbounded log rate is reachable
  from remote input. Remote-controlled fields and repeated failures are bounded at
  the emission site so an attacker cannot turn accepted or rejected input into a
  log-flood resource sink.

## §13 Genesis

- `genesis.json`: network_id, parameters snapshot, allocations `[{addr, amount}]`.
- `genesis_hash = H(D_GENESIS ‖ canonical(genesis.json))` is embedded in the height-0
  header; every node validates its local file against it.
- **Genesis spend-test (CI, every build):** construct a Tx from a genesis allocation
  keypair (test fixtures), run it through V1–V9 and the STF, assert success. *(2.0's
  C3 — genesis funds unspendable because two components derived addresses
  differently — becomes a red build here, permanently.)*

## §14 Lean 4 specification mapping (A8 / plan D5)

- `spec/Spec/Tx.lean` — normative encoding of V1–V9 (this document defers to the Lean
  once ratified; divergences are spec bugs, filed as such).
- `spec/Spec/Stf.lean` — §8 apply semantics + conservation invariant as a theorem
  target.
- `lake exe vectors` — exports JSON `{name, input_bytes, expected}` covering: each
  V-rule pass/fail boundary, nonce equality edges, overflow edges (u64::MAX
  neighborhoods), V3 binding mismatches, V9 self-send, block-atomicity cases.
- `cj3-kernel` CI conformance test consumes the vectors; red = merge blocked.
- Fork choice (§10) and beacon (§4) get Lean treatment in Phase 2+ at Sarah's/Al's
  discretion — deliberately later, per RS §6 (deterministic state machine first).

## §15 Open-TBD index

P-1/P-2 hash-target constants → P-006 proposal, Al review · P-11 → struck as moot by
G0-A · `ACTIVE_SIZE_PARAM` value → held class/parameter ruling plus explicit human
upgrade · P-3 validation budget + P-4 SIS parameters → P-003 · P-7 quality-span
value/curve and P-9/P-12 economics → Al (+ Sarah for P-7;
Ken for P-12) · P-8/P-10 ingress/drift defaults
→ P-001 proposal, G0 ratification · Beacon technology → P-002 · Hash/signature
primitives (§1) → G0 confirmation (D15) · Reward curve shaping incl. μ-balance hook
(§5.2) → Al + Sarah.

---
*CJ3 Protocol Specification v0.1-DRAFT — DARQ Labs LLC.*
