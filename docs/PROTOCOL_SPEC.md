# COINjecture 3.0 — Protocol Specification

**Version:** v0.2-DRAFT · becomes normative at Gate G0 ratification
**Authority:** subordinate to `docs/ENGINEERING_PLAN.md` axioms A1–A11 and decision log
**Convention:** `TBD(P-xxx)` marks values discovered by the named packet — agents MUST
NOT invent these (A11). "MUST/MUST NOT/SHOULD" per RFC 2119.

**Gate status:** G0 is HOLD. G0-A is ratified and incorporated below. G0-B, G0-C, G0-D
and G0-E were ruled on 2026-11-22 and are incorporated below. G0-F is held: Al rules it
after P-007 is merged with exact-merge-SHA D6 green. This document remains draft.

**Ruling citations:** `G0-` plus a label (for example `G0-C3d`) names a clause of the
2026-11-22 ruling, recorded verbatim in `loop/LEDGER.md`. A bare B1–B12 is a block rule
of §6.2. "Packet P-011" is the SIS solver-envelope packet that G0-B4 commissions; it is
not parameter P-11.

---

## §1 Notation and cryptographic primitives

- `H(·)` — SHA-256. Confirmed (G0-C2a).
- `XOF(·)` — SHAKE-256, used only for expanding instance seeds into matrices.
- Signatures — Ed25519 (plan D15), verified exactly as §1.3 states (G0-C7). The signed
  message is the 32-byte signing digest of §1.2.
- **Domain tags** (byte-string constants in `cj3-types`, each used exactly once):
  `D_ADDR`, `D_TX`, `D_HDR`, `D_INST`, `D_SOL`, `D_BEACON`, `D_GENESIS`. Their bytes
  and preimages are in §1.2.
- **Canonical encoding:** single deterministic codec for all consensus structures:
  CJ3 Encoding v1 (§1.1) — fixed field order, fixed-width little-endian integers, no
  floats anywhere. An input that is not exactly the encoding of a value is REJECTED,
  not ignored. One implementation in `cj3-types`; all crates import it.
- **Integers only.** All consensus arithmetic is unsigned integer with `checked_*`
  operations; comparisons of magnitudes use squared norms / scaled integers so that no
  floating point exists on any consensus path (A5). Overflow in any check ⇒ the object
  is `Invalid` (never wrap, never saturate).

### §1.1 CJ3 Encoding v1 — grammar (G0-C1)

- **(a)** Integers are fixed width and little-endian: `u16` in 2 bytes, `u32` in 4, `u64` in 8.
   Zero is the all-zero bytes of the field's width, and no value has a second encoding.
   This replaces v0.1's "minimal-length integers".
- **(b)** A fixed-length byte array `[N]` is its N bytes, with no length in front.
- **(c)** A structure is its fields concatenated in the order its layout gives (§5.2, §6.1,
   §6.3, §7). There are no tags, names, padding, optional fields, version bytes or
   checksums. A change of layout is a protocol upgrade with new domain-tag bytes.
   Nothing in band announces a version.
- **(d)** Two parts vary in length, both in the block: the solution bytes and the transaction
   list. Each has a `u32` count in front. A decoder checks a count against its limit and
   against the bytes that remain before it allocates.
- **(e)** An input is valid only if it is exactly the encoding of a value. It is rejected, with
   a distinct typed error per case and never a panic, when it is shorter than required,
   when bytes remain after the last field, when a count or the whole exceeds its limit,
   when the solution length is not the exact length §6.3 gives, or when `class_id` names
   no registered class. Non-minimal, duplicated, missing and unknown fields cannot be
   written in this grammar: every attempt at one is one of those rejections.
- **(f)** There is no envelope. Framing messages on the wire belongs to Phase 3.

### §1.2 Hash, domain tags and preimages (G0-C2)

`H` is SHA-256. A domain tag is 16 bytes: the ASCII text below, padded on the right with
`0x00`.

| Tag | ASCII text | The 16 bytes (hex) |
|---|---|---|
| `D_ADDR` | `CJ3/v1/addr` | `434a332f76312f616464720000000000` |
| `D_TX` | `CJ3/v1/tx` | `434a332f76312f747800000000000000` |
| `D_HDR` | `CJ3/v1/header` | `434a332f76312f686561646572000000` |
| `D_INST` | `CJ3/v1/instance` | `434a332f76312f696e7374616e636500` |
| `D_SOL` | `CJ3/v1/solution` | `434a332f76312f736f6c7574696f6e00` |
| `D_BEACON` | `CJ3/v1/beacon` | `434a332f76312f626561636f6e000000` |
| `D_GENESIS` | `CJ3/v1/genesis` | `434a332f76312f67656e657369730000` |

