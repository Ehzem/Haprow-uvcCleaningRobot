%% UV-C BOT | URDF-ALIGNED INVERSE KINEMATICS
% Analytic calculation; ROS keyboard monitor supplies measured Gazebo feedback.
clearvars; clc; close all;
root=fileparts(mfilename('fullpath')); addpath(root); p=uvc_parameters();
vDesired=.15;          % m/s, +X forward
omegaDesired=.20;      % rad/s, +Z left turn
stopTime=5; T=0;       % Constant input; optional mathematical shared ramp only
assert(abs(vDesired)<=p.maxLinear && abs(omegaDesired)<=p.maxAngular, ...
    'Requested body velocities exceed the Gazebo limits');
wheel=p.Jinv*[vDesired;omegaDesired];
omegaL=wheel(1); omegaR=wheel(2); % Already positive-forward URDF joint rates
rpm=wheel*60/(2*pi);
recovered=p.J*wheel;
assert(norm(recovered-[vDesired;omegaDesired])<1e-10,'IK/FK round trip failed');
fprintf('URDF r=%.5f m, L=%.5f m\n',p.r,p.L);
fprintf('Requested v=%.4f m/s, yaw=%+.4f rad/s\n',vDesired,omegaDesired);
fprintf('Joint rates L/R: %+.6f / %+.6f rad/s\n',omegaL,omegaR);
fprintf('Wheel RPM L/R: %+.6f / %+.6f\n',rpm(1),rpm(2));
fprintf('Recovered v/yaw: %.6f m/s, %+.6f rad/s\n',recovered(1),recovered(2));
t=linspace(0,stopTime,501).';
[x,y,heading]=uvc_path(t,vDesired,omegaDesired,T);
figure('Name','URDF inverse kinematics (ideal model)'); tiledlayout(1,2);
nexttile; plot(x,y,'LineWidth',1.4); axis equal; grid on;
xlabel('X forward (m)'); ylabel('Y left (m)'); title('Predicted axle-midpoint path');
nexttile; plot(t,rad2deg(heading),'LineWidth',1.4); grid on;
xlabel('Time (s)'); ylabel('Signed heading (deg)'); title('Predicted heading');
fprintf('No frame correction is applied: base_footprint is below the drive axle.\n');
fprintf('This verifies equations only, not tire slip, contact physics or measured accuracy.\n');
