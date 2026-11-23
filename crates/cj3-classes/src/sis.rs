//! Sampled Short Integer Solution (SIS) instance derivation and checking.
//!
//! Concrete hardness is an assumption supported by reductions, estimator models,
//! and measurements; it is not a per-instance proof.
//!
//! This module holds the validity rules of a SIS tuple, the derivation, the checker
//! and the SIS solution encoding. The 20-byte size-parameter layout and the class
//! identifier are imported from `cj3-types`. No tuple is named here: every function
//! that needs one takes it as an argument (ruling G0-C8e).

use cj3_types::{
    ClassId, DecodeError, SisSizeParam,
    codec::{Reader, len_u64},
};
use sha3::{
    Shake256,
    digest::{ExtendableOutput, Update, XofReader},
};

use crate::{Invalid, ProblemClass, QUALITY_SCALE, Quality};

/// The largest squared norm bound that encoding v1 admits: `32767^2`.
///
/// A coefficient is a 16-bit two's-complement integer. The checker rejects a
/// coefficient whose square exceeds the bound, so under a larger bound some valid
/// solutions could not be written (Protocol Spec §5.2, ruling G0-C4).
pub const MAX_BETA_SQUARED: u64 = 1_073_676_289;

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
    /// `beta_squared` is above `32767^2`, so the tuple is not admissible under
    /// encoding v1.
    NormBoundExceedsEncoding,
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
    /// norm admits the trivial modulus vector, quality cannot fit, the bound is above
    /// [`MAX_BETA_SQUARED`], or allocation dimensions cannot be represented on the
    /// current target.
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
        if beta_squared > MAX_BETA_SQUARED {
            return Err(ParameterError::NormBoundExceedsEncoding);
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

    /// Validates the tuple that a 20-byte size-parameter layout carries.
    ///
    /// The layout is bytes; this is where its four values are judged.
    ///
    /// # Errors
    ///
    /// Returns [`ParameterError`] exactly as [`Self::new`] does.
    pub fn from_layout(layout: SisSizeParam) -> Result<Self, ParameterError> {
        Self::new(layout.n, layout.m, layout.q, layout.beta_squared)
    }

    /// Returns the tuple as the 20-byte layout carries it.
    #[must_use]
    pub const fn layout(self) -> SisSizeParam {
        SisSizeParam {
            n: self.n,
            m: self.m,
            q: self.q,
            beta_squared: self.beta_squared,
        }
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
/// A coefficient is a 16-bit two's-complement integer, which is exactly what the
/// encoding carries. Every value of this type therefore has one encoding, and every
/// input of the right length has one value (Protocol Spec §5.2, ruling G0-C3e).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SisSolution(Box<[i16]>);

impl SisSolution {
    /// Bytes per coefficient.
    pub const COEFFICIENT_LEN: usize = 2;

    /// Wraps a coefficient vector.
    #[must_use]
    pub fn new(coefficients: Vec<i16>) -> Self {
        Self(coefficients.into_boxed_slice())
    }

    /// Returns the signed coefficients in index order.
    #[must_use]
    pub fn coefficients(&self) -> &[i16] {
        &self.0
    }

    /// Encodes the coefficients in index order, each a 16-bit two's-complement
    /// integer, little-endian.
    ///
    /// The result is the solution bytes of a block when the vector has `m`
    /// coefficients for the block's tuple; [`Self::decode`] accepts no other length.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.0.len().saturating_mul(Self::COEFFICIENT_LEN));
        for coefficient in &self.0 {
            bytes.extend_from_slice(&coefficient.to_le_bytes());
        }
        bytes
    }

    /// Decodes solution bytes for the tuple given as an argument.
    ///
    /// The input must be exactly `2 * m` bytes; that length comes from the imported
    /// size-parameter layout. Every such input decodes. Whether the vector is a
    /// solution is [`check_sis`]'s question, not the decoder's.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::SolutionLengthMismatch`] when the length is not `2 * m`.
    pub fn decode(input: &[u8], parameters: SisParameters) -> Result<Self, DecodeError> {
        let expected = parameters.layout().solution_len();
        let actual = len_u64(input.len());
        if actual != expected {
            return Err(DecodeError::SolutionLengthMismatch { expected, actual });
        }
        let mut reader = Reader::new(input);
        let mut coefficients = Vec::with_capacity(input.len() / Self::COEFFICIENT_LEN);
        while reader.remaining() != 0 {
            coefficients.push(i16::from_le_bytes(reader.array()?));
        }
        Ok(Self::new(coefficients))
    }
}

/// The SIS class, bound to the identifier that `cj3-types` assigns it.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Sis;

impl ProblemClass for Sis {
    const CLASS_ID: u16 = ClassId::Sis.to_u16();
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
            <Sis as ProblemClass>::derive_instance([0_u8; 32], parameters(2, 3, 3))
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

    /// NON-NORMATIVE stand-ins for the active tuple (ruling G0-C8e; P-007 resume
    /// ruling, clause 2.3). Both are tuples already measured in `bench/p003-sis`. The
    /// choice makes neither a candidate for P-4, and no library code names a tuple.
    fn non_normative_stand_ins() -> [SisParameters; 2] {
        [
            SisParameters::new(8, 112, 67, 112).expect("measured tuple is valid"),
            SisParameters::new(16, 288, 257, 288).expect("measured tuple is valid"),
        ]
    }

    #[test]
    fn sis_is_bound_to_the_identifier_the_class_table_assigns() {
        assert_eq!(<Sis as ProblemClass>::CLASS_ID, 1);
        assert_eq!(<Sis as ProblemClass>::CLASS_ID, ClassId::Sis.to_u16());
    }

    #[test]
    fn a_norm_bound_above_32767_squared_is_not_admissible() {
        assert_eq!(MAX_BETA_SQUARED, 32_767 * 32_767);
        let q = 4_294_967_291;
        assert!(SisParameters::new(1, 2, q, MAX_BETA_SQUARED).is_ok());
        assert_eq!(
            SisParameters::new(1, 2, q, MAX_BETA_SQUARED + 1),
            Err(ParameterError::NormBoundExceedsEncoding)
        );
    }

    #[test]
    fn a_tuple_and_its_layout_convert_both_ways_and_the_layout_is_judged() {
        for parameters in non_normative_stand_ins() {
            let layout = parameters.layout();
            assert_eq!(layout.n, parameters.n());
            assert_eq!(layout.m, parameters.m());
            assert_eq!(layout.q, parameters.q());
            assert_eq!(layout.beta_squared, parameters.beta_squared());
            assert_eq!(SisParameters::from_layout(layout), Ok(parameters));

            let mut not_prime = layout;
            not_prime.q += 1;
            assert_eq!(
                SisParameters::from_layout(not_prime),
                Err(ParameterError::NonPrimeModulus)
            );
        }
    }

    #[test]
    fn solution_bytes_are_sixteen_bit_twos_complement_little_endian_in_index_order() {
        let solution = SisSolution::new(vec![1, -1, 0, i16::MAX, i16::MIN, 258, -2]);
        assert_eq!(
            solution.encode(),
            [
                0x01, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff, 0x7f, 0x00, 0x80, 0x02, 0x01, 0xfe, 0xff
            ]
        );
    }

    #[test]
    fn solution_codec_round_trips_at_each_stand_in_tuple() {
        for parameters in non_normative_stand_ins() {
            let m = usize::try_from(parameters.m()).expect("small");
            let coefficients = (0..m)
                .map(|index| match index % 5 {
                    0 => i16::MAX,
                    1 => i16::MIN,
                    2 => -1,
                    3 => 0,
                    _ => i16::try_from(index).expect("small"),
                })
                .collect::<Vec<_>>();
            let solution = SisSolution::new(coefficients);
            let bytes = solution.encode();
            assert_eq!(bytes.len(), 2 * m);
            let decoded = SisSolution::decode(&bytes, parameters).expect("exact length decodes");
            assert_eq!(decoded, solution);
            assert_eq!(decoded.encode(), bytes);
        }
    }

    #[test]
    fn solution_length_holds_at_the_boundary_one_below_and_one_above() {
        for parameters in non_normative_stand_ins() {
            let exact = 2 * usize::try_from(parameters.m()).expect("small");
            let bytes = vec![0_u8; exact + 1];
            assert!(SisSolution::decode(&bytes[..exact], parameters).is_ok());
            for wrong in [0, 1, exact - 2, exact - 1, exact + 1] {
                assert_eq!(
                    SisSolution::decode(&bytes[..wrong], parameters),
                    Err(DecodeError::SolutionLengthMismatch {
                        expected: u64::try_from(exact).expect("small"),
                        actual: u64::try_from(wrong).expect("small"),
                    })
                );
            }
        }
    }

    #[test]
    fn a_decoded_vector_goes_to_the_checker_which_applies_the_coefficient_guard() {
        // Every 2 * m bytes decode; the checker, not the decoder, judges the vector.
        let [parameters, _] = non_normative_stand_ins();
        let m = usize::try_from(parameters.m()).expect("small");
        let instance = SisInstance::from_column_major(
            parameters,
            vec![0_u32; usize::try_from(parameters.n()).expect("small") * m],
        )
        .expect("zero matrix is in range");

        let mut bytes = vec![0_u8; 2 * m];
        assert_eq!(
            check_sis(
                &instance,
                &SisSolution::decode(&bytes, parameters).expect("decodes")
            ),
            Err(Invalid::ZeroVector)
        );

        // 11 * 11 = 121 exceeds beta_squared = 112: rejected before it adds to the norm.
        bytes[2..4].copy_from_slice(&11_i16.to_le_bytes());
        assert_eq!(
            check_sis(
                &instance,
                &SisSolution::decode(&bytes, parameters).expect("decodes")
            ),
            Err(Invalid::CoefficientOutOfRange { index: 1 })
        );

        // The most negative coefficient has no positive counterpart in 16 bits and is
        // still judged by its square.
        bytes[2..4].copy_from_slice(&i16::MIN.to_le_bytes());
        assert_eq!(
            check_sis(
                &instance,
                &SisSolution::decode(&bytes, parameters).expect("decodes")
            ),
            Err(Invalid::CoefficientOutOfRange { index: 1 })
        );

        bytes[2..4].copy_from_slice(&(-10_i16).to_le_bytes());
        let quality = check_sis(
            &instance,
            &SisSolution::decode(&bytes, parameters).expect("decodes"),
        )
        .expect("the zero matrix accepts any short nonzero vector");
        assert_eq!(quality.get(), 112 * QUALITY_SCALE / 100);
    }
}