A preimage is its tag, then fixed-width fields in this order, each encoded by §1.1.

| Value | Definition | Preimage length |
|---|---|---|
| `addr(pk)` | `H(D_ADDR ‖ pk[32])` | 48 bytes |
| signing digest | `H(D_TX ‖ network_id u32 ‖ from[32] ‖ to[32] ‖ amount u64 ‖ fee u64 ‖ nonce u64)` | 108 bytes |
| header hash | `H(D_HDR ‖ Header)` | 322 bytes |
| `instance_seed` | `H(D_INST ‖ network_id u32 ‖ height u64 ‖ parent_hash[32] ‖ beacon_out[32])` | 92 bytes |
| `sol_commit` | `H(D_SOL ‖ class_id u16 ‖ instance_seed[32] ‖ SHA-256(solution bytes)[32] ‖ miner_addr[32])` | 114 bytes |
| beacon input | `H(D_BEACON ‖ parent_hash[32])` | 48 bytes |

- `addr(pk)` is the whole digest.
- In the signing digest, `network_id` is the transaction's own field, and the Ed25519
  message is the 32-byte digest.
- `parent_hash` is the header hash of the parent.
- In `sol_commit` the inner hash is over the §5.2 solution bytes alone, without the
  block's count in front; it is the only use of `H` without a tag in these layouts.
- `D_GENESIS` has its bytes now; the genesis layout and preimage are ruled with the
  Phase 1 genesis packet.
- Under one tag every preimage has the same length and the same field positions, and
  preimages under different tags differ in their first 16 bytes. So two different inputs
  never give the same preimage (G0-C2d).

### §1.3 Ed25519 verification (G0-C7; ratifies GAP-12)

Ed25519 as RFC 8032 section 5.1, pure (no context, no prehash), with exactly this
verification. Inputs: public key `A`, 32 bytes; signature `R ‖ S`, 32 and 32 bytes;
message `M`, the 32-byte signing digest of §1.2.

- **(a)** Decode `A` and `R` as points by RFC 8032 section 5.1.3 exactly. A y-coordinate not
   below `p = 2^255 - 19` fails. A y with no matching x fails. An x of zero with the sign
   bit set fails.
- **(b)** Reject if `A` or `R` is one of the eight points whose order divides 8, the identity
   included.
- **(c)** Read `S` as a little-endian integer. Reject unless `S < L`, the order of the base
   point.
- **(d)** `k = SHA-512(R ‖ A ‖ M)` over the bytes as received, read little-endian and reduced
   modulo `L`.
- **(e)** Accept if and only if `[S]B = R + [k]A`. The form multiplied through by 8 is not a
   substitute and must not be used. Batch verification is not used on a consensus path.
- **(f)** An all-zero signature and an all-zero key are rejected by (b).

## §2 Identifiers and addresses

