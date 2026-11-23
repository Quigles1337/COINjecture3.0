//! Ed25519 verification, exactly as Protocol Spec §1.3 states it (ruling G0-C7,
//! which ratifies GAP-12).
//!
//! Ed25519 as RFC 8032 section 5.1, pure (no context, no prehash). Every acceptance
//! rule is written in this module, clause by clause. `curve25519-dalek` supplies only
//! arithmetic: square roots in the field, group operations and scalars. No library's
//! `verify` or `verify_strict` is called, and none is taken as §1.3; conformance is
//! shown by the vectors under `bench/p007-vectors/ed25519/`.
//!
//! This is the one implementation. Every crate that verifies a signature imports it.

use core::cmp::Ordering;

use curve25519_dalek::{
    edwards::{CompressedEdwardsY, EdwardsPoint},
    scalar::Scalar,
};
use sha2::{Digest as _, Sha512};

use crate::keys::{PublicKey, Signature};

/// `p = 2^255 - 19`, little-endian.
const P: [u8; 32] = [
    0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
];

/// `p - 1`, little-endian: the y-coordinate `-1`.
const P_MINUS_ONE: [u8; 32] = [
    0xec, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
];

/// `1`, little-endian: the y-coordinate of the identity.
const ONE: [u8; 32] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// `L = 2^252 + 27742317777372353535851937790883648493`, the order of the base
/// point, little-endian.
const L: [u8; 32] = [
    0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
];

/// Which encoded point a rejection is about.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointRole {
    /// `A`, the public key.
    A,
    /// `R`, the first 32 bytes of the signature.
    R,
}

/// The §1.3 clause that rejects a signature.
///
/// When more than one clause rejects, the one reported follows a fixed order: clause
/// (a) for `A` and then for `R`, clause (b) for `A` and then for `R`, clause (c), then
/// clause (e). That order is not a consensus matter: every variant rejects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignatureError {
    /// Clause (a): a y-coordinate not below `p = 2^255 - 19`.
    YNotBelowP(PointRole),
    /// Clause (a): a y with no matching x.
    NoMatchingX(PointRole),
    /// Clause (a): an x of zero with the sign bit set.
    ZeroXWithSignBit(PointRole),
    /// Clause (b): one of the eight points whose order divides 8, the identity
    /// included.
    SmallOrder(PointRole),
    /// Clause (c): `S`, read as a little-endian integer, is not below `L`.
    SNotBelowL,
    /// Clause (e): `[S]B = R + [k]A` does not hold.
    EquationFailed,
}

/// Compares two 32-byte little-endian integers.
fn is_below(value: &[u8; 32], bound: &[u8; 32]) -> bool {
    value.iter().rev().cmp(bound.iter().rev()) == Ordering::Less
}

/// Clause (a): decodes a point by RFC 8032 section 5.1.3 exactly.
fn decode_point(role: PointRole, encoded: &[u8; 32]) -> Result<EdwardsPoint, SignatureError> {
    let sign_bit_set = encoded[31] & 0x80 != 0;
    let mut y = *encoded;
    y[31] &= 0x7f;

    // A y-coordinate not below p fails.
    if !is_below(&y, &P) {
        return Err(SignatureError::YNotBelowP(role));
    }
    // A y with no matching x fails.
    let point = CompressedEdwardsY(*encoded)
        .decompress()
        .ok_or(SignatureError::NoMatchingX(role))?;
    // An x of zero with the sign bit set fails. On this curve x is zero exactly when
    // y is 1 or -1.
    if sign_bit_set && (y == ONE || y == P_MINUS_ONE) {
        return Err(SignatureError::ZeroXWithSignBit(role));
    }
    Ok(point)
}

