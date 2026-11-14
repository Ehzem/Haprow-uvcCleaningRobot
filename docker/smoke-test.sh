#!/usr/bin/env bash
set -e
source /etc/addison_ws.bash
# Always test with a private virtual X server and software rendering.
export LIBGL_ALWAYS_SOFTWARE=1
export QT_QPA_PLATFORM=xcb
export ROS_DOMAIN_ID="${SMOKE_ROS_DOMAIN_ID:-79}"
export GZ_PARTITION="addison_smoke_$$"
exec xvfb-run -a python3 /usr/local/lib/addison-smoke-test.py
