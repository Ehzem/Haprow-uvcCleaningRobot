#!/usr/bin/env python3
"""Record Linux CPU counters to CSV without installing monitoring packages."""
import argparse
import csv
from dataclasses import dataclass
from datetime import datetime, timezone
import json
import math
import os
from pathlib import Path
import platform
import signal
import time

CPU_FIELDS = ('user', 'nice', 'kernel', 'idle', 'iowait', 'irq', 'softirq', 'steal')


@dataclass
class Process:
    pid: int
    name: str
    user: int
    kernel: int
    start: int
    threads: int
    last_cpu: int
    role: str = 'other'


def parse_process_stat(text):
    left, right = text.index('('), text.rindex(')')
    fields = text[right+2:].split()  # First field here is field 3 (state).
    return Process(int(text[:left].strip()), text[left+1:right], int(fields[11]),
                   int(fields[12]), int(fields[19]), int(fields[17]), int(fields[36]))


def cpu_percent(previous, current):
    delta = [b-a for a, b in zip(previous[:8], current[:8])]
    if len(delta) != 8 or min(delta) < 0 or sum(delta) <= 0:
        return None
    total = sum(delta)
    result = {name+'_pct': 100*value/total for name, value in zip(CPU_FIELDS, delta)}
    # Waiting for I/O and stolen virtual-CPU time are not active execution.
    result['busy_pct'] = sum(result[name+'_pct'] for name in
                             ('user', 'nice', 'kernel', 'irq', 'softirq'))
    return result


def process_delta(previous, current, previous_uptime, hz):
    if previous is not None and previous.start == current.start:
        delta = (current.user-previous.user, current.kernel-previous.kernel)
        return delta if min(delta) >= 0 else None
    if current.start >= previous_uptime*hz:
        return current.user, current.kernel  # Born during the sampled interval.
    return None  # No valid baseline, e.g. PID reuse or newly readable process.


def classify(pid, name, argv):
    if pid == os.getpid():
        return 'recorder'
    text = ' '.join(argv[:6]).lower()
    for role, markers in (
        ('keyboard', ('uvc_keyboard', 'teleop_twist_keyboard')),
        ('rviz', ('rviz2',)),
        ('gazebo', ('gz sim', 'gz-sim', 'gz_sim')),
        ('ros_bridge', ('parameter_bridge', 'image_bridge')),
        ('robot_state', ('robot_state_publisher',)),
        ('build', ('colcon', 'cmake', 'cc1plus', 'cc1', 'g++', 'ninja', '/make')),
    ):
        if any(marker in text or marker in name.lower() for marker in markers):
            return role
    return 'other'


def cgroup_directory(proc=Path('/proc'), root=Path('/sys/fs/cgroup')):
    try:
        for line in (proc/'self/cgroup').read_text().splitlines():
            if line.startswith('0::'):
                candidate = root / line[3:].lstrip('/')
                if (candidate/'cpu.stat').is_file():
                    return candidate
    except OSError:
        pass
    return None


def read_group(directory):
    if directory is None:
        return {}
    try:
        return {key: int(value) for key, value in
                (line.split() for line in (directory/'cpu.stat').read_text().splitlines())}
    except (OSError, ValueError):
        return {}


def snapshot(group):
    cpus = {}
    for line in Path('/proc/stat').read_text().splitlines():
        fields = line.split()
        if fields and fields[0].startswith('cpu'):
            cpus[fields[0]] = list(map(int, fields[1:9]))
    uptime = float(Path('/proc/uptime').read_text().split()[0])
    processes, unreadable = {}, 0
    for entry in Path('/proc').iterdir():
        if not entry.name.isdecimal():
            continue
        try:
            process = parse_process_stat((entry/'stat').read_text())
            try:
                argv = (entry/'cmdline').read_bytes().decode(errors='replace').split('\0')
            except OSError:
                argv = []
            process.role = classify(process.pid, process.name, argv)
            processes[process.pid] = process
        except FileNotFoundError:
            continue  # Normal: process exited while sampling.
        except (OSError, ValueError, IndexError):
            unreadable += 1
    return {'cpus': cpus, 'uptime': uptime, 'processes': processes, 'unreadable': unreadable,
            'group': read_group(group), 'time': time.monotonic()}


