"""Derive the P-007 vectors with the Python standard library only (ruling G0-C8).

    python -I bench/p007-vectors/derive.py            write the vector files
    python -I bench/p007-vectors/derive.py --check    fail if a committed file differs

Inputs, all committed beside this script:

    ed25519/rfc8032-section-7.1.txt      transcription of RFC 8032 section 7.1
    ed25519/cctv/ed25519vectors.json     C2SP CCTV ed25519 vectors, verbatim, pinned
    solver-output-n8.txt                 stdout of the out-of-process solver

Outputs, all generated and never edited by hand:

    encoding-v1.vectors.txt              G0-C8a: every layout and preimage, all bytes
    WORKED-VECTORS.md                    the same vectors, field by field, for reading
    shake-expansion.vectors.txt          G0-C8b: the expansion rule
    ed25519/c7-expected.vectors.txt      G0-C8c: section 1.3, clause by clause
    ed25519/C7-SUMMARY.md                counts of the above
    ../../fuzz/seeds/<target>/<name>     seed inputs for the fuzz targets: the worked
                                         bytes above, so the fuzzers start from inputs
                                         that decode

Status (Gate G0-F ruling, clause 2.4). Three of the outputs are NORMATIVE conformance
vectors from the merge of PR #21: encoding-v1.vectors.txt, shake-expansion.vectors.txt
and ed25519/c7-expected.vectors.txt. What is normative is the relation: these inputs
give these bytes and these verdicts, and every implementation reproduces them. The
status text of each file is written below; a change to it changes no vector value.

The input values assign nothing: no network identifier, fee, target, root, timestamp
unit or tuple. Every fixture value below is a test value. The two SIS tuples are
stand-ins for the active tuple (ruling G0-C8e; P-007 resume ruling, clause 2.3): both
were already measured in bench/p003-sis, the choice is non-normative, and it makes
neither a candidate for P-4.
"""

import hashlib
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import cj3_encoding_v1 as ref  # noqa: E402  (after the path insert, on purpose)

CCTV_COMMIT = "b88c4b206254ac543df240097e0f1fcc6a664821"
CCTV_BLOB = "ebb2192a14d88c26979c40be0aff4cfba6892ed3"
CCTV_SHA256 = "b38e84caf3e7e89170ff520292dbeae421b0a794c27408ce5ce973018fe3d7f9"
SOLVER_OUTPUT_SHA256 = "89b0713d8725ef2b1830b4209d4e55b7cc55fe0815c0abcab33f195b819e8873"

# NON-NORMATIVE stand-ins for the active tuple, both measured in bench/p003-sis.
STAND_IN_N8 = (8, 112, 67, 112)
STAND_IN_N16 = (16, 288, 257, 288)

# NON-NORMATIVE fixture values. Each is chosen so that its bytes show the byte order.
NETWORK_ID = 0x04030201
HEIGHT = 0x0000000000000102
PARENT_HASH = bytes(range(0x00, 0x20))
BEACON_OUT = bytes(range(0x20, 0x40))
HASH_TARGET = bytes(range(0x40, 0x60))  # opaque: its reading as a number is not ruled
TX_ROOT = bytes(range(0x60, 0x80))  # opaque: its construction is not ruled
STATE_ROOT = bytes(range(0x80, 0xA0))  # opaque: its construction is not ruled
TIMESTAMP = 0xA7A6A5A4A3A2A1A0  # its unit is not ruled
HEADER_NONCE = 0xB7B6B5B4B3B2B1B0

TX1 = {"signer": "1", "to_key": "2", "amount": 0x1122334455667788, "fee": 0x0102030405060708, "nonce": 0xA5}
# A self-send (V9) with a zero amount and the largest fee.
TX2 = {"signer": "2", "to_key": "2", "amount": 0, "fee": 0xFFFFFFFFFFFFFFFF, "nonce": 1}


# ---------------------------------------------------------------------------
# Small helpers.
# ---------------------------------------------------------------------------


def read_bytes(*parts):
    with open(os.path.join(HERE, *parts), "rb") as handle:
        return handle.read()


def read_key_values(*parts):
    values = {}
    for line in read_bytes(*parts).decode("ascii").splitlines():
        line = line.rstrip()
        if not line or line.startswith("#"):
            continue
        key, _, value = line.partition(" =")
        values[key] = value.strip()
    return values


def wrap_hex(data, width=32):
    """Hex in rows of `width` bytes; continuation rows start with two spaces."""
    text = data.hex()
    rows = [text[i : i + 2 * width] for i in range(0, len(text), 2 * width)] or [""]
    return "\n  ".join(rows)


def wrap_ints(values, width=16):
    rows = [" ".join(str(v) for v in values[i : i + width]) for i in range(0, len(values), width)]
    return "\n  ".join(rows or [""])


