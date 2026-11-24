"""Worked examples for the tx_root and state commitment that P-101's FRAME proposes.

    python -I -B bench/p101-roots/derive.py            write the example files
    python -I -B bench/p101-roots/derive.py --check    fail if a committed file differs

STATUS: PROPOSED, NOT RULED. Nothing here is a conformance vector, and nothing here is
normative. Al's ruling "P-101 TO ITS FRAME" (clause 3.3) orders P-101's FRAME to propose
how tx_root and the state commitment are built, with one worked example of each derived
by a script that uses only a standard library. This is that script. The FRAME is
loop/reports/C12-p101-builder.md, and Al rules on it.

The six domain tags below are PROPOSED names and bytes. Al assigns tags; the FRAME does
not, and neither does this script. No Rust source names them.

Inputs, read from the normative vectors of CJ3 Encoding v1 and never changed:

    ../p007-vectors/encoding-v1.vectors.txt   the two worked transactions and the three
                                              addresses of the RFC 8032 test keys

Outputs, generated and never edited by hand:

    p101-roots.examples.txt     every preimage and digest of the examples, as key = value
    WORKED-EXAMPLES.md          the same examples, laid out for reading

The account values are test values. They assign nothing: no balance, no allocation, no
genesis state. D8 permits Python only under bench/; nothing here is part of the node.
"""

import hashlib
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "..", "p007-vectors", "encoding-v1.vectors.txt")
SOURCE_SHA256 = "f5061cef1227fee26e732b11adbb0efbadb528c6647a86d70ceaed795f17d855"

STATUS = "PROPOSED, NOT RULED. Not a conformance vector. The tags are proposed, not assigned."


def tag(text):
    """A 16-byte domain tag: the ASCII text, padded on the right with 0x00 (G0-C2b)."""
    raw = text.encode("ascii")
    assert len(raw) < 16
    return raw + bytes(16 - len(raw))


# PROPOSED tags. Al assigns tags.
TAGS = [
    ("D_TXLEAF", "CJ3/v1/txleaf"),
    ("D_TXNODE", "CJ3/v1/txnode"),
    ("D_TXROOT", "CJ3/v1/txroot"),
    ("D_STLEAF", "CJ3/v1/stleaf"),
    ("D_STNODE", "CJ3/v1/stnode"),
    ("D_STROOT", "CJ3/v1/stroot"),
]
D = {name: tag(text) for name, text in TAGS}
RULED_TAGS = ["addr", "tx", "header", "instance", "solution", "beacon", "genesis"]

ZERO32 = bytes(32)
TX_LEN = 188


def H(preimage):
    return hashlib.sha256(preimage).digest()


def u16(value):
    return value.to_bytes(2, "little")


def u32(value):
    return value.to_bytes(4, "little")


def u64(value):
    return value.to_bytes(8, "little")


# --- tx_root: a list tree in the shape of RFC 6962, with the count in the root ----------


def tx_leaf_preimage(tx):
    assert len(tx) == TX_LEN
    return D["D_TXLEAF"] + tx


def tx_node_preimage(left, right):
    return D["D_TXNODE"] + left + right


def tx_root_preimage(count, top):
    return D["D_TXROOT"] + u32(count) + top


def split_point(n):
    """The largest power of two strictly less than n, for n >= 2."""
    k = 1
    while 2 * k < n:
        k *= 2
    return k


def tx_top(txs, trace=None, path="top"):
    """The top hash of a non-empty list, by the recursive definition."""
    if len(txs) == 1:
        pre = tx_leaf_preimage(txs[0])
        if trace is not None:
            trace.append((path, "leaf", pre, H(pre)))
        return H(pre)
    k = split_point(len(txs))
    left = tx_top(txs[:k], trace, path + ".L")
    right = tx_top(txs[k:], trace, path + ".R")
    pre = tx_node_preimage(left, right)
    if trace is not None:
        trace.append((path, "node", pre, H(pre)))
    return H(pre)


def tx_root(txs, trace=None):
    top = tx_top(txs, trace) if txs else ZERO32
    pre = tx_root_preimage(len(txs), top)
    if trace is not None:
        trace.append(("root", "root", pre, H(pre)))
    return H(pre)


