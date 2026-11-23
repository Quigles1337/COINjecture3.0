# C12 Builder Report — P-101, the kernel: FRAME, and a STOP for Al's ruling

This is P-101's report. It holds move 1 of the packet, the FRAME, and nothing after it.
Al's ruling "LIVE CJ3 RULING - P-101 TO ITS FRAME" (cited here as "launch 3.3" and so on)
says that P-101 runs first, that this session writes its FRAME and stops there, and that
it writes no kernel code (launch 3.1). No file under `crates/cj3-kernel`,
`crates/cj3-store` or `spec/` has changed. The session's other work is reported in
[`C12-p101-frame-ruling-builder.md`](C12-p101-frame-ruling-builder.md).

**Everything in sections 4 to 10 is a proposal.** Nothing in it is ruled, and no tag is
assigned. Where a decision is already settled, section 11 names the place that settles
it.

Clause names, as elsewhere: `G0-` plus a label is the 2026-11-22 ruling; "resume 2.6" the
P-007 resume ruling; "pass 3.5" the Gate G0-F ruling; "launch 3.3" this one. A bare
B1–B12 is a block rule, V1–V9 a transaction rule, D1–D17 a ledger decision. Decisions
that this FRAME puts to Al are numbered K-1 to K-12, and settled ones S-1 to S-20, so
that neither collides with those.

## 1. The packet, in my own words

P-101 is the head of Phase 1. It turns the ratified Lean encoding of Protocol Spec §7,
§8 and §11 into Rust:

- the one validity predicate for a transaction, V1 to V9, first failure wins;
- the state-transition function: for each transaction, validate against the state as it
  now stands, then debit, credit, pay the fee and raise the nonce, each with checked
  arithmetic; then credit the Model 4 reward; then compare the state commitment with the
  header; and keep the result only if every step succeeded;
- a concrete store of accounts that obeys the three `LawfulStateOps` laws, with no other
  way to write to it;
- a reward whose quality input can only be the checker's output for this block;
- and the two commitments the header carries as 32 opaque bytes today: `tx_root` and
  `state_root`. How they are built was carried to this FRAME (G0-F3, pass 3.5).

**Done-condition, proposed.** P-101 is done when Al has ruled the decisions of section 3;
the kernel, the store and the two commitments are merged by Al with exact-merge-SHA D6
green; the D6 job `conservation-invariant` runs a real test in place of its deferral
marker; the `lean-conformance` job runs the Rust kernel against the Lean cases in place
of `P101_RUST_CONFORMANCE=NOT_YET_ADMITTED`; and the report shows the three laws
discharged as K-4 is ruled.

## 2. Lane

**HUMAN, every part.** Argued on the five D17 conditions:

| Condition | Holds? | Why |
|---|---|---|
| Approved and unblocked | Yes, to its FRAME | Launch 3.1. The build needs Al's ruling on this FRAME first (launch 4.7). |
| Bounded non-semantic surface | **No** | V-rules and the STF are the first two surfaces the prompt's SCOPE tripwire names. Two new consensus constructions are added. |
| No formal-spec content | **No** | K-4 adds a Lean file under `spec/Spec/`, and K-5 binds vector cases, which D17 names as HUMAN. |
| No decision invention | Yes, as a control | Every decision is in section 3 for Al. |
| No owned-TBD fill | Yes, as a control | Section 9 shows that the kernel takes each unfilled value as an input. |

The builder merges nothing in P-101.

## 3. The decisions P-101 needs from Al

Twelve are open. Each has a section that gives the question, the proposal, what else was
weighed, the evidence, and what it would cost to change after Phase 1 (launch 3.2).

| ID | Decision | Proposal, in one line | Section |
|---|---|---|---|
| K-1 | How `tx_root` is built | A list tree in the shape of RFC 6962, with tagged leaves and nodes and the count in the root | 4 |
| K-2 | How the state commitment is built, and what an empty account is | A binary radix tree over the address bits that records each split, with the count in the root; an account of balance 0 and nonce 0 is not a leaf | 5 |
| K-3 | Six new domain tags | Names and bytes proposed. **Al assigns tags** | 6 |
| K-4 | How the store discharges the three laws, and what "prove" binds | A Lean model of the store with the three laws proved, and the Rust store tied to it by tests and by having one write path | 7 |
| K-5 | How the kernel is tested against the Lean cases while they stay symbolic | A reviewed table that gives each of the 27 cases concrete, labeled test values; the verdicts come from the Lean artifact | 8.1 |
| K-6 | Whether Lean also exports cases it has computed | Yes, as a second and separate artifact, at the level of a decoded candidate | 8.2 |
| K-7 | Where the first half of B9 (`tx_root` matches the body) is checked | In the kernel, before any transaction is applied. The Lean model has no place for it today | 10.1 |
| K-8 | Who computes the quality Q that enters the reward | The kernel itself, by calling the checker on the derived instance. No caller can hand one in | 10.2 |
| K-9 | The Rust types of the unfilled inputs | `R_MAX` as a non-zero `u64`; the subsidy as a per-height input of type `Amount` | 9 |
| K-10 | Two new edges between crates | `cj3-kernel` → `cj3-store` and `cj3-kernel` → `cj3-classes` | 10.3 |
| K-11 | What P-101 leaves to the other Phase 1 packets | Durable storage and crash consistency; genesis; the mutation spot check and the G1 report | 12 |
| K-12 | One pull request or several | Three, in order: bytes, store, kernel | 13 |

## 4. K-1 — `tx_root`

**Question.** The header carries `tx_root` as 32 opaque bytes (G0-C3d). Protocol Spec §6.1
calls it the "Merkle root of canonical tx encodings". Which tree, which hash inputs?

### 4.1 The structure, and why this one (launch 3.3a)

A binary hash tree over the transactions in block order, in the shape that RFC 6962
(Certificate Transparency) defines for a list, with three changes: every hash input opens
with a 16-byte domain tag, as every other preimage of CJ3 does; the number of
transactions is hashed into the root; and the empty list has a root of the same form.

For a list `T = [t_0, …, t_(n-1)]`, each `t_i` the 188 bytes of a transaction (§7):

```text
leaf(t)      = H(D_TXLEAF ‖ t[188])
node(l, r)   = H(D_TXNODE ‖ l[32] ‖ r[32])

top([t])     = leaf(t)
top(T)       = node(top(T[0..k)), top(T[k..n)))      for n >= 2,
               where k is the largest power of two strictly less than n

tx_root(T)   = H(D_TXROOT ‖ n u32 ‖ top[32])
               where top = top(T) if n >= 1, and 32 zero bytes if n = 0
```

Why this one:

- **It is the shape the specification already names**, a Merkle root over the canonical
  encodings, and RFC 6962's split rule is the long-published way to define one for a
  count that is not a power of two.
- **It never counts a node twice.** The two halves `T[0..k)` and `T[k..n)` share no
  transaction and together are the whole list. The same tree can be computed level by
  level, pairing neighbours: an odd node at the end of a level is carried up as it is,
  never paired with itself.
- **It gives a short proof that a transaction is in a block**: the transaction, its
  position and at most 13 sibling hashes at the provisional `BLOCK_MAX_BYTES`, which
  admits at most 5,575 transactions.
- **A header then determines its whole body.** `tx_root` binds the count and every byte
  of every transaction, signatures included; `sol_commit` already binds the solution.

### 4.2 Every hash input (launch 3.3b)

Laid out as Protocol Spec §1.2 lays out a preimage: the tag, then fixed-width fields in
order, each encoded by §1.1.

| Value | Definition | Preimage length |
|---|---|---|
| transaction leaf | `H(D_TXLEAF ‖ transaction[188])` | 204 bytes |
| transaction node | `H(D_TXNODE ‖ left[32] ‖ right[32])` | 80 bytes |
| `tx_root` | `H(D_TXROOT ‖ count u32 ‖ top[32])` | 52 bytes |

- `transaction` is the 188-byte layout of §7, whole: all eight fields, `sig` included.
- `left` and `right` are each a transaction leaf or a transaction node.
- `count` is the number of transactions, the same number as the `u32` count in front of
  the transaction list in the block (§6.3).
- `top` is the hash of the whole tree, or 32 zero bytes when `count` is 0.
- Under each tag every preimage has one length and the same field positions, and the
  three tags differ from each other and from the seven ruled tags, as G0-C2d requires of
  the others. No new use of `H` without a tag is added.

The three tags are proposed in section 6.

### 4.3 Encoding, order and the empty case (launch 3.3c)

- **Leaf.** The transaction's 188 canonical bytes, unchanged. Nothing is added: no index,
  no length.
- **Order.** Block order: the order of the transaction list in the block, which is the
  order §8 applies them in. Nothing is sorted.
