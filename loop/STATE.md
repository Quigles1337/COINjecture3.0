# COINjecture 3.0 Loop State

CYCLE: 11
PHASE: Phase 1 — open; Gate G0 PASS with items carried
PACKET: none in flight — the Gate G0 closeout (HUMAN) is on `feat/g0f-gate-record`; PR #22 awaits Al's review and merge; P-101 and packet P-011 are unblocked and not started
BRANCH: main
REMOTE: https://github.com/Quigles1337/COINjecture3.0
CAPACITY_FLAG: cj2-no-remediation-queued-or-running
CAPACITY_OBSERVED_AT: 2026-11-22, Al's ruling Part 0.3 — no COINjecture 2.0 remediation work is queued or running; CJ3 may proceed until Al states another condition
STATUS: G0_PASS — CLOSEOUT READY FOR AL'S REVIEW IN PR #22; THE BUILDER DOES NOT MERGE IT; AL NAMES WHICH OF P-101 AND PACKET P-011 RUNS FIRST

## Ground-truth state

- Al ratified D1, D2, D14, D15, D16, and D17 in the 2026-08-15 live ruling; D9 and
  D10 were reconfirmed.
- Continuous-batch autonomous execution is authorized for P-001 through P-007, P-009,
  and P-010 subject to every tripwire and packet-boundary capacity re-check. The
  2026-08-16 P-005 HUMAN-lane ruling now authorizes the Lean/vector implementation
  inside its exact symbolic/SI boundary and adds a mandatory non-merge review tail.
- P-001 merged through PR #1 at
  `8367de08bb3d3766bf49b9970eb3109fd1af4389`; exact-head PR CI and exact-merge-SHA
  mainline CI both passed all eleven jobs. Its ten crates remain empty boundaries with
  no protocol behavior.
- P-002 selected Wesolowski repeated squaring in a transparently derived imaginary-
  quadratic class group under D14. No production VDF crate or security/timing
  parameter was admitted; current pure-Rust options do not yet clear the full A9,
  evaluator, maturity, audit, and packaging bar.
- P-002 merged through PR #3 at
  `7ecba896c33c5a003bd6a9424f6742cd69c1156c`; exact-head PR CI and exact-merge-SHA
  mainline CI both passed all eleven jobs. `cj3-beacon` now exposes the bounded trait
  plus an opt-in iterated-hash devnet placeholder that compile-fails under the
  `cj3_testnet` tag.
- P-003 added the static problem-class contract, deterministic sampled-SIS prototype,
  exact safe-Rust checker, untrusted out-of-process LLL demonstrator, reproducible
  estimator/solver/verifier evidence, and provisional P-3/P-4 recommendations. It
  assigned no numeric class ID, codec, cadence, reward, or Sarah-owned value.
- P-003 merged through PR #5 at
  `e00561572c48137d535a44bcce55c04ad7db732b`; final-head run `31926469157` and
  exact-merge-SHA mainline run `31926645964` both passed all eleven D6 jobs. The
  sealed zero-finding security report is preserved at
  `loop/reports/P-003-security-scan.md`.
- P-004 added a bounded, class-neutral safe-Rust admission controller and a
  hash-pinned, read-only legacy calibration driver. SubsetSum, SAT, and TSP were
  rejected on sampled-hardness/checker-contract evidence; GraphColoring,
  Factorization, and SVP remain insufficient; Custom is not a concrete class.
- P-004 merged through PR #7 at
  `4644374f5073a929bf5ecc88c0e191f0d9bab1be`; exact-head run `31954357570` and
  exact-merge-SHA mainline run `31954588282` both passed all eleven D6 jobs. The
  inherited legacy-only `bincode 1.3.3` maintenance warning remains explicitly
  quarantined outside CJ3's workspace/runtime.
- P-005 originally classified the whole Lean/vector packet HUMAN before
  implementation. Draft PR #9 contains only its FRAME/STOP checkpoint; no Lean rule,
  vector, CI admission, Rust state machine, normative semantic choice, or owned TBD
  was added. Checkpoint run `31955875423` passed all eleven D6 jobs. Al's 2026-08-16
  ruling now authorizes implementation strictly from Protocol Spec §§7–8 and §14,
  keeps every owned/G0 TBD symbolic or absent, keeps SI-001/2/3 abstract and
  unresolved, and makes any new semantic ambiguity a fresh stop.
