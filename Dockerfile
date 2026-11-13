# syntax=docker/dockerfile:1
FROM ros:jazzy-ros-base-noble

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
ENV DEBIAN_FRONTEND=noninteractive

# Jazzy's ros_gz packages supply the matching Gazebo Harmonic vendor libraries.
RUN apt-get update && apt-get install -y --no-install-recommends \
      build-essential cmake git less nano ninja-build pkg-config \
      python3-colcon-common-extensions python3-matplotlib python3-rosdep \
      python3-vcstool tmux tini xauth xvfb \
      libegl1 libgl1 libgl1-mesa-dri libglx-mesa0 libxkbcommon-x11-0 \
      mesa-utils mesa-vulkan-drivers \
      ros-jazzy-desktop ros-jazzy-ros-gz ros-jazzy-rmw-cyclonedds-cpp \
      ros-jazzy-teleop-twist-keyboard \
    && rm -rf /var/lib/apt/lists/*

ARG USER_UID=1000
ARG USER_GID=1000
# Ubuntu Noble images may already contain the uid-1000 account "ubuntu".
RUN if id ubuntu >/dev/null 2>&1; then \
      usermod --login itadmin --home /home/itadmin --move-home ubuntu; \
      groupmod --new-name itadmin ubuntu; \
      groupmod --gid "${USER_GID}" itadmin; \
      usermod --uid "${USER_UID}" --gid "${USER_GID}" --shell /bin/bash itadmin; \
    else \
      groupadd --gid "${USER_GID}" itadmin; \
      useradd --uid "${USER_UID}" --gid "${USER_GID}" --create-home --shell /bin/bash itadmin; \
    fi

ENV ROS_WORKSPACE=/home/itadmin/addison_ws \
    RMW_IMPLEMENTATION=rmw_cyclonedds_cpp \
    QT_X11_NO_MITSHM=1 \
    TERM=xterm-256color
WORKDIR /home/itadmin/addison_ws
COPY --chown=itadmin:itadmin src/ ./src/

# Resolve all packages, including navigation, docking, control and test tools.
# Fail the build on unresolved dependencies instead of producing a partial image.
RUN if [ ! -f /etc/ros/rosdep/sources.list.d/20-default.list ]; then rosdep init; fi \
    && rosdep update --rosdistro jazzy \
    && apt-get update \
    && source /opt/ros/jazzy/setup.bash \
    && rosdep install --from-paths src --ignore-src --rosdistro jazzy -y \
    && rosdep check --from-paths src --ignore-src --rosdistro jazzy \
    && rm -rf /var/lib/apt/lists/*

COPY docker/environment.bash /etc/addison_ws.bash
COPY docker/entrypoint.sh /usr/local/bin/workspace-entrypoint
COPY docker/tmux.sh /usr/local/bin/workspace-tmux
COPY docker/smoke-test.sh /usr/local/bin/workspace-smoke-test
COPY docker/smoke-test.py /usr/local/lib/addison-smoke-test.py
RUN chmod +x /usr/local/bin/workspace-entrypoint /usr/local/bin/workspace-tmux \
      /usr/local/bin/workspace-smoke-test \
    && printf '\nsource /etc/addison_ws.bash\n' >> /home/itadmin/.bashrc \
    && chown -R itadmin:itadmin /home/itadmin/addison_ws

USER itadmin
ARG BUILD_JOBS=2
# Never reuse host build/install products; compile every source package here.
RUN source /opt/ros/jazzy/setup.bash \
    && CMAKE_BUILD_PARALLEL_LEVEL="${BUILD_JOBS}" MAKEFLAGS="-j${BUILD_JOBS}" \
       colcon build --symlink-install --executor sequential \
         --cmake-args -DCMAKE_BUILD_TYPE=Release \
    && source install/setup.bash \
    && ros2 pkg prefix uvc_robot_description \
    && ros2 pkg prefix yahboom_rosmaster_gazebo \
    && tmux -V

ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/workspace-entrypoint"]
CMD ["bash"]
