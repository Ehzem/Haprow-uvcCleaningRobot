%% UV-C BOT | CAD DIMENSIONS, MASS AND MODEL INSPECTION
% This script reads parameters from the packaged actual .slx block values.
% It does NOT alter or simulate the model.

%% 0. Load packaged model
clearvars; clc;
root=fileparts(mfilename('fullpath'));
mdl='UVC_Bot_Validated';
load_system(fullfile(root,[mdl '.slx']));

%% 1. Rear drive wheels: positions, track and clearance
left=str2num(get_param([mdl '/Mount RL'],'TranslationCartesianOffset')); %#ok<ST2NM>
right=str2num(get_param([mdl '/Mount RR'],'TranslationCartesianOffset')); %#ok<ST2NM>
rearTrack=abs(right(1)-left(1));
rearRad=str2double(get_param([mdl '/Drive Wheel RL'],'CylinderRadius'));
rearWidth=str2double(get_param([mdl '/Drive Wheel RL'],'CylinderLength'));
centralDims=str2num(get_param([mdl '/Chassis'],'BrickDimensions')); %#ok<ST2NM>
innerFaceGap=rearTrack-rearWidth;
centralSideClearance=(innerFaceGap-centralDims(1))/2;
fprintf('\n=== REAR DRIVE GEOMETRY ===\n');
fprintf('RL mount: [%.3f %.3f %.3f] mm\n',left*1000);
fprintf('RR mount: [%.3f %.3f %.3f] mm\n',right*1000);
fprintf('Rear wheel center track: %.3f mm (CAD target 338.413 mm)\n',rearTrack*1000);
fprintf('Rear wheel diameter / axial width: %.1f / %.1f mm\n',2*rearRad*1000,rearWidth*1000);
fprintf('Inner wheel-face gap (nominal): %.3f mm\n',innerFaceGap*1000);
fprintf('Central plate width: %.1f mm\n',centralDims(1)*1000);
fprintf('Nominal clearance on each side: %.3f mm\n',centralSideClearance*1000);

%% 2. Front caster positions and geometry
fprintf('\n=== FRONT CASTER GEOMETRY ===\n');
frontX=zeros(1,2);
sides={'FL','FR'};
for i=1:2
    s=sides{i};
    p=str2num(get_param([mdl '/Caster Mount ' s],'TranslationCartesianOffset')); %#ok<ST2NM>
    cr=str2double(get_param([mdl '/Caster Wheel ' s],'CylinderRadius'));
    cw=str2double(get_param([mdl '/Caster Wheel ' s],'CylinderLength'));
    trailPos=str2num(get_param([mdl '/Caster Axle Transform ' s],'TranslationCartesianOffset')); %#ok<ST2NM>
    wheelCenterApprox=p+[0,trailPos(2),0];
    frontX(i)=wheelCenterApprox(1);
    fprintf('%s steering pivot (model): [%.2f %.2f %.2f] mm\n',s,p*1000);
    fprintf('%s wheel centre (zero swivel, nominal): [%.2f %.2f %.2f] mm\n',s,wheelCenterApprox*1000);
    fprintf('%s wheel dia/width: %.1f / %.1f mm, caster trail %.1f mm\n',s,2*cr*1000,cw*1000,abs(trailPos(2))*1000);
end
fprintf('Front caster centre track: %.3f mm\n',abs(frontX(2)-frontX(1))*1000);
fprintf('NOTE: 20 mm caster trail is a modelling assumption, not verified from CAD.\n');

%% 3. Chassis physical pieces and envelope
fprintf('\n=== CHASSIS GEOMETRY ===\n');
centralOffset=str2num(get_param([mdl '/Chassis Height Offset'],'TranslationCartesianOffset')); %#ok<ST2NM>
fprintf('Central plate: [%.1f %.1f %.1f] mm\n',centralDims*1000);
fprintf('Central plate offset relative to robot frame: [%.1f %.1f %.1f] mm\n',centralOffset*1000);
for s={'Left','Right'}
    name=s{1};
    d=str2num(get_param([mdl '/Chassis Wing ' name],'BrickDimensions')); %#ok<ST2NM>
    p=str2num(get_param([mdl '/Chassis Wing Mount ' name],'TranslationCartesianOffset')); %#ok<ST2NM>
    fprintf('%s wing: dimensions [%.1f %.1f %.1f] mm, mount [%.1f %.1f %.1f] mm\n',name,d*1000,p*1000);
end
fprintf('Design outer chassis footprint: 400 x 500 mm; central structural plate: 260 mm wide.\n');
fprintf('IMPORTANT: visually simplified wheel-well body is not an imported full CAD shell.\n');

%% 4. Mass allocation actually configured in Solid blocks
parts={'Chassis','Chassis Wing Left','Chassis Wing Right', ...
    'Drive Wheel RL','Drive Wheel RR','Caster Wheel FL','Caster Wheel FR', ...
    'Caster Fork FL','Caster Fork FR'};
totalMass=0;
fprintf('\n=== MODEL MASS ALLOCATION ===\n');
for i=1:numel(parts)
    b=[mdl '/' parts{i}];
    inertiaType=get_param(b,'InertiaType');
    if strcmpi(inertiaType,'Custom')
        m=str2double(get_param(b,'Mass'));
        totalMass=totalMass+m;
        fprintf('%-23s %7.3f kg   (%s)\n',parts{i},m,inertiaType);
    else
        fprintf('%-23s MASS NOT READ: %s (computed from geometry/density)\n',parts{i},inertiaType);
    end
end
fprintf('Total explicitly assigned mass: %.3f kg; design mass 35.000 kg\n',totalMass);
fprintf('NOTE: battery/mast location and measured centre of mass not represented accurately yet.\n');

%% 5. Physical and kinematic design constants
fprintf('\n=== DIFFERENTIAL DRIVE EQUATIONS ===\n');
r=rearRad; L=rearTrack;
fprintf('Linear velocity v = %.8f * (wL + wR)\n',r/2);
fprintf('Yaw rate Omega = %.8f * (wR - wL)\n',r/L);
fprintf('Inverse wL = %.8f*v - %.8f*Omega\n',1/r,L/(2*r));
fprintf('Inverse wR = %.8f*v + %.8f*Omega\n',1/r,L/(2*r));
fprintf('All wheel speeds above are physical forward-positive rad/s; joint input sign is opposite.\n');
