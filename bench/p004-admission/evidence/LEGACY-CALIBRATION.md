# P-004 legacy-class admission calibration

This report is generated from the adjacent JSONL evidence. `eligible_for_adr_review` is not admission; only a ratified ADR can admit a class. Timing observations do not prove hardness.
Checker summaries are per-operation integer averages from bounded timed batches; every JSONL row records its batch size.

| candidate | fixture | solver median ns | checker p95/max ns | median asymmetry | P-3 comparison | disposition |
|---|---|---:|---:|---:|---|---|
| legacy-subset-sum | deterministic-one-dimensional-dp-96 | 2044000 | 9/9 | 227111.111x | supported | reject |
| legacy-sat | deterministic-bruteforce-18-last-assignment | 14198500 | 18/19 | 788805.555x | supported | reject |
| legacy-tsp | deterministic-nearest-neighbor-two-opt-96 | 636300 | 4169/4180 | 155.574x | supported | reject |
| legacy-graph-coloring-descriptor | inventory-only | n/a | n/a | n/a | unknown | insufficient_evidence |
| legacy-factorization-descriptor | inventory-only | n/a | n/a | n/a | unknown | insufficient_evidence |
| legacy-svp-descriptor | inventory-only | n/a | n/a | n/a | unknown | insufficient_evidence |
| legacy-custom-payload | inventory-only-not-a-class | n/a | n/a | n/a | unknown | reject |

## legacy-subset-sum — reject

- Family: SubsetSum
- Source revision: 58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff
- Source paths: core/src/problem.rs:6-15,85-90,133-140, consensus/src/miner.rs:94-100,139-347,699-724, consensus/src/problem\_registry.rs:162-188,353, docs/RESEARCH\_SURVEY.md:54-58,67-78,142-145
- Fixture: deterministic-one-dimensional-dp-96
- Note: Timing is calibration evidence only; it does not prove distributional hardness. The driver invokes the exact legacy solver and checker, but this deterministic fixture is not sampled from the active generator and makes no hardness claim.

| required question | state | evidence basis |
|---|---|---|
| hard sampled distribution | rejected | The active generator plants a subset in bounded positive integers, while the survey records that random knapsack/subset-sum ensembles are typically easy and pseudopolynomial algorithms apply; no distributional reduction covers this generator. |
| parameter floor | unknown | The registry supplies empirical size seeds and an absolute maximum, not an attack-derived parameter floor with a published falsification threshold. |
| A1/A2 derived instance | unknown | Mining generation is seeded from parent hash plus height, but the descriptor's class-local generate_instance method is unimplemented and this packet found no admission-grade proof that every validation path re-derives the supplied ProblemType. |
| A3 pure checker | rejected | Verification maps an out-of-range index to zero and sums i64 values without checked arithmetic; quality is floating point. This fails CJ3's strict hostile-input, exact-integer checker contract even though the happy-path function is deterministic. |
| checker within provisional P-3 | supported | maximum measured checker observation compared with the provisional 15 ms P-3 recommendation; G0 has not ratified the budget |

## legacy-sat — reject

- Family: SAT
- Source revision: 58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff
- Source paths: core/src/problem.rs:6-20,91-109,150-155, consensus/src/miner.rs:101-121,349-465,725-776, consensus/src/problem\_registry.rs:190-216,354, docs/RESEARCH\_SURVEY.md:54-65,67-78,142-145,159-161
- Fixture: deterministic-bruteforce-18-last-assignment
- Note: Timing is calibration evidence only; it does not prove distributional hardness. The unit-clause fixture deliberately selects the exact source brute-force path and its last assignment; it is not the active planted 3-CNF distribution and makes no hardness claim.

| required question | state | evidence basis |
|---|---|---|
| hard sampled distribution | rejected | The active generator uses a planted formula at three clauses per variable, not the approximately 4.267 random-3-SAT transition discussed in the survey, and the planting loop does not force every generated clause to contain a planted-satisfying literal. No average-case guarantee or measured hard distribution is supplied. |
| parameter floor | unknown | The registry's scaling exponent, size ratio, and maximum are empirical seeds; no solver-attack survey establishes a defensible variable/clause floor. |
| A1/A2 derived instance | unknown | Mining generation is deterministic from parent data, but it sits outside the descriptor contract and the current source evidence does not establish validator-side re-derivation on every acceptance path. |
| A3 pure checker | rejected | The checker returns bool/floating quality rather than typed integer quality, and literal zero reaches unsigned index subtraction without a strict decode error. This is not CJ3's bounded fail-closed checker surface. |
| checker within provisional P-3 | supported | maximum measured checker observation compared with the provisional 15 ms P-3 recommendation; G0 has not ratified the budget |

## legacy-tsp — reject

- Family: TSP
- Source revision: 58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff
- Source paths: core/src/problem.rs:16-24,27-67,110-149, consensus/src/miner.rs:122-124,468-545,777-796, consensus/src/problem\_registry.rs:218-244,355, docs/RESEARCH\_SURVEY.md:54-58,67-78,142-145
- Fixture: deterministic-nearest-neighbor-two-opt-96
- Note: Timing is calibration evidence only; it does not prove distributional hardness. The bounded symmetric matrix selects the exact legacy nearest-neighbor plus two-opt path, but it is not an StdRng sample from the active generator and makes no hardness claim.

