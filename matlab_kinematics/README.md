# UVC kinematics aligned with the robot URDF

`uvc_parameters.m` reads the actual robot definition at
`../src/yahboom_rosmaster/uvc_robot_description/urdf/robots/uvc_robot.urdf`.
It checks that wheel-joint geometry and the Gazebo DiffDrive parameters agree.

| Quantity | URDF convention/value |
| --- | --- |
| Wheel radius | 0.080 m |
| Wheel-centre separation | 0.32044 m |
| Robot forward / left / up | +X / +Y / +Z |
| Positive heading | Left turn about +Z |
| Positive wheel-joint velocity | Forward rolling, both axes +Y |
| Base origin | Ground below drive-axle midpoint |
| Horizontal axle-frame correction | Zero |
| Chassis collision centre | 0.10886 m ahead of the axle |

For wheel rates in rad/s:

```text
v   = r * (left + right) / 2
yaw = r * (right - left) / L
left  = (v - L*yaw/2) / r
right = (v + L*yaw/2) / r
```

For `v=0.15 m/s`, `yaw=+0.20 rad/s`, the expected wheel speeds are approximately
**left 14.079960 RPM, right 21.729902 RPM**. Straight motion at `0.15 m/s`
requires both wheels at `17.904931 RPM`.

## MATLAB examples

Keep this directory alongside `src/` as in the repository. Open MATLAB in this
folder and run, in order:

```matlab
uvc_preflight
uvc_geometry_and_mass
uvc_forward_kinematics
uvc_inverse_kinematics
```

The numerical scripts now have valid MATLAB identifiers instead of numbered
filenames, so they can be run directly from the Command Window.
The examples read the URDF using `xmlread` and do not need a `.slx` file or
Simscape. The numerical scripts need MATLAB with Java XML support; the symbolic
`.m`/`.mlx` derivation additionally needs Symbolic Math Toolbox. The `.mlx`
source has been updated and its old cached outputs cleared.

These are **ideal mathematical predictions**, not measured dynamics tests.
The old missing `UVC_Bot_Validated.slx`, hardcoded Windows packaging path,
negative-forward joint commands, 135 mm correction and recorded accuracy claims
have been removed from the example workflow. An optional shared exponential
input ramp is mathematical only; it is not Gazebo's acceleration model.
MATLAB execution is not available in the development host used for this change;
Python numerical checks and actual Gazebo motion checks were run instead.

## Test actual motion with keys

After rebuilding/sourcing `uvc_robot_description`, start the operating theatre
in one terminal, then the keyboard monitor in another:

```bash
ros2 launch yahboom_rosmaster_gazebo operating_theatre.launch.py
# In another sourced terminal:
ros2 run uvc_robot_description uvc_keyboard
```

Or, on the Docker host, use this checkout with the existing image:

```bash
./docker/test-motion.sh
# Without GUI windows:
./docker/test-motion.sh --headless
```

The host helper starts a disposable container, mounts this checkout's `src/`
read-only, rebuilds the UVC description, and starts the theatre/tmux. It does not
mount or replace your existing development volumes. It uses ROS domain 87
(`UVC_TEST_ROS_DOMAIN_ID` can override it) and a unique Gazebo partition, so an
existing simulation on the default Docker domain is separate. In tmux, press Ctrl+B,
release, then 2 to select the teleop shell. Run
`ros2 run uvc_robot_description uvc_keyboard` there once. Keep keyboard focus
in that terminal, not the Gazebo window. The tool refuses to start if another
velocity publisher is already active; an idle second controller would otherwise
override movement commands with zeros.

| Keys | Body command |
| --- | --- |
| W / S | Forward / reverse |
| A / D | Positive / negative in-place yaw (left / right) |
| Q / E | Forward with positive / negative yaw |
| Z / C | Reverse with positive / negative yaw |
| Space or X | Stop |
| Esc or Ctrl+C | Stop and exit |

Hold/repeat a movement key. The tool sends zero after 0.4 seconds without a
movement/stop key, and sends stop messages on normal exit, Ctrl+C, or SIGTERM.
Default speeds are 0.15 m/s and 0.4 rad/s. To change them within URDF limits:

```bash
ros2 run uvc_robot_description uvc_keyboard --linear-speed 0.10 --angular-speed 0.20
```

The screen shows the body command, expected wheel RPM from inverse kinematics,
actual `/joint_states` wheel rates, wheel-derived forward kinematics and `/odom`.
Targets are steady-state: acceleration transients and contact/slip can produce
differences. Odometry is wheel-derived, so it is not independent proof of the
physical path. Inspect Gazebo motion as well. Run only one velocity-command
publisher while testing. Check with `ros2 topic info /cmd_vel`: during manual
testing the publisher count should be 1. Two keyboard nodes can fight because
the idle one repeatedly publishes zero; stop the extra instance rather than
changing wheel geometry or increasing the speed.

## Checks

From the workspace root, without ROS running:

```bash
python3 -m unittest discover -s src/yahboom_rosmaster/uvc_robot_description/tests -v
```

After building/sourcing both description and Gazebo packages, this optional
integration test starts its own test world, drives the keyboard node through a
pseudo-terminal, and checks wheel feedback, odometry, physical motion direction,
key timeout, duplicate-controller rejection and Ctrl+C stopping:

```bash
python3 src/yahboom_rosmaster/uvc_robot_description/tests/verify_keyboard_motion.py
```

It uses ROS domain 86 by default (`UVC_TEST_ROS_DOMAIN_ID` can override it) and a
unique Gazebo partition. Gazebo sensors need a graphics context; use
`xvfb-run -a python3 ...` when available on a headless system. Test logs go to
`/tmp/uvc_keyboard_gazebo.log` and `/tmp/uvc_keyboard_terminal.log`.

MATLAB references: [script naming](https://www.mathworks.com/help/matlab/matlab_env/specify-file-names.html), [running scripts](https://www.mathworks.com/help/matlab/ref/run.html).
