# Haprow-uvcCleaningRobot

ROS 2 Jazzy workspace for a CAD-based UVC cleaning robot in a Gazebo operating
theatre. Includes the hospital environment, a level floor, differential-drive
robot simulation, odometry, lidar and IMU, plus a Docker environment with tmux.

## Workspace layout

The GitHub repository root represents `addison_ws` itself. The tracked layout is:

```text
addison_ws/
├── .dockerignore                      Docker build-context exclusions
├── .gitignore                         Local/credential exclusions
├── Dockerfile                         Workspace image definition
├── README.md                          This guide
├── compose.yaml                       Base container configuration
├── build/                             Uploaded host build snapshot
├── install/                           Uploaded host install snapshot
├── log/                               Uploaded host colcon logs
├── docker/
│   ├── README.md                      Detailed Docker guide
│   ├── compose.dev.yaml                Persistent workspace/login volumes
│   ├── compose.dri.yaml                Intel/AMD GPU overlay
│   ├── compose.gui.yaml                X11 GUI overlay
│   ├── compose.nvidia.yaml             NVIDIA GPU overlay
│   ├── connect-github.py               Git connection helper
│   ├── entrypoint.sh                   Container entrypoint
│   ├── environment.bash                ROS/workspace environment
│   ├── smoke-test.py                   Simulation verification
│   ├── smoke-test.sh                   Isolated smoke-test launcher
│   ├── tests/
│   │   └── test_connect_github.py      Git connection regression tests
│   └── tmux.sh                        Simulation/terminal tmux session
└── src/
    └── yahboom_rosmaster/
        ├── README.md                  Source-package overview
        ├── mecanum_drive_controller/
        ├── uvc_robot_description/     UVC URDF, meshes, launch and RViz
        ├── yahboom_rosmaster/          Original ROSMASTER metapackage
        ├── yahboom_rosmaster_bringup/
        ├── yahboom_rosmaster_description/
        ├── yahboom_rosmaster_docking/
        ├── yahboom_rosmaster_gazebo/   Theatre world, models and ROS bridge
        ├── yahboom_rosmaster_localization/
        ├── yahboom_rosmaster_msgs/
        ├── yahboom_rosmaster_navigation/
        └── yahboom_rosmaster_system_tests/
```

`build/`, `install/`, and `log/` are included on GitHub as the requested host
snapshot. Docker builds fresh outputs instead of copying those working files.
Machine-local `.native-build/`, `.native-install/`, and `.native-log/` are ignored,
as are credentials and the local `.upstream-git/` backup. They are not part of
the uploaded layout. Docker images, containers and volumes are stored separately
by Docker, not as directories in this repository.

## Team setup with Docker

These steps are for a fresh **Ubuntu 24.04** PC. ROS and Gazebo do not need to
be installed on the host; the Docker image supplies them. Commands are marked
as **host terminal** or **inside Docker** so you know where to run them.

### 1. Install Docker and Git — host terminal

```bash
sudo apt-get update
sudo apt-get install -y git docker.io docker-compose-v2
sudo systemctl enable --now docker
```

### 2. Clone the public repository and build — host terminal

```bash
git clone https://github.com/Ehzem/Haprow-uvcCleaningRobot.git ~/addison_ws
cd ~/addison_ws

sudo env LOCAL_UID="$(id -u)" LOCAL_GID="$(id -g)" docker compose build
```

The first build downloads the dependencies and compiles the workspace. No
GitHub login is needed to clone this public repository or build the image.

### 3. Open the development container — host terminal

```bash
cd ~/addison_ws
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml run --rm workspace bash
```

This opens a normal shell without automatically starting tmux, Gazebo or RViz.
The development configuration preserves the container workspace and GitHub CLI
login in local Docker volumes between sessions. These volumes use host disk
space and are separate from the host's `~/addison_ws` checkout. Reopening uses
the same volumes rather than making another workspace copy.

System packages installed manually with `apt` inside a temporary container
are not in these volumes. Add permanent package dependencies to the Dockerfile
and rebuild the image. Do not remove the development volumes until their work
has been pushed or backed up.

### 4. Verify the workspace — inside Docker

```bash
test -f /.dockerenv && echo "You are inside Docker"

cd /home/itadmin/addison_ws
ls src
git remote -v
gh --version
```

The Git origin should be
`https://github.com/Ehzem/Haprow-uvcCleaningRobot.git`. New images initialize
this connection automatically and include GitHub CLI. The image uses the source
from the local build context; opening a container does not automatically pull
new commits from GitHub. The shell prompt can
look similar to the host prompt; the `/.dockerenv` check confirms the container.

Run the simulation check if you want to verify robot spawning, sensor topics
and forward motion without opening desktop windows:

```bash
workspace-smoke-test
```

A successful check prints `"result": "PASS"`.

### 5. Sign in to push changes — inside Docker

Before a teammate can push to this repository, the owner must invite them
through **GitHub repository → Settings → Collaborators**, and they must accept.
Public access allows cloning and fetching, but does not grant push permission.
Alternatively, a teammate can push to their own fork and open a pull request.

