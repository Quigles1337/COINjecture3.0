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
- Estimator image ID after the recorded rebuild:
  `sha256:097fa06e1c7be40ac9c1081a00a99c293d0fef59125cbedbbf065ebf6e36c3d4`.
- Estimator base image manifest:
  `sagemath/sagemath:9.5@sha256:ec32d9752b3a11c628103ca6802db890b63cbe9bb480cfea02de09656ecc84a2`.

Elapsed times are wall-clock values from `System.Diagnostics.Stopwatch` or Rust
`Instant`. They are not cycle counts, constant-time claims, cross-machine guarantees,
or a substitute for sustained-load profiling.