- **Empty list.** `count` is 0 and `top` is 32 zero bytes, so
  `tx_root = H(D_TXROOT ‖ 00000000 ‖ 00…00)`, a constant:
  `7a75fa88e989eb1ef8050356562439fd2b97d5f69e239bf453e6f4c73bb24b1b` with the proposed
  tag. A non-empty list can have a `top` of 32 zero bytes only by a hash that outputs
  them, and even then its `count` differs, so the two never share a preimage.
- **One transaction.** `top` is its leaf. There is no node.

### 4.4 Two different lists never give the same root (launch 3.3d)

"Never" means: unless two different byte strings with the same SHA-256 are found. That
is the one assumption, and it is the assumption every hash in §1.2 already rests on.

Suppose `tx_root(T) = tx_root(T')`. The two root preimages have the same tag and length,
so they are equal byte for byte or SHA-256 has collided. Equal preimages give the same
`count` and the same `top`. If the count is 0, both lists are empty. Otherwise both
lists have the same length n, and I show by induction on n that equal tops mean equal
lists.

- n = 1. Both tops are leaf hashes of 204-byte preimages. Equal preimages give the same
  188 bytes.
- n ≥ 2. The split point k depends on n alone, so both tops are node hashes, over halves
  of lengths k and n − k in both lists. Equal node preimages give equal left hashes and
  equal right hashes. By induction the left halves are equal and the right halves are
  equal.

The three properties the ruling names:

- **No last node counted twice.** Each transaction is under exactly one leaf, and the
  definition never pairs a hash with itself. A tree that repeats its last node at an odd
  level gives `[a, b, c]` and `[a, b, c, c]` the same root (CVE-2012-2459). Here they
  differ twice over: by the count, and by the shape. The script checks it for every
  length from 1 to 39, and the worked example shows one case.
- **Leaves and inner nodes hashed differently.** Different tags and different lengths
  (204 and 80 bytes). The induction above does not need this, because the count already
  fixes which hashes are leaves. It protects any place that handles a subtree hash
  without the count beside it.
- **The count bound into the root.** It is a field of the root preimage, and it fixes the
  shape.

### 4.5 Proofs (launch 3.3e)

P-101 would build neither. This is what each would consist of.

- **That a transaction is present at a position.** The count n; the position i; the 188
  bytes; and the sibling hashes from the leaf up to the top, lowest first. Their number
  is fixed by (i, n). The verifier walks the split rule from (0, n) down to i to learn
  on which side each sibling stands, recomputes the top, then computes
  `H(D_TXROOT ‖ n ‖ top)` and compares with the header's `tx_root`. Size:
  `8 + 188 + 32 × depth` bytes, and depth is at most `ceil(log2 n)`.
- **That nothing is at a position.** The count and the top open the root; a position at
  or past the count holds nothing.
- **That a given transaction is not in a block.** There is no short proof. The tree is
  ordered by position, not by content, so the only proof is the whole list. A light
  client cannot learn cheaply that a transaction was left out. If that is wanted later,
  it needs a second commitment ordered by content, and a header field to carry it.

### 4.6 Worked example (launch 3.3f)

Derived by `bench/p101-roots/derive.py`, which uses only the Python standard library.
`tx1` and `tx2` are the two worked transactions of the normative Encoding v1 vectors
(`out.tx1.bytes`, `out.tx2.bytes` in `bench/p007-vectors/encoding-v1.vectors.txt`). The
list is `[tx1, tx2, tx1]`. A block could not hold `tx1` twice, because V4 would reject
the second; the root is defined for any list, and the repeat shows that a repeated entry
is hashed as an entry of its own.

For a count of 3 the split point is 2: `top = node(node(leaf 0, leaf 1), leaf 2)`.

```text
D_TXLEAF (proposed)  434a332f76312f74786c656166000000
D_TXNODE (proposed)  434a332f76312f74786e6f6465000000
D_TXROOT (proposed)  434a332f76312f7478726f6f74000000

tx1, 188 bytes
  010203049ef0a8666abb323a768463c016e0c046cb423892ea3740057bd69efc
  13ab9ca7c7323f203903fd278b28f0945f0113d2d869a7e03fecd03d7295a1aa
  56e3580a88776655443322110807060504030201a500000000000000d75a9801
  82b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a88f28fc9
  f824eda4adecf1c4c78bfa09e0e7a4925192cfdc82def4b653bdd3233019bc86
  b7c1e95c4cca95963ff3f400ea170760c16081e0e7a38cca7a419c03

tx2, 188 bytes
  01020304c7323f203903fd278b28f0945f0113d2d869a7e03fecd03d7295a1aa
  56e3580ac7323f203903fd278b28f0945f0113d2d869a7e03fecd03d7295a1aa
  56e3580a0000000000000000ffffffffffffffff01000000000000003d4017c3
  e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c6ca82445
  a38ff5436509a6fba739492a1a4f890f9ca7d181a243fd634c37b737063db09b
  492eed43eae97492e62eb561af881f7b2283ff202df99bd082aa4303

leaf 0 = H(D_TXLEAF ‖ tx1)                       204-byte preimage
       = 9faaa6eb02f154e1fdbe8e03b7005fa45010ebbe53411e744fcc4a077d4c4b0c
leaf 1 = H(D_TXLEAF ‖ tx2)                       204-byte preimage
       = 0552a38452496711b93f6fdcce039f74bff665b3f041c8403eb8ac3a274ff6ba
leaf 2 = H(D_TXLEAF ‖ tx1)                       the same preimage as leaf 0
       = 9faaa6eb02f154e1fdbe8e03b7005fa45010ebbe53411e744fcc4a077d4c4b0c

node over leaves 0 and 1, 80-byte preimage
  434a332f76312f74786e6f6465000000
  9faaa6eb02f154e1fdbe8e03b7005fa45010ebbe53411e744fcc4a077d4c4b0c
  0552a38452496711b93f6fdcce039f74bff665b3f041c8403eb8ac3a274ff6ba
       = 14058d693cdfa83e1c5365c0346c3e0789e4a5c050f5de7d64a0fab3714c3950

top: node over that node and leaf 2, 80-byte preimage
  434a332f76312f74786e6f6465000000
  14058d693cdfa83e1c5365c0346c3e0789e4a5c050f5de7d64a0fab3714c3950
  9faaa6eb02f154e1fdbe8e03b7005fa45010ebbe53411e744fcc4a077d4c4b0c
       = bd68584813606c2efcb5c55317d68543d9bfc22d62c8a4b8b44decfd2a17ad55

tx_root, 52-byte preimage
  434a332f76312f7478726f6f74000000                                   D_TXROOT
  03000000                                                           count u32
  bd68584813606c2efcb5c55317d68543d9bfc22d62c8a4b8b44decfd2a17ad55   top
       = 5a8fcf32579b618ffb1237d03bce98d7d4651ec72d14bd7b380972d91711052b
```

Other lists, for comparison:

| List | Count | `tx_root` |
|---|---:|---|
| (none) | 0 | `7a75fa88e989eb1ef8050356562439fd2b97d5f69e239bf453e6f4c73bb24b1b` |
| tx1 | 1 | `b220b0acadf52d9183aa9cc497b966ba4e28cfc7a3cd83c9a4cb09629defbec9` |
| tx1 tx2 | 2 | `5ba5f149304ac02e73b0e3140737cd49c02da092d24298642ad6eb6180a6a7c7` |
| tx2 tx1 | 2 | `62e7f04d50f4021788dec500df59c55d4a0a83344d991b29ec0a9d2a4d3120ae` |
| tx1 tx2 tx1 | 3 | `5a8fcf32579b618ffb1237d03bce98d7d4651ec72d14bd7b380972d91711052b` |
| tx1 tx2 tx1 tx1 | 4 | `2efc5a47e92751fb2c1815f3f339d4922783ba07035f2a92a63c58d475f187f2` |

The last two rows are the case that a tree which repeats its last node gets wrong.
[`bench/p101-roots/WORKED-EXAMPLES.md`](../../bench/p101-roots/WORKED-EXAMPLES.md) shows
every preimage field by field, and a proof that `tx1` is at position 2 of 3.

### 4.7 What else was weighed, the evidence, and the cost of changing it

| Alternative | Why not |
|---|---|
| Pair neighbours and repeat the last node at an odd level | Two different lists share a root (CVE-2012-2459). The ruling excludes it by name. |
| RFC 6962 exactly as written: prefix bytes `00` and `01`, no count, the empty root `SHA-256("")` | It would add uses of `H` without a tag, where §1.2 says `sol_commit`'s inner hash is the only one; and the count would not be in the root. |
| One hash over the count and all transactions end to end | Unambiguous, because every transaction is 188 bytes, and the least code. But a proof that a transaction is present would be the whole body; the preimage would vary in length under one tag, against G0-C2d; and §6.1 says "Merkle root". |
| Sort the leaves by content | It would allow a short proof of absence, but order is part of what a block means: §8 applies transactions in order. |

