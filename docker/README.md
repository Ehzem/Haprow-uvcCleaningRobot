# Docker development guide

The image builds the workspace on Ubuntu 24.04 with ROS 2 Jazzy, Gazebo Harmonic
through ROS vendor packages, RViz, Nav2, SLAM Toolbox, localization, docking,
ROS 2 control, tmux and GitHub CLI (`gh`). Dependency installation comes from the
workspace package manifests. No host ROS installation is needed.

For a fresh PC, start with the [main README's team setup](../README.md#team-setup-with-docker).
The commands below assume Ubuntu 24.04 with Docker Engine and Compose installed.
Commands using `sudo docker` run on the **host**; ROS and GitHub login commands
labelled below run **inside the container**, without sudo.

## Build and open the development workspace

On the host, install the required tools if necessary:

```bash
sudo apt-get update
sudo apt-get install -y git docker.io docker-compose-v2
sudo systemctl enable --now docker
```

Clone once into an unused `~/addison_ws` directory, then build:

```bash
git clone https://github.com/Ehzem/Haprow-uvcCleaningRobot.git ~/addison_ws
cd ~/addison_ws
sudo env LOCAL_UID="$(id -u)" LOCAL_GID="$(id -g)" docker compose build
```

The account inside the image is always `itadmin`, with its workspace at
`/home/itadmin/addison_ws`; the host username may be different. Build arguments
match its UID/GID to the host user, which also supports X11 display access.
The image tag is `addison-workspace:jazzy`.

Open a normal development shell, without starting the simulation:

```bash
cd ~/addison_ws
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml run --rm workspace bash
```

Inside the container:

```bash
if [ -f /.dockerenv ]; then
    echo "You are inside Docker"
else
    echo "You are on the host"
fi
cd /home/itadmin/addison_ws
ls src
git remote -v
gh --version
```

ROS and the workspace are sourced in interactive shells, including tmux windows.
The host `sudo` prompt uses your host Ubuntu password. The container account
`itadmin` has no configured sudo password; run the documented ROS/Git commands
without sudo inside it.
Use `exit` to leave. Run the same development-shell command on the host to reopen
that persistent workspace.

## What persists

| Content | Development overlay | Base Compose without the overlay |
| --- | --- | --- |
| Workspace source, commits, branches, build outputs | Stored in a local named volume | Lost when the `--rm` container is removed |
| GitHub CLI login in `~/.config/gh` | Stored in a second local named volume | Lost when the container is removed |
| Packages installed manually with `apt` | Lost when the container is removed | Lost when the container is removed |
| Files edited directly in the host checkout | Remain on the host; separate from the container volume | Remain on the host |

Volumes use host disk space. Reopening a development container reuses the same
volumes. The development workspace is initialized from the image only the first
time its volume is created. Rebuilding the image does not overwrite an existing
volume, and container edits do not automatically modify the host checkout.
Use Git to share source changes between those copies. Do not run a volume-removal
command until its work has been pushed or backed up.

For permanent system dependencies, update the Dockerfile (or package manifests
for ROS dependencies), rebuild the image, and start a new container. Files and
credentials in development volumes are not copied back into the image.

To inspect disk usage, run on the host:

```bash
sudo docker system df
```

For disposable testing with no persistent workspace, use this host command:

```bash
sudo docker compose run --rm workspace bash
```

## GitHub and team access

At image build time, `workspace-connect-github` fetches the public repository and
prepares the `main` branch to track `origin/main`. Host `.git` metadata and login
credentials are not copied. Source, setup files and documentation come from the
local build context, preserving local additions, edits and deletions. Cached
image layers may reuse previously fetched metadata. Starting or reopening a
container does not automatically fetch or pull new commits.

The repository also tracks a host snapshot of `build/`, `install/`, and `log/`.
Docker compiles its own outputs rather than copying those working files from the
host, so differences in those directories are expected. The fetched Git history
still contains the committed snapshot. Stage source/configuration files
intentionally; `git add .` can include generated differences.

Inside the container, sign in with your own account and set your commit identity:

```bash
gh auth login --hostname github.com --git-protocol https --web
gh auth setup-git
read -r -p "Your commit author name: " commit_author_name
read -r -p "Your commit author email: " commit_author_email
git config --local user.name "$commit_author_name"
git config --local user.email "$commit_author_email"
```

Follow the device-code browser sign-in. Public access permits cloning/fetching.
Pushing to the shared repository requires a collaborator invitation from its
owner and acceptance by the developer. A developer can instead push to their own
fork and open a pull request. No personal account is authenticated in the image.

A feature branch starts from the current local commit and retains local working
changes; it does not automatically advance to the newest remote commit:

```bash
git fetch origin
git switch -c feature/your-change
# Edit source or configuration for your task.
git add -- src docker Dockerfile compose.yaml README.md .gitignore .dockerignore
git diff --cached --stat
git commit -m "Describe your change"
git push -u origin feature/your-change
```

Use a new branch name for each task. Fetching is safe for local files; pulling or
switching to commits that modify the tracked build snapshots can be blocked by
container-local generated changes. Inspect and back up your work before deciding
how to resolve them; do not discard them blindly.

To build against a fork or another branch, pass arguments explicitly through sudo:

```bash
sudo env LOCAL_UID="$(id -u)" LOCAL_GID="$(id -g)" \
  GITHUB_REPOSITORY=https://github.com/YOUR-ACCOUNT/YOUR-FORK.git \
  GITHUB_BRANCH=main docker compose build
```

Replace the fork URL before running. This changes the new image's origin; it does
not rewrite an existing development volume's Git remote. Never put tokens in
build arguments. Running `workspace-connect-github` again preserves an existing
repository and developer branch; it rejects a different existing origin.

## Simulation with tmux

For headless simulation in the persistent workspace, run on the host:

```bash
cd ~/addison_ws
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml run --rm workspace workspace-tmux --headless
```

This opens tmux windows `0:simulation`, `1:terminal` and `2:teleop`. Switch with
Ctrl+B, release, then press 0/1/2. If no display is available, the helper uses
Xvfb and software rendering so lidar can run. In the teleop window:

```bash
ros2 run teleop_twist_keyboard teleop_twist_keyboard
```

The robot accepts ordinary Twist messages on `/cmd_vel`. Explicitly stop it when
finished:

```bash
ros2 topic pub --once /cmd_vel geometry_msgs/msg/Twist '{}'
```

Detaching from tmux ends a foreground `run --rm` container once its main process
exits; the development volumes retain files, not running processes. To keep the
simulation running after detaching, use a background service on the host:

```bash
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml up -d workspace
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml exec workspace bash
```

Inside that shell, run `workspace-tmux --headless`, then detach with Ctrl+B, D.
To reattach or stop the service, run on the host:

```bash
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml exec workspace bash -ic 'tmux attach -t theatre'
# Stop the service while retaining its development volumes:
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml down
```

## Gazebo and RViz windows

These examples require an X11/XWayland desktop. For the optional `xhost` command,
install `x11-xserver-utils` on the host if necessary. Grant access only to the
local desktop user whose UID matches the image user:

```bash
xhost +si:localuser:"$(id -un)"
sudo env DISPLAY="$DISPLAY" docker compose \
  -f compose.yaml -f docker/compose.dev.yaml -f docker/compose.gui.yaml \
  run --rm workspace workspace-tmux
```

After the GUI container exits:

```bash
xhost -si:localuser:"$(id -un)"
```

The GUI overlay defaults to software rendering. It mounts the host display
socket; the development overlay additionally mounts its two named volumes.
The host home directory and Docker socket are not mounted, and privileged mode
is not used.

For Intel/AMD acceleration, `/dev/dri` must exist and the user needs the group IDs
that grant access to its device nodes. On systems with `video` and `render`
groups, run on the host:

```bash
sudo env DISPLAY="$DISPLAY" \
  VIDEO_GID="$(getent group video | cut -d: -f3)" \
  RENDER_GID="$(getent group render | cut -d: -f3)" \
  docker compose -f compose.yaml -f docker/compose.dev.yaml \
  -f docker/compose.gui.yaml -f docker/compose.dri.yaml run --rm workspace workspace-tmux
```

For NVIDIA, the host must have the NVIDIA driver and NVIDIA Container Toolkit
configured for Docker. With the same X11 grant:

```bash
sudo env DISPLAY="$DISPLAY" docker compose \
  -f compose.yaml -f docker/compose.dev.yaml -f docker/compose.gui.yaml \
  -f docker/compose.nvidia.yaml run --rm workspace workspace-tmux
```

## Validation and image updates

The build runs dependency checks, compiles all packages, installs tmux/GitHub CLI
and initializes Git. Test the baked image without a desktop or GPU:

```bash
sudo docker compose run --rm -T workspace workspace-smoke-test
```

To test the current persistent development workspace instead:

```bash
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml run --rm -T workspace workspace-smoke-test
```

The smoke test uses a virtual X server/software rendering, a separate ROS domain
(default 79, configurable with `SMOKE_ROS_DOMAIN_ID`) and a unique Gazebo partition.
It waits for odometry, wheel states, lidar and IMU, commands short forward motion,
stops the robot and checks its Gazebo presence. It prints `"result": "PASS"` or
exits nonzero. Its log is `/tmp/addison-smoke-test.log` inside that container.
Omit `--rm` when investigating a failure so you can inspect the stopped container.

To change compiler parallelism or refresh upstream packages, run on the host:

```bash
sudo env LOCAL_UID="$(id -u)" LOCAL_GID="$(id -g)" BUILD_JOBS=4 docker compose build
# Refresh the base image and rerun all installation/build steps:
sudo env LOCAL_UID="$(id -u)" LOCAL_GID="$(id -g)" docker compose build --pull --no-cache
```

Default build parallelism is two jobs. Larger values need more RAM. The image
tracks Jazzy apt repositories rather than a frozen set of package versions.

Source is baked into new images. An existing development volume retains its own
source and build outputs. After updating that source through Git, rebuild it
inside the container with `colcon build --symlink-install --executor sequential`,
then `source install/setup.bash`. For native host builds, use the separate
`.native-*` directories documented in the main README.

For live editing of only the host's source without a development volume, this
optional host command mounts `src/` into a temporary container:

```bash
sudo docker compose run --rm \
  --volume "$PWD/src:/home/itadmin/addison_ws/src" workspace bash
```

Rebuild inside that container after source changes. Source edits persist on the
host, while its temporary build/install products and Git commits do not. Do not
mount host `build/` or `install/` over the image's generated outputs.

Normal development networking defaults to ROS domain 42, Gazebo partition
`addison_docker` and Linux host networking with Cyclone DDS. Other processes must
use compatible settings to communicate. When using sudo, pass custom Compose
settings via `sudo env NAME=value docker compose ...` so they reach Compose.

The environment supplies navigation dependencies. It does not create a theatre
map, configure UVC Nav2 or model UV dose/disinfection physics.

## References

- [Official ROS Docker images](https://hub.docker.com/_/ros)
- [ROS/Gazebo version pairing](https://gazebosim.org/docs/harmonic/ros_installation/)
- [Docker Engine on Ubuntu](https://docs.docker.com/engine/install/ubuntu/)
- [GitHub collaborator access](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/repository-access-and-collaboration/permission-levels-for-a-personal-account-repository)
