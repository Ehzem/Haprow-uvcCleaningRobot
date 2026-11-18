%% UV-C BOT | READ ACTUAL URDF GEOMETRY AND MASS
clearvars; clc;
root=fileparts(mfilename('fullpath')); addpath(root); p=uvc_parameters();
fprintf('URDF: %s\n',p.urdf);
fprintf('Left drive joint [x y z] mm:  [%.3f %.3f %.3f]\n',p.left*1000);
fprintf('Right drive joint [x y z] mm: [%.3f %.3f %.3f]\n',p.right*1000);
fprintf('Drive track %.3f mm, radius %.3f mm, axial width %.3f mm\n', ...
    p.L*1000,p.r*1000,p.width*1000);
fprintf('Axle midpoint [x y z] mm: [%.3f %.3f %.3f]\n',p.axle*1000);
fprintf('Left caster [x y z] mm:  [%.3f %.3f %.3f]\n',p.leftCaster*1000);
fprintf('Right caster [x y z] mm: [%.3f %.3f %.3f]\n',p.rightCaster*1000);
fprintf('Caster sphere radius %.3f mm (fixed low-friction proxy, not swivel joints)\n',p.casterRadius*1000);
fprintf('Chassis collision centre [x y z] mm: [%.3f %.3f %.3f]\n',p.chassisCenter*1000);
fprintf('Chassis collision size [x y z] mm: [%.3f %.3f %.3f]\n',p.chassisSize*1000);
fprintf('Total explicitly assigned URDF mass %.3f kg (simulation approximation)\n',p.mass);
fprintf('Robot body centre is ahead of the axle; base origin is under the axle.\n');
fprintf('Forward +X, left +Y, yaw +Z; both joint axes +Y.\n');
fprintf('FK matrix:\n'); disp(p.J);
fprintf('IK matrix:\n'); disp(p.Jinv);
fprintf('Drive body limits: %.3f m/s, %.3f rad/s\n',p.maxLinear,p.maxAngular);
