#!/usr/bin/env python3
"""Check the .glb files of Kits 3-6 (kit_signs, kit_nature, kit_water, kit_barriers)
against the ART-PIPELINE export rules, reusing check_glb.check() with the expected sizes
of these kits.

Usage:  python3 tools/blender/check_props_3_6.py      (exit code 1 if any check fails)

Sizes are (x, y = height, z) extents in metres; a tuple is a (min, max) range (organic
props), tolerance +-0.06 m as in check_glb.py.
"""

import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import check_glb  # noqa: E402

SIZES = {
    # kit_signs
    "enclosure_sign": (2.36, 1.95, 0.66),
    "info_board": (1.0, 1.49, 0.61),
    "map_board": (2.26, 2.43, 0.6),
    "food_box": (0.62, 0.6, 0.6),
    "food_box_stack": (1.29, 1.2, 0.73),
    # kit_nature
    "tree_round": ((3.0, 3.6), (4.7, 5.2), (2.7, 3.4)),
    "tree_grove": ((2.5, 3.0), (5.3, 6.2), (2.4, 3.0)),
    "tree_eucalyptus": ((2.8, 3.4), (6.6, 7.2), (1.6, 3.4)),
    "bush": ((1.1, 1.5), (0.9, 1.2), (1.0, 1.5)),
    "flower_bed": (2.0, (0.4, 0.6), 1.0),
    "rock": ((1.0, 1.6), (0.6, 1.0), (0.8, 1.2)),
    "bamboo": ((1.0, 1.8), (2.9, 3.1), (1.0, 1.8)),
    "reed": ((0.6, 1.1), (1.0, 1.3), (0.6, 1.1)),
    "grass_tuft": ((0.15, 0.4), (0.25, 0.4), (0.15, 0.4)),
    # kit_water — 1 m tiles, water surface at y = 0, bank grass at y = 0.05
    "water_river_straight": (1.0, (0.0, 0.01), 1.0),
    "water_river_bank": (1.0, (0.05, 0.16), 1.0),
    "water_river_curve": (1.0, (0.05, 0.16), 1.0),
    "water_river_inner": (1.0, (0.05, 0.16), 1.0),
    "water_pond": (1.0, (0.0, 0.01), 1.0),
    "water_pond_edge": (1.0, (0.05, 0.16), 1.0),
    "water_pond_corner": (1.0, (0.05, 0.16), 1.0),
    "bridge_wood": (3.4, 1.27, 2.61),
    "jetty_wood": (3.8, 0.7, 1.8),
    "lily_pad": (0.88, (0.05, 0.15), 0.71),
    "duck": (0.31, 0.34, 0.52),
    "frog": (0.39, 0.2, 0.4),
    # kit_barriers
    "road_block": (2.1, 1.05, 0.83),
    "repair_sign": (0.73, 1.32, 0.21),
    "zookeeper_cart": (2.31, 1.0, 1.12),
    "traffic_cone": (0.4, 0.46, 0.4),
    "fallen_tree": (2.42, 1.58, 4.26),
    "gate_zoo_closed": (3.12, 2.6, 0.72),
}


def main():
    check_glb.EXPECTED_SIZES.update(SIZES)
    base = os.path.join(check_glb.REPO, "assets", "models", "props")
    files = [os.path.join(base, n + ".glb") for n in SIZES]
    missing = [f for f in files if not os.path.exists(f)]
    for f in missing:
        print(f"MISSING {os.path.relpath(f, check_glb.REPO)}")
    rc = check_glb.main([f for f in files if os.path.exists(f)])
    return 1 if (rc or missing) else 0


if __name__ == "__main__":
    sys.exit(main())
