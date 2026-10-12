# C7 / P-007 Builder Report — `cj3-types`

## FRAME — recorded before build edits

### Packet and done-condition

P-007 is the Phase 0 shared-types packet. Its stated done-condition is a
`cj3-types` implementation that owns all consensus canonical encodings, the concrete
domain tags, the one `addr()` derivation, checked `u64` amount newtypes, and codec fuzz
targets admitted into D6 CI.

This pass is bounded by the live instruction to implement every independently
specified part without deciding SI-001, SI-002, SI-003, an Al/Sarah/G0-owned TBD, or
an unspecified canonical-byte semantic. The packet is complete only if its whole
done-condition can be satisfied soundly. A partial, explicitly non-complete result is
permitted only to preserve a fully specified and independently useful sub-surface.

### Lane classification

**Overall packet: HUMAN / semantic-ambiguity STOP expected.** The done-condition
requires consensus-semantic canonical bytes, concrete domain-separation bytes, and
codec behavior. Protocol Spec v0.1 states that it becomes normative only at G0;
§1/§15 still reserve hash confirmation to G0; no byte values are assigned to the
named domain tags; and SI-002/SI-003 explicitly require P-007/G0 HUMAN ratification of
signed-coefficient encoding and SHAKE consumption. Selecting any such representation
would trip D17's INVENTION and HUMAN-lane rules.

**Independently safe sub-surface: AUTO.** Engineering Plan D7 already binds integer
amounts as `u64` newtypes with checked arithmetic and an error on overflow/underflow.
That representation does not depend on a wire codec, hash, domain tag, protocol
parameter, signature convention, or any open SI. This pass will implement and test
only that amount boundary, then stop the unresolved packet rather than claim its
done-condition.

The live parallel-worktree delegation is higher authority than the autonomous
prompt's canonical-checkout wording for this isolated, no-push pass. The worktree is
based exactly on main SHA `44ae3ae8807bc7ae594494870bffa96da6ea0ca1`, has the
canonical origin, contains no `OneDrive` segment, and will not push, open a PR, or
merge.

### Predicted diff surface

- `loop/reports/C7-p007-builder.md` — FRAME, ambiguity dependency graph, verification,
  A1–A11 adversary pass, and calibration.
- `crates/cj3-types/src/lib.rs` — one opaque `Amount(u64)` boundary, checked
  construction/access/arithmetic, typed arithmetic failure, and boundary tests.

No dependency, lockfile, CI workflow, active codec-fuzz handler, canonical codec,
hash/address implementation, domain-byte constant, SIS implementation, governing
document, packet/state/batch log, or other crate is predicted to change.

### Top risks

1. **Semantic laundering:** a convenient serializer, domain marker, or address hash
   could become a de facto consensus choice before G0.
2. **Fake fuzz admission:** activating D6's codec gate against a placeholder or
   noncanonical parser could falsely represent P-007 as complete.
3. **Money invariant weakness:** exposing raw arithmetic traits or unchecked mutation
   could let downstream code bypass D7 even if local unit tests pass.

**Falsifier:** this approach is wrong if the supposedly independent amount API needs
any canonical byte layout, owned parameter, saturating/wrapping operation, floating
point, or if a higher-authority source already assigns the missing codec/domain/SIS
conventions and this pass failed to find it.

**Confidence: HIGH** that the amount boundary is independently specified and that the
remaining done-condition must stop for HUMAN/G0 ratification.

## Autonomy/preflight evidence

- `git rev-parse HEAD` returned
  `44ae3ae8807bc7ae594494870bffa96da6ea0ca1`; branch is
  `feat/p007-cj3-types`; the worktree was clean before this report.
- `git remote -v` returned the exact canonical fetch and push URL,
  `https://github.com/Quigles1337/COINjecture3.0`.
- `gh repo view Quigles1337/COINjecture3.0 --json visibility,nameWithOwner` returned
  `PRIVATE` for the canonical repository.
