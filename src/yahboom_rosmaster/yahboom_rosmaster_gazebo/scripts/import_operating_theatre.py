#!/usr/bin/env python3
"""Convert the supplied hospital 3MF into local, metre-scale Gazebo meshes.

Usage: python3 import_operating_theatre.py '/path/hospital assembly.3mf'
The assembly uses Y up; Gazebo uses Z up. The tiled floor top becomes z=0.
Only geometry and colour data are read from the source archive.
"""
import argparse
from pathlib import Path
import struct
import xml.etree.ElementTree as ET
import zipfile


def sub(parent, tag, text=None, **attrs):
    node = ET.SubElement(parent, tag, attrs)
    if text is not None:
        node.text = str(text)
    return node


def write_xml(path, root):
    ET.indent(root)
    ET.ElementTree(root).write(path, encoding='utf-8', xml_declaration=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    args = parser.parse_args()
    package = Path(__file__).resolve().parents[1]
    dest = package / 'models' / 'operating_theatre'
    (dest / 'meshes').mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(args.source) as archive:
        root = ET.fromstring(archive.read('3D/3dmodel.model'))
    ns = {'m': 'http://schemas.microsoft.com/3dmanufacturing/core/2015/02'}
    if root.get('unit') != 'millimeter':
        raise ValueError('Expected millimetre source geometry')
    groups = {e.get('id'): [c.get('color') for c in e]
              for e in root.iter() if e.tag.endswith('}colorgroup')}
    objects = {e.get('id'): e for e in root.findall('m:resources/m:object', ns)}
    sdf = ET.Element('sdf', version='1.9')
    model = sub(sdf, 'model', name='operating_theatre')
    sub(model, 'static', 'true')
    link = sub(model, 'link', name='structure')
    count = 0
    for item in root.findall('m:build/m:item', ns):
        if item.get('transform'):
            raise ValueError('This source unexpectedly has a build transform')
        oid = item.get('objectid')
        if oid == '129':  # Ceiling omitted for an unobstructed overview.
            continue
        obj = objects[oid]
        vertices = [((float(v.get('x')) + 671.933952) / 1000,
                     -float(v.get('z')) / 1000,
                     (float(v.get('y')) - 256.698576) / 1000)
                    for v in obj.findall('m:mesh/m:vertices/m:vertex', ns)]
        # This export's floor tiles span z=-0.009144 to z=0. Replace their
        # visuals and collisions with the single level floor below.
        heights = [v[2] for v in vertices]
        if abs(max(heights)) < 1e-6 and min(heights) >= -0.02:
            continue
        triangles = obj.findall('m:mesh/m:triangles/m:triangle', ns)
        mesh_path = dest / 'meshes' / f'body_{oid}.stl'
        with mesh_path.open('wb') as out:
            out.write(b'Operating theatre; metres; Z up'.ljust(80, b'\0'))
            out.write(struct.pack('<I', len(triangles)))
            for tri in triangles:
                points = [vertices[int(tri.get(k))] for k in ('v1', 'v2', 'v3')]
                a, b, c = points
                u = [b[i]-a[i] for i in range(3)]
                v = [c[i]-a[i] for i in range(3)]
                normal = (u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0])
                length = sum(n*n for n in normal)**0.5 or 1
                out.write(struct.pack('<12fH', *(n/length for n in normal),
                                      *a, *b, *c, 0))
        uri = f'model://operating_theatre/meshes/{mesh_path.name}'
        for kind in ('visual', 'collision'):
            node = sub(link, kind, name=f'body_{oid}_{kind}')
            mesh = sub(sub(node, 'geometry'), 'mesh')
            sub(mesh, 'uri', uri)
            if kind == 'visual':
                # STL has no embedded materials; retain each body's default 3MF colour.
                colour = groups[obj.get('pid')][int(obj.get('pindex', '0'))]
                rgba = ' '.join(str(int(colour[i:i+2], 16)/255) for i in (1,3,5,7))
                material = sub(node, 'material')
                sub(material, 'ambient', rgba)
                sub(material, 'diffuse', rgba)
        count += 1
    # A single flat surface at the highest original tile top (z=0).
    for kind in ('collision', 'visual'):
        node = sub(link, kind, name=f'continuous_floor_{kind}')
        sub(node, 'pose', '0 0 -0.025 0 0 0')
        sub(sub(sub(node, 'geometry'), 'box'), 'size', '8.3 6.0 0.05')
        if kind == 'visual':
            mat = sub(node, 'material')
            sub(mat, 'ambient', '0.55 0.65 0.65 1')
            sub(mat, 'diffuse', '0.55 0.65 0.65 1')
    write_xml(dest / 'model.sdf', sdf)
    config = ET.Element('model')
    sub(config, 'name', 'Operating theatre')
    sub(config, 'version', '1.0')
    sub(config, 'sdf', 'model.sdf', version='1.9')
    sub(config, 'description', 'Hospital CAD assembly, metres and Z up, open ceiling.')
    write_xml(dest / 'model.config', config)
    print(f'Imported {count} bodies into {dest}')


if __name__ == '__main__':
    main()
