//! Sampled Short Integer Solution (SIS) instance derivation and checking.
//!
//! Concrete hardness is an assumption supported by reductions, estimator models,
//! and measurements; it is not a per-instance proof. P-003 intentionally leaves the
//! production class identifier and canonical solution codec to their owning gates.

use sha3::{
    Shake256,
    digest::{ExtendableOutput, Update, XofReader},
};

use crate::{Invalid, ProblemClass, QUALITY_SCALE, Quality};

/// Parameter validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterError {
    /// Both matrix dimensions must be nonzero.
    ZeroDimension,
    /// SIS requires more columns than rows.
    InsufficientColumns,
    /// The modulus must be prime.
    NonPrimeModulus,
    /// The squared-norm bound must be nonzero.
    ZeroNormBound,
    /// The bound would admit the trivial vector `q * e_i`.
    TrivialModulusVector,
    /// `beta_squared * QUALITY_SCALE` would not fit in the quality result type.
    QualityRangeOverflow,
    /// The matrix entry count is not representable on this target.
    DimensionOverflow,
}

/// Validated SIS tuple `(n, m, q, beta_squared)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SisParameters {
    n: u32,
    m: u32,
    q: u32,
    beta_squared: u64,
}

impl SisParameters {
    /// Validates and constructs one SIS parameter tuple.
    ///
    /// # Errors
    ///
    /// Returns [`ParameterError`] if dimensions are invalid, `q` is not prime, the
    /// norm admits the trivial modulus vector, quality cannot fit, or allocation
    /// dimensions cannot be represented on the current target.
    pub fn new(n: u32, m: u32, q: u32, beta_squared: u64) -> Result<Self, ParameterError> {
        if n == 0 || m == 0 {
            return Err(ParameterError::ZeroDimension);
        }
        if m <= n {
            return Err(ParameterError::InsufficientColumns);
        }
        if !is_prime(q) {
            return Err(ParameterError::NonPrimeModulus);
        }
        if beta_squared == 0 {
            return Err(ParameterError::ZeroNormBound);
        }
        if beta_squared >= u64::from(q) * u64::from(q) {
            return Err(ParameterError::TrivialModulusVector);
        }
        if beta_squared.checked_mul(QUALITY_SCALE).is_none() {
            return Err(ParameterError::QualityRangeOverflow);
        }
        let entries = u64::from(n)
            .checked_mul(u64::from(m))
            .ok_or(ParameterError::DimensionOverflow)?;
        usize::try_from(entries).map_err(|_| ParameterError::DimensionOverflow)?;

        Ok(Self {
            n,
            m,
            q,
            beta_squared,
        })
    }

    /// Returns the row count `n`.
    #[must_use]
    pub const fn n(self) -> u32 {
        self.n
    }

    /// Returns the column count `m`.
    #[must_use]
    pub const fn m(self) -> u32 {
        self.m
    }

    /// Returns the prime modulus `q`.
    #[must_use]
    pub const fn q(self) -> u32 {
        self.q
    }

    /// Returns the squared Euclidean bound `beta_squared`.
    #[must_use]
    pub const fn beta_squared(self) -> u64 {
        self.beta_squared
    }

    /// Returns whether a nonzero binary solution follows from the pigeonhole bound.
    ///
    /// This is a conservative integer certificate: `m > n * ceil(log2(q))`
    /// implies `2^m > q^n`. It can return `false` for tuples that satisfy the tighter
    /// inequality.
    #[must_use]
    pub fn guarantees_binary_solution(self) -> bool {
        u128::from(self.m) > u128::from(self.n) * u128::from(ceil_log2(self.q))
            && self.beta_squared >= u64::from(self.m)
    }

    fn entry_count(self) -> usize {
        let entries = u64::from(self.n) * u64::from(self.m);
        usize::try_from(entries).expect("validated SIS entry count fits usize")
    }
}

/// Deterministically expanded matrix, stored in protocol-specified column-major order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SisInstance {
    parameters: SisParameters,
    column_major: Box<[u32]>,
}

