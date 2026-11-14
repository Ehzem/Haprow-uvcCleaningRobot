# Haprow workspace source packages

This directory contains the ROS 2 packages used by **Haprow-uvcCleaningRobot**.
The supported project environment is **Ubuntu 24.04, ROS 2 Jazzy and Gazebo
Harmonic**. Use the [main workspace README](../../README.md) for installation,
Docker development, GitHub collaboration and native-build commands.

- `uvc_robot_description`: UVC CAD robot, joints, collision approximations and RViz.
- `yahboom_rosmaster_gazebo`: operating theatre world, environment models,
  UVC simulation launch and ROS–Gazebo bridge.
- Other `yahboom_rosmaster_*` packages and `mecanum_drive_controller`: original
  ROSMASTER support, navigation, localization, docking and utilities.

The default theatre launch runs the UVC robot with a flat floor. Nav2 packages
are included, but UVC autonomous navigation and a theatre map still need setup.

The original support packages derive from
[Automatic Addison's yahboom_rosmaster](https://github.com/automaticaddison/yahboom_rosmaster)
at commit `6f99244`. Their existing license files are retained. The old upstream
ROS Lyrical/robot-arm tutorial is not the setup guide for this workspace.
