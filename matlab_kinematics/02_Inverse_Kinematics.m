%% UV-C BOT | INVERSE KINEMATICS
% Desired robot body velocity -> wheel speed -> physical Simscape result.
% Run sections using Ctrl+Enter. Uses UVC_Bot_Validated.slx.

%% 0. Initialize
clearvars; clc; close all;
root=fileparts(mfilename('fullpath'));
mdl='UVC_Bot_Validated';
load_system(fullfile(root,[mdl '.slx']));
r=0.080; L=0.338413; T=0.3;

%% 1. User-requested linear and yaw velocities
vDesired=0.15;         % m/s
omegaDesired=0.20;     % rad/s, positive means left turn (+Y forward)
stopTime=5;
omegaL=(vDesired-L*omegaDesired/2)/r;
omegaR=(vDesired+L*omegaDesired/2)/r;
rpmL=omegaL*60/(2*pi); rpmR=omegaR*60/(2*pi);
fprintf('Requested v %.4f m/s, yaw %.4f rad/s\n',vDesired,omegaDesired);
fprintf('Inverse kinematics: left %.4f RPM; right %.4f RPM\n',rpmL,rpmR);

%% 2. Apply commands and simulate
% Negative joint speeds correspond to forward-rolling wheel speed.
sides={'RL','RR'}; jointRates=[-omegaL,-omegaR];
for i=1:2
    b=[mdl '/Motor Position ' sides{i}];
    set_param(b,'Expr',sprintf('%.12g*(u-%.12g*(1-exp(-u/%.12g)))',jointRates(i),T,T));
end
set_param(mdl,'StartTime','0','StopTime',num2str(stopTime), ...
    'SolverType','Variable-step','Solver','ode45','RelTol','1e-3', ...
    'MaxStep','0.01','ReturnWorkspaceOutputs','on');
tic; out=sim(mdl); elapsed=toc;

%% 3. Read sensors and transform from frame origin to axle midpoint
xTS=out.get('robot_x'); yTS=out.get('robot_y'); omegaTS=out.get('robot_omega_z');
t=xTS.Time(:);
x=squeeze(xTS.Data); x=x(:);
y=interp1(yTS.Time(:),squeeze(yTS.Data),t,'linear','extrap'); y=y(:);
tw=omegaTS.Time(:); ww=squeeze(omegaTS.Data); ww=ww(:);
headingW=cumtrapz(tw,ww);
heading=interp1(tw,headingW,t,'linear','extrap'); heading=heading-heading(1);
% CAD longitudinal axle offset from robot reference frame, -0.135 m.
offsetY=-0.135;
ax=x-offsetY*sin(heading);
ay=y+offsetY*cos(heading);
ax=ax-ax(1); ay=ay-ay(1);

%% 4. Compute desired 2D trajectory and errors
q=omegaDesired*(t-T*(1-exp(-t/T)));
if abs(omegaDesired)<1e-12
    xTheory=zeros(size(t)); yTheory=vDesired*(t-T*(1-exp(-t/T)));
    R=Inf;
else
    R=vDesired/omegaDesired;
    xTheory=-R*(1-cos(q)); yTheory=R*sin(q);
end
err=hypot(ax-xTheory,ay-yTheory);
rmse=sqrt(trapz(t,err.^2)/(t(end)-t(1)));
mask=tw>=max(tw(1),stopTime-1);
actualW=mean(ww(mask));
expectedW=mean(omegaDesired*(1-exp(-tw(mask)/T)));
fprintf('\n--- IK VALIDATION ---\n');
fprintf('Simulation time %.1f s; execution %.2f s\n',t(end),elapsed);
fprintf('Expected final XY [%.4f, %.4f] m\n',xTheory(end),yTheory(end));
fprintf('Measured final XY [%.4f, %.4f] m\n',ax(end),ay(end));
fprintf('Expected / measured heading %.3f / %.3f degrees\n',rad2deg(q(end)),rad2deg(heading(end)));
fprintf('Heading error %.3f %%\n',100*abs(heading(end)-q(end))/max(abs(q(end)),eps));
fprintf('Yaw-rate error %.3f %%\n',100*abs(actualW-expectedW)/max(abs(expectedW),eps));
fprintf('Final XY error %.4f m; trajectory RMSE %.4f m\n',err(end),rmse);
fprintf('Theoretical turning radius %.4f m\n',R);

%% 5. Supervisor plots
figure('Name','IK validation | desired vs actual');
tiledlayout(1,2);
nexttile;
plot(ax,ay,'b',xTheory,yTheory,'r--','LineWidth',1.5); axis equal; grid on;
xlabel('X displacement (m)'); ylabel('Y displacement (m)');
legend('Simulated','Requested','Location','best'); title('XY trajectory');
nexttile;
plot(t,rad2deg(heading),'b',t,rad2deg(q),'r--','LineWidth',1.5); grid on;
xlabel('Time (s)'); ylabel('Heading change (deg)');
legend('Simulated','Requested','Location','best'); title('Heading validation');

%% 6. Supervisor notes
% Last recorded benchmark (before packaging): v=0.15 m/s, yaw=0.2 rad/s;
% wheel speeds 13.8654/21.9444 RPM; heading error 1.109%%;
% yaw-rate error 0.639%%; final XY error 7.7 mm; trajectory RMSE 5.4 mm.
% These tests use prescribed motion and approximate sphere-ground contact.
