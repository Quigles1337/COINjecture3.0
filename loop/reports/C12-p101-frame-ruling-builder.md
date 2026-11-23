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

## 2. BUILD

### 2.1 What was built, commit by commit

| Commit | Clause | What it holds |
|---|---|---|
| `359a1ae` | launch 4.2 | The ruling, word for word, in `loop/LEDGER.md`. Nothing else. |
| `a09a510` | prompt, move 1 | The FRAME above and the preflight evidence. |
| `b59f0fe` | launch 4.5 | Text only: the meaning of the D10 sentence beside its two quotes (2.1), and the date on one bullet of `loop/STATE.md` (2.9). |
| `16b8fdf` | launch 4.4 | Step h in the builder prompt. Nothing else in that file. |
| `966e9c2` | launch 4.3a | The word size pinned to 64: `.cargo/config.toml`, the refusal in `cj3-types` (`build.rs`, `src/lib.rs`), the fuzz gates' flags, three new failing builds in the D6 check, the step's comment in `ci.yml`. |
| `8c6fd8b` | launch 4.3b | `force-soft` in the manifest of `cj3-types`; the new check `scripts/ci/check-sha2-portable.ps1`; its step in `ci.yml`. |
| `39a080f` | launch 4.3 | Text only: the README limitation. |
| `64e6b92` | launch 3.3f | `bench/p101-roots`: the standard-library script and the two files it writes; one line in `.gitattributes`. |
| `c1673fa` | launch 4.6 | P-101's FRAME, `loop/reports/C12-p101-builder.md`. |
| `0ab81e9` | launch 4.3b | A second, small commit for item (b), from the adversary pass (§4.1, finding 3). |
| the closing commit | prompt | The index after the verbatim text in the LEDGER; `loop/PACKETS.md`, `loop/STATE.md`; the README rows this ruling makes false; this report from §2 on; two new BATCH-LOG lines. |

Item (b) is in two commits, not one, as item (c) was in the last session (launch 2.11).
No commit holds work of two items.

### 2.2 Tripwires

- **SCOPE: held, with one file beyond the prediction.** The files of
  `git diff --stat main` are inside §1.5 except `.gitattributes`, which gained one line
  so that the two generated example files stay byte-identical to what the script writes
  on every platform, as the other `bench/` directories already have. In the README more
  than the limitation changed; each is a sentence this ruling makes false, and each is
  listed in §4.2 point 9. Nothing under `spec/`, `crates/cj3-kernel` or
  `crates/cj3-store` changed. Neither lockfile, neither `deny.toml`,
  `rust-toolchain.toml` and nothing under `docs/` changed. No Rust source names a
  proposed tag.
- **INVENTION: held.** No protocol value is written. P-101's FRAME proposes and assigns
  nothing. The account values in the worked examples are test values and are labeled so
  in the script and in both outputs.
- **REPETITION: not tripped.**
- **INTERPRETATION: not tripped.** The places where clauses had to be fitted together
  are in §1.2. One gap between the specification and the Lean model turned up while the
  FRAME was written, and it is put to Al there as decision K-7, not decided.
- **ENVIRONMENT: one slip.** I pushed the branch before reading the visibility again.
  I read it directly after: `PUBLIC`, and the COINjecture 2.0 loop still read
  `CAPACITY_FLAG: none`. Nothing was exposed that should not have been, but the prompt
  asks for the reading before the push, and I had it in the wrong order.
- **DEGRADATION: three slips, each caught, none shipped as wrong.**
  1. In the FRAME I typed one number, a nonce in the state example, from memory of the
     script's output. It was wrong. I then checked every hex string of eight or more
     digits and every number of ten or more digits in the FRAME against the script's
     outputs and the normative vectors by program: 58 hex strings, all found; the one
     wrong number, corrected. The two 188-byte transactions in the FRAME are equal to
     the vectors' byte for byte.
  2. I fed two edits to Python through a shell heredoc, which the notes on this machine
     warn against. The first went through; I kept it after reading its diff and scanning
     the file for control bytes (none). The second failed outright on an apostrophe and
     changed nothing, and I made it and the later edits from script files.
  3. My first planted test of the hash check changed nothing in the manifest, because a
     path with backslashes was mangled on its way in, and the check passed. The pass
     was suspicious on its face, the printed manifest showed the plant had not happened,
     and the test was redone (§3.2).