**Evidence.** RFC 6962 section 2.1 for the shape. CVE-2012-2459 for the repeated last
node. In the script, before anything is written: for every count from 0 to 69 the
recursive definition equals the level-by-level computation; every position's proof
verifies, and fails for another transaction, another position and another count; and for
every length from 1 to 39 a list and the same list with its last entry repeated have
different roots. The two public documents are cited from memory and were not fetched in
this session.

**Cost to change after Phase 1.** Every header commits to `tx_root`. Before a network
exists the cost is one function, its vectors and this section. Once a test network has a
genesis block, a change is a protocol upgrade with new tag bytes (§1.1(c)), and every
client that checks a proof changes with it.

## 5. K-2 — the state commitment

**Question.** Protocol Spec §8 keeps the state, `account(addr) -> { balance u64, nonce
u64 }`, "in an authenticated structure (Merkle-ized; structure choice in P-101 within
spec constraints)", and B10 compares its root with the header's `state_root`. Which
structure, which hash inputs, and what is an account that holds nothing?

### 5.1 The structure, and why this one (launch 3.3a)

A binary radix tree over the 256 bits of the address. A leaf is one account. A node
stands where the addresses below it first differ, and its preimage records that bit
position. A node therefore always has two children. The number of accounts is hashed
into the root.

A state `σ` gives every 32-byte address an account, and all but finitely many accounts
are (balance 0, nonce 0). Its leaves are the others:

```text
Leaves(σ)        = { (a, σ(a)) : σ(a) is not (balance 0, nonce 0) }

bit(a, i)        = bit (7 - i mod 8) of byte floor(i / 8) of a,   for 0 <= i < 256
                   (bit 0 is the most significant bit of the first byte)

leaf(a, b, n)    = H(D_STLEAF ‖ a[32] ‖ b u64 ‖ n u64)
node(d, l, r)    = H(D_STNODE ‖ d u16 ‖ l[32] ‖ r[32])

top({x})         = leaf(x)
top(L)           = node(d, top(L0), top(L1))                      for 2 or more leaves,
                   where d is the first bit at which two addresses in L differ,
                   L0 is the leaves of L whose address has bit d = 0, and L1 the rest

state commitment = H(D_STROOT ‖ count u64 ‖ top[32])
                   where count is the number of leaves, top = top(Leaves(σ)) if
                   count >= 1, and 32 zero bytes if count = 0
```

Why this one:

- **The root is a function of the state and of nothing else.** The definition reads only
  the set of leaves. The order in which accounts were written, and any that were written
  and emptied, leave no trace. Two nodes that hold the same accounts compute the same
  root whatever their history. The script checks this against a second, incremental
  implementation, in three different orders of insertion and deletion, on sixty
  generated states.
- **Its size is a function of the number of accounts alone.** n leaves give exactly
  n − 1 nodes. Addresses that share a long prefix cost one node, not one node per shared
  bit. The worked example has two addresses that share 255 bits.
- **One account changes one path.** Changing an account rehashes its leaf and the nodes
  above it. Adding or removing one adds or removes one leaf and one node. So a short
  proof can show the commitment before and after a change, and a store can update the
  commitment without touching the rest. P-101's first implementation would not use this:
  it would recompute from the definition at each block (section 7.2). The structure
  leaves the door open and costs nothing for doing so.
- **One path proves presence and one path proves absence** (section 5.5).
- **There is no "empty" value inside the tree.** Every child of every node is a leaf or
  a node, so no constant has to be kept apart from the hash outputs.

### 5.2 Every hash input (launch 3.3b)

| Value | Definition | Preimage length |
|---|---|---|
| account leaf | `H(D_STLEAF ‖ address[32] ‖ balance u64 ‖ nonce u64)` | 64 bytes |
| state node | `H(D_STNODE ‖ split u16 ‖ left[32] ‖ right[32])` | 82 bytes |
| state commitment | `H(D_STROOT ‖ count u64 ‖ top[32])` | 56 bytes |

- `address` is the account's 32-byte address. `balance` and `nonce` are its two fields,
  in the order §8 writes them, each a `u64` as §1.1(a) encodes one.
- `split` is the bit position, 0 to 255, at which the addresses under the node first
  differ. `left` is the subtree whose addresses have 0 at that bit, `right` the subtree
  whose addresses have 1. Each is an account leaf or a state node.
- `count` is the number of account leaves. `top` is the hash of the whole tree, or 32
  zero bytes when `count` is 0.
- The state commitment is the value the header carries as `state_root`.
- Under each tag every preimage has one length and the same field positions.

Why the split is in the node's preimage: section 5.4 shows that the leaves alone already
tell two states apart. The split is there for proofs. A verifier has to know at which
bit each node divides its addresses, and it can trust that only if the root binds it.

### 5.3 Encoding, order and the empty cases (launch 3.3c)

- **An account** is 16 bytes: `balance u64 ‖ nonce u64`, little-endian as §1.1(a) gives.
- **A leaf** is 48 bytes after its tag: `address[32] ‖ balance u64 ‖ nonce u64`. The
  address is in the leaf, so a leaf says whose account it is without its position.
- **The order of keys.** Ascending by address, the 32 bytes compared as unsigned bytes
  from the first byte on. Left is 0 and right is 1 at every node, so reading the leaves
  from left to right gives that order. This is an order on byte strings. It does not
  decide how a 32-byte value is read as a number, which pass 3.5 carries to the first
  Phase 2 FRAME; see point 6 of section 15.
- **An empty account** is one with balance 0 and nonce 0. It is not a leaf. An address
  that was never written and an address whose account is (0, 0) are the same thing, in
  the commitment as in the Lean model, where `account` returns a value for every
  address. Writing (0, 0) to an address removes its leaf if it had one.
- **The empty state.** `count` is 0 and `top` is 32 zero bytes, so the commitment is the
  constant `H(D_STROOT ‖ 0000000000000000 ‖ 00…00)`:
  `e4411c856160484248e2847be8afe5b20110fe4cf7a5db73d650063b414e6580` with the proposed
  tag.
- **One account.** `top` is its leaf. There is no node.

Under the STF as ratified, a leaf is never removed in practice: a balance falls only by
a debit, and the same transaction raises the sender's nonce. The empty-account rule
matters for one case today, a transfer of amount 0 to an address with no account. It
writes (0, 0) there, and the state must not change.

### 5.4 Two different states never give the same root (launch 3.3d)

The same one assumption as in 4.4.

*The commitment is well defined.* `Leaves(σ)` depends on σ alone. `top` depends on the
set of leaves alone: the first differing bit is a property of the set, and so are the
two halves. So equal states give equal commitments.

*Different states give different commitments.* Suppose two states have the same
commitment. The root preimages are equal, so the counts are equal and the tops are
equal. If the count is 0, both states are empty. Otherwise I show by induction on the
number of leaves that two sets of leaves with the same top are the same set.

- A leaf preimage is 64 bytes under `D_STLEAF` and a node preimage is 82 bytes under
  `D_STNODE`. A leaf hash that equals a node hash is a collision. So the two tops are
  both leaves or both nodes.
- Both leaves: equal preimages give the same address, balance and nonce.
- Both nodes: equal preimages give the same split, the same left hash and the same right
  hash. Each side has fewer leaves, so by induction the left sets are equal and the
  right sets are equal.

Two states with the same leaves are the same state, since every other account is (0, 0)
in both.

The three properties the ruling names:

- **No node counted twice.** `L0` and `L1` share no leaf and together are `L`, and both
  are non-empty by the choice of the split. Nothing is repeated or padded.
- **Leaves and inner nodes hashed differently.** Different tags and different lengths.
  Here the induction needs it: the shape is not known in advance, so a leaf must not be
  mistaken for a node.
- **The count and the shape bound into the root.** The count is a field of the root.
  The shape is bound node by node: each node hashes its split, and each leaf hashes its
  whole address.

### 5.5 Proofs (launch 3.3e)

P-101 would build neither. This is what each would consist of.

- **That an account is present.** The count; the leaf, as address, balance and nonce; and
  the steps from the leaf up to the top, lowest first, each a split and a sibling hash.
  At each step the verifier reads the bit of the address at the split: 0 puts the
  running hash on the left, 1 on the right. It requires the splits to decrease strictly
  from the leaf upward and each to be below 256, recomputes the top, then computes
  `H(D_STROOT ‖ count ‖ top)` and compares with the header's `state_root`. Size:
  `8 + 48 + 34 × depth` bytes. Depth is at most 256, and at most the number of other
  accounts.
