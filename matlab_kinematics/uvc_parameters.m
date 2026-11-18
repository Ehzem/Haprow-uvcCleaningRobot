function p = uvc_parameters(urdfPath)
% Read the actual Gazebo robot, rather than keeping independent CAD constants.
% +X forward, +Y left, +Z up. Positive wheel joint rates roll forward.
if nargin == 0
    root = fileparts(mfilename('fullpath'));
    urdfPath = fullfile(root,'..','src','yahboom_rosmaster', ...
        'uvc_robot_description','urdf','robots','uvc_robot.urdf');
end
assert(isfile(urdfPath),'URDF not found: %s',urdfPath);
doc = xmlread(urdfPath);
p.urdf = urdfPath;
left = named(doc,'joint','left_wheel_joint');
right = named(doc,'joint','right_wheel_joint');
p.left = vec(first(left,'origin'),'xyz');
p.right = vec(first(right,'origin'),'xyz');
assert(isequal(vec(first(left,'axis'),'xyz'),[0 1 0]) && ...
    isequal(vec(first(right,'axis'),'xyz'),[0 1 0]),'Expected +Y wheel axes');
lc = first(named(doc,'link','left_wheel_link'),'collision');
rc = first(named(doc,'link','right_wheel_link'),'collision');
p.r = str2double(char(first(lc,'cylinder').getAttribute('radius')));
rr = str2double(char(first(rc,'cylinder').getAttribute('radius')));
p.width = str2double(char(first(lc,'cylinder').getAttribute('length')));
p.L = p.left(2)-p.right(2);
assert(p.r>0 && p.L>0 && abs(p.r-rr)<1e-10,'Invalid/equal-radius drive geometry');
assert(abs(p.left(1)-p.right(1))<1e-10 && ...
    abs(p.left(3)-p.right(3))<1e-10,'Wheels must share a transverse axle');
p.axle = (p.left+p.right)/2;
assert(norm(p.axle(1:2))<1e-10,'Expected base origin below drive axle');
p.planarFrameCorrection = [0 0];
p.leftCaster = vec(first(named(doc,'joint','left_caster_joint'),'origin'),'xyz');
p.rightCaster = vec(first(named(doc,'joint','right_caster_joint'),'origin'),'xyz');
p.casterRadius = str2double(char(first(first(named(doc,'link','left_caster_link'), ...
    'collision'),'sphere').getAttribute('radius')));
p.chassisCenter = vec(first(named(doc,'collision','chassis'),'origin'),'xyz');
p.chassisSize = vec(first(named(doc,'collision','chassis'),'box'),'size');
plugin = named(doc,'plugin','gz::sim::systems::DiffDrive');
assert(abs(number(plugin,'wheel_radius')-p.r)<1e-10 && ...
    abs(number(plugin,'wheel_separation')-p.L)<1e-10,'Gazebo/URDF geometry mismatch');
p.maxLinear = number(plugin,'max_linear_velocity');
p.maxAngular = number(plugin,'max_angular_velocity');
masses = doc.getElementsByTagName('mass');
p.mass = 0;
for k=0:masses.getLength-1
    p.mass = p.mass+str2double(char(masses.item(k).getAttribute('value')));
end
p.J = [p.r/2 p.r/2; -p.r/p.L p.r/p.L];
p.Jinv = [1/p.r -p.L/(2*p.r); 1/p.r p.L/(2*p.r)];
assert(norm(p.J*p.Jinv-eye(2),'fro')<1e-10,'FK/IK matrices do not invert');
end

function node = named(doc,tag,name)
items = doc.getElementsByTagName(tag);
for k=0:items.getLength-1
    node = items.item(k);
    if strcmp(char(node.getAttribute('name')),name), return; end
end
error('Missing %s named %s',tag,name);
end
function node = first(parent,tag)
items = parent.getElementsByTagName(tag);
assert(items.getLength>0,'Missing XML element %s',tag);
node = items.item(0);
end
function values = vec(node,attribute)
values = sscanf(char(node.getAttribute(attribute)),'%f').';
assert(numel(values)==3,'Expected three coordinates in %s',attribute);
end
function value = number(parent,tag)
value = str2double(char(first(parent,tag).getTextContent));
assert(isfinite(value),'Invalid number in %s',tag);
end