### 2.3 What changed on this machine outside the repository

- Temporary build directories were made and removed by the check scripts. Two probe
  directories under the user's temporary directory were removed after use.
- `Cargo.lock` was rewritten twice for the planted tests of §3.2 and restored from git
  each time. `git diff` was byte-identical before and after the planted tests.
- Read-only: the registry copies of `curve25519-dalek 4.1.3`, `sha2 0.10.9`,
  `keccak 0.1.6`, `sha3 0.10.9` and `subtle 2.6.1`, to state what the README says about
  them. `#print axioms` was run in the built Lean project from a file outside the
  repository; `spec/` has no change.
- Nothing was installed.

## 3. VERIFY

### 3.1 Launch 4.3a: the word size

- **What the library did before.** With the setting at `32`, on this 64-bit host,
  `cj3-types` built without complaint before this change: the library's build record
  read `cargo:rustc-cfg=curve25519_dalek_bits="32"`. With both `32` and `64` given, the
  library failed to compile (`error[E0659]: … is ambiguous`, both bodies of code
  switched on).
- **After.** The compiler invocations of a real build, read with `cargo check -v`:
  `curve25519_dalek`, `sha2` and `cj3_types` are each compiled with
  `--cfg 'curve25519_dalek_backend="serial"' --cfg 'curve25519_dalek_bits="64"'`, and
  the library's build record reads `curve25519_dalek_bits="64"` and
  `curve25519_dalek_backend="serial"`.
- **The check**, here and in D6, run as a workflow step runs it (exit code 0):

  ```text
  ARITHMETIC_BACKEND_CASE=configuration-file EXPECTED=builds RESULT=PASS
  ARITHMETIC_BACKEND_CASE=rustflags-serial-64 EXPECTED=builds RESULT=PASS
  ARITHMETIC_BACKEND_CASE=rustflags-without-either-setting EXPECTED=does-not-compile REASON=cj3-types-refuses-the-back-end-and-the-word-size RESULT=PASS
  ARITHMETIC_BACKEND_CASE=backend-absent EXPECTED=does-not-compile REASON=cj3-types-refuses-the-back-end RESULT=PASS
  ARITHMETIC_BACKEND_CASE=backend-simd EXPECTED=does-not-compile REASON=cj3-types-refuses-the-back-end RESULT=PASS
  ARITHMETIC_BACKEND_CASE=backend-fiat EXPECTED=does-not-compile REASON=cj3-types-refuses-the-back-end RESULT=PASS
  ARITHMETIC_BACKEND_CASE=backend-serial-and-simd EXPECTED=does-not-compile REASON=the-library-does-not-compile RESULT=PASS
  ARITHMETIC_BACKEND_CASE=bits-absent EXPECTED=does-not-compile REASON=cj3-types-refuses-the-word-size RESULT=PASS
  ARITHMETIC_BACKEND_CASE=bits-32 EXPECTED=does-not-compile REASON=cj3-types-refuses-the-word-size RESULT=PASS
  ARITHMETIC_BACKEND_CASE=bits-32-and-64 EXPECTED=does-not-compile REASON=the-library-does-not-compile RESULT=PASS
  ARITHMETIC_BACKEND=PASS backend=serial bits=64 builds=2 refusals=8
  ```

  The three builds the clause names are the last three. Each failing case must show
  exactly the refusals it names, each by its own message, so a failure is not credited
  to the wrong setting.
- **The fuzz gates.** In D6 run `45179918980` the smoke job printed
  `FUZZ_RUSTFLAGS=--cfg curve25519_dalek_backend="serial" --cfg curve25519_dalek_bits="64"`
  and `CODEC_FUZZ_SMOKE=PASS TOOLCHAIN=nightly-2026-11-15 TARGETS=6 TOTAL_RUNS=740300
  SEED=20261122`. The nightly long job uses the same shared script and has not run with
  the new flag; its first run with it is the first scheduled run after this merges.
