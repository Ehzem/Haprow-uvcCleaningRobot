#!/usr/bin/env bash
# Test this checkout's UVC keyboard controls using the existing Docker image.
set -euo pipefail
root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${root_dir}"
mode="${1:-}"
if [[ "${mode}" != '' && "${mode}" != '--headless' ]]; then
    echo 'Usage: ./docker/test-motion.sh [--headless]' >&2
    exit 2
fi
compose=(sudo docker compose -f compose.yaml)
launch='workspace-tmux'
if [[ "${mode}" == '--headless' ]]; then
    launch='workspace-tmux --headless'
else
    : "${DISPLAY:?Run from a desktop terminal or use --headless}"
    command -v xhost >/dev/null || { echo 'Install x11-xserver-utils on the host first.' >&2; exit 1; }
    desktop_user="$(id -un)"
    # Revoke only a grant we added; preserve any pre-existing grant.
    if [[ "$(xhost)" != *"SI:localuser:${desktop_user}"* ]]; then
        xhost "+si:localuser:${desktop_user}"
        trap 'xhost "-si:localuser:${desktop_user}" >/dev/null' EXIT
    fi
    compose=(sudo env "DISPLAY=${DISPLAY}" docker compose -f compose.yaml -f docker/compose.gui.yaml)
fi
# A disposable container: source is read-only and existing development volumes
# are not mounted. Build products go only to this temporary container.
"${compose[@]}" run --rm \
    --env "ROS_DOMAIN_ID=${UVC_TEST_ROS_DOMAIN_ID:-87}" \
    --env "GZ_PARTITION=uvc_keyboard_test_$$" \
    --volume "${root_dir}/src:/home/itadmin/addison_ws/src:ro" \
    --volume "${root_dir}/docker/tmux.sh:/usr/local/bin/workspace-tmux:ro" \
    workspace bash -ic \
    "colcon build --packages-select uvc_robot_description --symlink-install --executor sequential && source install/setup.bash && ${launch}"
