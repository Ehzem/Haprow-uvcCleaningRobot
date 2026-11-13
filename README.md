# Haprow-uvcCleaningRobot

ROS 2 Jazzy workspace for a CAD-based UVC cleaning robot in a Gazebo operating
theatre. Includes the hospital environment, a level floor, differential-drive
robot simulation, odometry, lidar and IMU, plus a Docker environment with tmux.

## Workspace layout

```text
src/yahboom_rosmaster/
  uvc_robot_description/             Robot URDF, meshes, visualization launch
  yahboom_rosmaster_gazebo/          Worlds, environment models, simulation bridge
  yahboom_rosmaster_bringup/         Existing ROSMASTER launch files
  yahboom_rosmaster_navigation/      Navigation utilities and configurations
  ...                               Other original Yahboom support packages
Dockerfile                          Image building the complete workspace
compose.yaml                        Container configuration
docker/                            Container helpers and usage instructions
```

## Run with Docker

Docker Engine and Compose must be installed on the host. From this directory:

```bash
sudo env LOCAL_UID="$(id -u)" LOCAL_GID="$(id -g)" docker compose build
sudo docker compose run --rm workspace bash
```

Inside the container, confirm where you are and list the source:

```bash
test -f /.dockerenv && echo 'You are inside Docker'
ls /home/itadmin/addison_ws/src
```

See [Docker instructions](docker/README.md) for GUI access, tmux, GPU options,
source mounts, persistence and the simulation smoke test.

## Build and run natively

Use Ubuntu 24.04 with ROS 2 Jazzy and rosdep installed. Clone this repository
into `~/addison_ws` to support the original X3 launcher's home-relative paths.
Install and build the source dependencies from the workspace root:

```bash
source /opt/ros/jazzy/setup.bash
rosdep update --rosdistro jazzy
rosdep install --from-paths src --ignore-src --rosdistro jazzy -y
colcon build --symlink-install --executor sequential
source install/setup.bash
ros2 launch yahboom_rosmaster_gazebo operating_theatre.launch.py
```

The theatre launcher spawns the UVC robot. The original ROSMASTER X3 launcher
remains available separately. See the
[theatre documentation](src/yahboom_rosmaster/yahboom_rosmaster_gazebo/OPERATING_THEATRE.md)
and [UVC description](src/yahboom_rosmaster/uvc_robot_description/README.md).

## Simulation scope

The UVC robot uses imported CAD visual geometry with approximated mass, inertia,
collision shapes and caster contacts. Lidar and IMU are simulation additions.
UV dose/disinfection physics are not implemented. Navigation dependencies are
included; a theatre SLAM map and UVC Nav2 configuration are still needed for
autonomous navigation.

## Source and attribution

The original Yahboom support packages come from
[Automatic Addison's yahboom_rosmaster](https://github.com/automaticaddison/yahboom_rosmaster),
starting from commit `6f99244`. Existing package license files are preserved.
The hospital and UVC visuals were converted from the CAD exports supplied for
this project. Original Downloads files are not required to run the simulation.

Host build products (`build/`, `install/`, `log/`), credentials, and local
upstream Git metadata are excluded from version control. Rebuild the workspace
after cloning; Docker images are built separately rather than committed here.
