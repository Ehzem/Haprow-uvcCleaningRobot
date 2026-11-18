%% UV-C BOT | URDF-ALIGNED FORWARD KINEMATICS
% Ideal no-slip examples. No unprovided .slx model or measured accuracy claims.
clearvars; clc; close all;
root=fileparts(mfilename('fullpath')); addpath(root); p=uvc_parameters();
% [v; yaw] = [r/2 r/2; -r/L r/L] * [left; right] in rad/s.
% +X forward, +Y left, +Z yaw; positive wheel-joint rates roll forward.
% Base origin is horizontally at the axle midpoint: no 135 mm correction.
cases = [27 27 10; -15 15 3; 15 27 5];
names = {'Straight forward','Positive in-place yaw','Forward left curve'};
T=0; % Constant wheel speed. Optional assumed shared ramp; not Gazebo dynamics.
figure('Name','URDF forward kinematics (ideal model)');
tiledlayout(1,3);
for k=1:size(cases,1)
    rpm=cases(k,1:2).';
    wheel=rpm*2*pi/60;
    body=p.J*wheel;
    assert(abs(body(1))<=p.maxLinear && abs(body(2))<=p.maxAngular, ...
        'Example exceeds Gazebo body-speed limits');
    t=linspace(0,cases(k,3),501).';
    [x,y,heading]=uvc_path(t,body(1),body(2),T);
    assert(norm(p.Jinv*body-wheel)<1e-10,'FK/IK round trip failed');
    nexttile; plot(x,y,'LineWidth',1.4); axis equal; grid on;
    xlabel('X forward (m)'); ylabel('Y left (m)'); title(names{k});
    fprintf('%s: L/R %.2f / %.2f RPM, v %.6f m/s, yaw %+.6f rad/s\n', ...
        names{k},rpm(1),rpm(2),body(1),body(2));
    fprintf('  Ideal final x/y/heading: %.4f m, %.4f m, %+.3f deg\n', ...
        x(end),y(end),rad2deg(heading(end)));
end
fprintf('Pure positive yaw uses left negative/right positive. Signs are not discarded.\n');
fprintf('These plots are predictions. Compare actual motion with the ROS keyboard monitor.\n');
