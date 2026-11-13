#!/usr/bin/env python3
"""Import the supplied centimetre-scale UVC OBJ/MTL into a movable ROS/Gazebo URDF.

Usage: python3 import_uvc_robot.py '/path/UVC Robot V 67.obj'
The measured drive axle defines the origin; +X is CAD front, +Y is left.
Drive dimensions come from the CAD. Mass, inertia and collision proxies are
simulation approximations, not manufacturer specifications.
"""
import argparse
from collections import defaultdict
from pathlib import Path
import struct
import xml.etree.ElementTree as E


def add(parent, tag, text=None, **attrs):
    n = E.SubElement(parent, tag, attrs)
    if text is not None:
        n.text = str(text)
    return n


def xml(path, root):
    E.indent(root)
    E.ElementTree(root).write(path, encoding='utf-8', xml_declaration=True)


def inertial(link, mass, xyz, inertia):
    n = add(link, 'inertial')
    add(n, 'origin', xyz=xyz)
    add(n, 'mass', value=str(mass))
    add(n, 'inertia', ixx=str(inertia[0]), iyy=str(inertia[1]), izz=str(inertia[2]),
        ixy='0', ixz='0', iyz='0')


def box(link, name, xyz, size):
    n = add(link, 'collision', name=name)
    add(n, 'origin', xyz=xyz)
    add(add(n, 'geometry'), 'box', size=size)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    args = parser.parse_args()
    package = Path(__file__).resolve().parents[1]
    out = package
    mesh_dir = package / 'meshes/uvc_robot'
    urdf_dir = package / 'urdf/robots'
    urdf_dir.mkdir(parents=True, exist_ok=True)
    mesh_dir.mkdir(parents=True, exist_ok=True)
    colours = {}
    material = None
    for line in args.source.with_suffix('.mtl').read_text().splitlines():
        s = line.split()
        if not s:
            continue
        if s[0] == 'newmtl':
            material = s[1]
        elif s[0] == 'Kd':
            colours[material] = ' '.join(s[1:4]) + ' 1'
    vertices, faces = [], defaultdict(list)
    group, material = 'body', None
    for line in args.source.read_text().splitlines():
        s = line.split()
        if not s:
            continue
        if s[0] == 'v':
            x, y, z = map(float, s[1:4])
            vertices.append((y*.01 + .11608, -(x*.01 + .06246), z*.01 - .01971))
        elif s[0] == 'g':
            group = s[1]
        elif s[0] == 'usemtl':
            material = s[1]
        elif s[0] == 'f':
            ids = [int(v.split('/')[0]) for v in s[1:]]
            ids = [i-1 if i > 0 else len(vertices)+i for i in ids]
            for i in range(1, len(ids)-1):
                faces[group, material].append((ids[0], ids[i], ids[i+1]))
    robot = E.Element('robot', name='uvc_robot')
    add(robot, 'link', name='base_footprint')
    base = add(robot, 'link', name='base_link')
    joint = add(robot, 'joint', name='base_joint', type='fixed')
    add(joint, 'parent', link='base_footprint')
    add(joint, 'child', link='base_link')
    inertial(base, 35, '0.10 0 0.45', (9.5, 9.7, 1.8))
    box(base, 'chassis', '0.10886 0 0.19315', '0.50 0.40 0.28')
    box(base, 'bumpers', '0.10886 0 0.17315', '0.524 0.424 0.04')
    box(base, 'tower_and_lamps', '0.10886 0 1.03', '0.29 0.29 1.49')
    box(base, 'sensor_platform', '0.124 0 1.84065', '0.33 0.30 0.115')
    links = {'base_link': base}
    origins = {'base_link': (0, 0, 0)}
    for side, y in [('left', .16022), ('right', -.16022)]:
        name = f'{side}_wheel_link'
        link = add(robot, 'link', name=name)
        links[name], origins[name] = link, (0, y, .08)
        inertial(link, 1, '0 0 0', (.0019, .0032, .0019))
        collision = add(link, 'collision')
        add(collision, 'origin', rpy='1.57079632679 0 0')
        add(add(collision, 'geometry'), 'cylinder', radius='.08', length='.0613')
        joint = add(robot, 'joint', name=f'{side}_wheel_joint', type='continuous')
        add(joint, 'parent', link='base_link')
        add(joint, 'child', link=name)
        add(joint, 'origin', xyz=f'0 {y} .08')
        add(joint, 'axis', xyz='0 1 0')
        add(joint, 'limit', effort='25', velocity='10')
        gazebo = add(robot, 'gazebo', reference=name)
        add(gazebo, 'mu1', '1.0')
        add(gazebo, 'mu2', '1.0')
    # Low-friction sphere contacts approximate the two freely swivelling casters.
    for side, y in [('left', .15), ('right', -.15)]:
        name = f'{side}_caster_link'
        link = add(robot, 'link', name=name)
        inertial(link, .2, '0 0 0', (.0001, .0001, .0001))
        add(add(add(link, 'collision'), 'geometry'), 'sphere', radius='.03725')
        joint = add(robot, 'joint', name=f'{side}_caster_joint', type='fixed')
        add(joint, 'parent', link='base_link')
        add(joint, 'child', link=name)
        add(joint, 'origin', xyz=f'.274 {y} .03725')
        # Preserve this link so its low contact friction stays separate.
        gj = add(robot, 'gazebo', reference=f'{side}_caster_joint')
        add(gj, 'preserveFixedJoint', 'true')
        gazebo = add(robot, 'gazebo', reference=name)
        add(gazebo, 'mu1', '0.01')
        add(gazebo, 'mu2', '0.01')
    for index, ((group, material), triangles) in enumerate(faces.items()):
        link_name = {'Wheel_Drive_Left': 'left_wheel_link',
                     'Wheel_Drive_Right': 'right_wheel_link'}.get(group, 'base_link')
        offset = origins[link_name]
        filename = f'part_{index:02d}.stl'
        with (mesh_dir / filename).open('wb') as f:
            f.write(f'{group}; metres; ROS axes'.encode()[:80].ljust(80, b'\0'))
            f.write(struct.pack('<I', len(triangles)))
            for tri in triangles:
                a, b, c = [tuple(vertices[i][k]-offset[k] for k in range(3)) for i in tri]
                u, v = [b[k]-a[k] for k in range(3)], [c[k]-a[k] for k in range(3)]
                n = (u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0])
                length = sum(x*x for x in n)**.5 or 1
                f.write(struct.pack('<12fH', *(x/length for x in n), *a, *b, *c, 0))
        visual = add(links[link_name], 'visual', name=f'cad_part_{index:02d}')
        add(add(visual, 'geometry'), 'mesh',
            filename=f'package://uvc_robot_description/meshes/uvc_robot/{filename}')
        mat = add(visual, 'material', name=f'material_{index}')
        add(mat, 'color', rgba=colours.get(material, '.7 .7 .7 1'))
    # Simulation-only range sensor; placed just ahead of the chassis for visibility.
    for frame, xyz in [('laser_frame', '.39 0 .34'), ('imu_link', '.10 0 .20')]:
        link = add(robot, 'link', name=frame)
        inertial(link, .01, '0 0 0', (1e-5, 1e-5, 1e-5))
        joint = add(robot, 'joint', name=frame+'_joint', type='fixed')
        add(joint, 'parent', link='base_link')
        add(joint, 'child', link=frame)
        add(joint, 'origin', xyz=xyz)
    lidar = add(add(robot, 'gazebo', reference='laser_frame'), 'sensor', name='uvc_lidar', type='gpu_lidar')
    add(lidar, 'topic', '/scan')
    add(lidar, 'gz_frame_id', 'laser_frame')
    add(lidar, 'update_rate', '10')
    add(lidar, 'always_on', 'true')
    ray = add(lidar, 'ray')
    h = add(add(ray, 'scan'), 'horizontal')
    for tag, value in [('samples','720'), ('resolution','1'), ('min_angle','-3.14159265'), ('max_angle','3.14159265')]:
        add(h, tag, value)
    rg = add(ray, 'range')
    for tag, value in [('min','.12'), ('max','20'), ('resolution','.01')]:
        add(rg, tag, value)
    imu = add(add(robot, 'gazebo', reference='imu_link'), 'sensor', name='uvc_imu', type='imu')
    add(imu, 'topic', '/imu/data')
    add(imu, 'gz_frame_id', 'imu_link')
    add(imu, 'update_rate', '50')
    add(imu, 'always_on', 'true')
    gazebo = add(robot, 'gazebo')
    drive = add(gazebo, 'plugin', filename='gz-sim-diff-drive-system', name='gz::sim::systems::DiffDrive')
    for tag, value in [('left_joint','left_wheel_joint'), ('right_joint','right_wheel_joint'),
                       ('wheel_separation','.32044'), ('wheel_radius','.08'),
                       ('topic','/cmd_vel'), ('odom_topic','/odom'), ('tf_topic','/tf'),
                       ('frame_id','odom'), ('child_frame_id','base_footprint'),
                       ('odom_publish_frequency','30'), ('max_linear_velocity','.3'),
                       ('max_angular_velocity','1'), ('max_linear_acceleration','.5')]:
        add(drive, tag, value)
    state = add(gazebo, 'plugin', filename='gz-sim-joint-state-publisher-system', name='gz::sim::systems::JointStatePublisher')
    add(state, 'topic', '/joint_states')
    add(state, 'joint_name', 'left_wheel_joint')
    add(state, 'joint_name', 'right_wheel_joint')
    xml(urdf_dir / 'uvc_robot.urdf', robot)
    print(f'Imported {len(faces)} coloured mesh parts into {out}')


if __name__ == '__main__':
    main()