- **That an address has no account.** If the count is 0, the root is the constant of the
  empty state. Otherwise: the leaf that is reached by following the *asked* address's
  bits down the tree, with its steps. The verifier takes each side from the bits of the
  address it asked for, recomputes the commitment, and checks that the leaf's address is
  a different one. If the asked address had an account, the same walk would have ended
  at it, because at every node on the way it would be among the addresses below.

The proof of absence is sound for a root that is the commitment of some state, which is
what B10 makes of every `state_root` in a valid block. A verifier that takes a root from
a header relies on B10 for that, as it does for everything else the root means.

### 5.6 Worked example (launch 3.3f)

Derived by the same script. Three addresses are `addr` of the RFC 8032 test keys 1 to 3
(`out.addr.key1` to `key3` in the normative vectors). The fourth, `near`, is the address
of key 1 with its last bit flipped; no key is known for it, and an address in a
transaction is any 32 bytes. The balances and nonces are test values and assign nothing.

| Account | Address | Balance | Nonce |
|---|---|---:|---:|
| key3 | `0e6c2ed42872d1765d664649e3ee0fd1cf78ecc2bb7f8e007012fafb7c4666b4` | 578437695752307201 | 0 |
| near | `9ef0a8666abb323a768463c016e0c046cb423892ea3740057bd69efc13ab9ca6` | 1 | 0 |
| key1 | `9ef0a8666abb323a768463c016e0c046cb423892ea3740057bd69efc13ab9ca7` | 0 | 1735880461161533969 |
| key2 | `c7323f203903fd278b28f0945f0113d2d869a7e03fecd03d7295a1aa56e3580a` | 18446744073709551615 | 2 |

The rows are in ascending order of address. The first bytes are `0e`, `9e`, `9e`, `c7`,
in bits `0000 1110`, `1001 1110`, `1001 1110`, `1100 0111`. All four first differ at bit
0, where key3 has 0. The other three first differ at bit 1, where key2 has 1. `near` and
key1 first differ at bit 255. So:

```text
top = node(0, leaf key3, node(1, node(255, leaf near, leaf key1), leaf key2))

D_STLEAF (proposed)  434a332f76312f73746c656166000000
D_STNODE (proposed)  434a332f76312f73746e6f6465000000
D_STROOT (proposed)  434a332f76312f7374726f6f74000000

leaf key3, 64-byte preimage
  434a332f76312f73746c656166000000                                   D_STLEAF
  0e6c2ed42872d1765d664649e3ee0fd1cf78ecc2bb7f8e007012fafb7c4666b4   address
  0102030405060708                                                   balance u64
  0000000000000000                                                   nonce u64
       = 5fcdfcc494b2fba4a6b59e5956ae36688cbcabb30798704f071943855570b8cc

leaf near, 64-byte preimage
  434a332f76312f73746c656166000000
  9ef0a8666abb323a768463c016e0c046cb423892ea3740057bd69efc13ab9ca6
  0100000000000000
  0000000000000000
       = 183936ce3697ca26379fe4d829be41d6034c9c02e1a721e25cfd3e9615da54c0

leaf key1, 64-byte preimage
  434a332f76312f73746c656166000000
  9ef0a8666abb323a768463c016e0c046cb423892ea3740057bd69efc13ab9ca7
  0000000000000000
  1112131415161718
       = 575879aa4e81db5c348e68964ebb593e8a0ccb92a1bed99ec801f808c6f7ff86

leaf key2, 64-byte preimage
  434a332f76312f73746c656166000000
  c7323f203903fd278b28f0945f0113d2d869a7e03fecd03d7295a1aa56e3580a
  ffffffffffffffff
  0200000000000000
       = 40bc503ca99c1e74bc766fbc4aa0273b615644a5b638abfa4170319bdf0d829f

node at split 255, over near and key1, 82-byte preimage
  434a332f76312f73746e6f6465000000                                   D_STNODE
  ff00                                                               split u16
  183936ce3697ca26379fe4d829be41d6034c9c02e1a721e25cfd3e9615da54c0   left
  575879aa4e81db5c348e68964ebb593e8a0ccb92a1bed99ec801f808c6f7ff86   right
       = 11ad17b62796475a071ae0adbe1f46a9290b44fabdff5248ef7aee9d2c70703f

node at split 1, over that node and key2, 82-byte preimage
  434a332f76312f73746e6f6465000000
  0100
  11ad17b62796475a071ae0adbe1f46a9290b44fabdff5248ef7aee9d2c70703f
  40bc503ca99c1e74bc766fbc4aa0273b615644a5b638abfa4170319bdf0d829f
       = f4b48ad64a1d1e85acc0af424039c7b161549b2e33be1be38a040cebb54b945b

top: node at split 0, over key3 and that node, 82-byte preimage
  434a332f76312f73746e6f6465000000
  0000
  5fcdfcc494b2fba4a6b59e5956ae36688cbcabb30798704f071943855570b8cc
  f4b48ad64a1d1e85acc0af424039c7b161549b2e33be1be38a040cebb54b945b
       = 623e68172fcf98e3ff9130a334666c7f8b1f7f6ff72eb3f55d5bdf46d313f18d

state commitment, 56-byte preimage
  434a332f76312f7374726f6f74000000                                   D_STROOT
  0400000000000000                                                   count u64
  623e68172fcf98e3ff9130a334666c7f8b1f7f6ff72eb3f55d5bdf46d313f18d   top
       = e0644a8dee6570eb9cc507dd934521c17db42a3f83520ed940a7ec3f5a0fefa4
```

A proof that key1's account is present: the count 4, the leaf, and three steps.

```text
split 255  sibling 183936ce3697ca26379fe4d829be41d6034c9c02e1a721e25cfd3e9615da54c0
split   1  sibling 40bc503ca99c1e74bc766fbc4aa0273b615644a5b638abfa4170319bdf0d829f
split   0  sibling 5fcdfcc494b2fba4a6b59e5956ae36688cbcabb30798704f071943855570b8cc
```

Bits 255, 1 and 0 of key1's address are 1, 0 and 1, so the running hash goes right,
left, right.

A proof that an address has no account. The address asked for is
`d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a`, the 32 bytes of test
key 1's public key, used here only as an address that holds nothing. Its bits 0 and 1
are both 1, so the walk goes right twice and reaches the leaf of key2. The proof is the
count 4, that leaf, and its two steps:

```text
split   1  sibling 11ad17b62796475a071ae0adbe1f46a9290b44fabdff5248ef7aee9d2c70703f
split   0  sibling 5fcdfcc494b2fba4a6b59e5956ae36688cbcabb30798704f071943855570b8cc
```

Other states, for comparison:

| State | Leaves | State commitment |
|---|---:|---|
| no accounts | 0 | `e4411c856160484248e2847be8afe5b20110fe4cf7a5db73d650063b414e6580` |
| key3 alone | 1 | `3a9276d97b7498b43cae86b04374fffad9edb14da2db91eaf312ebb19589f30b` |
| the four accounts above | 4 | `e0644a8dee6570eb9cc507dd934521c17db42a3f83520ed940a7ec3f5a0fefa4` |
| the same, plus an address whose account is (0, 0) | 4 | `e0644a8dee6570eb9cc507dd934521c17db42a3f83520ed940a7ec3f5a0fefa4` |
| the same, without `near` | 3 | `acdf4c3adfeed61b529a48eb26c836b558b203e877ef773786073040d06ae99e` |
| the same, with one unit more on key3 | 4 | `01e0b0b4b5d0e548273b0b3938dfb75a1f2e46fe937562d3a28e1423bc241508` |

The third and fourth rows are equal on purpose.

### 5.7 What else was weighed, the evidence, and the cost of changing it

| Alternative | For it | Why not |
|---|---|---|
| **The list tree of K-1 over the leaves in address order** | One tree for both roots: the least code and the least to review. Presence is K-1's proof; absence is two neighbouring leaves | Adding an account moves every later leaf one place, which changes the shape to its right. The commitment after such a change cannot be computed from a short proof, and a store cannot update it without the leaves to the right. This is the alternative I would choose if one tree for both roots is worth more than that. |
| A radix tree with a node at every bit and a constant for an empty side, single leaves collapsed | No split field. It is the commonest form of "sparse Merkle tree" | It needs a value for "empty" that no hash may ever equal. And two addresses that share k bits cost k nodes: an account holder can search for such pairs, so the size of the tree is not a function of the number of accounts. |
| A 16-way trie with a serialisation format, as Ethereum uses | Widely deployed | Much more code, and an encoding that this repository does not have. |
| A balanced search tree, hashed as it stands | Simple to update | Its root depends on the order of insertions, so it is not a function of the state. It fails launch 3.3d. |
| One hash over the sorted leaves | The least code | No proof shorter than the whole state, and §8 says "Merkle-ized". |
| *Empty accounts:* keep a leaf for every address ever written | No rule for removal | A transfer of amount 0 would add a leaf for ever, at the price of a fee; Ethereum removed such leaves in EIP-161 after the attacks of 2016. And the commitment would no longer be a function of the state as the Lean model sees it, where `account` cannot tell "never written" from "empty". |

