# P-006 offline difficulty simulation

**NON-NORMATIVE RESEARCH FIXTURE — HUMAN/G0 RATIFICATION REQUIRED**

This directory tests a hypothetical two-knob controller in normalized time. It is not
chain code and does not define Protocol Spec P-1, P-2, P-11, a hash-target encoding,
an SIS size, or a Phase-2 retarget function.

The model keeps the two observations structurally separate:

- the hypothetical hash controller sees only simulated inter-block intervals; and
- the hypothetical size controller sees only a checker-derived quality margin.

The adversarial quality scenarios do not inject a fake score. They model a miner who
possesses multiple valid solutions and publishes one with less quality margin. Every
published value is still assumed to be recomputed by the checker. This distinction is
important: authentic measurements can remain strategically selected measurements.

## Model and limitations

One target interval is the dimensionless value `1.0`. Each simulated block consists
of an exponential solution-acquisition delay plus an exponential hash-race delay.
The relative contribution of those stages and the elasticity between solve power and
checked quality are swept as sensitivities. The model uses no miner-reported time,
power, quality field, or reward input.

This is deliberately not a security proof or a calibrated production-miner model.
P-003 did not solve its provisional P-4 tuple, P-002 did not choose a production VDF
delay, and no propagation distribution is available. Those gaps make an absolute
block time in seconds underdetermined. The simulator therefore refuses to manufacture
one.

`config.json` contains all candidates, scenarios, seeds, sensitivity axes, bounds,
and predeclared acceptance thresholds. The configuration is capped at 1,000,000 bytes
and the aggregate matrix at 20,000,000 simulated blocks before a run begins. Scenario
identifiers are restricted before Markdown generation. The 36 candidate coordinates
are proposal inputs only.

## Reproduce

The sealed evidence used CPython 3.11.9. From this directory with that runtime:

```powershell
python simulate.py check
python -m unittest -v test_simulate.py
```

To deliberately regenerate the committed artifacts after reviewing a model change:

```powershell
python simulate.py generate
python simulate.py check
```

`check` regenerates `evidence/results.json` and
`evidence/STABILITY-ENVELOPE.md` in a temporary directory and compares exact bytes.
The output embeds SHA-256 hashes of the configuration and simulator source.

## Evidence contract

- `evidence/results.json` is the complete stable-schema machine result; no failed
  coordinate is omitted.
- `evidence/STABILITY-ENVELOPE.md` is generated from that same in-memory result.
- `evidence/ENVIRONMENT.md` records the host/runtime and final artifact hashes.
- Passing an offline envelope is necessary proposal evidence only. Ratification,
  Protocol Spec edits, and Phase-2 implementation remain HUMAN/G0 work.
