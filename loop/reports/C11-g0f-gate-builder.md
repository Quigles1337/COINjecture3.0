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