class Document:
    """A `key = value` text file. Continuation lines start with two spaces."""

    def __init__(self, header_lines):
        self.lines = ["# " + line if line else "#" for line in header_lines]
        self.lines.append("")

    def comment(self, text=""):
        self.lines.append(("# " + text).rstrip())

    def blank(self):
        self.lines.append("")

    def put(self, key, value):
        self.lines.append(("%s = %s" % (key, value)).rstrip())

    def put_int(self, key, value):
        self.put(key, str(value))

    def put_hex(self, key, data):
        self.put(key, wrap_hex(data))

    def put_ints(self, key, values):
        self.put(key, wrap_ints(values))

    def text(self):
        return "\n".join(self.lines).rstrip("\n") + "\n"


def field_table(fields):
    """A Markdown table: offset, length, field, bytes. Shows every byte."""
    rows = ["| Offset | Length | Field | Bytes (hex) |", "|---:|---:|---|---|"]
    offset = 0
    for name, data in fields:
        shown = "<br>".join(
            "`%s`" % data[i : i + 32].hex() for i in range(0, len(data), 32)
        ) or "(empty)"
        rows.append("| %d | %d | %s | %s |" % (offset, len(data), name, shown))
        offset += len(data)
    rows.append("| | **%d** | **total** | |" % offset)
    return "\n".join(rows)


def hex_block(data):
    rows = [data[i : i + 32].hex() for i in range(0, len(data), 32)]
    return "```text\n" + "\n".join(rows) + "\n```"


# ---------------------------------------------------------------------------
# G0-C8a: worked vectors for every layout and preimage.
# ---------------------------------------------------------------------------