**Evidence.**

- The script's checks, run before anything is written: on sixty generated states of 0 to
  22 accounts, in which a third of the addresses are made to share 160 or more leading
  bits with one another, the commitment by the definition equals the commitment of a second implementation that inserts and
  deletes one key at a time, in ascending order, in descending order, and after writing
  other values and adding and removing other addresses. An added (0, 0) account changes
  nothing. Every account's proof of presence verifies and fails for a changed balance.
  Proofs of absence verify for addresses that hold nothing and fail for ones that hold
  something. A changed balance, a changed nonce and a removed account each change the
  root.
- Hash counts, exact: recomputing the commitment of n accounts costs n leaves, n − 1
  nodes and one root, 2n hashes. Each preimage is 56, 64 or 82 bytes, which is two
  SHA-256 compression calls, so 4n calls. Changing one account without adding or
  removing a leaf costs its depth plus 2 hashes.
- **Not measured:** what that costs in time with the portable SHA-256 that launch 4.3b
  now pins. P-101's build can measure it at 10^3 to 10^6 accounts and report it.
- EIP-161 is cited from memory and was not fetched in this session.

**Cost to change after Phase 1.** As for K-1, and more: the genesis header's
`state_root` is the first value computed with it (the Phase 1 genesis packet), and any
stored state, snapshot or proof depends on it. Before a network exists the cost is one
function, its vectors and this section.

**What this proposal does not bound.** The number of leaves. A new leaf costs what a
transfer costs, which is P-9's to set. Nothing here limits how large the state grows.

## 6. K-3 — six new domain tags

**Question.** Sections 4 and 5 need six tags that do not exist. Which names and bytes?

**Proposal.** Al assigns tags; this FRAME does not (launch 3.3b). Each proposed tag is 16
bytes, its ASCII text padded on the right with `0x00`, as G0-C2b lays out the seven.

| Proposed tag | ASCII text | The 16 bytes (hex) | Opens |
|---|---|---|---|
| `D_TXLEAF` | `CJ3/v1/txleaf` | `434a332f76312f74786c656166000000` | the transaction leaf preimage |
| `D_TXNODE` | `CJ3/v1/txnode` | `434a332f76312f74786e6f6465000000` | the transaction node preimage |
| `D_TXROOT` | `CJ3/v1/txroot` | `434a332f76312f7478726f6f74000000` | the `tx_root` preimage |
| `D_STLEAF` | `CJ3/v1/stleaf` | `434a332f76312f73746c656166000000` | the account leaf preimage |
| `D_STNODE` | `CJ3/v1/stnode` | `434a332f76312f73746e6f6465000000` | the state node preimage |
| `D_STROOT` | `CJ3/v1/stroot` | `434a332f76312f7374726f6f74000000` | the state commitment preimage |

The first three share their first nine bytes with `D_TX` (`CJ3/v1/tx`). They are still
different tags: a tag is all 16 bytes, and `D_TX` has `00` where these have `6c`, `6e`
and `72`. The script asserts that the six differ from each other and from the seven
ruled tags.

**Weighed.** *One node tag for both trees:* two tags fewer, but a hash from one tree
could then be offered as a node of the other, and showing that harmless takes an
argument that separate tags make unnecessary. *One tag per tree, with a byte for leaf,
node or root:* then one tag opens preimages of three lengths, against G0-C2d. *No tags,
RFC 6962's prefix bytes:* against §1.2, as in 4.7.

**Evidence.** Protocol Spec §1: tags are "byte-string constants in `cj3-types`, each
used exactly once". G0-C2d. The source-policy gate already fails on a ruled tag named in
any Rust source outside `crates/cj3-types`; the same rule would take the six new names
when they are assigned.

**Cost to change after Phase 1.** A tag is in every preimage it opens. Changing one is
changing the commitment.

## 7. K-4 — how the store discharges the three laws

**Question.** Protocol Spec §8: "the concrete authenticated store MUST instantiate the
Lean `LawfulStateOps` contract and prove, without axioms or admitted premises" the
three laws: read-after-write, read-other-address, and the additive replacement equation
`totalBalances(setAccount(s, a, new)) + balance(account(s, a)) = totalBalances(s) +
balance(new)`. A Lean theorem cannot be about Rust code. So what is proved, about what,
and what ties the Rust store to it?

### 7.1 The proposal

**The store, in Rust.** One type, in `cj3-store`, with private fields: an ordered map
from address to account that holds only the accounts that are not (0, 0). It has exactly
these operations:

```text
account(a)           the stored account, or (0, 0) if there is none
set_account(a, new)  if new is (0, 0), remove a's entry; otherwise store new at a
total_balances()     the sum of the stored balances, as a checked u128
commitment()         section 5, computed from the map
```

`set_account` is the only operation that changes a store. The kernel reads with
`account` and writes with `set_account`, in the sequence `Spec/Stf.lean` gives, and
there is no other path: no cache, no batch write, no adapter.

**The model, in Lean.** A new file, `spec/Spec/Store.lean`, defines the same store as a
Lean value: a list of (address, account) pairs with no address twice and no (0, 0)
account, and the same three operations by the same three rules. It then proves
`LawfulStateOps` for it, for every context whose `account` is the model's:

- *read-after-write* and *read-other-address* by cases on whether the address is in the
  list;
- *the additive equation* by induction on the list: removing a's entry lowers the sum by
  exactly a's old balance, which is 0 if there was no entry, and storing `new` raises it
  by `new`'s balance, which is 0 if `new` is (0, 0).

`conservationTarget_holds` then applies to the model with nothing left to assume.

The laws are stated for every address, of any length, because the Lean model's addresses
are byte strings. The model's keys are byte strings of any length too, so the laws hold
as stated, with no premise about length. The Rust store is that model with keys of 32
bytes only, and it stays inside those keys by its types: `set_account` takes an
`Address`.

**"Without axioms or admitted premises."** No `axiom`, `sorry` or `admit` in the file:
the Lean gate already fails on those words in every `Spec/*.lean`. P-101 would add one
more check to that gate: it prints the axioms the new theorem depends on and fails
unless they are among `propext` and `Quot.sound`. Those two are what the ratified
conservation proof depends on today:

```text
'Cj3.Spec.Stf.conservationTarget_holds' depends on axioms: [propext, Quot.sound]
'Cj3.Spec.Stf.applyTransaction_conserves' depends on axioms: [propext, Quot.sound]
'Cj3.Spec.Stf.reward_le_subsidy' depends on axioms: [propext]
```

(`#print axioms`, Lean 4.33.0, run on this machine against the built project; nothing in
the repository changed.) They are Lean's own, and its simplifier and `omega` use them.
If Al reads "without axioms" as excluding even those, the ratified proofs already do not
meet it, and that would be a finding about P-005, not a choice for P-101.

**What ties the Rust store to the model.** Not a proof. Four things:

1. *The same definition.* The Rust operations are the model's three rules over a
   standard ordered map, a few lines each, reviewed side by side with the Lean file.
2. *The laws as tests on the Rust store.* Each law is run over generated stores,
   addresses and accounts, through the same `account` and `set_account` the kernel
   calls. The alias cases of the ratified proof are each generated on purpose: sender =
   recipient, sender = miner, recipient = miner, all three equal, all distinct.
3. *One write path, enforced.* The store's fields are private and `set_account` is its
   only mutating operation, so the compiler refuses any other write.
4. *The Lean-computed cases of K-6*, if Al admits them: the model run in Lean, the store
   run in Rust, the same operations, compared.

That is the model-based conformance A8 and D5 prescribe: "the Rust kernel conforms via
generated test vectors in CI", "not extraction".

**Atomicity.** A block is applied to a copy of the store, and the copy replaces the
store only if every step succeeded. A copy of a lawful store is a lawful store, so no
second structure needs its own laws. Copying costs time and memory in proportion to the
number of accounts. That is not a consensus matter; the durable store (section 12) would
replace the copy with something cheaper, under the same laws and the same tests.

### 7.2 What else was weighed

