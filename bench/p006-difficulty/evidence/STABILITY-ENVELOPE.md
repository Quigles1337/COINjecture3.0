# P-006 stability envelope

**NON-NORMATIVE RESEARCH FIXTURE — HUMAN/G0 RATIFICATION REQUIRED**

This generated report compares hypothetical, dimensionless controller
coordinates. It does not fill Protocol Spec P-1, P-2, or P-11, and it does
not authorize Phase-2 implementation. All protocol selection remains HUMAN/G0.

## Experiment contract

- Candidate coordinates: 36.
- Fixed sensitivity runs per candidate/scenario: 24.
- Normalized target interval: 1.0; no seconds or absolute P-1 value is assigned.
- Hash observation: inter-block interval EMA only.
- Size observation: synthetic checker-derived quality margin only.
- Required scenarios: stationary, 4x/0.25x hash steps, 4x/0.25x solve
  steps, opposed shocks, 64-block opposed oscillation, and 35%/51%
  strategic quality-margin suppression.

## Predeclared envelope

A coordinate passes the honest/unmanipulated-quality envelope only if every
fixed-seed/sensitivity run stays within the configured tail-interval,
controller-state, recovery,
and excursion bounds. The adversarial envelope additionally requires at least
80% instance-size retention.
The exact thresholds are preserved in `config.json` and `results.json`; they
are research acceptance rules, not consensus constants.

## Result

- Honest/unmanipulated-quality envelope passes: **0 / 36**.
- Adversarial-envelope passes: **0 / 36**.
- Restricted hash-only envelope passes: **36 / 36**.
- Unique restricted hash-loop settings passing: **6 / 6**.

### Scenario pass counts

| scenario | schedule class | passing coordinates |
|---|---|---:|
| stationary_honest | honest_baseline | 36 / 36 |
| hash_step_up_4x | honest_step | 36 / 36 |
| hash_step_down_4x | honest_step | 36 / 36 |
| solve_step_up_4x | honest_step | 0 / 36 |
| solve_step_down_4x | honest_step | 0 / 36 |
| opposed_hash_up_solve_down | adversarial_power_schedule | 0 / 36 |
| periodic_opposed_64 | adversarial_power_schedule | 0 / 36 |
| quality_suppression_35pct | adversarial_quality_selection | 24 / 36 |
| quality_suppression_51pct | adversarial_quality_selection | 12 / 36 |

Here `honest` means the published quality signal is not strategically
selected; that envelope deliberately still contains the opposed one-time
and periodic adversarial hash/solve-power schedules shown above.

All 36 full-grid coordinates passed the restricted stationary/hash-step
envelope. Those coordinates contain 6/6 unique hash-loop settings (three
EMA coordinates × two caps), each repeated across six size settings,
so this experiment supports only a broad non-normative hash-loop region:
EMA coordinates of 16–64 blocks and per-update caps of 1.125×–1.25× under
the fixed model gain. It does not distinguish or ratify P-2 within that
region. No coordinate passed a solve-power shock or the opposed periodic
schedule across all fixed seeds, solve-stage shares, and quality
elasticities; the tested P-11 region of 64–256 blocks therefore has no
robust candidate in this model.

No tested coordinate passed every unmanipulated-quality scenario and sensitivity.
That is a negative result; the grid must not be promoted by selecting
the least-bad row.

### Structural adversarial finding

No coordinate passed the adversarial envelope. Although each observed
quality value is recomputed by the checker, a miner able to choose among
valid solutions can publish a threshold-margin solution and suppress
information about its better solution. The modeled size loop responds by
lowering instance size. Changing only EMA windows or clamp magnitudes
changes the speed of that drift, not its direction or eventual incentive.

Some slow size coordinates remain above the finite-run retention bound
only because they also fail to adapt to honest solve-power shocks. Under
the model's persistent suppression equation, the asymptotic size ratio is
`(1 - adversarial_share)^(1 / elasticity)`: 0.423–0.806 for the
35% case and 0.240–0.700 for the 51% case across the configured
elasticity sweep. Slowness delays the bias; it does not remove it.

Therefore this experiment does **not** support freezing a size retarget
driven solely by winning-block quality margin. G0/Phase 2 must either
supply an independently justified manipulation-resistant observable,
retain a human-ratified static size between explicit upgrades, or open the
D2 fallback review. This report makes no such decision.

## First-solver / solution-reuse boundary

The same stochastic model makes the solve-once boundary explicit without
inventing a propagation delay or miner count. Before disclosure, an equal-
power miner pays the solution and hash stages. A copier of a public valid
solution still must rerun the full miner-bound eligibility hash race, but no
longer pays the solve stage:

| normalized solve-stage share | initial cycle mean | initial variance | post-disclosure hash-only mean | post-disclosure variance |
|---:|---:|---:|---:|---:|
| 0.200 | 1.000 | 0.680 | 0.800 | 0.640 |
| 0.500 | 1.000 | 0.500 | 0.500 | 0.250 |

This quantifies the head-start/variance boundary, not fork probability.
A fork probability requires propagation, competing-miner count/power, and
withholding behavior that are absent and therefore remain UNKNOWN.

## Absolute-cadence limitation

P-003 did not solve its provisional P-4 candidate, P-002 did not assign a
production VDF delay, and this packet has no propagation/reference-miner
distribution. An absolute target in seconds would therefore be invented, not
discovered. P-1 remains unresolved. Candidate windows in this report are
expressed in blocks and remain unratified P-2/P-11 proposal inputs.

## Interpretation limits

- The simulator is a sensitivity model, not a proof of Nakamoto security,
  selfish-mining resistance, or real SIS solver behavior.
- Exponential stage delays and the quality-response equation are explicit
  assumptions. The elasticity sweep reduces but does not remove model risk.
- Same-height solution reuse removes a rival's solve stage after disclosure,
  but its effect depends on propagation and fork-race data absent here. It
  remains a Phase-2 adversarial-corpus/network-model obligation rather than a
  fabricated numeric result.
- A stable hash-rate loop cannot rehabilitate a manipulable size signal; the
  two conclusions must remain separate.

## Reproduction seals

- `config.json` SHA-256: `FAADF793F6B196C66F48AA2C5A5E2CCDF12A8BAEC2517FFB6C4A0CF23F13F51D`.
- `simulate.py` SHA-256: `C90157F08DA2A93080177862B7A5FA1B5D9D0DA2F36159CED47801B8A5DB12A5`.
- `python simulate.py check` regenerates both committed artifacts in a
  temporary directory and compares exact bytes.
