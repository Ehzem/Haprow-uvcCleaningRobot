%% UV-C BOT | FORWARD KINEMATICS
% CAD-aligned wheel track: 338.413 mm. Joint rotation sign is negative for forward.
% Run sections individually with Ctrl+Enter, or run the whole file.
% Needs the packaged UVC_Bot_Validated.slx and its logged motion sensors.

%% 0. Initialize model and parameters
clearvars; clc; close all;
root = fileparts(mfilename('fullpath'));
addpath(root);
mdl = 'UVC_Bot_Validated';
load_system(fullfile(root,[mdl '.slx']));
r = 0.080;
L = 0.338413;
T = 0.3;
% All test cases modify only the in-memory model; no save_system is called.

%% 1. Straight-line forward motion | 27 / 27 RPM, 10 seconds
% Prescribed wheel speed profile: omega(t)=omega_ss*(1-exp(-t/T)).
rpmL = 27; rpmR = 27; duration = 10;
[outStraight, ts] = run_wheel_case(mdl,rpmL,rpmR,T,duration);
[tS,xS,yS,omS] = get_motion(outStraight);
% Reference-frame forward displacement is valid for a straight robot.
vSS = r*(rpmL+rpmR)*pi/60;
vs = vSS*(1-exp(-tS/T));
ys = vSS*(tS-T*(1-exp(-tS/T)));
yMeasured = yS-yS(1);
figure('Name','FK 01 Straight-line validation');
tiledlayout(2,1);
nexttile; plot(tS,gradient(yS,tS),'b',tS,vs,'r--','LineWidth',1.3); grid on;
xlabel('Time (s)'); ylabel('Forward speed (m/s)'); legend('Measured derivative','Ideal');
nexttile; plot(tS,yMeasured,'b',tS,ys,'r--','LineWidth',1.3); grid on;
xlabel('Time (s)'); ylabel('Forward displacement (m)'); legend('Measured','Ideal');
fprintf('\nSTRAIGHT: expected %.4f m, measured %.4f m, error %.2f %%, runtime %.2f s\n', ...
    ys(end),yMeasured(end),100*abs(yMeasured(end)-ys(end))/max(abs(ys(end)),eps),ts);

%% 2. In-place rotation | +27 / -27 joint RPM, 3 seconds
% Physical forward-rolling RPM: left -27, right +27.
rpmL = -27; rpmR = 27; duration = 3;
[outSpin, tr] = run_wheel_case(mdl,rpmL,rpmR,T,duration);
[tR,xR,yR,omegaR] = get_motion(outSpin);
% Sensor omega Z may be sign-convention-dependent: compare magnitude for spin.
thetaR = cumtrapz(tR,omegaR);
spinSS = r*(2*27*2*pi/60)/L;
thetaIdealR = spinSS*(tR-T*(1-exp(-tR/T)));
figure('Name','FK 02 Rotation validation');
tiledlayout(2,1);
nexttile; plot(tR,abs(omegaR),'b',tR,spinSS*(1-exp(-tR/T)),'r--','LineWidth',1.3);
grid on; xlabel('Time (s)'); ylabel('Yaw speed (rad/s)'); legend('Measured','Ideal');
nexttile; plot(tR,rad2deg(abs(thetaR)),'b',tR,rad2deg(thetaIdealR),'r--','LineWidth',1.3);
grid on; xlabel('Time (s)'); ylabel('Heading change (deg)'); legend('Measured','Ideal');
% 6-DOF follower frame is offset 135 mm from axle midpoint in local -Y.
% This was established experimentally and should be checked against CAD frame origin.
[xm,ym] = axle_trajectory(xR,yR,thetaR,-0.135);
drift = hypot(xm-xm(1),ym-ym(1));
fprintf('\nSPIN: ideal %.2f deg, measured %.2f deg, heading error %.2f %%\n', ...
    rad2deg(thetaIdealR(end)),rad2deg(abs(thetaR(end))), ...
    100*abs(abs(thetaR(end))-thetaIdealR(end))/thetaIdealR(end));
