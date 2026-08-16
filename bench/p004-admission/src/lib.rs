//! Offline admission-bench controller for problem-class candidates.
//!
//! This crate is analysis tooling, not node or consensus code. It keeps benchmark
//! evidence separate from admission decisions: even a fully supported result is only
//! eligible for human ADR review.

#![forbid(unsafe_code)]

use std::{
    error::Error,
    fmt,
    fs::File,
    hint::black_box,
    io::{self, BufWriter, Write},
    marker::PhantomData,
    path::Path,
    time::{Duration, Instant},
};

use cj3_classes::ProblemClass;

/// Stable schema identifier written into every JSONL record.
pub const SCHEMA_VERSION: u32 = 1;

const MAX_WARMUPS: u32 = 1_000;
const MAX_SAMPLES: u32 = 10_000;
const MAX_CHECKER_REPETITIONS: u32 = 1_000_000;

/// Result of one required admission-evidence question.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceState {
    /// Evidence supports the requirement at the scope stated in `basis`.
    Supported,
    /// Available evidence contradicts the requirement.
    Rejected,
    /// The required evidence is absent or does not resolve the question.
    Unknown,
    /// The source does not implement the surface needed to evaluate the question.
    NotImplemented,
}

impl EvidenceState {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Rejected => "rejected",
            Self::Unknown => "unknown",
            Self::NotImplemented => "not_implemented",
        }
    }
}

/// One evidence state plus the exact basis for assigning it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceItem {
    /// Mechanical state of the evidence question.
    pub state: EvidenceState,
    /// Source pointer and bounded interpretation.
    pub basis: &'static str,
}

/// Immutable provenance and admission evidence for one candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CandidateMetadata {
    /// Stable bench-local candidate name; this is not a consensus class identifier.
    pub candidate: &'static str,
    /// Human-readable problem family.
    pub family: &'static str,
    /// Exact source revision used by the adapter.
    pub source_revision: &'static str,
    /// Source paths that define generation, solving, checking, or registry status.
    pub source_paths: &'static [&'static str],
    /// Published distributional-hardness evidence.
    pub hardness: EvidenceItem,
    /// Parameter-floor evidence against known attacks.
    pub parameter_floor: EvidenceItem,
    /// Fit with protocol-derived instance generation under A1/A2.
    pub derived_instance: EvidenceItem,
    /// Fit with a deterministic pure checker under A3.
    pub pure_checker: EvidenceItem,
}

/// Bounded benchmark controls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunConfig {
    /// Untimed iterations used to initialize code and allocations.
    pub warmups: u32,
    /// Timed iterations retained as raw observations.
    pub samples: u32,
    /// Checker calls in each timed batch; summaries report one-call integer averages.
    pub checker_repetitions: u32,
    /// Provisional checker ceiling in nanoseconds; `None` leaves the gate unknown.
    pub provisional_checker_budget_ns: Option<u64>,
}

impl RunConfig {
    /// Validates resource bounds before candidate code runs.
    ///
    /// # Errors
    ///
    /// Returns [`BenchError::InvalidConfig`] for zero or excessive sample counts.
    pub fn validate(self) -> Result<Self, BenchError> {
        if self.warmups > MAX_WARMUPS {
            return Err(BenchError::InvalidConfig("warmups exceed 1000"));
        }
        if self.samples == 0 || self.samples > MAX_SAMPLES {
            return Err(BenchError::InvalidConfig(
                "samples must be in the inclusive range 1..=10000",
            ));
        }
        if self.checker_repetitions == 0 || self.checker_repetitions > MAX_CHECKER_REPETITIONS {
            return Err(BenchError::InvalidConfig(
                "checker repetitions must be in the inclusive range 1..=1000000",
            ));
        }
        Ok(self)
    }
}

/// Adapter contract consumed by the class-neutral runner.
///
/// Solvers live only in this offline process. The separately invoked `check` method
/// validates every candidate output before a timing observation is accepted.
pub trait BenchCandidate {
    /// Prepared problem instance.
    type Instance;
    /// Candidate solution returned by the solver adapter.
    type Solution;