- P-005's builder tail is now explicit, as of 2026-08-16: complete the adversary pass and exact-head CI,
  mark PR #9 ready-for-review, and do not merge it. Al's line-by-line review and merge
  are the packet done-condition. P-101 remains blocked on both Gate G0 and that merged,
  human-reviewed P-005. While review is pending, P-006, P-007, P-009, and P-010 (if
  present) may execute out of strict order under their own tripwires.
- P-005 resumed only far enough to create a partial draft Lean candidate. The §8 STF
  applies §11's quality-scaled reward, but §8's conservation equation permits total
  issuance to increase by exactly `subsidy(height)`. Because §11 reward may exceed
  subsidy and no funding debit exists, `SI-004` records a new HUMAN semantic
  ambiguity. No relationship, owner value, or funding mechanism was inferred.
- Al resolved SI-004 on 2026-08-16 by ratifying Model 4 bounded-above quality
  normalization. Reward is `subsidy · min(Q,R_MAX·SCALE)/(R_MAX·SCALE)` with floor
  division and `R_MAX ≥ 1`; §8 conservation adds realized reward, while subsidy is an
  issuance ceiling. No funding account exists. The P-7 value and curve shaping remain
  owner-controlled and unfilled. P-005 is authorized to resume, and Lean must prove
  `reward ≤ subsidy` from the formula rather than assume it.
- A concurrent local Codex task wrote to the same `spec/` surface during the first
  build. It was stopped before any commit, push, PR mutation, hosted CI, or queue
  advance. Its partial Tx/STF candidates are preserved in PR #9's checkpoint and are
  not represented as normative or fully verified.
- PR #9 was kept draft while the Model 4 implementation, full Lake build,
  deterministic symbolic JSON artifact, adversary pass, and active Lean D6 gate were
  completed. Its authorized builder transition is ready-for-review, never merge.
- P-005's Model 4 builder surface is complete at implementation commit
  `388ede835e8c9e668f5f0126918193ff59f589cd`. The complete Lean 4.33.0 project builds;
  27 symbolic/non-normative vectors regenerate exactly at SHA-256
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`; the STF proves
  `reward ≤ subsidy` from Model 4 and uses that exact reward for credit/conservation;
  the full local D6-equivalent and A1–A11 adversary passes are green. The sealed
  exact-range security diff found zero reportable findings and is preserved durably at
  `loop/reports/P-005-security-diff-scan.md`. Exact implementation/report head
  `f494a605825a2d2dcd15d8babb71193abccae18f` passed all eleven hosted D6 jobs in run
  `31963016862`. The evidence-only closeout head's check rollup is attached to PR #9;
  the PR is handed to Al ready-for-review and remains unmerged with no Sarah contact.
- Al completed that first line-by-line review without merging and returned F1/F2/F3.
  The resumed HUMAN remediation is implemented at
  `97fa5110af60b832a9f3b26dd57cc7690a31cc75`: `LawfulStateOps` states the additive
  total-balance replacement, read-after-write, and read-other-address laws;
  `applyTransaction_conserves` explicitly eliminates all five sender/recipient/miner
  alias partitions; `applyBlockCandidate_conserves` and
  `conservationTarget_holds` discharge successful candidate conservation; and
  `Tx.validate` has no `unreachable!` path while `v9_allows_self_send` remains.
  Protocol Spec §11, the LEDGER, P-101, and Gate G2 bind `Context.rewardInputs` to
  `check(derive_instance(instance_seed,size_param),solution)` and forbid
  block-supplied quality.
- The second-review remediation passes the full pinned Lake build, unchanged
  27-vector regeneration at SHA-256
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`, the complete
  local D6-equivalent suite, and A1–A11. Codex Security scan
  `1b01d903-6760-4402-8925-311735487350` sealed complete coverage with zero
  reportable findings; its deterministic report is durable at
  `loop/reports/P-005-second-review-security-diff-scan.md`, SHA-256
  `E1C78E9602266105A6739C7104637A384F4C280618C1C056805827F726C07B68`. Hosted
  content/evidence-head D6 run `31965617214` passed all eleven jobs on exact SHA
  `737d462e9ebf01bb9b162ee55bbc39fe11b3db06`. The final evidence-only rollup is
  pushed after this record and must itself pass exact-head D6; its SHA/run evidence is
  attached directly to PR #9 so no post-run commit invalidates the handoff head.
