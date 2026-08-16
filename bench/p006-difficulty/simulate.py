#!/usr/bin/env python3
"""Deterministic, non-normative P-006 two-knob difficulty experiment.

This is offline analysis tooling. It does not define a consensus retarget, a block
time, an SIS size, or a protocol parameter. All values are normalized research
coordinates that require HUMAN/G0 ratification before any protocol use.
"""

from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import math
import random
import re
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable


NON_NORMATIVE = "NON-NORMATIVE RESEARCH FIXTURE — HUMAN/G0 RATIFICATION REQUIRED"
SCHEMA = "cj3-p006-nonnormative-simulation-v1"
ROOT = Path(__file__).resolve().parent
DEFAULT_CONFIG = ROOT / "config.json"
DEFAULT_EVIDENCE = ROOT / "evidence"
NUMERIC_STATE_MIN = 2.0**-24
NUMERIC_STATE_MAX = 2.0**24
MAX_CONFIG_BYTES = 1_000_000
MAX_SIMULATED_BLOCKS = 20_000_000
SCENARIO_ID = re.compile(r"^[a-z0-9_]+$")


class ConfigError(ValueError):
    """Raised when an input exceeds the bounded experiment contract."""


@dataclass(frozen=True)
class Candidate:
    """One hypothetical controller coordinate, never a protocol selection."""

    hash_ema_blocks: int
    hash_step_limit: float
    size_window_blocks: int
    size_step_limit: float

    @property
    def candidate_id(self) -> str:
        hash_step = str(self.hash_step_limit).replace(".", "p")
        size_step = str(self.size_step_limit).replace(".", "p")
        return (
            f"h{self.hash_ema_blocks}-hc{hash_step}-"
            f"s{self.size_window_blocks}-sc{size_step}"
        )


@dataclass(frozen=True)
class RunMetrics:
    """Bounded metrics from one fixed-seed sensitivity run."""

    tail_interval_mean: float
    tail_hash_state_mean: float
    tail_size_state_mean: float
    expected_hash_state: float | None
    expected_size_state: float | None
    recovery_blocks: int | None
    max_abs_hash_log2: float
    max_abs_size_log2: float
    adversarial_blocks: int
    numeric_saturation_count: int


def require_number(value: Any, name: str, low: float, high: float) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ConfigError(f"{name} must be numeric")
    result = float(value)
    if not math.isfinite(result) or result < low or result > high:
        raise ConfigError(f"{name} must be finite and within [{low}, {high}]")
    return result


def require_int(value: Any, name: str, low: int, high: int) -> int:
    if isinstance(value, bool) or not isinstance(value, int):
        raise ConfigError(f"{name} must be an integer")
    if value < low or value > high:
        raise ConfigError(f"{name} must be within [{low}, {high}]")
    return value


def load_config(path: Path) -> dict[str, Any]:
    try:
        if path.stat().st_size > MAX_CONFIG_BYTES:
            raise ConfigError(
                f"configuration exceeds the {MAX_CONFIG_BYTES}-byte input bound"
            )
        config = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ConfigError(f"cannot load {path}: {error}") from error
    if not isinstance(config, dict):
        raise ConfigError("configuration root must be an object")
    validate_config(config)
    return config