    /// Returns immutable provenance and evidence states.
    fn metadata(&self) -> CandidateMetadata;
    /// Returns the fixture name within the candidate family.
    fn case_id(&self) -> &'static str;
    /// Prepares one deterministic instance outside the solve/check timing windows.
    ///
    /// # Errors
    ///
    /// Returns a bounded diagnostic when the fixture cannot be prepared exactly.
    fn prepare(&self) -> Result<Self::Instance, String>;
    /// Runs the untrusted/offline solver adapter.
    ///
    /// # Errors
    ///
    /// Returns a bounded diagnostic when the solver fails, times out, or is unsupported.
    fn solve(&self, instance: &Self::Instance) -> Result<Self::Solution, String>;
    /// Independently validates one solver output and returns integer quality.
    ///
    /// # Errors
    ///
    /// Returns a bounded diagnostic when the output is invalid or checking fails closed.
    fn check(&self, instance: &Self::Instance, solution: &Self::Solution) -> Result<u64, String>;
}

/// Solver fixture paired with any already implemented CJ3 [`ProblemClass`].
pub trait ProblemClassSolver<C: ProblemClass> {
    /// Produces a decoded candidate solution for the derived instance.
    ///
    /// # Errors
    ///
    /// Returns a bounded diagnostic when no candidate solution is produced.
    fn solve(&self, instance: &C::Instance) -> Result<C::Solution, String>;
}

/// Generic adapter proving the runner consumes any implemented [`ProblemClass`].
pub struct ProblemClassCandidate<C: ProblemClass, S> {
    metadata: CandidateMetadata,
    case_id: &'static str,
    seed: [u8; 32],
    size: C::SizeParam,
    solver: S,
    class: PhantomData<fn() -> C>,
}

impl<C: ProblemClass, S> ProblemClassCandidate<C, S> {
    /// Creates a generic candidate without assigning or interpreting a class ID.
    #[must_use]
    pub const fn new(
        metadata: CandidateMetadata,
        case_id: &'static str,
        seed: [u8; 32],
        size: C::SizeParam,
        solver: S,
    ) -> Self {
        Self {
            metadata,
            case_id,
            seed,
            size,
            solver,
            class: PhantomData,
        }
    }
}

impl<C, S> BenchCandidate for ProblemClassCandidate<C, S>
where
    C: ProblemClass,
    S: ProblemClassSolver<C>,
{
    type Instance = C::Instance;
    type Solution = C::Solution;

    fn metadata(&self) -> CandidateMetadata {
        self.metadata
    }

    fn case_id(&self) -> &'static str {
        self.case_id
    }

    fn prepare(&self) -> Result<Self::Instance, String> {
        Ok(C::derive_instance(self.seed, self.size))
    }

    fn solve(&self, instance: &Self::Instance) -> Result<Self::Solution, String> {
        self.solver.solve(instance)
    }

    fn check(&self, instance: &Self::Instance, solution: &Self::Solution) -> Result<u64, String> {
        C::check(instance, solution)
            .map(cj3_classes::Quality::get)
            .map_err(|error| format!("ProblemClass checker rejected output: {error:?}"))
    }
}

/// Four-statistic summary retained for each timed phase.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimingSummary {
    /// Smallest observation in nanoseconds.
    pub min_ns: u64,
    /// Median observation in nanoseconds.
    pub median_ns: u64,
    /// Nearest-rank 95th percentile observation in nanoseconds.
    pub p95_ns: u64,
    /// Largest observation in nanoseconds.
    pub max_ns: u64,
}

/// Candidate disposition before the required human ADR decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Recommendation {
    /// At least one required criterion was mechanically rejected.
    Reject,
    /// No rejection was established, but evidence remains absent or unimplemented.
    InsufficientEvidence,
    /// All machine-evaluable criteria passed; only human ADR review may admit it.
    EligibleForAdrReview,
}

impl Recommendation {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Reject => "reject",
            Self::InsufficientEvidence => "insufficient_evidence",
            Self::EligibleForAdrReview => "eligible_for_adr_review",
        }
    }
}