- Al's 2026-08-16 governance injection ratified AMEND-1/2/3 and approved/unblocked
  P-009. `docs/AUDIT_TRACEABILITY.md` was verified at SHA-256
  `58C762568A63D8A4AFEACDEB1535AA9C930099F49118D2B45B9B483B8D560EFC` and preserved
  unchanged; the amendments are applied to Protocol Spec §7/§12 and Gate G1 and
  recorded in the LEDGER overlay.
- P-009's required first action copied both supplied source audits byte-for-byte into
  `loop/evidence/`. Source and durable-copy sizes/hashes are sealed in
  `loop/evidence/P-009-SOURCE-MANIFEST.md`. P-009 then completed the authorized
  source interpretation: matrix v0.2 accounts for all 33 third-party findings, all 25
  Lean claim-points, and all eight Lean recommendations. GAP-7 through GAP-13 were
  proposals in that matrix and were ruled on 2026-11-22 (G0-E); the absent original
  Codex report's exact per-finding inventory remains UNKNOWN.
- P-010 replaced the stale P-001 README with a 254-line evidence-linked repository
  index. At its merge it recorded private/pre-testnet scope (the scope row now reads
  public, per ruling Part 0.1), the open G0 position, implemented versus planned
  surfaces, audit lineage, A1–A11, the crate map, governance, evidence checks, working
  commands, limitations/TBD ownership, and the exact TBD license line.
- P-010 merged through PR #10 at
  `3b43cca42e14e53412747fb2b559d6fc29c4ac9d`; final PR-head run `31957723975` and
  exact-merge-SHA mainline run `31957922841` both passed all eleven D6 jobs. Its
  adversary pass found and fixed one broken rendered heading fragment before merge.
- P-010's docs-only closeout merged through PR #11 at
  `1e986cb22956f2970e08281e18a11282c812ed9e`; exact-head run `31958216281` and
  exact-merge-SHA mainline run `31958393946` both passed all eleven D6 jobs.
- The standing README drift-control rule is RATIFIED: every G0–G4 closeout refreshes
  the README status block and roadmap marker in the same closeout.
- `loop/reports/SPEC-ISSUES.md` has no open issue. The 2026-11-22 ruling resolved
  SI-001 (conditional SIS-hardness wording, G0-B1), SI-002 (undefined `s_max`, G0-C4)
  and SI-003 (the SHAKE candidate-byte convention, G0-C5). SI-004 is resolved by the
  ratified Model 4 ruling.
- The three pinned audit-tool caches are established on `main`; subsequent packet PRs
  can use the verified cache-hit path.
- GitHub reports `main` is not currently protected (read again on 2026-11-22:
  `gh api repos/Quigles1337/COINjecture3.0/branches/main/protection` returned 404
  "Branch not protected"). P-001 documents the required contract in
  `.github/BRANCH_PROTECTION.md` and does not claim the setting is active.
- Capacity (D11): Part 0.3 of Al's 2026-11-22 ruling states that no COINjecture 2.0
  remediation work is queued or running and that CJ3 may proceed. That is the capacity
  condition for preflight step (f) and for the pickup guard until Al states another.
  `remediation-priority` in either project's loop still pauses CJ3 immediately.
- `docs/RESEARCH_SURVEY.md` and
  `loop/PROMPTS/AUTONOMOUS_BUILDER.md` were supplied for the first governance commit.
- P-008 remains blocked on the exact Al-supplied public frontend repository URL.
- Al's second P-005 review accepts F1/F2/F3 as closed and authorizes PR #9 to merge.
  Effective with that merge, the Lean V1–V9/STF encoding, including conservation
  under `LawfulStateOps`, is HUMAN-RATIFIED. The symbolic vectors remain explicitly
  non-normative; the 2026-11-22 ruling resolved SI-001/SI-002/SI-003 without changing
  any Lean file or vector.
- P-101 carries the three `LawfulStateOps` laws, checker-derived `rewardInputs`
  provenance, checked construction of the `R_MAX·SCALE` divisor, and concrete storage
  read/write coherence. Effective with PR #9's merge, P-101 was blocked on Gate G0
  only, until the gate passed on 2026-11-23.
- CJ3 pins Lean 4.33.0 while the COINjecture 2.0 Lean project is recorded by Al as
  Lean 4.28.0 plus Mathlib 4.28.0. The compatibility gap is reserved for the D16
  reveal and MUST NOT be resolved beforehand.
