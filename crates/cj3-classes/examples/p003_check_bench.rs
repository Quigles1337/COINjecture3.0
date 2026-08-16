//! Integer-only P-003 checker microbenchmark.
//!
//! The fixture is an explicit all-zero matrix with a unit solution, forcing every
//! matrix-vector term to be visited while remaining valid. It is a validation-cost
//! fixture, not a protocol-distributed SIS instance and not a hardness experiment.

#![forbid(unsafe_code)]

use std::{env, hint::black_box, process::ExitCode, time::Instant};

use cj3_classes::sis::{SisInstance, SisParameters, SisSolution, check_sis};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("P003 checker benchmark: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<(), String> {
    let values = env::args().skip(1).collect::<Vec<_>>();
    if values.len() != 5 {
        return Err(String::from(
            "usage: p003_check_bench <n> <m> <q> <beta-squared> <samples>",
        ));
    }
    let n = parse::<u32>(&values[0], "n")?;
    let m = parse::<u32>(&values[1], "m")?;
    let q = parse::<u32>(&values[2], "q")?;
    let beta_squared = parse::<u64>(&values[3], "beta-squared")?;
    let samples = parse::<usize>(&values[4], "samples")?;
    if samples == 0 {
        return Err(String::from("samples must be nonzero"));
    }

    let parameters = SisParameters::new(n, m, q, beta_squared)
        .map_err(|error| format!("invalid parameters: {error:?}"))?;
    let entries = usize::try_from(u64::from(n) * u64::from(m))
        .map_err(|_| String::from("matrix is too large for this target"))?;
    let instance = SisInstance::from_column_major(parameters, vec![0_u32; entries])
        .map_err(|error| format!("fixture construction failed: {error:?}"))?;
    let solution_len = usize::try_from(m).map_err(|_| String::from("m does not fit usize"))?;
    let mut coefficients = vec![0_i64; solution_len];
    coefficients[0] = 1;
    let solution = SisSolution::new(coefficients);

    check_sis(black_box(&instance), black_box(&solution))
        .map_err(|error| format!("warmup fixture failed: {error:?}"))?;
    let mut durations = Vec::with_capacity(samples);
    for _ in 0..samples {
        let started = Instant::now();
        let quality = check_sis(black_box(&instance), black_box(&solution))
            .map_err(|error| format!("fixture failed: {error:?}"))?;
        black_box(quality);
        durations.push(started.elapsed().as_nanos());
    }
    durations.sort_unstable();
    let total = durations.iter().copied().sum::<u128>();
    let median = durations[durations.len() / 2];
    let p95_index = ((durations.len() - 1) * 95) / 100;
    let p95 = durations[p95_index];

    println!(
        "{{\"n\":{n},\"m\":{m},\"q\":{q},\"beta_squared\":{beta_squared},\"samples\":{samples},\"min_ns\":{},\"median_ns\":{median},\"p95_ns\":{p95},\"max_ns\":{},\"total_ns\":{total}}}",
        durations[0],
        durations[durations.len() - 1]
    );
    Ok(())
}

fn parse<T>(value: &str, label: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    value
        .parse::<T>()
        .map_err(|_| format!("{label} is not a valid decimal integer"))
}
