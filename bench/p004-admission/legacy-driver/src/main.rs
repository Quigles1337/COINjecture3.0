use std::{env, error::Error, path::PathBuf};

use cj3_admission_bench::{
    run_candidate, write_jsonl, write_markdown, BenchCandidate, CandidateMetadata, EvidenceItem,
    EvidenceState, RunConfig, RunRecord,
};
use coinject_consensus::solve_problem_blocking;
use coinject_core::{Clause, ProblemType, Solution};

const LEGACY_REVISION: &str = "58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff";
const PROVISIONAL_CHECKER_BUDGET_NS: u64 = 15_000_000;

struct Arguments {
    jsonl: PathBuf,
    markdown: PathBuf,
    warmups: u32,
    samples: u32,
    checker_repetitions: u32,
}

struct LegacyExecutable {
    metadata: CandidateMetadata,
    case_id: &'static str,
    fixture_note: &'static str,
    problem: ProblemType,
}

impl BenchCandidate for LegacyExecutable {
    type Instance = ProblemType;
    type Solution = Solution;

    fn metadata(&self) -> CandidateMetadata {
        self.metadata
    }

    fn case_id(&self) -> &'static str {
        self.case_id
    }

    fn prepare(&self) -> Result<Self::Instance, String> {
        Ok(self.problem.clone())
    }

    fn solve(&self, instance: &Self::Instance) -> Result<Self::Solution, String> {
        solve_problem_blocking(instance.clone())
            .map(|(solution, _, _)| solution)
            .ok_or_else(|| "the legacy solver returned no candidate".to_owned())
    }

    fn check(&self, instance: &Self::Instance, solution: &Self::Solution) -> Result<u64, String> {
        let quality = solution.quality(instance);
        if !quality.is_finite() || quality <= 0.0 || quality > 1.0 {
            return Err(format!(
                "legacy checker/quality result is invalid: {quality}"
            ));
        }
        let scaled = (quality * 1_000_000.0).round();
        if scaled < 0.0 || scaled > u64::MAX as f64 {
            return Err("scaled legacy quality is outside u64".to_owned());
        }
        Ok(scaled as u64)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = parse_arguments()?;
    let config = RunConfig {
        warmups: arguments.warmups,
        samples: arguments.samples,
        checker_repetitions: arguments.checker_repetitions,
        provisional_checker_budget_ns: Some(PROVISIONAL_CHECKER_BUDGET_NS),
    }
    .validate()?;

    let mut records = Vec::new();
    for candidate in executable_candidates() {
        let mut record = run_candidate(&candidate, config)?;
        record.note = format!("{} {}", record.note, candidate.fixture_note);
        records.push(record);
    }
    records.extend(unimplemented_inventory(config));

    write_jsonl(&arguments.jsonl, &records)?;
    write_markdown(&arguments.markdown, &records)?;
    println!("P004_LEGACY_RECORDS={}", records.len());
    println!("P004_JSONL={}", arguments.jsonl.display());
    println!("P004_MARKDOWN={}", arguments.markdown.display());
    Ok(())
}

fn parse_arguments() -> Result<Arguments, Box<dyn Error>> {
    let mut jsonl = None;
    let mut markdown = None;
    let mut warmups = 2_u32;
    let mut samples = 9_u32;
    let mut checker_repetitions = 10_000_u32;
    let mut arguments = env::args().skip(1);
    while let Some(flag) = arguments.next() {
        let value = arguments
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--jsonl" => jsonl = Some(PathBuf::from(value)),
            "--markdown" => markdown = Some(PathBuf::from(value)),
            "--warmups" => warmups = value.parse()?,
            "--samples" => samples = value.parse()?,
            "--checker-repetitions" => checker_repetitions = value.parse()?,
            _ => return Err(format!("unknown argument: {flag}").into()),
        }
    }
    Ok(Arguments {
        jsonl: jsonl.ok_or("--jsonl is required")?,
        markdown: markdown.ok_or("--markdown is required")?,
        warmups,
        samples,
        checker_repetitions,
    })
}

fn executable_candidates() -> [LegacyExecutable; 3] {
    [subset_sum_candidate(), sat_candidate(), tsp_candidate()]
}