- P-005 is COMPLETE. PR #9's final reviewed head
  `8d8ec440f86cbe06fc30eff78f9cfcc6076c0180` passed all eleven D6 jobs in run
  `31967696333`; the expected-head guarded merge produced
  `916e5027f0918973bf45d6a2bf90abd2ae253197`; and exact-merge-SHA mainline run
  `31968030240` passed all eleven jobs. The Lean V1–V9/STF encoding is therefore
  HUMAN-RATIFIED as of that merge. P-101 was then blocked on Gate G0 only.
- P-005's docs-only immutable closeout merged through PR #13 at
  `44ae3ae8807bc7ae594494870bffa96da6ea0ca1`; exact-head run `31968497094` and
  exact-merge-SHA run `31968806291` both passed all eleven D6 jobs.
- P-006 is COMPLETE. PR #16's exact head
  `eb6b96d612bf8de4f43d6f42639da874b811bd0b` passed all eleven D6 jobs in run
  `31971074815`; merge `7244f094aa22d1890039a535ad997c9775ba9ed3` passed exact-
  merge-SHA run `31971438778`. The sealed matrix contains 5,971,968 simulated blocks.
  The full honest/unmanipulated-quality and adversarial envelopes are both 0/36. The
  isolated hash loop passes all 6/6 unique settings, but no tested 64–256-block size
  window survives the solve-power and interaction sensitivities. G0-A subsequently
  ratified static `size_param`, struck P-11 as moot, and confined P-1/P-2 to the hash-
  target controller. P-1 remains unfilled. P-2 is provisionally `W=32`, gain `1/8`,
  clamp `[8/9,9/8]`, subject to re-derivation when P-1 is ratified. No dynamic size
  retarget is selected or permitted.
- P-007 was INCOMPLETE in the HUMAN lane until its resume on 2026-11-22; the resumed
  build is described at the end of this list. PR #14 preserved only the
  independently specified checked `Amount(u64)` boundary and the P-007 stop report at
  exact head `9acea83ca17d67a19e0d41aeb5e7275666a54013`; run `31969740766` passed all
  eleven D6 jobs. Al merged PR #14 on 2026-10-12 as
  `358114048c3a38f4821b88fce8dffd14abbd42a1`, and D6 run `38323998550` on that commit
  concluded success; the merge stands (ruling Part 0.4). The rulings its stop needed
  are now given: canonical codec grammar (G0-C1), hash, domain bytes and preimages
  (G0-C2), layouts (G0-C3), SI-002 (G0-C4), SI-003 (G0-C5), P-8 limits (G0-C6), strict
  Ed25519 semantics (G0-C7) and the evidence owed, including live codec fuzz targets
  (G0-C8). SI-001 is resolved by G0-B1.
- P-009 is COMPLETE. Refreshed PR #15 head
  `34ce596d0f1160b152a33d95b1f6ddf25e4111c4` passed all eleven D6 jobs in run
  `31971741833`; merge `ce0b9752c9101e89f93a90379cd9e5ac1a08842d` passed exact-
  merge-SHA run `31972017916`. Matrix v0.2 records 17 mapped, 10 partial, and six
  surface-excluded third-party findings; tiers all 25 Lean claims without importing
  proof credit; and left GAP-7 through GAP-13 as proposals, which the 2026-11-22
  ruling then ruled (G0-E).
- Al's 2026-08-16 partial Gate G0 ruling set the gate disposition to **HOLD**, not
  PASS; the gate passed on 2026-11-23. G0-A ratifies D2 Option 2: `size_param` is an active protocol constant changed
  only by explicit human-ratified upgrade; B3 compares the header value to that
  constant; P-11 is struck as moot; and every phase gate re-reviews size adequacy.
- G0-A merged through PR #18 at
  `9bbbd43fd9e4c07f8b389f182b34281183be3737`. Exact-head D6 run `31975177926` and
  exact-merge-SHA D6 run `31977067852` both passed all eleven jobs.
- P-1/P-2 concern the hash-target knob only. P-1 remains unfilled and receives no
  builder proposal because P-006 cannot derive an absolute cadence. P-2 is RATIFIED
  AS PROVISIONAL: `W=32`, gain `1/8`, clamp `[8/9,9/8]`, selected as a conservative
  interior point of the 36/36 passing region rather than a tuned optimum. P-2 MUST be
  re-derived when P-1 is ratified and is not yet implemented.
