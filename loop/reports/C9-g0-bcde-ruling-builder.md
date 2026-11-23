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