```
addr(pk) = H(D_ADDR ‖ pk[32])        // the whole digest; 48-byte preimage (§1.2)
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
| P-1 | `TARGET_BLOCK_TIME` — hash-target cadence only | **UNFILLED; no builder proposal** — P-006 normalized to `1.0` and cannot derive seconds; ruled from the packet P-011 report | G0-A follow-up ruling; G0-B4e |
| P-2 | Hash-target EMA controller | **PROVISIONAL:** `W = 32`, gain `1/8`, multiplicative clamp `[8/9, 9/8]`; re-derive when P-1 is ratified | G0-A follow-up; P-006 hash-only envelope; G0-B4e |
| P-3 | `VALIDATION_BUDGET` — admission ceiling on the whole per-block cost of validating one solution (ms, reference hardware) | **PROVISIONAL: 15 ms**, scope as the P-3 note below gives; re-ruled from the packet P-011 measurements | G0-B2; gates Tier-1 trigger (plan D4) |
| P-4 | SIS `(n, m, q, β²)` | **UNFILLED** — the proposed `(128, 3840, 16411, 3840)` is REJECTED; no tuple is ratified | G0-B3; ruled from the packet P-011 report (G0-B4e) |
| P-5 | `θ` threshold quality (see §5.3) | `SCALE` (i.e. ‖s‖² ≤ β²) | definitional |
| P-6 | `SCALE` quality fixed-point | 1_000_000 | definitional |
| P-7 | `R_MAX` quality span divisor (`R_MAX ≥ 1`) | TBD — owner Al (+ Sarah), not agent-inventable | economics; Model 4 semantics ratified, value/curve unfilled |
| P-8 | `TX_MAX_BYTES`, `BLOCK_MAX_BYTES`, `SOL_MAX_BYTES` | `TX_MAX_BYTES = 188`; `SOL_MAX_BYTES = 2 * m` for the active tuple (derived; unfilled while P-4 is); `BLOCK_MAX_BYTES = 1,048,576` **PROVISIONAL** | G0-C6; ingress bounds |
| P-9 | `FEE_MIN` | TBD — owner Al | economics |
| P-10 | `TIMESTAMP_DRIFT_Δ` | **UNFILLED** — ruled together with P-1 and the unit of the timestamp field | G0-D1; §6 B11 |
| P-11 | `SIZE_RETARGET_WINDOW` | STRUCK — MOOT; no dynamic size retarget | G0-A ratified |
| P-12 | Subsidy schedule | placeholder constant; owner Al + Ken | explicitly out of program scope |

`size_param` is the symbolic active protocol constant `ACTIVE_SIZE_PARAM`. G0-A
ratifies its static status, not its numeric value: G0-B3d keeps the value UNFILLED,
and Al rules it with P-4 from the packet P-011 report (G0-B4e). It changes only through
an explicit human-ratified protocol upgrade, never through block history or miner
observations.

**P-3 provisional acceptance and scope (G0-B2).** `VALIDATION_BUDGET = 15 ms` is
accepted as provisional, with its scope corrected.

- **(a)** It covers the whole per-block cost of validating one solution: `derive_instance` plus
   `check`, single-threaded, release profile.
- **(b)** It is an admission ceiling measured on reference hardware (plan D4). It is never a
   run-time check: no clock enters consensus (A3). At run time the tuple bounds the cost.
- **(c)** The evidence so far times `check` alone on a pre-built matrix
   (`crates/cj3-classes/examples/p003_check_bench.rs`,
   `bench/p003-sis/verifier-results.jsonl`). Derivation is not in it. P-003's margins of
   2.68x and 1.92x describe `check` alone and are not to be quoted for the budget.
- **(d)** Reference host v0 is the machine in `bench/p003-sis/ENVIRONMENT.md`. The measurement
   is repeated with derivation included, on that host and on one other class of machine,
   inside packet P-011. Al re-rules the budget and the hardware definition from that.

**P-4 rejection (G0-B3).** The tuple `(n, m, q, beta_squared) = (128, 3840, 16411, 3840)`
is rejected, and so is the 128-bit threshold as a way of choosing a tuple.

- **(a)** Every block must carry a valid solution (B6), so the active tuple has to be one that
   miners can solve inside the block interval. `bench/p003-sis/estimator-results.jsonl`
   puts the cost of finding a solution at this tuple at 2^143.956 (rough model) to
   2^165.520 (default model) operations, at reduction block size 493.
   `bench/p003-sis/solver-results.jsonl` shows the bundled demonstrator solving n = 16
   and returning no solution at n = 24 in its one recorded run. A chain with this tuple
   would never produce a block. P-003's report says the tuple is "not a final
   cadence-compatible P-4".
- **(b)** A 128-bit floor is the right test for a scheme that needs nobody to solve the
   problem. A mined puzzle needs miners to solve it. Under D2 as G0-A amended it, chain
   weight comes from the hash race alone: the tuple sets how much useful work a block
   carries. It also sets the head start a faster solver gets in the hash race, which is
   why it has to be chosen from measured solve times.
- **(c)** What stands from P-003: the checker, the derivation, the measurement harness, and the
   family `q(n)` = first prime above `n^2`, `m(n) = 2 n ceil(log2 q(n))`,
   `beta_squared(n) = m(n)` as the candidate family. The family is not ratified.
- **(d)** P-4 and `ACTIVE_SIZE_PARAM` stay UNFILLED.

**The cadence link (G0-B4).** Packet P-011, the SIS solver envelope, measures and
reports. It proposes no value for P-1, P-4, P-10 or `ACTIVE_SIZE_PARAM`. Al rules P-1,
P-4, P-10, `ACTIVE_SIZE_PARAM` and the re-derived P-2 together, from that report. The
packet blocks Phase 2. It does not block P-007 or P-101. Its definition is in
`loop/PACKETS.md`.

**P-8 size limits (G0-C6).**

- **(a)** `TX_MAX_BYTES = 188`: the exact size of a transaction, so V1's bound and exact decode
   agree.
- **(b)** `SOL_MAX_BYTES = 2 * m` for the active tuple. It is derived, and unfilled while P-4
   is.
- **(c)** `BLOCK_MAX_BYTES = 1,048,576`, PROVISIONAL. No evidence stands behind this number. It
   is a conservative round figure so that decoders have a bound, and Al re-rules it with
   P-1.

**P-10 (G0-D1).** `TIMESTAMP_DRIFT_Δ` stays UNFILLED. No value can be evidenced while
P-1 is unfilled. Al rules P-10, and the unit of the timestamp field, together with P-1.

**P-7, P-9 and P-12 (G0-D3)** are unchanged: owner values, UNFILLED.

**P-2 provisional ratification / P-1 evidence boundary:** P-006 supports the broad
hash-only region `W ∈ {16, 32, 64}` blocks and multiplicative upper cap
`c ∈ {9/8, 5/4}` under fixed gain `1/8`; all six unique coordinates passed. Al has
provisionally ratified `W = 32`, gain `1/8`, and clamp `[8/9, 9/8]` as a conservative
interior point, not a tuned or statistically preferred optimum. P-2 MUST be re-derived
when P-1 is ratified. P-1 remains unfilled and proposal-free because P-006 contains
only normalized time and cannot justify an absolute cadence.

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
question** (chiavdf is C++, forbidden on the trusted path by A9). The beacon input is
the §1.2 preimage `H(D_BEACON ‖ parent_hash[32])`, 48 bytes (G0-C2c).

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

P-3 bounds `derive_instance` plus `check` together, as an admission ceiling measured on
reference hardware; it is never a run-time check (§3, G0-B2).

**Registry.** `cj3-classes` holds admitted classes keyed by `CLASS_ID`. Genesis
registry = { SIS }. Admission of any further class requires: a P-004 bench report
(published hardness assumption, parameter floor clearing known attacks, measured
solve/verify asymmetry, `derive_instance` plus `check` cost ≤ P-3) + an ADR ratified by
Al. *(The registry is COINjecture's identity — a growing, audited catalog of useful
problems. The anchor is merely the class the chain's security leans on. Plan D1.)*

**Class identifiers (G0-C3b).** SIS is 1. 0 is never assigned. This fills the
identifier P-003 left unbound.

### §5.2 Genesis class: SIS (Short Integer Solution)

- **Instance.** `A ∈ Z_q^{n×m}` expanded column-wise from `XOF(seed)` with rejection
  sampling to uniform mod q. `(n, m, q, β²)` = P-4.
- **Expansion convention (G0-C5).** The convention of `crates/cj3-classes/src/sis.rs` is
  ratified as it stands. SHAKE-256 absorbs the 32 seed bytes and nothing else. Its
  output is read four bytes at a time, each group a little-endian u32. A group is
  accepted when it is below `floor(2^32 / q) * q`, and it yields its value modulo q. A
  rejected group is used up whole: the next candidate starts at the next four-byte
  boundary. Accepted values fill the matrix column by column, rows 0 to n-1 within a
  column, until n * m are placed. The existing q = 17 fixture in
  `bench/p003-sis/SHAKE-VECTOR.md` is now normative (G0-C8b).
- **Size parameter layout (G0-C3c).** 20 bytes: `n u32`, `m u32`, `q u32`,
  `beta_squared u64`.
- **Solution.** `s ∈ Z^m`, `s ≠ 0`. Its encoding is `2 * m` bytes: the m coefficients in
  index order, each a 16-bit two's-complement integer, little-endian (G0-C3e).
- **check(A, s):**
  1. decode strictly; `len(s) == m`; reject a coefficient when its square exceeds `β²`,
     before it adds to the norm; `s ≠ 0`;
  2. `A·s ≡ 0 (mod q)` — integer matrix-vector product, checked arithmetic;
  3. `‖s‖² ≤ β²` — squared norm, integer-only (no square roots on consensus paths);
  4. `Quality Q = (β² · SCALE) / ‖s‖²` (integer division, u128 intermediate).
     `Q ≥ SCALE` ⟺ valid (θ, P-5); larger Q = shorter vector = better solution.
- **Coefficient guard (G0-C4).** The guard in step 1 replaces v0.1's
  `|sᵢ| ≤ s_max(P-4)`. It follows from the norm bound and changes no set of valid
  solutions. A tuple with `beta_squared` above `32767^2 = 1,073,676,289` is not
  admissible under encoding v1.
- **Hardness basis (an assumption, labeled as A10 requires).** For uniformly sampled A,
  SIS has a conditional worst-case-to-average-case reduction lineage (Ajtai 1996;
  Micciancio-Regev 2007; surveyed in Peikert 2016). In parameter regimes that meet the
  reductions' asymptotic conditions, an efficient solver for random instances would give
  an efficient approximation algorithm for certain worst-case lattice problems. The
  reductions do not prove those worst-case problems hard, do not by themselves cover any
  concrete finite tuple, and say nothing about how long a solver takes on an instance. A
  concrete tuple therefore rests on three separately labeled things: the assumption that
  the worst-case problems are hard, estimator models at a pinned estimator commit, and
  measured solver and checker cost. None of the three is a proof. Parameters are chosen
  from a measured solver envelope (ruling G0-B4), with estimator output cited as a
  consistency check.
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

**Layout (G0-C3d).** The header is these fields concatenated in this order: 306 bytes
with the SIS anchor. `size_param` is in the layout of the class that `class_id` names;
for SIS that is the 20 bytes of §5.2. `tx_root` and `state_root` are carried as 32
opaque bytes: how `tx_root` and the state commitment are built is ruled at the P-101
FRAME (G0-F3). The unit of `timestamp` is ruled together with P-1 and P-10 (G0-D1).

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

**Exact preimages, and one carried question (G0-C2c, G0-F3).** The hashes in B5, B7 and
B8 are the `instance_seed`, `sol_commit` and header-hash preimages of §1.2. The byte
order in which a 32-byte value is read as a number — for B8, and for the weight formula
and tie-break of §10 — is ruled at the first Phase 2 FRAME. §1.1(a) does not decide it.

### §6.3 Block layout and solution commitment (G0-C3f, G0-C2c)

A block is: the header, a `u32` solution length, the solution bytes, a `u32` transaction
count, the transactions. The solution length must equal `2 * m` for the header's
`size_param`. The whole must not exceed `BLOCK_MAX_BYTES`.

`sol_commit` is the §1.2 preimage. Its inner hash is over the solution bytes alone,
without the block's count in front.

## §7 Transactions

```
Tx { network_id u32, from [32], to [32], amount u64, fee u64,
     nonce u64, pubkey [32], sig [64] }