| Alternative | Why not, or not now |
|---|---|
| Translate the Rust store to Lean with a tool (Aeneas) and prove the laws about the translation | It would be a proof about the code itself. It adds a research tool to the chain of trust, restricts the Rust that can be written, and D5 already records that this route "is not production-ready". If Al reads §8 as requiring a proof about the Rust code, this is the route, and it is a packet of its own. |
| A model checker or verifier for Rust (Kani, Creusot) | Bounded or annotation-heavy; a new tool in D6; and the result is not a Lean theorem, which is what `LawfulStateOps` is. |
| Tests only, no Lean model | Then `LawfulStateOps` is never instantiated, which §8 requires. |
| A trait in the kernel that any store may implement | Flexible, but every implementation would owe the laws, and a trait invites a second write path. §8 forbids an "alternate write path, cache, or adapter". |

**Evidence.** `spec/Spec/Stf.lean`: `StateOps` (three operations, of which
`stateRootMatches` appears in no law), `LawfulStateOps` (three laws, each for every
state and every address), `conservationTarget_holds`. The axiom listing above. Protocol
Spec §8, the last bullet. I have not written or compiled the model; that the three laws
can be proved for it in Lean 4.33 without Mathlib is a prediction, and section 14 names
it as the first risk.

**Cost to change after Phase 1.** The laws are fixed by the ratified Lean. What could
change is the store behind them. Because the commitment is a function of the state
(K-2), a different store that obeys the same laws gives the same roots, so replacing the
store is not a protocol change. That is the reason to settle K-2 as a function of the
state now.

## 8. K-5 and K-6 — testing the kernel against the Lean vectors

### 8.1 K-5 — the 27 symbolic cases

**Question.** `spec/vectors/p005-draft.json` holds 27 cases. Each `input_bytes` is an
expression in words, such as `canonical_tx_where_nonce_plus_1_eq_state_nonce`, not
bytes, and the artifact stays symbolic and non-normative (pass 2.4). Protocol Spec §14
says the kernel's conformance test "consumes the vectors; red = merge blocked". How does
Rust consume an expression in words?

**Proposal.** A table in the kernel's tests, one entry per case name. An entry builds the
concrete situation its expression describes — a state, parameters, and a transaction or
block — out of labeled test values, runs it through the kernel's public entry point, and
states what it observed. The *expected* side is not in the table: it is read from the
Lean artifact.

- The case list comes from the artifact. A case with no entry fails the test. An entry
  with no case fails the test. So a change to the Lean cases turns D6 red until the table
  is brought into line and reviewed.
- `expected.accepted` and `expected.rule` are compared with what the kernel returned: a
  rejection must be by the rule the case names.
- `expected.state` is one of a small set of strings (`exact_pre_block_state`,
  `applied_with_fee_transfer_and_nonce_increment`, and so on). Each string has one
  checking function, and an unknown string fails the test.
- Every entry carries its expression beside its values, so a reviewer can read the two
  together. Each entry also has a control: one named change to its values that must flip
  the verdict, so an entry cannot pass by building the wrong situation.
- Every value an entry uses is a test value and is named as one. None fills P-7, P-9,
  P-12 or a network identifier.
- **V7** (resume 2.6): no wire input can fail it, because the layout fixes `to` at 32
  bytes. Its rejecting case, `v7-destination-31-bytes-reject`, is supplied at the level
  of a decoded candidate, as its `interface` field already says. The kernel's rule
  functions are written over a candidate whose recipient is a byte string, so V7 is a
  real test of length and not a comment, and the table calls it with 31 bytes.
- Reading the artifact needs no new dependency. A standard-library script under
  `bench/` turns the JSON into the `key = value` text form that the P-007 tests already
  read, and checks the JSON's digest as it does so. D6 fails if the committed text
  differs from what the script derives. That is the P-007 pattern.

The binding is where meaning enters, and D17 names vector definitions as HUMAN. Al
reviews the table entry by entry.

**Weighed.** *Make the P-005 artifact concrete:* Al has ruled that it stays symbolic.
*Parse the JSON in Rust with a library:* a test-only dependency (`serde_json` and what
it brings, a float formatter among them) for 27 records of strings and booleans.
*Compare case names only:* that would test nothing.

**Evidence.** The artifact: 27 cases, each with `expected.accepted`, `expected.rule` and
`expected.state`; `scripts/ci/active/lean-conformance.ps1`, which already pins the 27
names and prints `P101_RUST_CONFORMANCE=NOT_YET_ADMITTED`.

**Cost to change after Phase 1.** Test code only. No protocol effect.

### 8.2 K-6 — cases that Lean has computed

**Question.** In K-5 Lean supplies the verdict and a person supplies the situation.
Should Lean also run its own definitions on concrete values and export what they give?

**Proposal.** Yes, as a second artifact, separate from the P-005 file and with a status
that Al rules. A new Lean module instantiates the ratified `validate` and
`applyBlockAtomic` on the store model of K-4, at the level of a decoded candidate:
amounts, fees, nonces, balances and the reward inputs are concrete numbers; the two
things Lean does not implement, signature verification and address derivation, are given
as stated outcomes per case. Lean evaluates and exports, for each case, the inputs and
its own result: accepted or the failing stage, and the accounts and total afterwards.
The Rust kernel runs the same inputs at the same level and must agree.

It is what makes the Lean encoding a mechanical second reviewer of the kernel's
arithmetic and ordering (risk R4), and not only of its case list.

**Weighed.** *K-5 alone:* less Lean to review, and the kernel's arithmetic is then
checked only by tests written by the kernel's author. *Concrete bytes and real
signatures in Lean:* that needs SHA-256 and Ed25519 in Lean.

**Evidence.** `Tx.validate` and `Stf.applyBlockAtomic` are computable definitions; the
exporter `lake exe vectors` already runs in D6. That the instantiation evaluates in
reasonable time is not tested here.

**Cost to change after Phase 1.** None to the protocol. The cost is now: roughly 300
lines of Lean in the HUMAN lane, which Al reviews.

## 9. K-9 and launch 3.5 — every unfilled value stays an input

The kernel takes four values that are not filled. It supplies none of them.

| Value | Status | How the kernel takes it | Proposed Rust type |
|---|---|---|---|
| The chain's network identifier (V8) | No identifier is assigned (pass 2.4) | A field of the parameters the caller passes to every entry point | `u32`, the width of the transaction's field |
| `FEE_MIN` (V5) | P-9, TBD, owner Al | The same | `Amount` |
| `R_MAX` (§11) | P-7, TBD, owner Al (+ Sarah) | The same | a non-zero `u64` |
| `subsidy(height)` (§11) | P-12, placeholder, owner Al + Ken | A schedule the caller passes, asked for the block's height | `Amount` per height |

**No code path supplies a default, by construction and by two gates.**

1. The parameter type has private fields and one constructor that takes every field. A
   caller cannot build it with one left out, and there is no update syntax from a base
   value because there is no base value.
2. It does not implement `Default`. A compile-time assertion in the crate makes adding
   such an implementation a build failure.
3. There is no constant, static or function in the crate, outside its tests, that
   returns a value of that type or of the schedule's. The kernel never calls the
   constructor: it only receives a reference. The source-policy gate gains a rule that
   the constructor's name appears in `crates/` only at its definition and in test code,
   and proves the rule on a planted source, as the domain-tag rule does. The first
   packet that needs real values, the genesis packet or the node, has to change that
   rule, in a pull request Al reviews.
4. There is no global and no environment variable. Every entry point takes the
   parameters as an argument.
5. The subsidy schedule is an interface with no implementation outside tests.
6. Test values are named as non-normative stand-ins and live only in test code.

`R_MAX ≥ 1` is carried by the type, so the divisor `R_MAX · SCALE` is non-zero before
any division, and the product is computed with checked `u128` multiplication as §11
requires. A non-zero `u64` times 1,000,000 always fits, and the check is kept anyway.

**K-9, the question in this.** The Lean model has `rMax : Nat`, with no upper bound. A
Rust type has one. *Proposal:* a non-zero `u64`. *Weighed:* `u128`, which the product
would then overflow, turning a parameter into a run-time error; `u32`, which is narrower
than any need shown. *Evidence:* `min(Q, R_MAX · SCALE)` with Q a `u64`: once
`R_MAX · SCALE` exceeds `u64::MAX` the cap never binds, and a larger `R_MAX` only
divides the reward further. *Cost to change:* it bounds what P-7's owner may choose, at
2^64 − 1. Widening it later is a type change, not a protocol change, unless a value
beyond the bound is wanted.

**Values the kernel does not take.** P-1, P-2, P-10 and the timestamp unit belong to
block rules, which are Phase 2. P-3 is never a run-time value (G0-B2b). P-4 and
`ACTIVE_SIZE_PARAM`: the kernel never compares against the active tuple, which is B3; it
reads the header's own `size_param` and only after the block rules have held (10.2).
`BLOCK_MAX_BYTES` is the provisional constant of `cj3-types` and bounds decoding, which
happens before the kernel. `TX_MAX_BYTES = 188`, `SCALE = 1,000,000` and `θ = SCALE` are
filled (G0-C6a; P-6 and P-5, definitional). D12's allowlist of test networks arrives
with the packet that first assigns an identifier.

