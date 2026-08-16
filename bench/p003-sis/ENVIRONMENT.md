# P-003 measurement environment

Measurements were taken across 2026-08-15–16 EDT from the canonical checkout, with no
other benchmark process intentionally scheduled by the packet.

- OS: Microsoft Windows 11 Home 10.0.26200, build 26200.
- CPU: AMD Ryzen AI 9 HX 370 with Radeon 890M, 12 physical cores / 24 logical CPUs.
- Physical memory: 33,363,439,616 bytes.
- Rust: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, LLVM 22.1.6,
  `x86_64-pc-windows-msvc`; Cargo 1.97.1.
- Rust profile: Cargo `release` defaults from this workspace (no custom profile).
- Docker: client/server 29.5.2; Docker Desktop 4.76.0; Linux engine on WSL2 kernel
  5.15.167.4.
- Estimator runtime image manifest (used directly; no local Dockerfile or dependency
  rebuild is part of the recorded run):
  `sagemath/sagemath:9.5@sha256:ec32d9752b3a11c628103ca6802db890b63cbe9bb480cfea02de09656ecc84a2`.
- Estimator checkout: exact commit
  `3e48ef421ec256afddb3e7d2249a77eab6e9ba12`, required clean before execution, with
  `estimator/sis_lattice.py` SHA-256
  `d68ec5d0f471cf4904126211d8b2579186fa6dce645ac7339e95bd621a505be1`.
- Estimator container controls: checkout and runner mounted read-only, root filesystem
  read-only, temporary storage limited to a 64 MiB `tmpfs`, and networking disabled.

Elapsed times are wall-clock values from `System.Diagnostics.Stopwatch` or Rust
`Instant`. They are not cycle counts, constant-time claims, cross-machine guarantees,
or a substitute for sustained-load profiling.