def build_worked():
    rfc = read_key_values("ed25519", "rfc8032-section-7.1.txt")
    keys = {}
    for index in ("1", "2", "3"):
        secret = bytes.fromhex(rfc["rfc8032.%s.secret_key" % index])
        public = bytes.fromhex(rfc["rfc8032.%s.public_key" % index])
        # The transcription is checked against this script's own RFC 8032 code.
        assert ref.public_key_of(secret) == public, index
        keys[index] = (secret, public)

    doc = Document(
        [
            "CJ3 Encoding v1: worked vectors for every layout of ruling G0-C3 and every",
            "preimage of ruling G0-C2c, with all bytes shown (ruling G0-C8a).",
            "",
            "STATUS: NORMATIVE conformance vectors for the layouts and preimages of CJ3",
            "Encoding v1, from the merge of PR #21 (Gate G0-F ruling, clause 2.4). What is",
            "normative is the relation: these inputs give these bytes and these verdicts,",
            "and every implementation reproduces them. If the text of the specification",
            "and a vector ever disagree, that is a STOP and a ruling. Neither wins by",
            "default. The P-005 vectors stay symbolic and non-normative.",
            "",
            "The input values assign nothing: no network identifier, fee, target, root,",
            "timestamp unit or tuple. Every `in.` value is a fixture value. The tuple is a",
            "stand-in for the active tuple, already measured in bench/p003-sis; the choice",
            "is non-normative and makes it no candidate for P-4 (ruling G0-C8e; resume 2.3).",
            "The three keys are the public test keys of RFC 8032 section 7.1, tests 1 to 3.",
            "",
            "Derived twice: by derive.py (Python standard library only) and by the Rust",
            "tests in crates/cj3-types/tests and crates/cj3-classes/tests, which read the",
            "`in.` values, build the values with the Rust code and require every `out.`",
            "value. Written by derive.py. Do not edit by hand.",
            "",
            "Format: `key = value`. A line that starts with two spaces continues the value",
            "above it. Bytes are lower-case hex, and spaces inside them are not part of the",
            "value. Integers are decimal.",
        ]
    )
    md = [
        "# CJ3 Encoding v1 — worked vectors",
        "",
        "Generated by `derive.py` from the same values as `encoding-v1.vectors.txt`. Do not",
        "edit by hand. Every byte of every layout (ruling G0-C3) and of every preimage",
        "(ruling G0-C2c) is shown, field by field.",
        "",
        "**Status.** The normative conformance vectors for the layouts and preimages of CJ3",
        "Encoding v1 are in `encoding-v1.vectors.txt` (Gate G0-F ruling, clause 2.4). This",
        "file renders the same values for reading. The ruling names that file, not this one.",
        "The P-005 vectors stay symbolic and non-normative.",
        "",
        "**Fixtures.** Every input value is a non-normative fixture value. None assigns a",
        "network identifier, fee, timestamp unit, hash target, root or tuple. `hash_target`,",
        "`tx_root` and `state_root` are opaque patterns. The tuple `(8, 112, 67, 112)` stands",
        "in for the active tuple; it was already measured in `bench/p003-sis`, the choice is",
        "non-normative, and it makes the tuple no candidate for P-4 (ruling G0-C8e; resume",
        "2.3). The keys are the public test keys of RFC 8032 section 7.1, tests 1 to 3.",
        "",
        "Integers are little-endian. `H` is SHA-256.",
        "",
        "## Domain tags (§1.2)",
        "",
        "| Tag | ASCII text | The 16 bytes (hex) |",
        "|---|---|---|",
    ]
    for name, text, data in ref.TAGS:
        doc.put_hex("out.tag.%s" % name, data)
        md.append("| `%s` | `%s` | `%s` |" % (name, text, data.hex()))
    doc.blank()

    # --- addr(pk)
    md += ["", "## `addr(pk) = H(D_ADDR ‖ pk[32])` — 48-byte preimage", ""]
    addresses = {}
    doc.put_int("in.network_id", NETWORK_ID)
    for index in ("1", "2", "3"):
        secret, public = keys[index]
        preimage = ref.addr_preimage(public)
        address = ref.H(preimage)
        addresses[index] = address
        assert len(preimage) == 48
        doc.put_hex("in.key%s.secret_key" % index, secret)
        doc.put_hex("in.key%s.public_key" % index, public)
        doc.put_hex("out.addr.key%s.preimage" % index, preimage)
        doc.put_hex("out.addr.key%s" % index, address)
        md += [
            "### Key %s (RFC 8032 section 7.1, TEST %s)" % (index, index),
            "",
            field_table([("D_ADDR", ref.D_ADDR), ("pk [32]", public)]),
            "",
            "`addr` = `%s`" % address.hex(),
            "",
        ]
    doc.blank()

    # --- transactions
    transactions = []
    seeds = {}
    for name, spec in (("tx1", TX1), ("tx2", TX2)):
        secret, public = keys[spec["signer"]]
        sender = addresses[spec["signer"]]
        recipient = addresses[spec["to_key"]]
        preimage = ref.signing_preimage(
            NETWORK_ID, sender, recipient, spec["amount"], spec["fee"], spec["nonce"]
        )
        digest = ref.H(preimage)
        signature = ref.sign(secret, digest)
        assert ref.verify_clauses(public, signature, digest)["verdict"] == "ACCEPT"
        fields = ref.transaction_fields(
            NETWORK_ID, sender, recipient, spec["amount"], spec["fee"], spec["nonce"], public, signature
        )
        encoded = ref.join(fields)
        assert len(preimage) == 108 and len(encoded) == 188
        transactions.append(encoded)
        seeds[("transaction_codec", "worked-" + name)] = encoded
        if name == "tx1":
            seeds[("ed25519_verify", "worked-tx1")] = public + signature + digest

        doc.put("in.%s.signer" % name, "key" + spec["signer"])
        doc.put("in.%s.to" % name, "key" + spec["to_key"])
        doc.put_int("in.%s.amount" % name, spec["amount"])
        doc.put_int("in.%s.fee" % name, spec["fee"])
        doc.put_int("in.%s.nonce" % name, spec["nonce"])
        doc.put_hex("out.%s.signing_preimage" % name, preimage)
        doc.put_hex("out.%s.signing_digest" % name, digest)
        doc.put_hex("out.%s.sig" % name, signature)
        doc.put_hex("out.%s.bytes" % name, encoded)
        doc.blank()

        md += [
            "## Transaction `%s` — signer key %s, recipient `addr` of key %s"
            % (name, spec["signer"], spec["to_key"]),
            "",
            "`amount` = %d, `fee` = %d, `nonce` = %d." % (spec["amount"], spec["fee"], spec["nonce"]),
            "",
            "### Signing-digest preimage — 108 bytes",
            "",
            field_table(
                [
                    ("D_TX", ref.D_TX),
                    ("network_id u32", ref.u32(NETWORK_ID)),
                    ("from [32]", sender),
                    ("to [32]", recipient),
                    ("amount u64", ref.u64(spec["amount"])),
                    ("fee u64", ref.u64(spec["fee"])),
                    ("nonce u64", ref.u64(spec["nonce"])),
                ]
            ),
            "",
            "Signing digest (the 32-byte Ed25519 message) = `%s`" % digest.hex(),
            "",
            "### Transaction layout — 188 bytes",
            "",
            field_table(fields),
            "",
        ]

    # --- size parameter
    n, m, q, beta_squared = STAND_IN_N8
    size_fields = ref.size_param_fields(n, m, q, beta_squared)
    size_param = ref.join(size_fields)
    assert len(size_param) == 20
    doc.put_int("in.size_param.n", n)
    doc.put_int("in.size_param.m", m)
    doc.put_int("in.size_param.q", q)
    doc.put_int("in.size_param.beta_squared", beta_squared)
    doc.put_hex("out.size_param.bytes", size_param)
    doc.blank()
    md += [
        "## SIS size parameter — 20 bytes",
        "",
        "The non-normative stand-in tuple `(n, m, q, beta_squared) = (%d, %d, %d, %d)`." % STAND_IN_N8,
        "",
        field_table(size_fields),
        "",
    ]

    # --- instance seed and beacon input
    seed_preimage = ref.instance_seed_preimage(NETWORK_ID, HEIGHT, PARENT_HASH, BEACON_OUT)
    instance_seed = ref.H(seed_preimage)
    beacon_preimage = ref.beacon_input_preimage(PARENT_HASH)
    beacon_input = ref.H(beacon_preimage)
    assert len(seed_preimage) == 92 and len(beacon_preimage) == 48
    doc.put_int("in.header.height", HEIGHT)
    doc.put_hex("in.header.parent_hash", PARENT_HASH)
    doc.put_hex("in.header.beacon_out", BEACON_OUT)
    doc.put_hex("in.header.hash_target", HASH_TARGET)
    doc.put_hex("in.header.tx_root", TX_ROOT)
    doc.put_hex("in.header.state_root", STATE_ROOT)
    doc.put_int("in.header.timestamp", TIMESTAMP)
    doc.put("in.header.miner", "key3")
    doc.put_int("in.header.nonce", HEADER_NONCE)
    doc.put_hex("out.instance_seed.preimage", seed_preimage)
    doc.put_hex("out.instance_seed", instance_seed)
    doc.put_hex("out.beacon_input.preimage", beacon_preimage)
    doc.put_hex("out.beacon_input", beacon_input)
    doc.blank()
    md += [
        "## `instance_seed` — 92-byte preimage",
        "",
        field_table(
            [
                ("D_INST", ref.D_INST),
                ("network_id u32", ref.u32(NETWORK_ID)),
                ("height u64", ref.u64(HEIGHT)),
                ("parent_hash [32]", PARENT_HASH),
                ("beacon_out [32]", BEACON_OUT),
            ]
        ),
        "",
        "`instance_seed` = `%s`" % instance_seed.hex(),
        "",
        "## Beacon input — 48-byte preimage",
        "",
        field_table([("D_BEACON", ref.D_BEACON), ("parent_hash [32]", PARENT_HASH)]),
        "",
        "Beacon input = `%s`" % beacon_input.hex(),
        "",
    ]

    # --- solution
    solver_output = read_bytes("solver-output-n8.txt")
    assert hashlib.sha256(solver_output).hexdigest() == SOLVER_OUTPUT_SHA256, "solver output changed"
    coefficients = [int(token) for token in solver_output.split()]
    assert len(coefficients) == m
    solution = ref.solution_bytes(coefficients)
    assert len(solution) == 2 * m
    entries, _, _, _ = ref.expand(instance_seed, n, m, q)
    checked = ref.sis_check(entries, n, m, q, beta_squared, coefficients)
    assert checked[0] == "ok", checked
    inner = hashlib.sha256(solution).digest()
    doc.put_ints("in.solution.coefficients", coefficients)
    doc.put_hex("out.solution.bytes", solution)
    doc.put_hex("out.solution.sha256", inner)
    doc.put_int("out.solution.norm_squared", checked[1])
    doc.put_int("out.solution.quality", checked[2])
    doc.blank()
    md += [
        "## SIS solution — `2 * m` = %d bytes" % len(solution),
        "",
        "The coefficients are the output of the out-of-process solver for the instance that",
        "`instance_seed` derives under the stand-in tuple (`solver-output-n8.txt`). The",
        "standard-library checker of `cj3_encoding_v1.py` accepts them: squared norm %d," % checked[1],
        "quality `Q = floor(beta_squared * 1,000,000 / norm)` = %d." % checked[2],
        "",
        "Coefficients, in index order:",
        "",
        "```text",
        wrap_ints(coefficients).replace("\n  ", "\n"),
        "```",
        "",
        "Solution bytes: each coefficient a 16-bit two's-complement integer, little-endian.",
        "",
        hex_block(solution),
        "",
        "`SHA-256(solution bytes)` = `%s`" % inner.hex(),
        "",
    ]

    # --- sol_commit
    miner = addresses["3"]
    commit_preimage = ref.sol_commit_preimage(ref.CLASS_ID_SIS, instance_seed, solution, miner)
    sol_commit = ref.H(commit_preimage)
    assert len(commit_preimage) == 114
    doc.put_hex("out.sol_commit.preimage", commit_preimage)
    doc.put_hex("out.sol_commit", sol_commit)
    doc.blank()
    md += [
        "## `sol_commit` — 114-byte preimage",
        "",
        field_table(
            [
                ("D_SOL", ref.D_SOL),
                ("class_id u16", ref.u16(ref.CLASS_ID_SIS)),
                ("instance_seed [32]", instance_seed),
                ("SHA-256(solution bytes) [32]", inner),
                ("miner_addr [32]", miner),
            ]
        ),
        "",
        "`sol_commit` = `%s`" % sol_commit.hex(),
        "",
    ]

    # --- header and its hash
    head_fields = ref.header_fields(
        NETWORK_ID,
        HEIGHT,
        PARENT_HASH,
        BEACON_OUT,
        instance_seed,
        ref.CLASS_ID_SIS,
        size_param,
        HASH_TARGET,
        TX_ROOT,
        STATE_ROOT,
        sol_commit,
        TIMESTAMP,
        miner,
        HEADER_NONCE,
    )
    header = ref.join(head_fields)
    header_preimage = ref.header_hash_preimage(header)
    header_hash = ref.H(header_preimage)
    assert len(header) == 306 and len(header_preimage) == 322
    doc.put_hex("out.header.bytes", header)
    doc.put_hex("out.header_hash.preimage", header_preimage)
    doc.put_hex("out.header_hash", header_hash)
    doc.blank()
    md += [
        "## Header — 306 bytes with the SIS anchor",
        "",
        field_table(head_fields),
        "",
        "## Header hash — 322-byte preimage",
        "",
        field_table([("D_HDR", ref.D_HDR), ("Header", header)]),
        "",
        "Header hash = `%s`" % header_hash.hex(),
        "",
    ]

    # --- block
    block_fields = ref.block_fields(header, solution, transactions)
    block = ref.join(block_fields)
    assert len(block) == 306 + 4 + 2 * m + 4 + 188 * len(transactions)
    doc.put_hex("out.block.bytes", block)
    doc.put_int("out.block.length", len(block))
    md += [
        "## Block — %d bytes" % len(block),
        "",
        "The header, a `u32` solution length, the solution bytes, a `u32` transaction count,",
        "the transactions.",
        "",
        field_table(block_fields),
        "",
    ]
    seeds[("header_codec", "worked-header")] = header
    seeds[("header_codec", "worked-size-param")] = size_param
    seeds[("block_codec", "worked-block")] = block
    seeds[("length_limits", "worked-block")] = block
    seeds[("sis_solution_codec", "worked-solution")] = solution
    for index in ("1", "2"):
        seeds[("ed25519_verify", "rfc8032-" + index)] = (
            bytes.fromhex(rfc["rfc8032.%s.public_key" % index])
            + bytes.fromhex(rfc["rfc8032.%s.signature" % index])
            + bytes.fromhex(rfc["rfc8032.%s.message" % index])
        )
    seeds[("ed25519_verify", "zero-signature")] = keys["1"][1] + bytes(64)
    return doc.text(), "\n".join(md).rstrip("\n") + "\n", instance_seed, seeds


