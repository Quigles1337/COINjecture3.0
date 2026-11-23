# COINjecture 3.0 Packet Queue

**Queue status:** GATE G0 PASS — PHASE 1 OPEN. P-007 IS COMPLETE. P-101 RUNS FIRST
(launch 3.1) AND IS AT ITS FRAME, STOPPED FOR AL'S RULING; NO KERNEL CODE EXISTS. PACKET
P-011 IS UNBLOCKED AND NOT STARTED.
Al's ruling "LIVE CJ3 RULING - P-101 TO ITS FRAME" of 2026-11-23, recorded verbatim in
`loop/LEDGER.md`, confirms the merge of PR #22, rules the points of report C11 §4.2,
says that P-101 runs first and takes it to its FRAME; its clauses are cited below as
"launch 3.1" and so on. Its record is PR #23, which Al reviews and merges; the builder
does not.
Al's ruling "LIVE CJ3 RULING - GATE G0-F" of 2026-11-23, recorded verbatim in
`loop/LEDGER.md`, passes the gate with items carried; its clauses are cited below as
"pass 3.5" and so on. Its record was PR #22, which Al merged on 2026-11-23 as
`d2c5f75985c776ce1c9ef204c02481715f6fac99`. Al's ruling "LIVE CJ3 RULING - P-007 RESUME", also recorded verbatim, resumed
P-007; its clauses are cited below as "resume 2.2" and so on. G0-A is RATIFIED:
`size_param` is a static human-upgraded protocol constant, P-11 is struck as moot, P-1
remains unfilled with no proposal, and P-2 is provisionally ratified as `W=32`, gain
`1/8`, clamp `[8/9,9/8]`, subject to re-derivation when P-1 is ratified. Al's ruling of
2026-11-22, recorded verbatim in `loop/LEDGER.md`, ruled G0-B, G0-C, G0-D and G0-E and
held G0-F. PR #14 was merged by Al on 2026-10-12 as
`358114048c3a38f4821b88fce8dffd14abbd42a1`; it carried only the checked `Amount`
boundary and the P-007 stop report. PR #20, the 2026-11-22 ruling's record, was merged
by Al as `f601224db855e6cc69e840fb169d85df22c547b8`. PR #21, the rest of P-007, was
merged by Al on 2026-11-23 as `333c67086012d03f3c654cba20526e5c0f5e6b4d`, and P-007 is
COMPLETE (pass 1.1). P-008 remains blocked on the exact frontend URL. No held or
owner-controlled value was inferred.

**Order of work.** Ruling F2 of 2026-11-22 is carried out as far as the gate: the
ruling's record (PR #20, merged); P-007 (PR #21, merged); Al's G0-F ruling (given:
PASS). What remains of it is P-101 and packet P-011. Both are unblocked (pass 3.6). Al's
launch ruling says that P-101 runs first (launch 3.1) and takes it to its FRAME and no
further. The FRAME is written and stops for Al's ruling; the builder does not begin
P-101's build before that ruling (launch 4.7). Packet P-011 is not started, and Al says
when it runs.

Clauses of the 2026-11-22 ruling are cited below as `G0-` plus their label, so `G0-F3`
is clause F3 of that ruling, "pass 3.5" is a clause of the Gate G0-F ruling of
2026-11-23, and "launch 3.1" is a clause of the ruling "P-101 TO ITS FRAME" of the same
day.
Packet P-011 is not parameter P-11 (`SIZE_RETARGET_WINDOW`, struck).

