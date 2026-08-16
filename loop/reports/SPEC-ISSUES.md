# Protocol specification issues

This file records nonblocking Engineering Plan versus Protocol Spec interpretation
conflicts discovered under the autonomous builder's INTERPRETATION tripwire. The
builder proceeds on the stricter higher-authority reading; only the owning HUMAN gate
may change normative specification text.

## SI-001 — SIS reduction described as unconditional proof

- **Discovered by:** P-003, 2026-08-15
- **Status:** OPEN — G0/HUMAN
- **Higher authority:** `docs/ENGINEERING_PLAN.md` A10 requires hardness assumptions
  to be labeled as assumptions in code, documentation, and public material.
- **Conflicting text:** `docs/PROTOCOL_SPEC.md` §5.2 says the sampled SIS
  distribution's hardness is “provable, not assumed.”
- **Why this conflicts:** Ajtai's result and later Micciancio–Regev/GPV refinements
  are conditional worst-case-to-average-case reductions under asymptotic parameter
  constraints. They do not prove the source worst-case lattice problems hard, do not
  automatically cover an arbitrary concrete `(n, m, q, beta)` tuple, and do not prove
  any concrete instance takes a stated wall-clock duration. The current lattice
  estimator additionally labels its operation estimates as heuristic models.
- **Strict reading used by P-003:** call SIS's basis a conditional
  worst-case-to-average-case reduction; state the parameter conditions being relied
  upon; separately label concrete estimator output and measured solver cost; never
  claim unconditional or per-instance proven hardness.
- **Resolution required:** at G0, replace the unconditional wording with an A10-
  compliant conditional statement and review P-003's parameter evidence. No P-003
  code depends on changing the spec text.
- **Evidence:** M. Ajtai, *Generating Hard Instances of Lattice Problems* (STOC
  1996/ECCC TR96-007); D. Micciancio and O. Regev, *Worst-case to Average-case
  Reductions based on Gaussian Measures* (SIAM J. Comput. 2007); C. Peikert,
  *A Decade of Lattice Cryptography* (FnT TCS 2016); pinned `malb/lattice-estimator`
  commit `3e48ef421ec256afddb3e7d2249a77eab6e9ba12` and its README/model docs.

## SI-002 — SIS coefficient bound is referenced but not defined

- **Discovered by:** P-003, 2026-08-15
- **Status:** OPEN — G0/P-007 HUMAN
- **Text:** `docs/PROTOCOL_SPEC.md` §5.2 requires every decoded coefficient to obey
  `|s_i| <= s_max(P-4)`, but P-4 contains only `(n, m, q, beta_squared)` and no
  definition of `s_max` appears elsewhere in the specification.
- **Strict reading used by P-003:** before accumulating the global squared norm,
  reject any coefficient whose square exceeds `beta_squared`. This is a necessary
  consequence of the already normative global Euclidean bound and therefore does not
  alter the valid-solution set or invent another parameter. Canonical coefficient
  width and decoding remain absent from P-003.
- **Resolution required:** P-007/G0 must define the canonical signed-integer width and
  either define `s_max` explicitly or replace the check with the equivalent derived
  per-coefficient guard. No consensus byte encoding is inferred here.
