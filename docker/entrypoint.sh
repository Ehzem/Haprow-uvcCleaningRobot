#!/usr/bin/env bash
set -e
source /etc/addison_ws.bash
if [[ $# -eq 0 ]]; then
    set -- bash
fi
exec "$@"