- `cargo doc --package cj3-types` builds, so the rustdoc flags carry both settings.

### 3.2 Launch 4.3b: `sha2`

- **The check**, here and in D6:

  ```text
  SHA2_PORTABLE_PLANTED=PASS accepted=2 rejected=6
  SHA2_PORTABLE_REQUEST=PASS crate=cj3-types dependency=sha2 features=force-soft
  SHA2_PORTABLE_RESOLVED=PASS scope=workspace sha2=0.10.9 features=force-soft
  SHA2_PORTABLE_RESOLVED=PASS scope=fuzz sha2=0.10.9 features=force-soft
  SHA2_PORTABLE_COMPILED=PASS scope=workspace units=1 features=force-soft
  SHA2_PORTABLE=PASS crate=sha2 feature=force-soft
  ```

- **It fails when it should.** Three changes, each made to the working tree, run, and
  undone; `git diff` was byte-identical before and after:

  | Planted | Where the check stopped | Its words |
  |---|---|---|
  | The feature removed from the manifest of `cj3-types` | the request | `cj3-types' manifest: a sha2 lacks the feature force-soft (it has: ).` |
  | A second workspace crate asks for `sha2` with `force-soft-compact` | the resolution | `The workspace resolution: a sha2 has the feature force-soft-compact, which overrides force-soft.` |
  | `sha2` added as a build dependency of another crate, without the feature | the compiled units | `The build of the workspace tests: a sha2 lacks the feature force-soft (it has: ).` |

  In the third case the resolution passed, because a resolved feature list is a union,
  and only the reading of the compiled units saw the second `sha2`. That is the case the
  third reading exists for.
- The second and third plants cannot happen silently in any case: each needs a new line
  in `Cargo.lock`, and every D6 command runs with `--locked`. I updated the lockfile on
  purpose for the demonstration and restored it.
- **Neither lockfile changed**, and no dependency was added: a lockfile does not record
  features.
- **Every vector is unchanged under the portable code.** `derive.py --check` prints
  `P007_VECTORS=PASS files=16`, with the same three whole-file digests report C11 gives
  (`f5061cef…17d855`, `4165807f…a90156`, `161a4b02…bc27b7`). The 104 workspace tests
  pass, among them the 932 Ed25519 verdicts and every worked vector.

### 3.3 The README limitation

Each statement was read from the pinned source in the registry: the cfg ladder of
`sha2-0.10.9/src/sha256.rs` and `sha512.rs` (`force-soft-compact`, then `force-soft`,
then the processor paths); `curve25519-dalek-4.1.3/build.rs` (the word size, and that an
unrecognised value falls back to the pointer width); `keccak-0.1.6/src/lib.rs` (another
implementation only under `feature = "asm"` on `aarch64`). cargo's resolution shows
`keccak 0.1.6 features=[]` and `sha3 0.10.9 features=[]`. The `unsafe` count was redone
with a search of the same sources: one volatile read in
`backend/serial/u64/scalar.rs`; two blocks in `subtle`; one slice cast each in
`sha256.rs` and `sha512.rs`; the files that hold `sha2`'s processor intrinsics are not
compiled under `force-soft`.

### 3.4 P-101's FRAME and its worked examples

- `python -I -B bench/p101-roots/derive.py --check` prints
  `P101_ROOTS=PASS files=2 self_checks=171 status=proposed-not-ruled`. The two outputs:
  `WORKED-EXAMPLES.md`, SHA-256 `4d2e28e6…e31beb`, and `p101-roots.examples.txt`,
  SHA-256 `78fd1552…9e3ea1`.
- Six digests of the examples were recomputed with a second tool, `xxd -r -p |
  sha256sum`, from the preimages in the examples file: the empty `tx_root`, the worked
  `tx_root`, one account leaf, two state nodes and the state commitment. All matched.