| Packet | Phase | Queue status | Blocking condition |
|--------|-------|--------------|--------------------|
| P-001 | Phase 0 | COMPLETE — `8367de08` | PR #1 and exact-merge-SHA D6 CI green |
| P-002 | Phase 0 | COMPLETE — `7ecba896` | PR #3 and exact-merge-SHA D6 CI green |
| P-003 | Phase 0 | COMPLETE — `e0056157` | PR #5 and exact-merge-SHA D6 CI green |
| P-004 | Phase 0 | COMPLETE — `4644374f` | PR #7 and exact-merge-SHA D6 CI green |
| P-005 | Phase 0 | COMPLETE — HUMAN-RATIFIED — `916e5027` | PR #9 and exact-merge-SHA D6 green |
| P-006 | Phase 0 | COMPLETE — `7244f094`; G0-A applied | PR #16 exact-merge D6 green; static `size_param` ratified, P-11 moot, P-1 unfilled, P-2 provisional |
| P-007 | Phase 0 | COMPLETE — HUMAN — `333c6708` (PR #21); PR #14 merged earlier as `35811404` (checked amounts and stop report only) | PR #21 exact-head D6 run `45114968343` and exact-merge-SHA D6 run `45118879871` green; Al reviewed and merged it, and ruled the points of report C10 §4.2 (pass Part 2) |
| P-008 | Phase 0 / Phase 4 seam input | BLOCKED | Exact public frontend repository URL supplied by Al; URL MUST NOT be guessed |
| P-009 | Phase 0 / audit traceability | COMPLETE — `ce0b9752` | PR #15 and exact-merge-SHA D6 green; GAP-7–13 were proposals at merge and were ruled on 2026-11-22 (G0-E) |
| P-010 | Phase 0 / documentation | COMPLETE — `1e986cb2` | Feature PR #10 and closeout PR #11; both exact-merge-SHA D6 runs green |
| P-011 | Phase 0 / SIS solver envelope | UNBLOCKED (pass 3.6) — not started; commissioned by G0-B4 | P-101 runs first (launch 3.1); Al says when packet P-011 runs. Blocks Phase 2; does not block P-101. D1 stands on what it finds (pass 3.3). It measures and reports costs and judges nothing (launch 2.10) |
| P-101 | Phase 1 head | AT ITS FRAME — HUMAN — STOPPED FOR AL'S RULING (launch 3.1, 4.7). No kernel code exists | Al's ruling on the FRAME, `loop/reports/C12-p101-builder.md`: twelve decisions, among them how `tx_root` and the state commitment are built (pass 3.5, launch 3.3) and six proposed domain tags, which Al assigns. The build does not begin before that ruling |

## Seed packet definitions

The P-001 through P-007 and P-101 definitions below are seeded from
`docs/ENGINEERING_PLAN.md` §9.

### P-001 — Repo scaffold

Workspace, crate skeletons per Plan §2, full D6 CI green on the empty workspace,
branch protection config documented. **No protocol code.**

### P-002 — Beacon spike

Read/report: survey pure-Rust VDF options (Wesolowski, Pietrzak, class-group vs RSW),
grinding analysis including withholding; recommend; define `Beacon` trait; devnet
placeholder behind trait with `NOT-TESTNET-GRADE` banner.

### P-003 — SIS genesis class

Implement `ProblemClass` for SIS; parameter search (n, m, q, β candidates —
**discovered, not invented**, per A11) targeting the block cadence; measure
solve/verify asymmetry with the out-of-process solver; report measured numbers and
recommended floors. Reward-margin analysis is flagged for Al + Sarah.

### P-004 — Admission bench harness

Generic runner that takes any `ProblemClass` candidate and produces the
hardness/asymmetry report; run 2.0's legacy classes through it first as calibration
(read-only with respect to consensus).

### P-005 — Lean scaffold

Lake project; `Spec/Tx.lean` encoding V1–V9 from Protocol Spec §7;
`Spec/Stf.lean` skeleton; JSON vector exporter (`lake exe vectors`). **Designed Sarah
entry seam.** Conformance test lands with P-101, not here. The 2026-08-16 HUMAN-lane
authorization permits builder implementation only within the symbolic/SI boundary in
`loop/LEDGER.md`; Al's completed second review and PR #9 merge are its done-condition.

Pickup on 2026-08-16 classified the whole packet HUMAN because the done-condition
directly requires both `Spec/*.lean` content and protocol-vector definitions, each
named by D17 as HUMAN lane. Al reviewed the complete encoding twice, returned F1/F2/F3
after the first review, and accepted all three as closed after remediation. PR #9
merged at `916e5027f0918973bf45d6a2bf90abd2ae253197`; exact-head run `31967696333`
and exact-merge-SHA run `31968030240` both passed all eleven jobs. The V1–V9/STF
encoding is HUMAN-RATIFIED. The vector artifact remains explicitly non-normative and
symbolic. The 2026-11-22 ruling resolved SI-001/SI-002/SI-003 without changing any Lean
file or vector.

### P-006 — Difficulty simulation

Offline, no chain code: model the two-knob retarget under honest/adversarial hash- and
solve-power schedules; report stability envelope; feeds the D2 Phase-2 gate.

**Outcome:** COMPLETE through PR #16 at
`7244f094aa22d1890039a535ad997c9775ba9ed3`; exact-head run `31971074815` and
exact-merge-SHA run `31971438778` passed all eleven jobs. The sealed 5,971,968-block
matrix found 0/36 full honest/unmanipulated-quality passes and 0/36 full adversarial
passes. All 6/6 unique isolated hash-loop settings passed, but no tested size-window
setting survived the solve-power/interaction sensitivities. G0-A subsequently
ratified the static human-upgraded `size_param` option and struck P-11 as moot. P-1
and P-2 now apply only to the hash-target controller. P-1 remains unfilled and no
absolute cadence is proposed. P-2 is provisionally ratified as `W=32`, gain `1/8`,
clamp `[8/9,9/8]`: a conservative interior point, not a tuned optimum, which MUST be
re-derived when P-1 is ratified. It is not yet implemented.

### P-007 — `cj3-types`

Canonical encodings, domain tags, the single `addr()`, amount newtypes, codec fuzz
targets wired into CI.

**Outcome:** COMPLETE (pass 1.1). Al reviewed and merged PR #21 at
`2026-11-23T05:36:52Z` as `333c67086012d03f3c654cba20526e5c0f5e6b4d`. Its exact head
`92f9be92265c83c5f0b3e27acd3311027d910c56` passed all eleven D6 jobs in run
`45114968343`, and exact-merge-SHA run `45118879871` passed all eleven. The history of
the packet follows as it was recorded.

**The first part, PR #14.** PR #14's exact head
`9acea83ca17d67a19e0d41aeb5e7275666a54013` passed all eleven jobs in run
`31969740766`. Al merged it on 2026-10-12 as
`358114048c3a38f4821b88fce8dffd14abbd42a1`, and D6 run `38323998550` on that commit
concluded success. The merge stands (ruling Part 0.4). It carried only the
independently specified checked `Amount(u64)` boundary and the P-007 stop report. At
that merge the codec-fuzz job reported `NOT_YET_ADMITTED`, and its green was not a
fuzz-success claim; the resumed build below makes it live.

**What the stop needed, and where it is now ruled:** the canonical codec grammar
(G0-C1); SHA-256 confirmation and the bytes of all seven domain tags, with their
preimages (G0-C2); the layouts (G0-C3); the SI-002 signed-coefficient representation
and guard (G0-C3e, G0-C4); the SI-003 SHAKE consumption (G0-C5); the P-8 byte limits
(G0-C6); and GAP-12 strict Ed25519 behavior (G0-C7). The applied text is Protocol Spec
§1.1–§1.3, §5.2, §6.1, §6.3 and §7, v0.2-DRAFT when P-007 was built and v0.2, normative,
since the ruling of 2026-11-23 (pass 3.4).

**Resume (ruling F2):** on a new branch, in the HUMAN lane, not merged by the builder.
Al was to rule G0-F after P-007 was merged with exact-merge-SHA D6 green (F1), and has.
P-007 carries `tx_root` and `state_root` as 32 opaque bytes (F3).

**Resumed build (2026-11-22):** PR #21 from `feat/p007-encoding-v1`, under the P-007
resume ruling. The builder report is `loop/reports/C10-p007-builder.md`.

- `cj3-types` owns bytes: the grammar and typed decode errors, the seven domain tags,
  `H` and the six preimages, the single `addr()`, the transaction, header and block
  layouts, the class identifier table, the 20-byte SIS size-parameter layout and the
  §1.3 verification. `cj3-classes` owns meaning: it imports the layouts, binds SIS to
  class identifier 1, holds the tuple rules and the SIS solution encoding (resume 4.2,
  4.3).
- The evidence quoted below exists: (a) and (b) and (c) under `bench/p007-vectors/`,
  each derived by a standard-library script and by the Rust code; (d) as six cargo-fuzz
  targets under `fuzz/`, live in the D6 `codec-fuzz-smoke` job; (e) by construction, the
  codecs take the tuple as an argument and the stand-in tuples are `(8, 112, 67, 112)`
  and `(16, 288, 257, 288)`, both measured in `bench/p003-sis` and labeled
  non-normative.
- What was left for Al at review, and how he ruled it (pass Part 2):
  - `sha2 =0.10.9` and `curve25519-dalek =4.1.3` are accepted as pinned, the second on
    the condition of pass 4.5c: one arithmetic back end. Arithmetic written in
    `cj3-types` is refused (pass 2.1).
  - `libfuzzer-sys` is accepted for the fuzz crate alone, as test tooling outside the
    graph the node is built from, on the condition of pass 4.5b: `cargo deny` over the
    fuzz workspace with a policy of its own (pass 2.2).
  - The nightly long fuzz job may use the same pinned nightly toolchain and the same
    targets, and is made live (pass 2.3, 4.5a).
  - Three files are NORMATIVE conformance vectors from the merge of PR #21:
    `bench/p007-vectors/encoding-v1.vectors.txt`,
    `bench/p007-vectors/shake-expansion.vectors.txt` and
    `bench/p007-vectors/ed25519/c7-expected.vectors.txt`, the thirteen builder-authored
    vectors included. What is normative is the relation: these inputs give these bytes
    and these verdicts, and every implementation reproduces them. The input values
    assign nothing. If the text of the specification and a vector ever disagree, that
    is a STOP and a ruling; neither wins by default. The P-005 vectors stay symbolic
    and non-normative (pass 2.4).
  - The block codec's use of the active tuple for `SOL_MAX_BYTES` alone stands, and B3
    is a block rule that a successful decode never implies (pass 2.5). The order of
    checks of report C10 §1.5 stands (pass 2.6). The layout tests over arbitrary bytes
    stay (pass 2.7). The edits no clause named stand (pass 2.8). The `cj3-classes` API
    is accepted (pass 2.10), and so are the three ways of showing the `BLOCK_MAX_BYTES`
    boundary (pass 2.11).
  - The two stale statements report C10 left are corrected in the text-only commit of
    the Gate G0 closeout (pass 2.9, 4.4).
- The conditions of pass 4.5 are built in the Gate G0 closeout, PR #22, each in a commit
  of its own. `loop/reports/C11-g0f-gate-builder.md` is its report.

**Evidence P-007 owes for Part 2 (G0-C8, quoted from the ruling):**

> a. For every layout in C3 and every preimage in C2c, at least one worked vector with
> all of its bytes shown, derived twice independently: by the Rust code, and by a script
> that uses only a standard library, as bench/p003-sis/SHAKE-VECTOR.md did.
>
> b. For C5: the existing q = 17 fixture, now normative; one vector with a prime modulus
> above 2^31, where candidates are rejected; and one at each tuple used as a fixture.
>
> c. For C7: the RFC 8032 section 7.1 test vectors, and the C2SP CCTV ed25519 edge-case
> vectors pinned by commit, with the expected result for each derived from C7 clause by
> clause and recorded. Do not take expected results from the library under test.
>
> d. Fuzz targets, live in D6: no input makes a decoder panic; decode then encode
> returns the input; encode then decode returns the value; each length limit holds at
> the boundary, one below it and one above it.
>
> e. Until P-4 is ruled, no tuple is compiled in as active. Codecs take the tuple as an
> argument. Fixtures use tuples already measured in bench/p003-sis and are labeled
> non-normative.

### P-008 — Frontend contract extraction

Read-only, external: survey Sarah's public frontend repository at
`TBD(Al-supplied frontend URL)`; extract routes, payload shapes, event/stream
expectations, and auth assumptions into `docs/FRONTEND_SEAM.md`. No forks, no PRs, no
issues, no watches, and no action that generates a notification on Sarah's side. This
packet informs the Phase 4 seam; Protocol Spec §12's read-API surface MUST NOT
contradict it.

The legacy repository
`https://github.com/COINjecture-Network/COINjecture2.0` is explicitly **not** treated as
the missing frontend URL. Agents MUST NOT guess the frontend URL (A11).

### P-009 — Audit traceability verification

Read-only verification against the source audit documents: enumerate every finding
ID/class, diff it against `docs/AUDIT_TRACEABILITY.md`, report every unmapped or
mis-mapped item, and propose matrix v0.2. The third-party security audit and optional
Lean audit are first copied byte-for-byte from the supplied Downloads paths into
`loop/evidence/`; the Lean audit is checked specifically for anything bearing on
CJ3's A8 track. The Codex security-scan source remains outstanding, but P-009 does
not wait if exact committed 2.0 remediation records fully represent its findings and
are cited as substitutes. No `src/` change is authorized.

**Outcome:** COMPLETE through PR #15 at
`ce0b9752c9101e89f93a90379cd9e5ac1a08842d`; refreshed exact-head run `31971741833`
and exact-merge-SHA run `31972017916` passed all eleven jobs. Matrix v0.2 accounts for
33/33 third-party findings (17 mapped, 10 partial, six surface-excluded), all 25 Lean
claim-points, and R1–R8. GAP-7 through GAP-13 are proposals only. The absent original
Codex report's exact IDs/titles/severities/count remain UNKNOWN; exact committed CJ2
remediation pointers preserve the known five program families and two finding IDs
without pretending to replace the missing source inventory.

### P-010 — Institutional README

Replace the repository-root `README.md` with a GitHub-flavored, diligence-oriented
entry document below roughly 400 lines. It leads with the 2.0 audit lineage, derives
the structural response without claiming unimplemented consensus behavior, keeps one
current status block, condenses A1–A11, maps the planned crate architecture, explains
governance and evidence inspection, provides only currently working commands, states
limitations and owned TBDs, and marks the current G0–G4 position. A D6 badge is the
only permitted badge class; no drifting version badge is permitted.

The packet is docs-only and AUTO eligible. It may edit the README and its own durable
governance/report/bookkeeping surfaces, but no source, workflow, formal specification,
protocol vector, consensus semantic, parameter, or license decision. Its adversary
pass removes or points every unsupported capability sentence. The README status block
and roadmap marker become mandatory refresh surfaces at every gate closeout under the
standing rule in `loop/LEDGER.md`.

### P-011 — SIS solver envelope

Commissioned by G0-B4 as the cadence link, under the next free packet identifier.
Unblocked by pass 3.6 and not started. P-101 runs first (launch 3.1); Al says when this
packet runs. Lane AUTO if the five D17 conditions hold at pickup. It measures and
reports. It proposes no value for P-1, P-4, P-10 or `ACTIVE_SIZE_PARAM`.

**It judges nothing (launch 2.10).** Packet P-011 measures and reports costs. What is
worth calling useful work is Al's judgment when he rules P-1 and P-4.

**D1 stands on what this packet finds (pass 3.3).** SIS remains the genesis anchor
class. No tuple is ratified, and it is not yet measured whether the family has a size
that miners can solve at a cost worth calling useful work. This packet measures that. If
it finds no such size, D1 returns to Al.

Its terms, quoted from the 2026-11-22 ruling:

> a. For family points from n = 16 upward, in steps the report justifies, until two
> consecutive points fail: instances come from derive_instance on distinct recorded
> seeds; a reference solver using lattice reduction stronger than LLL runs out of
> process and untrusted; every output is re-checked by check_sis; a run that reaches the
> limit is recorded as censored and never dropped. The limit is 30 minutes per run. A
> point fails when fewer than half of its runs succeed inside the limit.
>
> b. At least 100 seeds per point while the median solve is under 60 seconds, and at
> least 20 beyond that. Report only the quantiles a sample supports.
>
> c. Reported per point: success rate; solve-time quantiles; the quality Q of each
> solution; peak memory; the estimator's block size and cost beside the measured cost,
> with any point where measurement beats the model by more than a factor of ten called
> out; and derive_instance plus check time on the same host (B2d).
>
> d. D8 keeps other languages out unless a decision admits them. This clause is that
> decision, for this use only: an external lattice-reduction tool may be run in a
> container pinned by digest for these measurements, as the estimator was. Nothing of
> the tool is linked into, or vendored beside, any cj3-* crate.
>
> e. I rule P-1, P-4, P-10, ACTIVE_SIZE_PARAM and the re-derived P-2 together, from that
> report. The packet blocks Phase 2. It does not block P-007 or P-101.

The family is the candidate family of G0-B3c: `q(n)` = first prime above `n^2`,
`m(n) = 2 n ceil(log2 q(n))`, `beta_squared(n) = m(n)`. It is not ratified.

The packet also carries the P-3 measurement of G0-B2d: reference host v0 is the machine
in `bench/p003-sis/ENVIRONMENT.md`, and the `derive_instance` plus `check` measurement
is repeated with derivation included, on that host and on one other class of machine.
Al re-rules the budget and the hardware definition from that. Pass 3.5 lists it among
what the report carries, as "the re-measured P-3".

### P-101 — Kernel

Phase 1 head, unblocked by Gate G0 PASS (pass 3.1, 3.6). It runs first (launch 3.1).
Its FRAME is written, in `loop/reports/C12-p101-builder.md`, and stops for Al's ruling;
its build has not begun and no kernel code exists (launch 3.1, 4.7). V1–V9 + STF
versus Lean vectors. Its concrete
authenticated store MUST discharge all three
`LawfulStateOps` laws from Protocol Spec §8 and prove that the kernel's actual storage
reads and writes are coherent with them. Its `Context.rewardInputs` wiring MUST
consume only the Q returned by
`check(derive_instance(instance_seed, size_param), solution)` for the validated block
and MUST expose no block-supplied quality path; this is the binding C2 structural
boundary and is re-checked at Gate G2. The concrete `R_MAX·SCALE` divisor MUST be
constructed with checked u128 multiplication, proved nonzero before floor division,
and the result checked back to u64. P-101 proposes how `tx_root` and the state
commitment are built in its FRAME and stops there for Al's ruling (pass 3.5).

**The FRAME (launch Part 3).** It puts twelve decisions to Al, K-1 to K-12, and lists
twenty that are already settled with the place that settles each. It proposes `tx_root`
as a list tree in the shape of RFC 6962 with the count in the root, and the state
commitment as a binary radix tree over the address bits that records each split, with
the count in the root and no leaf for an account of balance 0 and nonce 0. It proposes
six domain tags by name and bytes; Al assigns tags, and the FRAME does not (launch
3.3b). Its worked examples are under `bench/p101-roots`, derived by a standard-library
script, and are proposed, not ruled: they are not conformance vectors. None of this is
ruled.

The P-005 vectors stay symbolic and non-normative (pass 2.4).

## Carried to later packets and gates (pass 3.5; G0 ruling Part 4)

Gate G0 is PASS with these carried, each to the boundary where Al rules it. Protocol
Spec v0.2 is normative except for every value marked UNFILLED or PROVISIONAL and every
item of pass 3.5, and no implementation may assume a value for one (pass 3.4).

- From packet P-011's report (pass 3.5): P-1, P-4, P-10, `ACTIVE_SIZE_PARAM`, the
  re-derived P-2, the re-measured P-3, the re-ruled `BLOCK_MAX_BYTES` and the timestamp
  unit. They block Phase 2.
- How `tx_root` and the state commitment are built (pass 3.5): P-101 proposes both in
  its FRAME and stops there for Al's ruling. The proposal is made
  (`loop/reports/C12-p101-builder.md`, sections 4 and 5) and is not ruled.
- At the first Phase 2 FRAME (pass 3.5): the byte order in which a 32-byte value is
  read as a number (B8, and the weight formula and tie-break of Protocol Spec §10).
  G0-C1a does not decide it.
- With the Phase 1 genesis packet (pass 3.5): the genesis layout and its preimage.
- On the packet that introduces the mempool: GAP-9, tested at Gate G3 (G0-E3).
- On Phase 3: GAP-7, GAP-8, GAP-10, GAP-11 and GAP-13, each tested at Gate G3 (G0-E2).
  For these and for GAP-9, the builder proposes each operating default with its
  reasoning in the packet report, and Al reviews them at G3 (G0-E4).
- With their owners (pass 3.5): P-7, P-9 and P-12. In the ruling's words, "D10 and D16
  are still not ruled"; every value and seat they reserve stays reserved and UNFILLED,
  and every prohibition on contacting or notifying anyone stays in force (Part 0.2 of
  the 2026-11-22 ruling). The meaning of the quoted sentence, as Al gives it in clause
  2.1 of his ruling "LIVE CJ3 RULING - P-101 TO ITS FRAME" of 2026-11-23 (cited as
  "launch 2.1"): D10 and D16 stand as ruled on 2026-08-15, and neither the 2026-11-22
  ruling nor the gate ruling revisited them.
- P-008 (pass 3.2, 3.5): blocked on the URL, and carried.

## Pickup guard

Every pickup MUST re-read `loop/STATE.md`, check this repository's `CAPACITY_FLAG`, and
re-check the live D11 condition. The current condition is Part 0.3 of Al's 2026-11-22
ruling: no COINjecture 2.0 remediation work is queued or running, and CJ3 may proceed.
It holds until Al states another. `remediation-priority` in either project pauses CJ3
immediately (D11).