/// Complete durable record for one measured or unmeasured candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunRecord {
    /// Candidate provenance and admission evidence.
    pub metadata: CandidateMetadata,
    /// Fixture or inventory row name.
    pub case_id: &'static str,
    /// Bounded run controls.
    pub config: RunConfig,
    /// One-time instance-preparation cost.
    pub prepare_ns: Option<u64>,
    /// Solver timing summary, absent for an unmeasured class.
    pub solver: Option<TimingSummary>,
    /// Independent-checker timing summary, absent for an unmeasured class.
    pub checker: Option<TimingSummary>,
    /// Median solver/checker ratio multiplied by 1000.
    pub asymmetry_milli: Option<u64>,
    /// Minimum integer quality observed across accepted samples.
    pub quality_min: Option<u64>,
    /// Maximum integer quality observed across accepted samples.
    pub quality_max: Option<u64>,
    /// Checker-budget assessment using the maximum observation.
    pub checker_budget: EvidenceState,
    /// Machine-derived disposition; never an admission decision.
    pub recommendation: Recommendation,
    /// Honest limitation or failure detail.
    pub note: String,
}

impl RunRecord {
    /// Creates an explicit inventory record when no executable class exists.
    #[must_use]
    pub fn unmeasured(
        metadata: CandidateMetadata,
        case_id: &'static str,
        config: RunConfig,
        note: impl Into<String>,
    ) -> Self {
        let checker_budget = EvidenceState::Unknown;
        let recommendation = recommendation(metadata, checker_budget);
        Self {
            metadata,
            case_id,
            config,
            prepare_ns: None,
            solver: None,
            checker: None,
            asymmetry_milli: None,
            quality_min: None,
            quality_max: None,
            checker_budget,
            recommendation,
            note: note.into(),
        }
    }
}

/// Admission-bench failures that prevent a valid durable record.
#[derive(Debug)]
pub enum BenchError {
    /// Benchmark controls exceeded their fixed resource bounds.
    InvalidConfig(&'static str),
    /// Candidate preparation, solving, or checking failed.
    Candidate {
        /// Phase that failed.
        phase: &'static str,
        /// Adapter-supplied failure detail.
        detail: String,
    },
    /// A duration could not be represented as a `u64` nanosecond count.
    DurationOverflow,
    /// The timed checker batch was too short to resolve a nonzero per-call duration.
    TimerResolution,
    /// Integer ratio arithmetic overflowed.
    RatioOverflow,
    /// Durable evidence could not be written.
    Io(io::Error),
}

impl fmt::Display for BenchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(detail) => write!(formatter, "invalid benchmark config: {detail}"),
            Self::Candidate { phase, detail } => {
                write!(formatter, "candidate {phase} failed: {detail}")
            }
            Self::DurationOverflow => formatter.write_str("duration exceeds u64 nanoseconds"),
            Self::TimerResolution => formatter.write_str(
                "checker timing resolved to zero nanoseconds per call; increase repetitions",
            ),
            Self::RatioOverflow => formatter.write_str("asymmetry ratio exceeds u64"),
            Self::Io(error) => write!(formatter, "evidence I/O failed: {error}"),
        }
    }
}

