# Docker: complete Addison workspace

The image contains Ubuntu 24.04, ROS 2 Jazzy, Gazebo Harmonic through ROS vendor
packages, RViz, Nav2, SLAM Toolbox, robot localization, AprilTag/docking tools,
ROS 2 control, all dependencies resolved from the workspace manifests, colcon,
compiler tools, Python tools, keyboard teleoperation, **tmux**, and GitHub CLI (`gh`). Every source
package is built inside the image. The host does not need ROS or Gazebo installed.

The package layout is unchanged. In the container the workspace lives at
`/home/itadmin/addison_ws`, with `uvc_robot_description` separate from the Gazebo
package. The account is `itadmin`, by default UID/GID 1000. This home/workspace
path also supports the original X3 launcher's existing home-relative paths.
No host `build`, `install`, logs, credentials or original Downloads files are
copied into the image. Converted CAD assets are already in `src`.

## Open the container and check you are inside

Once the image is built, run these commands in your **host terminal**:

```bash
cd /home/itadmin/addison_ws
sudo docker compose run --rm workspace bash
```

Enter your host Ubuntu password if sudo asks. This opens a normal shell inside
the container; it does not start tmux, Gazebo or RViz.

Then run this check in the newly opened shell:

```bash
if [ -f /.dockerenv ]; then
    echo "You are inside Docker"
else
    echo "You are on the host"
fi
```

The prompt can look the same inside and outside because the container uses the
same username and workspace path. To see your workspace inside the container:

```bash
cd /home/itadmin/addison_ws
ls
ls src
```

Run `exit` to return to the host. The `--rm` option removes this temporary
container when you exit; the built image and host workspace remain. Files edited
only inside this temporary container are lost when it is removed.

## Host setup and image build

Use a Linux host with Docker Engine and the Compose v2 plugin. GUI examples are
for an Ubuntu X11/XWayland desktop. Docker Desktop on other operating systems
requires separate display/network configuration.

On Ubuntu 24.04, one distribution-package installation option is:

```bash
sudo apt-get update
sudo apt-get install -y docker.io docker-compose-v2
sudo systemctl enable --now docker
sudo docker version
sudo docker compose version
```

Use `sudo docker` if your account does not already have Docker access. The
commands below assume it does; replace `docker` with `sudo docker` as needed.

Build from the workspace root, not the `docker` directory:

```bash
cd /home/itadmin/addison_ws
export LOCAL_UID="$(id -u)"
export LOCAL_GID="$(id -g)"
docker compose build
```

`BUILD_JOBS=2` limits compiler parallelism by default. Increase it only if memory
allows, e.g. `BUILD_JOBS=4 docker compose build`. The first build downloads several
GB of ROS/Gazebo packages and compiles the whole workspace. Builds stop if rosdep
or compilation fails. No packages are silently skipped. For refreshed upstream
packages use `docker compose build --pull --no-cache`.

The default image tag is `addison-workspace:jazzy`. Base-image and apt package
versions track the Jazzy repositories; this is not a frozen dependency snapshot.

## GitHub connection and team development

Every image build now initializes `/home/itadmin/addison_ws` as a Git repository
connected to `https://github.com/Ehzem/Haprow-uvcCleaningRobot.git`, with its
`main` branch tracking `origin/main`. New containers inherit that connection.
Git metadata is fetched from the public repository during the image build;
host Git metadata and credentials are never copied. Source is still taken
from the local Docker build context, so local uncommitted source changes are
preserved and appear as working-tree differences against GitHub. Builds need
network access to GitHub. Rebuilding an image does not change a running container.

To use a fork or another branch, set `GITHUB_REPOSITORY` and `GITHUB_BRANCH`
before `docker compose build`. Never put tokens or passwords in build arguments.

### Fresh machine

After installing Docker Engine and Compose, clone and build on the host:

```bash
git clone https://github.com/Ehzem/Haprow-uvcCleaningRobot.git ~/addison_ws
cd ~/addison_ws
sudo env LOCAL_UID="$(id -u)" LOCAL_GID="$(id -g)" docker compose build
sudo docker compose -f compose.yaml -f docker/compose.dev.yaml run --rm workspace bash
```

The development overlay uses local named volumes for the container workspace
and that developer's GitHub CLI login. They survive removal of a `--rm`
container. Without the overlay, edits, commits and login inside a temporary
container disappear when that container is removed. These volumes do not copy
credentials into the image or share them between devices.

Inside the container, verify the connection:

```bash
cd /home/itadmin/addison_ws
git remote -v
git branch -vv
```

### Team member login and pushing

A public repository allows everyone to clone/fetch; it does not grant everyone
permission to push. The repository owner must invite each team member through
GitHub **Settings → Collaborators**, and the member must accept. Alternatively,
a member can push to their own fork and open a pull request.

Each developer signs into their own GitHub account inside the container,
without sudo. No account password or access token is bundled in the image:

```bash
gh auth login --hostname github.com --git-protocol https --web
gh auth setup-git
git config --local user.name "Your Name"
git config --local user.email "your-email-or-github-noreply-address"
```

Follow the device-code browser sign-in printed by `gh`. To work on a branch:

```bash
git fetch origin
git switch -c feature/your-change
# Edit the desired source or configuration files.
git add -- src docker Dockerfile compose.yaml README.md .gitignore .dockerignore
git diff --cached --stat
git commit -m "Describe your change"
git push -u origin feature/your-change
```

