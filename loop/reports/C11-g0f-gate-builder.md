# C11 Builder Report — Gate G0 PASS: the gate record, two text corrections and four build items

Al's live ruling "LIVE CJ3 RULING - GATE G0-F" is recorded verbatim in
[`loop/LEDGER.md`](../LEDGER.md#gate-g0-f-ruling-overlay--gate-g0-pass--pr-21-confirmed--report-c10-points-ruled--phase-1-open-al-2026-11-23)
as the first commit of this branch (`266316e`). This report cites its clauses as
"pass 2.4", "pass 4.5c" and so on. The prefix is needed because `G0-F1` and `G0-F3`
already name clauses F1 and F3 of the 2026-11-22 ruling. A clause of that ruling keeps
its `G0-` prefix, a clause of the P-007 resume ruling is "resume 2.3", a bare B1–B12 is a
block rule of Protocol Spec §6.2, a bare D1–D17 is a ledger decision, and "packet P-011"
is the SIS solver-envelope packet, never parameter P-11.

## 1. FRAME — written before any build edit

One commit precedes this FRAME on the branch: the verbatim overlay. No file under
`crates/`, `fuzz/`, `bench/`, `spec/`, `scripts/` or `.github/` had changed when this was
written.

### 1.1 The session and its done-condition

The whole scope is pass Part 4. It is done when:

1. the preflight has passed, with the merge of PR #21 and its exact-merge-SHA D6 run
   read on GitHub (pass 4.1, 1.1);
2. the ruling is in the LEDGER verbatim as the first commit of a new branch (pass 4.2);
3. the gate record is written, documentation only (pass 4.3), and this report carries
   the evidence for pass 3.2;
4. one text-only commit corrects the two stale statements of pass 2.9 and gives the
   three vector files the status pass 2.4 rules, with every vector value byte-identical
   and that shown here (pass 4.4);
5. each of the four build items of pass 4.5 is in a commit of its own and green, or is
   left out of the pull request and reported;
6. the adversary pass over the whole diff is written down, the exact head is green in
   D6 on origin, the pull request is marked ready, and it is left unmerged (pass 4.6).

Neither P-101 nor packet P-011 is picked up here. Both are unblocked (pass 3.6), and
Al names the order in the message that launches the next session.

### 1.2 Clause check before acting

The ruling orders a STOP if a clause is ambiguous or conflicts with an axiom, the Lean
files, an earlier ruling or another clause. I read every clause against A1–A11, the
four Lean sources, Protocol Spec v0.2-DRAFT, the Engineering Plan and the three earlier
rulings before the first edit. I found no clause that needs a STOP. Seven places needed
care. Each is settled by the text, and where a fact remains for Al it is recorded in
§4.2 rather than decided here.

- **Pass 3.2 with pass 3.5, on D10.** Pass 3.2 says "D9 and D10 were ruled then"
  (2026-08-15), which the LEDGER confirms word for word. Pass 3.5 says "D10 and D16 are
  still not ruled". The word "still" points back to Part 0.2 of the 2026-11-22 ruling
  ("D10 and D16 are not ruled here"), so the sentence adds nothing to what that ruling
  already left and PR #20 already recorded. No action of this session depends on it.
  The record carries each sentence in the ruling's own words.
- **Pass 4.3 with the standing README rule.** Pass 4.3 names the README's status block
  and one limitation. The standing rule of 2026-08-16 also requires the roadmap marker
  to be refreshed in the same closeout. Pass 4.3 does not forbid it, so the marker is
  refreshed under the standing rule and listed as such.
- **Pass 4.3 on the Protocol Specification.** The edit is bounded by the clause itself:
  the version line, the gate-status paragraph, and any other sentence that pass 3.1 or
  3.4 makes false. I predict three more such sentences: the footer, and two entries of
  the §15 index. No rule, layout or value changes.
- **Pass 4.5c, "serial back end".** `curve25519-dalek` 4.1.3 names its back ends
  `serial`, `fiat` and `simd`, and its build script reads one configuration value,
  `curve25519_dalek_backend`. The clause therefore names a single setting. Two facts
  remain after it, and both go to §4.2: the serial back end is two bodies of code
  chosen by pointer width, and `sha2` still selects an implementation by processor.
- **Pass 4.5d, the exemptions.** The rule reaches Rust source outside
  `crates/cj3-types`. The tests of `cj3-types` are inside that directory and the vector
  scripts are Python, so both exemptions hold by where the rule looks. One existing
  doc comment outside the crate names a tag (`crates/cj3-beacon/src/devnet.rs`). A gate
  that reads the whole text of a file is the stricter of the two ways to read "names",
  and it satisfies the other, so the gate reads the whole text and that comment is
  reworded.
- **Pass 4.4 on the Lean file.** The comment is a doc comment on `v1`. Resume 3.1's
  conditions apply: no definition, theorem, vector or exported string changes, and
  `spec/vectors/p005-draft.json` is byte-identical, with its digest given both times.
- **Pass 3.6, "the message that launches it".** It tells this session only that it
  launches neither packet.

### 1.3 Lane

**HUMAN.** Argued on the five D17 conditions:

| Condition | Holds? | Why |
|---|---|---|
| Approved and unblocked | Yes | Pass Part 4 orders exactly this work. |
| Bounded non-semantic surface | **No** | The gate record states which text is normative, and pass 4.5c fixes how a consensus crate is built. |
| No formal-spec content | **No** | A comment in `spec/Spec/Tx.lean` changes, and so do the status lines of vector files, which D17 names as HUMAN. |
| No decision invention | Yes, as a control | Every status and disposition written is the ruling's. |
| No owned-TBD fill | Yes, as a control | P-1, P-4, P-7, P-9, P-10, P-12 and `ACTIVE_SIZE_PARAM` stay unfilled. |

A phase-gate record is HUMAN by the prompt in any case. The builder does not merge.

### 1.4 How each build item is to be built

- **(a) The nightly long fuzz job.** One list of targets, in one shared script that
  both gate handlers load, so "the same targets" holds by construction. The D6 smoke
  handler keeps its fixed runs and fixed seed. The nightly handler runs each target for
  a fixed time on the toolchain pinned in `fuzz/rust-toolchain.toml`. How long, and
  from which seed, are operating choices of mine and are reported as such.
- **(b) `cargo deny` over the fuzz workspace.** `fuzz/deny.toml` is the root policy
  with one licence exception, NCSA, for `libfuzzer-sys` alone. A check compares the two
  files so that the fuzz policy cannot drift from the root one. Whether the fuzz graph
  passes the root policy's other rules (duplicate versions, sources, advisories) is not
  known until it is run. If it needs any other exception, the item is left out and
  reported.
- **(c) One arithmetic back end.** A committed cargo configuration passes
  `--cfg curve25519_dalek_backend="serial"` to every build started inside the
  repository. `cj3-types` refuses to compile unless that setting is exactly `serial`,
  judged on the same variable the library's own build script reads. cargo-fuzz replaces
  the configured flags with its own, so the fuzz handlers pass the setting themselves.
  D6 proves the refusal each run by building `cj3-types` with the setting absent and
  wrong and requiring each build to fail with the guard's message.
- **(d) Source policy.** `check-source-policy.ps1` reads every Rust source outside
  `crates/cj3-types` and fails on any of the seven tag names. It proves the rule on a
  planted violation each run, so a pass is not a silent miss.

### 1.5 Predicted diff surface

- `loop/LEDGER.md` — done above; one short index of this ruling at closeout.
- **Pass 4.4:** `spec/Spec/Tx.lean` (one doc comment), `README.md` (one diagram label),
  `bench/p007-vectors/derive.py`, and the three vector files. Predicted beyond the
  clause, because the ruling makes their status text false: the generated
  `bench/p007-vectors/WORKED-VECTORS.md` and the hand-written
  `bench/p007-vectors/README.md`.
- **Pass 4.5a:** `scripts/ci/active/full-fuzz-corpus.ps1` (new), a shared fuzz script
  under `scripts/ci/` (new), `scripts/ci/active/codec-fuzz-smoke.ps1`,
  `scripts/ci/check-phase-gate.ps1`, `.github/workflows/nightly.yml`, and the comments
  that say one job alone uses the nightly toolchain (`fuzz/rust-toolchain.toml`,
  `.github/workflows/ci.yml`).
- **Pass 4.5b:** `fuzz/deny.toml` (new), a drift check under `scripts/ci/` (new),
  `.github/workflows/ci.yml`, and the comment at the head of `fuzz/Cargo.toml`.
- **Pass 4.5c:** `.cargo/config.toml` (new), `.gitignore`, `crates/cj3-types/build.rs`
  (new), `crates/cj3-types/src/lib.rs`, the shared fuzz script, a check under
  `scripts/ci/` (new), `.github/workflows/ci.yml`, and the README limitation.
- **Pass 4.5d:** `scripts/ci/check-source-policy.ps1` and one doc comment in
  `crates/cj3-beacon/src/devnet.rs`.
- **Pass 4.3:** `docs/PROTOCOL_SPEC.md`, `docs/ENGINEERING_PLAN.md`, `README.md`,
  `loop/PACKETS.md`, `loop/STATE.md`, `loop/reports/BATCH-LOG.md` and this report.

Not predicted: any Lean definition, theorem or vector; any vector value; the root
`deny.toml`; `rust-toolchain.toml`; either lockfile; any `cj3-*` source other than the
two named above; `docs/AUDIT_TRACEABILITY.md`; `loop/reports/SPEC-ISSUES.md`; the
builder prompt; any earlier report or BATCH-LOG row.

### 1.6 Top risks, falsifier, confidence

1. **A guard that passes while the library still picks another back end.** A
   configuration value can hold several values at once, and some tools replace the
   configured flags. Control: an exact match on the variable the library reads, and
   negative builds in D6.
2. **A status line that moves a vector value.** Control: the digest of every line that
   is not a comment, before and after; `derive.py --check`; the Rust tests that read
   the files.
3. **A gate record that says more than the ruling.** Control: the ruling's own
   sentences, and an adversary pass clause by clause.

**Falsifier:** this approach is wrong if any negative build compiles; if any value
line of a vector file differs after regeneration; if `cargo deny` over the fuzz
workspace needs anything beyond NCSA for `libfuzzer-sys`; or if the nightly job cannot
run green on the pinned toolchain.

**Confidence:** MED-HIGH on the record and the text commit; MED-HIGH on (c) and (d);
MED on (b), which is unknown until it runs; MED on the first hosted run of (a), which
cannot run on this Windows host.

## Preflight evidence (prompt STEP 0; pass 4.1 and 1.1)

| Step | Result | Evidence |
|---|---|---|
| a. Path | PASS | `pwd -W` returned `C:/Users/LEET/COINjecture3.0`; no `OneDrive` segment |
| b. Remote | PASS | `git remote -v`: fetch and push are both `https://github.com/Quigles1337/COINjecture3.0` |
| c. Branch and tree | PASS | on `main`, `git status --porcelain` empty, `HEAD` = `origin/main` = `git ls-remote origin main` = `333c67086012d03f3c654cba20526e5c0f5e6b4d` after `git fetch` |
| d. Visibility | PASS | `gh repo view Quigles1337/COINjecture3.0 --json visibility` returned `PUBLIC` |
| e. Authorization | PASS | `loop/LEDGER.md` holds D17 RATIFIED (2026-08-15 overlay) |
| f. Capacity | PASS | `loop/STATE.md` carries the condition of Part 0.3 of the 2026-11-22 ruling, which holds until Al states another, and this ruling states none; `C:\Users\LEET\COINjecture2.0-network\loop\STATE.md` reads `CAPACITY_FLAG: none`, not `remediation-priority` |
| g. Survey and P-008 | PASS | `docs/RESEARCH_SURVEY.md` present; the P-008 URL is still `TBD(Al-supplied frontend URL)`, so P-008 stays blocked |
| Pass 1.1 | PASS | `gh pr view 21`: state `MERGED`, merged at `2026-11-23T05:36:52Z` by `Quigles1337`, merge commit `333c67086012d03f3c654cba20526e5c0f5e6b4d`, head `92f9be92265c83c5f0b3e27acd3311027d910c56`. `gh run view 45118879871`: workflow `D6 CI`, event `push`, that same commit, conclusion `success`, eleven jobs each `success` with five to ten steps (<https://github.com/Quigles1337/COINjecture3.0/actions/runs/45118879871>). The run was in progress when first read. Nothing was written to the repository until it concluded. |

## 2. BUILD

### 2.1 What was built, commit by commit

| Commit | Clause | What it holds |
|---|---|---|
| `266316e` | pass 4.2 | The ruling, word for word, in `loop/LEDGER.md`. Nothing else. |
| `713e2a3` | prompt, move 1 | The FRAME above and the preflight evidence. |
| `0a2720b` | pass 4.4 | Text only: the doc comment on `v1` in `spec/Spec/Tx.lean`; the README diagram label; the status lines of the three vector files, changed in `derive.py` and regenerated. |
| `49850e7` | pass 4.5a | The D6 Nightly `full-fuzz-corpus` job is live. One shared script holds the target list for both fuzz gates. |
| `aaadea3` | pass 4.5b | `fuzz/deny.toml`, a check that it is the root policy plus one line, and `cargo deny` over the fuzz workspace in the `dependency-policy` job. |
| `6f85350` | pass 4.5c | `.cargo/config.toml`; the refusal in `cj3-types` (`build.rs` and `src/lib.rs`); the setting passed by the fuzz gates; a check with positive and negative builds in the `unit-and-property-tests` job; the README limitation. |
| `169d808` | pass 4.5d | The domain-tag rule in `check-source-policy.ps1`, with planted sources; one doc comment in `crates/cj3-beacon/src/devnet.rs` reworded. |
| `e90c824` | pass 4.5c | A fix to item (c): the check left a failing exit code behind (§2.2). |
| `bd8273c` | pass 4.3 | The gate record: Protocol Spec, Engineering Plan, README, PACKETS, STATE, and the index after the verbatim text in the LEDGER. |
| the closing commit | prompt | This report from §2 on, and two new BATCH-LOG lines. |

Item (c) is in two commits, not one. I chose a fix commit over rewriting pushed history,
so that every run cited below names a commit that is on the branch. No commit holds
work of two items.

### 2.2 Tripwires

- **SCOPE: held, with two files beyond the prediction.** The files of
  `git diff --stat main...HEAD` are inside §1.5 except `spec/README.md` and
  `bench/p007-vectors/cj3_encoding_v1.py`, and the README changed in more places than
  §1.5 names. Each is a sentence the ruling makes false, and each is listed in §4.2
  point 8. No file of `cj3-kernel`,
  `cj3-consensus`, `cj3-net`, `cj3-rpc`, `cj3-store`, `cj3-node`, `cj3-classes` or
  `cj3-solver-sis` changed. The root `deny.toml`, `rust-toolchain.toml` and both
  lockfiles did not change.
- **INVENTION: held.** No protocol value is written. Values I chose, none of them a
  protocol value: 600 seconds of fuzzing per target each night, the nightly seed (the
  UTC date), the job's 120-minute limit, and the exact form of the licence exception
  (`libfuzzer-sys@0.4.13`, pinned to the version).
- **REPETITION: not tripped.** No failure repeated three times.
- **INTERPRETATION: not tripped.** The places where clauses had to be fitted together
  are in §1.2, and none needed a choice that changes what a node accepts.
- **ENVIRONMENT: held.** Visibility read `PUBLIC` again before the first push, and the
  COINjecture 2.0 loop still read `CAPACITY_FLAG: none`.
- **DEGRADATION: four slips, each caught, none shipped as wrong.**
  1. I applied the nightly-workflow edit through a shell heredoc, which ate a
     backslash and left a control byte in the `sed` line of `nightly.yml`. I knew of
     this hazard and used the wrong tool. A `grep` of the result showed it before
     anything was committed; I then scanned every touched file for control bytes (none).
  2. My first version of the domain-tag rule returned its result in a form its own
     planted-source proof could not read. The proof failed on the first run, which is
     what it is for.
  3. The back-end check passed on this machine and failed in D6 run `45121772451`: its
     last build is one that must fail, and a workflow step ends with the exit code of
     the last program. I had run it under `pwsh -File`, which hides that. Fixed in
     `e90c824`; I then ran each new script the way a workflow step runs it.
  4. I pushed the gate record while D6 run `45122293148` on `e90c824` was still going.
     The push cancelled it with nine jobs green and the fuzz job unfinished. Nothing
     was lost, because the next run covers the same code, but I gave up a complete run
     on the last code commit for no reason.

### 2.3 What changed on this machine outside the repository

- WSL Ubuntu: build output in `~/cj3-fuzz-target` and a new `~/cj3-fuzz-stable-target`.
  Nothing was installed there or on Windows.
- Temporary build and planted-source directories were removed by the scripts that made
  them.
- Read-only: the registry copies of `curve25519-dalek 4.1.3`, `sha2 0.10.9` and
  `subtle 2.6.1` under `~/.cargo/registry`, to state what the README says about them.

## 3. VERIFY

### 3.1 Pass 4.4: every vector value and the Lean artifact are byte-identical

A value line is any line of a vector file that does not start with `#`. The digest of
the value lines of each file, before the change (at `713e2a3`) and after (at `0a2720b`):

| File | Value lines | SHA-256 of the value lines, before and after | Comment lines, before → after |
|---|---:|---|---|
| `encoding-v1.vectors.txt` | 165 | `8a623b8c52da433766de026f6e7e5f2df1be7b55467525b230386063195a52cd` | 21 → 24 |
| `shake-expansion.vectors.txt` | 406 | `32e338b415f1a248d04bc9d0037b843db914c003f8db3e7361427b7fcc4e4851` | 27 → 34 |
| `ed25519/c7-expected.vectors.txt` | 934 | `e5b468a25d73fcea8fcf71ea6a5b8a18b2ba075b3aeec5f8121109599861e1c7` | 27 → 32 |

The counts include empty lines; the third file holds 932 vectors.
`git diff -U0` of each file has no changed line that is not a comment. Of the sixteen
files `derive.py` writes, four changed digest: those three and `WORKED-VECTORS.md`. The
eleven fuzz seeds, which are the worked bytes, are unchanged, and so is
`ed25519/C7-SUMMARY.md`. `derive.py --check` reports `P007_VECTORS=PASS files=16`, and
the nineteen Rust tests that read the files pass.

Whole-file digests, for the record:

| File | Before | After |
|---|---|---|
| `encoding-v1.vectors.txt` | `d00e21a47f6b06d02813c94a68d31318005c0f0d4ea81e13bfb40c8de84acfe0` | `f5061cef1227fee26e732b11adbb0efbadb528c6647a86d70ceaed795f17d855` |
| `shake-expansion.vectors.txt` | `b2640db1b6a09399a503fd947d2d53822051c496a7e51564eb9e8476e4b9f5e8` | `4165807f0f386aef075a98fa7ba74a43a6b037e3bd873a3ffe4b47ad5ea90156` |
| `ed25519/c7-expected.vectors.txt` | `2bf9fc029aeb9521722aa84f0f534d3f199cebcd6d9a793ffeede68e3f145f4b` | `161a4b021955677685e674f3c8eb79eb34a2ea0823256e379d09b77f49bc27b7` |

**`spec/vectors/p005-draft.json` (resume 3.1, applied by pass 2.9).** Git blob
`cc0c3f71c3f7af7db50287f4f56e526bbc56d831` at `main` and at the head of this branch.
SHA-256 before and after:
`30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`. After the comment
changed, the Lean gate rebuilt `Spec.Tx`, `Spec.Stf`, `Spec.Vectors` and `Main` and
regenerated the same vectors:
`P005_RATIFIED_SPEC_BUILD=PASS LEAN=4.33.0 VECTORS=27 SHA256=30CABF85…C324E1`. The one
change in `spec/Spec/Tx.lean` is the doc comment on `v1`; no definition, theorem or
exported string changed, and the NORMATIVE STATUS line is untouched.

### 3.2 Evidence for pass 3.2

Pass 3.2 states how the conditions of Engineering Plan §4 are met. Each statement,
checked on 2026-11-23:

**"The reports of P-001 to P-007, P-009 and P-010 are in loop/reports and their work
is merged."** Each merge commit is an ancestor of `main`
(`git merge-base --is-ancestor`), each pull request reads `MERGED` (`gh pr view`), and
each merge commit has a `push`-event D6 run that concluded success
(`gh api …/actions/runs?head_sha=`).

| Packet | Report in `loop/reports` | Pull request | Merge commit | D6 run on the merge commit |
|---|---|---|---|---|
| P-001 | `C1-p001-builder.md` | #1 | `8367de08bb3d3766bf49b9970eb3109fd1af4389` | `31921663254` |
| P-002 | `C2-p002-builder.md` | #3 | `7ecba896c33c5a003bd6a9424f6742cd69c1156c` | `31923608134` |
| P-003 | `C3-p003-builder.md` | #5 | `e00561572c48137d535a44bcce55c04ad7db732b` | `31926645964` |
| P-004 | `C4-p004-builder.md` | #7 | `4644374f5073a929bf5ecc88c0e191f0d9bab1be` | `31954588282` |
| P-005 | `C5-p005-builder.md` | #9 | `916e5027f0918973bf45d6a2bf90abd2ae253197` | `31968030240` |
| P-006 | `C7-p006-builder.md` | #16 | `7244f094aa22d1890039a535ad997c9775ba9ed3` | `31971438778` |
| P-007 | `C7-p007-builder.md`, `C10-p007-builder.md` | #14, #21 | `358114048c3a38f4821b88fce8dffd14abbd42a1`, `333c67086012d03f3c654cba20526e5c0f5e6b4d` | `38323998550`, `45118879871` |
| P-009 | `C7-p009-builder.md` | #15 | `ce0b9752c9101e89f93a90379cd9e5ac1a08842d` | `31972017916` |
| P-010 | `C6-p010-builder.md` | #10, #11 | `3b43cca42e14e53412747fb2b559d6fc29c4ac9d`, `1e986cb22956f2970e08281e18a11282c812ed9e` | `31957922841`, `31958393946` |

**"D1, D2, D14 and D15 were ratified on 2026-08-15."** The LEDGER's live autonomy
ruling overlay of 2026-08-15 holds "D1 RATIFIED as proposed", "D2 RATIFIED as proposed",
"D14 RATIFIED as proposed, with final VDF technology selected by P-002" and "D15
RATIFIED as proposed".

**"…and are confirmed here on the spike evidence since, D2 as G0-A amended it and D1 as
3.3 conditions it."** The spike evidence since, by decision:

| Decision | Spike evidence | Where |
|---|---|---|
| D1 | The sampled-SIS class, its checker and derivation, and the measurements that led Al to reject the proposed tuple and commission packet P-011 | `C3-p003-builder.md`, `bench/p003-sis/`; `C4-p004-builder.md` (no legacy class admitted); G0-B3, G0-B4 |
| D2 | The difficulty simulation: 0/36 for the size controller, 6/6 for the isolated hash loop | `C7-p006-builder.md`, `bench/p006-difficulty/`; the G0-A amendment |
| D14 | The beacon spike and the trait-gated boundary | `C2-p002-builder.md`, `crates/cj3-beacon` |
| D15 | Ed25519 verification exactly as §1.3, with 932 vectors | `C10-p007-builder.md`, `bench/p007-vectors/ed25519/` |

That table points at where the evidence is. The confirmation is Al's.

**"D9 and D10 were ruled then, and D9 was superseded on 2026-11-22."** The LEDGER holds
"D9 — RULED (Al, 2026-08-15)" and "D10 — RULED (Al, 2026-08-15)". Part 0.1 of the
2026-11-22 ruling says "D9's 'private through the build' is superseded".

**"P-008 stays blocked and is carried."** `loop/PACKETS.md` still reads
`TBD(Al-supplied frontend URL)`.

**The standing size-adequacy item of G0-A.** There is no active `size_param` to retain
or change at this gate: `ACTIVE_SIZE_PARAM` and P-4 are UNFILLED. Pass 3.3 is the gate's
word on adequacy, and it is recorded as such in the LEDGER index.

### 3.3 The four build items

**(a) The nightly long fuzz job.**

- On this Windows host both handlers load the shared script, pass the toolchain check
  and stop where they must: `cargo-fuzz version mismatch. Required: cargo-fuzz 0.13.2.
  Found: cargo-fuzz 0.13.1.` A budget that does not name every target is refused.
- In WSL (cargo-fuzz 0.13.2, `nightly-2026-11-15`), each of the six targets ran under a
  time limit, as the nightly handler runs them, and exited 0. For ten seconds each, with
  the serial setting: 142,120 runs of `transaction_codec`, 571,372 of `header_codec`,
  500,806 of `block_codec`, 46,026 of `sis_solution_codec`, 75,250 of `ed25519_verify`
  and 342 of `length_limits`.
- Hosted: D6 Nightly run `45121770277`, dispatched on the branch at `169d808`,
  concluded success at its first attempt. From its log:
  `GATE=full-fuzz-corpus STATUS=ACTIVE`,
  `FUZZ_TOOLCHAIN=rustc 1.102.0-nightly (5f4effd57 2026-11-13)`,
  `FUZZ_DRIVER=cargo-fuzz 0.13.2`,
  `FUZZ_RUSTFLAGS=--cfg curve25519_dalek_backend="serial"`, each of the six targets
  `SECONDS=600 RESULT=PASS`, and `FULL_FUZZ_CORPUS=PASS TOOLCHAIN=nightly-2026-11-15
  TARGETS=6 TOTAL_SECONDS=3600 SEED=20261123`. libFuzzer reported 36,924,773 runs in
  all: 3,526,978 of `transaction_codec`, 15,675,015 of `header_codec`, 15,526,840 of
  `block_codec`, 823,695 of `sis_solution_codec`, 1,366,320 of `ed25519_verify` and
  5,925 of `length_limits`. The job took 61 minutes. That is about fifty times the
  smoke run. It is still not evidence that no defect exists.

**(b) `cargo deny` over the fuzz workspace.**

- With the root policy unchanged, `cargo deny --manifest-path fuzz/Cargo.toml --config
  deny.toml --locked --all-features check` fails on exactly one thing:
  `error[rejected]: failed to satisfy license requirements` at
  `license = "(MIT OR Apache-2.0) AND NCSA"` in `libfuzzer-sys v0.4.13`, and prints
  `advisories ok, bans ok, licenses FAILED, sources ok`.
- With `fuzz/deny.toml` it prints `advisories ok, bans ok, licenses ok, sources ok`.
  **It needs no other exception.**
- The drift check passes on the committed file
  (`FUZZ_DENY_POLICY=PASS root_lines=37 fuzz_lines=38 added=1`). It failed on each of
  five changes I made to a copy and then undid: a second line added; a rule changed
  from `deny` to `warn`; the exception widened to every version of the crate; NCSA
  added to the allow-list; and the exception moved to another table. The restored file
  was byte-identical to the committed one.

**(c) One arithmetic back end.**

- What the library did before: the build records in this checkout's `target/` from
  earlier builds read `cargo:rustc-cfg=curve25519_dalek_backend="simd"`. After the
  change the record reads `"serial"`, and `cj3_types` is compiled with
  `--cfg 'curve25519_dalek_backend="serial"'`.
- The check, here and in D6:

  ```text
  ARITHMETIC_BACKEND_CASE=configuration-file EXPECTED=builds RESULT=PASS
  ARITHMETIC_BACKEND_CASE=rustflags-serial EXPECTED=builds RESULT=PASS
  ARITHMETIC_BACKEND_CASE=rustflags-without-the-setting EXPECTED=does-not-compile REASON=cj3-types-refuses RESULT=PASS
  ARITHMETIC_BACKEND_CASE=rustflags-simd EXPECTED=does-not-compile REASON=cj3-types-refuses RESULT=PASS
  ARITHMETIC_BACKEND_CASE=rustflags-fiat EXPECTED=does-not-compile REASON=cj3-types-refuses RESULT=PASS
  ARITHMETIC_BACKEND_CASE=rustflags-serial-and-simd EXPECTED=does-not-compile REASON=the-library-does-not-compile RESULT=PASS
  ARITHMETIC_BACKEND=PASS backend=serial builds=2 refusals=4
  ```

- cargo-fuzz, in WSL. With `RUSTFLAGS` unset, the fuzz build stops at the crate's
  refusal ("error: cj3-types must be built with exactly …") and
  `error: could not compile cj3-types (lib) due to 1 previous error`. That shows
  cargo-fuzz's own flags replace the configuration file's. With the setting in
  `RUSTFLAGS`, all six targets build and run.
- Two more ways to lose the flag, each tried once and each refused with the same
  message: a `[target.x86_64-pc-windows-msvc] rustflags` entry given on the command
  line, and a build started from outside the repository with `--manifest-path`. The
  second first failed for another reason: outside the repository the pinned toolchain
  does not apply either. With `cargo +1.97.1` it reached the refusal.
- The tests, the vectors and the fuzz properties under it: 104 workspace tests pass,
  among them the 932 Ed25519 verdicts and every worked vector. Those verdicts agreed
  with the standard-library derivation under the vector back end in PR #21's runs, and
  agree under the serial back end now. The seven fuzz properties pass as ordinary tests
  on stable 1.97.1, named explicitly, with the flags from the configuration file.
- The zero-unsafe gate, in a fresh target directory:
  `GEIGER_POLICY=PASS packages=11 unsafe_count=0`. `cargo doc -p cj3-types` builds, so
  the rustdoc flags are in place.

**(d) Source policy.**

- Before the comment in `cj3-beacon` was reworded, the new rule failed on the tree as it
  stood: `domain tag constant named outside crates/cj3-types:
  crates/cj3-beacon/src/devnet.rs names D_BEACON`.
- After: `DOMAIN_TAG_POLICY=PASS tags=7 read_outside_cj3_types=24 planted_flagged=4
  planted_passed=3`, then `SOURCE_POLICY=PASS files=29 crates=10`.
- A file I planted under `fuzz/fuzz_targets/` with a tag name in a comment failed the
  gate. A similar file planted under `crates/cj3-types/tests/` passed. Both were
  removed.
- On every run the gate plants seven sources in a temporary tree. It must flag exactly
  four: a use in another `cj3-*` crate, a comment in a fuzz target, a crate whose name
  only starts with `cj3-types`, and a bench source. It must pass three: a test of
  `cj3-types`, a source of `cj3-types`, and lookalike names (`d_tx`, `MAX_D_TX_COUNT`,
  `D_TXN`).

### 3.4 Local checks at the last code commit

| Check | Result |
|---|---|
| `cargo fmt --all -- --check`; the same for `fuzz/` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --all-targets --all-features --locked` | PASS, 104 tests in 16 suites |
| `cargo build --workspace --all-targets --all-features --locked` | PASS |
| `cargo deny --locked --all-features check`, root `deny.toml` unchanged | `advisories ok, bans ok, licenses ok, sources ok` |
| `cargo deny` over `fuzz/` with `fuzz/deny.toml` | `advisories ok, bans ok, licenses ok, sources ok` |
| `cargo audit --deny warnings`; the same with `--file fuzz/Cargo.lock` | PASS, 34 and 33 crates |
| `python -I -B bench/p007-vectors/derive.py --check` | `P007_VECTORS=PASS files=16` |
| `scripts/ci/check-source-policy.ps1` | PASS, lines in §3.3 (d) |
| `scripts/ci/verify-geiger.ps1`, fresh target directory | `GEIGER_POLICY=PASS packages=11 unsafe_count=0` |
| `scripts/ci/check-arithmetic-backend.ps1` | PASS, lines in §3.3 (c) |
| `scripts/ci/check-fuzz-deny-policy.ps1` | PASS, line in §3.3 (b) |
| Lean gate (`check-phase-gate.ps1 -Gate lean-conformance`) | PASS, 27 vectors, digest in §3.1 |
| The three new check scripts, each run as a workflow step runs it (dot-sourced, then the exit code) | exit 0 |

A caution about report C10 §3.3, which says the fuzz properties were run "on stable"
with `cargo test --lib --locked` in `fuzz/`. A cargo run inside `fuzz/` uses the nightly
that `fuzz/rust-toolchain.toml` pins unless a toolchain is named. I did not record then
whether one was named. This time it was (`cargo +1.97.1`).

### 3.5 Origin

The exact-head run for the final head is recorded on
[PR #22](https://github.com/Quigles1337/COINjecture3.0/pull/22). This file cannot hold
the run of the commit that contains it, so it makes no green claim for that head.

Earlier heads can be cited:

| Commit | Run | Result |
|---|---|---|
| `169d808` (all four build items) | D6 [`45121772451`](https://github.com/Quigles1337/COINjecture3.0/actions/runs/45121772451) | **Failure.** Five jobs green; `unit-and-property-tests` failed at the back-end check's exit code after every case had printed PASS (§2.2, slip 3). The later jobs were skipped. |
| `e90c824` (the fix) | D6 [`45122293148`](https://github.com/Quigles1337/COINjecture3.0/actions/runs/45122293148) | **Cancelled** by my next push (§2.2, slip 4). Nine jobs had succeeded, `unit-and-property-tests` among them; `codec-fuzz-smoke` and `locked-build` were cancelled. |
| `bd8273c` (the gate record, on the same code) | D6 [`45122858340`](https://github.com/Quigles1337/COINjecture3.0/actions/runs/45122858340) | **Success**, all eleven jobs, each with five to eleven steps. Its log lines are below. |
| `169d808` | D6 Nightly [`45121770277`](https://github.com/Quigles1337/COINjecture3.0/actions/runs/45121770277), `workflow_dispatch` on the branch | **Success.** `full-fuzz-corpus` ran for 61 minutes; its lines are in §3.3 (a). No commit after `169d808` touches the fuzz gates. |

Read from the log of run `45122858340` with `gh run view --log`. The commits after
`e90c824` change documentation and one docstring of a bench script, so this run is of
the final code:

- `dependency-policy`: `advisories ok, bans ok, licenses ok, sources ok` for the
  workspace under the unchanged `deny.toml`;
  `FUZZ_DENY_POLICY=PASS root_lines=37 fuzz_lines=38 added=1 exception=libfuzzer-sys@0.4.13:NCSA`;
  then `advisories ok, bans ok, licenses ok, sources ok` for the fuzz workspace under
  `fuzz/deny.toml`.
- `unsafe-audit`:
  `DOMAIN_TAG_POLICY=PASS tags=7 read_outside_cj3_types=24 planted_flagged=4 planted_passed=3`,
  `SOURCE_POLICY=PASS files=29 crates=10`,
  `GEIGER_POLICY=PASS packages=11 unsafe_count=0`.
- `unit-and-property-tests`: `P007_VECTORS=PASS files=16`;
  `ARITHMETIC_BACKEND=PASS backend=serial builds=2 refusals=4`; then the workspace
  tests.
- `codec-fuzz-smoke`: `FUZZ_TOOLCHAIN=rustc 1.102.0-nightly (5f4effd57 2026-11-13)`,
  `FUZZ_DRIVER=cargo-fuzz 0.13.2`,
  `FUZZ_RUSTFLAGS=--cfg curve25519_dalek_backend="serial"`, each of the six targets
  `RESULT=PASS`, and `CODEC_FUZZ_SMOKE=PASS TOOLCHAIN=nightly-2026-11-15 TARGETS=6
  TOTAL_RUNS=740300 SEED=20261122`. The smoke run is unchanged in runs and seed, and now
  runs through the shared script.
- `lean-conformance`: `P005_RATIFIED_SPEC_BUILD=PASS LEAN=4.33.0 VECTORS=27
  SHA256=30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`.
- `conservation-invariant` and `genesis-spend-test`: still `NOT_YET_ADMITTED`. Their
  green is a deferral marker, not a test result.

## 4. ADVERSARY PASS

Seat switched. I re-read the whole diff against `main` as someone trying to make two
honest nodes disagree, slip a second way of building past a gate, or make this record
claim more than the ruling gives.

### 4.1 Findings

1. **The pin was needed.** Every D6 run before this branch, the 932-vector agreement of
   PR #21 included, ran on whichever vector code the hosted runner's processor selected.
   Two nodes on different processors ran different arithmetic. They do not now.
2. **The serial back end is two bodies of code.** The library has a 64-bit and a 32-bit
   serial implementation and chooses by pointer width. Pass 4.5c names the serial back
   end, and that is what is enforced. A 32-bit node would still run arithmetic that no
   D6 job builds or tests. The README says so.
3. **`sha2` is outside the pin.** On x86 processors it still chooses between portable
   code and processor extensions when it runs. Pass 4.5c names `curve25519-dalek`. The
   README says so.
4. **Two values at once never reach my guard on x86_64.** With the setting given as
   `serial` and `simd` together, the library itself fails to compile. The exact
   comparison in `build.rs` matters on targets with no vector back end, where the
   library would accept the pair. The D6 check accepts either reason for that one case
   and prints which it saw.
5. **Ways a build can lose the flag.** A `RUSTFLAGS` variable; a `[target.…] rustflags`
   entry in a user's own cargo configuration, which outranks `[build] rustflags`; a
   build started from outside the repository. I tried each, and in each the crate
   refuses to compile (§3.3). A build system that is not cargo is covered by the first
   half of the crate's test; I did not try one.
6. **The domain-tag rule matches names.** It does not catch the tag bytes typed out
   again in another crate, or a constant that `cj3-types` itself re-exports under
   another name. The second needs a change inside `cj3-types`, which is reviewed. The
   first is the residual risk of finding 4 of report C10, now smaller, not gone.
7. **The source-policy gate now needs git.** It lists Rust sources with `git ls-files`
   so that build output is not read and a new, uncommitted source is. Without a
   repository it fails; it does not pass.
8. **The drift check compares text.** A reformatted but equivalent `fuzz/deny.toml`
   fails it. That is the strict side of the trade.
9. **The nightly job keeps no corpus.** Each night starts from the committed seeds with
   a new seed number. The job's name, `full-fuzz-corpus`, is the plan's name for it and
   promises more than that. A failing input is printed in the log, because the runner
   is discarded.
10. **A fuzz failure at night blocks nothing.** The nightly workflow is not a D6 check.
    Someone has to read it.
11. **The Lean comment says a little more than "not unresolved".** It now says where
    the decoder was resolved (Protocol Spec §1.1 and §7). That is true and is prose, but
    it is more than pass 2.9 asked.
12. **Panic paths and unsafe.** `crates/cj3-types/build.rs` has no `unwrap` or `expect`
    and forbids unsafe code. No library code changed. `unsafe` in `cj3-*` crates is
    zero in 11 packages.

No Critical. Nothing found needed a STOP.

### 4.2 Points for Al — noticed, recorded, not decided

1. **D10, in pass 3.2 and pass 3.5.** Pass 3.2 says D10 was ruled on 2026-08-15; pass
   3.5 says "D10 and D16 are still not ruled". I read "still" as pointing at Part 0.2 of
   the 2026-11-22 ruling and changed nothing because of it. The record quotes each
   sentence. If pass 3.5 means more, the LEDGER index, PACKETS and STATE need a line.
2. **The word size of the serial back end** (finding 2). The library has a second
   setting, `curve25519_dalek_bits`. Pinning it to 64, or refusing to compile on a
   32-bit target, would make "the same arithmetic" exact. Neither is done: pass 4.5c
   does not ask for it.
3. **`sha2`** (finding 3). `sha2 0.10.9` has a `force-soft` feature that selects its
   portable code everywhere. It is not enabled: pass 4.5c does not ask for it.
4. **Signature checks are slower on the serial back end** than on the vector one.
   Nothing here measures by how much. It bears on the limits Al re-rules with P-1.
5. **The domain-tag rule** (findings 6 and 7): names, not bytes; comments included; one
   comment in `cj3-beacon` reworded so the tree passes.
6. **The nightly job's operating choices** (finding 9): 600 seconds per target, the UTC
   date as seed, no corpus kept between nights, a 120-minute limit.
7. **The licence exception is pinned to a version**, `libfuzzer-sys@0.4.13`. A bump of
   that crate then needs an edit to `fuzz/deny.toml` and to the drift check. Pass 4.5b
   says "for that crate alone"; the version pin is the narrower form.
8. **Edits no clause named.** Each exists because the ruling makes a sentence false:
   - `bench/p007-vectors/WORKED-VECTORS.md` (generated) and the status section of
     `bench/p007-vectors/README.md`, which said the vectors claim no status;
   - the version of the specification named in that guide and in the docstring of
     `cj3_encoding_v1.py`;
   - one sentence of `spec/README.md`, which said P-101 remains blocked on Gate G0;
   - in the README: the roadmap marker (required by the standing rule of 2026-08-16),
     two items of the evidence section, four rows of the repository map and one row of
     the table of unfilled values;
   - comments in `fuzz/Cargo.toml`, `fuzz/rust-toolchain.toml` and `ci.yml` that
     described the fuzz crate as outside `cargo deny` or the nightly toolchain as used
     by one job.
9. **Stale text left.** The bullet in `loop/STATE.md` that begins "P-005's builder tail
   is now explicit" is written in the present tense of 2026-08-16. Later bullets
   supersede it. I left it because this ruling did not make it false.
10. **"A cost worth calling useful work" (pass 3.3).** Packet P-011's terms say when a
    family point fails. They give no measure of what cost is worth calling useful work.
    Its FRAME can report the measured costs and leave that judgment to Al, or take a
    measure from Al beforehand.
11. **Item (c) is in two commits** (§2.1).

### 4.3 The record against the ruling

| Clause | Where it is recorded | More or less than the clause? |
|---|---|---|
| 1.1 | LEDGER index; PACKETS (P-007 row and section); STATE; README status block | Neither. |
| 2.1 to 2.3, 2.5 to 2.8, 2.10, 2.11 | LEDGER index; PACKETS, P-007 section | Shortened. The verbatim text governs. |
| 2.4 | The three vector files; the vectors guide; PACKETS; STATE; README | The sentence on the relation is quoted whole in each vector file. `WORKED-VECTORS.md` adds that the ruling names the other file, not it. |
| 2.9 | `spec/Spec/Tx.lean`; README diagram | The Lean comment adds where the decoder was resolved (finding 11). |
| 3.1 | Protocol Spec; Engineering Plan; README; PACKETS; STATE | Neither. |
| 3.2 | Engineering Plan §4; §3.2 above | The plan's line shortens it. |
| 3.3 | README limitation; PACKETS, packet P-011; STATE; LEDGER index | The README carries all five sentences. |
| 3.4 | Protocol Spec, "Normative status" | Adds that §15 indexes the carried items that are values or constructions of the document. P-008, D10 and D16 are carried and are not in §15. |
| 3.5 | PACKETS, "Carried"; STATE, "Next action" | PACKETS keeps the GAP items of G0-E beside them, which pass 3.5 does not list. |
| 3.6 | PACKETS; STATE; README | Adds "neither has started". |

### 4.4 A1–A11, banned identifiers, unsafe, TBD integrity

| Axiom | What I looked for | Result |
|---|---|---|
| A1–A4 | A consensus structure, instance input, scoring input or fork-choice input changed | None. No codec, preimage, checker or rule changed. |
| A5 | A float or an amount path | None. |
| A6 | A validity check moved or merged | None. |
| A7 | A new gate that can pass by doing nothing | Each fails closed: the back-end refusal, the planted-source proof, the "no sources found" throw, the drift check. One did fail open in the other direction (§2.2, slip 3). |
| A8 | Rust or prose deciding what Lean owns | One Lean doc comment. No definition, theorem or vector. |
| A9 | The trusted base | No dependency added or changed. One back end instead of a choice by processor. `unsafe` in `cj3-*`: zero. |
| A10 | A claim beyond the evidence | The README says what the pin does not cover; the nightly run is called what it is. |
| A11 | A verification claim without a pointer | Each claim in §3 names its command or run. |

The source-policy gate passed over 29 Rust files, so none of `solve_time`,
`work_score`, `reported_*` or `self_reported` is present. P-1, P-4, P-7, P-9, P-10, P-12
and `ACTIVE_SIZE_PARAM` are unfilled, and no value was written for any carried item.

## 5. HANDOFF — no MERGE

[PR #22](https://github.com/Quigles1337/COINjecture3.0/pull/22) carries this change. Its
description holds the exact-head D6 run. It is marked ready and left unmerged. Pass 4.6
keeps the review and the merge with Al. Neither P-101 nor packet P-011 is started: Al
says which runs first in the message that launches it (pass 3.6).

## 6. CALIBRATE

- **Diff surface:** as predicted, plus two files of stale text and more README
  sentences than I named (§2.2).
- **Risk 1, a guard that passes on the wrong back end:** did not materialize, and the
  work on it found something I had not predicted: the library refuses a doubled value
  itself on x86_64.
- **Risk 2, a status line that moves a value:** did not materialize.
- **Risk 3, a record that says more than the ruling:** materialized twice in draft and
  was caught on the re-read. I had written that every open item of the spec's §15 is
  carried, and that packet P-011 "tests" D1's condition where the ruling says
  "measures". §4.3 lists what remains.
- **Confidence against result:** MED on (b) was low: the fuzz graph passed the root
  policy at the first run with the one exception. MED-HIGH on (c) was high for the
  code and wrong for its check, which failed in D6 on an exit code. MED on (a)'s first
  hosted run was low: the nightly job ran green at its first attempt. The hosted
  failure came where I was more confident.
- **Surprises:** the doubled value; that a cargo run inside `fuzz/` uses the nightly
  without being asked; that a workflow step's exit code is the last program's.
- **One process improvement:** run every new CI script locally the way the workflow
  runs it, and do not push while a run I mean to cite is still going.

## VERIFIED

- Preflight steps a–g and pass 1.1: the table above.
- Every vector value is byte-identical, and so is `spec/vectors/p005-draft.json`: §3.1.
- Each statement of pass 3.2: §3.2.
- The four build items: §3.3 and §3.5.
- Format, lints, tests, locked build, both dependency policies, both audits, source
  policy, unsafe count and the Lean gate at the last code commit: §3.4.

## ASSUMED

- "Serial back end" in pass 4.5c means the library's setting of that name. Safe: it is
  the library's own word, and the two things it leaves open are in §4.2.
- A gate that reads comments satisfies "names a domain tag constant". Safe: it is the
  stricter reading and meets the other.
- The nightly budget and seed are mine to choose. Safe: they are operating values, the
  log names them, and §4.2 lists them.
- Correcting a sentence the ruling makes false is within the session. Safe: resume 1.2
  and pass 2.8 confirmed the same practice twice, and each edit is listed for Al to
  strike.
- The report number is C11, after C10.

## UNKNOWN

- Whether pass 3.5's sentence on D10 and D16 means more than Part 0.2 did (§4.2 point 1).
- Whether Al wants the word size or `sha2` pinned as well (§4.2 points 2 and 3).
- What cost is "worth calling useful work" (§4.2 point 10).
- The D6 result for the final head: recorded on PR #22 once the run completes.