def validate_config(config: dict[str, Any]) -> None:
    if config.get("schema") != SCHEMA:
        raise ConfigError(f"configuration schema must be exactly {SCHEMA}")
    if config.get("normative_status") != NON_NORMATIVE:
        raise ConfigError("exact non-normative status marker is required")
    require_number(config.get("normalized_target_interval"), "target", 1.0, 1.0)
    blocks = require_int(config.get("blocks_per_run"), "blocks_per_run", 128, 10_000)
    shock = require_int(config.get("shock_block"), "shock_block", 32, blocks - 32)
    require_int(config.get("tail_blocks"), "tail_blocks", 32, blocks - shock)

    seeds = config.get("fixed_seeds")
    if not isinstance(seeds, list) or not 1 <= len(seeds) <= 32:
        raise ConfigError("fixed_seeds must contain 1..32 entries")
    normalized_seeds = [require_int(seed, "seed", 0, 2**63 - 1) for seed in seeds]
    if len(set(normalized_seeds)) != len(normalized_seeds):
        raise ConfigError("fixed_seeds must be unique")

    for key in ("solve_fraction_sensitivities", "quality_elasticity_sensitivities"):
        values = config.get(key)
        if not isinstance(values, list) or not 1 <= len(values) <= 8:
            raise ConfigError(f"{key} must contain 1..8 entries")
        for value in values:
            require_number(value, key, 0.05, 4.0)
    for value in config["solve_fraction_sensitivities"]:
        if float(value) >= 0.95:
            raise ConfigError("solve fractions must leave a positive hash-race share")

    quality_headroom = require_number(
        config.get("quality_headroom"), "quality_headroom", 0.01, 10.0
    )
    require_number(config.get("quality_noise_sigma"), "quality_noise_sigma", 0.0, 1.0)
    quality_target = require_number(
        config.get("quality_target"), "quality_target", 1.01, 20.0
    )
    if not math.isclose(
        quality_target, 1.0 + quality_headroom, rel_tol=0.0, abs_tol=1e-12
    ):
        raise ConfigError(
            "quality_target must equal 1 + quality_headroom so the stated equilibrium is valid"
        )
    require_number(config.get("hash_gain"), "hash_gain", 0.001, 1.0)
    require_number(config.get("size_gain"), "size_gain", 0.001, 2.0)

    grid = config.get("candidate_grid")
    if not isinstance(grid, dict):
        raise ConfigError("candidate_grid must be an object")
    grid_contract = {
        "hash_ema_blocks": (4, 1_024, True),
        "hash_step_limit": (1.001, 4.0, False),
        "size_window_blocks": (8, 4_096, True),
        "size_step_limit": (1.001, 4.0, False),
    }
    candidate_count = 1
    for key, (low, high, integral) in grid_contract.items():
        values = grid.get(key)
        if not isinstance(values, list) or not 1 <= len(values) <= 16:
            raise ConfigError(f"candidate_grid.{key} must contain 1..16 entries")
        candidate_count *= len(values)
        for value in values:
            if integral:
                require_int(value, f"candidate_grid.{key}", int(low), int(high))
            else:
                require_number(value, f"candidate_grid.{key}", float(low), float(high))
    if candidate_count > 256:
        raise ConfigError("candidate grid is bounded to 256 coordinates")

    envelope = config.get("envelope")
    if not isinstance(envelope, dict):
        raise ConfigError("envelope must be an object")
    require_number(
        envelope.get("tail_interval_relative_error_max"),
        "tail interval error",
        0.01,
        1.0,
    )
    require_number(
        envelope.get("tail_hash_state_relative_error_max"),
        "tail hash-state error",
        0.01,
        4.0,
    )
    require_number(
        envelope.get("tail_size_state_relative_error_max"),
        "tail size-state error",
        0.01,
        4.0,
    )
    require_number(
        envelope.get("recovery_band_relative_error"),
        "recovery band",
        0.01,
        1.0,
    )
    require_int(
        envelope.get("recovery_hold_blocks"),
        "recovery hold",
        4,
        blocks - shock,
    )
    require_int(
        envelope.get("recovery_blocks_max"),
        "recovery maximum",
        1,
        blocks - shock,
    )
    require_number(
        envelope.get("state_excursion_log2_max"),
        "state excursion",
        1.0,
        32.0,
    )
    require_number(
        envelope.get("adversarial_size_retention_min"),
        "adversarial size retention",
        0.01,
        1.0,
    )

    scenarios = config.get("scenarios")
    if not isinstance(scenarios, list) or not 1 <= len(scenarios) <= 32:
        raise ConfigError("scenarios must contain 1..32 entries")
    identifiers: set[str] = set()
    for scenario in scenarios:
        if not isinstance(scenario, dict):
            raise ConfigError("every scenario must be an object")
        identifier = scenario.get("id")
        if (
            not isinstance(identifier, str)
            or SCENARIO_ID.fullmatch(identifier) is None
            or identifier in identifiers
        ):
            raise ConfigError(
                "scenario ids must be unique lowercase ASCII letters/digits/underscores"
            )
        identifiers.add(identifier)
        schedule_class = scenario.get("schedule_class")
        if schedule_class not in {
            "honest_baseline",
            "honest_step",
            "adversarial_power_schedule",
            "adversarial_quality_selection",
        }:
            raise ConfigError("scenario.schedule_class is missing or unsupported")
        kind = scenario.get("kind")
        if kind == "constant":
            require_number(scenario.get("hash_power"), "hash_power", 0.05, 20.0)
            require_number(scenario.get("solve_power"), "solve_power", 0.05, 20.0)
        elif kind == "step":
            for key in ("hash_before", "hash_after", "solve_before", "solve_after"):
                require_number(scenario.get(key), key, 0.05, 20.0)
        elif kind == "periodic_opposed":
            require_int(scenario.get("period_blocks"), "period_blocks", 8, blocks)
            for key in ("hash_high", "hash_low", "solve_high", "solve_low"):
                require_number(scenario.get(key), key, 0.05, 20.0)
        else:
            raise ConfigError(f"unsupported scenario kind: {kind!r}")
        require_number(
            scenario.get("adversary_share_after_shock", 0.0),
            "adversary share",
            0.0,
            1.0,
        )
        require_number(
            scenario.get("quality_margin_retained", 1.0),
            "quality margin retained",
            0.0,
            1.0,
        )
        if not isinstance(scenario.get("honest"), bool):
            raise ConfigError("scenario.honest must be boolean")
    work_units = (
        candidate_count
        * len(scenarios)
        * len(seeds)
        * len(config["solve_fraction_sensitivities"])
        * len(config["quality_elasticity_sensitivities"])
        * blocks
    )
    if work_units > MAX_SIMULATED_BLOCKS:
        raise ConfigError(
            f"experiment requests {work_units} simulated blocks; "
            f"maximum is {MAX_SIMULATED_BLOCKS}"
        )