impl Error for BenchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for BenchError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Runs one candidate with independent solve and checker timing windows.
///
/// # Errors
///
/// Returns a bounded configuration, candidate adapter, duration, or ratio error. A
/// failed sample is never omitted from an otherwise successful record.
pub fn run_candidate<C: BenchCandidate>(
    candidate: &C,
    config: RunConfig,
) -> Result<RunRecord, BenchError> {
    let config = config.validate()?;
    let prepare_start = Instant::now();
    let instance = candidate
        .prepare()
        .map_err(|detail| BenchError::Candidate {
            phase: "prepare",
            detail,
        })?;
    let prepare_ns = elapsed_ns(prepare_start.elapsed())?;

    for _ in 0..config.warmups {
        let solution =
            candidate
                .solve(black_box(&instance))
                .map_err(|detail| BenchError::Candidate {
                    phase: "warmup solver",
                    detail,
                })?;
        black_box(
            candidate
                .check(black_box(&instance), black_box(&solution))
                .map_err(|detail| BenchError::Candidate {
                    phase: "warmup checker",
                    detail,
                })?,
        );
    }

    let capacity = usize::try_from(config.samples)
        .map_err(|_| BenchError::InvalidConfig("sample count does not fit usize"))?;
    let mut solver_ns = Vec::with_capacity(capacity);
    let mut checker_ns = Vec::with_capacity(capacity);
    let mut qualities = Vec::with_capacity(capacity);

    for _ in 0..config.samples {
        let solver_start = Instant::now();
        let solution =
            candidate
                .solve(black_box(&instance))
                .map_err(|detail| BenchError::Candidate {
                    phase: "solver sample",
                    detail,
                })?;
        solver_ns.push(elapsed_ns(solver_start.elapsed())?);

        let checker_start = Instant::now();
        let mut quality = None;
        for _ in 0..config.checker_repetitions {
            quality = Some(
                candidate
                    .check(black_box(&instance), black_box(&solution))
                    .map_err(|detail| BenchError::Candidate {
                        phase: "checker sample",
                        detail,
                    })?,
            );
            black_box(quality);
        }
        let checker_batch_ns = elapsed_ns(checker_start.elapsed())?;
        checker_ns.push(checker_per_operation(
            checker_batch_ns,
            config.checker_repetitions,
        )?);
        let quality = quality.ok_or(BenchError::InvalidConfig(
            "checker repetitions unexpectedly produced no observation",
        ))?;
        qualities.push(black_box(quality));
    }

    let solver = summarize(&solver_ns);
    let checker = summarize(&checker_ns);
    let asymmetry_milli = u128::from(solver.median_ns)
        .checked_mul(1_000)
        .and_then(|value| value.checked_div(u128::from(checker.median_ns.max(1))))
        .ok_or(BenchError::RatioOverflow)
        .and_then(|value| u64::try_from(value).map_err(|_| BenchError::RatioOverflow))?;
    let checker_budget =
        config
            .provisional_checker_budget_ns
            .map_or(EvidenceState::Unknown, |budget| {
                if checker.max_ns <= budget {
                    EvidenceState::Supported
                } else {
                    EvidenceState::Rejected
                }
            });
    let metadata = candidate.metadata();

    Ok(RunRecord {
        metadata,
        case_id: candidate.case_id(),
        config,
        prepare_ns: Some(prepare_ns),
        solver: Some(solver),
        checker: Some(checker),
        asymmetry_milli: Some(asymmetry_milli),
        quality_min: qualities.iter().copied().min(),
        quality_max: qualities.iter().copied().max(),
        checker_budget,
        recommendation: recommendation(metadata, checker_budget),
        note: "Timing is calibration evidence only; it does not prove distributional hardness."
            .to_owned(),
    })
}