def tx_top_bottom_up(txs):
    """The same top hash, computed level by level. An odd node at the end of a level is
    carried up as it is. It is never paired with itself."""
    level = [H(tx_leaf_preimage(tx)) for tx in txs]
    while len(level) > 1:
        nxt = [H(tx_node_preimage(level[i], level[i + 1])) for i in range(0, len(level) - 1, 2)]
        if len(level) % 2 == 1:
            nxt.append(level[-1])
        level = nxt
    return level[0]


def tx_proof(txs, index):
    """The sibling hashes on the way from leaf `index` up to the top, lowest first."""
    def walk(lo, hi, acc):
        if hi - lo == 1:
            return
        k = split_point(hi - lo)
        if index < lo + k:
            walk(lo, lo + k, acc)
            acc.append(tx_top(txs[lo + k:hi]))
        else:
            walk(lo + k, hi, acc)
            acc.append(tx_top(txs[lo:lo + k]))
    siblings = []
    walk(0, len(txs), siblings)
    return siblings


def tx_verify(root, count, index, tx, siblings):
    """True if `tx` is at position `index` of the `count` transactions under `root`."""
    if not (0 <= index < count) or len(tx) != TX_LEN:
        return False
    # The sides are fixed by (index, count): recompute them from the split rule.
    sides = []
    lo, hi = 0, count
    while hi - lo > 1:
        k = split_point(hi - lo)
        if index < lo + k:
            sides.append("L")
            hi = lo + k
        else:
            sides.append("R")
            lo = lo + k
    if len(sides) != len(siblings):
        return False
    node = H(tx_leaf_preimage(tx))
    for side, sibling in zip(reversed(sides), siblings):
        node = H(tx_node_preimage(node, sibling) if side == "L" else tx_node_preimage(sibling, node))
    return H(tx_root_preimage(count, node)) == root


# --- State commitment: a binary radix tree over the address bits, splits recorded -------


