# P-006 simulation environment and evidence seal

**NON-NORMATIVE RESEARCH FIXTURE — HUMAN/G0 RATIFICATION REQUIRED**

- Captured: 2026-08-16 EDT.
- Delegated CJ3 base revision:
  `44ae3ae8807bc7ae594494870bffa96da6ea0ca1`.
- OS: Microsoft Windows 11 Home 10.0.26200, build 26200, 64-bit.
- Processor: AMD Ryzen AI 9 HX 370 with Radeon 890M, 12 physical cores / 24
  logical processors.
- Runtime: CPython 3.11.9, MSC v.1938, 64-bit AMD64.
- Dependencies: Python standard library only; no network, container, native extension,
  external process, wall-clock timing, or host entropy is consumed by the simulator.
- Experiment size: 36 controller coordinates × 9 scenarios × 24 fixed
  seed/sensitivity runs × 768 simulated blocks = 5,971,968 simulated blocks.

## SHA-256 seal

- `config.json`:
  `FAADF793F6B196C66F48AA2C5A5E2CCDF12A8BAEC2517FFB6C4A0CF23F13F51D`
- `simulate.py`:
  `C90157F08DA2A93080177862B7A5FA1B5D9D0DA2F36159CED47801B8A5DB12A5`
- `test_simulate.py`:
  `1067F5CCC7FDDA505AE7BD7EA0F97C0D3DE07C24F394EC0EEF7C39F7A5B95630`
- `evidence/results.json`:
  `32ABD54939FE6CB4F3D1912A60CF1A2D7B5176E27C913BBC47255FF0F477698B`
- `evidence/STABILITY-ENVELOPE.md`:
  `BF4C71C4A2BAE7BB7EC27FEFDA4CF84BDDC0CA46086D018E3ABF98AB9DE59BC3`

`python simulate.py check` regenerated both evidence outputs in an isolated temporary
directory and compared exact bytes on this runtime. The simulator embeds the exact
configuration and simulator hashes in `results.json`; the generated Markdown is
derived from the same in-memory result. This seal establishes reproducibility of the
recorded experiment, not portability of floating-point pseudo-random sequences across
unrecorded Python implementations or proof that the model represents production SIS
solver behavior.