# ---------------------------------------------------------------------------
# G0-C8b: the expansion rule.
# ---------------------------------------------------------------------------


def build_expansion(instance_seed):
    prime_above_2_31 = 2147483659
    assert prime_above_2_31 > 1 << 31 and ref.is_prime(prime_above_2_31)
    assert all(not ref.is_prime(v) for v in range((1 << 31) + 1, prime_above_2_31))

    cases = [
        (
            "q17",
            "The existing q = 17 fixture of bench/p003-sis/SHAKE-VECTOR.md, now normative.",
            bytes(32),
            (2, 3, 17, 3),
        ),
        (
            "q2147483659-zero-seed",
            "A prime modulus above 2^31 (the first one), where a candidate is rejected.",
            bytes(32),
            (2, 3, prime_above_2_31, 3),
        ),
        (
            "q2147483659-seed-a5",
            "The same modulus on another seed, with rejections between accepted values.",
            bytes([0xA5]) * 32,
            (3, 8, prime_above_2_31, 8),
        ),
        (
            "stand-in-n8",
            "At the stand-in tuple (8, 112, 67, 112), on the worked vectors' instance_seed.",
            instance_seed,
            STAND_IN_N8,
        ),
        (
            "stand-in-n16",
            "At the stand-in tuple (16, 288, 257, 288), on the same seed.",
            instance_seed,
            STAND_IN_N16,
        ),
    ]

    doc = Document(
        [
            "CJ3 Encoding v1: vectors for the expansion rule of Protocol Spec section 5.2",
            "(ruling G0-C5; evidence of ruling G0-C8b).",
            "",
            "STATUS: NORMATIVE conformance vectors for the expansion rule, from the merge",
            "of PR #21 (Gate G0-F ruling, clause 2.4; before it, ruling G0-C8b and P-007",
            "resume ruling, clause 2.3). What is normative is the relation: these inputs",
            "give these bytes and these verdicts, and every implementation reproduces",
            "them. The input values assign nothing: no network identifier, fee, target,",
            "root, timestamp unit or tuple. If the text of the specification and a vector",
            "ever disagree, that is a STOP and a ruling. Neither wins by default.",
            "",
            "They test the rule itself. They may use any valid modulus and they make no",
            "tuple a candidate for P-4. That includes the vector at each stand-in tuple:",
            "the choice of the tuple is non-normative, and the expansion it yields is a",
            "normative vector of the rule.",
            "",
            "The rule: SHAKE-256 absorbs the 32 seed bytes and nothing else. Its output is",
            "read four bytes at a time, each group a little-endian u32. A group is accepted",
            "when it is below floor(2^32 / q) * q, and it yields its value modulo q. A",
            "rejected group is used up whole. Accepted values fill the matrix column by",
            "column, rows 0 to n-1 within a column, until n * m are placed.",
            "",
            "Derived twice: by derive.py (hashlib.shake_256) and by the Rust test",
            "crates/cj3-classes/tests/p007_vectors.rs, which calls derive_instance.",
            "Written by derive.py. Do not edit by hand.",
            "",
            "`entries` are in column-major order: entry (row r, column c) is at index",
            "c * n + r. `rejected_groups` are zero-based indexes of the rejected four-byte",
            "groups. `first_groups` are the first eight groups as integers.",
        ]
    )
    doc.put("cases", " ".join(name for name, _, _, _ in cases))
    doc.blank()
    for name, description, seed, (n, m, q, beta_squared) in cases:
        entries, consumed, rejected, limit = ref.expand(seed, n, m, q)
        assert len(entries) == n * m and all(0 <= value < q for value in entries)
        packed = b"".join(ref.u32(value) for value in entries)
        doc.comment(description)
        doc.put_hex("%s.seed" % name, seed)
        doc.put_int("%s.n" % name, n)
        doc.put_int("%s.m" % name, m)
        doc.put_int("%s.q" % name, q)
        doc.put_int("%s.beta_squared" % name, beta_squared)
        doc.put_int("%s.acceptance_limit" % name, limit)
        doc.put_ints("%s.first_groups" % name, ref.expansion_words(seed, 8))
        doc.put_int("%s.groups_consumed" % name, consumed)
        doc.put_ints("%s.rejected_groups" % name, rejected)
        doc.put_ints("%s.entries" % name, entries)
        doc.put_hex("%s.entries_u32le_sha256" % name, hashlib.sha256(packed).digest())
        doc.blank()
        if name == "q17":
            # The values recorded by P-003 on 2026-08-15, unchanged.
            assert entries == [4, 12, 6, 7, 3, 15] and limit == 4294967295
            assert ref.expansion_words(seed, 8) == [
                2189203445,
                1667912835,
                499334002,
                1326586150,
                1481038609,
                1949709156,
                1461188637,
                1647357397,
            ]
        if name.startswith("q2147483659"):
            assert rejected, "the vector above 2^31 must show a rejection"
    return doc.text()