def candidates(config: dict[str, Any]) -> Iterable[Candidate]:
    grid = config["candidate_grid"]
    for values in itertools.product(
        grid["hash_ema_blocks"],
        grid["hash_step_limit"],
        grid["size_window_blocks"],
        grid["size_step_limit"],
    ):
        yield Candidate(
            hash_ema_blocks=int(values[0]),
            hash_step_limit=float(values[1]),
            size_window_blocks=int(values[2]),
            size_step_limit=float(values[3]),
        )


def powers_at(
    scenario: dict[str, Any], height: int, shock_block: int
) -> tuple[float, float]:
    kind = scenario["kind"]
    if kind == "constant":
        return float(scenario["hash_power"]), float(scenario["solve_power"])
    if kind == "step":
        suffix = "before" if height < shock_block else "after"
        return float(scenario[f"hash_{suffix}"]), float(scenario[f"solve_{suffix}"])
    if kind == "periodic_opposed":
        period = int(scenario["period_blocks"])
        high_phase = (height // period) % 2 == 0
        if high_phase:
            return float(scenario["hash_high"]), float(scenario["solve_low"])
        return float(scenario["hash_low"]), float(scenario["solve_high"])
    raise AssertionError(f"validated scenario kind became unknown: {kind}")


def tail_expected_states(
    scenario: dict[str, Any], blocks: int, shock_block: int
) -> tuple[float | None, float | None]:
    if scenario["kind"] == "periodic_opposed":
        return None, None
    return powers_at(scenario, blocks - 1, shock_block)


def bounded_multiplier(raw: float, limit: float) -> float:
    return min(limit, max(1.0 / limit, raw))


def select_published_quality(
    checked_quality: float, retained_margin: float, adversarial_winner: bool
) -> float:
    """Model valid-solution selection without creating a forged quality value."""

    if not math.isfinite(checked_quality) or checked_quality < 1.0:
        raise ArithmeticError("a simulated valid solution cannot have quality below 1")
    if not math.isfinite(retained_margin) or not 0.0 <= retained_margin <= 1.0:
        raise ArithmeticError("retained quality margin must stay within [0, 1]")
    if not adversarial_winner:
        return checked_quality
    return 1.0 + (checked_quality - 1.0) * retained_margin


def relative_error(observed: float, expected: float) -> float:
    return abs(observed / expected - 1.0)


def first_sustained_recovery(
    intervals: list[float],
    shock_block: int,
    band: float,
    hold: int,
) -> int | None:
    rolling_width = 32
    if shock_block + rolling_width + hold > len(intervals):
        return None
    rolling: list[float] = []
    running = sum(intervals[shock_block : shock_block + rolling_width])
    rolling.append(running / rolling_width)
    for start in range(shock_block + 1, len(intervals) - rolling_width + 1):
        running += intervals[start + rolling_width - 1] - intervals[start - 1]
        rolling.append(running / rolling_width)
    for offset in range(0, len(rolling) - hold + 1):
        if all(abs(value - 1.0) <= band for value in rolling[offset : offset + hold]):
            return offset + rolling_width + hold - 1
    return None


def simulate_one(
    config: dict[str, Any],
    candidate: Candidate,
    scenario: dict[str, Any],
    seed: int,
    solve_fraction: float,
    elasticity: float,
) -> RunMetrics:
    rng = random.Random(seed)
    blocks = int(config["blocks_per_run"])
    shock = int(config["shock_block"])
    target = float(config["normalized_target_interval"])
    hash_gain = float(config["hash_gain"])
    size_gain = float(config["size_gain"])
    headroom = float(config["quality_headroom"])
    quality_target = float(config["quality_target"])
    noise_sigma = float(config["quality_noise_sigma"])
    noise_mean_correction = -(noise_sigma**2) / 2.0

    hash_state = 1.0
    size_state = 1.0
    interval_ema = target
    ema_alpha = 2.0 / (candidate.hash_ema_blocks + 1.0)
    quality_window: list[float] = []
    intervals: list[float] = []
    hash_states: list[float] = []
    size_states: list[float] = []
    adversarial_blocks = 0
    numeric_saturation_count = 0

    for height in range(blocks):
        hash_power, solve_power = powers_at(scenario, height, shock)
        solve_mean = solve_fraction * size_state / solve_power
        hash_mean = (1.0 - solve_fraction) * hash_state / hash_power
        solve_delay = rng.expovariate(1.0 / solve_mean)
        hash_delay = rng.expovariate(1.0 / hash_mean)
        interval = solve_delay + hash_delay

        quality_noise = math.exp(rng.gauss(noise_mean_correction, noise_sigma))
        intrinsic_margin = headroom * ((solve_power / size_state) ** elasticity)
        checked_quality = max(1.0, 1.0 + intrinsic_margin * quality_noise)
        adversary_share = (
            float(scenario.get("adversary_share_after_shock", 0.0))
            if height >= shock
            else 0.0
        )
        retained = float(scenario.get("quality_margin_retained", 1.0))
        adversarial_winner = adversary_share > 0.0 and rng.random() < adversary_share
        if adversarial_winner:
            adversarial_blocks += 1
        checked_quality = select_published_quality(
            checked_quality, retained, adversarial_winner
        )

        intervals.append(interval)
        hash_states.append(hash_state)
        size_states.append(size_state)
        quality_window.append(checked_quality)

        interval_ema = (1.0 - ema_alpha) * interval_ema + ema_alpha * interval
        requested_hash_multiplier = (target / interval_ema) ** hash_gain
        hash_state *= bounded_multiplier(
            requested_hash_multiplier, candidate.hash_step_limit
        )

        if (height + 1) % candidate.size_window_blocks == 0:
            mean_quality = sum(quality_window) / len(quality_window)
            requested_size_multiplier = (mean_quality / quality_target) ** size_gain
            size_state *= bounded_multiplier(
                requested_size_multiplier, candidate.size_step_limit
            )
            quality_window.clear()

        if not (math.isfinite(hash_state) and math.isfinite(size_state)):
            raise ArithmeticError(
                "controller state became non-finite: "
                f"candidate={candidate.candidate_id} scenario={scenario['id']} "
                f"seed={seed} solve_fraction={solve_fraction} elasticity={elasticity} "
                f"height={height} hash_state={hash_state} size_state={size_state}"
            )
        if not (
            NUMERIC_STATE_MIN <= hash_state <= NUMERIC_STATE_MAX
            and NUMERIC_STATE_MIN <= size_state <= NUMERIC_STATE_MAX
        ):
            numeric_saturation_count += 1
            hash_state = min(NUMERIC_STATE_MAX, max(NUMERIC_STATE_MIN, hash_state))
            size_state = min(NUMERIC_STATE_MAX, max(NUMERIC_STATE_MIN, size_state))

    tail = int(config["tail_blocks"])
    expected_hash, expected_size = tail_expected_states(scenario, blocks, shock)
    envelope = config["envelope"]
    recovery = None
    if scenario["kind"] == "step":
        recovery = first_sustained_recovery(
            intervals,
            shock,
            float(envelope["recovery_band_relative_error"]),
            int(envelope["recovery_hold_blocks"]),
        )
    return RunMetrics(
        tail_interval_mean=sum(intervals[-tail:]) / tail,
        tail_hash_state_mean=sum(hash_states[-tail:]) / tail,
        tail_size_state_mean=sum(size_states[-tail:]) / tail,
        expected_hash_state=expected_hash,
        expected_size_state=expected_size,
        recovery_blocks=recovery,
        max_abs_hash_log2=max(abs(math.log2(value)) for value in hash_states),
        max_abs_size_log2=max(abs(math.log2(value)) for value in size_states),
        adversarial_blocks=adversarial_blocks,
        numeric_saturation_count=numeric_saturation_count,
    )


def rounded(value: float) -> float:
    return float(f"{value:.8f}")


def summarize_scenario(
    config: dict[str, Any], scenario: dict[str, Any], runs: list[RunMetrics]
) -> dict[str, Any]:
    interval_errors = [relative_error(run.tail_interval_mean, 1.0) for run in runs]
    hash_errors = [
        relative_error(run.tail_hash_state_mean, run.expected_hash_state)
        for run in runs
        if run.expected_hash_state is not None
    ]
    size_errors = [
        relative_error(run.tail_size_state_mean, run.expected_size_state)
        for run in runs
        if run.expected_size_state is not None
    ]
    size_retentions = [
        run.tail_size_state_mean / run.expected_size_state
        for run in runs
        if run.expected_size_state is not None
    ]
    recoveries = [run.recovery_blocks for run in runs if run.recovery_blocks is not None]
    missing_recoveries = sum(
        1
        for run in runs
        if scenario["kind"] == "step" and run.recovery_blocks is None
    )
    return {
        "scenario": scenario["id"],
        "schedule_class": scenario["schedule_class"],
        "honest": scenario["honest"],
        "run_count": len(runs),
        "tail_interval_mean_of_means": rounded(
            sum(run.tail_interval_mean for run in runs) / len(runs)
        ),
        "tail_interval_relative_error_worst": rounded(max(interval_errors)),
        "tail_hash_state_relative_error_worst": (
            rounded(max(hash_errors)) if hash_errors else None
        ),
        "tail_size_state_relative_error_worst": (
            rounded(max(size_errors)) if size_errors else None
        ),
        "tail_size_retention_min": (
            rounded(min(size_retentions)) if size_retentions else None
        ),
        "recovery_blocks_worst": max(recoveries) if recoveries else None,
        "recovery_missing_runs": missing_recoveries,
        "state_excursion_log2_worst": rounded(
            max(
                max(run.max_abs_hash_log2, run.max_abs_size_log2)
                for run in runs
            )
        ),
        "adversarial_blocks_total": sum(run.adversarial_blocks for run in runs),
        "numeric_saturation_runs": sum(
            1 for run in runs if run.numeric_saturation_count > 0
        ),
        "numeric_saturation_events": sum(
            run.numeric_saturation_count for run in runs
        ),
    }


def classify_candidate(
    config: dict[str, Any], scenario_summaries: list[dict[str, Any]]
) -> tuple[bool, bool, list[str]]:
    honest_pass = True
    adversarial_pass = True
    failures: list[str] = []
    for summary in scenario_summaries:
        scenario_id = summary["scenario"]
        base_ok = scenario_passes(config, summary, require_retention=False)
        summary["base_envelope_pass"] = base_ok
        if summary["honest"]:
            if not base_ok:
                honest_pass = False
                failures.append(f"honest-envelope:{scenario_id}")
        else:
            attack_ok = scenario_passes(config, summary, require_retention=True)
            summary["attack_envelope_pass"] = attack_ok
            if not attack_ok:
                adversarial_pass = False
                failures.append(f"adversarial-envelope:{scenario_id}")
    return honest_pass, honest_pass and adversarial_pass, failures


def scenario_passes(
    config: dict[str, Any],
    summary: dict[str, Any],
    *,
    require_retention: bool,
) -> bool:
    envelope = config["envelope"]
    interval_ok = (
        summary["tail_interval_relative_error_worst"]
        <= envelope["tail_interval_relative_error_max"]
    )
    excursion_ok = (
        summary["state_excursion_log2_worst"] <= envelope["state_excursion_log2_max"]
    )
    hash_ok = (
        summary["tail_hash_state_relative_error_worst"] is None
        or summary["tail_hash_state_relative_error_worst"]
        <= envelope["tail_hash_state_relative_error_max"]
    )
    size_ok = (
        summary["tail_size_state_relative_error_worst"] is None
        or summary["tail_size_state_relative_error_worst"]
        <= envelope["tail_size_state_relative_error_max"]
    )
    recovery_ok = summary["recovery_missing_runs"] == 0 and (
        summary["recovery_blocks_worst"] is None
        or summary["recovery_blocks_worst"] <= envelope["recovery_blocks_max"]
    )
    retention_ok = not require_retention or (
        summary["tail_size_retention_min"] is not None
        and summary["tail_size_retention_min"]
        >= envelope["adversarial_size_retention_min"]
    )
    return (
        interval_ok
        and excursion_ok
        and hash_ok
        and size_ok
        and recovery_ok
        and retention_ok
        and summary["numeric_saturation_runs"] == 0
    )


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest().upper()


def run_matrix(config: dict[str, Any], config_path: Path) -> dict[str, Any]:
    candidate_rows: list[dict[str, Any]] = []
    for candidate in candidates(config):
        scenario_summaries: list[dict[str, Any]] = []
        for scenario in config["scenarios"]:
            runs = [
                simulate_one(
                    config,
                    candidate,
                    scenario,
                    seed,
                    float(solve_fraction),
                    float(elasticity),
                )
                for seed, solve_fraction, elasticity in itertools.product(
                    config["fixed_seeds"],
                    config["solve_fraction_sensitivities"],
                    config["quality_elasticity_sensitivities"],
                )
            ]
            scenario_summaries.append(summarize_scenario(config, scenario, runs))
        honest_pass, adversarial_pass, failures = classify_candidate(
            config, scenario_summaries
        )
        candidate_rows.append(
            {
                "candidate_id": candidate.candidate_id,
                "coordinates": {
                    "hash_ema_blocks": candidate.hash_ema_blocks,
                    "hash_step_limit": candidate.hash_step_limit,
                    "size_window_blocks": candidate.size_window_blocks,
                    "size_step_limit": candidate.size_step_limit,
                },
                "honest_envelope_pass": honest_pass,
                "adversarial_envelope_pass": adversarial_pass,
                "failures": failures,
                "scenarios": scenario_summaries,
            }
        )

    honest_ids = [
        row["candidate_id"] for row in candidate_rows if row["honest_envelope_pass"]
    ]
    adversarial_ids = [
        row["candidate_id"]
        for row in candidate_rows
        if row["adversarial_envelope_pass"]
    ]
    hash_only_ids = []
    hash_only_scenarios = {
        "stationary_honest",
        "hash_step_up_4x",
        "hash_step_down_4x",
    }
    for row in candidate_rows:
        selected = [
            scenario
            for scenario in row["scenarios"]
            if scenario["scenario"] in hash_only_scenarios
        ]
        if len(selected) == len(hash_only_scenarios) and all(
            scenario["base_envelope_pass"] for scenario in selected
        ):
            hash_only_ids.append(row["candidate_id"])
    hash_only_unique_settings = sorted(
        {
            (
                int(row["coordinates"]["hash_ema_blocks"]),
                float(row["coordinates"]["hash_step_limit"]),
            )
            for row in candidate_rows
            if row["candidate_id"] in hash_only_ids
        }
    )
    all_unique_hash_settings = {
        (
            int(row["coordinates"]["hash_ema_blocks"]),
            float(row["coordinates"]["hash_step_limit"]),
        )
        for row in candidate_rows
    }
    scenario_pass_counts = {
        scenario["id"]: sum(
            1
            for row in candidate_rows
            for candidate_scenario in row["scenarios"]
            if candidate_scenario["scenario"] == scenario["id"]
            and (
                candidate_scenario["base_envelope_pass"]
                if scenario["honest"]
                else candidate_scenario["attack_envelope_pass"]
            )
        )
        for scenario in config["scenarios"]
    }
    scenario_classes = {
        scenario["id"]: scenario["schedule_class"]
        for scenario in config["scenarios"]
    }
    solution_reuse_boundary = [
        {
            "solve_fraction": rounded(float(solve_fraction)),
            "initial_cycle_mean": 1.0,
            "initial_cycle_variance": rounded(
                float(solve_fraction) ** 2 + (1.0 - float(solve_fraction)) ** 2
            ),
            "post_disclosure_equal_hash_power_mean": rounded(
                1.0 - float(solve_fraction)
            ),
            "post_disclosure_equal_hash_power_variance": rounded(
                (1.0 - float(solve_fraction)) ** 2
            ),
        }
        for solve_fraction in config["solve_fraction_sensitivities"]
    ]
    return {
        "schema": config["schema"],
        "normative_status": NON_NORMATIVE,
        "authority_boundary": {
            "absolute_target_time": "UNRESOLVED; P-1 remains TBD(P-006)/G0",
            "protocol_windows_and_clamps": "UNRATIFIED candidate coordinates only",
            "size_signal": "synthetic checker-derived sensitivity model, not measured SIS behavior",
            "protocol_spec_modified": False,
        },
        "model": {
            "normalized_target_interval": config["normalized_target_interval"],
            "block_interval": "exponential solution delay plus exponential hash delay",
            "hash_observation": "inter-block interval EMA only",
            "size_observation": "checker-derived quality margin only",
            "quality_suppression": "adversarial winner publishes a valid lower-margin solution; no forged field",
            "numeric_guard": "states clamp at 2^-24..2^24 only to continue recording an unstable coordinate; any hit fails the envelope",
            "solution_reuse_boundary": solution_reuse_boundary,
        },
        "evidence_inputs": {
            "config_sha256": file_sha256(config_path),
            "simulator_sha256": file_sha256(Path(__file__).resolve()),
            "fixed_seeds": config["fixed_seeds"],
            "solve_fraction_sensitivities": config["solve_fraction_sensitivities"],
            "quality_elasticity_sensitivities": config[
                "quality_elasticity_sensitivities"
            ],
            "runs_per_candidate_scenario": len(config["fixed_seeds"])
            * len(config["solve_fraction_sensitivities"])
            * len(config["quality_elasticity_sensitivities"]),
            "simulated_block_count": len(candidate_rows)
            * len(config["scenarios"])
            * len(config["fixed_seeds"])
            * len(config["solve_fraction_sensitivities"])
            * len(config["quality_elasticity_sensitivities"])
            * int(config["blocks_per_run"]),
        },
        "envelope": config["envelope"],
        "summary": {
            "candidate_count": len(candidate_rows),
            "honest_envelope_pass_count": len(honest_ids),
            "adversarial_envelope_pass_count": len(adversarial_ids),
            "honest_envelope_candidate_ids": honest_ids,
            "adversarial_envelope_candidate_ids": adversarial_ids,
            "hash_only_envelope_pass_count": len(hash_only_ids),
            "hash_only_envelope_candidate_ids": hash_only_ids,
            "hash_only_unique_setting_pass_count": len(hash_only_unique_settings),
            "hash_only_unique_setting_count": len(all_unique_hash_settings),
            "hash_only_unique_settings": [
                {
                    "hash_ema_blocks": setting[0],
                    "hash_step_limit": setting[1],
                }
                for setting in hash_only_unique_settings
            ],
            "scenario_pass_counts": scenario_pass_counts,
            "scenario_classes": scenario_classes,
        },
        "candidates": candidate_rows,
    }


def json_bytes(result: dict[str, Any]) -> bytes:
    return (json.dumps(result, indent=2, sort_keys=True) + "\n").encode("utf-8")


def scenario_by_id(row: dict[str, Any], scenario_id: str) -> dict[str, Any]:
    for scenario in row["scenarios"]:
        if scenario["scenario"] == scenario_id:
            return scenario
    raise AssertionError(f"missing scenario summary: {scenario_id}")


def markdown_bytes(result: dict[str, Any]) -> bytes:
    summary = result["summary"]
    candidates_all = result["candidates"]
    honest = [row for row in candidates_all if row["honest_envelope_pass"]]
    adversarial = [row for row in candidates_all if row["adversarial_envelope_pass"]]
    lines = [
        "# P-006 stability envelope",
        "",
        f"**{NON_NORMATIVE}**",
        "",
        "This generated report compares hypothetical, dimensionless controller",
        "coordinates. It does not fill Protocol Spec P-1, P-2, or P-11, and it does",
        "not authorize Phase-2 implementation. All protocol selection remains HUMAN/G0.",
        "",
        "## Experiment contract",
        "",
        f"- Candidate coordinates: {summary['candidate_count']}.",
        f"- Fixed sensitivity runs per candidate/scenario: {result['evidence_inputs']['runs_per_candidate_scenario']}.",
        "- Normalized target interval: 1.0; no seconds or absolute P-1 value is assigned.",
        "- Hash observation: inter-block interval EMA only.",
        "- Size observation: synthetic checker-derived quality margin only.",
        "- Required scenarios: stationary, 4x/0.25x hash steps, 4x/0.25x solve",
        "  steps, opposed shocks, 64-block opposed oscillation, and 35%/51%",
        "  strategic quality-margin suppression.",
        "",
        "## Predeclared envelope",
        "",
        "A coordinate passes the honest/unmanipulated-quality envelope only if every",
        "fixed-seed/sensitivity run stays within the configured tail-interval,",
        "controller-state, recovery,",
        "and excursion bounds. The adversarial envelope additionally requires at least",
        f"{result['envelope']['adversarial_size_retention_min']:.0%} instance-size retention.",
        "The exact thresholds are preserved in `config.json` and `results.json`; they",
        "are research acceptance rules, not consensus constants.",
        "",
        "## Result",
        "",
        f"- Honest/unmanipulated-quality envelope passes: **{summary['honest_envelope_pass_count']} / {summary['candidate_count']}**.",
        f"- Adversarial-envelope passes: **{summary['adversarial_envelope_pass_count']} / {summary['candidate_count']}**.",
        f"- Restricted hash-only envelope passes: **{summary['hash_only_envelope_pass_count']} / {summary['candidate_count']}**.",
        f"- Unique restricted hash-loop settings passing: **{summary['hash_only_unique_setting_pass_count']} / {summary['hash_only_unique_setting_count']}**.",
        "",
    ]
    lines.extend(
        [
            "### Scenario pass counts",
            "",
            "| scenario | schedule class | passing coordinates |",
            "|---|---|---:|",
            *[
                f"| {scenario_id} | {summary['scenario_classes'][scenario_id]} | {count} / {summary['candidate_count']} |"
                for scenario_id, count in summary["scenario_pass_counts"].items()
            ],
            "",
            "Here `honest` means the published quality signal is not strategically",
            "selected; that envelope deliberately still contains the opposed one-time",
            "and periodic adversarial hash/solve-power schedules shown above.",
            "",
            "All 36 full-grid coordinates passed the restricted stationary/hash-step",
            "envelope. Those coordinates contain 6/6 unique hash-loop settings (three",
            "EMA coordinates × two caps), each repeated across six size settings,",
            "so this experiment supports only a broad non-normative hash-loop region:",
            "EMA coordinates of 16–64 blocks and per-update caps of 1.125×–1.25× under",
            "the fixed model gain. It does not distinguish or ratify P-2 within that",
            "region. No coordinate passed a solve-power shock or the opposed periodic",
            "schedule across all fixed seeds, solve-stage shares, and quality",
            "elasticities; the tested P-11 region of 64–256 blocks therefore has no",
            "robust candidate in this model.",
            "",
        ]
    )
    if honest:
        lines.extend(
            [
                "### Honest candidate region (proposal input only)",
                "",
                "| candidate | hash EMA | hash step cap | size window | size step cap | worst 35% suppression retention |",
                "|---|---:|---:|---:|---:|---:|",
            ]
        )
        for row in honest:
            coords = row["coordinates"]
            suppression = scenario_by_id(row, "quality_suppression_35pct")
            lines.append(
                "| {candidate_id} | {hash_ema_blocks} | {hash_step_limit:.3f}× | "
                "{size_window_blocks} | {size_step_limit:.3f}× | {retention:.3f}× |".format(
                    candidate_id=row["candidate_id"],
                    retention=suppression["tail_size_retention_min"],
                    **coords,
                )
            )
        lines.append("")
    else:
        lines.extend(
            [
                "No tested coordinate passed every unmanipulated-quality scenario and sensitivity.",
                "That is a negative result; the grid must not be promoted by selecting",
                "the least-bad row.",
                "",
            ]
        )

    if not adversarial:
        lines.extend(
            [
                "### Structural adversarial finding",
                "",
                "No coordinate passed the adversarial envelope. Although each observed",
                "quality value is recomputed by the checker, a miner able to choose among",
                "valid solutions can publish a threshold-margin solution and suppress",
                "information about its better solution. The modeled size loop responds by",
                "lowering instance size. Changing only EMA windows or clamp magnitudes",
                "changes the speed of that drift, not its direction or eventual incentive.",
                "",
                "Some slow size coordinates remain above the finite-run retention bound",
                "only because they also fail to adapt to honest solve-power shocks. Under",
                "the model's persistent suppression equation, the asymptotic size ratio is",
                "`(1 - adversarial_share)^(1 / elasticity)`: 0.423–0.806 for the",
                "35% case and 0.240–0.700 for the 51% case across the configured",
                "elasticity sweep. Slowness delays the bias; it does not remove it.",
                "",
                "Therefore this experiment does **not** support freezing a size retarget",
                "driven solely by winning-block quality margin. G0/Phase 2 must either",
                "supply an independently justified manipulation-resistant observable,",
                "retain a human-ratified static size between explicit upgrades, or open the",
                "D2 fallback review. This report makes no such decision.",
                "",
            ]
        )
    else:
        lines.extend(
            [
                "### Adversarial candidate region (proposal input only)",
                "",
                *[f"- `{row['candidate_id']}`" for row in adversarial],
                "",
            ]
        )

    lines.extend(
        [
            "## First-solver / solution-reuse boundary",
            "",
            "The same stochastic model makes the solve-once boundary explicit without",
            "inventing a propagation delay or miner count. Before disclosure, an equal-",
            "power miner pays the solution and hash stages. A copier of a public valid",
            "solution still must rerun the full miner-bound eligibility hash race, but no",
            "longer pays the solve stage:",
            "",
            "| normalized solve-stage share | initial cycle mean | initial variance | post-disclosure hash-only mean | post-disclosure variance |",
            "|---:|---:|---:|---:|---:|",
            *[
                "| {solve_fraction:.3f} | {initial_cycle_mean:.3f} | "
                "{initial_cycle_variance:.3f} | "
                "{post_disclosure_equal_hash_power_mean:.3f} | "
                "{post_disclosure_equal_hash_power_variance:.3f} |".format(**row)
                for row in result["model"]["solution_reuse_boundary"]
            ],
            "",
            "This quantifies the head-start/variance boundary, not fork probability.",
            "A fork probability requires propagation, competing-miner count/power, and",
            "withholding behavior that are absent and therefore remain UNKNOWN.",
            "",
            "## Absolute-cadence limitation",
            "",
            "P-003 did not solve its provisional P-4 candidate, P-002 did not assign a",
            "production VDF delay, and this packet has no propagation/reference-miner",
            "distribution. An absolute target in seconds would therefore be invented, not",
            "discovered. P-1 remains unresolved. Candidate windows in this report are",
            "expressed in blocks and remain unratified P-2/P-11 proposal inputs.",
            "",
            "## Interpretation limits",
            "",
            "- The simulator is a sensitivity model, not a proof of Nakamoto security,",
            "  selfish-mining resistance, or real SIS solver behavior.",
            "- Exponential stage delays and the quality-response equation are explicit",
            "  assumptions. The elasticity sweep reduces but does not remove model risk.",
            "- Same-height solution reuse removes a rival's solve stage after disclosure,",
            "  but its effect depends on propagation and fork-race data absent here. It",
            "  remains a Phase-2 adversarial-corpus/network-model obligation rather than a",
            "  fabricated numeric result.",
            "- A stable hash-rate loop cannot rehabilitate a manipulable size signal; the",
            "  two conclusions must remain separate.",
            "",
            "## Reproduction seals",
            "",
            f"- `config.json` SHA-256: `{result['evidence_inputs']['config_sha256']}`.",
            f"- `simulate.py` SHA-256: `{result['evidence_inputs']['simulator_sha256']}`.",
            "- `python simulate.py check` regenerates both committed artifacts in a",
            "  temporary directory and compares exact bytes.",
            "",
        ]
    )
    return ("\n".join(lines).rstrip() + "\n").encode("utf-8")


def produce(config_path: Path, output_dir: Path) -> tuple[bytes, bytes]:
    config = load_config(config_path)
    result = run_matrix(config, config_path)
    return json_bytes(result), markdown_bytes(result)


def write_outputs(config_path: Path, output_dir: Path) -> None:
    json_output, markdown_output = produce(config_path, output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    (output_dir / "results.json").write_bytes(json_output)
    (output_dir / "STABILITY-ENVELOPE.md").write_bytes(markdown_output)


def check_outputs(config_path: Path, output_dir: Path) -> None:
    with tempfile.TemporaryDirectory(prefix="cj3-p006-check-") as temporary:
        regenerated = Path(temporary)
        write_outputs(config_path, regenerated)
        for name in ("results.json", "STABILITY-ENVELOPE.md"):
            committed_path = output_dir / name
            regenerated_path = regenerated / name
            if not committed_path.is_file():
                raise FileNotFoundError(f"missing committed evidence: {committed_path}")
            if committed_path.read_bytes() != regenerated_path.read_bytes():
                raise RuntimeError(f"committed evidence differs from regeneration: {name}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "command",
        choices=("generate", "check"),
        help="generate committed evidence or verify it byte-for-byte",
    )
    parser.add_argument("--config", type=Path, default=DEFAULT_CONFIG)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_EVIDENCE)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        if args.command == "generate":
            write_outputs(args.config.resolve(), args.output_dir.resolve())
            print(f"P006_GENERATE=PASS output={args.output_dir.resolve()}")
        else:
            check_outputs(args.config.resolve(), args.output_dir.resolve())
            print(f"P006_REPRODUCIBILITY=PASS output={args.output_dir.resolve()}")
    except (ArithmeticError, ConfigError, FileNotFoundError, RuntimeError) as error:
        print(f"P006_SIMULATION=FAIL reason={error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
