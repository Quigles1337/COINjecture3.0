"""Standard-library regression tests for the non-normative P-006 simulator."""

from __future__ import annotations

import copy
import json
import math
import tempfile
import unittest
from pathlib import Path

import simulate


class SimulationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.config = simulate.load_config(simulate.DEFAULT_CONFIG)

    def test_candidate_grid_is_bounded_and_stable(self) -> None:
        rows = list(simulate.candidates(self.config))
        self.assertEqual(36, len(rows))
        self.assertEqual(
            "h16-hc1p125-s64-sc1p125",
            rows[0].candidate_id,
        )
        self.assertEqual(len(rows), len({row.candidate_id for row in rows}))

    def test_fixed_seed_run_is_deterministic(self) -> None:
        candidate = next(iter(simulate.candidates(self.config)))
        scenario = self.config["scenarios"][0]
        first = simulate.simulate_one(
            self.config, candidate, scenario, 6006001, 0.2, 1.0
        )
        second = simulate.simulate_one(
            self.config, candidate, scenario, 6006001, 0.2, 1.0
        )
        self.assertEqual(first, second)

    def test_quality_suppression_never_forges_below_threshold(self) -> None:
        self.assertEqual(1.0, simulate.select_published_quality(2.0, 0.0, True))
        self.assertEqual(1.5, simulate.select_published_quality(2.0, 0.5, True))
        self.assertEqual(2.0, simulate.select_published_quality(2.0, 0.0, False))
        with self.assertRaises(ArithmeticError):
            simulate.select_published_quality(0.99, 0.0, True)
        with self.assertRaises(ArithmeticError):
            simulate.select_published_quality(math.nan, 0.0, True)
        candidate = next(iter(simulate.candidates(self.config)))
        scenario = next(
            row
            for row in self.config["scenarios"]
            if row["id"] == "quality_suppression_51pct"
        )
        metrics = simulate.simulate_one(
            self.config, candidate, scenario, 6006001, 0.2, 1.0
        )
        self.assertGreater(metrics.adversarial_blocks, 0)
        self.assertGreater(metrics.tail_size_state_mean, 0.0)

    def test_unbounded_configuration_fails_closed(self) -> None:
        mutated = copy.deepcopy(self.config)
        mutated["blocks_per_run"] = 10_001
        with self.assertRaises(simulate.ConfigError):
            simulate.validate_config(mutated)

    def test_aggregate_work_limit_fails_closed(self) -> None:
        mutated = copy.deepcopy(self.config)
        mutated["blocks_per_run"] = 10_000
        mutated["shock_block"] = 1_000
        mutated["tail_blocks"] = 1_000
        mutated["fixed_seeds"] = list(range(32))
        mutated["solve_fraction_sensitivities"] = [
            0.1,
            0.2,
            0.3,
            0.4,
            0.5,
            0.6,
            0.7,
            0.8,
        ]
        mutated["quality_elasticity_sensitivities"] = [
            0.5,
            0.75,
            1.0,
            1.25,
            1.5,
            2.0,
            2.5,
            3.0,
        ]
        with self.assertRaises(simulate.ConfigError):
            simulate.validate_config(mutated)

    def test_recovery_is_credited_only_after_observation_and_hold(self) -> None:
        intervals = [1.0] * 100
        self.assertEqual(
            55,
            simulate.first_sustained_recovery(
                intervals, shock_block=10, band=0.2, hold=24
            ),
        )

    def test_wrong_normative_marker_fails_closed(self) -> None:
        mutated = copy.deepcopy(self.config)
        mutated["normative_status"] = "normative"
        with self.assertRaises(simulate.ConfigError):
            simulate.validate_config(mutated)

    def test_generate_then_check_is_byte_exact(self) -> None:
        with tempfile.TemporaryDirectory(prefix="cj3-p006-test-") as temporary:
            output = Path(temporary)
            simulate.write_outputs(simulate.DEFAULT_CONFIG, output)
            first = (output / "results.json").read_bytes()
            simulate.check_outputs(simulate.DEFAULT_CONFIG, output)
            payload = json.loads(first)
            self.assertEqual(simulate.NON_NORMATIVE, payload["normative_status"])
            self.assertFalse(payload["authority_boundary"]["protocol_spec_modified"])
            self.assertEqual(5_971_968, payload["evidence_inputs"]["simulated_block_count"])
            self.assertEqual(6, payload["summary"]["hash_only_unique_setting_count"])
            self.assertEqual(
                6, payload["summary"]["hash_only_unique_setting_pass_count"]
            )


if __name__ == "__main__":
    unittest.main()