/// Writes one stable-schema JSON object per candidate.
///
/// # Errors
///
/// Returns an I/O error if the target cannot be created or fully written.
pub fn write_jsonl(path: impl AsRef<Path>, records: &[RunRecord]) -> Result<(), BenchError> {
    let mut writer = BufWriter::new(File::create(path)?);
    for record in records {
        write_json_record(&mut writer, record)?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}

/// Writes a human-readable report derived only from the supplied run records.
///
/// # Errors
///
/// Returns an I/O error if the target cannot be created or fully written.
pub fn write_markdown(path: impl AsRef<Path>, records: &[RunRecord]) -> Result<(), BenchError> {
    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "# P-004 legacy-class admission calibration")?;
    writeln!(writer)?;
    writeln!(
        writer,
        "This report is generated from the adjacent JSONL evidence. `eligible_for_adr_review` is not admission; only a ratified ADR can admit a class. Timing observations do not prove hardness."
    )?;
    writeln!(
        writer,
        "Checker summaries are per-operation integer averages from bounded timed batches; every JSONL row records its batch size."
    )?;
    writeln!(writer)?;
    writeln!(
        writer,
        "| candidate | fixture | solver median ns | checker p95/max ns | median asymmetry | P-3 comparison | disposition |"
    )?;
    writeln!(writer, "|---|---|---:|---:|---:|---|---|")?;
    for record in records {
        let solver = record
            .solver
            .map_or_else(|| "n/a".to_owned(), |value| value.median_ns.to_string());
        let checker = record.checker.map_or_else(
            || "n/a".to_owned(),
            |value| format!("{}/{}", value.p95_ns, value.max_ns),
        );
        let asymmetry = record.asymmetry_milli.map_or_else(
            || "n/a".to_owned(),
            |value| format!("{}.{:03}x", value / 1_000, value % 1_000),
        );
        writeln!(
            writer,
            "| {} | {} | {solver} | {checker} | {asymmetry} | {} | {} |",
            markdown_cell(record.metadata.candidate),
            markdown_cell(record.case_id),
            record.checker_budget.as_str(),
            record.recommendation.as_str(),
        )?;
    }

    for record in records {
        writeln!(writer)?;
        writeln!(
            writer,
            "## {} — {}",
            markdown_inline(record.metadata.candidate),
            record.recommendation.as_str()
        )?;
        writeln!(writer)?;
        writeln!(
            writer,
            "- Family: {}",
            markdown_inline(record.metadata.family)
        )?;
        writeln!(
            writer,
            "- Source revision: {}",
            markdown_inline(record.metadata.source_revision)
        )?;
        writeln!(
            writer,
            "- Source paths: {}",
            markdown_inline(&record.metadata.source_paths.join(", "))
        )?;
        writeln!(writer, "- Fixture: {}", markdown_inline(record.case_id))?;
        writeln!(writer, "- Note: {}", markdown_inline(&record.note))?;
        writeln!(writer)?;
        writeln!(writer, "| required question | state | evidence basis |")?;
        writeln!(writer, "|---|---|---|")?;
        write_evidence_row(
            &mut writer,
            "hard sampled distribution",
            record.metadata.hardness,
        )?;
        write_evidence_row(
            &mut writer,
            "parameter floor",
            record.metadata.parameter_floor,
        )?;
        write_evidence_row(
            &mut writer,
            "A1/A2 derived instance",
            record.metadata.derived_instance,
        )?;
        write_evidence_row(&mut writer, "A3 pure checker", record.metadata.pure_checker)?;
        write_evidence_row(
            &mut writer,
            "checker within provisional P-3",
            EvidenceItem {
                state: record.checker_budget,
                basis: "maximum measured checker observation compared with the provisional 15 ms P-3 recommendation; G0 has not ratified the budget",
            },
        )?;
    }
    writer.flush()?;
    Ok(())
}

fn elapsed_ns(duration: Duration) -> Result<u64, BenchError> {
    u64::try_from(duration.as_nanos()).map_err(|_| BenchError::DurationOverflow)
}

fn checker_per_operation(batch_ns: u64, repetitions: u32) -> Result<u64, BenchError> {
    let per_operation = batch_ns / u64::from(repetitions);
    if per_operation == 0 {
        return Err(BenchError::TimerResolution);
    }
    Ok(per_operation)
}

fn summarize(observations: &[u64]) -> TimingSummary {
    let mut sorted = observations.to_vec();
    sorted.sort_unstable();
    let last = sorted.len() - 1;
    let median = sorted[last / 2 + last % 2];
    let p95 = sorted[last.saturating_mul(95).div_ceil(100)];
    TimingSummary {
        min_ns: sorted[0],
        median_ns: median,
        p95_ns: p95,
        max_ns: sorted[last],
    }
}

fn recommendation(metadata: CandidateMetadata, checker_budget: EvidenceState) -> Recommendation {
    let states = [
        metadata.hardness.state,
        metadata.parameter_floor.state,
        metadata.derived_instance.state,
        metadata.pure_checker.state,
        checker_budget,
    ];
    if states.contains(&EvidenceState::Rejected) {
        Recommendation::Reject
    } else if states
        .iter()
        .all(|state| *state == EvidenceState::Supported)
    {
        Recommendation::EligibleForAdrReview
    } else {
        Recommendation::InsufficientEvidence
    }
}

fn write_evidence_row<W: Write>(
    writer: &mut W,
    question: &str,
    evidence: EvidenceItem,
) -> io::Result<()> {
    writeln!(
        writer,
        "| {} | {} | {} |",
        markdown_cell(question),
        evidence.state.as_str(),
        markdown_cell(evidence.basis)
    )
}

fn markdown_cell(value: &str) -> String {
    value
        .replace('|', "\\|")
        .replace(['\r', '\n'], " ")
        .trim()
        .to_owned()
}

fn markdown_inline(value: &str) -> String {
    value
        .replace(['\r', '\n'], " ")
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .trim()
        .to_owned()
}