# ---------------------------------------------------------------------------
# G0-C8c: section 1.3, clause by clause.
# ---------------------------------------------------------------------------

CLAUSE_KEYS = ["a_A", "a_R", "b_A_small", "b_R_small", "c_S_below_L", "e_equation"]


def build_ed25519(worked_text):
    cctv_raw = read_bytes("ed25519", "cctv", "ed25519vectors.json")
    assert hashlib.sha256(cctv_raw).hexdigest() == CCTV_SHA256, "CCTV file is not the pinned blob"
    cctv = json.loads(cctv_raw.decode("utf-8"))
    rfc = read_key_values("ed25519", "rfc8032-section-7.1.txt")

    vectors = []  # (source, id, key, sig, message, flags)
    for test_id in ("1", "2", "3", "1024", "sha-abc"):
        secret = bytes.fromhex(rfc["rfc8032.%s.secret_key" % test_id])
        key = bytes.fromhex(rfc["rfc8032.%s.public_key" % test_id])
        message = bytes.fromhex(rfc["rfc8032.%s.message" % test_id])
        signature = bytes.fromhex(rfc["rfc8032.%s.signature" % test_id])
        # The transcription is reproduced by this script's own signer.
        assert ref.public_key_of(secret) == key and ref.sign(secret, message) == signature
        vectors.append(("rfc8032", test_id, key, signature, message, []))

    for vector in cctv:
        vectors.append(
            (
                "cctv",
                str(vector["number"]),
                bytes.fromhex(vector["key"]),
                bytes.fromhex(vector["sig"]),
                vector["msg"].encode("ascii"),
                vector.get("flags") or [],
            )
        )
    assert len(cctv) == 914

    # Builder-authored vectors. Neither source above exercises clause (c), a y with no
    # matching x, or clause (f) directly, so these are added. Their expected results are
    # derived exactly as the others are.
    key1 = bytes.fromhex(rfc["rfc8032.1.public_key"])
    sig1 = bytes.fromhex(rfc["rfc8032.1.signature"])
    r1, s1 = sig1[:32], int.from_bytes(sig1[32:], "little")
    y_two = (2).to_bytes(32, "little")
    worked = {}
    for line in worked_text.splitlines():
        if line.startswith("out.tx1.signing_digest = ") or line.startswith("out.tx1.sig = "):
            worked[line.split(" = ")[0]] = line.split(" = ")[1]
    tx1_digest = bytes.fromhex(worked["out.tx1.signing_digest"])
    # The smallest y below 19 that is on the curve and is not a small-order point. The
    # 255-bit value y + p then encodes, for a decoder that reduces modulo p, a point of
    # large order: a blocklist of small-order points does not catch it, and only the
    # test "y not below p" of clause (a) does. CCTV has no such vector.
    large_order_y = None
    for candidate in range(2, 19):
        point, reason = ref.point_decode(candidate.to_bytes(32, "little"))
        if reason is None and not ref.is_small_order(point):
            large_order_y = candidate
            break
    assert large_order_y is not None
    y_plus_p = (large_order_y + ref.P).to_bytes(32, "little")
    assert int.from_bytes(y_plus_p, "little") < 1 << 255
    extras = [
        ("a-large-order-y-plus-p", y_plus_p, sig1, b""),
        ("r-large-order-y-plus-p", key1, y_plus_p + sig1[32:], b""),
        ("s-plus-l", key1, r1 + (s1 + ref.L).to_bytes(32, "little"), b""),
        ("s-equals-l", key1, r1 + ref.L.to_bytes(32, "little"), b""),
        ("s-all-ones", key1, r1 + b"\xff" * 32, b""),
        ("s-equals-l-minus-1", key1, r1 + (ref.L - 1).to_bytes(32, "little"), b""),
        ("zero-signature", key1, bytes(64), b""),
        ("zero-key", bytes(32), sig1, b""),
        ("zero-key-zero-signature", bytes(32), bytes(64), b""),
        ("message-changed", key1, sig1, b"\x00"),
        ("a-no-matching-x", y_two, sig1, b""),
        ("r-no-matching-x", key1, y_two + sig1[32:], b""),
        ("tx1-signing-digest-other-key", bytes.fromhex(rfc["rfc8032.2.public_key"]), sig1, tx1_digest),
    ]
    for name, key, signature, message in extras:
        vectors.append(("extra", name, key, signature, message, []))

    doc_lines = [
        "# Ed25519 verification vectors with the expected result of each derived from",
        "# Protocol Spec section 1.3 (ruling G0-C7) clause by clause (ruling G0-C8c).",
        "#",
        "# STATUS: NORMATIVE conformance vectors for Protocol Spec section 1.3, the",
        "# thirteen builder-authored (`extra`) vectors included, from the merge of PR #21",
        "# (Gate G0-F ruling, clause 2.4). What is normative is the relation: these",
        "# inputs give these bytes and these verdicts, and every implementation",
        "# reproduces them. The input values assign nothing: no network identifier, fee,",
        "# target, root, timestamp unit or tuple. If the text of the specification and a",
        "# vector ever disagree, that is a STOP and a ruling. Neither wins by default.",
        "#",
        "# Sources: `rfc8032` is RFC 8032 section 7.1 (rfc8032-section-7.1.txt). `cctv` is",
        "# C2SP CCTV ed25519/ed25519vectors.json at commit",
        "# %s," % CCTV_COMMIT,
        "# SHA-256 %s" % CCTV_SHA256,
        "# (cctv/ed25519vectors.json, with its licence beside it). `extra` vectors are",
        "# builder-authored for the clauses neither source exercises.",
        "#",
        "# No expected result is taken from a library under test or from the CCTV flags.",
        "# Each was computed by verify_clauses in cj3_encoding_v1.py, a standard-library",
        "# implementation of RFC 8032 section 5.1 written from the RFC. The CCTV flags are",
        "# printed last, for cross-reference only. The Rust test",
        "# crates/cj3-types/tests/p007_ed25519_vectors.rs requires cj3_types::ed25519::verify",
        "# to return the same verdict for every line. Written by derive.py. Do not edit.",
        "#",
        "# Columns, separated by `|`:",
        "#   source | id | key | signature | message (hex) |",
        "#   (a) A decodes | (a) R decodes | (b) A small order | (b) R small order |",
        "#   (c) S < L | (e) [S]B = R + [k]A | verdict | CCTV flags",
        "# A clause that cannot be evaluated because a point did not decode reads `n/a`.",
        "# The verdict is ACCEPT or the first clause that rejects, in the order (a) A,",
        "# (a) R, (b) A, (b) R, (c), (e).",
        "",
    ]
    by_source = {}
    flag_table = {}
    for source, vector_id, key, signature, message, flags in vectors:
        record = ref.verify_clauses(key, signature, message)
        columns = [source, vector_id, key.hex(), signature.hex(), message.hex()]
        columns += [record[name] for name in CLAUSE_KEYS]
        columns += [record["verdict"], ",".join(flags)]
        doc_lines.append("|".join(columns))
        by_source.setdefault(source, {}).setdefault(record["verdict"], 0)
        by_source[source][record["verdict"]] += 1
        if source == "cctv":
            for flag in flags or ["(no flag)"]:
                flag_table.setdefault(flag, {}).setdefault(record["verdict"], 0)
                flag_table[flag][record["verdict"]] += 1
    expected_text = "\n".join(doc_lines) + "\n"

    verdicts = sorted({verdict for counts in by_source.values() for verdict in counts})
    md = [
        "# Section 1.3 verdicts — summary",
        "",
        "Generated by `derive.py` from `c7-expected.vectors.txt`. Do not edit by hand.",
        "",
        "Every verdict was computed by `verify_clauses` in `cj3_encoding_v1.py`, a",
        "standard-library implementation written from RFC 8032 section 5.1 and Protocol Spec",
        "§1.3. The verdict is `ACCEPT` or the first clause that rejects.",
        "",
        "## Verdicts by source",
        "",
        "| Source | Vectors | " + " | ".join("`%s`" % verdict for verdict in verdicts) + " |",
        "|---|---:|" + "---:|" * len(verdicts),
    ]
    for source in ("rfc8032", "cctv", "extra"):
        counts = by_source[source]
        md.append(
            "| %s | %d | " % (source, sum(counts.values()))
            + " | ".join(str(counts.get(verdict, 0)) for verdict in verdicts)
            + " |"
        )
    md += [
        "",
        "## CCTV flags against verdicts",
        "",
        "For cross-reference only: the flags come from the CCTV file and decided nothing.",
        "A vector with several flags is counted once under each.",
        "",
        "| CCTV flag | Vectors | " + " | ".join("`%s`" % verdict for verdict in verdicts) + " |",
        "|---|---:|" + "---:|" * len(verdicts),
    ]
    for flag in sorted(flag_table):
        counts = flag_table[flag]
        md.append(
            "| `%s` | %d | " % (flag, sum(counts.values()))
            + " | ".join(str(counts.get(verdict, 0)) for verdict in verdicts)
            + " |"
        )
    # What the flags predict under section 1.3, checked rather than assumed.
    accepted = rejected_noncanonical = 0
    for source, vector_id, key, signature, message, flags in vectors:
        if source != "cctv":
            continue
        verdict = ref.verify_clauses(key, signature, message)["verdict"]
        low_order = "low_order_A" in flags or "low_order_R" in flags
        expected_accept = not low_order and "low_order_residue" not in flags
        assert (verdict == "ACCEPT") == expected_accept, (vector_id, flags, verdict)
        if "non_canonical_A" in flags or "non_canonical_R" in flags:
            assert verdict.startswith("a:") or verdict.startswith("b:"), (vector_id, verdict)
            rejected_noncanonical += 1
        accepted += verdict == "ACCEPT"
    md += [
        "",
        "## What the comparison shows",
        "",
        "Checked by `derive.py` for all 914 CCTV vectors, and asserted there:",
        "",
        "- a vector is accepted exactly when it is flagged neither `low_order_A` nor",
        "  `low_order_R` nor `low_order_residue`; %d are accepted;" % accepted,
        "- every vector flagged `non_canonical_A` or `non_canonical_R` (%d of them) is"
        % rejected_noncanonical,
        "  rejected by clause (a) or clause (b), before any equation is tested.",
        "",
        "A point with a low-order component that is not itself of small order is not",
        "rejected by clause (b). Such a signature is accepted only when the unmultiplied",
        "equation of clause (e) holds.",
    ]
    return expected_text, "\n".join(md) + "\n"