def bit(address, index):
    """Bit `index` of a 32-byte address, the most significant bit of byte 0 first."""
    return (address[index // 8] >> (7 - index % 8)) & 1


def first_difference(a, b):
    """The index of the first bit at which two different addresses differ."""
    for index in range(256):
        if bit(a, index) != bit(b, index):
            return index
    raise ValueError("equal addresses")


def st_leaf_preimage(address, balance, nonce):
    assert len(address) == 32
    return D["D_STLEAF"] + address + u64(balance) + u64(nonce)


def st_node_preimage(split, left, right):
    return D["D_STNODE"] + u16(split) + left + right


def st_root_preimage(count, top):
    return D["D_STROOT"] + u64(count) + top


def normalize(accounts):
    """The leaves of a state: its accounts that are not (balance 0, nonce 0), in
    ascending order of address, the 32 bytes compared as unsigned bytes."""
    kept = [(a, b, n) for a, (b, n) in accounts.items() if (b, n) != (0, 0)]
    return sorted(kept)


def st_top(leaves, trace=None, path="top"):
    """The top hash of a non-empty sorted list of leaves, by the recursive definition."""
    if len(leaves) == 1:
        pre = st_leaf_preimage(*leaves[0])
        if trace is not None:
            trace.append((path, "leaf", pre, H(pre)))
        return H(pre)
    # In sorted order the first and the last leaf differ earliest.
    split = first_difference(leaves[0][0], leaves[-1][0])
    cut = next(i for i, leaf in enumerate(leaves) if bit(leaf[0], split) == 1)
    left = st_top(leaves[:cut], trace, path + ".0")
    right = st_top(leaves[cut:], trace, path + ".1")
    pre = st_node_preimage(split, left, right)
    if trace is not None:
        trace.append((path, "node", pre, H(pre)))
    return H(pre)


def st_root(accounts, trace=None):
    leaves = normalize(accounts)
    top = st_top(leaves, trace) if leaves else ZERO32
    pre = st_root_preimage(len(leaves), top)
    if trace is not None:
        trace.append(("root", "root", pre, H(pre)))
    return H(pre)


def st_path(leaves, key):
    """Follow `key` down the tree of a non-empty state. Returns the leaf reached and the
    (split, sibling) pairs on the way, lowest first."""
    steps = []
    while len(leaves) > 1:
        split = first_difference(leaves[0][0], leaves[-1][0])
        cut = next(i for i, leaf in enumerate(leaves) if bit(leaf[0], split) == 1)
        if bit(key, split) == 0:
            steps.append((split, st_top(leaves[cut:])))
            leaves = leaves[:cut]
        else:
            steps.append((split, st_top(leaves[:cut])))
            leaves = leaves[cut:]
    return leaves[0], list(reversed(steps))


def st_recompute(key, leaf, steps):
    """The top hash that a leaf and its steps give, taking each side from `key`. Returns
    None if the splits are not strictly decreasing from the leaf upward."""
    node = H(st_leaf_preimage(*leaf))
    previous = 256
    for split, sibling in steps:
        if not 0 <= split < previous:
            return None
        previous = split
        if bit(key, split) == 0:
            node = H(st_node_preimage(split, node, sibling))
        else:
            node = H(st_node_preimage(split, sibling, node))
    return node


def st_verify_present(root, count, leaf, steps):
    """True if `leaf` (address, balance, nonce) is in the state under `root`."""
    if leaf[1:] == (0, 0):
        return False
    top = st_recompute(leaf[0], leaf, steps)
    return top is not None and H(st_root_preimage(count, top)) == root


def st_verify_absent(root, count, key, leaf=None, steps=None):
    """True if `key` has no leaf in the state under `root`: its account is (0, 0)."""
    if count == 0:
        return H(st_root_preimage(0, ZERO32)) == root
    if leaf is None or leaf[0] == key:
        return False
    top = st_recompute(key, leaf, steps)
    return top is not None and H(st_root_preimage(count, top)) == root


class Radix:
    """An independent way to reach the same tree: insert and delete one key at a time.
    Used only to check that the root does not depend on the order of operations."""

    def __init__(self):
        self.root = None  # None | ("leaf", addr, bal, nonce) | ("node", split, left, right)

    def set(self, address, balance, nonce):
        self.root = self._delete(self.root, address)
        if (balance, nonce) != (0, 0):
            self.root = self._insert(self.root, address, ("leaf", address, balance, nonce))

    def _any_leaf(self, node, address):
        while node[0] == "node":
            node = node[3] if bit(address, node[1]) else node[2]
        return node

    def _insert(self, node, address, leaf):
        if node is None:
            return leaf
        split = first_difference(self._any_leaf(node, address)[1], address)
        return self._place(node, address, leaf, split)

    def _place(self, node, address, leaf, split):
        if node[0] == "leaf" or node[1] > split:
            return ("node", split, node, leaf) if bit(address, split) else ("node", split, leaf, node)
        if bit(address, node[1]):
            return ("node", node[1], node[2], self._place(node[3], address, leaf, split))
        return ("node", node[1], self._place(node[2], address, leaf, split), node[3])

    def _delete(self, node, address):
        if node is None:
            return None
        if node[0] == "leaf":
            return None if node[1] == address else node
        if bit(address, node[1]):
            child = self._delete(node[3], address)
            return node[2] if child is None else ("node", node[1], node[2], child)
        child = self._delete(node[2], address)
        return node[3] if child is None else ("node", node[1], child, node[3])

    def _hash(self, node):
        if node[0] == "leaf":
            return 1, H(st_leaf_preimage(node[1], node[2], node[3]))
        left_count, left = self._hash(node[2])
        right_count, right = self._hash(node[3])
        return left_count + right_count, H(st_node_preimage(node[1], left, right))

    def commitment(self):
        count, top = self._hash(self.root) if self.root is not None else (0, ZERO32)
        return H(st_root_preimage(count, top))


# --- Self-checks: they run every time, before anything is written -----------------------


def stream(label):
    """A deterministic byte stream: SHA-256 in counter mode. No random module."""
    counter = 0
    while True:
        yield from hashlib.sha256(label + counter.to_bytes(8, "little")).digest()
        counter += 1


def take(source, count):
    return bytes(next(source) for _ in range(count))


def self_check():
    checks = 0

    # Tags: six new 16-byte strings, distinct from each other and from the seven ruled.
    ruled = {tag("CJ3/v1/" + name) for name in RULED_TAGS}
    proposed = set(D.values())
    assert len(proposed) == 6 and all(len(value) == 16 for value in proposed)
    assert not (proposed & ruled)
    checks += 1

    # tx tree: the recursive definition and the level-by-level computation agree, every
    # position verifies, and a proof for one position fails at every other.
    source = stream(b"p101-roots/tx")
    txs = [take(source, TX_LEN) for _ in range(70)]
    roots = set()
    for n in range(0, 70):
        root = tx_root(txs[:n])
        roots.add(root)
        if n:
            assert tx_top(txs[:n]) == tx_top_bottom_up(txs[:n])
        for index in range(n):
            siblings = tx_proof(txs[:n], index)
            assert tx_verify(root, n, index, txs[index], siblings)
            assert not tx_verify(root, n, index, txs[(index + 1) % 70], siblings)
            assert not tx_verify(root, n, (index + 1) % n, txs[index], siblings) or n == 1
            assert not tx_verify(root, n + 1, index, txs[index], siblings)
        checks += 1
    assert len(roots) == 70
    # No last node is counted twice: a list and the same list with its last entry
    # repeated have different roots, for every length.
    for n in range(1, 40):
        assert tx_root(txs[:n]) != tx_root(txs[:n] + [txs[n - 1]])
        checks += 1
    # Order matters.
    assert tx_root([txs[0], txs[1]]) != tx_root([txs[1], txs[0]])
    checks += 1

    # State tree: the root is a function of the state. Insertion order, deletions and
    # (0, 0) accounts do not change it; presence and absence proofs verify.
    source = stream(b"p101-roots/state")
    for trial in range(60):
        size = trial % 23
        accounts = {}
        base = take(source, 32)
        for index in range(size):
            if index % 3 == 0:
                # A key that shares a long prefix with `base`: flip one late bit.
                position = 8 * 20 + (take(source, 1)[0] % 96)
                raw = bytearray(base)
                raw[position // 8] ^= 0x80 >> (position % 8)
                address = bytes(raw)
            else:
                address = take(source, 32)
            balance = int.from_bytes(take(source, 8), "little") if index % 5 else 0
            nonce = int.from_bytes(take(source, 2), "little") if index % 4 else 0
            accounts[address] = (balance, nonce)
        root = st_root(accounts)
        leaves = normalize(accounts)

        forward, backward = Radix(), Radix()
        for address in sorted(accounts):
            forward.set(address, *accounts[address])
        for address in sorted(accounts, reverse=True):
            backward.set(address, *accounts[address])
        assert forward.commitment() == root and backward.commitment() == root

        # Write other values first, add and remove strangers, then settle.
        churn = Radix()
        strangers = [take(source, 32) for _ in range(5)]
        for address in strangers:
            churn.set(address, 7, 7)
        for address in accounts:
            churn.set(address, 1, 1)
        for address in strangers:
            churn.set(address, 0, 0)
        for address in accounts:
            churn.set(address, *accounts[address])
        assert churn.commitment() == root

        with_empty = dict(accounts)
        with_empty[take(source, 32)] = (0, 0)
        assert st_root(with_empty) == root

        for leaf in leaves:
            reached, steps = st_path(leaves, leaf[0])
            assert reached == leaf
            assert st_verify_present(root, len(leaves), leaf, steps)
            changed = (leaf[0], leaf[1] + 1, leaf[2])
            assert not st_verify_present(root, len(leaves), changed, steps)
            assert not st_verify_absent(root, len(leaves), leaf[0], leaf, steps)
        for _ in range(4):
            key = take(source, 32)
            if key in accounts:
                continue
            if leaves:
                reached, steps = st_path(leaves, key)
                assert st_verify_absent(root, len(leaves), key, reached, steps)
                assert not st_verify_present(root, len(leaves), (key, 1, 0), steps)
            else:
                assert st_verify_absent(root, 0, key)
        # A changed balance, a changed nonce and a removed account each change the root.
        for leaf in leaves[:3]:
            other = dict(accounts)
            other[leaf[0]] = (leaf[1] ^ 1, leaf[2])
            assert st_root(other) != root
            other[leaf[0]] = (leaf[1], leaf[2] ^ 1)
            assert st_root(other) != root
            del other[leaf[0]]
            assert st_root(other) != root
        checks += 1
    return checks


# --- The worked examples ----------------------------------------------------------------


def read_source():
    with open(SOURCE, "rb") as handle:
        raw = handle.read().replace(b"\r\n", b"\n")
    digest = hashlib.sha256(raw).hexdigest()
    if digest != SOURCE_SHA256:
        raise SystemExit("encoding-v1.vectors.txt is not the file this script was written for: " + digest)
    values = {}
    key = None
    for line in raw.decode("ascii").splitlines():
        if not line or line.startswith("#"):
            continue
        if line.startswith("  "):
            # A long value continues on lines indented by two spaces.
            values[key] += line.strip()
        else:
            key, value = line.split(" = ", 1)
            values[key] = value
    return values


def hexlines(data, width=32):
    text = data.hex()
    return [text[i:i + 2 * width] for i in range(0, len(text), 2 * width)] or [""]


def fields(parts):
    """Render a preimage as its fields: (name, bytes) pairs."""
    return [(name, value.hex()) for name, value in parts]


def build():
    src = read_source()
    tx1 = bytes.fromhex(src["out.tx1.bytes"])
    tx2 = bytes.fromhex(src["out.tx2.bytes"])
    key1 = bytes.fromhex(src["out.addr.key1"])
    key2 = bytes.fromhex(src["out.addr.key2"])
    key3 = bytes.fromhex(src["out.addr.key3"])
    # An address that shares its first 255 bits with key1's: the last bit is flipped.
    near = key1[:31] + bytes([key1[31] ^ 0x01])
    assert first_difference(key1, near) == 255

    kv = []   # (key, value) pairs for the machine-readable file
    md = []   # lines of the readable file

    def emit(key, value):
        kv.append((key, value if isinstance(value, str) else value.hex()))

    def show_preimage(title, key, parts):
        pre = b"".join(value for _, value in parts)
        digest = H(pre)
        emit(key + ".preimage", pre)
        emit(key, digest)
        md.append("**%s** — preimage, %d bytes:" % (title, len(pre)))
        md.append("")
        md.append("| Field | Bytes (hex) |")
        md.append("|---|---|")
        for name, value in parts:
            md.append("| %s | `%s` |" % (name, "` `".join(hexlines(value))))
        md.append("")
        md.append("SHA-256: `%s`" % digest.hex())
        md.append("")
        return digest

    md.append("# P-101 worked examples: the proposed tx_root and state commitment")
    md.append("")
    md.append("> **" + STATUS + "**")
    md.append("> Generated by `derive.py`, which uses only the Python standard library. Do not")
    md.append("> edit by hand. The proposal is in `loop/reports/C12-p101-builder.md`; Al rules on it.")
    md.append("> The account values are test values and assign nothing.")
    md.append("")
    md.append("## Proposed domain tags")
    md.append("")
    md.append("Each is 16 bytes: the ASCII text, padded on the right with `0x00`, as ruling G0-C2b")
    md.append("lays out a tag. Al assigns tags. These six are proposed and not assigned.")
    md.append("")
    md.append("| Proposed tag | ASCII text | The 16 bytes (hex) | Opens |")
    md.append("|---|---|---|---|")
    opens = {
        "D_TXLEAF": "the transaction leaf preimage",
        "D_TXNODE": "the transaction node preimage",
        "D_TXROOT": "the `tx_root` preimage",
        "D_STLEAF": "the account leaf preimage",
        "D_STNODE": "the state node preimage",
        "D_STROOT": "the state commitment preimage",
    }
    for name, text in TAGS:
        emit("proposed.tag." + name, D[name])
        md.append("| `%s` | `%s` | `%s` | %s |" % (name, text, D[name].hex(), opens[name]))
    md.append("")

    # ---- tx_root ----
    md.append("## Example 1: tx_root of three transactions")
    md.append("")
    md.append("The list is `[tx1, tx2, tx1]`, where `tx1` and `tx2` are the two worked transactions")
    md.append("of the normative vectors (`out.tx1.bytes`, `out.tx2.bytes` in")
    md.append("`bench/p007-vectors/encoding-v1.vectors.txt`). A block could not hold `tx1` twice,")
    md.append("because V4 would reject the second; the root is defined for any list, and the")
    md.append("repeat is here to show that a repeated entry is hashed as an entry of its own.")
    md.append("")
    md.append("Shape for a count of 3: the split point is 2, so the top is")
    md.append("`node(node(leaf 0, leaf 1), leaf 2)`. The third leaf is not paired with itself.")
    md.append("")
    emit("tx.example.count", "3")
    emit("tx.example.list", "tx1 tx2 tx1")
    l0 = show_preimage("Leaf 0 (tx1)", "tx.example.leaf0", [("`D_TXLEAF`", D["D_TXLEAF"]), ("transaction [188]", tx1)])
    l1 = show_preimage("Leaf 1 (tx2)", "tx.example.leaf1", [("`D_TXLEAF`", D["D_TXLEAF"]), ("transaction [188]", tx2)])
    l2 = show_preimage("Leaf 2 (tx1 again)", "tx.example.leaf2", [("`D_TXLEAF`", D["D_TXLEAF"]), ("transaction [188]", tx1)])
    assert l0 == l2
    n01 = show_preimage("Node over leaves 0 and 1", "tx.example.node01", [("`D_TXNODE`", D["D_TXNODE"]), ("left [32]", l0), ("right [32]", l1)])
    top = show_preimage("Top: node over (0,1) and leaf 2", "tx.example.top", [("`D_TXNODE`", D["D_TXNODE"]), ("left [32]", n01), ("right [32]", l2)])
    root = show_preimage("tx_root", "tx.example.root", [("`D_TXROOT`", D["D_TXROOT"]), ("count u32", u32(3)), ("top [32]", top)])
    example = [tx1, tx2, tx1]
    assert root == tx_root(example) and top == tx_top_bottom_up(example)

    md.append("### A proof that tx1 is at position 2 of 3")
    md.append("")
    siblings = tx_proof(example, 2)
    assert siblings == [n01] and tx_verify(root, 3, 2, tx1, siblings)
    emit("tx.example.proof.index", "2")
    emit("tx.example.proof.siblings", " ".join(s.hex() for s in siblings))
    md.append("The proof is the count `3`, the position `2`, the 188 transaction bytes and one")
    md.append("sibling hash, lowest first:")
    md.append("")
    md.append("```text")
    md.append(siblings[0].hex())
    md.append("```")
    md.append("")
    md.append("For position 2 of 3 the split rule puts the leaf on the right of the top node, so")
    md.append("the verifier computes `H(D_TXNODE ‖ sibling ‖ leaf 2)`, then `H(D_TXROOT ‖ 3 ‖ top)`,")
    md.append("and compares with `tx_root`.")
    md.append("")

    md.append("### Other lists, for comparison")
    md.append("")
    md.append("| List | Count | tx_root |")
    md.append("|---|---:|---|")
    others = [
        ("empty", "(none)", []),
        ("one", "tx1", [tx1]),
        ("two", "tx1 tx2", [tx1, tx2]),
        ("two-swapped", "tx2 tx1", [tx2, tx1]),
        ("three", "tx1 tx2 tx1", example),
        ("four", "tx1 tx2 tx1 tx1", [tx1, tx2, tx1, tx1]),
    ]
    seen = set()
    for key, label, txs in others:
        value = tx_root(txs)
        seen.add(value)
        emit("tx.case." + key + ".list", label)
        emit("tx.case." + key + ".root", value)
        md.append("| `%s` | %d | `%s` |" % (label, len(txs), value.hex()))
    assert len(seen) == len(others)
    empty_pre = tx_root_preimage(0, ZERO32)
    emit("tx.case.empty.preimage", empty_pre)
    md.append("")
    md.append("The empty list has count 0 and 32 zero bytes where the top would be. Its root")
    md.append("preimage is `%s`." % empty_pre.hex())
    md.append("")
    md.append("The last two rows are the case that a tree which repeats its last node gets")
    md.append("wrong: `[tx1, tx2, tx1]` and `[tx1, tx2, tx1, tx1]` must not share a root, and")
    md.append("they do not.")
    md.append("")

    # ---- state ----
    accounts = {
        key3: (0x0807060504030201, 0),
        near: (1, 0),
        key1: (0, 0x1817161514131211),
        key2: (0xFFFFFFFFFFFFFFFF, 2),
    }
    leaves = normalize(accounts)
    assert [leaf[0] for leaf in leaves] == [key3, near, key1, key2]

    md.append("## Example 2: the state commitment of four accounts")
    md.append("")
    md.append("Three addresses are `addr` of the RFC 8032 test keys 1 to 3 (`out.addr.key1`,")
    md.append("`out.addr.key2`, `out.addr.key3` in the normative vectors). The fourth, called")
    md.append("`near`, is the address of key 1 with its last bit flipped. No key is known for it;")
    md.append("an address read from a transaction is any 32 bytes. It shares 255 bits with the")
    md.append("address of key 1, to show that a long shared prefix costs one node, not a chain.")
    md.append("")
    md.append("| Account | Address | Balance | Nonce |")
    md.append("|---|---|---:|---:|")
    names = {key3: "key3", near: "near", key1: "key1", key2: "key2"}
    for address, balance, nonce in leaves:
        emit("state.example.account." + names[address] + ".address", address)
        emit("state.example.account." + names[address] + ".balance", str(balance))
        emit("state.example.account." + names[address] + ".nonce", str(nonce))
        md.append("| %s | `%s` | %d | %d |" % (names[address], address.hex(), balance, nonce))
    md.append("")
    md.append("The rows are in ascending order of address, the 32 bytes compared as unsigned")
    md.append("bytes. Bit 0 of an address is the most significant bit of its first byte.")
    md.append("")
    md.append("Shape: the four addresses first differ at bit 0 (key3 has 0, the others 1). The")
    md.append("other three first differ at bit 1 (near and key1 have 0, key2 has 1). near and key1")
    md.append("first differ at bit 255. So the top is")
    md.append("`node(0, leaf key3, node(1, node(255, leaf near, leaf key1), leaf key2))`.")
    md.append("")

    def leaf_parts(address, balance, nonce):
        return [("`D_STLEAF`", D["D_STLEAF"]), ("address [32]", address), ("balance u64", u64(balance)), ("nonce u64", u64(nonce))]

    hashes = {}
    for address, balance, nonce in leaves:
        hashes[address] = show_preimage("Leaf %s" % names[address], "state.example.leaf." + names[address], leaf_parts(address, balance, nonce))
    n255 = show_preimage("Node at split 255, over near and key1", "state.example.node255", [("`D_STNODE`", D["D_STNODE"]), ("split u16", u16(255)), ("left [32]", hashes[near]), ("right [32]", hashes[key1])])
    n1 = show_preimage("Node at split 1, over (near, key1) and key2", "state.example.node1", [("`D_STNODE`", D["D_STNODE"]), ("split u16", u16(1)), ("left [32]", n255), ("right [32]", hashes[key2])])
    n0 = show_preimage("Top: node at split 0, over key3 and the rest", "state.example.top", [("`D_STNODE`", D["D_STNODE"]), ("split u16", u16(0)), ("left [32]", hashes[key3]), ("right [32]", n1)])
    sroot = show_preimage("State commitment", "state.example.root", [("`D_STROOT`", D["D_STROOT"]), ("count u64", u64(4)), ("top [32]", n0)])
    assert sroot == st_root(accounts)

    md.append("### A proof that key1's account is present")
    md.append("")
    reached, steps = st_path(leaves, key1)
    assert reached[0] == key1 and st_verify_present(sroot, 4, reached, steps)
    assert steps == [(255, hashes[near]), (1, hashes[key2]), (0, hashes[key3])]
    emit("state.example.present.key", key1)
    emit("state.example.present.steps", " ".join("%d:%s" % (s, h.hex()) for s, h in steps))
    md.append("The proof is the count `4`, the leaf (address, balance, nonce) and three steps,")
    md.append("lowest first, each a split index and a sibling hash:")
    md.append("")
    md.append("```text")
    for split, sibling in steps:
        md.append("split %3d  sibling %s" % (split, sibling.hex()))
    md.append("```")
    md.append("")
    md.append("At each step the verifier reads the bit of the address at the split index: 1 puts")
    md.append("the running hash on the right, 0 on the left. Bits 255, 1 and 0 of key1's address")
    md.append("are 1, 0 and 1. The splits must decrease strictly from the leaf upward.")
    md.append("")

    md.append("### A proof that an address is absent")
    md.append("")
    absent = bytes.fromhex(src["in.key1.public_key"])
    assert absent not in accounts
    reached, steps = st_path(leaves, absent)
    assert st_verify_absent(sroot, 4, absent, reached, steps)
    emit("state.example.absent.key", absent)
    emit("state.example.absent.reached", names[reached[0]])
    emit("state.example.absent.steps", " ".join("%d:%s" % (s, h.hex()) for s, h in steps))
    md.append("The address asked for is `%s`" % absent.hex())
    md.append("(the 32 bytes of test key 1's public key, used here only as an address with no")
    md.append("account). Following its bits down the tree reaches the leaf of **%s**." % names[reached[0]])
    md.append("The proof is the count `4`, that leaf, and its steps:")
    md.append("")
    md.append("```text")
    for split, sibling in steps:
        md.append("split %3d  sibling %s" % (split, sibling.hex()))
    md.append("```")
    md.append("")
    md.append("The verifier takes each side from the bits of the address it asked for, recomputes")
    md.append("the commitment, and checks that the leaf's address is a different one. If the")
    md.append("address had an account, the same walk would have ended at it.")
    md.append("")

    md.append("### Other states, for comparison")
    md.append("")
    md.append("| State | Leaves | State commitment |")
    md.append("|---|---:|---|")
    with_empty = dict(accounts)
    with_empty[absent] = (0, 0)
    without_near = {k: v for k, v in accounts.items() if k != near}
    one_more_unit = dict(accounts)
    one_more_unit[key3] = (accounts[key3][0] + 1, 0)
    cases = [
        ("empty", "no accounts", {}),
        ("single", "key3 alone", {key3: accounts[key3]}),
        ("example", "the four accounts above", accounts),
        ("example-with-empty-account", "the same, plus an address whose account is (0, 0)", with_empty),
        ("example-without-near", "the same, without near", without_near),
        ("example-one-unit-more", "the same, with one unit more on key3", one_more_unit),
    ]
    for key, label, state in cases:
        value = st_root(state)
        emit("state.case." + key + ".leaves", str(len(normalize(state))))
        emit("state.case." + key + ".root", value)
        md.append("| %s | %d | `%s` |" % (label, len(normalize(state)), value.hex()))
    assert st_root(with_empty) == sroot
    assert len({st_root(state) for _, _, state in cases}) == len(cases) - 1
    empty_state_pre = st_root_preimage(0, ZERO32)
    emit("state.case.empty.preimage", empty_state_pre)
    md.append("")
    md.append("The empty state has count 0 and 32 zero bytes where the top would be. Its preimage")
    md.append("is `%s`." % empty_state_pre.hex())
    md.append("")
    md.append("The third and fourth rows are equal on purpose: an account with balance 0 and")
    md.append("nonce 0 is not a leaf, so a state does not change when such an address is named.")
    md.append("")

    text = ["# " + STATUS,
            "# Generated by bench/p101-roots/derive.py (Python standard library only). Do not edit.",
            "# The proposal is in loop/reports/C12-p101-builder.md. Al rules on it.",
            "# Inputs read from bench/p007-vectors/encoding-v1.vectors.txt, sha256 " + SOURCE_SHA256,
            "# The account values are test values and assign nothing."]
    text += ["%s = %s" % pair for pair in kv]
    return {
        "p101-roots.examples.txt": "\n".join(text) + "\n",
        "WORKED-EXAMPLES.md": "\n".join(md).rstrip("\n") + "\n",
    }


def main(argv):
    check = argv[1:] == ["--check"]
    if argv[1:] and not check:
        raise SystemExit(__doc__)
    checks = self_check()
    outputs = build()
    failed = False
    for name, content in sorted(outputs.items()):
        path = os.path.join(HERE, name)
        data = content.encode("utf-8")
        digest = hashlib.sha256(data).hexdigest()
        if check:
            try:
                with open(path, "rb") as handle:
                    committed = handle.read().replace(b"\r\n", b"\n")
            except OSError:
                committed = None
            if committed != data:
                print("P101_ROOTS_DIFFERS %s" % name)
                failed = True
                continue
        else:
            with open(path, "wb") as handle:
                handle.write(data)
        print("P101_ROOTS_OK %s sha256=%s" % (name, digest))
    if failed:
        raise SystemExit(1)
    print("P101_ROOTS=PASS files=%d self_checks=%d status=proposed-not-ruled" % (len(outputs), checks))


if __name__ == "__main__":
    main(sys.argv)
