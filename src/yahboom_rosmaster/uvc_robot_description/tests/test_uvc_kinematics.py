"""Verify command signs, wheel geometry, inverse/forward mappings and key timeout."""
from pathlib import Path
import math
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET

PACKAGE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PACKAGE / 'scripts'))
from uvc_kinematics import DriveGeometry, active_command, command_for_key


class KinematicsTests(unittest.TestCase):
    def setUp(self):
        self.urdf = PACKAGE / 'urdf/robots/uvc_robot.urdf'
        self.drive = DriveGeometry.from_urdf(self.urdf)

    def test_geometry_is_derived_from_actual_joints(self):
        self.assertAlmostEqual(self.drive.radius, .08)
        self.assertAlmostEqual(self.drive.separation, .32044)

    def test_positive_wheels_drive_forward_and_right_faster_turns_left(self):
        v, yaw = self.drive.forward(2., 2.)
        self.assertAlmostEqual(v, .16)
        self.assertAlmostEqual(yaw, 0.)
        v, yaw = self.drive.forward(-1., 1.)
        self.assertAlmostEqual(v, 0.)
        self.assertGreater(yaw, 0.)
        self.assertLess(self.drive.forward(1., -1.)[1], 0.)

    def test_inverse_forward_round_trip_for_reverse_spins_and_curves(self):
        for v, yaw in [(.15, .2), (-.15, 0.), (0., -.4), (.15, -.4), (-.15, .4), (0., 0.)]:
            actual = self.drive.forward(*self.drive.inverse(v, yaw))
            self.assertAlmostEqual(actual[0], v)
            self.assertAlmostEqual(actual[1], yaw)
        left, right = self.drive.inverse(.15, .2)
        self.assertAlmostEqual(left * 30/math.pi, 14.07996035, places=7)
        self.assertAlmostEqual(right * 30/math.pi, 21.72990185, places=7)

    def test_plugin_geometry_mismatch_is_rejected(self):
        tree = ET.parse(self.urdf)
        tree.find("gazebo/plugin[@name='gz::sim::systems::DiffDrive']/wheel_separation").text = '.338413'
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'bad.urdf'
            tree.write(path)
            with self.assertRaisesRegex(ValueError, 'disagrees'):
                DriveGeometry.from_urdf(path)

    def test_keyboard_and_deadman_commands(self):
        self.assertEqual(command_for_key('w', .15, .4), (.15, 0.))
        self.assertEqual(command_for_key('s', .15, .4), (-.15, 0.))
        self.assertEqual(command_for_key('a', .15, .4), (0., .4))
        self.assertEqual(command_for_key('d', .15, .4), (0., -.4))
        self.assertEqual(command_for_key('q', .15, .4), (.15, .4))
        self.assertEqual(command_for_key('e', .15, .4), (.15, -.4))
        self.assertEqual(command_for_key('x', .15, .4), (0., 0.))
        self.assertIsNone(command_for_key('?', .15, .4))
        self.assertEqual(active_command((.15, .4), 1., 1.2, .4), (.15, .4))
        self.assertEqual(active_command((.15, .4), 1., 1.5, .4), (0., 0.))


if __name__ == '__main__':
    unittest.main()
