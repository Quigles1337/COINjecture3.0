# P-004 admission bench

This directory contains a class-neutral, safe-Rust admission controller and the
read-only COINjecture 2.0 calibration driver. It is offline analysis tooling, not a
node dependency, class registry, or admission decision.

## Evidence contract

Each record keeps these questions independent:

1. Is there a published hardness argument for the sampled distribution?
2. Is there an attack-derived parameter floor?
3. Does instance generation fit A1/A2's protocol-derived shape?
4. Is checking a deterministic, bounded, exact A3 function?
5. Did the maximum observed checker cost remain within P-003's provisional 15 ms
   recommendation on the recorded host?

`eligible_for_adr_review` means only that the machine-evaluable questions passed. It
never adds a class, assigns a `CLASS_ID`, ratifies a parameter, or bypasses the
required human ADR.

## Reproducing the legacy calibration

The driver compiles in an isolated temporary Cargo project with path dependencies on
an exact, hash-verified 2.0 checkout. It directly invokes 2.0's public
`solve_problem_blocking` and `Solution::quality` implementations, then sends the
observations through the generic controller. It never edits or links 2.0 into CJ3.
The committed fixtures deterministically select the three legacy solver/checker paths;
they are not samples from the active 2.0 generator and cannot support a hardness
claim. Generator/distribution findings come from exact source inspection instead.

```powershell
./bench/p004-admission/run-legacy-calibration.ps1 `
  -LegacyRoot C:\Users\LEET\COINjecture2.0-network
```

The committed `legacy-driver-Cargo.lock` pins the isolated driver's transitive graph.
`-BootstrapLock` is only for deliberately refreshing that lock after reviewing the
exact 2.0 revision and source hashes embedded in the script.

Outputs are written under `evidence/`:

- `legacy-results.jsonl` — stable-schema machine evidence;
- `LEGACY-CALIBRATION.md` — report derived from the same in-memory records;
- `ENVIRONMENT.md` — host, toolchain, revisions, source pins, and output hashes.

Checker calls are timed in bounded batches (10,000 calls per sample by default), and
the JSONL summaries report an integer per-operation average. This avoids treating
sub-clock-resolution single calls as zero-cost checks while keeping the batch size
explicit and reproducible.

The active 2.0 mining path has three executable variants: SubsetSum, SAT, and TSP.
GraphColoring, Factorization, and SVP are registry descriptors whose optional
generation/check methods remain unimplemented. The Custom variant is an opaque user
payload, not a concrete problem class. The harness records all seven surfaces and
does not invent executable behavior for the four non-runnable entries.
