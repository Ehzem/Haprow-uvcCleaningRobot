#!/usr/bin/env python3
"""Launch the operating theatre with the supplied UVC CAD robot."""
import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import AppendEnvironmentVariable, DeclareLaunchArgument, IncludeLaunchDescription
from launch.conditions import UnlessCondition
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node


def generate_launch_description():
    package = get_package_share_directory('yahboom_rosmaster_gazebo')
    gz_package = get_package_share_directory('ros_gz_sim')
    description_package = get_package_share_directory('uvc_robot_description')
    args = [
        DeclareLaunchArgument('headless', default_value='false'),
        DeclareLaunchArgument('use_rviz', default_value='true'),
        DeclareLaunchArgument('x', default_value='0.0'),
        DeclareLaunchArgument('y', default_value='0.0'),
        DeclareLaunchArgument('z', default_value='0.02'),
        DeclareLaunchArgument('yaw', default_value='0.0'),
    ]
    return LaunchDescription(args + [
        AppendEnvironmentVariable('GZ_SIM_RESOURCE_PATH', os.path.join(package, 'models')),
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(os.path.join(gz_package, 'launch/gz_sim.launch.py')),
            launch_arguments={'gz_args': ['-r -s -v 3 ', os.path.join(package, 'worlds/operating_theatre.world')]}.items(),
        ),
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(os.path.join(gz_package, 'launch/gz_sim.launch.py')),
            launch_arguments={'gz_args': '-g'}.items(),
            condition=UnlessCondition(LaunchConfiguration('headless')),
        ),
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(os.path.join(
                description_package, 'launch/robot_state_publisher.launch.py')),
            launch_arguments={'use_sim_time': 'true',
                              'use_rviz': LaunchConfiguration('use_rviz')}.items(),
        ),
        Node(package='ros_gz_sim', executable='create', output='screen',
             arguments=['-topic', '/robot_description', '-name', 'uvc_robot',
                        '-x', LaunchConfiguration('x'), '-y', LaunchConfiguration('y'),
                        '-z', LaunchConfiguration('z'), '-Y', LaunchConfiguration('yaw')]),
        Node(package='ros_gz_bridge', executable='parameter_bridge', output='screen',
             parameters=[{'config_file': os.path.join(package, 'config/uvc_robot_bridge.yaml')}]),
    ])
