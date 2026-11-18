# URDF-aligned keyboard validation

The Python unit tests pass for URDF geometry extraction, FK/IK inversion,
forward/reverse/turn signs, rejection of a mismatched drive configuration,
keyboard mappings and key-timeout stopping.

The installed keyboard node was exercised through a pseudo-terminal against an
isolated Gazebo theatre. Each key was repeated for two seconds. The measured
wheel rates and odometry matched their targets after acceleration, and Gazebo
pose checks confirmed forward/reverse and left/right rotation directions.

| Key | Target v (m/s), yaw (rad/s) | Measured left/right joints (rad/s) | Result |
| --- | --- | --- | --- |
| W | +0.1500, +0.0000 | +1.8750, +1.8750 | PASS |
| S | -0.1500, +0.0000 | -1.8750, -1.8750 | PASS |
| A | +0.0000, +0.4000 | -0.8011, +0.8011 | PASS |
| D | +0.0000, -0.4000 | +0.8011, -0.8011 | PASS |
| Q | +0.1500, +0.4000 | +1.0739, +2.6761 | PASS |
| E | +0.1500, -0.4000 | +2.6761, +1.0739 | PASS |

Both the no-key timeout and Ctrl+C stop checks passed. A second keyboard
instance was also rejected before it could publish competing zero commands. The updated package also
built and passed its unit tests inside the existing Docker image with the host
source mounted read-only. No persistent development volumes were modified.

These results validate simulated commands, signs and telemetry; they are not
physical-hardware calibration or a no-slip guarantee. MATLAB is not installed on
the test host, so its scripts were checked structurally and against independent
Python numerical results, not executed in MATLAB. The `.mlx` source was updated
and old cached output removed; the readable `.m` version is also provided.

To rerun, see [the test instructions](README.md#checks).
