%% UV-C URDF preflight (no missing Simscape model or Windows paths required)
clearvars; clc;
root=fileparts(mfilename('fullpath')); addpath(root);
p=uvc_parameters();
fprintf('URDF: %s\n',p.urdf);
fprintf('Wheel radius %.5f m, separation %.5f m\n',p.r,p.L);
fprintf('Forward +X, left +Y, yaw +Z; positive wheel rates roll forward.\n');
fprintf('Planar axle correction: [%.3f %.3f] m\n',p.planarFrameCorrection);
fprintf('Run uvc_geometry_and_mass, uvc_forward_kinematics, and uvc_inverse_kinematics.\n');
fprintf('These are ideal mathematical checks, not measured Simscape validation.\n');
fprintf('For measured Gazebo feedback: ros2 run uvc_robot_description uvc_keyboard\n');
