//! Untrusted, out-of-process SIS solver demonstrator.
//!
//! This binary's text interface is a P-003 benchmark protocol, not a canonical wire
//! encoding. Nodes and consensus crates do not depend on it. Every candidate is
//! rechecked by `cj3-classes` before it reaches standard output.

#![forbid(unsafe_code)]

use std::{env, process::ExitCode};

use cj3_classes::sis::{SisInstance, SisParameters, SisSolution, check_sis, derive_instance};
use puremp::{Int, Rational, lll_reduce_delta};

fn main() -> ExitCode {
    match run() {
        Ok(solution) => {
            let line = solution
                .coefficients()
                .iter()
                .map(i64::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            println!("{line}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("P003 external solver: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<SisSolution, String> {
    let mut arguments = env::args();
    let program = arguments
        .next()
        .unwrap_or_else(|| String::from("cj3-solver-sis"));
    let values = arguments.collect::<Vec<_>>();
    if values.len() != 5 {
        return Err(format!(
            "usage: {program} <n> <m> <q> <beta-squared> <64-hex-char-seed>"
        ));
    }

    let n = parse_decimal::<u32>(&values[0], "n")?;
    let m = parse_decimal::<u32>(&values[1], "m")?;
    let q = parse_decimal::<u32>(&values[2], "q")?;
    let beta_squared = parse_decimal::<u64>(&values[3], "beta-squared")?;
    let seed = parse_seed(&values[4])?;
    let parameters = SisParameters::new(n, m, q, beta_squared)
        .map_err(|error| format!("invalid parameters: {error:?}"))?;
    let instance = derive_instance(seed, parameters);
    solve(&instance).ok_or_else(|| String::from("LLL produced no candidate within the bound"))
}

fn parse_decimal<T>(value: &str, label: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    value
        .parse::<T>()
        .map_err(|_| format!("{label} is not a valid decimal integer"))
}

fn parse_seed(value: &str) -> Result<[u8; 32], String> {
    if value.len() != 64 || !value.is_ascii() {
        return Err(String::from(
            "seed must contain exactly 64 ASCII hex digits",
        ));
    }
    let mut seed = [0_u8; 32];
    for (index, byte) in seed.iter_mut().enumerate() {
        let offset = index * 2;
        *byte = u8::from_str_radix(&value[offset..offset + 2], 16)
            .map_err(|_| String::from("seed contains a non-hex digit"))?;
    }
    Ok(seed)
}

fn solve(instance: &SisInstance) -> Option<SisSolution> {
    let basis = kernel_basis(instance)?;
    let integer_basis = basis
        .iter()
        .map(|row| row.iter().copied().map(Int::from_i64).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let delta = Rational::new(Int::from_i64(99), Int::from_i64(100));
    let reduced = lll_reduce_delta(&integer_basis, &delta);

    reduced.into_iter().find_map(|row| {
        let coefficients = row.iter().map(Int::to_i64).collect::<Option<Vec<_>>>()?;
        let candidate = SisSolution::new(coefficients);
        check_sis(instance, &candidate).ok().map(|_| candidate)
    })
}

fn kernel_basis(instance: &SisInstance) -> Option<Vec<Vec<i64>>> {
    let parameters = instance.parameters();
    let rows = usize::try_from(parameters.n()).ok()?;
    let columns = usize::try_from(parameters.m()).ok()?;
    let modulus = u64::from(parameters.q());
    let mut reduced = (0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| {
                    instance
                        .get(u32::try_from(row).ok()?, u32::try_from(column).ok()?)
                        .map(u64::from)
                })
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()?;

    let mut pivot_columns = Vec::with_capacity(rows);
    let mut pivot_row = 0_usize;
    for column in 0..columns {
        let selected = (pivot_row..rows).find(|&row| reduced[row][column] != 0);
        let Some(selected) = selected else {
            continue;
        };
        reduced.swap(pivot_row, selected);
        let inverse = modular_inverse(reduced[pivot_row][column], modulus)?;
        for value in &mut reduced[pivot_row] {
            *value = multiply_mod(*value, inverse, modulus);
        }
        let normalized = reduced[pivot_row].clone();
        for (row, values) in reduced.iter_mut().enumerate() {
            if row == pivot_row {
                continue;
            }
            let factor = values[column];
            if factor == 0 {
                continue;
            }
            for (value, pivot) in values.iter_mut().zip(&normalized) {
                let product = multiply_mod(factor, *pivot, modulus);
                *value = (*value + modulus - product) % modulus;
            }
        }
        pivot_columns.push(column);
        pivot_row += 1;
        if pivot_row == rows {
            break;
        }
    }

    let mut is_pivot = vec![false; columns];
    for &column in &pivot_columns {
        is_pivot[column] = true;
    }
    let mut basis = Vec::with_capacity(columns);
    for &column in &pivot_columns {
        let mut vector = vec![0_i64; columns];
        vector[column] = i64::from(parameters.q());
        basis.push(vector);
    }
    for free_column in (0..columns).filter(|&column| !is_pivot[column]) {
        let mut vector = vec![0_i64; columns];
        vector[free_column] = 1;
        for (row, &pivot_column) in pivot_columns.iter().enumerate() {
            let value = reduced[row][free_column];
            vector[pivot_column] = if value == 0 {
                0
            } else {
                i64::try_from(modulus - value).ok()?
            };
        }
        basis.push(vector);
    }
    (basis.len() == columns).then_some(basis)
}

fn multiply_mod(left: u64, right: u64, modulus: u64) -> u64 {
    let product = u128::from(left) * u128::from(right);
    u64::try_from(product % u128::from(modulus)).expect("modular product is below u64 modulus")
}

fn modular_inverse(value: u64, modulus: u64) -> Option<u64> {
    if value == 0 || modulus > i64::MAX as u64 {
        return None;
    }
    let (mut old_remainder, mut remainder) = (i128::from(modulus), i128::from(value));
    let (mut old_coefficient, mut coefficient) = (0_i128, 1_i128);
    while remainder != 0 {
        let quotient = old_remainder / remainder;
        (old_remainder, remainder) = (remainder, old_remainder - quotient * remainder);
        (old_coefficient, coefficient) = (coefficient, old_coefficient - quotient * coefficient);
    }
    (old_remainder == 1).then(|| {
        u64::try_from(old_coefficient.rem_euclid(i128::from(modulus)))
            .expect("inverse residue is below u64 modulus")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_seed_strictly() {
        assert_eq!(parse_seed(&"00".repeat(32)), Ok([0_u8; 32]));
        assert!(parse_seed("00").is_err());
        assert!(parse_seed(&format!("{}zz", "00".repeat(31))).is_err());
    }

    #[test]
    fn modular_inverse_covers_every_nonzero_residue() {
        for value in 1..17 {
            let inverse = modular_inverse(value, 17).expect("prime-field element is invertible");
            assert_eq!(multiply_mod(value, inverse, 17), 1);
        }
    }

    #[test]
    fn kernel_basis_rows_satisfy_relation_and_are_full_rank_by_construction() {
        let parameters = SisParameters::new(2, 5, 17, 5).expect("valid tuple");
        let instance =
            SisInstance::from_column_major(parameters, vec![1, 0, 0, 1, 2, 3, 4, 5, 6, 7])
                .expect("valid matrix");
        let basis = kernel_basis(&instance).expect("kernel basis");
        assert_eq!(basis.len(), 5);
        for row in basis {
            for matrix_row in 0..2_u32 {
                let residue = row.iter().enumerate().fold(0_i128, |sum, (column, value)| {
                    let entry = instance
                        .get(matrix_row, u32::try_from(column).expect("small test index"))
                        .expect("in bounds");
                    (sum + i128::from(entry) * i128::from(*value)).rem_euclid(17)
                });
                assert_eq!(residue, 0);
            }
        }
    }

    #[test]
    fn exact_lll_finds_and_rechecks_toy_witness() {
        let parameters = SisParameters::new(2, 11, 17, 11).expect("valid tuple");
        let instance = derive_instance([0_u8; 32], parameters);
        let solution = solve(&instance).expect("toy instance should be solved by exact LLL");
        assert!(check_sis(&instance, &solution).is_ok());
    }
}
