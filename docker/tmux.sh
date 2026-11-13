#!/usr/bin/env bash
set -e
headless=false
rviz=true
case "${1:-}" in
    '') ;;
    --headless) headless=true; rviz=false ;;
    *) echo 'Usage: workspace-tmux [--headless]' >&2; exit 2 ;;
esac
session=theatre
if ! tmux has-session -t "${session}" 2>/dev/null; then
    launcher='ros2 launch yahboom_rosmaster_gazebo operating_theatre.launch.py'
    if [[ "${headless}" == true && -z "${DISPLAY:-}" ]]; then
        launcher="env LIBGL_ALWAYS_SOFTWARE=1 xvfb-run -a ${launcher}"
    fi
    tmux new-session -d -s "${session}" -n simulation -c "${ROS_WORKSPACE}" \
        "bash -ic '${launcher} headless:=${headless} use_rviz:=${rviz}; exec bash'"
    tmux new-window -t "${session}" -n terminal -c "${ROS_WORKSPACE}" bash
    tmux new-window -t "${session}" -n teleop -c "${ROS_WORKSPACE}" bash
    tmux select-window -t "${session}:simulation"
fi
if [[ -n "${TMUX:-}" ]]; then
    exec tmux switch-client -t "${session}"
fi
exec tmux attach-session -t "${session}"
