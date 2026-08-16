# Independent SHAKE-256 expansion vector

The first six matrix field elements for an all-zero 32-byte seed and `q = 17` were
computed independently with Python's standard-library `hashlib`, not the RustCrypto
implementation used by `cj3-classes`:

```powershell
python -c "import hashlib; seed=bytes(32); q=17; data=hashlib.shake_256(seed).digest(256); limit=((1<<32)//q)*q; words=[int.from_bytes(data[i:i+4],'little') for i in range(0,len(data),4)]; vals=[w%q for w in words if w<limit][:6]; print(vals); print(words[:8]); print(limit)"
```

Recorded output on 2026-08-15 EDT:

```text
[4, 12, 6, 7, 3, 15]
[2189203445, 1667912835, 499334002, 1326586150, 1481038609, 1949709156, 1461188637, 1647357397]
4294967295
```

Words are interpreted little-endian. The acceptance limit is
`floor(2^32 / q) * q`; therefore `u32::MAX` is the only rejected word for `q=17`.
The Rust unit test locks the six-element output and separately exercises both the
rejected tail and a modulus that divides `2^32`.