- `loop/LEDGER.md` contains D17 RATIFIED and the explicit authorization for P-007.
- `loop/STATE.md` records `CAPACITY_FLAG: cj2-blocked-on-external`, observed at the
  P-005 exact-merge boundary. A fresh targeted read-only search of the available
  `C:\Users\LEET\COINjecture2.0` checkout found no `GATE-1`, `GATE-2`,
  `remediation-priority`, `blocked-on-external`, or contrary capacity marker.
- `docs/RESEARCH_SURVEY.md` is present. P-008 remains blocked and is outside this
  packet.

## BUILD — admitted sub-surface and tripwire stop

### Implemented without semantic invention

`crates/cj3-types/src/lib.rs` now defines one opaque `Amount(u64)` with:

- the full unsigned `u64` value domain and an explicit `ZERO`;
- `checked_add` and `checked_sub` returning typed `Overflow`/`Underflow` errors;
- no `Add`, `Sub`, wrapping, saturating, floating-point, serialization, or byte-layout
  implementation; and
- four unit tests covering zero/maximum representation, exact success cases,
  overflow, underflow, and a representative cross-product checked against Rust's
  primitive `checked_*` reference behavior.

This is exactly the higher-authority Engineering Plan D7 boundary. It neither
implements nor implies a canonical encoding.

### Deliberately absent

The pass did **not** add a hash dependency, Ed25519 dependency, address/public-key
wrapper, `addr()` implementation, domain-tag byte constant, encoder/decoder,
transaction/header/SIS wire structure, fuzz target, codec corpus, or active
`codec-fuzz-smoke` handler. It also did not edit the SIS prototype's explicitly
nonnormative SHAKE convention. Activating any of these now would silently choose an
open semantic or make a deferred gate look complete.

### Semantic-ambiguity tripwire — TRIPPED

P-007 cannot satisfy its complete done-condition from the current authority chain.
The minimum HUMAN/G0 ratifications needed are:

1. **Canonical codec grammar:** select the exact envelope/framing and define byte
   order; canonical zero; minimal unsigned-integer representation; list/byte-string
   lengths; struct/version/schema treatment; fixed field-order behavior; and exact
   rejection rules for nonminimal, duplicate, trailing, missing, and unknown data.
   The current prose constrains properties but does not identify a unique byte
   language.
2. **Hash and domain boundary:** confirm the §1 `H` primitive at G0; assign the exact
   byte string for each of `D_ADDR`, `D_TX`, `D_HDR`, `D_INST`, `D_SOL`, `D_BEACON`,
   and `D_GENESIS`; and ratify unambiguous concatenation/framing. Named tags without
   bytes cannot produce a unique `addr(pk)` or signing preimage.
3. **SI-002:** define the canonical signed coefficient encoding, including width or
   minimal representation, byte order, sign convention, and rejection of alternate
   encodings; then either define `s_max` or ratify the already documented equivalent
   per-coefficient squared-bound guard.
4. **SI-003:** ratify SHAKE-256 candidate width, candidate byte order, exact rejection
   ceiling, and rejected-candidate consumption before the existing prototype can be
   treated as consensus derivation or supplied with canonical vectors.
5. **P-8 limits:** ratify `TX_MAX_BYTES`, `BLOCK_MAX_BYTES`, and `SOL_MAX_BYTES` before
   the canonical decoders and fuzz boundary corpus claim the protocol's size-rejection
   behavior.

SI-001's hardness-wording correction remains an independent G0/HUMAN issue. It does
not alter the checked amount API and this pass did not try to resolve it, but G0 itself
cannot close while it remains open.

### Dependency graph

```text
canonical codec grammar ──────────────┬─> Tx/Header/body/genesis codecs ─┐
P-8 exact byte limits ────────────────┘                                  │
SI-002 signed representation + bound ───> SIS solution codec ────────────┤
SI-003 SHAKE consumption convention ────> consensus instance vectors ────┤
G0 hash confirmation + exact domain bytes ─> addr/signing/hash vectors ──┤
                                                                         v
                                                  codec fuzz targets + active D6 gate
                                                                         |
                                                                         v
                                                           P-007 done-condition
```