fn write_json_record<W: Write>(writer: &mut W, record: &RunRecord) -> io::Result<()> {
    write!(
        writer,
        "{{\"schema_version\":{SCHEMA_VERSION},\"candidate\":"
    )?;
    write_json_string(writer, record.metadata.candidate)?;
    write!(writer, ",\"family\":")?;
    write_json_string(writer, record.metadata.family)?;
    write!(writer, ",\"source_revision\":")?;
    write_json_string(writer, record.metadata.source_revision)?;
    write!(writer, ",\"source_paths\":[")?;
    for (index, path) in record.metadata.source_paths.iter().enumerate() {
        if index != 0 {
            writer.write_all(b",")?;
        }
        write_json_string(writer, path)?;
    }
    write!(writer, "],\"case_id\":")?;
    write_json_string(writer, record.case_id)?;
    write!(
        writer,
        ",\"config\":{{\"warmups\":{},\"samples\":{},\"checker_repetitions\":{},\"provisional_checker_budget_ns\":",
        record.config.warmups, record.config.samples, record.config.checker_repetitions
    )?;
    write_optional_u64(writer, record.config.provisional_checker_budget_ns)?;
    write!(writer, "}},\"prepare_ns\":")?;
    write_optional_u64(writer, record.prepare_ns)?;
    write!(writer, ",\"solver_ns\":")?;
    write_timing(writer, record.solver)?;
    write!(writer, ",\"checker_ns\":")?;
    write_timing(writer, record.checker)?;
    write!(writer, ",\"asymmetry_milli\":")?;
    write_optional_u64(writer, record.asymmetry_milli)?;
    write!(writer, ",\"quality_min\":")?;
    write_optional_u64(writer, record.quality_min)?;
    write!(writer, ",\"quality_max\":")?;
    write_optional_u64(writer, record.quality_max)?;
    write!(writer, ",\"evidence\":{{\"hardness\":")?;
    write_evidence(writer, record.metadata.hardness)?;
    write!(writer, ",\"parameter_floor\":")?;
    write_evidence(writer, record.metadata.parameter_floor)?;
    write!(writer, ",\"derived_instance\":")?;
    write_evidence(writer, record.metadata.derived_instance)?;
    write!(writer, ",\"pure_checker\":")?;
    write_evidence(writer, record.metadata.pure_checker)?;
    write!(writer, "}},\"checker_budget_state\":")?;
    write_json_string(writer, record.checker_budget.as_str())?;
    write!(writer, ",\"recommendation\":")?;
    write_json_string(writer, record.recommendation.as_str())?;
    write!(writer, ",\"note\":")?;
    write_json_string(writer, &record.note)?;
    writer.write_all(b"}")
}

fn write_optional_u64<W: Write>(writer: &mut W, value: Option<u64>) -> io::Result<()> {
    match value {
        Some(number) => write!(writer, "{number}"),
        None => writer.write_all(b"null"),
    }
}

fn write_timing<W: Write>(writer: &mut W, value: Option<TimingSummary>) -> io::Result<()> {
    match value {
        Some(timing) => write!(
            writer,
            "{{\"min\":{},\"median\":{},\"p95\":{},\"max\":{}}}",
            timing.min_ns, timing.median_ns, timing.p95_ns, timing.max_ns
        ),
        None => writer.write_all(b"null"),
    }
}

fn write_evidence<W: Write>(writer: &mut W, evidence: EvidenceItem) -> io::Result<()> {
    write!(writer, "{{\"state\":")?;
    write_json_string(writer, evidence.state.as_str())?;
    write!(writer, ",\"basis\":")?;
    write_json_string(writer, evidence.basis)?;
    writer.write_all(b"}")
}