- The standing selection-bias lesson is binding: an honestly recomputed quantity can
  still be adversarially distributed through selective publication. Any future
  miner-behavior control loop must evidence withholding resistance or be rejected.
  P-006 modeled equilibrium size retention of `0.423–0.806` at 35% strategic share
  and `0.240–0.700` at 51%.
- Al's ruling of 2026-11-22 is recorded verbatim in `loop/LEDGER.md`. It ruled G0-B,
  G0-C, G0-D and G0-E, held G0-F, and left the gate at **HOLD**, where it stayed until
  the ruling of 2026-11-23. Its clauses are cited here as `G0-` plus their label.
- Visibility: the repository is PUBLIC by Al's decision, and D9's "private through the
  build" is superseded (Part 0.1). Preflight step (d) now requires `PUBLIC` and stops on
  any other value. "No visibility changes" stays a standing prohibition. D10 and D16
  are not ruled; every value and seat they reserve stays reserved and UNFILLED, and
  every prohibition on contacting or notifying anyone stays in force (Part 0.2).
- G0-B: SI-001 is resolved by the supplied "Hardness basis" text. P-3 is accepted as
  provisional at `VALIDATION_BUDGET = 15 ms` over `derive_instance` plus `check`; the
  evidence so far times `check` alone. The proposed P-4 tuple
  `(128, 3840, 16411, 3840)` and the 128-bit threshold as a way of choosing a tuple are
  rejected; P-4 and `ACTIVE_SIZE_PARAM` stay UNFILLED and the candidate family is not
  ratified. Packet P-011, the SIS solver envelope, is commissioned and queued after
  P-007.
- G0-C: CJ3 Encoding v1 is ruled — grammar, SHA-256, the seven domain tags and their
  preimages, the transaction, size-parameter, header, solution and block layouts, the
  SIS coefficient guard, the SHAKE expansion convention of
  `crates/cj3-classes/src/sis.rs` as it stands, the P-8 limits (`TX_MAX_BYTES = 188`;
  `SOL_MAX_BYTES = 2 * m`, unfilled while P-4 is; `BLOCK_MAX_BYTES = 1,048,576`
  provisional) and the exact Ed25519 verification. SIS is class 1; 0 is never assigned.
  None of it is implemented: P-007 owes the code and the G0-C8 evidence.
- G0-D: P-10 and the timestamp unit stay UNFILLED and are ruled with P-1. P-7, P-9 and
  P-12 are unchanged owner values.
- G0-E: GAP-12 is ratified by G0-C7. GAP-7, GAP-8, GAP-10, GAP-11 and GAP-13 are
  ratified as Phase 3 requirements and are in Protocol Spec §12. GAP-9 is ratified as a
  requirement on the packet that introduces the mempool. Gate G3 tests each. Their
  numbers are operating configuration that the builder proposes and Al reviews at G3.
  The original Codex report's inventory stays UNKNOWN and does not block G0.
- The cycle 9 session wrote no protocol code. Its record was PR #20, with the builder
  report at `loop/reports/C9-g0-bcde-ruling-builder.md`. Al reviewed and merged PR #20
  at `2026-11-23T03:51:34Z` as `f601224db855e6cc69e840fb169d85df22c547b8`; exact-merge-SHA
  D6 run `45108419759` passed all eleven jobs.
- Al's ruling "LIVE CJ3 RULING - P-007 RESUME" is recorded verbatim in `loop/LEDGER.md`
  as the first commit of the P-007 branch. Its clauses are cited as "resume 2.2" and so
  on. It confirms PR #20 and its four unnamed edits; rules the eight points of report C9
  §4.2; fixes the five D17 conditions, now recorded beside D17; orders one text-only
  commit for stale statements; and resumes P-007 in the HUMAN lane.