impl SisInstance {
    /// Builds an instance from already validated column-major field elements.
    ///
    /// This constructor is intended for tests and external measurement tools. Chain
    /// instances use [`derive_instance`].
    ///
    /// # Errors
    ///
    /// Returns [`InstanceError`] when the matrix length is wrong or an entry is not
    /// in the canonical range `[0, q)`.
    pub fn from_column_major(
        parameters: SisParameters,
        column_major: Vec<u32>,
    ) -> Result<Self, InstanceError> {
        let expected = parameters.entry_count();
        if column_major.len() != expected {
            return Err(InstanceError::WrongEntryCount {
                expected,
                actual: column_major.len(),
            });
        }
        if let Some((index, _)) = column_major
            .iter()
            .enumerate()
            .find(|(_, value)| **value >= parameters.q)
        {
            return Err(InstanceError::NonCanonicalEntry { index });
        }
        Ok(Self {
            parameters,
            column_major: column_major.into_boxed_slice(),
        })
    }

    /// Returns the validated parameter tuple.
    #[must_use]
    pub const fn parameters(&self) -> SisParameters {
        self.parameters
    }

    /// Returns the entry at `(row, column)`, or `None` when either index is outside
    /// the matrix.
    #[must_use]
    pub fn get(&self, row: u32, column: u32) -> Option<u32> {
        if row >= self.parameters.n || column >= self.parameters.m {
            return None;
        }
        let index = u64::from(column) * u64::from(self.parameters.n) + u64::from(row);
        let index = usize::try_from(index).ok()?;
        self.column_major.get(index).copied()
    }

    /// Returns the raw column-major field elements.
    #[must_use]
    pub fn as_column_major(&self) -> &[u32] {
        &self.column_major
    }
}

/// Explicit-instance construction failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceError {
    /// The supplied flat matrix has the wrong number of entries.
    WrongEntryCount {
        /// Required number of entries.
        expected: usize,
        /// Supplied number of entries.
        actual: usize,
    },
    /// One entry is not in `[0, q)`.
    NonCanonicalEntry {
        /// Index in the column-major array.
        index: usize,
    },
}

/// Decoded SIS coefficient vector.
///
/// This is deliberately not a wire codec. P-007 owns the canonical byte format and
/// strict decoding rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SisSolution(Box<[i64]>);

impl SisSolution {
    /// Wraps an already decoded coefficient vector.
    #[must_use]
    pub fn new(coefficients: Vec<i64>) -> Self {
        Self(coefficients.into_boxed_slice())
    }

    /// Returns the decoded signed coefficients.
    #[must_use]
    pub fn coefficients(&self) -> &[i64] {
        &self.0
    }
}

/// SIS class marker whose numeric registry identifier remains an explicit binding.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Sis<const ID: u16>;

impl<const ID: u16> ProblemClass for Sis<ID> {
    const CLASS_ID: u16 = ID;
    type SizeParam = SisParameters;
    type Instance = SisInstance;
    type Solution = SisSolution;

    fn derive_instance(seed: [u8; 32], size: Self::SizeParam) -> Self::Instance {
        derive_instance(seed, size)
    }

    fn check(inst: &Self::Instance, sol: &Self::Solution) -> Result<Quality, Invalid> {
        check_sis(inst, sol)
    }
}

/// Expands a uniform SIS matrix from SHAKE-256 in column-major order.
#[must_use]
pub fn derive_instance(seed: [u8; 32], parameters: SisParameters) -> SisInstance {
    let mut shake = Shake256::default();
    shake.update(&seed);
    let mut reader = shake.finalize_xof();
    let mut column_major = Vec::with_capacity(parameters.entry_count());
    let mut word_bytes = [0_u8; 4];
    while column_major.len() < parameters.entry_count() {
        reader.read(&mut word_bytes);
        let word = u32::from_le_bytes(word_bytes);
        if let Some(value) = reduce_uniform_word(word, parameters.q) {
            column_major.push(value);
        }
    }
    SisInstance {
        parameters,
        column_major: column_major.into_boxed_slice(),
    }
}

