# Operating theatre with UVC robot

For a native host run, first follow the [native host build steps](../../../docker/README.md#native-host-build),
then launch from a desktop terminal:

```bash
cd ~/addison_ws
source /opt/ros/jazzy/setup.bash
source .native-install/setup.bash
ros2 launch yahboom_rosmaster_gazebo operating_theatre_keyboard.launch.py
```

On the desktop, the unified launch opens Gazebo, RViz, and a **UVC Keyboard**
window. Focus the keyboard window to drive. Press Esc there to stop the robot
and exit the session. Add `headless:=true` to skip the Gazebo window. For ROS or Gazebo
diagnostics from another shell, use `ROS_DOMAIN_ID=78` and
`GZ_PARTITION=uvc_operating_theatre`. The launch arguments `ros_domain_id`
and `gz_partition` can override these defaults.

Inside Docker, use the already sourced shell and `operating_theatre.launch.py`
with the keyboard controller in the tmux teleop pane.
If RViz reports a `GLIBC_PRIVATE` symbol error from a Snap library in the VS Code
integrated terminal, run the launch from a regular desktop terminal.
GUI windows need the [Docker GUI configuration](../../../docker/README.md#gazebo-and-rviz-windows);
`workspace-tmux --headless` handles a server without a display.

This starts the theatre and the supplied UVC CAD robot, with differential drive,
odometry, lidar, IMU, two forward-facing cameras, ROS bridges, robot transforms, and RViz.
The theatre launch no longer spawns ROSMASTER X3 or its controllers.
For Gazebo without RViz, append `use_rviz:=false`. For a server without a Gazebo
window, append `headless:=true use_rviz:=false`; rendered sensors still require
graphics support. Stop with Ctrl+C before relaunching.

The robot starts in the clear central area at `(0, 0, 0.02)` metres and settles
onto the floor. Override its starting pose with `x:=... y:=... z:=... yaw:=...`.

## Testing with less graphics load

Stop existing simulation launches before starting another one. On a desktop,
skip the Gazebo window and keep the RViz sensor displays:

```bash
ros2 launch yahboom_rosmaster_gazebo operating_theatre_keyboard.launch.py headless:=true
```

Both cameras publish at 640 × 360 and 5 Hz of simulation time. Gazebo still
renders the camera images and lidar data in the server. If another ROS or
Gazebo session needs to stay open, set `ROS_DOMAIN_ID` and `GZ_PARTITION` to
unused values before launching, and use the same values in the keyboard
terminal. This keeps the simulation clocks and velocity topics from overlapping.

## Driving

The unified launch starts the keyboard controller for you. With the simulation-only
launch, run it manually from a sourced interactive terminal:

```bash
ros2 run uvc_robot_description uvc_keyboard
```

W/S move forward/back, A/D turn in place, Q/E command forward curves, Space/X
stop and Esc/Ctrl+C stop/exit. Hold/repeat keys; timeout stopping is 0.65 seconds.
See the [kinematics guide](../../../matlab_kinematics/README.md) for equations
and a Docker host helper for testing the current checkout.


The UVC model accepts `geometry_msgs/msg/Twist` on `/cmd_vel`. It uses two drive
wheels and front caster contact approximations; it cannot strafe like the X3.
For example, in another sourced terminal, move slowly forward:

```bash
ros2 topic pub --rate 10 /cmd_vel geometry_msgs/msg/Twist '{linear: {x: 0.1}, angular: {z: 0.0}}'
```

After stopping that publisher with Ctrl+C, explicitly send a stop command:

```bash
ros2 topic pub --once /cmd_vel geometry_msgs/msg/Twist '{}'
```

A velocity command persists until replaced; do not assume stopping the publisher
stops the robot. The model limits linear speed to 0.3 m/s and turning speed to
1 rad/s. It publishes `/odom`, `/tf`, `/joint_states`, `/scan`, and `/imu/data`.

## Checking the sensors in RViz

Select **Move Camera** in the RViz toolbar. In the main 3D view, left-drag to
orbit, middle-drag to pan, and scroll to zoom.

With the launch running, use the **Left camera** and **Right camera** panes to see
theatre equipment from the robot. In the 3D view, **Laser scan** shows the
horizontal slice measured by the lidar, and **IMU orientation** shows the IMU
axes. The 3D view is fixed to `odom`, so scans stay in the same coordinate
system as the robot moves. Drive and turn the robot to compare the camera
images and scan against the scene in Gazebo.

From another sourced terminal, check that frames arrive on both image topics
and inspect one message from each remaining sensor topic:

```bash
ros2 topic hz /stereo/left/image
ros2 topic hz /stereo/right/image
ros2 topic echo /stereo/left/camera_info --once
ros2 topic echo /stereo/right/camera_info --once
ros2 topic echo /scan --once --qos-reliability best_effort
ros2 topic echo /imu/data --once --qos-reliability best_effort
ros2 run tf2_ros tf2_echo odom laser_frame
```

Stop each `hz` or `tf2_echo` command with Ctrl+C before running the next one.
If a sensor has no data, check that Gazebo is playing and use `gz topic -l`
to see whether the corresponding Gazebo topic is present.

## UVC model

`uvc_robot_description/urdf/robots/uvc_robot.urdf` and
`uvc_robot_description/meshes/uvc_robot/` contain the imported robot. Its
state-publisher launch and RViz configuration also live in that description
package; the Gazebo package keeps the theatre world and simulation bridge.
Paths above are relative to `src/yahboom_rosmaster/`.
The source is `UVC Robot V 67.obj` with its supplied MTL; these particular
OBJ coordinates are centimetres. The 35 mesh parts retain their material colours.
The model is approximately 0.56 m long, 0.46 m wide, and 1.90 m tall.
Its drive wheels have 0.08 m radius and 0.32044 m centre separation, measured
from the CAD. +X points toward the CAD front bumper; +Y points left. The origin
is on the floor below the drive axle.

Wheel joints, differential-drive control, sensors, mass and inertia were added
for simulation. Total mass is approximated as 37.42 kg. Chassis, tower and lamp
collisions use conservative box proxies; casters use low-friction spheres.
The lidar is a simulation addition just ahead of the chassis at z=0.34 m. It is
not a claim that the CAD includes a physical lidar at that location. Purple lamp
surfaces preserve the CAD appearance; UV dose/disinfection physics are not modeled.

To regenerate the robot from the same source export:

```bash
python3 src/yahboom_rosmaster/uvc_robot_description/scripts/import_uvc_robot.py "$HOME/Downloads/UVC Robot V 67.obj"
colcon --log-base .native-log build --build-base .native-build --install-base .native-install \
  --packages-select uvc_robot_description yahboom_rosmaster_gazebo --symlink-install
source .native-install/setup.bash
```

These regeneration examples assume a native build and access to the original
CAD files, which are not included in GitHub or the Docker image. Inside Docker,
rebuild with ordinary `colcon build` using its existing `build/` and `install/`.
The importer requires only Python's standard library. Runtime uses installed
local meshes and does not need the files in Downloads.

## Theatre geometry

The source is the supplied `hospital assembly.3mf`. It contains the room,
bed, lamp and monitoring equipment. Assets are in `models/operating_theatre`.
No external model downloads are needed for the world.

- Source millimetres are converted to metres, with Y up changed to Z up.
- Source `(X, Y, Z)` maps to `(X + 671.933952, -Z, Y - 256.698576)` before
  division by 1000. The original highest tile surface becomes z=0.
- Equipment placement is preserved, using static triangle mesh collisions.
- The ceiling is omitted to see into the room; original wall heights remain.
- The 35 individual tile visuals and collisions are replaced by a single
  8.3 by 6.0 m flat floor, with visible and collision surfaces at z=0.
  The original structural slab remains below it.

To regenerate the theatre:

```bash
python3 src/yahboom_rosmaster/yahboom_rosmaster_gazebo/scripts/import_operating_theatre.py "$HOME/Downloads/hospital assembly.3mf"
colcon --log-base .native-log build --build-base .native-build --install-base .native-install \
  --packages-select uvc_robot_description yahboom_rosmaster_gazebo --symlink-install
source .native-install/setup.bash
```

The axis transforms, component names and tile/ceiling selection in the importers
are specific to the supplied exports. Original CAD files are unchanged.

## Navigation and validation

The CAD robot is larger than the X3. Nav2 configuration must use its dimensions
and a differential-drive motion model, along with a new SLAM map of this room.
This change does not configure or validate autonomous Nav2 navigation.

Validation: URDF-to-SDF conversion and format checks, package build, and an
isolated headless drive check. The robot physically travelled about 0.35 m and
turned about 0.50 rad, while odometry, joint state, lidar and IMU messages arrived.

To use the original X3 again, the original launcher remains available:

```bash
ros2 launch yahboom_rosmaster_gazebo yahboom_rosmaster.gazebo.launch.py world_file:=operating_theatre.world
```
