# COINjecture 3.0 — Protocol Specification

**Version:** v0.1-DRAFT · becomes normative at Gate G0 ratification
**Authority:** subordinate to `docs/ENGINEERING_PLAN.md` axioms A1–A11 and decision log
**Convention:** `TBD(P-xxx)` marks values discovered by the named packet — agents MUST
NOT invent these (A11). "MUST/MUST NOT/SHOULD" per RFC 2119.

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
| P-1 | `TARGET_BLOCK_TIME` | TBD(P-006) | difficulty sim |
| P-2 | `RETARGET_WINDOW_W` | TBD(P-006) | difficulty sim |
| P-3 | `VALIDATION_BUDGET` — max checker cost per block (ms, reference hardware) | TBD(P-003) | asymmetry bench; gates Tier-1 trigger (plan D4) |
| P-4 | SIS `(n, m, q, β²)` | TBD(P-003) | parameter search |
| P-5 | `θ` threshold quality (see §5.3) | `SCALE` (i.e. ‖s‖² ≤ β²) | definitional |
| P-6 | `SCALE` quality fixed-point | 1_000_000 | definitional |
| P-7 | `R_MAX` reward multiplier cap | TBD — owner Al (+ Sarah), not agent-inventable | economics |
| P-8 | `TX_MAX_BYTES`, `BLOCK_MAX_BYTES`, `SOL_MAX_BYTES` | TBD(P-001 defaults, ratify G0) | ingress bounds |
| P-9 | `FEE_MIN` | TBD — owner Al | economics |
| P-10 | `TIMESTAMP_DRIFT_Δ` | TBD(P-001 default, ratify G0) | §6 B11 |
| P-11 | `SIZE_RETARGET_WINDOW` | TBD(P-006) | difficulty sim |
| P-12 | Subsidy schedule | placeholder constant; owner Al + Ken | explicitly out of program scope |

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
- **Reward-margin caution (risk R7):** the Q-multiplier curve interacts with
  diminishing returns of reduction algorithms. Curve shaping — including any
  μ-balance normalization — is owned by Al + Sarah and is NOT agent-inventable.

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
  size_param      SizeParam   // derived by retarget, verified B3
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
- **B3** `hash_target` and `size_param` EQUAL the retarget function outputs computed
  from ancestor headers (§10). *Difficulty is derived and checked, never read.*
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
  `Σ balances(post) = Σ balances(pre) + subsidy(height)`. Fees transfer, never mint.

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
- **Retarget — two knobs, two timescales:**
  - `hash_target`: clamped EMA on inter-block times over `W` (P-2), targeting P-1.
    Standard Nakamoto-style; exact clamps TBD(P-006).
  - `size_param`: slower window (P-11) adjusting instance size toward a target solve
    envelope inferred from on-chain quality margins. Exact function TBD(P-006) —
    **Phase 2 MUST NOT freeze this before the P-006 stability report is ratified**
    (risk R2; promotion criteria in plan D2 if unstabilizable).

## §11 Rewards

`reward(height, Q) = subsidy(height) · min(Q, R_MAX·SCALE) / SCALE`, computed in u128,
checked back to u64, credited in §8 step 3, and **validated by every node (B12)** —
the reward is derived from the checker's Q, never read from the block. `subsidy(·)` is
the P-12 placeholder pending economics ownership. The multiplier is bounded (risk R7)
so quality variance cannot recreate the fork-choice variance A4 removed.

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

P-1/P-2/P-11 retarget constants → P-006 · P-3 validation budget + P-4 SIS parameters →
P-003 · P-7/P-9/P-12 economics → Al (+ Ken for P-12) · P-8/P-10 ingress/drift defaults
→ P-001 proposal, G0 ratification · Beacon technology → P-002 · Hash/signature
primitives (§1) → G0 confirmation (D15) · Reward curve shaping incl. μ-balance hook
(§5.2) → Al + Sarah.

---
*CJ3 Protocol Specification v0.1-DRAFT — DARQ Labs LLC.*
