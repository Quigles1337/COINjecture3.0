# C12 Builder Report — the ruling "P-101 TO ITS FRAME": two build items, one preflight step, two text corrections

Al's live ruling "LIVE CJ3 RULING - P-101 TO ITS FRAME" is recorded verbatim in
[`loop/LEDGER.md`](../LEDGER.md#p-101-frame-ruling-overlay--pr-22-confirmed--report-c11-points-ruled--p-101-to-its-frame-al-2026-11-23)
as the first commit of this branch (`359a1ae`). This report cites its clauses as
"launch 2.1", "launch 4.3a" and so on, because it is the message that launches P-101
(pass 3.6). A clause of the Gate G0-F ruling is still "pass 3.5", a clause of the P-007
resume ruling "resume 2.3", and a clause of the 2026-11-22 ruling keeps its `G0-` prefix.
A bare B1–B12 is a block rule, a bare D1–D17 a ledger decision, and "packet P-011" is the
SIS solver-envelope packet, never parameter P-11.

This file is the session's report. P-101's own report, which holds the FRAME that
launch Part 3 orders, is [`C12-p101-builder.md`](C12-p101-builder.md) (launch 4.6).

## 1. FRAME — written before any build edit

One commit precedes this FRAME on the branch: the verbatim overlay. No file under
`crates/`, `fuzz/`, `bench/`, `spec/`, `scripts/`, `.cargo/` or `.github/` had changed
when this was written, and neither had the builder prompt, `loop/PACKETS.md`,
`loop/STATE.md` or the README.

### 1.1 The session and its done-condition

The whole scope is launch Part 4. It is done when:

1. the preflight has passed, with the merge of PR #22 and its exact-merge-SHA D6 run
   read on GitHub (launch 4.1, 1.1);
2. the ruling is in the LEDGER verbatim as the first commit of a new branch (launch 4.2);
3. each of the two build items of launch 4.3 is in a commit of its own and green, or is
   left out of the pull request and reported, and the README limitation says what is
   then true of the curve arithmetic, of SHA-2 and of SHA-3;
4. the builder prompt has step h after step g and no other change (launch 4.4);
5. one commit holds the text of launch 2.1 and 2.9 and nothing else (launch 4.5);
6. P-101's FRAME is in P-101's report, complete enough to rule as it stands, with its
   worked examples derived by a standard-library script under `bench/` (launch 4.6,
   Part 3);
7. the adversary pass over the whole diff is written down, the exact head is green in D6
   on origin, the pull request is marked ready, and it is left unmerged (launch 4.7).

P-101's build does not begin here (launch 3.1, 4.7). This session writes no kernel code.

### 1.2 Clause check before acting

The ruling orders a STOP if a clause is ambiguous or conflicts with an axiom, the Lean
files, an earlier ruling or another clause. I read every clause against A1–A11, the four
Lean sources, Protocol Spec v0.2, the Engineering Plan and the four earlier rulings
before the first edit. I found no clause that needs a STOP. Ten places needed care. Each
is settled by the text, and where something remains for Al it is listed in §4.2 of this
report and not decided here.

- **Launch 2.1, "No index line changes".** I read it as: no line of an index already in
  the LEDGER changes. Every earlier ruling has an index after its verbatim text, added at
  closeout, and this one gets the same. It is a new section, and it is listed as an edit
  no clause named.
- **Launch 2.1, "quote the loose sentence".** Two places quote pass 3.5's last sentence:
  the "Carried" list of `loop/PACKETS.md` and one bullet of `loop/STATE.md`. Those two
  get the meaning beside them, citing the clause. A third place in `loop/STATE.md` does
  not quote it: it restates Part 0.2 of the 2026-11-22 ruling as "D10 and D16 are not
  ruled", without that ruling's word "here". The commit of launch 4.5 does not touch it.
  It is handled at closeout and listed for Al.
- **Launch 2.9, where the date goes.** The clause gives the words and says to change
  nothing else. I put them after "explicit", set off by a comma, and do not re-wrap the
  bullet, so not even a line break changes.
- **Launch 4.3, the README limitation.** It is to say what is "then" true of three
  things. One of them, SHA-3, belongs to neither build item. So the limitation is
  corrected in a text-only commit after both items, and each build item stays in a
  commit of its own.
- **Launch 4.3a, "doubled".** `curve25519-dalek` 4.1.3 reads the setting in its build
  script and accepts exactly `32` or `64`. Anything else, a doubled value included, it
  treats as unset and picks by pointer width. So with both values given, either the
  library fails to compile or it compiles and `cj3-types` must refuse on its own exact
  comparison. The clause requires the build to fail, and both are failures. The check
  accepts those two reasons for that one case and prints which it saw, as it already
  does for the doubled back end.
- **Launch 4.3b, "the resolved build does not have it".** I read "has it" as: every
  `sha2` that a D6 build compiles is compiled with `force-soft`, and the feature is in
  effect. `sha2` 0.10.9 has a second feature, `force-soft-compact`, which its source
  tests first: with both enabled the library runs a different body of portable code. A
  check that also fails on that second feature is the stricter of the two readings and
  satisfies the other. It is listed for Al.
- **Launch 4.4, the step's wording.** I write the step in the ruling's own words. One
  case falls between its two sentences: a run that fails after a target has started, for
  a reason that is not a target (a time limit, a lost runner). The step's text does not
  decide it, and it is listed for Al.
- **Launch 4.4 and this session's preflight.** Step h is not in the prompt until this
  session adds it. Launch 2.6 says it "makes every session read it", so the latest
  nightly run on `main` is read in this preflight as well and reported below.
- **Launch 3.3b with 3.3f.** The FRAME proposes tags by name and bytes and assigns none.
  A worked example with all of its bytes shown needs tag bytes, so the script uses the
  proposed ones and every file says they are proposed and unassigned. No Rust source
  names them, and nothing under `crates/` changes.
- **Launch 3.3f, "the only code this session adds beyond 4.3".** A D6 step that re-runs
  the example script would be a workflow change, which is code. So the script is not
  wired into D6. Its output is checked on this machine, and the report gives the digests.

### 1.3 Lane

**HUMAN.** Argued on the five D17 conditions:

| Condition | Holds? | Why |
|---|---|---|
| Approved and unblocked | Yes | Launch Part 4 orders exactly this work. |
| Bounded non-semantic surface | **No** | Launch 4.3 fixes how a consensus crate is built, and the FRAME proposes two consensus constructions. |
| No formal-spec content | Yes, as a control | No file under `spec/` changes. The FRAME says what Lean content P-101 would add; it adds none. |
| No decision invention | Yes, as a control | The FRAME proposes and assigns nothing. Every decision is listed for Al. |
| No owned-TBD fill | Yes, as a control | P-1, P-4, P-7, P-9, P-10, P-12 and `ACTIVE_SIZE_PARAM` stay unfilled. |

The ruling keeps the review and the merge with Al in any case (launch 4.7).

### 1.4 How each item is to be built

- **Launch 4.3a, the word size.** The same three places as the back end: the cargo
  configuration passes `--cfg curve25519_dalek_bits="64"` beside the back-end flag; the
  build script of `cj3-types` reads `CARGO_CFG_CURVE25519_DALEK_BITS`, the variable the
  library's own build script reads, and reports anything but exactly `64`; the crate
  refuses to compile on that report, with a message of its own so the check can tell the
  two refusals apart. The fuzz gates pass both flags, because cargo-fuzz replaces the
  configured ones. The D6 check gains three builds that must fail: the setting absent,
  set to `32`, and doubled. Its existing cases keep proving the back end, each now with
  the word size set, so that a failure is attributed to the right setting.
- **Launch 4.3b, `sha2`.** One feature in the manifest of `cj3-types`. No dependency is
  added and neither lockfile changes, because a lockfile does not record features. A new
  check reads what cargo resolved for the workspace and for the fuzz workspace, and what
  a build of the workspace actually compiled, and fails unless every `sha2` it finds has
  `force-soft` and lacks `force-soft-compact`. It proves its own judgment on planted
  records each run, so a pass is not a silent miss.
- **The README limitation.** Rewritten from the registry sources of the three crates as
  pinned, read again after both items are built.
- **Launch 4.4.** One step, h, after step g, in the ruling's words.
- **Launch 4.5.** Two insertions in `loop/STATE.md` and one in `loop/PACKETS.md`.

### 1.5 Predicted diff surface

- `loop/LEDGER.md` — done above; one short index of this ruling at closeout.
- **Launch 4.5:** `loop/PACKETS.md` (one sentence) and `loop/STATE.md` (two insertions).
- **Launch 4.4:** `loop/PROMPTS/AUTONOMOUS_BUILDER.md` (step h only).
- **Launch 4.3a:** `.cargo/config.toml`, `crates/cj3-types/build.rs`,
  `crates/cj3-types/src/lib.rs`, `scripts/ci/codec-fuzz-common.ps1`,
  `scripts/ci/check-arithmetic-backend.ps1`, and the step's comment in
  `.github/workflows/ci.yml`.
- **Launch 4.3b:** `crates/cj3-types/Cargo.toml`, one new check under `scripts/ci/`, and
  one step in `.github/workflows/ci.yml`.
- **The limitation:** `README.md`.
- **Launch 4.6:** `loop/reports/C12-p101-builder.md` (new) and a new directory under
  `bench/` with one standard-library script and the worked examples it writes.
- **Closeout:** this report, `loop/PACKETS.md`, `loop/STATE.md`,
  `loop/reports/BATCH-LOG.md` (new lines only), and the README sentences this ruling
  makes false.

Not predicted: any file under `spec/`; any source of `cj3-kernel`, `cj3-store` or any
other `cj3-*` crate than the two files of `cj3-types` named above; `Cargo.lock` or
`fuzz/Cargo.lock`; either `deny.toml`; `rust-toolchain.toml`; the Protocol Specification;
the Engineering Plan; any vector file; any earlier report or BATCH-LOG row.

### 1.6 Top risks, falsifier, confidence

1. **A word-size guard that passes while the library still picks by pointer width.** The
   library ignores a value it does not recognise. Control: an exact comparison on the
   variable the library reads, and three builds in D6 that must fail.
2. **A feature check that reads the wrong thing.** A resolved feature list is a union
   over the workspace; a build can compile a crate more than once. Control: the check
   reads the compiled units as well as the resolution, and proves its judgment on
   planted records.
3. **A FRAME that decides instead of proposing**, or a worked example that reads as a
   normative vector. Control: every decision is in one table with its status; every file
   under the new `bench/` directory carries a "proposed, not ruled" line; no tag is
   written in Rust.

**Falsifier:** this approach is wrong if any of the three new builds compiles; if the
existing vectors or tests change a single verdict under the pinned word size or the
portable hash; if either lockfile changes; or if the feature check passes on a planted
record that lacks the feature.

**Confidence:** HIGH on the text commits and the prompt step; MED-HIGH on 4.3a, which
repeats a pattern that already runs in D6; MED on 4.3b's check, whose build-reading half
is new; MED on the FRAME's completeness, which Al judges.

## Preflight evidence (prompt STEP 0; launch 4.1 and 1.1)

| Step | Result | Evidence |
|---|---|---|
| a. Path | PASS | `pwd -W` returned `C:/Users/LEET/COINjecture3.0`; no `OneDrive` segment |
| b. Remote | PASS | `git remote -v`: fetch and push are both `https://github.com/Quigles1337/COINjecture3.0` |
| c. Branch and tree | PASS | on `main`, `git status --porcelain` empty, `HEAD` = `origin/main` = `git ls-remote origin main` = `d2c5f75985c776ce1c9ef204c02481715f6fac99` after `git fetch` |
| d. Visibility | PASS | `gh repo view Quigles1337/COINjecture3.0 --json visibility` returned `PUBLIC` |
| e. Authorization | PASS | `loop/LEDGER.md` holds D17 RATIFIED (2026-08-15 overlay) |
| f. Capacity | PASS | `loop/STATE.md` carries the condition of Part 0.3 of the 2026-11-22 ruling, which holds until Al states another, and this ruling states none; `C:\Users\LEET\COINjecture2.0-network\loop\STATE.md` reads `CAPACITY_FLAG: none`, not `remediation-priority` |
| g. Survey and P-008 | PASS | `docs/RESEARCH_SURVEY.md` present; the P-008 URL is still `TBD(Al-supplied frontend URL)`, so P-008 stays blocked |
| Launch 1.1 | PASS | `gh pr view 22`: state `MERGED`, merged at `2026-11-23T12:44:59Z` by `Quigles1337`, merge commit `d2c5f75985c776ce1c9ef204c02481715f6fac99`, head `8bc958b70e694e5c1f0a87cabf4bc7d35ca6ba01`. `gh run view 45177244858`: workflow `D6 CI`, event `push`, that same commit, conclusion `success`, eleven jobs each `success` with five to eleven steps (<https://github.com/Quigles1337/COINjecture3.0/actions/runs/45177244858>). The run was in progress when first read. Nothing was written to the repository until it concluded. |
| The nightly run (launch 2.6, 4.4) | Reported; no STOP | The latest completed D6 Nightly run on `main` is `45117443199`: event `schedule`, commit `f601224db855e6cc69e840fb169d85df22c547b8`, conclusion `success`, 2026-11-23T05:23Z (<https://github.com/Quigles1337/COINjecture3.0/actions/runs/45117443199>). That commit is from before PR #22, so its `full-fuzz-corpus` job was still the deferral marker: five steps, 23 seconds, no target ran and none failed. **The long fuzz job has not yet run on `main`.** Its first scheduled run there is the one of 2026-11-24, 05:17 UTC. The only long run so far is `45121770277`, dispatched on the PR #22 branch at `169d808`, where all six targets passed (report C11 §3.3). |