- Every hex string and long number in the FRAME was compared with the script's outputs
  by program (§2.2).
- The script is not wired into D6 (launch 3.3f), so nothing but a person running
  `--check` notices if the examples and the script drift apart.

### 3.5 Local checks at the last code commit

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --all-targets --all-features --locked` | PASS, 104 tests |
| `python -I -B bench/p007-vectors/derive.py --check` | `P007_VECTORS=PASS files=16` |
| `scripts/ci/check-arithmetic-backend.ps1`, as a workflow step | PASS, exit 0, lines in §3.1 |
| `scripts/ci/check-sha2-portable.ps1`, as a workflow step | PASS, exit 0, lines in §3.2 |
| `scripts/ci/check-source-policy.ps1` | `DOMAIN_TAG_POLICY=PASS tags=7 …`, `SOURCE_POLICY=PASS files=29 crates=10` |
| `cargo doc --package cj3-types --no-deps --locked` | builds |

Not run on this machine this session, and taken from the hosted run instead: both
`cargo deny` runs, both `cargo audit` runs, the zero-unsafe scan, the Lean gate and the
fuzz targets.

### 3.6 Origin

The exact-head run for the final head is recorded on
[PR #23](https://github.com/Quigles1337/COINjecture3.0/pull/23). This file cannot hold
the run of the commit that contains it, so it makes no green claim for that head.

One earlier head can be cited:

| Commit | Run | Result |
|---|---|---|
| `39a080f` (both build items and the README limitation) | D6 [`45179918980`](https://github.com/Quigles1337/COINjecture3.0/actions/runs/45179918980) | **Success**, all eleven jobs, each with five to eleven steps, at its first attempt. |

Read from its log with `gh run view --log`: the lines of §3.1 and §3.2 exactly as
above; `P007_VECTORS=PASS files=16`;
`FUZZ_DENY_POLICY=PASS root_lines=37 fuzz_lines=38 added=1 exception=libfuzzer-sys@0.4.13:NCSA`;
`DOMAIN_TAG_POLICY=PASS tags=7 read_outside_cj3_types=24 planted_flagged=4 planted_passed=3`;
`SOURCE_POLICY=PASS files=29 crates=10`; `GEIGER_POLICY=PASS packages=11 unsafe_count=0`;
`P005_RATIFIED_SPEC_BUILD=PASS LEAN=4.33.0 VECTORS=27
SHA256=30CABF852D623549CD5293628D5B3899BE805D543B537CB223F1B6FAB5C324E1`. The commits
after `39a080f` add the example script, which D6 does not run, one flag in the hash
check (`0ab81e9`), and documents. `conservation-invariant` and `genesis-spend-test` are
still `NOT_YET_ADMITTED`: their green is a deferral marker, not a test result.

## 4. ADVERSARY PASS

Seat switched. I re-read the whole diff against `main` as someone trying to make two
honest nodes run different code, slip a build past a gate, or make this record or the
FRAME claim more than the ruling gives.

### 4.1 Findings

1. **The word-size pin was needed.** Before it, nothing stopped a build with the 32-bit
   arithmetic on any host. The vectors would very likely still have passed under it,
   which is the point: two bodies of code that agree on every test are still two bodies
   of code.
2. **The guard judges the flags `cj3-types` is built with, not the library's.** cargo on
   the stable toolchain gives every crate of a build the same flags, so the two agree.
   A build system that is not cargo, a compiler wrapper, or cargo's unstable per-package
   flags on a nightly toolchain could give the library other flags than `cj3-types`.
   The same was true of the back-end pin (report C11, finding 5). I did not try one.
3. **The hash check read cargo's resolution with default features.** The builds switch
   every feature on. Fixed in `0ab81e9`: the resolution is read with every feature on.
   No workspace member has a feature today, so the result did not change.
4. **`force-soft-compact` would silently defeat `force-soft`.** The library tests it
   first. The check fails on it. The clause names only `force-soft`; the stricter check
   is listed for Al (§4.2 point 2).
5. **The hash check reads the workspace's test build.** A build of another shape, a
   release build of a future node or a build of one package, is not read. Features only
   add, and `cj3-types` asks for `force-soft` without condition, so every `sha2` that
   `cj3-types` links has it in any build. What another shape could add is only
   `force-soft-compact`, through a dependency that is not compiled for tests, and the
   resolution reading covers dependencies whatever they are compiled for.
6. **SHA-3 is unpinned.** It runs one body of code on every processor as built, but no
   gate fails if `keccak`'s `asm` feature is enabled later. The README says so.
7. **`cpufeatures` is still compiled.** `sha2` and `curve25519-dalek` each keep it as a
   dependency on x86_64. Under the two pins neither calls it. It is in the lockfile as
   before.
8. **No 32-bit target is built anywhere.** That the 64-bit code compiles and reproduces
   the vectors on a 32-bit target follows from the library's own selection and is not
   shown. The README says so.
9. **Step h adds a pointer to its clause** beyond the ruling's words, as step d has one.
   The rest of the step is the ruling's wording.
10. **The examples are not checked by D6.** Launch 3.3f makes the script the only added
    code, so no workflow step runs it. A later edit to one without the other would go
    unnoticed until someone runs `--check`.
11. **The FRAME's worked examples could be mistaken for vectors.** Each of the three
    files says "proposed, not ruled" in its first lines, and the README row and
    `loop/PACKETS.md` say it again. The script's digest check ties the examples to one
    exact version of the normative vector file: a change to that file makes the script
    refuse to run.
12. **The FRAME found a gap it does not own**: nothing in Protocol Spec §8 or in the
    Lean model checks `tx_root` against the body (decision K-7). It is put to Al, not
    logged in `loop/reports/SPEC-ISSUES.md`, which this session's scope does not reach.
13. **Panic paths and unsafe.** `crates/cj3-types/build.rs` has no `unwrap` or `expect`
    and forbids unsafe code. No library code changed beyond one `compile_error!`.
    `unsafe` in `cj3-*` crates is zero in 11 packages (D6 run `45179918980`).

No Critical. Nothing found needed a STOP.

### 4.2 Points for Al — noticed, recorded, not decided

1. **P-101's FRAME** is the main thing for Al to rule: twelve decisions, K-1 to K-12, in
   `loop/reports/C12-p101-builder.md`, section 3. Its own section 15 lists seven points
   beyond them. Three to read first: K-7 is a gap in the record (finding 12); "prove" in
   Protocol Spec §8 is read as a proof about a Lean model, with the Rust store tied to
   it by tests (K-4); and the proposed state commitment is less common than its nearest
   alternative, which the FRAME names.
2. **The hash check is stricter than the clause.** It also fails on
   `force-soft-compact` (finding 4). If Al wants the clause's letter only, it is two
   lines to remove.
3. **SHA-3** (finding 6). One more judgment in the same check would make D6 fail if
   `keccak`'s `asm` feature were ever enabled. It is not done: no clause asks for it.
4. **A nightly failure that is neither of step h's two cases.** A run that fails after
   a target has started, for a reason that is not a target: the job's 120-minute limit,
   a lost runner. Step h's text does not say whether that is a STOP. I would stop and
   ask.
5. **The long fuzz job has not yet run on `main`**, with or without the new flag. Its
   first scheduled run is that of 2026-11-24, 05:17 UTC. I did not dispatch one: no
   clause asks for it. The next session's step h reads it.
6. **Where the date went in launch 2.9.** "P-005's builder tail is now explicit, as of
   2026-08-16: …". The line is now longer than its neighbours, because re-wrapping would
   have changed something else in the bullet.
7. **A third place in `loop/STATE.md` spoke of D10 and D16.** It restated Part 0.2 of
   the 2026-11-22 ruling as "D10 and D16 are not ruled", without that ruling's "here".
   It is not a quote of the loose sentence, so launch 4.5's commit left it. At closeout
   I changed it to "are not ruled in that ruling (they stand as ruled on 2026-08-15;
   launch 2.1)". If Al wants it as it was, it is one line.
8. **The README limitation is in a commit of its own**, after both build items, not
   inside either (§1.2).
9. **Edits no clause named.** Each exists because this ruling makes a sentence false, or
   because the prompt requires the file to be exact at a STOP:
   - the index after the verbatim text in `loop/LEDGER.md`, a new section; no earlier
     index line changed;
   - `loop/PACKETS.md`: the queue status, the order of work, the citation note, the rows
     and sections of P-101 and packet P-011 (launch 2.10 is recorded in the latter), and
     one line of the "Carried" list;
   - `loop/STATE.md`: the head lines, new bullets for this ruling, "Next action", the
     sentence of point 7, and the tense of one sentence in the bullet on pass 4.5c,
     which said the README "says" what this ruling's build items have made untrue;
   - in the README, beyond the limitation: the "Phase" and "Current stop" rows of the
     status block and the line above them, the G1 row of the roadmap, the `scripts/ci/`
     row of the repository map, and a new row for `bench/p101-roots/`;
   - one line in `.gitattributes`;
   - comments in `.cargo/config.toml`, `crates/cj3-types/build.rs`,
     `scripts/ci/codec-fuzz-common.ps1` and `ci.yml` that spoke of one setting where
     there are now two.
10. **Stale text left.** `spec/README.md` still says of P-101 "it has not started". Its
    build has not; its FRAME is written. I left the sentence, because I had said in §1.3
    that no file under `spec/` would change in this session. It is one sentence.
11. **Item (b) is in two commits** (§2.1).
12. **The order of my first push and the visibility reading** (§2.2).

### 4.3 The record against the ruling

| Clause | Where it is recorded | More or less than the clause? |
|---|---|---|
| 1.1 | LEDGER index; PACKETS; STATE; README status block | Neither. |
| 2.1 | PACKETS, "Carried"; STATE, the bullet that quotes pass 3.5; LEDGER index | The meaning is in the clause's own words in each place. The LEDGER index carries it in a new entry; no earlier index line changed. STATE has one more sentence changed (§4.2 point 7). |
| 2.2, 4.3a | `.cargo/config.toml`, `crates/cj3-types`, the D6 check; STATE; README limitation | The check also attributes each refusal to its setting, which the clause does not ask. |
| 2.3, 4.3b | `crates/cj3-types/Cargo.toml`, the new check; STATE; README limitation | Stricter: the check also fails on `force-soft-compact`. The README quotes the sentence on code that mines. |
| 2.4 | LEDGER index; STATE | Neither. |
| 2.5 to 2.8, 2.11 | LEDGER index; STATE | Shortened. The verbatim text governs. |
| 2.9 | STATE | Neither: the quoted words, and nothing else in the bullet. |
| 2.10 | PACKETS, packet P-011, table row and section; STATE; LEDGER index | The clause's two sentences, whole, in the section. |
| 3.1 | PACKETS; STATE; README; P-101's FRAME, sections 1 and 16 | Adds "no kernel code exists", which is the fact. |
| 3.2 to 3.6 | P-101's FRAME | Al judges whether it is complete. It lists twelve open decisions, where the gate ruling named two. |
| 4.3, the limitation | README | Adds what stays true of `unsafe` code in the dependencies, recounted, because the earlier text said it and one part of it changed. |
| 4.4 | The builder prompt, step h | Adds a pointer to the clause (finding 9). |
| 4.5 | Commit `b59f0fe` | Neither. |
| 4.6 | `loop/reports/C12-p101-builder.md` | Neither. |

### 4.4 A1–A11, banned identifiers, unsafe, TBD integrity

| Axiom | What I looked for | Result |
|---|---|---|
| A1–A4 | A consensus structure, instance input, scoring input or fork-choice input changed | None. No codec, preimage, checker or rule changed. The same bytes hash to the same digests under the portable code: every vector is unchanged. |
| A5 | A float or an amount path | None. |
| A6 | A validity check moved or merged | None. |
| A7 | A new gate that can pass by doing nothing | Each fails closed: the word-size refusal; the hash check on "no sha2 found"; its planted records. One planted test of mine did pass by doing nothing, and I caught it (§2.2). |
| A8 | Rust or prose deciding what Lean owns | No file under `spec/` changed. The FRAME proposes Lean content and adds none. |
| A9 | The trusted base | No dependency added or changed. Less code can now run: one word size, one hash implementation. `unsafe` in `cj3-*`: zero. |
| A10 | A claim beyond the evidence | The README says what is not shown (no 32-bit target is built) and what is not enforced (SHA-3). The FRAME says what was not measured and what is cited from memory. |
| A11 | A verification claim without a pointer | Each claim in §3 names its command or run. |

The source-policy gate passed over 29 Rust files, so none of `solve_time`,
`work_score`, `reported_*` or `self_reported` is present. P-1, P-4, P-7, P-9, P-10, P-12
and `ACTIVE_SIZE_PARAM` are unfilled. No tag is assigned and no Rust source names a
proposed one.

## 5. HANDOFF — no MERGE

[PR #23](https://github.com/Quigles1337/COINjecture3.0/pull/23) carries this change. Its
description holds the exact-head D6 run. It is marked ready and left unmerged. Launch
4.7 keeps the review, the merge and the ruling with Al. P-101's build is not begun.

## 6. CALIBRATE

- **Diff surface:** as predicted, plus one line in `.gitattributes` and more README rows
  than I named.
- **Risk 1, a word-size guard that passes on the wrong code:** did not materialize. The
  probe before the change showed the risk was real: a build at 32 compiled.
- **Risk 2, a feature check that reads the wrong thing:** materialized once in the
  check, where the adversary pass found the resolution read with default features, and
  once in my own test of it, which planted nothing. Both are fixed. The planted second
  `sha2` showed that the third reading earns its place.
- **Risk 3, a FRAME that decides:** did not materialize as far as I can judge; Al judges
  it better.
- **Confidence against result:** HIGH on the text commits held. MED-HIGH on 4.3a was
  about right: green in D6 at its first hosted run. MED on the hash check was about
  right too. MED on the FRAME's completeness stands: K-7 turned up while I wrote it,
  which is evidence for the method and against the idea that the list is closed.
- **Surprises:** that the 32-bit code could be built silently; that a resolved feature
  list hides a second copy of a crate; how much longer the FRAME is than I predicted.
- **One process improvement:** copy no number or digest by hand into a report. Paste it
  from the tool's output, and check the finished file against that output by program
  before committing, as I did only after the first mistake.

## VERIFIED

- Preflight steps a–g, launch 1.1 and the nightly reading: the table above.
- The two build items: §3.1, §3.2 and §3.6.
- What the README limitation says: §3.3.
- The worked examples and every digest the FRAME quotes: §3.4.
- Format, lints, tests, the vector derivation and the three policy scripts at the last
  code commit: §3.5. The other D6 gates: the hosted run of §3.6.

## ASSUMED

- cargo on the pinned stable toolchain gives every crate of a build the same flags.
  Safe: it is cargo's documented behaviour for `[build] rustflags` and for `RUSTFLAGS`,
  and the verbose build of §3.1 shows it for the three crates that matter.
- A citation prefix of my choosing, "launch", is acceptable. Safe: it is recorded in the
  LEDGER index, as "pass" and "resume" were, and Al can change it in one place.
- Correcting a sentence the ruling makes false is within the session. Safe: resume 1.2,
  pass 2.8 and launch 2.8 confirmed the same practice, and each edit is listed for Al to
  strike.
- The report numbers are C12, after C11.

## UNKNOWN

- How Al rules the twelve decisions of P-101's FRAME.
- Whether the 64-bit curve code compiles and reproduces the vectors on a 32-bit target.
  A build for such a target would resolve it; no D6 job builds one.
- What the first nightly long run on `main` finds. The next session's step h reads it.
- The D6 result for the final head: recorded on PR #23 once the run completes.