## 10. The other open decisions

### 10.1 K-7 — where the first half of B9 is checked

**Question.** B9 has two halves: "`tx_root` matches body", and every transaction
satisfies V1–V9 in order. §8 lists the steps of `apply_block`: "Validate B1–B8,
B11–B12", then the transactions, then the reward, then B10. The first half of B9 is in
none of them. In `Spec/Stf.lean`, `Block` has no `tx_root`, and the comment on `Context`
says `blockRulesHold` "stands only for B1–B8 and B11–B12". So as written, nothing in §8
or in the Lean model checks that the header's `tx_root` is the root of the body.

**Proposal.** A ruling that the first half of B9 is among the block rules that
`blockRulesHold` stands for, and that P-101 checks it in the kernel's block entry,
before any transaction is applied: the kernel holds the header and the body together,
and A6 says the apply path trusts nothing. A mismatch is reported at the block-rules
stage. The Lean definitions do not change. One comment in `Spec/Stf.lean` would change
its wording, text only, under the conditions of resume 3.1.

**Weighed.** *Amend the Lean model:* give `Block` a `tx_root`, `StateOps` a matching
operation and `Error` a ninth case. It is the most faithful, and it reopens a ratified
file. *Leave it to Phase 2:* then the kernel can be handed a body its header does not
commit to, and only its caller stands in the way.

**Evidence.** Protocol Spec §6.2 (B9) and §8 (steps 1 to 4); `spec/Spec/Stf.lean`, the
`Block` structure and the comment above `Context`. I have not logged this in
`loop/reports/SPEC-ISSUES.md`: that file is outside this session's scope, and the gap is
put to Al here instead. If he wants it logged as SI-005, it is one entry.

**Cost to change after Phase 1.** Small while Phase 2 does not exist. After it, the
check has an owner on each side of an interface.

### 10.2 K-8 — who computes Q

**Question.** F3 binds `Context.rewardInputs` to
`check(derive_instance(instance_seed, size_param), solution)` and forbids any
block-supplied quality. Block validation, which runs that check for B6, is Phase 2. In
P-101, where does Q come from?

**Proposal.** The kernel computes it. At the one place where reward inputs are built, it
calls the class's `derive_instance` on the header's `instance_seed` and `size_param`,
decodes the solution with the class's codec, and calls `check`. The kernel's entry point
has no parameter, field or constructor through which a quality can arrive. Two things
make that structural:

- `Quality` can only be made inside `cj3-classes`: its constructor is private to that
  crate (`crates/cj3-classes/src/lib.rs`, `Quality::from_raw`). No other crate can turn
  a number into a `Quality`.
- The kernel calls the checker itself, on this block's fields, so the quality is not
  only *a* checker output but the output *for this block*.

The kernel derives nothing until the block rules have held. Deriving an instance costs
work in proportion to the tuple, and B3 is what bounds the tuple. Phase 2's B6 then has
no second check to run: it reads the kernel's result.

**Weighed.** *Q as an argument of type `Quality`:* the kernel stays free of
`cj3-classes`, as the plan's diagram draws it, but a caller could pass the quality of
another block, and only the G2 test would catch it. *A token type made by `cj3-classes`
that carries the seed, the tuple and the solution's digest beside Q:* sound, and more
machinery than one call.

**Evidence.** The LEDGER's first-review ruling (F3) and Protocol Spec §11; the private
constructor; the plan's Gate G2, which tests a spoofed quality and a wrong instance. The
normative worked block of Encoding v1 carries a solution that the checker accepts with
Q = 7,000,000 at the stand-in tuple, so P-101's tests can run the real derivation and
check without a ratified tuple.

**Cost to change after Phase 1.** It is the interface between the kernel and Phase 2's
block validation. Cheap before P-201 exists, a rewrite of both sides after.

### 10.3 K-10 — two new edges between crates

**Question.** Engineering Plan §2 draws `cj3-kernel` over `cj3-types` alone. K-4 and K-8
each need one more edge.

**Proposal.** `cj3-kernel` → `cj3-store`, for the one concrete store, and `cj3-kernel` →
`cj3-classes`, for the checker. Neither adds a dependency from outside the workspace or
a cycle: `cj3-store` and `cj3-classes` depend on `cj3-types` only. The plan's diagram
would gain the two edges in the same pull request, text only.

**Weighed.** The alternatives are the ones K-4 and K-8 rejected: a store trait in the
kernel, and Q as an argument.

**Evidence.** The manifests: `cj3-kernel`, `cj3-store` and `cj3-consensus` have no
dependencies today; `cj3-classes` has `cj3-types` and `sha3`.

**Cost to change after Phase 1.** A refactor inside the workspace. No protocol effect.

## 11. Settled, with the place that settles each

| ID | What is settled | Where |
|---|---|---|
| S-1 | The rule set V1–V9, their order, and that the first failure is the result | `spec/Spec/Tx.lean`, `validate`; Protocol Spec §7 |
| S-2 | V7 stays in the rule set and is tested at the level of a candidate | resume 2.6; `Tx.lean`, `v7` |
| S-3 | The sequence of reads and writes for one transaction, each read made after the write before it, and the eight stages at which a block can fail | `spec/Spec/Stf.lean`, `applyValidatedTransaction`, `Error` |
| S-4 | The reward: Model 4, `u128`, floor division, checked back to `u64`, at most the subsidy | the SI-004 ruling; `Stf.lean`, `rewardNat`, `reward_le_subsidy`; Protocol Spec §11 |
| S-5 | Reward inputs come only from the checker over the derived instance | the first-review ruling, F3; Protocol Spec §11. K-8 is only *how* |
| S-6 | A block is applied whole or not at all; no partial-apply interface | `Stf.lean`, `applyBlockAtomic`, `rejection_preserves_state`; Protocol Spec §8 step 5 |
| S-7 | One validity predicate, called by the mempool and by block application alike, on the transaction's bytes | Protocol Spec §7; `Stf.lean`, `applyTransaction` |
| S-8 | `H` is SHA-256; a tag is 16 bytes; integers are fixed-width little-endian | G0-C1, G0-C2 |
| S-9 | Every tagged preimage is built in `cj3-types`, which owns bytes. So the two commitments are functions of `cj3-types`, and the kernel and the store call them | resume 4.2; pass 4.5d and the source-policy gate |
| S-10 | Amounts are `u64` with checked arithmetic; no float anywhere near one | D7, A5 |
| S-11 | Conformance is by vectors in CI, not by extraction | D5, A8 |
| S-12 | The state is `account(addr) -> { balance u64, nonce u64 }` | Protocol Spec §8 |
| S-13 | `tx_root` and `state_root` are 32 bytes at fixed places in the header, and so each is a SHA-256 output | G0-C3d |
| S-14 | The P-005 vectors stay symbolic and non-normative | pass 2.4, resume 2.7 |
| S-15 | No `axiom`, `sorry` or `admit` in a Lean file | the first-review ruling, F1; the Lean gate |
| S-16 | The state root is compared after the reward is credited | `Stf.lean`, `applyBlockCandidate`; Protocol Spec §8 steps 3 and 4 |
| S-17 | B12: the block layout has no reward entry to match; the reward is derived and credited | G0-C3f; Protocol Spec §11 |
| S-18 | A new dependency is proposed in a FRAME with its A9 case, licence and pin. **P-101 proposes none** | resume 4.4 |
| S-19 | B1–B8 and B11 are block validation, Phase 2. The kernel takes them as a rule set it is given | Engineering Plan §4; `Stf.lean`, `blockRulesHold` |
| S-20 | The lane: HUMAN, as section 2 argues | D17 and the prompt's SCOPE tripwire |

## 12. K-11 — what P-101 leaves to the other Phase 1 packets (launch 3.4)

`loop/PACKETS.md` seeds P-101 alone; the plan says "P-101…P-104" and names their
subjects. The queue is Al's to approve (Engineering Plan §8).

**Proposal.** P-101 holds the kernel, the in-memory store with its laws, the two
commitments, and the two D6 jobs it owns. It leaves:

| Left to | What | Why not in P-101 |
|---|---|---|
| A durable-store packet (P-102) | The chain and state on disk; recovery; the crash test of AMEND-2, which kills the process in the middle of a block and requires the exact state from before the block; an update of the commitment that does not recompute the whole | G1 names crash consistency as a property of `cj3-store` on restart. It needs a storage format and a failure model, which are decisions of their own. |
| The genesis packet (P-103) | The genesis layout and its preimage (carried, pass 3.5); the allocations; the first state commitment; the genesis spend-test, which makes the D6 job `genesis-spend-test` live; the first network identifier and D12's allowlist | The ruling carries the layout to that packet. |
| A G1 closeout packet (P-104) | The mutation spot check on the validity predicate; the Gate G1 report; the README refresh the standing rule requires at a gate | They are gate criteria and judge the whole phase. |
| Phase 2 | B1–B8 and B11 as real rules; the mempool (GAP-9) | Engineering Plan §4. |
| Whichever packet first needs them | A proof format on the wire for 4.5 and 5.5; a transaction identifier, which nothing in P-101 needs | Neither is used inside the kernel. |

