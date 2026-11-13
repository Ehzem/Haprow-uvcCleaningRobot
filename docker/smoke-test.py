#!/usr/bin/env python3
"""Verify installed packages, robot spawn, sensors, and forward motion in the image."""
import json
import math
import os
from pathlib import Path
import signal
import subprocess
import time

import rclpy
from geometry_msgs.msg import Twist
from nav_msgs.msg import Odometry
from rclpy.qos import qos_profile_sensor_data
from sensor_msgs.msg import Imu, JointState, LaserScan


def main():
    log_path = Path('/tmp/addison-smoke-test.log')
    log = log_path.open('w')
    process = subprocess.Popen(
        ['ros2', 'launch', 'yahboom_rosmaster_gazebo', 'operating_theatre.launch.py',
         'headless:=true', 'use_rviz:=false'],
        stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
    rclpy.init()
    node = rclpy.create_node('container_smoke_test')
    publisher = node.create_publisher(Twist, '/cmd_vel', 10)
    messages = dict(odom=0, scan=0, joints=0, imu=0)
    latest = {}

    def odom(msg):
        messages['odom'] += 1
        latest['xy'] = (msg.pose.pose.position.x, msg.pose.pose.position.y)

    def scan(msg):
        messages['scan'] += 1
        latest['finite_ranges'] = sum(math.isfinite(v) for v in msg.ranges)

    def count(name):
        def callback(_msg):
            messages[name] += 1
        return callback

    subscriptions = [
        node.create_subscription(Odometry, '/odom', odom, 10),
        node.create_subscription(LaserScan, '/scan', scan, qos_profile_sensor_data),
        node.create_subscription(JointState, '/joint_states', count('joints'), 10),
        node.create_subscription(Imu, '/imu/data', count('imu'), qos_profile_sensor_data),
    ]
    try:
        deadline = time.monotonic() + 45
        while not (all(messages.values()) and latest.get('finite_ranges', 0)):
            if process.poll() is not None:
                raise RuntimeError('Simulation exited before sensors were ready')
            if time.monotonic() > deadline:
                raise RuntimeError(f'Timed out waiting for topics: {messages}')
            rclpy.spin_once(node, timeout_sec=.1)
        start = latest['xy']
        command = Twist()
        command.linear.x = .10
        deadline = time.monotonic() + 12
        distance = 0.0
        while distance < .08 and time.monotonic() < deadline:
            publisher.publish(command)
            rclpy.spin_once(node, timeout_sec=.05)
            distance = math.dist(start, latest['xy'])
        publisher.publish(Twist())
        if distance < .02:
            raise RuntimeError(f'Forward command did not move odometry: {distance:.4f} m')
        pose = subprocess.run(['gz', 'model', '-m', 'uvc_robot', '--pose'],
                              text=True, capture_output=True, timeout=8, check=True)
        if 'uvc_robot' not in pose.stdout:
            raise RuntimeError('Robot not present in the Gazebo scene')
        print(json.dumps({'result': 'PASS', 'messages': messages,
                          'distance_m': round(distance, 3),
                          'finite_lidar_ranges': latest['finite_ranges'],
                          'gazebo_pose': pose.stdout.strip()}, indent=2))
    except Exception:
        log.flush()
        print(log_path.read_text()[-12000:])
        raise
    finally:
        publisher.publish(Twist())
        node.destroy_node()
        rclpy.shutdown()
        if process.poll() is None:
            os.killpg(process.pid, signal.SIGINT)
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
        log.close()


if __name__ == '__main__':
    main()
