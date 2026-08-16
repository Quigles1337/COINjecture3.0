# P-005 Lean draft checkpoint — stopped on SI-004

This Lake project is the draft formal model authorized for P-005. Its governing
sources are Protocol Spec §§7–8 and the §14 vector case list. The source files are
not normative until Al resolves SI-004, the completed project passes its adversary
and CI gates, and Al then completes the required line-by-line review and merges PR #9.

The model deliberately leaves these surfaces abstract:

- strict transaction decoding and canonical byte encoding;
- signature verification and the signing-preimage construction;
- public-key-to-address derivation;
- `TX_MAX_BYTES`, `FEE_MIN`, and the chain network identifier;
- B-rule validation, reward/subsidy selection, and authenticated-state-root logic.

SI-001, SI-002, and SI-003 remain unresolved in
`loop/reports/SPEC-ISSUES.md`. They are not instantiated by this project. In
particular, the exported `input_bytes` values are symbolic expressions, not a
transaction wire encoding or SIS byte convention.

The partial Tx/STF targets currently compile on the pinned local toolchain:

```powershell
cd spec
lake build Spec.Tx Spec.Stf
```

The full Lake project and vector executable are **not** verified at this checkpoint,
and no generated JSON artifact is committed. The draft vector source labels symbolic
cases as non-canonical interfaces; it does not define wire bytes. See SI-004 in
`loop/reports/SPEC-ISSUES.md` for the subsidy/reward conflict that stopped the build.

P-101 remains responsible for the Rust kernel consumer after Gate G0 and after human
review and merge of P-005.