fn subset_sum_candidate() -> LegacyExecutable {
    let numbers: Vec<i64> = (1_i64..=96)
        .map(|index| (index * 37).rem_euclid(997) + 1)
        .collect();
    let target = numbers
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 7 == 0)
        .map(|(_, value)| value)
        .sum();
    LegacyExecutable {
        metadata: CandidateMetadata {
            candidate: "legacy-subset-sum",
            family: "SubsetSum",
            source_revision: LEGACY_REVISION,
            source_paths: &[
                "core/src/problem.rs:6-15,85-90,133-140",
                "consensus/src/miner.rs:94-100,139-347,699-724",
                "consensus/src/problem_registry.rs:162-188,353",
                "docs/RESEARCH_SURVEY.md:54-58,67-78,142-145",
            ],
            hardness: EvidenceItem {
                state: EvidenceState::Rejected,
                basis: "The active generator plants a subset in bounded positive integers, while the survey records that random knapsack/subset-sum ensembles are typically easy and pseudopolynomial algorithms apply; no distributional reduction covers this generator.",
            },
            parameter_floor: EvidenceItem {
                state: EvidenceState::Unknown,
                basis: "The registry supplies empirical size seeds and an absolute maximum, not an attack-derived parameter floor with a published falsification threshold.",
            },
            derived_instance: EvidenceItem {
                state: EvidenceState::Unknown,
                basis: "Mining generation is seeded from parent hash plus height, but the descriptor's class-local generate_instance method is unimplemented and this packet found no admission-grade proof that every validation path re-derives the supplied ProblemType.",
            },
            pure_checker: EvidenceItem {
                state: EvidenceState::Rejected,
                basis: "Verification maps an out-of-range index to zero and sums i64 values without checked arithmetic; quality is floating point. This fails CJ3's strict hostile-input, exact-integer checker contract even though the happy-path function is deterministic.",
            },
        },
        case_id: "deterministic-one-dimensional-dp-96",
        fixture_note: "The driver invokes the exact legacy solver and checker, but this deterministic fixture is not sampled from the active generator and makes no hardness claim.",
        problem: ProblemType::SubsetSum { numbers, target },
    }
}

fn sat_candidate() -> LegacyExecutable {
    let variables = 18;
    let clauses = (1..=variables)
        .map(|variable| Clause {
            literals: vec![variable as i32],
        })
        .collect();
    LegacyExecutable {
        metadata: CandidateMetadata {
            candidate: "legacy-sat",
            family: "SAT",
            source_revision: LEGACY_REVISION,
            source_paths: &[
                "core/src/problem.rs:6-20,91-109,150-155",
                "consensus/src/miner.rs:101-121,349-465,725-776",
                "consensus/src/problem_registry.rs:190-216,354",
                "docs/RESEARCH_SURVEY.md:54-65,67-78,142-145,159-161",
            ],
            hardness: EvidenceItem {
                state: EvidenceState::Rejected,
                basis: "The active generator uses a planted formula at three clauses per variable, not the approximately 4.267 random-3-SAT transition discussed in the survey, and the planting loop does not force every generated clause to contain a planted-satisfying literal. No average-case guarantee or measured hard distribution is supplied.",
            },
            parameter_floor: EvidenceItem {
                state: EvidenceState::Unknown,
                basis: "The registry's scaling exponent, size ratio, and maximum are empirical seeds; no solver-attack survey establishes a defensible variable/clause floor.",
            },
            derived_instance: EvidenceItem {
                state: EvidenceState::Unknown,
                basis: "Mining generation is deterministic from parent data, but it sits outside the descriptor contract and the current source evidence does not establish validator-side re-derivation on every acceptance path.",
            },
            pure_checker: EvidenceItem {
                state: EvidenceState::Rejected,
                basis: "The checker returns bool/floating quality rather than typed integer quality, and literal zero reaches unsigned index subtraction without a strict decode error. This is not CJ3's bounded fail-closed checker surface.",
            },
        },
        case_id: "deterministic-bruteforce-18-last-assignment",
        fixture_note: "The unit-clause fixture deliberately selects the exact source brute-force path and its last assignment; it is not the active planted 3-CNF distribution and makes no hardness claim.",
        problem: ProblemType::SAT { variables, clauses },
    }
}