/// Checks a decoded SIS solution using exact integer arithmetic.
///
/// # Errors
///
/// Returns [`Invalid`] for malformed dimensions, zero or oversized vectors,
/// arithmetic overflow, a nonzero modular row, or an unrepresentable quality.
pub fn check_sis(inst: &SisInstance, sol: &SisSolution) -> Result<Quality, Invalid> {
    let coefficients = sol.coefficients();
    let expected = usize::try_from(inst.parameters.m).map_err(|_| Invalid::WrongLength {
        expected: usize::MAX,
        actual: coefficients.len(),
    })?;
    if coefficients.len() != expected {
        return Err(Invalid::WrongLength {
            expected,
            actual: coefficients.len(),
        });
    }

    let mut any_nonzero = false;
    let mut norm_squared = 0_u128;
    for (index, coefficient) in coefficients.iter().copied().enumerate() {
        any_nonzero |= coefficient != 0;
        let magnitude = u128::from(coefficient.unsigned_abs());
        let square = magnitude
            .checked_mul(magnitude)
            .ok_or(Invalid::NormOverflow)?;
        if square > u128::from(inst.parameters.beta_squared) {
            return Err(Invalid::CoefficientOutOfRange { index });
        }
        norm_squared = norm_squared
            .checked_add(square)
            .ok_or(Invalid::NormOverflow)?;
        if norm_squared > u128::from(inst.parameters.beta_squared) {
            return Err(Invalid::NormTooLarge);
        }
    }
    if !any_nonzero {
        return Err(Invalid::ZeroVector);
    }

    let modulus = i128::from(inst.parameters.q);
    for row in 0..inst.parameters.n {
        let mut residue = 0_i128;
        for (column, coefficient) in coefficients.iter().copied().enumerate() {
            let column = u32::try_from(column).map_err(|_| Invalid::RelationOverflow)?;
            let entry = inst.get(row, column).ok_or(Invalid::RelationOverflow)?;
            let product = i128::from(entry)
                .checked_mul(i128::from(coefficient))
                .ok_or(Invalid::RelationOverflow)?;
            residue = residue
                .checked_add(product)
                .ok_or(Invalid::RelationOverflow)?
                .rem_euclid(modulus);
        }
        if residue != 0 {
            return Err(Invalid::ModularRelation {
                row: usize::try_from(row).map_err(|_| Invalid::RelationOverflow)?,
            });
        }
    }

    let numerator = u128::from(inst.parameters.beta_squared)
        .checked_mul(u128::from(QUALITY_SCALE))
        .ok_or(Invalid::QualityOverflow)?;
    let raw = numerator
        .checked_div(norm_squared)
        .ok_or(Invalid::QualityOverflow)?;
    let raw = u64::try_from(raw).map_err(|_| Invalid::QualityOverflow)?;
    Ok(Quality::from_raw(raw))
}

fn reduce_uniform_word(word: u32, q: u32) -> Option<u32> {
    let range = u64::from(u32::MAX) + 1;
    let acceptance_limit = (range / u64::from(q)) * u64::from(q);
    (u64::from(word) < acceptance_limit).then_some(word % q)
}

fn ceil_log2(value: u32) -> u32 {
    u32::BITS - (value - 1).leading_zeros()
}