- Resume Part 3 is applied in one text-only commit. The sentences in `spec/Spec/Tx.lean`,
  `spec/Spec.lean`, `spec/Main.lean` and `spec/README.md` that called SI-001, SI-002 or
  SI-003 open, unresolved or pending are corrected; no definition, theorem, vector or
  exported string changed; `spec/vectors/p005-draft.json` is byte-identical (SHA-256
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1` before and after).
  The q = 17 fixture is recorded as normative for the expansion rule. The P-005 vectors
  stay symbolic and non-normative (resume 2.7).
- P-007 is COMPLETE (pass 1.1). Al reviewed and merged PR #21 at `2026-11-23T05:36:52Z`
  as `333c67086012d03f3c654cba20526e5c0f5e6b4d`. Its exact head
  `92f9be92265c83c5f0b3e27acd3311027d910c56` passed all eleven D6 jobs in run
  `45114968343`, and exact-merge-SHA run `45118879871` passed all eleven.
  `cj3-types` holds CJ3 Encoding v1: the grammar and the typed decode errors, the seven
  domain tags, `H` and the six preimages, the single `addr()`, the transaction, header
  and block layouts, the class identifier table, the 20-byte SIS size-parameter layout
  and the §1.3 Ed25519 verification. `cj3-classes` imports those layouts, binds SIS to
  class identifier 1, rejects a `beta_squared` above `32767^2`, and holds the SIS
  solution encoding; `derive_instance` and `check_sis` are unchanged. No tuple is
  compiled in as active, and P-1, P-4, P-10 and `ACTIVE_SIZE_PARAM` are unfilled.
- P-007's evidence (G0-C8): worked vectors for every layout and preimage, the expansion
  vectors, and 932 Ed25519 vectors (5 from RFC 8032 §7.1, the 914 of C2SP CCTV at commit
  `b88c4b206254ac543df240097e0f1fcc6a664821`, 13 builder-authored), each derived by a
  standard-library script under `bench/p007-vectors/` and again by the Rust code. The
  CCTV copy is under a three-clause BSD licence kept beside it.
- Al's ruling "LIVE CJ3 RULING - GATE G0-F" of 2026-11-23 is recorded verbatim in
  `loop/LEDGER.md` as the first commit of `feat/g0f-gate-record`. Its clauses are cited
  as "pass 3.5" and so on; `G0-F1` and `G0-F3` remain clauses of the 2026-11-22 ruling.
- **Gate G0 is PASS, with the items of pass 3.5 carried, and Phase 1 is open** (pass
  3.1). The conditions of Engineering Plan §4 are met as pass 3.2 states, and the
  evidence for each is in `loop/reports/C11-g0f-gate-builder.md` §3.2. D1, D2, D14 and
  D15 are confirmed, D2 as G0-A amended it.
- D1 stands on a condition (pass 3.3). SIS remains the genesis anchor class. No tuple is
  ratified, and it is not yet measured whether the family has a size that miners can
  solve at a cost worth calling useful work. Packet P-011 measures that. If it finds no
  such size, D1 returns to Al.
- Size adequacy, the standing item of G0-A, at this gate: there is no active
  `size_param` to retain or to change, because `ACTIVE_SIZE_PARAM` and P-4 are UNFILLED.
  Pass 3.3 is the gate's word on adequacy.
- The Protocol Specification is v0.2 and NORMATIVE from that ruling (pass 3.4): its
  rules and layouts bind every implementation and change only by Al's ruling. Excepted
  are every value marked UNFILLED or PROVISIONAL and every item pass 3.5 carries. No
  implementation may assume a value for one.
- Three files are NORMATIVE conformance vectors from the merge of PR #21 (pass 2.4):
  `bench/p007-vectors/encoding-v1.vectors.txt` for the layouts and preimages of Encoding
  v1, `bench/p007-vectors/shake-expansion.vectors.txt` for the expansion rule, and
  `bench/p007-vectors/ed25519/c7-expected.vectors.txt` for Protocol Spec §1.3, the
  thirteen builder-authored vectors included. What is normative is the relation: these
  inputs give these bytes and these verdicts, and every implementation reproduces them.
  The input values assign nothing. If the text of the specification and a vector ever
  disagree, that is a STOP and a ruling; neither wins by default. The P-005 vectors stay
  symbolic and non-normative.
- The other points of report C10 §4.2 are ruled (pass Part 2). `sha2 =0.10.9` and
  `curve25519-dalek =4.1.3` are accepted as pinned, the second on the condition of pass
  4.5c, and arithmetic written in `cj3-types` is refused. `libfuzzer-sys` is accepted
  for the fuzz crate alone, on the condition of pass 4.5b. The builder's reading of the
  active tuple in the block codec, its order of checks, its layout tests over arbitrary
  bytes, its unnamed edits, the `cj3-classes` API and the three ways of showing the
  `BLOCK_MAX_BYTES` boundary stand or are accepted.
- The Gate G0 closeout is PR #22 from `feat/g0f-gate-record`, in the HUMAN lane. It
  holds the verbatim overlay; one text-only commit (pass 4.4); the four build items of
  pass 4.5, each in a commit of its own, with one later fix to item (c); and the gate
  record (pass 4.3). The builder does not merge it.
- Pass 4.4, text only. The doc comment on `v1` in `spec/Spec/Tx.lean` no longer calls
  the strict canonical decoder unresolved, and the README diagram no longer calls the
  Lean rules and vectors planned. No Lean definition, theorem, vector or exported string
  changed; `spec/vectors/p005-draft.json` is byte-identical (SHA-256
  `30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1` before and after).
  The status lines of the three vector files say what pass 2.4 rules; they were changed
  in `derive.py` and regenerated, and only comment lines differ.
- Pass 4.5a. The D6 `codec-fuzz-smoke` job stays a bounded smoke run: six cargo-fuzz
  targets, a fixed number of runs, a fixed seed. The D6 Nightly `full-fuzz-corpus` job
  is LIVE: the same six targets on the same toolchain, `nightly-2026-11-15`, each for
  600 seconds, from a seed that is the UTC date. One script holds the target list for
  both. No corpus is kept from one night to the next.
- Pass 4.5b. `cargo deny` runs over the fuzz workspace in the `dependency-policy` job
  with `fuzz/deny.toml`: the root policy plus the licence term NCSA for
  `libfuzzer-sys@0.4.13` alone. A check fails D6 if that file differs from `deny.toml`
  in anything else. The root `deny.toml` is unchanged.
- Pass 4.5c. Every build of `cj3-types` in this repository uses `curve25519-dalek`'s
  serial back end: `.cargo/config.toml` passes the setting, the fuzz gates pass it
  themselves, and the crate refuses to compile unless the setting is exactly `serial`.
  D6 proves the refusal on every run. Before this, on x86_64, the library chose a back
  end that picks vector code by processor at run time. It is not a claim that no unsafe
  code remains on the path: the README limitation says what remains, that the serial
  back end is two bodies of code chosen by pointer width, and that `sha2` still chooses
  an implementation by processor.
- Pass 4.5d. The source-policy gate fails on a domain tag constant named in any Rust
  source outside `crates/cj3-types`, and proves the rule on planted sources every run.
  One doc comment in `crates/cj3-beacon/src/devnet.rs` named a tag and is reworded.
- In the ruling's words, "D10 and D16 are still not ruled" (pass 3.5). Every value and
  seat they reserve stays reserved and UNFILLED, and every prohibition on contacting or
  notifying anyone stays in force. The meaning of the quoted sentence, as Al gives it in
  clause 2.1 of his ruling "LIVE CJ3 RULING - P-101 TO ITS FRAME" of 2026-11-23 (cited
  as "launch 2.1"): D10 and D16 stand as ruled on 2026-08-15, and neither the 2026-11-22
  ruling nor the gate ruling revisited them.
- The builder report is `loop/reports/C11-g0f-gate-builder.md`. Its §4.2 lists the
  points left for Al.

## Next action

1. Al reviews PR #22 and merges it. The builder does not.
2. Al says which of P-101 and packet P-011 runs first, in the message that launches it
   (pass 3.6). Both are unblocked. The builder starts neither before that message.
   Packet P-011 blocks Phase 2 and does not block P-101.
3. Carried, each to the boundary where Al rules it (pass 3.5): P-1, P-4, P-10,
   `ACTIVE_SIZE_PARAM`, the re-derived P-2, the re-measured P-3, the re-ruled
   `BLOCK_MAX_BYTES` and the timestamp unit (from packet P-011's report; they block
   Phase 2); how `tx_root` and the state commitment are built (P-101 proposes both in
   its FRAME and stops there for Al's ruling); the byte order in which a 32-byte value
   is read as a number (the first Phase 2 FRAME); the genesis layout and its preimage
   (the Phase 1 genesis packet); P-7, P-9 and P-12 (their owners).
4. P-008 remains blocked on the exact frontend URL, and is carried.
5. On any resume, run the preflight and re-check D11 before touching the repository.
   `remediation-priority` in either project pauses CJ3.

No frontend URL may be inferred for P-008, and no Al- or Sarah-owned TBD may be
filled.
