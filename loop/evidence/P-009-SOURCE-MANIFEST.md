# P-009 source evidence manifest

**Captured:** 2026-08-16
**Scope:** byte-for-byte durability ingest only; no document finding was interpreted
before this commit.

| evidence source | supplied path | durable repository path | bytes | source SHA-256 | durable-copy SHA-256 |
|---|---|---|---:|---|---|
| Third-party COINjecture 2.0 security audit | `C:\Users\LEET\Downloads\COINjecture-2.0-Security-Audit.docx` | `loop/evidence/COINjecture-2.0-Security-Audit.docx` | 31,811 | `D6A9100E9E69A9677EC0A562C486FFF8876839CC8378CAE1FA157326E22B7A7F` | `D6A9100E9E69A9677EC0A562C486FFF8876839CC8378CAE1FA157326E22B7A7F` |
| Optional DARQ Lean audit | `C:\Users\LEET\Downloads\DARQ-LV-001_COINjecture_v2.6_Lean_Audit.pdf` | `loop/evidence/DARQ-LV-001_COINjecture_v2.6_Lean_Audit.pdf` | 27,414 | `4E20AA8B287C70F8D0871D9D53FF55BE251FBDDD8113CB2786CD733FFD9C9C30` | `4E20AA8B287C70F8D0871D9D53FF55BE251FBDDD8113CB2786CD733FFD9C9C30` |

PowerShell `Get-FileHash -Algorithm SHA256` was run independently on each supplied
source and each durable copy. Equal sizes and hashes establish exact copies. The
Downloads files remain untouched.

Git raw-blob verification also matched before publication:

- DOCX working-file/index blob: `54ac9a3a41e423d3e062a1c5f614a1fea742890f`;
- PDF working-file/index blob: `04dbf8eb2a00ef945c5195e82ad8161c25ab050b`.

The root `.gitattributes` marks `*.docx` and `*.pdf` as binary so checkout line-ending
normalization cannot mutate either durable copy on another platform.

The Codex security-scan report source is outstanding exactly as recorded in the
2026-08-16 LEDGER overlay. P-009 will test whether its findings are already fully
represented by exact committed COINjecture 2.0 remediation pointers rather than wait
for a file whose path was not supplied.
