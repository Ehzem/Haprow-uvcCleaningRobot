#!/usr/bin/env python3
"""Connect a new container workspace to Git without replacing working files."""
import argparse
import os
from pathlib import Path
import subprocess


def connect(workspace, repository, branch):
    root = Path(workspace).resolve()
    root.mkdir(parents=True, exist_ok=True)

    def git(*args, **kwargs):
        return subprocess.run(['git', '-C', str(root), *args], check=True, **kwargs)

    if not (root / '.git').exists():
        git('init', '-b', branch)
    origin = subprocess.run(['git', '-C', str(root), 'remote', 'get-url', 'origin'],
                            capture_output=True, text=True)
    if origin.returncode == 0:
        if origin.stdout.strip() != repository:
            raise RuntimeError('Existing origin differs; leaving the repository untouched')
    else:
        git('remote', 'add', 'origin', repository)
    head = subprocess.run(['git', '-C', str(root), 'rev-parse', '--verify', 'HEAD'],
                          capture_output=True)
    if head.returncode == 0:
        print('Existing workspace repository preserved; origin is already configured.')
        return
    # No host credentials are needed for the public repository. Nothing is
    # checked out over the locally copied source or container-built outputs.
    git('-c', 'credential.helper=', 'fetch', 'origin', branch,
        env={**os.environ, 'GIT_TERMINAL_PROMPT': '0'})
    remote_ref = f'refs/remotes/origin/{branch}'
    git('update-ref', f'refs/heads/{branch}', remote_ref)
    git('read-tree', 'HEAD')
    git('branch', f'--set-upstream-to=origin/{branch}', branch)
    files = git('ls-files', '-z', capture_output=True).stdout.split(b'\0')
    missing = []
    for filename in files:
        if not filename:
            continue
        name = os.fsdecode(filename)
        if name.startswith(('build/', 'install/', 'log/')):
            continue
        if not os.path.lexists(root / name):
            missing.append(filename)
    if missing:
        git('checkout-index', '--stdin', '-z', input=b'\0'.join(missing) + b'\0')
    print(f'Connected {root} to {repository} on {branch}; existing files preserved.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--workspace', default=os.environ.get(
        'ROS_WORKSPACE', '/home/itadmin/addison_ws'))
    parser.add_argument('--repository', default=(
        'https://github.com/Ehzem/Haprow-uvcCleaningRobot.git'))
    parser.add_argument('--branch', default='main')
    args = parser.parse_args()
    connect(args.workspace, args.repository, args.branch)


if __name__ == '__main__':
    main()
