%% ONE-TIME PACKAGING | Copies current tested model into this clean folder
% INSTRUCTIONS:
% 1. Download/unzip this folder anywhere (e.g. D:\UVC_Bot_Supervisor).
% 2. In MATLAB, save your working UVC_Sphere_Contact_Test model.
% 3. Run THIS script from the unpacked folder.
% 4. Then run 03, 01, and 02. This script never deletes your original.

%% 0. Locate files
clearvars; clc;
dest=fileparts(mfilename('fullpath'));
sourceRoot='D:\UVC-Matlab_Simulation';
sourceModel=fullfile(sourceRoot,'UVC_Sphere_Contact_Test.slx');
targetModel=fullfile(dest,'UVC_Bot_Validated.slx');
assert(isfile(sourceModel),'Missing original .slx: %s',sourceModel);
if bdIsLoaded('UVC_Sphere_Contact_Test')
    if strcmp(get_param('UVC_Sphere_Contact_Test','Dirty'),'on')
        error('Save the currently open UVC_Sphere_Contact_Test model first, then rerun packaging.');
    end
end

%% 1. Copy, without altering or renaming the original
if isfile(targetModel)
    error('Target already exists: %s. Move/archive it before rebuilding; will not overwrite.',targetModel);
end
[ok,msg]=copyfile(sourceModel,targetModel);
assert(ok,'Copy failed: %s',msg);

%% 2. Dependency audit (read-only)
% This is advisory: some dependencies are not detectable by static analysis.
try
    [req, products]=matlab.codetools.requiredFilesAndProducts(sourceModel); %#ok<ASGLU>
    req=string(req);
catch err
    warning('Automatic dependency scan unavailable: %s',err.message);
    req=strings(0,1);
end
fid=fopen(fullfile(dest,'DEPENDENCIES_REVIEW.txt'),'w');
fprintf(fid,'Source model: %s\nPackaged model: %s\n',sourceModel,targetModel);
fprintf(fid,'Required: Simulink, Simscape, Simscape Multibody.\n');
fprintf(fid,'Any geometry or project paths below must be checked on demonstration PC.\n\n');
for k=1:numel(req)
    fprintf(fid,'%s\n',req(k));
end
fclose(fid);

%% 3. Copy only local external assets if present
% Sphere model created here uses primitive solids; normally no external CAD file.
% Copy any local MATs from the original workspace as optional assets, in case
% model callbacks refer to stored robot parameters. Never overwrite.
assetDir=fullfile(dest,'assets');
if ~exist(assetDir,'dir'), mkdir(assetDir); end
mats=dir(fullfile(sourceRoot,'*.mat'));
for k=1:numel(mats)
    src=fullfile(sourceRoot,mats(k).name);
    dst=fullfile(assetDir,mats(k).name);
    if ~isfile(dst), copyfile(src,dst); end
end

%% 4. Test loading of renamed model and configuration consistency
try
    load_system(targetModel);
    assert(bdIsLoaded('UVC_Bot_Validated'), 'Renamed model did not load.');
    assert(getSimulinkBlockHandle('UVC_Bot_Validated/Robot Motion Sensor')~=-1, ...
        'Motion sensor missing in packaged model.');
    assert(getSimulinkBlockHandle('UVC_Bot_Validated/Motor Position RL')~=-1, ...
        'Motor input missing in packaged model.');
    fprintf('\nPACKAGED MODEL LOADED SUCCESSFULLY.\n');
catch err
    warning('Copy created, but load/validation needs manual attention: %s',err.message);
end
fprintf('Clean folder: %s\n',dest);
fprintf('Main model: %s\n',targetModel);
disp('Next: run 03_Robot_Geometry_and_Mass.m, then 01_Forward_Kinematics.m and 02_Inverse_Kinematics.m');
