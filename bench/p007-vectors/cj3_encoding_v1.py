"""CJ3 Encoding v1: a second, independent derivation (P-007, ruling G0-C8).

Python standard library only. Nothing here is imported by, linked into or generated
from any Rust crate; it exists so that every layout, preimage, expansion and signature
verdict of Protocol Spec v0.2 sections 1.1 to 1.3, 5.2, 6.1, 6.3 and 7 is derived twice
(v0.2-DRAFT when this was written; the same text became v0.2, normative, on
2026-11-23). D8 permits Python only under bench/, never in the node.

The functions follow the specification text, not the Rust code.
"""

import hashlib

# ---------------------------------------------------------------------------
# Section 1.1: grammar. Integers are fixed width and little-endian.
# ---------------------------------------------------------------------------


def u16(value):
    assert 0 <= value < 1 << 16
    return value.to_bytes(2, "little")


def u32(value):
    assert 0 <= value < 1 << 32
    return value.to_bytes(4, "little")


def u64(value):
    assert 0 <= value < 1 << 64
    return value.to_bytes(8, "little")


def fixed(data, length):
    """A fixed-length byte array [N] is its N bytes, with no length in front."""
    assert isinstance(data, (bytes, bytearray)) and len(data) == length
    return bytes(data)


# ---------------------------------------------------------------------------
# Section 1.2: hash, domain tags and preimages.
# ---------------------------------------------------------------------------


def tag(text):
    """A domain tag is 16 bytes: the ASCII text, padded on the right with 0x00."""
    raw = text.encode("ascii")
    assert len(raw) <= 16
    return raw + b"\x00" * (16 - len(raw))


D_ADDR = tag("CJ3/v1/addr")
D_TX = tag("CJ3/v1/tx")
D_HDR = tag("CJ3/v1/header")
D_INST = tag("CJ3/v1/instance")
D_SOL = tag("CJ3/v1/solution")
D_BEACON = tag("CJ3/v1/beacon")
D_GENESIS = tag("CJ3/v1/genesis")

TAGS = [
    ("D_ADDR", "CJ3/v1/addr", D_ADDR),
    ("D_TX", "CJ3/v1/tx", D_TX),
    ("D_HDR", "CJ3/v1/header", D_HDR),
    ("D_INST", "CJ3/v1/instance", D_INST),
    ("D_SOL", "CJ3/v1/solution", D_SOL),
    ("D_BEACON", "CJ3/v1/beacon", D_BEACON),
    ("D_GENESIS", "CJ3/v1/genesis", D_GENESIS),
]


def H(data):
    """H is SHA-256."""
    return hashlib.sha256(data).digest()


def addr_preimage(public_key):
    return D_ADDR + fixed(public_key, 32)


def addr(public_key):
    return H(addr_preimage(public_key))


def signing_preimage(network_id, sender, recipient, amount, fee, nonce):
    return (
        D_TX
        + u32(network_id)
        + fixed(sender, 32)
        + fixed(recipient, 32)
        + u64(amount)
        + u64(fee)
        + u64(nonce)
    )


def instance_seed_preimage(network_id, height, parent_hash, beacon_out):
    return D_INST + u32(network_id) + u64(height) + fixed(parent_hash, 32) + fixed(beacon_out, 32)


def sol_commit_preimage(class_id, instance_seed, solution, miner_addr):
    # The inner hash is over the solution bytes alone, without the block's count.
    return (
        D_SOL
        + u16(class_id)
        + fixed(instance_seed, 32)
        + hashlib.sha256(solution).digest()
        + fixed(miner_addr, 32)
    )


def beacon_input_preimage(parent_hash):
    return D_BEACON + fixed(parent_hash, 32)


def header_hash_preimage(header):
    return D_HDR + header


# ---------------------------------------------------------------------------
# Sections 5.2, 6.1, 6.3 and 7: layouts.
# ---------------------------------------------------------------------------