/// Verifies an Ed25519 signature exactly as Protocol Spec §1.3 states.
///
/// Inputs: public key `A`, 32 bytes; signature `R ‖ S`, 32 and 32 bytes; message `M`.
/// On a consensus path `M` is the 32-byte signing digest of §1.2, and
/// [`crate::tx::Transaction::verify_signature`] passes exactly that. The message is a
/// slice so that the RFC 8032 and CCTV test vectors, whose messages have other
/// lengths, run through this same function.
///
/// - **(a)** Decode `A` and `R` as points by RFC 8032 section 5.1.3 exactly.
/// - **(b)** Reject if `A` or `R` is one of the eight points whose order divides 8.
/// - **(c)** Read `S` as a little-endian integer. Reject unless `S < L`.
/// - **(d)** `k = SHA-512(R ‖ A ‖ M)` over the bytes as received, read little-endian
///   and reduced modulo `L`.
/// - **(e)** Accept if and only if `[S]B = R + [k]A`. The form multiplied through by 8
///   is not used, and there is no batch verification.
/// - **(f)** An all-zero signature and an all-zero key are rejected by (b).
///
/// # Errors
///
/// Returns the [`SignatureError`] of the first clause that rejects.
pub fn verify(
    public_key: &PublicKey,
    signature: &Signature,
    message: &[u8],
) -> Result<(), SignatureError> {
    let a_bytes = public_key.as_bytes();
    let (Some(r_bytes), Some(s_bytes)) = (
        signature.as_bytes().first_chunk::<32>(),
        signature.as_bytes().last_chunk::<32>(),
    ) else {
        // A 64-byte array always has both halves; this arm rejects and cannot run.
        return Err(SignatureError::EquationFailed);
    };

    // (a)
    let a = decode_point(PointRole::A, a_bytes)?;
    let r = decode_point(PointRole::R, r_bytes)?;

    // (b)
    if a.is_small_order() {
        return Err(SignatureError::SmallOrder(PointRole::A));
    }
    if r.is_small_order() {
        return Err(SignatureError::SmallOrder(PointRole::R));
    }

    // (c)
    if !is_below(s_bytes, &L) {
        return Err(SignatureError::SNotBelowL);
    }
    let s = Scalar::from_bytes_mod_order(*s_bytes);

    // (d)
    let mut hasher = Sha512::new();
    hasher.update(r_bytes);
    hasher.update(a_bytes);
    hasher.update(message);
    let wide: [u8; 64] = hasher.finalize().into();
    let k = Scalar::from_bytes_mod_order_wide(&wide);

    // (e) [S]B = R + [k]A, tested as [k](-A) + [S]B = R. Nothing is multiplied by 8.
    let left = EdwardsPoint::vartime_double_scalar_mul_basepoint(&k, &-a, &s);
    if left == r {
        Ok(())
    } else {
        Err(SignatureError::EquationFailed)
    }
}

#[cfg(test)]
mod tests {
    use curve25519_dalek::{edwards::CompressedEdwardsY, scalar::Scalar, traits::Identity};

    use super::{L, ONE, P, P_MINUS_ONE, PointRole, SignatureError, is_below, verify};
    use crate::{
        keys::{PublicKey, Signature},
        test_support::unhex,
    };

    /// Builds the clause (a) rejection for the point it is given.
    type ClauseAReason = fn(PointRole) -> SignatureError;

    // RFC 8032 section 7.1, TEST 1 (empty message).
    const RFC_KEY: &str = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
    const RFC_SIG: &str = "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155\
                           5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b";

    /// The eight encodings of the points whose order divides 8.
    const SMALL_ORDER: [&str; 8] = [
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000080",
        "0100000000000000000000000000000000000000000000000000000000000000",
        "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
        "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
        "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
        "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
        "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
    ];

    /// The six other encodings that a lenient decoder maps to those points, with the
    /// clause (a) reason each must fail for.
    const NON_CANONICAL: [(&str, ClauseAReason); 6] = [
        (
            "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
            SignatureError::YNotBelowP,
        ),
        (
            "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            SignatureError::YNotBelowP,
        ),
        (
            "0100000000000000000000000000000000000000000000000000000000000080",
            SignatureError::ZeroXWithSignBit,
        ),
        (
            "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
            SignatureError::YNotBelowP,
        ),
        (
            "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            SignatureError::YNotBelowP,
        ),
        (
            "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            SignatureError::ZeroXWithSignBit,
        ),
    ];

    fn rfc_key() -> PublicKey {
        PublicKey::from_bytes(unhex(RFC_KEY))
    }

    fn rfc_signature() -> Signature {
        Signature::from_bytes(unhex(RFC_SIG))
    }

    fn with_r(r: [u8; 32]) -> Signature {
        let mut bytes = *rfc_signature().as_bytes();
        bytes[..32].copy_from_slice(&r);
        Signature::from_bytes(bytes)
    }

    #[test]
    fn l_is_the_order_of_the_base_point() {
        assert_eq!(Scalar::from_bytes_mod_order(L), Scalar::ZERO);
        let mut l_minus_one = L;
        l_minus_one[0] -= 1;
        assert_eq!(Scalar::from_bytes_mod_order(l_minus_one), -Scalar::ONE);
        assert!(is_below(&l_minus_one, &L));
        assert!(!is_below(&L, &L));
    }

    #[test]
    fn p_is_the_field_modulus() {
        // The arithmetic library reduces y modulo p, so y = p decodes there as y = 0
        // and y = p + 1 as y = 1. That agreement pins the constant.
        let zero = CompressedEdwardsY([0_u8; 32]).decompress();
        assert!(zero.is_some());
        assert_eq!(CompressedEdwardsY(P).decompress(), zero);
        let mut p_plus_one = P;
        p_plus_one[0] += 1;
        assert_eq!(
            CompressedEdwardsY(p_plus_one).decompress(),
            CompressedEdwardsY(ONE).decompress()
        );
        // y = p - 1 is the point of order 2.
        let order_two = CompressedEdwardsY(P_MINUS_ONE)
            .decompress()
            .expect("y = -1 is on the curve");
        let identity = curve25519_dalek::edwards::EdwardsPoint::identity();
        assert_ne!(order_two, identity);
        assert_eq!(order_two + order_two, identity);
        assert!(is_below(&P_MINUS_ONE, &P));
        assert!(!is_below(&P, &P));
    }

