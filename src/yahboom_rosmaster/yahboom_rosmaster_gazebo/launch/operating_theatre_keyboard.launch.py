"""Start the operating theatre and open its interactive keyboard controller."""
from pathlib import Path
from shutil import which

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import (
    DeclareLaunchArgument,
    ExecuteProcess,
    IncludeLaunchDescription,
    OpaqueFunction,
    RegisterEventHandler,
    SetEnvironmentVariable,
    Shutdown,
)
from launch.event_handlers import OnProcessExit
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration


def start_desktop_session(context):
    terminal_name = LaunchConfiguration('terminal').perform(context)
    terminal = which(terminal_name)
    if terminal is None:
        raise RuntimeError(f'Keyboard terminal executable not found: {terminal_name}')

    keyboard = ExecuteProcess(
        cmd=[terminal, '-T', 'UVC Keyboard', '-e', 'sh', '-c',
             'ros2 run uvc_robot_description uvc_keyboard\n'
             'status=$?\n'
             'if [ "$status" -ne 0 ]; then\n'
             '    printf "Keyboard exited with status %s. Press Enter to close.\\n" "$status"\n'
             '    read -r ignored\n'
             'fi\n'
             'exit "$status"'],
        name='uvc_keyboard_terminal',
        output='screen',
    )
    return [
        RegisterEventHandler(OnProcessExit(
            target_action=keyboard,
            on_exit=[Shutdown(reason='UVC keyboard terminal exited')],
        )),
        keyboard,
    ]


def generate_launch_description():
    package = Path(get_package_share_directory('yahboom_rosmaster_gazebo'))
    return LaunchDescription([
        DeclareLaunchArgument(
            'headless', default_value='false',
            description='Run the Gazebo server without opening its GUI.'),
        DeclareLaunchArgument(
            'terminal', default_value='x-terminal-emulator',
            description='Desktop terminal executable supporting -T and -e.'),
        DeclareLaunchArgument(
            'ros_domain_id', default_value='78',
            description='ROS domain for this desktop simulation.'),
        DeclareLaunchArgument(
            'gz_partition', default_value='uvc_operating_theatre',
            description='Gazebo Transport partition for this desktop simulation.'),
        SetEnvironmentVariable('ROS_DOMAIN_ID', LaunchConfiguration('ros_domain_id')),
        SetEnvironmentVariable('GZ_PARTITION', LaunchConfiguration('gz_partition')),
        OpaqueFunction(function=start_desktop_session),
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(str(package / 'launch/operating_theatre.launch.py')),
            launch_arguments={'headless': LaunchConfiguration('headless')}.items(),
        ),
    ])