This repository also tracks the host `build/`, `install/`, and `log/` snapshot.
The container compiles its own outputs, so those directories normally differ.
Stage the source/configuration files intentionally rather than `git add .` to
avoid unintentionally committing generated files. Pulls that change those
tracked generated outputs can require resolving local differences first;
inspect them instead of discarding local work blindly.

The development volume is initialized from the image only the first time it
is created. Later image rebuilds do not replace an existing volume's workspace.
Use Git to update its source and rebuild with colcon as needed. Do not remove
the volume until you have pushed or backed up your work. Updating the image's
system tools/dependencies still requires an image rebuild and a new container.

## Terminal and tmux

Open a sourced shell:

```bash
docker compose run --rm workspace
```

ROS and the workspace are sourced automatically in every interactive shell,
including new tmux windows. To run the simulation without a desktop window:

```bash
docker compose run --rm workspace workspace-tmux --headless
```

This creates a `theatre` tmux session with three windows: `simulation`, `terminal`,
and `teleop`. The headless session uses Xvfb when no display is available, so
Gazebo's lidar still has a rendering context. Switch windows with Ctrl+B then
0/1/2; detach with Ctrl+B then D. A foreground `docker compose run --rm` container
ends after its main process exits, so use a persistent container for detach/reattach:

```bash
docker compose up -d workspace
docker compose exec workspace bash
# Inside the container:
workspace-tmux --headless
# Detach from tmux, then leave the shell with exit.
# Later, reattach from the host:
docker compose exec workspace bash -ic 'tmux attach -t theatre'
# Stop the persistent container:
docker compose down
```

To use keyboard control, select the teleop window and run:

```bash
ros2 run teleop_twist_keyboard teleop_twist_keyboard
```

The UVC model uses ordinary Twist messages on `/cmd_vel`. Send an explicit stop
when finished: `ros2 topic pub --once /cmd_vel geometry_msgs/msg/Twist '{}'`.

## Gazebo and RViz windows

The default GUI overlay uses Mesa software rendering, which works without a GPU
passthrough configuration but can be slower. First allow only your local desktop
user to access X11; the image user must have your host UID, as set during build:

```bash
xhost +si:localuser:"$(id -un)"
docker compose -f compose.yaml -f docker/compose.gui.yaml run --rm workspace workspace-tmux
```

After the GUI container exits, revoke that local-user display grant:

```bash
xhost -si:localuser:"$(id -un)"
```

If using `sudo docker`, preserve the desktop `DISPLAY`, for example:

```bash
sudo env DISPLAY="$DISPLAY" docker compose -f compose.yaml -f docker/compose.gui.yaml run --rm workspace workspace-tmux
```

Only `/tmp/.X11-unix` is mounted. Host home directories and the Docker socket are
not mounted, and privileged mode is not used.

For Intel/AMD graphics, when `/dev/dri` exists, add the optional DRI overlay:

```bash
export VIDEO_GID="$(getent group video | cut -d: -f3)"
export RENDER_GID="$(getent group render | cut -d: -f3)"
docker compose -f compose.yaml -f docker/compose.gui.yaml -f docker/compose.dri.yaml run --rm workspace workspace-tmux
```

For NVIDIA, install the host driver and NVIDIA Container Toolkit first, configure
the Docker runtime, and use the GPU overlay:

```bash
docker compose -f compose.yaml -f docker/compose.gui.yaml -f docker/compose.nvidia.yaml run --rm workspace workspace-tmux
```

## Validation

The image build runs rosdep dependency checks, builds all packages and verifies
that the UVC/Gazebo packages and tmux are installed. Run the included simulation
check without a desktop or GPU:

```bash
docker compose run --rm workspace workspace-smoke-test
```

It uses a virtual X server and software rendering, launches the theatre and CAD
robot, waits for odometry, wheel states, lidar and IMU, commands a short forward
motion, stops the robot and checks that Gazebo contains it. It prints `PASS` or
exits nonzero. The simulation log is `/tmp/addison-smoke-test.log` inside the
container; omit `--rm` if you want to inspect a failed container afterward.

## Source changes and persistence

Source is baked into the image. After editing host files, rebuild with
`docker compose build`. Host edits are not automatically reflected in containers.
To experiment with host source mounted inside a container:

```bash
docker compose run --rm \
  --volume "$PWD/src:/home/itadmin/addison_ws/src" workspace bash
# Inside the container:
colcon build --symlink-install --executor sequential
source install/setup.bash
```

Changes to mounted source persist on the host. Rebuilt install files and other
container-only changes disappear when a `--rm` container exits. Avoid mounting
host `build` or `install`; rebuilds should use the container's ROS/compiler stack.

ROS networking defaults to domain 42 and Gazebo partition `addison_docker`, using
Linux host networking. Change `ROS_DOMAIN_ID`/`GZ_PARTITION` in the environment
when you deliberately want to connect other processes. The default separate
ROS domain avoids mixing with a host simulation on domain 0. Cyclone DDS avoids
requiring shared host IPC for DDS discovery/data transport.

This environment supplies navigation dependencies; it does not create a theatre
map or configure UVC autonomous navigation. Those remain separate tasks.

## References

- [Official ROS Docker image and Jazzy tags](https://hub.docker.com/_/ros)
- [ROS/Gazebo version pairing and installation](https://gazebosim.org/docs/harmonic/ros_installation/)
- [Docker Engine installation on Ubuntu](https://docs.docker.com/engine/install/ubuntu/)
- [Docker Compose GPU configuration](https://docs.docker.com/compose/how-tos/gpu-support/)