# ---------------------------------------------------------------------------


def main(argv):
    check = argv[1:] == ["--check"]
    if argv[1:] and not check:
        sys.stderr.write(__doc__)
        return 2

    worked_text, worked_md, instance_seed, seeds = build_worked()
    expansion_text = build_expansion(instance_seed)
    expected_text, summary_md = build_ed25519(worked_text)

    outputs = [
        (("encoding-v1.vectors.txt",), worked_text),
        (("WORKED-VECTORS.md",), worked_md),
        (("shake-expansion.vectors.txt",), expansion_text),
        (("ed25519", "c7-expected.vectors.txt"), expected_text),
        (("ed25519", "C7-SUMMARY.md"), summary_md),
    ]
    for (target, seed_name), data in sorted(seeds.items()):
        outputs.append(((os.pardir, os.pardir, "fuzz", "seeds", target, seed_name), data))
    status = 0
    for parts, text in outputs:
        path = os.path.join(HERE, *parts)
        data = text if isinstance(text, bytes) else text.encode("utf-8")
        digest = hashlib.sha256(data).hexdigest()
        name = "/".join(parts)
        if check:
            committed = read_bytes(*parts) if os.path.exists(path) else None
            if committed != data:
                status = 1
                print("P007_VECTOR_DRIFT %s derived_sha256=%s" % (name, digest))
            else:
                print("P007_VECTOR_OK %s sha256=%s" % (name, digest))
        else:
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "wb") as handle:
                handle.write(data)
            print("P007_VECTOR_WRITTEN %s sha256=%s" % (name, digest))
    if check and status == 0:
        print("P007_VECTORS=PASS files=%d" % len(outputs))
    return status


if __name__ == "__main__":
    sys.exit(main(sys.argv))
