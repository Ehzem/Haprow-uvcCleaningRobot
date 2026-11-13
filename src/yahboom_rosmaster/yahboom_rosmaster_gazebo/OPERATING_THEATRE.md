# Operating theatre with UVC robot

Launch from a terminal on the desktop:

```bash
cd /home/itadmin/addison_ws
source /opt/ros/jazzy/setup.bash
source install/setup.bash
ros2 launch yahboom_rosmaster_gazebo operating_theatre.launch.py
```

This starts the theatre and the supplied UVC CAD robot, with differential drive,
odometry, simulated lidar and IMU, ROS bridges, robot transforms, and RViz.
The theatre launch no longer spawns ROSMASTER X3 or its controllers.
For Gazebo without RViz, append `use_rviz:=false`. For a server without a Gazebo
window, append `headless:=true use_rviz:=false`; rendered sensors still require
graphics support. Stop with Ctrl+C before relaunching.

The robot starts in the clear central area at `(0, 0, 0.02)` metres and settles
onto the floor. Override its starting pose with `x:=... y:=... z:=... yaw:=...`.

## Driving

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

## UVC model

`uvc_robot_description/urdf/robots/uvc_robot.urdf` and
`uvc_robot_description/meshes/uvc_robot/` contain the imported robot. Its
state-publisher launch and RViz configuration also live in that description
package; the Gazebo package keeps the theatre world and simulation bridge. The source is `UVC Robot V 67.obj` with its supplied MTL; these particular
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
Camera streams from the X3 are not reproduced by this launch.

To regenerate the robot from the same source export:

```bash
python3 src/yahboom_rosmaster/uvc_robot_description/scripts/import_uvc_robot.py '/home/itadmin/Downloads/UVC Robot V 67.obj'
colcon build --packages-select uvc_robot_description yahboom_rosmaster_gazebo --symlink-install
```

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
python3 src/yahboom_rosmaster/yahboom_rosmaster_gazebo/scripts/import_operating_theatre.py '/home/itadmin/Downloads/hospital assembly.3mf'
colcon build --packages-select uvc_robot_description yahboom_rosmaster_gazebo --symlink-install
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