    #[test]
    fn little_endian_comparison_orders_by_the_most_significant_byte() {
        let mut low = [0xff_u8; 32];
        low[31] = 0x00;
        let mut high = [0x00_u8; 32];
        high[31] = 0x01;
        assert!(is_below(&low, &high));
        assert!(!is_below(&high, &low));
        assert!(!is_below(&low, &low));
    }

    #[test]
    fn rfc_8032_test_1_verifies_and_any_change_of_message_does_not() {
        assert_eq!(verify(&rfc_key(), &rfc_signature(), b""), Ok(()));
        assert_eq!(
            verify(&rfc_key(), &rfc_signature(), b"\0"),
            Err(SignatureError::EquationFailed)
        );
    }

    #[test]
    fn clause_c_rejects_s_plus_l_which_names_the_same_scalar() {
        let signature = rfc_signature();
        let mut bytes = *signature.as_bytes();
        // S + L, as a 256-bit little-endian addition.
        let mut carry = 0_u16;
        for (byte, addend) in bytes[32..].iter_mut().zip(L) {
            let sum = u16::from(*byte) + u16::from(addend) + carry;
            *byte = u8::try_from(sum & 0xff).expect("masked to a byte");
            carry = sum >> 8;
        }
        assert_eq!(carry, 0, "S + L fits 256 bits");
        assert_eq!(
            verify(&rfc_key(), &Signature::from_bytes(bytes), b""),
            Err(SignatureError::SNotBelowL)
        );

        let mut exactly_l = *signature.as_bytes();
        exactly_l[32..].copy_from_slice(&L);
        assert_eq!(
            verify(&rfc_key(), &Signature::from_bytes(exactly_l), b""),
            Err(SignatureError::SNotBelowL)
        );
    }

    #[test]
    fn clause_b_rejects_each_of_the_eight_small_order_points_as_a_and_as_r() {
        for encoded in SMALL_ORDER {
            let point: [u8; 32] = unhex(encoded);
            assert_eq!(
                verify(&PublicKey::from_bytes(point), &rfc_signature(), b""),
                Err(SignatureError::SmallOrder(PointRole::A)),
                "A = {encoded}"
            );
            assert_eq!(
                verify(&rfc_key(), &with_r(point), b""),
                Err(SignatureError::SmallOrder(PointRole::R)),
                "R = {encoded}"
            );
        }
    }

    #[test]
    fn clause_a_rejects_each_alternative_encoding_as_a_and_as_r() {
        for (encoded, reason) in NON_CANONICAL {
            let point: [u8; 32] = unhex(encoded);
            assert_eq!(
                verify(&PublicKey::from_bytes(point), &rfc_signature(), b""),
                Err(reason(PointRole::A)),
                "A = {encoded}"
            );
            assert_eq!(
                verify(&rfc_key(), &with_r(point), b""),
                Err(reason(PointRole::R)),
                "R = {encoded}"
            );
        }
    }

    #[test]
    fn clause_a_rejects_a_y_with_no_matching_x() {
        // y = 2 is not the y-coordinate of any point on the curve.
        let mut two = [0_u8; 32];
        two[0] = 2;
        assert_eq!(
            verify(&PublicKey::from_bytes(two), &rfc_signature(), b""),
            Err(SignatureError::NoMatchingX(PointRole::A))
        );
        assert_eq!(
            verify(&rfc_key(), &with_r(two), b""),
            Err(SignatureError::NoMatchingX(PointRole::R))
        );
    }

    #[test]
    fn clause_f_an_all_zero_signature_and_an_all_zero_key_are_rejected_by_b() {
        assert_eq!(
            verify(&rfc_key(), &Signature::from_bytes([0_u8; 64]), b""),
            Err(SignatureError::SmallOrder(PointRole::R))
        );
        assert_eq!(
            verify(&PublicKey::from_bytes([0_u8; 32]), &rfc_signature(), b""),
            Err(SignatureError::SmallOrder(PointRole::A))
        );
        assert_eq!(
            verify(
                &PublicKey::from_bytes([0_u8; 32]),
                &Signature::from_bytes([0_u8; 64]),
                b""
            ),
            Err(SignatureError::SmallOrder(PointRole::A))
        );
    }

    #[test]
    fn a_is_judged_before_r_and_decoding_before_small_order() {
        let (non_canonical, reason) = NON_CANONICAL[0];
        let small: [u8; 32] = unhex(SMALL_ORDER[0]);
        // A is small order and R does not decode: clause (a) for R comes first.
        assert_eq!(
            verify(
                &PublicKey::from_bytes(small),
                &with_r(unhex(non_canonical)),
                b""
            ),
            Err(reason(PointRole::R))
        );
        // Both fail clause (a): A is reported.
        assert_eq!(
            verify(
                &PublicKey::from_bytes(unhex(non_canonical)),
                &with_r(unhex(non_canonical)),
                b""
            ),
            Err(reason(PointRole::A))
        );
    }
}