fn is_prime(value: u32) -> bool {
    if value < 2 {
        return false;
    }
    if value.is_multiple_of(2) {
        return value == 2;
    }
    let mut divisor = 3_u32;
    while u64::from(divisor) * u64::from(divisor) <= u64::from(value) {
        if value.is_multiple_of(divisor) {
            return false;
        }
        divisor += 2;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parameters(n: u32, m: u32, beta_squared: u64) -> SisParameters {
        SisParameters::new(n, m, 17, beta_squared).expect("test tuple is valid")
    }

    #[test]
    fn validates_parameter_invariants_and_quality_range() {
        assert_eq!(
            SisParameters::new(0, 2, 17, 2),
            Err(ParameterError::ZeroDimension)
        );
        assert_eq!(
            SisParameters::new(2, 2, 17, 2),
            Err(ParameterError::InsufficientColumns)
        );
        assert_eq!(
            SisParameters::new(1, 2, 15, 2),
            Err(ParameterError::NonPrimeModulus)
        );
        assert_eq!(
            SisParameters::new(1, 2, 17, 0),
            Err(ParameterError::ZeroNormBound)
        );
        assert_eq!(
            SisParameters::new(1, 2, 17, 289),
            Err(ParameterError::TrivialModulusVector)
        );
        assert_eq!(
            SisParameters::new(1, 2, 4_294_967_291, u64::MAX / QUALITY_SCALE + 1),
            Err(ParameterError::QualityRangeOverflow)
        );
    }

    #[test]
    fn identifies_binary_pigeonhole_regime() {
        let sufficient = SisParameters::new(2, 11, 17, 11).expect("valid tuple");
        let too_few_columns = SisParameters::new(2, 10, 17, 10).expect("valid tuple");
        let too_small_norm = SisParameters::new(2, 11, 17, 10).expect("valid tuple");
        assert!(sufficient.guarantees_binary_solution());
        assert!(!too_few_columns.guarantees_binary_solution());
        assert!(!too_small_norm.guarantees_binary_solution());
    }

    #[test]
    fn shake_expansion_matches_independent_vector_and_column_order() {
        let instance = derive_instance([0_u8; 32], parameters(2, 3, 3));
        assert_eq!(instance.as_column_major(), &[4, 12, 6, 7, 3, 15]);
        assert_eq!(instance.get(0, 0), Some(4));
        assert_eq!(instance.get(1, 0), Some(12));
        assert_eq!(instance.get(0, 1), Some(6));
        assert_eq!(instance.get(1, 2), Some(15));
        assert_eq!(instance.get(2, 0), None);
        assert_eq!(instance.get(0, 3), None);
        assert_eq!(
            instance,
            <Sis<42> as ProblemClass>::derive_instance([0_u8; 32], parameters(2, 3, 3))
        );
    }

    #[test]
    fn rejection_sampling_excludes_incomplete_tail() {
        assert_eq!(reduce_uniform_word(u32::MAX, 17), None);
        assert_eq!(reduce_uniform_word(u32::MAX - 1, 17), Some(16));
        assert_eq!(reduce_uniform_word(u32::MAX, 2), Some(1));
    }

    #[test]
    fn explicit_instances_reject_shape_and_field_errors() {
        let params = parameters(1, 2, 2);
        assert_eq!(
            SisInstance::from_column_major(params, vec![1]),
            Err(InstanceError::WrongEntryCount {
                expected: 2,
                actual: 1
            })
        );
        assert_eq!(
            SisInstance::from_column_major(params, vec![1, 17]),
            Err(InstanceError::NonCanonicalEntry { index: 1 })
        );
    }

    #[test]
    fn accepts_relation_and_computes_integer_quality() {
        let params = parameters(1, 2, 4);
        let instance = SisInstance::from_column_major(params, vec![1, 1]).expect("valid matrix");
        let solution = SisSolution::new(vec![1, -1]);
        let quality = check_sis(&instance, &solution).expect("valid relation");
        assert_eq!(quality.get(), 2 * QUALITY_SCALE);
    }

    #[test]
    fn rejects_wrong_length_zero_relation_coefficient_and_norm() {
        let params = parameters(1, 3, 2);
        let instance = SisInstance::from_column_major(params, vec![1, 1, 0]).expect("valid matrix");
        assert_eq!(
            check_sis(&instance, &SisSolution::new(vec![1, -1])),
            Err(Invalid::WrongLength {
                expected: 3,
                actual: 2
            })
        );
        assert_eq!(
            check_sis(&instance, &SisSolution::new(vec![0, 0, 0])),
            Err(Invalid::ZeroVector)
        );
        assert_eq!(
            check_sis(&instance, &SisSolution::new(vec![1, 0, 0])),
            Err(Invalid::ModularRelation { row: 0 })
        );
        assert_eq!(
            check_sis(&instance, &SisSolution::new(vec![2, 0, 0])),
            Err(Invalid::CoefficientOutOfRange { index: 0 })
        );
        assert_eq!(
            check_sis(&instance, &SisSolution::new(vec![1, -1, 1])),
            Err(Invalid::NormTooLarge)
        );
    }
}