```

**Layout (G0-C3a).** A transaction is 188 bytes: `network_id u32`, `from [32]`,
`to [32]`, `amount u64`, `fee u64`, `nonce u64`, `pubkey [32]`, `sig [64]`. `pubkey` is
the RFC 8032 public-key encoding, and `sig` is `R` then `S`.

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

**Encoding v1 values for V1 and V2.** `TX_MAX_BYTES = 188`, the exact size of a
transaction, so V1's bound and exact decode agree (G0-C6a). V2's message is the 32-byte
signing digest of §1.2, and V2 verifies exactly as §1.3 states (G0-C2c, G0-C7). The
V-rule text above is unchanged.

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
  `H(header)` wins. **Quality Q appears nowhere in this section (A4).** The byte order
  in which a 32-byte value is read as a number, for this formula and this tie-break, is
  ruled at the first Phase 2 FRAME (G0-F3).
- **Retarget — hash target only:**
  - `hash_target`: clamped EMA on inter-block times over `W` (P-2), targeting P-1.
    P-2 is provisionally `W=32`, gain `1/8`, clamp `[8/9,9/8]`; P-1 remains unfilled,
    so no complete retarget implementation is admitted. P-2 MUST be re-derived when
    P-1 is ratified.
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
- **Ratified Phase 3 requirements (G0-E2).** GAP-7, GAP-8, GAP-10, GAP-11 and GAP-13 are
  ratified as worded in `docs/AUDIT_TRACEABILITY.md` §5. Gate G3 tests each.
  - **GAP-7 — authenticated gossip and freshness.** Require one authenticated peer
    transport/envelope model; never forward an unverified sender; define replay and
    freshness checks; forbid a configuration that silently disables authentication.
    Covers H1, H2, M8.
  - **GAP-8 — peer-set resource and eclipse discipline.** Bound peer-exchange entries,
    total peers, concurrent dials, and address-class/subnet concentration. Covers H3,
    M9.
  - **GAP-10 — authoritative client address.** Use the socket peer by default; honor
    forwarding headers only through an explicitly configured trusted-proxy boundary.
    Covers H10.
  - **GAP-11 — public error hygiene.** Map internal errors to bounded typed public
    responses without filesystem, query, stack, or internal diagnostic disclosure.
    Covers M10's second half.
  - **GAP-13 — bounded limiter/TLS state.** Bound rate-limiter key cardinality and
    eviction; set handshake/idle/read timeouts and total/per-peer connection ceilings.
    Covers L5.
- **Mempool requirement (G0-E3).** GAP-9 — per-principal mempool occupancy — is ratified
  as a requirement on the packet that introduces the mempool. Gate G3 tests it. Its
  wording in `docs/AUDIT_TRACEABILITY.md` §5 is: "Define per-sender limits and an
  adversarially safe eviction rule in addition to global/per-peer bounds. Covers H5."
- **Operating configuration (G0-E4).** The numbers these requirements need (peer and
  dial limits, timeouts, limiter sizes, per-sender mempool limits) are operating
  configuration, not consensus parameters. The builder proposes each default with its
  reasoning in the packet report, and Al reviews them at G3.

## §13 Genesis

- `genesis.json`: network_id, parameters snapshot, allocations `[{addr, amount}]`.
- `genesis_hash = H(D_GENESIS ‖ canonical(genesis.json))` is embedded in the height-0
  header; every node validates its local file against it. `D_GENESIS` has its bytes now
  (§1.2); the genesis layout and its preimage are ruled with the Phase 1 genesis packet
  (G0-C2c, G0-F3).
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

Status after the 2026-11-22 ruling. Still open, with the boundary where each is ruled:

- P-1 hash-target cadence → UNFILLED, no builder proposal. Al rules P-1, P-4, P-10,
  `ACTIVE_SIZE_PARAM` and the re-derived P-2 together, from the packet P-011 report
  (G0-B4e); the re-ruled `BLOCK_MAX_BYTES` and the timestamp unit come from the same
  report (G0-F3). They block Phase 2.
- P-2 hash-target controller → provisionally ratified (`W=32`, gain `1/8`, clamp
  `[8/9,9/8]`), re-derive when P-1 is ratified.
- P-3 validation budget → PROVISIONAL 15 ms over `derive_instance` plus `check`; Al
  re-rules the budget and the hardware definition from the packet P-011 measurements
  (G0-B2d).
- P-4 SIS tuple and the `ACTIVE_SIZE_PARAM` value → UNFILLED; the proposed tuple is
  rejected and the candidate family is not ratified (G0-B3). A change of the value
  after it is ruled needs an explicit human-ratified upgrade (G0-A).
- P-7 quality-span value/curve and P-9/P-12 economics → their owners (Al; + Sarah for
  P-7; Ken for P-12), UNFILLED (G0-D3).
- P-8 → `SOL_MAX_BYTES = 2 * m` is unfilled while P-4 is; `BLOCK_MAX_BYTES = 1,048,576`
  is provisional and re-ruled with P-1 (G0-C6).
- P-10 timestamp drift, and the unit of the timestamp field → UNFILLED, ruled with P-1
  (G0-D1).
- How `tx_root` and the state commitment are built → the P-101 FRAME (G0-F3).
- The byte order in which a 32-byte value is read as a number (B8; §10 weight and
  tie-break) → the first Phase 2 FRAME (G0-F3).
- The genesis layout and its preimage → the Phase 1 genesis packet (G0-C2c, G0-F3).
- Operating defaults for the Phase 3 and mempool requirements → proposed by the builder
  in the packet report, reviewed by Al at G3 (G0-E4).
- Beacon technology → P-002.
- Reward curve shaping incl. μ-balance hook (§5.2) → Al + Sarah.
- Gate disposition (G0-F) → held; Al rules it after P-007 is merged with
  exact-merge-SHA D6 green (G0-F1).

Closed: P-11 → struck as moot by G0-A · hash primitive → SHA-256 confirmed (G0-C2a) ·
canonical encoding, domain-tag bytes and preimages → §1.1, §1.2 (G0-C1, G0-C2) ·
Ed25519 verification → §1.3 (G0-C7) · class identifier → SIS is 1 (G0-C3b) ·
`TX_MAX_BYTES = 188` (G0-C6a) · SIS coefficient encoding and guard, and the SHAKE
expansion convention → §5.2 (G0-C3e, G0-C4, G0-C5).

---
*CJ3 Protocol Specification v0.2-DRAFT — DARQ Labs LLC.*