fn tsp_candidate() -> LegacyExecutable {
    let cities = 96;
    let mut distances = vec![vec![0_u64; cities]; cities];
    for row in 0..cities {
        for column in (row + 1)..cities {
            let mixed = row * 131 + column * 197 + (row ^ column) * 17;
            let distance = u64::try_from(mixed % 99 + 1).expect("fixture distance fits u64");
            distances[row][column] = distance;
            distances[column][row] = distance;
        }
    }
    LegacyExecutable {
        metadata: CandidateMetadata {
            candidate: "legacy-tsp",
            family: "TSP",
            source_revision: LEGACY_REVISION,
            source_paths: &[
                "core/src/problem.rs:16-24,27-67,110-149",
                "consensus/src/miner.rs:122-124,468-545,777-796",
                "consensus/src/problem_registry.rs:218-244,355",
                "docs/RESEARCH_SURVEY.md:54-58,67-78,142-145",
            ],
            hardness: EvidenceItem {
                state: EvidenceState::Rejected,
                basis: "The active generator samples complete symmetric matrices with small independent weights, while the survey records that random TSP instances are typically easy; worst-case NP-hardness does not establish this distribution's hardness.",
            },
            parameter_floor: EvidenceItem {
                state: EvidenceState::Unknown,
                basis: "The registry provides empirical size seeds and a maximum but no attack-derived floor, approximation target, or threshold decision bound.",
            },
            derived_instance: EvidenceItem {
                state: EvidenceState::Unknown,
                basis: "Mining generation is deterministic from parent data, but the descriptor generation method is unimplemented and no class-local CJ3-shaped derivation/check contract exists.",
            },
            pure_checker: EvidenceItem {
                state: EvidenceState::Rejected,
                basis: "Validity checks only that a permutation visits each city; it imposes no cost threshold. Quality uses floating point and unchecked matrix indexing/addition, so malformed data can panic or overflow instead of returning a typed invalid result.",
            },
        },
        case_id: "deterministic-nearest-neighbor-two-opt-96",
        fixture_note: "The bounded symmetric matrix selects the exact legacy nearest-neighbor plus two-opt path, but it is not an StdRng sample from the active generator and makes no hardness claim.",
        problem: ProblemType::TSP { cities, distances },
    }
}

fn unimplemented_inventory(config: RunConfig) -> Vec<RunRecord> {
    vec![
        descriptor_only(
            config,
            "legacy-graph-coloring-descriptor",
            "GraphColoring",
            &[
                "consensus/src/problem_registry.rs:246-272,356",
                "consensus/src/problem_registry.rs:109-156,353-358",
            ],
            "The default registry contains metadata only. ProblemType has no GraphColoring variant, and the descriptor inherits unimplemented generation and verification methods.",
        ),
        descriptor_only(
            config,
            "legacy-factorization-descriptor",
            "Factorization",
            &[
                "consensus/src/problem_registry.rs:274-303,357",
                "consensus/src/problem_registry.rs:109-156,353-358",
            ],
            "The default registry contains metadata only. ProblemType has no Factorization variant, and the descriptor inherits unimplemented generation and verification methods.",
        ),
        descriptor_only(
            config,
            "legacy-svp-descriptor",
            "SVP",
            &[
                "consensus/src/problem_registry.rs:305-335,358",
                "consensus/src/problem_registry.rs:109-156,353-358",
            ],
            "The default registry contains metadata only. ProblemType has no SVP variant, and the descriptor inherits unimplemented generation and verification methods; the name alone does not import SIS/SVP distributional evidence.",
        ),
        custom_payload(config),
    ]
}

fn descriptor_only(
    config: RunConfig,
    candidate: &'static str,
    family: &'static str,
    source_paths: &'static [&'static str],
    note: &'static str,
) -> RunRecord {
    let not_implemented = EvidenceItem {
        state: EvidenceState::NotImplemented,
        basis: note,
    };
    RunRecord::unmeasured(
        CandidateMetadata {
            candidate,
            family,
            source_revision: LEGACY_REVISION,
            source_paths,
            hardness: EvidenceItem {
                state: EvidenceState::Unknown,
                basis: "A complexity label and empirical scaling exponent are not a published sampled-distribution hardness argument.",
            },
            parameter_floor: EvidenceItem {
                state: EvidenceState::Unknown,
                basis: "No implemented generator, attack model, or measured floor is present.",
            },
            derived_instance: not_implemented,
            pure_checker: not_implemented,
        },
        "inventory-only",
        config,
        note,
    )
}

fn custom_payload(config: RunConfig) -> RunRecord {
    let not_implemented = EvidenceItem {
        state: EvidenceState::NotImplemented,
        basis: "Custom is an opaque user payload with no class-specific solver or checker; the legacy solver returns no candidate and verification rejects the variant.",
    };
    RunRecord::unmeasured(
        CandidateMetadata {
            candidate: "legacy-custom-payload",
            family: "Custom",
            source_revision: LEGACY_REVISION,
            source_paths: &[
                "core/src/problem.rs:21-24,80-81,129,156",
                "consensus/src/miner.rs:125",
            ],
            hardness: not_implemented,
            parameter_floor: not_implemented,
            derived_instance: EvidenceItem {
                state: EvidenceState::Rejected,
                basis: "The payload is user supplied rather than derived from protocol state.",
            },
            pure_checker: not_implemented,
        },
        "inventory-only-not-a-class",
        config,
        "Custom is inventoried for completeness but is not a concrete legacy class and cannot enter an admission timing run.",
    )
}