**Weighed.** *Durable storage inside P-101:* then the laws, the kernel and a storage
format are reviewed in one packet, and the kernel waits on a format decision.

**Evidence.** Engineering Plan §4 (Phase 1 and Gate G1); the owners table in
`scripts/ci/check-phase-gate.ps1`, which gives `conservation-invariant` and
`lean-conformance` to P-101 and `genesis-spend-test` to the genesis packet; AMEND-2.

**Cost to change after Phase 1.** None to the protocol. It is the order of work.

## 13. K-12 — one pull request or three (launch 3.6)

**Proposal: three, in this order, each in the HUMAN lane, each with its own exact-head
D6, each merged by Al.** They ask a reviewer three different questions.

| Part | Holds | The reviewer's question | Predicted size |
|---|---|---|---|
| **A. Bytes** | In `cj3-types`: the six tags as Al assigns them, `tx_root`, the state commitment. The worked examples become vectors derived twice, by the script and by Rust; their status is Al's to rule at review, as with P-007. The source-policy rule takes the new names. Protocol Spec §1.2, §6.1, §8 and §15 gain the ruled text | Do these bytes match the ruling? | About 250 lines of Rust source, 350 of tests, 150 changed in the script, plus generated vectors and the specification text |
| **B. Store** | `cj3-store`: the in-memory store and its four operations. `spec/Spec/Store.lean`: the model and the three laws. The axiom check in the Lean gate. The laws as tests on the Rust store | Is the model the store, and are the laws proved? | About 250 lines of Rust source, 350 of tests, 250 of Lean, 40 of gate script |
| **C. Kernel** | `cj3-kernel`: parameters, V1–V9, the STF, the reward, atomic block application, the first half of B9 as K-7 is ruled, Q as K-8 is ruled. The 27-case table (K-5). The conservation test. Two D6 jobs made live | Does the kernel do what the Lean says, and nothing else? | About 750 lines of Rust source, 1,000 of tests, 250 of scripts and workflow |

About 3,600 lines in all, not counting generated files. If K-6 is admitted, part C gains
about 300 lines of Lean and 200 of Rust, or K-6 becomes a fourth part after C.

**Weighed.** *One pull request:* one review of about 3,600 lines across three questions,
and one D6 run that cannot say which part broke. *Two, with A and B together:* the Lean
proof would wait on the review of byte layouts, which it does not depend on.

**Evidence.** PR #21 (P-007) was one pull request: 20,619 added lines in 76 files by
GitHub's count, most of them generated vectors. The sizes above are predictions; the
calibration at the end of P-101 will grade them.

## 14. Predicted diff surface, risks, falsifier, confidence

**Predicted diff surface of the build**, if the proposals are ruled as they stand:
`crates/cj3-types/src/` (tags, the two commitments, their tests);
`crates/cj3-store/` (source, manifest, tests); `crates/cj3-kernel/` (source, manifest,
tests); `spec/Spec/Store.lean`, `spec/Spec.lean` (one import) and, for K-7, one comment
in `spec/Spec/Stf.lean`; `bench/p101-roots/` (the examples become vectors) and a small
new directory under `bench/` for the case list of K-5; `scripts/ci/` (two gate
handlers, the Lean gate, the source-policy gate); `.github/workflows/ci.yml`;
`docs/PROTOCOL_SPEC.md`, `docs/ENGINEERING_PLAN.md` (the diagram), `README.md`;
`Cargo.lock` for the two new edges inside the workspace, with no new package; and the
loop files. **Not predicted:** any change to a definition or theorem in `Tx.lean` or
`Stf.lean`; `spec/vectors/p005-draft.json`; any vector file of P-007; either
`deny.toml`; a new dependency; `cj3-consensus`, `cj3-net`, `cj3-rpc`, `cj3-node`.

**Top three risks.**

1. **The laws cannot be proved for the model in Lean 4.33 without Mathlib** at a cost
   that fits the packet, or the proof depends on an axiom beyond `propext` and
   `Quot.sound`. Control: part B is a pull request of its own, so a failure there stops
   before the kernel is built on it.
2. **A table entry builds the wrong situation and passes.** Control: the expression
   beside the values, a control change per entry that must flip the verdict, and Al's
   review of the table.
3. **The Rust commitments and the script agree on what was tested and differ on what was
   not**: deep trees, long shared prefixes, the largest counts. Control: the second
   implementation in the script, property tests in Rust over the same generators, and
   the vectors.

**Falsifier.** This approach is wrong if any of the three laws cannot be stated and
proved for the model without `sorry`; if any law fails on the Rust store in any alias
case; if the Rust commitments differ from the script's on any generated list or state;
if any of the 27 cases needs a value that the kernel cannot be given as an input; or if
a quality can reach the reward by any path but the kernel's own call to the checker.

**Confidence.** HIGH that the V-rules and the STF can be written to mirror the Lean
line for line. MED-HIGH on the two commitments. MED on the Lean proof of the laws,
which I have not attempted. MED that the list of decisions is complete: K-7 was found
while writing this, and another gap of its kind may be waiting.

## 15. Points for Al beyond the twelve decisions

1. **K-7 is a gap in the record, not only a design choice.** As written, neither §8 nor
   the Lean model checks `tx_root` against the body.
2. **"Prove" in §8** (K-4). The proposal proves the laws about a Lean model and ties the
   Rust store to it by tests and by construction. If the sentence means a proof about
   the Rust code itself, P-101 as proposed does not deliver it.
3. **The commitment of K-2 is less common than the alternatives.** A radix tree that
   records its splits is a textbook structure, and its hashed form is simple, but I do
   not know a deployed chain that uses exactly this one. The list tree of K-1 over
   sorted leaves is the conservative choice, at the cost section 5.7 gives.
4. **Nothing bounds the number of accounts.** That is P-9's subject, not this FRAME's.
5. **No time was measured** for recomputing the commitment with the portable SHA-256
   that launch 4.3b pins. It bears on K-2 only through how soon the durable store has to
   stop recomputing.
6. **The order of keys is an order on byte strings.** If the first Phase 2 FRAME rules
   that a 32-byte value is read as a little-endian number, "ascending by address" here
   will not be ascending by that number. It does not have to be; I name it so that it is
   not a surprise.
7. **Three public documents are cited from memory:** RFC 6962, CVE-2012-2459 and
   EIP-161. None was fetched in this session.

## 16. STOP

This FRAME is move 1 of P-101 and the end of this session's work on it. Moves 2 to 6
wait for Al's ruling. No kernel code is written, no tag is assigned, and no value is
filled.

## VERIFIED

- The worked examples and every digest in sections 4.6 and 5.6: written by
  `bench/p101-roots/derive.py`, which prints `P101_ROOTS=PASS files=2 self_checks=171`;
  `--check` passes on the committed files. Six of the digests were recomputed with a
  second tool (`xxd -r -p | sha256sum`) from the preimages in
  `bench/p101-roots/p101-roots.examples.txt`, and matched.
- The inputs of the examples are read from
  `bench/p007-vectors/encoding-v1.vectors.txt`, whose SHA-256 the script checks
  (`f5061cef…17d855`).
- The axioms the ratified theorems depend on: the `#print axioms` output quoted in
  section 7.1.
- What `Stf.lean`, `Tx.lean`, the specification and the manifests say, where sections
  7 to 12 cite them: read in this session.
- That the six proposed tags differ from each other and from the seven ruled: asserted
  by the script on every run.

## ASSUMED

- SHA-256 has no known collision. Safe: every preimage of §1.2 rests on it already.
- RFC 6962's split rule, CVE-2012-2459 and EIP-161 are as I remember them. Safe for the
  proposal, which is defined here in full and does not depend on them; they are context.
- A copy of the store per block is acceptable for an in-memory first implementation.
  Safe: it is not a consensus matter, and section 12 names the packet that replaces it.

## UNKNOWN

- Whether the three laws can be proved for the model without Mathlib. Resolved by part B.
- What recomputing the commitment costs in time. Resolved by a measurement in P-101.
- Whether K-6's Lean instantiation evaluates in reasonable time. Resolved by trying it.
- Whether any decision is missing from section 3. Resolved by Al's review, and by the
  build, which stops at the first one it meets.