def utc():
    return datetime.now(timezone.utc).isoformat()


def record(output, interval, duration):
    output.mkdir(parents=True, exist_ok=False)
    hz = os.sysconf('SC_CLK_TCK')
    group = cgroup_directory()
    affinity = len(os.sched_getaffinity(0))
    quota = None
    if group is not None:
        try:
            limit, period = (group/'cpu.max').read_text().split()
            if limit != 'max':
                quota = int(limit)/int(period)
        except (OSError, ValueError):
            pass
    capacity = min(affinity, quota) if quota is not None else affinity
    previous = snapshot(group)
    start = previous['time']
    metadata = dict(started_utc=utc(), interval_seconds=interval,
                    duration_requested_seconds=duration, kernel=platform.release(),
                    inside_docker=Path('/.dockerenv').exists(), clock_ticks_per_second=hz,
                    online_host_cpus=len(previous['cpus'])-1, affinity_cpu_count=affinity,
                    cgroup_v2_path=str(group) if group else None, cpu_max_quota_cores=quota,
                    configured_cgroup_capacity_cores=capacity,
                    notes=[
                        'system.csv uses host-wide /proc/stat, even inside Docker.',
                        'processes.csv records all visible PIDs; percentages include all process threads.',
                        '100% process/cgroup CPU means one fully occupied logical CPU.',
                        'cgroup.csv covers the recorder cgroup, normally its Docker container when inside Docker.',
                        'Parent cgroup quotas are not inferred; own cpu.max and affinity define configured capacity.',
                        'No command-line arguments or credentials are written.',
                        'Sampling can miss short-lived processes; host/cgroup aggregate counters still include their CPU time.',
                        'I/O wait and steal are reported separately from busy execution.'])
    (output/'metadata.json').write_text(json.dumps(metadata, indent=2)+'\n')
    system_fields = ['timestamp_utc', 'elapsed_s', 'interval_s', 'cpu', 'busy_pct'] + [n+'_pct' for n in CPU_FIELDS]
    process_fields = ['timestamp_utc', 'elapsed_s', 'interval_s', 'pid', 'start_ticks', 'process',
                      'role', 'threads', 'last_cpu', 'user_cpu_pct', 'kernel_cpu_pct',
                      'cpu_pct_one_core', 'cpu_pct_host_capacity', 'observed_cpu_seconds']
    group_fields = ['timestamp_utc', 'elapsed_s', 'interval_s', 'cpu_pct_one_core',
                    'cpu_pct_configured_capacity', 'usage_usec_delta', 'throttled_usec_delta',
                    'nr_throttled_delta', 'unreadable_pids', 'visible_pids']
    handles = [(output/name).open('w', newline='') for name in ('system.csv', 'processes.csv', 'cgroup.csv')]
    writers = [csv.DictWriter(handle, fieldnames=fields) for handle, fields in
               zip(handles, (system_fields, process_fields, group_fields))]
    for writer in writers:
        writer.writeheader()
    stop = False
    def request_stop(_signal, _frame):
        nonlocal stop
        stop = True
    handlers = {s: signal.signal(s, request_stop) for s in (signal.SIGINT, signal.SIGTERM)}
    samples = group_samples = 0
    totals = {}
    busy_sum = peak_busy = group_usec = 0.
    busy_duration = 0.
    next_time = start+interval
    print(f'Recording CPU to {output.resolve()} every {interval:g}s; Ctrl+C stops and saves.', flush=True)
    try:
        while not stop:
            deadline = min(next_time, start+duration) if duration is not None else next_time
            time.sleep(max(0., deadline-time.monotonic()))
            current = snapshot(group)
            dt = current['time']-previous['time']
            elapsed = current['time']-start
            stamp = utc()
            common = dict(timestamp_utc=stamp, elapsed_s=round(elapsed, 6), interval_s=round(dt, 6))
            host_cpus = max(1, len(current['cpus'])-1)
            for name, counters in current['cpus'].items():
                values = cpu_percent(previous['cpus'].get(name, []), counters)
                if values is None:
                    continue
                writers[0].writerow(dict(common, cpu=name, **values))
                if name == 'cpu':
                    busy_sum += values['busy_pct']*dt
                    busy_duration += dt
                    peak_busy = max(peak_busy, values['busy_pct'])
            for pid, process in current['processes'].items():
                delta = process_delta(previous['processes'].get(pid), process, previous['uptime'], hz)
                if delta is None:
                    continue
                user_pct, kernel_pct = [100*ticks/hz/dt for ticks in delta]
                used = sum(delta)/hz
                writers[1].writerow(dict(common, pid=pid, start_ticks=process.start,
                    process=process.name, role=process.role, threads=process.threads,
                    last_cpu=process.last_cpu, user_cpu_pct=user_pct, kernel_cpu_pct=kernel_pct,
                    cpu_pct_one_core=user_pct+kernel_pct,
                    cpu_pct_host_capacity=(user_pct+kernel_pct)/host_cpus, observed_cpu_seconds=used))
                entry = totals.setdefault((pid, process.start), dict(pid=pid, start_ticks=process.start,
                    process=process.name, role=process.role, observed_cpu_seconds=0., peak_cpu_pct_one_core=0.))
                entry['observed_cpu_seconds'] += used
                entry['peak_cpu_pct_one_core'] = max(entry['peak_cpu_pct_one_core'], user_pct+kernel_pct)
            cg, old = current['group'], previous['group']
            row = dict(common, unreadable_pids=current['unreadable'], visible_pids=len(current['processes']))
            if 'usage_usec' in cg and 'usage_usec' in old and cg['usage_usec'] >= old['usage_usec']:
                used = cg['usage_usec']-old['usage_usec']
                group_usec += used
                group_samples += 1
                row.update(cpu_pct_one_core=100*used/1e6/dt,
                           cpu_pct_configured_capacity=100*used/1e6/dt/capacity,
                           usage_usec_delta=used)
                for key in ('throttled_usec', 'nr_throttled'):
                    if key in cg and key in old:
                        row[key+'_delta'] = max(0, cg[key]-old[key])
            writers[2].writerow(row)
            for handle in handles:
                handle.flush()
            samples += 1
            previous = current
            if duration is not None and elapsed >= duration:
                break
            next_time = max(next_time+interval, current['time']+.001)
    finally:
        for handle in handles:
            handle.close()
        for sig, handler in handlers.items():
            signal.signal(sig, handler)
        elapsed = previous['time']-start
        summary = dict(ended_utc=utc(), samples=samples, recorded_seconds=elapsed,
            host_average_busy_pct=busy_sum/busy_duration if busy_duration else None,
            host_peak_sample_busy_pct=peak_busy if samples else None,
            cgroup_observed_cpu_seconds=group_usec/1e6 if group_samples else None,
            top_processes=sorted(totals.values(), key=lambda x:x['observed_cpu_seconds'], reverse=True)[:25])
        (output/'summary.json').write_text(json.dumps(summary, indent=2)+'\n')
    print(f'Saved {samples} samples; see system.csv, processes.csv, cgroup.csv and summary.json.', flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--interval', type=float, default=1., help='seconds between samples (default 1)')
    parser.add_argument('--duration', type=float, help='seconds to record; otherwise stop with Ctrl+C')
    parser.add_argument('--output', type=Path, help='new output directory; existing directories are never overwritten')
    args = parser.parse_args()
    if not (math.isfinite(args.interval) and args.interval > 0):
        parser.error('--interval must be positive and finite')
    if args.duration is not None and not (math.isfinite(args.duration) and args.duration > 0):
        parser.error('--duration must be positive and finite')
    if not Path('/proc/stat').is_file():
        parser.error('This recorder requires Linux /proc')
    output = args.output or Path('cpu_recordings')/datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S.%fZ')
    if output.exists():
        parser.error('Output directory already exists; choose a new run name')
    record(output, args.interval, args.duration)


if __name__ == '__main__':
    main()