CLASS_ID_SIS = 1


def transaction_fields(network_id, sender, recipient, amount, fee, nonce, pubkey, sig):
    """The transaction layout as (name, bytes) pairs, in order: 188 bytes."""
    return [
        ("network_id u32", u32(network_id)),
        ("from [32]", fixed(sender, 32)),
        ("to [32]", fixed(recipient, 32)),
        ("amount u64", u64(amount)),
        ("fee u64", u64(fee)),
        ("nonce u64", u64(nonce)),
        ("pubkey [32]", fixed(pubkey, 32)),
        ("sig [64]", fixed(sig, 64)),
    ]


def size_param_fields(n, m, q, beta_squared):
    """The SIS size-parameter layout: 20 bytes."""
    return [
        ("n u32", u32(n)),
        ("m u32", u32(m)),
        ("q u32", u32(q)),
        ("beta_squared u64", u64(beta_squared)),
    ]


def header_fields(
    network_id,
    height,
    parent_hash,
    beacon_out,
    instance_seed,
    class_id,
    size_param,
    hash_target,
    tx_root,
    state_root,
    sol_commit,
    timestamp,
    miner_addr,
    nonce,
):
    """The header layout: 306 bytes with the SIS anchor."""
    return [
        ("network_id u32", u32(network_id)),
        ("height u64", u64(height)),
        ("parent_hash [32]", fixed(parent_hash, 32)),
        ("beacon_out [32]", fixed(beacon_out, 32)),
        ("instance_seed [32]", fixed(instance_seed, 32)),
        ("class_id u16", u16(class_id)),
        ("size_param (SIS, 20 bytes)", fixed(size_param, 20)),
        ("hash_target [32]", fixed(hash_target, 32)),
        ("tx_root [32]", fixed(tx_root, 32)),
        ("state_root [32]", fixed(state_root, 32)),
        ("sol_commit [32]", fixed(sol_commit, 32)),
        ("timestamp u64", u64(timestamp)),
        ("miner_addr [32]", fixed(miner_addr, 32)),
        ("nonce u64", u64(nonce)),
    ]


def solution_bytes(coefficients):
    """m coefficients in index order, each a 16-bit two's-complement integer, LE."""
    out = bytearray()
    for coefficient in coefficients:
        assert -32768 <= coefficient <= 32767
        out += coefficient.to_bytes(2, "little", signed=True)
    return bytes(out)


def block_fields(header, solution, transactions):
    """The block layout: header, u32 length, solution, u32 count, transactions."""
    fields = [
        ("header", header),
        ("solution length u32", u32(len(solution))),
        ("solution bytes", solution),
        ("transaction count u32", u32(len(transactions))),
    ]
    for index, transaction in enumerate(transactions):
        fields.append(("transaction %d" % index, fixed(transaction, 188)))
    return fields


def join(fields):
    return b"".join(data for _, data in fields)


# ---------------------------------------------------------------------------
# Section 5.2: the expansion convention (ruling G0-C5) and the checker.
# ---------------------------------------------------------------------------