Each teammate signs in with their own account. Run these commands **without
sudo** inside Docker, and follow the device-code browser login instructions:

```bash
gh auth login --hostname github.com --git-protocol https --web
gh auth setup-git

read -r -p "Your commit author name: " commit_author_name
read -r -p "Your commit author email: " commit_author_email

git config --local user.name "$commit_author_name"
git config --local user.email "$commit_author_email"
```

The author email can be a GitHub noreply address. No GitHub login or credentials
are bundled in the image or shared automatically between teammates.

### 6. Create a branch, edit and push — inside Docker

Choose a new branch name for each task; `feature/my-change` is an example:

```bash
git switch -c feature/my-change

# Edit the source or configuration files for your task.
git add src docker Dockerfile compose.yaml README.md .gitignore .dockerignore
git diff --cached --stat
git commit -m "Describe the changes"
git push -u origin feature/my-change
```

Review the staged summary before committing. The repository includes the host's
`build/`, `install/`, and `log/` snapshot, while Docker generates its own outputs.
Those differences are expected. Stage source/configuration files intentionally
rather than using `git add .` to avoid committing generated output accidentally.
The branch-creation command does not switch to the latest remote commit: it
starts from the current local commit and keeps existing working-file changes.

### 7. Exit and reopen

To return to the host, run **inside Docker**:

```bash
exit
```

To reopen the same persistent workspace later, run in the **host terminal**:

```bash
cd ~/addison_ws
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml run --rm workspace bash
```

The existing development volume keeps its files, commits and branch. Image
rebuilds do not overwrite an existing volume's workspace; use Git to update its
source and rebuild it with colcon as needed. Keep source changes synchronized
between the separate host and container checkouts through GitHub.

See [Docker instructions](docker/README.md) for GUI access, tmux, GPU options,
source mounts, image updates and additional persistence details.

## Run the operating theatre and RViz inside Docker

These commands assume the image has already been built and you are using an
Ubuntu X11/XWayland desktop. Gazebo shows the operating theatre with the UVC
robot; RViz shows the robot and its live laser scan. A theatre occupancy map is
not included in RViz yet.

### 1. Open a GUI-enabled container — host terminal

If you are already inside a shell opened without the GUI overlay, run `exit`
first. Reopen with both the development and GUI configurations:

```bash
cd ~/addison_ws
# If xhost is missing: sudo apt-get install -y x11-xserver-utils
xhost +si:localuser:"$(id -un)"
sudo env DISPLAY="$DISPLAY" docker compose \
  -f compose.yaml -f docker/compose.dev.yaml -f docker/compose.gui.yaml \
  run --rm workspace bash
```

This reuses the persistent development workspace and enables desktop windows.
The default GUI configuration uses software rendering, so a GPU setup is not
required. See the [Docker guide](docker/README.md#gazebo-and-rviz-windows) for
optional hardware acceleration.

### 2. Start Gazebo, the UVC robot and RViz — inside Docker

```bash
cd /home/itadmin/addison_ws
source /opt/ros/jazzy/setup.bash
source install/setup.bash
ros2 launch yahboom_rosmaster_gazebo operating_theatre.launch.py \
  headless:=false use_rviz:=true
```

Gazebo and RViz open as separate windows. The theatre launcher spawns the UVC
robot, publishes its transforms, and starts the simulation/ROS bridges.

### 3. Stop and return to the host

Press **Ctrl+C** in the launch terminal, then run inside Docker:

```bash
exit
```

After returning to the host terminal, revoke the display-access grant:

```bash
xhost -si:localuser:"$(id -un)"
```

For running directly on Ubuntu without Docker, use the separate
[native host build guide](docker/README.md#native-host-build).
See the [theatre documentation](src/yahboom_rosmaster/yahboom_rosmaster_gazebo/OPERATING_THEATRE.md)
and [UVC description](src/yahboom_rosmaster/uvc_robot_description/README.md)
for robot details and driving commands.

## Simulation scope

The UVC robot uses imported CAD visual geometry with approximated mass, inertia,
collision shapes and caster contacts. Lidar and IMU are simulation additions.
UV dose/disinfection physics are not implemented. Navigation dependencies are
included; a theatre SLAM map and UVC Nav2 configuration are still needed for
autonomous navigation.

## Source and attribution

The original Yahboom support packages come from
[Automatic Addison's yahboom_rosmaster](https://github.com/automaticaddison/yahboom_rosmaster),
starting from commit `6f99244`. Existing package license files are preserved.
The hospital and UVC visuals were converted from the CAD exports supplied for
this project. Original Downloads files are not required to run the simulation.

The repository includes `build/`, `install/`, and `log/` as a snapshot of the
current host workspace, in addition to all source and Docker configuration.
Generated setup files, CMake caches, and symbolic links can contain absolute paths
to `/home/itadmin/addison_ws` and `/opt/ros/jazzy`, so another machine may need a
fresh build. Credentials and local upstream Git metadata remain excluded.
Docker images are built separately rather than committed here.
