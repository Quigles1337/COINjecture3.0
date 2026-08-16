"""Pinned homogeneous Euclidean-SIS estimator inputs for P-003.

Run only against malb/lattice-estimator commit
3e48ef421ec256afddb3e7d2249a77eab6e9ba12. Output is JSON Lines so the raw
model results can be diffed without parsing human-oriented pretty printing.
"""

import json

from estimator import SIS
from sage.all import log, sqrt


CANDIDATES = (
    (16, 288, 257, 288),
    (24, 480, 577, 480),
    (32, 704, 1031, 704),
    (40, 880, 1601, 880),
    (48, 1152, 2309, 1152),
    (64, 1664, 4099, 1664),
    (80, 2080, 6421, 2080),
    (96, 2688, 9221, 2688),
    (128, 3840, 16411, 3840),
)


for n, m, q, beta_squared in CANDIDATES:
    parameters = SIS.Parameters(
        n=n,
        m=m,
        q=q,
        length_bound=sqrt(beta_squared),
        norm=2,
    )
    default = SIS.lattice(parameters)
    rough = SIS.estimate.rough(parameters, quiet=True)["lattice"]
    print(
        json.dumps(
            {
                "n": n,
                "m": m,
                "q": q,
                "beta_squared": beta_squared,
                "default_log2_rop": float(log(default["rop"], 2)),
                "default_block_size": int(default["beta"]),
                "default_attack_dimension": int(default["d"]),
                "rough_log2_rop": float(log(rough["rop"], 2)),
                "rough_block_size": int(rough["beta"]),
                "rough_attack_dimension": int(rough["d"]),
            },
            sort_keys=True,
        )
    )
