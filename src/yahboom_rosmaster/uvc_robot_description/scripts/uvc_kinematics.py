"""URDF-backed differential-drive geometry, using +X forward and +Z left yaw."""
from dataclasses import dataclass
import math
import xml.etree.ElementTree as ET


@dataclass(frozen=True)
class DriveGeometry:
    radius: float
    separation: float
    max_linear: float
    max_angular: float

    @classmethod
    def from_urdf(cls, filename):
        robot = ET.parse(filename).getroot()
        joints = [robot.find(f"joint[@name='{side}_wheel_joint']") for side in ('left', 'right')]
        if any(j is None for j in joints):
            raise ValueError('Both drive-wheel joints are required')
        positions = []
        radii = []
        for side, joint in zip(('left', 'right'), joints):
            axis = tuple(map(float, joint.find('axis').get('xyz').split()))
            if joint.get('type') != 'continuous' or axis != (0., 1., 0.):
                raise ValueError('Expected continuous wheels with positive Y joint axes')
            if joint.find('parent').get('link') != 'base_link':
                raise ValueError('Expected wheel origins in base_link')
            positions.append(tuple(map(float, joint.find('origin').get('xyz').split())))
            cylinder = robot.find(f"link[@name='{side}_wheel_link']/collision/geometry/cylinder")
            radii.append(float(cylinder.get('radius')))
        left, right = positions
        if not (math.isclose(left[0], right[0], abs_tol=1e-9)
                and math.isclose(left[2], right[2], abs_tol=1e-9)
                and left[1] > right[1]
                and math.isclose(radii[0], radii[1], abs_tol=1e-9)):
            raise ValueError('Expected equal-radius wheels on a common transverse axle')
        separation = left[1] - right[1]
        if not (math.isclose((left[0]+right[0])/2, 0., abs_tol=1e-9)
                and math.isclose((left[1]+right[1])/2, 0., abs_tol=1e-9)):
            raise ValueError('Expected the base origin horizontally at the axle midpoint')
        plugin = robot.find("gazebo/plugin[@name='gz::sim::systems::DiffDrive']")
        if plugin is None:
            raise ValueError('Missing Gazebo DiffDrive configuration')
        if not (math.isclose(float(plugin.findtext('wheel_radius')), radii[0], abs_tol=1e-9)
                and math.isclose(float(plugin.findtext('wheel_separation')), separation, abs_tol=1e-9)):
            raise ValueError('Gazebo drive geometry disagrees with URDF wheel geometry')
        if radii[0] <= 0 or separation <= 0:
            raise ValueError('Wheel radius and separation must be positive')
        return cls(radii[0], separation, float(plugin.findtext('max_linear_velocity')),
                   float(plugin.findtext('max_angular_velocity')))

    def inverse(self, linear, yaw):
        """Body m/s, rad/s -> left/right joint rad/s (positive rolls forward)."""
        return ((linear - self.separation * yaw / 2) / self.radius,
                (linear + self.separation * yaw / 2) / self.radius)

    def forward(self, left, right):
        return (self.radius * (left + right) / 2,
                self.radius * (right - left) / self.separation)


KEY_DIRECTIONS = {
    'w': (1, 0), 's': (-1, 0), 'a': (0, 1), 'd': (0, -1),
    'q': (1, 1), 'e': (1, -1), 'z': (-1, 1), 'c': (-1, -1),
    ' ': (0, 0), 'x': (0, 0),
}


def command_for_key(key, linear_speed, angular_speed):
    direction = KEY_DIRECTIONS.get(key.lower())
    return None if direction is None else (direction[0] * linear_speed, direction[1] * angular_speed)


def active_command(command, last_key_time, now, timeout):
    return command if now - last_key_time < timeout else (0., 0.)
