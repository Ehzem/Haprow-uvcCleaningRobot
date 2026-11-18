function [x,y,heading] = uvc_path(t,v,yaw,T)
% Exact ideal axle-midpoint trajectory from the origin, initial heading zero.
% Constant body v/yaw or an optional shared exponential wheel-speed ramp.
% The ramp is a mathematical input assumption, not Gazebo's acceleration model.
if nargin<4, T=0; end
assert(T>=0,'Ramp time must be non-negative');
if T==0
    elapsed=t;
else
    elapsed=t-T*(1-exp(-t/T));
end
heading=yaw*elapsed;
if abs(yaw)<1e-12
    x=v*elapsed; y=zeros(size(t));
else
    radius=v/yaw;
    x=radius*sin(heading);
    y=radius*(1-cos(heading));
end
end