There is no sound path from the current symbolic names/prose constraints to those
byte-exact outputs. That is the packet STOP; it is not a request for permission to
choose convenient defaults.

## VERIFY

### Local mechanical results

- `cargo test -p cj3-types --locked`: **PASS**, four unit tests.
- `cargo fmt --all -- --check`: **PASS**.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  **PASS**.
- `cargo test --workspace --all-targets --all-features --locked`: **PASS**, 22 Rust
  tests across the workspace (including the four new `cj3-types` tests).
- `cargo build --workspace --all-targets --all-features --locked`: **PASS**.
- `cargo deny --locked --all-features check`: **PASS** — advisories, bans, licenses,
  and sources all reported `ok`.
- `cargo audit --deny warnings`: **PASS** against 22 locked dependencies.
- `scripts/ci/verify-geiger.ps1`: **PASS**, 11 packages, zero unsafe; its source-policy
  prerequisite reported 14 source files and 10 `cj3-*` crates.
- `scripts/ci/check-source-policy.ps1`: **PASS**.
- `git diff --check`: **PASS**.
- Pinned Lean 4.33.0 `lake build`: **PASS**, all ten jobs. The enclosing local
  `lean-conformance` handler then reported vector byte-hash drift solely because this
  Windows worktree has `core.autocrlf=true` and `.gitattributes` does not pin the JSON
  artifact's EOL. Disk CRLF SHA-256 was
  `B45D9472FB23174605DA4318FF0A6C35D1929EC0DD2CAA59E805AA87AEF92AD9`; read-only LF
  normalization reproduced the generated/committed semantic artifact SHA-256
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`. No P-005 file
  changed. This pre-existing Windows portability defect is reported, not hidden and
  not repaired through P-007's semantic-stop branch.
- `codec-fuzz-smoke`: `STATUS=NOT_YET_ADMITTED OWNER=P-007 canonical codecs`, exactly
  as expected. This is a deferral, **not** a fuzz-pass claim.
- Conservation and genesis handlers remain their explicit Phase 1 deferrals.

No hosted D6 result exists for this isolated pass: the live delegation forbids push,
PR creation, and merge. The parent packet integrator must decide whether to carry the
safe amount commit forward; any pushed integration head then requires exact-head D6.

## ADVERSARY PASS — A1–A11

Seat switched after re-reading the complete two-file diff.

| Axiom | Attack reviewed | Result |
|---|---|---|
| A1 | Could a caller self-report a value through the new type? | No consensus input or derivation API exists here; `Amount` only carries a checked integer. |
| A2 | Could the change admit miner-selected instances? | No instance surface changed. |
| A3 | Did timing, metadata, quality, or banned self-report identifiers enter? | No; the only non-test state is one `u64`. Banned-identifier scan is clean. |
| A4 | Did amount ordering or arithmetic affect fork choice? | No consensus/fork-choice dependency exists. |
| A5 | Could money wrap, saturate, become negative, or use a float? | No arithmetic traits are implemented; add/sub are checked and return distinct typed failures; boundary tests pass; no float exists. |
| A6 | Could this create a prevalidated state-apply bypass? | No state or apply API changed. |
| A7 | Could malformed remote input panic? | No decoder or remote-input path exists. Production code contains no panic, unwrap, expect, or unreachable path. |
| A8 | Did Rust decide a Lean-owned V-rule/STF semantic? | No. The amount type matches ratified D7 and does not implement transaction validity, conservation, or encoding. |
| A9 | Did the TCB gain unsafe/native/dependency surface? | No dependency or lockfile changed; crate retains `forbid(unsafe_code)`; geiger reports zero unsafe. |
| A10 | Is any incomplete capability presented as complete? | No. Source docs and this report state that codecs, domains, address derivation, and fuzz admission are absent. |
| A11 | Are verification claims tied to evidence, including the local Lean red? | Yes. Commands/results and both hashes are recorded above; no hosted-CI or fuzz success is claimed. |

### Findings

1. **STOP / blocking:** current authority does not determine unique canonical bytes,
   domain bytes, `addr()` output, SIS coefficient bytes, or SHAKE expansion. Closing
   P-007 anyway would be a consensus-split risk.
2. **Verification portability, non-P-007:** the P-005 byte-for-byte vector gate can
   false-red in a fresh Windows checkout because Git converts the committed LF JSON
   to CRLF. The generated vector remains byte-exact after LF normalization. This
   deserves a separately framed CI-evidence repair (for example, an explicit EOL
   policy), not an unpredicted edit mixed into this stopped consensus packet.
3. **Checked-type limitation, accepted:** `Amount::get` is necessary for inspection
   and a future ratified encoder, so Rust cannot prevent downstream code from
   extracting a primitive and misusing it. No unchecked arithmetic is provided on
   `Amount` itself; P-101 must keep value-changing operations in the typed checked
   API and its Lean-conformance boundary.

No Critical was introduced. The first finding prevents packet completion and is the
required HUMAN/G0 stop.

## CALIBRATE

- **Predicted vs actual diff:** exact. Only `crates/cj3-types/src/lib.rs` and this
  report changed; no manifest, lockfile, workflow, handler, codec, or governance file
  changed.
- **Risks materialized:** semantic laundering and fake fuzz admission were avoided by
  stopping. Checked arithmetic held. An unpredicted pre-existing Windows EOL defect
  surfaced in the unrelated Lean evidence handler and was preserved as an explicit
  red rather than explained away.
- **Confidence vs result:** HIGH remained appropriate. D7 was independently
  implementable; the unique-byte done-condition was not.
- **Surprise:** a byte-hash gate over a Git text artifact is platform-sensitive while
  `.gitattributes` leaves its EOL unspecified.
- **One process improvement:** every generated artifact checked byte-for-byte should
  declare its Git EOL policy at admission time and be verified once from a fresh
  Windows and Linux checkout.

## VERIFIED / ASSUMED / UNKNOWN

### VERIFIED

- The worktree began clean at the exact delegated main SHA, with private canonical
  origin and D17/D11 evidence as recorded under preflight.
- Engineering Plan D7 uniquely specifies the implemented checked `u64` amount
  behavior; the new API and its tests use no codec or open parameter.
- SI-002 and SI-003 are OPEN P-007/G0 HUMAN issues in
  `loop/reports/SPEC-ISSUES.md`; Protocol Spec §1/§15 also leaves hash confirmation,
  domain bytes, and the actual canonical grammar unresolved.
- All Rust/workspace/security-policy checks listed in VERIFY passed. The Lean build
  passed and the byte-hash wrapper's exact CRLF/LF discrepancy is recorded.
- The final code/report diff contains no owned TBD value, domain byte, canonical
  encoding choice, dependency, `unsafe`, float, banned identifier, or remote action.

### ASSUMED

- The fresh absence of a contrary marker in the available CJ2 checkout, together
  with CJ3's current `cj2-blocked-on-external` state, is sufficient for this isolated
  read/build/commit pass. This is safe because the branch is not pushed or merged and
  the parent re-checks D11 at its next externally visible boundary.
- A single `Amount` newtype is the smallest honest interpretation of D7's “amounts
  are `u64` newtypes.” Distinct fee/balance/subsidy wrapper semantics are not named by
  current authority and were therefore not invented.

### UNKNOWN

- The HUMAN answers to the five ratification groups above.
- Whether G0 keeps SHA-256 and what exact domain bytes/codec language it selects.
- Whether `s_max` becomes an explicit parameter or the derived squared-bound guard is
  ratified, and the exact SI-003 XOF convention.
- Whether the parent will integrate the independently safe amount commit before G0 or
  leave all P-007 code together for the post-ruling continuation.

## Packet result

**P-007: STOPPED / INCOMPLETE at the semantic-ambiguity tripwire.** The checked amount
sub-surface is locally complete and mechanically green; canonical encodings, domain
tags, `addr()`, and codec fuzz admission remain unimplemented pending the explicit
HUMAN/G0 decisions above.