fn write_json_string<W: Write>(writer: &mut W, value: &str) -> io::Result<()> {
    writer.write_all(b"\"")?;
    for character in value.chars() {
        match character {
            '"' => writer.write_all(b"\\\"")?,
            '\\' => writer.write_all(b"\\\\")?,
            '\u{08}' => writer.write_all(b"\\b")?,
            '\u{0c}' => writer.write_all(b"\\f")?,
            '\n' => writer.write_all(b"\\n")?,
            '\r' => writer.write_all(b"\\r")?,
            '\t' => writer.write_all(b"\\t")?,
            control if control <= '\u{1f}' => write!(writer, "\\u{:04x}", u32::from(control))?,
            other => {
                let mut encoded = [0_u8; 4];
                writer.write_all(other.encode_utf8(&mut encoded).as_bytes())?;
            }
        }
    }
    writer.write_all(b"\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use cj3_classes::sis::{Sis, SisInstance, SisParameters, SisSolution};

    const SUPPORTED: EvidenceItem = EvidenceItem {
        state: EvidenceState::Supported,
        basis: "test basis",
    };

    const fn metadata(candidate: &'static str) -> CandidateMetadata {
        CandidateMetadata {
            candidate,
            family: "test",
            source_revision: "0123456789abcdef",
            source_paths: &["test/source.rs"],
            hardness: SUPPORTED,
            parameter_floor: SUPPORTED,
            derived_instance: SUPPORTED,
            pure_checker: SUPPORTED,
        }
    }

    struct EchoCandidate;

    impl BenchCandidate for EchoCandidate {
        type Instance = u64;
        type Solution = u64;

        fn metadata(&self) -> CandidateMetadata {
            metadata("echo\n\"")
        }

        fn case_id(&self) -> &'static str {
            "one"
        }

        fn prepare(&self) -> Result<Self::Instance, String> {
            Ok(7)
        }

        fn solve(&self, instance: &Self::Instance) -> Result<Self::Solution, String> {
            Ok(*instance)
        }

        fn check(
            &self,
            instance: &Self::Instance,
            solution: &Self::Solution,
        ) -> Result<u64, String> {
            if instance == solution {
                Ok(1)
            } else {
                Err("mismatch".to_owned())
            }
        }
    }

    #[test]
    fn runs_bounded_samples_and_escapes_json() {
        let record = run_candidate(
            &EchoCandidate,
            RunConfig {
                warmups: 1,
                samples: 3,
                checker_repetitions: 10,
                provisional_checker_budget_ns: Some(u64::MAX),
            },
        )
        .expect("bounded candidate runs");
        assert_eq!(record.recommendation, Recommendation::EligibleForAdrReview);
        assert_eq!(record.quality_min, Some(1));
        assert_eq!(record.quality_max, Some(1));

        let mut output = Vec::new();
        write_json_record(&mut output, &record).expect("JSON writes");
        let output = String::from_utf8(output).expect("JSON is UTF-8");
        assert!(output.contains("\"candidate\":\"echo\\n\\\"\""));
        assert!(!output.contains("echo\n"));
    }

    #[test]
    fn rejects_unbounded_configs_before_candidate_code() {
        let error = run_candidate(
            &EchoCandidate,
            RunConfig {
                warmups: 0,
                samples: 0,
                checker_repetitions: 1,
                provisional_checker_budget_ns: None,
            },
        )
        .expect_err("zero samples fail closed");
        assert!(matches!(error, BenchError::InvalidConfig(_)));
    }

    #[test]
    fn rejects_checker_batches_below_timer_resolution() {
        assert!(matches!(
            checker_per_operation(9_999, 10_000),
            Err(BenchError::TimerResolution)
        ));
        assert_eq!(
            checker_per_operation(10_000, 10_000).expect("one nanosecond resolves"),
            1
        );
    }

    struct NoSolution;

    impl ProblemClassSolver<Sis<0>> for NoSolution {
        fn solve(&self, _instance: &SisInstance) -> Result<SisSolution, String> {
            Err("fixture intentionally has no solver".to_owned())
        }
    }

    #[test]
    fn accepts_any_existing_problem_class_through_the_generic_adapter() {
        let parameters = SisParameters::new(1, 2, 17, 2).expect("valid tiny tuple");
        let candidate = ProblemClassCandidate::<Sis<0>, _>::new(
            metadata("sis-generic-shape"),
            "compile-only",
            [0_u8; 32],
            parameters,
            NoSolution,
        );
        assert_eq!(candidate.metadata().candidate, "sis-generic-shape");
        assert_eq!(candidate.case_id(), "compile-only");
        assert_eq!(
            candidate.prepare().expect("derives").parameters(),
            parameters
        );
    }
}