| required question | state | evidence basis |
|---|---|---|
| hard sampled distribution | rejected | The active generator samples complete symmetric matrices with small independent weights, while the survey records that random TSP instances are typically easy; worst-case NP-hardness does not establish this distribution's hardness. |
| parameter floor | unknown | The registry provides empirical size seeds and a maximum but no attack-derived floor, approximation target, or threshold decision bound. |
| A1/A2 derived instance | unknown | Mining generation is deterministic from parent data, but the descriptor generation method is unimplemented and no class-local CJ3-shaped derivation/check contract exists. |
| A3 pure checker | rejected | Validity checks only that a permutation visits each city; it imposes no cost threshold. Quality uses floating point and unchecked matrix indexing/addition, so malformed data can panic or overflow instead of returning a typed invalid result. |
| checker within provisional P-3 | supported | maximum measured checker observation compared with the provisional 15 ms P-3 recommendation; G0 has not ratified the budget |

## legacy-graph-coloring-descriptor — insufficient_evidence

- Family: GraphColoring
- Source revision: 58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff
- Source paths: consensus/src/problem\_registry.rs:246-272,356, consensus/src/problem\_registry.rs:109-156,353-358
- Fixture: inventory-only
- Note: The default registry contains metadata only. ProblemType has no GraphColoring variant, and the descriptor inherits unimplemented generation and verification methods.

| required question | state | evidence basis |
|---|---|---|
| hard sampled distribution | unknown | A complexity label and empirical scaling exponent are not a published sampled-distribution hardness argument. |
| parameter floor | unknown | No implemented generator, attack model, or measured floor is present. |
| A1/A2 derived instance | not_implemented | The default registry contains metadata only. ProblemType has no GraphColoring variant, and the descriptor inherits unimplemented generation and verification methods. |
| A3 pure checker | not_implemented | The default registry contains metadata only. ProblemType has no GraphColoring variant, and the descriptor inherits unimplemented generation and verification methods. |
| checker within provisional P-3 | unknown | maximum measured checker observation compared with the provisional 15 ms P-3 recommendation; G0 has not ratified the budget |

## legacy-factorization-descriptor — insufficient_evidence

- Family: Factorization
- Source revision: 58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff
- Source paths: consensus/src/problem\_registry.rs:274-303,357, consensus/src/problem\_registry.rs:109-156,353-358
- Fixture: inventory-only
- Note: The default registry contains metadata only. ProblemType has no Factorization variant, and the descriptor inherits unimplemented generation and verification methods.

| required question | state | evidence basis |
|---|---|---|
| hard sampled distribution | unknown | A complexity label and empirical scaling exponent are not a published sampled-distribution hardness argument. |
| parameter floor | unknown | No implemented generator, attack model, or measured floor is present. |
| A1/A2 derived instance | not_implemented | The default registry contains metadata only. ProblemType has no Factorization variant, and the descriptor inherits unimplemented generation and verification methods. |
| A3 pure checker | not_implemented | The default registry contains metadata only. ProblemType has no Factorization variant, and the descriptor inherits unimplemented generation and verification methods. |
| checker within provisional P-3 | unknown | maximum measured checker observation compared with the provisional 15 ms P-3 recommendation; G0 has not ratified the budget |

## legacy-svp-descriptor — insufficient_evidence

- Family: SVP
- Source revision: 58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff
- Source paths: consensus/src/problem\_registry.rs:305-335,358, consensus/src/problem\_registry.rs:109-156,353-358
- Fixture: inventory-only
- Note: The default registry contains metadata only. ProblemType has no SVP variant, and the descriptor inherits unimplemented generation and verification methods; the name alone does not import SIS/SVP distributional evidence.

| required question | state | evidence basis |
|---|---|---|
| hard sampled distribution | unknown | A complexity label and empirical scaling exponent are not a published sampled-distribution hardness argument. |
| parameter floor | unknown | No implemented generator, attack model, or measured floor is present. |
| A1/A2 derived instance | not_implemented | The default registry contains metadata only. ProblemType has no SVP variant, and the descriptor inherits unimplemented generation and verification methods; the name alone does not import SIS/SVP distributional evidence. |
| A3 pure checker | not_implemented | The default registry contains metadata only. ProblemType has no SVP variant, and the descriptor inherits unimplemented generation and verification methods; the name alone does not import SIS/SVP distributional evidence. |
| checker within provisional P-3 | unknown | maximum measured checker observation compared with the provisional 15 ms P-3 recommendation; G0 has not ratified the budget |

## legacy-custom-payload — reject

- Family: Custom
- Source revision: 58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff
- Source paths: core/src/problem.rs:21-24,80-81,129,156, consensus/src/miner.rs:125
- Fixture: inventory-only-not-a-class
- Note: Custom is inventoried for completeness but is not a concrete legacy class and cannot enter an admission timing run.

| required question | state | evidence basis |
|---|---|---|
| hard sampled distribution | not_implemented | Custom is an opaque user payload with no class-specific solver or checker; the legacy solver returns no candidate and verification rejects the variant. |
| parameter floor | not_implemented | Custom is an opaque user payload with no class-specific solver or checker; the legacy solver returns no candidate and verification rejects the variant. |
| A1/A2 derived instance | rejected | The payload is user supplied rather than derived from protocol state. |
| A3 pure checker | not_implemented | Custom is an opaque user payload with no class-specific solver or checker; the legacy solver returns no candidate and verification rejects the variant. |
| checker within provisional P-3 | unknown | maximum measured checker observation compared with the provisional 15 ms P-3 recommendation; G0 has not ratified the budget |
