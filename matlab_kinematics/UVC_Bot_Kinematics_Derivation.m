%% UV-C BOT | FORWARD AND INVERSE KINEMATICS DERIVATION
% Differential-drive mobile robot
% Symbolic derivation from wheel rolling constraints
%
% Assumptions:
% 1. Two independently driven rear wheels
% 2. Pure rolling without slipping
% 3. Flat ground
% 4. Symmetric wheel positions about axle midpoint
% 5. Passive front casters
%
% Required: MATLAB Symbolic Math Toolbox

clear; clc; close all;
p = uvc_parameters();
% URDF conventions: +X forward, +Y left, +Z yaw.
% Wheel joint rates are positive forward; planar axle-frame correction is zero.

%% 1. DEFINE SYMBOLIC VARIABLES

syms r L real positive
syms wL wR v Omega real
syms vL vR real

fprintf('============================================\n');
fprintf('  UV-C BOT DIFFERENTIAL DRIVE KINEMATICS\n');
fprintf('============================================\n\n');

%% 2. DERIVE WHEEL ROLLING CONSTRAINTS

% Pure rolling relates wheel angular and linear velocities.

eqWheelLeft  = vL == r*wL;
eqWheelRight = vR == r*wR;

fprintf('STEP 1: Wheel rolling constraints\n');
disp(eqWheelLeft);
disp(eqWheelRight);

%% 3. DERIVE ROBOT BODY VELOCITY RELATIONSHIPS

% At the axle midpoint, each wheel has a contribution
% from the robot's translation and rotation.
%
% vL = v - (L/2)*Omega
% vR = v + (L/2)*Omega

eqLeft  = r*wL == v - (L/2)*Omega;
eqRight = r*wR == v + (L/2)*Omega;

fprintf('\nSTEP 2: Robot body rolling equations\n');
disp(eqLeft);
disp(eqRight);

%% 4. DERIVE FORWARD KINEMATICS SYMBOLICALLY

% Solve the two rolling equations for the unknown
% robot linear and angular velocities.

fk = solve([eqLeft, eqRight], [v, Omega]);

v_forward = simplify(fk.v);
Omega_forward = simplify(fk.Omega);

fprintf('\nSTEP 3: DERIVED FORWARD KINEMATICS\n');

fprintf('Linear velocity v = \n');
pretty(v_forward);

fprintf('Angular velocity Omega = \n');
pretty(Omega_forward);

% Expected:
% v = r*(wL+wR)/2
% Omega = r*(wR-wL)/L

%% 5. DERIVE INVERSE KINEMATICS SYMBOLICALLY

% Solve the rolling equations for wheel speeds.

ik = solve([eqLeft, eqRight], [wL, wR]);

wL_inverse = simplify(ik.wL);
wR_inverse = simplify(ik.wR);

fprintf('\nSTEP 4: DERIVED INVERSE KINEMATICS\n');

fprintf('Left wheel angular velocity = \n');
pretty(wL_inverse);

fprintf('Right wheel angular velocity = \n');
pretty(wR_inverse);

% Expected:
% wL = (v-L*Omega/2)/r
% wR = (v+L*Omega/2)/r

%% 6. DERIVE KINEMATIC MATRICES

% Forward kinematics Jacobian:
% [v; Omega] = J*[wL; wR]

J = simplify(jacobian( ...
    [v_forward; Omega_forward], [wL; wR]));

fprintf('\nSTEP 5: FORWARD KINEMATICS MATRIX\n');
disp(J);

% Inverse kinematics matrix:
% [wL; wR] = Jinv*[v; Omega]

Jinv = simplify(jacobian( ...
    [wL_inverse; wR_inverse], [v; Omega]));

fprintf('\nSTEP 6: INVERSE KINEMATICS MATRIX\n');
disp(Jinv);

fprintf('\nMatrix consistency J*Jinv:\n');
disp(simplify(J*Jinv));

%% 7. SUBSTITUTE ACTUAL UV-C ROBOT PARAMETERS

wheelRadius = p.r;          % metres, read from URDF
wheelSeparation = p.L;      % joint-centre separation, read from URDF

J_numeric = double(subs(J, ...
    [r, L], [wheelRadius, wheelSeparation]));

Jinv_numeric = double(subs(Jinv, ...
    [r, L], [wheelRadius, wheelSeparation]));

fprintf('\nSTEP 7: UV-C BOT NUMERICAL MATRICES\n');

fprintf('Forward kinematics matrix:\n');
disp(J_numeric);

fprintf('Inverse kinematics matrix:\n');
disp(Jinv_numeric);

%% 8. FORWARD KINEMATICS EXAMPLE

rpmLeft = 15;
rpmRight = 27;

omegaLeft = rpmLeft*2*pi/60;
omegaRight = rpmRight*2*pi/60;

bodyVelocity = J_numeric*[omegaLeft; omegaRight];

fprintf('\nSTEP 8: FORWARD KINEMATICS EXAMPLE\n');

fprintf('Left wheel speed: %.2f RPM\n',rpmLeft);
fprintf('Right wheel speed: %.2f RPM\n',rpmRight);
fprintf('Robot linear velocity: %.6f m/s\n',bodyVelocity(1));
fprintf('Robot yaw rate: %.6f rad/s\n',bodyVelocity(2));

%% 9. INVERSE KINEMATICS EXAMPLE

desiredLinearVelocity = 0.15;
desiredYawRate = 0.20;

wheelSpeeds = Jinv_numeric*[ ...
    desiredLinearVelocity; desiredYawRate];

rpm = wheelSpeeds*60/(2*pi);

fprintf('\nSTEP 9: INVERSE KINEMATICS EXAMPLE\n');

fprintf('Desired linear speed: %.4f m/s\n', ...
    desiredLinearVelocity);
fprintf('Desired yaw rate: %.4f rad/s\n', ...
    desiredYawRate);

fprintf('Required left wheel: %.4f RPM\n',rpm(1));
fprintf('Required right wheel: %.4f RPM\n',rpm(2));

%% 10. VERIFY DERIVED MATRICES

assert(isequal(simplify(J*Jinv),sym(eye(2))));

recoveredBodyVelocity = J_numeric*wheelSpeeds;

assert(norm(recoveredBodyVelocity - ...
    [desiredLinearVelocity; desiredYawRate]) < 1e-10);

fprintf('\nALGEBRAIC CONSISTENCY CHECK: PASSED (not measured simulation validation)\n');
