#!/usr/bin/env python3
"""Keyboard motion checks with URDF-derived expected and measured wheel speeds."""
import argparse
import math
from pathlib import Path
import select
import signal
import sys
import termios
import time
import tty

from ament_index_python.packages import get_package_share_directory
import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy
from rclpy.signals import SignalHandlerOptions
from geometry_msgs.msg import Twist
from nav_msgs.msg import Odometry
from sensor_msgs.msg import JointState
from uvc_kinematics import DriveGeometry, active_command, command_for_key


class KeyboardMotion(Node):
    def __init__(self, geometry):
        super().__init__('uvc_keyboard_motion')
        self.geometry = geometry
        self.publisher = self.create_publisher(Twist, '/cmd_vel', 10)
        self.wheels = None
        self.odom = None
        self.wheel_time = self.odom_time = 0.
        qos = QoSProfile(depth=1, reliability=ReliabilityPolicy.BEST_EFFORT)
        self.create_subscription(JointState, '/joint_states', self.joints, qos)
        self.create_subscription(Odometry, '/odom', self.odometry, qos)

    def joints(self, message):
        velocities = dict(zip(message.name, message.velocity))
        if all(name in velocities for name in ('left_wheel_joint', 'right_wheel_joint')):
            self.wheels = (velocities['left_wheel_joint'], velocities['right_wheel_joint'])
            self.wheel_time = time.monotonic()

    def odometry(self, message):
        self.odom = (message.twist.twist.linear.x, message.twist.twist.angular.z)
        self.odom_time = time.monotonic()

    def send(self, command):
        message = Twist()
        message.linear.x, message.angular.z = command
        self.publisher.publish(message)

    def status(self, command, now):
        expected = self.geometry.inverse(*command)
        rpm = lambda speeds: f'L {speeds[0]*30/math.pi:+6.2f}, R {speeds[1]*30/math.pi:+6.2f} RPM'
        measured = rpm(self.wheels) if self.wheels and now-self.wheel_time < 1 else 'waiting/stale'
        odom = (f'v {self.odom[0]:+.3f} m/s, yaw {self.odom[1]:+.3f} rad/s'
                if self.odom and now-self.odom_time < 1 else 'waiting/stale')
        reconstructed = self.geometry.forward(*self.wheels) if self.wheels else None
        reconstruction = (f'{reconstructed[0]:+.3f} m/s, {reconstructed[1]:+.3f} rad/s'
                          if reconstructed and now-self.wheel_time < 1 else 'waiting/stale')
        return (f'Command: v {command[0]:+.3f} m/s, yaw {command[1]:+.3f} rad/s\n'
                f'Equation target: {rpm(expected)}\nWheel feedback:  {measured}\n'
                f'Wheel-derived v/yaw: {reconstruction}\nOdometry: {odom}\n'
                'Targets are steady-state; acceleration/slip can cause transient differences.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--linear-speed', type=float, default=.15)
    parser.add_argument('--angular-speed', type=float, default=.4)
    parser.add_argument('--key-timeout', type=float, default=.4)
    options, ros_args = parser.parse_known_args()
    if not sys.stdin.isatty():
        parser.error('Run in an interactive terminal, e.g. docker exec -it or the tmux teleop window')
    path = Path(get_package_share_directory('uvc_robot_description')) / 'urdf/robots/uvc_robot.urdf'
    geometry = DriveGeometry.from_urdf(path)
    if not (0 < options.linear_speed <= geometry.max_linear
            and 0 < options.angular_speed <= geometry.max_angular
            and math.isfinite(options.key_timeout) and options.key_timeout > 0):
        parser.error('Use positive finite speeds within the URDF drive limits and a positive timeout')
    # Keep DDS alive during Ctrl+C so the finally block can publish a stop.
    rclpy.init(args=ros_args, signal_handler_options=SignalHandlerOptions.NO)
    node = KeyboardMotion(geometry)
    # Allow DDS discovery before sending any command. An idle second instance
    # would otherwise publish zeros over the active controller's commands.
    discovery_deadline = time.monotonic() + 1.0
    while rclpy.ok() and time.monotonic() < discovery_deadline:
        rclpy.spin_once(node, timeout_sec=.05)
    if node.count_publishers(node.publisher.topic_name) > 1:
        node.get_logger().error(
            'Another publisher is already using ' + node.publisher.topic_name
            + '. Close the other keyboard/teleop controller and run only one. '
              'This instance has not sent any velocity commands.')
        node.destroy_node()
        rclpy.shutdown()
        return 2
    fd = sys.stdin.fileno()
    original = termios.tcgetattr(fd)
    command, last_key = (0., 0.), float('-inf')
    last_send = last_display = 0.
    def request_stop(_signum, _frame):
        raise KeyboardInterrupt
    signal.signal(signal.SIGTERM, request_stop)
    signal.signal(signal.SIGHUP, request_stop)
    try:
        tty.setcbreak(fd)
        while rclpy.ok():
            if select.select([sys.stdin], [], [], .02)[0]:
                key = sys.stdin.read(1)
                if key in ('\x03', '\x1b', ''):
                    break
                requested = command_for_key(key, options.linear_speed, options.angular_speed)
                if requested is not None:
                    command, last_key = requested, time.monotonic()
            rclpy.spin_once(node, timeout_sec=0.)
            now = time.monotonic()
            current = active_command(command, last_key, now, options.key_timeout)
            if now-last_send >= .05:
                node.send(current)
                last_send = now
            if now-last_display >= .2:
                print('\x1b[2J\x1b[H'
                      f'UVC URDF: radius {geometry.radius:.5f} m, track {geometry.separation:.5f} m\n'
                      'W/S forward/back | A/D rotate left/right | Q/E forward curves\n'
                      'Z/C reverse curves | Space/X stop | Esc or Ctrl+C quit\n'
                      f'Hold/repeat a movement key; stops after {options.key_timeout:.2f}s without a key.\n'
                      'Use only this velocity publisher while testing.\n\n'
                      + node.status(current, now), flush=True)
                last_display = now
    except (KeyboardInterrupt, rclpy.executors.ExternalShutdownException):
        pass
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, original)
        if rclpy.ok():
            # Repeat the stop while DDS is alive before destroying the publisher.
            for _ in range(5):
                node.send((0., 0.))
                rclpy.spin_once(node, timeout_sec=.02)
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    sys.exit(main())
