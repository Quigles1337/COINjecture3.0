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

## SI-003 — SIS SHAKE expansion lacks a normative candidate-byte convention

- **Discovered by:** P-003 adversary pass, 2026-08-16
- **Status:** OPEN — G0/P-007 HUMAN
- **Text:** `docs/PROTOCOL_SPEC.md` §5.2 requires SHAKE-256 plus rejection sampling
  to derive the matrix, but it does not define the candidate word width, byte order,
  exact acceptance ceiling, or whether rejected candidates consume a complete fixed-
  width chunk before the next candidate is read.
- **Why this matters:** independent implementations can all use unbiased rejection
  sampling yet derive different matrices from the same seed. Once the class is wired
  into consensus, that ambiguity could split validation.
- **Strict reading used by P-003:** the safe-Rust prototype reads four-byte little-
  endian candidates and accepts `word < floor(2^32/q) * q`. Its independent fixture
  and unit test are explicitly nonnormative implementation evidence; no P-007 wire or
  consensus rule is claimed.
- **Resolution required:** P-007/G0 must ratify exact XOF consumption, candidate width,
  byte order, rejection ceiling, and canonical test vectors before the SIS class can
  enter a consensus registry. If G0 chooses another convention, update the prototype
  and fixture together before admission.

## SI-004 — §8 conservation target conflicts with §11 quality-scaled reward credit

- **Discovered by:** P-005 resumed HUMAN implementation, 2026-08-16
- **Status:** OPEN — Al (+ Sarah at D16 reveal) / HUMAN; blocks P-005 completion
- **Text A:** `docs/PROTOCOL_SPEC.md` §8 step 3 requires the STF to apply §11 reward to
  `miner_addr`; §11 defines
  `reward(height, Q) = subsidy(height) · min(Q, R_MAX·SCALE) / SCALE`.
- **Text B:** §8's conservation invariant requires
  `Σ balances(post) = Σ balances(pre) + subsidy(height)` for every applied block and
  states that fees transfer rather than mint.
- **Why this conflicts:** valid solutions have `Q ≥ SCALE`, and neither the current
  prose nor an unfilled owner value requires `Q = SCALE` or `R_MAX = 1`. Therefore
  §11 can direct a miner credit greater than `subsidy(height)`, while the §8 theorem
  permits total issuance to increase by exactly the subsidy. No premium-funding or
  debit account is specified, so both statements cannot hold for the general case.
- **Strict reading used by P-005:** stop before choosing the theorem target or reward
  funding semantics. Keep `reward`, `subsidy`, `R_MAX`, and any relationship between
  them abstract. Do not equate reward with subsidy, force an owner value, mint the
  quality premium, or invent a funding pool through Lean code or fixtures.
- **Resolution required:** the HUMAN owner must ratify one coherent issuance model
  and amend the affected §8/§11 prose before P-005 can complete. Examples of the
  decision surface—not recommendations—include whether total issuance tracks the
  full quality-scaled reward, whether the premium is funded by an explicit debit,
  or whether the multiplier is constrained so reward equals subsidy.
- **P-005 impact:** partial draft `Spec/Tx.lean` and `Spec/Stf.lean` candidates are
  checkpoint evidence only. PR #9 remains draft, Lean CI is not admitted, no JSON
  vector artifact is ratified, and the PR cannot be marked ready-for-review.
