#!/usr/bin/env python3
"""Publish the UVC robot description and optionally open its RViz view."""
from pathlib import Path
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.conditions import IfCondition
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node
from launch_ros.parameter_descriptions import ParameterValue


def generate_launch_description():
    package = Path(get_package_share_directory('uvc_robot_description'))
    description = (package / 'urdf/robots/uvc_robot.urdf').read_text()
    # Absolute installed paths let Gazebo resolve meshes without extra search paths.
    description = description.replace('package://uvc_robot_description/', f'file://{package}/')
    sim_time = {'use_sim_time': ParameterValue(LaunchConfiguration('use_sim_time'), value_type=bool)}
    return LaunchDescription([
        DeclareLaunchArgument('use_sim_time', default_value='false'),
        DeclareLaunchArgument('use_rviz', default_value='true'),
        Node(package='robot_state_publisher', executable='robot_state_publisher',
             parameters=[sim_time, {'robot_description': description}], output='screen'),
        Node(package='rviz2', executable='rviz2', output='screen',
             arguments=['-d', str(package / 'rviz/uvc_robot.rviz')],
             parameters=[sim_time], condition=IfCondition(LaunchConfiguration('use_rviz'))),
    ])
