# UVC robot description

This package follows the layout of `yahboom_rosmaster_description`:

- `urdf/robots/uvc_robot.urdf`: robot links, joints, collision proxies and Gazebo extensions.
- `meshes/uvc_robot/`: metre-scale CAD visual meshes, with colours defined in the URDF.
- `launch/robot_state_publisher.launch.py`: robot description, transforms and optional RViz.
- `rviz/uvc_robot.rviz`: robot and scan visualization.
- `scripts/import_uvc_robot.py`: source OBJ/MTL import utility.

The theatre world, environment meshes, ROS–Gazebo bridge and simulation launch
remain in `yahboom_rosmaster_gazebo`.

First build/source the native workspace using the
[main README](../../../README.md#build-and-run-natively), or open the Docker
development shell, which sources its environment automatically.
Start the simulation using:

```bash
ros2 launch yahboom_rosmaster_gazebo operating_theatre.launch.py
```

To inspect the robot description independently:

```bash
ros2 launch uvc_robot_description robot_state_publisher.launch.py
```

Without the simulator, moving wheel joint transforms require a joint-state
publisher; RViz also needs an available fixed frame, e.g. `base_footprint`.

Regenerate the imported robot from the supplied centimetre-scale OBJ and MTL:

```bash
python3 src/yahboom_rosmaster/uvc_robot_description/scripts/import_uvc_robot.py "$HOME/Downloads/UVC Robot V 67.obj"
colcon --log-base .native-log build --build-base .native-build --install-base .native-install \
  --packages-select uvc_robot_description --symlink-install
source .native-install/setup.bash
```

The regeneration commands above use the native host layout. They need the
original OBJ/MTL files, which are not included in GitHub or Docker. For an
existing Docker workspace, use these commands instead:

```bash
colcon build --packages-select uvc_robot_description --symlink-install
source install/setup.bash
```

Physical mass, inertia, caster contacts and collision shapes are approximations.
Lidar and IMU are simulation additions. The source CAD remains unchanged.
