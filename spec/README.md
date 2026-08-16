# P-005 Lean draft — SI-004 Model 4 ratified

This Lake project is the draft formal model authorized for P-005. Its governing
sources are Protocol Spec §§7–8, §11's ratified Model 4 formula, and the §14 vector
case list. The source files remain draft until the completed project passes its
adversary and CI gates and Al completes the required line-by-line review and merges
PR #9.

The model deliberately leaves these surfaces abstract:

- strict transaction decoding and canonical byte encoding;
- signature verification and the signing-preimage construction;
- public-key-to-address derivation;
- `TX_MAX_BYTES`, `FEE_MIN`, and the chain network identifier;
- B-rule validation, the subsidy schedule, the P-7 `R_MAX` value/curve shaping, and
  authenticated-state-root logic.

The reward is not an arbitrary interface value. `Spec/Stf.lean` defines the ratified
floor-divided Model 4 reward, proves its mathematical numerator fits u128, proves the
credited result fits u64, and proves `reward ≤ subsidy` without taking that inequality
as an assumption. The STF credit and conservation target both use that exact function.

SI-001, SI-002, and SI-003 remain unresolved in
`loop/reports/SPEC-ISSUES.md`. They are not instantiated by this project. In
particular, the exported `input_bytes` values are symbolic expressions, not a
transaction wire encoding or SIS byte convention.

Build the complete pinned project and print the deterministic symbolic vectors with:

```powershell
cd spec
lake build
lake exe vectors
```

The committed artifact is `vectors/p005-draft.json`. Regenerate it with:

```powershell
lake exe vectors -- vectors/p005-draft.json
```

The draft vector source labels every `input_bytes` value as a noncanonical abstract
interface. It does not define transaction wire bytes, populate a protocol TBD, or
instantiate a subsidy, fee, network, `R_MAX`, or μ-balance value.

P-101 remains responsible for the Rust kernel consumer after Gate G0 and after human
review and merge of P-005.
