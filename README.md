# Haprow-uvcCleaningRobot

ROS 2 Jazzy workspace for a CAD-based UVC cleaning robot in a Gazebo operating
theatre. Includes the hospital environment, a level floor, differential-drive
robot simulation, odometry, lidar and IMU, plus a Docker environment with tmux.

## Workspace layout

```text
src/yahboom_rosmaster/
  uvc_robot_description/             Robot URDF, meshes, visualization launch
  yahboom_rosmaster_gazebo/          Worlds, environment models, simulation bridge
  yahboom_rosmaster_bringup/         Existing ROSMASTER launch files
  yahboom_rosmaster_navigation/      Navigation utilities and configurations
  ...                               Other original Yahboom support packages
Dockerfile                          Image building the complete workspace
compose.yaml                        Container configuration
docker/                            Container helpers and usage instructions
```

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
this connection automatically and include GitHub CLI. The shell prompt can
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

## Build and run natively

Use Ubuntu 24.04 with ROS 2 Jazzy and rosdep installed. Clone this repository
into `~/addison_ws` to support the original X3 launcher's home-relative paths.
Install and build the source dependencies from the workspace root:

```bash
source /opt/ros/jazzy/setup.bash
rosdep update --rosdistro jazzy
rosdep install --from-paths src --ignore-src --rosdistro jazzy -y
colcon build --symlink-install --executor sequential
source install/setup.bash
ros2 launch yahboom_rosmaster_gazebo operating_theatre.launch.py
```

The theatre launcher spawns the UVC robot. The original ROSMASTER X3 launcher
remains available separately. See the
[theatre documentation](src/yahboom_rosmaster/yahboom_rosmaster_gazebo/OPERATING_THEATRE.md)
and [UVC description](src/yahboom_rosmaster/uvc_robot_description/README.md).

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