fprintf('Estimated axle midpoint net drift %.4f mm, max excursion %.4f mm, runtime %.2f s\n', ...
    drift(end)*1000,max(drift)*1000,tr);

%% 3. Curved path | left 15 / right 27 RPM, 5 seconds
rpmL = 15; rpmR = 27; duration = 5;
[outCurve, tc] = run_wheel_case(mdl,rpmL,rpmR,T,duration);
[tC,xC,yC,omegaC] = get_motion(outCurve);
thetaC = cumtrapz(tC,omegaC);
[xm,ym] = axle_trajectory(xC,yC,thetaC,-0.135);
xm = xm-xm(1); ym = ym-ym(1);
vC = r*(rpmL+rpmR)*pi/60;
omC = r*(rpmR-rpmL)*2*pi/(60*L);
R = vC/omC;
q = omC*(tC-T*(1-exp(-tC/T)));
xTheory = -R*(1-cos(q)); yTheory = R*sin(q);
err = hypot(xm-xTheory,ym-yTheory);
rmse = sqrt(trapz(tC,err.^2)/(tC(end)-tC(1)));
figure('Name','FK 03 Curved path validation');
tiledlayout(1,2);
nexttile; plot(xm,ym,'b',xTheory,yTheory,'r--','LineWidth',1.4);
axis equal; grid on; xlabel('X (m)'); ylabel('Y (m)'); legend('Measured','Ideal');
nexttile; plot(tC,rad2deg(thetaC),'b',tC,rad2deg(q),'r--','LineWidth',1.4);
grid on; xlabel('Time (s)'); ylabel('Yaw change (deg)'); legend('Measured','Ideal');
fprintf('\nCURVE: R theoretical %.4f m, final XY error %.4f m, RMSE %.4f m, runtime %.2f s\n', ...
    R,err(end),rmse,tc);

%% 4. Notes for supervisor
% 1. These tests use prescribed joint motion (not a torque motor controller).
% 2. Spherical contact proxies improve simulation speed but tire slip validation remains.
% 3. Earlier measured baselines: 10-s straight distance error 0.84%%;
%    CAD-corrected spin heading error 1.462%%; CAD-corrected curve heading error 1.12%%.
% 4. Simulation results above are RE-RUN RESULTS; they may differ slightly.

function [out, elapsed] = run_wheel_case(mdl,leftRPM,rightRPM,T,seconds)
    values = [-leftRPM,-rightRPM]*2*pi/60; % Simscape negative = forward
    sides = {'RL','RR'};
    for i = 1:2
        b = [mdl '/Motor Position ' sides{i}];
        assert(getSimulinkBlockHandle(b)~=-1,'Missing motor command block: %s',b);
        expr = sprintf('%.12g*(u-%.12g*(1-exp(-u/%.12g)))',values(i),T,T);
        set_param(b,'Expr',expr);
    end
    set_param(mdl,'StartTime','0','StopTime',num2str(seconds), ...
        'SolverType','Variable-step','Solver','ode45','RelTol','1e-3', ...
        'MaxStep','0.01','ReturnWorkspaceOutputs','on');
    tic; out = sim(mdl); elapsed = toc;
end
function [t,x,y,omega] = get_motion(out)
    xt = out.get('robot_x'); yt = out.get('robot_y'); wt = out.get('robot_omega_z');
    t = xt.Time(:); x = squeeze(xt.Data); x = x(:);
    y = interp1(yt.Time(:),squeeze(yt.Data),t,'linear','extrap'); y=y(:);
    w = squeeze(wt.Data); w=w(:);
    omega = interp1(wt.Time(:),w,t,'linear','extrap'); omega=omega(:);
end
function [xa,ya] = axle_trajectory(x,y,theta,offsetY)
    xa = x-offsetY*sin(theta); ya=y+offsetY*cos(theta);
end
