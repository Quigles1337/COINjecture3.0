# C9 — Gate G0 ruling record (G0-B, G0-C, G0-D, G0-E) — Builder Report

Al's live ruling of 2026-11-22 is recorded verbatim in
[`loop/LEDGER.md`](../LEDGER.md#gate-g0-ruling-overlay--g0-b-to-g0-e-ruled--g0-f-held-al-2026-11-22).
This report cites a clause of that ruling as `G0-` plus its label (`G0-B3`, `G0-C7e`),
and Parts 0, 5 and 6 as "Part 0.1", "F2" and "Part 6 step 3". A bare B1–B12 is a
block rule of Protocol Spec §6.2, and a bare D1–D17 is a ledger decision.

## 1. FRAME — written before any edit except the verbatim LEDGER overlay

### Task and done-condition

Part 6 of the ruling is this session's whole scope. The session is done when:

1. the preflight has passed as Parts 0.1 and 0.3 amend it;
2. the ruling is in `loop/LEDGER.md` word for word, as the first edit
   (commit `7937083`);
3. one documentation-only pull request applies Parts 0 to 5 to the eight document
   surfaces Part 6 step 3 names;
4. nothing under `crates/`, `spec/`, `bench/`, `scripts/` or `.github/` has changed;
5. the adversary pass has compared the diff with the ruling clause by clause, with
   A1–A11 and with the Lean files, and this report lists every place where an amended
   sentence says more or less than its clause;
6. the pull request is open, its exact head is green in D6, it is marked ready, and
   it is left unmerged for Al.

No protocol code is written. No packet is picked up.

### Clause check before acting

The ruling orders a STOP if a clause is ambiguous or conflicts with an axiom, the Lean
files or another clause. Before the first edit I read every clause against A1–A11,
`spec/Spec/Tx.lean`, `spec/Spec/Stf.lean`, `spec/Spec/Vectors.lean`,
`crates/cj3-classes/src/sis.rs` and the P-003 evidence files. I found no clause that
needs a STOP. Section 4 lists what I checked and the points I am handing to Al
instead of deciding.

### Lane

**HUMAN ruling, supplied; documentation record; no builder merge.** Decision
ratification is HUMAN under D17 and Al has performed it. This session applies his text
and has no authority to extend it. Part 6 step 6 forbids the builder to merge, so the
five AUTO conditions used by earlier packet reports (approved and unblocked; bounded
non-semantic surface; no formal-spec content; no decision invention; no owned-TBD
fill) are recorded as controls on the diff, not as a merge licence.

### Predicted diff surface

- `loop/LEDGER.md` — the verbatim overlay, then a short index of what it supersedes.
- `docs/PROTOCOL_SPEC.md` — v0.2-DRAFT: §1 and new §1.1–§1.3 (encoding grammar, tags
  and preimages, Ed25519 verification), §2, §3, §4 (one sentence), §5.1, §5.2, §6.1,
  §6.2 (one note), new §6.3, §7, §10 (one line), §12, §13 (one note), §15.
- `loop/reports/SPEC-ISSUES.md` — status and resolution of SI-001, SI-002, SI-003.
- `docs/AUDIT_TRACEABILITY.md` — §5 only.
- `docs/ENGINEERING_PLAN.md` — one line under the D1 ADR and the Gate G0 line in §4.
- `README.md` — the status block, the limitation G0-B3d orders, every line that calls
  PR #14 draft or unmerged, and the lines this ruling makes false.
- `loop/PACKETS.md`, `loop/STATE.md` — brought to the truth.
- `loop/reports/BATCH-LOG.md` — two new lines.
- `loop/PROMPTS/AUTONOMOUS_BUILDER.md` — step (d).
- this report.

Not predicted: any file under `crates/`, `spec/`, `bench/`, `scripts/` or `.github/`;
any earlier report; any earlier BATCH-LOG line; any earlier ledger entry.

### Top risks

1. **Paraphrase drift.** A spec sentence that restates a clause in my words can say
   more or less than the clause. Control: carry the clause's own sentences into the
   spec and change only cross-references; check tag bytes, lengths and constants by
   script; compare clause by clause in the adversary pass.
2. **Label collision.** The ruling's clauses B1–B4 and D1–D3 share names with block
   rules B1–B12 and decisions D1–D17. Control: every citation outside the verbatim
   block carries the `G0-` prefix.
3. **Stale statements left standing.** Step 4 and the "step (d) only" limit keep me
   out of files that will now contradict the record (the Lean header comments, the
   bench fixture note, the bootstrap paragraph of the prompt). Control: change none of
   them and list each one in section 4.

**Falsifier:** this approach is wrong if any amended sentence cannot be traced to a
clause, if the diff touches a forbidden path, or if a number in the applied text
differs from the ruling's.

**Confidence: MED-HIGH** that the record can be applied without choosing a reading;
MED that the adversary pass finds no place where my wording drifted.

## Preflight evidence (Part 6 step 1; prompt STEP 0 as Parts 0.1 and 0.3 amend it)

| Step | Result | Evidence |
|---|---|---|
| a. Path | PASS | `pwd -W` returned `C:/Users/LEET/COINjecture3.0`; no `OneDrive` segment |
| b. Remote | PASS | `git remote -v`: fetch and push are both `https://github.com/Quigles1337/COINjecture3.0` |
| c. Branch and tree | PASS | on `main`, `git status --porcelain` empty, `HEAD` = `origin/main` = `git ls-remote origin main` = `358114048c3a38f4821b88fce8dffd14abbd42a1` after `git fetch` |
| d. Visibility (Part 0.1) | PASS | `gh repo view Quigles1337/COINjecture3.0 --json visibility` returned `PUBLIC` |
| e. Authorization | PASS | `loop/LEDGER.md` holds D17 RATIFIED (2026-08-15 overlay) |
| f. Capacity (Part 0.3) | PASS | the launch ruling states that no COINjecture 2.0 remediation work is queued or running; a read of `C:\Users\LEET\COINjecture2.0-network\loop\STATE.md` showed `CAPACITY_FLAG: none`, not `remediation-priority` |
| g. Survey and P-008 | PASS | `docs/RESEARCH_SURVEY.md` present; the P-008 URL is still `TBD(Al-supplied frontend URL)`, so P-008 stays blocked |

Part 0.4 was checked against GitHub before it was written anywhere:
`gh pr view 14` returned state `MERGED`, merged at `2026-10-12T23:47:49Z`, merge commit
`358114048c3a38f4821b88fce8dffd14abbd42a1`, head `9acea83ca17d67a19e0d41aeb5e7275666a54013`;
`gh run view 38323998550` returned workflow `D6 CI`, event `push`, branch `main`, that
same commit, conclusion `success`
(<https://github.com/Quigles1337/COINjecture3.0/actions/runs/38323998550>).

## 2. BUILD

The first edit of the session was the verbatim overlay (commit `7937083`, LEDGER only).
The FRAME above was committed next (`efa68e3`). The applying change followed:

- `docs/PROTOCOL_SPEC.md` is v0.2-DRAFT. New §1.1 (grammar), §1.2 (hash, tags,
  preimages), §1.3 (Ed25519 verification) and §6.3 (block layout). Amended §1, §2, §3,
  §4, §5.1, §5.2, §6.1, §6.2, §7, §10, §12, §13 and the §15 index. No section was
  renumbered and no existing heading was renamed, so existing links still resolve.
- `loop/reports/SPEC-ISSUES.md`: SI-001, SI-002 and SI-003 are RESOLVED, each with a
  link to the overlay. The original entries stay below each resolution.
- `docs/AUDIT_TRACEABILITY.md` §5: a status follows each gap's unchanged wording.
- `docs/ENGINEERING_PLAN.md`: one pointer line under the D1 ADR; the Gate G0 line.
- `README.md`: status block, the G0-B3d limitation, the PR #14 lines, and the rows and
  markers the ruling made false (listed in 4.1).
- `loop/PACKETS.md`: queue status, order of work, P-007 ready to resume with G0-C8
  quoted, new packet P-011 with G0-B4 quoted, carried items, pickup guard on Part 0.3.
- `loop/STATE.md`: header, the stale bullets, the ruling's state, next actions.
- `loop/reports/BATCH-LOG.md`: two new rows; no earlier row changed.
- `loop/PROMPTS/AUTONOMOUS_BUILDER.md`: step (d).
- `loop/LEDGER.md`: a short index after the verbatim block of what the ruling
  supersedes, what it leaves unruled and where it is applied.

Tripwires: SCOPE held (every touched file and spec section was predicted). INVENTION
held: every value this change writes was supplied by Al (15 ms provisional, 188,
1,048,576 provisional, class identifier 1), and the packet identifier P-011 follows
clause B4's own rule. REPETITION, INTERPRETATION, ENVIRONMENT and DEGRADATION did not
trip.

## 3. VERIFY

Local checks, each run by script against the ruling text as recorded in the LEDGER:

- The seven tag byte strings equal their ASCII text padded on the right with `0x00` to
  16 bytes, and the spec's tag table is identical to the ruling's.
- The six preimage definitions and lengths in the spec are identical to the ruling's.
  The lengths add up: 48, 108, 322, 92, 114, 48; transaction 188; size parameter 20;
  header 306.
- The field order of the spec's Header block equals clause C3d's order.
- The "Hardness basis" bullet equals clause B1's text word for word (139 words).
- `32767^2 = 1,073,676,289`, and the candidate family at n = 128 gives
  `(128, 3840, 16411, 3840)`.
- GAP-7, GAP-8, GAP-10, GAP-11 and GAP-13 read the same in Protocol Spec §12 as in
  `docs/AUDIT_TRACEABILITY.md` §5, and every gap wording in §5 is unchanged from `main`.
- Of the 152 sentences in Parts 0 to 5 of the ruling, 101 appear in the applied
  documents word for word. The other 51 are accounted for in 4.1: they are
  instructions to the builder, headings, first-person sentences turned to "Al", clause
  references turned into section references, or lists turned into tables.
- `git diff --check` is clean. `git diff --name-only main` lists only the ten
  predicted documents and this report; nothing under `crates/`, `spec/`, `bench/`,
  `scripts/` or `.github/`.

The ruling's cited evidence was read in the repository before it was repeated:

- `bench/p003-sis/estimator-results.jsonl`, the n = 128 row: `rough_log2_rop` 143.956,
  `default_log2_rop` 165.52035644596074, block size 493 in both models.
- `bench/p003-sis/solver-results.jsonl`: three n = 16 runs `solved-and-rechecked`; one
  n = 24 run, `completed-no-candidate`.
- `bench/p003-sis/verifier-results.jsonl`, the n = 128 row: p95 5,589,000 ns and
  maximum 7,817,500 ns, which give 15 ms / 5.589 ms = 2.68 and 15 ms / 7.8175 ms = 1.92.
  The bench times `check` on a pre-built matrix.
- `loop/reports/C3-p003-builder.md` line 273: "this is not a final cadence-compatible
  P-4".
- `crates/cj3-classes/src/sis.rs` lines 248–265 and 339–343 do what clause C5
  describes: SHAKE-256 over the seed alone, four-byte little-endian groups, acceptance
  below `floor(2^32 / q) * q`, the value modulo q, a rejected group consumed whole,
  column-major fill. `SisParameters::new` accepts any prime `u32`, so the vector with a
  modulus above 2^31 that clause C8b asks for is constructible.

Origin D6: the exact-head run for this branch is recorded on
[PR #20](https://github.com/Quigles1337/COINjecture3.0/pull/20). This file cannot
hold the run of the commit that contains it, so it makes no green claim.

## 4. ADVERSARY PASS

### 4.1 Clause by clause: where the applied text says more or less

Every sentence of a clause that is not listed here is in the applied documents word
for word. Throughout, "I" became "Al", `||` became `‖`, and code names gained
backticks.

| Clause | Applied in | Difference |
|---|---|---|
| Part 0.1 | prompt step (d) | **More:** the step now gives its reason ("public by Al's decision", with the LEDGER pointer). The RE-VERIFY sentence is unchanged. **Left standing:** the BOOTSTRAP paragraph still says the first push "is only legal because (d) verified private"; the ruling allows no other change in that file. |
| Part 0.2 | STATE, LEDGER index | None. |
| Part 0.3 | STATE, PACKETS pickup guard | **More:** both keep D11's rule that `remediation-priority` in either project pauses CJ3. The `CAPACITY_FLAG` value `cj2-no-remediation-queued-or-running` is a label I chose for the condition. |
| Part 0.4 | STATE, PACKETS, README, BATCH-LOG | Every line that called PR #14 draft or unmerged is corrected. Left as written: earlier BATCH-LOG rows, earlier reports, and the verbatim G0-A overlays in the LEDGER. |
| B1 | Spec §5.2; Plan D1 | Spec: none (139 words identical; the label is bold). Plan: one pointer line; it says the D1 wording "is resolved by ruling G0-B1". |
| B2 | Spec §3, §5.1 | "(D4)" reads "(plan D4)"; "the packet B4 commissions" reads "packet P-011". **More:** §5.1 gained one sentence restating B2a and B2b, and the registry's admission condition "checker cost ≤ P-3" now reads "`derive_instance` plus `check` cost ≤ P-3". No clause orders the registry edit; I made it so the admission bar agrees with B2a. Strike it if the bar should keep its old words. **Left standing:** the trait comment "Bounded by VALIDATION_BUDGET (P-3)" on `check`, and Plan D4's "a class whose checker exceeds the per-block validation budget". |
| B3 | Spec §3; README | Spec: a–d carried; the README instruction in B3d is not spec text. README: one limitation that compresses B3a; it drops the model names and the block size 493 and keeps both exponents, the n = 16 and n = 24 results and "would never produce a block". |
| B4 | PACKETS (quoted); Spec §3 | PACKETS quotes a–e. The identifier is P-011. The spec carries only the head of B4 and B4e and points to PACKETS for a–d. |
| C1 | Spec §1, §1.1 | "the order C3 gives" reads "the order its layout gives (§5.2, §6.1, §6.3, §7)"; "the exact length C3 gives" reads "§6.3 gives". **More:** §1's summary bullet restates the grammar in one line. |
| C2 | Spec §1.2, §2, §4 | Lists became tables. "C1" reads "§1.1" and "C3e" reads "§5.2". §2's formula changed from `H(D_ADDR ‖ pk)[0..32]` to `H(D_ADDR ‖ pk[32])`. §4 gained one sentence naming the beacon input. |
| C3 | Spec §5.1, §5.2, §6.1, §6.3, §7 | C3d's field list is the existing Header block. **More:** "for SIS that is the 20 bytes of §5.2", which follows from C3c. §6.3 is new; v0.1's Header block already pointed to a §6.3 that did not exist, and that pointer now resolves. |
| C4 | Spec §5.2 | "A coefficient is encoded as C3e says" is carried by the Solution bullet. |
| C5 | Spec §5.2 | None. **More:** one sentence from C8b (the q = 17 fixture is now normative) and a note that the bench file still calls it nonnormative. |
| C6 | Spec §3, §7 | None. |
| C7 | Spec §1.3 | "C2c" reads "§1.2". |
| C8 | PACKETS (quoted) | **Less in the spec:** only C8b's fixture status reached the spec. C8 is evidence a packet owes, so the rest lives with P-007 in PACKETS. |
| D1–D3 | Spec §3 | "TIMESTAMP_DRIFT" is the spec's `TIMESTAMP_DRIFT_Δ`. |
| E1, E2 | Spec §1.3, §12; audit §5 | The five gaps read the same in §12 as in audit §5, "Covers …" tails included. |
| E3 | Spec §12; audit §5; PACKETS | **Placement is mine:** GAP-9 sits in §12 (Networking and RPC) although it binds the packet that introduces the mempool. No clause says where it goes. Its §5 wording is quoted beside it. |
| E4 | Spec §12; audit §5; PACKETS | "these" reads "these requirements". |
| E5 | audit §5 | None. |
| F1–F3 | Spec header, §6.1, §6.2, §10, §13, §15; PACKETS; STATE | "P-007 carries both as 32 opaque bytes" reads, in the spec, "`tx_root` and `state_root` are carried as 32 opaque bytes"; PACKETS names P-007. |

Edits no clause names, made because the ruling made a statement false:

- **README**, outside the status block and the B3d limitation: the roadmap marker for
  G0 (the standing drift-control rule requires it); the A10 row's link text; the Lean
  limitation ("pending SI-001/SI-002/SI-003" became a statement that the ruling changed
  no Lean file or vector); and the unfilled-values rows for P-3, P-4, P-8, P-10, the
  codec row and the audit-matrix row.
- **Audit matrix §5**: the heading no longer ends "— not ratified"; an opening
  paragraph says a ratified requirement is not an implemented control (GAP-1); a
  closing paragraph says §1 and §2 are as P-009 wrote them.
- **STATE and PACKETS**: the P-005 and P-009 bullets no longer say the SIs are pending
  or the gaps are proposals; the P-010 bullet says the README recorded private scope
  at its merge.
- **LEDGER index**: my words, not Al's. It says so, and says the verbatim text wins.

### 4.2 Points for Al — noticed, recorded verbatim, not decided

None of these changed what this session wrote. Each one will matter at a later FRAME.

1. **C2d with C3d.** "Under one tag every preimage has the same length" holds because
   SIS is the only registered class. A second class whose `size_param` layout is not
   20 bytes would give `D_HDR` preimages of a second length.
2. **C1d and C1e, the transaction count.** No limit is named for it. Its bound follows
   from `BLOCK_MAX_BYTES` and from the bytes that remain. The ruling does not say which
   typed error an oversized count gets: "count exceeds its limit" or "shorter than
   required". Both reject, so consensus does not depend on it. P-007's FRAME has to
   pick one.
3. **C8b with C8e.** C8b makes the q = 17 fixture normative and asks for a vector with
   a prime modulus above 2^31. C8e says fixtures use tuples already measured in
   `bench/p003-sis` and are labeled non-normative. The q = 17 fixture's tuple
   `(2, 3, 17, 3)` and any tuple with such a modulus are not among the measured
   tuples. I did not stop on this, because C8b's own third item separates a "vector"
   from "a tuple used as a fixture", and nothing written this session depends on it.
   P-007 needs Al's line on whether C8e's limit and label reach the C8b vectors.
4. **B4, "the five D17 conditions".** The prompt names them; neither it nor the LEDGER
   lists them. Reports C1, C5, C6 and C7-p006 use one list (approved and unblocked;
   bounded non-semantic surface; no formal-spec content; no decision invention; no
   owned-TBD fill). The G0 handoff and C8 used other wordings. Packet P-011's pickup
   will classify against the first list unless Al fixes one.
5. **Packet P-011 beside parameter P-11.** The next free identifier is one character
   away from the struck parameter. The documents always write "packet P-011".
6. **V7 under encoding v1.** `to` is a fixed 32 bytes, so no wire input can fail V7;
   a short or long transaction fails V1. The Lean vector
   `v7-destination-31-bytes-reject` already supplies its candidate through the V1
   interface "for rule isolation", so nothing conflicts. P-101's conformance test has
   to inject that case at the candidate level.
7. **The P-005 vectors.** The LEDGER called them non-normative "pending
   SI-001/002/003". The ruling resolves the three issues and does not give the vectors
   a new status. The documents now say they remain symbolic and non-normative and
   were not changed.
8. **Part 6 step 2.** I put the overlay on this branch as its first commit and pushed
   nothing to `main`. The plan's contract forbids direct pushes, and step 6 keeps the
   merge with Al.

### 4.3 Stale statements left standing because the ruling keeps me out of the file

- `spec/Spec/Tx.lean`, header comment: "SI-001, SI-002, and SI-003 remain OPEN".
  `spec/README.md`: "SI-001, SI-002, and SI-003 remain unresolved" and the vectors are
  non-normative "pending SI-001/SI-002/SI-003". These are HUMAN-lane files.
- `bench/p003-sis/SHAKE-VECTOR.md`: calls the q = 17 fixture nonnormative and SI-003
  undecided.
- `crates/cj3-classes/src/lib.rs`: "its still-unratified numeric registry identifier".
- `loop/PROMPTS/AUTONOMOUS_BUILDER.md`, BOOTSTRAP paragraph: "(d) verified private".
- `docs/ENGINEERING_PLAN.md`: the D2 ADR's G0-A paragraph ("G0-B through G0-E await
  normative text"), the header line "Repo home: unratified (see D9)", and D4's
  "checker".
- `docs/AUDIT_TRACEABILITY.md` §1 rows H1, H2, H3, H5, H10, M7, M8, M9, M10, L2 and
  L5, and the §2 A8 conclusion, still call the gaps proposed or the SIs unresolved.

Older README drift that this ruling did not cause and I did not touch: the
repository-map row that says `spec/` defines no formal rule or vector; the sentence
that lists Lean conformance among the deferred phase jobs although
`scripts/ci/active/lean-conformance.ps1` exists; and two links to
`docs/AUDIT_TRACEABILITY.md#1-finding-class-matrix`, an anchor that heading does not
produce.

### 4.4 A1–A11, banned identifiers, unsafe, TBD integrity

| Axiom | What I looked for | Result |
|---|---|---|
| A1 | A self-reported field entering a layout | None. The header and transaction layouts carry exactly v0.1's fields. |
| A2 | A miner-chosen input to the instance | None. The `instance_seed` preimage is `network_id`, `height`, `parent_hash`, `beacon_out`; `timestamp` is not in it. |
| A3 | A clock reaching the checker | None. The spec now says P-3 is an admission ceiling and never a run-time check. |
| A4 | Quality entering §10 | None. The P-4 note states that chain weight comes from the hash race alone. |
| A5 | A float or a signed amount | None. The only signed integers are SIS coefficients. |
| A6 | A weakened V-rule | None. The V-rule text is unchanged; V1 and V2 gained their concrete values. |
| A7 | A panic path or fail-open rule | None. §1.1(e) requires typed errors and no panic. |
| A8 | A change to the Lean source of truth | None. See 4.5. |
| A9 | Foreign code on a trusted path | None. G0-B4d's container is recorded as one use, nothing linked or vendored. |
| A10 | A hardness or capability claim without its label | None found. The B1 text, the provisional labels, "no evidence stands behind this number" and "not implemented" are all carried. |
| A11 | A verification claim without a pointer | None found. PR #14's merge and run, the bench figures and the branch-protection read each carry a pointer. This change claims no D6 result for itself. |

The diff adds none of `solve_time`, `work_score`, `reported_*` or `self_reported`
outside this sentence. No crate changed, so the `unsafe` count is unchanged. No
builder-chosen value fills a TBD: P-1, P-4, P-7, P-9, P-10, P-12 and
`ACTIVE_SIZE_PARAM` are still unfilled.

### 4.5 The Lean files

I read `spec/Spec/Tx.lean`, `spec/Spec/Stf.lean` and `spec/Spec/Vectors.lean` against
Parts 1 to 4.

- `Candidate`'s fields are in clause C3a's order: network identifier, sender,
  recipient, amount, fee, nonce, public key, signature.
- `ValidationContext` keeps `txMaxBytes`, `strictCanonicalDecode`,
  `signatureVerifies` and `addressOf` abstract. The ruling gives each a concrete
  definition and contradicts none of the rules built on them. V1 tests
  `input.size ≤ txMaxBytes` and then decodes; with `TX_MAX_BYTES = 188` and exact
  decode, the two agree, as C6a says.
- The vector cases `v1-size-at-limit-pass`, `v1-size-over-limit-reject` and
  `v1-unknown-field-reject` keep their expected results under encoding v1.
- `Stf.lean` models the block as transactions, miner address and expected state root.
  Nothing in the ruling touches the reward function or the conservation theorems.

I found no conflict between a clause and a Lean definition or theorem. The stale
header comments are listed in 4.3.

## 5. HANDOFF — no MERGE

[PR #20](https://github.com/Quigles1337/COINjecture3.0/pull/20) carries this change.
Its description holds the exact-head D6 run. It is marked ready and left unmerged.
Part 6 step 6 keeps the review and the merge with Al.

## 6. CALIBRATE

- **Diff surface:** as predicted, file for file and section for section.
- **Risk 1, paraphrase drift:** materialized three times and was caught before the
  push. §6.3 said "solution length" where the clause says "the block's count"; the
  GAP-9 sentence gave clause E3 an "as worded" it does not have; and the lettered
  clauses would have rendered as one run-on paragraph until they became list items.
- **Risk 2, label collision:** held. Every citation outside the verbatim block carries
  `G0-` or names its Part.
- **Risk 3, stale statements:** materialized as predicted; 4.3 lists them.
- **Confidence against result:** MED-HIGH was about right. The clause check found
  nothing that needed a STOP and eight points for Al.
- **Surprise:** the Git Bash `grep` on this machine aborts silently when `-i` meets
  several `-e` patterns. Two searches came back empty for that reason. I re-ran them
  with ripgrep before relying on any "no match".
- **One process improvement:** when a ruling supplies normative text, check sentence
  containment by script before the manual pass. The manual pass is then spent on the
  sentences that changed.

## VERIFIED

- Preflight steps a–g: the table above.
- PR #14 merged as `358114048c3a38f4821b88fce8dffd14abbd42a1` on 2026-10-12, and run
  `38323998550` on that commit concluded success: `gh pr view 14`, `gh run view`.
- The ruling's bench figures, the P-003 quotation and the `sis.rs` convention: section
  3, with file and line pointers.
- The byte strings, lengths, constants, tables and sentence containment: section 3.
- `main` is not protected: `gh api repos/Quigles1337/COINjecture3.0/branches/main/protection`
  returned 404 "Branch not protected" on 2026-11-22.
- No file under `crates/`, `spec/`, `bench/`, `scripts/` or `.github/` changed:
  `git diff --name-only main`.

## ASSUMED

- The overlay belongs on the branch as its first commit (4.2 point 8). Safe: the merge
  puts it on `main`, and nothing was pushed there.
- P-011 is "the next free packet identifier". Safe: Phase 0 uses P-001 to P-010.
- README lines beyond the two named surfaces may be corrected. Safe: the standing
  drift-control rule and D13 require the README to be true, and 4.1 lists each line.
- This report and its number (C9, after C8) are wanted. Safe: the prompt requires a
  report for the adversary findings.

## UNKNOWN

- Whether clause C8e's limit and label reach the C8b vectors. Al's line, before
  P-007's FRAME, resolves it.
- Which typed error an oversized transaction count gets. P-007's FRAME, reviewed by
  Al, resolves it.
- What ruling gives the P-005 vectors a status now that the three issues are resolved.
- Who corrects the Lean header comments and `spec/README.md`, and when. They are
  HUMAN-lane files.
- The canonical list behind "the five D17 conditions".
- Whether Al wants the registry admission wording changed (4.1, B2).
- The D6 result for the final head: recorded on PR #20 once the run completes.