def expand(seed, n, m, q):
    """Expand a matrix from SHAKE-256 over the 32 seed bytes and nothing else.

    The output is read four bytes at a time, each group a little-endian u32. A group is
    accepted when it is below floor(2^32 / q) * q, and it yields its value modulo q. A
    rejected group is used up whole. Accepted values fill the matrix column by column,
    rows 0 to n-1 within a column, until n * m are placed.

    Returns (entries in column-major order, groups consumed, indexes of rejected
    groups, the acceptance limit).
    """
    assert len(seed) == 32
    limit = ((1 << 32) // q) * q
    wanted = n * m
    size = 4 * wanted + 64
    while True:
        stream = hashlib.shake_256(seed).digest(size)
        entries = []
        rejected = []
        consumed = 0
        for offset in range(0, len(stream) - 3, 4):
            word = int.from_bytes(stream[offset : offset + 4], "little")
            if word < limit:
                entries.append(word % q)
            else:
                rejected.append(consumed)
            consumed += 1
            if len(entries) == wanted:
                return entries, consumed, rejected, limit
        size *= 2


def expansion_words(seed, count):
    """The first `count` four-byte groups of the SHAKE-256 output, as integers."""
    stream = hashlib.shake_256(seed).digest(4 * count)
    return [int.from_bytes(stream[i : i + 4], "little") for i in range(0, 4 * count, 4)]


QUALITY_SCALE = 1_000_000


def sis_check(entries, n, m, q, beta_squared, coefficients):
    """Section 5.2 check(A, s). Returns ("ok", norm_squared, quality) or (reason,)."""
    if len(coefficients) != m:
        return ("wrong-length",)
    norm_squared = 0
    for index, coefficient in enumerate(coefficients):
        square = coefficient * coefficient
        if square > beta_squared:
            return ("coefficient-out-of-range", index)
        norm_squared += square
        if norm_squared > beta_squared:
            return ("norm-too-large",)
    if not any(coefficients):
        return ("zero-vector",)
    for row in range(n):
        total = 0
        for column, coefficient in enumerate(coefficients):
            total += entries[column * n + row] * coefficient
        if total % q != 0:
            return ("modular-relation", row)
    return ("ok", norm_squared, (beta_squared * QUALITY_SCALE) // norm_squared)


def is_prime(value):
    if value < 2:
        return False
    if value % 2 == 0:
        return value == 2
    divisor = 3
    while divisor * divisor <= value:
        if value % divisor == 0:
            return False
        divisor += 2
    return True


# ---------------------------------------------------------------------------
# Section 1.3: Ed25519 as RFC 8032 section 5.1, pure.
# ---------------------------------------------------------------------------

P = 2**255 - 19
L = 2**252 + 27742317777372353535851937790883648493
D = (-121665 * pow(121666, P - 2, P)) % P
SQRT_M1 = pow(2, (P - 1) // 4, P)
IDENTITY = (0, 1, 1, 0)


def _inv(value):
    return pow(value, P - 2, P)


def point_add(left, right):
    """Extended twisted Edwards addition (RFC 8032 section 5.1.4). Complete."""
    a = (left[1] - left[0]) * (right[1] - right[0]) % P
    b = (left[1] + left[0]) * (right[1] + right[0]) % P
    c = 2 * left[3] * right[3] * D % P
    d = 2 * left[2] * right[2] % P
    e, f, g, h = b - a, d - c, d + c, b + a
    return (e * f % P, g * h % P, f * g % P, e * h % P)


def point_mul(scalar, point):
    result = IDENTITY
    while scalar > 0:
        if scalar & 1:
            result = point_add(result, point)
        point = point_add(point, point)
        scalar >>= 1
    return result


def point_equal(left, right):
    if (left[0] * right[2] - right[0] * left[2]) % P != 0:
        return False
    if (left[1] * right[2] - right[1] * left[2]) % P != 0:
        return False
    return True


def point_decode(encoded):
    """RFC 8032 section 5.1.3, exactly. Returns (point, None) or (None, reason)."""
    assert len(encoded) == 32
    value = int.from_bytes(encoded, "little")
    sign = value >> 255
    y = value & ((1 << 255) - 1)
    # A y-coordinate not below p fails.
    if y >= P:
        return None, "y-not-below-p"
    x_squared = (y * y - 1) * _inv(D * y * y + 1) % P
    x = pow(x_squared, (P + 3) // 8, P)
    if (x * x - x_squared) % P != 0:
        x = x * SQRT_M1 % P
    # A y with no matching x fails.
    if (x * x - x_squared) % P != 0:
        return None, "no-matching-x"
    # An x of zero with the sign bit set fails.
    if x == 0 and sign == 1:
        return None, "zero-x-with-sign-bit"
    if (x & 1) != sign:
        x = P - x
    return (x, y, 1, x * y % P), None


def point_encode(point):
    z_inverse = _inv(point[2])
    x = point[0] * z_inverse % P
    y = point[1] * z_inverse % P
    return (y | ((x & 1) << 255)).to_bytes(32, "little")


_BASE_Y = 4 * _inv(5) % P
BASE, _ = point_decode(_BASE_Y.to_bytes(32, "little"))
assert BASE is not None


def is_small_order(point):
    """True for the eight points whose order divides 8, the identity included."""
    return point_equal(point_mul(8, point), IDENTITY)


def verify_clauses(public_key, signature, message):
    """Evaluate section 1.3 clause by clause.

    Returns a dict with one entry per clause and the verdict. The verdict is "ACCEPT"
    or the first clause that rejects, in the fixed order: (a) for A, (a) for R, (b) for
    A, (b) for R, (c), (e). Clauses that can still be evaluated after a rejection are
    evaluated and recorded, so that the record shows every clause, not only the first.
    """
    assert len(public_key) == 32 and len(signature) == 64
    r_bytes, s_bytes = signature[:32], signature[32:]

    # (a) Decode A and R as points by RFC 8032 section 5.1.3 exactly.
    a_point, a_reason = point_decode(public_key)
    r_point, r_reason = point_decode(r_bytes)

    # (b) Reject if A or R is one of the eight points whose order divides 8.
    a_small = None if a_point is None else is_small_order(a_point)
    r_small = None if r_point is None else is_small_order(r_point)

    # (c) Read S as a little-endian integer. Reject unless S < L.
    s = int.from_bytes(s_bytes, "little")
    s_below_l = s < L

    # (d) k = SHA-512(R || A || M) over the bytes as received, reduced modulo L.
    k = int.from_bytes(hashlib.sha512(r_bytes + public_key + message).digest(), "little") % L

    # (e) Accept if and only if [S]B = R + [k]A. Nothing is multiplied by 8.
    equation = None
    if a_point is not None and r_point is not None:
        equation = point_equal(point_mul(s, BASE), point_add(r_point, point_mul(k, a_point)))

    if a_reason is not None:
        verdict = "a:A:" + a_reason
    elif r_reason is not None:
        verdict = "a:R:" + r_reason
    elif a_small:
        verdict = "b:A"
    elif r_small:
        verdict = "b:R"
    elif not s_below_l:
        verdict = "c"
    elif not equation:
        verdict = "e"
    else:
        verdict = "ACCEPT"

    def tri(value):
        return "n/a" if value is None else ("yes" if value else "no")

    return {
        "a_A": "ok" if a_reason is None else a_reason,
        "a_R": "ok" if r_reason is None else r_reason,
        "b_A_small": tri(a_small),
        "b_R_small": tri(r_small),
        "c_S_below_L": tri(s_below_l),
        "e_equation": tri(equation),
        "verdict": verdict,
    }


def secret_expand(secret_key):
    assert len(secret_key) == 32
    digest = hashlib.sha512(secret_key).digest()
    scalar = int.from_bytes(digest[:32], "little")
    scalar &= (1 << 254) - 8
    scalar |= 1 << 254
    return scalar, digest[32:]


def public_key_of(secret_key):
    scalar, _ = secret_expand(secret_key)
    return point_encode(point_mul(scalar, BASE))


def sign(secret_key, message):
    """RFC 8032 section 5.1.6. Used only to make fixture signatures."""
    scalar, prefix = secret_expand(secret_key)
    public_key = point_encode(point_mul(scalar, BASE))
    r = int.from_bytes(hashlib.sha512(prefix + message).digest(), "little") % L
    r_bytes = point_encode(point_mul(r, BASE))
    k = int.from_bytes(hashlib.sha512(r_bytes + public_key + message).digest(), "little") % L
    s = (r + k * scalar) % L
    return r_bytes + s.to_bytes(32, "little")
